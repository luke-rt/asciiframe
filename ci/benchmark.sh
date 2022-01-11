#!/bin/bash

set -vex

case "$OSTYPE" in
    darwin*)
        echo "hello" ;;
        # brew install hyperfine
    linux*)
        wget https://github.com/sharkdp/hyperfine/releases/download/v1.12.0/hyperfine_1.12.0_amd64.deb
        sudo dpkg -i hyperfine_1.12.0_amd64.deb ;;
esac

echo "=== BENCHMARK RESULTS:"
hyperfine --warmup 3 --prepare "rm -r ci/benchmarks/test.sh || true" "asc -f ci/benchmarks/test.mov -o ci/benchmarks/test.sh"
