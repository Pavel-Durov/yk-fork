use dynasmrt::{dynasm, x64::Assembler, DynasmApi};
use std::alloc::{alloc, handle_alloc_error, Layout};
use std::collections::HashMap;
use std::sync::OnceLock;
use yksmp::Location::{Direct, Indirect, Register};
use yksmp::Record;

use crate::trace::swt::cfg::{
    dwarf_to_dynasm_reg, reg_num_to_ykrt_control_point_rsp_offset, CPTransitionDirection,
    LiveVarsBuffer, CP_VERBOSE,
};

// Create a thread-safe wrapper for the buffer pointer
#[derive(Debug, Clone)]
struct ThreadSafeBuffer {
    ptr: *mut u8,
    layout: Layout,
    size: i32,
}

// Explicitly mark our wrapper as thread-safe
// This is safe because we control all access to the raw pointer and ensure there's
// no data race (each thread uses a different buffer for optimized vs unoptimized)
unsafe impl Sync for ThreadSafeBuffer {}
unsafe impl Send for ThreadSafeBuffer {}

impl ThreadSafeBuffer {
    unsafe fn new(ptr: *mut u8, layout: Layout, size: i32) -> Self {
        ThreadSafeBuffer { ptr, layout, size }
    }
}

static OPT_BUFFER: OnceLock<ThreadSafeBuffer> = OnceLock::new();
static UNOPT_BUFFER: OnceLock<ThreadSafeBuffer> = OnceLock::new();
// Primary temporary register - used in buffer copy and destination live vars copy.
static TEMP_REG_PRIMARY: u8 = 0;
// Secondary temporary register - used in buffer copy.
static TEMP_REG_SECONDARY: u8 = 1;

#[derive(Debug, Clone)]
struct RestoreTempRegisters<'a> {
    src_location: &'a yksmp::Location,
    dst_location: &'a yksmp::Location,
    src_var_indirect_index: i32,
}

// Helper function to handle additional locations for register transfers
fn handle_register_to_register_additional_locations(
    asm: &mut dynasmrt::Assembler<dynasmrt::x64::X64Relocation>,
    src_reg_val_rbp_offset: i32,
    dst_add_locs: &Vec<i16>,
    src_val_size: &u16,
    dst_reg_num: &u16,
) {
    for location in dst_add_locs {
        // Write any additional locations that were tracked for this variable.
        // Numbers greater or equal to zero are registers in Dwarf notation.
        // Negative numbers are offsets relative to RBP.
        if *location >= 0 {
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
                _ => panic!("unexpect Register to Register value size {}", src_val_size),
            }
        } else {
            let rbp_offset = i32::try_from(*location).unwrap();
            match *src_val_size {
                1 => dynasm!(asm
                    ; mov Rb(TEMP_REG_PRIMARY), BYTE [rbp - src_reg_val_rbp_offset]
                    ; mov BYTE [rbp + rbp_offset], Rb(TEMP_REG_PRIMARY)
                ),
                2 => dynasm!(asm
                    ; mov Rw(TEMP_REG_PRIMARY), WORD [rbp - src_reg_val_rbp_offset]
                    ; mov WORD [rbp + rbp_offset], Rw(TEMP_REG_PRIMARY)
                ),
                4 => dynasm!(asm
                    ; mov Rd(TEMP_REG_PRIMARY), DWORD [rbp - src_reg_val_rbp_offset]
                    ; mov DWORD [rbp + rbp_offset], Rd(TEMP_REG_PRIMARY)
                ),
                8 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD [rbp - src_reg_val_rbp_offset]
                    ; mov QWORD [rbp + rbp_offset], Rq(TEMP_REG_PRIMARY)
                ),
                _ => panic!(
                    "Unexpected Indirect to Register value size: {}",
                    src_val_size
                ),
            }
        }
    }
}

// Helper function to handle additional locations for indirect-to-register transfers
fn handle_indirect_to_register_additional_locations(
    asm: &mut dynasmrt::Assembler<dynasmrt::x64::X64Relocation>,
    dst_add_locs: &Vec<i16>,
    src_val_size: &u16,
    dst_reg_num: &u16,
    temp_buffer_offset: i32,
    live_vars_buffer: &LiveVarsBuffer,
) {
    for location in dst_add_locs {
        // Write any additional locations that were tracked for this variable.
        // Numbers greater or equal to zero are registers in Dwarf notation.
        // Negative numbers are offsets relative to RBP.
        if *location >= 0 {
            let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
            match *src_val_size {
                1 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rb(dst_reg), BYTE [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                2 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rw(dst_reg), WORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                4 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rd(dst_reg), DWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                8 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rq(dst_reg), QWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                _ => panic!(
                    "Indirect2Register - unexpected value size when setting additional locations: {}",
                    src_val_size
                ),
            }
        } else {
            let rbp_offset = i32::try_from(*location).unwrap();
            match *src_val_size {
                1 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rb(TEMP_REG_SECONDARY), BYTE [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                    ; mov BYTE [rbp + rbp_offset], Rb(TEMP_REG_SECONDARY)
                ),
                2 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rw(TEMP_REG_SECONDARY), WORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                    ; mov WORD [rbp + rbp_offset], Rw(TEMP_REG_SECONDARY)
                ),
                4 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rd(TEMP_REG_SECONDARY), DWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                    ; mov DWORD [rbp + rbp_offset], Rd(TEMP_REG_SECONDARY)
                ),
                8 => dynasm!(asm
                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                    ; mov Rq(TEMP_REG_SECONDARY), QWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                    ; mov QWORD [rbp + rbp_offset], Rq(TEMP_REG_SECONDARY)
                ),
                _ => panic!(
                    "Indirect2Register - unexpected value size when setting additional locations: {}",
                    src_val_size
                ),
            }
        }
    }
}

pub(crate) fn set_destination_live_vars(
    asm: &mut Assembler,
    src_rec: &Record,
    dst_rec: &Record,
    rbp_offset_reg_store: i64,
    live_vars_buffer: LiveVarsBuffer,
) -> HashMap<u16, u16> {
    // Map of destination register numbers to their value sizes.
    let mut dest_reg_nums = HashMap::new();
    // List of temporary registers to restore.
    let mut restore_temp_registers = Vec::new();
    // Index of the source live variable in the temporary buffer.
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
            Register(src_reg_num, src_val_size, _src_add_locs) => {
                let src_reg_offset = reg_num_to_ykrt_control_point_rsp_offset(*src_reg_num);
                let src_reg_val_rbp_offset =
                    i32::try_from(rbp_offset_reg_store - src_reg_offset as i64).unwrap();

                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        if *dst_reg_num == TEMP_REG_PRIMARY.into()
                            || *dst_reg_num == TEMP_REG_SECONDARY.into()
                        {
                            restore_temp_registers.push(RestoreTempRegisters {
                                src_location: src_location,
                                dst_location: dst_location,
                                src_var_indirect_index: src_var_indirect_index,
                            });
                        } else {
                            // Handle additional locations
                            handle_register_to_register_additional_locations(
                                asm,
                                src_reg_val_rbp_offset,
                                dst_add_locs,
                                src_val_size,
                                dst_reg_num,
                            );
                            assert!(
                            dst_val_size == src_val_size,
                            "Register2Register - src and dst val size must match. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
                            dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                            if *CP_VERBOSE {
                                println!(
                                    "Register2Register - src: {:?} dst: {:?}",
                                    src_location, dst_location
                                );
                            }
                            // TODO: skip copying to the same register with the same value size
                            // if src_reg_num == dst_reg_num && src_val_size == dst_val_size {
                            //     continue;
                            // }
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
                                    todo!(
                                        "unexpect Register to Register value size {}",
                                        src_val_size
                                    )
                                }
                            }
                        }
                    }
                    Indirect(_dst_reg_num, dst_off, dst_val_size) => {
                        assert!(
                            dst_val_size == src_val_size,
                            "Register2Indirect - src and dst val size must match. got src: {} and dst: {}",
                            src_val_size, dst_val_size
                        );
                        if *CP_VERBOSE {
                            println!(
                                "Register2Indirect - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        match *src_val_size {
                            1 => dynasm!(asm
                                ; mov Rb(TEMP_REG_PRIMARY), BYTE [rbp - src_reg_val_rbp_offset]
                                ; mov BYTE [rbp + *dst_off], al
                            ),
                            2 => dynasm!(asm
                                ; mov Rw(TEMP_REG_PRIMARY), WORD [rbp - src_reg_val_rbp_offset]
                                ; mov WORD [rbp + *dst_off], ax
                            ),
                            4 => dynasm!(asm
                                ; mov Rd(TEMP_REG_PRIMARY), DWORD [rbp - src_reg_val_rbp_offset]
                                ; mov DWORD [rbp + *dst_off], eax
                            ),
                            8 => {
                                dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD [rbp - src_reg_val_rbp_offset]
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
                assert!(!live_vars_buffer.ptr.is_null(), "Live vars buffer is null");
                let temp_buffer_offset = live_vars_buffer.variables[&src_var_indirect_index];
                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        if *dst_reg_num == TEMP_REG_PRIMARY.into()
                            || *dst_reg_num == TEMP_REG_SECONDARY.into()
                        {
                            restore_temp_registers.push(RestoreTempRegisters {
                                src_location: src_location,
                                dst_location: dst_location,
                                src_var_indirect_index: src_var_indirect_index,
                            });
                        } else {
                            if *CP_VERBOSE {
                                println!(
                                    "Indirect2Register - src: {:?} dst: {:?}",
                                    src_location, dst_location
                                );
                            }
                            dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                            assert!(*src_reg_num == 6, "Indirect register is expected to be rbp");
                            // Set register additional locations
                            handle_indirect_to_register_additional_locations(
                                asm,
                                dst_add_locs,
                                src_val_size,
                                dst_reg_num,
                                temp_buffer_offset,
                                &live_vars_buffer,
                            );
                            let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                            match *dst_val_size {
                                1 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rb(dst_reg), BYTE [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                                2 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rw(dst_reg), WORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                                4 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rd(dst_reg), DWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                                8 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rq(dst_reg), QWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                                _ => panic!(
                                    "Unexpected Indirect to Register value size: {}",
                                    src_val_size
                                ),
                            }
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
                        // TODO: Opt - move temp buffer address load to be one-time instead of every indirect2indirect.
                        // TODO: Opt - skip if the same offset on src and dst.
                        let min_size = src_val_size.min(dst_val_size);
                        match min_size {
                            1 => dynasm!(asm
                                ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                ; mov Rb(TEMP_REG_PRIMARY), BYTE [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                                ; mov BYTE [rbp + i32::try_from(*dst_off).unwrap()], Rb(TEMP_REG_PRIMARY)
                            ),
                            2 => dynasm!(asm
                                ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                ; mov Rw(TEMP_REG_PRIMARY), WORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                                ; mov WORD [rbp + i32::try_from(*dst_off).unwrap()], Rw(TEMP_REG_PRIMARY)
                            ),
                            4 => dynasm!(asm
                                ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                ; mov Rd(TEMP_REG_PRIMARY), DWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                                ; mov DWORD [rbp + i32::try_from(*dst_off).unwrap()], Rd(TEMP_REG_PRIMARY)
                            ),
                            8 => {
                                dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]
                                    ; mov QWORD [rbp + i32::try_from(*dst_off).unwrap()], Rq(TEMP_REG_PRIMARY)
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

    // Restore the temporary registers if any were used.
    for restore_temp_register in restore_temp_registers {
        let src_location = restore_temp_register.src_location;
        let dst_location = restore_temp_register.dst_location;
        let src_var_indirect_index = restore_temp_register.src_var_indirect_index;

        match src_location {
            Register(src_reg_num, src_val_size, _src_add_locs) => {
                let src_reg_offset = reg_num_to_ykrt_control_point_rsp_offset(*src_reg_num);
                let src_reg_val_rbp_offset =
                    i32::try_from(rbp_offset_reg_store - src_reg_offset as i64).unwrap();
                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        // Handle additional locations
                        handle_register_to_register_additional_locations(
                            asm,
                            src_reg_val_rbp_offset,
                            dst_add_locs,
                            src_val_size,
                            dst_reg_num,
                        );
                        assert!(
                                dst_val_size == src_val_size,
                                "Register2Register - src and dst val size must match. Got src: {} and dst: {}",
                                src_val_size,
                                dst_val_size
                            );
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                        if *CP_VERBOSE {
                            println!(
                                "Register2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        // TODO: skip copying to the same register with the same value size
                        // if src_reg_num == dst_reg_num && src_val_size == dst_val_size {
                        //     continue;
                        // }
                        let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                        assert!(
                            *dst_val_size == *src_val_size,
                            "Indirect2Register value size mismatch. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
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
                    _ => panic!(
                        "Unexpected target for Register source location - src: {:?}, dst: {:?}",
                        src_location, dst_location
                    ),
                }
            }
            Indirect(src_reg_num, _src_off, src_val_size) => {
                assert!(!live_vars_buffer.ptr.is_null(), "Live vars buffer is null");
                let temp_buffer_offset = live_vars_buffer.variables[&src_var_indirect_index];
                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        if *CP_VERBOSE {
                            println!(
                                "Indirect2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        assert!(*src_reg_num == 6, "Indirect register is expected to be rbp");
                        handle_indirect_to_register_additional_locations(
                            asm,
                            dst_add_locs,
                            src_val_size,
                            dst_reg_num,
                            temp_buffer_offset,
                            &live_vars_buffer,
                        );
                        let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                        assert!(
                            *dst_val_size == *src_val_size,
                            "Indirect2Register value size mismatch. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
                        match *dst_val_size {
                            1 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rb(dst_reg), BYTE [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                            2 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rw(dst_reg), WORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                            4 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rd(dst_reg), DWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                            8 => dynasm!(asm
                                    ; mov Rq(TEMP_REG_PRIMARY), QWORD live_vars_buffer.ptr as i64
                                    ; mov Rq(dst_reg), QWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset]),
                            _ => panic!(
                                "Unexpected Indirect to Register value size: {}",
                                src_val_size
                            ),
                        }
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                    }
                    _ => panic!(
                        "Unexpected target for Indirect source location - src: {:?}, dst: {:?}",
                        src_location, dst_location
                    ),
                }
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

fn allocate_buffer(
    src_rec: &Record,
    cp_direction: CPTransitionDirection,
) -> Option<&ThreadSafeBuffer> {
    let src_val_buffer_size = calculate_live_vars_buffer_size(src_rec);

    if src_val_buffer_size == 0 {
        return None;
    }

    let buffer_cell = if cp_direction == CPTransitionDirection::UnoptToOpt {
        &OPT_BUFFER
    } else {
        &UNOPT_BUFFER
    };

    // Get the buffer - either from the OnceLock or create it
    let thread_safe_buffer = buffer_cell.get_or_init(|| {
        if *CP_VERBOSE {
            println!("Allocating new buffer for direction {:?}", cp_direction);
        }
        unsafe {
            let layout = Layout::from_size_align(src_val_buffer_size as usize, 16).unwrap();
            let ptr = alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            ThreadSafeBuffer::new(ptr, layout, src_val_buffer_size)
        }
    });
    return Some(thread_safe_buffer);
}
pub(crate) fn copy_live_vars_to_temp_buffer(
    asm: &mut Assembler,
    src_rec: &Record,
    cp_direction: CPTransitionDirection,
) -> LiveVarsBuffer {
    let thread_safe_buffer = allocate_buffer(src_rec, cp_direction);
    if thread_safe_buffer.is_none() {
        return LiveVarsBuffer {
            ptr: 0 as *mut u8,
            layout: Layout::new::<u8>(),
            variables: HashMap::new(),
            size: 0,
        };
    }
    if *CP_VERBOSE {
        if let Some(buffer) = thread_safe_buffer {
            println!(
                "Using buffer at {:p} for direction {:?}",
                buffer.ptr, cp_direction
            );
        }
    }

    let temp_live_vars_buffer = thread_safe_buffer.unwrap();
    let mut src_var_indirect_index = 0;
    let mut variables = HashMap::new();
    // Set up the pointer to the temporary buffer
    dynasm!(asm
        ; mov Rq(TEMP_REG_PRIMARY), QWORD temp_live_vars_buffer.ptr as i64
    );
    for (_, src_var) in src_rec.live_vars.iter().enumerate() {
        let src_location = src_var.get(0).unwrap();
        match src_location {
            Indirect(_, src_off, src_val_size) => {
                // TODO: handle different value sizes
                assert!(
                    *src_val_size == 8,
                    "Only 8-byte Indirect values supported in this example"
                );
                let temp_buffer_offset = (src_var_indirect_index * (*src_val_size as i32)) as i32;
                dynasm!(asm
                    ; mov Rq(TEMP_REG_SECONDARY), QWORD [rbp + i32::try_from(*src_off).unwrap()]
                    ; mov QWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset], Rq(TEMP_REG_SECONDARY)
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
    LiveVarsBuffer {
        ptr: temp_live_vars_buffer.ptr,
        layout: temp_live_vars_buffer.layout,
        variables: variables,
        size: temp_live_vars_buffer.size,
    }
}

#[cfg(test)]
mod live_vars_tests {
    use super::*;
    use crate::trace::swt::cfg::{REG64_BYTESIZE, REG_OFFSETS};
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
    fn test_register_to_register_restore_temp_registers() {
        let src_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Register(14, 8, vec![])]),
                LiveVar::new(vec![Location::Register(13, 8, vec![-80, -200])]),
                LiveVar::new(vec![Location::Register(15, 8, vec![-72])]),
                LiveVar::new(vec![Location::Register(12, 8, vec![-56])]),
                LiveVar::new(vec![Location::Register(0, 8, vec![8, -16, -88])]),
                LiveVar::new(vec![Location::Register(3, 8, vec![-64])]),
            ],
        };

        let dst_rec = Record {
            offset: 0,
            size: 0,
            id: 0,
            live_vars: vec![
                LiveVar::new(vec![Location::Register(13, 8, vec![])]),
                LiveVar::new(vec![Location::Register(14, 8, vec![-80])]),
                LiveVar::new(vec![Location::Register(12, 8, vec![-64])]),
                LiveVar::new(vec![Location::Register(15, 8, vec![-72])]),
                LiveVar::new(vec![Location::Register(0, 8, vec![-16])]),
                LiveVar::new(vec![Location::Register(3, 8, vec![-88, -8])]),
            ],
        };
        let mut asm = Assembler::new().unwrap();
        let temp_live_vars_buffer = LiveVarsBuffer {
            ptr: std::ptr::null_mut(),
            layout: Layout::new::<u8>(),
            variables: HashMap::new(),
            size: 0,
        };

        let dest_reg_nums =
            set_destination_live_vars(&mut asm, &src_rec, &dst_rec, 0, temp_live_vars_buffer);
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions.len(), 18);
        // r14 -> r13
        assert_eq!(
            instructions[0],
            format!(
                "mov r13, qword ptr [rbp + {}]",
                REG_OFFSETS.get(&14).unwrap()
            )
        );
        // r13 -> r14 - additional location
        assert_eq!(
            instructions[1],
            format!(
                "mov rax, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&13).unwrap()
            )
        );
        assert_eq!(
            instructions[2],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 80)
        );
        // r13 -> r14
        assert_eq!(
            instructions[3],
            format!(
                "mov r14, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&13).unwrap()
            )
        );
        // r15 -> r12 - additional location
        assert_eq!(instructions[4], "mov rax, qword ptr [rbp]");
        assert_eq!(
            instructions[5],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 64)
        );
        // r15 -> r12
        assert_eq!(instructions[6], "mov r12, qword ptr [rbp]");
        // r12 -> r15 - additional location
        assert_eq!(
            instructions[7],
            format!(
                "mov rax, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&12).unwrap()
            )
        );
        assert_eq!(
            instructions[8],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 72)
        );
        // r12 -> r15
        assert_eq!(
            instructions[9],
            format!(
                "mov r15, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&12).unwrap()
            )
        );
        // rbx -> rbx - additional location -88
        assert_eq!(
            instructions[10],
            format!(
                "mov rax, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&3).unwrap()
            )
        );
        assert_eq!(
            instructions[11],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 88)
        );
        assert_eq!(
            instructions[12],
            format!(
                "mov rax, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&3).unwrap()
            )
        );
        // rbx -> rbx - additional location -8
        assert_eq!(
            instructions[13],
            format!("mov qword ptr [rbp - {}], rax", 8)
        );
        // // rbx -> rbx
        assert_eq!(
            instructions[14],
            format!(
                "mov rbx, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&3).unwrap()
            )
        );
        // rax -> rax - additional location -16
        assert_eq!(
            instructions[15],
            format!(
                "mov rax, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&0).unwrap()
            )
        );
        assert_eq!(
            instructions[16],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 16)
        );
        assert_eq!(
            instructions[17],
            format!(
                "mov rax, qword ptr [rbp + 0x{0:x}]",
                REG_OFFSETS.get(&0).unwrap()
            )
        );
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

        assert_eq!(format!("movabs rax, 0x{:x}", ptr as i64), instructions[0]);
        assert_eq!("mov r15, qword ptr [rax + riz + 8]", instructions[1]);
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

        assert_eq!(format!("movabs rax, 0x{:x}", ptr as i64), instructions[0]);
        assert_eq!("mov rax, qword ptr [rax + riz + 8]", instructions[1]);
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
        let lvb =
            copy_live_vars_to_temp_buffer(&mut asm, &src_rec, CPTransitionDirection::UnoptToOpt);
        assert_eq!(32, lvb.size);
        assert_eq!(3, lvb.variables.len());

        // Finalise and disassemble the code.
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);

        assert_eq!(
            format!("movabs rax, 0x{:x}", lvb.ptr as i64),
            instructions[0]
        );
        // 1st indirect
        assert_eq!("mov rcx, qword ptr [rbp + 0x38]", instructions[1]);
        assert_eq!("mov qword ptr [rax + riz], rcx", instructions[2]);
        // 2nd indirect
        assert_eq!("mov rcx, qword ptr [rbp + 0x48]", instructions[3]);
        assert_eq!("mov qword ptr [rax + riz + 8], rcx", instructions[4]);
        // 3rd indirect
        assert_eq!("mov rcx, qword ptr [rbp + 0xac]", instructions[5]);
        assert_eq!("mov qword ptr [rax + riz + 0x10], rcx", instructions[6]);
    }
}
