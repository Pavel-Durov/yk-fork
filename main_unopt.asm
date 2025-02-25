0x0000000000205210 <+0>:	push   rbp
   0x0000000000205211 <+1>:	mov    rbp,rsp
   0x0000000000205214 <+4>:	push   r15
   0x0000000000205216 <+6>:	push   r14
   0x0000000000205218 <+8>:	push   r13
   0x000000000020521a <+10>:	push   r12
   0x000000000020521c <+12>:	push   rbx
   0x000000000020521d <+13>:	sub    rsp,0x58
   0x0000000000205221 <+17>:	mov    r12,rsi
   0x0000000000205224 <+20>:	mov    r14d,edi
   0x0000000000205227 <+23>:	mov    edi,0xb
   0x000000000020522c <+28>:	xor    esi,esi
   0x000000000020522e <+30>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205233 <+35>:	lea    rax,[rip+0x264e]        # 0x207888 <shadowstack_0>
   0x000000000020523a <+42>:	mov    rax,QWORD PTR [rax]
   0x000000000020523d <+45>:	mov    rbx,rax
   0x0000000000205240 <+48>:	add    rbx,0x8
   0x0000000000205244 <+52>:	mov    r15,rax
   0x0000000000205247 <+55>:	add    r15,0x10
   0x000000000020524b <+59>:	mov    rcx,rax
   0x000000000020524e <+62>:	add    rcx,0x14
   0x0000000000205252 <+66>:	mov    QWORD PTR [rbp-0x58],rcx
   0x0000000000205256 <+70>:	mov    rcx,rax
   0x0000000000205259 <+73>:	add    rcx,0x18
   0x000000000020525d <+77>:	mov    QWORD PTR [rbp-0x50],rcx
   0x0000000000205261 <+81>:	mov    rcx,rax
   0x0000000000205264 <+84>:	add    rcx,0x1c
   0x0000000000205268 <+88>:	mov    QWORD PTR [rbp-0x48],rcx
   0x000000000020526c <+92>:	mov    rcx,rax
   0x000000000020526f <+95>:	add    rcx,0x20
   0x0000000000205273 <+99>:	mov    QWORD PTR [rbp-0x68],rcx
   0x0000000000205277 <+103>:	mov    rcx,rax
   0x000000000020527a <+106>:	add    rcx,0x24
   0x000000000020527e <+110>:	mov    QWORD PTR [rbp-0x78],rcx
   0x0000000000205282 <+114>:	mov    rcx,rax
   0x0000000000205285 <+117>:	add    rcx,0x28
   0x0000000000205289 <+121>:	mov    QWORD PTR [rbp-0x70],rcx
   0x000000000020528d <+125>:	mov    rcx,rax
   0x0000000000205290 <+128>:	add    rcx,0x2c
   0x0000000000205294 <+132>:	mov    QWORD PTR [rbp-0x60],rcx
   0x0000000000205298 <+136>:	mov    rcx,rax
   0x000000000020529b <+139>:	add    rcx,0x30
   0x000000000020529f <+143>:	mov    QWORD PTR [rbp-0x40],rcx
   0x00000000002052a3 <+147>:	mov    DWORD PTR [rax],0x0
   0x00000000002052a9 <+153>:	mov    DWORD PTR [rax+0x4],r14d
   0x00000000002052ad <+157>:	jmp    0x2052af <main+159>
   0x00000000002052af <+159>:	mov    edi,0xb
   0x00000000002052b4 <+164>:	mov    esi,0x1
   0x00000000002052b9 <+169>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002052be <+174>:	mov    QWORD PTR [rbx],r12
   0x00000000002052c1 <+177>:	xor    edi,edi
   0x00000000002052c3 <+179>:	call   0x2055e0 <yk_mt_new@plt>
   0x00000000002052c8 <+184>:	mov    rbx,rax
   0x00000000002052cb <+187>:	jmp    0x2052cd <main+189>
   0x00000000002052cd <+189>:	mov    edi,0xb
   0x00000000002052d2 <+194>:	mov    esi,0x2
   0x00000000002052d7 <+199>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002052dc <+204>:	mov    QWORD PTR [rbp-0x38],rbx
   0x00000000002052e0 <+208>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x00000000002052e4 <+212>:	xor    esi,esi
   0x00000000002052e6 <+214>:	call   0x2055f0 <yk_mt_hot_threshold_set@plt>
   0x00000000002052eb <+219>:	jmp    0x2052ed <main+221>
   0x00000000002052ed <+221>:	mov    edi,0xb
   0x00000000002052f2 <+226>:	mov    esi,0x3
   0x00000000002052f7 <+231>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002052fc <+236>:	call   0x205600 <yk_location_new@plt>
   0x0000000000205301 <+241>:	mov    rbx,rax
   0x0000000000205304 <+244>:	jmp    0x205306 <main+246>
   0x0000000000205306 <+246>:	mov    edi,0xb
   0x000000000020530b <+251>:	mov    esi,0x4
   0x0000000000205310 <+256>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205315 <+261>:	mov    QWORD PTR [rbp-0x30],rbx
   0x0000000000205319 <+265>:	mov    DWORD PTR [r15],0x4
   0x0000000000205320 <+272>:	mov    rax,QWORD PTR [rbp-0x30]
   0x0000000000205324 <+276>:	jmp    0x205326 <main+278>
   0x0000000000205326 <+278>:	mov    edi,0xb
   0x000000000020532b <+283>:	mov    esi,0x5
   0x0000000000205330 <+288>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205335 <+293>:	mov    eax,DWORD PTR [r15]
   0x0000000000205338 <+296>:	jmp    0x20533a <main+298>
   0x000000000020533a <+298>:	mov    edi,0xb
   0x000000000020533f <+303>:	mov    esi,0x6
   0x0000000000205344 <+308>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205349 <+313>:	jmp    0x20534b <main+315>
   0x000000000020534b <+315>:	mov    edi,0xb
   0x0000000000205350 <+320>:	mov    esi,0x7
   0x0000000000205355 <+325>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x000000000020535a <+330>:	cmp    DWORD PTR [r15],0x0
   0x000000000020535e <+334>:	setg   al
   0x0000000000205361 <+337>:	mov    r14,QWORD PTR [rbp-0x48]
   0x0000000000205365 <+341>:	mov    r12,QWORD PTR [rbp-0x68]
   0x0000000000205369 <+345>:	mov    r13,QWORD PTR [rbp-0x78]
   0x000000000020536d <+349>:	mov    rbx,QWORD PTR [rbp-0x70]
   0x0000000000205371 <+353>:	mov    rcx,QWORD PTR [rbp-0x60]
   0x0000000000205375 <+357>:	mov    rdx,QWORD PTR [rbp-0x40]
   0x0000000000205379 <+361>:	mov    rsi,QWORD PTR [rbp-0x50]
   0x000000000020537d <+365>:	mov    rdi,QWORD PTR [rbp-0x58]
   0x0000000000205381 <+369>:	test   al,0x1
   0x0000000000205383 <+371>:	jne    0x20538a <main+378>
   0x0000000000205385 <+373>:	jmp    0x2054b4 <main+676>
   0x000000000020538a <+378>:	mov    edi,0xb
   0x000000000020538f <+383>:	mov    esi,0x8
   0x0000000000205394 <+388>:	mov    r14,r13
   0x0000000000205397 <+391>:	mov    r12,rbx
   0x000000000020539a <+394>:	mov    r13,rcx
   0x000000000020539d <+397>:	mov    rbx,r15
   0x00000000002053a0 <+400>:	mov    r15,rdx
   0x00000000002053a3 <+403>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002053a8 <+408>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x00000000002053ac <+412>:	lea    rsi,[rbp-0x30]
   0x00000000002053b0 <+416>:	mov    edx,0x1
   0x00000000002053b5 <+421>:	mov    rcx,r13
   0x00000000002053b8 <+424>:	mov    r13,QWORD PTR [rbp-0x68]
   0x00000000002053bc <+428>:	movabs r11,0x2055d0
   0x00000000002053c6 <+438>:	call   r11
=> 0x00000000002053c9 <+441>:	mov    r15,rbxs
   0x00000000002053cc <+444>:	jmp    0x2053ce <main+446>
   0x00000000002053ce <+446>:	mov    edi,0xb
   0x00000000002053d3 <+451>:	mov    esi,0x9
   0x00000000002053d8 <+456>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002053dd <+461>:	mov    eax,DWORD PTR [r15]
   0x00000000002053e0 <+464>:	and    eax,0x1
   0x00000000002053e3 <+467>:	mov    rdx,QWORD PTR [rbp-0x58]
   0x00000000002053e7 <+471>:	mov    DWORD PTR [rdx],eax
   0x00000000002053e9 <+473>:	mov    eax,DWORD PTR [r15]
   0x00000000002053ec <+476>:	or     eax,0x1
   0x00000000002053ef <+479>:	mov    r8,QWORD PTR [rbp-0x50]
   0x00000000002053f3 <+483>:	mov    DWORD PTR [r8],eax
   0x00000000002053f6 <+486>:	mov    eax,DWORD PTR [r15]
   0x00000000002053f9 <+489>:	shr    eax,1
   0x00000000002053fb <+491>:	mov    rsi,QWORD PTR [rbp-0x48]
   0x00000000002053ff <+495>:	mov    DWORD PTR [rsi],eax
   0x0000000000205401 <+497>:	mov    eax,DWORD PTR [r15]
   0x0000000000205404 <+500>:	sar    eax,1
   0x0000000000205406 <+502>:	mov    DWORD PTR [r13+0x0],eax
   0x000000000020540a <+506>:	xor    eax,eax
   0x000000000020540c <+508>:	sub    eax,DWORD PTR [r15]
   0x000000000020540f <+511>:	sar    eax,1
   0x0000000000205411 <+513>:	mov    DWORD PTR [r14],eax
   0x0000000000205414 <+516>:	mov    eax,DWORD PTR [r15]
   0x0000000000205417 <+519>:	xor    eax,0x1
   0x000000000020541a <+522>:	mov    DWORD PTR [r12],eax
   0x000000000020541e <+526>:	mov    eax,DWORD PTR [r15]
   0x0000000000205421 <+529>:	xor    eax,0xffffffff
   0x0000000000205424 <+532>:	mov    rbx,QWORD PTR [rbp-0x60]
   0x0000000000205428 <+536>:	mov    DWORD PTR [rbx],eax
   0x000000000020542a <+538>:	mov    eax,DWORD PTR [r15]
   0x000000000020542d <+541>:	shl    eax,1
   0x000000000020542f <+543>:	mov    rcx,QWORD PTR [rbp-0x40]
   0x0000000000205433 <+547>:	mov    DWORD PTR [rcx],eax
   0x0000000000205435 <+549>:	mov    rdi,QWORD PTR ds:0x2078a0
   0x000000000020543d <+557>:	mov    edx,DWORD PTR [rdx]
   0x000000000020543f <+559>:	mov    ecx,DWORD PTR [r8]
   0x0000000000205442 <+562>:	mov    r8d,DWORD PTR [rsi]
   0x0000000000205445 <+565>:	mov    r9d,DWORD PTR [r13+0x0]
   0x0000000000205449 <+569>:	movabs rsi,0x204714
   0x0000000000205453 <+579>:	mov    al,0x0
   0x0000000000205455 <+581>:	call   0x205610 <fprintf@plt>
   0x000000000020545a <+586>:	jmp    0x20545c <main+588>
   0x000000000020545c <+588>:	mov    edi,0xb
   0x0000000000205461 <+593>:	mov    esi,0xa
   0x0000000000205466 <+598>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x000000000020546b <+603>:	mov    rdi,QWORD PTR ds:0x2078a0
   0x0000000000205473 <+611>:	mov    edx,DWORD PTR [r14]
   0x0000000000205476 <+614>:	mov    ecx,DWORD PTR [r12]
   0x000000000020547a <+618>:	mov    r8d,DWORD PTR [rbx]
   0x000000000020547d <+621>:	mov    rax,QWORD PTR [rbp-0x40]
   0x0000000000205481 <+625>:	mov    r9d,DWORD PTR [rax]
   0x0000000000205484 <+628>:	movabs rsi,0x204738
   0x000000000020548e <+638>:	mov    al,0x0
   0x0000000000205490 <+640>:	call   0x205610 <fprintf@plt>
   0x0000000000205495 <+645>:	jmp    0x205497 <main+647>
   0x0000000000205497 <+647>:	mov    edi,0xb
   0x000000000020549c <+652>:	mov    esi,0xb
   0x00000000002054a1 <+657>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002054a6 <+662>:	mov    eax,DWORD PTR [r15]
   0x00000000002054a9 <+665>:	add    eax,0xffffffff
   0x00000000002054ac <+668>:	mov    DWORD PTR [r15],eax
   0x00000000002054af <+671>:	jmp    0x20534b <main+315>
   0x00000000002054b4 <+676>:	mov    edi,0xb
   0x00000000002054b9 <+681>:	mov    esi,0xc
   0x00000000002054be <+686>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002054c3 <+691>:	mov    rdi,QWORD PTR ds:0x2078a0
   0x00000000002054cb <+699>:	movabs rsi,0x204732
   0x00000000002054d5 <+709>:	mov    al,0x0
   0x00000000002054d7 <+711>:	call   0x205610 <fprintf@plt>
   0x00000000002054dc <+716>:	jmp    0x2054de <main+718>
   0x00000000002054de <+718>:	mov    edi,0xb
   0x00000000002054e3 <+723>:	mov    esi,0xd
   0x00000000002054e8 <+728>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x00000000002054ed <+733>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x00000000002054f1 <+737>:	call   0x2055b0 <yk_location_drop@plt>
   0x00000000002054f6 <+742>:	jmp    0x2054f8 <main+744>
   0x00000000002054f8 <+744>:	mov    edi,0xb
   0x00000000002054fd <+749>:	mov    esi,0xe
   0x0000000000205502 <+754>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205507 <+759>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x000000000020550b <+763>:	call   0x205590 <yk_mt_shutdown@plt>
   0x0000000000205510 <+768>:	jmp    0x205512 <main+770>
   0x0000000000205512 <+770>:	mov    edi,0xb
   0x0000000000205517 <+775>:	mov    esi,0xf
   0x000000000020551c <+780>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205521 <+785>:	jmp    0x205543 <main+819>
   0x0000000000205523 <+787>:	mov    edi,0xb
   0x0000000000205528 <+792>:	mov    esi,0x10
   0x000000000020552d <+797>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205532 <+802>:	xor    eax,eax
   0x0000000000205534 <+804>:	add    rsp,0x58
   0x0000000000205538 <+808>:	pop    rbx
   0x0000000000205539 <+809>:	pop    r12
   0x000000000020553b <+811>:	pop    r13
   0x000000000020553d <+813>:	pop    r14
   0x000000000020553f <+815>:	pop    r15
   0x0000000000205541 <+817>:	pop    rbp
   0x0000000000205542 <+818>:	ret
   0x0000000000205543 <+819>:	mov    edi,0xb
   0x0000000000205548 <+824>:	mov    esi,0x11
   0x000000000020554d <+829>:	call   0x2055c0 <__yk_trace_basicblock@plt>
   0x0000000000205552 <+834>:	jmp    0x205523 <main+787>
