#!/bin/bash

# set -eu

export YKB_TRACER=swt
export CP_TRANSITION_DEBUG_MODE=false

~/.cargo/bin/cargo test ::yk_debug_str_outline.c
~/.cargo/bin/cargo test ::promote.c
# ~/.cargo/bin/cargo test ::simple_inline.c
~/.cargo/bin/cargo test ::yk_unroll_safe_vs_yk_outline.c
~/.cargo/bin/cargo test ::floats_return.c
~/.cargo/bin/cargo test ::icmp_ptr.c
~/.cargo/bin/cargo test ::neg_ptradd.c
~/.cargo/bin/cargo test ::simple_nested.c
~/.cargo/bin/cargo test ::nested_sidetrace.c
~/.cargo/bin/cargo test ::signextend_negative.c
~/.cargo/bin/cargo test ::double.c
~/.cargo/bin/cargo test ::nested_writetoptr.c
~/.cargo/bin/cargo test ::float_div.c
~/.cargo/bin/cargo test ::call_args.c
~/.cargo/bin/cargo test ::early_return2.c
~/.cargo/bin/cargo test ::truncate.c
~/.cargo/bin/cargo test ::udiv.c
~/.cargo/bin/cargo test ::trace_too_long_hwt.c
~/.cargo/bin/cargo test ::neg_ptradd_dyn_ptr.c
~/.cargo/bin/cargo test ::simple_fprintf.c
~/.cargo/bin/cargo test ::bf.O3.c
~/.cargo/bin/cargo test ::simple.c
~/.cargo/bin/cargo test ::strarray.c
~/.cargo/bin/cargo test ::outline.c
~/.cargo/bin/cargo test ::float_binop.c
~/.cargo/bin/cargo test ::ptrtoint.c
~/.cargo/bin/cargo test ::conditionals.c
~/.cargo/bin/cargo test ::switch_non_default.c
~/.cargo/bin/cargo test ::mutable_global.c