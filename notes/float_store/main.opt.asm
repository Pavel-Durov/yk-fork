0x0000000000204e50 <+0>:	push   rbp
0x0000000000204e51 <+1>:	mov    rbp,rsp
0x0000000000204e54 <+4>:	push   r15
0x0000000000204e56 <+6>:	push   r14
0x0000000000204e58 <+8>:	push   r13
0x0000000000204e5a <+10>:	push   r12
0x0000000000204e5c <+12>:	push   rbx
0x0000000000204e5d <+13>:	sub    rsp,0x48
0x0000000000204e61 <+17>:	mov    r12,rsi
0x0000000000204e64 <+20>:	mov    r14d,edi
0x0000000000204e67 <+23>:	xor    edi,edi
0x0000000000204e69 <+25>:	xor    esi,esi
0x0000000000204e6b <+27>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204e70 <+32>:	mov    edi,0xf4240
0x0000000000204e75 <+37>:	call   0x205730 <malloc@plt>
0x0000000000204e7a <+42>:	mov    rcx,rax
0x0000000000204e7d <+45>:	add    rcx,0x50
0x0000000000204e81 <+49>:	mov    rdx,0xfffffffffffffff8
0x0000000000204e88 <+56>:	mov    QWORD PTR fs:[rdx],rax
0x0000000000204e8c <+60>:	mov    rdx,0xfffffffffffffff0
0x0000000000204e93 <+67>:	mov    QWORD PTR fs:[rdx],rcx
0x0000000000204e97 <+71>:	mov    rbx,rax
0x0000000000204e9a <+74>:	add    rbx,0x8
0x0000000000204e9e <+78>:	mov    r15,rax
0x0000000000204ea1 <+81>:	add    r15,0x10
0x0000000000204ea5 <+85>:	mov    rdx,rax
0x0000000000204ea8 <+88>:	add    rdx,0x14
0x0000000000204eac <+92>:	mov    rcx,rax
0x0000000000204eaf <+95>:	add    rcx,0x18
0x0000000000204eb3 <+99>:	mov    QWORD PTR [rbp-0x50],rcx
0x0000000000204eb7 <+103>:	mov    r13,rax
0x0000000000204eba <+106>:	add    r13,0x20
0x0000000000204ebe <+110>:	mov    rcx,rax
0x0000000000204ec1 <+113>:	add    rcx,0x28
0x0000000000204ec5 <+117>:	mov    QWORD PTR [rbp-0x48],rcx
0x0000000000204ec9 <+121>:	mov    rcx,rax
0x0000000000204ecc <+124>:	add    rcx,0x30
0x0000000000204ed0 <+128>:	mov    QWORD PTR [rbp-0x40],rcx
0x0000000000204ed4 <+132>:	mov    rcx,rax
0x0000000000204ed7 <+135>:	add    rcx,0x38
0x0000000000204edb <+139>:	mov    QWORD PTR [rbp-0x60],rcx
0x0000000000204edf <+143>:	mov    rcx,rax
0x0000000000204ee2 <+146>:	add    rcx,0x40
0x0000000000204ee6 <+150>:	mov    QWORD PTR [rbp-0x70],rcx
0x0000000000204eea <+154>:	mov    DWORD PTR [rax],0x0
0x0000000000204ef0 <+160>:	mov    DWORD PTR [rax+0x4],r14d
0x0000000000204ef4 <+164>:	jmp    0x204ef6 <main+166>
0x0000000000204ef6 <+166>:	mov    QWORD PTR [rbp-0x68],rdx
0x0000000000204efa <+170>:	xor    edi,edi
0x0000000000204efc <+172>:	mov    esi,0x1
0x0000000000204f01 <+177>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204f06 <+182>:	mov    QWORD PTR [rbx],r12
0x0000000000204f09 <+185>:	xor    edi,edi
0x0000000000204f0b <+187>:	call   0x205700 <yk_mt_new@plt>
0x0000000000204f10 <+192>:	mov    rbx,rax
0x0000000000204f13 <+195>:	jmp    0x204f15 <main+197>
0x0000000000204f15 <+197>:	xor    edi,edi
0x0000000000204f17 <+199>:	mov    esi,0x2
0x0000000000204f1c <+204>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204f21 <+209>:	mov    QWORD PTR [rbp-0x38],rbx
0x0000000000204f25 <+213>:	mov    rdi,QWORD PTR [rbp-0x38]
0x0000000000204f29 <+217>:	xor    esi,esi
0x0000000000204f2b <+219>:	call   0x2056b0 <yk_mt_hot_threshold_set@plt>
0x0000000000204f30 <+224>:	mov    r12,QWORD PTR [rbp-0x60]
0x0000000000204f34 <+228>:	jmp    0x204f36 <main+230>
0x0000000000204f36 <+230>:	xor    edi,edi
0x0000000000204f38 <+232>:	mov    esi,0x3
0x0000000000204f3d <+237>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204f42 <+242>:	call   0x2056e0 <yk_location_new@plt>
0x0000000000204f47 <+247>:	mov    rbx,rax
0x0000000000204f4a <+250>:	mov    r14,QWORD PTR [rbp-0x48]
0x0000000000204f4e <+254>:	jmp    0x204f50 <main+256>
0x0000000000204f50 <+256>:	xor    edi,edi
0x0000000000204f52 <+258>:	mov    esi,0x4
0x0000000000204f57 <+263>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204f5c <+268>:	mov    QWORD PTR [rbp-0x30],rbx
0x0000000000204f60 <+272>:	mov    DWORD PTR [r15],0x4
0x0000000000204f67 <+279>:	mov    rax,QWORD PTR [rbp-0x68]
0x0000000000204f6b <+283>:	mov    rcx,QWORD PTR [rbp-0x50]
0x0000000000204f6f <+287>:	mov    QWORD PTR [rcx],rax
0x0000000000204f72 <+290>:	mov    QWORD PTR [r13+0x0],rax
0x0000000000204f76 <+294>:	mov    rax,QWORD PTR [rbp-0x40]
0x0000000000204f7a <+298>:	mov    QWORD PTR [rax],r14
0x0000000000204f7d <+301>:	mov    rax,QWORD PTR [rbp-0x30]
0x0000000000204f81 <+305>:	jmp    0x204f83 <main+307>
0x0000000000204f83 <+307>:	xor    edi,edi
0x0000000000204f85 <+309>:	mov    esi,0x5
0x0000000000204f8a <+314>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204f8f <+319>:	mov    eax,DWORD PTR [r15]
0x0000000000204f92 <+322>:	mov    rbx,r12
0x0000000000204f95 <+325>:	jmp    0x204f97 <main+327>
0x0000000000204f97 <+327>:	xor    edi,edi
0x0000000000204f99 <+329>:	mov    esi,0x6
0x0000000000204f9e <+334>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204fa3 <+339>:	mov    r12,QWORD PTR [rbp-0x68]
0x0000000000204fa7 <+343>:	movss  xmm0,DWORD PTR [r12]
0x0000000000204fad <+349>:	mov    r14,QWORD PTR [rbp-0x50]
0x0000000000204fb1 <+353>:	jmp    0x204fb3 <main+355>
0x0000000000204fb3 <+355>:	xor    edi,edi
0x0000000000204fb5 <+357>:	mov    esi,0x7
0x0000000000204fba <+362>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204fbf <+367>:	mov    rax,QWORD PTR [r14]
0x0000000000204fc2 <+370>:	jmp    0x204fc4 <main+372>
0x0000000000204fc4 <+372>:	xor    edi,edi
0x0000000000204fc6 <+374>:	mov    esi,0x8
0x0000000000204fcb <+379>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204fd0 <+384>:	mov    rax,QWORD PTR [r13+0x0]
0x0000000000204fd4 <+388>:	jmp    0x204fd6 <main+390>
0x0000000000204fd6 <+390>:	xor    edi,edi
0x0000000000204fd8 <+392>:	mov    esi,0x9
0x0000000000204fdd <+397>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204fe2 <+402>:	mov    rax,QWORD PTR [rbp-0x48]
0x0000000000204fe6 <+406>:	movsd  xmm0,QWORD PTR [rax]
0x0000000000204fea <+410>:	jmp    0x204fec <main+412>
0x0000000000204fec <+412>:	xor    edi,edi
0x0000000000204fee <+414>:	mov    esi,0xa
0x0000000000204ff3 <+419>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000204ff8 <+424>:	mov    rcx,QWORD PTR [rbp-0x40]
0x0000000000204ffc <+428>:	mov    rax,QWORD PTR [rcx]
0x0000000000204fff <+431>:	jmp    0x205001 <main+433>
0x0000000000205001 <+433>:	xor    edi,edi
0x0000000000205003 <+435>:	mov    esi,0xb
0x0000000000205008 <+440>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x000000000020500d <+445>:	jmp    0x20500f <main+447>
0x000000000020500f <+447>:	xor    edi,edi
0x0000000000205011 <+449>:	mov    esi,0xc
0x0000000000205016 <+454>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x000000000020501b <+459>:	cmp    DWORD PTR [r15],0x0
0x000000000020501f <+463>:	setg   al
0x0000000000205022 <+466>:	mov    rcx,QWORD PTR [rbp-0x40]
0x0000000000205026 <+470>:	mov    rdx,QWORD PTR [rbp-0x70]
0x000000000020502a <+474>:	mov    rsi,QWORD PTR [rbp-0x48]
0x000000000020502e <+478>:	test   al,0x1
0x0000000000205030 <+480>:	jne    0x205037 <main+487>
0x0000000000205032 <+482>:	jmp    0x2051df <main+911>
0x0000000000205037 <+487>:	xor    edi,edi
0x0000000000205039 <+489>:	mov    rbx,rsi
0x000000000020503c <+492>:	mov    esi,0xd
0x0000000000205041 <+497>:	mov    r14,rcx
0x0000000000205044 <+500>:	mov    QWORD PTR [rbp-0x58],r13
0x0000000000205048 <+504>:	mov    r13,rdx
0x000000000020504b <+507>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000205050 <+512>:	mov    rdi,QWORD PTR [rbp-0x38]
0x0000000000205054 <+516>:	lea    rsi,[rbp-0x30]
0x0000000000205058 <+520>:	xor    edx,edx
0x000000000020505a <+522>:	movabs r11,0x2056d0
0x0000000000205064 <+532>:	call   r11
0x0000000000205067 <+535>:	mov    r14,QWORD PTR [rbp-0x50]
0x000000000020506b <+539>:	mov    r13,QWORD PTR [rbp-0x58]
0x000000000020506f <+543>:	mov    rbx,QWORD PTR [rbp-0x60]
0x0000000000205073 <+547>:	jmp    0x205075 <main+549>
0x0000000000205075 <+549>:	xor    edi,edi
0x0000000000205077 <+551>:	mov    esi,0xe
0x000000000020507c <+556>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000205081 <+561>:	cvtsi2ss xmm0,DWORD PTR [r15]
0x0000000000205086 <+566>:	movss  xmm1,DWORD PTR [rip+0x2872]        # 0x207900
0x000000000020508e <+574>:	divss  xmm0,xmm1
0x0000000000205092 <+578>:	movss  DWORD PTR [rbx],xmm0
0x0000000000205096 <+582>:	mov    rdi,QWORD PTR ds:0x2079e0
0x000000000020509e <+590>:	mov    edx,DWORD PTR [r15]
0x00000000002050a1 <+593>:	movss  xmm0,DWORD PTR [rbx]
0x00000000002050a5 <+597>:	cvtss2sd xmm0,xmm0
0x00000000002050a9 <+601>:	movabs rsi,0x207910
0x00000000002050b3 <+611>:	mov    al,0x1
0x00000000002050b5 <+613>:	call   0x205720 <fprintf@plt>
0x00000000002050ba <+618>:	jmp    0x2050bc <main+620>
0x00000000002050bc <+620>:	xor    edi,edi
0x00000000002050be <+622>:	mov    esi,0xf
0x00000000002050c3 <+627>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x00000000002050c8 <+632>:	movss  xmm0,DWORD PTR [rbx]
0x00000000002050cc <+636>:	mov    rax,QWORD PTR [r14]
0x00000000002050cf <+639>:	movss  DWORD PTR [rax],xmm0
0x00000000002050d3 <+643>:	mov    rdi,QWORD PTR ds:0x2079e0
0x00000000002050db <+651>:	mov    edx,DWORD PTR [r15]
0x00000000002050de <+654>:	movss  xmm0,DWORD PTR [r12]
0x00000000002050e4 <+660>:	cvtss2sd xmm0,xmm0
0x00000000002050e8 <+664>:	movabs rsi,0x207910
0x00000000002050f2 <+674>:	mov    al,0x1
0x00000000002050f4 <+676>:	call   0x205720 <fprintf@plt>
0x00000000002050f9 <+681>:	jmp    0x2050fb <main+683>
0x00000000002050fb <+683>:	xor    edi,edi
0x00000000002050fd <+685>:	mov    esi,0x10
0x0000000000205102 <+690>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000205107 <+695>:	mov    rdi,QWORD PTR ds:0x2079e0
0x000000000020510f <+703>:	mov    edx,DWORD PTR [r15]
0x0000000000205112 <+706>:	mov    rax,QWORD PTR [r13+0x0]
0x0000000000205116 <+710>:	movss  xmm0,DWORD PTR [rax]
0x000000000020511a <+714>:	cvtss2sd xmm0,xmm0
0x000000000020511e <+718>:	movabs rsi,0x207910
0x0000000000205128 <+728>:	mov    al,0x1
0x000000000020512a <+730>:	call   0x205720 <fprintf@plt>
0x000000000020512f <+735>:	jmp    0x205131 <main+737>
0x0000000000205131 <+737>:	xor    edi,edi
0x0000000000205133 <+739>:	mov    esi,0x11
0x0000000000205138 <+744>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x000000000020513d <+749>:	cvtsi2sd xmm0,DWORD PTR [r15]
0x0000000000205142 <+754>:	movsd  xmm1,QWORD PTR [rip+0x27be]        # 0x207908
0x000000000020514a <+762>:	divsd  xmm0,xmm1
0x000000000020514e <+766>:	mov    r13,QWORD PTR [rbp-0x70]
0x0000000000205152 <+770>:	movsd  QWORD PTR [r13+0x0],xmm0
0x0000000000205158 <+776>:	mov    rdi,QWORD PTR ds:0x2079e0
0x0000000000205160 <+784>:	mov    edx,DWORD PTR [r15]
0x0000000000205163 <+787>:	movss  xmm0,DWORD PTR [rbx]
0x0000000000205167 <+791>:	cvtss2sd xmm0,xmm0
0x000000000020516b <+795>:	movabs rsi,0x207910
0x0000000000205175 <+805>:	mov    al,0x1
0x0000000000205177 <+807>:	call   0x205720 <fprintf@plt>
0x000000000020517c <+812>:	jmp    0x20517e <main+814>
0x000000000020517e <+814>:	xor    edi,edi
0x0000000000205180 <+816>:	mov    esi,0x12
0x0000000000205185 <+821>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x000000000020518a <+826>:	movsd  xmm0,QWORD PTR [r13+0x0]
0x0000000000205190 <+832>:	mov    rax,QWORD PTR [rbp-0x40]
0x0000000000205194 <+836>:	mov    rax,QWORD PTR [rax]
0x0000000000205197 <+839>:	movsd  QWORD PTR [rax],xmm0
0x000000000020519b <+843>:	mov    rdi,QWORD PTR ds:0x2079e0
0x00000000002051a3 <+851>:	mov    edx,DWORD PTR [r15]
0x00000000002051a6 <+854>:	mov    rax,QWORD PTR [rbp-0x48]
0x00000000002051aa <+858>:	movsd  xmm0,QWORD PTR [rax]
0x00000000002051ae <+862>:	movabs rsi,0x207910
0x00000000002051b8 <+872>:	mov    al,0x1
0x00000000002051ba <+874>:	call   0x205720 <fprintf@plt>
0x00000000002051bf <+879>:	jmp    0x2051c1 <main+881>
0x00000000002051c1 <+881>:	xor    edi,edi
0x00000000002051c3 <+883>:	mov    esi,0x13
0x00000000002051c8 <+888>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x00000000002051cd <+893>:	mov    eax,DWORD PTR [r15]
0x00000000002051d0 <+896>:	add    eax,0xffffffff
0x00000000002051d3 <+899>:	mov    DWORD PTR [r15],eax
0x00000000002051d6 <+902>:	mov    r13,QWORD PTR [rbp-0x58]
0x00000000002051da <+906>:	jmp    0x20500f <main+447>
0x00000000002051df <+911>:	xor    edi,edi
0x00000000002051e1 <+913>:	mov    esi,0x14
0x00000000002051e6 <+918>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x00000000002051eb <+923>:	mov    rdi,QWORD PTR [rbp-0x30]
0x00000000002051ef <+927>:	call   0x205710 <yk_location_drop@plt>
0x00000000002051f4 <+932>:	jmp    0x2051f6 <main+934>
0x00000000002051f6 <+934>:	xor    edi,edi
0x00000000002051f8 <+936>:	mov    esi,0x15
0x00000000002051fd <+941>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000205202 <+946>:	mov    rdi,QWORD PTR [rbp-0x38]
0x0000000000205206 <+950>:	call   0x2056a0 <yk_mt_shutdown@plt>
0x000000000020520b <+955>:	jmp    0x20520d <main+957>
0x000000000020520d <+957>:	xor    edi,edi
0x000000000020520f <+959>:	mov    esi,0x16
0x0000000000205214 <+964>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000205219 <+969>:	jmp    0x205238 <main+1000>
0x000000000020521b <+971>:	xor    edi,edi
0x000000000020521d <+973>:	mov    esi,0x17
0x0000000000205222 <+978>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000205227 <+983>:	xor    eax,eax
0x0000000000205229 <+985>:	add    rsp,0x48
0x000000000020522d <+989>:	pop    rbx
0x000000000020522e <+990>:	pop    r12
0x0000000000205230 <+992>:	pop    r13
0x0000000000205232 <+994>:	pop    r14
0x0000000000205234 <+996>:	pop    r15
0x0000000000205236 <+998>:	pop    rbp
0x0000000000205237 <+999>:	ret
0x0000000000205238 <+1000>:	xor    edi,edi
0x000000000020523a <+1002>:	mov    esi,0x18
0x000000000020523f <+1007>:	call   0x2056c0 <__yk_trace_basicblock_dummy@plt>
0x0000000000205244 <+1012>:	jmp    0x20521b <main+971>
