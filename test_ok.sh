#!/bin/bash

set -e

export YKB_TRACER=swt
export CP_TRANSITION_DEBUG_MODE=false

ARGS=""

if [ "$1" = "verbose" ]; then
    ARGS="-- --nocapture"
fi

export SWT_MODULE_CLONE_SKIP_FAILING_TEST=true

# find ./tests/c -type f -name "*.c" -exec grep -L "SWT_MODULE_CLONE_SKIP_FAILING_TEST" {} \; | xargs -n 1 basename
~/.cargo/bin/cargo test ::ptrtoint.c $ARGS
~/.cargo/bin/cargo test ::promote.c $ARGS
~/.cargo/bin/cargo test ::neg_ptradd_dyn.c $ARGS
~/.cargo/bin/cargo test ::qsort.c $ARGS
~/.cargo/bin/cargo test ::arg_mapping_callee.c $ARGS
~/.cargo/bin/cargo test ::float.c $ARGS
~/.cargo/bin/cargo test ::truncate.c $ARGS
~/.cargo/bin/cargo test ::switch_default.c $ARGS
~/.cargo/bin/cargo test ::indirect_branch.c $ARGS
~/.cargo/bin/cargo test ::control_point_in_nested_loop.c $ARGS
~/.cargo/bin/cargo test ::resume_and_branch.c $ARGS
~/.cargo/bin/cargo test ::promote_guard.c $ARGS
~/.cargo/bin/cargo test ::inst_type_depends_global.c $ARGS
~/.cargo/bin/cargo test ::safepoint_const.c $ARGS
~/.cargo/bin/cargo test ::funcptrarg_hasir.c $ARGS
~/.cargo/bin/cargo test ::choice.c $ARGS
~/.cargo/bin/cargo test ::yk_debug_str.c $ARGS
~/.cargo/bin/cargo test ::nested_sidetrace.c $ARGS
~/.cargo/bin/cargo test ::double.c $ARGS
~/.cargo/bin/cargo test ::indirect_external_function_call.c $ARGS
~/.cargo/bin/cargo test ::select.c $ARGS
~/.cargo/bin/cargo test ::smmultisrc2.c $ARGS
~/.cargo/bin/cargo test ::unroll_safe_implies_noinline.c $ARGS
~/.cargo/bin/cargo test ::dyn_ptradd_multidim.c $ARGS
~/.cargo/bin/cargo test ::sidetrace_phinode.old.c $ARGS
~/.cargo/bin/cargo test ::nested_writetoptr.c $ARGS
~/.cargo/bin/cargo test ::outline.c $ARGS
~/.cargo/bin/cargo test ::early_return2.c $ARGS
~/.cargo/bin/cargo test ::simple2.c $ARGS
~/.cargo/bin/cargo test ::missing_control_point.c $ARGS
~/.cargo/bin/cargo test ::promote_expr.c $ARGS
~/.cargo/bin/cargo test ::early_return1.c $ARGS
~/.cargo/bin/cargo test ::no_trace_annotation.c $ARGS
~/.cargo/bin/cargo test ::shadow_longjmp.c $ARGS
~/.cargo/bin/cargo test ::intrinsic_noinline.c $ARGS
~/.cargo/bin/cargo test ::goto_loop.c $ARGS
~/.cargo/bin/cargo test ::simple_interp_loop1.c $ARGS
~/.cargo/bin/cargo test ::pthread_create.c $ARGS
~/.cargo/bin/cargo test ::call_ext_in_obj.c $ARGS
~/.cargo/bin/cargo test ::control_point_not_in_loop.c $ARGS
~/.cargo/bin/cargo test ::signextend_positive.c $ARGS
~/.cargo/bin/cargo test ::call_args.c $ARGS
~/.cargo/bin/cargo test ::many_threads_one_loc.c $ARGS
~/.cargo/bin/cargo test ::simple_non_serialised.c $ARGS
~/.cargo/bin/cargo test ::blockmap.c $ARGS
~/.cargo/bin/cargo test ::mutable_global.c $ARGS
~/.cargo/bin/cargo test ::smmultisrc.c $ARGS
~/.cargo/bin/cargo test ::aot_debuginfo.c $ARGS
~/.cargo/bin/cargo test ::dyn_ptradd_simple.c $ARGS
~/.cargo/bin/cargo test ::inline_const.c $ARGS
~/.cargo/bin/cargo test ::icmp_ptr.c $ARGS
~/.cargo/bin/cargo test ::ashr_exact.c $ARGS
~/.cargo/bin/cargo test ::reentrant.c $ARGS
~/.cargo/bin/cargo test ::zext.c $ARGS
~/.cargo/bin/cargo test ::yk_debug_str_outline.c $ARGS
~/.cargo/bin/cargo test ::noopts.c $ARGS
~/.cargo/bin/cargo test ::call_ext_simple.c $ARGS
~/.cargo/bin/cargo test ::internal_linkage_same_obj.c $ARGS
~/.cargo/bin/cargo test ::unroll_safe_inlines.c $ARGS
~/.cargo/bin/cargo test ::constexpr.c $ARGS
~/.cargo/bin/cargo test ::simple5.c $ARGS
~/.cargo/bin/cargo test ::stats3.c $ARGS
~/.cargo/bin/cargo test ::void_ret.c $ARGS
~/.cargo/bin/cargo test ::unmapped_setjmp.c $ARGS
~/.cargo/bin/cargo test ::ykd_opt_off.c $ARGS
~/.cargo/bin/cargo test ::trace_too_long_hwt.c $ARGS
~/.cargo/bin/cargo test ::sidetrace_while.old.c $ARGS
~/.cargo/bin/cargo test ::simple_interp_loop2.c $ARGS
~/.cargo/bin/cargo test ::trace_too_long.c $ARGS
~/.cargo/bin/cargo test ::funcptrarg_noir.c $ARGS
~/.cargo/bin/cargo test ::simple_peeling.c $ARGS
~/.cargo/bin/cargo test ::simple4.c $ARGS
~/.cargo/bin/cargo test ::neg_ptradd_dyn_ptr.c $ARGS
~/.cargo/bin/cargo test ::float_div.c $ARGS
~/.cargo/bin/cargo test ::const_global.c $ARGS
~/.cargo/bin/cargo test ::strarray.c $ARGS
~/.cargo/bin/cargo test ::stats2.c $ARGS
~/.cargo/bin/cargo test ::shadow_reentrant.c $ARGS
~/.cargo/bin/cargo test ::rel_path.c $ARGS
~/.cargo/bin/cargo test ::switch_non_default.c $ARGS
~/.cargo/bin/cargo test ::calls_double.c $ARGS
~/.cargo/bin/cargo test ::many_threads_many_locs.c $ARGS
~/.cargo/bin/cargo test ::funcptrarg_pretrace.c $ARGS
~/.cargo/bin/cargo test ::struct_simple.c $ARGS
~/.cargo/bin/cargo test ::switch_nested_guard.c $ARGS
~/.cargo/bin/cargo test ::intrinsics.c $ARGS
~/.cargo/bin/cargo test ::signextend_negative.c $ARGS
~/.cargo/bin/cargo test ::not_loopy_funcs_inlined_by_default.c $ARGS
~/.cargo/bin/cargo test ::varargs.c $ARGS
~/.cargo/bin/cargo test ::floats.c $ARGS
~/.cargo/bin/cargo test ::indirect_call.c $ARGS
~/.cargo/bin/cargo test ::float_consts.c $ARGS
~/.cargo/bin/cargo test ::simple_fprintf.c $ARGS
~/.cargo/bin/cargo test ::choice2.c $ARGS
~/.cargo/bin/cargo test ::pt_zero_len_call.c $ARGS
~/.cargo/bin/cargo test ::stats4.c $ARGS
~/.cargo/bin/cargo test ::setlongjmp.c $ARGS
~/.cargo/bin/cargo test ::unintptr_t_to_ptr.c $ARGS
~/.cargo/bin/cargo test ::simple.c $ARGS
~/.cargo/bin/cargo test ::simple_nested.c $ARGS
~/.cargo/bin/cargo test ::fp_in_out.c $ARGS
~/.cargo/bin/cargo test ::varargs_inlined.c $ARGS
~/.cargo/bin/cargo test ::dyn_ptradd_mixed.c $ARGS
~/.cargo/bin/cargo test ::fib.c $ARGS
~/.cargo/bin/cargo test ::side-trace.c $ARGS
~/.cargo/bin/cargo test ::float_binop.c $ARGS
~/.cargo/bin/cargo test ::simple3.c $ARGS
~/.cargo/bin/cargo test ::outline_recursion.c $ARGS
~/.cargo/bin/cargo test ::awkward_unmappable.c $ARGS
~/.cargo/bin/cargo test ::inline_asm.c $ARGS
~/.cargo/bin/cargo test ::switch_many_guards_failing.c $ARGS
~/.cargo/bin/cargo test ::yk_unroll_safe_vs_yk_outline.c $ARGS
~/.cargo/bin/cargo test ::neg_ptradd.c $ARGS
~/.cargo/bin/cargo test ::conditionals.c $ARGS
~/.cargo/bin/cargo test ::ptr_global.c $ARGS
~/.cargo/bin/cargo test ::float_mul.c $ARGS
~/.cargo/bin/cargo test ::stats1.c $ARGS
~/.cargo/bin/cargo test ::bf.O0.c $ARGS
