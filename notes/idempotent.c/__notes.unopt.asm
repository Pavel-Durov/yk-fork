>>> info reg
rax            0x0                 0
rbx            0x7ffff6f2a018      140737336483864
rcx            0x1                 1
rdx            0xfffffffffffffffc  -4
rsi            0x7ffff7f7fef8      140737353613048
rdi            0x20baa0            2144928
rbp            0x7fffffffe170      0x7fffffffe170
rsp            0x7fffffffe0d0      0x7fffffffe0d0
r8             0x0                 0
r9             0x1                 1
r10            0x7ffff74f51c7      140737342558663
r11            0x293               659
r12            0x7ffff6f2a028      140737336483880
r13            0x20b660            2143840
r14            0x7ffff6f2a010      140737336483856
r15            0x7ffff6f2a020      140737336483872
rip            0x207c96            0x207c96 <main+502>
eflags         0x206               [ PF IF ]
cs             0x33                51
ss             0x2b                43
ds             0x0                 0
es             0x0                 0
fs             0x0                 0
gs             0x0                 0
k0             0xe0bfbf3f          3770662719
k1             0xc0ff9000          3237974016
k2             0xf                 15
k3             0x0                 0
k4             0x0                 0
k5             0x0                 0
k6             0x0                 0
k7             0x0                 0


Dump of assembler code for function main:
   0x0000000000207aa0 <+0>:	push   rbp
   0x0000000000207aa1 <+1>:	mov    rbp,rsp
   0x0000000000207aa4 <+4>:	push   r15
   0x0000000000207aa6 <+6>:	push   r14
   0x0000000000207aa8 <+8>:	push   r13
   0x0000000000207aaa <+10>:	push   r12
   0x0000000000207aac <+12>:	push   rbx
   0x0000000000207aad <+13>:	sub    rsp,0x78
   0x0000000000207ab1 <+17>:	mov    edi,0x4
   0x0000000000207ab6 <+22>:	xor    esi,esi
   0x0000000000207ab8 <+24>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207abd <+29>:	mov    edi,0xf4240
   0x0000000000207ac2 <+34>:	call   0x208890 <malloc@plt>
   0x0000000000207ac7 <+39>:	mov    r14,rax
   0x0000000000207aca <+42>:	mov    rax,r14
   0x0000000000207acd <+45>:	add    rax,0x20
   0x0000000000207ad1 <+49>:	mov    rcx,0xfffffffffffffff8
   0x0000000000207ad8 <+56>:	mov    QWORD PTR fs:[rcx],r14
   0x0000000000207adc <+60>:	mov    rcx,0xfffffffffffffff0
   0x0000000000207ae3 <+67>:	mov    QWORD PTR fs:[rcx],rax
   0x0000000000207ae7 <+71>:	mov    r12,r14
   0x0000000000207aea <+74>:	add    r12,0x8
   0x0000000000207aee <+78>:	mov    r15,r14
   0x0000000000207af1 <+81>:	add    r15,0x10
   0x0000000000207af5 <+85>:	mov    rax,r14
   0x0000000000207af8 <+88>:	add    rax,0x18
   0x0000000000207afc <+92>:	mov    QWORD PTR [rbp-0x40],rax
   0x0000000000207b00 <+96>:	jmp    0x207b02 <main+98>
   0x0000000000207b02 <+98>:	mov    edi,0x4
   0x0000000000207b07 <+103>:	mov    esi,0x1
   0x0000000000207b0c <+108>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b11 <+113>:	xor    edi,edi
   0x0000000000207b13 <+115>:	call   0x208830 <yk_mt_new@plt>
   0x0000000000207b18 <+120>:	mov    r13,rax
   0x0000000000207b1b <+123>:	jmp    0x207b1d <main+125>
   0x0000000000207b1d <+125>:	mov    edi,0x4
   0x0000000000207b22 <+130>:	mov    esi,0x2
   0x0000000000207b27 <+135>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b2c <+140>:	mov    rdi,r13
   0x0000000000207b2f <+143>:	xor    esi,esi
   0x0000000000207b31 <+145>:	call   0x208840 <yk_mt_hot_threshold_set@plt>
   0x0000000000207b36 <+150>:	jmp    0x207b38 <main+152>
   0x0000000000207b38 <+152>:	mov    edi,0x4
   0x0000000000207b3d <+157>:	mov    esi,0x3
   0x0000000000207b42 <+162>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b47 <+167>:	jmp    0x207b49 <main+169>
   0x0000000000207b49 <+169>:	mov    edi,0x4
   0x0000000000207b4e <+174>:	mov    esi,0x4
   0x0000000000207b53 <+179>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b58 <+184>:	call   0x208850 <yk_location_new@plt>
   0x0000000000207b5d <+189>:	mov    rbx,rax
   0x0000000000207b60 <+192>:	jmp    0x207b62 <main+194>
   0x0000000000207b62 <+194>:	mov    edi,0x4
   0x0000000000207b67 <+199>:	mov    esi,0x5
   0x0000000000207b6c <+204>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b71 <+209>:	mov    QWORD PTR [rbp-0x30],rbx
   0x0000000000207b75 <+213>:	jmp    0x207b77 <main+215>
   0x0000000000207b77 <+215>:	mov    edi,0x4
   0x0000000000207b7c <+220>:	mov    esi,0x6
   0x0000000000207b81 <+225>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b86 <+230>:	mov    QWORD PTR [r14],0x4
   0x0000000000207b8d <+237>:	mov    rbx,r12
   0x0000000000207b90 <+240>:	jmp    0x207b92 <main+242>
   0x0000000000207b92 <+242>:	mov    edi,0x4
   0x0000000000207b97 <+247>:	mov    esi,0x7
   0x0000000000207b9c <+252>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207ba1 <+257>:	mov    QWORD PTR [rbx],0x25
   0x0000000000207ba8 <+264>:	jmp    0x207baa <main+266>
   0x0000000000207baa <+266>:	mov    edi,0x4
   0x0000000000207baf <+271>:	mov    esi,0x8
   0x0000000000207bb4 <+276>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bb9 <+281>:	mov    DWORD PTR [r15],0x26
   0x0000000000207bc0 <+288>:	jmp    0x207bc2 <main+290>
   0x0000000000207bc2 <+290>:	mov    edi,0x4
   0x0000000000207bc7 <+295>:	mov    esi,0x9
   0x0000000000207bcc <+300>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bd1 <+305>:	mov    rax,QWORD PTR [rbp-0x40]
   0x0000000000207bd5 <+309>:	mov    QWORD PTR [rax],0x27
   0x0000000000207bdc <+316>:	jmp    0x207bde <main+318>
   0x0000000000207bde <+318>:	mov    edi,0x4
   0x0000000000207be3 <+323>:	mov    esi,0xa
   0x0000000000207be8 <+328>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bed <+333>:	jmp    0x207bef <main+335>
   0x0000000000207bef <+335>:	mov    edi,0x4
   0x0000000000207bf4 <+340>:	mov    esi,0xb
   0x0000000000207bf9 <+345>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bfe <+350>:	jmp    0x207c00 <main+352>
   0x0000000000207c00 <+352>:	mov    edi,0x4
   0x0000000000207c05 <+357>:	mov    esi,0xc
   0x0000000000207c0a <+362>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c0f <+367>:	jmp    0x207c11 <main+369>
   0x0000000000207c11 <+369>:	mov    edi,0x4
   0x0000000000207c16 <+374>:	mov    esi,0xd
   0x0000000000207c1b <+379>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c20 <+384>:	mov    rax,QWORD PTR [rbp-0x40]
   0x0000000000207c24 <+388>:	jmp    0x207c26 <main+390>
   0x0000000000207c26 <+390>:	mov    edi,0x4
   0x0000000000207c2b <+395>:	mov    esi,0xe
   0x0000000000207c30 <+400>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c35 <+405>:	mov    rax,QWORD PTR [r14]
   0x0000000000207c38 <+408>:	cmp    rax,0x0
   0x0000000000207c3c <+412>:	sete   al
   0x0000000000207c3f <+415>:	test   al,0x1
   0x0000000000207c41 <+417>:	jne    0x207fb5 <main+1301>
   0x0000000000207c47 <+423>:	jmp    0x207c49 <main+425>
   0x0000000000207c49 <+425>:	mov    edi,0x4
   0x0000000000207c4e <+430>:	mov    esi,0xf
   0x0000000000207c53 <+435>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c58 <+440>:	mov    QWORD PTR [rbp-0x48],r13
   0x0000000000207c5c <+444>:	mov    QWORD PTR [rbp-0x50],rbx
   0x0000000000207c60 <+448>:	mov    QWORD PTR [rbp-0x60],r15
   0x0000000000207c64 <+452>:	mov    QWORD PTR [rbp-0x98],r14
   0x0000000000207c6b <+459>:	jmp    0x207c6d <main+461>
   0x0000000000207c6d <+461>:	mov    edi,0x4
   0x0000000000207c72 <+466>:	mov    esi,0x10
   0x0000000000207c77 <+471>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c7c <+476>:	lea    rsi,[rbp-0x30]
   0x0000000000207c80 <+480>:	mov    rdi,r13
   0x0000000000207c83 <+483>:	xor    edx,edx
   0x0000000000207c85 <+485>:	mov    r12,QWORD PTR [rbp-0x40]
   0x0000000000207c89 <+489>:	movabs r11,0x208820

Opt Live vars
Register(14, 8, [-152]) - 0x98
Register(3, 8, []) 
Register(15, 8, [-96]) - 0x60
Register(12, 8, [-64]) - 0x40
Register(13, 8, [-72, 5]) - 0x0x48





PATCHPOINT CALL
   0x0000000000207c93 <+499>:	call   r11
=> 0x0000000000207c96 <+502>:	mov    rax,r12
   0x0000000000207c99 <+505>:	mov    r13,rax
   0x0000000000207c9c <+508>:	jmp    0x207c9e <main+510>
   0x0000000000207c9e <+510>:	mov    edi,0x4
   0x0000000000207ca3 <+515>:	mov    esi,0x11
   0x0000000000207ca8 <+520>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207cad <+525>:	mov    r12,QWORD PTR [rbx]
   0x0000000000207cb0 <+528>:	mov    rdi,r12
   0x0000000000207cb3 <+531>:	mov    esi,0x2
   0x0000000000207cb8 <+536>:	call   0x2079a0 <add_uintptr_t>
   0x0000000000207cbd <+541>:	mov    QWORD PTR [rbp-0x90],rax
   0x0000000000207cc4 <+548>:	mov    QWORD PTR [rbp-0x78],r12
   0x0000000000207cc8 <+552>:	mov    rax,QWORD PTR [rbp-0x48]
   0x0000000000207ccc <+556>:	jmp    0x207cce <main+558>
   0x0000000000207cce <+558>:	mov    edi,0x4
   0x0000000000207cd3 <+563>:	mov    esi,0x12
   0x0000000000207cd8 <+568>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207cdd <+573>:	mov    rdi,QWORD PTR [rbp-0x90]
   0x0000000000207ce4 <+580>:	call   0x208860 <__yk_idempotent_promote_i64@plt>
   0x0000000000207ce9 <+585>:	mov    rbx,rax
   0x0000000000207cec <+588>:	jmp    0x207cee <main+590>
   0x0000000000207cee <+590>:	mov    edi,0x4
   0x0000000000207cf3 <+595>:	mov    esi,0x13
   0x0000000000207cf8 <+600>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207cfd <+605>:	mov    r13,QWORD PTR [rbp-0x60]
   0x0000000000207d01 <+609>:	mov    r15d,DWORD PTR [r13+0x0]
   0x0000000000207d05 <+613>:	mov    edi,r15d
   0x0000000000207d08 <+616>:	mov    esi,0x3
   0x0000000000207d0d <+621>:	call   0x2079f0 <add_uint32_t>
   0x0000000000207d12 <+626>:	mov    r12d,eax
   0x0000000000207d15 <+629>:	mov    r14,QWORD PTR [rbp-0x98]
   0x0000000000207d1c <+636>:	mov    QWORD PTR [rbp-0x58],rbx
   0x0000000000207d20 <+640>:	jmp    0x207d22 <main+642>
   0x0000000000207d22 <+642>:	mov    edi,0x4
   0x0000000000207d27 <+647>:	mov    esi,0x14
   0x0000000000207d2c <+652>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d31 <+657>:	mov    edi,r12d
   0x0000000000207d34 <+660>:	call   0x208800 <__yk_idempotent_promote_i32@plt>
   0x0000000000207d39 <+665>:	mov    ebx,eax
   0x0000000000207d3b <+667>:	jmp    0x207d3d <main+669>
   0x0000000000207d3d <+669>:	mov    edi,0x4
   0x0000000000207d42 <+674>:	mov    esi,0x15
   0x0000000000207d47 <+679>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d4c <+684>:	mov    r13,QWORD PTR [rbp-0x40]
   0x0000000000207d50 <+688>:	mov    r15,QWORD PTR [r13+0x0]
   0x0000000000207d54 <+692>:	mov    rdi,r15
   0x0000000000207d57 <+695>:	mov    esi,0x4
   0x0000000000207d5c <+700>:	call   0x207a40 <add_uint64_t>
   0x0000000000207d61 <+705>:	mov    r12,rax
   0x0000000000207d64 <+708>:	mov    rax,QWORD PTR [rbp-0x60]
   0x0000000000207d68 <+712>:	mov    rcx,QWORD PTR [rbp-0x50]
   0x0000000000207d6c <+716>:	mov    DWORD PTR [rbp-0x34],ebx
   0x0000000000207d6f <+719>:	mov    r13,rax
   0x0000000000207d72 <+722>:	jmp    0x207d74 <main+724>
   0x0000000000207d74 <+724>:	mov    edi,0x4
   0x0000000000207d79 <+729>:	mov    esi,0x16
   0x0000000000207d7e <+734>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d83 <+739>:	mov    rdi,r12
   0x0000000000207d86 <+742>:	call   0x208860 <__yk_idempotent_promote_i64@plt>
   0x0000000000207d8b <+747>:	mov    rbx,rax
   0x0000000000207d8e <+750>:	jmp    0x207d90 <main+752>
   0x0000000000207d90 <+752>:	mov    edi,0x4
   0x0000000000207d95 <+757>:	mov    esi,0x17
   0x0000000000207d9a <+762>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d9f <+767>:	mov    r15,QWORD PTR [rbp-0x78]
   0x0000000000207da3 <+771>:	mov    rdi,r15
   0x0000000000207da6 <+774>:	call   0x208870 <__yk_promote_usize@plt>
   0x0000000000207dab <+779>:	mov    r12,QWORD PTR [rbp-0x40]
   0x0000000000207daf <+783>:	mov    QWORD PTR [rbp-0x70],rax
   0x0000000000207db3 <+787>:	mov    QWORD PTR [rbp-0x68],rbx
   0x0000000000207db7 <+791>:	jmp    0x207db9 <main+793>
   0x0000000000207db9 <+793>:	mov    edi,0x4
   0x0000000000207dbe <+798>:	mov    esi,0x18
   0x0000000000207dc3 <+803>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207dc8 <+808>:	mov    ebx,DWORD PTR [r13+0x0]
   0x0000000000207dcc <+812>:	mov    edi,ebx
   0x0000000000207dce <+814>:	call   0x2087d0 <__yk_promote_c_unsigned_int@plt>
   0x0000000000207dd3 <+819>:	mov    rcx,r13
   0x0000000000207dd6 <+822>:	mov    r13d,eax
   0x0000000000207dd9 <+825>:	jmp    0x207ddb <main+827>
   0x0000000000207ddb <+827>:	mov    edi,0x4
   0x0000000000207de0 <+832>:	mov    esi,0x19
   0x0000000000207de5 <+837>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207dea <+842>:	mov    r15,QWORD PTR [r12]
   0x0000000000207dee <+846>:	mov    rdi,r15
   0x0000000000207df1 <+849>:	call   0x208870 <__yk_promote_usize@plt>
   0x0000000000207df6 <+854>:	mov    rbx,rax
   0x0000000000207df9 <+857>:	jmp    0x207dfb <main+859>
   0x0000000000207dfb <+859>:	mov    edi,0x4
   0x0000000000207e00 <+864>:	mov    esi,0x1a
   0x0000000000207e05 <+869>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e0a <+874>:	mov    r15,QWORD PTR [rbp-0x70]
   0x0000000000207e0e <+878>:	mov    rdi,r15
   0x0000000000207e11 <+881>:	mov    esi,0x2
   0x0000000000207e16 <+886>:	call   0x2079a0 <add_uintptr_t>
   0x0000000000207e1b <+891>:	mov    QWORD PTR [rbp-0x88],rax
   0x0000000000207e22 <+898>:	mov    DWORD PTR [rbp-0x7c],r13d
   0x0000000000207e26 <+902>:	jmp    0x207e28 <main+904>
   0x0000000000207e28 <+904>:	mov    edi,0x4
   0x0000000000207e2d <+909>:	mov    esi,0x1b
   0x0000000000207e32 <+914>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e37 <+919>:	mov    rdi,QWORD PTR [rbp-0x88]
   0x0000000000207e3e <+926>:	call   0x208860 <__yk_idempotent_promote_i64@plt>
   0x0000000000207e43 <+931>:	mov    r13,rax
   0x0000000000207e46 <+934>:	jmp    0x207e48 <main+936>
   0x0000000000207e48 <+936>:	mov    edi,0x4
   0x0000000000207e4d <+941>:	mov    esi,0x1c
   0x0000000000207e52 <+946>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e57 <+951>:	mov    r12d,DWORD PTR [rbp-0x7c]
   0x0000000000207e5b <+955>:	mov    edi,r12d
   0x0000000000207e5e <+958>:	mov    esi,0x3
   0x0000000000207e63 <+963>:	call   0x2079f0 <add_uint32_t>
   0x0000000000207e68 <+968>:	mov    r15d,eax
   0x0000000000207e6b <+971>:	jmp    0x207e6d <main+973>
   0x0000000000207e6d <+973>:	mov    edi,0x4
   0x0000000000207e72 <+978>:	mov    esi,0x1d
   0x0000000000207e77 <+983>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e7c <+988>:	mov    edi,r15d
   0x0000000000207e7f <+991>:	call   0x208800 <__yk_idempotent_promote_i32@plt>
   0x0000000000207e84 <+996>:	mov    r12d,eax
   0x0000000000207e87 <+999>:	jmp    0x207e89 <main+1001>
   0x0000000000207e89 <+1001>:	mov    edi,0x4
   0x0000000000207e8e <+1006>:	mov    esi,0x1e
   0x0000000000207e93 <+1011>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e98 <+1016>:	mov    rdi,rbx
   0x0000000000207e9b <+1019>:	mov    esi,0x4
   0x0000000000207ea0 <+1024>:	call   0x207a40 <add_uint64_t>
   0x0000000000207ea5 <+1029>:	mov    r15,rax
   0x0000000000207ea8 <+1032>:	mov    DWORD PTR [rbp-0x80],r12d
   0x0000000000207eac <+1036>:	mov    eax,DWORD PTR [rbp-0x34]
   0x0000000000207eaf <+1039>:	mov    r12d,eax
   0x0000000000207eb2 <+1042>:	jmp    0x207eb4 <main+1044>
   0x0000000000207eb4 <+1044>:	mov    edi,0x4
   0x0000000000207eb9 <+1049>:	mov    esi,0x1f
   0x0000000000207ebe <+1054>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207ec3 <+1059>:	mov    rdi,r15>>> info reg
rax            0x0                 0
rbx            0x7ffff6f2a018      140737336483864
rcx            0x1                 1
rdx            0xfffffffffffffffc  -4
rsi            0x7ffff7f7fef8      140737353613048
rdi            0x20baa0            2144928
rbp            0x7fffffffe170      0x7fffffffe170
rsp            0x7fffffffe0d0      0x7fffffffe0d0
r8             0x0                 0
r9             0x1                 1
r10            0x7ffff74f51c7      140737342558663
r11            0x293               659
r12            0x7ffff6f2a028      140737336483880
r13            0x20b660            2143840
r14            0x7ffff6f2a010      140737336483856
r15            0x7ffff6f2a020      140737336483872
rip            0x207c96            0x207c96 <main+502>
eflags         0x206               [ PF IF ]
cs             0x33                51
ss             0x2b                43
ds             0x0                 0
es             0x0                 0
fs             0x0                 0
gs             0x0                 0
k0             0xe0bfbf3f          3770662719
k1             0xc0ff9000          3237974016
k2             0xf                 15
k3             0x0                 0
k4             0x0                 0
k5             0x0                 0
k6             0x0                 0
k7             0x0                 0


Dump of assembler code for function main:
   0x0000000000207aa0 <+0>:	push   rbp
   0x0000000000207aa1 <+1>:	mov    rbp,rsp
   0x0000000000207aa4 <+4>:	push   r15
   0x0000000000207aa6 <+6>:	push   r14
   0x0000000000207aa8 <+8>:	push   r13
   0x0000000000207aaa <+10>:	push   r12
   0x0000000000207aac <+12>:	push   rbx
   0x0000000000207aad <+13>:	sub    rsp,0x78
   0x0000000000207ab1 <+17>:	mov    edi,0x4
   0x0000000000207ab6 <+22>:	xor    esi,esi
   0x0000000000207ab8 <+24>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207abd <+29>:	mov    edi,0xf4240
   0x0000000000207ac2 <+34>:	call   0x208890 <malloc@plt>
   0x0000000000207ac7 <+39>:	mov    r14,rax
   0x0000000000207aca <+42>:	mov    rax,r14
   0x0000000000207acd <+45>:	add    rax,0x20
   0x0000000000207ad1 <+49>:	mov    rcx,0xfffffffffffffff8
   0x0000000000207ad8 <+56>:	mov    QWORD PTR fs:[rcx],r14
   0x0000000000207adc <+60>:	mov    rcx,0xfffffffffffffff0
   0x0000000000207ae3 <+67>:	mov    QWORD PTR fs:[rcx],rax
   0x0000000000207ae7 <+71>:	mov    r12,r14
   0x0000000000207aea <+74>:	add    r12,0x8
   0x0000000000207aee <+78>:	mov    r15,r14
   0x0000000000207af1 <+81>:	add    r15,0x10
   0x0000000000207af5 <+85>:	mov    rax,r14
   0x0000000000207af8 <+88>:	add    rax,0x18
   0x0000000000207afc <+92>:	mov    QWORD PTR [rbp-0x40],rax
   0x0000000000207b00 <+96>:	jmp    0x207b02 <main+98>
   0x0000000000207b02 <+98>:	mov    edi,0x4
   0x0000000000207b07 <+103>:	mov    esi,0x1
   0x0000000000207b0c <+108>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b11 <+113>:	xor    edi,edi
   0x0000000000207b13 <+115>:	call   0x208830 <yk_mt_new@plt>
   0x0000000000207b18 <+120>:	mov    r13,rax
   0x0000000000207b1b <+123>:	jmp    0x207b1d <main+125>
   0x0000000000207b1d <+125>:	mov    edi,0x4
   0x0000000000207b22 <+130>:	mov    esi,0x2
   0x0000000000207b27 <+135>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b2c <+140>:	mov    rdi,r13
   0x0000000000207b2f <+143>:	xor    esi,esi
   0x0000000000207b31 <+145>:	call   0x208840 <yk_mt_hot_threshold_set@plt>
   0x0000000000207b36 <+150>:	jmp    0x207b38 <main+152>
   0x0000000000207b38 <+152>:	mov    edi,0x4
   0x0000000000207b3d <+157>:	mov    esi,0x3
   0x0000000000207b42 <+162>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b47 <+167>:	jmp    0x207b49 <main+169>
   0x0000000000207b49 <+169>:	mov    edi,0x4
   0x0000000000207b4e <+174>:	mov    esi,0x4
   0x0000000000207b53 <+179>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b58 <+184>:	call   0x208850 <yk_location_new@plt>
   0x0000000000207b5d <+189>:	mov    rbx,rax
   0x0000000000207b60 <+192>:	jmp    0x207b62 <main+194>
   0x0000000000207b62 <+194>:	mov    edi,0x4
   0x0000000000207b67 <+199>:	mov    esi,0x5
   0x0000000000207b6c <+204>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b71 <+209>:	mov    QWORD PTR [rbp-0x30],rbx
   0x0000000000207b75 <+213>:	jmp    0x207b77 <main+215>
   0x0000000000207b77 <+215>:	mov    edi,0x4
   0x0000000000207b7c <+220>:	mov    esi,0x6
   0x0000000000207b81 <+225>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207b86 <+230>:	mov    QWORD PTR [r14],0x4
   0x0000000000207b8d <+237>:	mov    rbx,r12
   0x0000000000207b90 <+240>:	jmp    0x207b92 <main+242>
   0x0000000000207b92 <+242>:	mov    edi,0x4
   0x0000000000207b97 <+247>:	mov    esi,0x7
   0x0000000000207b9c <+252>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207ba1 <+257>:	mov    QWORD PTR [rbx],0x25
   0x0000000000207ba8 <+264>:	jmp    0x207baa <main+266>
   0x0000000000207baa <+266>:	mov    edi,0x4
   0x0000000000207baf <+271>:	mov    esi,0x8
   0x0000000000207bb4 <+276>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bb9 <+281>:	mov    DWORD PTR [r15],0x26
   0x0000000000207bc0 <+288>:	jmp    0x207bc2 <main+290>
   0x0000000000207bc2 <+290>:	mov    edi,0x4
   0x0000000000207bc7 <+295>:	mov    esi,0x9
   0x0000000000207bcc <+300>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bd1 <+305>:	mov    rax,QWORD PTR [rbp-0x40]
   0x0000000000207bd5 <+309>:	mov    QWORD PTR [rax],0x27
   0x0000000000207bdc <+316>:	jmp    0x207bde <main+318>
   0x0000000000207bde <+318>:	mov    edi,0x4
   0x0000000000207be3 <+323>:	mov    esi,0xa
   0x0000000000207be8 <+328>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bed <+333>:	jmp    0x207bef <main+335>
   0x0000000000207bef <+335>:	mov    edi,0x4
   0x0000000000207bf4 <+340>:	mov    esi,0xb
   0x0000000000207bf9 <+345>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207bfe <+350>:	jmp    0x207c00 <main+352>
   0x0000000000207c00 <+352>:	mov    edi,0x4
   0x0000000000207c05 <+357>:	mov    esi,0xc
   0x0000000000207c0a <+362>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c0f <+367>:	jmp    0x207c11 <main+369>
   0x0000000000207c11 <+369>:	mov    edi,0x4
   0x0000000000207c16 <+374>:	mov    esi,0xd
   0x0000000000207c1b <+379>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c20 <+384>:	mov    rax,QWORD PTR [rbp-0x40]
   0x0000000000207c24 <+388>:	jmp    0x207c26 <main+390>
   0x0000000000207c26 <+390>:	mov    edi,0x4
   0x0000000000207c2b <+395>:	mov    esi,0xe
   0x0000000000207c30 <+400>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c35 <+405>:	mov    rax,QWORD PTR [r14]
   0x0000000000207c38 <+408>:	cmp    rax,0x0
   0x0000000000207c3c <+412>:	sete   al
   0x0000000000207c3f <+415>:	test   al,0x1
   0x0000000000207c41 <+417>:	jne    0x207fb5 <main+1301>
   0x0000000000207c47 <+423>:	jmp    0x207c49 <main+425>
   0x0000000000207c49 <+425>:	mov    edi,0x4
   0x0000000000207c4e <+430>:	mov    esi,0xf
   0x0000000000207c53 <+435>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c58 <+440>:	mov    QWORD PTR [rbp-0x48],r13
   0x0000000000207c5c <+444>:	mov    QWORD PTR [rbp-0x50],rbx
   0x0000000000207c60 <+448>:	mov    QWORD PTR [rbp-0x60],r15
   0x0000000000207c64 <+452>:	mov    QWORD PTR [rbp-0x98],r14
   0x0000000000207c6b <+459>:	jmp    0x207c6d <main+461>
   0x0000000000207c6d <+461>:	mov    edi,0x4
   0x0000000000207c72 <+466>:	mov    esi,0x10
   0x0000000000207c77 <+471>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207c7c <+476>:	lea    rsi,[rbp-0x30]
   0x0000000000207c80 <+480>:	mov    rdi,r13
   0x0000000000207c83 <+483>:	xor    edx,edx
   0x0000000000207c85 <+485>:	mov    r12,QWORD PTR [rbp-0x40]
   0x0000000000207c89 <+489>:	movabs r11,0x208820

Opt Live vars
Register(14, 8, [-152]) - 0x98
Register(3, 8, []) 
Register(15, 8, [-96]) - 0x60
Register(12, 8, [-64]) - 0x40
Register(13, 8, [-72, 5]) - 0x0x48





PATCHPOINT CALL
   0x0000000000207c93 <+499>:	call   r11
=> 0x0000000000207c96 <+502>:	mov    rax,r12
   0x0000000000207c99 <+505>:	mov    r13,rax
   0x0000000000207c9c <+508>:	jmp    0x207c9e <main+510>
   0x0000000000207c9e <+510>:	mov    edi,0x4
   0x0000000000207ca3 <+515>:	mov    esi,0x11
   0x0000000000207ca8 <+520>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207cad <+525>:	mov    r12,QWORD PTR [rbx]
   0x0000000000207cb0 <+528>:	mov    rdi,r12
   0x0000000000207cb3 <+531>:	mov    esi,0x2
   0x0000000000207cb8 <+536>:	call   0x2079a0 <add_uintptr_t>
   0x0000000000207cbd <+541>:	mov    QWORD PTR [rbp-0x90],rax
   0x0000000000207cc4 <+548>:	mov    QWORD PTR [rbp-0x78],r12
   0x0000000000207cc8 <+552>:	mov    rax,QWORD PTR [rbp-0x48]
   0x0000000000207ccc <+556>:	jmp    0x207cce <main+558>
   0x0000000000207cce <+558>:	mov    edi,0x4
   0x0000000000207cd3 <+563>:	mov    esi,0x12
   0x0000000000207cd8 <+568>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207cdd <+573>:	mov    rdi,QWORD PTR [rbp-0x90]
   0x0000000000207ce4 <+580>:	call   0x208860 <__yk_idempotent_promote_i64@plt>
   0x0000000000207ce9 <+585>:	mov    rbx,rax
   0x0000000000207cec <+588>:	jmp    0x207cee <main+590>
   0x0000000000207cee <+590>:	mov    edi,0x4
   0x0000000000207cf3 <+595>:	mov    esi,0x13
   0x0000000000207cf8 <+600>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207cfd <+605>:	mov    r13,QWORD PTR [rbp-0x60]
   0x0000000000207d01 <+609>:	mov    r15d,DWORD PTR [r13+0x0]
   0x0000000000207d05 <+613>:	mov    edi,r15d
   0x0000000000207d08 <+616>:	mov    esi,0x3
   0x0000000000207d0d <+621>:	call   0x2079f0 <add_uint32_t>
   0x0000000000207d12 <+626>:	mov    r12d,eax
   0x0000000000207d15 <+629>:	mov    r14,QWORD PTR [rbp-0x98]
   0x0000000000207d1c <+636>:	mov    QWORD PTR [rbp-0x58],rbx
   0x0000000000207d20 <+640>:	jmp    0x207d22 <main+642>
   0x0000000000207d22 <+642>:	mov    edi,0x4
   0x0000000000207d27 <+647>:	mov    esi,0x14
   0x0000000000207d2c <+652>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d31 <+657>:	mov    edi,r12d
   0x0000000000207d34 <+660>:	call   0x208800 <__yk_idempotent_promote_i32@plt>
   0x0000000000207d39 <+665>:	mov    ebx,eax
   0x0000000000207d3b <+667>:	jmp    0x207d3d <main+669>
   0x0000000000207d3d <+669>:	mov    edi,0x4
   0x0000000000207d42 <+674>:	mov    esi,0x15
   0x0000000000207d47 <+679>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d4c <+684>:	mov    r13,QWORD PTR [rbp-0x40]
   0x0000000000207d50 <+688>:	mov    r15,QWORD PTR [r13+0x0]
   0x0000000000207d54 <+692>:	mov    rdi,r15
   0x0000000000207d57 <+695>:	mov    esi,0x4
   0x0000000000207d5c <+700>:	call   0x207a40 <add_uint64_t>
   0x0000000000207d61 <+705>:	mov    r12,rax
   0x0000000000207d64 <+708>:	mov    rax,QWORD PTR [rbp-0x60]
   0x0000000000207d68 <+712>:	mov    rcx,QWORD PTR [rbp-0x50]
   0x0000000000207d6c <+716>:	mov    DWORD PTR [rbp-0x34],ebx
   0x0000000000207d6f <+719>:	mov    r13,rax
   0x0000000000207d72 <+722>:	jmp    0x207d74 <main+724>
   0x0000000000207d74 <+724>:	mov    edi,0x4
   0x0000000000207d79 <+729>:	mov    esi,0x16
   0x0000000000207d7e <+734>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d83 <+739>:	mov    rdi,r12
   0x0000000000207d86 <+742>:	call   0x208860 <__yk_idempotent_promote_i64@plt>
   0x0000000000207d8b <+747>:	mov    rbx,rax
   0x0000000000207d8e <+750>:	jmp    0x207d90 <main+752>
   0x0000000000207d90 <+752>:	mov    edi,0x4
   0x0000000000207d95 <+757>:	mov    esi,0x17
   0x0000000000207d9a <+762>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207d9f <+767>:	mov    r15,QWORD PTR [rbp-0x78]
   0x0000000000207da3 <+771>:	mov    rdi,r15
   0x0000000000207da6 <+774>:	call   0x208870 <__yk_promote_usize@plt>
   0x0000000000207dab <+779>:	mov    r12,QWORD PTR [rbp-0x40]
   0x0000000000207daf <+783>:	mov    QWORD PTR [rbp-0x70],rax
   0x0000000000207db3 <+787>:	mov    QWORD PTR [rbp-0x68],rbx
   0x0000000000207db7 <+791>:	jmp    0x207db9 <main+793>
   0x0000000000207db9 <+793>:	mov    edi,0x4
   0x0000000000207dbe <+798>:	mov    esi,0x18
   0x0000000000207dc3 <+803>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207dc8 <+808>:	mov    ebx,DWORD PTR [r13+0x0]
   0x0000000000207dcc <+812>:	mov    edi,ebx
   0x0000000000207dce <+814>:	call   0x2087d0 <__yk_promote_c_unsigned_int@plt>
   0x0000000000207dd3 <+819>:	mov    rcx,r13
   0x0000000000207dd6 <+822>:	mov    r13d,eax
   0x0000000000207dd9 <+825>:	jmp    0x207ddb <main+827>
   0x0000000000207ddb <+827>:	mov    edi,0x4
   0x0000000000207de0 <+832>:	mov    esi,0x19
   0x0000000000207de5 <+837>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207dea <+842>:	mov    r15,QWORD PTR [r12]
   0x0000000000207dee <+846>:	mov    rdi,r15
   0x0000000000207df1 <+849>:	call   0x208870 <__yk_promote_usize@plt>
   0x0000000000207df6 <+854>:	mov    rbx,rax
   0x0000000000207df9 <+857>:	jmp    0x207dfb <main+859>
   0x0000000000207dfb <+859>:	mov    edi,0x4
   0x0000000000207e00 <+864>:	mov    esi,0x1a
   0x0000000000207e05 <+869>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e0a <+874>:	mov    r15,QWORD PTR [rbp-0x70]
   0x0000000000207e0e <+878>:	mov    rdi,r15
   0x0000000000207e11 <+881>:	mov    esi,0x2
   0x0000000000207e16 <+886>:	call   0x2079a0 <add_uintptr_t>
   0x0000000000207e1b <+891>:	mov    QWORD PTR [rbp-0x88],rax
   0x0000000000207e22 <+898>:	mov    DWORD PTR [rbp-0x7c],r13d
   0x0000000000207e26 <+902>:	jmp    0x207e28 <main+904>
   0x0000000000207e28 <+904>:	mov    edi,0x4
   0x0000000000207e2d <+909>:	mov    esi,0x1b
   0x0000000000207e32 <+914>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e37 <+919>:	mov    rdi,QWORD PTR [rbp-0x88]
   0x0000000000207e3e <+926>:	call   0x208860 <__yk_idempotent_promote_i64@plt>
   0x0000000000207e43 <+931>:	mov    r13,rax
   0x0000000000207e46 <+934>:	jmp    0x207e48 <main+936>
   0x0000000000207e48 <+936>:	mov    edi,0x4
   0x0000000000207e4d <+941>:	mov    esi,0x1c
   0x0000000000207e52 <+946>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e57 <+951>:	mov    r12d,DWORD PTR [rbp-0x7c]
   0x0000000000207e5b <+955>:	mov    edi,r12d
   0x0000000000207e5e <+958>:	mov    esi,0x3
   0x0000000000207e63 <+963>:	call   0x2079f0 <add_uint32_t>
   0x0000000000207e68 <+968>:	mov    r15d,eax
   0x0000000000207e6b <+971>:	jmp    0x207e6d <main+973>
   0x0000000000207e6d <+973>:	mov    edi,0x4
   0x0000000000207e72 <+978>:	mov    esi,0x1d
   0x0000000000207e77 <+983>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e7c <+988>:	mov    edi,r15d
   0x0000000000207e7f <+991>:	call   0x208800 <__yk_idempotent_promote_i32@plt>
   0x0000000000207e84 <+996>:	mov    r12d,eax
   0x0000000000207e87 <+999>:	jmp    0x207e89 <main+1001>
   0x0000000000207e89 <+1001>:	mov    edi,0x4
   0x0000000000207e8e <+1006>:	mov    esi,0x1e
   0x0000000000207e93 <+1011>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207e98 <+1016>:	mov    rdi,rbx
   0x0000000000207e9b <+1019>:	mov    esi,0x4
   0x0000000000207ea0 <+1024>:	call   0x207a40 <add_uint64_t>
   0x0000000000207ea5 <+1029>:	mov    r15,rax
   0x0000000000207ea8 <+1032>:	mov    DWORD PTR [rbp-0x80],r12d
   0x0000000000207eac <+1036>:	mov    eax,DWORD PTR [rbp-0x34]
   0x0000000000207eaf <+1039>:	mov    r12d,eax
   0x0000000000207eb2 <+1042>:	jmp    0x207eb4 <main+1044>
   0x0000000000207eb4 <+1044>:	mov    edi,0x4
   0x0000000000207eb9 <+1049>:	mov    esi,0x1f
   0x0000000000207ebe <+1054>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207ec3 <+1059>:	mov    rdi,r15
   0x0000000000207ec6 <+1062>:	call   0x208860 <__yk_idempotent_promote_i64@plt>
   0x0000000000207ecb <+1067>:	mov    rbx,rax
   0x0000000000207ece <+1070>:	jmp    0x207ed0 <main+1072>
   0x0000000000207ed0 <+1072>:	mov    edi,0x4
   0x0000000000207ed5 <+1077>:	mov    esi,0x20
   0x0000000000207eda <+1082>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207edf <+1087>:	mov    rdi,QWORD PTR ds:0x20ab80
   0x0000000000207ee7 <+1095>:	mov    rdx,QWORD PTR [r14]
   0x0000000000207eea <+1098>:	movabs rsi,0x20aa70
   0x0000000000207ef4 <+1108>:	mov    rcx,QWORD PTR [rbp-0x58]
   0x0000000000207ef8 <+1112>:	mov    r8,r13
   0x0000000000207efb <+1115>:	mov    al,0x0
   0x0000000000207efd <+1117>:	call   0x208880 <fprintf@plt>
   0x0000000000207f02 <+1122>:	mov    r15,QWORD PTR [rbp-0x60]
   0x0000000000207f06 <+1126>:	jmp    0x207f08 <main+1128>
   0x0000000000207f08 <+1128>:	mov    edi,0x4
   0x0000000000207f0d <+1133>:	mov    esi,0x21
   0x0000000000207f12 <+1138>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207f17 <+1143>:	mov    rdi,QWORD PTR ds:0x20ab80
   0x0000000000207f1f <+1151>:	mov    rdx,QWORD PTR [r14]
   0x0000000000207f22 <+1154>:	movabs rsi,0x20aa64
   0x0000000000207f2c <+1164>:	mov    ecx,r12d
   0x0000000000207f2f <+1167>:	mov    r8d,DWORD PTR [rbp-0x80]
   0x0000000000207f33 <+1171>:	mov    al,0x0
   0x0000000000207f35 <+1173>:	call   0x208880 <fprintf@plt>

Load rbx additional location rbp-0x50
rbp-0x50 is not set cause its missing from live varibales
   0x0000000000207f3a <+1178>:	mov    r12,QWORD PTR [rbp-0x50]
   0x0000000000207f3e <+1182>:	mov    r13,QWORD PTR [rbp-0x48]
   0x0000000000207f42 <+1186>:	jmp    0x207f44 <main+1188>
   0x0000000000207f44 <+1188>:	mov    edi,0x4
   0x0000000000207f49 <+1193>:	mov    esi,0x22
   0x0000000000207f4e <+1198>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207f53 <+1203>:	mov    rdi,QWORD PTR ds:0x20ab80
   0x0000000000207f5b <+1211>:	mov    rdx,QWORD PTR [r14]
   0x0000000000207f5e <+1214>:	movabs rsi,0x20aa70
   0x0000000000207f68 <+1224>:	mov    rcx,QWORD PTR [rbp-0x68]
   0x0000000000207f6c <+1228>:	mov    r8,rbx
   0x0000000000207f6f <+1231>:	mov    al,0x0
   0x0000000000207f71 <+1233>:	call   0x208880 <fprintf@plt>
   0x0000000000207f76 <+1238>:	jmp    0x207f78 <main+1240>
   0x0000000000207f78 <+1240>:	mov    edi,0x4
   0x0000000000207f7d <+1245>:	mov    esi,0x23
   0x0000000000207f82 <+1250>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207f87 <+1255>:	mov    rax,QWORD PTR [r14]
   0x0000000000207f8a <+1258>:	add    rax,0xffffffffffffffff
   0x0000000000207f8e <+1262>:	mov    QWORD PTR [r14],rax
   0x0000000000207f91 <+1265>:	cmp    rax,0x0
   0x0000000000207f95 <+1269>:	sete   al

Load r12 to rbx
(>>> set $r12=0x7ffff6f2a018)



   0x0000000000207f98 <+1272>:	mov    rbx,r12
   0x0000000000207f9b <+1275>:	test   al,0x1
   0x0000000000207f9d <+1277>:	jne    0x207fa4 <main+1284>

Loop end
   0x0000000000207f9f <+1279>:	jmp    0x20804e <main+1454>
   0x0000000000207fa4 <+1284>:	mov    edi,0x4
   0x0000000000207fa9 <+1289>:	mov    esi,0x24
   0x0000000000207fae <+1294>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207fb3 <+1299>:	jmp    0x207fb5 <main+1301>
   0x0000000000207fb5 <+1301>:	mov    edi,0x4
   0x0000000000207fba <+1306>:	mov    esi,0x25
   0x0000000000207fbf <+1311>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207fc4 <+1316>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x0000000000207fc8 <+1320>:	call   0x2087f0 <yk_location_drop@plt>
   0x0000000000207fcd <+1325>:	jmp    0x207fcf <main+1327>
   0x0000000000207fcf <+1327>:	mov    edi,0x4
   0x0000000000207fd4 <+1332>:	mov    esi,0x26
   0x0000000000207fd9 <+1337>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207fde <+1342>:	mov    rdi,r13
   0x0000000000207fe1 <+1345>:	call   0x2087c0 <yk_mt_shutdown@plt>
   0x0000000000207fe6 <+1350>:	jmp    0x207fe8 <main+1352>
   0x0000000000207fe8 <+1352>:	mov    edi,0x4
   0x0000000000207fed <+1357>:	mov    esi,0x27
   0x0000000000207ff2 <+1362>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207ff7 <+1367>:	jmp    0x207ff9 <main+1369>
   0x0000000000207ff9 <+1369>:	mov    edi,0x4
   0x0000000000207ffe <+1374>:	mov    esi,0x28
   0x0000000000208003 <+1379>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208008 <+1384>:	jmp    0x20800a <main+1386>
   0x000000000020800a <+1386>:	mov    edi,0x4
   0x000000000020800f <+1391>:	mov    esi,0x29
   0x0000000000208014 <+1396>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208019 <+1401>:	jmp    0x20801b <main+1403>
   0x000000000020801b <+1403>:	mov    edi,0x4
   0x0000000000208020 <+1408>:	mov    esi,0x2a
   0x0000000000208025 <+1413>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020802a <+1418>:	jmp    0x20802c <main+1420>
   0x000000000020802c <+1420>:	mov    edi,0x4
   0x0000000000208031 <+1425>:	mov    esi,0x2b
   0x0000000000208036 <+1430>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020803b <+1435>:	jmp    0x20803d <main+1437>
   0x000000000020803d <+1437>:	mov    edi,0x4
   0x0000000000208042 <+1442>:	mov    esi,0x2c
   0x0000000000208047 <+1447>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020804c <+1452>:	jmp    0x208082 <main+1506>
   0x000000000020804e <+1454>:	mov    edi,0x4
   0x0000000000208053 <+1459>:	mov    esi,0x2d
   0x0000000000208058 <+1464>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>

Actual jump to start of the loop
   0x000000000020805d <+1469>:	jmp    0x207c6d <main+461>
   0x0000000000208062 <+1474>:	mov    edi,0x4
   0x0000000000208067 <+1479>:	mov    esi,0x2e
   0x000000000020806c <+1484>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208071 <+1489>:	xor    eax,eax
   0x0000000000208073 <+1491>:	add    rsp,0x78
   0x0000000000208077 <+1495>:	pop    rbx
   0x0000000000208078 <+1496>:	pop    r12
   0x000000000020807a <+1498>:	pop    r13
   0x000000000020807c <+1500>:	pop    r14
   0x000000000020807e <+1502>:	pop    r15
   0x0000000000208080 <+1504>:	pop    rbp
   0x0000000000208081 <+1505>:	ret
   0x0000000000208082 <+1506>:	mov    edi,0x4
   0x0000000000208087 <+1511>:	mov    esi,0x2f
   0x000000000020808c <+1516>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208091 <+1521>:	jmp    0x208062 <main+1474>


c * 2
OptToUnopt:

end:

            x/gx $r13
            0x7ffff6f2a018: 0x0000000000000025
            x/gx $rbx
            0x20b660: 0x0000000000000000

UnoptToOpt:

end:

            x/gx $r13
            0x20b660: 0x0000000000000000
            x/gx $rbx
            0x7ffff6f2a018: 0x0000000000000025




OptToUnopt ExecTrace: true

            x/gx $r13
            0x20b660: 0x0000000000000000
            x/gx $rbx
            0x20b660: 0x0000000000000000

0x0000000000207f98  main+1272 mov    rbx,r12


   0x0000000000207ece <+1070>:	jmp    0x207ed0 <main+1072>
   0x0000000000207ed0 <+1072>:	mov    edi,0x4
   0x0000000000207ed5 <+1077>:	mov    esi,0x20
   0x0000000000207eda <+1082>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207edf <+1087>:	mov    rdi,QWORD PTR ds:0x20ab80
   0x0000000000207ee7 <+1095>:	mov    rdx,QWORD PTR [r14]
   0x0000000000207eea <+1098>:	movabs rsi,0x20aa70
   0x0000000000207ef4 <+1108>:	mov    rcx,QWORD PTR [rbp-0x58]
   0x0000000000207ef8 <+1112>:	mov    r8,r13
   0x0000000000207efb <+1115>:	mov    al,0x0
   0x0000000000207efd <+1117>:	call   0x208880 <fprintf@plt>
   0x0000000000207f02 <+1122>:	mov    r15,QWORD PTR [rbp-0x60]
   0x0000000000207f06 <+1126>:	jmp    0x207f08 <main+1128>
   0x0000000000207f08 <+1128>:	mov    edi,0x4
   0x0000000000207f0d <+1133>:	mov    esi,0x21
   0x0000000000207f12 <+1138>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207f17 <+1143>:	mov    rdi,QWORD PTR ds:0x20ab80
   0x0000000000207f1f <+1151>:	mov    rdx,QWORD PTR [r14]
   0x0000000000207f22 <+1154>:	movabs rsi,0x20aa64
   0x0000000000207f2c <+1164>:	mov    ecx,r12d
   0x0000000000207f2f <+1167>:	mov    r8d,DWORD PTR [rbp-0x80]
   0x0000000000207f33 <+1171>:	mov    al,0x0
   0x0000000000207f35 <+1173>:	call   0x208880 <fprintf@plt>

Load rbx additional location rbp-0x50
rbp-0x50 is not set cause its missing from live varibales
   0x0000000000207f3a <+1178>:	mov    r12,QWORD PTR [rbp-0x50]
   0x0000000000207f3e <+1182>:	mov    r13,QWORD PTR [rbp-0x48]
   0x0000000000207f42 <+1186>:	jmp    0x207f44 <main+1188>
   0x0000000000207f44 <+1188>:	mov    edi,0x4
   0x0000000000207f49 <+1193>:	mov    esi,0x22
   0x0000000000207f4e <+1198>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207f53 <+1203>:	mov    rdi,QWORD PTR ds:0x20ab80
   0x0000000000207f5b <+1211>:	mov    rdx,QWORD PTR [r14]
   0x0000000000207f5e <+1214>:	movabs rsi,0x20aa70
   0x0000000000207f68 <+1224>:	mov    rcx,QWORD PTR [rbp-0x68]
   0x0000000000207f6c <+1228>:	mov    r8,rbx
   0x0000000000207f6f <+1231>:	mov    al,0x0
   0x0000000000207f71 <+1233>:	call   0x208880 <fprintf@plt>
   0x0000000000207f76 <+1238>:	jmp    0x207f78 <main+1240>
   0x0000000000207f78 <+1240>:	mov    edi,0x4
   0x0000000000207f7d <+1245>:	mov    esi,0x23
   0x0000000000207f82 <+1250>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207f87 <+1255>:	mov    rax,QWORD PTR [r14]
   0x0000000000207f8a <+1258>:	add    rax,0xffffffffffffffff
   0x0000000000207f8e <+1262>:	mov    QWORD PTR [r14],rax
   0x0000000000207f91 <+1265>:	cmp    rax,0x0
   0x0000000000207f95 <+1269>:	sete   al

Load r12 to rbx
(>>> set $r12=0x7ffff6f2a018)



   0x0000000000207f98 <+1272>:	mov    rbx,r12
   0x0000000000207f9b <+1275>:	test   al,0x1
   0x0000000000207f9d <+1277>:	jne    0x207fa4 <main+1284>

Loop end
   0x0000000000207f9f <+1279>:	jmp    0x20804e <main+1454>
   0x0000000000207fa4 <+1284>:	mov    edi,0x4
   0x0000000000207fa9 <+1289>:	mov    esi,0x24
   0x0000000000207fae <+1294>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207fb3 <+1299>:	jmp    0x207fb5 <main+1301>
   0x0000000000207fb5 <+1301>:	mov    edi,0x4
   0x0000000000207fba <+1306>:	mov    esi,0x25
   0x0000000000207fbf <+1311>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207fc4 <+1316>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x0000000000207fc8 <+1320>:	call   0x2087f0 <yk_location_drop@plt>
   0x0000000000207fcd <+1325>:	jmp    0x207fcf <main+1327>
   0x0000000000207fcf <+1327>:	mov    edi,0x4
   0x0000000000207fd4 <+1332>:	mov    esi,0x26
   0x0000000000207fd9 <+1337>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207fde <+1342>:	mov    rdi,r13
   0x0000000000207fe1 <+1345>:	call   0x2087c0 <yk_mt_shutdown@plt>
   0x0000000000207fe6 <+1350>:	jmp    0x207fe8 <main+1352>
   0x0000000000207fe8 <+1352>:	mov    edi,0x4
   0x0000000000207fed <+1357>:	mov    esi,0x27
   0x0000000000207ff2 <+1362>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000207ff7 <+1367>:	jmp    0x207ff9 <main+1369>
   0x0000000000207ff9 <+1369>:	mov    edi,0x4
   0x0000000000207ffe <+1374>:	mov    esi,0x28
   0x0000000000208003 <+1379>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208008 <+1384>:	jmp    0x20800a <main+1386>
   0x000000000020800a <+1386>:	mov    edi,0x4
   0x000000000020800f <+1391>:	mov    esi,0x29
   0x0000000000208014 <+1396>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208019 <+1401>:	jmp    0x20801b <main+1403>
   0x000000000020801b <+1403>:	mov    edi,0x4
   0x0000000000208020 <+1408>:	mov    esi,0x2a
   0x0000000000208025 <+1413>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020802a <+1418>:	jmp    0x20802c <main+1420>
   0x000000000020802c <+1420>:	mov    edi,0x4
   0x0000000000208031 <+1425>:	mov    esi,0x2b
   0x0000000000208036 <+1430>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020803b <+1435>:	jmp    0x20803d <main+1437>
   0x000000000020803d <+1437>:	mov    edi,0x4
   0x0000000000208042 <+1442>:	mov    esi,0x2c
   0x0000000000208047 <+1447>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020804c <+1452>:	jmp    0x208082 <main+1506>
   0x000000000020804e <+1454>:	mov    edi,0x4
   0x0000000000208053 <+1459>:	mov    esi,0x2d
   0x0000000000208058 <+1464>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>

Actual jump to start of the loop
   0x000000000020805d <+1469>:	jmp    0x207c6d <main+461>
   0x0000000000208062 <+1474>:	mov    edi,0x4
   0x0000000000208067 <+1479>:	mov    esi,0x2e
   0x000000000020806c <+1484>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208071 <+1489>:	xor    eax,eax
   0x0000000000208073 <+1491>:	add    rsp,0x78
   0x0000000000208077 <+1495>:	pop    rbx
   0x0000000000208078 <+1496>:	pop    r12
   0x000000000020807a <+1498>:	pop    r13
   0x000000000020807c <+1500>:	pop    r14
   0x000000000020807e <+1502>:	pop    r15
   0x0000000000208080 <+1504>:	pop    rbp
   0x0000000000208081 <+1505>:	ret
   0x0000000000208082 <+1506>:	mov    edi,0x4
   0x0000000000208087 <+1511>:	mov    esi,0x2f
   0x000000000020808c <+1516>:	call   0x2087e0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000208091 <+1521>:	jmp    0x208062 <main+1474>


c * 2
OptToUnopt:

end:

            x/gx $r13
            0x7ffff6f2a018: 0x0000000000000025
            x/gx $rbx
            0x20b660: 0x0000000000000000

UnoptToOpt:

end:

            x/gx $r13
            0x20b660: 0x0000000000000000
            x/gx $rbx
            0x7ffff6f2a018: 0x0000000000000025




OptToUnopt ExecTrace: true

            x/gx $r13
            0x20b660: 0x0000000000000000
            x/gx $rbx
            0x20b660: 0x0000000000000000

0x0000000000207f98  main+1272 mov    rbx,r12



DEBUG: Patchpoint #0 - Processing tracked reg 3 while handling DwarfRegNum 14
DEBUG: Patchpoint #0 - SpillOffsets for reg 3: { }
DEBUG: Patchpoint #0 - Processing tracked reg 6 while handling DwarfRegNum 14
DEBUG: Patchpoint #0 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #0 - Processing tracked reg 12 while handling DwarfRegNum 14
DEBUG: Patchpoint #0 - SpillOffsets for reg 12: { -64 }
DEBUG: Patchpoint #0 - Processing tracked reg 13 while handling DwarfRegNum 14
DEBUG: Patchpoint #0 - SpillOffsets for reg 13: { -72 5 }
DEBUG: Patchpoint #0 - Processing tracked reg 15 while handling DwarfRegNum 14
DEBUG: Patchpoint #0 - SpillOffsets for reg 15: { -96 }
DEBUG: Patchpoint #0 - Final Extras for reg 14: { -152 }
DEBUG: Patchpoint #0 - Initial Extras for reg 3: { }
DEBUG: Patchpoint #0 - Processing tracked reg 6 while handling DwarfRegNum 3
DEBUG: Patchpoint #0 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #0 - Processing tracked reg 12 while handling DwarfRegNum 3
DEBUG: Patchpoint #0 - SpillOffsets for reg 12: { -64 }
DEBUG: Patchpoint #0 - Processing tracked reg 13 while handling DwarfRegNum 3
DEBUG: Patchpoint #0 - SpillOffsets for reg 13: { -72 5 }
DEBUG: Patchpoint #0 - Processing tracked reg 14 while handling DwarfRegNum 3
DEBUG: Patchpoint #0 - SpillOffsets for reg 14: { -152 }
DEBUG: Patchpoint #0 - Processing tracked reg 15 while handling DwarfRegNum 3
DEBUG: Patchpoint #0 - SpillOffsets for reg 15: { -96 }
DEBUG: Patchpoint #0 - Final Extras for reg 3: { }
DEBUG: Patchpoint #0 - Initial Extras for reg 15: { -96 }
DEBUG: Patchpoint #0 - Processing tracked reg 3 while handling DwarfRegNum 15
DEBUG: Patchpoint #0 - SpillOffsets for reg 3: { }
DEBUG: Patchpoint #0 - Processing tracked reg 6 while handling DwarfRegNum 15
DEBUG: Patchpoint #0 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #0 - Processing tracked reg 12 while handling DwarfRegNum 15
DEBUG: Patchpoint #0 - SpillOffsets for reg 12: { -64 }
DEBUG: Patchpoint #0 - Processing tracked reg 13 while handling DwarfRegNum 15
DEBUG: Patchpoint #0 - SpillOffsets for reg 13: { -72 5 }
DEBUG: Patchpoint #0 - Processing tracked reg 14 while handling DwarfRegNum 15
DEBUG: Patchpoint #0 - SpillOffsets for reg 14: { -152 }
DEBUG: Patchpoint #0 - Final Extras for reg 15: { -96 }
DEBUG: Patchpoint #0 - Initial Extras for reg 12: { -64 }
DEBUG: Patchpoint #0 - Processing tracked reg 3 while handling DwarfRegNum 12
DEBUG: Patchpoint #0 - SpillOffsets for reg 3: { }
DEBUG: Patchpoint #0 - Processing tracked reg 6 while handling DwarfRegNum 12
DEBUG: Patchpoint #0 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #0 - Processing tracked reg 13 while handling DwarfRegNum 12
DEBUG: Patchpoint #0 - SpillOffsets for reg 13: { -72 5 }
DEBUG: Patchpoint #0 - Processing tracked reg 14 while handling DwarfRegNum 12
DEBUG: Patchpoint #0 - SpillOffsets for reg 14: { -152 }
DEBUG: Patchpoint #0 - Processing tracked reg 15 while handling DwarfRegNum 12
DEBUG: Patchpoint #0 - SpillOffsets for reg 15: { -96 }
DEBUG: Patchpoint #0 - Final Extras for reg 12: { -64 }
DEBUG: Patchpoint #0 - Initial Extras for reg 13: { -72 5 }
DEBUG: Patchpoint #0 - Processing tracked reg 3 while handling DwarfRegNum 13
DEBUG: Patchpoint #0 - SpillOffsets for reg 3: { }
DEBUG: Patchpoint #0 - Processing tracked reg 6 while handling DwarfRegNum 13
DEBUG: Patchpoint #0 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #0 - Processing tracked reg 12 while handling DwarfRegNum 13
DEBUG: Patchpoint #0 - SpillOffsets for reg 12: { -64 }
DEBUG: Patchpoint #0 - Processing tracked reg 14 while handling DwarfRegNum 13
DEBUG: Patchpoint #0 - SpillOffsets for reg 14: { -152 }
DEBUG: Patchpoint #0 - Processing tracked reg 15 while handling DwarfRegNum 13
DEBUG: Patchpoint #0 - SpillOffsets for reg 15: { -96 }
DEBUG: Patchpoint #0 - Final Extras for reg 13: { -72 5 }
DEBUG: Patchpoint #1 - Initial Extras for reg 14: { -104 }
DEBUG: Patchpoint #1 - Processing tracked reg 3 while handling DwarfRegNum 14
DEBUG: Patchpoint #1 - SpillOffsets for reg 3: { -80 5 }
DEBUG: Patchpoint #1 - Processing tracked reg 6 while handling DwarfRegNum 14
DEBUG: Patchpoint #1 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #1 - Processing tracked reg 12 while handling DwarfRegNum 14
DEBUG: Patchpoint #1 - SpillOffsets for reg 12: { -112 }
DEBUG: Patchpoint #1 - Processing tracked reg 13 while handling DwarfRegNum 14
DEBUG: Patchpoint #1 - SpillOffsets for reg 13: { -72 }
DEBUG: Patchpoint #1 - Processing tracked reg 15 while handling DwarfRegNum 14
DEBUG: Patchpoint #1 - SpillOffsets for reg 15: { -64 }
DEBUG: Patchpoint #1 - Final Extras for reg 14: { -104 }
DEBUG: Patchpoint #1 - Initial Extras for reg 13: { -72 }
DEBUG: Patchpoint #1 - Processing tracked reg 3 while handling DwarfRegNum 13
DEBUG: Patchpoint #1 - SpillOffsets for reg 3: { -80 5 }
DEBUG: Patchpoint #1 - Processing tracked reg 6 while handling DwarfRegNum 13
DEBUG: Patchpoint #1 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #1 - Processing tracked reg 12 while handling DwarfRegNum 13
DEBUG: Patchpoint #1 - SpillOffsets for reg 12: { -112 }
DEBUG: Patchpoint #1 - Processing tracked reg 14 while handling DwarfRegNum 13
DEBUG: Patchpoint #1 - SpillOffsets for reg 14: { -104 }
DEBUG: Patchpoint #1 - Processing tracked reg 15 while handling DwarfRegNum 13
DEBUG: Patchpoint #1 - SpillOffsets for reg 15: { -64 }
DEBUG: Patchpoint #1 - Final Extras for reg 13: { -72 }
DEBUG: Patchpoint #1 - Initial Extras for reg 15: { -64 }
DEBUG: Patchpoint #1 - Processing tracked reg 3 while handling DwarfRegNum 15
DEBUG: Patchpoint #1 - SpillOffsets for reg 3: { -80 5 }
DEBUG: Patchpoint #1 - Processing tracked reg 6 while handling DwarfRegNum 15
DEBUG: Patchpoint #1 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #1 - Processing tracked reg 12 while handling DwarfRegNum 15
DEBUG: Patchpoint #1 - SpillOffsets for reg 12: { -112 }
DEBUG: Patchpoint #1 - Processing tracked reg 13 while handling DwarfRegNum 15
DEBUG: Patchpoint #1 - SpillOffsets for reg 13: { -72 }
DEBUG: Patchpoint #1 - Processing tracked reg 14 while handling DwarfRegNum 15
DEBUG: Patchpoint #1 - SpillOffsets for reg 14: { -104 }
DEBUG: Patchpoint #1 - Final Extras for reg 15: { -64 }
DEBUG: Patchpoint #1 - Initial Extras for reg 12: { -112 }
DEBUG: Patchpoint #1 - Processing tracked reg 3 while handling DwarfRegNum 12
DEBUG: Patchpoint #1 - SpillOffsets for reg 3: { -80 5 }
DEBUG: Patchpoint #1 - Processing tracked reg 6 while handling DwarfRegNum 12
DEBUG: Patchpoint #1 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #1 - Processing tracked reg 13 while handling DwarfRegNum 12
DEBUG: Patchpoint #1 - SpillOffsets for reg 13: { -72 }
DEBUG: Patchpoint #1 - Processing tracked reg 14 while handling DwarfRegNum 12
DEBUG: Patchpoint #1 - SpillOffsets for reg 14: { -104 }
DEBUG: Patchpoint #1 - Processing tracked reg 15 while handling DwarfRegNum 12
DEBUG: Patchpoint #1 - SpillOffsets for reg 15: { -64 }
DEBUG: Patchpoint #1 - Final Extras for reg 12: { -112 }
DEBUG: Patchpoint #1 - Initial Extras for reg 3: { -80 5 }
DEBUG: Patchpoint #1 - Processing tracked reg 6 while handling DwarfRegNum 3
DEBUG: Patchpoint #1 - SpillOffsets for reg 6: { }
DEBUG: Patchpoint #1 - Processing tracked reg 12 while handling DwarfRegNum 3
DEBUG: Patchpoint #1 - SpillOffsets for reg 12: { -112 }
DEBUG: Patchpoint #1 - Processing tracked reg 13 while handling DwarfRegNum 3
DEBUG: Patchpoint #1 - SpillOffsets for reg 13: { -72 }
DEBUG: Patchpoint #1 - Processing tracked reg 14 while handling DwarfRegNum 3
DEBUG: Patchpoint #1 - SpillOffsets for reg 14: { -104 }
DEBUG: Patchpoint #1 - Processing tracked reg 15 while handling DwarfRegNum 3
DEBUG: Patchpoint #1 - SpillOffsets for reg 15: { -64 }
DEBUG: Patchpoint #1 - Final Extras for reg 3: { -80 5 }
