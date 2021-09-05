use casper_engine_test_support::AccountHash;
use casper_types::{U256, Key, runtime_args, RuntimeArgs, contracts::{ContractHash}};
use test_env::{Sender, TestEnv, TestContract};

use crate::uniswap_instance::UniswapInstance;

const NAME: &str = "uniswap_router";

fn deploy() -> (TestEnv, UniswapInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();


    // deploy factory contract
    let env_factory = TestEnv::new();
    let owner_factory = env_factory.next_user();
    let factory_contract = TestContract::new(
        &env_factory,
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
        &env_wcspr,
        "wcspr.wasm",
        "wcspr",
        Sender(owner_wcspr),
        runtime_args! {}
    );

    // deploy library contract
    let env_library = TestEnv::new();
    let owner_library = env_library.next_user();
    let library_contract = TestContract::new(
        &env_library,
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
    
    (env, token, owner)
}

#[test]
fn test_uniswap_deploy()
{
    let (env, token, owner) = deploy();
    println!("{}", owner);
    let self_hash: Key = token.uniswap_contract_address();
    let zero_addr:Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    assert_ne!(self_hash, zero_addr);
}


/*
#[test]
fn test_erc20_deploy() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.decimals(), DECIMALS);
    assert_eq!(token.total_supply(), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}


#[test]
fn test_erc20_transfer() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.transfer(Sender(owner), user, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(user), amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_too_much() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = U256::from(INIT_TOTAL_SUPPLY) + U256::one();
    token.transfer(Sender(owner), user, amount);
}

#[test]
fn test_erc20_approve() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(Sender(owner), user, amount);
    assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_erc20_transfer_from() {
    let (env, token, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 3.into();
    token.approve(Sender(owner), spender, allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(INIT_TOTAL_SUPPLY) - amount
    );
    assert_eq!(token.balance_of(spender), 0.into());
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(token.allowance(owner, spender), allowance - amount);
}

#[test]
#[should_panic]
fn test_erc20_transfer_from_too_much() {
    let (env, token, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 12.into();
    token.approve(Sender(owner), spender, allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, owner) = deploy();
    token.constructor(
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
}
*/