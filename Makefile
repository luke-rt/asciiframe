TEST_DIR = ci/benchmarks
ARTIFACT_DIR = ci/artifacts

.PHONY: default
default: run

.PHONY: run
run:
	cargo run -- -f ${TEST_DIR}/test.mov
	rm -rf ${ARTIFACT_DIR}/*
	cargo run -- -f ${TEST_DIR}/test.mov -o ${ARTIFACT_DIR}/test.sh

.PHONY: deps
deps:
	sh ci/deps.sh

.PHONY: install
install:
	cargo install --path .

.PHONY: build
build:
	cargo build --release --locked

.PHONY: lint
lint:
	cargo fmt --all
	cargo clippy

.PHONY: clean
clean:
	cargo clean
