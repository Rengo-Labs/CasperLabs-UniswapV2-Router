#![no_main]
#![no_std]
#![feature(slice_range)]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec, prelude::v1::Box};
use casper_contract::{contract_api::{runtime, storage}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, U128, U256, URef, runtime_args};
use crate::vec::Vec;
use contract_utils::{ContractContext, OnChainContractStorage};
use uniswap_v2_library::{self, UniswapV2Library};

#[derive(Default)]
struct Uniswap(OnChainContractStorage);
impl ContractContext<OnChainContractStorage> for Uniswap {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl UniswapV2Library<OnChainContractStorage> for Uniswap {}
impl Uniswap {
    fn constructor(&mut self, contract_hash:ContractHash, package_hash:ContractPackageHash) {
        UniswapV2Library::init(self, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Uniswap::default().constructor(contract_hash, package_hash);
}

#[no_mangle]
fn sort_tokens() {

    let _token_a:Key = runtime::get_named_arg("token_a");
    let _token_b:Key = runtime::get_named_arg("token_b");

    let token_a:ContractHash = _token_a.into_hash().unwrap_or_default().into();
    let token_b:ContractHash = _token_b.into_hash().unwrap_or_default().into();
    
    let (token_0, token_1) = Uniswap::default().sort_tokens(token_a, token_b);
    runtime::ret(CLValue::from_t((token_0, token_1)).unwrap_or_revert())
}

#[no_mangle]
fn get_reserves() {
    
    let _factory:Key = runtime::get_named_arg("factory");
    let _token_a:Key = runtime::get_named_arg("token_a");
    let _token_b:Key = runtime::get_named_arg("token_b");

    let token_a:ContractHash = _token_a.into_hash().unwrap_or_default().into();
    let token_b:ContractHash = _token_b.into_hash().unwrap_or_default().into();
    let factory:ContractHash = _factory.into_hash().unwrap_or_default().into();

    let (reserve_a, reserve_b) = Uniswap::default().get_reserves(factory, token_a, token_b);
    runtime::ret(CLValue::from_t((reserve_a, reserve_b)).unwrap_or_revert())
}

#[no_mangle]
// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
fn quote() {
    
    let amount_a: U256 = runtime::get_named_arg("amount_a");
    let reserve_a: U128 = runtime::get_named_arg("reserve_a");
    let reserve_b: U128 = runtime::get_named_arg("reserve_b");
    
    let amount_b: U256 = Uniswap::default().quote(amount_a, reserve_a, reserve_b);
    runtime::ret(CLValue::from_t(amount_b).unwrap_or_revert())
}

#[no_mangle]
// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
fn get_amount_out(){
    
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let reserve_in: U256 = runtime::get_named_arg("reserve_in");
    let reserve_out: U256 = runtime::get_named_arg("reserve_out");
    
    let amount_out: U256 = Uniswap::default().get_amount_out(amount_in, reserve_in, reserve_out);
    runtime::ret(CLValue::from_t(amount_out).unwrap_or_revert())
}

#[no_mangle]
// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
fn get_amount_in() {
    
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let reserve_in: U256 = runtime::get_named_arg("reserve_in");
    let reserve_out: U256 = runtime::get_named_arg("reserve_out");

    let amount_in: U256 = Uniswap::default().get_amount_in(amount_out, reserve_in, reserve_out);
    runtime::ret(CLValue::from_t(amount_in).unwrap_or_revert())
}

#[no_mangle]
// performs chained getAmountOut calculations on any number of pairs
fn get_amounts_out(){

    let _factory:Key = runtime::get_named_arg("factory");
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let _path: Vec<Key> = runtime::get_named_arg("path");

    let factory:ContractHash = _factory.into_hash().unwrap_or_default().into();
    let mut path:Vec<ContractHash> = Vec::new();
    for value in _path{
        path.push(value.into_hash().unwrap_or_default().into());
    }

    let amounts:Vec<U256> = Uniswap::default().get_amounts_out(factory, amount_in, path);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert())
}

#[no_mangle]
// performs chained getAmountIn calculations on any number of pairs
fn get_amounts_in(){

    let _factory:Key = runtime::get_named_arg("factory");
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let _path: Vec<Key> = runtime::get_named_arg("path");

    let mut path:Vec<ContractHash> = Vec::new();
    for value in _path{
        path.push(value.into_hash().unwrap_or_default().into());
    }

    let amounts:Vec<U256> = Uniswap::default().get_amounts_in(factory, amount_out, path);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert())
}

#[no_mangle]
fn pair_for() {

    let factory:Key = runtime::get_named_arg("factory");
    let token_a:Key = runtime::get_named_arg("token_a");
    let token_b:Key = runtime::get_named_arg("token_b");

    let ret = Uniswap::default().pair_for(factory, token_a, token_b);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert())
}

fn get_entry_points() -> EntryPoints {

    let mut entry_points = EntryPoints::new();
    
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "sort_tokens",
        vec![
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
        ],
        CLType::Tuple2([Box::new(ContractHash::cl_type()), Box::new(ContractHash::cl_type())]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_reserves",
        vec![
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
        ],
        CLType::Tuple2([Box::new(CLType::U128), Box::new(CLType::U128)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "quote",
        vec![
            Parameter::new("amount_a", U256::cl_type()),
            Parameter::new("reserve_a", U128::cl_type()),
            Parameter::new("reserve_b", U128::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amount_out",
        vec![
            Parameter::new("amount_in", U256::cl_type()),
            Parameter::new("reserve_in", U256::cl_type()),
            Parameter::new("reserve_out", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amount_in",
        vec![
            Parameter::new("amount_out", U256::cl_type()),
            Parameter::new("reserve_in", U256::cl_type()),
            Parameter::new("reserve_out", U256::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amounts_out",
        vec![
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("amount_in", U256::cl_type()),
            Parameter::new("path", CLType::List(Box::new(Key::cl_type()))),
        ],
        CLType::List(Box::new(U256::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amounts_in",
        vec![
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("amount_out", U256::cl_type()),
            Parameter::new("path", CLType::List(Box::new(Key::cl_type()))),
        ],
        CLType::List(Box::new(U256::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "pair_for",
        vec![
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,          // USING THIS FOR INTERNAL FUNCTION CALLS...
        "package_hash" => package_hash
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
