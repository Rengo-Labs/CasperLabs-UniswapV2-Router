use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

use crate::uniswap_instance::UniswapInstance;

use more_asserts;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

const NAME: &str = "uniswap_router";

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

    let token2_owner = if owner.is_none() {
        env.next_user()
    } else {
        owner.unwrap()
    };
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

    let token3_owner = if owner.is_none() {
        env.next_user()
    } else {
        owner.unwrap()
    };
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
    (token1_contract, token2_contract, token3_contract)
}

fn deploy_uniswap_router() -> (
    TestEnv,
    UniswapInstance,
    AccountHash,
    Key,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    // deploy factory contract
    let factory_contract = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner),
        runtime_args! {
            "fee_to_setter" => Key::from(owner)
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );

    // deploy wcspr contract
    let wcspr = TestContract::new(&env, "wcspr.wasm", "wcspr", Sender(owner), runtime_args! {});

    // deploy library contract
    let library_contract = TestContract::new(
        &env,
        "library.wasm",
        "library",
        Sender(owner),
        runtime_args! {},
    );

    // deploy pair contract
    let pair_contract = TestContract::new(
        &env,
        "pair.wasm",
        "pair",
        Sender(owner),
        runtime_args! {
            "callee_contract_hash" => Key::from(owner),
            "factory_hash" => Key::Hash(factory_contract.contract_hash()),
        },
    );

    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner)); // deploy dummy tokens for pair initialize

    let args: RuntimeArgs = runtime_args! {
        "token_a" => Key::Hash(token1.contract_hash()),
        "token_b" => Key::Hash(token2.contract_hash()),
        "pair_hash" => Key::Hash(pair_contract.contract_hash())
    };
    let args0: RuntimeArgs = runtime_args! {
        "token_a" => Key::Hash(wcspr.contract_hash()),
        "token_b" => Key::Hash(token2.contract_hash()),
        "pair_hash" => Key::Hash(pair_contract.contract_hash())
    };
    let args1: RuntimeArgs = runtime_args! {
        "token_a" => Key::Hash(token1.contract_hash()),
        "token_b" => Key::Hash(wcspr.contract_hash()),
        "pair_hash" => Key::Hash(pair_contract.contract_hash())
    };

    let _amount: U256 = 1000.into();
    let args2: RuntimeArgs = runtime_args! {
        "to" => Key::Hash(token1.contract_hash()),
        "amount" => _amount
    };
    let args3: RuntimeArgs = runtime_args! {
        "to" => Key::Hash(token2.contract_hash()),
        "amount" => _amount
    };

    factory_contract.call_contract(Sender(owner), "create_pair", args1);
    pair_contract.call_contract(Sender(owner), "erc20_mint", args2);
    pair_contract.call_contract(Sender(owner), "erc20_mint", args3);

    // Need to do mint and sync calls on token
    let amount: U256 = 50.into();
    let args: RuntimeArgs = runtime_args! {
        "recipient" => Key::Hash(pair_contract.contract_hash()),
        "amount" => amount
    };
    token1.call_contract(Sender(owner), "transfer", args);

    let amount: U256 = 950.into();
    let args: RuntimeArgs = runtime_args! {
        "recipient" => Key::Hash(pair_contract.contract_hash()),
        "amount" => amount
    };
    token2.call_contract(Sender(owner), "transfer", args);

    pair_contract.call_contract(Sender(owner), "sync", runtime_args! {});

    let router_contract = TestContract::new(
        &env,
        "uniswap-v2-router.wasm",
        NAME,
        Sender(owner),
        runtime_args! {
            "factory" => Key::Hash(factory_contract.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "library" => Key::Hash(library_contract.contract_hash()),
            "pair" => Key::Hash(pair_contract.contract_hash()),
        },
    );
    let router_package_hash: ContractPackageHash =
        router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    let token = UniswapInstance::new(
        &env,
        Key::Hash(router_contract.contract_hash()),
        Sender(owner),
    );

    (
        env,
        token,
        owner,
        router_package_hash,
        pair_contract,
        token1,
        token2,
        token3,
        wcspr,
    )
}

#[test]
fn test_uniswap_deploy() {
    let (env, token, owner, _, _, _, _, _, _) = deploy_uniswap_router();
    let self_hash: Key = token.uniswap_contract_address();
    let package_hash: Key = token.uniswap_contract_package_hash();
    let uniswap_router_address: Key = token.uniswap_router_address();

    let zero_addr: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    assert_ne!(self_hash, zero_addr);
    assert_ne!(package_hash, zero_addr);
    assert_ne!(uniswap_router_address, zero_addr);
}

#[test]
fn add_liquidity()
{
    let (env, uniswap, owner, router_package_hash, _, token1, token2, token3, _) =
        deploy_uniswap_router();

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let to = Key::Hash(token3.contract_hash());

    let mut rng = rand::thread_rng();
    let amount_a_desired: U256 = rng.gen_range(300..600).into();
    let amount_b_desired: U256 = rng.gen_range(300..600).into();
    let amount_a_min: U256 = rng.gen_range(1..250).into();
    let amount_b_min: U256 = rng.gen_range(1..250).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // approve the router to spend tokens
    uniswap.approve(
        &token1,
        Sender(owner),
        router_package_hash,
        amount_a_desired,
    );
    uniswap.approve(
        &token2,
        Sender(owner),
        router_package_hash,
        amount_b_desired,
    );

    uniswap.add_liquidity(
        Sender(owner),
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
    );
    let (amount_a, amount_b, _): (U256, U256, U256) = uniswap.add_liquidity_result();

    more_asserts::assert_ge!(amount_a, amount_a_min);
    more_asserts::assert_ge!(amount_b, amount_b_min);
}

#[test]
fn add_liquidity_cspr()
{
    let (env, uniswap, owner, router_package_hash, _, token1, token2, _, _) =
        deploy_uniswap_router();

    let to = Key::Hash(token2.contract_hash());

    let mut rng = rand::thread_rng();
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = rng.gen_range(300..600).into();
    let amount_cspr_desired: U256 = rng.gen_range(300..600).into();
    let amount_token_min: U256 = rng.gen_range(1..250).into();
    let amount_cspr_min: U256 = rng.gen_range(1..250).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.approve(
        &token1,
        Sender(owner),
        router_package_hash,
        amount_token_desired,
    );
    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        to,
        deadline.into(),
    );

    let (amount_token, amount_cspr, _): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();
    more_asserts::assert_ge!(amount_token, amount_token_min);
    more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
}

#[test]
fn remove_liquidity()
{
    let (env, uniswap, owner, router_package_hash, pair_contract, token1, token2, token3, _) =
        deploy_uniswap_router();
    let mut rng = rand::thread_rng();

    // NO need to create pair, because pair of token1 and token2 already created in deploy_uniswap_router() method above.
    // The remove_liquidity() call below should be able to find that pair.

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let liquidity: U256 = rng.gen_range(300..500).into();
    let amount_a_min: U256 = rng.gen_range(1..250).into();
    let amount_b_min: U256 = rng.gen_range(1..250).into();
    let to = Key::Hash(token3.contract_hash());

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // approve router on pair
    let args: RuntimeArgs = runtime_args! {
        "spender" => router_package_hash,
        "amount" => liquidity
    };
    pair_contract.call_contract(Sender(owner), "approve", args);

    uniswap.remove_liquidity(
        Sender(owner),
        token_a,
        token_b,
        liquidity,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
    );

    let (amount_a, amount_b): (U256, U256) = uniswap.remove_liquidity_result();
    more_asserts::assert_ge!(amount_a, amount_a_min);
    more_asserts::assert_ge!(amount_b, amount_b_min);
}

#[test]
fn remove_liquidity_cspr() {
    let (env, uniswap, owner, router_package_hash, pair_contract, token1, token2, _, _) = deploy_uniswap_router();
    let mut rng = rand::thread_rng();

    // Here we do need to first create the pair, because pair for token1 and wcspr isn't created anywhere.
    // First Add liquidity
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = rng.gen_range(300..600).into();
    let amount_cspr_desired: U256 = rng.gen_range(300..600).into();
    let amount_token_min: U256 = rng.gen_range(100..250).into();
    let amount_cspr_min: U256 = rng.gen_range(100..250).into();
    let to = Key::Hash(token2.contract_hash());

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.approve(
        &token1,
        Sender(owner),
        router_package_hash,
        amount_token_desired,
    );
    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        to,
        deadline.into(),
    );

    // Remove liquidity
    let token: Key = Key::Hash(token1.contract_hash());
    let liquidity: U256 = rng.gen_range(50..100).into();
    let amount_token_min: U256 = rng.gen_range(0..50).into();
    let amount_cspr_min: U256 = rng.gen_range(0..50).into();
    let to = Key::Hash(token2.contract_hash());

    // approve router on pair
    let args: RuntimeArgs = runtime_args! {
        "spender" => router_package_hash,
        "amount" => liquidity
    };
    pair_contract.call_contract(Sender(owner), "approve", args);

    uniswap.remove_liquidity_cspr(
        Sender(owner),
        token,
        liquidity,
        amount_token_min,
        amount_cspr_min,
        to,
        deadline.into(),
    );

    let (amount_token, amount_cspr): (U256, U256) = uniswap.remove_liquidity_cspr_result();
    more_asserts::assert_ge!(amount_token, amount_token_min);
    more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
}

#[test]
fn remove_liquidity_with_permit() {
    let (env, uniswap, owner, router_package_hash, pair_contract, token1, token2, token3, _) =
        deploy_uniswap_router();
    let mut rng = rand::thread_rng();

    // NO need to create pair, because pair of token1 and token2 already created in deploy_uniswap_router() method above.
    // The remove_liquidity() call below should be able to find that pair.

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let liquidity: U256 = rng.gen_range(50..100).into();
    let amount_a_min: U256 = rng.gen_range(0..50).into();
    let amount_b_min: U256 = rng.gen_range(0..50).into();
    let to = Key::Hash(token3.contract_hash());
    let approve_max = false;

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    let blocktime: U256 = deadline.into();

    let data: String = format!(
        "{}{}{}{}",
        Key::from(owner),
        router_package_hash,
        liquidity,
        blocktime
    );
    let (signature, public_key): (String, String) = uniswap.calculate_signature(&data);
    println!("Returned Signature: {}", signature);
    println!("Returned Public-Key: {}", public_key);

    uniswap.remove_liquidity_with_permit(
        Sender(owner),
        token_a,
        token_b,
        liquidity,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
        approve_max,
        public_key,
        signature,
    );

    let (amount_a, amount_b): (U256, U256) = uniswap.remove_liquidity_with_permit_result();
    more_asserts::assert_ge!(amount_a, amount_a_min);
    more_asserts::assert_ge!(amount_b, amount_b_min);
}

#[test]
fn remove_liquidity_cspr_with_permit() {
    let (env, uniswap, owner, router_package_hash, _, token1, token2, _, _) =
        deploy_uniswap_router();
    let mut rng = rand::thread_rng();

    // Here we do need to first create the pair, because pair for token1 and wcspr isn't created anywhere.
    // First Add liquidity
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = rng.gen_range(300..600).into();
    let amount_cspr_desired: U256 = rng.gen_range(300..600).into();
    let amount_token_min: U256 = rng.gen_range(1..250).into();
    let amount_cspr_min: U256 = rng.gen_range(1..250).into();
    let to = Key::Hash(token2.contract_hash());

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.approve(
        &token1,
        Sender(owner),
        router_package_hash,
        amount_token_desired,
    );
    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        to,
        deadline.into(),
    );

    // Now remove liquidity
    let token = Key::Hash(token1.contract_hash());
    let liquidity: U256 = rng.gen_range(50..100).into();
    let amount_token_min: U256 = rng.gen_range(0..50).into();
    let amount_cspr_min: U256 = rng.gen_range(0..50).into();
    let to = Key::Hash(token2.contract_hash());
    let approve_max = false;

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };
    let data: String = format!(
        "{}{}{}{}",
        Key::from(owner),
        router_package_hash,
        liquidity,
        deadline
    );
    let (signature, public_key): (String, String) = uniswap.calculate_signature(&data);
    println!("Returned Signature: {}", signature);
    println!("Returned Public-Key: {}", public_key);

    uniswap.remove_liquidity_cspr_with_permit(
        Sender(owner),
        token,
        liquidity,
        amount_token_min,
        amount_cspr_min,
        to,
        deadline.into(),
        approve_max,
        public_key,
        signature,
    );

    let (amount_token, amount_cspr): (U256, U256) =
        uniswap.remove_liquidity_cspr_with_permit_result();
    more_asserts::assert_ge!(amount_token, amount_token_min);
    more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
}

#[test]
fn swap_exact_tokens_for_tokens() {
    let (env, uniswap, owner, router_package_hash, _, token1, token2, token3, _) =
        deploy_uniswap_router();

    let mut rng = rand::thread_rng();
    let amount_in: U256 = rng.gen_range(300..400).into();
    let amount_out_min: U256 = rng.gen_range(0..10).into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(token2.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // give allowance to input token
    uniswap.approve(&token1, Sender(owner), router_package_hash, amount_in);

    uniswap.swap_exact_tokens_for_tokens(
        Sender(owner),
        amount_in,
        amount_out_min,
        path,
        to,
        deadline.into(),
    );
}

#[test]
fn swap_tokens_for_exact_tokens() {
    let (env, uniswap, owner, router_package_hash, _, token1, token2, token3, _) =
        deploy_uniswap_router();

    let mut rng = rand::thread_rng();
    let amount_in_max: U256 = rng.gen_range(300..600).into();
    let amount_out: U256 = rng.gen_range(0..250).into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(token2.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // give allowance to input token
    uniswap.approve(&token1, Sender(owner), router_package_hash, amount_in_max);

    uniswap.swap_tokens_for_exact_tokens(
        Sender(owner),
        amount_out,
        amount_in_max,
        path,
        to,
        deadline.into(),
    );
}

#[test]
fn swap_exact_cspr_for_tokens() {
    let (env, uniswap, owner, router_package_hash, _, _, token2, token3, wcspr) =
        deploy_uniswap_router();

    let mut rng = rand::thread_rng();
    let amount_in: U256 = rng.gen_range(300..600).into();
    let amount_out_min: U256 = rng.gen_range(0..250).into();
    let path: Vec<Key> = vec![
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(token2.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // give allowance to input token
    uniswap.approve(&wcspr, Sender(owner), router_package_hash, amount_in);

    uniswap.swap_exact_cspr_for_tokens(
        Sender(owner),
        amount_out_min,
        amount_in,
        path,
        to,
        deadline.into(),
    );
}

#[test]
fn swap_tokens_for_exact_cspr() {
    let (env, uniswap, owner, router_package_hash, _, token1, _, token3, wcspr) =
        deploy_uniswap_router();

    let mut rng = rand::thread_rng();
    let amount_in_max: U256 = rng.gen_range(300..600).into();
    let amount_out: U256 = rng.gen_range(0..250).into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // give allowance to input token
    uniswap.approve(&wcspr, Sender(owner), router_package_hash, amount_in_max);

    uniswap.swap_tokens_for_exact_cspr(
        Sender(owner),
        amount_out,
        amount_in_max,
        path,
        to,
        deadline.into(),
    );
}

#[test]
fn swap_exact_tokens_for_cspr() {
    let (env, uniswap, owner, router_package_hash, _, token1, _, token3, wcspr) =
        deploy_uniswap_router();

    let mut rng = rand::thread_rng();
    let amount_in: U256 = rng.gen_range(300..600).into();
    let amount_out_min: U256 = rng.gen_range(0..250).into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // give allowance to input token
    uniswap.approve(&wcspr, Sender(owner), router_package_hash, amount_in);

    uniswap.swap_exact_tokens_for_cspr(
        Sender(owner),
        amount_in,
        amount_out_min,
        path,
        to,
        deadline.into(),
    );
}

#[test]
fn swap_cspr_for_exact_tokens() {
    let (env, uniswap, owner, router_package_hash, _, _, token2, token3, wcspr) =
        deploy_uniswap_router();

    let mut rng = rand::thread_rng();
    let amount_in_max: U256 = rng.gen_range(300..600).into();
    let amount_out: U256 = rng.gen_range(0..250).into();
    let path: Vec<Key> = vec![
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(token2.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // give allowance to input token
    uniswap.approve(&wcspr, Sender(owner), router_package_hash, amount_in_max);

    uniswap.swap_cspr_for_exact_tokens(
        Sender(owner),
        amount_out,
        amount_in_max,
        path,
        to,
        deadline.into(),
    );
}
