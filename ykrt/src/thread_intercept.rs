use std::os::raw::{c_void, c_int};

#[no_mangle]
pub extern "C" fn __wrap_pthread_create(
    _: *mut c_void,
    _: *const c_void,
    _: extern "C" fn(*mut c_void) -> *mut c_void,
    _: *mut c_void,
) -> c_int {
    println!("Intercepted pthread_create!");
    panic!("Panic on thread!");
}
