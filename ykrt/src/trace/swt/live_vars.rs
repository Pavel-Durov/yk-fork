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
static TEMP_REG_PRIMARY: u8 = 0; // RAX
                                 // Secondary temporary register - used in buffer copy.
static TEMP_REG_SECONDARY: u8 = 1; // RDX

#[derive(Debug, Clone)]
struct RestoreTempRegisters<'a> {
    src_location: &'a yksmp::Location,
    dst_location: &'a yksmp::Location,
    src_var_indirect_index: i32,
}

// Define structs for each memory operation to make parameter ordering clear
struct MemToRegParams {
    src_ptr: i64,
    src_offset: i32,
    dst_reg: u8,
    size: u16,
}

struct MemToMemParams {
    src_ptr: i64,
    src_offset: i32,
    dst_offset: i32,
    size: u16,
}

struct RbpToRegParams {
    rbp_offset: i32,
    dst_reg: u8,
    size: u16,
}

struct RegToRbpParams {
    src_reg: u8,
    rbp_offset: i32,
    size: u16,
}

// Helper function to generate assembly for memory-to-register operations
fn emit_mem_to_reg(asm: &mut Assembler, params: MemToRegParams) {
    match params.size {
        1 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rb(params.dst_reg), BYTE [Rq(TEMP_REG_PRIMARY) + params.src_offset]),
        2 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rw(params.dst_reg), WORD [Rq(TEMP_REG_PRIMARY) + params.src_offset]),
        4 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rd(params.dst_reg), DWORD [Rq(TEMP_REG_PRIMARY) + params.src_offset]),
        8 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rq(params.dst_reg), QWORD [Rq(TEMP_REG_PRIMARY) + params.src_offset]),
        _ => panic!("Unsupported value size: {}", params.size),
    }
}

// Helper function to generate assembly for memory-to-memory operations
fn emit_mem_to_mem(asm: &mut Assembler, params: MemToMemParams) {
    match params.size {
        1 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rb(TEMP_REG_SECONDARY), BYTE [Rq(TEMP_REG_PRIMARY) + params.src_offset]
            ; mov BYTE [rbp + params.dst_offset], Rb(TEMP_REG_SECONDARY)
        ),
        2 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rw(TEMP_REG_SECONDARY), WORD [Rq(TEMP_REG_PRIMARY) + params.src_offset]
            ; mov WORD [rbp + params.dst_offset], Rw(TEMP_REG_SECONDARY)
        ),
        4 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rd(TEMP_REG_SECONDARY), DWORD [Rq(TEMP_REG_PRIMARY) + params.src_offset]
            ; mov DWORD [rbp + params.dst_offset], Rd(TEMP_REG_SECONDARY)
        ),
        8 => dynasm!(asm
            ; mov Rq(TEMP_REG_PRIMARY), QWORD params.src_ptr
            ; mov Rq(TEMP_REG_SECONDARY), QWORD [Rq(TEMP_REG_PRIMARY) + params.src_offset]
            ; mov QWORD [rbp + params.dst_offset], Rq(TEMP_REG_SECONDARY)
        ),
        _ => panic!("Unsupported value size: {}", params.size),
    }
}

// Helper function to generate assembly for rbp-relative register loads
fn emit_rbp_to_reg(asm: &mut Assembler, params: RbpToRegParams) {
    match params.size {
        1 => dynasm!(asm; mov Rb(params.dst_reg), BYTE [rbp - params.rbp_offset]),
        2 => dynasm!(asm; mov Rw(params.dst_reg), WORD [rbp - params.rbp_offset]),
        4 => dynasm!(asm; mov Rd(params.dst_reg), DWORD [rbp - params.rbp_offset]),
        8 => dynasm!(asm; mov Rq(params.dst_reg), QWORD [rbp - params.rbp_offset]),
        _ => panic!("Unsupported value size: {}", params.size),
    }
}

// Helper function to generate assembly for rbp-relative register stores
fn emit_reg_to_rbp(asm: &mut Assembler, params: RegToRbpParams) {
    match params.size {
        1 => dynasm!(asm; mov BYTE [rbp + params.rbp_offset], Rb(params.src_reg)),
        2 => dynasm!(asm; mov WORD [rbp + params.rbp_offset], Rw(params.src_reg)),
        4 => dynasm!(asm; mov DWORD [rbp + params.rbp_offset], Rd(params.src_reg)),
        8 => dynasm!(asm; mov QWORD [rbp + params.rbp_offset], Rq(params.src_reg)),
        _ => panic!("Unsupported value size: {}", params.size),
    }
}

// Handles additional locations for register-to-register.
fn handle_register_to_register_additional_locations(
    asm: &mut dynasmrt::Assembler<dynasmrt::x64::X64Relocation>,
    src_reg_val_rbp_offset: i32,
    dst_add_locs: &Vec<i16>,
    src_val_size: &u16,
    dst_reg_num: &u16,
    dest_reg_nums: &mut HashMap<u16, u16>,
) {
    for location in dst_add_locs {
        // Write any additional locations that were tracked for this variable.
        // Numbers greater or equal to zero are registers in Dwarf notation.
        // Negative numbers are offsets relative to RBP.
        if *location >= 0 {
            dest_reg_nums.insert(*location as u16, *src_val_size);
            let dst_reg = dwarf_to_dynasm_reg((*location).try_into().unwrap());
            emit_rbp_to_reg(
                asm,
                RbpToRegParams {
                    rbp_offset: src_reg_val_rbp_offset,
                    dst_reg,
                    size: *src_val_size,
                },
            );
        } else {
            let rbp_offset = i32::try_from(*location).unwrap();
            // Load from source register and store to destination rbp offset
            emit_rbp_to_reg(
                asm,
                RbpToRegParams {
                    rbp_offset: src_reg_val_rbp_offset,
                    dst_reg: TEMP_REG_PRIMARY,
                    size: *src_val_size,
                },
            );
            emit_reg_to_rbp(
                asm,
                RegToRbpParams {
                    src_reg: TEMP_REG_PRIMARY,
                    rbp_offset,
                    size: *src_val_size,
                },
            );
        }
    }
}

// Handles additional locations for indirect-to-register.
fn handle_indirect_to_register_additional_locations(
    asm: &mut dynasmrt::Assembler<dynasmrt::x64::X64Relocation>,
    dst_add_locs: &Vec<i16>,
    src_val_size: &u16,
    dst_reg_num: &u16,
    temp_buffer_offset: i32,
    live_vars_buffer: &LiveVarsBuffer,
    dest_reg_nums: &mut HashMap<u16, u16>,
) {
    for location in dst_add_locs {
        // Write any additional locations that were tracked for this variable.
        // Numbers greater or equal to zero are registers in Dwarf notation.
        // Negative numbers are offsets relative to RBP.
        if *location >= 0 {
            dest_reg_nums.insert(*location as u16, *src_val_size);
            let dst_reg = dwarf_to_dynasm_reg((*location).try_into().unwrap());
            emit_mem_to_reg(
                asm,
                MemToRegParams {
                    src_ptr: live_vars_buffer.ptr as i64,
                    src_offset: temp_buffer_offset,
                    dst_reg,
                    size: *src_val_size,
                },
            );
        } else {
            let rbp_offset = i32::try_from(*location).unwrap();
            emit_mem_to_mem(
                asm,
                MemToMemParams {
                    src_ptr: live_vars_buffer.ptr as i64,
                    src_offset: temp_buffer_offset,
                    dst_offset: rbp_offset,
                    size: *src_val_size,
                },
            );
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
    let mut used_temp_registers = Vec::new();
    // Index of the source live variable in the temporary buffer.
    let mut src_var_indirect_index = 0;

    // Ensure we have matching live variables
    assert!(
        src_rec.live_vars.len() == dst_rec.live_vars.len(),
        "Source and destination live variable counts don't match: src={}, dst={}",
        src_rec.live_vars.len(),
        dst_rec.live_vars.len()
    );

    for (index, src_var) in src_rec.live_vars.iter().enumerate() {
        let dst_var = &dst_rec.live_vars[index];
        if src_var.len() > 1 || dst_var.len() > 1 {
            todo!("Deal with multi register locations");
        }

        let src_location = &src_var.get(0).unwrap();
        let dst_location = &dst_var.get(0).unwrap();

        match src_location {
            Register(src_reg_num, src_val_size, src_add_locs) => {
                let src_reg_offset = reg_num_to_ykrt_control_point_rsp_offset(*src_reg_num);
                let src_reg_val_rbp_offset =
                    i32::try_from(rbp_offset_reg_store - src_reg_offset as i64).unwrap();

                match dst_location {
                    Register(dst_reg_num, dst_val_size, dst_add_locs) => {
                        if *dst_reg_num == TEMP_REG_PRIMARY.into()
                            || *dst_reg_num == TEMP_REG_SECONDARY.into()
                        {
                            used_temp_registers.push(RestoreTempRegisters {
                                src_location: src_location,
                                dst_location: dst_location,
                                src_var_indirect_index: src_var_indirect_index,
                            });
                        } else {
                            // Handle additional locations for both source and destination
                            handle_register_to_register_additional_locations(
                                asm,
                                src_reg_val_rbp_offset,
                                dst_add_locs,
                                src_val_size,
                                dst_reg_num,
                                &mut dest_reg_nums,
                            );

                            assert!(
                                dst_val_size == src_val_size,
                                "Register2Register - src and dst val size must match. Got src: {} and dst: {}",
                                src_val_size,
                                dst_val_size
                            );
                            if *CP_VERBOSE {
                                println!(
                                    "Register2Register - src: {:?} dst: {:?}",
                                    src_location, dst_location
                                );
                            }
                            dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                            let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                            emit_rbp_to_reg(
                                asm,
                                RbpToRegParams {
                                    rbp_offset: src_reg_val_rbp_offset,
                                    dst_reg,
                                    size: *src_val_size,
                                },
                            );
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
                        // First load from rbp-relative source
                        emit_rbp_to_reg(
                            asm,
                            RbpToRegParams {
                                rbp_offset: src_reg_val_rbp_offset,
                                dst_reg: TEMP_REG_PRIMARY,
                                size: *src_val_size,
                            },
                        );
                        // Then store to rbp-relative destination
                        emit_reg_to_rbp(
                            asm,
                            RegToRbpParams {
                                src_reg: TEMP_REG_PRIMARY,
                                rbp_offset: i32::try_from(*dst_off).unwrap(),
                                size: *src_val_size,
                            },
                        );
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
                            used_temp_registers.push(RestoreTempRegisters {
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
                            assert!(*src_reg_num == 6, "Indirect register is expected to be rbp");
                            // Set register additional locations
                            handle_indirect_to_register_additional_locations(
                                asm,
                                dst_add_locs,
                                src_val_size,
                                dst_reg_num,
                                temp_buffer_offset,
                                &live_vars_buffer,
                                &mut dest_reg_nums,
                            );
                            dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                            let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                            assert!(
                                *dst_val_size == *src_val_size,
                                "Indirect2Register value size mismatch. Got src: {} and dst: {}",
                                src_val_size,
                                dst_val_size
                            );
                            emit_mem_to_reg(
                                asm,
                                MemToRegParams {
                                    src_ptr: live_vars_buffer.ptr as i64,
                                    src_offset: temp_buffer_offset,
                                    dst_reg,
                                    size: *dst_val_size,
                                },
                            );
                        }
                    }
                    Indirect(_dst_reg_num, dst_off, dst_val_size) => {
                        if *CP_VERBOSE {
                            println!(
                                "Indirect2Indirect - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }

                        // NOTE: Sometimes source and destination sizes are different, so we use the minimum
                        let size = if *src_val_size != *dst_val_size {
                            if *CP_VERBOSE {
                                println!(
                                    "WARNING: Indirect2Indirect size mismatch - Source: {} (at offset {}) Destination: {} (at offset {}). Using min size.",
                                    src_val_size, temp_buffer_offset, dst_val_size, dst_off
                                );
                            }
                            std::cmp::min(*src_val_size, *dst_val_size)
                        } else {
                            *src_val_size
                        };

                        emit_mem_to_mem(
                            asm,
                            MemToMemParams {
                                src_ptr: live_vars_buffer.ptr as i64,
                                src_offset: temp_buffer_offset,
                                dst_offset: i32::try_from(*dst_off).unwrap(),
                                size,
                            },
                        );
                    }
                    _ => panic!(
                        "Unexpected target for Indirect source location - src: {:?}, dst: {:?}",
                        src_location, dst_location
                    ),
                }
                src_var_indirect_index += 1;
            }
            Direct(_, _, _) => { /* Do nothing */ }
            _ => panic!("Unexpected source location: {:?}", src_location),
        }
    }
    // Restore the temporary registers if any were used.
    for temp_register in used_temp_registers {
        let src_location = temp_register.src_location;
        let dst_location = temp_register.dst_location;
        let src_var_indirect_index = temp_register.src_var_indirect_index;

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
                            &mut dest_reg_nums,
                        );
                        assert!(
                            *dst_val_size == *src_val_size,
                            "Register2Register - src and dst val size must match. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
                        if *CP_VERBOSE {
                            println!(
                                "Register2Register - src: {:?} dst: {:?}",
                                src_location, dst_location
                            );
                        }
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                        let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                        assert!(
                            *dst_val_size == *src_val_size,
                            "Indirect2Register value size mismatch. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
                        match *src_val_size {
                            1 => dynasm!(asm; mov Rb(dst_reg), BYTE [rbp - src_reg_val_rbp_offset]),
                            2 => dynasm!(asm; mov Rw(dst_reg), WORD [rbp - src_reg_val_rbp_offset]),
                            4 => {
                                dynasm!(asm; mov Rd(dst_reg), DWORD [rbp - src_reg_val_rbp_offset])
                            }
                            8 => {
                                dynasm!(asm; mov Rq(dst_reg), QWORD [rbp - src_reg_val_rbp_offset])
                            }
                            _ => {
                                panic!("unexpect Register to Register value size {}", src_val_size)
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
                            &mut dest_reg_nums,
                        );
                        dest_reg_nums.insert(*dst_reg_num, *dst_val_size);
                        let dst_reg = dwarf_to_dynasm_reg((*dst_reg_num).try_into().unwrap());
                        assert!(
                            *dst_val_size == *src_val_size,
                            "Indirect2Register value size mismatch. Got src: {} and dst: {}",
                            src_val_size,
                            dst_val_size
                        );
                        emit_mem_to_reg(
                            asm,
                            MemToRegParams {
                                src_ptr: live_vars_buffer.ptr as i64,
                                src_offset: temp_buffer_offset,
                                dst_reg,
                                size: *dst_val_size,
                            },
                        );
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

// Calculates the size of the live vars buffer.
// The buffer is aligned to 16 bytes.
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

// Allocates a temporary buffer for the live vars.
// This allocation happens only once per direction.
// The buffer is aligned to 16 bytes.
fn allocate_buffer(
    src_rec: &Record,
    cp_direction: CPTransitionDirection,
) -> Option<&ThreadSafeBuffer> {
    let src_val_buffer_size = calculate_live_vars_buffer_size(src_rec);

    if src_val_buffer_size == 0 {
        return None;
    }

    let buffer_cell = match cp_direction {
        CPTransitionDirection::UnoptToOpt => &OPT_BUFFER,
        CPTransitionDirection::OptToUnopt => &UNOPT_BUFFER,
    };

    // Get the buffer - either from the OnceLock or create it
    let thread_safe_buffer = buffer_cell.get_or_init(|| unsafe {
        let layout = Layout::from_size_align(src_val_buffer_size as usize, 16)
            .expect("Failed to create layout for live vars buffer");
        let ptr = alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        if *CP_VERBOSE {
            println!(
                "Allocated buffer of size {} at {:p} for direction {:?}",
                src_val_buffer_size, ptr, cp_direction
            );
        }
        ThreadSafeBuffer::new(ptr, layout, src_val_buffer_size)
    });

    Some(thread_safe_buffer)
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
                // Calculate offset in buffer for this variable
                let temp_buffer_offset = (src_var_indirect_index * (*src_val_size as i32)) as i32;
                let src_rbp_offset = i32::try_from(*src_off).unwrap();

                // Different handling based on size
                match *src_val_size {
                    1 => dynasm!(asm
                        ; mov Rb(TEMP_REG_SECONDARY), BYTE [rbp + src_rbp_offset]
                        ; mov BYTE [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset], Rb(TEMP_REG_SECONDARY)
                    ),
                    2 => dynasm!(asm
                        ; mov Rw(TEMP_REG_SECONDARY), WORD [rbp + src_rbp_offset]
                        ; mov WORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset], Rw(TEMP_REG_SECONDARY)
                    ),
                    4 => dynasm!(asm
                        ; mov Rd(TEMP_REG_SECONDARY), DWORD [rbp + src_rbp_offset]
                        ; mov DWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset], Rd(TEMP_REG_SECONDARY)
                    ),
                    8 => dynasm!(asm
                        ; mov Rq(TEMP_REG_SECONDARY), QWORD [rbp + src_rbp_offset]
                        ; mov QWORD [Rq(TEMP_REG_PRIMARY) + temp_buffer_offset], Rq(TEMP_REG_SECONDARY)
                    ),
                    _ => panic!("Unsupported value size in temporary copy: {}", src_val_size),
                }

                variables.insert(src_var_indirect_index, temp_buffer_offset);
                src_var_indirect_index += 1;
            }
            Direct(_, _, _) => {
                // DO NOTHING - Direct variables don't need buffer storage
            }
            Register(_, _, _) => {
                // DO NOTHING - Register variables don't need buffer storage
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
        variables,
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

    // New tests for helper functions
    #[test]
    fn test_emit_mem_to_reg() {
        for size in [1, 2, 4, 8].iter() {
            let mut asm = Assembler::new().unwrap();
            let test_ptr = 0x1234567890ABCDEF;
            let test_offset = 42;
            let test_dst_reg = 15; // r15

            emit_mem_to_reg(
                &mut asm,
                MemToRegParams {
                    src_ptr: test_ptr,
                    src_offset: test_offset,
                    dst_reg: test_dst_reg,
                    size: *size,
                },
            );

            let buffer = asm.finalize().unwrap();
            let instructions = get_asm_instructions(&buffer);

            assert_eq!(
                instructions.len(),
                2,
                "Should have exactly 2 instructions for size {}",
                size
            );

            assert_eq!(
                instructions[0],
                format!("movabs rax, 0x{:x}", test_ptr),
                "First instruction should load the pointer for size {}",
                size
            );

            let expected_second = match size {
                1 => format!("mov r15b, byte ptr [rax + riz + 0x{:x}]", test_offset),
                2 => format!("mov r15w, word ptr [rax + riz + 0x{:x}]", test_offset),
                4 => format!("mov r15d, dword ptr [rax + riz + 0x{:x}]", test_offset),
                8 => format!("mov r15, qword ptr [rax + riz + 0x{:x}]", test_offset),
                _ => unreachable!(),
            };

            assert_eq!(
                instructions[1], expected_second,
                "Second instruction should load value of size {} into register",
                size
            );
        }
    }

    #[test]
    fn test_emit_mem_to_mem() {
        for size in [1, 2, 4, 8].iter() {
            let mut asm = Assembler::new().unwrap();
            let test_ptr = 0x1234567890ABCDEF;
            let test_src_offset = 42;
            let test_dst_offset = 24;

            emit_mem_to_mem(
                &mut asm,
                MemToMemParams {
                    src_ptr: test_ptr,
                    src_offset: test_src_offset,
                    dst_offset: test_dst_offset,
                    size: *size,
                },
            );

            let buffer = asm.finalize().unwrap();
            let instructions = get_asm_instructions(&buffer);

            assert_eq!(
                instructions.len(),
                3,
                "Should have exactly 3 instructions for size {}",
                size
            );

            assert_eq!(
                instructions[0],
                format!("movabs rax, 0x{:x}", test_ptr),
                "First instruction should load the pointer for size {}",
                size
            );

            let expected_second = match size {
                1 => format!("mov cl, byte ptr [rax + riz + 0x{:x}]", test_src_offset),
                2 => format!("mov cx, word ptr [rax + riz + 0x{:x}]", test_src_offset),
                4 => format!("mov ecx, dword ptr [rax + riz + 0x{:x}]", test_src_offset),
                8 => format!("mov rcx, qword ptr [rax + riz + 0x{:x}]", test_src_offset),
                _ => unreachable!(),
            };

            assert_eq!(
                instructions[1], expected_second,
                "Second instruction should load value of size {} into temp register",
                size
            );

            let expected_third = match size {
                1 => format!("mov byte ptr [rbp + 0x{:x}], cl", test_dst_offset),
                2 => format!("mov word ptr [rbp + 0x{:x}], cx", test_dst_offset),
                4 => format!("mov dword ptr [rbp + 0x{:x}], ecx", test_dst_offset),
                8 => format!("mov qword ptr [rbp + 0x{:x}], rcx", test_dst_offset),
                _ => unreachable!(),
            };

            assert_eq!(
                instructions[2], expected_third,
                "Third instruction should store value of size {} to memory",
                size
            );
        }
    }

    #[test]
    fn test_emit_rbp_to_reg() {
        for size in [1, 2, 4, 8].iter() {
            let mut asm = Assembler::new().unwrap();
            let test_rbp_offset = 64;
            let test_dst_reg = 15; // r15

            emit_rbp_to_reg(
                &mut asm,
                RbpToRegParams {
                    rbp_offset: test_rbp_offset,
                    dst_reg: test_dst_reg,
                    size: *size,
                },
            );

            let buffer = asm.finalize().unwrap();
            let instructions = get_asm_instructions(&buffer);

            assert_eq!(
                instructions.len(),
                1,
                "Should have exactly 1 instruction for size {}",
                size
            );

            let expected = match size {
                1 => format!("mov r15b, byte ptr [rbp - 0x{:x}]", test_rbp_offset),
                2 => format!("mov r15w, word ptr [rbp - 0x{:x}]", test_rbp_offset),
                4 => format!("mov r15d, dword ptr [rbp - 0x{:x}]", test_rbp_offset),
                8 => format!("mov r15, qword ptr [rbp - 0x{:x}]", test_rbp_offset),
                _ => unreachable!(),
            };

            assert_eq!(
                instructions[0], expected,
                "Instruction should load value of size {} from rbp-relative address into register",
                size
            );
        }
    }

    #[test]
    fn test_emit_reg_to_rbp() {
        for size in [1, 2, 4, 8].iter() {
            let mut asm = Assembler::new().unwrap();
            let test_rbp_offset = 64;
            let test_src_reg = 15; // r15

            emit_reg_to_rbp(
                &mut asm,
                RegToRbpParams {
                    src_reg: test_src_reg,
                    rbp_offset: test_rbp_offset,
                    size: *size,
                },
            );

            let buffer = asm.finalize().unwrap();
            let instructions = get_asm_instructions(&buffer);

            assert_eq!(
                instructions.len(),
                1,
                "Should have exactly 1 instruction for size {}",
                size
            );

            let expected = match size {
                1 => format!("mov byte ptr [rbp + 0x{:x}], r15b", test_rbp_offset),
                2 => format!("mov word ptr [rbp + 0x{:x}], r15w", test_rbp_offset),
                4 => format!("mov dword ptr [rbp + 0x{:x}], r15d", test_rbp_offset),
                8 => format!("mov qword ptr [rbp + 0x{:x}], r15", test_rbp_offset),
                _ => unreachable!(),
            };

            assert_eq!(
                instructions[0], expected,
                "Instruction should store value of size {} from register to rbp-relative address",
                size
            );
        }
    }

    #[test]
    #[should_panic(expected = "Unsupported value size")]
    fn test_emit_mem_to_reg_invalid_size() {
        let mut asm = Assembler::new().unwrap();
        emit_mem_to_reg(
            &mut asm,
            MemToRegParams {
                src_ptr: 0x1234,
                src_offset: 0,
                dst_reg: 15,
                size: 3, // Invalid size
            },
        );
    }

    #[test]
    #[should_panic(expected = "Unsupported value size")]
    fn test_emit_mem_to_mem_invalid_size() {
        let mut asm = Assembler::new().unwrap();
        emit_mem_to_mem(
            &mut asm,
            MemToMemParams {
                src_ptr: 0x1234,
                src_offset: 0,
                dst_offset: 0,
                size: 16, // Invalid size
            },
        );
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
    fn test_register_to_register_with_additional_location_indirect() {
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

        let rbp_offset_reg_store: i32 = 200;
        let dest_reg_nums = set_destination_live_vars(
            &mut asm,
            &src_rec,
            &dst_rec,
            rbp_offset_reg_store as i64,
            temp_live_vars_buffer,
        );
        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);
        assert_eq!(instructions.len(), 18);
        dbg!(&instructions);
        // r14 -> r13
        assert_eq!(
            instructions[0],
            format!(
                "mov r13, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&14).unwrap()
            )
        );
        // r13 -> r14 - additional location
        assert_eq!(
            instructions[1],
            format!(
                "mov rax, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&13).unwrap()
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
                "mov r14, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&13).unwrap()
            )
        );
        // r15 -> r12 - additional location
        assert_eq!(
            instructions[4],
            format!(
                "mov rax, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&15).unwrap()
            )
        );
        assert_eq!(
            instructions[5],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 64)
        );
        // r15 -> r12
        assert_eq!(
            instructions[6],
            format!(
                "mov r12, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&15).unwrap()
            )
        );
        // r12 -> r15 - additional location
        assert_eq!(
            instructions[7],
            format!(
                "mov rax, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&12).unwrap()
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
                "mov r15, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&12).unwrap()
            )
        );
        // rbx -> rbx - additional location -88
        assert_eq!(
            instructions[10],
            format!(
                "mov rax, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&3).unwrap()
            )
        );
        assert_eq!(
            instructions[11],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 88)
        );
        assert_eq!(
            instructions[12],
            format!(
                "mov rax, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&3).unwrap()
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
                "mov rbx, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&3).unwrap()
            )
        );
        // rax -> rax - additional location -16
        assert_eq!(
            instructions[15],
            format!(
                "mov rax, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&0).unwrap()
            )
        );
        assert_eq!(
            instructions[16],
            format!("mov qword ptr [rbp - 0x{0:x}], rax", 16)
        );
        assert_eq!(
            instructions[17],
            format!(
                "mov rax, qword ptr [rbp - 0x{0:x}]",
                rbp_offset_reg_store - REG_OFFSETS.get(&0).unwrap()
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
        assert_eq!("mov rcx, qword ptr [rax + riz + 8]", instructions[1]);
        assert_eq!("mov qword ptr [rbp + 6], rcx", instructions[2]);
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

    #[test]
    #[should_panic(expected = "Unsupported value size")]
    fn test_emit_rbp_to_reg_invalid_size() {
        let mut asm = Assembler::new().unwrap();
        emit_rbp_to_reg(
            &mut asm,
            RbpToRegParams {
                rbp_offset: 64,
                dst_reg: 15,
                size: 3, // Invalid size
            },
        );
    }

    #[test]
    #[should_panic(expected = "Unsupported value size")]
    fn test_emit_reg_to_rbp_invalid_size() {
        let mut asm = Assembler::new().unwrap();
        emit_reg_to_rbp(
            &mut asm,
            RegToRbpParams {
                src_reg: 15,
                rbp_offset: 64,
                size: 16, // Invalid size
            },
        );
    }

    #[test]
    fn test_helper_functions_with_different_registers() {
        // Test with different registers (not just r15)
        let test_registers = [0, 1, 2, 3, 7, 8, 12]; // rax, rcx, rdx, rbx, rdi, r8, r12

        for reg in test_registers.iter() {
            let mut asm = Assembler::new().unwrap();

            // Test rbp_to_reg with this register
            emit_rbp_to_reg(
                &mut asm,
                RbpToRegParams {
                    rbp_offset: 32,
                    dst_reg: *reg,
                    size: 8,
                },
            );

            let buffer = asm.finalize().unwrap();
            let instructions = get_asm_instructions(&buffer);

            let reg_name = match reg {
                0 => "rax",
                1 => "rcx",
                2 => "rdx",
                3 => "rbx",
                7 => "rdi",
                8 => "r8",
                12 => "r12",
                _ => panic!("Test register not handled"),
            };

            assert_eq!(
                instructions[0],
                format!("mov {}, qword ptr [rbp - 0x20]", reg_name),
                "Should correctly use register {}",
                reg_name
            );
        }
    }

    #[test]
    fn test_mem_to_reg_mem_edge_cases() {
        // Test with zero offset
        let mut asm = Assembler::new().unwrap();
        emit_mem_to_reg(
            &mut asm,
            MemToRegParams {
                src_ptr: 0x1234,
                src_offset: 0, // Zero offset
                dst_reg: 15,
                size: 8,
            },
        );

        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);

        assert_eq!(
            instructions[1],
            "mov r15, qword ptr [rax + riz]", // Should have no explicit offset
            "Should handle zero offset correctly"
        );

        // Test with negative offset
        let mut asm = Assembler::new().unwrap();
        emit_mem_to_mem(
            &mut asm,
            MemToMemParams {
                src_ptr: 0x1234,
                src_offset: -8, // Negative offset
                dst_offset: 16,
                size: 4,
            },
        );

        let buffer = asm.finalize().unwrap();
        let instructions = get_asm_instructions(&buffer);

        assert_eq!(
            instructions[1],
            "mov ecx, dword ptr [rax + riz - 8]", // Should have negative offset
            "Should handle negative offset correctly"
        );
    }
}
