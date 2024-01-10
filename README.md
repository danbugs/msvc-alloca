# msvc-alloca

This crate provides a safe-ish wrapper around the MSVC `_alloca` function.

This is similar to [alloca-rs](https://github.com/playXE/alloca-rs), but the main difference is that we compile our alloca wrapper with `/GS-`, which disables stack protection and enables it to link with a `/SUBSYSTEM:NATIVE`/`/KERNEL`/`/DRIVER` binary.

For example usage, check out `example/`.