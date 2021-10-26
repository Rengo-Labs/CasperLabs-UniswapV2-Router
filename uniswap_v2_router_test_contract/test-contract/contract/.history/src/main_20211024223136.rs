#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage, system, account},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::Bytes,
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, ApiError, CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256, U512
};

pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let router_address: Key = runtime::get_named_arg("router_address");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::router_key(),
        ContractHash::from(router_address.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn add_liquidity() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let amount_a_desired: U256 = runtime::get_named_arg("amount_a_desired");
    let amount_b_desired: U256 = runtime::get_named_arg("amount_b_desired");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let pair: Option<Key> = runtime::get_named_arg("pair");

    let args: RuntimeArgs = runtime_args! {
        "token_a" => token_a,
        "token_b" => token_b,
        "amount_a_desired" => amount_a_desired,
        "amount_b_desired" => amount_b_desired,
        "amount_a_min" => amount_a_min,
        "amount_b_min" => amount_b_min,
        "to" => to,
        "deadline" => deadline,
        "pair" => pair
    };

    let (amount_a, amount_b, liquidity): (U256, U256, U256) =
        runtime::call_contract(router_address, "add_liquidity", args);
    mappings::set_key(
        &mappings::add_liquidity_key(),
        (amount_a, amount_b, liquidity),
    );
}

#[no_mangle]
fn add_liquidity_cspr() {

    let router_address: Key = runtime::get_named_arg("router_hash");
    let router_address: ContractHash = ContractHash::from(router_address.into_hash().unwrap_or_revert());

    let self_hash: Key = runtime::get_named_arg("self_hash");
    let self_hash: ContractHash = ContractHash::from(self_hash.into_hash().unwrap_or_revert());

    let token: Key = runtime::get_named_arg("token");
    let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
    let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let pair: Option<Key> = runtime::get_named_arg("pair");

    // get caller purse
    let caller_purse: URef = account::get_main_purse();


    let args: RuntimeArgs = runtime_args! {
        "token" => token,
        "amount_token_desired" => amount_token_desired,
        "amount_cspr_desired" => amount_cspr_desired,
        "amount_token_min" => amount_token_min,
        "amount_cspr_min" => amount_cspr_min,
        "to" => to,
        "deadline" => deadline,
        "pair" => pair,
        "purse" => caller_purse
    };

    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) =
        runtime::call_contract(router_address, "add_liquidity_cspr", args);

    // this entry points context is session therefore it can't access contract keys. Therefore to set the keys, it calls new entrypoint method.
    let _: () = runtime::call_contract(self_hash, "set_liquidity_cspr_keys", runtime_args! { "amount_token" => amount_token, "amount_cspr" => amount_cspr, "liquidity" => liquidity});
}

#[no_mangle]
fn set_liquidity_cspr_keys() {

    let amount_token: U256 = runtime::get_named_arg("amount_token");
    let amount_cspr: U256 = runtime::get_named_arg("amount_cspr");
    let liquidity: U256 = runtime::get_named_arg("liquidity");

    mappings::set_key(
        &mappings::add_liquidity_cspr_key(),
        (amount_token, amount_cspr, liquidity),
    );
}


#[no_mangle]
fn remove_liquidity() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "token_a" => token_a,
        "token_b" => token_b,
        "liquidity" => liquidity,
        "amount_a_min" => amount_a_min,
        "amount_b_min" => amount_b_min,
        "to" => to,
        "deadline" => deadline
    };

    let (amount_a, amount_b): (U256, U256) =
        runtime::call_contract(router_address, "remove_liquidity", args);
    mappings::set_key(&mappings::remove_liquidity_key(), (amount_a, amount_b));
}

#[no_mangle]
fn remove_liquidity_cspr() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let token: Key = runtime::get_named_arg("token");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "token" => token,
        "liquidity" => liquidity,
        "amount_token_min" => amount_token_min,
        "amount_cspr_min" => amount_cspr_min,
        "to" => to,
        "deadline" => deadline
    };

    let (amount_token, amount_cspr): (U256, U256) =
        runtime::call_contract(router_address, "remove_liquidity_cspr", args);
    mappings::set_key(
        &mappings::remove_liquidity_cspr_key(),
        (amount_token, amount_cspr),
    );
}

#[no_mangle]
fn remove_liquidity_with_permit() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let approve_max: bool = runtime::get_named_arg("approve_max");
    let public_key: String = runtime::get_named_arg("public_key");
    let signature: String = runtime::get_named_arg("signature");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "token_a" => token_a,
        "token_b" => token_b,
        "liquidity" => liquidity,
        "amount_a_min" => amount_a_min,
        "amount_b_min" => amount_b_min,
        "to" => to,
        "deadline" => deadline,
        "approve_max" => approve_max,
        "public_key" => public_key,
        "signature" => signature
    };

    let (amount_a, amount_b): (U256, U256) =
        runtime::call_contract(router_address, "remove_liquidity_with_permit", args);
    mappings::set_key(
        &mappings::remove_liquidity_with_permit_key(),
        (amount_a, amount_b),
    );
}

#[no_mangle]
fn remove_liquidity_cspr_with_permit() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let token: Key = runtime::get_named_arg("token");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let approve_max: bool = runtime::get_named_arg("approve_max");
    let public_key: String = runtime::get_named_arg("public_key");
    let signature: String = runtime::get_named_arg("signature");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "token" => token,
        "liquidity" => liquidity,
        "amount_token_min" => amount_token_min,
        "amount_cspr_min" => amount_cspr_min,
        "to" => to,
        "deadline" => deadline,
        "approve_max" => approve_max,
        "public_key" => public_key,
        "signature" => signature
    };

    let (amount_a, amount_b): (U256, U256) =
        runtime::call_contract(router_address, "remove_liquidity_cspr_with_permit", args);
    mappings::set_key(
        &mappings::remove_liquidity_cspr_with_permit_key(),
        (amount_a, amount_b),
    );
}

#[no_mangle]
fn swap_exact_tokens_for_tokens() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "amount_in" => amount_in,
        "amount_out_min" => amount_out_min,
        "path" => path,
        "to" => to,
        "deadline" => deadline
    };

    let amounts: Vec<U256> =
        runtime::call_contract(router_address, "swap_exact_tokens_for_tokens", args);
    mappings::set_key(&mappings::swap_exact_tokens_for_tokens(), amounts);
}

#[no_mangle]
fn swap_tokens_for_exact_tokens() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "amount_out" => amount_out,
        "amount_in_max" => amount_in_max,
        "path" => path,
        "to" => to,
        "deadline" => deadline
    };

    let amounts: Vec<U256> =
        runtime::call_contract(router_address, "swap_tokens_for_exact_tokens", args);
    mappings::set_key(&mappings::swap_tokens_for_exact_tokens(), amounts);
}

#[no_mangle]
fn swap_exact_cspr_for_tokens() {
    let router_address: Key = runtime::get_named_arg("router_hash");
    let router_address: ContractHash = ContractHash::from(router_address.into_hash().unwrap_or_revert());

    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    // create purse and send balance in it.
    //let contract_purse: URef = system::create_purse();
    let caller_purse: URef = account::get_main_purse();
    //let _ = system::transfer_from_purse_to_purse(caller_purse, contract_purse, U512::from(amount_cspr_desired.as_u128()), None);

    let args: RuntimeArgs = runtime_args! {
        "amount_out_min" => amount_out_min,
        "amount_in" => amount_in,
        "path" => path,
        "to" => to,
        "deadline" => deadline,
        "purse" => caller_purse
    };

    let amounts: Vec<U256> =
        runtime::call_contract(router_address, "swap_exact_cspr_for_tokens", args);
    //mappings::set_key(&mappings::swap_exact_cspr_for_tokens(), amounts);
}

#[no_mangle]
fn swap_tokens_for_exact_cspr() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "amount_out" => amount_out,
        "amount_in_max" => amount_in_max,
        "path" => path,
        "to" => to,
        "deadline" => deadline
    };

    let amounts: Vec<U256> =
        runtime::call_contract(router_address, "swap_tokens_for_exact_cspr", args);
    mappings::set_key(&mappings::swap_tokens_for_exact_cspr(), amounts);
}

#[no_mangle]
fn swap_exact_tokens_for_cspr() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let args: RuntimeArgs = runtime_args! {
        "amount_in" => amount_in,
        "amount_out_min" => amount_out_min,
        "path" => path,
        "to" => to,
        "deadline" => deadline
    };

    let amounts: Vec<U256> =
        runtime::call_contract(router_address, "swap_exact_tokens_for_cspr", args);
    mappings::set_key(&mappings::swap_exact_tokens_for_cspr(), amounts);
}

#[no_mangle]
fn swap_cspr_for_exact_tokens() {
    let router_address: ContractHash = mappings::get_key(&mappings::router_key());

    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");

    // create purse and send balance in it.
    let contract_purse: URef = system::create_purse();
    //let caller_purse: URef = account::get_main_purse();
    //let _ = system::transfer_from_purse_to_purse(caller_purse, contract_purse, U512::from(amount_cspr_desired.as_u128()), None);
    let purse: Option<URef> = Some(contract_purse);

    let args: RuntimeArgs = runtime_args! {
        "amount_out" => amount_out,
        "amount_in_max" => amount_in_max,
        "path" => path,
        "to" => to,
        "deadline" => deadline,
        "purse" => purse
    };

    let amounts: Vec<U256> =
        runtime::call_contract(router_address, "swap_cspr_for_exact_tokens", args);
    mappings::set_key(&mappings::swap_cspr_for_exact_tokens(), amounts);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("router_address", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("add_liquidity"),
        vec![
            Parameter::new("token_a", CLType::Key),
            Parameter::new("token_b", CLType::Key),
            Parameter::new("amount_a_desired", CLType::U256),
            Parameter::new("amount_b_desired", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key)))
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("add_liquidity_cspr"),
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("amount_token_desired", CLType::U256),
            Parameter::new("amount_cspr_desired", CLType::U256), // we don't have msg.value in casperlabs, therefore get amount_cspr_desired from parameter
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("router_hash", CLType::Key),
            Parameter::new("self_hash", CLType::Key),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("set_liquidity_cspr_keys"),
        vec![
            Parameter::new("amount_token", CLType::U256),
            Parameter::new("amount_cspr", CLType::U256), // we don't have msg.value in casperlabs, therefore get amount_cspr_desired from parameter
            Parameter::new("liquidity", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));


    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity"),
        vec![
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity_cspr"),
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity_with_permit"),
        vec![
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("approve_max", CLType::Bool),
            Parameter::new("public_key", CLType::String),
            Parameter::new("signature", CLType::String),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity_cspr_with_permit"),
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("approve_max", CLType::Bool),
            Parameter::new("public_key", CLType::String),
            Parameter::new("signature", CLType::String),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_exact_tokens_for_tokens"),
        vec![
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(CLType::Key))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_tokens_for_exact_tokens"),
        vec![
            Parameter::new("amount_out", CLType::U256),
            Parameter::new("amount_in_max", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(CLType::Key))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_exact_cspr_for_tokens"),
        vec![
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(CLType::Key))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("router_hash", CLType::Key),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_tokens_for_exact_cspr"),
        vec![
            Parameter::new("amount_out", CLType::U256),
            Parameter::new("amount_in_max", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(CLType::Key))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_exact_tokens_for_cspr"),
        vec![
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(CLType::Key))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_cspr_for_exact_tokens"),
        vec![
            Parameter::new("amount_out", CLType::U256),
            Parameter::new("amount_in_max", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(CLType::Key))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    let router_address: Key = runtime::get_named_arg("router_address");

    // Get parameters and pass it to the constructors
    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "router_address" => router_address
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
