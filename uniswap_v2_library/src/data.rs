use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::ContractHash;
use contract_utils::{get_key, set_key};

pub const SELF_HASH: &str = "self_hash";

pub fn self_hash() -> ContractHash { get_key(SELF_HASH).unwrap_or_revert()}
pub fn set_self_hash(contract_hash:ContractHash) { set_key(SELF_HASH, contract_hash);}
