use crate::trace::swt::cfg::reg_num_to_ykrt_control_point_rsp_offset;
use std::arch::asm;
use yksmp::Location::{Direct, Indirect, Register};
use yksmp::Record;

pub(crate) fn get_rb_value_at_offset(rbp_offset: i32, src_val_size: &u16) -> u64 {
    // Read the actual register value from frameaddr - reg_store_rbp_offset
    // In our assembly context, this is [rbp - reg_store_rbp_offset]
    unsafe {
        // Get the current RBP value
        let rbp: *const u8;
        std::arch::asm!("mov {}, rbp", out(reg) rbp, options(nostack, preserves_flags));
        // Calculate the register store address: rbp - reg_store_rbp_offset
        // let reg_store_addr = if rbp_offset < 0 {
        //     rbp.offset(rbp_offset as isize)
        // } else {
        //     rbp.offset(-(rbp_offset as isize))
        // };
        let reg_store_addr = rbp.offset(rbp_offset as isize);
        return match *src_val_size {
            1 => {
                let val = *(reg_store_addr as *const u8);
                val as u64
            }
            2 => {
                let val = *(reg_store_addr as *const u16);
                val as u64
            }
            4 => {
                let val = *(reg_store_addr as *const u32);
                val as u64
            }
            8 => {
                let val = *(reg_store_addr as *const u64);
                val
            }
            _ => panic!("Unsupported value size: {}", src_val_size),
        };
    }
}

pub(crate) fn debug_print_source_live_vars(src_rec: &Record, rbp_offset_reg_store: i64) {
    eprintln!("Source live vars values:");
    // Print Indirect and Direct locations first.
    for (_, src_var) in src_rec.live_vals.iter().enumerate() {
        let location = src_var.get(0).unwrap();
        match location {
            Indirect(_, src_off, src_val_size) => {
                let value = get_rb_value_at_offset(*src_off, src_val_size);
                println!("{:?}\tvalue=0x{:016x}", location, value);
            }
            Register(src_reg_num, src_val_size, _src_add_locs) => {
                let reg_store_offset = reg_num_to_ykrt_control_point_rsp_offset(*src_reg_num);
                let reg_store_rbp_offset =
                    i32::try_from(rbp_offset_reg_store - reg_store_offset as i64).unwrap();
                let value = get_rb_value_at_offset(-reg_store_rbp_offset, src_val_size);
                println!("{:?}\tvalue=0x{:016x}", location, value);
            }
            Direct(_, src_off, src_val_size) => {
                let value = get_rb_value_at_offset(*src_off, src_val_size);
                println!("{:?}\tvalue=0x{:016x}", location, value);
            }
            _ => panic!("Unexpected source location: {:?}", location),
        };
    }
}

pub(crate) fn debug_print_destination_live_vars(dst_rec: &Record, _rbp_offset_reg_store: i64) {
    println!("Destination live vars values:");
    for (_, dst_var) in dst_rec.live_vals.iter().enumerate() {
        let location = dst_var.get(0).unwrap();
        match location {
            Indirect(_, dst_off, dst_val_size) => {
                let value = get_rb_value_at_offset(*dst_off, dst_val_size);
                println!("{:?}\tvalue=0x{:016x}", location, value);
            }
            Register(_, dst_val_size, dst_add_locs) => {
                // Do not print register values cause its not correct, by the time this funciton is called they might be already clobbered.
                for add_loc in dst_add_locs {
                    if *add_loc >= 0 {
                        // Do not print register values cause its not correct, by the time this funciton is called they might be already clobbered.
                        println!("{:?}\tvalue=(cannot get register value)", location);
                    } else {
                        let rbp_offset = i32::try_from(*add_loc).unwrap();
                        let value = get_rb_value_at_offset(rbp_offset, dst_val_size);
                        println!(
                            "{:?}\tvalue=0x{:016x} (based on additional location)",
                            location, value
                        );
                    }
                }
            }
            Direct(_, src_off, src_val_size) => {
                let value = get_rb_value_at_offset(*src_off, src_val_size);
                println!("{:?}\tvalue=0x{:016x}", location, value);
            }
            _ => panic!("Unexpected source location: {:?}", location),
        };
    }
}

// Utility function to print the value of a register.
// Used for debugging.
// pub unsafe extern "C" fn get_reg_value(reg_num: u16) -> u64 {
//     let reg_val: u64;
//     match reg_num {
//         0 => asm!(
//             "mov {0}, rax",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         1 => asm!(
//             "mov {0}, rcx",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         2 => asm!(
//             "mov {0}, rdx",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         3 => asm!(
//             "mov {0}, rbx",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         6 => asm!(
//             "mov {0}, rbp",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         5 => asm!(
//             "mov {0}, rdi",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         7 => asm!(
//             "mov {0}, rdi",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         8 => asm!(
//             "mov {0}, r8",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         9 => asm!(
//             "mov {0}, r9",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         10 => asm!(
//             "mov {0}, r10",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         11 => asm!(
//             "mov {0}, r11",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         12 => asm!(
//             "mov {0}, r12",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         13 => asm!(
//             "mov {0}, r13",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         14 => asm!(
//             "mov {0}, r14",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         15 => asm!(
//             "mov {0}, r15",
//             out(reg) reg_val,
//             options(nostack, nomem, preserves_flags)
//         ),
//         _ => panic!("Unsupported register number: {}", reg_num),
//     }
//     return reg_val;
// }
