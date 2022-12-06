use std::time::{SystemTime, UNIX_EPOCH};
use tests_common::{account::AccountHash, deploys::*, functions::u256_to_u512, helpers::*, *};

fn deploy() -> (
    TestEnv,      // env
    AccountHash,  // owner
    TestContract, // router_contract
    TestContract, // flash_swapper
    TestContract, // pair_contract
    TestContract, // token1
    TestContract, // token2
    TestContract, // token3
    TestContract, // wcspr
    TestContract, // factory
    u64,          // time
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let now = now();
    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner), now);
    let factory_contract = deploy_factory(&env, owner, Key::Hash(token3.package_hash()), now);
    let wcspr = deploy_wcspr(
        &env,
        "WCSPR-1",
        owner,
        "Wrapped cspr".into(),
        "WCSPR".into(),
        9,
        0.into(),
        now,
    );
    let dai = deploy_wcspr(
        &env,
        "DAI-1",
        owner,
        "Dai token".into(),
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
    let pair_contract = deploy_pair(
        &env,
        "PAIR-1",
        owner,
        "pair",
        "PR",
        9,
        0.into(),
        Key::Hash(flash_swapper.package_hash()),
        Key::Hash(factory_contract.package_hash()),
        now,
    );
    let library_contract = deploy_library(&env, owner, now);
    let router_contract = deploy_router(
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
        router_contract,
        flash_swapper,
        pair_contract,
        token1,
        token2,
        token3,
        wcspr,
        factory_contract,
        now,
    )
}

#[test]
fn add_and_remove_liquidity_with_tokens() {
    let (env, owner, router, _, pair, token1, token2, _, _, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());
    let amount_a_desired: U256 = AMOUNT.into();
    let amount_b_desired: U256 = AMOUNT.into();
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
            "package_hash" => router_package_hash,
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
        now,
    );
    let (amount_a, amount_b, liquidity): (U256, U256, U256) =
        result_key(&env, owner, "add_liquidity");
    assert_ge!(amount_a, amount_a_min);
    assert_ge!(amount_b, amount_b_min);
    pair.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Address::Contract(router_package_hash),
            "amount" => liquidity
        },
        now,
    );
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "remove_liquidity",
            "package_hash" => router_package_hash,
            "token_a" => token_a,
            "token_b" => token_b,
            "liquidity" => liquidity,
            "amount_a_min" => amount_a_min,
            "amount_b_min" => amount_b_min,
            "to" => to,
            "deadline" => deadline,
        },
        now,
    );
    let (amount_a, amount_b): (U256, U256) = result_key(&env, owner, "remove_liquidity");
    assert_ge!(amount_a, amount_a_min);
    assert_ge!(amount_b, amount_b_min);
}

#[test]
fn add_and_remove_liquidity_with_cspr() {
    let (env, owner, router, _, pair, token1, _, _, _, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token = Key::Hash(token1.package_hash());
    let amount_token_desired: U256 = AMOUNT.into();
    let amount_cspr_desired: U256 = AMOUNT.into();
    let amount_token_min: U256 = 1000000.into();
    let amount_cspr_min: U256 = 1000000.into();
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
            "entrypoint" => "add_liquidity_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_cspr_desired),
            "token" => token,
            "amount_token_desired" => amount_token_desired,
            "amount_cspr_desired" => amount_cspr_desired,
            "amount_token_min" => amount_token_min,
            "amount_cspr_min" => amount_cspr_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_
        },
        now,
    );
    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) =
        result_key(&env, owner, "add_liquidity_cspr");
    assert_ge!(amount_token, amount_token_min);
    assert_ge!(amount_cspr, amount_cspr_min);
    pair.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Address::Contract(router_package_hash),
            "amount" => liquidity
        },
        now,
    );
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "remove_liquidity_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(liquidity),
            "token" => token,
            "liquidity" => liquidity,
            "amount_token_min" => amount_token_min,
            "amount_cspr_min" => amount_cspr_min,
            "to" => to,
            "deadline" => deadline,
        },
        now,
    );
    let (amount_token, amount_cspr): (U256, U256) =
        result_key(&env, owner, "remove_liquidity_cspr");
    assert_ge!(amount_token, amount_token_min);
    assert_ge!(amount_cspr, amount_cspr_min);
}

#[test]
fn swap_exact_tokens_for_tokens() {
    let (env, owner, router, _, pair, token1, token2, _, _, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());
    let amount_a_desired: U256 = AMOUNT.into();
    let amount_b_desired: U256 = AMOUNT.into();
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
            "package_hash" => router_package_hash,
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
        now,
    );
    let (amount_a, amount_b, _): (U256, U256, U256) = result_key(&env, owner, "add_liquidity");
    assert_ge!(amount_a, amount_a_min);
    assert_ge!(amount_b, amount_b_min);
    // SWAP
    let amount_in: U256 = 100000.into();
    let amount_out_min: U256 = 1000.into();
    let path: Vec<String> = vec![token_a.to_formatted_string(), token_b.to_formatted_string()];
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "swap_exact_tokens_for_tokens",
            "package_hash" => router_package_hash,
            "amount_in" => amount_in,
            "amount_out_min" => amount_out_min,
            "path" => path,
            "to" => to,
            "deadline" => deadline,
        },
        now,
    );
    let ret: Vec<U256> = result_key(&env, owner, "swap_exact_tokens_for_tokens");
    assert_eq!(ret, [100000.into(), 99699.into()]);
}

#[test]
fn swap_tokens_for_exact_tokens() {
    let (env, owner, router, _, pair, token1, token2, _, _, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token_a = Key::Hash(token1.package_hash());
    let token_b = Key::Hash(token2.package_hash());
    let amount_a_desired: U256 = AMOUNT.into();
    let amount_b_desired: U256 = AMOUNT.into();
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
            "package_hash" => router_package_hash,
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
        now,
    );
    let (amount_a, amount_b, _): (U256, U256, U256) = result_key(&env, owner, "add_liquidity");
    assert_ge!(amount_a, amount_a_min);
    assert_ge!(amount_b, amount_b_min);
    // SWAP
    let amount_in_max: U256 = 1000000.into();
    let amount_out: U256 = 10000.into();
    let path: Vec<String> = vec![token_a.to_formatted_string(), token_b.to_formatted_string()];
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "swap_tokens_for_exact_tokens",
            "package_hash" => router_package_hash,
            "amount_out" => amount_out,
            "amount_in_max" => amount_in_max,
            "path" => path,
            "to" => to,
            "deadline" => deadline,
        },
        now,
    );
    let ret: Vec<U256> = result_key(&env, owner, "swap_tokens_for_exact_tokens");
    assert_eq!(ret, [10031.into(), 10000.into()]);
}

#[test]
fn swap_exact_cspr_for_tokens() {
    let (env, owner, router, _, pair, token1, _, _, wcspr, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token = Key::Hash(token1.package_hash());
    let amount_token_desired: U256 = AMOUNT.into();
    let amount_cspr_desired: U256 = AMOUNT.into();
    let amount_token_min: U256 = 1000000.into();
    let amount_cspr_min: U256 = 1000000.into();
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
            "entrypoint" => "add_liquidity_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_cspr_desired),
            "token" => token,
            "amount_token_desired" => amount_token_desired,
            "amount_cspr_desired" => amount_cspr_desired,
            "amount_token_min" => amount_token_min,
            "amount_cspr_min" => amount_cspr_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_
        },
        now,
    );
    let (amount_token, amount_cspr, _): (U256, U256, U256) =
        result_key(&env, owner, "add_liquidity_cspr");
    assert_ge!(amount_token, amount_token_min);
    assert_ge!(amount_cspr, amount_cspr_min);
    // SWAP
    let amount_out_min: U256 = 10000.into();
    let amount_in: U256 = 1000000.into();
    let path: Vec<String> = vec![
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
        token.to_formatted_string(),
    ];
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "swap_exact_cspr_for_tokens",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_in),
            "amount_out_min" => amount_out_min,
            "amount_in" => amount_in,
            "path" => path,
            "to" => to,
            "deadline" => deadline,
        },
        now,
    );
    let ret: Vec<U256> = result_key(&env, owner, "swap_exact_cspr_for_tokens");
    assert_eq!(ret, [1000000.into(), 996990.into()]);
}

#[test]
fn swap_cspr_for_exact_tokens() {
    let (env, owner, router, _, pair, token1, _, _, wcspr, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token = Key::Hash(token1.package_hash());
    let amount_token_desired: U256 = AMOUNT.into();
    let amount_cspr_desired: U256 = AMOUNT.into();
    let amount_token_min: U256 = 1000000.into();
    let amount_cspr_min: U256 = 1000000.into();
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
            "entrypoint" => "add_liquidity_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_cspr_desired),
            "token" => token,
            "amount_token_desired" => amount_token_desired,
            "amount_cspr_desired" => amount_cspr_desired,
            "amount_token_min" => amount_token_min,
            "amount_cspr_min" => amount_cspr_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_
        },
        now,
    );
    let (amount_token, amount_cspr, _): (U256, U256, U256) =
        result_key(&env, owner, "add_liquidity_cspr");
    assert_ge!(amount_token, amount_token_min);
    assert_ge!(amount_cspr, amount_cspr_min);
    // SWAP
    let amount_out: U256 = 10000.into();
    let amount_in_max: U256 = 1000000.into();
    let path: Vec<String> = vec![
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
        token.to_formatted_string(),
    ];
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "swap_cspr_for_exact_tokens",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_in_max),
            "amount_out" => amount_out,
            "amount_in_max" => amount_in_max,
            "path" => path,
            "to" => to,
            "deadline" => deadline,
        },
        now,
    );
    let ret: Vec<U256> = result_key(&env, owner, "swap_cspr_for_exact_tokens");
    assert_eq!(ret, [10031.into(), 10000.into()]);
}

#[test]
fn swap_exact_tokens_for_cspr() {
    let (env, owner, router, _, pair, token1, _, _, wcspr, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token = Key::Hash(token1.package_hash());
    let amount_token_desired: U256 = AMOUNT.into();
    let amount_cspr_desired: U256 = AMOUNT.into();
    let amount_token_min: U256 = 1000000.into();
    let amount_cspr_min: U256 = 1000000.into();
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
            "entrypoint" => "add_liquidity_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_cspr_desired),
            "token" => token,
            "amount_token_desired" => amount_token_desired,
            "amount_cspr_desired" => amount_cspr_desired,
            "amount_token_min" => amount_token_min,
            "amount_cspr_min" => amount_cspr_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_
        },
        now,
    );
    let (amount_token, amount_cspr, _): (U256, U256, U256) =
        result_key(&env, owner, "add_liquidity_cspr");
    assert_ge!(amount_token, amount_token_min);
    assert_ge!(amount_cspr, amount_cspr_min);
    // SWAP
    let amount_out_min: U256 = 10000.into();
    let amount_in: U256 = 1000000.into();
    let path: Vec<String> = vec![
        token.to_formatted_string(),
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
    ];
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "swap_exact_tokens_for_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_in),
            "amount_out_min" => amount_out_min,
            "amount_in" => amount_in,
            "path" => path,
            "deadline" => deadline,
        },
        now,
    );
    let ret: Vec<U256> = result_key(&env, owner, "swap_exact_tokens_for_cspr");
    assert_eq!(ret, [1000000.into(), 996990.into()]);
}

#[test]
fn swap_tokens_for_exact_cspr() {
    let (env, owner, router, _, pair, token1, _, _, wcspr, _, now) = deploy();
    let router_package_hash: ContractPackageHash = router.package_hash().into();
    let token = Key::Hash(token1.package_hash());
    let amount_token_desired: U256 = AMOUNT.into();
    let amount_cspr_desired: U256 = AMOUNT.into();
    let amount_token_min: U256 = 1000000.into();
    let amount_cspr_min: U256 = 1000000.into();
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
            "entrypoint" => "add_liquidity_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_cspr_desired),
            "token" => token,
            "amount_token_desired" => amount_token_desired,
            "amount_cspr_desired" => amount_cspr_desired,
            "amount_token_min" => amount_token_min,
            "amount_cspr_min" => amount_cspr_min,
            "to" => to,
            "deadline" => deadline,
            "pair" => pair_
        },
        now,
    );
    let (amount_token, amount_cspr, _): (U256, U256, U256) =
        result_key(&env, owner, "add_liquidity_cspr");
    assert_ge!(amount_token, amount_token_min);
    assert_ge!(amount_cspr, amount_cspr_min);
    // SWAP
    let amount_out: U256 = 10000.into();
    let amount_in_max: U256 = 1000000.into();
    let path: Vec<String> = vec![
        token.to_formatted_string(),
        Key::Hash(wcspr.package_hash()).to_formatted_string(),
    ];
    call(
        &env,
        owner,
        SESSION_CODE_ROUTER,
        runtime_args! {
            "entrypoint" => "swap_tokens_for_exact_cspr",
            "package_hash" => router_package_hash,
            "amount" => u256_to_u512(amount_in_max),
            "amount_out" => amount_out,
            "amount_in_max" => amount_in_max,
            "path" => path,
            "to" => to,
            "deadline" => deadline,
        },
        now,
    );
    let ret: Vec<U256> = result_key(&env, owner, "swap_tokens_for_exact_cspr");
    assert_eq!(ret, [10031.into(), 10000.into()]);
}
