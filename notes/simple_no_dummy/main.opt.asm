Dump of assembler code for function main:
   0x0000000000203000 <+0>:	push   rbp
   0x0000000000203001 <+1>:	mov    rbp,rsp
   0x0000000000203004 <+4>:	push   r15
   0x0000000000203006 <+6>:	push   r14
   0x0000000000203008 <+8>:	push   r12
   0x000000000020300a <+10>:	push   rbx
   0x000000000020300b <+11>:	sub    rsp,0x10
   0x000000000020300f <+15>:	mov    rbx,rsi
   0x0000000000203012 <+18>:	mov    r14d,edi
   0x0000000000203015 <+21>:	mov    edi,0xf4240
   0x000000000020301a <+26>:	call   0x203460 <malloc@plt>
   0x000000000020301f <+31>:	mov    rcx,rax
   0x0000000000203022 <+34>:	add    rcx,0x20
   0x0000000000203026 <+38>:	mov    rdx,0xfffffffffffffff8
   0x000000000020302d <+45>:	mov    QWORD PTR fs:[rdx],rax
   0x0000000000203031 <+49>:	mov    rdx,0xfffffffffffffff0
   0x0000000000203038 <+56>:	mov    QWORD PTR fs:[rdx],rcx
   0x000000000020303c <+60>:	mov    rcx,rax
   0x000000000020303f <+63>:	add    rcx,0x8
   0x0000000000203043 <+67>:	mov    r15,rax
   0x0000000000203046 <+70>:	add    r15,0x10
   0x000000000020304a <+74>:	mov    r12,rax
   0x000000000020304d <+77>:	add    r12,0x14
   0x0000000000203051 <+81>:	mov    DWORD PTR [rax],0x0
   0x0000000000203057 <+87>:	mov    DWORD PTR [rax+0x4],r14d
=> 0x000000000020305b <+91>:	jmp    0x20305d <main+93>
   0x000000000020305d <+93>:	mov    QWORD PTR [rcx],rbx
   0x0000000000203060 <+96>:	xor    edi,edi
   0x0000000000203062 <+98>:	call   0x203420 <yk_mt_new@plt>
   0x0000000000203067 <+103>:	jmp    0x203069 <main+105>
   0x0000000000203069 <+105>:	mov    QWORD PTR [rbp-0x30],rax
   0x000000000020306d <+109>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x0000000000203071 <+113>:	xor    esi,esi
   0x0000000000203073 <+115>:	call   0x203430 <yk_mt_hot_threshold_set@plt>
   0x0000000000203078 <+120>:	jmp    0x20307a <main+122>
   0x000000000020307a <+122>:	call   0x203440 <yk_location_new@plt>
   0x000000000020307f <+127>:	jmp    0x203081 <main+129>
   0x0000000000203081 <+129>:	mov    QWORD PTR [rbp-0x28],rax
   0x0000000000203085 <+133>:	mov    DWORD PTR [r15],0x270e
   0x000000000020308c <+140>:	mov    DWORD PTR [r12],0x4
   0x0000000000203094 <+148>:	mov    rax,QWORD PTR [rbp-0x28]
   0x0000000000203098 <+152>:	jmp    0x20309a <main+154>
   0x000000000020309a <+154>:	mov    eax,DWORD PTR [r15]
   0x000000000020309d <+157>:	jmp    0x20309f <main+159>
   0x000000000020309f <+159>:	mov    eax,DWORD PTR [r12]
   0x00000000002030a3 <+163>:	jmp    0x2030a5 <main+165>
   0x00000000002030a5 <+165>:	jmp    0x2030a7 <main+167>
   0x00000000002030a7 <+167>:	cmp    DWORD PTR [r12],0x0
   0x00000000002030ac <+172>:	setg   al
   0x00000000002030af <+175>:	test   al,0x1
   0x00000000002030b1 <+177>:	jne    0x2030b5 <main+181>
   0x00000000002030b3 <+179>:	jmp    0x2030fa <main+250>
   0x00000000002030b5 <+181>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x00000000002030b9 <+185>:	lea    rsi,[rbp-0x28]
   0x00000000002030bd <+189>:	xor    edx,edx
   0x00000000002030bf <+191>:	movabs r11,0x203410
   0x00000000002030c9 <+201>:	call   r11
   0x00000000002030cc <+204>:	jmp    0x2030ce <main+206>
   0x00000000002030ce <+206>:	mov    rdi,QWORD PTR ds:0x205720
   0x00000000002030d6 <+214>:	mov    edx,DWORD PTR [r12]
   0x00000000002030da <+218>:	movabs rsi,0x20563a
   0x00000000002030e4 <+228>:	mov    al,0x0
   0x00000000002030e6 <+230>:	call   0x203450 <fprintf@plt>
   0x00000000002030eb <+235>:	jmp    0x2030ed <main+237>
   0x00000000002030ed <+237>:	mov    eax,DWORD PTR [r12]
   0x00000000002030f1 <+241>:	add    eax,0xffffffff
   0x00000000002030f4 <+244>:	mov    DWORD PTR [r12],eax
   0x00000000002030f8 <+248>:	jmp    0x2030a7 <main+167>
   0x00000000002030fa <+250>:	mov    rdi,QWORD PTR ds:0x205720
   0x0000000000203102 <+258>:	movabs rsi,0x205634
   0x000000000020310c <+268>:	mov    al,0x0
   0x000000000020310e <+270>:	call   0x203450 <fprintf@plt>
   0x0000000000203113 <+275>:	jmp    0x203115 <main+277>
   0x0000000000203115 <+277>:	mov    eax,DWORD PTR [r15]
   0x0000000000203118 <+280>:	jmp    0x20311a <main+282>
   0x000000000020311a <+282>:	mov    rdi,QWORD PTR [rbp-0x28]
   0x000000000020311e <+286>:	call   0x2033f0 <yk_location_drop@plt>
   0x0000000000203123 <+291>:	jmp    0x203125 <main+293>
   0x0000000000203125 <+293>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x0000000000203129 <+297>:	call   0x2033e0 <yk_mt_shutdown@plt>
   0x000000000020312e <+302>:	jmp    0x203130 <main+304>
   0x0000000000203130 <+304>:	jmp    0x203141 <main+321>
   0x0000000000203132 <+306>:	xor    eax,eax
   0x0000000000203134 <+308>:	add    rsp,0x10
   0x0000000000203138 <+312>:	pop    rbx
   0x0000000000203139 <+313>:	pop    r12
   0x000000000020313b <+315>:	pop    r14
   0x000000000020313d <+317>:	pop    r15
   0x000000000020313f <+319>:	pop    rbp
   0x0000000000203140 <+320>:	ret
   0x0000000000203141 <+321>:	jmp    0x203132 <main+306>
