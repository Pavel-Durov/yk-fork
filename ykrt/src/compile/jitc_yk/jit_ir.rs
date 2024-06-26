//! The JIT's Intermediate Representation (IR).
//!
//! This is the IR created by [trace_builder] and which is then optimised. The IR can feel a little
//! odd at first because we store indexes into vectors rather than use direct references. This
//! allows us to squeeze the amount of the memory used down (and also bypasses issues with
//! representing graph structures in Rust, but that's slightly accidental).
//!
//! Because using the IR can often involve getting hold of data nested several layers deep, we also
//! use a number of abbreviations/conventions to keep the length of source down to something
//! manageable (in alphabetical order):
//!
//!  * `const_`: a "constant"
//!  * `decl`: a "declaration" (e.g. a "function declaration" is a reference to an existing
//!    function somewhere else in the address space)
//!  * `m`: the name conventionally given to the shared [Module] instance (i.e. `m: Module`)
//!  * `Idx`: "index"
//!  * `Inst`: "instruction"
//!  * `Ty`: "type"

// For now, don't swamp others working in other areas of the system.
// FIXME: eventually delete.
#![allow(dead_code)]

use super::aot_ir;
use crate::compile::CompilationError;
use indexmap::IndexSet;
use num_traits::{PrimInt, ToBytes};
use std::{
    ffi::{c_void, CStr, CString},
    fmt, mem,
};
use typed_index_collections::{TiSlice, TiVec};
use ykaddr::addr::symbol_to_ptr;

// This is simple and can be shared across both IRs.
pub(crate) use super::aot_ir::Predicate;

/// The `Module` is the top-level container for JIT IR.
///
/// The IR is conceptually a list of word-sized instructions containing indices into auxiliary
/// vectors.
///
/// The instruction stream of a [Module] is partially mutable:
/// - you may append new instructions to the end.
/// - you may replace and instruction with another.
/// - you may NOT remove an instruction.
#[derive(Debug)]
pub(crate) struct Module {
    /// The ID of this compiled trace. In `cfg(test)` this value is meaningless: in
    /// `cfg(not(test))` the ID is obtained from [MT::next_compiled_trace_id()] and can be used to
    /// semi-uniquely distinguish traces (see [MT::compiled_trace_id] for more details).
    ctr_id: u64,
    /// The IR trace as a linear sequence of instructions.
    insts: TiVec<InstIdx, Inst>,
    /// The arguments pool for [CallInst]s. Indexed by [ArgsIdx].
    args: Vec<Operand>,
    /// The constant pool. Indexed by [ConstIdx].
    consts: IndexSet<Constant>,
    /// The type pool. Indexed by [TyIdx].
    types: IndexSet<Ty>,
    /// The type index of the void type. Cached for convenience.
    void_ty_idx: TyIdx,
    /// The type index of a pointer type. Cached for convenience.
    ptr_ty_idx: TyIdx,
    /// The type index of an 8-bit integer. Cached for convenience.
    int8_ty_idx: TyIdx,
    /// The function declaration pool. These are declarations of externally compiled functions that
    /// the JITted trace might need to call. Indexed by [FuncDeclIdx].
    func_decls: IndexSet<FuncDecl>,
    /// The global variable declaration table.
    ///
    /// This is a collection of externally defined global variables that the trace may need to
    /// reference. Because they are externally initialised, these are *declarations*.
    global_decls: IndexSet<GlobalDecl>,
    /// Additional information for guards.
    guard_info: TiVec<GuardInfoIdx, GuardInfo>,
    /// Indirect calls.
    indirect_calls: TiVec<IndirectCallIdx, IndirectCallInst>,
    /// The virtual address of the global variable pointer array.
    ///
    /// This is an array (added to the LLVM AOT module and AOT codegenned by ykllvm) containing a
    /// pointer to each global variable in the AOT module. The indices of the elements correspond
    /// with [aot_ir::GlobalDeclIdx]s.
    ///
    /// This is marked `cfg(not(test))` because unit tests are not built with ykllvm, and thus the
    /// array will be absent.
    #[cfg(not(test))]
    globalvar_ptrs: &'static [*const ()],
}

impl Module {
    /// Create a new [Module].
    pub(crate) fn new(ctr_id: u64, global_decls_len: usize) -> Result<Self, CompilationError> {
        Self::new_internal(ctr_id, global_decls_len)
    }

    #[cfg(test)]
    pub(crate) fn new_testing() -> Self {
        Self::new_internal(0, 0).unwrap()
    }

    pub(crate) fn new_internal(
        ctr_id: u64,
        global_decls_len: usize,
    ) -> Result<Self, CompilationError> {
        // Create some commonly used types ahead of time. Aside from being convenient, this allows
        // us to find their (now statically known) indices in scenarios where Rust forbids us from
        // holding a mutable reference to the Module (and thus we cannot use [Module::ty_idx]).
        let mut types = IndexSet::new();
        let void_ty_idx = TyIdx::new(types.insert_full(Ty::Void).0)?;
        let ptr_ty_idx = TyIdx::new(types.insert_full(Ty::Ptr).0)?;
        let int8_ty_idx = TyIdx::new(types.insert_full(Ty::Integer(IntegerTy::new(8))).0)?;

        // Find the global variable pointer array in the address space.
        //
        // FIXME: consider passing this in to the control point to avoid a dlsym().
        #[cfg(not(test))]
        let globalvar_ptrs = {
            let ptr = symbol_to_ptr(GLOBAL_PTR_ARRAY_SYM).unwrap() as *const *const ();
            unsafe { std::slice::from_raw_parts(ptr, global_decls_len) }
        };
        #[cfg(test)]
        assert_eq!(global_decls_len, 0);

        Ok(Self {
            ctr_id,
            insts: TiVec::new(),
            args: Vec::new(),
            consts: IndexSet::new(),
            types,
            void_ty_idx,
            ptr_ty_idx,
            int8_ty_idx,
            func_decls: IndexSet::new(),
            global_decls: IndexSet::new(),
            guard_info: TiVec::new(),
            indirect_calls: TiVec::new(),
            #[cfg(not(test))]
            globalvar_ptrs,
        })
    }

    /// Get a pointer to an AOT-compiled global variable by a JIT [GlobalDeclIdx].
    ///
    /// # Panics
    ///
    /// Panics if the address cannot be located.
    pub(crate) fn globalvar_ptr(&self, idx: GlobalDeclIdx) -> *const () {
        let decl = self.global_decl(idx);
        #[cfg(not(test))]
        {
            // If the unwrap fails, then the AOT array was absent and something has gone wrong
            // during AOT codegen.
            self.globalvar_ptrs[usize::from(decl.global_ptr_idx())]
        }
        #[cfg(test)]
        {
            // In unit tests the global variable pointer array isn't present, as the
            // unit test binary wasn't compiled with ykllvm. Fall back on dlsym().
            symbol_to_ptr(decl.name().to_str().unwrap()).unwrap()
        }
    }

    /// Returns the type index of [Ty::Void].
    pub(crate) fn void_ty_idx(&self) -> TyIdx {
        self.void_ty_idx
    }

    /// Returns the type index of [Ty::Ptr].
    pub(crate) fn ptr_ty_idx(&self) -> TyIdx {
        self.ptr_ty_idx
    }

    /// Returns the type index of an 8-bit integer.
    pub(crate) fn int8_ty_idx(&self) -> TyIdx {
        self.int8_ty_idx
    }

    /// Return the instruction at the specified index.
    pub(crate) fn inst(&self, idx: InstIdx) -> &Inst {
        &self.insts[idx]
    }

    /// Return the indirect call at the specified index.
    pub(crate) fn indirect_call(&self, idx: IndirectCallIdx) -> &IndirectCallInst {
        &self.indirect_calls[idx]
    }

    /// Push an instruction to the end of the [Module].
    pub(crate) fn push(&mut self, inst: Inst) -> Result<InstIdx, CompilationError> {
        InstIdx::new(self.insts.len()).inspect(|_| self.insts.push(inst))
    }

    pub(crate) fn push_indirect_call(
        &mut self,
        inst: IndirectCallInst,
    ) -> Result<IndirectCallIdx, CompilationError> {
        IndirectCallIdx::new(self.indirect_calls.len()).inspect(|_| self.indirect_calls.push(inst))
    }

    /// Push an instruction to the end of the [Module] and create a local variable [Operand] out of
    /// the value that the instruction defines.
    ///
    /// This is useful for forwarding the local variable a instruction defines as operand of a
    /// subsequent instruction: an idiom used a lot (but not exclusively) in testing.
    ///
    /// Note: it is undefined behaviour to push an instruction that does not define a local
    /// variable.
    pub(crate) fn push_and_make_operand(
        &mut self,
        inst: Inst,
    ) -> Result<Operand, CompilationError> {
        // Assert that `inst` defines a local var.
        debug_assert!(inst.def_type(self).is_some());
        InstIdx::new(self.len()).map(|x| {
            self.insts.push(inst);
            Operand::Local(x)
        })
    }

    /// Returns the number of [Inst]s in the [Module].
    pub(crate) fn len(&self) -> usize {
        self.insts.len()
    }

    /// Return a slice of this module's instructions.
    pub(crate) fn insts(&self) -> &TiSlice<InstIdx, Inst> {
        &self.insts
    }

    /// Push a slice of arguments into the args pool.
    ///
    /// # Panics
    ///
    /// If `args` would overflow the index type.
    fn push_args(&mut self, args: Vec<Operand>) -> Result<ArgsIdx, CompilationError> {
        ArgsIdx::new(self.args.len()).inspect(|_| self.args.extend(args))
    }

    /// Add a [Ty] to the types pool and return its index. If the [Ty] already exists, an existing
    /// index will be returned.
    pub(crate) fn insert_ty(&mut self, ty: Ty) -> Result<TyIdx, CompilationError> {
        let (i, _) = self.types.insert_full(ty);
        TyIdx::new(i)
    }

    /// Return the [Ty] for the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn type_(&self, idx: TyIdx) -> &Ty {
        self.types.get_index(usize::from(idx)).unwrap()
    }

    /// Add a constant to the pool and return its index. If the constant already exists, an
    /// existing index will be returned.
    pub fn insert_const(&mut self, c: Constant) -> Result<ConstIdx, CompilationError> {
        let (i, _) = self.consts.insert_full(c);
        ConstIdx::new(i)
    }

    /// Return the const for the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn const_(&self, idx: ConstIdx) -> &Constant {
        self.consts.get_index(usize::from(idx)).unwrap()
    }

    /// Add a new [GlobalDecl] to the pool and return its index. If the [GlobalDecl] already
    /// exists, an existing index will be returned.
    pub fn insert_global_decl(
        &mut self,
        gd: GlobalDecl,
    ) -> Result<GlobalDeclIdx, CompilationError> {
        let (i, _) = self.global_decls.insert_full(gd);
        GlobalDeclIdx::new(i)
    }

    /// Return the global declaration for the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn global_decl(&self, idx: GlobalDeclIdx) -> &GlobalDecl {
        self.global_decls.get_index(usize::from(idx)).unwrap()
    }

    /// Add a [FuncDecl] to the function declaritons pool and return its index. If the [FuncDecl]
    /// already exists, an existing index will be returned.
    pub(crate) fn insert_func_decl(
        &mut self,
        fd: FuncDecl,
    ) -> Result<FuncDeclIdx, CompilationError> {
        let (i, _) = self.func_decls.insert_full(fd);
        FuncDeclIdx::new(i)
    }

    /// Return the [FuncDecl] for the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds
    pub(crate) fn func_decl(&self, idx: FuncDeclIdx) -> &FuncDecl {
        self.func_decls.get_index(usize::from(idx)).unwrap()
    }

    /// Return the type of the function declaration.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds
    pub(crate) fn func_type(&self, idx: FuncDeclIdx) -> &FuncTy {
        match self.type_(self.func_decl(idx).ty_idx) {
            Ty::Func(ft) => ft,
            _ => unreachable!(),
        }
    }

    pub(crate) fn push_guardinfo(
        &mut self,
        info: GuardInfo,
    ) -> Result<GuardInfoIdx, CompilationError> {
        GuardInfoIdx::new(self.guard_info.len()).inspect(|_| self.guard_info.push(info))
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "; compiled trace ID #{}\n\n; globals\n", self.ctr_id)?;
        for g in &self.global_decls {
            let tl = if g.is_threadlocal() {
                "    ; thread local"
            } else {
                ""
            };
            writeln!(f, "@{}{tl}", g.name.to_str().unwrap_or("<not valid UTF-8>"))?;
        }
        write!(f, "\nentry:")?;
        for (i, inst) in self.insts().iter_enumerated() {
            write!(f, "\n    {}", inst.display(i, self))?;
        }

        Ok(())
    }
}

/// A fixed-width integer type.
///
/// Note:
///   1. These integers range in size from 1..2^23 (inc.) bits. This is inherited [from LLVM's
///      integer type](https://llvm.org/docs/LangRef.html#integer-type).
///   2. Signedness is not specified. Interpretation of the bit pattern is delegated to operations
///      upon the integer.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct IntegerTy {
    num_bits: u32,
}

impl IntegerTy {
    /// Create a new integer type with the specified number of bits.
    pub(crate) fn new(num_bits: u32) -> Self {
        debug_assert!(num_bits > 0 && num_bits <= 0x800000);
        Self { num_bits }
    }

    /// Return the number of bits (1..2^23 (inc.)) this integer spans.
    pub(crate) fn num_bits(&self) -> u32 {
        self.num_bits
    }

    /// Return the number of bytes required to store this integer type.
    ///
    /// Padding for alignment is not included.
    pub(crate) fn byte_size(&self) -> usize {
        let bits = self.num_bits();
        let mut ret = bits / 8;
        // If it wasn't an exactly byte-sized thing, round up to the next byte.
        if bits % 8 != 0 {
            ret += 1;
        }
        // On any 32-bit-or-bigger platform, this `unwrap` can't fail.
        usize::try_from(ret).unwrap()
    }

    /// Instantiate a constant of value `val` and of the type described by `self`.
    pub(crate) fn make_constant<T: ToBytes + PrimInt>(
        &self,
        m: &mut Module,
        val: T,
    ) -> Result<Constant, CompilationError> {
        let ty = Ty::Integer(self.clone());
        let bytes = ToBytes::to_ne_bytes(&val).as_ref().to_vec();
        let ty_size = ty.byte_size().unwrap();
        debug_assert!(ty_size <= bytes.len());
        let bytes_trunc = bytes[0..ty_size].to_owned();
        let ty_idx = m.insert_ty(ty)?;
        Ok(Constant::new(ty_idx, bytes_trunc))
    }

    /// Format a constant integer value that is of the type described by `self`.
    fn const_to_str(&self, c: &Constant) -> String {
        aot_ir::const_int_bytes_to_string(self.num_bits, c.bytes())
    }
}

/// The declaration of a global variable.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct GlobalDecl {
    /// The name of the delcaration.
    name: CString,
    /// Whether the declaration is thread-local.
    is_threadlocal: bool,
    /// The declaration's index in the global variable pointer array.
    global_ptr_idx: aot_ir::GlobalDeclIdx,
}

impl GlobalDecl {
    pub(crate) fn new(
        name: CString,
        is_threadlocal: bool,
        global_ptr_idx: aot_ir::GlobalDeclIdx,
    ) -> Self {
        Self {
            name,
            is_threadlocal,
            global_ptr_idx,
        }
    }

    /// Return the name of the declaration (as a `&CStr`).
    pub(crate) fn name(&self) -> &CStr {
        &self.name
    }

    /// Return whether the declaration is a thread local.
    pub(crate) fn is_threadlocal(&self) -> bool {
        self.is_threadlocal
    }

    /// Return the declaration's index in the global variable pointer array.
    pub(crate) fn global_ptr_idx(&self) -> aot_ir::GlobalDeclIdx {
        self.global_ptr_idx
    }
}

/// Bit fiddling.
///
/// In the constants below:
///  - `*_SIZE`: the size of a field in bits.
///  - `*_MASK`: a mask with one bits occupying the field in question.
///  - `*_SHIFT`: the number of bits required to left shift a field's value into position (from the
///  LSB).
///
const OPERAND_IDX_MASK: u16 = 0x7fff;

/// The largest operand index we can express in 15 bits.
const MAX_OPERAND_IDX: u16 = (1 << 15) - 1;

/// The symbol name of the global variable pointers array.
const GLOBAL_PTR_ARRAY_SYM: &str = "__yk_globalvar_ptrs";

/// A packed 24-bit unsigned integer.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct U24([u8; 3]);

impl U24 {
    /// Create a [U24] from a `usize`. Returns `None` if it won't fit.
    ///
    /// Returns none if the value won't fit.
    fn from_usize(val: usize) -> Option<Self> {
        if val >= 1 << 24 {
            None
        } else {
            let b0 = val & 0xff;
            let b1 = (val & 0xff00) >> 8;
            let b2 = (val & 0xff0000) >> 16;
            Some(Self([b2 as u8, b1 as u8, b0 as u8]))
        }
    }

    /// Converts 3-bytes conceptually representing a `u24` to a usize.
    fn to_usize(self) -> usize {
        static_assertions::const_assert!(mem::size_of::<usize>() >= 3);
        let b0 = self.0[0] as usize; // most-significant byte.
        let b1 = self.0[1] as usize;
        let b2 = self.0[2] as usize;
        (b0 << 16) | (b1 << 8) | b2
    }
}

/// Helper to create index overflow errors.
///
/// FIXME: all of these should be checked at compile time.
fn index_overflow(typ: &str) -> CompilationError {
    CompilationError::LimitExceeded(format!("index overflow: {}", typ))
}

// Generate common methods for 24-bit index types.
macro_rules! index_24bit {
    ($struct:ident) => {
        impl $struct {
            pub(crate) fn new(v: usize) -> Result<Self, CompilationError> {
                U24::from_usize(v)
                    .ok_or(index_overflow(stringify!($struct)))
                    .map(|u| Self(u))
            }

            pub(crate) fn to_usize(self) -> usize {
                self.0.to_usize()
            }
        }

        impl From<usize> for $struct {
            /// Required for TiVec. **DO NOT USE INTERNALLY TO yk as this can `panic`!** Instead,
            /// use [Self::new].
            fn from(v: usize) -> Self {
                Self::new(v).unwrap()
            }
        }

        impl From<$struct> for usize {
            // Required for TiVec.
            fn from(v: $struct) -> Self {
                v.to_usize()
            }
        }
    };
}

// Generate common methods for 16-bit index types.
macro_rules! index_16bit {
    ($struct:ident) => {
        impl $struct {
            pub(crate) fn new(v: usize) -> Result<Self, CompilationError> {
                u16::try_from(v)
                    .map_err(|_| index_overflow(stringify!($struct)))
                    .map(|u| Self(u))
            }

            pub(crate) fn to_u16(self) -> u16 {
                self.0
            }
        }

        impl From<usize> for $struct {
            /// Required for TiVec. **DO NOT USE INTERNALLY TO yk as this can `panic`!** Instead,
            /// use [Self::new].
            fn from(v: usize) -> Self {
                Self::new(v).unwrap()
            }
        }

        impl From<$struct> for usize {
            fn from(s: $struct) -> usize {
                s.0.into()
            }
        }
    };
}

/// A function declaration index.
///
/// One of these is an index into the [Module::func_decls].
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FuncDeclIdx(U24);
index_24bit!(FuncDeclIdx);

/// A type index.
///
/// One of these is an index into the [Module::types].
///
/// A type index uniquely identifies a [Ty] in a [Module]. You can rely on this uniquness
/// property for type checking: you can compare type indices instead of the corresponding [Ty]s.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct TyIdx(U24);
index_24bit!(TyIdx);

/// An argument index. This denotes the start of a slice into [Module::args].
#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct ArgsIdx(u16);
index_16bit!(ArgsIdx);

/// A constant index.
///
/// One of these is an index into the [Module::consts].
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct ConstIdx(u16);
index_16bit!(ConstIdx);

/// A guard info index.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct GuardInfoIdx(pub(crate) u16);
index_16bit!(GuardInfoIdx);

/// A global variable declaration index.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct GlobalDeclIdx(U24);
index_24bit!(GlobalDeclIdx);

/// An indirect call index.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IndirectCallIdx(U24);
index_24bit!(IndirectCallIdx);

/// An instruction index.
///
/// One of these is an index into the [Module::insts].
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub(crate) struct InstIdx(u16);
index_16bit!(InstIdx);

/// A function's type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FuncTy {
    /// Ty indices for the function's formal arguments.
    arg_ty_idxs: Vec<TyIdx>,
    /// Ty index of the function's return type.
    ret_ty_idx: TyIdx,
    /// Is the function vararg?
    is_vararg: bool,
}

impl FuncTy {
    pub(crate) fn new(arg_ty_idxs: Vec<TyIdx>, ret_ty_idx: TyIdx, is_vararg: bool) -> Self {
        Self {
            arg_ty_idxs,
            ret_ty_idx,
            is_vararg,
        }
    }

    /// Return the number of arguments the function accepts (not including varargs arguments).
    pub(crate) fn num_args(&self) -> usize {
        self.arg_ty_idxs.len()
    }

    /// Returns the type index of the argument at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn arg_type<'a>(&self, m: &'a Module, idx: usize) -> &'a Ty {
        m.type_(self.arg_ty_idxs[idx])
    }

    /// Returns whether the function type has vararg arguments.
    pub(crate) fn is_vararg(&self) -> bool {
        self.is_vararg
    }

    /// Returns the type of the return value.
    pub(crate) fn ret_type<'a>(&self, m: &'a Module) -> &'a Ty {
        m.type_(self.ret_ty_idx)
    }

    /// Returns the type index of the return value.
    pub(crate) fn ret_ty_idx(&self) -> TyIdx {
        self.ret_ty_idx
    }
}

/// A structure's type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct StructTy {
    /// The types of the fields.
    field_ty_idxs: Vec<TyIdx>,
    /// The bit offsets of the fields (taking into account any required padding for alignment).
    field_bit_offs: Vec<usize>,
}

/// A type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Ty {
    Void,
    Integer(IntegerTy),
    Ptr,
    Func(FuncTy),
    Struct(StructTy),
    Unimplemented(String),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Integer(it) => write!(f, "i{}", it.num_bits()),
            Self::Ptr => write!(f, "ptr"),
            Self::Func(_) => todo!(),
            Self::Struct(_) => todo!(),
            Self::Unimplemented(_) => write!(f, "?type"),
        }
    }
}

impl Ty {
    /// Returns the size of the type in bits, or `None` if asking the size makes no sense.
    pub(crate) fn byte_size(&self) -> Option<usize> {
        // u16/u32 -> usize conversions could theoretically fail on some arches (which we probably
        // won't ever support).
        match self {
            Self::Void => Some(0),
            Self::Integer(it) => Some(it.byte_size()),
            Self::Ptr => {
                // FIXME: In theory pointers to different types could be of different sizes. We
                // should really ask LLVM how big the pointer was when it codegenned the
                // interpreter, and on a per-pointer basis.
                //
                // For now we assume (and ykllvm assserts this) that all pointers are void
                // pointer-sized.
                Some(mem::size_of::<*const c_void>())
            }
            Self::Func(_) => None,
            Self::Struct(_) => todo!(),
            Self::Unimplemented(_) => None,
        }
    }
}

/// An (externally defined, in the AOT code) function declaration.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FuncDecl {
    name: String,
    ty_idx: TyIdx,
}

impl FuncDecl {
    pub(crate) fn new(name: String, ty_idx: TyIdx) -> Self {
        Self { name, ty_idx }
    }

    /// Return the name of this function declaration.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn ty_idx(&self) -> TyIdx {
        self.ty_idx
    }
}

/// The packed representation of an instruction operand.
///
/// # Encoding
///
/// ```ignore
///  1             15
/// +---+--------------------------+
/// | k |         index            |
/// +---+--------------------------+
/// ```
///
///  - `k=0`: `index` is a local variable index
///  - `k=1`: `index` is a constant index
///
///  The IR can represent 2^{15} = 32768 locals, and as many constants.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct PackedOperand(u16);

impl PackedOperand {
    pub fn new(op: &Operand) -> Self {
        match op {
            Operand::Local(lidx) => {
                debug_assert!(lidx.to_u16() <= MAX_OPERAND_IDX);
                PackedOperand(lidx.to_u16())
            }
            Operand::Const(constidx) => {
                debug_assert!(constidx.to_u16() <= MAX_OPERAND_IDX);
                PackedOperand(constidx.to_u16() | !OPERAND_IDX_MASK)
            }
        }
    }

    /// Unpacks a [PackedOperand] into a [Operand].
    pub fn unpack(&self) -> Operand {
        if (self.0 & !OPERAND_IDX_MASK) == 0 {
            Operand::Local(InstIdx(self.0))
        } else {
            Operand::Const(ConstIdx(self.0 & OPERAND_IDX_MASK))
        }
    }
}

/// An unpacked representation of a operand.
///
/// This exists both as a convenience (as working with packed operands is laborious) and as a means
/// to add type safety when using operands.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Operand {
    Local(InstIdx),
    Const(ConstIdx),
}

impl Operand {
    /// Returns the size of the operand in bytes.
    ///
    /// Assumes no padding is required for alignment.
    ///
    /// # Panics
    ///
    /// Panics if asking for the size make no sense for this operand.
    pub(crate) fn byte_size(&self, m: &Module) -> usize {
        match self {
            Self::Local(l) => m.inst(*l).def_byte_size(m),
            Self::Const(cidx) => m.type_(m.const_(*cidx).ty_idx()).byte_size().unwrap(),
        }
    }

    /// Returns the type index of the operand.
    pub(crate) fn ty_idx(&self, m: &Module) -> TyIdx {
        match self {
            Self::Local(l) => m.inst(*l).def_ty_idx(m),
            Self::Const(c) => m.const_(*c).ty_idx(),
        }
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableOperand<'a> {
        DisplayableOperand { operand: self, m }
    }
}

pub(crate) struct DisplayableOperand<'a> {
    operand: &'a Operand,
    m: &'a Module,
}

impl fmt::Display for DisplayableOperand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.operand {
            Operand::Local(idx) => write!(f, "%{}", idx.to_u16()),
            Operand::Const(idx) => write!(f, "{}", self.m.const_(*idx).display(self.m)),
        }
    }
}

/// A constant value.
///
/// A constant value is represented as a type index and a "bag of bytes". The type index
/// determines the interpretation of the byte bag.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Constant {
    /// The type index of the constant value.
    ty_idx: TyIdx,
    /// The bytes of the constant value.
    bytes: Vec<u8>,
}

impl Constant {
    pub(crate) fn new(ty_idx: TyIdx, bytes: Vec<u8>) -> Self {
        Self { ty_idx, bytes }
    }

    pub(crate) fn ty_idx(&self) -> TyIdx {
        self.ty_idx
    }

    pub(crate) fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableConstant<'a> {
        DisplayableConstant { const_: self, m }
    }
}

pub(crate) struct DisplayableConstant<'a> {
    const_: &'a Constant,
    m: &'a Module,
}

impl fmt::Display for DisplayableConstant<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.m.type_(self.const_.ty_idx()) {
            Ty::Integer(t) => write!(f, "{}", t.const_to_str(self.const_)),
            x => todo!("{x:?}"),
        }
    }
}

#[derive(Debug)]
/// Stores additional guard information.
pub(crate) struct GuardInfo {
    /// Stackmap IDs for the active call frames.
    frames: Vec<u64>,
    /// Indices of live JIT variables.
    lives: Vec<InstIdx>,
}

impl GuardInfo {
    pub(crate) fn new(frames: Vec<u64>, lives: Vec<InstIdx>) -> Self {
        Self { frames, lives }
    }

    pub(crate) fn frames(&self) -> &Vec<u64> {
        &self.frames
    }

    pub(crate) fn lives(&self) -> &Vec<InstIdx> {
        &self.lives
    }
}

/// An IR instruction.
#[repr(u8)]
#[derive(Debug)]
pub enum Inst {
    Load(LoadInst),
    LookupGlobal(LookupGlobalInst),
    LoadTraceInput(LoadTraceInputInst),
    Call(DirectCallInst),
    IndirectCall(IndirectCallIdx),
    PtrAdd(PtrAddInst),
    Store(StoreInst),
    Icmp(IcmpInst),
    Guard(GuardInst),
    /// Describes an argument into the trace function. Its main use is to allow us to track trace
    /// function arguments in case we need to deoptimise them. At this moment the only trace
    /// function argument requiring tracking is the trace inputs.
    Arg(u16),
    /// Marks the place to loop back to at the end of the JITted code.
    TraceLoopStart,
    Assign(AssignInst),

    // Binary operations
    Add(AddInst),
    Sub(SubInst),
    Mul(MulInst),
    Or(OrInst),
    And(AndInst),
    Xor(XorInst),
    Shl(ShlInst),
    AShr(AShrInst),
    FAdd(FAddInst),
    FDiv(FDivInst),
    FMul(FMulInst),
    FRem(FRemInst),
    FSub(FSubInst),
    LShr(LShrInst),
    SDiv(SDivInst),
    SRem(SRemInst),
    UDiv(UDivInst),
    URem(URemInst),

    // Cast-like instructions
    SignExtend(SignExtendInst),
}

impl Inst {
    /// Returns the type of the local variable that the instruction defines (if any).
    pub(crate) fn def_type<'a>(&self, m: &'a Module) -> Option<&'a Ty> {
        let idx = self.def_ty_idx(m);
        if idx != m.void_ty_idx() {
            Some(m.type_(idx))
        } else {
            None
        }
    }

    /// Returns the type index of the local variable defined by the instruction.
    ///
    /// If the instruction doesn't define a type then the type index for [Ty::Void] is returned.
    pub(crate) fn def_ty_idx(&self, m: &Module) -> TyIdx {
        match self {
            Self::Load(li) => li.ty_idx(),
            Self::LookupGlobal(..) => m.ptr_ty_idx(),
            Self::LoadTraceInput(li) => li.ty_idx(),
            Self::Call(ci) => m.func_type(ci.target()).ret_ty_idx(),
            Self::PtrAdd(..) => m.ptr_ty_idx(),
            Self::Store(..) => m.void_ty_idx(),
            Self::Icmp(_) => m.int8_ty_idx(), // always returns a 0/1 valued byte.
            Self::Guard(..) => m.void_ty_idx(),
            Self::Arg(..) => m.ptr_ty_idx(),
            Self::TraceLoopStart => m.void_ty_idx(),
            Self::SignExtend(si) => si.dest_ty_idx(),
            Self::Assign(ai) => ai.opnd().ty_idx(m),
            // Binary operations
            Self::Add(i) => i.ty_idx(m),
            Self::And(i) => i.ty_idx(m),
            Self::Or(i) => i.ty_idx(m),
            Self::IndirectCall(idx) => {
                let inst = &m.indirect_calls[*idx];
                let ty = m.type_(inst.fty_idx);
                let Ty::Func(fty) = ty else { panic!() };
                fty.ret_ty_idx()
            }
            Self::Mul(i) => i.ty_idx(m),
            Self::LShr(i) => i.ty_idx(m),
            x => todo!("{x:?}"),
        }
    }

    /// Returns the size of the local variable that this instruction defines (if any).
    ///
    /// # Panics
    ///
    /// Panics if:
    ///  - The instruction defines no local variable.
    ///  - The instruction defines an unsized local variable.
    pub(crate) fn def_byte_size(&self, m: &Module) -> usize {
        if let Some(ty) = self.def_type(m) {
            if let Some(size) = ty.byte_size() {
                size
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }

    pub(crate) fn display<'a>(&'a self, inst_idx: InstIdx, m: &'a Module) -> DisplayableInst<'a> {
        DisplayableInst {
            inst: self,
            inst_idx,
            m,
        }
    }
}

pub(crate) struct DisplayableInst<'a> {
    inst: &'a Inst,
    inst_idx: InstIdx,
    m: &'a Module,
}

impl fmt::Display for DisplayableInst<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(dt) = self.inst.def_type(self.m) {
            write!(f, "%{}: {dt} = ", self.inst_idx.to_u16())?;
        }
        match self.inst {
            Inst::Load(x) => write!(f, "Load {}", x.operand().display(self.m)),
            Inst::LookupGlobal(x) => write!(
                f,
                "LookupGlobal {}",
                self.m
                    .global_decl(x.global_decl_idx)
                    .name
                    .to_str()
                    .unwrap_or("<not valid UTF-8>")
            ),
            Inst::Call(x) => {
                write!(
                    f,
                    "Call @{}({})",
                    self.m.func_decl(x.target).name(),
                    (0..x.num_args())
                        .map(|y| format!("{}", x.operand(self.m, y).display(self.m)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Inst::IndirectCall(x) => {
                let inst = &self.m.indirect_calls[*x];
                write!(
                    f,
                    "IndirectCall {}({})",
                    inst.target.unpack().display(self.m),
                    (0..inst.num_args())
                        .map(|y| format!("{}", inst.operand(self.m, y).display(self.m)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Inst::PtrAdd(x) => {
                write!(
                    f,
                    "PtrAdd {}, {}",
                    x.ptr().display(self.m),
                    x.offset().display(self.m)
                )
            }
            Inst::Store(x) => write!(
                f,
                "Store {}, {}",
                x.val.unpack().display(self.m),
                x.ptr.unpack().display(self.m)
            ),
            Inst::Icmp(x) => write!(
                f,
                "Icmp {}, {:?}, {}",
                x.left().display(self.m),
                x.pred,
                x.right().display(self.m)
            ),
            Inst::Guard(x) => write!(
                f,
                "Guard {}, {}",
                x.cond().display(self.m),
                if x.expect { "true" } else { "false " }
            ),
            Inst::LoadTraceInput(x) => {
                write!(
                    f,
                    "LoadTraceInput {}, {}",
                    x.off(),
                    self.m.type_(x.ty_idx())
                )
            }
            Inst::TraceLoopStart => write!(f, "TraceLoopStart"),
            Inst::Arg(i) => write!(f, "Arg({i})"),
            Inst::Add(x) => {
                write!(
                    f,
                    "Add {}, {}",
                    x.lhs().display(self.m),
                    x.rhs().display(self.m)
                )
            }
            Inst::And(x) => {
                write!(
                    f,
                    "And {}, {}",
                    x.lhs().display(self.m),
                    x.rhs().display(self.m)
                )
            }
            Inst::SignExtend(i) => {
                write!(
                    f,
                    "SignExtend {}, {}",
                    i.val().display(self.m),
                    self.m.type_(i.dest_ty_idx())
                )
            }
            Inst::Or(i) => {
                write!(
                    f,
                    "Or {}, {}",
                    i.lhs().display(self.m),
                    i.rhs().display(self.m),
                )
            }
            Inst::LShr(i) => {
                write!(
                    f,
                    "LShr {}, {}",
                    i.lhs().display(self.m),
                    i.rhs().display(self.m),
                )
            }
            Inst::Mul(i) => {
                write!(
                    f,
                    "Mul {}, {}",
                    i.lhs().display(self.m),
                    i.rhs().display(self.m),
                )
            }
            Inst::Assign(i) => write!(f, "{}", i.opnd().display(self.m)),
            x => todo!("{x:?}"),
        }
    }
}

macro_rules! inst {
    ($discrim:ident, $inst_type:ident) => {
        impl From<$inst_type> for Inst {
            fn from(inst: $inst_type) -> Inst {
                Inst::$discrim(inst)
            }
        }
    };
}

inst!(Load, LoadInst);
inst!(LookupGlobal, LookupGlobalInst);
inst!(Store, StoreInst);
inst!(LoadTraceInput, LoadTraceInputInst);
inst!(Call, DirectCallInst);
inst!(PtrAdd, PtrAddInst);
inst!(Icmp, IcmpInst);
inst!(Guard, GuardInst);
inst!(SignExtend, SignExtendInst);
inst!(Assign, AssignInst);

/// The operands for a [Inst::Load]
///
/// # Semantics
///
/// Loads a value from a given pointer operand.
///
#[derive(Debug)]
pub struct LoadInst {
    /// The pointer to load from.
    op: PackedOperand,
    /// The type of the pointee.
    ty_idx: TyIdx,
}

impl LoadInst {
    // FIXME: why do we need to provide a type index? Can't we get that from the operand?
    pub(crate) fn new(op: Operand, ty_idx: TyIdx) -> LoadInst {
        LoadInst {
            op: PackedOperand::new(&op),
            ty_idx,
        }
    }

    /// Return the pointer operand.
    pub(crate) fn operand(&self) -> Operand {
        self.op.unpack()
    }

    /// Returns the type index of the loaded value.
    pub(crate) fn ty_idx(&self) -> TyIdx {
        self.ty_idx
    }
}

/// The `LoadTraceInput` instruction.
///
/// ## Semantics
///
/// Loads a trace input out of the trace input struct. The variable is loaded from the specified
/// offset (`off`) and the resulting local variable is of the type indicated by the `ty_idx`.
///
/// FIXME (maybe): If we added a third `TraceInput` storage class to the register allocator, could
/// we kill this instruction kind entirely?
#[derive(Debug)]
#[repr(packed)]
pub struct LoadTraceInputInst {
    /// The byte offset to load from in the trace input struct.
    off: u32,
    /// The type of the resulting local variable.
    ty_idx: TyIdx,
}

impl LoadTraceInputInst {
    pub(crate) fn new(off: u32, ty_idx: TyIdx) -> LoadTraceInputInst {
        Self { off, ty_idx }
    }

    pub(crate) fn ty_idx(&self) -> TyIdx {
        self.ty_idx
    }

    pub(crate) fn off(&self) -> u32 {
        self.off
    }
}

/// The operands for a [Inst::LoadGlobal]
///
/// # Semantics
///
/// Loads a value from a given global variable.
///
/// FIXME: Codegenning this instruction leads to unoptimial code, since all this does is write a
/// constant pointer into a register only to immediately use that register in the following
/// instruction. We'd rather want to inline the constant into the next instruction. So instead of:
/// ```ignore
/// mov rax, 0x123abc
/// mov [rax], 0x1
/// ```
/// we would get
/// ```ignore
/// mov [0x123abc], 0x1
/// ```
/// However, this requires us to change the JIT IR to allow globals inside operands (we don't want
/// to implement a special global version for each instruction, e.g. LoadGlobal/StoreGlobal/etc).
/// The easiest way to do this is to make globals a subclass of constants, similarly to what LLVM
/// does.
#[derive(Debug)]
pub struct LookupGlobalInst {
    /// The pointer to load from.
    global_decl_idx: GlobalDeclIdx,
}

impl LookupGlobalInst {
    #[cfg(not(test))]
    pub(crate) fn new(global_decl_idx: GlobalDeclIdx) -> Result<Self, CompilationError> {
        Ok(Self { global_decl_idx })
    }

    #[cfg(test)]
    pub(crate) fn new(_global_decl_idx: GlobalDeclIdx) -> Result<Self, CompilationError> {
        panic!("Cannot lookup globals in cfg(test) as ykllvm will not have compiled this binary");
    }

    pub(crate) fn decl<'a>(&self, m: &'a Module) -> &'a GlobalDecl {
        m.global_decl(self.global_decl_idx)
    }

    /// Returns the index of the global to lookup.
    pub(crate) fn global_decl_idx(&self) -> GlobalDeclIdx {
        self.global_decl_idx
    }
}
///
/// The operands for a [Inst::IndirectCall]
///
/// # Semantics
///
/// Perform an indirect call to an external or AOT function.
#[derive(Debug)]
pub struct IndirectCallInst {
    /// The callee.
    target: PackedOperand,
    // Type of the target function.
    fty_idx: TyIdx,
    /// How many arguments in [Module::extra_args] is this call passing?
    num_args: u16,
    /// At what index do the contiguous operands in [Module::extra_args] start?
    args_idx: ArgsIdx,
}

impl IndirectCallInst {
    pub(crate) fn new(
        m: &mut Module,
        fty_idx: TyIdx,
        target: Operand,
        args: Vec<Operand>,
    ) -> Result<IndirectCallInst, CompilationError> {
        let num_args = u16::try_from(args.len()).map_err(|_| {
            CompilationError::LimitExceeded(format!(
                "{} arguments passed but at most {} can be handled",
                args.len(),
                u16::MAX
            ))
        })?;
        let args_idx = m.push_args(args)?;
        Ok(Self {
            target: PackedOperand::new(&target),
            fty_idx,
            num_args,
            args_idx,
        })
    }

    /// Return the [TyIdx] of the callee.
    pub(crate) fn fty_idx(&self) -> TyIdx {
        self.fty_idx
    }

    /// Return the callee [Operand].
    pub(crate) fn target(&self) -> Operand {
        self.target.unpack()
    }

    /// How many arguments is this call instruction passing?
    pub(crate) fn num_args(&self) -> usize {
        usize::from(self.num_args)
    }

    /// Fetch the operand at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the operand index is out of bounds.
    pub(crate) fn operand(&self, m: &Module, idx: usize) -> Operand {
        m.args[usize::from(self.args_idx) + idx].clone()
    }
}

/// The operands for a [Inst::Call]
///
/// # Semantics
///
/// Perform a call to an external or AOT function.
#[derive(Debug)]
#[repr(packed)]
pub struct DirectCallInst {
    /// The callee.
    target: FuncDeclIdx,
    /// At what index do the contiguous operands in [Module::args] start?
    args_idx: ArgsIdx,
    /// How many arguments in [Module::args] is this call passing?
    num_args: u16,
}

impl DirectCallInst {
    pub(crate) fn new(
        m: &mut Module,
        target: FuncDeclIdx,
        args: Vec<Operand>,
    ) -> Result<DirectCallInst, CompilationError> {
        let num_args = u16::try_from(args.len()).map_err(|_| {
            CompilationError::LimitExceeded(format!(
                "{} arguments passed but at most {} can be handled",
                args.len(),
                u16::MAX
            ))
        })?;
        let args_idx = m.push_args(args)?;
        Ok(Self {
            target,
            args_idx,
            num_args,
        })
    }

    /// Return the [FuncDeclIdx] of the callee.
    pub(crate) fn target(&self) -> FuncDeclIdx {
        self.target
    }

    /// How many arguments is this call instruction passing?
    pub(crate) fn num_args(&self) -> usize {
        usize::from(self.num_args)
    }

    /// Fetch the operand at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the operand index is out of bounds.
    pub(crate) fn operand(&self, m: &Module, idx: usize) -> Operand {
        m.args[usize::from(self.args_idx) + idx].clone()
    }
}

/// The operands for a [Inst::Store]
///
/// # Semantics
///
/// Stores a value into a pointer.
///
#[derive(Debug)]
pub struct StoreInst {
    /// The value to store.
    val: PackedOperand,
    /// The pointer to store into.
    ptr: PackedOperand,
}

impl StoreInst {
    pub(crate) fn new(val: Operand, ptr: Operand) -> Self {
        // FIXME: assert type of pointer
        Self {
            val: PackedOperand::new(&val),
            ptr: PackedOperand::new(&ptr),
        }
    }

    /// Returns the value operand: i.e. the thing that is going to be stored.
    pub(crate) fn val(&self) -> Operand {
        self.val.unpack()
    }

    /// Returns the pointer operand: i.e. where to store the thing.
    pub(crate) fn ptr(&self) -> Operand {
        self.ptr.unpack()
    }
}

/// A pointer offsetting instruction.
///
/// # Semantics
///
/// Returns a pointer value that is the result of adding the specified (byte) offset to the input
/// pointer operand.
#[derive(Debug)]
#[repr(packed)]
pub struct PtrAddInst {
    /// The pointer to offset
    ptr: PackedOperand,
    /// The offset.
    off: PackedOperand,
}

impl PtrAddInst {
    pub(crate) fn ptr(&self) -> Operand {
        let ptr = self.ptr;
        ptr.unpack()
    }

    pub(crate) fn offset(&self) -> Operand {
        let unaligned = std::ptr::addr_of!(self.off);
        unsafe { std::ptr::read_unaligned(unaligned) }.unpack()
    }

    pub(crate) fn new(ptr: Operand, off: Operand) -> Self {
        Self {
            ptr: PackedOperand::new(&ptr),
            off: PackedOperand::new(&off),
        }
    }
}

macro_rules! bin_op {
    ($discrim :ident, $struct: ident, $disp: literal) => {
        #[derive(Debug)]
        pub struct $struct {
            /// The left-hand side of the operation.
            lhs: PackedOperand,
            /// The right-hand side of the operation.
            rhs: PackedOperand,
        }

        impl $struct {
            pub(crate) fn new(lhs: Operand, rhs: Operand) -> Self {
                Self {
                    lhs: PackedOperand::new(&lhs),
                    rhs: PackedOperand::new(&rhs),
                }
            }

            pub(crate) fn lhs(&self) -> Operand {
                self.lhs.unpack()
            }

            pub(crate) fn rhs(&self) -> Operand {
                self.rhs.unpack()
            }

            /// Returns the type index of the operands being added.
            pub(crate) fn ty_idx(&self, m: &Module) -> TyIdx {
                self.lhs.unpack().ty_idx(m)
            }
        }

        impl From<$struct> for Inst {
            fn from(inst: $struct) -> Inst {
                Inst::$discrim(inst)
            }
        }
    };
}

bin_op!(Add, AddInst, "Add");
bin_op!(Sub, SubInst, "Sub");
bin_op!(Mul, MulInst, "Mul");
bin_op!(Or, OrInst, "Or");
bin_op!(And, AndInst, "And");
bin_op!(Xor, XorInst, "Xor");
bin_op!(Shl, ShlInst, "Shl");
bin_op!(AShr, AShrInst, "AShr");
bin_op!(FAdd, FAddInst, "FAdd");
bin_op!(FDiv, FDivInst, "FDiv");
bin_op!(FMul, FMulInst, "FMul");
bin_op!(FRem, FRemInst, "FRem");
bin_op!(FSub, FSubInst, "FSub");
bin_op!(LShr, LShrInst, "LShr");
bin_op!(SDiv, SDivInst, "SDiv");
bin_op!(SRem, SRemInst, "SRem");
bin_op!(UDiv, UDivInst, "UDiv");
bin_op!(URem, URemInst, "URem");

/// The operand for a [Inst::Icmp]
///
/// # Semantics
///
/// Compares two integer operands according to a predicate (e.g. greater-than). Defines a local
/// variable that dictates the truth of the comparison.
///
#[derive(Debug)]
pub struct IcmpInst {
    left: PackedOperand,
    pred: Predicate,
    right: PackedOperand,
}

impl IcmpInst {
    pub(crate) fn new(op1: Operand, pred: Predicate, op2: Operand) -> Self {
        Self {
            left: PackedOperand::new(&op1),
            pred,
            right: PackedOperand::new(&op2),
        }
    }

    /// Returns the left-hand-side of the comparison.
    ///
    /// E.g. in `x <= y`, it's `x`.
    pub(crate) fn left(&self) -> Operand {
        self.left.unpack()
    }

    /// Returns the right-hand-side of the comparison.
    ///
    /// E.g. in `x <= y`, it's `y`.
    pub(crate) fn right(&self) -> Operand {
        self.right.unpack()
    }

    /// Returns the predicate of the comparison.
    ///
    /// E.g. in `x <= y`, it's `<=`.
    pub(crate) fn predicate(&self) -> Predicate {
        self.pred
    }
}

/// The operand for a [Inst::Guard]
///
/// # Semantics
///
/// Guards a trace against diverging execution. The remainder of the trace will be compiled under
/// the assumption that (at runtime) the guard condition is true. If the guard condition is false,
/// then execution may not continue, and deoptimisation must occur.
///
#[derive(Debug)]
pub struct GuardInst {
    /// The condition to guard against.
    cond: PackedOperand,
    /// The expected outcome of the condition.
    expect: bool,
    /// Additional information about this guard.
    gidx: GuardInfoIdx,
}

impl GuardInst {
    pub(crate) fn new(cond: Operand, expect: bool, gidx: GuardInfoIdx) -> Self {
        GuardInst {
            cond: PackedOperand::new(&cond),
            expect,
            gidx,
        }
    }

    pub(crate) fn cond(&self) -> Operand {
        self.cond.unpack()
    }

    pub(crate) fn expect(&self) -> bool {
        self.expect
    }

    pub(crate) fn guard_info<'a>(&self, m: &'a Module) -> &'a GuardInfo {
        &m.guard_info[self.gidx]
    }
}

#[derive(Debug)]
pub struct SignExtendInst {
    /// The value to extend.
    val: PackedOperand,
    /// The type to extend to.
    dest_ty_idx: TyIdx,
}

impl SignExtendInst {
    pub(crate) fn new(val: &Operand, dest_ty_idx: TyIdx) -> Self {
        Self {
            val: PackedOperand::new(val),
            dest_ty_idx,
        }
    }

    pub(crate) fn val(&self) -> Operand {
        self.val.unpack()
    }

    pub(crate) fn dest_ty_idx(&self) -> TyIdx {
        self.dest_ty_idx
    }
}

#[derive(Debug)]
pub struct AssignInst {
    /// The condition to guard against.
    opnd: PackedOperand,
}

impl AssignInst {
    pub(crate) fn new(opnd: &Operand) -> Self {
        Self {
            opnd: PackedOperand::new(opnd),
        }
    }

    pub(crate) fn opnd(&self) -> Operand {
        self.opnd.unpack()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operand() {
        let op = PackedOperand::new(&Operand::Local(InstIdx(192)));
        assert_eq!(op.unpack(), Operand::Local(InstIdx(192)));

        let op = PackedOperand::new(&Operand::Local(InstIdx(0x7fff)));
        assert_eq!(op.unpack(), Operand::Local(InstIdx(0x7fff)));

        let op = PackedOperand::new(&Operand::Local(InstIdx(0)));
        assert_eq!(op.unpack(), Operand::Local(InstIdx(0)));

        let op = PackedOperand::new(&Operand::Const(ConstIdx(192)));
        assert_eq!(op.unpack(), Operand::Const(ConstIdx(192)));

        let op = PackedOperand::new(&Operand::Const(ConstIdx(0x7fff)));
        assert_eq!(op.unpack(), Operand::Const(ConstIdx(0x7fff)));

        let op = PackedOperand::new(&Operand::Const(ConstIdx(0)));
        assert_eq!(op.unpack(), Operand::Const(ConstIdx(0)));
    }

    #[test]
    fn use_case_update_inst() {
        let mut prog: Vec<Inst> = vec![
            LoadTraceInputInst::new(0, TyIdx::new(0).unwrap()).into(),
            LoadTraceInputInst::new(8, TyIdx::new(0).unwrap()).into(),
            LoadInst::new(
                Operand::Local(InstIdx(0)),
                TyIdx(U24::from_usize(0).unwrap()),
            )
            .into(),
        ];
        prog[2] = LoadInst::new(
            Operand::Local(InstIdx(1)),
            TyIdx(U24::from_usize(0).unwrap()),
        )
        .into();
    }

    /// Ensure that any given instruction fits in 64-bits.
    #[test]
    fn inst_size() {
        assert!(mem::size_of::<Inst>() <= mem::size_of::<u64>());
    }

    #[test]
    fn vararg_call_args() {
        // Set up a function to call.
        let mut m = Module::new_testing();
        let i32_tyidx = m.insert_ty(Ty::Integer(IntegerTy::new(32))).unwrap();
        let func_ty = Ty::Func(FuncTy::new(vec![i32_tyidx; 3], i32_tyidx, true));
        let func_ty_idx = m.insert_ty(func_ty).unwrap();
        let func_decl_idx = m
            .insert_func_decl(FuncDecl::new("foo".to_owned(), func_ty_idx))
            .unwrap();

        // Build a call to the function.
        let args = vec![
            Operand::Local(InstIdx(0)),
            Operand::Local(InstIdx(1)),
            Operand::Local(InstIdx(2)),
        ];
        let ci = DirectCallInst::new(&mut m, func_decl_idx, args).unwrap();

        // Now request the operands and check they all look as they should.
        assert_eq!(ci.operand(&m, 0), Operand::Local(InstIdx(0)));
        assert_eq!(ci.operand(&m, 1), Operand::Local(InstIdx(1)));
        assert_eq!(ci.operand(&m, 2), Operand::Local(InstIdx(2)));
        assert_eq!(
            m.args,
            vec![
                Operand::Local(InstIdx(0)),
                Operand::Local(InstIdx(1)),
                Operand::Local(InstIdx(2))
            ]
        );
    }

    #[test]
    #[should_panic]
    fn call_args_out_of_bounds() {
        // Set up a function to call.
        let mut m = Module::new_testing();
        let arg_ty_idxs = vec![m.ptr_ty_idx(); 3];
        let ret_ty_idx = m.insert_ty(Ty::Void).unwrap();
        let func_ty = FuncTy::new(arg_ty_idxs, ret_ty_idx, false);
        let func_ty_idx = m.insert_ty(Ty::Func(func_ty)).unwrap();
        let func_decl_idx = m
            .insert_func_decl(FuncDecl::new("blah".into(), func_ty_idx))
            .unwrap();

        // Now build a call to the function.
        let args = vec![
            Operand::Local(InstIdx(0)),
            Operand::Local(InstIdx(1)),
            Operand::Local(InstIdx(2)),
        ];
        let ci = DirectCallInst::new(&mut m, func_decl_idx, args).unwrap();

        // Request an operand with an out-of-bounds index.
        ci.operand(&m, 3);
    }

    #[test]
    fn u24_from_usize() {
        assert_eq!(U24::from_usize(0x000000), Some(U24([0x00, 0x00, 0x00])));
        assert_eq!(U24::from_usize(0x123456), Some(U24([0x12, 0x34, 0x56])));
        assert_eq!(U24::from_usize(0xffffff), Some(U24([0xff, 0xff, 0xff])));
        assert_eq!(U24::from_usize(0x1000000), None);
        assert_eq!(U24::from_usize(0x1234567), None);
        assert_eq!(U24::from_usize(0xfffffff), None);
    }

    #[test]
    fn u24_to_usize() {
        assert_eq!(U24([0x00, 0x00, 0x00]).to_usize(), 0x000000);
        assert_eq!(U24([0x12, 0x34, 0x56]).to_usize(), 0x123456);
        assert_eq!(U24([0xff, 0xff, 0xff]).to_usize(), 0xffffff);
    }

    #[test]
    fn u24_round_trip() {
        assert_eq!(U24::from_usize(0x000000).unwrap().to_usize(), 0x000000);
        assert_eq!(U24::from_usize(0x123456).unwrap().to_usize(), 0x123456);
        assert_eq!(U24::from_usize(0xffffff).unwrap().to_usize(), 0xffffff);
    }

    #[test]
    fn index24_fits() {
        assert!(TyIdx::new(0).is_ok());
        assert!(TyIdx::new(1).is_ok());
        assert!(TyIdx::new(0x1234).is_ok());
        assert!(TyIdx::new(0x123456).is_ok());
        assert!(TyIdx::new(0xffffff).is_ok());
    }

    #[test]
    fn index24_doesnt_fit() {
        assert!(TyIdx::new(0x1000000).is_err());
        assert!(TyIdx::new(0x1234567).is_err());
        assert!(TyIdx::new(0xeddedde).is_err());
        assert!(TyIdx::new(usize::MAX).is_err());
    }

    #[test]
    fn index16_fits() {
        assert!(ArgsIdx::new(0).is_ok());
        assert!(ArgsIdx::new(1).is_ok());
        assert!(ArgsIdx::new(0x1234).is_ok());
        assert!(ArgsIdx::new(0xffff).is_ok());
    }

    #[test]
    fn index16_doesnt_fit() {
        assert!(ArgsIdx::new(0x10000).is_err());
        assert!(ArgsIdx::new(0x12345).is_err());
        assert!(ArgsIdx::new(0xffffff).is_err());
        assert!(ArgsIdx::new(usize::MAX).is_err());
    }

    #[test]
    fn void_type_size() {
        assert_eq!(Ty::Void.byte_size(), Some(0));
    }

    #[test]
    fn stringify_int_consts() {
        fn check<T: ToBytes + PrimInt>(m: &mut Module, num_bits: u32, val: T, expect: &str) {
            assert!(mem::size_of::<T>() * 8 >= usize::try_from(num_bits).unwrap());
            let c = IntegerTy::new(num_bits).make_constant(m, val).unwrap();
            assert_eq!(c.display(&m).to_string(), expect);
        }

        let mut m = Module::new_testing();

        check(&mut m, 8, 0i8, "0i8");
        check(&mut m, 8, 111i8, "111i8");
        check(&mut m, 8, 127i8, "127i8");
        check(&mut m, 8, -128i8, "-128i8");
        check(&mut m, 8, -1i8, "-1i8");
        check(&mut m, 16, 123i16, "123i16");
        check(&mut m, 32, 123i32, "123i32");
        check(&mut m, 64, 456i64, "456i64");
        check(&mut m, 64, u64::MAX as i64, "-1i64");
        check(&mut m, 64, i64::MAX, "9223372036854775807i64");
    }

    #[test]
    fn print_module() {
        let mut m = Module::new_testing();
        m.push(LoadTraceInputInst::new(0, m.int8_ty_idx()).into())
            .unwrap();
        m.push(LoadTraceInputInst::new(8, m.int8_ty_idx()).into())
            .unwrap();
        m.push(LoadTraceInputInst::new(16, m.int8_ty_idx()).into())
            .unwrap();
        m.insert_global_decl(GlobalDecl::new(
            CString::new("some_global").unwrap(),
            false,
            aot_ir::GlobalDeclIdx::new(0),
        ))
        .unwrap();
        m.insert_global_decl(GlobalDecl::new(
            CString::new("some_thread_local").unwrap(),
            true,
            aot_ir::GlobalDeclIdx::new(1),
        ))
        .unwrap();
        let expect = [
            "; compiled trace ID #0",
            "",
            "; globals",
            "@some_global",
            "@some_thread_local    ; thread local",
            "",
            "entry:",
            "    %0: i8 = LoadTraceInput 0, i8",
            "    %1: i8 = LoadTraceInput 8, i8",
            "    %2: i8 = LoadTraceInput 16, i8",
        ]
        .join("\n");
        assert_eq!(m.to_string(), expect);
    }

    #[test]
    fn integer_type_sizes() {
        for i in 1..8 {
            assert_eq!(IntegerTy::new(i).byte_size(), 1);
        }
        for i in 9..16 {
            assert_eq!(IntegerTy::new(i).byte_size(), 2);
        }
        assert_eq!(IntegerTy::new(127).byte_size(), 16);
        assert_eq!(IntegerTy::new(128).byte_size(), 16);
        assert_eq!(IntegerTy::new(129).byte_size(), 17);
    }
}
