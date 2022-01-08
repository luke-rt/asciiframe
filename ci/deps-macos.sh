#!/bin/bash

brew install opencv
echo "export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"" >> ~/.bashrc
source ~/.bashrc
