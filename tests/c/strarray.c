// Run-time:
//   env-var: YKD_PRINT_IR=aot
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_PRINT_JITSTATE=1
//   stderr:
//     jit-state: start-tracing
//     pepper
//     jit-state: stop-tracing
//     --- Begin aot ---
//     ...
//     @fruits = internal constant [5 x ptr] [ptr @.str.2, ptr @.str.3, ptr @.str.4,...
//     ...
//     @.str.2 = private unnamed_addr constant [6 x i8] c"apple\00", align 1...
//     @.str.3 = private unnamed_addr constant [7 x i8] c"banana\00", align 1...
//     @.str.4 = private unnamed_addr constant [7 x i8] c"tomato\00", align 1...
//     ...
//     --- End aot ---
//     cucumber
//     jit-state: enter-jit-code
//     tomato
//     banana
//     jit-state: deoptimise
//     ...
//     jit-state: exit-jit-code
//   stdout:
//     exit

// Check that when we clone a `GlobalVariable` into the JITMod, and that global
// references other `GlobalVariables`, the other globals are cloned as well.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

static const char *const fruits[] = {"apple", "banana", "tomato", "cucumber",
                                     "pepper"};

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
    fprintf(stderr, "%s\n", fruits[i]);
    res += 2;
    i--;
  }
  printf("exit");
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
