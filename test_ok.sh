#!/bin/bash

set -e -pipefail

export YKB_TRACER=swt
export CP_TRANSITION_DEBUG_MODE=true

ARGS=""

if [ "$1" = "verbose" ]; then
    ARGS="-- --nocapture"
fi

~/.cargo/bin/cargo test ::simple2.c $ARGS
~/.cargo/bin/cargo test ::simple3.c $ARGS
~/.cargo/bin/cargo test ::simple4.c $ARGS
~/.cargo/bin/cargo test ::simple5.c $ARGS
~/.cargo/bin/cargo test ::choice2.c $ARGS
~/.cargo/bin/cargo test ::awkward_unmappable.c $ARGS
~/.cargo/bin/cargo test ::shadow_longjmp.c $ARGS
~/.cargo/bin/cargo test ::ykd_opt_off.c $ARGS
~/.cargo/bin/cargo test ::qsort.c $ARGS
~/.cargo/bin/cargo test ::many_threads_one_loc.c $ARGS
~/.cargo/bin/cargo test ::intrinsic_noinline.c $ARGS
~/.cargo/bin/cargo test ::stats2.c $ARGS
~/.cargo/bin/cargo test ::indirect_branch.c $ARGS
~/.cargo/bin/cargo test ::varargs_inlined.c $ARGS
~/.cargo/bin/cargo test ::constexpr.c $ARGS
~/.cargo/bin/cargo test ::smmultisrc2.c $ARGS
~/.cargo/bin/cargo test ::funcptrarg_hasir.c $ARGS
~/.cargo/bin/cargo test ::simple5.c $ARGS
~/.cargo/bin/cargo test ::reentrant.c $ARGS
~/.cargo/bin/cargo test ::setlongjmp.c $ARGS
~/.cargo/bin/cargo test ::unmapped_setjmp.c $ARGS
~/.cargo/bin/cargo test ::blockmap.c $ARGS
~/.cargo/bin/cargo test ::choice2.c $ARGS
~/.cargo/bin/cargo test ::safepoint_const.c $ARGS
~/.cargo/bin/cargo test ::awkward_unmappable.c $ARGS
~/.cargo/bin/cargo test ::ykd_opt_off.c $ARGS
