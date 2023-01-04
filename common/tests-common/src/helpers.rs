use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casperlabs_test_env::{TestContract, TestEnv};
use common::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, Address, CLTyped, Key, RuntimeArgs, U256, U512,
};

pub const BALANCES: &str = "balances";
pub const TREASURY_FEE: &str = "treasury_fee";
pub const SESSION_CODE_ROUTER: &str = "session-code-router.wasm";

pub const NAME: &str = "ERC20";
pub const SYMBOL: &str = "ERC";
pub const DECIMALS: u8 = 9;
pub const INIT_TOTAL_SUPPLY: U256 = U256([0, 0, 0, 0]);
pub const AMOUNT: U256 = U256([100_000_000_000, 0, 0, 0]);
pub const AMOUNT_U512: U512 = U512([100_000_000_000, 0, 0, 0, 0, 0, 0, 0]);
pub const WRAPPED_CSPR: &str = "Wrapped CSPR";

pub fn address_to_str(owner: &Address) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(&preimage)
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &U256, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}

pub fn init(
    owner: AccountHash,
    token1: &TestContract,
    token2: &TestContract,
    uniswap_factory: &TestContract,
    uniswap_router: &TestContract,
    now: u64,
) {
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(uniswap_router.package_hash())
        },
        now,
    );
    uniswap_router.call_contract(
        owner,
        "add_to_whitelist",
        runtime_args! {
            "user" => Key::Account(owner)
        },
        now,
    );
    // Minting
    token1.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount" => AMOUNT * 2
        },
        now,
    );
    token2.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount" => AMOUNT * 2
        },
        now,
    );
    // Approving
    token1.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Address::Contract(uniswap_router.package_hash().into()),
            "amount" => AMOUNT * 2
        },
        now,
    );
    token2.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Address::Contract(uniswap_router.package_hash().into()),
            "amount" => AMOUNT * 2
        },
        now,
    );
}

pub fn call(
    env: &TestEnv,
    sender: AccountHash,
    wasm: &str,
    runtime_args: RuntimeArgs,
    time: u64,
) -> TestContract {
    TestContract::new(env, wasm, "call", sender, runtime_args, time)
}

pub fn result_key<T: CLTyped + FromBytes>(env: &TestEnv, sender: AccountHash, key: &str) -> T {
    env.query_account_named_key(sender, &[key.into()])
}
