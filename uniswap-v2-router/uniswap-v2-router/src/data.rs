use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    contracts::{ContractPackageHash},
    Key,
};
use contract_utils::{get_key, set_key};

pub const WCSPR: &str = "wcspr";
pub const FACTORY: &str = "factory";
pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const LIBRARY_HASH: &str = "library_hash";

pub fn wcspr() -> ContractPackageHash {
    get_key(WCSPR).unwrap_or_revert()
}
pub fn set_wcspr(_wcspr: ContractPackageHash) {
    set_key(WCSPR, _wcspr);
}

pub fn factory() -> ContractPackageHash {
    get_key(FACTORY).unwrap_or_revert()
}
pub fn set_factory(_factory: ContractPackageHash) {
    set_key(FACTORY, _factory);
}

pub fn library_hash() -> ContractPackageHash {
    get_key(LIBRARY_HASH).unwrap_or_revert()
}
pub fn set_library_hash(library_hash: ContractPackageHash) {
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
