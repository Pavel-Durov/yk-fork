// ignore-if: test $YK_JIT_COMPILER != "yk" -o "$YKB_TRACER" = "swt"
// Run-time:
//   env-var: YKD_LOG_IR=-:aot,jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG_JITSTATE=-
//   stderr:
//     jitstate: start-tracing
//     i=4, elem=14
//     jitstate: stop-tracing
//     --- Begin aot ---
//     ...
//     ${{9_4}}: ptr = PtrAdd @array, 0 + (${{9_3}} * 4)
//     ...
//     --- End aot ---
//     --- Begin jit-pre-opt ---
//     ...
//     %{{14}}: i64 = Mul %{{12}}, 4i64
//     %{{15}}: ptr = PtrAdd %{{13}}, %{{14}}
//     ...
//     --- End jit-pre-opt ---
//     i=3, elem=13
//     jitstate: enter-jit-code
//     i=2, elem=12
//     i=1, elem=11
//     jitstate: deoptimise

// Check dynamic ptradd instructions work.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

uint32_t array[] = {10, 11, 12, 13, 14};

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int i = 4;
  NOOPT_VAL(loc);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    fprintf(stderr, "i=%d, elem=%d\n", i, array[i]);
    i--;
  }
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
