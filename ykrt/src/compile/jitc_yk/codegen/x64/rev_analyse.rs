//! Perform a reverse analysis on a module's instructions.
//!
//! This is used for for the following purposes:
//!   1. To pass hints to the register allocator about which variables should end up in which
//!      registers. In order to do that, this file has to be kept carefully in sync with
//!      `ls_regalloc.rs`. Failure to do so won't impact correctness, but it will impact
//!      performance, as inaccurate hints will lead the register allocator to generate suboptimal
//!      code.
//!   2. To inline `PtrAdd`s into `Load`s/`Store`s when possible.
//!   3. In part because of (3) -- which is platform specific and thus not part of "normal" module
//!      optimisations -- perform dead-code analysis. Note: the DCE in this module entirely
//!      subsumes the functionality of `dead_code.rs`, so if you use this module for you don't need
//!      to use `dead_code.rs` as well.

use super::{Register, VarLocation};
use crate::compile::jitc_yk::{
    codegen::x64::{ARG_FP_REGS, ARG_GP_REGS},
    jit_ir::{
        BinOp, BinOpInst, DirectCallInst, DynPtrAddInst, ICmpInst, Inst, InstIdx, LoadInst, Module,
        Operand, PtrAddInst, SExtInst, SelectInst, StoreInst, TraceKind, TruncInst, Ty, ZExtInst,
    },
    YkSideTraceInfo,
};
use dynasmrt::x64::Rq;
use std::sync::Arc;
use vob::Vob;

pub(crate) struct RevAnalyse<'a> {
    m: &'a Module,
    /// A `Vec<InstIdx>` with one entry per instruction. Each denotes the last instruction that the
    /// value produced by an instruction is used. By definition this must either be unused (if an
    /// instruction does not produce a value) or `>=` the offset in this vector.
    pub(crate) inst_vals_alive_until: Vec<InstIdx>,
    /// A `Vec<Option<PtrAddInst>>` that "inlines" pointer additions into load/stores. The
    /// `PtrAddInst` is not marked as used, for such instructions: note that it might be marked as
    /// used by other instructions!
    pub(crate) ptradds: Vec<Option<PtrAddInst>>,
    /// A `Vob` with one entry per instruction, denoting whether the value resulting from an
    /// instruction is used. This implicitly enables a layer of dead-code elimination: it doesn't
    /// cause JIT IR instructions to be removed, but it allows a code generator to avoid generating
    /// code for some of them.
    pub(crate) used_insts: Vob,
    /// What [Register] should an instruction aim to put its output to?
    pub(crate) reg_hints: Vec<Option<Register>>,
}

impl<'a> RevAnalyse<'a> {
    pub(crate) fn new(m: &'a Module) -> RevAnalyse<'a> {
        Self {
            m,
            inst_vals_alive_until: vec![InstIdx::try_from(0).unwrap(); m.insts_len()],
            ptradds: vec![None; m.insts_len()],
            used_insts: Vob::from_elem(false, usize::from(m.last_inst_idx()) + 1),
            reg_hints: vec![None; m.insts_len()],
        }
    }

    /// Analyse a trace header. If the trace is [TraceKind::HeaderAndBody], you must call
    /// [Self::analyse_body] as soon as you have processed the trace header.
    pub fn analyse_header(&mut self) {
        // First we populate the register hints for the end of the trace...
        match self.m.tracekind() {
            TraceKind::HeaderOnly => {
                for ((iidx, inst), jump_op) in
                    self.m.iter_skipping_insts().zip(self.m.trace_header_end())
                {
                    match inst {
                        Inst::Param(pinst) => {
                            if let VarLocation::Register(reg) = VarLocation::from_yksmp_location(
                                self.m,
                                iidx,
                                self.m.param(pinst.paramidx()),
                            ) {
                                self.push_reg_hint_fixed(jump_op.unpack(self.m), reg);
                            }
                        }
                        _ => break,
                    }
                }
            }
            TraceKind::HeaderAndBody => {
                // We don't care where the register allocator ends at the end of the header, so we
                // don't propagate backwards from `TraceHeaderEnd`.
            }
            TraceKind::Sidetrace(sti) => {
                let sti = Arc::clone(sti)
                    .as_any()
                    .downcast::<YkSideTraceInfo<Register>>()
                    .unwrap();
                let vlocs = &sti.entry_vars;
                // Side-traces don't have a trace body since we don't apply loop peeling and thus use
                // `trace_header_end` to store the jump variables.
                debug_assert_eq!(vlocs.len(), self.m.trace_header_end().len());

                for (vloc, jump_op) in vlocs.iter().zip(self.m.trace_header_end()) {
                    if let VarLocation::Register(reg) = *vloc {
                        self.push_reg_hint_fixed(jump_op.unpack(self.m), reg);
                    }
                }
            }
        }

        // ...and then we perform the rest of the reverse analysis.
        let mut iter = self.m.iter_skipping_insts().rev();
        match self.m.tracekind() {
            TraceKind::HeaderOnly | TraceKind::Sidetrace(_) => {
                for (iidx, inst) in self.m.iter_skipping_insts().rev() {
                    self.analyse(iidx, inst);
                }
            }
            TraceKind::HeaderAndBody => {
                // OPT: We could pass the index for `TraceHeaderEnd` around, perhaps in the
                // `TraceKind` to avoid having to find it this way.
                let mut next = iter.next();
                while let Some((_, inst)) = next {
                    if let Inst::TraceHeaderEnd = inst {
                        break;
                    }
                    next = iter.next();
                }

                while let Some((iidx, inst)) = next {
                    self.analyse(iidx, inst);
                    next = iter.next();
                }
            }
        }
    }

    /// Analyse a trace body. This must be called iff both of the following are true:
    ///   1. the trace is [TraceKind::HeaderAndBody]
    ///   2. [Self::analyse_header] has already been called.
    pub fn analyse_body(&mut self, header_end_vlocs: &[VarLocation]) {
        for (jump_op, vloc) in self.m.trace_body_end().iter().zip(header_end_vlocs) {
            if let VarLocation::Register(reg) = vloc {
                self.push_reg_hint_fixed(jump_op.unpack(self.m), *reg);
            }
        }

        for (iidx, inst) in self.m.iter_skipping_insts().rev() {
            if let Inst::TraceHeaderEnd = inst {
                break;
            }
            self.analyse(iidx, inst);
        }
    }

    fn analyse(&mut self, iidx: InstIdx, inst: Inst) {
        if self.used_insts.get(usize::from(iidx)).unwrap()
            || inst.is_internal_inst()
            || inst.is_guard()
            || inst.has_store_effect(self.m)
        {
            self.used_insts.set(usize::from(iidx), true);

            match inst {
                Inst::TraceHeaderEnd | Inst::TraceBodyEnd | Inst::SidetraceEnd => {
                    // These are handled in [Self::analyse_header] or [Self::analyse_body].
                }
                Inst::BinOp(x) => self.an_binop(iidx, x),
                Inst::Call(x) => self.an_call(iidx, x),
                Inst::ICmp(x) => self.an_icmp(iidx, x),
                Inst::PtrAdd(x) => self.an_ptradd(iidx, x),
                Inst::DynPtrAdd(x) => self.an_dynptradd(iidx, x),
                // "Inline" `PtrAdd`s into loads/stores, and don't mark the `PtrAdd` as used. This
                // means that some (though not all) `PtrAdd`s will not lead to actual code being
                // generated.
                Inst::Load(x) => {
                    if self.an_load(iidx, x) {
                        return;
                    }
                }
                Inst::Store(x) => {
                    if self.an_store(iidx, x) {
                        return;
                    }
                }
                Inst::SExt(x) => self.an_sext(iidx, x),
                Inst::ZExt(x) => self.an_zext(iidx, x),
                Inst::Select(x) => self.an_select(iidx, x),
                Inst::Trunc(x) => self.an_trunc(iidx, x),
                _ => (),
            }

            // Calculate inst_vals_alive_until
            inst.map_operand_vars(self.m, &mut |x| {
                self.used_insts.set(usize::from(x), true);
                if self.inst_vals_alive_until[usize::from(x)] < iidx {
                    self.inst_vals_alive_until[usize::from(x)] = iidx;
                }
            });
        }
    }

    /// Is the instruction at [iidx] a tombstone or otherwise known to be dead (i.e. equivalent to
    /// a tombstone)?
    pub(crate) fn is_inst_tombstone(&self, iidx: InstIdx) -> bool {
        !self.used_insts[usize::from(iidx)]
    }

    /// Is the value produced by instruction `query_iidx` used after (but not including!)
    /// instruction `cur_idx`?
    pub(super) fn is_inst_var_still_used_after(
        &self,
        cur_iidx: InstIdx,
        query_iidx: InstIdx,
    ) -> bool {
        usize::from(cur_iidx) < usize::from(self.inst_vals_alive_until[usize::from(query_iidx)])
    }

    /// Is the value produced by instruction `query_iidx` used at or after instruction `cur_idx`?
    pub(super) fn is_inst_var_still_used_at(&self, cur_iidx: InstIdx, query_iidx: InstIdx) -> bool {
        usize::from(cur_iidx) <= usize::from(self.inst_vals_alive_until[usize::from(query_iidx)])
    }

    /// Propagate the hint for the instruction being processed at `iidx` to `op`, if appropriate
    /// for `op`.
    fn push_reg_hint(&mut self, iidx: InstIdx, op: Operand) {
        if let Operand::Var(op_iidx) = op {
            self.reg_hints[usize::from(op_iidx)] = self.reg_hints[usize::from(iidx)];
        }
    }

    /// Propagate the hint for the [RegConstraint::OutputCanBeSameAsInput] instruction being
    /// processed at `iidx` to `op`, if appropriate for `op`.
    ///
    /// Note: this function should only be used for situations where an instruction can, with no
    /// special help from the register allocator, move a value.
    fn push_reg_hint_outputcanbesameasinput(&mut self, iidx: InstIdx, op: Operand) {
        if let Operand::Var(op_iidx) = op {
            // This needs to be kept carefully in sync with the logic in
            // [ls_regalloc::RegConstraint::OutputCanBeSameAsInput].
            if !self.is_inst_var_still_used_after(iidx, op_iidx) {
                self.reg_hints[usize::from(op_iidx)] = self.reg_hints[usize::from(iidx)];
            }
        }
    }

    /// Set the hint for to `op` to `reg`, if appropriate for `op`.
    fn push_reg_hint_fixed(&mut self, op: Operand, reg: Register) {
        if let Operand::Var(op_iidx) = op {
            self.reg_hints[usize::from(op_iidx)] = Some(reg);
        }
    }

    fn an_binop(&mut self, iidx: InstIdx, binst: BinOpInst) {
        match binst.binop() {
            BinOp::Add | BinOp::And | BinOp::Or | BinOp::Xor => {
                self.push_reg_hint(iidx, binst.lhs(self.m));
            }
            BinOp::AShr | BinOp::LShr | BinOp::Shl => {
                self.push_reg_hint(iidx, binst.lhs(self.m));
                self.push_reg_hint_fixed(binst.rhs(self.m), Register::GP(Rq::RCX));
            }
            BinOp::Mul | BinOp::SDiv | BinOp::UDiv => {
                self.push_reg_hint_fixed(binst.lhs(self.m), Register::GP(Rq::RAX));
            }
            BinOp::Sub => match (binst.lhs(self.m), binst.rhs(self.m)) {
                (Operand::Const(_), _) => {
                    self.push_reg_hint(iidx, binst.rhs(self.m));
                }
                _ => {
                    self.push_reg_hint(iidx, binst.lhs(self.m));
                }
            },
            _ => (),
        }
    }

    fn an_call(&mut self, _: InstIdx, cinst: DirectCallInst) {
        let mut gp_regs = ARG_GP_REGS.iter();
        let mut fp_regs = ARG_FP_REGS.iter();
        for aidx in cinst.iter_args_idx() {
            match self.m.type_(self.m.arg(aidx).tyidx(self.m)) {
                Ty::Void => unreachable!(),
                Ty::Integer(_) | Ty::Ptr => {
                    if let Some(reg) = gp_regs.next() {
                        self.push_reg_hint_fixed(self.m.arg(aidx), Register::GP(*reg));
                    }
                }
                Ty::Func(_) => todo!(),
                Ty::Float(_) => {
                    if let Some(reg) = fp_regs.next() {
                        self.push_reg_hint_fixed(self.m.arg(aidx), Register::FP(*reg));
                    }
                }
                Ty::Unimplemented(_) => panic!(),
            }
        }
    }

    fn an_icmp(&mut self, iidx: InstIdx, icinst: ICmpInst) {
        self.push_reg_hint(iidx, icinst.lhs(self.m));
    }

    fn an_ptradd(&mut self, iidx: InstIdx, painst: PtrAddInst) {
        self.push_reg_hint_outputcanbesameasinput(iidx, painst.ptr(self.m));
    }

    fn an_dynptradd(&mut self, iidx: InstIdx, dpainst: DynPtrAddInst) {
        self.push_reg_hint(iidx, dpainst.num_elems(self.m));
    }

    /// Analyse a [LoadInst]. Returns `true` if it has been inlined and should not go through the
    /// normal "calculate `inst_vals_alive_until`" phase.
    fn an_load(&mut self, iidx: InstIdx, inst: LoadInst) -> bool {
        if let Operand::Var(op_iidx) = inst.operand(self.m) {
            if let Inst::PtrAdd(pa_inst) = self.m.inst(op_iidx) {
                self.ptradds[usize::from(iidx)] = Some(pa_inst);
                if let Operand::Var(y) = pa_inst.ptr(self.m) {
                    if self.inst_vals_alive_until[usize::from(y)] < iidx {
                        self.inst_vals_alive_until[usize::from(y)] = iidx;
                        self.push_reg_hint_outputcanbesameasinput(iidx, pa_inst.ptr(self.m));
                    }
                    self.used_insts.set(usize::from(y), true);
                }
                return true;
            }
        }
        false
    }

    /// Analyse a [StoreInst]. Returns `true` if it has been inlined and should not go through the
    /// normal "calculate `inst_vals_alive_until`" phase.
    fn an_store(&mut self, iidx: InstIdx, inst: StoreInst) -> bool {
        if let Operand::Var(op_iidx) = inst.tgt(self.m) {
            if let Inst::PtrAdd(pa_inst) = self.m.inst(op_iidx) {
                self.ptradds[usize::from(iidx)] = Some(pa_inst);
                if let Operand::Var(y) = pa_inst.ptr(self.m) {
                    if self.inst_vals_alive_until[usize::from(y)] < iidx {
                        self.inst_vals_alive_until[usize::from(y)] = iidx;
                    }
                    self.used_insts.set(usize::from(y), true);
                }
                if let Operand::Var(y) = inst.val(self.m) {
                    if self.inst_vals_alive_until[usize::from(y)] < iidx {
                        self.inst_vals_alive_until[usize::from(y)] = iidx;
                    }
                    self.used_insts.set(usize::from(y), true);
                }
                return true;
            }
        }
        false
    }

    fn an_sext(&mut self, iidx: InstIdx, seinst: SExtInst) {
        self.push_reg_hint(iidx, seinst.val(self.m));
    }

    fn an_zext(&mut self, iidx: InstIdx, zeinst: ZExtInst) {
        self.push_reg_hint(iidx, zeinst.val(self.m));
    }

    fn an_trunc(&mut self, iidx: InstIdx, tinst: TruncInst) {
        self.push_reg_hint(iidx, tinst.val(self.m));
    }

    fn an_select(&mut self, iidx: InstIdx, sinst: SelectInst) {
        self.push_reg_hint(iidx, sinst.trueval(self.m));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_matches::assert_matches;
    use vob::vob;

    fn rev_analyse_header<'a>(m: &'a Module) -> RevAnalyse<'a> {
        let mut rev_an = RevAnalyse::new(m);
        rev_an.analyse_header();
        rev_an
    }

    #[test]
    fn alive_until() {
        let m = Module::from_str(
            "
            entry:
              %0: i8 = param 0
              header_start [%0]
              %2: i8 = %0
              header_end [%2]
            ",
        );
        let rev_an = rev_analyse_header(&m);
        assert_eq!(
            rev_an.inst_vals_alive_until,
            vec![3, 0, 0, 0]
                .iter()
                .map(|x: &usize| InstIdx::try_from(*x).unwrap())
                .collect::<Vec<_>>()
        );

        let m = Module::from_str(
            "
            entry:
              %0: i8 = param 0
              header_start [%0]
              %2: i8 = add %0, %0
              %3: i8 = add %0, %0
              %4: i8 = %2
              header_end [%4]
            ",
        );
        let rev_an = rev_analyse_header(&m);
        assert_eq!(
            rev_an.inst_vals_alive_until,
            vec![2, 0, 5, 0, 0, 0]
                .iter()
                .map(|x: &usize| InstIdx::try_from(*x).unwrap())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn inline_ptradds() {
        let m = Module::from_str(
            "
            entry:
              %0: ptr = param 0
              %1: ptr = ptr_add %0, 8
              %2: i8 = load %1
              %3: ptr = ptr_add %0, 16
              *%1 = 1i8
              black_box %2
              black_box %3
            ",
        );
        let rev_an = rev_analyse_header(&m);
        assert_eq!(
            rev_an.used_insts,
            vob![true, false, true, true, true, true, true]
        );
        assert_matches!(
            rev_an.ptradds.as_slice(),
            &[None, None, Some(_), None, Some(_), None, None]
        );
        let ptradd = rev_an.ptradds[2].unwrap();
        assert_eq!(ptradd.ptr(&m), Operand::Var(InstIdx::try_from(0).unwrap()));
        assert_eq!(ptradd.off(), 8);
        let ptradd = rev_an.ptradds[4].unwrap();
        assert_eq!(ptradd.ptr(&m), Operand::Var(InstIdx::try_from(0).unwrap()));
        assert_eq!(ptradd.off(), 8);
    }
}
