use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U256, U128, ContractHash};
use test_env::{Sender, TestContract, TestEnv};

pub struct LibraryInstance(TestContract);

impl LibraryInstance {

    pub fn new(env: &TestEnv, router_address: Key, library_address: Key, sender: Sender) -> LibraryInstance {
        LibraryInstance(TestContract::new(
            env,
            "contract.wasm",
            "LibraryTest",
            sender,
            runtime_args! {
                "router_address" => router_address,
                "library_address" => library_address
                // contract_name is passed seperately, so we don't need to pass it here.
            },
        ))
    }

    pub fn constructor(
        &self,
        sender: Sender,
        name: &str,
        symbol: &str,
        decimals: u8,
        initial_supply: U256,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "initial_supply" => initial_supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals
            },
        );
    }

    pub fn library_contract_address(&self) -> Key {
        let address: ContractHash = self.0.query_named_key(String::from("self_hash"));
        Key::from(address)
    }

    pub fn quote(&self, sender:Sender, amount_a: U256, reserve_a: U128, reserve_b: U128) {

        self.0.call_contract(
            sender,
            "quote",
            runtime_args! {
                "amount_a" => amount_a,
                "reserve_a" => reserve_a,
                "reserve_b" => reserve_b
            }
        );
    }

    pub fn get_reserves(&self, sender:Sender, factory: Key, token_a: Key, token_b: Key) {

        self.0.call_contract(
            sender,
            "get_reserves",
            runtime_args! {
                "factory" => factory,
                "token_a" => token_a,
                "token_b" => token_b
            }
        );
    }

    pub fn get_amount_out(&self, sender:Sender, amount_in: U256, reserve_in: U256, reserve_out: U256) {

        self.0.call_contract(
            sender,
            "get_amount_out",
            runtime_args! {
                "amount_in" => amount_in,
                "reserve_in" => reserve_in,
                "reserve_out" => reserve_out
            }
        );
    }

    pub fn get_amount_in(&self, sender:Sender, amount_out: U256, reserve_in: U256, reserve_out: U256) {

        self.0.call_contract(
            sender,
            "get_amount_in",
            runtime_args! {
                "amount_out" => amount_out,
                "reserve_in" => reserve_in,
                "reserve_out" => reserve_out
            }
        );
    }

    pub fn get_amounts_out(&self, sender:Sender, factory: Key, amount_in: U256, path: Vec<Key>) {

        self.0.call_contract(
            sender,
            "get_amounts_out",
            runtime_args! {
                "factory" => factory,
                "amount_in" => amount_in,
                "path" => path
            }
        );
    }

    pub fn get_amounts_in(&self, sender:Sender, factory: Key, amount_out: U256, path: Vec<Key>) {

        self.0.call_contract(
            sender,
            "get_amounts_in",
            runtime_args! {
                "factory" => factory,
                "amount_out" => amount_out,
                "path" => path
            }
        );
    }

    pub fn add_liquidity(
        &self,
        sender: Sender,
        token_a: Key,
        token_b: Key,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        deadline: U256,
        pair: Option<Key>
    ) {
        self.0.call_contract(
            sender,
            "add_liquidity",
            runtime_args! {
                "token_a" => token_a,
                "token_b" => token_b,
                "amount_a_desired" => amount_a_desired,
                "amount_b_desired" => amount_b_desired,
                "amount_a_min" => amount_a_min,
                "amount_b_min" => amount_b_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair
            },
        );
    }
    
    pub fn approve(&self, token: &TestContract, sender: Sender, spender: Key, amount: U256) {
        token.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount
            },
        );
    }

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
