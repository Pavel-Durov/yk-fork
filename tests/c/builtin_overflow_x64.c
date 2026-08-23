// ignore-if: test ${YK_ARCH} != "x86_64"
// Compiler:
//   env-var: YKB_EXTRA_CC_FLAGS=-O0
// Run-time:
//   env-var: YKD_LOG_IR=aot,hir
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG=4
//   stderr:
//     yk-tracing: start-tracing
//     2: sadd=1 uadd=1 ssub=1
//     2: usub=1 smul=1 umul=1
//     yk-tracing: stop-tracing
//     --- Begin aot ---
//     ...
//     %{{_}}: {0: i64, 64: i1} = call llvm.sadd.with.overflow.i64(%{{_}}, %{{_}})
//     ...
//     %{{_}}: {0: i64, 64: i1} = call llvm.uadd.with.overflow.i64(%{{_}}, %{{_}})
//     ...
//     %{{_}}: {0: i64, 64: i1} = call llvm.ssub.with.overflow.i64(%{{_}}, %{{_}})
//     ...
//     %{{_}}: {0: i64, 64: i1} = call llvm.usub.with.overflow.i64(%{{_}}, %{{_}})
//     ...
//     %{{_}}: {0: i64, 64: i1} = call llvm.smul.with.overflow.i64(%{{_}}, %{{_}})
//     ...
//     %{{_}}: {0: i64, 64: i1} = call llvm.umul.with.overflow.i64(%{{_}}, %{{_}})
//     ...
//     --- End aot ---
//     --- Begin hir ---
//     ...
//     %{{_}}: i1 = soverflow add %{{_}}, %{{_}}
//     ...
//     %{{_}}: i1 = uoverflow add %{{_}}, %{{_}}
//     ...
//     %{{_}}: i1 = soverflow sub %{{_}}, %{{_}}
//     ...
//     %{{_}}: i1 = uoverflow sub %{{_}}, %{{_}}
//     ...
//     %{{_}}: i1 = soverflow mul %{{_}}, %{{_}}
//     ...
//     %{{_}}: i1 = uoverflow mul %{{_}}, %{{_}}
//     ...
//     --- End hir ---
//     1: sadd=0 uadd=0 ssub=0
//     1: usub=0 smul=0 umul=0
//     exit

// Check that all six `llvm.*.with.overflow.i64` intrinsics lower and execute correctly.

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <yk.h>
#include <yk_testing.h>

int main(int argc, char **argv) {
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);
  YkLocation loc = yk_location_new();

  int i = 2;
  NOOPT_VAL(loc);
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(mt, &loc);

    long sx = LLONG_MAX - 1, sres;
    int sadd = __builtin_saddl_overflow(sx, (long)i, &sres);

    unsigned long ux = ULONG_MAX - 1, ures;
    int uadd = __builtin_uaddl_overflow(ux, (unsigned long)i, &ures);

    long sy = LLONG_MIN + 1, ssres;
    int ssub = __builtin_ssubl_overflow(sy, (long)i, &ssres);

    unsigned long uy = 1, usres;
    int usub = __builtin_usubl_overflow(uy, (unsigned long)i, &usres);

    long sz = LLONG_MAX, smres;
    int smul = __builtin_smull_overflow(sz, (long)i, &smres);

    unsigned long uz = ULONG_MAX, umres;
    int umul = __builtin_umull_overflow(uz, (unsigned long)i, &umres);

    // Split across two calls: this backend doesn't yet spill extra call args to the
    // stack, and a single call here would exceed the 6 GP argument registers.
    fprintf(stderr, "%d: sadd=%d uadd=%d ssub=%d\n", i, sadd, uadd, ssub);
    fprintf(stderr, "%d: usub=%d smul=%d umul=%d\n", i, usub, smul, umul);
    i--;
  }
  fprintf(stderr, "exit");
  yk_location_drop(loc);
  yk_mt_shutdown(mt);
  return (EXIT_SUCCESS);
}
