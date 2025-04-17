## yk_outline_dynamic_with_promote.c
CP_PATCHPOINT=1 CP_VERBOSE=1 YKB_TRACER=swt cargo test ::yk_outline_dynamic_with_promote.c -- --nocapture 


thread '<unnamed>' panicked at ykrt/src/compile/jitc_yk/trace_builder.rs:1521:9:
assertion `left == right` failed
  left: 0
 right: 8
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread '<unnamed>' panicked at ykrt/src/mt.rs:262:39:
called `Result::unwrap()` on an `Err` value: Any { .. }
fatal runtime error: failed to initiate panic, error 5



## trace_while_executing2.c

CP_PATCHPOINT=1 CP_VERBOSE=1 YKB_TRACER=swt cargo test ::trace_while_executing2.c -- --nocapture 

Pattern (error at line 14):
   ...
   |0: 3
   |yk-jit-event: deoptimise
   |yk-jit-event: start-tracing
>> |1: 3
   |yk-warning: tracing-aborted: tracing went outside of starting frame
   |0: 2
   |yk-jit-event: start-tracing
   ...

Text (error at line 51):
   ...
   |0: 3
   |yk-jit-event: deoptimise
   |yk-jit-event: start-tracing
>> |1: 1999903528


## idempotent.c

CP_PATCHPOINT=1 CP_VERBOSE=1 YKB_TRACER=swt cargo test ::idempotent.c -- --nocapture 

Pattern (error at line 57):
   ...
   |3: 41 41
   |3: 43 43
   |yk-jit-event: enter-jit-code
>> |2: 39 39
   |2: 41 41
   |2: 43 43
   |1: 39 39
   ...

Names at point of failure:
  {{size}}: 64

Text (error at line 823):
   ...
   |3: 41 41
   |3: 43 43
   |yk-jit-event: enter-jit-code
>> |yk-jit-event: deoptimise
   |2: 2 2
   |2: 41 41
   |2: 43 43
   ...

## idempotent_outline.c

CP_PATCHPOINT=1 CP_VERBOSE=1 YKB_TRACER=swt cargo test ::idempotent_outline.c -- --nocapture 

Pattern (error at line 31):
   ...
   |2: 20
   |yk-jit-event: enter-jit-code
   |yk-jit-event: deoptimise
>> |1: 16

Names at point of failure:
  {{size}}: 64
  {{v}}: 4

Text (error at line 536):
   ...
   |2: 20
   |yk-jit-event: enter-jit-code
   |yk-jit-event: deoptimise
>> |4222429319: 16889717288

## float_store.c

CP_PATCHPOINT=1 CP_VERBOSE=1 YKB_TRACER=swt cargo test ::float_store.c -- --nocapture 

test lang_tests::float_store.c ... FAILED

failures:

---- lang_tests::float_store.c status ----
Exited due to signal: 11

---- lang_tests::float_store.c stderr ----

yk-jit-event: start-tracing
4 -> 3.252033
4 -> 3.252033
4 -> 3.252033
4 -> 3.252033
4 -> 3.252033

