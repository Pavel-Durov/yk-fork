//! Trace promotion: promote values to constants when recording and compiling a trace.
//!
//! In C, these are exposed to the user via the `yk_promote` value which automatically picks the
//! right method in this module to call.
//!
//! # Performance Optimisation
//!
//! These functions use a two-level check to minimise TLS overhead:
//! 1. First check `any_thread_tracing()` - a simple atomic load (no TLS lookup)
//! 2. Only if that returns true, check `is_tracing()` - which requires TLS access
//!
//! When no thread is tracing (the common case after convergence), only the fast atomic
//! check is performed, avoiding all TLS lookups and `__tls_get_addr` calls.

use crate::mt::MTThread;
use std::ffi::{c_int, c_longlong, c_uint, c_void};

/// Promote a `c_int` during trace recording.
#[unsafe(no_mangle)]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub extern "C" fn __yk_promote_c_int(val: c_int) -> c_int {
    // Fast path: if no thread is tracing, skip TLS lookup entirely
    if MTThread::any_thread_tracing() && MTThread::is_tracing() {
        MTThread::with_borrow_mut(|mtt| {
            // We ignore the return value as we can't really cancel tracing from this function.
            mtt.promote_i32(val);
        });
    }
    val
}

/// Promote a `c_uint` during trace recording.
#[unsafe(no_mangle)]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub extern "C" fn __yk_promote_c_unsigned_int(val: c_uint) -> c_uint {
    // Fast path: if no thread is tracing, skip TLS lookup entirely
    if MTThread::any_thread_tracing() && MTThread::is_tracing() {
        MTThread::with_borrow_mut(|mtt| {
            // We ignore the return value as we can't really cancel tracing from this function.
            mtt.promote_u32(val);
        });
    }
    val
}

/// Promote a `usize` during trace recording.
#[unsafe(no_mangle)]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
pub extern "C" fn __yk_promote_c_long_long(val: c_longlong) -> c_longlong {
    // Fast path: if no thread is tracing, skip TLS lookup entirely
    if MTThread::any_thread_tracing() && MTThread::is_tracing() {
        MTThread::with_borrow_mut(|mtt| {
            // We ignore the return value as we can't really cancel tracing from this function.
            mtt.promote_i64(val);
        });
    }
    val
}

/// Promote a `usize` during trace recording.
#[unsafe(no_mangle)]
pub extern "C" fn __yk_promote_usize(val: usize) -> usize {
    // Fast path: if no thread is tracing, skip TLS lookup entirely
    if MTThread::any_thread_tracing() && MTThread::is_tracing() {
        MTThread::with_borrow_mut(|mtt| {
            // We ignore the return value as we can't really cancel tracing from this function.
            mtt.promote_usize(val);
        });
    }
    val
}

/// Promote a pointer during trace recording.
#[unsafe(no_mangle)]
pub extern "C" fn __yk_promote_ptr(val: *const c_void) -> *const c_void {
    // Fast path: if no thread is tracing, skip TLS lookup entirely
    if MTThread::any_thread_tracing() && MTThread::is_tracing() {
        MTThread::with_borrow_mut(|mtt| {
            // We ignore the return value as we can't really cancel tracing from this function.
            mtt.promote_usize(val as usize);
        });
    }
    val
}

/// Records a 64-bit return value of an idempotent function during trace recording.
#[unsafe(no_mangle)]
pub extern "C" fn __yk_idempotent_promote_i64(val: i64) -> i64 {
    // Fast path: if no thread is tracing, skip TLS lookup entirely
    if MTThread::any_thread_tracing() && MTThread::is_tracing() {
        MTThread::with_borrow_mut(|mtt| {
            // We ignore the return value as we can't really cancel tracing from this function.
            mtt.promote_i64(val);
        });
    }
    val
}

/// Records a 32-bit return value of an idempotent function during trace recording.
#[unsafe(no_mangle)]
pub extern "C" fn __yk_idempotent_promote_i32(val: i32) -> i32 {
    // Fast path: if no thread is tracing, skip TLS lookup entirely
    if MTThread::any_thread_tracing() && MTThread::is_tracing() {
        MTThread::with_borrow_mut(|mtt| {
            // We ignore the return value as we can't really cancel tracing from this function.
            mtt.promote_i32(val);
        });
    }
    val
}
