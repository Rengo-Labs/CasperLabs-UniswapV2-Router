use casper_engine_test_support::AccountHash;
use casper_types::{U256, Key, runtime_args, RuntimeArgs, contracts::{ContractHash}};
use test_env::{Sender, TestEnv, TestContract};

use crate::uniswap_instance::UniswapInstance;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

const NAME: &str = "uniswap_router";

fn deploy_dummy_tokens(env:&TestEnv) -> (TestContract, TestContract, TestContract) 
{
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
        }
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
        }
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
        }
    );

    (token1_contract, token2_contract, token3_contract)
}

fn deploy_uniswap_router() -> (TestEnv, UniswapInstance, AccountHash) 
{
    let env = TestEnv::new();
    let owner = env.next_user();

    // deploy factory contract
    let env_factory = TestEnv::new();
    let owner_factory = env.next_user();
    let factory_contract = TestContract::new(
        //&env_factory,
        &env,
        "factory.wasm",
        "factory",
        Sender(owner_factory),
        runtime_args! {
            "fee_to_setter" => Key::from(owner_factory)
            // contract_name is passed seperately, so we don't need to pass it here.
        }
    );
    
    // deploy wcspr contract
    let env_wcspr = TestEnv::new();
    let owner_wcspr = env_wcspr.next_user();
    let wcspr = TestContract::new(
        // &env_wcspr,
        &env,
        "wcspr.wasm",
        "wcspr",
        Sender(owner_wcspr),
        runtime_args! {}
    );

    // deploy library contract
    let env_library = TestEnv::new();
    let owner_library = env_library.next_user();
    let library_contract = TestContract::new(
        // &env_library,
        &env,
        "library.wasm",
        "library",
        Sender(owner_library),
        runtime_args! {}
    );
    
    let token = UniswapInstance::new(
        &env,
        NAME,
        Key::Hash(factory_contract.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(library_contract.contract_hash()),
        Sender(owner)
    );
    
    println!("Factory: {}", Key::Hash(factory_contract.contract_hash()).to_formatted_string());
    (env, token, owner)
}

/*
#[test]
fn test_uniswap_deploy()
{
    let (env, token, owner) = deploy_uniswap_router();
    println!("{}", owner);
    let self_hash: Key = token.uniswap_contract_address();
    let zero_addr:Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    assert_ne!(self_hash, zero_addr);
}


#[test]
fn test_add_liquidity()
{
    let (env, uniswap, owner) = deploy_uniswap_router();
    let (token1, token2, token3) = deploy_dummy_tokens(&env);


    // token_a: Key, token_b: Key, amount_a_desired: U256, amount_b_desired: U256, amount_a_min: U256, amount_b_min: U256, to:Key, deadline: U256
  
    let token1 = Key::Hash(token1.contract_hash());
    let token2 = Key::Hash(token2.contract_hash());
    let to = Key::Hash(token3.contract_hash());

    //let myKey: Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    //println!("{}, \n{}, \n{}", token1, token2, to);

    let mut rng = rand::thread_rng();
    let amount_a_desired: U256 = rng.gen_range(0..500).into();
    let amount_b_desired: U256 = rng.gen_range(0..500).into();
    let amount_a_min: U256 = rng.gen_range(0..250).into();
    let amount_b_min: U256 = rng.gen_range(0..250).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)),      // current epoch time in milisecond + 30 minutes
        Err(_) => 0
    };

    uniswap.add_liquidity(Sender(owner), token1, token2, amount_a_desired, amount_b_desired, amount_a_min, amount_b_min, to, deadline.into());
}
*/

#[test]
fn add_liquidity_cspr()
{
    let (env, uniswap, owner) = deploy_uniswap_router();
    let (token1, token2, token3) = deploy_dummy_tokens(&env);

    let mut rng = rand::thread_rng();
    let token = Key::Hash(token1.contract_hash());
    let amount_token_desired: U256 = rng.gen_range(0..500).into();
    let amount_cspr_desired: U256 = rng.gen_range(0..500).into();
    let amount_token_min: U256 = rng.gen_range(0..200).into();
    let amount_cspr_min: U256 = rng.gen_range(0..200).into();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)),      // current epoch time in milisecond + 30 minutes
        Err(_) => 0
    };
}

#[test]
fn remove_liquidity()
{
    let (env, uniswap, owner) = deploy_uniswap_router();
    let (token1, token2, token3) = deploy_dummy_tokens(&env);
    let mut rng = rand::thread_rng();
    
    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let liquidity:U256 = rng.gen_range(0..500).into();
    let amount_a_min:U256 = rng.gen_range(0..250).into();
    let amount_b_min:U256 = rng.gen_range(0..250).into();
    let to = Key::Hash(token3.contract_hash());

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)),      // current epoch time in milisecond + 30 minutes
        Err(_) => 0
    };
}

#[test]
fn remove_liquidity_cspr()
{
    let (env, uniswap, owner) = deploy_uniswap_router();
    let (token1, token2, token3) = deploy_dummy_tokens(&env);
    let mut rng = rand::thread_rng();

    let token: Key = Key::Hash(token1.contract_hash());
    let liquidity:U256 = rng.gen_range(0..500).into();
    let amount_token_min: U256 = rng.gen_range(0..200).into();
    let amount_cspr_min: U256 = rng.gen_range(0..200).into();
    let to = Key::Hash(token2.contract_hash());
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)),      // current epoch time in milisecond + 30 minutes
        Err(_) => 0
    };
}

#[test]
pub fn remove_liquidity_with_permit()
{
    let (env, uniswap, owner) = deploy_uniswap_router();
    let (token1, token2, token3) = deploy_dummy_tokens(&env);
    let mut rng = rand::thread_rng();

    let token_a = Key::Hash(token1.contract_hash());
    let token_b = Key::Hash(token2.contract_hash());
    let liquidity: U256 = rng.gen_range(0..500).into();
    let amount_a_min: U256 = rng.gen_range(0..200).into();
    let amount_b_min: U256 = rng.gen_range(0..200).into();
    let to = Key::Hash(token3.contract_hash());
    let approve_max = false;
    let v: u8 = rng.gen_range(0..200);
    let r: u32 = rng.gen_range(0..200);
    let s: u32 = rng.gen_range(0..200);
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)),      // current epoch time in milisecond + 30 minutes
        Err(_) => 0
    };

}

#[test]
fn remove_liquidity_cspr_with_permit()
{
    let (env, uniswap, owner) = deploy_uniswap_router();
    let (token1, token2, token3) = deploy_dummy_tokens(&env);
    let mut rng = rand::thread_rng();

    let token = Key::Hash(token1.contract_hash());
    let liquidity: U256 = rng.gen_range(0..500).into();
    let amount_token_min: U256 = rng.gen_range(0..500).into();
    let amount_cspr_min: U256 = rng.gen_range(0..500).into();
    let to = Key::Hash(token2.contract_hash());
    let approve_max = false;
    let v: u8 = rng.gen_range(0..200);
    let r: u32 = rng.gen_range(0..200);
    let s: u32 = rng.gen_range(0..200);
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)),      // current epoch time in milisecond + 30 minutes
        Err(_) => 0
    };
}

/*
#[test]
fn test_uniswap_swap_exact_tokens_for_tokens()
{
    let (env, token, owner) = deploy_uniswap_router();
    let user = env.next_user();
    let ret = token.swap_exact_tokens_for_tokens(200.into(), 100.into(), [].into(), user);
    println!("{:?}", ret);
}

#[test]
fn test_uniswap_swap_tokens_for_exact_tokens()
{
    let (env, token, owner) = deploy_uniswap_router();
    let user = env.next_user();
    let ret = token.swap_tokens_for_exact_tokens(200.into(), 100.into(), [].into(), user);
    println!("{:?}", ret);
}

#[test]
fn test_uniswap_swap_exact_cspr_for_tokens()
{
    let (env, token, owner) = deploy_uniswap_router();
    let user = env.next_user();
    let ret = token.swap_exact_cspr_for_tokens(200.into(), 100.into(), [].into(), user);
    println!("{:?}", ret);
}

#[test]
fn test_uniswap_swap_tokens_for_exact_cspr()
{
    let (env, token, owner) = deploy_uniswap_router();
    let user = env.next_user();
    let ret = token.swap_tokens_for_exact_cspr(200.into(), 100.into(), [].into(), user);
    println!("{:?}", ret);
}

#[test]
fn test_uniswap_swap_exact_tokens_for_cspr()
{
    let (env, token, owner) = deploy_uniswap_router();
    let user = env.next_user();
    let ret = token.swap_exact_tokens_for_cspr(200.into(), 100.into(), [].into(), user);
    println!("{:?}", ret);
}

#[test]
fn test_uniswap_swap_cspr_for_exact_tokens()
{
    let (env, token, owner) = deploy_uniswap_router();
    let user = env.next_user();
    let ret = token.swap_cspr_for_exact_tokens(200.into(), 100.into(), [].into(), user);
    println!("{:?}", ret);
}
*/