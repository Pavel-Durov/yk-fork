pub fn main() {
    ykbuild::apply_llvm_ld_library_path();

    // FIXME: This is a temporary hack because LLVM has problems if the main thread exits before
    // compilation threads have finished.
    println!("cargo:rustc-cfg=yk_llvm_sync_hack");
    println!("cargo:rerun-if-env-changed=YKB_TRACER");
    if std::env::var("YKB_TRACER") == Ok("swt".to_owned()) {
        println!("cargo:rustc-cfg=tracer_swt");
    }
}
