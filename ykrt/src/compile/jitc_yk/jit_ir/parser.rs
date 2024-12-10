//! A basic parser for JIT IR.
//!
//! This module -- which is only intended to be compiled-in in `cfg(test)` -- adds a `from_str`
//! method to [Module] which takes in JIT IR as a string, parses it, and produces a [Module]. This
//! makes it possible to write JIT IR tests using JIT IR concrete syntax.

use crate::compile::jitc_yk::aot_ir;

use super::super::{
    aot_ir::{BinOp, FloatPredicate, InstID, Predicate},
    jit_ir::{
        BinOpInst, BitCastInst, BlackBoxInst, Const, DirectCallInst, DynPtrAddInst, FCmpInst,
        FNegInst, FPExtInst, FPToSIInst, FloatTy, FuncDecl, FuncTy, GuardInfo, GuardInst, ICmpInst,
        IndirectCallInst, Inst, InstIdx, LoadInst, Module, Operand, PackedOperand, ParamIdx,
        ParamInst, PtrAddInst, SExtInst, SIToFPInst, SelectInst, StoreInst, TruncInst, Ty, TyIdx,
        ZExtInst,
    },
};
use fm::FMBuilder;
use lrlex::{lrlex_mod, DefaultLexerTypes, LRNonStreamingLexer};
use lrpar::{lrpar_mod, NonStreamingLexer, Span};
use regex::Regex;
use std::{collections::HashMap, convert::TryFrom, error::Error, sync::OnceLock};
use yksmp;

lrlex_mod!("compile/jitc_yk/jit_ir/jit_ir.l");
lrpar_mod!("compile/jitc_yk/jit_ir/jit_ir.y");

type StorageT = u8;

impl Module {
    /// Parse the string `s` into a [Module].
    ///
    /// # Panics
    ///
    /// If `s` is not parsable or otherwise does not lead to the creation of a valid `Module`.
    pub(crate) fn from_str(s: &str) -> Self {
        // Get the `LexerDef` for the `calc` language.
        let lexerdef = jit_ir_l::lexerdef();
        let lexer = lexerdef.lexer(s);
        // Pass the lexer to the parser and lex and parse the input.
        let (res, errs) = jit_ir_y::parse(&lexer);
        if !errs.is_empty() {
            for e in errs {
                eprintln!("{}", e.pp(&lexer, &jit_ir_y::token_epp));
            }
            panic!("Could not parse input");
        }
        match res {
            Some(Ok((func_types, func_decls, globals, bblocks))) => {
                let mut m = Module::new_testing();
                let mut p = JITIRParser {
                    m: &mut m,
                    lexer: &lexer,
                    inst_idx_map: HashMap::new(),
                    func_types_map: HashMap::new(),
                };
                p.process(func_types, func_decls, globals, bblocks).unwrap();
                m.assert_well_formed();
                m
            }
            _ => panic!("Could not produce JIT Module."),
        }
    }

    /// Assert that for the given IR input, as a string, `ir_input`, when transformed by
    /// `ir_transform(Module)` it produces a [Module] that when printed corresponds to the [fm]
    /// pattern `transformed_ptn`.
    ///
    /// `transformed_ptn` allows the following [fm] patterns:
    ///   * `{{.+?}}` matches against the text using [fm]'s name matching. If you have two patterns
    ///     with the same name (e.g. `${{1}} xyz ${{2}}`) then both must match the same literal
    ///     text.
    ///   * `{{_}}` matches against the text using [fm]'s "ignore" name matching. If you have two
    ///     patterns with `{{_}}` then they may match against different literal text.
    pub(crate) fn assert_ir_transform_eq<F>(ir_input: &str, ir_transform: F, transformed_ptn: &str)
    where
        F: FnOnce(Module) -> Module,
    {
        // We want to share the compilation of regexes amongst threads, *but* there is some locking
        // involved, so we want to `clone` the compiled regexes before using them for matching.
        // Hence this odd looking "`static` then `let`" dance.
        static PTN_RE: OnceLock<Regex> = OnceLock::new();
        static PTN_RE_IGNORE: OnceLock<Regex> = OnceLock::new();
        static LITERAL_RE: OnceLock<Regex> = OnceLock::new();
        let ptn_re = PTN_RE
            .get_or_init(|| Regex::new(r"\{\{.+?\}\}").unwrap())
            .clone();
        let ptn_re_ignore = PTN_RE_IGNORE
            .get_or_init(|| Regex::new(r"\{\{_\}\}").unwrap())
            .clone();
        let literal_re = LITERAL_RE
            .get_or_init(|| Regex::new(r"[a-zA-Z0-9\._]+").unwrap())
            .clone();

        let m = Self::from_str(ir_input);
        let m = ir_transform(m);
        let fmm = FMBuilder::new(transformed_ptn)
            .unwrap()
            .name_matcher_ignore(ptn_re_ignore, literal_re.clone())
            .name_matcher(ptn_re, literal_re)
            .build()
            .unwrap();
        if let Err(e) = fmm.matches(&m.to_string()) {
            panic!("{e}");
        }
    }
}

struct JITIRParser<'lexer, 'input: 'lexer, 'a> {
    lexer: &'lexer LRNonStreamingLexer<'lexer, 'input, DefaultLexerTypes<StorageT>>,
    m: &'a mut Module,
    /// A mapping from local operands (e.g. `%3`) in the input text to instruction offsets. For
    /// example if the first instruction in the input is `%3 = ...` the map will be `%3 -> %0`
    /// indicating that whenever we see `%3` in the input we map that to `%0` in the IR we're
    /// generating. To populate this map, each instruction that defines a new local operand needs
    /// to call [Self::add_assign].
    inst_idx_map: HashMap<InstIdx, InstIdx>,
    func_types_map: HashMap<String, TyIdx>,
}

impl<'lexer, 'input: 'lexer> JITIRParser<'lexer, 'input, '_> {
    fn process(
        &mut self,
        func_types: Vec<ASTFuncType>,
        func_decls: Vec<ASTFuncDecl>,
        _globals: Vec<()>,
        bblocks: Vec<ASTBBlock>,
    ) -> Result<(), Box<dyn Error>> {
        self.process_func_types(func_types)?;
        self.process_func_decls(func_decls)?;

        // We try and put trace inputs into registers, but place floating point values on the stack
        // as yksmp currently doesn't seem able to differentiate general purpose from floating
        // point.
        #[cfg(target_arch = "x86_64")]
        let mut gp_reg_off: u16 = 3; // FIXME: Why do need to start from register 3? I have no idea.
        #[cfg(target_arch = "x86_64")]
        let mut fp_reg_off: u16 = 17; // In DWARF, xmm registers are 17..32.
        let mut inst_off = 0;

        for bblock in bblocks.into_iter() {
            for inst in bblock.insts {
                match inst {
                    ASTInst::Assign { assign, val } => {
                        let op = self.process_operand(val)?;
                        let inst = match op {
                            Operand::Var(iidx) => Inst::Copy(iidx),
                            Operand::Const(cidx) => Inst::Const(cidx),
                        };
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::BinOp {
                        assign,
                        type_,
                        bin_op,
                        lhs,
                        rhs,
                    } => {
                        let tyidx = self.process_type(type_)?;
                        let lhs = self.process_operand(lhs)?;
                        let rhs = self.process_operand(rhs)?;
                        if lhs.tyidx(self.m) != tyidx || rhs.tyidx(self.m) != tyidx {
                            return Err(self.error_at_span(
                                assign,
                                "Binop result type incorrect for one or more operands",
                            ));
                        }
                        let inst = BinOpInst::new(lhs, bin_op, rhs);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::Call {
                        assign,
                        name: name_span,
                        args,
                    } => {
                        let name = &self.lexer.span_str(name_span)[1..];
                        let fd_idx = self.m.find_func_decl_idx_by_name(name);
                        let ops = self.process_operands(args)?;
                        let inst = DirectCallInst::new(self.m, fd_idx, ops)
                            .map_err(|e| self.error_at_span(name_span, &e.to_string()))?;
                        if let Some(x) = assign {
                            self.push_assign(inst.into(), x)?;
                        } else {
                            self.m.push(inst.into()).unwrap();
                        }
                    }
                    ASTInst::Guard {
                        cond,
                        is_true,
                        operands,
                    } => {
                        let mut live_vars = Vec::with_capacity(operands.len());
                        for op in operands {
                            live_vars.push((
                                InstID::new(0.into(), 0.into(), inst_off.into()),
                                PackedOperand::new(&self.process_operand(op)?),
                            ));
                            inst_off += 1;
                        }
                        let gidx = self
                            .m
                            .push_guardinfo(GuardInfo::new(
                                aot_ir::BBlockId::new(0.into(), 0.into()),
                                live_vars,
                                Vec::new(),
                            ))
                            .unwrap();
                        let inst = GuardInst::new(self.process_operand(cond)?, is_true, gidx);
                        self.m.push(inst.into()).unwrap();
                    }
                    ASTInst::ICall {
                        assign,
                        func_type: ft_span,
                        target,
                        args,
                    } => {
                        let ft_name = &self.lexer.span_str(ft_span);
                        let ftidx =
                            *self.func_types_map.get(ft_name.to_owned()).ok_or_else(|| {
                                self.error_at_span(
                                    ft_span,
                                    &format!("No such function type '{ft_name}'"),
                                )
                            })?;
                        let tgt = self.process_operand(target)?;
                        let ops = self.process_operands(args)?;
                        let ic = IndirectCallInst::new(self.m, ftidx, tgt, ops)
                            .map_err(|e| self.error_at_span(ft_span, &e.to_string()))?;
                        let icidx = self
                            .m
                            .push_indirect_call(ic)
                            .map_err(|e| self.error_at_span(ft_span, &e.to_string()))?;
                        let inst = Inst::IndirectCall(icidx);
                        if let Some(x) = assign {
                            self.push_assign(inst, x)?;
                        } else {
                            self.m.push(inst).unwrap();
                        }
                    }
                    ASTInst::ICmp {
                        assign,
                        type_,
                        pred,
                        lhs,
                        rhs,
                    } => {
                        let ty = self.process_type(type_)?;
                        match self.m.type_(ty) {
                            Ty::Integer(1) => (),
                            x => {
                                return Err(self.error_at_span(
                                    assign,
                                    &format!(
                                        "ICmp instructions must assign to an i1, not '{}'",
                                        x.display(self.m)
                                    ),
                                ))
                            }
                        }
                        let inst = ICmpInst::new(
                            self.process_operand(lhs)?,
                            pred,
                            self.process_operand(rhs)?,
                        );
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::FCmp {
                        assign,
                        type_,
                        pred,
                        lhs,
                        rhs,
                    } => {
                        let ty = self.process_type(type_)?;
                        match self.m.type_(ty) {
                            Ty::Integer(1) => (),
                            x => {
                                return Err(self.error_at_span(
                                    assign,
                                    &format!(
                                        "FCmp instructions must assign to an i1, not '{}'",
                                        x.display(self.m)
                                    ),
                                ))
                            }
                        }
                        let inst = FCmpInst::new(
                            self.process_operand(lhs)?,
                            pred,
                            self.process_operand(rhs)?,
                        );
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::Load {
                        assign,
                        type_,
                        val,
                        volatile,
                    } => {
                        let inst = LoadInst::new(
                            self.process_operand(val)?,
                            self.process_type(type_)?,
                            volatile,
                        );
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::Param {
                        assign,
                        type_,
                        tiidx,
                    } => {
                        let off = self
                            .lexer
                            .span_str(tiidx)
                            .parse::<usize>()
                            .map_err(|e| self.error_at_span(tiidx, &e.to_string()))?;
                        assert_eq!(self.m.params.len(), usize::try_from(off).unwrap());
                        let type_ = self.process_type(type_)?;
                        let size = self.m.type_(type_).byte_size().ok_or_else(|| {
                            self.error_at_span(
                                tiidx,
                                "Assigning a trace input to a zero-sized type is nonsensical",
                            )
                        })?;
                        match self.m.type_(type_) {
                            Ty::Void => unreachable!(),
                            Ty::Integer(_) | Ty::Ptr | Ty::Func(_) => {
                                if gp_reg_off == 15 {
                                    panic!("out of gp registers");
                                }
                                self.m.push_param(yksmp::Location::Register(
                                    gp_reg_off,
                                    u16::try_from(size).unwrap(),
                                    0,
                                    vec![],
                                ));
                                gp_reg_off += 1;
                                // FIXME: gross hack to avoid allocating RBP/RSP.
                                while gp_reg_off == 6 || gp_reg_off == 7 {
                                    gp_reg_off += 1;
                                }
                            }
                            Ty::Float(_) => {
                                if fp_reg_off == 32 {
                                    panic!("out of fp regisers");
                                }
                                self.m.push_param(yksmp::Location::Register(
                                    fp_reg_off,
                                    u16::try_from(size).unwrap(),
                                    0,
                                    vec![],
                                ));
                                fp_reg_off += 1;
                            }
                            Ty::Unimplemented(_) => todo!(),
                        }
                        let inst = ParamInst::new(ParamIdx::try_from(off).unwrap(), type_);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::PtrAdd {
                        assign,
                        type_: _,
                        ptr,
                        off,
                    } => {
                        let off = self
                            .lexer
                            .span_str(off)
                            .parse::<i32>()
                            .map_err(|e| self.error_at_span(off, &e.to_string()))?;
                        let inst = PtrAddInst::new(self.process_operand(ptr)?, off);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::DynPtrAdd {
                        assign,
                        type_: _,
                        ptr,
                        num_elems,
                        elem_size,
                    } => {
                        let elem_size = self
                            .lexer
                            .span_str(elem_size)
                            .parse::<u16>()
                            .map_err(|e| self.error_at_span(elem_size, &e.to_string()))?;
                        let inst = DynPtrAddInst::new(
                            self.process_operand(ptr)?,
                            self.process_operand(num_elems)?,
                            elem_size,
                        );
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::SExt { assign, type_, val } => {
                        let inst =
                            SExtInst::new(&self.process_operand(val)?, self.process_type(type_)?);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::ZExt { assign, type_, val } => {
                        let inst =
                            ZExtInst::new(&self.process_operand(val)?, self.process_type(type_)?);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::BitCast { assign, type_, val } => {
                        let inst = BitCastInst::new(
                            &self.process_operand(val)?,
                            self.process_type(type_)?,
                        );
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::SIToFP { assign, type_, val } => {
                        let inst =
                            SIToFPInst::new(&self.process_operand(val)?, self.process_type(type_)?);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::FPExt { assign, type_, val } => {
                        let inst =
                            FPExtInst::new(&self.process_operand(val)?, self.process_type(type_)?);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::FPToSI { assign, type_, val } => {
                        let inst =
                            FPToSIInst::new(&self.process_operand(val)?, self.process_type(type_)?);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::FNeg { assign, val } => {
                        let inst = FNegInst::new(self.process_operand(val)?);
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::Store { tgt, val, volatile } => {
                        let inst = StoreInst::new(
                            self.process_operand(tgt)?,
                            self.process_operand(val)?,
                            volatile,
                        );
                        self.m.push(inst.into()).unwrap();
                    }
                    ASTInst::BlackBox(op) => {
                        let inst = BlackBoxInst::new(self.process_operand(op)?);
                        self.m.push(inst.into()).unwrap();
                    }
                    ASTInst::TraceLoopStart(ops) => {
                        for op in ops {
                            let op = self.process_operand(op)?;
                            self.m.loop_start_vars.push(op);
                        }
                        self.m.push(Inst::TraceLoopStart).unwrap();
                    }
                    ASTInst::TraceLoopJump(ops) => {
                        for op in ops {
                            let op = self.process_operand(op)?;
                            self.m.loop_jump_operands.push(PackedOperand::new(&op));
                        }
                        self.m.push(Inst::TraceLoopJump).unwrap();
                    }
                    ASTInst::Trunc {
                        assign,
                        type_,
                        operand,
                    } => {
                        let inst = TruncInst::new(
                            &self.process_operand(operand)?,
                            self.process_type(type_)?,
                        );
                        self.push_assign(inst.into(), assign)?;
                    }
                    ASTInst::Select {
                        assign,
                        cond,
                        trueval,
                        falseval,
                    } => {
                        let inst = SelectInst::new(
                            self.process_operand(cond)?,
                            self.process_operand(trueval)?,
                            self.process_operand(falseval)?,
                        );
                        self.push_assign(inst.into(), assign)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn process_func_types(&mut self, func_types: Vec<ASTFuncType>) -> Result<(), Box<dyn Error>> {
        for ASTFuncType {
            name: name_span,
            arg_tys,
            is_varargs,
            rtn_ty,
        } in func_types
        {
            let name = self.lexer.span_str(name_span).to_owned();
            if self.func_types_map.get(&name).is_some() {
                return Err(format!("Duplicate function type '{name}'").into());
            }
            let arg_tys = {
                let mut mapped = Vec::with_capacity(arg_tys.len());
                for x in arg_tys.into_iter() {
                    mapped.push(self.process_type(x)?);
                }
                mapped
            };
            let rtn_ty = self.process_type(rtn_ty)?;
            let func_ty = self
                .m
                .insert_ty(Ty::Func(FuncTy::new(arg_tys, rtn_ty, is_varargs)))
                .map_err(|e| self.error_at_span(name_span, &e.to_string()))?;
            self.func_types_map.insert(name, func_ty);
        }
        Ok(())
    }

    fn process_func_decls(&mut self, func_decls: Vec<ASTFuncDecl>) -> Result<(), Box<dyn Error>> {
        for ASTFuncDecl {
            name: name_span,
            arg_tys,
            is_varargs,
            rtn_ty,
        } in func_decls
        {
            let name = self.lexer.span_str(name_span).to_owned();
            if self.func_types_map.get(&name).is_some() {
                todo!();
            }
            let arg_tys = {
                let mut mapped = Vec::with_capacity(arg_tys.len());
                for x in arg_tys.into_iter() {
                    mapped.push(self.process_type(x)?);
                }
                mapped
            };
            let rtn_ty = self.process_type(rtn_ty)?;
            let func_ty = self
                .m
                .insert_ty(Ty::Func(FuncTy::new(arg_tys, rtn_ty, is_varargs)))
                .map_err(|e| self.error_at_span(name_span, &e.to_string()))?;
            self.m
                .insert_func_decl(FuncDecl::new(name, func_ty))
                .map_err(|e| self.error_at_span(name_span, &e.to_string()))?;
        }
        Ok(())
    }

    fn process_operands(&mut self, ops: Vec<ASTOperand>) -> Result<Vec<Operand>, Box<dyn Error>> {
        let mut mapped = Vec::with_capacity(ops.len());
        for x in ops {
            mapped.push(self.process_operand(x)?);
        }
        Ok(mapped)
    }

    fn process_operand(&mut self, op: ASTOperand) -> Result<Operand, Box<dyn Error>> {
        match op {
            ASTOperand::ConstInt(span) => {
                let s = self.lexer.span_str(span);
                let [val, width] = <[&str; 2]>::try_from(s.split('i').collect::<Vec<_>>()).unwrap();
                let width = width
                    .parse::<u32>()
                    .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                let val = if val.starts_with("-") {
                    let val = val
                        .parse::<i64>()
                        .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                    if width < 64
                        && (val < -((1 << width) - 1) / 2 - 1 || val >= ((1 << width) - 1) / 2)
                    {
                        return Err(self.error_at_span(span,
                          &format!("Signed constant {val} exceeds the bit width {width} of the integer type")));
                    }
                    val as u64
                } else {
                    let val = val
                        .parse::<u64>()
                        .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                    if width < 64 && val > (1 << width) - 1 {
                        return Err(self.error_at_span(span,
                          &format!("Unsigned constant {val} exceeds the bit width {width} of the integer type")));
                    }
                    val
                };
                let tyidx = self.m.insert_ty(Ty::Integer(width)).unwrap();
                Ok(Operand::Const(
                    self.m
                        .insert_const(Const::Int(tyidx, val))
                        .map_err(|e| self.error_at_span(span, &e.to_string()))?,
                ))
            }
            ASTOperand::ConstPtr(span) => {
                let s = self.lexer.span_str(span);
                debug_assert!(matches!(&s[0..2], "0x" | "0X"));
                let val = usize::from_str_radix(&s[2..], 16)
                    .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                let const_ = Const::Ptr(val);
                Ok(Operand::Const(
                    self.m
                        .insert_const(const_)
                        .map_err(|e| self.error_at_span(span, &e.to_string()))?,
                ))
            }
            ASTOperand::ConstFloat(span) => {
                // unwrap must succeed if the parser is correct.
                let s = self.lexer.span_str(span).strip_suffix("float").unwrap();
                let val = s
                    .parse::<f32>()
                    .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                let tyidx = self.m.insert_ty(Ty::Float(FloatTy::Float)).unwrap();
                Ok(Operand::Const(
                    self.m
                        .insert_const(Const::Float(tyidx, val as f64))
                        .map_err(|e| self.error_at_span(span, &e.to_string()))?,
                ))
            }
            ASTOperand::ConstDouble(span) => {
                // unwrap must succeed if the parser is correct.
                let s = self.lexer.span_str(span).strip_suffix("double").unwrap();
                let val = s
                    .parse::<f64>()
                    .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                let tyidx = self.m.insert_ty(Ty::Float(FloatTy::Double)).unwrap();
                Ok(Operand::Const(
                    self.m
                        .insert_const(Const::Float(tyidx, val))
                        .map_err(|e| self.error_at_span(span, &e.to_string()))?,
                ))
            }
            ASTOperand::Local(span) => {
                let idx = self.lexer.span_str(span)[1..]
                    .parse::<usize>()
                    .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                let idx =
                    InstIdx::try_from(idx).map_err(|e| self.error_at_span(span, &e.to_string()))?;
                let mapped_idx = self.inst_idx_map.get(&idx).ok_or_else(|| {
                    self.error_at_span(span, &format!("Undefined local operand '%{idx}'"))
                })?;
                Ok(Operand::Var(*mapped_idx))
            }
        }
    }

    fn process_type(&mut self, type_: ASTType) -> Result<TyIdx, Box<dyn Error>> {
        match type_ {
            ASTType::Int(span) => {
                let width = self.lexer.span_str(span)[1..]
                    .parse::<u32>()
                    .map_err(|e| self.error_at_span(span, &e.to_string()))?;
                self.m
                    .insert_ty(Ty::Integer(width))
                    .map_err(|e| self.error_at_span(span, &e.to_string()))
            }
            ASTType::Float(span) => Ok(self
                .m
                .insert_ty(Ty::Float(FloatTy::Float))
                .map_err(|e| self.error_at_span(span, &e.to_string()))?),
            ASTType::Double(span) => Ok(self
                .m
                .insert_ty(Ty::Float(FloatTy::Double))
                .map_err(|e| self.error_at_span(span, &e.to_string()))?),
            ASTType::Ptr => Ok(self.m.ptr_tyidx()),
            ASTType::Void => Ok(self.m.void_tyidx()),
        }
    }

    /// Push `inst` to the end of [self.m] and add an assignment of a local operand (e.g. `%3 =
    /// ...`). Note: this must be called *before* inserting the instruction, as it allowws this
    /// function to capture cases where users try referencing the variable itself (e.g. `%0 = %0`
    /// will be caught as an error by this function).
    fn push_assign(&mut self, inst: Inst, span: Span) -> Result<(), Box<dyn Error>> {
        let iidx = self.lexer.span_str(span)[1..]
            .parse::<usize>()
            .map_err(|e| self.error_at_span(span, &e.to_string()))?;
        let iidx = InstIdx::try_from(iidx).map_err(|e| self.error_at_span(span, &e.to_string()))?;

        if usize::from(iidx) != self.m.insts_len() {
            return Err(self.error_at_span(
                span,
                &format!("Assignment should be '%{}'", self.m.insts_len()),
            ));
        }

        self.m.push(inst).unwrap();
        debug_assert!(self.inst_idx_map.get(&iidx).is_none());
        self.inst_idx_map.insert(iidx, self.m.last_inst_idx());
        Ok(())
    }

    /// Return an error message pinpointing `span` as the culprit.
    fn error_at_span(&self, span: Span, msg: &str) -> Box<dyn Error> {
        let ((line_off, col), _) = self.lexer.line_col(span);
        let code = self
            .lexer
            .span_lines_str(span)
            .split('\n')
            .next()
            .unwrap()
            .trim();
        Box::from(format!(
            "Line {}, column {}:\n  {}\n{}",
            line_off,
            col,
            code.trim(),
            msg
        ))
    }
}

struct ASTFuncType {
    name: Span,
    arg_tys: Vec<ASTType>,
    is_varargs: bool,
    rtn_ty: ASTType,
}

struct ASTFuncDecl {
    name: Span,
    arg_tys: Vec<ASTType>,
    is_varargs: bool,
    rtn_ty: ASTType,
}

#[derive(Debug)]
struct ASTBBlock {
    #[allow(dead_code)]
    label: Span,
    insts: Vec<ASTInst>,
}

#[derive(Debug)]
enum ASTInst {
    BinOp {
        assign: Span,
        #[allow(dead_code)]
        type_: ASTType,
        bin_op: BinOp,
        lhs: ASTOperand,
        rhs: ASTOperand,
    },
    Call {
        assign: Option<Span>,
        name: Span,
        args: Vec<ASTOperand>,
    },
    Guard {
        cond: ASTOperand,
        is_true: bool,
        operands: Vec<ASTOperand>,
    },
    ICall {
        assign: Option<Span>,
        func_type: Span,
        target: ASTOperand,
        args: Vec<ASTOperand>,
    },
    ICmp {
        assign: Span,
        #[allow(dead_code)]
        type_: ASTType,
        pred: Predicate,
        lhs: ASTOperand,
        rhs: ASTOperand,
    },
    FCmp {
        assign: Span,
        type_: ASTType,
        pred: FloatPredicate,
        lhs: ASTOperand,
        rhs: ASTOperand,
    },
    Load {
        assign: Span,
        type_: ASTType,
        val: ASTOperand,
        volatile: bool,
    },
    Param {
        assign: Span,
        type_: ASTType,
        tiidx: Span,
    },
    PtrAdd {
        assign: Span,
        #[allow(dead_code)]
        type_: ASTType,
        ptr: ASTOperand,
        off: Span,
    },
    DynPtrAdd {
        assign: Span,
        #[allow(dead_code)]
        type_: ASTType,
        ptr: ASTOperand,
        num_elems: ASTOperand,
        elem_size: Span,
    },
    SExt {
        assign: Span,
        #[allow(dead_code)]
        type_: ASTType,
        val: ASTOperand,
    },
    ZExt {
        assign: Span,
        #[allow(dead_code)]
        type_: ASTType,
        val: ASTOperand,
    },
    BitCast {
        assign: Span,
        #[allow(dead_code)]
        type_: ASTType,
        val: ASTOperand,
    },
    SIToFP {
        assign: Span,
        type_: ASTType,
        val: ASTOperand,
    },
    FPExt {
        assign: Span,
        type_: ASTType,
        val: ASTOperand,
    },
    FPToSI {
        assign: Span,
        type_: ASTType,
        val: ASTOperand,
    },
    FNeg {
        assign: Span,
        val: ASTOperand,
    },
    Store {
        tgt: ASTOperand,
        val: ASTOperand,
        volatile: bool,
    },
    BlackBox(ASTOperand),
    TraceLoopStart(Vec<ASTOperand>),
    TraceLoopJump(Vec<ASTOperand>),
    Trunc {
        assign: Span,
        type_: ASTType,
        operand: ASTOperand,
    },
    Select {
        assign: Span,
        cond: ASTOperand,
        trueval: ASTOperand,
        falseval: ASTOperand,
    },
    Assign {
        assign: Span,
        val: ASTOperand,
    },
}

#[derive(Debug)]
enum ASTOperand {
    Local(Span),
    ConstInt(Span),
    ConstPtr(Span),
    ConstFloat(Span),
    ConstDouble(Span),
}

#[derive(Debug)]
enum ASTType {
    Int(Span),
    Float(Span),
    Double(Span),
    Ptr,
    Void,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::jitc_yk::jit_ir::{FuncTy, Ty};

    #[test]
    #[ignore] // Requires changing the parser to parse the new param format.
    fn roundtrip() {
        let mut m = Module::new_testing();
        let i16_tyidx = m.insert_ty(Ty::Integer(16)).unwrap();

        m.push_param(yksmp::Location::Register(3, 1, 0, vec![]));
        m.push_param(yksmp::Location::Register(3, 1, 0, vec![]));
        let op1 = m
            .push_and_make_operand(ParamInst::new(ParamIdx::try_from(0).unwrap(), i16_tyidx).into())
            .unwrap();
        let op2 = m
            .push_and_make_operand(ParamInst::new(ParamIdx::try_from(1).unwrap(), i16_tyidx).into())
            .unwrap();
        let op3 = m
            .push_and_make_operand(BinOpInst::new(op1.clone(), BinOp::Add, op2.clone()).into())
            .unwrap();
        let op4 = m
            .push_and_make_operand(BinOpInst::new(op1.clone(), BinOp::Add, op3.clone()).into())
            .unwrap();
        m.push(BlackBoxInst::new(op3).into()).unwrap();
        m.push(BlackBoxInst::new(op4).into()).unwrap();
        let s = m.to_string();
        let parsed_m = Module::from_str(&s);
        assert_eq!(m.to_string(), parsed_m.to_string());
    }

    #[test]
    fn module_patterns() {
        Module::assert_ir_transform_eq(
            "
          entry:
            %0: i16 = param 0
            %1: i16 = param 1
            %2: i16 = add %0, %1
        ",
            |m| m,
            "
          ...
          entry:
            %{{0}}: i16 = param ...
            %{{1}}: i16 = param ...
            %{{_}}: i16 = add %{{0}}, %{{1}}
        ",
        );
    }

    #[test]
    fn all_jit_ir_syntax() {
        Module::from_str(
            "
            func_type ft1(i8, i32, ...) -> i64
            func_decl f1()
            func_decl f2(i8) -> i32
            func_decl f3(i8, i32, ...) -> i64
            func_decl f4(...)
            entry:
              %0: i32 = param 0
              %1: i8 = param 1
              %2: i32 = param 2
              %3: ptr = param 3
              %4: float = param 4
              %5: i8 = param 5
              %6: i16 = param 6
              %7: i32 = param 7
              %8: i64 = param 8
              %9: i32 = param 9
              %10: i32 = trunc %8
              %11: i32 = add %7, %9
              %12: i1 = eq %0, %2
              tloop_start [%0, %6]
              guard true, %12, [%0, %1, %2, 1i8]
              call @f1()
              %16: i32 = call @f2(%5)
              %17: i64 = call @f3(%5, %7, %0)
              call @f4(%0, %1)
              *%3 = %8
              %20: i32 = load %9
              %21: i64 = sext %10
              %22: i64 = zext %10
              %23: i32 = add %7, %9
              %24: i32 = sub %7, %9
              %25: i32 = mul %7, %9
              %26: i32 = or %7, %9
              %27: i32 = and %7, %9
              %28: i32 = xor %7, %9
              %29: i32 = shl %7, %9
              %30: i32 = ashr %7, %9
              %31: float = fadd %4, %4
              %32: float = fdiv %4, %4
              %33: float = fmul %4, %4
              %34: float = frem %4, %4
              %35: float = fsub %4, %4
              %36: i32 = lshr %0, 2i32
              %37: i32 = sdiv %0, 2i32
              %38: i32 = srem %0, 2i32
              %39: i32 = udiv %0, 2i32
              %40: i32 = urem %0, 2i32
              %41: i8 = add %1, 255i8
              %42: i16 = add %6, 32768i16
              %43: i32 = add %10, 2147483648i32
              %44: i64 = add %21, 9223372036854775808i64
              *%3 = 0x0
              *%3 = 0xFFFFFFFF
              %47: i1 = ne %0, %2
              %48: i1 = ugt %0, %2
              %49: i1 = uge %0, %2
              %50: i1 = ult %0, %2
              %51: i1 = ule %0, %2
              %52: i1 = sgt %0, %2
              %53: i1 = sge %0, %2
              %54: i1 = slt %0, %2
              %55: i1 = sle %0, %2
              %56: float = si_to_fp %43
              %57: double = fp_ext %56
              %58: i32 = fp_to_si %56
              %59: double = fadd 1double, 2.345double
              %60: float = fadd 1float, 2.345float
              %61: i64 = icall<ft1> %9(%5, %7, %0)
              %62: float = bitcast %7
              %63: float = fneg %54
              tloop_jump [%43, %58]
        ",
        );
    }

    #[test]
    fn func_decls() {
        let mut m = Module::from_str(
            "
              func_decl f1()
              func_decl f2(i8) -> i32
              func_decl f3(i8, i32, ...) -> i64
              func_decl f4(...)
        ",
        );
        // We don't have a "lookup a function declaration" function, but we can check that if we
        // "insert" identical IR function declarations to the module that no new function
        // declarations are actually added.
        assert_eq!(m.func_decls_len(), 4);

        let f1_tyidx = m
            .insert_ty(Ty::Func(FuncTy::new(Vec::new(), m.void_tyidx(), false)))
            .unwrap();
        m.insert_func_decl(FuncDecl::new("f1".to_owned(), f1_tyidx))
            .unwrap();
        assert_eq!(m.func_decls_len(), 4);

        let i32_tyidx = m.insert_ty(Ty::Integer(32)).unwrap();
        let f2_tyidx = m
            .insert_ty(Ty::Func(FuncTy::new(
                vec![m.int8_tyidx()],
                i32_tyidx,
                false,
            )))
            .unwrap();
        m.insert_func_decl(FuncDecl::new("f2".to_owned(), f2_tyidx))
            .unwrap();
        assert_eq!(m.func_decls_len(), 4);

        let i64_tyidx = m.insert_ty(Ty::Integer(64)).unwrap();
        let f3_tyidx = m
            .insert_ty(Ty::Func(FuncTy::new(
                vec![m.int8_tyidx(), i32_tyidx],
                i64_tyidx,
                true,
            )))
            .unwrap();
        m.insert_func_decl(FuncDecl::new("f3".to_owned(), f3_tyidx))
            .unwrap();
        assert_eq!(m.func_decls_len(), 4);

        let f4_tyidx = m
            .insert_ty(Ty::Func(FuncTy::new(Vec::new(), m.void_tyidx(), true)))
            .unwrap();
        m.insert_func_decl(FuncDecl::new("f4".to_owned(), f4_tyidx))
            .unwrap();
        assert_eq!(m.func_decls_len(), 4);
    }

    #[test]
    fn func_types() {
        let mut m = Module::from_str(
            "
              func_type f1()
              func_type f2(i8) -> i32
              func_type f3(i8, i32, ...) -> i64
              func_type f4(...)
        ",
        );
        // We don't have a "lookup a function type" function, but we can check that if we
        // "insert" identical IR function declarations to the module that no new function
        // types are actually added. We thus need to add all the non-function-types we're
        // interested in first.
        let i32_tyidx = m.insert_ty(Ty::Integer(32)).unwrap();
        let i64_tyidx = m.insert_ty(Ty::Integer(64)).unwrap();
        let types_len = m.types_len();

        m.insert_ty(Ty::Func(FuncTy::new(Vec::new(), m.void_tyidx(), false)))
            .unwrap();
        assert_eq!(m.types_len(), types_len);

        m.insert_ty(Ty::Func(FuncTy::new(
            vec![m.int8_tyidx()],
            i32_tyidx,
            false,
        )))
        .unwrap();
        assert_eq!(m.types_len(), types_len);

        m.insert_ty(Ty::Func(FuncTy::new(
            vec![m.int8_tyidx(), i32_tyidx],
            i64_tyidx,
            true,
        )))
        .unwrap();
        assert_eq!(m.types_len(), types_len);

        m.insert_ty(Ty::Func(FuncTy::new(Vec::new(), m.void_tyidx(), true)))
            .unwrap();
        assert_eq!(m.types_len(), types_len);
    }

    #[test]
    #[should_panic(expected = "Assignment should be '%1'")]
    fn discontinuous_operand_ids() {
        Module::from_str(
            "
          entry:
            %0: i16 = param 0
            %3: i16 = param 1
            %19: i16 = add %7, %3
        ",
        );
    }

    #[test]
    #[should_panic(expected = "Undefined local operand '%7'")]
    fn no_such_local_operand() {
        Module::from_str(
            "
          entry:
            %0: i16 = add %7, %3
        ",
        );
    }

    #[test]
    #[should_panic(expected = "Duplicate function type 't1'")]
    fn duplicate_func_type() {
        Module::from_str(
            "
          func_type t1()
          func_type t1()
          entry:
            %0: i8 = param 0
        ",
        );
    }

    #[test]
    #[should_panic(expected = "No such function type 't2'")]
    fn no_such_func_type() {
        Module::from_str(
            "
          func_type t1()
          entry:
            %0: ptr = param 0
            icall<t2> %0()
        ",
        );
    }

    #[test]
    #[should_panic(expected = "ICmp instructions must assign to an i1, not 'i8'")]
    fn icmp_assign_to_non_i1() {
        Module::from_str(
            "
          entry:
            %0: i8 = ult 1i8, 2i8
            ",
        );
    }

    #[test]
    #[should_panic(expected = "Signed constant -129 exceeds the bit width 8 of the integer type")]
    fn invalid_numbers1() {
        Module::from_str(
            "
          entry:
            %0: i8 = param 0
            %1: i8 = add %0, -128i8
            %2: i8 = add %0, -129i8
            ",
        );
    }

    #[test]
    #[should_panic(expected = "Unsigned constant 256 exceeds the bit width 8 of the integer type")]
    fn invalid_numbers2() {
        Module::from_str(
            "
          entry:
            %0: i8 = param 0
            %1: i8 = add %0, 255i8
            %2: i8 = add %0, 256i8
            ",
        );
    }

    #[test]
    #[should_panic(expected = "Binop result type incorrect for one or more operands")]
    fn invalid_binop_result_ty() {
        Module::from_str(
            "
          entry:
            %0: float = fadd 1.2double, 3.4double
            ",
        );
    }

    #[test]
    #[should_panic(expected = "Undefined local operand '%1'")]
    fn invalid_guard_inst_ref() {
        Module::from_str(
            "
          entry:
            %0: i1 = param 0
            guard true, %0, [%1]
            ",
        );
    }
}
