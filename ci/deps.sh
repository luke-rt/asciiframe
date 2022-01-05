#!/bin/bash

set -vex

case "$OSTYPE" in
    darwin*)
        brew install opencv ;;
    linux*)
        sudo apt-get update
        sudo apt-get install -y clang libclang-dev
        sudo ln -s libclang.so.1 /usr/lib/llvm-6.0/lib/libclang.so
        sudo apt-get install -y libopencv-dev ;;
esac
