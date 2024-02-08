fn main() {
    let mut build = cc::Build::new();

    // if os is windows
    #[cfg(target_os = "windows")]
    {
        build.file("_alloca_wrapper.c");
        build.flag("/GS-");
        build.flag("/c");
        build.flag("/kernel");
    }

    // if os is unix
    #[cfg(target_os = "linux")]
    {
        build.file("_alloca_wrapper_alt.c");
        build.flag("-fuse-ld=lld");
        std::env::set_var("AR_x86_64_pc_windows_msvc", "llvm-lib");
    }

    build.compiler("clang-cl");

    build.compile("alloca_wrapper");
}
