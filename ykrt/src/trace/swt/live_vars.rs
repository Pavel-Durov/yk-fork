use dynasmrt::{dynasm, x64::Assembler, DynasmApi};
use std::alloc::{alloc, handle_alloc_error, Layout};
use std::collections::HashMap;
use yksmp::Location::{Direct, Indirect, Register};
use yksmp::Record;

use crate::trace::swt::cfg::{
    dwarf_to_dynasm_reg, reg_num_to_ykrt_control_point_rsp_offset, LiveVarsBuffer, CP_VERBOSE,
};

pub(crate) fn set_destination_live_vars(
    asm: &mut Assembler,
    src_rec: &Record,
    dst_rec: &Record,
    rbp_offset_reg_store: i64,
    live_vars_buffer: LiveVarsBuffer,
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
                let src_reg_val_rbp_offset =
                    i32::try_from(rbp_offset_reg_store - src_reg_offset as i64).unwrap();

                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        // Handle additional locations
                        for location in dst_add_locs {
                            // Write any additional locations that were tracked for this variable.
                            // Numbers greater or equal to zero are registers in Dwarf notation.
                            // Negative numbers are offsets relative to RBP.
                            if *CP_VERBOSE {
                                println!("Register2Register - additional location: {:?}", location);
                            }
                            if *location >= 0 {
                                let dst_reg =
                                    dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
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
                                    _ => panic!(
                                        "unexpect Register to Register value size {}",
                                        src_val_size
                                    ),
                                }
                            } else if *location < 0 {
                                let rbp_offset = i32::try_from(*location).unwrap();
                                match *src_val_size {
                                    1 => dynasm!(asm
                                        ; mov al, BYTE [rbp - src_reg_val_rbp_offset]
                                        ; mov BYTE [rbp + rbp_offset], al
                                    ),
                                    2 => dynasm!(asm
                                        ; mov ax, WORD [rbp - src_reg_val_rbp_offset]
                                        ; mov WORD [rbp + rbp_offset], ax
                                    ),
                                    4 => dynasm!(asm
                                        ; mov eax, DWORD [rbp - src_reg_val_rbp_offset]
                                        ; mov DWORD [rbp + rbp_offset], eax
                                    ),
                                    8 => dynasm!(asm
                                        ; mov rax, QWORD [rbp - src_reg_val_rbp_offset]
                                        ; mov QWORD [rbp + rbp_offset], rax
                                    ),
                                    _ => panic!(
                                        "Unexpected Indirect to Register value size: {}",
                                        src_val_size
                                    ),
                                }
                            }
                        }
                        assert!(
                            dst_val_size == src_val_size,
                            "Register2Register - src and dst val size must match. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                        if *CP_VERBOSE {
                            println!(
                                "Register2Register - src: {:?} dst: {:?}, extras: {:?}",
                                src_location, dst_location, dst_add_locs
                            );
                        }
                        // skip copying to the same register with the same value size
                        if src_reg_num == dst_reg_num && src_val_size == dst_val_size {
                            continue;
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
                let temp_buffer_offset = live_vars_buffer.variables[&src_var_indirect_index];
                if *CP_VERBOSE {
                    println!(
                        "Indirect - buffer_index: {:?}, offset: {:?}",
                        src_var_indirect_index, temp_buffer_offset
                    );
                }
                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        if *CP_VERBOSE {
                            println!(
                                "Indirect2Register - src: {:?} dst: {:?}, extras: {:?}",
                                src_location, dst_location, dst_add_locs
                            );
                        }
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                        assert!(*src_reg_num == 6, "Indirect register is expected to be rbp");
                        let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                        match *dst_val_size {
                            1 => dynasm!(asm
                                ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                ; mov Rb(dst_reg), BYTE [rcx + temp_buffer_offset]),
                            2 => dynasm!(asm
                                ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                ; mov Rw(dst_reg), WORD [rcx + temp_buffer_offset]),
                            4 => dynasm!(asm
                                ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                ; mov Rd(dst_reg), DWORD [rcx + temp_buffer_offset]),
                            8 => dynasm!(asm
                                ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                ; mov Rq(dst_reg), QWORD [rcx + temp_buffer_offset]),
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
                                ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                ; mov al, BYTE [rcx + temp_buffer_offset]
                                ; mov BYTE [rbp + i32::try_from(*dst_off).unwrap()], al
                            ),
                            2 => dynasm!(asm
                                ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                ; mov ax, WORD [rcx + temp_buffer_offset]
                                ; mov WORD [rbp + i32::try_from(*dst_off).unwrap()], ax
                            ),
                            4 => dynasm!(asm
                                ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                ; mov eax, DWORD [rcx + temp_buffer_offset]
                                ; mov DWORD [rbp + i32::try_from(*dst_off).unwrap()], eax
                            ),
                            8 => {
                                dynasm!(asm
                                    ; mov rcx, QWORD live_vars_buffer.ptr as i64
                                    ; mov rax, QWORD [rcx + temp_buffer_offset]
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
    // Align the buffer size to 16 bytes (only round up, never down)
    if src_val_buffer_size % 16 == 0 {
        src_val_buffer_size
    } else {
        ((src_val_buffer_size / 16) + 1) * 16
    }
}

pub(crate) fn copy_live_vars_to_temp_buffer(
    asm: &mut Assembler,
    src_rec: &Record,
) -> LiveVarsBuffer {
    let src_val_buffer_size = calculate_live_vars_buffer_size(src_rec);

    let temp_live_vars_buffer = if src_val_buffer_size > 0 {
        unsafe {
            let layout = Layout::from_size_align(src_val_buffer_size as usize, 16).unwrap();
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            (ptr, layout)
        }
    } else {
        return LiveVarsBuffer {
            ptr: 0 as *mut u8,
            layout: Layout::new::<u8>(),
            variables: HashMap::new(),
            size: 0,
        };
    };
    let mut src_var_indirect_index = 0;
    let mut variables = HashMap::new();
    // Set up the pointer to the temporary buffer
    dynasm!(asm
        ; mov rcx, QWORD temp_live_vars_buffer.0 as i64
    );
    for (_, src_var) in src_rec.live_vars.iter().enumerate() {
        let src_location = src_var.get(0).unwrap();
        match src_location {
            Indirect(_, src_off, src_val_size) => {
                assert!(
                    *src_val_size == 8,
                    "Only 8-byte Indirect values supported in this example"
                );
                let temp_buffer_offset = (src_var_indirect_index * (*src_val_size as i32)) as i32;
                dynasm!(asm
                    ; mov rax, QWORD [rbp + i32::try_from(*src_off).unwrap()]
                    ; mov QWORD [rcx + temp_buffer_offset], rax
                );
                variables.insert(src_var_indirect_index, temp_buffer_offset);
                src_var_indirect_index += 1;
            }
            Direct(_, _, _) => {
                // DO NOTHING
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
    let live_vars_buffer = LiveVarsBuffer {
        ptr: temp_live_vars_buffer.0,
        layout: temp_live_vars_buffer.1,
        variables: variables,
        size: src_val_buffer_size,
    };
    live_vars_buffer
}

#[cfg(test)]
mod live_vars_tests {
    use super::*;
    use crate::trace::swt::cfg::REG64_BYTESIZE;
    use capstone::prelude::*;
    use dynasmrt::x64::Assembler;
    use yksmp::{LiveVar, Location, Record};

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
            // 12 is the padding
            16 + 8 + 4 + 8 + 12,
            buffer_size,
            "Buffer size should equal the sum of all live variable sizes + padding"
        );
    }
    #[test]
    fn calculate_live_vars_buffer_size_buffer_size_alignment() {
        // Test cases with different initial sizes
        let test_cases = vec![
            (0, 0),   // 0 should remain 0
            (1, 16),  // 1 should become 16
            (16, 16), // 16 should remain 16
            (17, 32), // 17 should become 32
            (31, 32), // 31 should become 32
            (32, 32), // 32 should remain 32
        ];
        for (val_size, expected_buffer_size) in test_cases {
            // Create a mock record with the given buffer size
            let mock_record = Record {
                offset: 0,
                size: 0,
                id: 0,
                live_vars: vec![LiveVar::new(vec![Location::Indirect(0, 0, val_size)])],
            };
            let buffer_size = calculate_live_vars_buffer_size(&mock_record);
            assert_eq!(
                buffer_size, expected_buffer_size,
                "Buffer size for input {} should be {}",
                val_size, expected_buffer_size
            );
        }
    }

    #[test]
    fn test_set_destination_live_vars_register_to_register() {
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
        let temp_live_vars_buffer = LiveVarsBuffer {
            ptr: 0 as *mut u8,
            layout: Layout::new::<u8>(),
            variables: HashMap::new(),
            size: 0,
        };
        let dest_reg_nums =
            set_destination_live_vars(&mut asm, &src_rec, &dst_rec, 0x10, temp_live_vars_buffer);
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions[0], "mov rdx, qword ptr [rbp - 0x10]");
        assert_eq!(
            dest_reg_nums.get(&1),
            Some(&8),
            "The destination register (rcx) should be recorded with its size"
        );
    }

    #[test]
    fn test_set_destination_live_vars_register_to_indirect() {
        let src_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![LiveVar::new(vec![Location::Register(15, 8, vec![])])],
        };
        let dst_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![LiveVar::new(vec![Location::Indirect(0, 0, 8)])],
        };
        let mut asm = Assembler::new().unwrap();
        let temp_live_vars_buffer = LiveVarsBuffer {
            ptr: std::ptr::null_mut(),
            layout: Layout::new::<u8>(),
            variables: HashMap::new(),
            size: 0,
        };

        let dest_reg_nums =
            set_destination_live_vars(&mut asm, &src_rec, &dst_rec, 0x10, temp_live_vars_buffer);
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!("mov rax, qword ptr [rbp - 0x10]", instructions[0]);
        assert_eq!("mov qword ptr [rbp], rax", instructions[1]);
        assert!(dest_reg_nums.is_empty());
    }

    #[test]
    fn test_set_destination_live_vars_indirect_to_register() {
        let src_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Indirect(6, 0, 8)]), // source indirect
            ],
        };
        let dst_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Register(15, 8, vec![])]), // destination register
            ],
        };
        let mut asm = Assembler::new().unwrap();
        let layout = Layout::from_size_align(8 as usize, 16).unwrap();
        let ptr = unsafe { alloc(layout) };

        let mut variables = HashMap::new();
        variables.insert(0 as i32, REG64_BYTESIZE as i32);
        let temp_live_vars_buffer = LiveVarsBuffer {
            ptr,
            layout,
            variables,
            size: 8 as i32,
        };

        let dest_reg_nums =
            set_destination_live_vars(&mut asm, &src_rec, &dst_rec, 0x10, temp_live_vars_buffer);
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);

        assert_eq!(format!("movabs rcx, 0x{:x}", ptr as i64), instructions[0]);
        assert_eq!("mov r15, qword ptr [rcx + 8]", instructions[1]);
        assert_eq!(
            dest_reg_nums.get(&15),
            Some(&8),
            "The destination register (r15) should be recorded with its size"
        );
    }

    #[test]
    fn test_set_destination_live_vars_indirect_to_indirect() {
        let src_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![LiveVar::new(vec![Location::Indirect(6, 12354, 8)])],
        };
        let dst_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![LiveVar::new(vec![Location::Indirect(6, 6, 8)])],
        };
        let mut asm = Assembler::new().unwrap();
        let layout = Layout::from_size_align(8 as usize, 16).unwrap();
        let ptr = unsafe { alloc(layout) };
        let mut variables = HashMap::new();
        variables.insert(0 as i32, REG64_BYTESIZE as i32);
        variables.insert(1 as i32, REG64_BYTESIZE as i32);
        let temp_live_vars_buffer = LiveVarsBuffer {
            ptr,
            layout,
            variables,
            size: 8 as i32,
        };

        let dest_reg_nums =
            set_destination_live_vars(&mut asm, &src_rec, &dst_rec, 0x10, temp_live_vars_buffer);
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);

        assert_eq!(format!("movabs rcx, 0x{:x}", ptr as i64), instructions[0]);
        assert_eq!("mov rax, qword ptr [rcx + 8]", instructions[1]);
        assert_eq!("mov qword ptr [rbp + 6], rax", instructions[2]);
        assert!(dest_reg_nums.is_empty());
    }

    #[test]
    fn test_copy_live_vars_to_temp_buffer() {
        let src_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Indirect(6, 56, 8)]),
                LiveVar::new(vec![Location::Indirect(6, 72, 8)]),
                LiveVar::new(vec![Location::Indirect(6, 172, 8)]),
            ],
        };

        let mut asm = Assembler::new().unwrap();
        let lvb = copy_live_vars_to_temp_buffer(&mut asm, &src_rec);
        assert_eq!(32, lvb.size);
        assert_eq!(3, lvb.variables.len());

        // Finalise and disassemble the code.
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);

        assert_eq!(
            format!("movabs rcx, 0x{:x}", lvb.ptr as i64),
            instructions[0]
        );
        // 1st indirect
        assert_eq!("mov rax, qword ptr [rbp + 0x38]", instructions[1]);
        assert_eq!("mov qword ptr [rcx], rax", instructions[2]);
        // 2nd indirect
        assert_eq!("mov rax, qword ptr [rbp + 0x48]", instructions[3]);
        assert_eq!("mov qword ptr [rcx + 8], rax", instructions[4]);
        // 3rd indirect
        assert_eq!("mov rax, qword ptr [rbp + 0xac]", instructions[5]);
        assert_eq!("mov qword ptr [rcx + 0x10], rax", instructions[6]);
    }
}
