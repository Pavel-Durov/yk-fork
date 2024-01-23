// Run-time:
//   env-var: YKD_PRINT_JITSTATE=1
//   env-var: YKD_SERIALISE_COMPILATION=1
//   stderr: ...

// Run: YKB_TRACER=sw cargo test ::simple_swt.c -- --nocapture
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

  int i = 0;
  NOOPT_VAL(loc);
  NOOPT_VAL(i);
  while (i < 10) {
    yk_mt_control_point(mt, &loc);
    i = inc(i);
  }
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
