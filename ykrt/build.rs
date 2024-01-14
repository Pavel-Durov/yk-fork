pub fn main() {
    ykbuild::apply_llvm_ld_library_path();
    println!("cargo:rerun-if-env-changed=YKB_TRACER");
    // Always compile in the LLVM JIT compiler.
    println!("cargo:rustc-cfg=jitc_llvm");
    // Always compile in our bespoke JIT compiler.
    println!("cargo:rustc-cfg=jitc_yk");
    // FIXME: This is a temporary hack because LLVM has problems if the main thread exits before
    // compilation threads have finished.
    println!("cargo:rustc-cfg=yk_llvm_sync_hack");
    // if std::env::var("YKB_TRACER") == Ok("sw".to_owned()) {
    //     println!("cargo:rustc-cfg=tracer_sw");
    // } else {
    //     println!("cargo:rustc-cfg=tracer_hwt");
    // }
    println!("cargo:rustc-cfg=tracer_hwt");
}
