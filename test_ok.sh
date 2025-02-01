#!/bin/bash

set -e -pipefail

export YKB_TRACER=swt
export CP_TRANSITION_DEBUG_MODE=false

ARGS=""

if [ "$1" = "verbose" ]; then
    ARGS="-- --nocapture"
fi

# Tests that are passing
echo "Running passing tests..."
~/.cargo/bin/cargo test ::safepoint_const.c $ARGS
~/.cargo/bin/cargo test ::switch_default.c $ARGS
~/.cargo/bin/cargo test ::simple_nested.c $ARGS
~/.cargo/bin/cargo test ::promote.c $ARGS
~/.cargo/bin/cargo test ::inst_type_depends_global.c $ARGS
~/.cargo/bin/cargo test ::truncate.c $ARGS
~/.cargo/bin/cargo test ::choice2.c $ARGS
~/.cargo/bin/cargo test ::ptr_global.c $ARGS
~/.cargo/bin/cargo test ::ptrtoint.c $ARGS
~/.cargo/bin/cargo test ::struct_simple.c $ARGS
~/.cargo/bin/cargo test ::double.c $ARGS
~/.cargo/bin/cargo test ::mutable_global.c $ARGS
~/.cargo/bin/cargo test ::neg_ptradd_dyn_ptr.c $ARGS
~/.cargo/bin/cargo test ::calls_double.c $ARGS
~/.cargo/bin/cargo test ::ashr_exact.c $ARGS
~/.cargo/bin/cargo test ::early_return1.c $ARGS
~/.cargo/bin/cargo test ::awkward_unmappable.c $ARGS
~/.cargo/bin/cargo test ::simple_interp_loop1.c $ARGS
~/.cargo/bin/cargo test ::dyn_ptradd_simple.c $ARGS
~/.cargo/bin/cargo test ::nested_writetoptr.c $ARGS
~/.cargo/bin/cargo test ::icmp_ptr.c $ARGS
~/.cargo/bin/cargo test ::aot_debuginfo.c $ARGS
~/.cargo/bin/cargo test ::simple.c $ARGS
~/.cargo/bin/cargo test ::dyn_ptradd_mixed.c $ARGS
~/.cargo/bin/cargo test ::strarray.c $ARGS
~/.cargo/bin/cargo test ::float_binop.c $ARGS
~/.cargo/bin/cargo test ::simple.c $ARGS