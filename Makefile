.PHONY: build
build:
	cargo build

.PHONY: start-dev
start-dev:
	./target/debug/rust_repo_generator

.PHONY: start
start:
	./target/release/

.PHONY: build-release
build-release:
	cargo build --release

