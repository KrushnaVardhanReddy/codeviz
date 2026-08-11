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

dev: wasm
	cd codeviz-web && npm run dev

test-flask:
	mkdir -p temp_repos
	[ -d "temp_repos/flask" ] || git clone --depth 1 https://github.com/pallets/flask.git temp_repos/flask
	echo '[graph]\nexclude = ["**/tests/**", "**/docs/**", "**/examples/**", "**/scripts/**"]' > temp_repos/flask/codeviz.toml
	time cargo run --release --bin codeviz-cli -- run temp_repos/flask --out-dir temp_repos/flask_codeviz_out
