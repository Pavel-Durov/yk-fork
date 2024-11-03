set logging on
set breakpoint pending on

break ykrt::trace::swt::cp::jump_into_optimised_version
break ykrt::trace::swt::cp::jump_into_unoptimised_version
# break build_asm_jump_into_unoptimised_version

# break buffered_vfprintf

break ykrt::trace::swt::cp::jump_into;
break main