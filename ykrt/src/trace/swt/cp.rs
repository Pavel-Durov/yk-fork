use crate::aotsmp::AOT_STACKMAPS;
use capstone::prelude::*;
use yksmp::{Record};
use dynasmrt::{dynasm, x64::Assembler, DynasmApi};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::c_void;
use std::sync::LazyLock;
use yksmp::Location::{Constant, Direct, Indirect, LargeConstant, Register};

/// The size of a 64-bit register in bytes.
pub(crate) static REG64_BYTESIZE: u64 = 8;

// Feature flags
pub static CP_TRANSITION_DEBUG_MODE: LazyLock<bool> = LazyLock::new(|| {
    env::var("YKRT_CP_TRANSITION_DEBUG")
        .map(|v| v.parse().unwrap_or(false))
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

pub struct ControlPointTransition {
    pub src_smid: ControlPointStackMapId,
    pub dst_smid: ControlPointStackMapId,
    pub frameaddr: *const c_void,
    pub rsp: *const c_void,
    pub trace_addr: *const c_void,
    pub exec_trace: bool,
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

fn reg_num_to_ykrt_control_point_stack_offset(dwarf_reg_num: u16) -> i32 {
    REG_OFFSETS
        .get(&dwarf_reg_num)
        .copied()
        .unwrap_or_else(|| panic!("Unsupported register {}", dwarf_reg_num))
}

fn calculate_live_vars_buffer_size(src_rec: &Record) -> i32 {
    let mut src_val_buffer_size: i32 = 0;
    for (_, src_var) in src_rec.live_vars.iter().enumerate() {
        match src_var.get(0).unwrap() {
            // TODO: do we need to store direct values?
            Direct(_, _, src_val_size) => {
                src_val_buffer_size += *src_val_size as i32;
            }
            Indirect(_, _, src_val_size) => {
                src_val_buffer_size += *src_val_size as i32;
            }
            _ => { /* DO NOTHING */ }
        }
    }
    src_val_buffer_size
}

fn copy_live_vars_to_temp_buffer(
    asm: &mut Assembler,
    src_rec: &Record
) {
    for (index, src_var) in src_rec.live_vars.iter().enumerate() {
        let temp_off = (index * REG64_BYTESIZE as usize) as i32; // assuming each value is 8 bytes
        let src_location = src_var.get(0).unwrap();
        match src_location {
            Direct(_, src_off, src_val_size) => {
                assert!(
                    *src_val_size == 8,
                    "Only 8-byte Direct values supported in this example"
                );
                dynasm!(asm
                    ; mov rax, QWORD [rbp + i32::try_from(*src_off).unwrap()]
                    ; mov [rsp + temp_off], rax
                );
            }
            Indirect(_, src_off, src_val_size) => {
                assert!(
                    *src_val_size == 8,
                    "Only 8-byte Indirect values supported in this example"
                );
                dynasm!(asm
                    ; mov rax, QWORD [rbp + i32::try_from(*src_off).unwrap()]
                    ; mov QWORD [rsp + temp_off], rax
                );
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
pub unsafe fn control_point_transition(transition: ControlPointTransition) {
    let ControlPointTransition {
        src_smid,
        dst_smid,
        frameaddr,
        rsp,
        trace_addr,
        exec_trace,
        exec_trace_fn,
    } = transition;
    let frameaddr = frameaddr as usize;
    let mut asm = Assembler::new().unwrap();

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
    if *CP_TRANSITION_DEBUG_MODE {
        println!("@@ TRANSITION from: {:?} to: {:?}", src_smid, dst_smid);
        dynasm!(asm; .arch x64; int3);
    }

    // Step 1. Calculate the size of the buffer for source live vars
    let src_val_buffer_size = calculate_live_vars_buffer_size(src_rec);

    // Calculate the offset from the RBP to the RSP where __ykrt_control_point_real stored the registers.
    let rbp_offset_to_ykrt_control_point_reg_store =
        src_frame_size as i64 + (14 * REG64_BYTESIZE) as i64;

    // Step 2. Set RBP and RSP
    // +-------------------------------------------+ <- RBP
    // |       Destination frame                   |
    // +-------------------------------------------+
    // |       Temporary Src Live Vars Buffer      |
    // +-------------------------------------------+ <- RSP
    dynasm!(asm
        ; .arch x64
        ; mov rbp, QWORD frameaddr as i64
        ; mov rsp, QWORD frameaddr as i64
        ; sub rsp, (dst_frame_size).try_into().unwrap() // adjust rsp
        ; sub rsp, src_val_buffer_size // Reserve buffer to store Direct and Indirect values
    );
    // Ensure that RSP remains 16-byte aligned throughout transitions to comply with the x86-64 ABI.
    assert_eq!(
        (frameaddr as i64 - dst_frame_size as i64) % 16,
        0,
        "RSP is not aligned to 16 bytes"
    );

    // Step 3. Copy src live vars into the buffer
    copy_live_vars_to_temp_buffer(&mut asm, src_rec);

    if *CP_TRANSITION_DEBUG_MODE {
        println!(
            "@@ src_rbp: 0x{:x}, src_rsp: 0x{:x}, src_rbp_offset: 0x{:x}, src_frame_size: 0x{:x}, dst_frame_size: 0x{:x}, rbp_offset_to_ykrt_control_point_reg_store: 0x{:x}",
            frameaddr as i64,
            frameaddr as i64 - rbp_offset_to_ykrt_control_point_reg_store,
            src_rbp_offset,
            src_frame_size,
            dst_frame_size,
            rbp_offset_to_ykrt_control_point_reg_store
        );
        println!("--------------------------------");
        println!("@@ src live vars - smid: {:?}", src_smid);
        for (index, src_var) in src_rec.live_vars.iter().enumerate() {
            let src_location = &src_var.get(0).unwrap();
            println!("{} - {:?}", index, src_location);
        }
        println!("@@ dst live vars - smid: {:?}", dst_smid);
        for (index, dst_var) in dst_rec.live_vars.iter().enumerate() {
            let dst_location = &dst_var.get(0).unwrap();
            println!("{} - {:?}", index, dst_location);
        }
        println!("--------------------------------");
    }

    let mut dest_reg_nums = HashMap::new();

    // Step 4. Set destination live vars
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
                let src_reg_offset = reg_num_to_ykrt_control_point_stack_offset(*src_reg_num);
                let src_reg_val_rbp_offset = i32::try_from(
                    rbp_offset_to_ykrt_control_point_reg_store - src_reg_offset as i64,
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
                        dest_reg_nums.insert(*dst_reg_num, dst_val_size);
                        // skip copying to the same register with the same value size
                        if src_reg_num == dst_reg_num && src_val_size == dst_val_size {
                            continue;
                        }
                        if *CP_TRANSITION_DEBUG_MODE {
                            println!(
                                "Register2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        let dest_reg = u8::try_from(*dst_reg_num).unwrap();
                        match *src_val_size {
                            1 => {
                                dynasm!(asm; mov Rb(dest_reg), BYTE [rbp - src_reg_val_rbp_offset])
                            }
                            2 => {
                                dynasm!(asm; mov Rw(dest_reg), WORD [rbp - src_reg_val_rbp_offset])
                            }
                            4 => {
                                dynasm!(asm; mov Rd(dest_reg), DWORD [rbp - src_reg_val_rbp_offset])
                            }
                            8 => {
                                dynasm!(asm; mov Rq(dest_reg), QWORD [rbp - src_reg_val_rbp_offset])
                            }
                            _ => {
                                todo!("unexpect Register to Register value size {}", src_val_size)
                            }
                        }
                    }
                    Indirect(_dst_reg_num, dst_off, dst_val_size) => {
                        assert!(
                            dst_val_size == src_val_size,
                            "Indirect2Register - src and dst val size must match. got src: {} and dst: {}",
                            src_val_size, dst_val_size
                        );
                        assert!(src_add_locs.len() == 0, "deal with additional info");
                        if *CP_TRANSITION_DEBUG_MODE {
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
                            8 => dynasm!(asm
                                ; mov rax, QWORD [rbp - src_reg_val_rbp_offset]
                                ; mov QWORD [rbp + *dst_off], rax
                            ),
                            _ => panic!(
                                "Unexpected Indirect to Register value size: {}",
                                src_val_size
                            ),
                        }
                    }
                    Direct(_dst_reg_num, dst_off, _dst_val_size) => {
                        if *CP_TRANSITION_DEBUG_MODE {
                            println!(
                                "Register2Direct - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        match *src_val_size {
                            1 => todo!(),
                            2 => todo!(),
                            4 => todo!(),
                            8 => dynasm!(asm
                                ; mov rax, QWORD [rbp - src_reg_val_rbp_offset]
                                ; mov rax, QWORD [rax]
                                ; mov [rbp + *dst_off], rax
                            ),
                            _ => panic!(
                                "Unexpected Indirect to Register value size: {}",
                                src_val_size
                            ),
                        }
                    }
                    Constant(_val) => {
                        todo!("implement Indirect to Constant")
                    }
                    LargeConstant(_val) => {
                        todo!("implement Indirect to LargeConstant")
                    }
                }
            }
            Indirect(src_reg_num, src_off, src_val_size) => {
                match dst_location {
                    Indirect(_dst_reg_num, dst_off, dst_val_size) => {
                        if *CP_TRANSITION_DEBUG_MODE {
                            println!(
                                "Indirect2Indirect - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        let temp_buffer_off = (index * REG64_BYTESIZE as usize) as i32;
                        // TODO: understand what to do where the size value is different
                        let min_size = src_val_size.min(dst_val_size);
                        match min_size {
                            1 => dynasm!(asm
                                ; mov al, BYTE [rsp + temp_buffer_off]
                                ; mov BYTE [rbp + i32::try_from(*dst_off).unwrap()], al
                            ),
                            2 => dynasm!(asm
                                ; mov ax, WORD [rsp + temp_buffer_off]
                                ; mov WORD [rbp + i32::try_from(*dst_off).unwrap()], ax
                            ),
                            4 => dynasm!(asm
                                ; mov eax, DWORD [rsp + temp_buffer_off]
                                ; mov DWORD [rbp + i32::try_from(*dst_off).unwrap()], eax
                            ),
                            8 => dynasm!(asm
                                ; mov rax, QWORD [rsp + temp_buffer_off]
                                ; mov QWORD [rbp + i32::try_from(*dst_off).unwrap()], rax
                            ),
                            _ => panic!("Unexpected Indirect to Indirect value size: {}", min_size),
                        }
                    }
                    Register(dst_reg_num, dst_val_size, _dst_add_locs) => {
                        if *CP_TRANSITION_DEBUG_MODE {
                            println!(
                                "Indirect2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        dest_reg_nums.insert(*dst_reg_num, dst_val_size);
                        assert!(*src_reg_num == 6, "Indirect register is expected to be rbp");
                        let dst_reg = u8::try_from(*dst_reg_num).unwrap();
                        let temp_buffer_off = (index * REG64_BYTESIZE as usize) as i32;
                        match *dst_val_size {
                            1 => dynasm!(asm; mov Rb(dst_reg), BYTE [rsp + temp_buffer_off]),
                            2 => dynasm!(asm; mov Rw(dst_reg), WORD [rsp + temp_buffer_off]),
                            4 => dynasm!(asm; mov Rd(dst_reg), DWORD [rsp + temp_buffer_off]),
                            8 => dynasm!(asm; mov Rq(dst_reg), QWORD [rsp + temp_buffer_off]),
                            _ => panic!("Unsupported source value size: {}", src_val_size),
                        }
                    }
                    _ => panic!("Unsupported dst location: {:?}", dst_location),
                }
            }
            Direct(_src_reg_num, src_off, src_val_size) => {
                let temp_buffer_off = (index * REG64_BYTESIZE as usize) as i32;
                match dst_location {
                    Register(dst_reg_num, dst_val_size, _dst_add_locs) => {
                        if *CP_TRANSITION_DEBUG_MODE {
                            println!(
                                "Direct2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        dest_reg_nums.insert(*dst_reg_num, dst_val_size);
                        let dst_reg = u8::try_from(*dst_reg_num).unwrap();
                        match *dst_val_size {
                            1 => todo!(),
                            2 => todo!(),
                            4 => todo!(),
                            8 => dynasm!(asm
                                ; lea Rq(dst_reg), [rsp + temp_buffer_off]
                            ),
                            _ => panic!("Unsupported source value size: {}", src_val_size),
                        }
                    }
                    Direct(_dst_reg_num, dst_off, _dst_val_size) => {
                        // skip copying the same offset cause both variables reuse the same shadowstack.
                        if *src_off == *dst_off {
                            continue;
                        }
                        if *CP_TRANSITION_DEBUG_MODE {
                            println!(
                                "@@ Direct2Direct src: {:?}, dst: {:?}",
                                src_location, dst_location
                            );
                        }

                        match *src_val_size {
                            1 => todo!(),
                            2 => todo!(),
                            4 => todo!(),
                            8 => dynasm!(asm
                                ; mov rax, QWORD [rsp + temp_buffer_off]
                                ; mov [rbp + i32::try_from(*dst_off).unwrap()], rax
                            ),
                            _ => panic!(
                                "Unexpected Indirect to Register value size: {}",
                                src_val_size
                            ),
                        }
                    }
                    _ => panic!("Unsupported dst location: {:?}", dst_location),
                }
            }
            _ => panic!("Unsupported source location: {:?}", src_location),
        }
    }

    // TODO: Restore all registers as they were saved by __ykrt_control_point_real
    // Issue: saved values are seems to be overridden by the values in the stack
    // for (reg_num, reg_offset) in REG_OFFSETS.iter() {
    //     if !dest_reg_nums.contains_key(reg_num) {
    //         if *CP_TRANSITION_DEBUG_MODE {
    //             println!(
    //                 "@@ Restoring reg: {:?}, reg_offset: {:?}",
    //                 reg_num, reg_offset
    //             );
    //         }
    //         dynasm!(asm
    //             ; int3
    //             ; mov Rq(u8::try_from(*reg_num).unwrap()), QWORD [rbp - rbp_offset_to_ykrt_control_point_reg_store as i32 + *reg_offset]
    //         );
    //     }
    // }

    if exec_trace {
        if *CP_TRANSITION_DEBUG_MODE {
            println!("@@ calling exec_trace");
        }
        // Move the arguments into the appropriate registers
        dynasm!(asm
            ; .arch x64
            ; int3
            ; mov rdi, QWORD frameaddr as i64                   // First argument
            ; mov rsi, QWORD rsp as i64    // Second argument
            ; mov rdx, QWORD trace_addr as i64          // Third argument
            ; mov rcx, QWORD exec_trace_fn as i64         // Move function pointer to rcx
            ; call rcx // Call the function - we don't care about rcx because its overridden in the exec_trace_fn
        );
    } else {
        let call_offset = calc_after_cp_offset(dst_rec.offset).unwrap();
        let dst_target_addr = i64::try_from(dst_rec.offset).unwrap() + call_offset;
        dynasm!(asm
            ; .arch x64
            ; add rsp, src_val_buffer_size // remove the buffer from the stack
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
mod tests {
    use super::*;
    use dynasmrt::{dynasm, x64::Assembler};
    use std::error::Error;
    use yksmp::{Record, Location, LiveVar};

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
        // Act: Calculate the call offset
        let offset = calc_after_cp_offset(code_ptr)?;
        // Assert: The offset should be 6 bytes
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
               LiveVar::new(vec![Location::Direct(0, 0, 8)]),
            ],
        };

        let buffer_size = calculate_live_vars_buffer_size(&mock_record);
        assert_eq!(
            buffer_size,
            16 + 8 + 4 + 8,
            "Buffer size should equal the sum of all live variable sizes"
        );
    }
}
