#[cfg(target_os = "linux")]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {
    ::std::env::set_var("DYLD_FALLBACK_LIBRARY_PATH", "$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/");
}

#[cfg(target_os = "windows")]
fn main() {}
