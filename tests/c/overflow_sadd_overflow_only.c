// Compiler:
//   env-var: YKB_EXTRA_CC_FLAGS=-O0
// Run-time:
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG_IR=aot,hir
//   env-var: YKD_LOG=4
//   stderr:
//     yk-tracing: start-tracing
//     5: sadd_ov=1
//     yk-tracing: stop-tracing
//     --- Begin aot ---
//     ...
//     %{{sadd}}: {0: i32, 32: i1} = call llvm.sadd.with.overflow.i32(%{{_}}, 2i32)
//     br bb{{sadd_bb}}
//     bb{{sadd_bb}}:
//     %{{sadd_ov}}: i1 = extractvalue %{{sadd}}, [1]
//     %{{sadd_r}}: i32 = extractvalue %{{sadd}}, [0]
//     ...
//     %{{_}}: i32 = call fprintf(%{{_}}, @{{_}}, %{{_}}, %{{_}})
//     ...
//     --- End aot ---
//     --- Begin hir ---
//     ...
//     %{{h_packed}}: i64 = sadd_overflow %{{h_lhs}}, %{{h_rhs}}
//     %{{h_sadd_r}}: i32 = extractval %{{h_packed}} [0]
//     %{{h_sadd_ov}}: i1 = extractval %{{h_packed}} [32]
//     store %{{h_sadd_r}}, %{{h_sr_ptr}}
//     ...
//     %{{h_stderr}}: ptr = 0x{{_}} ; @stderr
//     %{{h_stream}}: ptr = load %{{h_stderr}}
//     %{{h_i}}: i32 = load %{{h_iptr}}
//     %{{h_ovz}}: i32 = zext %{{h_sadd_ov}}
//     %{{h_fmt}}: ptr = 0x{{_}} ; @.str
//     %{{h_fprintf}}: ptr = 0x{{_}} ; @fprintf
//     %{{_}}: i32 = call %{{h_fprintf}}(%{{h_stream}}, %{{h_fmt}}, %{{h_i}}, %{{h_ovz}}) ; @fprintf
//     ...
//     --- End hir ---
//     4: sadd_ov=0
//     yk-execution: enter-jit-code {"trid": "0"}
//     3: sadd_ov=1
//     2: sadd_ov=0
//     1: sadd_ov=1
//     yk-execution: deoptimise {"trid": "0", "gidx": "0"}
//     exit

// Check that llvm.sadd.with.overflow behaves correctly when the result is unused.

#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <yk.h>
#include <yk_testing.h>

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int i = 5;
  NOOPT_VAL(loc);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);
    int sa = i % 2 ? INT_MAX : INT_MAX - 2;
    int result;
    NOOPT_VAL(sa);
    bool overflow = __builtin_sadd_overflow(sa, 2, &result);

    fprintf(stderr, "%d: sadd_ov=%d\n", i, overflow);
    i--;
  }
  fprintf(stderr, "exit\n");
  yk_location_drop(loc);
  yk_mt_shutdown(mt);
  return (EXIT_SUCCESS);
}
