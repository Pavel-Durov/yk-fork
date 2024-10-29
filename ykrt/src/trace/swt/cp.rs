//! The main end-user interface to the meta-tracing system.

use dynasmrt::{dynasm, x64::Assembler, DynasmApi, DynasmLabelApi, ExecutableBuffer};
use std::{assert_matches::debug_assert_matches, cell::RefCell, sync::Arc};

use parking_lot::{Condvar, Mutex, MutexGuard};

use crate::aotsmp::AOT_STACKMAPS;
use yksmp::Location::{Constant, Direct, Indirect, LargeConstant, Register};

use dynasmrt::x64::Rq;
use dynasmrt::x64::Rq::{RAX, RBP, RCX, RDI, RDX, RSI, RSP};

use std::sync::LazyLock;

// unoptimised - the original funcitons
const UNOPTIMISED_CONTROL_POINT_SMID: usize = 0;
// optimised - the cloned funcitons
const OPTIMISED_CONTROL_POINT_SMID: usize = 1;

// Do the same thing as we did with AOT_STACKMAPS - LazyLoad

// fn switch_into_optimised_version() -> Result<ExecutableBuffer, Assembler<X64Relocation>> {
//     let mut asm = dynasmrt::x64::Assembler::new().unwrap();

//     // Get the size of needed for the funciton, then do something like:
//     // dynasm!(asm;
//     //     mov rsp, rbp
//     //     pop rbp
//     // );

//     let (src_rec, src_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(UNOPTIMISED_CONTROL_POINT_SMID);
//     let (dst_rec, dst_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(OPTIMISED_CONTROL_POINT_SMID);

//     // save all the registers
//     // NOTE: the order is important cause we can use the dwarfnumber
//     // in the locations to get the registers
//     dynasm!(asm;
//         push r15;
//         push r14;
//         push r13;
//         push r12;
//         push r11;
//         push r10;
//         push r9;
//         push r8;
//         sub rsp, 16; // this is a span for rsp + rbp
//         push rsi;
//         push rdi;
//         push rbx;
//         push rcx;
//         push rdx;
//         push rax
//     );

//     // NOTE: from cloned to main
//     dynasm!(asm;
//         mov rsp, rbp
//         pop rbpR
// }


pub(crate) fn jump_into_unoptimised_version() {
    // build
    let exec_buffer = SWITCH_INTO_OPTIMISED_VERSION.as_ref();
    // exec
    let code_ptr = exec_buffer.as_ptr();
    println!("Executable buffer starts at: {:?}", code_ptr);
    type JitFunction = unsafe extern "C" fn();
    let func: unsafe fn() = unsafe { std::mem::transmute(code_ptr) };

    // Execute the code
    unsafe {
        func();
    }
}

pub(crate) static SWITCH_INTO_OPTIMISED_VERSION: LazyLock<Arc<ExecutableBuffer>> =
    LazyLock::new(|| {
        let asm_bytes = build_asm_jump_into_unoptimised_version();
        Arc::new(asm_bytes)
    });

fn reg_num_to_dynasm_reg(dwarf_reg_num: u16) -> Rq {
    match dwarf_reg_num {
        0 => Rq::RAX,
        1 => Rq::RDX,
        2 => Rq::RCX,
        3 => Rq::RBX,
        4 => Rq::RSI,
        5 => Rq::RDI,
        6 => Rq::RBP,
        7 => Rq::RSP,
        8 => Rq::R8,
        9 => Rq::R9,
        10 => Rq::R10,
        11 => Rq::R11,
        12 => Rq::R12,
        13 => Rq::R13,
        14 => Rq::R14,
        15 => Rq::R15,
        _ => panic!("Unsupported register"),
    }
}

fn reg_num_stack_offset(dwarf_reg_num: u16) -> i32 {
    match dwarf_reg_num {
        0 => 0,    // rax
        1 => 8,    // rdx
        2 => 16,   // rcx
        3 => 24,   // rbx
        4 => 40,   // rsi
        5 => 32,   // rdi
        8 => 64,   // r8
        9 => 72,   // r9
        10 => 80,  // r10
        11 => 88,  // r11
        12 => 96,  // r12
        13 => 104, // r13
        14 => 112, // r14
        15 => 120, // r15
        _ => panic!("Unsupported register"),
    }
}

fn build_asm_jump_into_unoptimised_version() -> ExecutableBuffer {
    let mut asm = dynasmrt::x64::Assembler::new().unwrap();

    let (src_rec, _) = AOT_STACKMAPS
        .as_ref()
        .unwrap()
        .get(OPTIMISED_CONTROL_POINT_SMID);

    let (dst_rec, _) = AOT_STACKMAPS
        .as_ref()
        .unwrap()
        .get(UNOPTIMISED_CONTROL_POINT_SMID);

    // Save all the registers to the stack
    dynasm!(asm
        ; .arch x64
        ; push r15    // offset 120
        ; push r14    // offset 112
        ; push r13    // offset 104
        ; push r12    // offset 96
        ; push r11    // offset 88
        ; push r10    // offset 80
        ; push r9     // offset 72
        ; push r8     // offset 64
        ; sub rsp, 16 // Allocates 16 bytes of padding for rsp and rbp
        ; push rsi    // offset 40
        ; push rdi    // offset 32
        ; push rbx    // offset 24
        ; push rcx    // offset 16
        ; push rdx    // offset 8
        ; push rax    // offset 0
    );

     for (index, src_var) in src_rec.live_vars.iter().enumerate() {
        let dst_var = &dst_rec.live_vars[index];
        if src_var.len() > 1 || dst_var.len() > 1 {
            todo!("Deal with multi register locations");
        }

        let src_location = &src_var.get(0).unwrap();
        let dst_location = &dst_var.get(0).unwrap();

        println!("@@ src {:?} dst {:?}", src_location, dst_location);
        // copy live vars
        match (src_location, dst_location) {
            // Src Register
            (
                Register(src_num, src_val_size, src_add_locs, _src_add_loc_reg),
                Register(dst_num, _dst_val_size, dst_add_locs, _dst_add_loc_reg),
            ) => {
                assert!(*src_add_locs == 0, "deal with additional info");
                assert!(*dst_add_locs == 0, "deal with additional info");

                // let offset = reg_num_stack_offset(*src_num);
                let offset = i32::try_from(src_num * 8).unwrap();
                let dest_reg = u8::try_from(*dst_num).unwrap();
                println!("@@ offset {} reg {}", offset, dest_reg);
                match *src_val_size {
                    1 => todo!(),
                    2 => todo!(),
                    4 => todo!(),
                    8 => {
                        dynasm!(asm
                            ; mov Rq(dest_reg), QWORD [rsp - offset]
                        )
                    },
                    _ => todo!()
                }
            }
            (
                Register(_src_num, _src_val_size, _src_add_locs, _src_add_loc_reg),
                Direct(_dst_reg_num, _dst_off, _dst_val_size),
            ) => {
                // panic!("direct to register is not expceted and not implemented");
            }
            (
                Register(src_num, src_val_size, src_add_locs, _src_add_loc_reg),
                Indirect(dst_reg_num, dst_off, dst_val_size),
            ) => {
                // TODO: implement
            }
            (
                Register(_src_num, _src_val_size, _src_add_locs, _src_add_loc_reg),
                Constant(_val),
            ) => {
                // TODO: Is that a valid case?
                panic!("Constant to register is not expected");
            }
            (
                Register(_src_num, _src_val_size, _src_add_locs, _src_add_loc_reg),
                LargeConstant(_val),
            ) => {
                panic!("Large constant to register is not expected");
            }

            // src Direct
            (
                Direct(_src_reg_num, _src_off, _src_val_size),
                Register(_dst_num, _dst_val_size, _dst_add_locs, _dst_add_loc_reg),
            ) => {
                todo!("implementation")
            }
            (
                Direct(src_reg_num, src_off, src_val_size),
                Direct(dst_reg_num, dst_off, dst_val_size),
            ) => {
                assert_eq!(
                    *src_val_size, *dst_val_size,
                    "Source and destination value sizes do not match"
                );

                let src_reg = u8::try_from(*src_reg_num).unwrap();
                let dst_reg = u8::try_from(*dst_reg_num).unwrap();

                match *src_val_size {
                    1 => dynasm!(asm; mov al, BYTE [Rq(src_reg) + *src_off]),
                    2 => dynasm!(asm; mov ax, WORD [Rq(src_reg) + *src_off]),
                    4 => dynasm!(asm; mov eax, DWORD [Rq(src_reg) + *src_off]),
                    8 => dynasm!(asm; mov rax, QWORD [Rq(src_reg) + *src_off]),
                    _ => panic!("Unsupported source value size: {}", src_val_size),
                }
                // Store the value from RAX into the destination memory location
                match *dst_val_size {
                    1 => dynasm!(asm; mov BYTE [Rq(dst_reg) + *dst_off], al),
                    2 => dynasm!(asm; mov WORD [Rq(dst_reg) + *dst_off], ax),
                    4 => dynasm!(asm; mov DWORD [Rq(dst_reg) + *dst_off], eax),
                    8 => dynasm!(asm; mov QWORD [Rq(dst_reg) + *dst_off], rax),
                    _ => panic!("Unsupported destination value size: {}", dst_val_size),
                }
            }

            (
                Direct(_src_reg_num, _src_off, _src_val_size),
                Indirect(_dst_reg_num, _dst_off, _dst_add_loc_reg),
            ) => {
                // todo!("implementation")
            }
            (Direct(_src_reg_num, _src_off, _src_val_size), Constant(_val)) => {
                panic!("Direct to constant is not expected");
            }
            (Direct(_src_reg_num, _src_off, _src_val_size), LargeConstant(_val)) => {
                panic!("Direct to large constant is not expected");
            }
            // src Indirect
            (
                Indirect(_src_reg_num, _src_off, _src_add_loc_reg),
                Register(_dst_num, _dst_val_size, _dst_add_locs, _dst_add_loc_reg),
            ) => {
                // todo!("implementation")
            }
            (
                Indirect(_src_reg_num, _src_off, _src_add_loc_reg),
                Direct(_dst_reg_num, _dst_off, _dst_val_size),
            ) => {
                // todo!("implementation")
            }
            (
                Indirect(_src_reg_num, _src_off, _src_add_loc_reg),
                Indirect(_dst_reg_num, _dst_off, _dst_val_size),
            ) => {
                // todo!("implementation")
            }
            (Indirect(_src_reg_num, _src_off, _src_add_loc_reg), Constant(_dst_val)) => {
                // todo!("implementation")
            }
            (Indirect(src_reg_num, src_off, src_add_loc_reg), LargeConstant(_dst_val)) => {
                // todo!("implementation")
            }
            // src Constant
            (
                Constant(_val),
                Register(_dst_num, _dst_val_size, _dst_add_locs, _dst_add_loc_reg),
            ) => {
                // todo!("implementation")
            }
            (Constant(_val), Direct(_dst_reg_num, _dst_off, _dst_val_size)) => {
                // todo!("implementation")
            }
            (Constant(_val), Indirect(_dst_reg_num, _dst_off, _dst_val_size)) => {
                // todo!("implementation")
            }
            (Constant(_src_val), Constant(_dst_val)) => {
                // todo!("implementation")
            }
            (Constant(_src_val), LargeConstant(_dst_val)) => {
                // todo!("implementation")
            }
            // src LargeConstant
            (
                LargeConstant(_val),
                Register(_dst_num, _dst_val_size, _dst_add_locs, _dst_add_loc_reg),
            ) => {
                // todo!("implementation")
            }
            (LargeConstant(_val), Direct(_dst_reg_num, _dst_off, _dst_val_size)) => {
                // todo!("implementation")
            }
            (LargeConstant(_val), Indirect(_dst_reg_num, _dst_off, _dst_val_size)) => {
                // todo!("implementation")
            }
            (LargeConstant(_src_val), Constant(_dst_val)) => {
                // todo!("implementation")
            }
            (LargeConstant(_src_val), LargeConstant(_dst_val)) => {
                // todo!("implementation")
            }
        }
    }


    // let function_label = asm.new_dynamic_label();
    // // Emit the call instruction (replace <function_label> with your actual label or function)
    // let call_start_offset = asm.offset();
    // dynasm!(asm
    //     ; call =>function_label
    // );
    // // Record the offset after emitting the call instruction
    // let call_end_offset = asm.offset();
    // // Calculate the size of the call instruction
    // let size_of_call = (call_end_offset.0 - call_start_offset.0) as i32;
    // println!("@@@@ size_of_call {:?}", size_of_call);

    let size_of_call = 5;
    let target_addr = dst_rec.offset + size_of_call;
    // TODO: jump to control point.
    dynasm!(asm; jmp target_addr as i32); // Cast to i32 if you're sure the address fits

    // Assembly code to restore registers
    // dynasm!(asm
    //     // Restore registers by popping them in reverse order
    //     ; pop rax     // Corresponds to push rax
    //     ; pop rdx     // Corresponds to push rdx
    //     ; pop rcx     // Corresponds to push rcx
    //     ; pop rbx     // Corresponds to push rbx
    //     ; pop rdi     // Corresponds to push rdi
    //     ; pop rsi     // Corresponds to push rsi
    //     ; add rsp, 16 // Reverse sub rsp, 16
    //     ; pop r8      // Corresponds to push r8
    //     ; pop r9      // Corresponds to push r9
    //     ; pop r10     // Corresponds to push r10
    //     ; pop r11     // Corresponds to push r11
    //     ; pop r12     // Corresponds to push r12
    //     ; pop r13     // Corresponds to push r13
    //     ; pop r14     // Corresponds to push r14
    //     ; pop r15     // Corresponds to push r15
    //     ; ret
    // );

    asm.finalize().unwrap()
}
