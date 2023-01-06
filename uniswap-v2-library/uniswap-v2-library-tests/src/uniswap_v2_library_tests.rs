use std::time::{SystemTime, UNIX_EPOCH};

use crate::uniswap_v2_library_instance::LibraryInstance;
use tests_common::{
    account::AccountHash,
    assert_ge,
    deploys::{deploy_library, *},
    functions::zero_address,
    helpers::{call, init, result_key, AMOUNT, SESSION_CODE_ROUTER},
    *,
};

fn deploy() -> (
    TestEnv,
    AccountHash,
    LibraryInstance,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let now = now();
    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner), now);
    let factory_contract = deploy_factory(&env, owner, Key::Hash(token3.package_hash()), now);
    let wcspr = deploy_wcspr(
        &env,
        "wcspr",
        owner,
        "wcspr".into(),
        "WCSPR".into(),
        9,
        0.into(),
        now,
    );
    let dai = deploy_wcspr(
        &env,
        "dai",
        owner,
        "dai".into(),
        "DAI".into(),
        9,
        0.into(),
        now,
    );
    let flash_swapper = deploy_flashswapper(
        &env,
        owner,
        Key::Hash(wcspr.package_hash()),
        Key::Hash(dai.package_hash()),
        Key::Hash(factory_contract.package_hash()),
        now,
    );
    let init_total_supply: U256 = 0.into();
    let pair_contract: TestContract = deploy_pair(
        &env,
        "pair",
        owner,
        "erc20",
        "ERC",
        9,
        init_total_supply,
        Key::Hash(flash_swapper.package_hash()),
        Key::Hash(factory_contract.package_hash()),
        now,
    );
    let library_contract = deploy_library(&env, owner, now);
    let router_contract: TestContract = deploy_router(
        &env,
        owner,
        Key::Hash(factory_contract.package_hash()),
        Key::Hash(wcspr.package_hash()),
        Key::Hash(library_contract.package_hash()),
        now,
    );
    init(
        owner,
        &token1,
        &token2,
        &factory_contract,
        &router_contract,
        now,
    );
    (
        env,
        owner,
        LibraryInstance::instance(library_contract),
        factory_contract,
        pair_contract,
        router_contract,
        token1,
        token2,
    )
}

#[test]
fn test_library_deploy() {
    let (_, _, library_contract, _, _, _, _, _) = deploy();
    self::assert_ne!(Key::from(library_contract.package_hash()), zero_address());
}

#[test]
fn quote() {
    let (_, owner, library_contract, _, _, _, _, _) = deploy();
    library_contract.quote(owner, 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_amount_out() {
    let (_, owner, library_contract, _, _, _, _, _) = deploy();
    library_contract.get_amount_out(owner, 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_amount_in() {
    let (_, owner, library_contract, _, _, _, _, _) = deploy();
    library_contract.get_amount_in(owner, 100.into(), 200.into(), 300.into());
}

#[test]
fn test_uniswap_get_reserves() {
    let (env, owner, library_contract, factory, pair, router, token1, token2) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());
    let amount_a_desired: U256 = AMOUNT;
    let amount_b_desired: U256 = AMOUNT;
    let amount_a_min: U256 = 1000000.into();
    let amount_b_min: U256 = 1000000.into();
    let to = Key::Account(owner);
    let pair_ = Some(Key::Hash(pair.package_hash()));
    let deadline: U256 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => (n.as_millis() + (1000 * (30 * 60))).into(), // current epoch time in milisecond + 30 minutes
        Err(_) => 0.into(),
    };
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "add_liquidity",
            "package_hash" => Key::from(router_package_hash),
            "token_a" => token_a,
            "token_b" => token_b,
            "amount_a_desired" => amount_a_desired,
            "amount_b_desired" => amount_b_desired,
            "amount_a_min" => amount_a_min,
            "amount_b_min" => amount_b_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_,
        },
        now(),
    );
    let ret: (U256, U256, U256) = result_key(&env, owner, "add_liquidity");
    assert_ge!(ret.0, amount_a_min);
    assert_ge!(ret.1, amount_b_min);
    library_contract.get_reserves(
        owner,
        Key::Hash(factory.package_hash()),
        Key::Hash(token1.package_hash()),
        Key::Hash(token2.package_hash()),
    );
}

#[test]
fn test_uniswap_get_amounts_out() {
    let (env, owner, library_contract, factory, pair, router, token1, token2) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());
    let amount_a_desired: U256 = AMOUNT;
    let amount_b_desired: U256 = AMOUNT;
    let amount_a_min: U256 = 1000000.into();
    let amount_b_min: U256 = 1000000.into();
    let to = Key::Account(owner);
    let pair_ = Some(Key::Hash(pair.package_hash()));
    let deadline: U256 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => (n.as_millis() + (1000 * (30 * 60))).into(), // current epoch time in milisecond + 30 minutes
        Err(_) => 0.into(),
    };
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "add_liquidity",
            "package_hash" => Key::from(router_package_hash),
            "token_a" => token_a,
            "token_b" => token_b,
            "amount_a_desired" => amount_a_desired,
            "amount_b_desired" => amount_b_desired,
            "amount_a_min" => amount_a_min,
            "amount_b_min" => amount_b_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_,
        },
        now(),
    );
    let ret: (U256, U256, U256) = result_key(&env, owner, "add_liquidity");
    assert_ge!(ret.0, amount_a_min);
    assert_ge!(ret.1, amount_b_min);
    let path: Vec<Key> = vec![
        Key::Hash(token1.package_hash()),
        Key::Hash(token2.package_hash()),
    ];
    library_contract.get_amounts_out(owner, Key::Hash(factory.package_hash()), 100.into(), path);
}

#[test]
fn test_uniswap_get_amounts_in() {
    let (env, owner, library_contract, factory, pair, router, token1, token2) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());
    let amount_a_desired: U256 = AMOUNT;
    let amount_b_desired: U256 = AMOUNT;
    let amount_a_min: U256 = 1000000.into();
    let amount_b_min: U256 = 1000000.into();
    let to = Key::Account(owner);
    let pair_ = Some(Key::Hash(pair.package_hash()));
    let deadline: U256 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => (n.as_millis() + (1000 * (30 * 60))).into(), // current epoch time in milisecond + 30 minutes
        Err(_) => 0.into(),
    };
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "add_liquidity",
            "package_hash" => Key::from(router_package_hash),
            "token_a" => token_a,
            "token_b" => token_b,
            "amount_a_desired" => amount_a_desired,
            "amount_b_desired" => amount_b_desired,
            "amount_a_min" => amount_a_min,
            "amount_b_min" => amount_b_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_,
        },
        now(),
    );
    let ret: (U256, U256, U256) = result_key(&env, owner, "add_liquidity");
    assert_ge!(ret.0, amount_a_min);
    assert_ge!(ret.1, amount_b_min);
    let path: Vec<Key> = vec![
        Key::Hash(token1.package_hash()),
        Key::Hash(token2.package_hash()),
    ];
    library_contract.get_amounts_in(owner, Key::Hash(factory.package_hash()), 100.into(), path);
}
