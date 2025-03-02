push   rbp
mov    rbp,rsp
push   r15
push   r14
push   r13
push   r12
push   rbx
sub    rsp,0x58
mov    r12,rsi
mov    r14d,edi
mov    edi,0xb
xor    esi,esi
call    0x205570 <__yk_trace_basicblock@plt>
lea    rax,[rip+0x261e]        # 0x207828 <shadowstack_0>
mov    rax,QWORD PTR [rax]
mov    rbx,rax
add    rbx,0x8
mov    r15,rax
add    r15,0x10
mov    rcx,rax
add    rcx,0x14
mov    QWORD PTR [rbp-0x78],rcx
mov    rcx,rax
add    rcx,0x18
mov    QWORD PTR [rbp-0x60],rcx
mov    rcx,rax
add    rcx,0x1c
mov    QWORD PTR [rbp-0x70],rcx
mov    rcx,rax
add    rcx,0x20
mov    QWORD PTR [rbp-0x68],rcx
mov    rcx,rax
add    rcx,0x24
mov    QWORD PTR [rbp-0x50],rcx
mov    r13,rax
add    r13,0x28
mov    rcx,rax
add    rcx,0x2c
mov    QWORD PTR [rbp-0x48],rcx
mov    rcx,rax
add    rcx,0x30
mov    QWORD PTR [rbp-0x40],rcx
mov    DWORD PTR [rax],0x0
mov    DWORD PTR [rax+0x4],r14d
jmp    0x20527b <main+155>
mov    edi,0xb
mov    esi,0x1
call   0x205570 <__yk_trace_basicblock@plt>
mov    QWORD PTR [rbx],r12
xor    edi,edi
call   0x205590 <yk_mt_new@plt>
mov    rbx,rax
jmp    0x205299 <main+185>
mov    edi,0xb
mov    esi,0x2
call   0x205570 <__yk_trace_basicblock@plt>
mov    QWORD PTR [rbp-0x38],rbx
mov    rdi,QWORD PTR [rbp-0x38]
xor    esi,esi
call   0x2055a0 <yk_mt_hot_threshold_set@plt>
jmp    0x2052b9 <main+217>
mov    edi,0xb
mov    esi,0x3
call   0x205570 <__yk_trace_basicblock@plt>
call   0x2055b0 <yk_location_new@plt>
mov    rbx,rax
jmp    0x2052d2 <main+242>
mov    edi,0xb
mov    esi,0x4
call   0x205570 <__yk_trace_basicblock@plt>
mov    QWORD PTR [rbp-0x30],rbx
mov    DWORD PTR [r15],0x4
mov    rax,QWORD PTR [rbp-0x30]
jmp    0x2052f2 <main+274>
mov    edi,0xb
mov    esi,0x5
call   0x205570 <__yk_trace_basicblock@plt>
mov    eax,DWORD PTR [r15]
jmp    0x205306 <main+294>
mov    edi,0xb
mov    esi,0x6
call   0x205570 <__yk_trace_basicblock@plt>
jmp    0x205317 <main+311>
mov    edi,0xb
mov    esi,0x7
call   0x205570 <__yk_trace_basicblock@plt>
cmp    DWORD PTR [r15],0x0
setg   al
mov    r14,QWORD PTR [rbp-0x70]
mov    r12,QWORD PTR [rbp-0x68]
mov    rcx,QWORD PTR [rbp-0x50]
mov    rdx,QWORD PTR [rbp-0x40]
mov    rbx,QWORD PTR [rbp-0x60]
mov    rsi,QWORD PTR [rbp-0x78]
test   al,0x1
jne    0x20534e <main+366>
jmp    0x205463 <main+643>
mov    edi,0xb
mov    QWORD PTR [rbp-0x58],r15
mov    r15,rbx
mov    rbx,rsi
mov    esi,0x8
call   0x205570 <__yk_trace_basicblock@plt>
mov    rdi,QWORD PTR [rbp-0x38]
lea    rsi,[rbp-0x30]
mov    edx,0x1
movabs r11,0x205580
call   r11
mov    r15,QWORD PTR [rbp-0x58]
jmp    0x205387 <main+423>
mov    edi,0xb
mov    esi,0x9
call   0x205570 <__yk_trace_basicblock@plt>
mov    eax,DWORD PTR [r15]
and    eax,0x1
mov    DWORD PTR [rbx],eax
mov    eax,DWORD PTR [r15]
or     eax,0x1
mov    rcx,QWORD PTR [rbp-0x60]
mov    DWORD PTR [rcx],eax
mov    eax,DWORD PTR [r15]
shr    eax,1
mov    DWORD PTR [r14],eax
mov    eax,DWORD PTR [r15]
sar    eax,1
mov    DWORD PTR [r12],eax
mov    rdi,QWORD PTR ds:0x207840
mov    edx,DWORD PTR [rbx]
mov    ecx,DWORD PTR [rcx]
mov    r8d,DWORD PTR [r14]
mov    r9d,DWORD PTR [r12]
movabs rsi,0x204714
mov    al,0x0
call   0x2055c0 <fprintf@plt>
jmp    0x2053e1 <main+513>
mov    edi,0xb
mov    esi,0xa
call   0x205570 <__yk_trace_basicblock@plt>
xor    eax,eax
sub    eax,DWORD PTR [r15]
sar    eax,1
mov    rcx,QWORD PTR [rbp-0x50]
mov    DWORD PTR [rcx],eax
mov    eax,DWORD PTR [r15]
xor    eax,0x1
mov    DWORD PTR [r13+0x0],eax
mov    eax,DWORD PTR [r15]
xor    eax,0xffffffff
mov    r8,QWORD PTR [rbp-0x48]
mov    DWORD PTR [r8],eax
mov    eax,DWORD PTR [r15]
shl    eax,1
mov    rsi,QWORD PTR [rbp-0x40]
mov    DWORD PTR [rsi],eax
mov    rdi,QWORD PTR ds:0x207840
mov    edx,DWORD PTR [rcx]
mov    ecx,DWORD PTR [r13+0x0]
mov    r8d,DWORD PTR [r8]
mov    r9d,DWORD PTR [rsi]
movabs rsi,0x204738
mov    al,0x0
call   0x2055c0 <fprintf@plt>
jmp    0x205446 <main+614>
mov    edi,0xb
mov    esi,0xb
call   0x205570 <__yk_trace_basicblock@plt>
mov    eax,DWORD PTR [r15]
add    eax,0xffffffff
mov    DWORD PTR [r15],eax
jmp    0x205317 <main+311>
mov    edi,0xb
mov    esi,0xc
call   0x205570 <__yk_trace_basicblock@plt>
mov    rdi,QWORD PTR ds:0x207840
movabs rsi,0x204732
mov    al,0x0
call   0x2055c0 <fprintf@plt>
jmp    0x20548d <main+685>
mov    edi,0xb
mov    esi,0xd
call   0x205570 <__yk_trace_basicblock@plt>
mov    rdi,QWORD PTR [rbp-0x30]
call   0x205560 <yk_location_drop@plt>
jmp    0x2054a7 <main+711>
mov    edi,0xb
mov    esi,0xe
call   0x205570 <__yk_trace_basicblock@plt>
mov    rdi,QWORD PTR [rbp-0x38]
call   0x205540 <yk_mt_shutdown@plt>
jmp    0x2054c1 <main+737>
mov    edi,0xb
mov    esi,0xf
call   0x205570 <__yk_trace_basicblock@plt>
jmp    0x2054f2 <main+786>
mov    edi,0xb
mov    esi,0x10
call   0x205570 <__yk_trace_basicblock@plt>
xor    eax,eax
add    rsp,0x58
pop    rbx
pop    r12
pop    r13
pop    r14
pop    r15
pop    rbp
ret
mov    edi,0xb
mov    esi,0x11
call   0x205570 <__yk_trace_basicblock@plt>
jmp    0x2054d2 <main+754>
