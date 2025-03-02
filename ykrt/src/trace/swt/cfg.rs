use std::alloc::Layout;
use std::collections::HashMap;
use std::env;
use std::sync::LazyLock;
/// The size of a 64-bit register in bytes.
pub(crate) static REG64_BYTESIZE: u64 = 8;

// Flag for verbose logging
pub(crate) static CP_VERBOSE: LazyLock<bool> =
    LazyLock::new(|| env::var("CP_VERBOSE").map(|v| v == "1").unwrap_or(false));

// Flag for asm breakpoints
pub(crate) static CP_BREAK: LazyLock<bool> =
    LazyLock::new(|| env::var("CP_BREAK").map(|v| v == "1").unwrap_or(false));

// Maps DWARF register numbers to `dynasm` register numbers.
// This function takes a DWARF register number as input and returns the
// corresponding dynasm register number1. The mapping is based on the
// x86_64 architecture, and it's important to note that some registers
// (rsi, rdi, rbp, and rsp) have a slightly different order in dynasm
// compared to their sequential DWARF numbering.
// https://docs.rs/dynasmrt/latest/dynasmrt/x64/enum.Rq.html
pub(crate) fn dwarf_to_dynasm_reg(dwarf_reg_num: u8) -> u8 {
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
pub(crate) static REG_OFFSETS: LazyLock<HashMap<u16, i32>> = LazyLock::new(|| {
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

pub(crate) fn reg_num_to_ykrt_control_point_rsp_offset(dwarf_reg_num: u16) -> i32 {
    REG_OFFSETS
        .get(&dwarf_reg_num)
        .copied()
        .unwrap_or_else(|| panic!("Unsupported register {}", dwarf_reg_num))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LiveVarsBuffer {
    pub ptr: *mut u8,
    pub layout: Layout,
    pub variables: HashMap<i32, i32>,
    pub size: i32,
}

#[cfg(test)]
mod cfg_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Unsupported DWARF register number:")]
    fn test_dwarf_to_dynasm_reg_invalid() {
        // Passing an invalid register number should panic.
        let _ = dwarf_to_dynasm_reg(100);
    }

    #[test]
    #[should_panic(expected = "Unsupported register")]
    fn test_reg_num_to_ykrt_control_point_rsp_offset_invalid() {
        let _ = reg_num_to_ykrt_control_point_rsp_offset(42);
    }
}
