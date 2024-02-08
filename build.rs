fn main() {
    let mut build = cc::Build::new();

    #[cfg(target_env = "msvc")]
    {
        build.file("_alloca_wrapper.c");
        build.flag("/GS-");
        build.flag("/c");
        build.flag("/kernel");
    }
    
    // not msvc
    #[cfg(not(target_env = "msvc"))]
    {
        build.file("_alloca_wrapper_alt.c");
    }

    build.compile("alloca_wrapper");
}