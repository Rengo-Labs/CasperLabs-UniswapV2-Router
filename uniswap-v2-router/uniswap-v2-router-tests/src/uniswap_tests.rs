use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256, U512,
};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::uniswap_instance::*;

use more_asserts;
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
        "erc20-token.wasm",
        "token1_contract",
        token1_owner,
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token1",
            "symbol" => "tk1",
            "decimals" => decimals
        },
        0,
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
        token2_owner,
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token2",
            "symbol" => "tk2",
            "decimals" => decimals
        },
        0,
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
        token3_owner,
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => "token3",
            "symbol" => "tk3",
            "decimals" => decimals
        },
        0,
    );
    (token1_contract, token2_contract, token3_contract)
}

fn deploy_pair_contract(
    env: &TestEnv,
    owner: AccountHash,
    factory_contract: Key,
    flash_swapper: Key,
) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 0.into();

    let pair_contract = TestContract::new(
        &env,
        "pair-token.wasm",
        "pair",
        owner,
        runtime_args! {
            "name" => "erc20",
            "symbol" => "ERC",
            "decimals" => decimals,
            "initial_supply" => init_total_supply,
            "factory_hash" => factory_contract,
            "callee_contract_hash" => flash_swapper
        },
        0,
    );

    pair_contract
}

fn deploy_uniswap_router() -> (
    TestEnv,         // env
    UniswapInstance, // token
    AccountHash,     // owner
    TestContract,    // router_contract
    TestContract,    // flash_swapper
    TestContract,    // pair_contract
    TestContract,    // token1
    TestContract,    // token2
    TestContract,    // token3
    TestContract,    // wcspr
    TestContract,    // factory
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner));

    // deploy factory contract
    let factory_contract = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        owner,
        runtime_args! {
            "fee_to_setter" => Key::Hash(token3.package_hash())
            // contract_name is passed seperately, so we don't need to pass it here.
        },
        0,
    );

    let decimals: u8 = 18;
    let init_total_supply: U256 = 0.into();
    // deploy wcspr contract
    let wcspr = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        owner,
        runtime_args! {
            "name" => "wcspr",
            "symbol" => "ERC",
            "decimals" => decimals
        },
        0,
    );

    // deploy wcspr contract
    let dai = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "dai",
        owner,
        runtime_args! {
            "name" => "dai",
            "symbol" => "dai",
            "decimals" => decimals
        },
        0,
    );

    // deploy flash swapper
    let flash_swapper = TestContract::new(
        &env,
        "flashswapper-token.wasm",
        "flash_swapper",
        owner,
        runtime_args! {
            "uniswap_v2_factory" => Key::Hash(factory_contract.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "dai" => Key::Hash(dai.package_hash())
        },
        0,
    );

    // deploy pair contract
    let pair_contract = TestContract::new(
        &env,
        "pair-token.wasm",
        "pair",
        owner,
        runtime_args! {
            "name" => "erc20",
            "symbol" => "ERC",
            "decimals" => decimals,
            "initial_supply" => init_total_supply,
            "factory_hash" => Key::Hash(factory_contract.package_hash()),
            "callee_package_hash" => Key::Hash(flash_swapper.package_hash())
        },
        0,
    );

    // deploy library contract
    let library_contract = TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        owner,
        runtime_args! {},
        0,
    );

    // Deploy Router Contract
    let router_contract = TestContract::new(
        &env,
        "uniswap-v2-router.wasm",
        NAME,
        owner,
        runtime_args! {
            "factory" => Key::Hash(factory_contract.package_hash()),
            "wcspr" => Key::Hash(wcspr.package_hash()),
            "library" => Key::Hash(library_contract.package_hash())
        },
        0,
    );

    // deploy Test contract
    let test_contract = UniswapInstance::new(
        &env,
        Key::Hash(router_contract.package_hash()),
        Key::Hash(library_contract.package_hash()),
        owner,
    );

    // insert router to the factory's white-list
    let router_package_hash: ContractPackageHash =
        router_contract.query_named_key("package_hash".to_string());
    factory_contract.call_contract(
        owner,
        "set_white_list",
        runtime_args! {"white_list" => Key::from(router_package_hash)},
        0,
    );

    token1.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => test_contract.test_contract_package_hash(),
            "amount" => U256::from(100000000),
        },
        0,
    );

    token1.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::from(owner),
            "amount" => U256::from(100000000),
        },
        0,
    );

    token2.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => test_contract.test_contract_package_hash(),
             "amount" => U256::from(100000000),
        },
        0,
    );
    token3.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => test_contract.test_contract_package_hash(),
             "amount" => U256::from(100000000),
        },
        0,
    );
    (
        env,
        test_contract,
        owner,
        router_contract,
        flash_swapper,
        pair_contract,
        token1,
        token2,
        token3,
        wcspr,
        factory_contract,
    )
}

#[test]
fn test_uniswap_deploy() {
    let (_, _, _, _, _, _, _, _, _, _, _) = deploy_uniswap_router();
}

#[test]
fn add_liquidity() {
    let (
        env,
        uniswap,
        owner,
        _router_contract,
        flash_swapper,
        _,
        token1,
        token2,
        _token3,
        _,
        factory,
    ) = deploy_uniswap_router();

    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());

    let amount_a_desired: U256 = U256::from(10000000);
    let amount_b_desired: U256 = U256::from(10000000);
    let amount_a_min: U256 = U256::from(100000);
    let amount_b_min: U256 = U256::from(100000);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Err(_) => 0,
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
    };

    // approvals of tokens are done in test contract, calling test contract's add_liquidity method
    uniswap.add_liquidity(
        owner,
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
    );
    let (amount_a, amount_b, _): (U256, U256, U256) = uniswap.add_liquidity_result();

    more_asserts::assert_ge!(amount_a, amount_a_min);
    more_asserts::assert_ge!(amount_b, amount_b_min);
}

#[test]
fn add_liquidity_cspr() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, _token2, _, _, factory) =
        deploy_uniswap_router();
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token = Key::Hash(token1.package_hash());

    let amount_token_desired: U256 = U256::from(10000000);
    let amount_cspr_desired: U256 = U256::from(100);
    let amount_token_min: U256 = U256::from(100000);
    let amount_cspr_min: U256 = U256::from(10);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_token_desired
        },
        0,
    );

    let amount: U512 = 1000.into();

    let _ = session_add_liquidity_cspr(
        &env,
        owner,
        amount,
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::Hash(router_contract.package_hash()),
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
        Key::Hash(router_contract.package_hash()),
        uniswap.test_contract_package_hash(),
    );

    // let (amount_token, amount_cspr, _): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();
    // more_asserts::assert_ge!(amount_token, amount_token_min);
    // more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
}

#[test]
fn remove_liquidity() {
    let (env, uniswap, owner, _router_contract, flash_swapper, _, token1, token2, _, _, factory) =
        deploy_uniswap_router();

    // First Add liquidity
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());

    let amount_a_desired: U256 = U256::from(10000000);
    let amount_b_desired: U256 = U256::from(10000000);
    let amount_a_min: U256 = U256::from(100000);
    let amount_b_min: U256 = U256::from(100000);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // Token Approving is done in test contract
    uniswap.add_liquidity(
        owner,
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
    );
    let (_, _, liquidity): (U256, U256, U256) = uniswap.add_liquidity_result();

    // Now remove liquidity
    uniswap.remove_liquidity(
        owner,
        token_a,
        token_b,
        liquidity,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Key::Hash(pair.package_hash()),
        uniswap.test_contract_package_hash(),
    );

    let (amount_a, amount_b): (U256, U256) = uniswap.remove_liquidity_result();
    more_asserts::assert_ge!(amount_a, amount_a_min);
    more_asserts::assert_ge!(amount_b, amount_b_min);
}

// #[test]
// fn remove_liquidity_cspr() {
//     let (env, uniswap, owner, router_contract, flash_swapper, _, token1, _token2, _, _, factory) =
//         deploy_uniswap_router();
//     let pair: TestContract = deploy_pair_contract(
//         &env,
//         owner,
//         Key::Hash(factory.package_hash()),
//         Key::Hash(flash_swapper.package_hash()),
//     );

//     let token = Key::Hash(token1.package_hash());

//     let amount_token_desired: U256 = U256::from(10000000);
//     let amount_cspr_desired: U256 = U256::from(100);
//     let amount_token_min: U256 = U256::from(100000);
//     let amount_cspr_min: U256 = U256::from(10);

//     let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
//         Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
//         Err(_) => 0,
//     };

//     let amount: U512 = 1000.into();
//     token1.call_contract(
//         owner,
//         "approve",
//         runtime_args! {
//             "spender" => Key::Hash(router_contract.package_hash()),
//             "amount" => amount_token_desired
//         },
//         0,
//     );

//     let _ = session_add_liquidity_cspr(
//         &env,
//         owner,
//         amount,
//         token,
//         amount_token_desired,
//         amount_cspr_desired,
//         amount_token_min,
//         amount_cspr_min,
//         Key::from(owner),
//         deadline.into(),
//         Some(Key::Hash(pair.package_hash())),
//         Key::Hash(router_contract.package_hash()),
//         uniswap.test_contract_package_hash(),
//     );

//     let (_amount_token, _amount_cspr, liquidity): (U256, U256, U256) =
//         uniswap.add_liquidity_cspr_result();

//     pair.call_contract(
//         owner,
//         "approve",
//         runtime_args! {
//             "spender" => Key::Hash(router_contract.package_hash()),
//             "amount" => U256::from(1),
//         },
//         0,
//     );

//     // Now Remove liquidity

//     let _ = session_remove_liquidity_cspr(
//         &env,
//         owner,
//         token,
//         U256::from(1),
//         amount_token_min,
//         amount_cspr_min,
//         Key::from(owner),
//         deadline.into(),
//         Key::Hash(router_contract.package_hash()),
//         uniswap.test_contract_package_hash(),
//     );

//     // let (amount_token, amount_cspr): (U256, U256) = uniswap.remove_liquidity_cspr_result();
//     // more_asserts::assert_ge!(amount_token, amount_token_min);
//     // more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
// }

#[test]
fn swap_exact_tokens_for_tokens() {
    let (
        env,
        uniswap,
        owner,
        _router_contract,
        flash_swapper,
        _,
        token1,
        token2,
        token3,
        _,
        factory,
    ) = deploy_uniswap_router();

    // first need to add liquidity
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token_a = Key::Hash(token1.package_hash());

    let token_b = Key::Hash(token2.package_hash());
    let to = Key::Hash(token3.package_hash());

    let amount_a_desired: U256 = U256::from(10000000);
    let amount_b_desired: U256 = U256::from(10000000);
    let amount_a_min: U256 = U256::from(100000);
    let amount_b_min: U256 = U256::from(100000);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity(
        owner,
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
    );

    // SWAP
    let amount_in: U256 = 100000.into();
    let amount_out_min: U256 = 1000.into();
    let path: Vec<String> = vec![token_a.to_formatted_string(), token_b.to_formatted_string()];
    let to: Key = Key::Hash(token3.package_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // approval done in test contract
    uniswap.swap_exact_tokens_for_tokens(
        owner,
        amount_in,
        amount_out_min,
        path,
        to,
        deadline.into(),
    );
}

#[test]
fn swap_tokens_for_exact_tokens() {
    let (
        env,
        uniswap,
        owner,
        _router_contract,
        flash_swapper,
        _,
        token1,
        token2,
        token3,
        _,
        factory,
    ) = deploy_uniswap_router();

    // first need to add liquidity
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());
    let to = Key::Hash(token3.package_hash());

    let amount_a_desired: U256 = U256::from(10000000);
    let amount_b_desired: U256 = U256::from(10000000);
    let amount_a_min: U256 = U256::from(100000);
    let amount_b_min: U256 = U256::from(100000);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity(
        owner,
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        to,
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
    );

    // Swap
    let amount_in_max: U256 = 1000000.into();
    let amount_out: U256 = 10000.into();
    let path: Vec<String> = vec![token_a.to_formatted_string(), token_b.to_formatted_string()];
    let to: Key = Key::Hash(token3.package_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.swap_tokens_for_exact_tokens(
        owner,
        amount_out,
        amount_in_max,
        path,
        to,
        deadline.into(),
    );
}

#[test]
fn swap_exact_cspr_for_tokens() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, wcspr, factory) =
        deploy_uniswap_router();
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token = Key::Hash(token1.package_hash());

    let amount_token_desired: U256 = U256::from(10000000);
    let amount_cspr_desired: U256 = U256::from(100);
    let amount_token_min: U256 = U256::from(100000);
    let amount_cspr_min: U256 = U256::from(10);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };
    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_token_desired
        },
        0,
    );
    let amount: U512 = 1000.into();

    let _ = session_add_liquidity_cspr(
        &env,
        owner,
        amount,
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::Hash(router_contract.package_hash()),
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
        Key::Hash(router_contract.package_hash()),
        uniswap.test_contract_package_hash(),
    );

    // let (amount_token, amount_cspr, _): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();
    // more_asserts::assert_ge!(amount_token, amount_token_min);
    // more_asserts::assert_ge!(amount_cspr, amount_cspr_min);

    // Swap
    let amount_in: U256 = 10.into();
    let amount_out_min: U256 = 10.into();
    let path: Vec<String> = vec![
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
        Key::Hash(token1.package_hash()).to_formatted_string(),
    ];
    let to: Key = Key::Hash(token2.package_hash());

    let _ = session_swap_exact_cspr_for_tokens(
        &env,
        owner,
        amount,
        amount_out_min,
        amount_in,
        path,
        to,
        deadline.into(),
        Key::Hash(router_contract.package_hash()),
    );
}

#[test]
fn swap_tokens_for_exact_cspr() {
    let (
        env,
        uniswap,
        owner,
        router_contract,
        flash_swapper,
        _,
        token1,
        _token2,
        _,
        wcspr,
        factory,
    ) = deploy_uniswap_router();
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token = Key::Hash(token1.package_hash());

    let amount_token_desired: U256 = U256::from(10000000);
    let amount_cspr_desired: U256 = U256::from(100);
    let amount_token_min: U256 = U256::from(100000);
    let amount_cspr_min: U256 = U256::from(10);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };
    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_token_desired
        },
        0,
    );
    let amount: U512 = 1000.into();

    let _ = session_add_liquidity_cspr(
        &env,
        owner,
        amount,
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::Hash(router_contract.package_hash()),
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
        Key::Hash(router_contract.package_hash()),
        uniswap.test_contract_package_hash(),
    );

    // let (amount_token, amount_cspr, _): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();
    // more_asserts::assert_ge!(amount_token, amount_token_min);
    // more_asserts::assert_ge!(amount_cspr, amount_cspr_min);

    // Calling Swap now
    let amount_in_max: U256 = 10000000.into();
    let amount_out: U256 = 10.into();
    let path: Vec<String> = vec![
        Key::Hash(token1.package_hash()).to_formatted_string(),
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
    ];

    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_in_max
        },
        0,
    );

    let _ = session_swap_tokens_for_exact_cspr(
        &env,
        owner,
        amount_out,
        amount_in_max,
        path,
        deadline.into(),
        Key::Hash(router_contract.package_hash()),
    );
}

#[test]
fn swap_exact_tokens_for_cspr() {
    let (
        env,
        uniswap,
        owner,
        router_contract,
        flash_swapper,
        _,
        token1,
        _token2,
        _,
        wcspr,
        factory,
    ) = deploy_uniswap_router();
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token = Key::Hash(token1.package_hash());

    let amount_token_desired: U256 = U256::from(10000000);
    let amount_cspr_desired: U256 = U256::from(1000);
    let amount_token_min: U256 = U256::from(100000);
    let amount_cspr_min: U256 = U256::from(100);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };
    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_token_desired
        },
        0,
    );
    let amount: U512 = 1000.into();

    let _ = session_add_liquidity_cspr(
        &env,
        owner,
        amount,
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::Hash(router_contract.package_hash()),
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
        Key::Hash(router_contract.package_hash()),
        uniswap.test_contract_package_hash(),
    );

    // let (amount_token, amount_cspr, _): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();
    // more_asserts::assert_ge!(amount_token, amount_token_min);
    // more_asserts::assert_ge!(amount_cspr, amount_cspr_min);

    // Swap
    let amount_in: U256 = 10000000.into();
    let amount_out_min: U256 = 10.into();

    let path: Vec<String> = vec![
        Key::Hash(token1.package_hash()).to_formatted_string(),
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
    ];

    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_in
        },
        0,
    );

    // Swap

    let _ = session_swap_exact_tokens_for_cspr(
        &env,
        owner,
        amount_in,
        amount_out_min,
        path,
        deadline.into(),
        Key::Hash(router_contract.package_hash()),
    );
}

#[test]
fn swap_cspr_for_exact_tokens() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, wcspr, factory) =
        deploy_uniswap_router();
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(flash_swapper.package_hash()),
    );

    let token = Key::Hash(token1.package_hash());

    let amount_token_desired: U256 = U256::from(10000000);
    let amount_cspr_desired: U256 = U256::from(100);
    let amount_token_min: U256 = U256::from(100000);
    let amount_cspr_min: U256 = U256::from(10);

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };
    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_token_desired
        },
        0,
    );
    let amount: U512 = 1000.into();

    let _ = session_add_liquidity_cspr(
        &env,
        owner,
        amount,
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::Hash(router_contract.package_hash()),
        deadline.into(),
        Some(Key::Hash(pair.package_hash())),
        Key::Hash(router_contract.package_hash()),
        uniswap.test_contract_package_hash(),
    );

    // let (amount_token, amount_cspr, _): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();
    // more_asserts::assert_ge!(amount_token, amount_token_min);
    // more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
    // calling swap now
    let amount_in_max: U256 = 10.into();
    let amount_out: U256 = 10.into();
    let path: Vec<String> = vec![
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
        Key::Hash(token1.package_hash()).to_formatted_string(),
    ];
    let to: Key = Key::Hash(token2.package_hash());
    wcspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(router_contract.package_hash()),
            "amount" => amount_in_max
        },
        0,
    );

    let _ = session_swap_cspr_for_exact_tokens(
        &env,
        owner,
        amount,
        amount_out,
        amount_in_max,
        path,
        to,
        deadline.into(),
        Key::Hash(router_contract.package_hash()),
    );
}
