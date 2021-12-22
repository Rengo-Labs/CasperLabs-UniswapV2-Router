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

fn deploy_pair_contract( env: &TestEnv, owner: AccountHash, factory_contract: Key, flash_swapper: Key) -> TestContract
{
    let decimals: u8 = 18;
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
            "factory_hash" => factory_contract,
            "callee_contract_hash" => flash_swapper
        },
    );

    pair_contract
}

fn deploy_uniswap_router() -> (
    TestEnv,                // env
    UniswapInstance,        // token
    AccountHash,            // owner
    TestContract,           // router_contract
    TestContract,           // flash_swapper
    TestContract,           // pair_contract
    TestContract,           // token1
    TestContract,           // token2
    TestContract,           // token3
    TestContract,           // wcspr
    TestContract            // factory
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let (token1, token2, token3) = deploy_dummy_tokens(&env, Some(owner));

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
    let init_total_supply: U256 = 0.into();
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
        NAME,
        Sender(owner),
        runtime_args! {
            "factory" => Key::Hash(factory_contract.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "library" => Key::Hash(library_contract.contract_hash())
        },
    );

    // deploy Test contract
    let test_contract = UniswapInstance::new(
        &env,
        Key::Hash(router_contract.contract_hash()),
        Key::Hash(library_contract.contract_hash()),
        Sender(owner),
    );


    // insert router to the factory's white-list
    let router_package_hash: ContractPackageHash = router_contract.query_named_key("package_hash".to_string());
    factory_contract.call_contract(Sender(owner), "set_white_list" ,runtime_args! {"white_list" => Key::from(router_package_hash)});


    let amount: U256 = 1000.into();
    token1.call_contract(Sender(owner), "mint", runtime_args!{
        "to" => test_contract.test_contract_package_hash(),
         "amount" => amount,
    });
    token2.call_contract(Sender(owner), "mint", runtime_args!{
        "to" => test_contract.test_contract_package_hash(),
         "amount" => amount,
    });
    token3.call_contract(Sender(owner), "mint", runtime_args!{
        "to" => test_contract.test_contract_package_hash(),
         "amount" => amount,
    });
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
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, token3, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());

    let mut rng = rand::thread_rng();
    let amount_a_desired: U256 = rng.gen_range(300..600).into();
    let amount_b_desired: U256 = rng.gen_range(300..600).into();
    let amount_a_min: U256 = rng.gen_range(1..250).into();
    let amount_b_min: U256 = rng.gen_range(1..250).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // approvals of tokens are done in test contract, calling test contract's add_liquidity method
    uniswap.add_liquidity(
        Sender(owner),
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash()))
    );
    let (amount_a, amount_b, _): (U256, U256, U256) = uniswap.add_liquidity_result();

    more_asserts::assert_ge!(amount_a, amount_a_min);
    more_asserts::assert_ge!(amount_b, amount_b_min);
}


#[test]
fn add_liquidity_cspr() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));

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

    // approving is done in test contract
    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
        Key::Hash(router_contract.contract_hash()),
        uniswap.test_contract_hash()
    );

    let (amount_token, amount_cspr, _): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();
    more_asserts::assert_ge!(amount_token, amount_token_min);
    more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
}


#[test]
fn remove_liquidity() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();


    // First Add liquidity
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());

    let mut rng = rand::thread_rng();
    let amount_a_desired: U256 = rng.gen_range(300..600).into();
    let amount_b_desired: U256 = rng.gen_range(300..600).into();
    let amount_a_min: U256 = rng.gen_range(1..50).into();
    let amount_b_min: U256 = rng.gen_range(1..50).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };


    // Token Approving is done in test contract
    uniswap.add_liquidity(
        Sender(owner),
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash()))
    );
    let (_, _, liquidity): (U256, U256, U256) = uniswap.add_liquidity_result();



    // Now remove liquidity
    uniswap.remove_liquidity(
        Sender(owner),
        token_a,
        token_b,
        liquidity,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Key::Hash(pair.contract_hash())
    );

    let (amount_a, amount_b): (U256, U256) = uniswap.remove_liquidity_result();
    more_asserts::assert_ge!(amount_a, amount_a_min);
    more_asserts::assert_ge!(amount_b, amount_b_min);
}


#[test]
fn remove_liquidity_cspr() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();


    let mut rng = rand::thread_rng();

    // Here we do need to first create the pair, because pair for token1 and wcspr isn't created anywhere.
    // First Add liquidity
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = rng.gen_range(300..400).into();
    let amount_cspr_desired: U256 = rng.gen_range(300..400).into();
    let amount_token_min: U256 = rng.gen_range(1..50).into();
    let amount_cspr_min: U256 = rng.gen_range(1..50).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
        Key::Hash(router_contract.contract_hash()),
        uniswap.test_contract_hash(),
    );
    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();


    // Now Remove liquidity
    uniswap.remove_liquidity_cspr(
        Sender(owner),
        token,
        liquidity,
        amount_token_min,
        amount_cspr_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Key::Hash(pair.contract_hash()),
    );

    let (amount_token, amount_cspr): (U256, U256) = uniswap.remove_liquidity_cspr_result();
    more_asserts::assert_ge!(amount_token, amount_token_min);
    more_asserts::assert_ge!(amount_cspr, amount_cspr_min);
}


#[test]
fn remove_liquidity_with_permit() {
    let (env, uniswap, owner, router_contract, flash_swapper ,pair_contract, token1, token2, _, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();


    // First need to add liquidity
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());

    let mut rng = rand::thread_rng();
    let amount_a_desired: U256 = rng.gen_range(300..600).into();
    let amount_b_desired: U256 = rng.gen_range(300..600).into();
    let amount_a_min: U256 = rng.gen_range(1..50).into();
    let amount_b_min: U256 = rng.gen_range(1..50).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // approve the router to spend tokens
    uniswap.add_liquidity(
        Sender(owner),
        token_a,
        token_b,
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash()))
    );
    let (_, _, liquidity): (U256, U256, U256) = uniswap.add_liquidity_result();



    // Now remove liquidity
    let approve_max = false;
    let blocktime: U256 = deadline.into();
    let permit_type_hash: String = pair_contract.query_named_key("permit_type_hash".to_string());
    let domain_separator: String = pair_contract.query_named_key("domain_separator".to_string());
    let nonces: U256 = pair_contract
        .query_dictionary("nonces", router_package_hash.to_formatted_string())
        .unwrap_or_default();

    let data: String = format!(
        "{}{}{}{}{}{}",
        permit_type_hash,
        uniswap.test_contract_package_hash(),
        router_package_hash,
        liquidity,
        nonces,
        blocktime
    );
    let (signature, public_key): (String, String) =
        uniswap.calculate_signature(&data, &domain_separator);

    uniswap.remove_liquidity_with_permit(
        Sender(owner),
        token_a,
        token_b,
        liquidity,
        amount_a_min,
        amount_b_min,
        uniswap.test_contract_package_hash(),
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
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, _, _, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    let mut rng = rand::thread_rng();

    // First Add liquidity
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));

    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = rng.gen_range(300..400).into();
    let amount_cspr_desired: U256 = rng.gen_range(300..400).into();
    let amount_token_min: U256 = rng.gen_range(1..50).into();
    let amount_cspr_min: U256 = rng.gen_range(1..50).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        uniswap.test_contract_package_hash(),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
        Key::Hash(router_contract.contract_hash()),
        uniswap.test_contract_hash()
    );
    let (amount_token, _, liquidity): (U256, U256, U256) = uniswap.add_liquidity_cspr_result();


    // Now remove liquidity
    let approve_max = false;

    let blocktime: U256 = deadline.into();
    let permit_type_hash: String = pair.query_named_key("permit_type_hash".to_string());
    let domain_separator: String = pair.query_named_key("domain_separator".to_string());
    let nonces: U256 = pair
        .query_dictionary("nonces", router_package_hash.to_formatted_string())
        .unwrap_or_default();

    let data: String = format!(
        "{}{}{}{}{}{}",
        permit_type_hash,
        uniswap.test_contract_package_hash(),
        router_package_hash,
        liquidity,
        nonces,
        blocktime
    );
    let (signature, public_key): (String, String) =
        uniswap.calculate_signature(&data, &domain_separator);

    // No need to approve router on pair now, but still need to approve router on token
    let args: RuntimeArgs = runtime_args! {
        "spender" => router_package_hash,
        "amount" => amount_token
    };

    token1.call_contract(Sender(owner), "approve", args);
    uniswap.remove_liquidity_cspr_with_permit(
        Sender(owner),
        token,
        liquidity,
        amount_token_min,
        amount_cspr_min,
        Key::from(owner),
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
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, token3, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    // first need to add liquidity
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));

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
        Some(Key::Hash(pair.contract_hash()))
    );



    // SWAP
    let amount_in: U256 = 50.into();
    let amount_out_min: U256 = 25.into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(token2.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    // approval done in test contract
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
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, token3, _, factory) =
        deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    // first need to add liquidity
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));

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
        Some(Key::Hash(pair.contract_hash()))
    );


    // Swap
    let amount_in_max: U256 = 50.into();
    let amount_out: U256 = 25.into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(token2.contract_hash()),
    ];
    let to: Key = Key::Hash(token3.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

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
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, wcspr, factory) =
    deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();


    // add liquidity to cspr pair
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = 400.into();
    let amount_cspr_desired: U256 = 400.into();
    let amount_token_min: U256 = 400.into();
    let amount_cspr_min: U256 = 400.into();


    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::from(owner),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
        Key::Hash(router_contract.contract_hash()),
        uniswap.test_contract_hash()
    );


    // Swap
    let amount_in: U256 = 50.into();
    let amount_out_min: U256 = 25.into();
    let path: Vec<Key> = vec![
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(token1.contract_hash()),
    ];
    let to: Key = Key::Hash(token2.contract_hash());

    uniswap.swap_exact_cspr_for_tokens(
        Sender(owner),
        amount_out_min,
        amount_in,
        path,
        to,
        deadline.into(),
        Key::Hash(router_contract.contract_hash())
    );
}


#[test]
fn swap_tokens_for_exact_cspr() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, wcspr, factory) =
    deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();


    // add liquidity to cspr pair
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = 400.into();
    let amount_cspr_desired: U256 = 400.into();
    let amount_token_min: U256 = 400.into();
    let amount_cspr_min: U256 = 400.into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::from(owner),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
        Key::Hash(router_contract.contract_hash()),
        uniswap.test_contract_hash()
    );


    // Calling Swap now
    let amount_in_max: U256 = 50.into();
    let amount_out: U256 = 25.into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
    ];

    uniswap.swap_tokens_for_exact_cspr(
        Sender(owner),
        amount_out,
        amount_in_max,
        path,
        deadline.into(),
    );
}


#[test]
fn swap_exact_tokens_for_cspr() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, wcspr, factory) =
    deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    // add liquidity to cspr pair
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = 400.into();
    let amount_cspr_desired: U256 = 400.into();
    let amount_token_min: U256 = 400.into();
    let amount_cspr_min: U256 = 400.into();

    let to = Key::Hash(token2.contract_hash());

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::from(owner),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
        Key::Hash(router_contract.contract_hash()),
        uniswap.test_contract_hash()
    );

    // Swap
    let amount_in: U256 = 50.into();
    let amount_out_min: U256 = 25.into();
    let path: Vec<Key> = vec![
        Key::Hash(token1.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
    ];

    uniswap.swap_exact_tokens_for_cspr(
        Sender(owner),
        amount_in,
        amount_out_min,
        path,
        deadline.into(),
    );
}


#[test]
fn swap_cspr_for_exact_tokens() {
    let (env, uniswap, owner, router_contract, flash_swapper, _, token1, token2, _, wcspr, factory) =
    deploy_uniswap_router();

    let router_package_hash: ContractPackageHash = router_contract.query_named_key(String::from("package_hash"));
    let router_package_hash: Key = router_package_hash.into();

    // add liquidity to cspr pair
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory.contract_hash()), Key::Hash(flash_swapper.contract_hash()));
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = 400.into();
    let amount_cspr_desired: U256 = 400.into();
    let amount_token_min: U256 = 400.into();
    let amount_cspr_min: U256 = 400.into();

    let to = Key::Hash(token2.contract_hash());

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    uniswap.add_liquidity_cspr(
        Sender(owner),
        token,
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        Key::from(owner),
        deadline.into(),
        Some(Key::Hash(pair.contract_hash())),
        Key::Hash(router_contract.contract_hash()),
        uniswap.test_contract_hash()
    );


    // calling swap now
    let amount_in_max: U256 = 50.into();
    let amount_out: U256 = 25.into();

    let path: Vec<Key> = vec![
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(token1.contract_hash()),
    ];

    uniswap.swap_cspr_for_exact_tokens(
        Sender(owner),
        amount_out,
        amount_in_max,
        path,
        to,
        deadline.into(),
        Key::Hash(router_contract.contract_hash()),
    );
}
