uniswap_core_directory = ../CasperLabs-UniswapV2-core
uniswap_router_directory = .

erc20_contract = ${uniswap_core_directory}/erc20/
factory_contract = ${uniswap_core_directory}/factory/
flash_swapper_contract = ${uniswap_core_directory}/flashswapper/
pair_contract = ${uniswap_core_directory}/pair/
wcspr_contract = ${uniswap_core_directory}/wcspr/
library_contract = ${uniswap_router_directory}/uniswap-v2-library/
router_contract = ${uniswap_router_directory}/uniswap-v2-router/
test_contract = ${uniswap_router_directory}/uniswap-v2-router-test-contract/test-contract/contract/

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_library_path = ${library_contract}/uniswap-v2-library-tests/wasm/
wasm_dest_router_path = ${router_contract}/uniswap-v2-router-tests/wasm/

all:
	# Build erc20
	cd ${erc20_contract} && make build-contract

	# Build factory
	cd ${factory_contract} && make build-contract

	# Build flash swapper
	cd ${flash_swapper_contract} && make build-contract

	# Build pair
	cd ${pair_contract} && make build-contract

	# Build wcspr
	cd ${wcspr_contract} && make build-contract

	# Build Library
	cd ${library_contract} && make build-contract

	# Build Router
	cd ${router_contract} && make build-contract

	# Build Test_Contract
	cd ${test_contract} && cargo build --release

	# copy wasm files
	make copy-wasm-file
clean:
	# clean erc20
	cd ${erc20_contract} && make clean

	# clean factory
	cd ${factory_contract} && make clean

	# clean flash swapper
	cd ${flash_swapper_contract} && make clean

	# clean pair
	cd ${pair_contract} && make clean

	# clean wcspr
	cd ${wcspr_contract} && make clean

	# clean library
	cd ${library_contract} && make clean

	# clean Router
	cd ${router_contract} && make clean

	# clean test contract
	cd ${test_contract} && cargo clean

# copy wasm to required directory
copy-wasm-file:
	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_library_path}
	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_library_path}
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_library_path}
	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_library_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_library_path}
	cp ${router_contract}${wasm_src_path}*.wasm ${wasm_dest_library_path}
	cp ${test_contract}${wasm_src_path}*.wasm ${wasm_dest_library_path}

	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_router_path}
	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_router_path}
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_router_path}
	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_router_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_router_path}
	cp ${library_contract}${wasm_src_path}*.wasm ${wasm_dest_router_path}
	cp ${test_contract}${wasm_src_path}*.wasm ${wasm_dest_router_path}

# run all tests sequentially
test:
	# Test Library
	cd ${library_contract} && make test

	# Test Router
	cd ${router_contract} && make test
