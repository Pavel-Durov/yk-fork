0x0000000000204e30 <+0>:	push   rbp
   0x0000000000204e31 <+1>:	mov    rbp,rsp
   0x0000000000204e34 <+4>:	push   r15
   0x0000000000204e36 <+6>:	push   r14
   0x0000000000204e38 <+8>:	push   r13
   0x0000000000204e3a <+10>:	push   r12
   0x0000000000204e3c <+12>:	push   rbx
   0x0000000000204e3d <+13>:	sub    rsp,0x58
   0x0000000000204e41 <+17>:	mov    r14,rsi
   0x0000000000204e44 <+20>:	mov    r12d,edi
   0x0000000000204e47 <+23>:	mov    edi,0x2
   0x0000000000204e4c <+28>:	xor    esi,esi
   0x0000000000204e4e <+30>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204e53 <+35>:	mov    edi,0xf4240
   0x0000000000204e58 <+40>:	call   0x205620 <malloc@plt>
   0x0000000000204e5d <+45>:	mov    rcx,rax
   0x0000000000204e60 <+48>:	add    rcx,0x34
   0x0000000000204e64 <+52>:	lea    rdx,[rip+0x2a1d]        # 0x207888 <shadowstack_0>
   0x0000000000204e6b <+59>:	mov    QWORD PTR [rdx],rcx
   0x0000000000204e6e <+62>:	mov    rbx,rax
   0x0000000000204e71 <+65>:	add    rbx,0x8
   0x0000000000204e75 <+69>:	mov    r15,rax
   0x0000000000204e78 <+72>:	add    r15,0x10
   0x0000000000204e7c <+76>:	mov    rcx,rax
   0x0000000000204e7f <+79>:	add    rcx,0x14
   0x0000000000204e83 <+83>:	mov    QWORD PTR [rbp-0x68],rcx
   0x0000000000204e87 <+87>:	mov    rcx,rax
   0x0000000000204e8a <+90>:	add    rcx,0x18
   0x0000000000204e8e <+94>:	mov    QWORD PTR [rbp-0x80],rcx
   0x0000000000204e92 <+98>:	mov    rcx,rax
   0x0000000000204e95 <+101>:	add    rcx,0x1c
   0x0000000000204e99 <+105>:	mov    QWORD PTR [rbp-0x58],rcx
   0x0000000000204e9d <+109>:	mov    rcx,rax
   0x0000000000204ea0 <+112>:	add    rcx,0x20
   0x0000000000204ea4 <+116>:	mov    QWORD PTR [rbp-0x78],rcx
   0x0000000000204ea8 <+120>:	mov    rcx,rax
   0x0000000000204eab <+123>:	add    rcx,0x24
   0x0000000000204eaf <+127>:	mov    QWORD PTR [rbp-0x70],rcx
   0x0000000000204eb3 <+131>:	mov    rcx,rax
   0x0000000000204eb6 <+134>:	add    rcx,0x28
   0x0000000000204eba <+138>:	mov    QWORD PTR [rbp-0x50],rcx
   0x0000000000204ebe <+142>:	mov    rcx,rax
   0x0000000000204ec1 <+145>:	add    rcx,0x2c
   0x0000000000204ec5 <+149>:	mov    QWORD PTR [rbp-0x48],rcx
   0x0000000000204ec9 <+153>:	mov    rcx,rax
   0x0000000000204ecc <+156>:	add    rcx,0x30
   0x0000000000204ed0 <+160>:	mov    QWORD PTR [rbp-0x40],rcx
   0x0000000000204ed4 <+164>:	mov    DWORD PTR [rax],0x0
   0x0000000000204eda <+170>:	mov    DWORD PTR [rax+0x4],r12d
   0x0000000000204ede <+174>:	jmp    0x204ee0 <main+176>
   0x0000000000204ee0 <+176>:	mov    edi,0x2
   0x0000000000204ee5 <+181>:	mov    esi,0x1
   0x0000000000204eea <+186>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204eef <+191>:	mov    QWORD PTR [rbx],r14
   0x0000000000204ef2 <+194>:	xor    edi,edi
   0x0000000000204ef4 <+196>:	call   0x2055e0 <yk_mt_new@plt>
   0x0000000000204ef9 <+201>:	mov    rbx,rax
   0x0000000000204efc <+204>:	jmp    0x204efe <main+206>
   0x0000000000204efe <+206>:	mov    edi,0x2
   0x0000000000204f03 <+211>:	mov    esi,0x2
   0x0000000000204f08 <+216>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204f0d <+221>:	mov    QWORD PTR [rbp-0x38],rbx
   0x0000000000204f11 <+225>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x0000000000204f15 <+229>:	xor    esi,esi
   0x0000000000204f17 <+231>:	call   0x2055f0 <yk_mt_hot_threshold_set@plt>
   0x0000000000204f1c <+236>:	jmp    0x204f1e <main+238>
   0x0000000000204f1e <+238>:	mov    edi,0x2
   0x0000000000204f23 <+243>:	mov    esi,0x3
   0x0000000000204f28 <+248>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204f2d <+253>:	call   0x205600 <yk_location_new@plt>
   0x0000000000204f32 <+258>:	mov    rbx,rax
   0x0000000000204f35 <+261>:	jmp    0x204f37 <main+263>
   0x0000000000204f37 <+263>:	mov    edi,0x2
   0x0000000000204f3c <+268>:	mov    esi,0x4
   0x0000000000204f41 <+273>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204f46 <+278>:	mov    QWORD PTR [rbp-0x30],rbx
   0x0000000000204f4a <+282>:	mov    DWORD PTR [r15],0x4
   0x0000000000204f51 <+289>:	mov    rax,QWORD PTR [rbp-0x30]
   0x0000000000204f55 <+293>:	jmp    0x204f57 <main+295>
   0x0000000000204f57 <+295>:	mov    edi,0x2
   0x0000000000204f5c <+300>:	mov    esi,0x5
   0x0000000000204f61 <+305>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204f66 <+310>:	mov    eax,DWORD PTR [r15]
   0x0000000000204f69 <+313>:	jmp    0x204f6b <main+315>
   0x0000000000204f6b <+315>:	mov    edi,0x2
   0x0000000000204f70 <+320>:	mov    esi,0x6
   0x0000000000204f75 <+325>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204f7a <+330>:	jmp    0x204f7c <main+332>
   0x0000000000204f7c <+332>:	mov    edi,0x2
   0x0000000000204f81 <+337>:	mov    esi,0x7
   0x0000000000204f86 <+342>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204f8b <+347>:	cmp    DWORD PTR [r15],0x0
   0x0000000000204f8f <+351>:	setg   al
   0x0000000000204f92 <+354>:	mov    r14,QWORD PTR [rbp-0x80]
   0x0000000000204f96 <+358>:	mov    r12,QWORD PTR [rbp-0x58]
   0x0000000000204f9a <+362>:	mov    r13,QWORD PTR [rbp-0x78]
   0x0000000000204f9e <+366>:	mov    rbx,QWORD PTR [rbp-0x70]
   0x0000000000204fa2 <+370>:	mov    rcx,QWORD PTR [rbp-0x50]
   0x0000000000204fa6 <+374>:	mov    rdx,QWORD PTR [rbp-0x48]
   0x0000000000204faa <+378>:	mov    rsi,QWORD PTR [rbp-0x40]
   0x0000000000204fae <+382>:	mov    rdi,QWORD PTR [rbp-0x68]
   0x0000000000204fb2 <+386>:	test   al,0x1
   0x0000000000204fb4 <+388>:	jne    0x204fbb <main+395>
   0x0000000000204fb6 <+390>:	jmp    0x2050e6 <main+694>
   0x0000000000204fbb <+395>:	mov    edi,0x2
   0x0000000000204fc0 <+400>:	mov    QWORD PTR [rbp-0x60],r15
   0x0000000000204fc4 <+404>:	mov    esi,0x8
   0x0000000000204fc9 <+409>:	mov    r12,r13
   0x0000000000204fcc <+412>:	mov    r13,rbx
   0x0000000000204fcf <+415>:	mov    rbx,rcx
   0x0000000000204fd2 <+418>:	mov    r15,rdx
   0x0000000000204fd5 <+421>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000204fda <+426>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x0000000000204fde <+430>:	lea    rsi,[rbp-0x30]
   0x0000000000204fe2 <+434>:	xor    edx,edx
   0x0000000000204fe4 <+436>:	mov    rcx,rbx
   0x0000000000204fe7 <+439>:	mov    rbx,QWORD PTR [rbp-0x68]
   0x0000000000204feb <+443>:	movabs r11,0x2055d0
   0x0000000000204ff5 <+453>:	call   r11
   0x0000000000204ff8 <+456>:	mov    r15,QWORD PTR [rbp-0x60]
   0x0000000000204ffc <+460>:	jmp    0x204ffe <main+462>
   0x0000000000204ffe <+462>:	mov    edi,0x2
   0x0000000000205003 <+467>:	mov    esi,0x9
   0x0000000000205008 <+472>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020500d <+477>:	mov    eax,DWORD PTR [r15]
   0x0000000000205010 <+480>:	and    eax,0x1
   0x0000000000205013 <+483>:	mov    DWORD PTR [rbx],eax
   0x0000000000205015 <+485>:	mov    eax,DWORD PTR [r15]
   0x0000000000205018 <+488>:	or     eax,0x1
   0x000000000020501b <+491>:	mov    DWORD PTR [r14],eax
   0x000000000020501e <+494>:	mov    eax,DWORD PTR [r15]
   0x0000000000205021 <+497>:	shr    eax,1
   0x0000000000205023 <+499>:	mov    rsi,QWORD PTR [rbp-0x58]
   0x0000000000205027 <+503>:	mov    DWORD PTR [rsi],eax
   0x0000000000205029 <+505>:	mov    eax,DWORD PTR [r15]
   0x000000000020502c <+508>:	sar    eax,1
   0x000000000020502e <+510>:	mov    DWORD PTR [r12],eax
   0x0000000000205032 <+514>:	xor    eax,eax
   0x0000000000205034 <+516>:	sub    eax,DWORD PTR [r15]
   0x0000000000205037 <+519>:	sar    eax,1
   0x0000000000205039 <+521>:	mov    DWORD PTR [r13+0x0],eax
   0x000000000020503d <+525>:	mov    eax,DWORD PTR [r15]
   0x0000000000205040 <+528>:	xor    eax,0x1
   0x0000000000205043 <+531>:	mov    rcx,QWORD PTR [rbp-0x50]
   0x0000000000205047 <+535>:	mov    DWORD PTR [rcx],eax
   0x0000000000205049 <+537>:	mov    eax,DWORD PTR [r15]
   0x000000000020504c <+540>:	xor    eax,0xffffffff
   0x000000000020504f <+543>:	mov    rcx,QWORD PTR [rbp-0x48]
   0x0000000000205053 <+547>:	mov    DWORD PTR [rcx],eax
   0x0000000000205055 <+549>:	mov    eax,DWORD PTR [r15]
   0x0000000000205058 <+552>:	shl    eax,1
   0x000000000020505a <+554>:	mov    rcx,QWORD PTR [rbp-0x40]
   0x000000000020505e <+558>:	mov    DWORD PTR [rcx],eax
   0x0000000000205060 <+560>:	mov    rdi,QWORD PTR ds:0x2078a0
   0x0000000000205068 <+568>:	mov    edx,DWORD PTR [rbx]
   0x000000000020506a <+570>:	mov    ecx,DWORD PTR [r14]
   0x000000000020506d <+573>:	mov    r8d,DWORD PTR [rsi]
   0x0000000000205070 <+576>:	mov    r9d,DWORD PTR [r12]
   0x0000000000205074 <+580>:	movabs rsi,0x204714
   0x000000000020507e <+590>:	mov    al,0x0
   0x0000000000205080 <+592>:	call   0x205610 <fprintf@plt>
   0x0000000000205085 <+597>:	jmp    0x205087 <main+599>
   0x0000000000205087 <+599>:	mov    edi,0x2
   0x000000000020508c <+604>:	mov    esi,0xa
   0x0000000000205091 <+609>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000205096 <+614>:	mov    rdi,QWORD PTR ds:0x2078a0
   0x000000000020509e <+622>:	mov    edx,DWORD PTR [r13+0x0]
   0x00000000002050a2 <+626>:	mov    rax,QWORD PTR [rbp-0x50]
   0x00000000002050a6 <+630>:	mov    ecx,DWORD PTR [rax]
   0x00000000002050a8 <+632>:	mov    rax,QWORD PTR [rbp-0x48]
   0x00000000002050ac <+636>:	mov    r8d,DWORD PTR [rax]
   0x00000000002050af <+639>:	mov    rax,QWORD PTR [rbp-0x40]
   0x00000000002050b3 <+643>:	mov    r9d,DWORD PTR [rax]
   0x00000000002050b6 <+646>:	movabs rsi,0x204738
   0x00000000002050c0 <+656>:	mov    al,0x0
   0x00000000002050c2 <+658>:	call   0x205610 <fprintf@plt>
   0x00000000002050c7 <+663>:	jmp    0x2050c9 <main+665>
   0x00000000002050c9 <+665>:	mov    edi,0x2
   0x00000000002050ce <+670>:	mov    esi,0xb
   0x00000000002050d3 <+675>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x00000000002050d8 <+680>:	mov    eax,DWORD PTR [r15]
   0x00000000002050db <+683>:	add    eax,0xffffffff
   0x00000000002050de <+686>:	mov    DWORD PTR [r15],eax
   0x00000000002050e1 <+689>:	jmp    0x204f7c <main+332>
   0x00000000002050e6 <+694>:	mov    edi,0x2
   0x00000000002050eb <+699>:	mov    esi,0xc
   0x00000000002050f0 <+704>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x00000000002050f5 <+709>:	mov    rdi,QWORD PTR ds:0x2078a0
   0x00000000002050fd <+717>:	movabs rsi,0x204732
   0x0000000000205107 <+727>:	mov    al,0x0
   0x0000000000205109 <+729>:	call   0x205610 <fprintf@plt>
   0x000000000020510e <+734>:	jmp    0x205110 <main+736>
   0x0000000000205110 <+736>:	mov    edi,0x2
   0x0000000000205115 <+741>:	mov    esi,0xd
   0x000000000020511a <+746>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x000000000020511f <+751>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x0000000000205123 <+755>:	call   0x2055b0 <yk_location_drop@plt>
   0x0000000000205128 <+760>:	jmp    0x20512a <main+762>
   0x000000000020512a <+762>:	mov    edi,0x2
   0x000000000020512f <+767>:	mov    esi,0xe
   0x0000000000205134 <+772>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000205139 <+777>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x000000000020513d <+781>:	call   0x205590 <yk_mt_shutdown@plt>
   0x0000000000205142 <+786>:	jmp    0x205144 <main+788>
   0x0000000000205144 <+788>:	mov    edi,0x2
   0x0000000000205149 <+793>:	mov    esi,0xf
   0x000000000020514e <+798>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000205153 <+803>:	jmp    0x205175 <main+837>
   0x0000000000205155 <+805>:	mov    edi,0x2
   0x000000000020515a <+810>:	mov    esi,0x10
   0x000000000020515f <+815>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000205164 <+820>:	xor    eax,eax
   0x0000000000205166 <+822>:	add    rsp,0x58
   0x000000000020516a <+826>:	pop    rbx
   0x000000000020516b <+827>:	pop    r12
   0x000000000020516d <+829>:	pop    r13
   0x000000000020516f <+831>:	pop    r14
   0x0000000000205171 <+833>:	pop    r15
   0x0000000000205173 <+835>:	pop    rbp
   0x0000000000205174 <+836>:	ret
   0x0000000000205175 <+837>:	mov    edi,0x2
   0x000000000020517a <+842>:	mov    esi,0x11
   0x000000000020517f <+847>:	call   0x2055a0 <__yk_trace_basicblock_dummy@plt>
   0x0000000000205184 <+852>:	jmp    0x205155 <main+805>
