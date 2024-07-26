// Run-time:
//   stdout:
//     51

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

typedef int (*FuncPtr)(int);

__attribute__((noinline)) int add1(int num) { return num + 1; }

__attribute__((noinline)) int add0(int num) { return num; }

__attribute__((noinline)) int execute(FuncPtr f, int num) { return f(num); }

int main(int argc, char **argv) {
  int i = 0;
  YkMT *mt = yk_mt_new(NULL);
  YkLocation loc = yk_location_new();
  int result = 0;
  while (i < 10) {
    yk_mt_control_point(mt, &loc);
    if (i > 5) {
      result += execute(add0, i);
    } else {
      result += execute(add1, i);
    }
    i++;
  }
  printf("%d", result);
  yk_location_drop(loc);
  yk_mt_drop(mt);
  return (EXIT_SUCCESS);
}
