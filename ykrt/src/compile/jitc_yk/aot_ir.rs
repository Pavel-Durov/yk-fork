//! Yk's AOT IR deserialiser.
//!
//! This module contains the data structures for the AOT IR and a parser to read it from its
//! serialised format.
//!
//! The AOT IR accurately reflects the structure and semantics of the AOT binary. As such, it must
//! not be mutated after the fact.
//!
//! The IR is index-centric, meaning that when one data structure refers to another, it is by a
//! numeric index into a backing vector (and not via a Rust reference). We chose to do it like this
//! because a) references can't easily be serialised and deserialised; and b) we didn't want to do
//! another pass over the IR to convert to another version of the data structures that uses
//! references.
//!
//! Each kind of index has a distinct Rust type so that it cannot be accidentally used in place of
//! an unrelated index. This is enforced by `TiVec`.
//!
//! At a high level, the AOT IR contains:
//!  - Functions (which contain basic blocks, which contain individual instructions).
//!  - Global variable declarations.
//!  - Function definitions/declarations.
//!  - Constant values.
//!  - Types, for use by all of the above.
//!
//! Throughout we use the term "definition" to mean something for which we have total IR knowledge
//! of, whereas a "declaration" is something compiled externally that we typically only know the
//! symbol name, address and type of.
//!
//! Elements of the IR can be converted to human-readable forms by calling `to_string()` on them.
//! This is used for testing, but can also be used for debugging.

use byteorder::{NativeEndian, ReadBytesExt};
use deku::prelude::*;
use std::{
    error::Error,
    ffi::CString,
    fmt::{self, Display},
    fs,
    path::PathBuf,
};
use typed_index_collections::TiVec;

/// A magic number that all bytecode payloads begin with.
const MAGIC: u32 = 0xedd5f00d;
/// The version of the bytecode format.
const FORMAT_VERSION: u32 = 0;

/// The symbol name of the control point function (after ykllvm has transformed it).
const CONTROL_POINT_NAME: &str = "__ykrt_control_point";
const LLVM_DEBUG_CALL_NAME: &str = "llvm.dbg.value";

/// The argument index of the trace inputs (live variables) struct at call-sites to the control
/// point call.
const CTRL_POINT_ARGIDX_INPUTS: usize = 2;

// Generate common methods for index types.
macro_rules! index {
    ($struct:ident) => {
        impl $struct {
            #[allow(dead_code)]
            pub(crate) fn new(v: usize) -> Self {
                Self(v)
            }
        }

        impl From<usize> for $struct {
            fn from(idx: usize) -> Self {
                Self(idx)
            }
        }

        impl From<$struct> for usize {
            fn from(s: $struct) -> usize {
                s.0
            }
        }
    };
}

/// An index into [Module::funcs].
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct FuncIdx(usize);
index!(FuncIdx);

/// An index into [Module::types].
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TypeIdx(usize);
index!(TypeIdx);

/// An index into [Func::bblocks].
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct BBlockIdx(usize);
index!(BBlockIdx);

/// An index into [BBlock::instrs].
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct InstrIdx(usize);
index!(InstrIdx);

/// An index into [Module::consts].
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ConstIdx(usize);
index!(ConstIdx);

/// An index into [Module::global_decls].
///
/// Note: these are "declarations" and not "definitions" because they all been AOT code-generated
/// already, and thus come "pre-initialised".
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct GlobalDeclIdx(usize);
index!(GlobalDeclIdx);

/// An index into [FuncType::arg_ty_idxs].
/// ^ FIXME: no it's not! But it should be!
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ArgIdx(usize);
index!(ArgIdx);

/// Helper function for deku `map` attribute. It is necessary to write all the types out in full to
/// avoid type inference errors, so it's easier to have a single helper function rather than inline
/// this into each `map` attribute.
fn map_to_string(v: Vec<u8>) -> Result<String, DekuError> {
    if let Ok(x) = CString::from_vec_with_nul(v) {
        if let Ok(x) = x.into_string() {
            return Ok(x);
        }
    }
    Err(DekuError::Parse("Couldn't map string".to_owned()))
}

/// Helper function for deku `map` attribute. It is necessary to write all the types out in full to
/// avoid type inference errors, so it's easier to have a single helper function rather than inline
/// this into each `map` attribute.
fn map_to_tivec<I, T>(v: Vec<T>) -> Result<TiVec<I, T>, DekuError> {
    Ok(TiVec::from(v))
}

/// A binary operator.
#[deku_derive(DekuRead)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[deku(type = "u8")]
pub(crate) enum BinOp {
    Add = 0,
    Sub,
    Mul,
    Or,
    And,
    Xor,
    Shl,
    AShr,
    FAdd,
    FDiv,
    FMul,
    FRem,
    FSub,
    LShr,
    SDiv,
    SRem,
    UDiv,
    URem,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

/// Uniquely identifies an instruction within a [Module].
#[deku_derive(DekuRead)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) struct InstructionID {
    /// The index of the parent function.
    func_idx: FuncIdx,
    bb_idx: BBlockIdx,
    inst_idx: InstrIdx,
}

impl InstructionID {
    pub(crate) fn new(func_idx: FuncIdx, bb_idx: BBlockIdx, inst_idx: InstrIdx) -> Self {
        Self {
            func_idx,
            bb_idx,
            inst_idx,
        }
    }
}

/// Uniquely identifies a basic block within a [Module].
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BBlockId {
    func_idx: FuncIdx,
    bb_idx: BBlockIdx,
}

impl BBlockId {
    pub(crate) fn new(func_idx: FuncIdx, bb_idx: BBlockIdx) -> Self {
        Self { func_idx, bb_idx }
    }

    pub(crate) fn func_idx(&self) -> FuncIdx {
        self.func_idx
    }

    pub(crate) fn bb_idx(&self) -> BBlockIdx {
        self.bb_idx
    }

    pub(crate) fn is_entry(&self) -> bool {
        self.bb_idx == BBlockIdx(0)
    }
}

/// Predicates for use in numeric comparisons.
#[deku_derive(DekuRead)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[deku(type = "u8")]
pub(crate) enum Predicate {
    Equal = 0,
    NotEqual,
    UnsignedGreater,
    UnsignedGreaterEqual,
    UnsignedLess,
    UnsignedLessEqual,
    SignedGreater,
    SignedGreaterEqual,
    SignedLess,
    SignedLessEqual,
    // FIXME: add floating-point-specific predicates.
}

impl Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The operations that a [Instruction::Cast] can perform.
///
/// FIXME: There are many other operations that we can add here on-demand. See the inheritance
/// hierarchy here: https://llvm.org/doxygen/classllvm_1_1CastInst.html
#[deku_derive(DekuRead)]
#[derive(Debug, Clone, Copy)]
#[deku(type = "u8")]
pub(crate) enum CastKind {
    SignExtend = 0,
}

impl Display for CastKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[deku_derive(DekuRead)]
#[derive(Debug)]
#[deku(type = "u8")]
pub(crate) enum Operand {
    #[deku(id = "0")]
    Constant(ConstIdx),
    // FIXME: rename this to `Local` for consistency with ykllvm's serialiser.
    #[deku(id = "1")]
    LocalVariable(InstructionID),
    #[deku(id = "2")]
    Global(GlobalDeclIdx),
    #[deku(id = "3")]
    Func(FuncIdx),
    #[deku(id = "4")]
    Arg { func_idx: FuncIdx, arg_idx: ArgIdx },
}

impl Operand {
    /// For a [Self::LocalVariable] operand return the instruction that defines the variable.
    ///
    /// Panics for other kinds of operand.
    ///
    /// OPT: This is expensive.
    pub(crate) fn to_instr<'a>(&self, aotmod: &'a Module) -> &'a Instruction {
        let Self::LocalVariable(iid) = self else {
            panic!()
        };
        &aotmod.funcs[iid.func_idx].bblocks[iid.bb_idx].instrs[iid.inst_idx]
    }

    /// Returns the [Type] of the operand.
    pub(crate) fn type_<'a>(&self, m: &'a Module) -> &'a Type {
        match self {
            Self::LocalVariable(_) => {
                // The `unwrap` can't fail for a `LocalVariable`.
                self.to_instr(m).def_type(m).unwrap()
            }
            Self::Constant(cidx) => m.type_(m.const_(*cidx).type_idx()),
            Self::Arg { func_idx, arg_idx } => {
                let Type::Func(ft) = m.type_(m.func(*func_idx).type_idx) else {
                    panic!()
                };
                m.type_(ft.arg_ty_idxs()[usize::from(*arg_idx)])
            }
            _ => todo!(),
        }
    }

    /// Return the `InstructionID` of a local variable operand. Panics if called on other kinds of
    /// operands.
    pub(crate) fn to_instr_id(&self) -> InstructionID {
        let Self::LocalVariable(iid) = self else {
            panic!()
        };
        iid.clone()
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
            Operand::Constant(const_idx) => {
                write!(f, "{}", self.m.consts[*const_idx].display(self.m))
            }
            Operand::LocalVariable(iid) => {
                write!(
                    f,
                    "${}_{}",
                    usize::from(iid.bb_idx),
                    usize::from(iid.inst_idx)
                )
            }
            Operand::Global(gidx) => write!(f, "@{}", self.m.global_decls[*gidx].name()),
            Operand::Func(fidx) => write!(f, "{}", self.m.funcs[*fidx].name()),
            Operand::Arg { arg_idx, .. } => write!(f, "$arg{}", usize::from(*arg_idx)),
        }
    }
}

#[deku_derive(DekuRead)]
#[derive(Debug)]
pub(crate) struct DeoptSafepoint {
    pub(crate) id: Operand,
    #[deku(temp)]
    num_lives: u32,
    #[deku(count = "num_lives")]
    pub(crate) lives: Vec<Operand>,
}

impl DeoptSafepoint {
    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableDeoptSafepoint<'a> {
        DisplayableDeoptSafepoint { safepoint: self, m }
    }
}

pub(crate) struct DisplayableDeoptSafepoint<'a> {
    safepoint: &'a DeoptSafepoint,
    m: &'a Module,
}

impl fmt::Display for DisplayableDeoptSafepoint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lives_s = self
            .safepoint
            .lives
            .iter()
            .map(|a| a.display(self.m).to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "[safepoint: {}, ({})]",
            self.safepoint.id.display(self.m),
            lives_s
        )
    }
}

/// An instruction.
///
/// An instruction is conceptually an [Opcode] and a list of [Operand]s. The semantics of the
/// instruction, and the meaning of the operands, are determined by the opcode.
///
/// Instructions that compute a value define a new local variable in the parent [Func]. In such a
/// case the newly defined variable can be referenced in the operands of later instructions by the
/// [InstructionID] of the [Instruction] that defined the variable.
///
/// In other words, an instruction and the variable it defines are both identified by the same
/// [InstructionID].
///
/// The type of the variable defined by an instruction (if any) can be determined by
/// [Instruction::def_type()].
#[deku_derive(DekuRead)]
#[derive(Debug, strum_macros::Display)]
#[repr(u8)]
#[deku(type = "u8")]
pub(crate) enum Instruction {
    #[deku(id = "0")]
    Nop,
    #[deku(id = "1")]
    Load { ptr: Operand, type_idx: TypeIdx },
    #[deku(id = "2")]
    Store { val: Operand, ptr: Operand },
    #[deku(id = "3")]
    Alloca { type_idx: TypeIdx, count: usize },
    #[deku(id = "4")]
    Call {
        callee: FuncIdx,
        #[deku(temp)]
        num_args: u32,
        #[deku(count = "num_args")]
        args: Vec<Operand>,
        #[deku(temp)]
        has_safepoint: u8,
        #[deku(cond = "*has_safepoint != 0", default = "None")]
        safepoint: Option<DeoptSafepoint>,
    },
    #[deku(id = "5")]
    Br {
        /// The block this branch points to.
        succ: BBlockIdx,
    },
    #[deku(id = "6")]
    CondBr {
        cond: Operand,
        true_bb: BBlockIdx,
        false_bb: BBlockIdx,
        safepoint: DeoptSafepoint,
    },
    #[deku(id = "7")]
    ICmp {
        type_idx: TypeIdx,
        lhs: Operand,
        pred: Predicate,
        rhs: Operand,
    },
    #[deku(id = "8")]
    Ret {
        #[deku(temp)]
        has_val: u8,
        #[deku(cond = "*has_val != 0", default = "None")]
        val: Option<Operand>,
    },
    #[deku(id = "9")]
    InsertValue { agg: Operand, elem: Operand },
    /// This opcode adds to the `ptr` operand:
    ///  - a constant offset
    ///  - zero or more dynamic offsets.
    ///
    /// where each dynamic offset is:
    ///  - A potentially dynamic element count.
    ///  - A constant element size.
    ///
    /// A dynamic offset is computed at runtime by multiplying the element count by the element
    /// size.
    #[deku(id = "10")]
    PtrAdd {
        // The type index of a pointer.
        //
        // FIXME: the type will always be `ptr`, so this field could be elided if we provide a way
        // for us to find the pointer type index quickly.
        type_idx: TypeIdx,
        /// The pointer to offset from.
        ptr: Operand,
        /// The constant offset (in bytes).
        ///
        /// This is signed to allow for negative array indices and negative pointer arithmetic.
        const_off: isize,
        /// The number of dynamic offsets.
        #[deku(temp)]
        num_dyn_offs: usize,
        /// The element counts for the dynamic offsets.
        ///
        /// These are interpreted as signed values to allow negative indexing and negative pointer
        /// arithmetic.
        #[deku(count = "num_dyn_offs")]
        dyn_elem_counts: Vec<Operand>,
        /// The element sizes for the dynamic offsets (in bytes).
        ///
        /// These are unsigned values.
        #[deku(count = "num_dyn_offs")]
        dyn_elem_sizes: Vec<usize>,
    },
    #[deku(id = "11")]
    BinaryOp {
        lhs: Operand,
        binop: BinOp,
        rhs: Operand,
    },
    /// An opcode that is designed to cover cast-like operations. E.g. bitcasts, sign extends, zero
    /// extends etc.
    #[deku(id = "12")]
    Cast {
        /// The cast-like operation to perform.
        cast_kind: CastKind,
        /// The value to be operated upon.
        val: Operand,
        /// The resulting type of the operation.
        dest_type_idx: TypeIdx,
    },
    #[deku(id = "13")]
    Switch {
        test_val: Operand,
        default_dest: BBlockIdx,
        #[deku(temp)]
        num_cases: usize,
        #[deku(count = "num_cases")]
        case_values: Vec<u64>,
        #[deku(count = "num_cases")]
        case_dests: Vec<BBlockIdx>,
        safepoint: DeoptSafepoint,
    },
    #[deku(id = "14")]
    Phi {
        #[deku(temp)]
        num_incoming: usize,
        #[deku(count = "num_incoming")]
        incoming_bbs: Vec<BBlockIdx>,
        #[deku(count = "num_incoming")]
        incoming_vals: Vec<Operand>,
    },
    #[deku(id = "15")]
    IndirectCall {
        fty_idx: TypeIdx,
        callop: Operand,
        #[deku(temp)]
        num_args: u32,
        #[deku(count = "num_args")]
        args: Vec<Operand>,
    },
    #[deku(id = "255")]
    Unimplemented(#[deku(until = "|v: &u8| *v == 0", map = "map_to_string")] String),
}

impl Instruction {
    /// Find the name of a local variable.
    ///
    /// This is used when stringifying the instruction.
    ///
    /// FIXME: This is very slow and could be optimised.
    fn local_name(&self, m: &Module) -> String {
        for f in m.funcs.iter() {
            for (bb_idx, bb) in f.bblocks.iter().enumerate() {
                for (inst_idx, instr) in bb.instrs.iter().enumerate() {
                    if std::ptr::addr_eq(instr, self) {
                        return format!("${}_{}", bb_idx, inst_idx);
                    }
                }
            }
        }
        panic!(); // malformed IR.
    }

    /// Returns the [Type] of the local variable defined by this instruction or `None` if this
    /// instruction does not define a new local variable.
    pub(crate) fn def_type<'a>(&self, m: &'a Module) -> Option<&'a Type> {
        match self {
            Self::Alloca { .. } => Some(&Type::Ptr),
            Self::BinaryOp { lhs, .. } => Some(lhs.type_(m)),
            Self::Br { .. } => None,
            Self::Call { callee, .. } => {
                // The type of the newly-defined local is the return type of the callee.
                if let Type::Func(ft) = m.type_(m.func(*callee).type_idx) {
                    let ty = m.type_(ft.ret_ty);
                    if ty != &Type::Void {
                        Some(ty)
                    } else {
                        None
                    }
                } else {
                    panic!(); // IR malformed.
                }
            }
            Self::CondBr { .. } => None,
            Self::InsertValue { agg, .. } => Some(agg.type_(m)),
            Self::ICmp { type_idx, .. } => Some(m.type_(*type_idx)),
            Self::Load { type_idx, .. } => Some(m.type_(*type_idx)),
            Self::PtrAdd { type_idx, .. } => Some(m.type_(*type_idx)),
            Self::Ret { .. } => {
                // Subtle: although `Ret` might make a value, that's not a local value in the
                // parent function.
                None
            }
            Self::Store { .. } => None,
            Self::Cast { dest_type_idx, .. } => Some(m.type_(*dest_type_idx)),
            Self::Switch { .. } => None,
            Self::Phi { incoming_vals, .. } => {
                // Indexing cannot crash: correct PHI nodes have at least one incoming value.
                Some(incoming_vals[0].type_(m))
            }
            Self::IndirectCall { fty_idx, .. } => {
                // The type of the newly-defined local is the return type of the callee.
                if let Type::Func(ft) = m.type_(*fty_idx) {
                    let ty = m.type_(ft.ret_ty);
                    if ty != &Type::Void {
                        Some(ty)
                    } else {
                        None
                    }
                } else {
                    panic!(); // IR malformed.
                }
            }
            Self::Unimplemented(_) => None,
            Self::Nop => None,
        }
    }

    pub(crate) fn is_mappable_call(&self, aot_mod: &Module) -> bool {
        match self {
            Self::Call { callee, .. } => !aot_mod.func(*callee).is_declaration(),
            _ => false,
        }
    }

    /// If `self` is a call to the control point, then return the live variables struct argument
    /// being passed to it. Otherwise return None.
    pub(crate) fn control_point_call_trace_inputs(&self, aot_mod: &Module) -> Option<&Operand> {
        match self {
            Self::Call { callee, args, .. } => {
                if aot_mod.func(*callee).name == CONTROL_POINT_NAME {
                    let arg = &args[CTRL_POINT_ARGIDX_INPUTS];
                    // It should be a pointer (to a struct, but we can't check that).
                    debug_assert!(matches!(arg.type_(aot_mod), &Type::Ptr));
                    Some(arg)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn safepoint(&self) -> Option<&DeoptSafepoint> {
        match self {
            Self::Call { safepoint, .. } => safepoint.as_ref(),
            Self::CondBr { ref safepoint, .. } => Some(safepoint),
            _ => None,
        }
    }

    pub(crate) fn is_debug_call(&self, aot_mod: &Module) -> bool {
        match self {
            Self::Call { callee, .. } => aot_mod.func(*callee).name == LLVM_DEBUG_CALL_NAME,
            _ => false,
        }
    }

    /// Determine if two instructions in the (immutable) AOT IR are the same based on pointer
    /// identity.
    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableInstruction<'a> {
        DisplayableInstruction {
            instruction: self,
            m,
        }
    }
}

pub(crate) struct DisplayableInstruction<'a> {
    instruction: &'a Instruction,
    m: &'a Module,
}

impl fmt::Display for DisplayableInstruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(t) = self.instruction.def_type(self.m) {
            // If the instruction defines a local, we will format the instruction like it's an
            // assignment. Here we print the left-hand side.
            write!(
                f,
                "{}: {} = ",
                self.instruction.local_name(self.m),
                t.display(self.m)
            )?;
        }

        match self.instruction {
            Instruction::Alloca { type_idx, count } => write!(
                f,
                "alloca {}, {}",
                self.m.type_(*type_idx).display(self.m),
                count
            ),
            Instruction::BinaryOp { lhs, binop, rhs } => {
                write!(
                    f,
                    "{}, {binop}, {}",
                    lhs.display(self.m),
                    rhs.display(self.m)
                )
            }
            Instruction::Br { succ } => write!(f, "br bb{}", usize::from(*succ)),
            Instruction::Call {
                callee,
                args,
                safepoint,
            } => {
                let args_s = args
                    .iter()
                    .map(|a| a.display(self.m).to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let safepoint_s = safepoint
                    .as_ref()
                    .map_or("".to_string(), |sp| format!(" {}", sp.display(self.m)));
                write!(
                    f,
                    "call {}({}){}",
                    self.m.func(*callee).name(),
                    args_s,
                    safepoint_s
                )
            }
            Instruction::CondBr {
                cond,
                true_bb,
                false_bb,
                safepoint,
            } => write!(
                f,
                "condbr {}, bb{}, bb{} {}",
                cond.display(self.m),
                usize::from(*true_bb),
                usize::from(*false_bb),
                safepoint.display(self.m)
            ),
            Instruction::ICmp { lhs, pred, rhs, .. } => write!(
                f,
                "icmp {}, {pred}, {}",
                lhs.display(self.m),
                rhs.display(self.m)
            ),
            Instruction::Load { ptr, .. } => write!(f, "load {}", ptr.display(self.m)),
            Instruction::PtrAdd {
                ptr,
                const_off,
                dyn_elem_counts,
                dyn_elem_sizes,
                ..
            } => {
                if dyn_elem_counts.is_empty() {
                    write!(f, "PtrAdd {}, {}", ptr.display(self.m), const_off)
                } else {
                    let dyns = dyn_elem_counts
                        .iter()
                        .zip(dyn_elem_sizes)
                        .map(|(c, s)| format!("({} * {})", c.display(self.m), s))
                        .collect::<Vec<_>>();
                    write!(
                        f,
                        "PtrAdd {}, {} + {}",
                        ptr.display(self.m),
                        const_off,
                        dyns.join(" + ")
                    )
                }
            }
            Instruction::Ret { val } => match val {
                None => write!(f, "ret"),
                Some(v) => write!(f, "ret {}", v.display(self.m)),
            },
            Instruction::Store { ptr, val } => {
                write!(f, "store {}, {}", val.display(self.m), ptr.display(self.m))
            }
            Instruction::InsertValue { agg, elem } => write!(
                f,
                "insertvalue {}, {}",
                agg.display(self.m),
                elem.display(self.m)
            ),
            Instruction::Cast {
                cast_kind,
                val,
                dest_type_idx,
            } => write!(
                f,
                "{cast_kind} {}, {}",
                val.display(self.m),
                self.m.types[*dest_type_idx].display(self.m)
            ),
            Instruction::Switch {
                test_val,
                default_dest,
                case_values,
                case_dests,
                safepoint,
            } => {
                let cases = case_values
                    .iter()
                    .zip(case_dests)
                    .map(|(val, dest)| format!("{} -> bb{}", val, usize::from(*dest)))
                    .collect::<Vec<_>>();
                write!(
                    f,
                    "switch {}, bb{}, [{}] {}",
                    test_val.display(self.m),
                    usize::from(*default_dest),
                    cases.join(", "),
                    safepoint.display(self.m)
                )
            }
            Instruction::Phi {
                incoming_vals,
                incoming_bbs,
            } => {
                let args = incoming_bbs
                    .iter()
                    .zip(incoming_vals)
                    .map(|(bb, val)| format!("bb{} -> {}", usize::from(*bb), val.display(self.m)))
                    .collect::<Vec<_>>();
                write!(f, "phi {}", args.join(", "))
            }
            Instruction::IndirectCall {
                fty_idx: _,
                callop,
                args,
            } => {
                let args_s = args
                    .iter()
                    .map(|a| a.display(self.m).to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "call {}({})", callop.display(self.m), args_s)
            }
            Instruction::Unimplemented(s) => write!(f, "unimplemented <<{}>>", s),
            Instruction::Nop => write!(f, "nop"),
        }
    }
}

/// A basic block containing IR [Instruction]s.
#[deku_derive(DekuRead)]
#[derive(Debug)]
pub(crate) struct BBlock {
    #[deku(temp)]
    num_instrs: usize,
    #[deku(count = "num_instrs", map = "map_to_tivec")]
    pub(crate) instrs: TiVec<InstrIdx, Instruction>,
}

impl BBlock {
    // Returns true if this block is terminated by a return, false otherwise.
    pub fn is_return(&self) -> bool {
        matches!(self.instrs.last().unwrap(), Instruction::Ret { .. })
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableBBlock<'a> {
        DisplayableBBlock { bblock: self, m }
    }
}

pub(crate) struct DisplayableBBlock<'a> {
    bblock: &'a BBlock,
    m: &'a Module,
}

impl fmt::Display for DisplayableBBlock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in &self.bblock.instrs {
            writeln!(f, "    {}", x.display(self.m))?;
        }
        Ok(())
    }
}

/// A function definition or declaration.
///
/// If the function was compiled by ykllvm as part of the interpreter binary, then we have IR for
/// the function body, and the function is said to be a *function definition*.
///
/// Conversely, if the function was *not* compiled by ykllvm as part of the interpreter binary (as
/// is the case for shared library functions), then we have no IR for the function body, and the
/// function is said to be a *function declaration*.
///
/// [Func::is_declaration()] can be used to determine if the [Func] is a definition or a
/// declaration.
#[deku_derive(DekuRead)]
#[derive(Debug)]
pub(crate) struct Func {
    #[deku(until = "|v: &u8| *v == 0", map = "map_to_string")]
    name: String,
    type_idx: TypeIdx,
    outline: bool,
    #[deku(temp)]
    num_bblocks: usize,
    #[deku(count = "num_bblocks", map = "map_to_tivec")]
    bblocks: TiVec<BBlockIdx, BBlock>,
}

impl Func {
    fn is_declaration(&self) -> bool {
        self.bblocks.is_empty()
    }

    pub(crate) fn is_outline(&self) -> bool {
        self.outline
    }

    /// Return the [BBlock] at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of range.
    pub(crate) fn bblock(&self, bb_idx: BBlockIdx) -> &BBlock {
        &self.bblocks[bb_idx]
    }

    /// Return the name of the function.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Return the type index of the function.
    pub(crate) fn type_idx(&self) -> TypeIdx {
        self.type_idx
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableFunc<'a> {
        DisplayableFunc { func_: self, m }
    }
}

pub(crate) struct DisplayableFunc<'a> {
    func_: &'a Func,
    m: &'a Module,
}

impl fmt::Display for DisplayableFunc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = &self.m.types[self.func_.type_idx];
        if let Type::Func(fty) = ty {
            write!(
                f,
                "func {}({}",
                self.func_.name,
                fty.arg_ty_idxs
                    .iter()
                    .enumerate()
                    .map(|(i, t)| format!("$arg{}: {}", i, self.m.types[*t].display(self.m)))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
            if fty.is_vararg {
                write!(f, ", ...")?;
            }
            write!(f, ")")?;
            let ret_ty = &self.m.types[fty.ret_ty];
            if ret_ty != &Type::Void {
                write!(f, " -> {}", ret_ty.display(self.m))?;
            }
            if self.func_.is_declaration() {
                // declarations have no body, so print it as such.
                writeln!(f, ";")
            } else {
                writeln!(f, " {{")?;
                for (i, b) in self.func_.bblocks.iter().enumerate() {
                    write!(f, "  bb{}:\n{}", i, b.display(self.m))?;
                }
                writeln!(f, "}}")
            }
        } else {
            unreachable!();
        }
    }
}

/// Return the stringified constant integer obtained by interpreting `bytes` as `num-bits`-wide
/// constant integer.
///
/// FIXME: For now we just handle common integer types, but eventually we will need to
/// implement printing of aribitrarily-sized (in bits) integers. Consider using a bigint
/// library so we don't have to do it ourself?
///
/// This discussion may help:
/// https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/.E2.9C.94.20Big.20Integer.20library.20with.20bit.20granularity/near/393733327
pub(crate) fn const_int_bytes_to_string(num_bits: u32, bytes: &[u8]) -> String {
    // All of the unwraps below are safe due to:
    debug_assert!(bytes.len() * 8 >= usize::try_from(num_bits).unwrap());

    let mut bytes = bytes;
    match num_bits {
        1 => format!("{}i1", bytes.read_i8().unwrap() & 1),
        8 => format!("{}i8", bytes.read_i8().unwrap()),
        16 => format!("{}i16", bytes.read_i16::<NativeEndian>().unwrap()),
        32 => format!("{}i32", bytes.read_i32::<NativeEndian>().unwrap()),
        64 => format!("{}i64", bytes.read_i64::<NativeEndian>().unwrap()),
        _ => todo!("{}", num_bits),
    }
}

/// A fixed-width integer type.
///
/// Note:
///   1. These integers range in size from 1..2^23 (inc.) bits. This is inherited [from LLVM's
///      integer type](https://llvm.org/docs/LangRef.html#integer-type).
///   2. Signedness is not specified. Interpretation of the bit pattern is delegated to operations
///      upon the integer.
#[deku_derive(DekuRead)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IntegerType {
    num_bits: u32,
}

impl IntegerType {
    /// Create a new integer type with the specified number of bits.
    #[cfg(test)]
    pub(crate) fn new(num_bits: u32) -> Self {
        debug_assert!(num_bits > 0 && num_bits <= 0x800000);
        Self { num_bits }
    }

    /// Return the number of bits (1..2^23 (inc.)) this integer spans.
    pub(crate) fn num_bits(&self) -> u32 {
        debug_assert!(self.num_bits > 0 && self.num_bits <= 0x800000);
        self.num_bits
    }

    /// Return the number of bytes required to store this integer type.
    ///
    /// Padding for alignment is not included.
    #[cfg(test)]
    pub(crate) fn byte_size(&self) -> usize {
        let bits = self.num_bits();
        let mut ret = bits / 8;
        // If it wasn't an exactly byte-sized thing, round up to the next byte.
        if bits % 8 != 0 {
            ret += 1;
        }
        usize::try_from(ret).unwrap()
    }

    /// Format a constant integer value that is of the type described by `self`.
    fn const_to_string(&self, c: &Constant) -> String {
        const_int_bytes_to_string(self.num_bits, c.bytes())
    }
}

impl Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i{}", self.num_bits)
    }
}

#[deku_derive(DekuRead)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FuncType {
    /// The number of formal arguments the function takes.
    #[deku(temp)]
    num_args: usize,
    /// Type indices for the function's formal arguments.
    #[deku(count = "num_args")]
    arg_ty_idxs: Vec<TypeIdx>,
    /// Type index of the function's return type.
    ret_ty: TypeIdx,
    /// Is the function vararg?
    is_vararg: bool,
}

impl FuncType {
    #[cfg(test)]
    fn new(arg_ty_idxs: Vec<TypeIdx>, ret_ty_idx: TypeIdx, is_vararg: bool) -> Self {
        Self {
            arg_ty_idxs,
            ret_ty: ret_ty_idx,
            is_vararg,
        }
    }

    pub(crate) fn arg_ty_idxs(&self) -> &[TypeIdx] {
        &self.arg_ty_idxs
    }

    pub(crate) fn ret_ty(&self) -> TypeIdx {
        self.ret_ty
    }

    pub(crate) fn is_vararg(&self) -> bool {
        self.is_vararg
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableFuncType<'a> {
        DisplayableFuncType { func_type: self, m }
    }
}

pub(crate) struct DisplayableFuncType<'a> {
    func_type: &'a FuncType,
    m: &'a Module,
}

impl fmt::Display for DisplayableFuncType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = self
            .func_type
            .arg_ty_idxs
            .iter()
            .map(|t| self.m.types[*t].display(self.m).to_string())
            .collect::<Vec<_>>();
        if self.func_type.is_vararg() {
            args.push("...".to_owned());
        }
        write!(f, "func({})", args.join(", "))?;
        let rty = self.m.type_(self.func_type.ret_ty);
        if rty != &Type::Void {
            write!(f, " -> {}", rty.display(self.m))?
        }
        Ok(())
    }
}

#[deku_derive(DekuRead)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StructType {
    /// The number of fields the struct has.
    #[deku(temp)]
    num_fields: usize,
    /// The types of the fields.
    #[deku(count = "num_fields")]
    field_ty_idxs: Vec<TypeIdx>,
    /// The bit offsets of the fields (taking into account any required padding for alignment).
    #[deku(count = "num_fields")]
    field_bit_offs: Vec<usize>,
}

impl StructType {
    /// Returns the type index of the specified field index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn field_type_idx(&self, idx: usize) -> TypeIdx {
        self.field_ty_idxs[idx]
    }

    /// Returns the byte offset of the specified field index.
    ///
    /// # Panics
    ///
    /// Panics if the field is not byte-aligned or the index is out of bounds.
    pub(crate) fn field_byte_off(&self, idx: usize) -> usize {
        let bit_off = self.field_bit_offs[idx];
        if bit_off % 8 != 0 {
            todo!();
        }
        bit_off / 8
    }

    /// Returns the number of fields in the struct.
    pub(crate) fn num_fields(&self) -> usize {
        self.field_ty_idxs.len()
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableStructType<'a> {
        DisplayableStructType {
            struct_type: self,
            m,
        }
    }
}

pub(crate) struct DisplayableStructType<'a> {
    struct_type: &'a StructType,
    m: &'a Module,
}

impl Display for DisplayableStructType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.struct_type
                .field_ty_idxs
                .iter()
                .enumerate()
                .map(|(i, ti)| format!(
                    "{}: {}",
                    self.struct_type.field_bit_offs[i],
                    self.m.types[*ti].display(self.m)
                ))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

const TYKIND_VOID: u8 = 0;
const TYKIND_INTEGER: u8 = 1;
const TYKIND_PTR: u8 = 2;
const TYKIND_FUNC: u8 = 3;
const TYKIND_STRUCT: u8 = 4;
const TYKIND_UNIMPLEMENTED: u8 = 255;

/// A type.
#[deku_derive(DekuRead)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[deku(type = "u8")]
pub(crate) enum Type {
    #[deku(id = "TYKIND_VOID")]
    Void,
    #[deku(id = "TYKIND_INTEGER")]
    Integer(IntegerType),
    #[deku(id = "TYKIND_PTR")]
    Ptr,
    #[deku(id = "TYKIND_FUNC")]
    Func(FuncType),
    #[deku(id = "TYKIND_STRUCT")]
    Struct(StructType),
    #[deku(id = "TYKIND_UNIMPLEMENTED")]
    Unimplemented(#[deku(until = "|v: &u8| *v == 0", map = "map_to_string")] String),
}

impl Type {
    fn const_to_string(&self, c: &Constant) -> String {
        match self {
            Self::Void => "void".to_owned(),
            Self::Integer(it) => it.const_to_string(c),
            Self::Ptr => {
                // FIXME: write a stringifier for constant pointers.
                "const_ptr".to_owned()
            }
            Self::Func(_) => unreachable!(), // No such thing as a constant function in our IR.
            Self::Struct(_) => {
                // FIXME: write a stringifier for constant structs.
                "const_struct".to_owned()
            }
            Self::Unimplemented(s) => format!("?cst<{}>", s),
        }
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableType<'a> {
        DisplayableType { type_: self, m }
    }
}

#[derive(Debug)]
pub(crate) struct DisplayableType<'a> {
    type_: &'a Type,
    m: &'a Module,
}

impl fmt::Display for DisplayableType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_ {
            Type::Void => write!(f, "void"),
            Type::Integer(x) => write!(f, "{}", x),
            Type::Ptr => write!(f, "ptr"),
            Type::Func(ft) => write!(f, "{}", ft.display(self.m)),
            Type::Struct(st) => write!(f, "{}", st.display(self.m)),
            Type::Unimplemented(s) => write!(f, "?ty<{}>", s),
        }
    }
}

/// A constant.
#[deku_derive(DekuRead)]
#[derive(Debug)]
pub(crate) struct Constant {
    type_idx: TypeIdx,
    #[deku(temp)]
    num_bytes: usize,
    #[deku(count = "num_bytes")]
    bytes: Vec<u8>,
}

impl Constant {
    /// Return a byte slice of the constant's value.
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the type index of the constant.
    pub(crate) fn type_idx(&self) -> TypeIdx {
        self.type_idx
    }

    pub(crate) fn display<'a>(&'a self, m: &'a Module) -> DisplayableConstant<'a> {
        DisplayableConstant { constant: self, m }
    }
}

pub(crate) struct DisplayableConstant<'a> {
    constant: &'a Constant,
    m: &'a Module,
}

impl Display for DisplayableConstant<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.m.types[self.constant.type_idx].const_to_string(self.constant)
        )
    }
}

/// A global variable declaration, identified by its symbol name.
///
/// Since the AOT IR doesn't capture the initialisers of global variables (externally compiled or
/// otherwise), all global variables are considered *declarations*.
#[deku_derive(DekuRead)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GlobalDecl {
    is_threadlocal: bool,
    #[deku(until = "|v: &u8| *v == 0", map = "map_to_string")]
    name: String,
}

impl Display for GlobalDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalDecl({}, tls={})", self.name, self.is_threadlocal)
    }
}

impl GlobalDecl {
    pub(crate) fn is_threadlocal(&self) -> bool {
        self.is_threadlocal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

/// An AOT IR module.
///
/// This is the top-level container for the AOT IR.
///
/// A module is platform dependent, as type sizes and alignment are baked-in.
#[deku_derive(DekuRead)]
#[derive(Debug, Default)]
pub(crate) struct Module {
    #[deku(assert = "*magic == MAGIC", temp)]
    magic: u32,
    #[deku(assert = "*version == FORMAT_VERSION")]
    version: u32,
    /// The bit-size of what LLVM calls "the pointer indexing type", for address space zero.
    ///
    /// This is the signed integer LLVM uses for computing GEP offsets in the default pointer
    /// address space. This is needed because in certain cases we are required to sign-extend or
    /// truncate to this width.
    ptr_off_bitsize: u8,
    #[deku(temp)]
    num_funcs: usize,
    #[deku(count = "num_funcs", map = "map_to_tivec")]
    funcs: TiVec<FuncIdx, Func>,
    #[deku(temp)]
    num_consts: usize,
    #[deku(count = "num_consts", map = "map_to_tivec")]
    consts: TiVec<ConstIdx, Constant>,
    #[deku(temp)]
    num_global_decls: usize,
    #[deku(count = "num_global_decls", map = "map_to_tivec")]
    global_decls: TiVec<GlobalDeclIdx, GlobalDecl>,
    #[deku(temp)]
    num_types: usize,
    #[deku(count = "num_types", map = "map_to_tivec")]
    types: TiVec<TypeIdx, Type>,
}

impl Module {
    /// Find a function by its name.
    ///
    /// # Panics
    ///
    /// Panics if no function exists with that name.
    pub(crate) fn func_idx(&self, find_func: &str) -> FuncIdx {
        // OPT: create a cache in the Module.
        self.funcs
            .iter()
            .enumerate()
            .find(|(_, f)| f.name == find_func)
            .map(|(f_idx, _)| FuncIdx(f_idx))
            .unwrap()
    }

    pub(crate) fn ptr_off_bitsize(&self) -> u8 {
        self.ptr_off_bitsize
    }

    /// Return the block uniquely identified (in this module) by the specified [BBlockId].
    pub(crate) fn bblock(&self, bid: &BBlockId) -> &BBlock {
        self.funcs[bid.func_idx].bblock(bid.bb_idx)
    }

    pub(crate) fn constant(&self, co: &ConstIdx) -> &Constant {
        &self.consts[*co]
    }

    pub(crate) fn const_type(&self, c: &Constant) -> &Type {
        &self.types[c.type_idx]
    }

    /// Lookup a constant by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn const_(&self, ci: ConstIdx) -> &Constant {
        &self.consts[ci]
    }

    /// Lookup a type by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn type_(&self, idx: TypeIdx) -> &Type {
        &self.types[idx]
    }

    /// Lookup a function by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn func(&self, idx: FuncIdx) -> &Func {
        &self.funcs[idx]
    }

    /// Lookup a global variable declaration by its index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub(crate) fn global_decl(&self, idx: GlobalDeclIdx) -> &GlobalDecl {
        &self.global_decls[idx]
    }

    /// Return the number of global variable declarations.
    pub(crate) fn global_decls_len(&self) -> usize {
        self.global_decls.len()
    }

    #[allow(dead_code)]
    pub(crate) fn dump(&self) {
        eprintln!("{}", self);
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("# IR format version: {}\n", self.version))?;
        f.write_fmt(format_args!("# Num funcs: {}\n", self.funcs.len()))?;
        f.write_fmt(format_args!("# Num consts: {}\n", self.consts.len()))?;
        f.write_fmt(format_args!(
            "# Num global decls: {}\n",
            self.global_decls.len()
        ))?;
        f.write_fmt(format_args!("# Num types: {}\n", self.types.len()))?;

        for func in &self.funcs {
            write!(f, "\n{}", func.display(self))?;
        }
        Ok(())
    }
}

/// Deserialise an AOT module from the slice `data`.
pub(crate) fn deserialise_module(data: &[u8]) -> Result<Module, Box<dyn Error>> {
    let ((_, _), modu) = Module::from_bytes((data, 0))?;
    Ok(modu)
}

/// Deserialise and print IR from an on-disk file.
///
/// Used for support tooling (in turn used by tests too).
pub fn print_from_file(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let data = fs::read(path)?;
    let ir = deserialise_module(&data)?;
    println!("{}", ir);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::{PrimInt, ToBytes};
    use std::mem;

    #[test]
    fn string_deser() {
        let check = |s: &str| {
            assert_eq!(
                &map_to_string(CString::new(s).unwrap().into_bytes_with_nul()).unwrap(),
                s
            );
        };
        check("testing");
        check("the quick brown fox jumped over the lazy dog");
        check("");
        check("         ");
    }

    #[test]
    fn const_int_strings() {
        // Check (in an endian neutral manner) that a `num-bits`-sized integer of value `num`, when
        // converted to a constant IR integer, then stringified, results in the string `expect`.
        //
        // When `num` has a bit size greater than `num_bits` the most significant bits of `num` are
        // treated as undefined: they can be any value as IR stringification will ignore them.
        fn check<T: ToBytes + PrimInt>(num_bits: u32, num: T, expect: &str) {
            assert!(mem::size_of::<T>() * 8 >= usize::try_from(num_bits).unwrap());

            // Get a byte-vector for `num`.
            let bytes = ToBytes::to_ne_bytes(&num).as_ref().to_vec();

            // Construct an IR constant and check it stringifies ok.
            let it = IntegerType { num_bits };
            let c = Constant {
                type_idx: TypeIdx::new(0),
                bytes,
            };
            assert_eq!(it.const_to_string(&c), expect);
        }

        check(1, 1u8, "1i1");
        check(1, 0u8, "0i1");
        check(1, 254u8, "0i1");
        check(1, 255u8, "1i1");
        check(1, 254u64, "0i1");
        check(1, 255u64, "1i1");

        check(16, 0u16, "0i16");
        check(16, u16::MAX, "-1i16");
        check(16, 12345u16, "12345i16");
        check(16, 12345u64, "12345i16");
        check(16, i16::MIN as u16, &format!("{}i16", i16::MIN));
        check(16, i16::MIN as u64, &format!("{}i16", i16::MIN));

        check(32, 0u32, "0i32");
        check(32, u32::MAX, "-1i32");
        check(32, 12345u32, "12345i32");
        check(32, 12345u64, "12345i32");
        check(32, i32::MIN as u32, &format!("{}i32", i32::MIN));
        check(32, i32::MIN as u64, &format!("{}i32", i32::MIN));

        check(64, 0u64, "0i64");
        check(64, u64::MAX, "-1i64");
        check(64, 12345678u64, "12345678i64");
        check(64, i64::MIN as u64, &format!("{}i64", i64::MIN));
    }

    #[test]
    fn integer_type_sizes() {
        for i in 1..8 {
            assert_eq!(IntegerType::new(i).byte_size(), 1);
        }
        for i in 9..16 {
            assert_eq!(IntegerType::new(i).byte_size(), 2);
        }
        assert_eq!(IntegerType::new(127).byte_size(), 16);
        assert_eq!(IntegerType::new(128).byte_size(), 16);
        assert_eq!(IntegerType::new(129).byte_size(), 17);
    }

    #[test]
    fn stringify_func_types() {
        let mut m = Module::default();

        let i8_tyidx = TypeIdx::new(m.types.len());
        m.types.push(Type::Integer(IntegerType { num_bits: 8 }));
        let void_tyidx = TypeIdx::new(m.types.len());
        m.types.push(Type::Void);

        let fty = Type::Func(FuncType::new(vec![i8_tyidx], i8_tyidx, false));
        assert_eq!(fty.display(&m).to_string(), "func(i8) -> i8");

        let fty = Type::Func(FuncType::new(vec![i8_tyidx], i8_tyidx, true));
        assert_eq!(fty.display(&m).to_string(), "func(i8, ...) -> i8");

        let fty = Type::Func(FuncType::new(vec![], i8_tyidx, false));
        assert_eq!(fty.display(&m).to_string(), "func() -> i8");

        let fty = Type::Func(FuncType::new(vec![], void_tyidx, false));
        assert_eq!(fty.display(&m).to_string(), "func()");
    }
}
