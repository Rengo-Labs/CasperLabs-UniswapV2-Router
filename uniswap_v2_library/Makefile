prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p uniswap_v2_library --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/uniswap-v2-library.wasm 2>/dev/null | true

test-only:
	cargo test -p uniswap_v2_library_tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm uniswap_v2_library_tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf uniswap_v2_library_tests/wasm/*.wasm
