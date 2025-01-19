set logging enabled on
set breakpoint pending on

# break ykrt::trace::swt::cp::debug_return_into_unopt_cp
# break ykrt::trace::swt::cp::debug_return_into_opt_cp

break ykrt::trace::mt
# break __yk_clone_main
break main

break simple2.c:23.c:17
break simple.c:44.c:17
break simple.c:52
# break buffered_vfprintf

# break before cp
break *0x0000000000202b9a
# break after cp
break *0x0000000000202b9d

break ykrt/src/mt.rs:428

# break ykcapi::__ykrt_control_point_real
# break ykrt::mt::MT::control_point


define print_live_vars
    p &mt
    p *&mt
    p &loc
    p *&loc
    p &i
    p *&i
    p &res
    p *&res
end


define print_yk_registers
    p/x $rax
    p/x $rcx
    p/x $rbx
    p/x $rdi
    p/x $rsi
    p/x $r8
    p/x $r9
    p/x $r10
    p/x $r11
    p/x $r12
    p/x $r13
    p/x $r14
    p/x $r15
end
