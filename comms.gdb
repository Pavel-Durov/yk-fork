# logging
set logging file gdb_output.txt
set logging enabled on

set breakpoint pending on

# break ykrt::trace::swt::cp::debug_return_into_unopt_cp
# break ykrt::trace::swt::cp::debug_return_into_opt_cp
# break __yk_clone_main
# break ykrt::compile::jitc_yk::codegen::x64::deopt:327
# break ykrt::compile::jitc_yk::codegen::x64::deopt::__yk_deopt
break main::76

# break ykrt/src/mt.rs:460 # exec trace

# break ykcapi::__ykrt_control_point_real
# break ykrt::mt::MT::control_point
# break ykrt::mt::MT::control_point
# break ykrt::mt::MT::queue_root_compile_job
# break ykrt::trace::swt::cp::swt_module_cp_transition


break ykrt::mt::__yk_exec_trace

define print_live_vars
    p &mt
    p *&mt
    p &loc
    p *&loc
    p &i
    p *&i
    p &res
    p *&res
endbreak ykrt::mt::MT::queue_root_compile_job

define print_yk_registers
    printf "Register Values:\n"
    printf "RAX: %016x\n", $rax
    printf "RCX: %016x\n", $rcx
    printf "RBX: %016x\n", $rbx
    printf "RDI: %016x\n", $rdi
    printf "RSI: %016x\n", $rsi
    printf "R8:  %016x\n", $r8
    printf "R9:  %016x\n", $r9
    printf "R10: %016x\n", $r10
    printf "R11: %016x\n", $r11
    printf "R12: %016x\n", $r12
    printf "R13: %016x\n", $r13
    printf "R14: %016x\n", $r14
    printf "R15: %016x\n", $r15
end

define print_yk_registers_as_rsp_offset
printf "RSP Offsets and Values:\n"
    printf "RAX (0x60): %016x\n", *((unsigned long *)($rsp + 0x60))
    printf "RCX (0x58): %016x\n", *((unsigned long *)($rsp + 0x58))
    printf "RBX (0x50): %016x\n", *((unsigned long *)($rsp + 0x50))
    printf "RDI (0x48): %016x\n", *((unsigned long *)($rsp + 0x48))
    printf "RSI (0x40): %016x\n", *((unsigned long *)($rsp + 0x40))
    printf "R8  (0x38): %016x\n", *((unsigned long *)($rsp + 0x38))
    printf "R9  (0x30): %016x\n", *((unsigned long *)($rsp + 0x30))
    printf "R10 (0x28): %016x\n", *((unsigned long *)($rsp + 0x28))
    printf "R11 (0x20): %016x\n", *((unsigned long *)($rsp + 0x20))
    printf "R12 (0x18): %016x\n", *((unsigned long *)($rsp + 0x18))
    printf "R13 (0x10): %016x\n", *((unsigned long *)($rsp + 0x10))
    printf "R14 (0x08): %016x\n", *((unsigned long *)($rsp + 0x08))
    printf "R15 (0x00): %016x\n", *((unsigned long *)($rsp + 0x00))
end

dashboard assembly -style height 15




>>> p &mt
$4 = (YkMT **) 0x7fffffffe1a8
