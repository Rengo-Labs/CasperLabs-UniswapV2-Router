use casper_engine_test_support::AccountHash;
use casper_types::{contracts::ContractHash, runtime_args, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

use crate::uniswap_v2_library_instance::UniswapInstance;

const NAME: &str = "uniswap_router";

fn deploy_dummy_tokens(env: &TestEnv) -> (TestContract, TestContract, TestContract) {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();

    let token1_owner = env.next_user();
    let token1_contract = TestContract::new(
        &env,
        "token.wasm",
        "token1_contract",
        Sender(token1_owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token1",
            "symbol" => "tk1",
            "decimals" => decimals
        },
    );

    let token2_owner = env.next_user();
    let token2_contract = TestContract::new(
        &env,
        "token.wasm",
        "token2_contract",
        Sender(token2_owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token2",
            "symbol" => "tk2",
            "decimals" => decimals
        },
    );

    let token3_owner = env.next_user();
    let token3_contract = TestContract::new(
        &env,
        "token.wasm",
        "token3_contract",
        Sender(token3_owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token3",
            "symbol" => "tk3",
            "decimals" => decimals
        },
    );

    println!(
        "DT1: {}",
        Key::Hash(token1_contract.contract_hash()).to_formatted_string()
    );
    println!(
        "DT2: {}",
        Key::Hash(token2_contract.contract_hash()).to_formatted_string()
    );
    println!(
        "DT3: {}",
        Key::Hash(token3_contract.contract_hash()).to_formatted_string()
    );

    (token1_contract, token2_contract, token3_contract)
}

fn deploy_uniswap_router() -> (TestEnv, UniswapInstance, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    // deploy factory contract
    let owner_factory = env.next_user();
    let factory_contract = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner_factory),
        runtime_args! {
            "fee_to_setter" => Key::from(owner_factory)
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );

    // deploy wcspr contract
    let owner_wcspr = env.next_user();
    let wcspr = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        Sender(owner_wcspr),
        runtime_args! {},
    );

    // deploy library contract
    let owner_library = env.next_user();
    let library_contract = TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        Sender(owner_library),
        runtime_args! {},
    );

    let token = UniswapInstance::new(
        &env,
        NAME,
        Key::Hash(factory_contract.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(library_contract.contract_hash()),
        Sender(owner),
    );

    println!(
        "Factory: {}",
        Key::Hash(factory_contract.contract_hash()).to_formatted_string()
    );
    println!(
        "WCSPR: {}",
        Key::Hash(wcspr.contract_hash()).to_formatted_string()
    );
    println!(
        "Library: {}",
        Key::Hash(library_contract.contract_hash()).to_formatted_string()
    );
    (env, token, owner, factory_contract)
}

#[test]
fn test_uniswap_deploy() {
    let (_, token, owner, _) = deploy_uniswap_router();
    println!("Owner: {}", owner);
    let self_hash: Key = token.uniswap_contract_address();
    let zero_addr: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(self_hash, zero_addr);
}

#[test]
fn quote() {
    let (_, uniswap, owner, _) = deploy_uniswap_router();

    uniswap.quote(Sender(owner), 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_reserves() {
    let (env, uniswap, owner, factory) = deploy_uniswap_router();
    let (token1, token2, _) = deploy_dummy_tokens(&env);

    uniswap.get_reserves(
        Sender(owner),
        factory.contract_hash().into(),
        token1.contract_hash().into(),
        token2.contract_hash().into(),
    );
}

#[test]
fn test_uniswap_get_amount_out() {
    let (_, uniswap, owner, _) = deploy_uniswap_router();

    uniswap.get_amount_out(Sender(owner), 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_amount_in() {
    let (_, uniswap, owner, _) = deploy_uniswap_router();

    uniswap.get_amount_in(Sender(owner), 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_amounts_out() {
    let (env, uniswap, owner, factory) = deploy_uniswap_router();
    let (token1, token2, _) = deploy_dummy_tokens(&env);

    let mut path: Vec<ContractHash> = Vec::new();
    path.push(token1.contract_hash().into());
    path.push(token2.contract_hash().into());

    uniswap.get_amounts_out(
        Sender(owner),
        factory.contract_hash().into(),
        100.into(),
        path,
    );
}

#[test]
fn test_uniswap_get_amounts_in() {
    let (env, uniswap, owner, factory) = deploy_uniswap_router();
    let (token1, token2, _) = deploy_dummy_tokens(&env);

    let mut path: Vec<ContractHash> = Vec::new();
    path.push(token1.contract_hash().into());
    path.push(token2.contract_hash().into());

    uniswap.get_amounts_in(
        Sender(owner),
        factory.contract_hash().into(),
        100.into(),
        path,
    );
}
