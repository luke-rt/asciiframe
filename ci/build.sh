#!/bin/bash

set -vex

case "$OSTYPE" in
    darwin*)
        export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
        echo "=== Installed brew packages:"
	    brew list --versions ;;
    linux*)
esac

echo "=== Environment variable dump:"
export

cargo build --verbose
