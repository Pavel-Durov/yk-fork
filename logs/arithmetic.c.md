arithmetic.c - segfault on __pthread_kill_implementation

notes:
- when in gdb session no segfault...

command:
rsync_cmd bencher16.soft-dev.org RUST_BACKTRACE=1 YKB_TRACER=swt CP_TRANSITION_DEBUG_MODE=true /home/pd/.cargo/bin/cargo test arithmetic.c
