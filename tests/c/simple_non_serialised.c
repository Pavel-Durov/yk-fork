// Run-time:
//   env-var: YKD_LOG_JITSTATE=1
//   env-var: YKD_STATS=/dev/null
//   stderr:
//     jit-state: start-tracing
//     i=4
//     jit-state: stop-tracing
//     i=3
//     jit-state: enter-jit-code
//     i=2
//     i=1
//     jit-state: deoptimise
//   stdout:
//     exit

// Check that basic trace compilation in a thread works.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

bool test_compiled_event(YkCStats);

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int res = 9998;
  int i = 4;
  NOOPT_VAL(loc);
  NOOPT_VAL(res);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    if (i == 3)
      __ykstats_wait_until(mt, test_compiled_event);
    fprintf(stderr, "i=%d\n", i);
    res += 2;
    i--;
  }
  printf("exit");
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}

bool test_compiled_event(YkCStats stats) {
  return stats.traces_compiled_ok == 1;
}
