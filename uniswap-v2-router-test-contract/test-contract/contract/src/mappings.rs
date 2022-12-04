use core::convert::TryInto;

use alloc::{format, string::String};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ApiError, CLTyped, URef,
};

const SELF_PURSE: &str = "self_purse";

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

pub fn self_hash_key() -> String {
    format!("self_hash")
}

pub fn self_package_key() -> String {
    format!("package_hash")
}

pub fn router_key() -> String {
    format!("router_hash")
}

pub fn library_key() -> String {
    format!("library_hash")
}

pub fn add_liquidity_key() -> String {
    format!("add_liquidity_result")
}

pub fn add_liquidity_cspr_key() -> String {
    format!("add_liquidity_cspr_result")
}

pub fn remove_liquidity_key() -> String {
    format!("remove_liquidity_result")
}

pub fn remove_liquidity_cspr_key() -> String {
    format!("remove_liquidity_cspr_result")
}

pub fn swap_exact_tokens_for_tokens() -> String {
    format!("swap_exact_tokens_for_tokens")
}

pub fn swap_tokens_for_exact_tokens() -> String {
    format!("swap_tokens_for_exact_tokens")
}

pub fn swap_exact_cspr_for_tokens() -> String {
    format!("swap_exact_cspr_for_tokens")
}

pub fn swap_tokens_for_exact_cspr() -> String {
    format!("swap_tokens_for_exact_cspr")
}

pub fn swap_exact_tokens_for_cspr() -> String {
    format!("swap_exact_tokens_for_cspr")
}

pub fn swap_cspr_for_exact_tokens() -> String {
    format!("swap_cspr_for_exact_tokens")
}

pub fn purse_balance() -> String {
    format!("purse_balance")
}

pub fn set_self_purse(purse: URef) {
    runtime::put_key(&SELF_PURSE, purse.into());
}

pub fn get_self_purse() -> URef {
    let destination_purse_key = runtime::get_key(&SELF_PURSE).unwrap_or_revert();

    match destination_purse_key.as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::User(40 as u16)),
    }
}
