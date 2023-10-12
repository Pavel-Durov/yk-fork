
use std::os::raw::{c_int, c_void};

use std::{os::raw::{c_void, c_int}, ffi::CString, ptr::null_mut};

use libc::{dlsym, malloc, pthread_create, pthread_attr_t, pthread_t};

const SHADOW_STACK_SIZE: usize = 1000000;

// fn thread_entry(arg: *mut c_void) -> *mut c_void {
//     let str = CString::new("shadowstack_0").unwrap();
//     let result = unsafe {
//         let addr = dlsym(null_mut(), str.as_ptr() as *const i8);
//         println!("@@@@@@ DLSYM add {:?}", addr);
//         let malloc_address = malloc(SHADOW_STACK_SIZE);
//         println!("@@@@@@ malloc_address {:?}", malloc_address);
//         *(addr as *mut *mut c_void) = malloc_address;
//         let result: i32 = pthread_create(thread, attr, start_routine, arg);
//         println!("@@@@@@ pthread_create result: {:?}", result);
//         result
//     };
//     return result;
// }

#[no_mangle]
pub extern "C" fn __wrap_pthread_create(
    thread: *mut pthread_t,
    attr: *const pthread_attr_t,
    start_routine: extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    arg: *mut libc::c_void,
) -> c_int {
    let str = CString::new("shadowstack_0").unwrap();

    let result = unsafe {
        let addr = dlsym(null_mut(), str.as_ptr() as *const i8);
        println!("@@@@@@ DLSYM add {:?}", addr);
        let malloc_address = malloc(SHADOW_STACK_SIZE);
        println!("@@@@@@ malloc_address {:?}", malloc_address);
        *(addr as *mut *mut c_void) = malloc_address;
        let result: i32 = pthread_create(thread, attr, start_routine, arg);
        println!("@@@@@@ pthread_create result: {:?}", result);
        println!("@@@@@@ addr: {:?}", addr);
        result
    };

    println!("Running code after calling the real pthread_create...");

    result
}