#!/bin/bash

# set -eu

export YKB_TRACER=swt
export CP_TRANSITION_DEBUG_MODE=false

ARGS=""

if [ "$1" = "verbose" ]; then
    ARGS="-- --nocapture"
fi


# Tests that are currently failing
echo "Running failing tests..."
~/.cargo/bin/cargo test ::outline_recursion.c $ARGS
~/.cargo/bin/cargo test ::floats_return.c $ARGS
~/.cargo/bin/cargo test ::yk_unroll_safe_vs_yk_outline.c $ARGS
~/.cargo/bin/cargo test ::strarray.c $ARGS
~/.cargo/bin/cargo test ::ykd_opt_off.c $ARGS
~/.cargo/bin/cargo test ::no_trace_annotation.c $ARGS
~/.cargo/bin/cargo test ::simple_inline.c $ARGS
~/.cargo/bin/cargo test ::nested_writetoptr.c $ARGS
~/.cargo/bin/cargo test ::nested_sidetrace.c $ARGS
~/.cargo/bin/cargo test ::udiv.c $ARGS
~/.cargo/bin/cargo test ::internal_linkage_same_obj.c $ARGS