case "$OSTYPE" in
    darwin*)
        export DYLD_FALLBACK_LIBRARY_PATH = "$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
        export LIBCLANG_PATH = "/usr/lib/llvm-10/lib/libclang.so"
        cargo build --verbose --release --locked --target=x86_64-apple-darwin
    linux*)
        cargo build --verbose --release --locked --target=x86_64-unknown-linux-gnu
esac

