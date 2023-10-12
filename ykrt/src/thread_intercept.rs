
use std::os::raw::{c_int, c_void};

use std::{ffi::CString, ptr::null_mut};

use libc::{dlsym, malloc, pthread_create, pthread_attr_t, pthread_t, free};

const SHADOW_STACK_SIZE: usize = 1000000;

// pthread functions from the system's C library.
extern {
    fn pthread_cleanup_push(routine: extern "C" fn(arg: *mut c_void), arg: *mut c_void);
    fn pthread_cleanup_pop(execute: c_int);
}

struct ThreadData {
    pub arg: *mut c_void,
    pub original_func: extern "C" fn(*mut c_void) -> *mut c_void,
}

impl ThreadData {
    fn new(arg: *mut c_void, original_func: extern "C" fn(*mut c_void) -> *mut c_void) -> Self {
        ThreadData { arg, original_func }
    }
}


extern "C" fn deallocate_stack(stack_ptr: *mut c_void) {
    println!("[YK] deallocate_stack {:?}", stack_ptr);
    unsafe {
        free(stack_ptr);
    }
}

extern "C" fn __start_routine(arg: *mut c_void) -> *mut c_void {
    let str = CString::new("shadowstack_0").unwrap();
    unsafe {
        let data = arg as *mut ThreadData;
        let addr = dlsym(null_mut(), str.as_ptr() as *const i8);
        let malloc_address = malloc(SHADOW_STACK_SIZE);
        
        // pthread_cleanup_push(deallocate_stack, malloc_address as *mut c_void);

        *(addr as *mut *mut c_void) = malloc_address;
        
        let result = (data.as_ref().unwrap().original_func)(data.as_ref().unwrap().arg);
        
        // pthread_cleanup_pop(1);
        free(malloc_address);
        result
    }
}


#[no_mangle]
pub extern "C" fn __wrap_pthread_create(
    thread: *mut pthread_t,
    attr: *const pthread_attr_t,
    start_routine: extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    arg: *mut libc::c_void,
) -> c_int {
    let result = unsafe {
        let data_to_pass_to_thread = ThreadData::new(arg, start_routine);
        pthread_create(
            thread,
            attr,
            __start_routine as extern "C" fn(*mut c_void) -> *mut c_void,
            &data_to_pass_to_thread as *const ThreadData as *mut c_void,
        )
    };

    println!("Running code after calling the real pthread_create...");

    result
}