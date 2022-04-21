// Compiler:
// Run-time:
//   env-var: YKD_PRINT_JITSTATE=1
//   env-var: YKD_PRINT_IR=jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   stderr:
//     ...
//     jit-state: start-tracing
//     3: 47
//     jit-state: stop-tracing
//     --- Begin jit-pre-opt ---
//     ...
//     --- End jit-pre-opt ---
//     2: 47
//     jit-state: enter-jit-code
//     1: 47
//     jit-state: enter-stopgap
//     ...

// Check that tracing a cascading "if...else if...else" works.

#include <stdio.h>
#include <stdlib.h>
#include <yk.h>
#include <yk_testing.h>

__attribute__((noinline)) int f(int x) {
  if (x == 0)
    return 30;
  else if (x == 1)
    return 47;
  else
    return 52;
}

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new();
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int i = 3, x = 1;
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    NOOPT_VAL(x);
    fprintf(stderr, "%d: %d\n", i, f(x));
    i--;
  }

  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
