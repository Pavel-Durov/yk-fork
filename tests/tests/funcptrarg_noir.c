// Run-time:
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_PRINT_IR=jit-pre-opt
//   env-var: YKD_PRINT_JITSTATE=1
//   stderr:
//     ...
//     --- Begin jit-pre-opt ---
//     ...
//     %{{44}} = call i64 %{{43}}(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.{{str}}, i64 0, i64 0))...
//     ...
//     --- End jit-pre-opt ---
//     ...

// Test indirect calls where we don't have IR for the callee.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

int bar(size_t (*func)(const char *)) {
  int a = func("abc");
  return a;
}

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new();
  yk_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int z = 0, i = 2;
  size_t (*f)(const char *) = strlen;
  NOOPT_VAL(i);
  NOOPT_VAL(z);
  NOOPT_VAL(f);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    z = bar(f);
    i--;
  }
  NOOPT_VAL(z);
  assert(z == 3);

  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
