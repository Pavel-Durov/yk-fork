# PR split plan — intrinsics work

Current working tree (8 files, ~655 insertions) is four unrelated changes stacked.
Split as below; PRs 1–4 are independent of each other, PR 5 depends on PR 4.

## PR 1 — ignore `llvm.prefetch`

**Files:** `ykrt/src/compile/j2/aot_to_hir.rs` (2 lines)

Add `"prefetch" => Ok(())` to `p_llvm_intrinsic`. A hint we're free to drop.

Smallest thing that ships. No test needed.

## PR 2 — `Select` on `Ty::Func`

**Files:** `ykrt/src/compile/j2/x64/x64hir_to_asm.rs` (~8 lines)

`i_select`: merge the `Ty::Func(_)` arm into the pointer arm (`Ty::Func(_) | Ty::Ptr(0)`),
since a func value is a code pointer at the machine level. Non-zero addrspace becomes
`todo!()` instead of a failed `assert_eq!`.

Not an intrinsics change — it just fell into the same diff.

**TODO before shipping:** add a `cg_select` case with a func-typed operand. There is none
in the current diff.

## PR 3 — `llvm.fshl`

**Files:** `ykrt/src/compile/j2/aot_to_hir.rs` (~90 lines)

`p_fshl`, lowering funnel-shift-left to existing HIR instructions:

    s == 0 ? lhs : (lhs << s) | (rhs >> (bitw - s))     where s = shift % bitw

The `s == 0` special case exists because `rhs >> bitw` is poison. Asserts `bitw` is a
power of two (otherwise `shift % bitw` isn't a simple mask).

Self-contained: one file, no new HIR instruction, no backend work.

**TODO before shipping:** no test in the current diff. Add at least one covering `s == 0`
returning `lhs` — that case is the entire reason the `Select` is there.

## PR 4 — `Overflow` HIR instruction + x64 codegen

**Files:**
- `ykrt/src/compile/j2/hir.rs` — `Overflow` inst + `OverflowOp` enum
- `ykrt/src/compile/j2/hir.l` — `soverflow` / `uoverflow` tokens
- `ykrt/src/compile/j2/hir.y` — grammar rules + `OverflowOp` production
- `ykrt/src/compile/j2/hir_parser.rs` — `AstInst::Overflow`
- `ykrt/src/compile/j2/hir_to_asm.rs` — dispatch + `i_overflow` trait method
- `ykrt/src/compile/j2/x64/x64hir_to_asm.rs` — `i_overflow` + `cg_overflow` tests
- `ykrt/src/compile/j2/x64/x64regalloc.rs` — `GP_REGS_NO_RAX_RDX`

Largest PR, but coherent, and fully testable through the HIR parser — `cg_overflow`
already exercises it with no AOT wiring.

Review load sits here:
- Unsigned `mul` has to use one-operand `mul` (`RDX:RAX`), hence the extra register
  constraints and the `GP_REGS_NO_RAX_RDX` list.
- Signed overflow reads `OF`, unsigned reads `CF`; neither is meaningful unless the op
  runs at exactly the operands' bit width, so narrow values can't be widened to 32 bits
  the way `i_add` does.
- 8/16-bit widths are `todo!()`. `Overflow::assert_well_formed` accepts any int width,
  so those reach the backend and panic.
- Minor: `cse_eq` ends in `if ... { true } else { false }`.

## PR 5 — AOT wiring for `llvm.{s,u}{add,mul,sub}.with.overflow`

**Files:** `ykrt/src/compile/j2/aot_to_hir.rs`

- `with_overflow_op` — name matcher (unit-tested; correctly rejects `sadd.sat`, `smul.fix`)
- `p_with_overflow` — emits the wrapping `Add`/`Mul`/`Sub` **and** an `Overflow`
- `intrinsic_multi_results` map + `intrinsic_multi_result` helper
- `p_extractvalue` forwarding for values with no backing memory
- Dispatched before `p_ty`, because `p_ty` can't represent the `{result, overflow}` struct

Depends on PR 4.

Note: the arithmetic is performed twice at machine level (once for the result, once for
the flag).

**Fix before shipping:**
1. `intrinsic_multi_result` parks a null pointer as the call's local. Anything other than
   `extractvalue` consuming that call silently gets a valid-looking null pointer instead
   of a failure.
2. `intrinsic_multi_results` is keyed by `InstId` and never cleared between frames.

## Build blocker

`cargo check -p ykrt --tests` does not currently reach the Rust at all. `ykbuild/build.rs`
dies compiling ykllvm's C++:

    ykllvm/lib/Transforms/Yk/ControlPoint.cpp — 3 errors
    no matching ConstantInt::get overload taking a Value* where uint64_t / APInt expected

An ykllvm-vs-LLVM API mismatch, unrelated to this diff, but it means **none of the above
is compile-verified**. `ykllvm/` is gitignored, so `git status` doesn't show it.

---

This file is untracked but not ignored — don't let `git add -A` sweep it into one of the
PRs above.
