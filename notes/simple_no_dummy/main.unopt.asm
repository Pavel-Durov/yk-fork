Dump of assembler code for function main:
   0x0000000000203150 <+0>:	push   rbp
   0x0000000000203151 <+1>:	mov    rbp,rsp
   0x0000000000203154 <+4>:	push   r15
   0x0000000000203156 <+6>:	push   r14
   0x0000000000203158 <+8>:	push   r13
   0x000000000020315a <+10>:	push   r12
   0x000000000020315c <+12>:	push   rbx
   0x000000000020315d <+13>:	sub    rsp,0x18
   0x0000000000203161 <+17>:	mov    rbx,rsi
   0x0000000000203164 <+20>:	mov    r14d,edi
   0x0000000000203167 <+23>:	mov    edi,0x9
   0x000000000020316c <+28>:	xor    esi,esi
   0x000000000020316e <+30>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203173 <+35>:	mov    rax,0xfffffffffffffff0
   0x000000000020317a <+42>:	mov    rax,QWORD PTR fs:[rax]
   0x000000000020317e <+46>:	mov    r13,rax
   0x0000000000203181 <+49>:	add    r13,0x8
   0x0000000000203185 <+53>:	mov    r15,rax
   0x0000000000203188 <+56>:	add    r15,0x10
   0x000000000020318c <+60>:	mov    r12,rax
   0x000000000020318f <+63>:	add    r12,0x14
   0x0000000000203193 <+67>:	mov    DWORD PTR [rax],0x0
   0x0000000000203199 <+73>:	mov    DWORD PTR [rax+0x4],r14d
   0x000000000020319d <+77>:	jmp    0x20319f <main+79>
   0x000000000020319f <+79>:	mov    edi,0x9
   0x00000000002031a4 <+84>:	mov    esi,0x1
   0x00000000002031a9 <+89>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x00000000002031ae <+94>:	mov    QWORD PTR [r13+0x0],rbx
   0x00000000002031b2 <+98>:	xor    edi,edi
   0x00000000002031b4 <+100>:	call   0x203420 <yk_mt_new@plt>
   0x00000000002031b9 <+105>:	mov    rbx,rax
   0x00000000002031bc <+108>:	jmp    0x2031be <main+110>
   0x00000000002031be <+110>:	mov    edi,0x9
   0x00000000002031c3 <+115>:	mov    esi,0x2
   0x00000000002031c8 <+120>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x00000000002031cd <+125>:	mov    QWORD PTR [rbp-0x38],rbx
   0x00000000002031d1 <+129>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x00000000002031d5 <+133>:	xor    esi,esi
   0x00000000002031d7 <+135>:	call   0x203430 <yk_mt_hot_threshold_set@plt>
   0x00000000002031dc <+140>:	jmp    0x2031de <main+142>
   0x00000000002031de <+142>:	mov    edi,0x9
   0x00000000002031e3 <+147>:	mov    esi,0x3
   0x00000000002031e8 <+152>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x00000000002031ed <+157>:	call   0x203440 <yk_location_new@plt>
   0x00000000002031f2 <+162>:	mov    rbx,rax
   0x00000000002031f5 <+165>:	jmp    0x2031f7 <main+167>
   0x00000000002031f7 <+167>:	mov    edi,0x9
   0x00000000002031fc <+172>:	mov    esi,0x4
   0x0000000000203201 <+177>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203206 <+182>:	mov    QWORD PTR [rbp-0x30],rbx
   0x000000000020320a <+186>:	mov    DWORD PTR [r15],0x270e
   0x0000000000203211 <+193>:	mov    DWORD PTR [r12],0x4
   0x0000000000203219 <+201>:	mov    rax,QWORD PTR [rbp-0x30]
   0x000000000020321d <+205>:	jmp    0x20321f <main+207>
   0x000000000020321f <+207>:	mov    edi,0x9
   0x0000000000203224 <+212>:	mov    esi,0x5
   0x0000000000203229 <+217>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x000000000020322e <+222>:	mov    eax,DWORD PTR [r15]
   0x0000000000203231 <+225>:	jmp    0x203233 <main+227>
   0x0000000000203233 <+227>:	mov    edi,0x9
   0x0000000000203238 <+232>:	mov    esi,0x6
   0x000000000020323d <+237>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203242 <+242>:	mov    eax,DWORD PTR [r12]
   0x0000000000203246 <+246>:	jmp    0x203248 <main+248>
   0x0000000000203248 <+248>:	mov    edi,0x9
   0x000000000020324d <+253>:	mov    esi,0x7
   0x0000000000203252 <+258>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203257 <+263>:	jmp    0x203259 <main+265>
   0x0000000000203259 <+265>:	mov    edi,0x9
   0x000000000020325e <+270>:	mov    esi,0x8
   0x0000000000203263 <+275>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203268 <+280>:	cmp    DWORD PTR [r12],0x0
   0x000000000020326d <+285>:	setg   al
   0x0000000000203270 <+288>:	test   al,0x1
   0x0000000000203272 <+290>:	jne    0x203276 <main+294>
   0x0000000000203274 <+292>:	jmp    0x2032ee <main+414>
   0x0000000000203276 <+294>:	mov    edi,0x9
   0x000000000020327b <+299>:	mov    esi,0x9
   0x0000000000203280 <+304>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203285 <+309>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x0000000000203289 <+313>:	lea    rsi,[rbp-0x30]
   0x000000000020328d <+317>:	mov    edx,0x1
   0x0000000000203292 <+322>:	movabs r11,0x203410
   0x000000000020329c <+332>:	call   r11
=> 0x000000000020329f <+335>:	jmp    0x2032a1 <main+337>
   0x00000000002032a1 <+337>:	mov    edi,0x9
   0x00000000002032a6 <+342>:	mov    esi,0xa
   0x00000000002032ab <+347>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x00000000002032b0 <+352>:	mov    rdi,QWORD PTR ds:0x205720
   0x00000000002032b8 <+360>:	mov    edx,DWORD PTR [r12]
   0x00000000002032bc <+364>:	movabs rsi,0x20563a
   0x00000000002032c6 <+374>:	mov    al,0x0
   0x00000000002032c8 <+376>:	call   0x203450 <fprintf@plt>
   0x00000000002032cd <+381>:	jmp    0x2032cf <main+383>
   0x00000000002032cf <+383>:	mov    edi,0x9
   0x00000000002032d4 <+388>:	mov    esi,0xb
   0x00000000002032d9 <+393>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x00000000002032de <+398>:	mov    eax,DWORD PTR [r12]
   0x00000000002032e2 <+402>:	add    eax,0xffffffff
   0x00000000002032e5 <+405>:	mov    DWORD PTR [r12],eax
   0x00000000002032e9 <+409>:	jmp    0x203259 <main+265>
   0x00000000002032ee <+414>:	mov    edi,0x9
   0x00000000002032f3 <+419>:	mov    esi,0xc
   0x00000000002032f8 <+424>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x00000000002032fd <+429>:	mov    rdi,QWORD PTR ds:0x205720
   0x0000000000203305 <+437>:	movabs rsi,0x205634
   0x000000000020330f <+447>:	mov    al,0x0
   0x0000000000203311 <+449>:	call   0x203450 <fprintf@plt>
   0x0000000000203316 <+454>:	jmp    0x203318 <main+456>
   0x0000000000203318 <+456>:	mov    edi,0x9
   0x000000000020331d <+461>:	mov    esi,0xd
   0x0000000000203322 <+466>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203327 <+471>:	mov    eax,DWORD PTR [r15]
   0x000000000020332a <+474>:	jmp    0x20332c <main+476>
   0x000000000020332c <+476>:	mov    edi,0x9
   0x0000000000203331 <+481>:	mov    esi,0xe
   0x0000000000203336 <+486>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x000000000020333b <+491>:	mov    rdi,QWORD PTR [rbp-0x30]
   0x000000000020333f <+495>:	call   0x2033f0 <yk_location_drop@plt>
   0x0000000000203344 <+500>:	jmp    0x203346 <main+502>
   0x0000000000203346 <+502>:	mov    edi,0x9
   0x000000000020334b <+507>:	mov    esi,0xf
   0x0000000000203350 <+512>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203355 <+517>:	mov    rdi,QWORD PTR [rbp-0x38]
   0x0000000000203359 <+521>:	call   0x2033e0 <yk_mt_shutdown@plt>
   0x000000000020335e <+526>:	jmp    0x203360 <main+528>
   0x0000000000203360 <+528>:	mov    edi,0x9
   0x0000000000203365 <+533>:	mov    esi,0x10
   0x000000000020336a <+538>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x000000000020336f <+543>:	jmp    0x203391 <main+577>
   0x0000000000203371 <+545>:	mov    edi,0x9
   0x0000000000203376 <+550>:	mov    esi,0x11
   0x000000000020337b <+555>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x0000000000203380 <+560>:	xor    eax,eax
   0x0000000000203382 <+562>:	add    rsp,0x18
   0x0000000000203386 <+566>:	pop    rbx
   0x0000000000203387 <+567>:	pop    r12
   0x0000000000203389 <+569>:	pop    r13
   0x000000000020338b <+571>:	pop    r14
   0x000000000020338d <+573>:	pop    r15
   0x000000000020338f <+575>:	pop    rbp
   0x0000000000203390 <+576>:	ret
   0x0000000000203391 <+577>:	mov    edi,0x9
   0x0000000000203396 <+582>:	mov    esi,0x12
   0x000000000020339b <+587>:	call   0x203400 <__yk_trace_basicblock@plt>
   0x00000000002033a0 <+592>:	jmp    0x203371 <main+545>
