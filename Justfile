testdir := "ci/benchmarks"
artifactsdir := "ci/artifacts"

default: run

run:
	cargo run -- -f {{testdir}}/test.mov
	rm -rf {{artifactsdir}}/*
	cargo run -- -f {{testdir}}/test.mov -o {{artifactsdir}}/test.sh

deps:
	sh ci/deps.sh

install:
	cargo install --path .

build:
	cargo build --release --locked

lint:
	cargo fmt --all
	cargo clippy

clean:
	cargo clean
