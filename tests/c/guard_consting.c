// Run-time:
//   env-var: YKD_LOG_IR=jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   stderr:
//     ...
//     %{{30}}: i1 = sgt %{{_}}, 0i32
//     guard true, %21, [11:%0_7: %0, 11:%0_8: %1, 11:%0_9: %2, 11:%0_10: %3, 11:%9_2: 0i1] ; trace_gidx 1 safepoint_id 4
//     ...

// Check that if a guard's life variables include the condition operand, that
// is converted to a constant.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  yk_mt_sidetrace_threshold_set(mt, 5);
  YkLocation loc = yk_location_new();

  int res = 0;
  int i = 20;
  NOOPT_VAL(loc);
  NOOPT_VAL(res);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    if (i % 2 == 0)
      res += 1;
    else
      res += i;
    fprintf(stderr, "%d\n", res);
    i--;
  }
  NOOPT_VAL(res);
  printf("%d\n", res);
  yk_location_drop(loc);
  yk_mt_shutdown(mt);
  return (EXIT_SUCCESS);
}
