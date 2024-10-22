//! The main end-user interface to the meta-tracing system.

use dynasmrt::{dynasm, x64::Assembler, DynasmApi, DynasmLabelApi, ExecutableBuffer};
use std::{
    assert_matches::debug_assert_matches,
    cell::RefCell,
    cmp,
    collections::VecDeque,
    env,
    error::Error,
    ffi::c_void,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use parking_lot::{Condvar, Mutex, MutexGuard};
#[cfg(not(all(feature = "yk_testing", not(test))))]
use parking_lot_core::SpinWait;
use yksmp::LiveVar;
use yksmp;

use crate::{
    aotsmp::{load_aot_stackmaps, AOT_STACKMAPS},
    compile::{default_compiler, CompilationError, CompiledTrace, Compiler, GuardIdx},
    location::{HotLocation, HotLocationKind, Location, TraceFailed},
    log::{
        stats::{Stats, TimingState},
        Log, Verbosity,
    },
    trace::{default_tracer, AOTTraceIterator, TraceRecorder, Tracer},
};

// unoptimised - the original funcitons
const UNOPTIMISED_CONTROL_POINT_SMID: usize = 0;
// optimised - the cloned funcitons
const OPTIMISED_CONTROL_POINT_SMID: usize = 1;

// Do the same thing as we did with AOT_STACKMAPS - LazyLoad

fn switch_into_optimised_version() -> Result<ExecutableBuffer, Assembler> {
    let mut asm = dynasmrt::x64::Assembler::new().unwrap();
    
    // Get the size of needed for the funciton, then do something like:
    // dynasm!(asm;
    //     mov rsp, rbp
    //     pop rbp
    // );

    let (src_rec, src_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(UNOPTIMISED_CONTROL_POINT_SMID);
    let (dst_rec, dst_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(OPTIMISED_CONTROL_POINT_SMID);
    
    // save all the registers
    // NOTE: the order is important cause we can use the dwarfnumber
    // in the locations to get the registers
    dynasm!(asm;
        push r15;
        push r14;
        push r13;
        push r12;
        push r11;
        push r10;
        push r9;
        push r8;
        sub rsp, 16; // this is a span for rsp + rbp
        push rsi;
        push rdi;
        push rbx;
        push rcx;
        push rdx;
        push rax
    );

    // NOTE: from cloned to main
    dynasm!(asm;
        mov rsp, rbp
        pop rbp
    );



}

fn switch_into_unoptimised_version() -> Result<ExecutableBuffer, Assembler> {
    let mut asm = dynasmrt::x64::Assembler::new().unwrap();

    let (src_rec, src_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(OPTIMISED_CONTROL_POINT_SMID);
    let (dst_rec, dst_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(UNOPTIMISED_CONTROL_POINT_SMID);
    // save all the registers
    // NOTE: the order is important cause we can use the dwarfnumber
    // in the locations to get the registers
    dynasm!(asm;
            push r15;
            push r14;
            push r13;
            push r12;
            push r11;
            push r10;
            push r9;
            push r8;
            sub rsp, 16; // this is a span for rsp + rbp
            push rsi;
            push rdi;
            push rbx;
            push rcx;
            push rdx;
            push rax
        );

    // TODO: copy live variables over.
    for (index, src_var) in src_rec.live_vars.iter().enumerate() {
        let dst_var = dst_rec.live_vars[index];
        assert!(src_var.len() == 1);
        let src_location = src_var.get(0).unwrap();
        assert!(dst_var.len() == 1);
        let dst_location = dst_var.get(0).unwrap();

        // TODO: copy location from register to regist
        // Example: ykrt/src/compile/jitc_yk/codegen/x64/deopt.rs:
        // dynasm!(asm;
        //     push rax;
        //     push rbx;
        //     mov rax, [rbp]; // Load value from [rbp] into RAX
        //     mov rbx, [rbp - src_val_size]; // Load value from [rbp - offset] into RBX
        //     mov [max - 10], rbx; // Store RBX into [max - 10]
        //     pop rbx; // Restore RBX from the stack
        //     pop rax; // Restore RAX from the stack
        // );
        match src_location {
            yksmp::Location::Register(src_num, src_val_size, src_add_locs, src_add_loc_reg) => {
                // TODO: deal with extras as in deopt.rs
                assert!(*src_add_locs == 0, "deal with additional information");
                assert!(
                    src_add_loc_reg.len() == 0,
                    "deal with additional information"
                );
                match dst_location {
                    yksmp::Location::Register(
                        dst_num,
                        dst_val_size,
                        dst_add_locs,
                        dst_add_loc_reg,
                    ) => match dst_num {
                        0 => dynasm!(asm;
                            mov rax, QWORD [rsp - i32::try_from(src_num * 8).unwrap()]
                        ),
                        _ => todo!(),
                    },
                    _ => panic!("not implemented"),
                }
            }
            yksmp::Location::Direct(_, _, _) => {
                panic!("unimplemented");
            }
            yksmp::Location::Indirect(_, _, _) => {
                // NOTE: we need to calculate the size+ match
                panic!("unimplemented");
            }
            yksmp::Location::Constant(_) => {
                panic!("unimplemented");
            }
            yksmp::Location::LargeConstant(_) => {
                panic!("unimplemented");
            }
            _ => panic!(),
        }
    }

    // TODO: set the funciton call instruction size in asm
    let size_of_call = 0;
    let target_addr = dst_rec.offset + size_of_call;
    // TODO: jump to control point.
    dynasm!(asm; jmp target_addr as i32); // Cast to i32 if you're sure the address fits


    return asm.finalize();
}



