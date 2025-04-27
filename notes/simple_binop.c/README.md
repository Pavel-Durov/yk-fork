## Summary


## Asm transition code

### Reg sore at control point
>>> x/gx $rsp
0x7fffffffcb90:	0x00007ffff6ed7020
>>> x/gx 0x00007ffff6ed7020
0x7ffff6ed7020:	0x0000000000000004
>>> x/gx $r15
0x7ffff6ed7020:	0x0000000000000004
>>> p $rsp
$1 = (void *) 0x7fffffffcb90



>>> x/gx $rax
0x7ffff7fba1fe <_ZN8tempfile3env15DEFAULT_TEMPDIR17h27e24e1c6e58e22fE+22>:	0x0000000000000000
>>> x/gx $rsp
0x7fffffffcbf0:	0x00007ffff7fba1fe

>>> x/15gx $rsp
0x7fffffffcb90:	0x00007ffff6ed7020	0x00007ffff6ed7028
0x7fffffffcba0:	0x00007ffff6ed7030	0x00007ffff6ed702c
0x7fffffffcbb0:	0x00007ffff799a050	0x00007ffff7232be8
0x7fffffffcbc0:	0x00000000002086d0	0x0000000000000000
0x7fffffffcbd0:	0x00007fffffffcc50	0x00000000002086e0
0x7fffffffcbe0:	0x00007ffff6ed7024	0x00007ffff748d19c
0x7fffffffcbf0:	0x00007ffff7fba1fe	0x0000000000204a13
0x7fffffffcc00:	0x0000000000000000


src_rbp: 0x7fffffffcc80, reg_store: 0x7fffffffcb90, src_frame_size: 0x80, dst_frame_size: 0x80, rbp_offset_reg_store: 0xf0

r15=$rbp-0xf0
x/gx $rbp-0xf0
0x7fffffffcb90:	0x00007ffff6ed7020



              0x00007ffff7ffa001> ? movabs rbp,0x7fffffffcc80
              0x00007ffff7ffa00b  ? movabs rsp,0x7fffffffcc80
              0x00007ffff7ffa015  ? sub    rsp,0x80
              0x00007ffff7ffa01c  ? movabs rax,0x208af0

> temp storage
              0x00007ffff7ffa026  ? mov    rcx,QWORD PTR [rbp-0x50]
              0x00007ffff7ffa02d  ? mov    QWORD PTR [rax+riz*1+0x0],rcx
              0x00007ffff7ffa035  ? mov    rcx,QWORD PTR [rbp-0x48]
              0x00007ffff7ffa03c  ? mov    QWORD PTR [rax+riz*1+0x8],rcx
              0x00007ffff7ffa044  ? mov    rcx,QWORD PTR [rbp-0x40]
              0x00007ffff7ffa04b  ? mov    QWORD PTR [rax+riz*1+0x10],rcx


> Register2Indirect - src: Register(15, 8, []) dst: Indirect(6, -88, 8)
              0x00007ffff7ffa053  ? mov    rax,QWORD PTR [rbp-0xf0]
              0x00007ffff7ffa05a  ? mov    QWORD PTR [rbp-0x58],rax

> Register2Register - src: Register(3, 8, [-120]) dst: Register(3, 8, [-120])
              0x00007ffff7ffa061  ? mov    rax,QWORD PTR [rbp-0xa0]
              0x00007ffff7ffa068  ? mov    QWORD PTR [rbp-0x78],rax
              0x00007ffff7ffa06f  ? mov    rbx,QWORD PTR [rbp-0xa0]
> Register2Register - src: Register(14, 8, [-112]) dst: Register(15, 8, [-96])
              0x00007ffff7ffa076  ? mov    rax,QWORD PTR [rbp-0xe8]
              0x00007ffff7ffa07d  ? mov    QWORD PTR [rbp-0x60],rax
              0x00007ffff7ffa084  ? mov    r15,QWORD PTR [rbp-0xe8]
> Register2Register - src: Register(12, 8, [-104]) dst: Register(14, 8, [-112])
              0x00007ffff7ffa08b  ? mov    rax,QWORD PTR [rbp-0xd8]
              0x00007ffff7ffa092  ? mov    QWORD PTR [rbp-0x70],rax
              0x00007ffff7ffa099  ? mov    r14,QWORD PTR [rbp-0xd8]

> Register2Register - src: Register(13, 8, [-96]) dst: Register(12, 8, [-104])
              0x00007ffff7ffa0a0  ? mov    rax,QWORD PTR [rbp-0xe0]
              0x00007ffff7ffa0a7  ? mov    QWORD PTR [rbp-0x68],rax
              0x00007ffff7ffa0ae  ? mov    r12,QWORD PTR [rbp-0xe0]
> Register2Indirect - src: Register(2, 8, [-88]) dst: Indirect(6, -80, 8)
              0x00007ffff7ffa0b5  ? mov    rax,QWORD PTR [rbp-0x98]     <--------------------- rbp-0x98 hold garbage
              0x00007ffff7ffa0bc  ? mov    QWORD PTR [rbp-0x50],rax 
> Indirect2Register - src: Indirect(6, -80, 8) dst: Register(13, 8, [])
              0x00007ffff7ffa0c3  ? movabs rax,0x208af0
              0x00007ffff7ffa0cd  ? mov    r13,QWORD PTR [rax+riz*1+0x0]
>  Indirect2Indirect - src: Indirect(6, -72, 8) dst: Indirect(6, -72, 8)
              0x00007ffff7ffa0d5  ? movabs rax,0x208af0
              0x00007ffff7ffa0df  ? mov    rcx,QWORD PTR [rax+riz*1+0x8]
              0x00007ffff7ffa0e7  ? mov    QWORD PTR [rbp-0x48],rcx
> Indirect2Indirect - src: Indirect(6, -64, 8) dst: Indirect(6, -64, 8)
              0x00007ffff7ffa0ee  ? movabs rax,0x208af0
              0x00007ffff7ffa0f8  ? mov    rcx,QWORD PTR [rax+riz*1+0x10]
              0x00007ffff7ffa100  ? mov    QWORD PTR [rbp-0x40],rcx
> restorring registers
              0x00007ffff7ffa107  ? mov    rax,QWORD PTR [rbp-0x90]
              0x00007ffff7ffa10e  ? mov    rcx,QWORD PTR [rbp-0x98]
              0x00007ffff7ffa115  ? mov    rdi,QWORD PTR [rbp-0xa8]
              0x00007ffff7ffa11c  ? mov    rsi,QWORD PTR [rbp-0xb0]
              0x00007ffff7ffa123  ? mov    r8,QWORD PTR [rbp-0xb8]
              0x00007ffff7ffa12a  ? mov    r9,QWORD PTR [rbp-0xc0]
              0x00007ffff7ffa131  ? mov    r10,QWORD PTR [rbp-0xc8]
              0x00007ffff7ffa138  ? mov    r11,QWORD PTR [rbp-0xd0]
              0x00007ffff7ffa13f  ? sub    rsp,0x10
              0x00007ffff7ffa143  ? mov    QWORD PTR [rsp],rax
              0x00007ffff7ffa147  ? movabs rax,0x204d22
              0x00007ffff7ffa151  ? mov    QWORD PTR [rsp+0x8],rax
              0x00007ffff7ffa156  ? pop    rax


## Machine code
        Function: main
        # Machine code for function main: NoPHIs, TracksLiveness, NoVRegs, TiedOpsRewritten, TracksDebugUserValues
        Frame Objects:
          fi#-6: size=8, align=8, fixed, at location [SP-48]
          fi#-5: size=8, align=16, fixed, at location [SP-40]
          fi#-4: size=8, align=8, fixed, at location [SP-32]
          fi#-3: size=8, align=16, fixed, at location [SP-24]
          fi#-2: size=8, align=8, fixed, at location [SP-16]
          fi#-1: size=8, align=16, fixed, at location [SP-8]
          fi#0: size=8, align=8, at location [SP-64]
          fi#1: size=8, align=8, at location [SP-56]
          fi#2: size=8, align=8, at location [SP-128]
          fi#3: size=8, align=8, at location [SP-120]
          fi#4: size=8, align=8, at location [SP-112]
          fi#5: size=8, align=8, at location [SP-104]
          fi#6: size=8, align=8, at location [SP-96]
          fi#7: size=8, align=8, at location [SP-88]
          fi#8: size=8, align=8, at location [SP-80]
          fi#9: size=8, align=8, at location [SP-72]
        Function Live Ins: $edi, $rsi

        bb.0 (%ir-block.2, bb_id 0):
          successors: %bb.1
          liveins: $edi, $rsi, $r15, $r14, $r13, $r12, $rbx
          frame-setup PUSH64r killed $rbp, implicit-def $rsp, implicit $rsp
          frame-setup CFI_INSTRUCTION def_cfa_offset 16
          frame-setup CFI_INSTRUCTION offset $rbp, -16
          $rbp = frame-setup MOV64rr $rsp
          frame-setup CFI_INSTRUCTION def_cfa_register $rbp
          frame-setup PUSH64r killed $r15, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $r14, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $r13, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $r12, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $rbx, implicit-def $rsp, implicit $rsp
          $rsp = frame-setup SUB64ri32 $rsp(tied-def 0), 88, implicit-def dead $eflags
          CFI_INSTRUCTION offset $rbx, -56
          CFI_INSTRUCTION offset $r12, -48
          CFI_INSTRUCTION offset $r13, -40
          CFI_INSTRUCTION offset $r14, -32
          CFI_INSTRUCTION offset $r15, -24
          $r14 = MOV64rr $rsi
          $r12d = MOV32rr $edi
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags
          $esi = XOR32rr undef $esi(tied-def 0), undef $esi, implicit-def dead $eflags
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, implicit-def $rsp, implicit-def $ssp
          $edi = MOV32ri 1000000, implicit-def $rdi
          CALL64pcrel32 target-flags(x86-plt) @malloc, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit-def $rsp, implicit-def $ssp, implicit-def $rax
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 64, implicit-def dead $eflags
          renamable $rdx = MOV64rm $rip, 1, $noreg, target-flags(x86-gottpoff) @shadowstack_0, $noreg :: (load (s64) from got)
          MOV64mr killed renamable $rdx, 1, $noreg, 0, $fs, killed renamable $rcx :: (store (s64) into @shadowstack_0)
          $rbx = MOV64rr $rax
          renamable $rbx = ADD64ri32 killed renamable $rbx(tied-def 0), 8, implicit-def dead $eflags
          $r15 = MOV64rr $rax
          renamable $r15 = ADD64ri32 killed renamable $r15(tied-def 0), 16, implicit-def dead $eflags
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 20, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -120, $noreg, killed renamable $rcx :: (store (s64) into %stack.2)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 24, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -112, $noreg, killed renamable $rcx :: (store (s64) into %stack.3)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 28, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -104, $noreg, killed renamable $rcx :: (store (s64) into %stack.4)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 32, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -96, $noreg, killed renamable $rcx :: (store (s64) into %stack.5)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 36, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -88, $noreg, killed renamable $rcx :: (store (s64) into %stack.6)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 40, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -80, $noreg, killed renamable $rcx :: (store (s64) into %stack.7)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 44, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -72, $noreg, killed renamable $rcx :: (store (s64) into %stack.8)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 48, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -64, $noreg, killed renamable $rcx :: (store (s64) into %stack.9)
          MOV32mi renamable $rax, 1, $noreg, 0, $noreg, 0 :: (store (s32) into %ir.5)
          MOV32mr killed renamable $rax, 1, $noreg, 4, $noreg, killed renamable $r12d :: (store (s32) into %ir.6)
          JMP_1 %bb.1, debug-location !39; c/simple_binop.c:61:14

        bb.1 (%ir-block.19, bb_id 1):
        ; predecessors: %bb.0
          successors: %bb.2
          liveins: $rbx, $r14, $r15
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags
          $esi = MOV32ri 1
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi
          DBG_VALUE $noreg, $noreg, !"argc", !DIExpression(), debug-location !39; c/simple_binop.c:61:14 line no:61
          MOV64mr killed renamable $rbx, 1, $noreg, 0, $noreg, killed renamable $r14 :: (store (s64) into %ir.7)
          DBG_VALUE $noreg, $noreg, !"argv", !DIExpression(), debug-location !42; c/simple_binop.c:61:27 line no:61
          dead $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, implicit-def $rdi, debug-location !49; c/simple_binop.c:62:14
          CALL64pcrel32 @yk_mt_new, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit-def $rax, debug-location !49; c/simple_binop.c:62:14
          $rbx = MOV64rr $rax, debug-location !49; c/simple_binop.c:62:14
          JMP_1 %bb.2, debug-location !48; c/simple_binop.c:62:9

        bb.2 (%ir-block.21, bb_id 2):
        ; predecessors: %bb.1
          successors: %bb.3
          liveins: $rbx, $r15
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !48; c/simple_binop.c:62:9
          $esi = MOV32ri 2, debug-location !48; c/simple_binop.c:62:9
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !48; c/simple_binop.c:62:9
          MOV64mr $rbp, 1, $noreg, -56, $noreg, killed renamable $rbx, debug-location !48 :: (store (s64) into %ir.8); c/simple_binop.c:62:9
          renamable $rdi = MOV64rm $rbp, 1, $noreg, -56, $noreg, debug-location !50 :: (load (s64) from %ir.8); c/simple_binop.c:63:27
          $esi = XOR32rr undef $esi(tied-def 0), undef $esi, implicit-def dead $eflags, debug-location !51; c/simple_binop.c:63:3
          CALL64pcrel32 @yk_mt_hot_threshold_set, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit $esi, debug-location !51; c/simple_binop.c:63:3
          JMP_1 %bb.3, debug-location !52; c/simple_binop.c:64:14

        bb.3 (%ir-block.23, bb_id 3):
        ; predecessors: %bb.2
          successors: %bb.4
          liveins: $r15
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !53; c/simple_binop.c:64:20
          $esi = MOV32ri 3, debug-location !53; c/simple_binop.c:64:20
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !53; c/simple_binop.c:64:20
          CALL64pcrel32 @yk_location_new, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit-def $rax, debug-location !53; c/simple_binop.c:64:20
          $rbx = MOV64rr $rax, debug-location !53; c/simple_binop.c:64:20
          JMP_1 %bb.4, debug-location !53; c/simple_binop.c:64:20

        bb.4 (%ir-block.25, bb_id 4):
        ; predecessors: %bb.3
          successors: %bb.5
          liveins: $rbx, $r15
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !53; c/simple_binop.c:64:20
          $esi = MOV32ri 4, debug-location !53; c/simple_binop.c:64:20
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !53; c/simple_binop.c:64:20
          MOV64mr $rbp, 1, $noreg, -48, $noreg, killed renamable $rbx, debug-location !53 :: (store (s64) into %ir.26); c/simple_binop.c:64:20
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !63; c/simple_binop.c:66:7 line no:66 indirect
          MOV32mi renamable $r15, 1, $noreg, 0, $noreg, 4, debug-location !63 :: (store (s32) into %ir.10); c/simple_binop.c:66:7
          dead renamable $rax = MOV64rm $rbp, 1, $noreg, -48, $noreg, debug-location !64 :: (load (s64) from %ir.9); c/simple_binop.c:67:3
          INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], $rbp, 1, $noreg, -48, $noreg, $1:[mem:m], $rbp, 1, $noreg, -48, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !65, debug-location !64; c/simple_binop.c:67:3
          JMP_1 %bb.5, debug-location !66; c/simple_binop.c:68:3

        bb.5 (%ir-block.28, bb_id 5):
        ; predecessors: %bb.4
          successors: %bb.6
          liveins: $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !63; c/simple_binop.c:66:7 line no:66 indirect
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !66; c/simple_binop.c:68:3
          $esi = MOV32ri 5, debug-location !66; c/simple_binop.c:68:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !66; c/simple_binop.c:68:3
          dead renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !66 :: (load (s32) from %ir.10); c/simple_binop.c:68:3
          INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], renamable $r15, 1, $noreg, 0, $noreg, $1:[mem:m], renamable $r15, 1, $noreg, 0, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !67, debug-location !66; c/simple_binop.c:68:3
          JMP_1 %bb.6, debug-location !68; c/simple_binop.c:69:3

        bb.6 (%ir-block.30, bb_id 6):
        ; predecessors: %bb.5
          successors: %bb.7
          liveins: $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !63; c/simple_binop.c:66:7 line no:66 indirect
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !68; c/simple_binop.c:69:3
          $esi = MOV32ri 6, debug-location !68; c/simple_binop.c:69:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !68; c/simple_binop.c:69:3
          JMP_1 %bb.7, debug-location !68; c/simple_binop.c:69:3

        bb.7 (%ir-block.31, bb_id 7):
        ; predecessors: %bb.6, %bb.11
          successors: %bb.8, %bb.12
          liveins: $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !63; c/simple_binop.c:66:7 line no:66 indirect
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !69; c/simple_binop.c:69:10
          $esi = MOV32ri 7, debug-location !69; c/simple_binop.c:69:10
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !69; c/simple_binop.c:69:10
          STACKMAP 2, 0, 0, $rbp, -56, 3, 0, $rbp, -48, 3, renamable $r15, 3, 1, 8, $rbp, -120, 3, 1, 8, $rbp, -112, 3, 1, 8, $rbp, -104, 3, 1, 8, $rbp, -96, 3, 1, 8, $rbp, -88, 3, 1, 8, $rbp, -80, 3, 1, 8, $rbp, -72, 3, 1, 8, $rbp, -64, 3, renamable $al, 3, implicit-def dead early-clobber $r11, debug-location !68; c/simple_binop.c:69:3
          CMP32mi renamable $r15, 1, $noreg, 0, $noreg, 0, implicit-def $eflags, debug-location !70 :: (load (s32) from %ir.10); c/simple_binop.c:69:12
          renamable $al = SETCCr 15, implicit killed $eflags, debug-location !70; c/simple_binop.c:69:12
          renamable $r14 = MOV64rm $rbp, 1, $noreg, -112, $noreg :: (load (s64) from %stack.3)
          renamable $r12 = MOV64rm $rbp, 1, $noreg, -104, $noreg :: (load (s64) from %stack.4)
          renamable $r13 = MOV64rm $rbp, 1, $noreg, -96, $noreg :: (load (s64) from %stack.5)
          renamable $rcx = MOV64rm $rbp, 1, $noreg, -88, $noreg :: (load (s64) from %stack.6)
          renamable $rdx = MOV64rm $rbp, 1, $noreg, -72, $noreg :: (load (s64) from %stack.8)
          renamable $rsi = MOV64rm $rbp, 1, $noreg, -64, $noreg :: (load (s64) from %stack.9)
          renamable $rbx = MOV64rm $rbp, 1, $noreg, -120, $noreg :: (load (s64) from %stack.2)
          TEST8ri killed renamable $al, 1, implicit-def $eflags, debug-location !68; c/simple_binop.c:69:3
          JCC_1 %bb.8, 5, implicit killed $eflags, debug-location !68; c/simple_binop.c:69:3
          JMP_1 %bb.12, debug-location !68; c/simple_binop.c:69:3

        bb.8 (%ir-block.34, bb_id 8):
        ; predecessors: %bb.7
          successors: %bb.9
          liveins: $rbx, $r12, $r13, $r14, $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !63; c/simple_binop.c:66:7 line no:66 indirect
          $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !71; c/simple_binop.c:70:25
          $esi = MOV32ri 8, debug-location !71; c/simple_binop.c:70:25
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !71; c/simple_binop.c:70:25
          renamable $rdi = MOV64rm $rbp, 1, $noreg, -56, $noreg, debug-location !71 :: (load (s64) from %ir.8); c/simple_binop.c:70:25
          renamable $rsi = LEA64r $rbp, 1, $noreg, -48, $noreg, debug-location !73; c/simple_binop.c:70:5
          dead $edx = XOR32rr undef $edx(tied-def 0), undef $edx, implicit-def dead $eflags, implicit-def $rdx, debug-location !73; c/simple_binop.c:70:5

> Opt-> unop control_point call

          PATCHPOINT 0, 13, @__ykrt_control_point, 3, 0, $rdi, $rsi, $rdx, 0, $rbp, -56, 3, 0, $rbp, -48, 3, renamable $r15, 3, renamable $rbx, 3, renamable $r14, 3, renamable $r12, 3, renamable $r13, 3, 1, 8, $rbp, -88, 3, 1, 8, $rbp, -80, 3, 1, 8, $rbp, -72, 3, 1, 8, $rbp, -64, 3, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, liveout($bh, $bl, $bx, $ebx, $hbx, $rbx, $r12, $r13, $r14, $r15, $r12b, $r13b, $r14b, $r15b, $r12bh, $r13bh, $r14bh, $r15bh, $r12d, $r13d, $r14d, $r15d, $r12w, $r13w, $r14w, $r15w, $r12wh, $r13wh, $r14wh, $r15wh), implicit-def dead early-clobber $r11, debug-location !73 :: (load (s64) from %stack.0), (load (s64) from %stack.1), (load (s64) from %stack.7), (load (s64) from %stack.9), (load (s64) from %stack.8), (load (s64) from %stack.6); c/simple_binop.c:70:5
          JMP_1 %bb.9, debug-location !74; c/simple_binop.c:71:9


        ********** Machine Code when processing stackmap **********
        Function: __yk_unopt_main
        # Machine code for function __yk_unopt_main: NoPHIs, TracksLiveness, NoVRegs, TiedOpsRewritten, TracksDebugUserValues
        Frame Objects:
          fi#-6: size=8, align=8, fixed, at location [SP-48]
          fi#-5: size=8, align=16, fixed, at location [SP-40]
          fi#-4: size=8, align=8, fixed, at location [SP-32]
          fi#-3: size=8, align=16, fixed, at location [SP-24]
          fi#-2: size=8, align=8, fixed, at location [SP-16]
          fi#-1: size=8, align=16, fixed, at location [SP-8]
          fi#0: size=8, align=8, at location [SP-64]
          fi#1: size=8, align=8, at location [SP-56]
          fi#2: size=8, align=8, at location [SP-128]
          fi#3: size=8, align=8, at location [SP-104]
          fi#4: size=8, align=8, at location [SP-120]
          fi#5: size=8, align=8, at location [SP-112]
          fi#6: size=8, align=8, at location [SP-88]
          fi#7: size=8, align=8, at location [SP-80]
          fi#8: size=8, align=8, at location [SP-72]
          fi#9: size=8, align=8, at location [SP-96]
        Function Live Ins: $edi, $rsi

        bb.0 (%ir-block.2, bb_id 0):
          successors: %bb.1
          liveins: $edi, $rsi, $r15, $r14, $r13, $r12, $rbx
          frame-setup PUSH64r killed $rbp, implicit-def $rsp, implicit $rsp
          frame-setup CFI_INSTRUCTION def_cfa_offset 16
          frame-setup CFI_INSTRUCTION offset $rbp, -16
          $rbp = frame-setup MOV64rr $rsp
          frame-setup CFI_INSTRUCTION def_cfa_register $rbp
          frame-setup PUSH64r killed $r15, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $r14, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $r13, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $r12, implicit-def $rsp, implicit $rsp
          frame-setup PUSH64r killed $rbx, implicit-def $rsp, implicit $rsp
          $rsp = frame-setup SUB64ri32 $rsp(tied-def 0), 88, implicit-def dead $eflags
          CFI_INSTRUCTION offset $rbx, -56
          CFI_INSTRUCTION offset $r12, -48
          CFI_INSTRUCTION offset $r13, -40
          CFI_INSTRUCTION offset $r14, -32
          CFI_INSTRUCTION offset $r15, -24
          $r12 = MOV64rr $rsi
          $r14d = MOV32rr $edi
          $edi = MOV32ri 9
          $esi = XOR32rr undef $esi(tied-def 0), undef $esi, implicit-def dead $eflags
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, implicit-def $rsp, implicit-def $ssp
          renamable $rax = MOV64rm $rip, 1, $noreg, target-flags(x86-gottpoff) @shadowstack_0, $noreg :: (load (s64) from got)
          renamable $rax = MOV64rm killed renamable $rax, 1, $noreg, 0, $fs :: (dereferenceable load (s64) from @shadowstack_0)
          $rbx = MOV64rr $rax
          renamable $rbx = ADD64ri32 killed renamable $rbx(tied-def 0), 8, implicit-def dead $eflags
          $r15 = MOV64rr $rax
          renamable $r15 = ADD64ri32 killed renamable $r15(tied-def 0), 16, implicit-def dead $eflags
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 20, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -120, $noreg, killed renamable $rcx :: (store (s64) into %stack.2)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 24, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -96, $noreg, killed renamable $rcx :: (store (s64) into %stack.3)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 28, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -112, $noreg, killed renamable $rcx :: (store (s64) into %stack.4)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 32, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -104, $noreg, killed renamable $rcx :: (store (s64) into %stack.5)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 36, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -80, $noreg, killed renamable $rcx :: (store (s64) into %stack.6)
          $r13 = MOV64rr $rax
          renamable $r13 = ADD64ri32 killed renamable $r13(tied-def 0), 40, implicit-def dead $eflags
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 44, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -72, $noreg, killed renamable $rcx :: (store (s64) into %stack.7)
          $rcx = MOV64rr $rax
          renamable $rcx = ADD64ri32 killed renamable $rcx(tied-def 0), 48, implicit-def dead $eflags
          MOV64mr $rbp, 1, $noreg, -64, $noreg, killed renamable $rcx :: (store (s64) into %stack.8)
          MOV32mi renamable $rax, 1, $noreg, 0, $noreg, 0 :: (store (s32) into %ir.4)
          MOV32mr killed renamable $rax, 1, $noreg, 4, $noreg, killed renamable $r14d :: (store (s32) into %ir.5)
          JMP_1 %bb.1, debug-location !130; c/simple_binop.c:61:14

        bb.1 (%ir-block.18, bb_id 1):
        ; predecessors: %bb.0
          successors: %bb.2
          liveins: $rbx, $r12, $r13, $r15
          $edi = MOV32ri 9
          $esi = MOV32ri 1
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi
          DBG_VALUE $noreg, $noreg, !"argc", !DIExpression(), debug-location !130; c/simple_binop.c:61:14 line no:61
          MOV64mr killed renamable $rbx, 1, $noreg, 0, $noreg, killed renamable $r12 :: (store (s64) into %ir.6)
          DBG_VALUE $noreg, $noreg, !"argv", !DIExpression(), debug-location !133; c/simple_binop.c:61:27 line no:61
          dead $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, implicit-def $rdi, debug-location !136; c/simple_binop.c:62:14
          CALL64pcrel32 @yk_mt_new, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit-def $rax, debug-location !136; c/simple_binop.c:62:14
          $rbx = MOV64rr $rax, debug-location !136; c/simple_binop.c:62:14
          JMP_1 %bb.2, debug-location !135; c/simple_binop.c:62:9

        bb.2 (%ir-block.20, bb_id 2):
        ; predecessors: %bb.1
          successors: %bb.3
          liveins: $rbx, $r13, $r15
          $edi = MOV32ri 9, debug-location !135; c/simple_binop.c:62:9
          $esi = MOV32ri 2, debug-location !135; c/simple_binop.c:62:9
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !135; c/simple_binop.c:62:9
          MOV64mr $rbp, 1, $noreg, -56, $noreg, killed renamable $rbx, debug-location !135 :: (store (s64) into %ir.7); c/simple_binop.c:62:9
          renamable $rdi = MOV64rm $rbp, 1, $noreg, -56, $noreg, debug-location !137 :: (load (s64) from %ir.7); c/simple_binop.c:63:27
          $esi = XOR32rr undef $esi(tied-def 0), undef $esi, implicit-def dead $eflags, debug-location !138; c/simple_binop.c:63:3
          CALL64pcrel32 @yk_mt_hot_threshold_set, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit $esi, debug-location !138; c/simple_binop.c:63:3
          JMP_1 %bb.3, debug-location !139; c/simple_binop.c:64:14

        bb.3 (%ir-block.22, bb_id 3):
        ; predecessors: %bb.2
          successors: %bb.4
          liveins: $r13, $r15
          $edi = MOV32ri 9, debug-location !140; c/simple_binop.c:64:20
          $esi = MOV32ri 3, debug-location !140; c/simple_binop.c:64:20
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !140; c/simple_binop.c:64:20
          CALL64pcrel32 @yk_location_new, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit-def $rax, debug-location !140; c/simple_binop.c:64:20
          $rbx = MOV64rr $rax, debug-location !140; c/simple_binop.c:64:20
          JMP_1 %bb.4, debug-location !140; c/simple_binop.c:64:20

        bb.4 (%ir-block.24, bb_id 4):
        ; predecessors: %bb.3
          successors: %bb.5
          liveins: $rbx, $r13, $r15
          $edi = MOV32ri 9, debug-location !140; c/simple_binop.c:64:20
          $esi = MOV32ri 4, debug-location !140; c/simple_binop.c:64:20
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !140; c/simple_binop.c:64:20
          MOV64mr $rbp, 1, $noreg, -48, $noreg, killed renamable $rbx, debug-location !140 :: (store (s64) into %ir.25); c/simple_binop.c:64:20
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          MOV32mi renamable $r15, 1, $noreg, 0, $noreg, 4, debug-location !143 :: (store (s32) into %ir.9); c/simple_binop.c:66:7
          dead renamable $rax = MOV64rm $rbp, 1, $noreg, -48, $noreg, debug-location !144 :: (load (s64) from %ir.8); c/simple_binop.c:67:3
          INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], $rbp, 1, $noreg, -48, $noreg, $1:[mem:m], $rbp, 1, $noreg, -48, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !65, debug-location !144; c/simple_binop.c:67:3
          JMP_1 %bb.5, debug-location !145; c/simple_binop.c:68:3

        bb.5 (%ir-block.27, bb_id 5):
        ; predecessors: %bb.4
          successors: %bb.6
          liveins: $r13, $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !145; c/simple_binop.c:68:3
          $esi = MOV32ri 5, debug-location !145; c/simple_binop.c:68:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !145; c/simple_binop.c:68:3
          dead renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !145 :: (load (s32) from %ir.9); c/simple_binop.c:68:3
          INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], renamable $r15, 1, $noreg, 0, $noreg, $1:[mem:m], renamable $r15, 1, $noreg, 0, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !67, debug-location !145; c/simple_binop.c:68:3
          JMP_1 %bb.6, debug-location !146; c/simple_binop.c:69:3

        bb.6 (%ir-block.29, bb_id 6):
        ; predecessors: %bb.5
          successors: %bb.7
          liveins: $r13, $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !146; c/simple_binop.c:69:3
          $esi = MOV32ri 6, debug-location !146; c/simple_binop.c:69:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !146; c/simple_binop.c:69:3
          JMP_1 %bb.7, debug-location !146; c/simple_binop.c:69:3

        bb.7 (%ir-block.30, bb_id 7):
        ; predecessors: %bb.6, %bb.11
          successors: %bb.8, %bb.12
          liveins: $r13, $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !147; c/simple_binop.c:69:10
          $esi = MOV32ri 7, debug-location !147; c/simple_binop.c:69:10
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !147; c/simple_binop.c:69:10
          CMP32mi renamable $r15, 1, $noreg, 0, $noreg, 0, implicit-def $eflags, debug-location !148 :: (load (s32) from %ir.9); c/simple_binop.c:69:12
          renamable $al = SETCCr 15, implicit killed $eflags, debug-location !148; c/simple_binop.c:69:12
          renamable $r14 = MOV64rm $rbp, 1, $noreg, -112, $noreg :: (load (s64) from %stack.4)
          renamable $r12 = MOV64rm $rbp, 1, $noreg, -104, $noreg :: (load (s64) from %stack.5)
          renamable $rcx = MOV64rm $rbp, 1, $noreg, -80, $noreg :: (load (s64) from %stack.6)
          renamable $rdx = MOV64rm $rbp, 1, $noreg, -64, $noreg :: (load (s64) from %stack.8)
          renamable $rbx = MOV64rm $rbp, 1, $noreg, -96, $noreg :: (load (s64) from %stack.3)
          renamable $rsi = MOV64rm $rbp, 1, $noreg, -120, $noreg :: (load (s64) from %stack.2)
          STACKMAP 3, 0, 0, $rbp, -56, 3, 0, $rbp, -48, 3, renamable $r15, 3, renamable $rsi, 3, renamable $rbx, 3, renamable $r14, 3, renamable $r12, 3, killed renamable $rcx, 3, renamable $r13, 3, 1, 8, $rbp, -72, 3, killed renamable $rdx, 3, renamable $al, 3, implicit-def dead early-clobber $r11, debug-location !146 :: (load (s64) from %stack.0), (load (s64) from %stack.1), (load (s64) from %stack.7); c/simple_binop.c:69:3
          TEST8ri killed renamable $al, 1, implicit-def $eflags, debug-location !146; c/simple_binop.c:69:3
          JCC_1 %bb.8, 5, implicit killed $eflags, debug-location !146; c/simple_binop.c:69:3
          JMP_1 %bb.12, debug-location !146; c/simple_binop.c:69:3

        bb.8 (%ir-block.33, bb_id 8):
        ; predecessors: %bb.7
          successors: %bb.9
          liveins: $rbx, $rsi, $r12, $r13, $r14, $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !149; c/simple_binop.c:70:25
          MOV64mr $rbp, 1, $noreg, -88, $noreg, killed renamable $r15 :: (store (s64) into %stack.9)
          DBG_VALUE $rbp, 0, !"i", !DIExpression(DW_OP_constu, 88, DW_OP_minus, DW_OP_deref), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $r15 = MOV64rr killed $rbx
          $rbx = MOV64rr killed $rsi
          $esi = MOV32ri 8, debug-location !149; c/simple_binop.c:70:25
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !149; c/simple_binop.c:70:25
          renamable $rdi = MOV64rm $rbp, 1, $noreg, -56, $noreg, debug-location !149 :: (load (s64) from %ir.7); c/simple_binop.c:70:25
          renamable $rsi = LEA64r $rbp, 1, $noreg, -48, $noreg, debug-location !151; c/simple_binop.c:70:5
          $edx = MOV32ri 1, implicit-def $rdx, debug-location !151; c/simple_binop.c:70:5
          PATCHPOINT 1, 13, @__ykrt_control_point, 3, 0, $rdi, $rsi, $rdx, 0, $rbp, -56, 3, 0, $rbp, -48, 3, 1, 8, $rbp, -88, 3, renamable $rbx, 3, killed renamable $r15, 3, renamable $r14, 3, renamable $r12, 3, 1, 8, $rbp, -80, 3, renamable $r13, 3, 1, 8, $rbp, -72, 3, 1, 8, $rbp, -64, 3, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, liveout($bh, $bl, $bp, $bph, $bpl, $bx, $ebp, $ebx, $hbp, $hbx, $rbp, $rbx, $r12, $r13, $r14, $r12b, $r13b, $r14b, $r12bh, $r13bh, $r14bh, $r12d, $r13d, $r14d, $r12w, $r13w, $r14w, $r12wh, $r13wh, $r14wh), implicit-def dead early-clobber $r11, debug-location !151 :: (load (s64) from %stack.0), (load (s64) from %stack.1), (load (s64) from %stack.7), (load (s64) from %stack.9), (load (s64) from %stack.6), (load (s64) from %stack.8); c/simple_binop.c:70:5
          renamable $r15 = MOV64rm $rbp, 1, $noreg, -88, $noreg :: (load (s64) from %stack.9)
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          JMP_1 %bb.9, debug-location !152; c/simple_binop.c:71:9

        bb.9 (%ir-block.35, bb_id 9):
        ; predecessors: %bb.8
          successors: %bb.10
          liveins: $rbx, $r12, $r13, $r14, $r15
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !153; c/simple_binop.c:71:15
          $esi = MOV32ri 9, debug-location !153; c/simple_binop.c:71:15
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !153; c/simple_binop.c:71:15
          DBG_VALUE $rbp, 0, !"and", !DIExpression(DW_OP_constu, 120, DW_OP_minus, DW_OP_deref), debug-location !152; c/simple_binop.c:71:9 line no:71 indirect
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !153 :: (load (s32) from %ir.9); c/simple_binop.c:71:15
          renamable $eax = AND32ri killed renamable $eax(tied-def 0), 1, implicit-def dead $eflags, debug-location !155; c/simple_binop.c:71:17
          MOV32mr renamable $rbx, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !152 :: (store (s32) into %ir.10); c/simple_binop.c:71:9
          DBG_VALUE $rbp, 0, !"or", !DIExpression(DW_OP_constu, 96, DW_OP_minus, DW_OP_deref), debug-location !157; c/simple_binop.c:72:9 line no:72 indirect
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !158 :: (load (s32) from %ir.9); c/simple_binop.c:72:14
          renamable $eax = OR32ri killed renamable $eax(tied-def 0), 1, implicit-def dead $eflags, debug-location !159; c/simple_binop.c:72:16
          renamable $rcx = MOV64rm $rbp, 1, $noreg, -96, $noreg :: (load (s64) from %stack.3)
          MOV32mr renamable $rcx, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !157 :: (store (s32) into %ir.11); c/simple_binop.c:72:9
          DBG_VALUE $rbp, 0, !"lshr", !DIExpression(DW_OP_constu, 112, DW_OP_minus, DW_OP_deref), debug-location !161; c/simple_binop.c:73:9 line no:73 indirect
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !162 :: (load (s32) from %ir.9); c/simple_binop.c:73:22
          renamable $eax = SHR32ri killed renamable $eax(tied-def 0), 1, implicit-def dead $eflags, debug-location !163; c/simple_binop.c:73:24
          MOV32mr renamable $r14, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !161 :: (store (s32) into %ir.12); c/simple_binop.c:73:9
          DBG_VALUE $rbp, 0, !"ashr", !DIExpression(DW_OP_constu, 104, DW_OP_minus, DW_OP_deref), debug-location !165; c/simple_binop.c:74:9 line no:74 indirect
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !166 :: (load (s32) from %ir.9); c/simple_binop.c:74:16
          renamable $eax = SAR32ri killed renamable $eax(tied-def 0), 1, implicit-def dead $eflags, debug-location !167; c/simple_binop.c:74:18
          MOV32mr renamable $r12, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !165 :: (store (s32) into %ir.13); c/simple_binop.c:74:9
          renamable $rdi = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !168 :: (load (s64) from @stderr); c/simple_binop.c:75:13
          renamable $edx = MOV32rm killed renamable $rbx, 1, $noreg, 0, $noreg, debug-location !169 :: (load (s32) from %ir.10); c/simple_binop.c:75:58
          renamable $ecx = MOV32rm killed renamable $rcx, 1, $noreg, 0, $noreg, debug-location !170 :: (load (s32) from %ir.11); c/simple_binop.c:75:63
          renamable $r8d = MOV32rm killed renamable $r14, 1, $noreg, 0, $noreg, debug-location !171 :: (load (s32) from %ir.12); c/simple_binop.c:75:67
          renamable $r9d = MOV32rm killed renamable $r12, 1, $noreg, 0, $noreg, debug-location !172 :: (load (s32) from %ir.13); c/simple_binop.c:75:73
          renamable $rsi = MOV64ri @.str, debug-location !173; c/simple_binop.c:75:5
          $al = MOV8ri 0, debug-location !173; c/simple_binop.c:75:5
          CALL64pcrel32 @fprintf, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $al, implicit $rdi, implicit $rsi, implicit $edx, implicit $ecx, implicit $r8d, implicit $r9d, implicit-def $eax, debug-location !173; c/simple_binop.c:75:5
          JMP_1 %bb.10, debug-location !174; c/simple_binop.c:76:9

        bb.10 (%ir-block.50, bb_id 10):
        ; predecessors: %bb.9
          successors: %bb.11
          liveins: $r13, $r15
          DBG_VALUE $rbp, 0, !"ashr", !DIExpression(DW_OP_constu, 104, DW_OP_minus, DW_OP_deref), debug-location !165; c/simple_binop.c:74:9 line no:74 indirect
          DBG_VALUE $rbp, 0, !"lshr", !DIExpression(DW_OP_constu, 112, DW_OP_minus, DW_OP_deref), debug-location !161; c/simple_binop.c:73:9 line no:73 indirect
          DBG_VALUE $rbp, 0, !"or", !DIExpression(DW_OP_constu, 96, DW_OP_minus, DW_OP_deref), debug-location !157; c/simple_binop.c:72:9 line no:72 indirect
          DBG_VALUE $rbp, 0, !"and", !DIExpression(DW_OP_constu, 120, DW_OP_minus, DW_OP_deref), debug-location !152; c/simple_binop.c:71:9 line no:71 indirect
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !175; c/simple_binop.c:76:18
          $esi = MOV32ri 10, debug-location !175; c/simple_binop.c:76:18
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !175; c/simple_binop.c:76:18

> $rbp-80 ashr2

          DBG_VALUE $rbp, 0, !"ashr2", !DIExpression(DW_OP_constu, 80, DW_OP_minus, DW_OP_deref), debug-location !174; c/simple_binop.c:76:9 line no:76 indirect
          renamable $eax = XOR32rr undef $eax(tied-def 0), undef $eax, implicit-def dead $eflags, debug-location !177; c/simple_binop.c:76:17
          renamable $eax = SUB32rm killed renamable $eax(tied-def 0), renamable $r15, 1, $noreg, 0, $noreg, implicit-def dead $eflags, debug-location !177 :: (load (s32) from %ir.9); c/simple_binop.c:76:17
          renamable $eax = SAR32ri killed renamable $eax(tied-def 0), 1, implicit-def dead $eflags, debug-location !178; c/simple_binop.c:76:20
          renamable $rcx = MOV64rm $rbp, 1, $noreg, -80, $noreg :: (load (s64) from %stack.6)
          MOV32mr renamable $rcx, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !174 :: (store (s32) into %ir.14); c/simple_binop.c:76:9
          DBG_VALUE $r13, 0, !"xor", !DIExpression(), debug-location !180; c/simple_binop.c:77:9 line no:77 indirect
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !181 :: (load (s32) from %ir.9); c/simple_binop.c:77:15
          renamable $eax = XOR32ri killed renamable $eax(tied-def 0), 1, implicit-def dead $eflags, debug-location !182; c/simple_binop.c:77:17
          MOV32mr renamable $r13, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !180 :: (store (s32) into %ir.15); c/simple_binop.c:77:9
          DBG_VALUE $rbp, 0, !"xor2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !184; c/simple_binop.c:78:9 line no:78 indirect
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !185 :: (load (s32) from %ir.9); c/simple_binop.c:78:17
          renamable $eax = XOR32ri killed renamable $eax(tied-def 0), -1, implicit-def dead $eflags, debug-location !186; c/simple_binop.c:78:16
          renamable $r8 = MOV64rm $rbp, 1, $noreg, -72, $noreg :: (load (s64) from %stack.7)
          MOV32mr renamable $r8, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !184 :: (store (s32) into %ir.16); c/simple_binop.c:78:9
          DBG_VALUE $rbp, 0, !"shl", !DIExpression(DW_OP_constu, 64, DW_OP_minus, DW_OP_deref), debug-location !188; c/simple_binop.c:79:9 line no:79 indirect
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !189 :: (load (s32) from %ir.9); c/simple_binop.c:79:15
          renamable $eax = SHL32ri killed renamable $eax(tied-def 0), 1, implicit-def dead $eflags, debug-location !190; c/simple_binop.c:79:17
          renamable $rsi = MOV64rm $rbp, 1, $noreg, -64, $noreg :: (load (s64) from %stack.8)
          MOV32mr renamable $rsi, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !188 :: (store (s32) into %ir.17); c/simple_binop.c:79:9
          renamable $rdi = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !191 :: (load (s64) from @stderr); c/simple_binop.c:80:13
          renamable $edx = MOV32rm killed renamable $rcx, 1, $noreg, 0, $noreg, debug-location !192 :: (load (s32) from %ir.14); c/simple_binop.c:80:65
          renamable $ecx = MOV32rm renamable $r13, 1, $noreg, 0, $noreg, debug-location !193 :: (load (s32) from %ir.15); c/simple_binop.c:80:72
          renamable $r8d = MOV32rm killed renamable $r8, 1, $noreg, 0, $noreg, debug-location !194 :: (load (s32) from %ir.16); c/simple_binop.c:81:13
          renamable $r9d = MOV32rm killed renamable $rsi, 1, $noreg, 0, $noreg, debug-location !195 :: (load (s32) from %ir.17); c/simple_binop.c:81:19
          renamable $rsi = MOV64ri @.str.1, debug-location !196; c/simple_binop.c:80:5
          $al = MOV8ri 0, debug-location !196; c/simple_binop.c:80:5
          CALL64pcrel32 @fprintf, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $al, implicit $rdi, implicit $rsi, implicit $edx, implicit $ecx, implicit $r8d, implicit $r9d, implicit-def $eax, debug-location !196; c/simple_binop.c:80:5
          JMP_1 %bb.11, debug-location !197; c/simple_binop.c:82:6

        bb.11 (%ir-block.66, bb_id 11):
        ; predecessors: %bb.10
          successors: %bb.7
          liveins: $r13, $r15
          DBG_VALUE $rbp, 0, !"shl", !DIExpression(DW_OP_constu, 64, DW_OP_minus, DW_OP_deref), debug-location !188; c/simple_binop.c:79:9 line no:79 indirect
          DBG_VALUE $rbp, 0, !"xor2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !184; c/simple_binop.c:78:9 line no:78 indirect
          DBG_VALUE $r13, 0, !"xor", !DIExpression(), debug-location !180; c/simple_binop.c:77:9 line no:77 indirect
          DBG_VALUE $rbp, 0, !"ashr2", !DIExpression(DW_OP_constu, 80, DW_OP_minus, DW_OP_deref), debug-location !174; c/simple_binop.c:76:9 line no:76 indirect
          DBG_VALUE $rbp, 0, !"ashr", !DIExpression(DW_OP_constu, 104, DW_OP_minus, DW_OP_deref), debug-location !165; c/simple_binop.c:74:9 line no:74 indirect
          DBG_VALUE $rbp, 0, !"lshr", !DIExpression(DW_OP_constu, 112, DW_OP_minus, DW_OP_deref), debug-location !161; c/simple_binop.c:73:9 line no:73 indirect
          DBG_VALUE $rbp, 0, !"or", !DIExpression(DW_OP_constu, 96, DW_OP_minus, DW_OP_deref), debug-location !157; c/simple_binop.c:72:9 line no:72 indirect
          DBG_VALUE $rbp, 0, !"and", !DIExpression(DW_OP_constu, 120, DW_OP_minus, DW_OP_deref), debug-location !152; c/simple_binop.c:71:9 line no:71 indirect
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !197; c/simple_binop.c:82:6
          $esi = MOV32ri 11, debug-location !197; c/simple_binop.c:82:6
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !197; c/simple_binop.c:82:6
          renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !197 :: (load (s32) from %ir.9); c/simple_binop.c:82:6
          renamable $eax = ADD32ri killed renamable $eax(tied-def 0), -1, implicit-def dead $eflags, debug-location !197; c/simple_binop.c:82:6
          MOV32mr renamable $r15, 1, $noreg, 0, $noreg, killed renamable $eax, debug-location !197 :: (store (s32) into %ir.9); c/simple_binop.c:82:6
          JMP_1 %bb.7, debug-location !146; c/simple_binop.c:69:3

        bb.12 (%ir-block.69, bb_id 12):
        ; predecessors: %bb.7
          successors: %bb.13

          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !200; c/simple_binop.c:84:11
          $esi = MOV32ri 12, debug-location !200; c/simple_binop.c:84:11
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !200; c/simple_binop.c:84:11
          renamable $rdi = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !200 :: (load (s64) from @stderr); c/simple_binop.c:84:11
          renamable $rsi = MOV64ri @.str.2, debug-location !201; c/simple_binop.c:84:3
          $al = MOV8ri 0, debug-location !201; c/simple_binop.c:84:3
          CALL64pcrel32 @fprintf, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $al, implicit $rdi, implicit $rsi, implicit-def $eax, debug-location !201; c/simple_binop.c:84:3
          JMP_1 %bb.13, debug-location !202; c/simple_binop.c:85:3

        bb.13 (%ir-block.72, bb_id 13):
        ; predecessors: %bb.12
          successors: %bb.14

          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !202; c/simple_binop.c:85:3
          $esi = MOV32ri 13, debug-location !202; c/simple_binop.c:85:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !202; c/simple_binop.c:85:3
          renamable $rdi = MOV64rm $rbp, 1, $noreg, -48, $noreg, debug-location !202 :: (load (s64) from %ir.73); c/simple_binop.c:85:3
          CALL64pcrel32 @yk_location_drop, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, debug-location !202; c/simple_binop.c:85:3
          JMP_1 %bb.14, debug-location !203; c/simple_binop.c:86:18

        bb.14 (%ir-block.75, bb_id 14):
        ; predecessors: %bb.13
          successors: %bb.15

          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !203; c/simple_binop.c:86:18
          $esi = MOV32ri 14, debug-location !203; c/simple_binop.c:86:18
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !203; c/simple_binop.c:86:18
          renamable $rdi = MOV64rm $rbp, 1, $noreg, -56, $noreg, debug-location !203 :: (load (s64) from %ir.7); c/simple_binop.c:86:18
          CALL64pcrel32 @yk_mt_shutdown, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, debug-location !204; c/simple_binop.c:86:3
          JMP_1 %bb.15, debug-location !205; c/simple_binop.c:87:3

        bb.15 (%ir-block.77, bb_id 15):
        ; predecessors: %bb.14
          successors: %bb.17

          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !205; c/simple_binop.c:87:3
          $esi = MOV32ri 15, debug-location !205; c/simple_binop.c:87:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !205; c/simple_binop.c:87:3
          JMP_1 %bb.17, debug-location !205; c/simple_binop.c:87:3

        bb.16 (%ir-block.78, bb_id 16):
        ; predecessors: %bb.17

          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !205; c/simple_binop.c:87:3
          $esi = MOV32ri 16, debug-location !205; c/simple_binop.c:87:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !205; c/simple_binop.c:87:3
          $eax = XOR32rr undef $eax(tied-def 0), undef $eax, implicit-def dead $eflags, debug-location !205; c/simple_binop.c:87:3
          $rsp = frame-destroy ADD64ri32 $rsp(tied-def 0), 88, implicit-def dead $eflags, debug-location !205; c/simple_binop.c:87:3
          $rbx = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !205; c/simple_binop.c:87:3
          $r12 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !205; c/simple_binop.c:87:3
          $r13 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !205; c/simple_binop.c:87:3
          $r14 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !205; c/simple_binop.c:87:3
          $r15 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !205; c/simple_binop.c:87:3
          $rbp = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !205; c/simple_binop.c:87:3
          frame-destroy CFI_INSTRUCTION def_cfa $rsp, 8, debug-location !205; c/simple_binop.c:87:3
          RET64 implicit $eax, debug-location !205; c/simple_binop.c:87:3

        bb.17 (%ir-block.79, bb_id 17):
        ; predecessors: %bb.15
          successors: %bb.16

          CFI_INSTRUCTION def_cfa $rbp, 16, debug-location !205; c/simple_binop.c:87:3
          DBG_VALUE $r15, 0, !"i", !DIExpression(), debug-location !143; c/simple_binop.c:66:7 line no:66 indirect
          $edi = MOV32ri 9, debug-location !205; c/simple_binop.c:87:3
          $esi = MOV32ri 17, debug-location !205; c/simple_binop.c:87:3
          CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !205; c/simple_binop.c:87:3
          JMP_1 %bb.16, debug-location !205; c/simple_binop.c:87:3

        # End machine code for function __yk_unopt_main.

        ********** End of Machine Code **********

        yk-jit-event: start-tracing
        Allocated buffer of size 32 at 0x20722ab0 for direction OptToUnopt
        Using buffer at 0x20722ab0 for direction OptToUnopt
        Transition: OptToUnopt
        src_rbp: 0x7ffc61d40d00, reg_store: 0x7ffc61d40c10, src_frame_size: 0x80, dst_frame_size: 0x80, rbp_offset_reg_store: 0xf0
        Register2Indirect - src: Register(15, 8, []) dst: Indirect(6, -88, 8)
        Register2Register - src: Register(3, 8, [-120]) dst: Register(3, 8, [-120])
        Register2Register - src: Register(14, 8, [-112]) dst: Register(15, 8, [-96])
        Register2Register - src: Register(12, 8, [-104]) dst: Register(14, 8, [-112])
        Register2Register - src: Register(13, 8, [-96]) dst: Register(12, 8, [-104])
        Register2Indirect - src: Register(2, 8, [-88]) dst: Indirect(6, -80, 8)
        Indirect2Register - src: Indirect(6, -80, 8) dst: Register(13, 8, [])
        Indirect2Indirect - src: Indirect(6, -72, 8) dst: Indirect(6, -72, 8)
        Indirect2Indirect - src: Indirect(6, -64, 8) dst: Indirect(6, -64, 8)
        and 0
        or 5
        lshr 2
        ashr 2

        test lang_tests::simple_binop.c ... [0m[31mFAILED[0m

        failures:

        ---- lang_tests::simple_binop.c status ----
        Exited due to signal: 11

        ---- lang_tests::simple_binop.c stderr ----

        Literal text:
          |yk-jit-event: start-tracing
          |and 0
          |or 5
          |lshr 2
          |ashr 2
          |

        Pattern (error at line 6):
          ...
          |or 5
          |lshr 2
          |ashr 2
        >> |ashr2 -2
          |xor 5
          |xor2 -5
          |shl 8
          ...

        Text (error at line 6):
          ...
          |or 5
          |lshr 2
          |ashr 2
        >>

        ---- lang_tests::simple_binop.c stdout ----

        Allocated buffer of size 32 at 0x20722ab0 for direction OptToUnopt
        Using buffer at 0x20722ab0 for direction OptToUnopt
        Transition: OptToUnopt
        src_rbp: 0x7ffc61d40d00, reg_store: 0x7ffc61d40c10, src_frame_size: 0x80, dst_frame_size: 0x80, rbp_offset_reg_store: 0xf0
        Register2Indirect - src: Register(15, 8, []) dst: Indirect(6, -88, 8)
        Register2Register - src: Register(3, 8, [-120]) dst: Register(3, 8, [-120])
        Register2Register - src: Register(14, 8, [-112]) dst: Register(15, 8, [-96])
        Register2Register - src: Register(12, 8, [-104]) dst: Register(14, 8, [-112])
        Register2Register - src: Register(13, 8, [-96]) dst: Register(12, 8, [-104])
        Register2Indirect - src: Register(2, 8, [-88]) dst: Indirect(6, -80, 8)
        Indirect2Register - src: Indirect(6, -80, 8) dst: Register(13, 8, [])
        Indirect2Indirect - src: Indirect(6, -72, 8) dst: Indirect(6, -72, 8)
        Indirect2Indirect - src: Indirect(6, -64, 8) dst: Indirect(6, -64, 8)

        failures:
            lang_tests::simple_binop.c

        test result: [0m[31mFAILED[0m. 0 passed; 1 failed; 0 ignored; 0 measured; 155 filtered out

        [1m[31merror[0m[1m:[0m test failed, to rerun pass `-p tests --test c_tests`

        Caused by:
          process didn't exit successfully: `/home/pd/yk-fork/target/debug/deps/c_tests-e347ea94dd11cb54 '::simple_binop.c' --nocapture` (exit status: 1)
        Shared connection to bencher16.soft-dev.org closed.
        CompletedProcess(args=['ssh', '-t', 'bencher16.soft-dev.org', 'cd', 'yk-fork', '&&', 'CP_PRINT_MACHINE_CODE=1', 'CP_PATCHPOINT=1', 'CP_VERBOSE=1', 'RUST_BACKTRACE=1', 'YKB_TRACER=swt', '/home/pd/.cargo/bin/cargo', 'test', '::simple_binop.c', '--', '--nocapture'], returncode=1)


Source live vars:
  [0]: LiveVar { locs: [Direct(6, -56, 8)] }
  [1]: LiveVar { locs: [Direct(6, -48, 8)] }
  [2]: LiveVar { locs: [Register(15, 8, [])] }
  [3]: LiveVar { locs: [Register(3, 8, [-120])] }
  [4]: LiveVar { locs: [Register(14, 8, [-112])] }
  [5]: LiveVar { locs: [Register(12, 8, [-104])] }
  [6]: LiveVar { locs: [Register(13, 8, [-96])] }
  [7]: LiveVar { locs: [Register(2, 8, [-88])] }
  [8]: LiveVar { locs: [Indirect(6, -80, 8)] }
    Value at rbp+-80 = 0x7ffff6ed7038 (i64)
  [9]: LiveVar { locs: [Indirect(6, -72, 8)] }
    Value at rbp+-72 = 0x7ffff6ed703c (i64)
  [10]: LiveVar { locs: [Indirect(6, -64, 8)] }
    Value at rbp+-64 = 0x7ffff6ed7040 (i64)
Destination live vars:
  [0]: LiveVar { locs: [Direct(6, -56, 8)] }
  [1]: LiveVar { locs: [Direct(6, -48, 8)] }
  [2]: LiveVar { locs: [Indirect(6, -88, 8)] }
    Value at rbp+-88 = 0x7ffff6ed7034 (i64)
  [3]: LiveVar { locs: [Register(3, 8, [-120])] }
  [4]: LiveVar { locs: [Register(15, 8, [-96])] }
  [5]: LiveVar { locs: [Register(14, 8, [-112])] }
  [6]: LiveVar { locs: [Register(12, 8, [-104])] }
  [7]: LiveVar { locs: [Indirect(6, -80, 8)] }
    Value at rbp+-80 = 0x7ffff6ed7038 (i64)
  [8]: LiveVar { locs: [Register(13, 8, [])] }
  [9]: LiveVar { locs: [Indirect(6, -72, 8)] }
    Value at rbp+-72 = 0x7ffff6ed703c (i64)
  [10]: LiveVar { locs: [Indirect(6, -64, 8)] }
    Value at rbp+-64 = 0x7ffff6ed7040 (i64)

Register2Indirect - src: Register(15, 8, []) dst: Indirect(6, -88, 8)
Register2Register - src: Register(3, 8, [-120]) dst: Register(3, 8, [-120])
Register2Register - src: Register(14, 8, [-112]) dst: Register(15, 8, [-96])
Register2Register - src: Register(12, 8, [-104]) dst: Register(14, 8, [-112])
Register2Register - src: Register(13, 8, [-96]) dst: Register(12, 8, [-104])
Register2Indirect - src: Register(2, 8, [-88]) dst: Indirect(6, -80, 8)

Indirect2Register - src: Indirect(6, -80, 8) dst: Register(13, 8, [])
Indirect2Indirect - src: Indirect(6, -72, 8) dst: Indirect(6, -72, 8)
Indirect2Indirect - src: Indirect(6, -64, 8) dst: Indirect(6, -64, 8)







> Register2Indirect - src: Register(15, 8, []) dst: Indirect(6, -88, 8)
              0x00007ffff7ffa053  ? mov    rax,QWORD PTR [rbp-0xf0]
              0x00007ffff7ffa05a  ? mov    QWORD PTR [rbp-0x58],rax

> Register2Register - src: Register(3, 8, [-120]) dst: Register(3, 8, [-120])
              0x00007ffff7ffa061  ? mov    rax,QWORD PTR [rbp-0xa0]
              0x00007ffff7ffa068  ? mov    QWORD PTR [rbp-0x78],rax
              0x00007ffff7ffa06f  ? mov    rbx,QWORD PTR [rbp-0xa0]
> Register2Register - src: Register(14, 8, [-112]) dst: Register(15, 8, [-96])
              0x00007ffff7ffa076  ? mov    rax,QWORD PTR [rbp-0xe8]
              0x00007ffff7ffa07d  ? mov    QWORD PTR [rbp-0x60],rax
              0x00007ffff7ffa084  ? mov    r15,QWORD PTR [rbp-0xe8]
> Register2Register - src: Register(12, 8, [-104]) dst: Register(14, 8, [-112])
              0x00007ffff7ffa08b  ? mov    rax,QWORD PTR [rbp-0xd8]
              0x00007ffff7ffa092  ? mov    QWORD PTR [rbp-0x70],rax
              0x00007ffff7ffa099  ? mov    r14,QWORD PTR [rbp-0xd8]

> Register2Register - src: Register(13, 8, [-96]) dst: Register(12, 8, [-104])
              0x00007ffff7ffa0a0  ? mov    rax,QWORD PTR [rbp-0xe0]
              0x00007ffff7ffa0a7  ? mov    QWORD PTR [rbp-0x68],rax
              0x00007ffff7ffa0ae  ? mov    r12,QWORD PTR [rbp-0xe0]
> Register2Indirect - src: Register(2, 8, [-88]) dst: Indirect(6, -80, 8)
              0x00007ffff7ffa0b5  ? mov    rax,QWORD PTR [rbp-0x98]     <--------------------- rbp-0x98 hold garbage
              0x00007ffff7ffa0bc  ? mov    QWORD PTR [rbp-0x50],rax 
> Indirect2Register - src: Indirect(6, -80, 8) dst: Register(13, 8, [])
              0x00007ffff7ffa0c3  ? movabs rax,0x208af0
              0x00007ffff7ffa0cd  ? mov    r13,QWORD PTR [rax+riz*1+0x0]
>  Indirect2Indirect - src: Indirect(6, -72, 8) dst: Indirect(6, -72, 8)
              0x00007ffff7ffa0d5  ? movabs rax,0x208af0
              0x00007ffff7ffa0df  ? mov    rcx,QWORD PTR [rax+riz*1+0x8]
              0x00007ffff7ffa0e7  ? mov    QWORD PTR [rbp-0x48],rcx
> Indirect2Indirect - src: Indirect(6, -64, 8) dst: Indirect(6, -64, 8)
              0x00007ffff7ffa0ee  ? movabs rax,0x208af0
              0x00007ffff7ffa0f8  ? mov    rcx,QWORD PTR [rax+riz*1+0x10]
              0x00007ffff7ffa100  ? mov    QWORD PTR [rbp-0x40],rcx


>>> x/gx $r15 (store time)
0x7ffff6ed7020:	0x0000000000000004
  >>> x/gx $rbp-0xf0 (transition time)
  0x7fffffffcb90:	0x00007ffff6ed7020



>>> x/gx $rbx (store time)
0x7ffff6ed7024:	0x0000000000000000

  >>> x/gx $rbp-0xa0 (transition time)
  0x7fffffffcbe0:	0x00007ffff6ed7024
  >>> x/gx 0x00007ffff6ed7024 (transition time)
  0x7ffff6ed7024:	0x0000000000000000

>>> x/gx $r14 (store time)
0x7ffff6ed7028:	0x0000000000000000
  >>> x/gx $rbp-0xe8 (transition time)
  0x7fffffffcb98:	0x00007ffff6ed7028
  >>> x/gx 0x00007ffff6ed7028 (transition time)
  0x7ffff6ed7028:	0x0000000000000000

>>> x/gx $r12 (store time)
0x7ffff6ed702c:	0x0000000000000000
  >>> x/gx $rbp-0xd8 (transition time)
  0x7fffffffcba8:	0x00007ffff6ed702c
  >>> x/gx 0x00007ffff6ed702c (transition time)
  0x7ffff6ed702c:	0x0000000000000000

>>> x/gx $r13 (store time)
0x7ffff6ed7030:	0x0000000000000000
  >>> x/gx $rbp-0xe0 (transition time)
  0x7fffffffcba0:	0x00007ffff6ed7030
  >>> x/gx 0x00007ffff6ed7030 (transition time)
  0x7ffff6ed7030:	0x0000000000000000

>>> x/gx $rbx (store time)
0x7ffff6ed7024:	0x0000000000000000
>>> x/gx $rsp (store time)
0x7fffffffcbf0:	0x00007ffff7fba0fe
