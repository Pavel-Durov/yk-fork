// Run-time:
//   env-var: YKD_PRINT_IR=jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG_JITSTATE=-
//   stderr:
//     jit-state: start-tracing
//     i=4
//     jit-state: stop-tracing
//     --- Begin jit-pre-opt ---
//     ...
//     define {{ty}} @__yk_compiled_trace_0(ptr %0, ptr %1) {
//        ...
//        %{{fptr}} = getelementptr %YkCtrlPointVars, ptr %0, i32 0, i32 0...
//        %{{load}} = load...
//        ...
//        %{{a}} = add nsw i32 %{{b}}, 2...
//        ...
//        %{{cond}} = icmp sgt i32 %{{val}}, ...
//        br i1 %{{cond}}, label %{{guard-succ-bb}}, label %{{guard-fail-bb}}
//
//     {{guard-fail-bb}}:...
//       ...
//       %{{cprtn}} = call {{ty}} (...) @llvm.experimental.deoptimize.p0(...
//       ret {{ty}} %{{cprtn}}
//
//     {{guard-succ-bb}}:...
//        ...
//        br...
//     }
//     ...
//     --- End jit-pre-opt ---
//     i=3
//     jit-state: enter-jit-code
//     i=2
//     i=1
//     jit-state: deoptimise
//     ...

// Check that using a global constant works.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

const int add = 2;

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  int res = 0;
  YkLocation loc = yk_location_new();
  int i = 4;
  NOOPT_VAL(res);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    fprintf(stderr, "i=%d\n", i);
    res += add;
    i--;
  }
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_drop(mt);

  return (EXIT_SUCCESS);
}
