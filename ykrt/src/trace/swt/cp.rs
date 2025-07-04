use crate::aotsmp::AOT_STACKMAPS;
use crate::trace::swt::cfg::{dwarf_reg_to_str, CPTransitionDirection, ControlPointStackMapId};
use crate::trace::swt::cfg::{
    dwarf_to_dynasm_reg, reg_num_to_ykrt_control_point_rsp_offset, CP_BREAK,
    CP_VERBOSE, CP_VERBOSE_ASM, REG64_BYTESIZE, REG_OFFSETS,
};
use crate::trace::swt::debug::{debug_print_destination_live_vars, debug_print_source_live_vars};
use crate::trace::swt::live_vars::{copy_live_vars_to_temp_buffer, set_destination_live_vars};
use capstone::prelude::*;
use dynasmrt::{dynasm, x64::Assembler, DynasmApi, ExecutableBuffer};
use std::alloc::{dealloc, Layout};

use std::collections::HashMap;
use std::error::Error;
use std::ffi::c_void;

use crate::log::stats::Stats;

pub struct CPTransition {
    // The direction of the transition.
    pub direction: CPTransitionDirection,
    // The frame address of the caller.
    pub frameaddr: *const c_void,
    // The stack pointer of the caller.
    pub src_rsp: *const c_void,
    // The address of the trace to execute.
    pub trace_addr: *const c_void,
    // Flag to indicate whether to call __yk_exec_trace.
    pub exec_trace: bool,
}

pub(crate) unsafe fn swt_module_cp_transition(transition: CPTransition, stats: &Stats) {
    let frameaddr = transition.frameaddr as usize;
    let mut asm = Assembler::new().unwrap();

    let mut src_smid = ControlPointStackMapId::Opt;
    let mut dst_smid = ControlPointStackMapId::UnOpt;

    if transition.direction == CPTransitionDirection::UnoptToOpt {
        src_smid = ControlPointStackMapId::UnOpt;
        dst_smid = ControlPointStackMapId::Opt;
        stats.swt_transition_unopt_to_opt();
    } else {
        stats.swt_transition_opt_to_unopt();
    }

    let (src_rec, src_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(src_smid as usize);
    let (dst_rec, dst_pinfo) = AOT_STACKMAPS.as_ref().unwrap().get(dst_smid as usize);

    let mut src_frame_size: u64 = src_rec.size;
    if src_pinfo.hasfp {
        src_frame_size -= REG64_BYTESIZE;
    }
    let mut dst_frame_size: u64 = dst_rec.size;
    if dst_pinfo.hasfp {
        dst_frame_size -= REG64_BYTESIZE;
    }

    if *CP_BREAK {
        dynasm!(asm; .arch x64; int3);
    }

    // Set RBP and RSP
    dynasm!(asm
        ; .arch x64
        ; mov rbp, QWORD frameaddr as i64
        ; mov rsp, QWORD frameaddr as i64
        ; sub rsp, (dst_frame_size).try_into().unwrap() // adjust rsp
    );

    // Calculate the offset from the RBP to the RSP where __ykrt_control_point_real stored the registers.
    // Example: r15 address = rbp - rbp_offset_reg_store
    let rbp_offset_reg_store = src_frame_size as i64 + (14 * REG64_BYTESIZE) as i64;

    let temp_live_vars_buffer =
        copy_live_vars_to_temp_buffer(&mut asm, src_rec, transition.direction);
    if *CP_VERBOSE {
        println!(
            "Transition: {:?} ExecTrace: {:?}",
            transition.direction, transition.exec_trace
        );
        println!(
            "src_rbp: 0x{:x}, reg_store: 0x{:x}, src_frame_size: 0x{:x}, dst_frame_size: 0x{:x}, rbp_offset_reg_store: 0x{:x}",
            frameaddr as i64,
            frameaddr as i64 - rbp_offset_reg_store,
            src_frame_size,
            dst_frame_size,
            rbp_offset_reg_store
        );
    }

    // if *CP_VERBOSE {
    //     debug_print_source_live_vars(src_rec, rbp_offset_reg_store);
    // }
    // Set destination live vars
    let used_registers = set_destination_live_vars(
        &mut asm,
        src_rec,
        dst_rec,
        rbp_offset_reg_store,
        temp_live_vars_buffer.clone(),
    );

    assert_eq!(
        (frameaddr as i64 - dst_frame_size as i64) % 16,
        0,
        "RSP is not aligned to 16 bytes"
    );
    // Restore only unused registers.
    restore_registers(&mut asm, used_registers, rbp_offset_reg_store as i32);

    if *CP_VERBOSE {
        // Call debug_print_live_var_values from assembly
        dynasm!(asm
            ; .arch x64
            // Save caller-saved registers that might be used
            ; push rax
            ; push rcx
            ; push rdx
            ; push rsi
            ; push rdi
            ; push r8
            ; push r9
            ; push r10
            ; push r11

            // Set up arguments for debug_print_live_var_values(dst_rec, rbp_offset_reg_store)
            ; mov rdi, QWORD dst_rec as *const _ as i64  // First argument: dst_rec
            ; mov rsi, QWORD rbp_offset_reg_store        // Second argument: rbp_offset_reg_store

            // Call the function
            ; mov rax, QWORD debug_print_destination_live_vars as usize as i64
            ; call rax

            // Restore caller-saved registers
            ; pop r11
            ; pop r10
            ; pop r9
            ; pop r8
            ; pop rdi
            ; pop rsi
            ; pop rdx
            ; pop rcx
            ; pop rax
        );
    }
    if *CP_VERBOSE {
        println!(
            "transition.src_rsp: 0x{:x}, current_rsp: 0x{:x}",
            transition.src_rsp as i64,
            frameaddr as i64 - dst_frame_size as i64
        );
    }
    if transition.exec_trace {
        // FIXME: Why do we need this stack adjustment?
        // When we execute traces, we want to set the RSP to the same value as when 
        // the traces were collected (Unopt RSP). However, when we do that, it 
        // corrupts the stderr output of a few tests `idempotent.c` and `srem.c` 
        // which cause lang_tester to fail parsing the output with this error:
        // ```text
        // Can't convert stderr from 'YKD_SERIALISE_COMPILATION="1" "/tmp/.tmpOqHojX/idempotent"' into UTF-8
        // ```
        // These tests seems to work when RSP is set to the Opt RSP for some reason, 
        // but that's obviously wrong and it creates segfault in yklua. Adding 16 bytes 
        // to the stack fixes the issue.
        let trace_stack_adjustment = 2 * REG64_BYTESIZE; // 2 * 8 = 16 bytes
        
        dynasm!(asm
            ; .arch x64
            ; sub rsp, trace_stack_adjustment.try_into().unwrap()
            ; mov rdx, QWORD transition.trace_addr as i64
            ; jmp rdx
        );
    } else {
        let call_offset = calc_after_cp_offset(dst_rec.offset).unwrap();
        let dst_target_addr = i64::try_from(dst_rec.offset).unwrap() + call_offset;
        dynasm!(asm
            ; .arch x64
            // rsp is set to rbp - dst_frame_size
            // Allocate 16 bytes on the stack -
            // 0x0 - rax store
            // 0x8 - return address
            ; sub rsp, 0x10
            // Save the original rsp at 0x0
            ; mov [rsp], rax
            // Load the target address into rax at 0x8
            ; mov rax, QWORD dst_target_addr
            // Store the target address into 0x8
            ; mov [rsp + 0x8], rax
            // Restore the original rax
            ; pop rax
            // Load 8 bytes from rsp and jump to it
            ; ret
        );
    }
    // Execute the generated ASM code.
    let buffer = asm.finalize().unwrap();
    unsafe {
        execute_asm_buffer(buffer);
    }
}

/// Execute an assembled buffer with optional verbose assembly dumping
#[unsafe(no_mangle)]
unsafe fn execute_asm_buffer(buffer: ExecutableBuffer) {
    let func: unsafe fn() = std::mem::transmute(buffer.as_ptr());

    if *CP_VERBOSE_ASM {
        let cs = Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode64)
            .build()
            .unwrap();

        let instructions = cs
            .disasm_all(
                std::slice::from_raw_parts(
                    buffer.ptr(dynasmrt::AssemblyOffset(0)) as *const u8,
                    buffer.len(),
                ),
                0,
            )
            .unwrap();

        println!("ASM DUMP:");
        for i in instructions.iter() {
            println!(
                "  {:x}: {} {}",
                i.address(),
                i.mnemonic().unwrap(),
                i.op_str().unwrap()
            );
        }
    }

    func();
}

// Restores the registers from the rbp offset.
fn restore_registers(
    asm: &mut Assembler,
    exclude_registers: HashMap<u16, u16>,
    rbp_offset_reg_store: i32,
) {
    let mut sorted_offsets: Vec<(&u16, &i32)> = REG_OFFSETS.iter().collect();
    sorted_offsets.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending by value

    for (dwarf_reg_num, _) in sorted_offsets.iter() {
        if !exclude_registers.contains_key(dwarf_reg_num) {
            if *CP_VERBOSE {
                eprintln!(
                    "Restoring unused register to __ykrt_control_point save point: {:?}",
                    dwarf_reg_to_str(**dwarf_reg_num as u8)
                );
            }
            restore_register(
                asm,
                (**dwarf_reg_num).try_into().unwrap(),
                rbp_offset_reg_store,
            );
        }
    }
}

fn restore_register(asm: &mut Assembler, dwarf_reg_num: u16, rbp_offset_reg_store: i32) {
    let reg_offset = reg_num_to_ykrt_control_point_rsp_offset(dwarf_reg_num);
    let reg_val_rbp_offset = i32::try_from(rbp_offset_reg_store - reg_offset).unwrap();
    let dynasm_reg = dwarf_to_dynasm_reg(dwarf_reg_num.try_into().unwrap());
    dynasm!(asm
        ; mov Rq(dynasm_reg), QWORD [rbp - reg_val_rbp_offset]
    );
}

// Calculates the offset of the call instruction after the control point.
// Example:
//  CP Record offset points to 0x00000000002023a4, we want to find the
//  instruction at 0x00000000002023b1.
//  0x00000000002023a4 <+308>:	movabs $0x202620,%r11
//  0x00000000002023ae <+318>:	call   *%r11
//  0x00000000002023b1 <+321>:	jmp    0x2023b3 <main+323>
fn calc_after_cp_offset(rec_offset: u64) -> Result<i64, Box<dyn Error>> {
    // Define the maximum number of bytes to disassemble
    const MAX_CODE_SIZE: usize = 64;
    // Read the machine code starting at rec_offset
    let code_slice = unsafe { std::slice::from_raw_parts(rec_offset as *const u8, MAX_CODE_SIZE) };
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()
        .unwrap();
    // Disassemble the code
    let instructions = cs.disasm_all(code_slice, rec_offset as u64).unwrap();
    // Initialize the offset accumulator
    let mut offset: i64 = 0;
    for inst in instructions.iter() {
        offset += inst.bytes().len() as i64;

        if inst.mnemonic().unwrap_or("") == "call" {
            // Check if this is a dummy trace call by examining the preceding instructions
            // or by checking if the target is __yk_trace_basicblock_dummy
            if let Some(op_str) = inst.op_str() {
                if op_str.contains("__yk_trace_basicblock_dummy") {
                    // Skip this dummy call - continue to find the actual control point call
                    continue;
                }
            }
            return Ok(offset);
        }
    }

    Err(format!(
        "Call instruction not found within the code slice: {}, len:{}",
        rec_offset, MAX_CODE_SIZE
    )
    .into())
}

#[cfg(test)]
mod swt_cp_tests {
    use super::*;
    use dynasmrt::{dynasm, x64::Assembler};
    use std::error::Error;

    fn get_asm_instructions(buffer: &dynasmrt::ExecutableBuffer) -> Vec<String> {
        if buffer.len() == 0 {
            return vec![];
        }
        let code_ptr = buffer.ptr(dynasmrt::AssemblyOffset(0)) as *const u8;
        let code_size = buffer.len();
        // Use Capstone to disassemble and check the generated instructions
        let capstone = Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode64)
            .build()
            .unwrap();

        let instructions = capstone
            .disasm_all(
                unsafe { std::slice::from_raw_parts(code_ptr, code_size) },
                code_ptr as u64,
            )
            .expect("Failed to disassemble code");

        return instructions
            .iter()
            .map(|inst| {
                format!(
                    "{} {}",
                    inst.mnemonic().unwrap_or(""),
                    inst.op_str().unwrap_or("")
                )
            })
            .collect();
    }

    #[test]
    fn test_restore_registers_rbx() {
        let mut asm = Assembler::new().unwrap();
        let mut used_regs = HashMap::new();
        used_regs.insert(0, 8);
        // used_regs.insert(1, 8); // not used:
        used_regs.insert(2, 8);
        // used_regs.insert(3, 8); // used
        used_regs.insert(4, 8);
        used_regs.insert(5, 8);
        // used_regs.insert(6, 8); // not used:
        // used_regs.insert(7, 8); // not used:
        used_regs.insert(8, 8);
        used_regs.insert(9, 8);
        used_regs.insert(10, 8);
        used_regs.insert(11, 8);
        used_regs.insert(12, 8);
        used_regs.insert(13, 8);
        used_regs.insert(14, 8);
        used_regs.insert(15, 8);

        restore_registers(&mut asm, used_regs, 0);
        let buffer: dynasmrt::ExecutableBuffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0], "mov rbx, qword ptr [rbp + 0x50]");
    }

    #[test]
    fn test_restore_registers_no_instructions() {
        let mut asm = Assembler::new().unwrap();
        let mut used_regs = HashMap::new();
        used_regs.insert(0, 8);
        // used_regs.insert(1, 8); // not used:
        used_regs.insert(2, 8);
        used_regs.insert(3, 8);
        used_regs.insert(4, 8);
        used_regs.insert(5, 8);
        // used_regs.insert(6, 8); // not used:
        // used_regs.insert(7, 8); // not used:
        used_regs.insert(8, 8);
        used_regs.insert(9, 8);
        used_regs.insert(10, 8);
        used_regs.insert(11, 8);
        used_regs.insert(12, 8);
        used_regs.insert(13, 8);
        used_regs.insert(14, 8);
        used_regs.insert(15, 8);

        restore_registers(&mut asm, used_regs, 0);
        let buffer: dynasmrt::ExecutableBuffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions.len(), 0);
    }

    #[test]
    fn test_restore_registers_partial() {
        let mut asm = Assembler::new().unwrap();
        let mut used_regs = HashMap::new();
        used_regs.insert(0, 8);
        // used_regs.insert(1, 8); // not used:
        used_regs.insert(2, 8);
        used_regs.insert(3, 8);
        used_regs.insert(4, 8);
        used_regs.insert(5, 8);
        // used_regs.insert(6, 8); // not used
        // used_regs.insert(7, 8); // not used
        used_regs.insert(8, 8);
        used_regs.insert(9, 8);
        // used_regs.insert(10, 8); // not used
        used_regs.insert(11, 8);
        used_regs.insert(12, 8);
        used_regs.insert(13, 8);
        // used_regs.insert(14, 8); // not used
        used_regs.insert(15, 8);

        restore_registers(&mut asm, used_regs, 0);
        let buffer: dynasmrt::ExecutableBuffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions[0], "mov r10, qword ptr [rbp + 0x28]");
        assert_eq!(instructions[1], "mov r14, qword ptr [rbp + 8]");
    }

    #[test]
    fn test_restore_registers_empty_restore() {
        let mut asm = Assembler::new().unwrap();
        let used_regs = HashMap::new();
        restore_registers(&mut asm, used_regs, 0);
        let buffer: dynasmrt::ExecutableBuffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions[0], "mov rax, qword ptr [rbp + 0x60]");
        assert_eq!(instructions[1], "mov rcx, qword ptr [rbp + 0x58]");
        assert_eq!(instructions[2], "mov rbx, qword ptr [rbp + 0x50]");
        assert_eq!(instructions[3], "mov rdi, qword ptr [rbp + 0x48]");
        assert_eq!(instructions[4], "mov rsi, qword ptr [rbp + 0x40]");
        assert_eq!(instructions[5], "mov r8, qword ptr [rbp + 0x38]");
        assert_eq!(instructions[6], "mov r9, qword ptr [rbp + 0x30]");
        assert_eq!(instructions[7], "mov r10, qword ptr [rbp + 0x28]");
        assert_eq!(instructions[8], "mov r11, qword ptr [rbp + 0x20]");
        assert_eq!(instructions[9], "mov r12, qword ptr [rbp + 0x18]");
        assert_eq!(instructions[10], "mov r13, qword ptr [rbp + 0x10]");
        assert_eq!(instructions[11], "mov r14, qword ptr [rbp + 8]");
        assert_eq!(instructions[12], "mov r15, qword ptr [rbp]");
    }

    #[test]
    fn test_calc_after_cp_offset_with_call_instruction() -> Result<(), Box<dyn Error>> {
        // Arrange: Create a buffer with a call instruction
        let mut asm = Assembler::new().unwrap();
        let call_addr: i32 = 0x666;
        dynasm!(asm
            ; .arch x64
            ; nop
            ; call call_addr
            ; ret
        );
        let buffer = asm.finalize().unwrap();
        let code_ptr = buffer.ptr(dynasmrt::AssemblyOffset(0)) as u64;
        let offset = calc_after_cp_offset(code_ptr)?;
        assert_eq!(offset, 6, "The call offset should be 6 bytes");
        Ok(())
    }

    #[test]
    fn test_calc_after_cp_offset_with_movabs_and_nops() -> Result<(), Box<dyn Error>> {
        // Arrange: Create a buffer with movabs, multiple nops, and call instruction
        let mut asm = Assembler::new().unwrap();
        dynasm!(asm
            ; .arch x64
            ; nop                         // 1 byte
            ; mov r11, 0x202620           // 10 bytes
            ; call r11                    // 2 bytes
            ; ret                         // 1 byte
        );
        let buffer = asm.finalize().unwrap();
        let code_ptr = buffer.ptr(dynasmrt::AssemblyOffset(0)) as u64;
        let offset = calc_after_cp_offset(code_ptr)?;
        assert_eq!(offset, 11, "The call offset should be 11 bytes");
        Ok(())
    }
}
