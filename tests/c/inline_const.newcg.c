// ignore-if: test $YK_JIT_COMPILER != "yk" -o "$YKB_TRACER" = "swt"
// Run-time:
//   env-var: YKD_LOG_IR=-:jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG_JITSTATE=-
//   stderr:
//     jitstate: start-tracing
//     foo 3
//     jitstate: stop-tracing
//     --- Begin jit-pre-opt ---
//     ...
//     %{{2}}: i32 = Call @fprintf(%{{3}}, %{{4}}, 3i32)
//     ...
//     --- End jit-pre-opt ---
//     foo 3
//     jitstate: enter-jit-code
//     foo 3
//     foo 3
//     jitstate: deoptimise
//     exit

// Check that constant return values of inlined functions are properly mapped.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

__attribute__((noinline)) int foo(int i) { return 3; }

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int i = 4;
  NOOPT_VAL(loc);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    int x = foo(i);
    fprintf(stderr, "foo %d\n", x);
    i--;
  }
  fprintf(stderr, "exit\n");
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
