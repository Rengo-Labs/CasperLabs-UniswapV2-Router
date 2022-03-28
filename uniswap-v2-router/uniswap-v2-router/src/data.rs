use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    Key, URef, U512,
};
use contract_utils::{get_key, set_key};

pub const WCSPR: &str = "wcspr";
pub const FACTORY: &str = "factory";
pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const LIBRARY_HASH: &str = "library_hash";
pub const SELF_PURSE: &str = "self_purse";
pub const TOTAL_CSPR_DEPOSITED: &str = "total_cspr_deposited";

pub fn wcspr() -> ContractHash {
    get_key(WCSPR).unwrap_or_revert()
}
pub fn set_wcspr(_wcspr: ContractHash) {
    set_key(WCSPR, _wcspr);
}

pub fn factory() -> ContractHash {
    get_key(FACTORY).unwrap_or_revert()
}
pub fn set_factory(_factory: ContractHash) {
    set_key(FACTORY, _factory);
}

pub fn library_hash() -> ContractHash {
    get_key(LIBRARY_HASH).unwrap_or_revert()
}
pub fn set_library_hash(library_hash: ContractHash) {
    set_key(LIBRARY_HASH, library_hash);
}

pub fn self_hash() -> Key {
    get_key(SELF_HASH).unwrap_or_revert()
}
pub fn set_self_hash(contract_hash: Key) {
    set_key(SELF_HASH, contract_hash);
}

pub fn package_hash() -> ContractPackageHash {
    get_key(PACKAGE_HASH).unwrap_or_revert()
}
pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(PACKAGE_HASH, package_hash);
}

pub fn self_purse() -> URef {
    get_key(SELF_PURSE).unwrap_or_revert()
}
pub fn set_self_purse(self_purse: URef) {
    set_key(SELF_PURSE, self_purse);
}

pub fn total_cspr_deposited() -> U512 {
    get_key(TOTAL_CSPR_DEPOSITED).unwrap_or_revert()
}
pub fn set_total_cspr_deposited(total_cspr_deposited: U512) {
    set_key(TOTAL_CSPR_DEPOSITED, total_cspr_deposited);
}
