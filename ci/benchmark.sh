#!/bin/bash

set -vex

case "$OSTYPE" in
    darwin*)
        brew install hyperfine
        TARGET=x86_64-apple-darwin ;;
    linux*)
        wget https://github.com/sharkdp/hyperfine/releases/download/v1.12.0/hyperfine_1.12.0_amd64.deb
        sudo dpkg -i hyperfine_1.12.0_amd64.deb
        TARGET=x86_64-unknown-linux-gnu ;;
esac

echo "=== BENCHMARK RESULTS:"
hyperfine --warmup 3 --prepare "rm ci/benchmarks/test.sh || true" "target/$TARGET/release/asc -f ci/benchmarks/test.mov -o ci/benchmarks/test.sh"
