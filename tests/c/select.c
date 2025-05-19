// Run-time:
//   env-var: YKD_LOG_IR=jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG=5
//   stderr:
//     yk-tracing: start-tracing
//     4 1 1
//     yk-tracing: stop-tracing
//     --- Begin jit-pre-opt ---
//     ...
//     %{{13}}: i32 = %{{12}} ? 1i32 : 2i32
//     ...
//     --- End jit-pre-opt ---
//     3 2 3
//     yk-execution: enter-jit-code
//     2 1 4
//     1 2 6
//     yk-execution: deoptimise
//     exit

// Check that select instructions work.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int res = 0;
  int i = 4;
  NOOPT_VAL(loc);
  NOOPT_VAL(res);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    int v = i % 2 == 0 ? 1 : 2;
    res += v;
    fprintf(stderr, "%d %d %d\n", i, v, res);
    i--;
  }
  fprintf(stderr, "exit\n");
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_shutdown(mt);
  return (EXIT_SUCCESS);
}
