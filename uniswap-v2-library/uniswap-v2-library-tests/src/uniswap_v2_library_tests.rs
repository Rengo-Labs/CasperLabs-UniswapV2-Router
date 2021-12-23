use casper_engine_test_support::AccountHash;
use casper_types::{
    contracts::ContractHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use std::time::{SystemTime, UNIX_EPOCH};
use test_env::{Sender, TestContract, TestEnv};

use crate::uniswap_v2_library_instance::LibraryInstance;
/*
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
*/

fn deploy_dummy_tokens(
    env: &TestEnv,
    owner: Option<AccountHash>,
) -> (TestContract, TestContract, TestContract) {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();

    let token1_owner = if owner.is_none() {
        env.next_user()
    } else {
        owner.unwrap()
    };
    let token1_contract = TestContract::new(
        &env,
        "erc20-token.wasm",
        "token1_contract",
        Sender(token1_owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token1",
            "symbol" => "tk1",
            "decimals" => decimals
        },
    );

    let token2_owner = if owner.is_none() {
        env.next_user()
    } else {
        owner.unwrap()
    };
    let token2_contract = TestContract::new(
        &env,
        "erc20-token.wasm",
        "token2_contract",
        Sender(token2_owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token2",
            "symbol" => "tk2",
            "decimals" => decimals
        },
    );

    let token3_owner = if owner.is_none() {
        env.next_user()
    } else {
        owner.unwrap()
    };
    let token3_contract = TestContract::new(
        &env,
        "erc20-token.wasm",
        "token3_contract",
        Sender(token3_owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token3",
            "symbol" => "tk3",
            "decimals" => decimals
        },
    );

    (token1_contract, token2_contract, token3_contract)
}

fn deploy_library() -> (
    TestEnv,
    AccountHash,
    LibraryInstance,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) // env, owner, TestContract, LibraryContract, FactoryContract, Pair, Router
{
    let env = TestEnv::new();
    let owner = env.next_user();

    let (_, _, token3) = deploy_dummy_tokens(&env, Some(owner));

    // deploy factory contract
    let factory_contract = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner),
        runtime_args! {
            "fee_to_setter" => Key::Hash(token3.contract_hash())
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );

    let decimals: u8 = 18;
    // deploy wcspr contract
    let wcspr = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        Sender(owner),
        runtime_args! {
            "name" => "wcspr",
            "symbol" => "ERC",
            "decimals" => decimals
        },
    );

    // deploy wcspr contract
    let dai = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "dai",
        Sender(owner),
        runtime_args! {
            "name" => "dai",
            "symbol" => "dai",
            "decimals" => decimals
        },
    );

    // deploy flash swapper
    let flash_swapper = TestContract::new(
        &env,
        "flash-swapper.wasm",
        "flash_swapper",
        Sender(owner),
        runtime_args! {
            "uniswap_v2_factory" => Key::Hash(factory_contract.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "dai" => Key::Hash(dai.contract_hash())
        },
    );

    // deploy pair contract
    let init_total_supply: U256 = 0.into();
    let pair_contract = TestContract::new(
        &env,
        "pair-token.wasm",
        "pair",
        Sender(owner),
        runtime_args! {
            "name" => "erc20",
            "symbol" => "ERC",
            "decimals" => decimals,
            "initial_supply" => init_total_supply,
            "factory_hash" => Key::Hash(factory_contract.contract_hash()),
            "callee_contract_hash" => Key::Hash(flash_swapper.contract_hash())
        },
    );

    // deploy library contract
    let library_contract = TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        Sender(owner),
        runtime_args! {},
    );

    // Deploy Router Contract
    let router_contract = TestContract::new(
        &env,
        "uniswap-v2-router.wasm",
        "Uniswap Router",
        Sender(owner),
        runtime_args! {
            "factory" => Key::Hash(factory_contract.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "library" => Key::Hash(library_contract.contract_hash())
        },
    );

    // deploy Test contract
    let test_contract = LibraryInstance::new(
        &env,
        Key::Hash(router_contract.contract_hash()),
        Key::Hash(library_contract.contract_hash()),
        Sender(owner),
    );

    // insert router to the factory's white-list
    let router_package_hash: ContractPackageHash =
        router_contract.query_named_key("package_hash".to_string());
    factory_contract.call_contract(
        Sender(owner),
        "set_white_list",
        runtime_args! {"white_list" => Key::from(router_package_hash)},
    );

    (
        env,
        owner,
        test_contract,
        library_contract,
        factory_contract,
        pair_contract,
        router_contract,
    )
}

#[test]
fn test_library_deploy() {
    let (_, owner, _, library_contract, _, _, _) = deploy_library();
    println!("Owner: {}", owner);
    let self_hash: ContractHash = library_contract.query_named_key("self_hash".to_string());
    let zero_addr: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(Key::from(self_hash), zero_addr);
}

#[test]
fn quote() {
    let (_, owner, test_contract, _, _, _, _) = deploy_library();

    test_contract.quote(Sender(owner), 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_amount_out() {
    let (_, owner, test_contract, _, _, _, _) = deploy_library();

    test_contract.get_amount_out(Sender(owner), 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_amount_in() {
    let (_, owner, test_contract, _, _, _, _) = deploy_library();

    test_contract.get_amount_in(Sender(owner), 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_reserves() {
    let (env, owner, test_contract, _, factory, pair, router_contract) = deploy_library();
    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner));

    // need to create pair and add liquidity for this test
    let router_package_hash: ContractPackageHash =
        router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let to = Key::Hash(token3.contract_hash());

    let amount_a_desired: U256 = 400.into();
    let amount_b_desired: U256 = 400.into();
    let amount_a_min: U256 = 200.into();
    let amount_b_min: U256 = 200.into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // Minting to test contract
    token1.call_contract(
        Sender(owner),
        "mint",
        runtime_args! {
            "to" => Key::from(test_contract.package_hash_result()),
            "amount" => U256::from(100000)
        },
    );
    token2.call_contract(
        Sender(owner),
        "mint",
        runtime_args! {
            "to" => Key::from(test_contract.package_hash_result()),
            "amount" => U256::from(100000)
        },
    );

    test_contract.proxy_approve(
        Sender(owner),
        &token1,
        router_package_hash,
        amount_a_desired,
    );
    test_contract.proxy_approve(
        Sender(owner),
        &token2,
        router_package_hash,
        amount_b_desired,
    );

    test_contract.add_liquidity(
        Sender(owner),
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
    );

    test_contract.get_reserves(
        Sender(owner),
        Key::Hash(factory.contract_hash()),
        Key::Hash(token1.contract_hash()),
        Key::Hash(token2.contract_hash()),
    );
}

#[test]
fn test_uniswap_get_amounts_out() {
    let (env, owner, test_contract, _, factory, pair, router_contract) = deploy_library();
    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner));

    // need to create pair and liquidity for this test
    let router_package_hash: ContractPackageHash =
        router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let to = Key::Hash(token3.contract_hash());

    let amount_a_desired: U256 = 400.into();
    let amount_b_desired: U256 = 400.into();
    let amount_a_min: U256 = 200.into();
    let amount_b_min: U256 = 200.into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // Minting to library
    token1.call_contract(
        Sender(owner),
        "mint",
        runtime_args! {
            "to" => Key::from(test_contract.package_hash_result()),
            "amount" => U256::from(100000)
        },
    );
    token2.call_contract(
        Sender(owner),
        "mint",
        runtime_args! {
            "to" => Key::from(test_contract.package_hash_result()),
            "amount" => U256::from(100000)
        },
    );

    test_contract.proxy_approve(
        Sender(owner),
        &token1,
        router_package_hash,
        amount_a_desired,
    );
    test_contract.proxy_approve(
        Sender(owner),
        &token2,
        router_package_hash,
        amount_b_desired,
    );

    test_contract.add_liquidity(
        Sender(owner),
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
    );

    let mut path: Vec<Key> = Vec::new();
    path.push(Key::Hash(token1.contract_hash()));
    path.push(Key::Hash(token2.contract_hash()));

    test_contract.get_amounts_out(
        Sender(owner),
        Key::Hash(factory.contract_hash()),
        100.into(),
        path,
    );
}

#[test]
fn test_uniswap_get_amounts_in() {
    let (env, owner, test_contract, _, factory, pair, router_contract) = deploy_library();
    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner));

    // need to create pair and liquidity for this test
    let router_package_hash: ContractPackageHash =
        router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let to = Key::Hash(token3.contract_hash());

    let amount_a_desired: U256 = 400.into();
    let amount_b_desired: U256 = 400.into();
    let amount_a_min: U256 = 200.into();
    let amount_b_min: U256 = 200.into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // Minting to library
    token1.call_contract(
        Sender(owner),
        "mint",
        runtime_args! {
            "to" => Key::from(test_contract.package_hash_result()),
            "amount" => U256::from(100000)
        },
    );
    token2.call_contract(
        Sender(owner),
        "mint",
        runtime_args! {
            "to" => Key::from(test_contract.package_hash_result()),
            "amount" => U256::from(100000)
        },
    );

    test_contract.proxy_approve(
        Sender(owner),
        &token1,
        router_package_hash,
        amount_a_desired,
    );
    test_contract.proxy_approve(
        Sender(owner),
        &token2,
        router_package_hash,
        amount_b_desired,
    );

    test_contract.add_liquidity(
        Sender(owner),
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
    );

    let mut path: Vec<Key> = Vec::new();
    path.push(Key::Hash(token1.contract_hash()));
    path.push(Key::Hash(token2.contract_hash()));

    test_contract.get_amounts_in(
        Sender(owner),
        Key::Hash(factory.contract_hash()),
        100.into(),
        path,
    );
}
