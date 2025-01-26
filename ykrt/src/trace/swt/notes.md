Register Values:
RAX: 0000000000000001
RCX: 0000000000000000
RBX: 0000000000000001
RDI: 0000000000206620
RSI: 00000000ffffdd08
R8:  00000000f71f1cc0
R9:  0000000000000190
R10: 00000000f7201dd8
R11: 00000000f793c720
R12: 00000000f6e4b028
R13: 00000000f6e4b024
R14: 0000000000000001
R15: 00000000f6e4b020

--------------------------------
@@ src live vars - smid: Opt
0 - Direct(6, -48, 8)
1 - Direct(6, -56, 8)
2 - Register(15, 8, [])
3 - Register(13, 8, [])
4 - Register(12, 8, [])
@@ dst live vars - smid: UnOpt
0 - Direct(6, -48, 8)
1 - Direct(6, -56, 8)
2 - Register(12, 8, [])
3 - Register(15, 8, [])
4 - Register(13, 8, [])
--------------------------------
Register2Register - src: Register(15, 8, []) dst: Register(12, 8, [])
Register2Register - src: Register(13, 8, []) dst: Register(15, 8, [])
Register2Register - src: Register(12, 8, []) dst: Register(13, 8, [])

@@ Restoring reg_num: 0, dest_reg: 0, reg_offset: 0x60, reg_val_rbp_offset: 0x60
@@ Restoring reg_num: 2, dest_reg: 2, reg_offset: 0x58, reg_val_rbp_offset: 0x68
@@ Restoring reg_num: 3, dest_reg: 3, reg_offset: 0x50, reg_val_rbp_offset: 0x70
@@ Restoring reg_num: 5, dest_reg: 7, reg_offset: 0x48, reg_val_rbp_offset: 0x78
@@ Restoring reg_num: 4, dest_reg: 6, reg_offset: 0x40, reg_val_rbp_offset: 0x80
@@ Restoring reg_num: 8, dest_reg: 8, reg_offset: 0x38, reg_val_rbp_offset: 0x88
@@ Restoring reg_num: 9, dest_reg: 9, reg_offset: 0x30, reg_val_rbp_offset: 0x90
@@ Restoring reg_num: 10, dest_reg: 10, reg_offset: 0x28, reg_val_rbp_offset: 0x98
@@ Restoring reg_num: 11, dest_reg: 11, reg_offset: 0x20, reg_val_rbp_offset: 0xa0
@@ Restoring reg_num: 14, dest_reg: 14, reg_offset: 0x8, reg_val_rbp_offset: 0xb8

 V 0x00007ffff7fbc057> ? mov    rax,QWORD PTR [rbp-0x60]
 ? 0x00007ffff7fbc05e  ? mov    rdx,QWORD PTR [rbp-0x68]
 V 0x00007ffff7fbc065  ? mov    rbx,QWORD PTR [rbp-0x70]
 V 0x00007ffff7fbc06c  ? mov    rdi,QWORD PTR [rbp-0x78]
 V 0x00007ffff7fbc073  ? mov    rsi,QWORD PTR [rbp-0x80]
 V 0x00007ffff7fbc07a  ? mov    r8,QWORD PTR [rbp-0x88]
 V 0x00007ffff7fbc081  ? mov    r9,QWORD PTR [rbp-0x90]
 V 0x00007ffff7fbc088  ? mov    r10,QWORD PTR [rbp-0x98]
 V 0x00007ffff7fbc08f  ? mov    r11,QWORD PTR [rbp-0xa0]
 V 0x00007ffff7fbc096  ? mov    r14,QWORD PTR [rbp-0xb8]


Originals Register Values:
RAX: 0000000000000001
RCX: 0000000000000000
RBX: 0000000000000001
RDI: 0000000000206620
RSI: 00000000ffffdd08
R8:  00000000f71f1cc0
R9:  0000000000000190
R10: 00000000f7201dd8
R11: 00000000f793c720
    R12: 00000000f6e4b028
    R13: 00000000f6e4b024
R14: 0000000000000001
    R15: 00000000f6e4b020

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