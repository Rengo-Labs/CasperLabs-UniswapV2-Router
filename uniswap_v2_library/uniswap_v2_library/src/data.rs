use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::ContractHash;
use contract_utils::{get_key, set_key};

extern crate alloc;
use alloc::{ string::String, vec::Vec };

pub const SELF_HASH: &str = "self_hash";

pub fn self_hash() -> ContractHash { get_key(SELF_HASH).unwrap_or_revert()}
pub fn set_self_hash(contract_hash:ContractHash) { set_key(SELF_HASH, contract_hash);}

// Accepts a Contract Hash and converts it into a simple String Hash without hex(0x)|(contract-)
pub fn make_hash(contract_hash:&ContractHash) -> String {

    let formatted_hash = contract_hash.to_formatted_string();
    let splitted_hash = formatted_hash.split("-");
    let vec = splitted_hash.collect::<Vec<&str>>();
    vec[1].into()
}

// Accepts array of hashes and concats them without hex(0x)|(contract-)
pub fn encode_packed(args: &[&String]) -> String {
    
    let mut encoded_hash:String = "".into();
    for i in 0..args.len() {
        let hash = args[i];        
        encoded_hash.push_str(hash);
    }
    encoded_hash
}