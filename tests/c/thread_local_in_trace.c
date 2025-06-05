// ## FIXME: SWT can't handle control points not in main.
// ignore-if: test "$YKB_TRACER" = "swt"
// Run-time:
//   env-var: YKD_LOG_IR=jit-pre-opt
//   env-var: YKD_SERIALISE_COMPILATION=1
//   env-var: YKD_LOG=4
//   stderr:
//     ...
//     %{{8}}: ptr = lookup_global @shadowstack_0
//     %{{9}}: ptr = load %{{8}}
//     ...
//     %{{14}}: ptr = ptr_add %{{_}}, 8
//     ...
//     %{{18}}: i32 = mul %{{17}}, 2i32
//     %{{19}}: ptr = load %{{14}}
//     *%{{19}} = %{{18}}
//     ...
//     Run trace in a thread.
//     ...
//     res: {{thread_ptr}} 2
//     yk-execution: deoptimise ...
//     res: {{thread_ptr}} 2
//     ...

// Check that threads use a different shadow stack than normal execution.

#include <assert.h>
#include <err.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <yk.h>
#include <stdint.h>
#include <yk_testing.h>

#define NUM_THREADS 8
#define ITERS 100000

struct thread_data {
  YkLocation *loc;
  YkMT *mt;
};

__attribute__((noinline))
void foo(int a, int *res) {
  *res = a * 2;
}

// Decrement an integer from ITERS down to the thread's number, then return it.
static void *trace(void *arg) {
  struct thread_data *td = (struct thread_data *)arg;

  uintptr_t i = 3;
  int res = 0;
  NOOPT_VAL(i);
  while (i > 0) {
    yk_mt_control_point(td->mt, td->loc);
    foo(i, &res);
    fprintf(stderr, "res: %p %d\n", &res, *&res);
    i--;
  }
  void *x = (void *)&res;
	// Print this again so we can check with lang_tester that the compiled trace
  // and the thread use the same shadowstack.
  fprintf(stderr, "res: %p %d\n", &res, *&res);
  NOOPT_VAL(i);
  return x;
}

int main() {
  YkLocation loc = yk_location_new();
  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);

  // Create a trace.
  fprintf(stderr, "Create a trace.\n");
  struct thread_data data;
  data.loc = &loc;
  data.mt = mt;
  uintptr_t res_normal = (uintptr_t) trace(&data);

  // Now run a few threads that will use this trace.
  fprintf(stderr, "Run trace in a thread.\n");
  pthread_t thread1;
  pthread_create(&thread1, NULL, trace, &data);

  void *ret;
  pthread_join(thread1, &ret);
  uintptr_t res_thread = (uintptr_t) ret;

  fprintf(stderr, "%ld %ld\n", res_normal, res_thread);
  // Check that the thread uses a different shadow stack.
  assert(res_normal != res_thread);

  yk_location_drop(loc);
  yk_mt_shutdown(mt);

  return (EXIT_SUCCESS);
}
