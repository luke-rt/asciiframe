#!/bin/bash

set -vex

ci_dir="$(dirname "$0")"

case "$OSTYPE" in
    darwin*)
        "$ci_dir/deps-macos.sh" ;;
    linux*)
        "$ci_dir/deps-linux.sh" ;;
esac
