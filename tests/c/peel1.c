// ## The sanitisers fiddle with the generated code and mean we can't write the
// ## test we want.
// ignore-if: echo $RUSTFLAGS | grep "sanitizer" || test "$YKB_TRACER" = "swt"
// Compiler:
//   env-var: YKB_EXTRA_CC_FLAGS=-O2
// Run-time:
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG_IR=jit-post-opt
//   stderr:
//     0 1
//     --- Begin jit-post-opt ---
//     ...
//     guard true, ...
//     ...
//     guard true, ...
//     header_end [1i32, ...
//     ...
//     body_start ...
//     ...
//     guard true, ...
//     body_end [1i32, ...
//     --- End jit-post-opt ---
//     1 1
//     2 1
//     3 1
//     4 1

// Check that peeling works: a constant should be discovered by `header_end`
// that allows the body to have only 1 guard where the header must have 2.

#include <assert.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <yk.h>
#include <yk_testing.h>

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int i = 0;
  NOOPT_VAL(i);
  for (; i < 5; i++) {
    yk_mt_control_point(mt, &loc);
    int y = yk_promote(argc);

    fprintf(stderr, "%d %d\n", i, y);
  }

  yk_location_drop(loc);
  yk_mt_shutdown(mt);
  return (EXIT_SUCCESS);
}
