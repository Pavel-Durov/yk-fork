push   rbp
mov    rbp,rsp
push   r15
push   r14
push   r13
push   r12
push   rbx
sub    rsp,0x58
mov    r14,rsi
mov    r12d,edi
mov    edi,0x2
xor    esi,esi
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    edi,0xf4240
call   0x2055d0 <malloc@plt>
mov    rcx,rax
add    rcx,0x34
lea    rdx,[rip+0x29bd]        # 0x207828 <shadowstack_0>
mov    QWORD PTR [rdx],rcx
mov    rbx,rax
add    rbx,0x8
mov    r15,rax
add    r15,0x10
mov    rcx,rax
add    rcx,0x14
mov    QWORD PTR [rbp-0x78],rcx
mov    rcx,rax
add    rcx,0x18
mov    QWORD PTR [rbp-0x70],rcx
mov    rcx,rax
add    rcx,0x1c
mov    QWORD PTR [rbp-0x68],rcx
mov    rcx,rax
add    rcx,0x20
mov    QWORD PTR [rbp-0x60],rcx
mov    rcx,rax
add    rcx,0x24
mov    QWORD PTR [rbp-0x58],rcx
mov    rcx,rax
add    rcx,0x28
mov    QWORD PTR [rbp-0x50],rcx
mov    rcx,rax
add    rcx,0x2c
mov    QWORD PTR [rbp-0x48],rcx
mov    rcx,rax
add    rcx,0x30
mov    QWORD PTR [rbp-0x40],rcx
mov    DWORD PTR [rax],0x0
mov    DWORD PTR [rax+0x4],r12d
jmp    0x204ee0 <main+176>
mov    edi,0x2
mov    esi,0x1
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    QWORD PTR [rbx],r14
xor    edi,edi
call   0x205590 <yk_mt_new@plt>
mov    rbx,rax
jmp    0x204efe <main+206>
mov    edi,0x2
mov    esi,0x2
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    QWORD PTR [rbp-0x38],rbx
mov    rdi,QWORD PTR [rbp-0x38]
xor    esi,esi
call   0x2055a0 <yk_mt_hot_threshold_set@plt>
jmp    0x204f1e <main+238>
mov    edi,0x2
mov    esi,0x3
call   0x205550 <__yk_trace_basicblock_dummy@plt>
call   0x2055b0 <yk_location_new@plt>
mov    rbx,rax
jmp    0x204f37 <main+263>
mov    edi,0x2
mov    esi,0x4
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    QWORD PTR [rbp-0x30],rbx
mov    DWORD PTR [r15],0x4
mov    rax,QWORD PTR [rbp-0x30]
jmp    0x204f57 <main+295>
mov    edi,0x2
mov    esi,0x5
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    eax,DWORD PTR [r15]
jmp    0x204f6b <main+315>
mov    edi,0x2
mov    esi,0x6
call   0x205550 <__yk_trace_basicblock_dummy@plt>
jmp    0x204f7c <main+332>
mov    edi,0x2
mov    esi,0x7
call   0x205550 <__yk_trace_basicblock_dummy@plt>
cmp    DWORD PTR [r15],0x0
setg   al
mov    r14,QWORD PTR [rbp-0x70]
mov    r12,QWORD PTR [rbp-0x68]
mov    r13,QWORD PTR [rbp-0x60]
mov    rcx,QWORD PTR [rbp-0x58]
mov    rdx,QWORD PTR [rbp-0x48]
mov    rsi,QWORD PTR [rbp-0x40]
mov    rbx,QWORD PTR [rbp-0x78]
test   al,0x1
jne    0x204fb7 <main+391>
jmp    0x2050bd <main+653>
mov    edi,0x2
mov    esi,0x8
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    rdi,QWORD PTR [rbp-0x38]
lea    rsi,[rbp-0x30]
xor    edx,edx
movabs r11,0x205580
call   r11
jmp    0x204fdf <main+431>
mov    edi,0x2
mov    esi,0x9
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    eax,DWORD PTR [r15]
and    eax,0x1
mov    DWORD PTR [rbx],eax
mov    eax,DWORD PTR [r15]
or     eax,0x1
mov    DWORD PTR [r14],eax
mov    eax,DWORD PTR [r15]
shr    eax,1
mov    DWORD PTR [r12],eax
mov    eax,DWORD PTR [r15]
sar    eax,1
mov    DWORD PTR [r13+0x0],eax
mov    rdi,QWORD PTR ds:0x207840
mov    edx,DWORD PTR [rbx]
mov    ecx,DWORD PTR [r14]
mov    r8d,DWORD PTR [r12]
mov    r9d,DWORD PTR [r13+0x0]
movabs rsi,0x204714
mov    al,0x0
call   0x2055c0 <fprintf@plt>
jmp    0x205039 <main+521>
mov    edi,0x2
mov    esi,0xa
call   0x205550 <__yk_trace_basicblock_dummy@plt>
xor    eax,eax
sub    eax,DWORD PTR [r15]
sar    eax,1
mov    rcx,QWORD PTR [rbp-0x58]
mov    DWORD PTR [rcx],eax
mov    eax,DWORD PTR [r15]
xor    eax,0x1
mov    r8,QWORD PTR [rbp-0x50]
mov    DWORD PTR [r8],eax
mov    eax,DWORD PTR [r15]
xor    eax,0xffffffff
mov    rsi,QWORD PTR [rbp-0x48]
mov    DWORD PTR [rsi],eax
mov    eax,DWORD PTR [r15]
shl    eax,1
mov    r9,QWORD PTR [rbp-0x40]
mov    DWORD PTR [r9],eax
mov    rdi,QWORD PTR ds:0x207840
mov    edx,DWORD PTR [rcx]
mov    ecx,DWORD PTR [r8]
mov    r8d,DWORD PTR [rsi]
mov    r9d,DWORD PTR [r9]
movabs rsi,0x204738
mov    al,0x0
call   0x2055c0 <fprintf@plt>
jmp    0x2050a0 <main+624>
mov    edi,0x2
mov    esi,0xb
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    eax,DWORD PTR [r15]
add    eax,0xffffffff
mov    DWORD PTR [r15],eax
jmp    0x204f7c <main+332>
mov    edi,0x2
mov    esi,0xc
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    rdi,QWORD PTR ds:0x207840
movabs rsi,0x204732
mov    al,0x0
call   0x2055c0 <fprintf@plt>
jmp    0x2050e7 <main+695>
mov    edi,0x2
mov    esi,0xd
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    rdi,QWORD PTR [rbp-0x30]
call   0x205560 <yk_location_drop@plt>
jmp    0x205101 <main+721>
mov    edi,0x2
mov    esi,0xe
call   0x205550 <__yk_trace_basicblock_dummy@plt>
mov    rdi,QWORD PTR [rbp-0x38]
call   0x205540 <yk_mt_shutdown@plt>
jmp    0x20511b <main+747>
mov    edi,0x2
mov    esi,0xf
call   0x205550 <__yk_trace_basicblock_dummy@plt>
jmp    0x20514c <main+796>
mov    edi,0x2
mov    esi,0x10
call   0x205550 <__yk_trace_basicblock_dummy@plt>
xor    eax,eax
add    rsp,0x58
pop    rbx
pop    r12
pop    r13
pop    r14
pop    r15
pop    rbp
ret
mov    edi,0x2
mov    esi,0x11
call   0x205550 <__yk_trace_basicblock_dummy@plt>
jmp    0x20512c <main+764>