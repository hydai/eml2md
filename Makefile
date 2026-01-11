.PHONY: lint clippy build all

all: lint clippy build

lint:
	cargo fmt --all --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

build:
	cargo build
