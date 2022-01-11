#!/bin/bash

set -vex

case "$OSTYPE" in
    darwin*)
        export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
        TARGET=x86_64-apple-darwin ;;
    linux*)
        TARGET=x86_64-unknown-linux-gnu ;;
esac

cargo build --verbose --release --locked --target=$TARGET
