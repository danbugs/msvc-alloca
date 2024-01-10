fn main() {
    let mut build = cc::Build::new();

    build.file("_alloca_wrapper.c");

    #[cfg(target_env = "msvc")]
    {
        build.flag("/GS-");
        build.flag("/c");
        build.flag("/kernel");
    }

    build.compile("alloca_wrapper");
}