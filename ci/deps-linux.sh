#!/bin/bash

set -vex

sudo apt-get update

sudo apt-get install -y clang
sudo ln -s libclang.so.1 /usr/lib/llvm-10/lib/libclang.so

sudo apt-get -y install libopencv-dev
