use casperlabs_test_env::{TestContract, TestEnv};
use common::{account::AccountHash, *};

#[allow(clippy::too_many_arguments)]
pub fn deploy_erc20(
    env: &TestEnv,
    contract_name: &str,
    sender: AccountHash,
    name: &str,
    symbol: &str,
    decimals: u8,
    supply: U256,
    time: u64,
) -> TestContract {
    let swap_fee: U256 = 0.into();
    let origin_chainid: U256 = 43113.into();
    
    TestContract::new(
        env,
        "erc20_token.wasm",
        contract_name,
        sender,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "total_supply" => supply,
            "minter" => sender.to_formatted_string(),
            "swap_fee" => swap_fee,
            "dev" => "account-hash-55884917f4107a59e8c06557baee7fdada631af6d1c105984d196a84562854eb",
            "origin_chainid" => origin_chainid,
            "origin_contract_address" => "0x163e98bAe55ebAc01bFE63B1EAa9078E548f6a05",
        },
        time,
    )
}

pub fn deploy_factory(
    env: &TestEnv,
    owner: AccountHash,
    fee_to_setter: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => fee_to_setter
        },
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_wcspr(
    env: &TestEnv,
    contract_name: &str,
    owner: AccountHash,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "wcspr-token.wasm",
        contract_name,
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply
        },
        time,
    )
}

pub fn deploy_flashswapper(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: Key,
    dai: Key,
    factory: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "flashswapper-token.wasm",
        "flash_swapper",
        owner,
        runtime_args! {
            "wcspr" => wcspr,
            "dai" => dai,
            "uniswap_v2_factory" => factory
        },
        time,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_pair(
    env: &TestEnv,
    contract_name: &str,
    owner: AccountHash,
    name: &str,
    symbol: &str,
    decimals: u8,
    supply: U256,
    callee_package_hash: Key,
    factory_hash: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "pair-token.wasm",
        contract_name,
        owner,
        runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => supply,
            "callee_package_hash" => callee_package_hash,
            "factory_hash" => factory_hash
        },
        time,
    )
}

pub fn deploy_library(env: &TestEnv, owner: AccountHash, time: u64) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        time,
    )
}

pub fn deploy_router(
    env: &TestEnv,
    owner: AccountHash,
    factory: Key,
    wcspr: Key,
    library: Key,
    time: u64,
) -> TestContract {
    TestContract::new(
        env,
        "uniswap-v2-router.wasm",
        "Uniswap Router",
        owner,
        runtime_args! {
            "factory" => factory,
            "wcspr" => wcspr,
            "library" => library
        },
        time,
    )
}

pub fn deploy_dummy_tokens(
    env: &TestEnv,
    owner: Option<AccountHash>,
    time: u64,
) -> (TestContract, TestContract, TestContract) {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();
    let token1_owner = owner.unwrap_or_else(|| env.next_user());
    let token1_contract = deploy_erc20(
        env,
        "token1_contract",
        token1_owner,
        "token1",
        "tk1",
        decimals,
        init_total_supply,
        time,
    );
    let token2_owner = owner.unwrap_or_else(|| env.next_user());
    let token2_contract = deploy_erc20(
        env,
        "token2_contract",
        token2_owner,
        "token2",
        "tk2",
        decimals,
        init_total_supply,
        time,
    );
    let token3_owner = owner.unwrap_or_else(|| env.next_user());
    let token3_contract = deploy_erc20(
        env,
        "token3_contract",
        token3_owner,
        "token3",
        "tk3",
        decimals,
        init_total_supply,
        time,
    );
    (token1_contract, token2_contract, token3_contract)
}
