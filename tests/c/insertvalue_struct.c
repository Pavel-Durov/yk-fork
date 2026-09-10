// Compiler:
//   env-var: YKB_EXTRA_CC_FLAGS=-O1
// Run-time:
//   env-var: YKD_LOG_IR=aot,hir
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG=4
//   stderr:
//     yk-tracing: start-tracing
//     1
//     999
//     yk-tracing: stop-tracing
//     --- Begin aot ---
//     ...
//     func make_struct(%{{_}}: i8, %{{_}}: i64) -> {0: i8, 64: i64} {
//     ...
//     %{{parg_a}}: i8 = arg(0)
//     %{{parg_b}}: i64 = arg(1)
//     ...
//     %{{s0}}: {0: i8, 64: i64} = insert_val poison<{0: i8, 64: i64}>, %{{parg_a}}
//     %{{s}}: {0: i8, 64: i64} = insert_val %{{s0}}, %{{parg_b}}
//     ret %{{s}}
//     ...
//     %{{call_ret}}: {0: i8, 64: i64} = call make_struct(%{{_}}, %{{_}})...
//     ...
//     %{{a}}: i8 = extractvalue %{{call_ret}}, [0]
//     %{{b}}: i64 = extractvalue %{{call_ret}}, [1]
//     ...
//     --- End aot ---
//     --- Begin hir ---
//     ...
//     %{{a2}}: i8 = load %{{_}}
//     %{{b2}}: i64 = load %{{_}}
//     ...
//     %{{a2_ext}}: i32 = zext %{{a2}}
//     ...
//     %{{_}}: i32 = call %{{_}}(%{{_}}, %{{_}}, %{{a2_ext}}) ; @fprintf
//     ...
//     %{{_}}: i32 = call %{{_}}(%{{_}}, %{{_}}, %{{b2}}) ; @fprintf
//     ...
//     --- End hir ---
//     1
//     999
//     yk-execution: enter-jit-code {"trid": "0"}
//     1
//     999
//     1
//     999
//     yk-execution: deoptimise ...
//     exit

// Test that the trace builder handles a struct built via a chain of `insertvalue` instructions.

#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <yk_testing.h>

struct S {
  uint8_t a;
  uint64_t b;
};

__attribute__((noinline)) struct S make_struct(uint8_t a, uint64_t b) {
  struct S ret = {a, b};
  return ret;
}

void interp() {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int res = 9998;
  int i = 4;
  uint8_t a_val = 1;
  uint64_t b_val = 999;
  NOOPT_VAL(loc);
  NOOPT_VAL(res);
  NOOPT_VAL(i);
  NOOPT_VAL(a_val);
  NOOPT_VAL(b_val);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    struct S s1 = make_struct(a_val, b_val);
    fprintf(stderr, "%d\n", s1.a);
    fprintf(stderr, "%ld\n", s1.b);
    i--;
  }
  fprintf(stderr, "exit\n");
  NOOPT_VAL(res);
  yk_location_drop(loc);
  yk_mt_shutdown(mt);
}

int main(int argc, char **argv) {
  interp();
  return (EXIT_SUCCESS);
}
