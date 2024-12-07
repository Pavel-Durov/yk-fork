// ignore-if: test "$YKB_TRACER" != "swt"
// Run-time:
//   env-var: YKD_LOG_IR=-:aot,jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YK_LOG=4
//   status: success

// Check that functions which address is taken can refer to other
// functions which address is not taken. This test is specific for SWT
// with module cloning enabled. Note that the cloned functions will not
// be visible in the aot ir since they are not serialised.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

int add(int i, int j) { return i + j; }

int dec(int i) { return add(i, -1); }

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int res = 9998;
  int i = 4;
  NOOPT_VAL(loc);
  NOOPT_VAL(res);
  NOOPT_VAL(i);

  // Take a reference to the 'dec' function using a function pointer.
  // This will cause dec function to not be cloned.
  int (*dec_ptr)(int) = dec;

  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    fprintf(stderr, "%d\n", i);
    i = dec_ptr(i);
  }
  fprintf(stderr, "exit\n");
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_shutdown(mt);
  return (EXIT_SUCCESS);
}
