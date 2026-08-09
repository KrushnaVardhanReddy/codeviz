.PHONY: all build test lint fmt wasm clean

all: build test lint wasm

build:
	cargo build

test:
	cargo test --all

lint:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt --all

wasm:
	wasm-pack build codeviz-wasm --target web

clean:
	cargo clean
