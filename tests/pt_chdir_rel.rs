/// Test that it's OK for the process being traced to change directory, even if it was invoked with
/// a relative path.
///
/// This may seem like a rather arbitrary thing to check, but this test was derived from a real
/// bug: the PT backend was trying to create a libipt image using a (stale) relative path for the
/// main binary's object.
use std::{env, path::PathBuf, time::SystemTime};

#[inline(never)]
pub fn work_loop(iters: u64) -> u64 {
    let mut res = 0;
    for _ in 0..iters {
        // Computation which stops the compiler from eliminating the loop.
        res += SystemTime::now().elapsed().unwrap().subsec_nanos() as u64;
    }
    res
}

// FIXME: check if the chip actually supports PT.
#[cfg(perf_pt)]
#[test]
fn pt_chdir_rel() {
    let arg0 = env::args().next().unwrap();
    if arg0.starts_with("/") {
        // Reinvoke ourself with a relative path.
        let path = PathBuf::from(arg0);

        let dir = path.parent().unwrap();
        env::set_current_dir(&dir.to_str().unwrap()).unwrap();

        let prog = path.file_name().unwrap().to_str().unwrap();
        let prog_p = prog.as_ptr() as *const i8;

        let args = env::args().collect::<Vec<_>>();
        let mut args_p = args.iter().map(|a| a.as_ptr()).collect::<Vec<_>>();
        args_p[0] = prog_p as *const u8; // Replace absolute path.
        args_p.push(0 as *const u8); // NULL sentinel.

        // We don't use `std::process::Command` because it can't reliably handle a relative path.
        unsafe { libc::execv(prog_p, args_p.as_ptr() as *const *const i8) };
        unreachable!();
    }

    // When we get here, we have a process that was invoked with a relative path.

    use hwtracer::backends::perf_pt::PerfPTThreadTracer;
    use hwtracer::ThreadTracer;

    let mut tracer = PerfPTThreadTracer::default();

    tracer.start_tracing().unwrap();
    println!("{}", work_loop(env::args().len() as u64));
    let trace = tracer.stop_tracing().unwrap();

    // Now check that the trace decoder can still find its objects after we change dir.
    env::set_current_dir("/").unwrap();
    for b in trace.iter_blocks() {
        b.unwrap(); // this would error if the decoder was confused by changing dir.
    }
}
