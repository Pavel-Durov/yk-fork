## Summary

In opt version mt varible is stored as `rbp-88`, which is the extra location of the RAX register.
However since opt code is in no need of RAX register, it is killed, which mean that the additional locations are not set in the transition.
The unopt code expects the mt varible at `rbp-96` which is never set by the transition.
This results in  null pointer exception.

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
                    fi#0: size=8, align=8, at location [SP-56]
                    fi#1: size=8, align=8, at location [SP-96]
                    fi#2: size=8, align=8, at location [SP-88]
                    fi#3: size=8, align=8, at location [SP-80]
                    fi#4: size=8, align=8, at location [SP-64]
                    fi#5: size=8, align=8, at location [SP-72]
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
                    $rsp = frame-setup SUB64ri32 $rsp(tied-def 0), 56, implicit-def dead $eflags
                    CFI_INSTRUCTION offset $rbx, -56
                    CFI_INSTRUCTION offset $r12, -48
                    CFI_INSTRUCTION offset $r13, -40
                    CFI_INSTRUCTION offset $r14, -32
                    CFI_INSTRUCTION offset $r15, -24
                    KILL $rsi
                    KILL $edi
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags
                    $esi = XOR32rr undef $esi(tied-def 0), undef $esi, implicit-def dead $eflags
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, implicit-def $rsp, implicit-def $ssp
                    $edi = MOV32ri 1000000, implicit-def $rdi
                    CALL64pcrel32 target-flags(x86-plt) @malloc, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit-def $rsp, implicit-def $ssp, implicit-def $rax
                    $r14 = MOV64rr $rax
                    $rax = MOV64rr $r14
                    renamable $rax = ADD64ri32 killed renamable $rax(tied-def 0), 32, implicit-def dead $eflags
                    renamable $rcx = MOV64rm $rip, 1, $noreg, target-flags(x86-gottpoff) @shadowstack_0, $noreg :: (load (s64) from got)
                    MOV64mr killed renamable $rcx, 1, $noreg, 0, $fs, killed renamable $rax :: (store (s64) into @shadowstack_0)
                    $r13 = MOV64rr $r14
                    renamable $r13 = ADD64ri32 killed renamable $r13(tied-def 0), 4, implicit-def dead $eflags
                    $r15 = MOV64rr $r14
                    renamable $r15 = ADD64ri32 killed renamable $r15(tied-def 0), 8, implicit-def dead $eflags
                    $rax = MOV64rr $r14
                    renamable $rax = ADD64ri32 killed renamable $rax(tied-def 0), 16, implicit-def dead $eflags
                    MOV64mr $rbp, 1, $noreg, -56, $noreg, killed renamable $rax :: (store (s64) into %stack.4)
                    $r12 = MOV64rr $r14
                    renamable $r12 = ADD64ri32 killed renamable $r12(tied-def 0), 24, implicit-def dead $eflags
                    JMP_1 %bb.1, debug-location !76; c/udiv.c:0

                bb.1 (%ir-block.11, bb_id 1):
                    ; predecessors: %bb.0
                    successors: %bb.2
                    liveins: $r12, $r13, $r14, $r15
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !77; c/udiv.c:71:14
                    $esi = MOV32ri 1, debug-location !77; c/udiv.c:71:14
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !77; c/udiv.c:71:14
                    DBG_VALUE $noreg, $noreg, !"argv", !DIExpression(), debug-location !76; c/udiv.c:0 line no:70
                    DBG_VALUE $noreg, $noreg, !"argc", !DIExpression(), debug-location !76; c/udiv.c:0 line no:70
                    dead $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, implicit-def $rdi, debug-location !77; c/udiv.c:71:14
> $rax = yk_mt_new
                    CALL64pcrel32 @yk_mt_new, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit-def $rax, debug-location !77; c/udiv.c:71:14
> $rbp = $rax
                    $rbx = MOV64rr $rax, debug-location !77; c/udiv.c:71:14
                    JMP_1 %bb.2, debug-location !76; c/udiv.c:0

                bb.2 (%ir-block.13, bb_id 2):
                    ; predecessors: %bb.1
                    successors: %bb.3
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !78; c/udiv.c:72:3
                    $esi = MOV32ri 2, debug-location !78; c/udiv.c:72:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !78; c/udiv.c:72:3
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $rdi = MOV64rr $rbx, debug-location !78; c/udiv.c:72:3
                    $esi = XOR32rr undef $esi(tied-def 0), undef $esi, implicit-def dead $eflags, debug-location !78; c/udiv.c:72:3
                    CALL64pcrel32 @yk_mt_hot_threshold_set, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit $esi, debug-location !78; c/udiv.c:72:3
                    JMP_1 %bb.3, debug-location !79; c/udiv.c:73:3

                bb.3 (%ir-block.14, bb_id 3):
                    ; predecessors: %bb.2
                    successors: %bb.4
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    MOV64mr $rbp, 1, $noreg, -72, $noreg, killed renamable $r15 :: (store (s64) into %stack.3)
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !79; c/udiv.c:73:3
                    $esi = MOV32ri 3, debug-location !79; c/udiv.c:73:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !79; c/udiv.c:73:3
                    JMP_1 %bb.4, debug-location !80; c/udiv.c:73:14

                bb.4 (%ir-block.15, bb_id 4):
                    ; predecessors: %bb.3
                    successors: %bb.5
                    liveins: $rbx, $r12, $r13, $r14
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !81; c/udiv.c:73:20
                    $esi = MOV32ri 4, debug-location !81; c/udiv.c:73:20
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !81; c/udiv.c:73:20
                    CALL64pcrel32 @yk_location_new, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit-def $rax, debug-location !81; c/udiv.c:73:20
                    $r15 = MOV64rr $rax, debug-location !81; c/udiv.c:73:20
                    JMP_1 %bb.5, debug-location !81; c/udiv.c:73:20

                bb.5 (%ir-block.17, bb_id 5):
                    ; predecessors: %bb.4
                    successors: %bb.6
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !81; c/udiv.c:73:20
                    $esi = MOV32ri 5, debug-location !81; c/udiv.c:73:20
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !81; c/udiv.c:73:20
                    MOV64mr $rbp, 1, $noreg, -48, $noreg, killed renamable $r15, debug-location !81 :: (store (s64) into %ir.5); c/udiv.c:73:20
                    JMP_1 %bb.6, debug-location !76; c/udiv.c:0

                bb.6 (%ir-block.18, bb_id 6):
                    ; predecessors: %bb.5
                    successors: %bb.7
                    liveins: $rbx, $r12, $r13, $r14
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !83; c/udiv.c:75:7
                    $esi = MOV32ri 6, debug-location !83; c/udiv.c:75:7
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !83; c/udiv.c:75:7
                    DBG_VALUE 4, 0, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    MOV32mi renamable $r14, 1, $noreg, 0, $noreg, 4, debug-location !83 :: (store (s32) into %ir.6, !tbaa !84); c/udiv.c:75:7
                    renamable $r15 = MOV64rm $rbp, 1, $noreg, -72, $noreg :: (load (s64) from %stack.3)
                    JMP_1 %bb.7, debug-location !76; c/udiv.c:0

                bb.7 (%ir-block.19, bb_id 7):
                    ; predecessors: %bb.6
                    successors: %bb.8
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !89; c/udiv.c:76:12
                    $esi = MOV32ri 7, debug-location !89; c/udiv.c:76:12
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !89; c/udiv.c:76:12
                    DBG_VALUE 65535, 0, !"num1", !DIExpression(), debug-location !76; c/udiv.c:0 line no:76
                    MOV16mi renamable $r13, 1, $noreg, 0, $noreg, -1, debug-location !89 :: (store (s16) into %ir.7, !tbaa !90); c/udiv.c:76:12
                    JMP_1 %bb.8, debug-location !76; c/udiv.c:0

                bb.8 (%ir-block.20, bb_id 8):
                    ; predecessors: %bb.7
                    successors: %bb.9
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE 65535, $noreg, !"num1", !DIExpression(), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !93; c/udiv.c:77:12
                    $esi = MOV32ri 8, debug-location !93; c/udiv.c:77:12
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !93; c/udiv.c:77:12
                    DBG_VALUE 2147483647, 0, !"num2", !DIExpression(), debug-location !76; c/udiv.c:0 line no:77
                    MOV32mi renamable $r15, 1, $noreg, 0, $noreg, 2147483647, debug-location !93 :: (store (s32) into %ir.8, !tbaa !84); c/udiv.c:77:12
                    JMP_1 %bb.9, debug-location !76; c/udiv.c:0

                bb.9 (%ir-block.21, bb_id 9):
                    ; predecessors: %bb.8
                    successors: %bb.10
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE 2147483647, $noreg, !"num2", !DIExpression(), debug-location !76; c/udiv.c:0 line no:77
                    DBG_VALUE 65535, $noreg, !"num1", !DIExpression(), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !95; c/udiv.c:78:12
                    $esi = MOV32ri 9, debug-location !95; c/udiv.c:78:12
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !95; c/udiv.c:78:12
                    DBG_VALUE 4294967294, 0, !"num3", !DIExpression(), debug-location !76; c/udiv.c:0 line no:78
                    $eax = MOV32ri 4294967294, implicit-def $rax, debug-location !95; c/udiv.c:78:12
                    renamable $rcx = MOV64rm $rbp, 1, $noreg, -56, $noreg :: (load (s64) from %stack.4)
                    MOV64mr killed renamable $rcx, 1, $noreg, 0, $noreg, killed renamable $rax, debug-location !95 :: (store (s64) into %ir.9, !tbaa !96); c/udiv.c:78:12
                    JMP_1 %bb.10, debug-location !76; c/udiv.c:0

                bb.10 (%ir-block.22, bb_id 10):
                    ; predecessors: %bb.9
                    successors: %bb.11
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE 4294967294, $noreg, !"num3", !DIExpression(), debug-location !76; c/udiv.c:0 line no:78
                    DBG_VALUE 2147483647, $noreg, !"num2", !DIExpression(), debug-location !76; c/udiv.c:0 line no:77
                    DBG_VALUE 65535, $noreg, !"num1", !DIExpression(), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !99; c/udiv.c:79:11
                    $esi = MOV32ri 10, debug-location !99; c/udiv.c:79:11
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !99; c/udiv.c:79:11
                    DBG_VALUE 127, 0, !"num4", !DIExpression(), debug-location !76; c/udiv.c:0 line no:79
                    MOV8mi renamable $r12, 1, $noreg, 0, $noreg, 127, debug-location !99 :: (store (s8) into %ir.10, !tbaa !100); c/udiv.c:79:11
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], renamable $r13, 1, $noreg, 0, $noreg, $1:[mem:m], renamable $r13, 1, $noreg, 0, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !102, debug-location !101; c/udiv.c:80:3
                    JMP_1 %bb.11, debug-location !103; c/udiv.c:81:3

                bb.11 (%ir-block.23, bb_id 11):
                    ; predecessors: %bb.10
                    successors: %bb.12
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 127, $noreg, !"num4", !DIExpression(), debug-location !76; c/udiv.c:0 line no:79
                    DBG_VALUE 4294967294, $noreg, !"num3", !DIExpression(), debug-location !76; c/udiv.c:0 line no:78
                    DBG_VALUE 2147483647, $noreg, !"num2", !DIExpression(), debug-location !76; c/udiv.c:0 line no:77
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !103; c/udiv.c:81:3
                    $esi = MOV32ri 11, debug-location !103; c/udiv.c:81:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !103; c/udiv.c:81:3
                    dead renamable $eax = MOV32rm renamable $r15, 1, $noreg, 0, $noreg, debug-location !103 :: (load (s32) from %ir.8, !tbaa !84); c/udiv.c:81:3
                    DBG_VALUE $rbp, 0, !"num2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:77 indirect
                    INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], renamable $r15, 1, $noreg, 0, $noreg, $1:[mem:m], renamable $r15, 1, $noreg, 0, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !104, debug-location !103; c/udiv.c:81:3
                    JMP_1 %bb.12, debug-location !105; c/udiv.c:82:3

                bb.12 (%ir-block.25, bb_id 12):
                    ; predecessors: %bb.11
                    successors: %bb.13
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $rbp, 0, !"num2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:77 indirect
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 127, $noreg, !"num4", !DIExpression(), debug-location !76; c/udiv.c:0 line no:79
                    DBG_VALUE 4294967294, $noreg, !"num3", !DIExpression(), debug-location !76; c/udiv.c:0 line no:78
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !105; c/udiv.c:82:3
                    $esi = MOV32ri 12, debug-location !105; c/udiv.c:82:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !105; c/udiv.c:82:3
                    renamable $rcx = MOV64rm $rbp, 1, $noreg, -56, $noreg :: (load (s64) from %stack.4)
                    dead renamable $rax = MOV64rm renamable $rcx, 1, $noreg, 0, $noreg, debug-location !105 :: (load (s64) from %ir.9, !tbaa !96); c/udiv.c:82:3
                    DBG_VALUE $rbp, 0, !"num3", !DIExpression(DW_OP_constu, 56, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:78 indirect
                    INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], killed renamable $rcx, 1, $noreg, 0, $noreg, $1:[mem:m], renamable $rcx, 1, $noreg, 0, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !106, debug-location !105; c/udiv.c:82:3
                    JMP_1 %bb.13, debug-location !107; c/udiv.c:83:3

                bb.13 (%ir-block.27, bb_id 13):
                    ; predecessors: %bb.12
                    successors: %bb.14
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $rbp, 0, !"num3", !DIExpression(DW_OP_constu, 56, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:78 indirect
                    DBG_VALUE $rbp, 0, !"num2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:77 indirect
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 127, $noreg, !"num4", !DIExpression(), debug-location !76; c/udiv.c:0 line no:79
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !107; c/udiv.c:83:3
                    $esi = MOV32ri 13, debug-location !107; c/udiv.c:83:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !107; c/udiv.c:83:3
                    dead renamable $al = MOV8rm renamable $r12, 1, $noreg, 0, $noreg, debug-location !107 :: (load (s8) from %ir.10, !tbaa !100); c/udiv.c:83:3
                    DBG_VALUE $r12, $noreg, !"num4", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:79
                    INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], renamable $r12, 1, $noreg, 0, $noreg, $1:[mem:m], renamable $r12, 1, $noreg, 0, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !108, debug-location !107; c/udiv.c:83:3
                    JMP_1 %bb.14, debug-location !109; c/udiv.c:84:3

                bb.14 (%ir-block.29, bb_id 14):
                    ; predecessors: %bb.13
                    successors: %bb.15
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $r12, $noreg, !"num4", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:79
                    DBG_VALUE $rbp, 0, !"num3", !DIExpression(DW_OP_constu, 56, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:78 indirect
                    DBG_VALUE $rbp, 0, !"num2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:77 indirect
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !109; c/udiv.c:84:3
                    $esi = MOV32ri 14, debug-location !109; c/udiv.c:84:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !109; c/udiv.c:84:3
                    dead renamable $rax = MOV64rm $rbp, 1, $noreg, -48, $noreg, debug-location !109 :: (load (s64) from %ir.5); c/udiv.c:84:3
                    INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], $rbp, 1, $noreg, -48, $noreg, $1:[mem:m], $rbp, 1, $noreg, -48, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !110, debug-location !109; c/udiv.c:84:3
                    JMP_1 %bb.15, debug-location !111; c/udiv.c:85:3

                bb.15 (%ir-block.31, bb_id 15):
                    ; predecessors: %bb.14
                    successors: %bb.16
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $r12, $noreg, !"num4", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:79
                    DBG_VALUE $rbp, 0, !"num3", !DIExpression(DW_OP_constu, 56, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:78 indirect
                    DBG_VALUE $rbp, 0, !"num2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:77 indirect
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE 4, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !111; c/udiv.c:85:3
                    $esi = MOV32ri 15, debug-location !111; c/udiv.c:85:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !111; c/udiv.c:85:3
                    dead renamable $eax = MOV32rm renamable $r14, 1, $noreg, 0, $noreg, debug-location !111 :: (load (s32) from %ir.6, !tbaa !84); c/udiv.c:85:3
                    DBG_VALUE $r14, $noreg, !"i", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:75
                    INLINEASM &"" [sideeffect] [mayload] [maystore] [attdialect], $0:[mem:m], renamable $r14, 1, $noreg, 0, $noreg, $1:[mem:m], renamable $r14, 1, $noreg, 0, $noreg, $2:[clobber], implicit-def dead early-clobber $df, $3:[clobber], implicit-def early-clobber $fpsw, $4:[clobber], implicit-def dead early-clobber $eflags, !112, debug-location !111; c/udiv.c:85:3
                    JMP_1 %bb.16, debug-location !113; c/udiv.c:86:10

                bb.16 (%ir-block.33, bb_id 16):
                    ; predecessors: %bb.15
                    successors: %bb.17, %bb.25
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $r14, $noreg, !"i", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:75
                    DBG_VALUE $r12, $noreg, !"num4", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:79
                    DBG_VALUE $rbp, 0, !"num3", !DIExpression(DW_OP_constu, 56, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:78 indirect
                    DBG_VALUE $rbp, 0, !"num2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:77 indirect
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !113; c/udiv.c:86:10
                    $esi = MOV32ri 16, debug-location !113; c/udiv.c:86:10
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !113; c/udiv.c:86:10
                    STACKMAP 2, 0, 0, $rbp, -48, 3, renamable $r14, 3, renamable $r13, 3, renamable $r15, 3, 1, 8, $rbp, -56, 3, renamable $r12, 3, renamable $rbx, 3, renamable $al, 3, implicit-def dead early-clobber $r11, debug-location !115; c/udiv.c:86:3
                    renamable $eax = MOV32rm renamable $r14, 1, $noreg, 0, $noreg, debug-location !113 :: (load (s32) from %ir.6, !tbaa !84); c/udiv.c:86:10
                    DBG_VALUE $eax, $noreg, !"i", !DIExpression(), debug-location !76; c/udiv.c:0 line no:75
                    CMP32ri killed renamable $eax, 0, implicit-def $eflags, debug-location !114; c/udiv.c:86:12
                    renamable $al = SETCCr 15, implicit killed $eflags, debug-location !114; c/udiv.c:86:12
                    TEST8ri killed renamable $al, 1, implicit-def $eflags, debug-location !115; c/udiv.c:86:3
                    JCC_1 %bb.17, 5, implicit killed $eflags, debug-location !115; c/udiv.c:86:3
                    JMP_1 %bb.25, debug-location !115; c/udiv.c:86:3

                bb.17 (%ir-block.36, bb_id 17):
                    ; predecessors: %bb.16
                    successors: %bb.18
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $r12, $noreg, !"num4", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:79
                    DBG_VALUE $rbp, 0, !"num3", !DIExpression(DW_OP_constu, 56, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:78 indirect
                    DBG_VALUE $rbp, 0, !"num2", !DIExpression(DW_OP_constu, 72, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:77 indirect
                    DBG_VALUE $r13, $noreg, !"num1", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:76
                    DBG_VALUE $rbx, $noreg, !"mt", !DIExpression(), debug-location !76; c/udiv.c:0 line no:71
                    MOV64mr $rbp, 1, $noreg, -64, $noreg, killed renamable $r12 :: (store (s64) into %stack.5)
                    DBG_VALUE $rbp, 0, !"num4", !DIExpression(DW_OP_constu, 64, DW_OP_minus, DW_OP_deref), debug-location !76; c/udiv.c:0 line no:79 indirect
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !115; c/udiv.c:86:3
                    $esi = MOV32ri 17, debug-location !115; c/udiv.c:86:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !115; c/udiv.c:86:3

> $rbp-88 = mt
                    MOV64mr $rbp, 1, $noreg, -88, $noreg, renamable $rbx :: (store (s64) into %stack.1)
                    MOV64mr $rbp, 1, $noreg, -80, $noreg, renamable $r13 :: (store (s64) into %stack.2)
                    JMP_1 %bb.18, debug-location !115; c/udiv.c:86:3

                bb.18 (%ir-block.37, bb_id 18):
                    ; predecessors: %bb.17, %bb.35
                    successors: %bb.19
                    liveins: $rbx, $r13, $r14, $r15
                    $edi = XOR32rr undef $edi(tied-def 0), undef $edi, implicit-def dead $eflags, debug-location !116; c/udiv.c:87:5
                    $esi = MOV32ri 18, debug-location !116; c/udiv.c:87:5
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock_dummy, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !116; c/udiv.c:87:5
                    renamable $rsi = LEA64r $rbp, 1, $noreg, -48, $noreg, debug-location !116; c/udiv.c:87:5
                    $  = MOV64rr $rbx, debug-location !116; c/udiv.c:87:5
                    dead $edx = XOR32rr undef $edx(tied-def 0), undef $edx, implicit-def dead $eflags, implicit-def $rdx, debug-location !116; c/udiv.c:87:5
                    renamable $r12 = MOV64rm $rbp, 1, $noreg, -56, $noreg :: (load (s64) from %stack.4)
                    $rax = MOV64rr killed $rbx
                    renamable $rbx = MOV64rm $rbp, 1, $noreg, -64, $noreg :: (load (s64) from %stack.5)
                    DBG_VALUE $rbx, $noreg, !"num4", !DIExpression(DW_OP_deref), debug-location !76; c/udiv.c:0 line no:79

> call __ykrt_control_point
> $rdi = mt
> $rax = mt
> killed renamable $rax

                    PATCHPOINT 0, 13, @__ykrt_control_point, 3, 0, $rdi, $rsi, $rdx, 0, $rbp, -48, 3, renamable $r14, 3, renamable $r13, 3, renamable $r15, 3, renamable $r12, 3, renamable $rbx, 3, killed renamable $rax, 3, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, liveout($bh, $bl, $bx, $ebx, $hbx, $rbx, $r12, $r13, $r14, $r15, $r12b, $r13b, $r14b, $r15b, $r12bh, $r13bh, $r14bh, $r15bh, $r12d, $r13d, $r14d, $r15d, $r12w, $r13w, $r14w, $r15w, $r12wh, $r13wh, $r14wh, $r15wh), implicit-def dead early-clobber $r11, debug-location !116 :: (load (s64) from %stack.0); c/udiv.c:87:5
                    JMP_1 %bb.19, debug-location !117; c/udiv.c:88:21

                    ...


                    Function: __yk_unopt_main

                    ...

                bb.18 (%ir-block.36, bb_id 18):
                    ; predecessors: %bb.17, %bb.35
                    ...
                    PATCHPOINT 1, 13, @__ykrt_control_point, 3, 0, $rdi, $rsi, $rdx, 0, $rbp, -56, 3, renamable $r13, 3, renamable $r14, 3, renamable $r12, 3, renamable $r15, 3, renamable $rbx, 3, killed renamable $rax, 3, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, liveout($bh, $bl, $bx, $ebx, $hbx, $rbx, $r12, $r13, $r14, $r15, $r12b, $r13b, $r14b, $r15b, $r12bh, $r13bh, $r14bh, $r15bh, $r12d, $r13d, $r14d, $r15d, $r12w, $r13w, $r14w, $r15w, $r12wh, $r13wh, $r14wh, $r15wh), implicit-def dead early-clobber $r11, debug-location !273 :: (load (s64) from %stack.0); c/udiv.c:87:5
                    JMP_1 %bb.19, debug-location !274; c/udiv.c:88:21

                bb.19 (%ir-block.37, bb_id 19):
                    ; predecessors: %bb.18
                    successors: %bb.20
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    $edi = MOV32ri 13, debug-location !274; c/udiv.c:88:21
                    $esi = MOV32ri 19, debug-location !274; c/udiv.c:88:21
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !274; c/udiv.c:88:21
                    renamable $ax = MOV16rm killed renamable $r14, 1, $noreg, 0, $noreg, debug-location !274 :: (load (s16) from %ir.6, !tbaa !90); c/udiv.c:88:21
                    DBG_VALUE $ax, $noreg, !"num1", !DIExpression(), debug-location !248; c/udiv.c:0 line no:76
                    renamable $cx = MOV16ri 3
                    dead $edx = XOR32rr undef $edx(tied-def 0), undef $edx, implicit-def dead $eflags, implicit-def $dx
                    DIV16r killed renamable $cx, implicit-def $ax, implicit-def dead $dx, implicit-def dead $eflags, implicit $ax, implicit $dx
                    $cx = MOV16rr $ax
                    DBG_VALUE $cx, $noreg, !"udiv", !DIExpression(), debug-location !275; c/udiv.c:0 line no:88
                    renamable $eax = MOV32rm killed renamable $r12, 1, $noreg, 0, $noreg, debug-location !276 :: (load (s32) from %ir.7, !tbaa !84); c/udiv.c:89:22
                    DBG_VALUE $eax, $noreg, !"num2", !DIExpression(), debug-location !248; c/udiv.c:0 line no:77
                    renamable $esi = MOV32ri 3, debug-location !277; c/udiv.c:89:27
                    $edx = XOR32rr undef $edx(tied-def 0), undef $edx, implicit-def dead $eflags, debug-location !277; c/udiv.c:89:27
                    DIV32r killed renamable $esi, implicit-def $eax, implicit-def dead $edx, implicit-def dead $eflags, implicit $eax, implicit $edx, debug-location !277; c/udiv.c:89:27
                    $r12d = MOV32rr $eax, debug-location !277; c/udiv.c:89:27
                    DBG_VALUE $r12d, $noreg, !"udiv2", !DIExpression(), debug-location !275; c/udiv.c:0 line no:89
                    renamable $rax = MOV64rm killed renamable $r15, 1, $noreg, 0, $noreg, debug-location !278 :: (load (s64) from %ir.8, !tbaa !96); c/udiv.c:90:22
                    DBG_VALUE $rax, $noreg, !"num3", !DIExpression(), debug-location !248; c/udiv.c:0 line no:78
                    $esi = MOV32ri 3, implicit-def $rsi, debug-location !279; c/udiv.c:90:27
                    dead $edx = XOR32rr undef $edx(tied-def 0), undef $edx, implicit-def dead $eflags, implicit-def $rdx, debug-location !279; c/udiv.c:90:27
                    DIV64r killed renamable $rsi, implicit-def $rax, implicit-def dead $rdx, implicit-def dead $eflags, implicit $rax, implicit $rdx, debug-location !279; c/udiv.c:90:27
                    $r14 = MOV64rr $rax, debug-location !279; c/udiv.c:90:27
                    DBG_VALUE $r14, $noreg, !"udiv3", !DIExpression(), debug-location !275; c/udiv.c:0 line no:90
                    renamable $al = MOV8rm renamable $rbx, 1, $noreg, 0, $noreg, debug-location !280 :: (load (s8) from %ir.9, !tbaa !100); c/udiv.c:91:21
                    DBG_VALUE $al, $noreg, !"num4", !DIExpression(), debug-location !248; c/udiv.c:0 line no:79
                    renamable $dl = MOV8ri 3
                    $ax = MOVZX16rr8 killed renamable $al
                    DIV8r killed renamable $dl, implicit-def $al, implicit-def dead $ah, implicit-def dead $eflags, implicit $ax
                    MOV8mr $rbp, 1, $noreg, -41, $noreg, $al :: (store (s8) into %stack.6)
                    DBG_VALUE $rbp, 0, !"udiv4", !DIExpression(DW_OP_constu, 41, DW_OP_minus), debug-location !275; c/udiv.c:0 line no:91 indirect
                    renamable $rdi = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !281 :: (load (s64) from @stderr, !tbaa !125); c/udiv.c:92:13
                    renamable $edx = MOVZX32rr16 killed renamable $cx, debug-location !282; c/udiv.c:92:35
                    renamable $rsi = MOV64ri @.str, debug-location !283; c/udiv.c:92:5
                    $al = MOV8ri 0, debug-location !283; c/udiv.c:92:5
                    CALL64pcrel32 @fprintf, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $al, implicit $rdi, implicit $rsi, implicit $edx, implicit-def $eax, debug-location !283; c/udiv.c:92:5
                    $r15 = MOV64rr killed $rbx
                    JMP_1 %bb.20, debug-location !284; c/udiv.c:93:13

                bb.20 (%ir-block.49, bb_id 20):
                    ; predecessors: %bb.19
                    successors: %bb.21
                    liveins: $r13, $r14, $r15, $r12d
                    DBG_VALUE $rbp, 0, !"udiv4", !DIExpression(DW_OP_constu, 41, DW_OP_minus), debug-location !275; c/udiv.c:0 line no:91 indirect
                    DBG_VALUE $r14, $noreg, !"udiv3", !DIExpression(), debug-location !275; c/udiv.c:0 line no:90
                    DBG_VALUE $r12d, $noreg, !"udiv2", !DIExpression(), debug-location !275; c/udiv.c:0 line no:89
                    $edi = MOV32ri 13, debug-location !284; c/udiv.c:93:13
                    $esi = MOV32ri 20, debug-location !284; c/udiv.c:93:13
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !284; c/udiv.c:93:13
                    renamable $rdi = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !284 :: (load (s64) from @stderr, !tbaa !125); c/udiv.c:93:13
                    renamable $rsi = MOV64ri @.str.1, debug-location !285; c/udiv.c:93:5
                    $edx = MOV32rr killed $r12d, debug-location !285; c/udiv.c:93:5
                    $al = MOV8ri 0, debug-location !285; c/udiv.c:93:5
                    CALL64pcrel32 @fprintf, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $al, implicit $rdi, implicit $rsi, implicit $edx, implicit-def $eax, debug-location !285; c/udiv.c:93:5
                    JMP_1 %bb.21, debug-location !286; c/udiv.c:94:13

                bb.21 (%ir-block.52, bb_id 21):
                    ; predecessors: %bb.20
                    successors: %bb.22
                    liveins: $r13, $r14, $r15
                    DBG_VALUE $rbp, 0, !"udiv4", !DIExpression(DW_OP_constu, 41, DW_OP_minus), debug-location !275; c/udiv.c:0 line no:91 indirect
                    DBG_VALUE $r14, $noreg, !"udiv3", !DIExpression(), debug-location !275; c/udiv.c:0 line no:90
                    DBG_VALUE $r12d, $noreg, !"udiv2", !DIExpression(), debug-location !275; c/udiv.c:0 line no:89
                    $edi = MOV32ri 13, debug-location !286; c/udiv.c:94:13
                    $esi = MOV32ri 21, debug-location !286; c/udiv.c:94:13
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !286; c/udiv.c:94:13
                    renamable $rdi = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !286 :: (load (s64) from @stderr, !tbaa !125); c/udiv.c:94:13
                    renamable $rsi = MOV64ri @.str.2, debug-location !287; c/udiv.c:94:5
                    $rdx = MOV64rr killed $r14, debug-location !287; c/udiv.c:94:5
                    $al = MOV8ri 0, debug-location !287; c/udiv.c:94:5
                    CALL64pcrel32 @fprintf, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $al, implicit $rdi, implicit $rsi, implicit $rdx, implicit-def $eax, debug-location !287; c/udiv.c:94:5
> rbx = $rbp-96
> but nothing set this rbp offset cause the register was killed

                    renamable $rbx = MOV64rm $rbp, 1, $noreg, -96, $noreg :: (load (s64) from %stack.1)
                    renamable $r12 = MOV64rm $rbp, 1, $noreg, -72, $noreg :: (load (s64) from %stack.4)
                    JMP_1 %bb.22, debug-location !288; c/udiv.c:95:13

                bb.22 (%ir-block.55, bb_id 22):
                    ; predecessors: %bb.21
                    successors: %bb.23
                    liveins: $rbx, $r12, $r13, $r15
                    DBG_VALUE $rbp, 0, !"udiv4", !DIExpression(DW_OP_constu, 41, DW_OP_minus), debug-location !275; c/udiv.c:0 line no:91 indirect
                    DBG_VALUE $r14, $noreg, !"udiv3", !DIExpression(), debug-location !275; c/udiv.c:0 line no:90
                    $edi = MOV32ri 13, debug-location !288; c/udiv.c:95:13
                    $esi = MOV32ri 22, debug-location !288; c/udiv.c:95:13
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !288; c/udiv.c:95:13
                    renamable $rdi = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !288 :: (load (s64) from @stderr, !tbaa !125); c/udiv.c:95:13
                    renamable $edx = MOVZX32rm8 $rbp, 1, $noreg, -41, $noreg, debug-location !289 :: (load (s8) from %stack.6); c/udiv.c:95:35
                    renamable $rsi = MOV64ri @.str.3, debug-location !290; c/udiv.c:95:5
                    $al = MOV8ri 0, debug-location !290; c/udiv.c:95:5
                    CALL64pcrel32 @fprintf, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $al, implicit $rdi, implicit $rsi, implicit $edx, implicit-def $eax, debug-location !290; c/udiv.c:95:5
                    renamable $r14 = MOV64rm $rbp, 1, $noreg, -80, $noreg :: (load (s64) from %stack.2)
                    JMP_1 %bb.23, debug-location !291; c/udiv.c:96:6

                bb.23 (%ir-block.59, bb_id 23):
                    ; predecessors: %bb.22
                    successors: %bb.35, %bb.24
                    liveins: $rbx, $r12, $r13, $r14, $r15
                    DBG_VALUE $rbp, 0, !"udiv4", !DIExpression(DW_OP_constu, 41, DW_OP_minus), debug-location !275; c/udiv.c:0 line no:91 indirect
                    $edi = MOV32ri 13, debug-location !291; c/udiv.c:96:6
                    $esi = MOV32ri 23, debug-location !291; c/udiv.c:96:6
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !291; c/udiv.c:96:6
                    renamable $eax = MOV32rm renamable $r13, 1, $noreg, 0, $noreg, debug-location !291 :: (load (s32) from %ir.5, !tbaa !84); c/udiv.c:96:6
                    DBG_VALUE $eax, $noreg, !"i", !DIExpression(), debug-location !248; c/udiv.c:0 line no:75
                    $ecx = MOV32rr $eax, debug-location !291; c/udiv.c:96:6
                    renamable $ecx = ADD32ri killed renamable $ecx(tied-def 0), -1, implicit-def dead $eflags, debug-location !291; c/udiv.c:96:6
                    DBG_VALUE $ecx, $noreg, !"i", !DIExpression(), debug-location !248; c/udiv.c:0 line no:75
                    MOV32mr renamable $r13, 1, $noreg, 0, $noreg, killed renamable $ecx, debug-location !291 :: (store (s32) into %ir.5, !tbaa !84); c/udiv.c:96:6
                    CMP32ri killed renamable $eax, 1, implicit-def $eflags, debug-location !271; c/udiv.c:86:12
                    renamable $al = SETCCr 15, implicit killed $eflags, debug-location !271; c/udiv.c:86:12
                    STACKMAP 4, 0, 0, $rbp, -56, 3, renamable $r13, 3, renamable $r14, 3, 1, 8, $rbp, -64, 3, killed renamable $r12, 3, killed renamable $r15, 3, renamable $rbx, 3, renamable $al, 3, implicit-def dead early-clobber $r11, debug-location !272 :: (load (s64) from %stack.0), (load (s64) from %stack.3); c/udiv.c:86:3
                    TEST8ri killed renamable $al, 1, implicit-def $eflags, debug-location !272; c/udiv.c:86:3
                    JCC_1 %bb.35, 5, implicit killed $eflags, debug-location !272; c/udiv.c:86:3
                    JMP_1 %bb.24, debug-location !272; c/udiv.c:86:3

                bb.24 (%ir-block.63, bb_id 24):
                    ; predecessors: %bb.23
                    successors: %bb.25
                    liveins: $rbx
                    DBG_VALUE $ecx, $noreg, !"i", !DIExpression(), debug-location !248; c/udiv.c:0 line no:75
                    $edi = MOV32ri 13, debug-location !294; c/udiv.c:98:11
                    $esi = MOV32ri 24, debug-location !294; c/udiv.c:98:11
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !294; c/udiv.c:98:11
                    JMP_1 %bb.25, debug-location !294; c/udiv.c:98:11

                bb.25 (%ir-block.64, bb_id 25):
                    ; predecessors: %bb.16, %bb.24
                    successors: %bb.26
                    liveins: $rbx
                    $edi = MOV32ri 13, debug-location !294; c/udiv.c:98:11
                    $esi = MOV32ri 25, debug-location !294; c/udiv.c:98:11
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !294; c/udiv.c:98:11
                    renamable $rcx = MOV64rm $noreg, 1, $noreg, @stderr, $noreg, debug-location !294 :: (load (s64) from @stderr, !tbaa !125); c/udiv.c:98:11
                    renamable $rdi = MOV64ri @.str.4, debug-location !295; c/udiv.c:98:3
                    $esi = MOV32ri 5, implicit-def $rsi, debug-location !295; c/udiv.c:98:3
                    $edx = MOV32ri 1, implicit-def $rdx, debug-location !295; c/udiv.c:98:3
                    CALL64pcrel32 target-flags(x86-plt) @fwrite, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, implicit $rsi, implicit $rdx, implicit $rcx, implicit-def $rax, debug-location !295; c/udiv.c:98:3
                    JMP_1 %bb.26, debug-location !296; c/udiv.c:99:3

                bb.26 (%ir-block.67, bb_id 26):
                    ; predecessors: %bb.25
                    successors: %bb.27
                    liveins: $rbx
                    $edi = MOV32ri 13, debug-location !296; c/udiv.c:99:3
                    $esi = MOV32ri 26, debug-location !296; c/udiv.c:99:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !296; c/udiv.c:99:3
                    renamable $rdi = MOV64rm $rbp, 1, $noreg, -56, $noreg, debug-location !296 :: (load (s64) from %ir.4); c/udiv.c:99:3
                    CALL64pcrel32 @yk_location_drop, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, debug-location !296; c/udiv.c:99:3
                    JMP_1 %bb.27, debug-location !297; c/udiv.c:100:3

                bb.27 (%ir-block.69, bb_id 27):
                    ; predecessors: %bb.26
                    successors: %bb.28
                    liveins: $rbx
                    $edi = MOV32ri 13, debug-location !297; c/udiv.c:100:3
                    $esi = MOV32ri 27, debug-location !297; c/udiv.c:100:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !297; c/udiv.c:100:3
                    $rdi = MOV64rr killed $rbx, debug-location !297; c/udiv.c:100:3
                    CALL64pcrel32 @yk_mt_shutdown, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $rdi, debug-location !297; c/udiv.c:100:3
                    JMP_1 %bb.28, debug-location !298; c/udiv.c:102:1

                bb.28 (%ir-block.70, bb_id 28):
                    ; predecessors: %bb.27
                    successors: %bb.29

                    $edi = MOV32ri 13, debug-location !298; c/udiv.c:102:1
                    $esi = MOV32ri 28, debug-location !298; c/udiv.c:102:1
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !298; c/udiv.c:102:1
                    JMP_1 %bb.29, debug-location !298; c/udiv.c:102:1

                bb.29 (%ir-block.71, bb_id 29):
                    ; predecessors: %bb.28
                    successors: %bb.30

                    $edi = MOV32ri 13, debug-location !298; c/udiv.c:102:1
                    $esi = MOV32ri 29, debug-location !298; c/udiv.c:102:1
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !298; c/udiv.c:102:1
                    JMP_1 %bb.30, debug-location !298; c/udiv.c:102:1

                bb.30 (%ir-block.72, bb_id 30):
                    ; predecessors: %bb.29
                    successors: %bb.31

                    $edi = MOV32ri 13, debug-location !298; c/udiv.c:102:1
                    $esi = MOV32ri 30, debug-location !298; c/udiv.c:102:1
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !298; c/udiv.c:102:1
                    JMP_1 %bb.31, debug-location !298; c/udiv.c:102:1

                bb.31 (%ir-block.73, bb_id 31):
                    ; predecessors: %bb.30
                    successors: %bb.32

                    $edi = MOV32ri 13, debug-location !298; c/udiv.c:102:1
                    $esi = MOV32ri 31, debug-location !298; c/udiv.c:102:1
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !298; c/udiv.c:102:1
                    JMP_1 %bb.32, debug-location !298; c/udiv.c:102:1

                bb.32 (%ir-block.74, bb_id 32):
                    ; predecessors: %bb.31
                    successors: %bb.33

                    $edi = MOV32ri 13, debug-location !298; c/udiv.c:102:1
                    $esi = MOV32ri 32, debug-location !298; c/udiv.c:102:1
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !298; c/udiv.c:102:1
                    JMP_1 %bb.33, debug-location !298; c/udiv.c:102:1

                bb.33 (%ir-block.75, bb_id 33):
                    ; predecessors: %bb.32
                    successors: %bb.34

                    $edi = MOV32ri 13, debug-location !298; c/udiv.c:102:1
                    $esi = MOV32ri 33, debug-location !298; c/udiv.c:102:1
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !298; c/udiv.c:102:1
                    JMP_1 %bb.34, debug-location !299; c/udiv.c:101:3

                bb.34 (%ir-block.76, bb_id 34):
                    ; predecessors: %bb.33
                    successors: %bb.37

                    $edi = MOV32ri 13, debug-location !299; c/udiv.c:101:3
                    $esi = MOV32ri 34, debug-location !299; c/udiv.c:101:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !299; c/udiv.c:101:3
                    JMP_1 %bb.37, debug-location !299; c/udiv.c:101:3

                bb.35 (%ir-block.77, bb_id 35):
                    ; predecessors: %bb.23
                    successors: %bb.18
                    liveins: $rbx, $r13, $r14
                    DBG_VALUE $ecx, $noreg, !"i", !DIExpression(), debug-location !248; c/udiv.c:0 line no:75
                    DBG_VALUE $rbp, 0, !"udiv4", !DIExpression(DW_OP_constu, 41, DW_OP_minus), debug-location !275; c/udiv.c:0 line no:91 indirect
                    $edi = MOV32ri 13
                    $esi = MOV32ri 35
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi
                    JMP_1 %bb.18

                bb.36 (%ir-block.78, bb_id 36):
                    ; predecessors: %bb.37

                    $edi = MOV32ri 13, debug-location !299; c/udiv.c:101:3
                    $esi = MOV32ri 36, debug-location !299; c/udiv.c:101:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !299; c/udiv.c:101:3
                    $eax = XOR32rr undef $eax(tied-def 0), undef $eax, implicit-def dead $eflags, debug-location !299; c/udiv.c:101:3
                    $rsp = frame-destroy ADD64ri32 $rsp(tied-def 0), 56, implicit-def dead $eflags, debug-location !299; c/udiv.c:101:3
                    $rbx = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !299; c/udiv.c:101:3
                    $r12 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !299; c/udiv.c:101:3
                    $r13 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !299; c/udiv.c:101:3
                    $r14 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !299; c/udiv.c:101:3
                    $r15 = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !299; c/udiv.c:101:3
                    $rbp = frame-destroy POP64r implicit-def $rsp, implicit $rsp, debug-location !299; c/udiv.c:101:3
                    frame-destroy CFI_INSTRUCTION def_cfa $rsp, 8, debug-location !299; c/udiv.c:101:3
                    RET64 implicit $eax, debug-location !299; c/udiv.c:101:3

                bb.37 (%ir-block.79, bb_id 37):
                    ; predecessors: %bb.34
                    successors: %bb.36

                    CFI_INSTRUCTION def_cfa $rbp, 16, debug-location !299; c/udiv.c:101:3
                    $edi = MOV32ri 13, debug-location !299; c/udiv.c:101:3
                    $esi = MOV32ri 37, debug-location !299; c/udiv.c:101:3
                    CALL64pcrel32 target-flags(x86-plt) @__yk_trace_basicblock, <regmask $bh $bl $bp $bph $bpl $bx $ebp $ebx $hbp $hbx $rbp $rbx $r12 $r13 $r14 $r15 $r12b $r13b $r14b $r15b $r12bh $r13bh $r14bh $r15bh $r12d $r13d $r14d $r15d $r12w $r13w $r14w $r15w $r12wh and 3 more...>, implicit $rsp, implicit $ssp, implicit $edi, implicit $esi, debug-location !299; c/udiv.c:101:3
                    JMP_1 %bb.36, debug-location !299; c/udiv.c:101:3
