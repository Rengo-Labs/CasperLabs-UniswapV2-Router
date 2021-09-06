use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U256, ContractHash};
use test_env::{Sender, TestContract, TestEnv};

pub struct UniswapInstance(TestContract);

impl UniswapInstance {

    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        factory: Key,
        wcspr: Key, 
        library: Key,
        sender: Sender
    ) -> UniswapInstance {
        UniswapInstance(TestContract::new(
            env,
            "uniswap-v2-router.wasm",
            contract_name,
            sender,
            runtime_args! {
                "factory" => factory,
                "wcspr" => wcspr,
                "library" => library
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

    pub fn uniswap_contract_address(&self) -> Key {
        self.0.query_named_key(String::from("self_hash"))
    }

    pub fn swap_exact_tokens_for_tokens<T: Into<Key>>(&self, amount_in: U256, amount_out_min: U256, path: Vec<ContractHash>, to: T) -> Vec<U256> {
        let _to:Key = to.into();
        self.0
            .query_dictionary("swap_exact_tokens_for_tokens", _keys_to_str(&amount_in, &amount_out_min, &path, &_to))
            .unwrap_or_default()
    }

    pub fn swap_tokens_for_exact_tokens<T: Into<Key>>(&self, amount_out: U256, amount_in_max: U256, path: Vec<ContractHash>, to: T) -> Vec<U256>  {
        let _to:Key = to.into();
        self.0
            .query_dictionary("swap_tokens_for_exact_tokens", _keys_to_str(&amount_out, &amount_in_max, &path, &_to))
            .unwrap_or_default()
    }

    pub fn swap_exact_cspr_for_tokens<T: Into<Key>>(&self, amount_out_min: U256, amount_in: U256, path: Vec<ContractHash>, to: T) -> Vec<U256>  {
        let _to:Key = to.into();
        self.0
            .query_dictionary("swap_exact_cspr_for_tokens", _keys_to_str(&amount_out_min, &amount_in, &path, &_to))
            .unwrap_or_default()
    }

    pub fn swap_tokens_for_exact_cspr<T: Into<Key>>(&self, amount_out: U256, amount_in_max: U256, path: Vec<ContractHash>, to: T) -> Vec<U256> {
        let _to:Key = to.into();
        self.0
            .query_dictionary("swap_tokens_for_exact_cspr", _keys_to_str(&amount_out, &amount_in_max, &path, &_to))
            .unwrap_or_default()
    }

    pub fn swap_exact_tokens_for_cspr<T: Into<Key>>(&self, amount_in: U256, amount_out_min: U256, path: Vec<ContractHash>, to: T) -> Vec<U256> {
        let _to:Key = to.into();
        self.0
            .query_dictionary("swap_exact_tokens_for_cspr", _keys_to_str(&amount_in, &amount_out_min, &path, &_to))
            .unwrap_or_default()
    }

    pub fn swap_cspr_for_exact_tokens<T: Into<Key>>(&self, amount_out: U256, amount_in_max: U256, path: Vec<ContractHash>, to: T) -> Vec<U256>  {
        let _to:Key = to.into();
        self.0
            .query_dictionary("swap_cspr_for_exact_tokens", _keys_to_str(&amount_out, &amount_in_max, &path, &_to))
            .unwrap_or_default()
    }

    // pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
    //     self.0
    //         .query_dictionary("balances", key_to_str(&account.into()))
    //         .unwrap_or_default()
    // }

    // pub fn allowance<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
    //     let owner: Key = owner.into();
    //     let spender: Key = spender.into();
    //     self.0
    //         .query_dictionary("allowances", keys_to_str(&owner, &spender))
    //         .unwrap_or_default()
    // }

    // pub fn transfer<T: Into<Key>>(&self, sender: Sender, recipient: T, amount: U256) {
    //     self.0.call_contract(
    //         sender,
    //         "transfer",
    //         runtime_args! {
    //             "recipient" => recipient.into(),
    //             "amount" => amount
    //         },
    //     );
    // }

    // pub fn transfer_from<T: Into<Key>>(
    //     &self,
    //     sender: Sender,
    //     owner: T,
    //     recipient: T,
    //     amount: U256,
    // ) {
    //     self.0.call_contract(
    //         sender,
    //         "transfer_from",
    //         runtime_args! {
    //             "owner" => owner.into(),
    //             "recipient" => recipient.into(),
    //             "amount" => amount
    //         },
    //     );
    // }

    // pub fn approve<T: Into<Key>>(&self, sender: Sender, spender: T, amount: U256) {
    //     self.0.call_contract(
    //         sender,
    //         "approve",
    //         runtime_args! {
    //             "spender" => spender.into(),
    //             "amount" => amount
    //         },
    //     );
    // }

    // pub fn name(&self) -> String {
    //     self.0.query_named_key(String::from("name"))
    // }

    // pub fn symbol(&self) -> String {
    //     self.0.query_named_key(String::from("symbol"))
    // }

    // pub fn decimals(&self) -> u8 {
    //     self.0.query_named_key(String::from("decimals"))
    // }

    // pub fn total_supply(&self) -> U256 {
    //     self.0.query_named_key(String::from("total_supply"))
    // }
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

pub fn _keys_to_str(key_a: &U256, key_b: &U256, key_c: &Vec<ContractHash>, key_d: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    hasher.update(key_c.to_bytes().unwrap());
    hasher.update(key_d.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
