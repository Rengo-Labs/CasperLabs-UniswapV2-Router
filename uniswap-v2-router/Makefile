prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p uniswap-v2-router --target wasm32-unknown-unknown
	wasm-strip uniswap-v2-router/target/wasm32-unknown-unknown/release/uniswap-v2-router.wasm 2>/dev/null | true

build-purse-proxy:
	cargo build --release -p purse-proxy --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/purse-proxy.wasm 2>/dev/null | true

test-only:
	cargo test -p uniswap-v2-router-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm uniswap-v2-router-tests/wasm

test: build-contract build-purse-proxy copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf uniswap-v2-router-tests/wasm/*.wasm
