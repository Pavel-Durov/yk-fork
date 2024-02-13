// # Error:
// # ---- lang_tests::sidetrace_while.c status ----
// # Success
// # 
// # ---- lang_tests::sidetrace_while.c stderr ----
// # 
// # Pattern (error at line 4):
// #    |...
// #    |jit-state: execute-side-trace
// #    |...
// # >> |500
// # 
// # Text (error at line 1784):
// #    ...
// #    |jit-state: execute-side-trace
// #    |731627980
// #    |jit-state: deoptimise
// # >> |731628000
// ignore-if: test "$YKB_TRACER" == "swt"
// Run-time:
//   env-var: YKD_PRINT_IR=jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_PRINT_JITSTATE=1
//   stderr:
//     ...
//     jit-state: execute-side-trace
//     ...
//     500
//   stdout:
//     exit

// Test side tracing inside an unrolled while loop.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

__attribute__((yk_unroll_safe)) int foo(int i) {
  int z = 10;
  int res = 0;
  while (z > 0) {
    z--;
    if (i > 20) {
      res++;
    } else {
      res += 2;
    }
  }
  return res;
}

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  yk_mt_sidetrace_threshold_set(mt, 5);
  YkLocation loc = yk_location_new();

  int res = 0;
  int i = 30;
  NOOPT_VAL(loc);
  NOOPT_VAL(res);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    res += foo(i);
    fprintf(stderr, "%d\n", res);
    i--;
  }
  printf("exit");
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
