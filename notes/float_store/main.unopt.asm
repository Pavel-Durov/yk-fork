──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
>>> disass 0x000000000020545a
Dump of assembler code for function main:
   0x0000000000205250 <+0>:	push   rbp
   0x0000000000205251 <+1>:	mov    rbp,rsp
   0x0000000000205254 <+4>:	push   r15
   0x0000000000205256 <+6>:	push   r14
   0x0000000000205258 <+8>:	push   r13
   0x000000000020525a <+10>:	push   r12
   0x000000000020525c <+12>:	push   rbx
   0x000000000020525d <+13>:	sub    rsp,0x48
   0x0000000000205261 <+17>:	mov    rbx,rsi
   0x0000000000205264 <+20>:	mov    r14d,edi
   0x0000000000205267 <+23>:	mov    edi,0x9
   0x000000000020526c <+28>:	xor    esi,esi
   0x000000000020526e <+30>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205273 <+35>:	mov    rax,0xfffffffffffffff0
   0x000000000020527a <+42>:	mov    rax,QWORD PTR fs:[rax]
   0x000000000020527e <+46>:	mov    r13,rax
   0x0000000000205281 <+49>:	add    r13,0x8
   0x0000000000205285 <+53>:	mov    r15,rax
   0x0000000000205288 <+56>:	add    r15,0x10
   0x000000000020528c <+60>:	mov    r12,rax
   0x000000000020528f <+63>:	add    r12,0x14
   0x0000000000205293 <+67>:	mov    rcx,rax
   0x0000000000205296 <+70>:	add    rcx,0x18
   0x000000000020529a <+74>:	mov    QWORD PTR [rbp-0x68],rcx
   0x000000000020529e <+78>:	mov    rcx,rax
   0x00000000002052a1 <+81>:	add    rcx,0x20
   0x00000000002052a5 <+85>:	mov    QWORD PTR [rbp-0x50],rcx
   0x00000000002052a9 <+89>:	mov    rcx,rax
   0x00000000002052ac <+92>:	add    rcx,0x28
   0x00000000002052b0 <+96>:	mov    QWORD PTR [rbp-0x38],rcx
   0x00000000002052b4 <+100>:	mov    rcx,rax
   0x00000000002052b7 <+103>:	add    rcx,0x30
   0x00000000002052bb <+107>:	mov    QWORD PTR [rbp-0x48],rcx
   0x00000000002052bf <+111>:	mov    rcx,rax
   0x00000000002052c2 <+114>:	add    rcx,0x38
   0x00000000002052c6 <+118>:	mov    QWORD PTR [rbp-0x58],rcx
   0x00000000002052ca <+122>:	mov    rcx,rax
   0x00000000002052cd <+125>:	add    rcx,0x40
   0x00000000002052d1 <+129>:	mov    QWORD PTR [rbp-0x60],rcx
   0x00000000002052d5 <+133>:	mov    DWORD PTR [rax],0x0
   0x00000000002052db <+139>:	mov    DWORD PTR [rax+0x4],r14d
   0x00000000002052df <+143>:	jmp    0x2052e1 <main+145>
   0x00000000002052e1 <+145>:	mov    edi,0x9
   0x00000000002052e6 <+150>:	mov    esi,0x1
   0x00000000002052eb <+155>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x00000000002052f0 <+160>:	mov    QWORD PTR [r13+0x0],rbx
   0x00000000002052f4 <+164>:	xor    edi,edi
   0x00000000002052f6 <+166>:	call   0x2056f0 <yk_mt_new@plt>
   0x00000000002052fb <+171>:	mov    rbx,rax
   0x00000000002052fe <+174>:	jmp    0x205300 <main+176>
   0x0000000000205300 <+176>:	mov    edi,0x9
   0x0000000000205305 <+181>:	mov    esi,0x2
   0x000000000020530a <+186>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x000000000020530f <+191>:	mov    QWORD PTR [rbp-0x40],rbx
   0x0000000000205313 <+195>:	mov    rdi,QWORD PTR [rbp-0x40]
   0x0000000000205317 <+199>:	xor    esi,esi
   0x0000000000205319 <+201>:	call   0x205700 <yk_mt_hot_threshold_set@plt>
   0x000000000020531e <+206>:	mov    r13,QWORD PTR [rbp-0x50]
   0x0000000000205322 <+210>:	mov    r14,QWORD PTR [rbp-0x58]
   0x0000000000205326 <+214>:	jmp    0x205328 <main+216>
   0x0000000000205328 <+216>:	mov    edi,0x9
   0x000000000020532d <+221>:	mov    esi,0x3
   0x0000000000205332 <+226>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205337 <+231>:	call   0x205710 <yk_location_new@plt>
   0x000000000020533c <+236>:	mov    rbx,rax
   0x000000000020533f <+239>:	jmp    0x205341 <main+241>
   0x0000000000205341 <+241>:	mov    edi,0x9
   0x0000000000205346 <+246>:	mov    esi,0x4
   0x000000000020534b <+251>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205350 <+256>:	mov    QWORD PTR [rbp-0x30],rbx
   0x0000000000205354 <+260>:	mov    DWORD PTR [r15],0x4
   0x000000000020535b <+267>:	mov    rbx,QWORD PTR [rbp-0x68]
   0x000000000020535f <+271>:	mov    QWORD PTR [rbx],r12
   0x0000000000205362 <+274>:	mov    QWORD PTR [r13+0x0],r12
   0x0000000000205366 <+278>:	mov    rax,QWORD PTR [rbp-0x48]
   0x000000000020536a <+282>:	mov    rcx,QWORD PTR [rbp-0x38]
   0x000000000020536e <+286>:	mov    QWORD PTR [rax],rcx
   0x0000000000205371 <+289>:	mov    rax,QWORD PTR [rbp-0x30]
   0x0000000000205375 <+293>:	jmp    0x205377 <main+295>
   0x0000000000205377 <+295>:	mov    edi,0x9
   0x000000000020537c <+300>:	mov    esi,0x5
   0x0000000000205381 <+305>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205386 <+310>:	mov    eax,DWORD PTR [r15]
   0x0000000000205389 <+313>:	jmp    0x20538b <main+315>
   0x000000000020538b <+315>:	mov    edi,0x9
   0x0000000000205390 <+320>:	mov    esi,0x6
   0x0000000000205395 <+325>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x000000000020539a <+330>:	movss  xmm0,DWORD PTR [r12]
   0x00000000002053a0 <+336>:	jmp    0x2053a2 <main+338>
   0x00000000002053a2 <+338>:	mov    edi,0x9
   0x00000000002053a7 <+343>:	mov    esi,0x7
   0x00000000002053ac <+348>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x00000000002053b1 <+353>:	mov    rax,QWORD PTR [rbx]
   0x00000000002053b4 <+356>:	jmp    0x2053b6 <main+358>
   0x00000000002053b6 <+358>:	mov    edi,0x9
   0x00000000002053bb <+363>:	mov    esi,0x8
   0x00000000002053c0 <+368>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x00000000002053c5 <+373>:	mov    rax,QWORD PTR [r13+0x0]
   0x00000000002053c9 <+377>:	jmp    0x2053cb <main+379>
   0x00000000002053cb <+379>:	mov    edi,0x9
   0x00000000002053d0 <+384>:	mov    esi,0x9
   0x00000000002053d5 <+389>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x00000000002053da <+394>:	mov    rax,QWORD PTR [rbp-0x38]
   0x00000000002053de <+398>:	movsd  xmm0,QWORD PTR [rax]
   0x00000000002053e2 <+402>:	jmp    0x2053e4 <main+404>
   0x00000000002053e4 <+404>:	mov    edi,0x9
   0x00000000002053e9 <+409>:	mov    esi,0xa
   0x00000000002053ee <+414>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x00000000002053f3 <+419>:	mov    rcx,QWORD PTR [rbp-0x48]
   0x00000000002053f7 <+423>:	mov    rax,QWORD PTR [rcx]
   0x00000000002053fa <+426>:	jmp    0x2053fc <main+428>
   0x00000000002053fc <+428>:	mov    edi,0x9
   0x0000000000205401 <+433>:	mov    esi,0xb
   0x0000000000205406 <+438>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x000000000020540b <+443>:	jmp    0x20540d <main+445>
   0x000000000020540d <+445>:	mov    edi,0x9
   0x0000000000205412 <+450>:	mov    esi,0xc
   0x0000000000205417 <+455>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x000000000020541c <+460>:	cmp    DWORD PTR [r15],0x0
   0x0000000000205420 <+464>:	setg   al
   0x0000000000205423 <+467>:	mov    rcx,QWORD PTR [rbp-0x48]
   0x0000000000205427 <+471>:	mov    rdx,QWORD PTR [rbp-0x60]
   0x000000000020542b <+475>:	mov    rsi,QWORD PTR [rbp-0x38]
   0x000000000020542f <+479>:	test   al,0x1
   0x0000000000205431 <+481>:	jne    0x205438 <main+488>
   0x0000000000205433 <+483>:	jmp    0x2055f1 <main+929>
   0x0000000000205438 <+488>:	mov    edi,0x9
   0x000000000020543d <+493>:	mov    esi,0xd
   0x0000000000205442 <+498>:	mov    r14,rcx
   0x0000000000205445 <+501>:	mov    r13,rdx
   0x0000000000205448 <+504>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x000000000020544d <+509>:	mov    rdi,QWORD PTR [rbp-0x40]
   0x0000000000205451 <+513>:	lea    rsi,[rbp-0x30]
   0x0000000000205455 <+517>:	mov    edx,0x1
   0x000000000020545a <+522>:	movabs r11,0x2056e0
   0x0000000000205464 <+532>:	call   r11
=> 0x0000000000205467 <+535>:	mov    r13,QWORD PTR [rbp-0x50]
   0x000000000020546b <+539>:	mov    r14,QWORD PTR [rbp-0x58]
   0x000000000020546f <+543>:	jmp    0x205471 <main+545>
   0x0000000000205471 <+545>:	mov    edi,0x9
   0x0000000000205476 <+550>:	mov    esi,0xe
   0x000000000020547b <+555>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205480 <+560>:	cvtsi2ss xmm0,DWORD PTR [r15]
   0x0000000000205485 <+565>:	movss  xmm1,DWORD PTR [rip+0x2473]        # 0x207900
   0x000000000020548d <+573>:	divss  xmm0,xmm1
   0x0000000000205491 <+577>:	movss  DWORD PTR [r14],xmm0
   0x0000000000205496 <+582>:	mov    rdi,QWORD PTR ds:0x2079e0
   0x000000000020549e <+590>:	mov    edx,DWORD PTR [r15]
   0x00000000002054a1 <+593>:	movss  xmm0,DWORD PTR [r14]
   0x00000000002054a6 <+598>:	cvtss2sd xmm0,xmm0
   0x00000000002054aa <+602>:	movabs rsi,0x207910
   0x00000000002054b4 <+612>:	mov    al,0x1
   0x00000000002054b6 <+614>:	call   0x205720 <fprintf@plt>
   0x00000000002054bb <+619>:	jmp    0x2054bd <main+621>
   0x00000000002054bd <+621>:	mov    edi,0x9
   0x00000000002054c2 <+626>:	mov    esi,0xf
   0x00000000002054c7 <+631>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x00000000002054cc <+636>:	movss  xmm0,DWORD PTR [r14]
   0x00000000002054d1 <+641>:	mov    rax,QWORD PTR [rbx]
   0x00000000002054d4 <+644>:	movss  DWORD PTR [rax],xmm0
   0x00000000002054d8 <+648>:	mov    rdi,QWORD PTR ds:0x2079e0
   0x00000000002054e0 <+656>:	mov    edx,DWORD PTR [r15]
   0x00000000002054e3 <+659>:	movss  xmm0,DWORD PTR [r12]
   0x00000000002054e9 <+665>:	cvtss2sd xmm0,xmm0
   0x00000000002054ed <+669>:	movabs rsi,0x207910
   0x00000000002054f7 <+679>:	mov    al,0x1
   0x00000000002054f9 <+681>:	call   0x205720 <fprintf@plt>
   0x00000000002054fe <+686>:	jmp    0x205500 <main+688>
   0x0000000000205500 <+688>:	mov    edi,0x9
   0x0000000000205505 <+693>:	mov    esi,0x10
   0x000000000020550a <+698>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x000000000020550f <+703>:	mov    rdi,QWORD PTR ds:0x2079e0
   0x0000000000205517 <+711>:	mov    edx,DWORD PTR [r15]
   0x000000000020551a <+714>:	mov    rax,QWORD PTR [r13+0x0]
   0x000000000020551e <+718>:	movss  xmm0,DWORD PTR [rax]
   0x0000000000205522 <+722>:	cvtss2sd xmm0,xmm0
   0x0000000000205526 <+726>:	movabs rsi,0x207910
   0x0000000000205530 <+736>:	mov    al,0x1
   0x0000000000205532 <+738>:	call   0x205720 <fprintf@plt>
   0x0000000000205537 <+743>:	jmp    0x205539 <main+745>
   0x0000000000205539 <+745>:	mov    edi,0x9
   0x000000000020553e <+750>:	mov    esi,0x11
   0x0000000000205543 <+755>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205548 <+760>:	cvtsi2sd xmm0,DWORD PTR [r15]
   0x000000000020554d <+765>:	movsd  xmm1,QWORD PTR [rip+0x23b3]        # 0x207908
   0x0000000000205555 <+773>:	divsd  xmm0,xmm1
   0x0000000000205559 <+777>:	mov    r13,QWORD PTR [rbp-0x60]
   0x000000000020555d <+781>:	movsd  QWORD PTR [r13+0x0],xmm0
   0x0000000000205563 <+787>:	mov    rdi,QWORD PTR ds:0x2079e0
   0x000000000020556b <+795>:	mov    edx,DWORD PTR [r15]
   0x000000000020556e <+798>:	movss  xmm0,DWORD PTR [r14]
   0x0000000000205573 <+803>:	cvtss2sd xmm0,xmm0
   0x0000000000205577 <+807>:	movabs rsi,0x207910
   0x0000000000205581 <+817>:	mov    al,0x1
   0x0000000000205583 <+819>:	call   0x205720 <fprintf@plt>
   0x0000000000205588 <+824>:	jmp    0x20558a <main+826>
   0x000000000020558a <+826>:	mov    edi,0x9
   0x000000000020558f <+831>:	mov    esi,0x12
   0x0000000000205594 <+836>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205599 <+841>:	movsd  xmm0,QWORD PTR [r13+0x0]
   0x000000000020559f <+847>:	mov    rax,QWORD PTR [rbp-0x48]
   0x00000000002055a3 <+851>:	mov    rax,QWORD PTR [rax]
   0x00000000002055a6 <+854>:	movsd  QWORD PTR [rax],xmm0
   0x00000000002055aa <+858>:	mov    rdi,QWORD PTR ds:0x2079e0
   0x00000000002055b2 <+866>:	mov    edx,DWORD PTR [r15]
   0x00000000002055b5 <+869>:	mov    rax,QWORD PTR [rbp-0x38]
   0x00000000002055b9 <+873>:	movsd  xmm0,QWORD PTR [rax]
   0x00000000002055bd <+877>:	movabs rsi,0x207910
   0x00000000002055c7 <+887>:	mov    al,0x1
   0x00000000002055c9 <+889>:	call   0x205720 <fprintf@plt>
   0x00000000002055ce <+894>:	jmp    0x2055d0 <main+896>
   0x00000000002055d0 <+896>:	mov    edi,0x9
   0x00000000002055d5 <+901>:	mov    esi,0x13
   0x00000000002055da <+906>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x00000000002055df <+911>:	mov    eax,DWORD PTR [r15]
   0x00000000002055e2 <+914>:	add    eax,0xffffffff
   0x00000000002055e5 <+917>:	mov    DWORD PTR [r15],eax
   0x00000000002055e8 <+920>:	mov    r13,QWORD PTR [rbp-0x50]
   0x00000000002055ec <+924>:	jmp    0x20540d <main+445>
   0x00000000002055f1 <+929>:	mov    edi,0x9
   0x00000000002055f6 <+934>:	mov    esi,0x14
   0x00000000002055fb <+939>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205600 <+944>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x0000000000205604 <+948>:	call   0x2056c0 <yk_location_drop@plt>
   0x0000000000205609 <+953>:	jmp    0x20560b <main+955>
   0x000000000020560b <+955>:	mov    edi,0x9
   0x0000000000205610 <+960>:	mov    esi,0x15
   0x0000000000205615 <+965>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x000000000020561a <+970>:	mov    rdi,QWORD PTR [rbp-0x40]
   0x000000000020561e <+974>:	call   0x2056a0 <yk_mt_shutdown@plt>
   0x0000000000205623 <+979>:	jmp    0x205625 <main+981>
   0x0000000000205625 <+981>:	mov    edi,0x9
   0x000000000020562a <+986>:	mov    esi,0x16
   0x000000000020562f <+991>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205634 <+996>:	jmp    0x205656 <main+1030>
   0x0000000000205636 <+998>:	mov    edi,0x9
   0x000000000020563b <+1003>:	mov    esi,0x17
   0x0000000000205640 <+1008>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205645 <+1013>:	xor    eax,eax
   0x0000000000205647 <+1015>:	add    rsp,0x48
   0x000000000020564b <+1019>:	pop    rbx
   0x000000000020564c <+1020>:	pop    r12
   0x000000000020564e <+1022>:	pop    r13
   0x0000000000205650 <+1024>:	pop    r14
   0x0000000000205652 <+1026>:	pop    r15
   0x0000000000205654 <+1028>:	pop    rbp
   0x0000000000205655 <+1029>:	ret
   0x0000000000205656 <+1030>:	mov    edi,0x9
   0x000000000020565b <+1035>:	mov    esi,0x18
   0x0000000000205660 <+1040>:	call   0x2056d0 <__yk_trace_basicblock@plt>
   0x0000000000205665 <+1045>:	jmp    0x205636 <main+998>
