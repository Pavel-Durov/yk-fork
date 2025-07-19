use std::alloc::Layout;
use std::collections::HashMap;
use std::env;
use std::sync::LazyLock;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlPointStackMapId {
    // unoptimised (original functions) control point stack map id
    Opt = 0,
    // optimised (cloned functions) control point stack map id
    UnOpt = 1,
}

/// The size of a 64-bit register in bytes.
pub(crate) static REG64_BYTESIZE: u64 = 8;

// Flag for verbose logging
pub static YKB_SWT_VERBOSE: LazyLock<bool> = LazyLock::new(|| {
    env::var("YKB_SWT_VERBOSE")
        .map(|v| v == "1")
        .unwrap_or(false)
});

// Flag for verbose logging of asm
pub static YKB_SWT_VERBOSE_ASM: LazyLock<bool> = LazyLock::new(|| {
    env::var("YKB_SWT_VERBOSE_ASM")
        .map(|v| v == "1")
        .unwrap_or(false)
});

// Flag for control point break
pub(crate) static YKB_SWT_CP_BREAK: LazyLock<bool> = LazyLock::new(|| {
    env::var("YKB_SWT_CP_BREAK")
        .map(|v| v == "1")
        .unwrap_or(false)
});

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
        1 => 2,   // RDX
        2 => 1,   // RCX
        3 => 3,   // RBX
        4 => 6,   // RSI
        5 => 7,   // RDI
        6 => 5,   // RBP
        7 => 4,   // RSP
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

// Mapping of DWARF register numbers to offsets in the __ykrt_control_point stack frame.
pub(crate) static REG_OFFSETS: LazyLock<HashMap<u16, i32>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(0, 0x60); // rax
    // 1 => 8,  // rdx - is not saved
    m.insert(2, 0x58); // rcx
    m.insert(3, 0x50); // rbx
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LiveVarsBuffer {
    pub ptr: *mut u8,
    pub layout: Layout,
    // varibles are only used in tests - can eb removed
    pub variables: HashMap<i32, i32>,
    pub size: i32,
}

#[cfg(test)]
mod cfg_tests {
    use super::*;

    #[test]
    fn test_dwarf_to_dynasm_reg_all_valid_registers() {
        // Test all valid DWARF register numbers and their expected DynASM mappings
        assert_eq!(dwarf_to_dynasm_reg(0), 0);   // RAX -> RAX
        assert_eq!(dwarf_to_dynasm_reg(1), 2);   // RDX -> RDX (note: different order)
        assert_eq!(dwarf_to_dynasm_reg(2), 1);   // RCX -> RCX (note: different order)
        assert_eq!(dwarf_to_dynasm_reg(3), 3);   // RBX -> RBX
        assert_eq!(dwarf_to_dynasm_reg(4), 6);   // RSI -> RSI (note: different order)
        assert_eq!(dwarf_to_dynasm_reg(5), 7);   // RDI -> RDI (note: different order)
        assert_eq!(dwarf_to_dynasm_reg(6), 5);   // RBP -> RBP (note: different order)
        assert_eq!(dwarf_to_dynasm_reg(7), 4);   // RSP -> RSP (note: different order)
        assert_eq!(dwarf_to_dynasm_reg(8), 8);   // R8 -> R8
        assert_eq!(dwarf_to_dynasm_reg(9), 9);   // R9 -> R9
        assert_eq!(dwarf_to_dynasm_reg(10), 10); // R10 -> R10
        assert_eq!(dwarf_to_dynasm_reg(11), 11); // R11 -> R11
        assert_eq!(dwarf_to_dynasm_reg(12), 12); // R12 -> R12
        assert_eq!(dwarf_to_dynasm_reg(13), 13); // R13 -> R13
        assert_eq!(dwarf_to_dynasm_reg(14), 14); // R14 -> R14
        assert_eq!(dwarf_to_dynasm_reg(15), 15); // R15 -> R15
    }

    #[test]
    #[should_panic(expected = "Unsupported DWARF register number: 16")]
    fn test_dwarf_to_dynasm_reg_invalid_16() {
    
        let _ = dwarf_to_dynasm_reg(16);
    }
}
