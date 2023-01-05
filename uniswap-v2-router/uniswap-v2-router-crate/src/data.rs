use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use common::{functions::account_zero_address, keys::*, unwrap_or_revert::UnwrapOrRevert, *};

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct Whitelist {
    dict: Dict,
}

impl Whitelist {
    pub fn instance() -> Whitelist {
        Whitelist {
            dict: Dict::instance(WHITELIST),
        }
    }

    pub fn init() {
        Dict::init(WHITELIST)
    }

    pub fn get(&self, user: &Key) -> bool {
        self.dict.get_by_key(user).unwrap_or_default()
    }

    pub fn set(&self, user: &Key, value: bool) {
        self.dict.set_by_key(user, value);
    }
}

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

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner)
}

pub fn get_owner() -> Key {
    get_key(OWNER).unwrap_or_else(account_zero_address)
}
