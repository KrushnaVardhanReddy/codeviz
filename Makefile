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
	echo '[graph]\nexclude = ["**/tests/**", "**/docs/**", "**/examples/**", "**/scripts/**"]\nentry_points = ["flask/app.py::Flask::wsgi_app", "flask/app.py::Flask::full_dispatch_request", "flask/app.py::Flask::dispatch_request"]' > temp_repos/flask/codeviz.toml
	time cargo run --release --bin codeviz-cli -- export temp_repos/flask --format json --output codeviz-web/public/flask.json

test-httpie:
	mkdir -p temp_repos
	[ -d "temp_repos/httpie" ] || git clone --depth 1 https://github.com/httpie/cli.git temp_repos/httpie
	echo '[graph]\nexclude = ["**/tests/**", "**/docs/**", "**/extras/**"]\nentry_points = ["httpie/core.py::raw_main", "httpie/client.py::collect_messages", "httpie/output/writer.py::write_message"]' > temp_repos/httpie/codeviz.toml
	time cargo run --release --bin codeviz-cli -- export temp_repos/httpie --format json --output codeviz-web/public/httpie.json
