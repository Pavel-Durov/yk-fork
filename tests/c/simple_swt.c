// Run-time:
//   env-var: YKB_TRACER=sw
//   stderr: ...

// Check that basic trace compilation works for Software Tracer.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

int add1(int i){
  return i + 1;
}

int inc(int i){
  return add1(i);
}

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
    inc(res);
    i--;
  }
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
