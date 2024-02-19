// # Error:
// # 
// # ---- lang_tests::funcptrarg_hasir.c status ----
// # Error
// # 
// # ---- lang_tests::funcptrarg_hasir.c stderr ----
// # 
// # Pattern (error at line 2):
// #    |...
// # >> |jit-state: enter-jit-code
// #    |z=4
// #    |...
// # 
// # Text (error at line 4):
// #    |jit-state: start-tracing
// #    |z=4
// #    |jit-state: stop-tracing
// # >> |don't know how to handle operand:   %3 = getelementptr i8, ptr %2, i32 0
// ignore-if: test "$YKB_TRACER" == "swt"
// Run-time:
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_PRINT_JITSTATE=1
//   stderr:
//     ...
//     jit-state: enter-jit-code
//     z=4
//     ...

// Test indirect calls where we have IR for the callee.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <yk.h>
#include <yk_testing.h>

__attribute__((noinline)) int foo(int a) {
  NOOPT_VAL(a);
  return a + 1;
}

int bar(int (*func)(int)) {
  int a = func(3);
  return a;
}

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int z = 0, i = 4;
  NOOPT_VAL(i);
  NOOPT_VAL(z);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    z = bar(foo);
    assert(z == 4);
    fprintf(stderr, "z=%d\n", z);
    i--;
  }

  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
