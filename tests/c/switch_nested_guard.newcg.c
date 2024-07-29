// Run-time:
//   env-var: YKD_LOG_IR=-:aot
//   env-var: YK_LOG=4
//   env-var: YKD_SERIALISE_COMPILATION=1

// Check that guard failures in nested switches work as expected.

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 3);
  YkLocation loc = yk_location_new();
  // i = 11 -> gfedcblagfed
  // i = 12 -> hwt: gfedcblagfed(c), swt: gfedcblagfed(e)
  // otiginal i = 100;
  int i = 12;
  int j = 0;
  int k = 0;
  NOOPT_VAL(i);
  NOOPT_VAL(j);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    char c, d;
    switch (j % 7) {
    case 6:
      switch (k % 5) {
      case 4:
        d = 'i';
        break;
      case 3:
        d = 'j';
        break;
      case 2:
        d = 'k';
        break;
      case 1:
        d = 'l';
        break;
      case 0:
        d = 'm';
        break;
      }
      printf("1> %c\n", d);
      c = 'a';
      break;
    case 5:
      c = 'b';
      break;
    case 4:
      c = 'c';
      break;
    case 3:
      c = 'd';
      break;
    case 2:
      c = 'e';
      break;
    case 1:
      c = 'f';
      break;
    case 0:
      c = 'g';
      break;
    }
    printf("2> %c\n", c);
    i--;
    j++;
    k++;
  }
  yk_location_drop(loc);
  yk_mt_drop(mt);
  printf("\n");

  return (EXIT_SUCCESS);
}
