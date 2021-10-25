erc20_contract = UniswapV2Core/CasperLabs-UniswapV2-core/erc20/
factory_contract = UniswapV2Core/CasperLabs-UniswapV2-core/factory/
flash_swapper_contract = UniswapV2Core/CasperLabs-UniswapV2-core/flash\ swapper/
pair_contract = UniswapV2Core/CasperLabs-UniswapV2-core/pair/
wcspr_contract = UniswapV2Core/CasperLabs-UniswapV2-core/wcspr/
library_contract = UniswapV2Router01_1.3_Update/uniswap_v2_library/
router_contract = UniswapV2Router01_1.3_Update/uniswap_v2_router/
test_contract = UniswapV2Router01_1.3_Update/uniswap_v2_router_test_contract/test-contract/contract/

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_path = UniswapV2Router01_1.3_Update/uniswap_v2_router/uniswap_v2_router_tests/wasm/

build-contract:
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
	
# copy wasm to required directory with new names
copy-wasm-file:	
	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_path}token.wasm
	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_path}factory.wasm
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_path}flash-swapper.wasm
	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_path}pair.wasm
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_path}wcspr.wasm
	cp ${library_contract}${wasm_src_path}*.wasm ${wasm_dest_path}library.wasm
	cp ${router_contract}${wasm_src_path}*.wasm ${wasm_dest_path}uniswap-v2-router.wasm
	cp ${test_contract}${wasm_src_path}*.wasm ${wasm_dest_path}test_contract.wasm
	
	

