#[cfg(target_os = "linux")]
fn main() {
    ::std::env::set_var("LIBCLANG_PATH", "/usr/lib/llvm-10/lib/libclang.so");
}

#[cfg(target_os = "macos")]
fn main() {
    ::std::env::set_var("DYLD_FALLBACK_LIBRARY_PATH", "$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/");
}

#[cfg(target_os = "windows")]
fn main() {}
