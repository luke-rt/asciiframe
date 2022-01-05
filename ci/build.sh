#!/bin/bash

set -vex

case "$OSTYPE" in
    darwin*)
        export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
        cargo build --verbose
    linux*)
        cargo build --verbose ;;
esac

