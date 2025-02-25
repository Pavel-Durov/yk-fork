use crate::aotsmp::AOT_STACKMAPS;
use capstone::prelude::*;
use dynasmrt::{dynasm, x64::Assembler, DynasmApi};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::c_void;
use std::sync::LazyLock;
use yksmp::Location::{Constant, Direct, Indirect, LargeConstant, Register};
use yksmp::Record;
/// The size of a 64-bit register in bytes.
pub(crate) static REG64_BYTESIZE: u64 = 8;

// Flag for verbose logging
pub static CP_VERBOSE: LazyLock<bool> = LazyLock::new(|| {
    env::var("CP_VERBOSE")
        .map(|v| v == "1")
        .unwrap_or(false)
});

// Flag for asm breakpoints
pub static CP_BREAK: LazyLock<bool> = LazyLock::new(|| {
    env::var("CP_BREAK")
    .map(|v| v == "1")
    .unwrap_or(false)
});

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlPointStackMapId {
    // unoptimised (original functions) control point stack map id
    Opt = 0,
    // optimised (cloned functions) control point stack map id
    UnOpt = 1,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPTransitionDirection {
    UnoptToOpt = 0,
    OptToUnopt = 1,
}

pub struct CPTransition {
    // The direction of the transition.
    pub direction: CPTransitionDirection,
    // The frame address of the caller.
    pub frameaddr: *const c_void,
    // The stack pointer of the caller.
    pub rsp: *const c_void,
    // The address of the trace to execute.
    pub trace_addr: *const c_void,
    // Flag to indicate whether to call __yk_exec_trace.
    pub exec_trace: bool,
    // The function pointer to __yk_exec_trace.
    pub exec_trace_fn: ExecTraceFn,
}

pub(crate) type ExecTraceFn = unsafe extern "C" fn(
    frameaddr: *const c_void,
    rsp: *const c_void,
    trace_addr: *const c_void,
) -> !;

// We use the registers saved by the control point.
// __ykrt_control_point:
// "push rax",
// "push rcx",
// "push rbx",
// "push rdi",
// "push rsi",
// "push r8",
// "push r9",
// "push r10",
// "push r11",
// "push r12",
// "push r13",
// "push r14",
// "push r15",
static REG_OFFSETS: LazyLock<HashMap<u16, i32>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(0, 0x60); // rax
                       // 1 => 8,  // rdx - is not saved
    m.insert(2, 0x58); // rcx
    m.insert(3, 0x50); // rbx
                       // Question: why rsi and rdi are not at their index?
    m.insert(5, 0x48); // rdi
    m.insert(4, 0x40); // rsi
                       // 6 => 0x48 - not saved
                       // 7 => 0x40 - not saved
    m.insert(8, 0x38); // r8
    m.insert(9, 0x30); // r9
    m.insert(10, 0x28); // r10
    m.insert(11, 0x20); // r11
    m.insert(12, 0x18); // r12
    m.insert(13, 0x10); // r13
    m.insert(14, 0x8); // r14
    m.insert(15, 0x0); // r15
    m
});

// Maps DWARF register numbers to `dynasm` register numbers.
// This function takes a DWARF register number as input and returns the
// corresponding dynasm register number1. The mapping is based on the
// x86_64 architecture, and it's important to note that some registers
// (rsi, rdi, rbp, and rsp) have a slightly different order in dynasm
// compared to their sequential DWARF numbering.
// https://docs.rs/dynasmrt/latest/dynasmrt/x64/enum.Rq.html
fn dwarf_to_dynasm_reg(dwarf_reg_num: u8) -> u8 {
    match dwarf_reg_num {
        0 => 0,   // RAX
        2 => 1,   // RCX
        1 => 2,   // RDX
        3 => 3,   // RBX
        7 => 4,   // RSP
        6 => 5,   // RBP
        4 => 6,   // RSI
        5 => 7,   // RDI
        8 => 8,   // R8
        9 => 9,   // R9
        10 => 10, // R10
        11 => 11, // R11
        12 => 12, // R12
        13 => 13, // R13
        14 => 14, // R14
        15 => 15, // R15
        _ => panic!("Unsupported DWARF register number: {}", dwarf_reg_num),
    }
}

fn reg_num_to_ykrt_control_point_rsp_offset(dwarf_reg_num: u16) -> i32 {
    REG_OFFSETS
        .get(&dwarf_reg_num)
        .copied()
        .unwrap_or_else(|| panic!("Unsupported register {}", dwarf_reg_num))
}

fn calculate_live_vars_buffer_size(src_rec: &Record) -> i32 {
    let mut src_val_buffer_size: i32 = 0;
    for (_, src_var) in src_rec.live_vars.iter().enumerate() {
        match src_var.get(0).unwrap() {
            Indirect(_, _, src_val_size) => {
                src_val_buffer_size += *src_val_size as i32;
            }
            _ => { /* DO NOTHING */ }
        }
    }
    src_val_buffer_size
}

fn copy_live_vars_to_temp_buffer(asm: &mut Assembler, src_rec: &Record) {
    let mut src_var_indirect_index = 0;
    for (_, src_var) in src_rec.live_vars.iter().enumerate() {
        let src_location = src_var.get(0).unwrap();
        match src_location {
            Direct(_, _, _) => {
                // DO NOTHING
            }
            Indirect(_, src_off, src_val_size) => {
                assert!(
                    *src_val_size == 8,
                    "Only 8-byte Indirect values supported in this example"
                );
                let temp_buffer_offset = (src_var_indirect_index * (*src_val_size as i32)) as i32;
                dynasm!(asm
                    ; mov rax, QWORD [rbp + i32::try_from(*src_off).unwrap()]
                    ; mov QWORD [rsp + temp_buffer_offset], rax // This causes collision with saved registers by __ykrt_control_point
                );
                src_var_indirect_index += 1;
            }
            Register(_reg_num, _val_size, _add_locs) => {
                // DO NOTHING
            }
            _ => panic!(
                "Unsupported source location in temporary copy: {:?}",
                src_location
            ),
        }
    }
}

fn set_destination_live_vars(
    asm: &mut Assembler,
    src_rec: &Record,
    dst_rec: &Record,
    rbp_offset_reg_store: i64,
) -> HashMap<u16, u16> {
    let mut dest_reg_nums = HashMap::new();
    let mut src_var_indirect_index = 0;
    for (index, src_var) in src_rec.live_vars.iter().enumerate() {
        let dst_var = &dst_rec.live_vars[index];
        if src_var.len() > 1 || dst_var.len() > 1 {
            todo!("Deal with multi register locations");
        }
        assert!(
            src_rec.live_vars.len() == dst_rec.live_vars.len(),
            "Expected single register location, got src: {} and dst: {}",
            src_rec.live_vars.len(),
            dst_rec.live_vars.len()
        );

        let src_location = &src_var.get(0).unwrap();
        let dst_location = &dst_var.get(0).unwrap();

        match src_location {
            Register(src_reg_num, src_val_size, src_add_locs) => {
                let src_reg_offset = reg_num_to_ykrt_control_point_rsp_offset(*src_reg_num);
                let src_reg_val_rbp_offset = i32::try_from(
                    rbp_offset_reg_store - src_reg_offset as i64,
                )
                .unwrap();

                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        assert!(
                            src_add_locs.len() == 0 && dst_add_locs.len() == 0,
                            "Register2Register - deal with additional info"
                        );
                        assert!(
                            dst_val_size == src_val_size,
                            "Register2Register - src and dst val size must match. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                        // skip copying to the same register with the same value size
                        if src_reg_num == dst_reg_num && src_val_size == dst_val_size {
                            continue;
                        }
                        if *CP_VERBOSE {
                            println!(
                                "Register2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                        match *src_val_size {
                            1 => {
                                dynasm!(asm; mov Rb(dst_reg), BYTE [rbp - src_reg_val_rbp_offset])
                            }
                            2 => {
                                dynasm!(asm; mov Rw(dst_reg), WORD [rbp - src_reg_val_rbp_offset])
                            }
                            4 => {
                                dynasm!(asm; mov Rd(dst_reg), DWORD [rbp - src_reg_val_rbp_offset])
                            }
                            8 => {
                                dynasm!(asm; mov Rq(dst_reg), QWORD [rbp - src_reg_val_rbp_offset])
                            }
                            _ => {
                                todo!("unexpect Register to Register value size {}", src_val_size)
                            }
                        }
                    }
                    Indirect(_dst_reg_num, dst_off, dst_val_size) => {
                        assert!(
                            dst_val_size == src_val_size,
                            "Register2Indirect - src and dst val size must match. got src: {} and dst: {}",
                            src_val_size, dst_val_size
                        );
                        assert!(src_add_locs.len() == 0, "deal with additional info");
                        if *CP_VERBOSE {
                            println!(
                                "Register2Indirect - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        match *src_val_size {
                            1 => dynasm!(asm
                                ; mov al, BYTE [rbp - src_reg_val_rbp_offset]
                                ; mov BYTE [rbp + *dst_off], al
                            ),
                            2 => dynasm!(asm
                                ; mov ax, WORD [rbp - src_reg_val_rbp_offset]
                                ; mov WORD [rbp + *dst_off], ax
                            ),
                            4 => dynasm!(asm
                                ; mov eax, DWORD [rbp - src_reg_val_rbp_offset]
                                ; mov DWORD [rbp + *dst_off], eax
                            ),
                            8 => {
                                dynasm!(asm
                                    ; mov rax, QWORD [rbp - src_reg_val_rbp_offset]
                                    ; mov QWORD [rbp + *dst_off], rax
                                );
                            }
                            _ => panic!(
                                "Unexpected Indirect to Register value size: {}",
                                src_val_size
                            ),
                        }
                    }
                    _ => panic!(
                        "Unexpected target for Register source location - src: {:?}, dst: {:?}",
                        src_location, dst_location
                    ),
                }
            }
            Indirect(src_reg_num, _src_off, src_val_size) => {
                let temp_buffer_offset = (src_var_indirect_index * REG64_BYTESIZE as usize) as i32; // assuming each value is 8 bytes
                                                                                                    // if *CP_VERBOSE {
                                                                                                    //     println!("Indirect - temp_buffer_offset: {:?} src_var_indirect_index: {:?}", temp_buffer_offset, src_var_indirect_index);
                                                                                                    // }
                match dst_location {
                    Register(dst_reg_num, dst_val_size, _dst_add_locs) => {
                        if *CP_VERBOSE {
                            println!(
                                "Indirect2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                        assert!(*src_reg_num == 6, "Indirect register is expected to be rbp");
                        let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                        match *dst_val_size {
                            1 => dynasm!(asm; mov Rb(dst_reg), BYTE [rsp + temp_buffer_offset]),
                            2 => dynasm!(asm; mov Rw(dst_reg), WORD [rsp + temp_buffer_offset]),
                            4 => dynasm!(asm; mov Rd(dst_reg), DWORD [rsp + temp_buffer_offset]),
                            8 => dynasm!(asm; mov Rq(dst_reg), QWORD [rsp + temp_buffer_offset]),
                            _ => panic!(
                                "Unexpected Indirect to Register value size: {}",
                                src_val_size
                            ),
                        }
                    }
                    Indirect(_dst_reg_num, dst_off, dst_val_size) => {
                        if *CP_VERBOSE {
                            println!(
                                "Indirect2Indirect - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        // TODO: understand what to do where the size value is different
                        let min_size = src_val_size.min(dst_val_size);
                        match min_size {
                            1 => dynasm!(asm
                                ; mov al, BYTE [rsp + temp_buffer_offset]
                                ; mov BYTE [rbp + i32::try_from(*dst_off).unwrap()], al
                            ),
                            2 => dynasm!(asm
                                ; mov ax, WORD [rsp + temp_buffer_offset]
                                ; mov WORD [rbp + i32::try_from(*dst_off).unwrap()], ax
                            ),
                            4 => dynasm!(asm
                                ; mov eax, DWORD [rsp + temp_buffer_offset]
                                ; mov DWORD [rbp + i32::try_from(*dst_off).unwrap()], eax
                            ),
                            8 => {
                                dynasm!(asm
                                    ; mov rax, QWORD [rsp + temp_buffer_offset]
                                    ; mov QWORD [rbp + i32::try_from(*dst_off).unwrap()], rax
                                );
                            }
                            _ => panic!("Unexpected Indirect to Indirect value size: {}", min_size),
                        }
                    }
                    _ => panic!(
                        "Unexpected target for Indirect source location - src: {:?}, dst: {:?}",
                        src_location, dst_location
                    ),
                }
                src_var_indirect_index += 1;
            }
            Direct(_, _, _) => {
                // Do nothing
            }
            _ => panic!("Unexpected source location: {:?}", src_location),
        }
    }
    dest_reg_nums
}

/*
General stack layout:
    +---------------------------------------------+  <-- RBP (frameaddr, source frame)
    |  RSP - frame size                           |
    +---------------------------------------------+  <--- RSP
    | __ykrt_control_point - Saved Registers      |  (rax, rcx, rbx, rdi, rsi, r8-r15)
    +---------------------------------------------+
    |  Temporary buffer for Source Live Vars      |
    +---------------------------------------------+  <-- RSP
*/
pub unsafe fn control_point_transition(transition: CPTransition) {
    let frameaddr = transition.frameaddr as usize;
    let mut asm = Assembler::new().unwrap();

    let mut src_smid = ControlPointStackMapId::Opt;
    let mut dst_smid = ControlPointStackMapId::UnOpt;

    if transition.direction == CPTransitionDirection::UnoptToOpt {
        src_smid = ControlPointStackMapId::UnOpt;
        dst_smid = ControlPointStackMapId::Opt;
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

    let src_rbp_offset = src_frame_size as i32 + REG64_BYTESIZE as i32;
    if *CP_VERBOSE {
        println!("@@ transition from {:?} to {:?}, exec_trace: {:?}", src_smid, dst_smid, transition.exec_trace);
    }
    if *CP_BREAK {
        dynasm!(asm; .arch x64; int3);
    }

    // Step 1. Calculate the size of the buffer for source live vars
    let src_val_buffer_size = calculate_live_vars_buffer_size(src_rec);

    // Calculate the offset from the RBP to the RSP where __ykrt_control_point_real stored the registers.
    let rbp_offset_reg_store =
        src_frame_size as i64 + (14 * REG64_BYTESIZE) as i64;

    // Step 2. Set RBP and RSP
    // +-------------------------------------------+ <- RBP
    // |       Destination frame                   |
    // +-------------------------------------------+
    // |       Indirect Live Vars                  |
    // +-------------------------------------------+ <- RSP
    dynasm!(asm
        ; .arch x64
        ; mov rbp, QWORD frameaddr as i64
        ; mov rsp, QWORD frameaddr as i64
        ; sub rsp, (dst_frame_size).try_into().unwrap() // adjust rsp
        ; sub rsp, src_val_buffer_size // Reserve buffer to store Direct and Indirect values
    );

    // Step 3. Copy src live vars into the buffer
    copy_live_vars_to_temp_buffer(&mut asm, src_rec);

    if *CP_VERBOSE {
        println!(
            "@@ src_rbp: 0x{:x}, src_rsp: 0x{:x}, src_rbp_offset: 0x{:x}, src_frame_size: 0x{:x}, dst_frame_size: 0x{:x}, rbp_offset_reg_store: 0x{:x}",
            frameaddr as i64,
            frameaddr as i64 - rbp_offset_reg_store,
            src_rbp_offset,
            src_frame_size,
            dst_frame_size,
            rbp_offset_reg_store
        );
        // println!("--------------------------------");
        // println!("@@ src live vars - smid: {:?}", src_smid);
        // for (index, src_var) in src_rec.live_vars.iter().enumerate() {
        //     let src_location = &src_var.get(0).unwrap();
        //     println!("{} - {:?}", index, src_location);
        // }
        // println!("@@ dst live vars - smid: {:?}", dst_smid);
        // for (index, dst_var) in dst_rec.live_vars.iter().enumerate() {
        //     let dst_location = &dst_var.get(0).unwrap();
        //     println!("{} - {:?}", index, dst_location);
        // }
        // println!("--------------------------------");
    }

    // Step 4. Set destination live vars
    let used_registers = set_destination_live_vars(
        &mut asm,
        src_rec,
        dst_rec,
        rbp_offset_reg_store,
    );

    restore_registers(
        &mut asm,
        used_registers,
        rbp_offset_reg_store as i32,
    );

    // Ensure that RSP remains 16-byte aligned throughout transitions to comply with the x86-64 ABI.
    assert_eq!(
        (frameaddr as i64 - dst_frame_size as i64) % 16,
        0,
        "RSP is not aligned to 16 bytes"
    );

    if transition.exec_trace {
        if *CP_VERBOSE {
            println!("@@ calling exec_trace");
        }
        if *CP_BREAK {
            dynasm!(asm; .arch x64; int3); // breakpoint
        }
        dynasm!(asm
            ; .arch x64
            ; sub rsp, 0x8                // Align rsp to 16-byte boundary after call
            ; add rsp, src_val_buffer_size // remove the temporary buffer from the stack
            ; mov rdi, QWORD frameaddr as i64              // First argument
            ; mov rsi, QWORD transition.rsp as i64    //   Second argument
            ; mov rdx, QWORD transition.trace_addr as i64             // Third argument
            ; mov rcx, QWORD transition.exec_trace_fn as i64          // Move function pointer to rcx
            ; call rcx // Call the function - we don't care about rcx because its overridden in the __yk_exec_trace
        );
    } else {
        let call_offset = calc_after_cp_offset(dst_rec.offset).unwrap();
        let dst_target_addr = i64::try_from(dst_rec.offset).unwrap() + call_offset;
        dynasm!(asm
            ; .arch x64
            ; add rsp, src_val_buffer_size // remove the temporary buffer from the stack
            ; sub rsp, 0x10 // reserves 16 bytes of space on the stack.
            ; mov [rsp], rax // save rsp
            ; mov rax, QWORD dst_target_addr // loads the target address into rax
            ; mov [rsp + 0x8], rax // stores the target address into rsp+8
            ; pop rax // restores the original rax at rsp
            ; ret // loads 8 bytes from rsp and jumps to it
        );
    }
    let buffer = asm.finalize().unwrap();
    let func: unsafe fn() = std::mem::transmute(buffer.as_ptr());
    func();
}

fn restore_registers(
    asm: &mut Assembler,
    exclude_registers: HashMap<u16, u16>,
    rbp_offset_reg_store: i32,
) {
    let mut sorted_offsets: Vec<(&u16, &i32)> = REG_OFFSETS.iter().collect();
    sorted_offsets.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending by value

    for (reg_num, _) in sorted_offsets.iter() {
        if !exclude_registers.contains_key(reg_num) {
            restore_register(
                asm,
                (**reg_num).try_into().unwrap(),
                rbp_offset_reg_store,
            );
        }
    }
}

fn restore_register(
    asm: &mut Assembler,
    dwarf_reg_num: u16,
    rbp_offset_reg_store: i32,
) {
    let reg_offset = reg_num_to_ykrt_control_point_rsp_offset(dwarf_reg_num);
    let reg_val_rbp_offset =
        i32::try_from(rbp_offset_reg_store - reg_offset as i32).unwrap();
    let dest_reg = dwarf_to_dynasm_reg(dwarf_reg_num.try_into().unwrap());
    dynasm!(asm
        ; mov Rq(dest_reg), QWORD [rbp - reg_val_rbp_offset]
    );
    // if *CP_VERBOSE {
    //     println!(
    //         "@@ Restoring reg_num: {:?}, dest_reg: {:?}, reg_offset: 0x{:x}, reg_val_rbp_offset: 0x{:x}",
    //         dwarf_reg_num,
    //         dest_reg,
    //         reg_offset,
    //         reg_val_rbp_offset
    //     );
    // }
}

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
    use yksmp::{LiveVar, Location, Record};
    // use capstone::Capstone;

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

        return instructions.iter().map(|inst| format!("{} {}", inst.mnemonic().unwrap_or(""), inst.op_str().unwrap_or(""))).collect();
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
        let mut used_regs = HashMap::new();
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
        // Act: Calculate the call offset
        let offset = calc_after_cp_offset(code_ptr)?;
        // Assert: The offset should be 13 bytes
        assert_eq!(offset, 11, "The call offset should be 11 bytes");
        Ok(())
    }

    #[test]
    fn test_calculate_live_vars_buffer_size() {
        let mock_record = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Indirect(0, 0, 16)]),
                LiveVar::new(vec![Location::Indirect(0, 0, 8)]),
                LiveVar::new(vec![Location::Indirect(0, 0, 4)]),
                LiveVar::new(vec![Location::Indirect(0, 0, 8)]),
            ],
        };

        let buffer_size = calculate_live_vars_buffer_size(&mock_record);
        assert_eq!(
            buffer_size,
            16 + 8 + 4 + 8,
            "Buffer size should equal the sum of all live variable sizes"
        );
    }

    #[test]
    fn test_calculate_live_vars_buffer_size_aligned_to_16() {
        let mock_record = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Indirect(0, 0, 8)]),
            ],
        };

        let buffer_size = calculate_live_vars_buffer_size(&mock_record);
        assert_eq!(
            buffer_size,
            8,
            "Buffer size should be 8 bytes"
        );
    }

    #[test]
    fn test_set_destination_live_vars_register_to_register() {
        // Arrange: Create mock `src_rec` and `dst_rec` for Register-to-Register live vars
        let src_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Register(15, 8, vec![])]), // r15, size 8
            ],
        };

        let dst_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Register(1, 8, vec![])]), // rcx, size 8
            ],
        };

        let mut asm = Assembler::new().unwrap();
        // Act: Call the function to test Register-to-Register copy
        let dest_reg_nums = set_destination_live_vars(&mut asm, &src_rec, &dst_rec, 0x10);
        // Finalize the assembly and disassemble the instructions
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions[0], "mov rdx, qword ptr [rbp - 0x10]");

        // Verify dest_reg_nums maps rcx to its value size
        assert_eq!(
            dest_reg_nums.get(&1),
            Some(&8),
            "The destination register (rcx) should be recorded with its size"
        );
    }
    // TODO: test register to indirect
    // TODO: test indirect to register
    // TODO: test indirect to indirect
}
