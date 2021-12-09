// Compiler:
//   env-var: YKD_PRINT_JITSTATE=1
// Run-time:
//   env-var: YKD_PRINT_IR=jit-pre-opt
//   stderr:
//     jit-state: start-tracing
//     ...
//     define internal %YkCtrlPointVars @__yk_compiled_trace_0(%YkCtrl...
//        ...
//     }
//     ...
//     jit-state: stop-tracing
//     jit-state: enter-jit-code
//     intrinsics: guard-failure

// Check that basic trace compilation works.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

void _yk_test(int i, int res) {
  if (i == 0)
    assert(res == 2);
}

int main(int argc, char **argv) {
  int res = 0;
  YkLocation loc = yk_location_new();
  int i = 3;
  NOOPT_VAL(res);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_control_point(&loc);
    memcpy(&res, &i, 4);
    i--;
  }
  NOOPT_VAL(res);
  assert(res == 0);

  return (EXIT_SUCCESS);
}