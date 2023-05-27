uniswap_core_directory = ../CasperLabs-UniswapV2-Core/

wasm_src_path = target/wasm32-unknown-unknown/release/

wasm_dest_library = uniswap-v2-library/uniswap-v2-library-tests/wasm/
wasm_dest_router = uniswap-v2-router/uniswap-v2-router-tests/wasm/

prepare:
	rustup target add wasm32-unknown-unknown

build-dependencies:
	cd ${uniswap_core_directory} && make build-all

build-contract-uniswap-v2-library:
	cargo build --release -p uniswap-v2-library -p session-code-router --target wasm32-unknown-unknown
build-contract-uniswap-v2-router:
	cargo build --release -p uniswap-v2-router -p session-code-router --target wasm32-unknown-unknown

build-all:
	make build-contract-uniswap-v2-library
	make build-contract-uniswap-v2-router

copy-wasm-file-uniswap-v2-library:
	cp ${uniswap_core_directory}${wasm_src_path}*.wasm ${wasm_dest_library}
	cp ${wasm_src_path}*.wasm ${wasm_dest_library}
copy-wasm-file-uniswap-v2-router:
	cp ${uniswap_core_directory}${wasm_src_path}*.wasm ${wasm_dest_router}
	cp ${wasm_src_path}*.wasm ${wasm_dest_router}

copy-wasm-file-all:
	make copy-wasm-file-uniswap-v2-library
	make copy-wasm-file-uniswap-v2-router

test-uniswap-v2-library:
	cargo test -p uniswap-v2-library-tests
test-uniswap-v2-router:
	cargo test -p uniswap-v2-router-tests

test-all:
	# make test-uniswap-v2-library
	make test-uniswap-v2-router

all:
	make build-dependencies
	make build-all
	make copy-wasm-file-all
	make test-all

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf Cargo.lock
	find . -name "*.wasm" -delete

git-clean:
	git rm -rf --cached .
	git add .

run-critical-test:
	make build-all && make copy-wasm-file-all && cargo test --package uniswap-v2-router-tests --lib -- uniswap_tests::add_and_remove_liquidity_with_tokens --exact --nocapture
