//use alloc::string::String;

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::{ContractHash, ContractPackageHash}, Key};
use contract_utils::{get_key, set_key, Dict};

pub const WETH: &str = "weth";
pub const FACTORY: &str = "factory";
pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const LIBRARY_HASH: &str = "library_hash";
pub const TRANSFER_HELPER_HASH: &str = "transfer_helper_hash";

pub fn weth() -> ContractHash { get_key(WETH).unwrap_or_revert() }
pub fn set_weth(_weth: ContractHash) { set_key(WETH, _weth); }

pub fn factory() -> ContractHash { get_key(FACTORY).unwrap_or_revert()}
pub fn set_factory(_factory: ContractHash) { set_key(FACTORY, _factory);}

pub fn self_hash() -> Key { get_key(SELF_HASH).unwrap_or_revert()}
pub fn set_self_hash(contract_hash: Key) { set_key(SELF_HASH, contract_hash);}

pub fn package_hash() -> ContractPackageHash { get_key(PACKAGE_HASH).unwrap_or_revert()}
pub fn set_package_hash(package_hash: ContractPackageHash) { set_key(PACKAGE_HASH, package_hash);}

pub fn library_hash() -> Key { get_key(LIBRARY_HASH).unwrap_or_revert()}
pub fn set_library_hash(contract_hash: Key) { set_key(LIBRARY_HASH, contract_hash);}

pub fn transfer_helper_hash() -> Key { get_key(TRANSFER_HELPER_HASH).unwrap_or_revert()}
pub fn set_transfer_helper_hash(contract_hash: Key) { set_key(TRANSFER_HELPER_HASH, contract_hash);}