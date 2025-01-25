#!/bin/bash

set -eu

export YKB_TRACER=swt
export CP_TRANSITION_DEBUG_MODE=false

~/.cargo/bin/cargo test ::simple2.c
~/.cargo/bin/cargo test ::simple3.c
~/.cargo/bin/cargo test ::simple4.c
~/.cargo/bin/cargo test ::simple5.c
~/.cargo/bin/cargo test ::choice2.c
~/.cargo/bin/cargo test ::awkward_unmappable.c
~/.cargo/bin/cargo test ::shadow_longjmp.c
~/.cargo/bin/cargo test ::ykd_opt_off.c
~/.cargo/bin/cargo test ::qsort.c
~/.cargo/bin/cargo test ::many_threads_one_loc.c
~/.cargo/bin/cargo test ::intrinsic_noinline.c
~/.cargo/bin/cargo test ::stats2.c
~/.cargo/bin/cargo test ::indirect_branch.c
~/.cargo/bin/cargo test ::varargs_inlined.c
~/.cargo/bin/cargo test ::constexpr.c
~/.cargo/bin/cargo test ::smmultisrc2.c
~/.cargo/bin/cargo test ::funcptrarg_hasir.c
~/.cargo/bin/cargo test ::simple5.c
~/.cargo/bin/cargo test ::reentrant.c
~/.cargo/bin/cargo test ::setlongjmp.c
~/.cargo/bin/cargo test ::unmapped_setjmp.c
~/.cargo/bin/cargo test ::blockmap.c
~/.cargo/bin/cargo test ::choice2.c
~/.cargo/bin/cargo test ::safepoint_const.c
~/.cargo/bin/cargo test ::awkward_unmappable.c
~/.cargo/bin/cargo test ::ykd_opt_off.c
