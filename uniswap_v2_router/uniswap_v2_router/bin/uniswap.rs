#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, string::String, vec, vec::Vec, boxed::Box};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, ApiError, U256, bytesrepr::{Bytes}
};
use contract_utils::{ContractContext, OnChainContractStorage};
use uniswap_v2_router::{self, UniswapV2Router};
use uniswap_v2_router::config::*;


#[derive(Default)]
struct Uniswap(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Uniswap 
{
    fn storage(&self) -> &OnChainContractStorage 
    {
        &self.0
    }
}

impl UniswapV2Router<OnChainContractStorage> for Uniswap {}

impl Uniswap 
{
    fn constructor(&mut self, factory:ContractHash, wcspr: ContractHash, contract_hash: ContractHash, package_hash: ContractPackageHash,
        library_hash: ContractHash) 
    {
        UniswapV2Router::init(self, factory, wcspr, Key::from(contract_hash), package_hash, Key::from(library_hash));
    }
}

#[no_mangle]
/// Constructor to initialize required key pairs.
/// 
/// Parameters-> factory:ContractHash, contract_hash:ContractHash, package_hash:ContractHash, library_hash:ContractHash, transfer_helper_hash:ContractHash
fn constructor() 
{
    let factory: ContractHash= runtime::get_named_arg("factory");
    let wcspr: ContractHash= runtime::get_named_arg("wcspr");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let library_hash: ContractHash = runtime::get_named_arg("library_hash");

    Uniswap::default().constructor(factory, wcspr, contract_hash, package_hash, library_hash);
}

#[no_mangle]
/// Add tokens to liquidity pool.
/// 
/// Parameters-> token_a:ContractHash, token_b:ContractHash, amount_a_desired:U256, amount_b_desired:U256, amount_a_min:U256, amount_b_min:U256, to:Key, deadline: U256
fn add_liquidity() 
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let token_a:ContractHash = runtime::get_named_arg("token_a");
    let token_b:ContractHash = runtime::get_named_arg("token_b");
    let amount_a_desired:U256 = runtime::get_named_arg("amount_a_desired");
    let amount_b_desired:U256 = runtime::get_named_arg("amount_b_desired");
    let amount_a_min:U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min:U256 = runtime::get_named_arg("amount_b_min");
    let to:Key = runtime::get_named_arg("to");

    let (amount_a, amount_b, liquidity): (U256, U256, U256) = Uniswap::default().add_liquidity(token_a, token_b, amount_a_desired, amount_b_desired, amount_a_min, amount_b_min, to);
    runtime::ret(CLValue::from_t((amount_a, amount_b, liquidity)).unwrap_or_revert());
}

#[no_mangle]
/// Add cspr to liquidity pool.
/// 
/// Parameters-> token:ContractHash, amount_token_desired:U256, amount_cspr_desired:U256, amount_token_min:U256, amount_cspr_min:U256, to:Key, deadline:U256
fn add_liquidity_cspr() 
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let token:ContractHash = runtime::get_named_arg("token");
    let amount_token_desired:U256 = runtime::get_named_arg("amount_token_desired");
    let amount_cspr_desired:U256 = runtime::get_named_arg("amount_cspr_desired");
    let amount_token_min:U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min:U256 = runtime::get_named_arg("amount_cspr_min");
    let to:Key = runtime::get_named_arg("to");

    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) = Uniswap::default().add_liquidity_cspr(token, amount_token_desired, amount_cspr_desired, amount_token_min, amount_cspr_min, to);
    runtime::ret(CLValue::from_t((amount_token, amount_cspr, liquidity)).unwrap_or_revert());
}


#[no_mangle]
/// Remove from liquidity pool.
/// 
/// Parameters-> token_a:ContractHash, token_b:ContractHash, liquidity:U256, amount_a_min:U256, amount_b_min:U256, to:Key, deadline:U256
fn remove_liquidity() 
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let token_a: ContractHash = runtime::get_named_arg("token_a");
    let token_b: ContractHash = runtime::get_named_arg("token_b");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to:Key = runtime::get_named_arg("to");

    let (amount_a, amount_b) :(U256, U256) = Uniswap::default().remove_liquidity(token_a, token_b, liquidity, amount_a_min, amount_b_min, to);
    runtime::ret(CLValue::from_t((amount_a, amount_b)).unwrap_or_revert());
}

#[no_mangle]
/// Remove cspr from liquidity pool.
/// 
/// Parameters-> token:ContractHash, liquidity:U256, amount_token_min:U256, amount_cspr_min:U256, to:Key, deadline:U256
fn remove_liquidity_cspr() 
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let token: ContractHash = runtime::get_named_arg("token");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");

    let (amount_token, amount_cspr) :(U256, U256) = Uniswap::default().remove_liquidity_cspr(token, liquidity, amount_token_min, amount_cspr_min, to, deadline);
    runtime::ret(CLValue::from_t((amount_token, amount_cspr)).unwrap_or_revert());
}

#[no_mangle]
/// Remove from liquidity pool with permit.
/// 
/// Parameters-> token_a:ContractHash, token_b:ContractHash, liquidity:U256, amount_a_min:U256, amount_b_min:U256, to:Key, approve_max:bool
/// v:u8, r:Bytes, s:Bytes, deadline:U256
fn remove_liquidity_with_permit()
{
    let token_a: ContractHash = runtime::get_named_arg("token_a");
    let token_b: ContractHash = runtime::get_named_arg("token_b");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let approve_max: bool = runtime::get_named_arg("approve_max");
    let v: u8 = runtime::get_named_arg("v");
    let r: Bytes = runtime::get_named_arg("r");
    let s: Bytes = runtime::get_named_arg("s");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let (amount_a, amount_b) :(U256, U256) = Uniswap::default().remove_liquidity_with_permit(token_a, token_b, liquidity, amount_a_min, amount_b_min, 
        to, approve_max, v, r, s, deadline);

    runtime::ret(CLValue::from_t((amount_a, amount_b)).unwrap_or_revert());
}

#[no_mangle]
/// Remove cspr from liquidity pool with permit.
/// 
/// Parameters-> token:ContractHash, liquidity:U256, amount_token_min:U256, amount_cspr_min:U256, to:Key, approve_max:bool
/// v:u8, r:Bytes, s:Bytes, deadline:U256
fn remove_liquidity_cspr_with_permit() 
{
    let token: ContractHash = runtime::get_named_arg("token");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let approve_max: bool = runtime::get_named_arg("approve_max");
    let v: u8 = runtime::get_named_arg("v");
    let r: Bytes = runtime::get_named_arg("r");
    let s: Bytes = runtime::get_named_arg("s");
    let deadline: U256 = runtime::get_named_arg("deadline");

    let (amount_token, amount_cspr) :(U256, U256) = Uniswap::default().remove_liquidity_cspr_with_permit(token, liquidity, amount_token_min, amount_cspr_min, 
        to, approve_max, v, r, s, deadline);

    runtime::ret(CLValue::from_t((amount_token, amount_cspr)).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact tokens for tokens.
/// 
/// Parameters-> amount_in:U256, amount_out_min:U256, path:Vec<ContractHash>, to:Key, deadline:U256
fn swap_exact_tokens_for_tokens()
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let path: Vec<ContractHash> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");

    let amounts : Vec<U256> = Uniswap::default().swap_exact_tokens_for_tokens(amount_in, amount_out_min, path, to);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap tokens for exact tokens.
/// 
/// Parameters-> amount_out:U256, amount_in_max:U256, path:Vec<ContractHash>, to:Key, deadline:U256
fn swap_tokens_for_exact_tokens()
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<ContractHash> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");

    let amounts : Vec<U256> = Uniswap::default().swap_tokens_for_exact_tokens(amount_out, amount_in_max, path, to);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact cspr for tokens.
/// 
/// Parameters-> amount_out_min:U256, amount_in:U256, path:Vec<ContractHash>, to:Key, deadline:U256
fn swap_exact_cspr_for_tokens()
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let path: Vec<ContractHash>  = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");

    let amounts : Vec<U256> = Uniswap::default().swap_exact_cspr_for_tokens(amount_out_min, amount_in, path, to);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap tokens for exact cspr.
/// 
/// Parameters-> amount_out:U256, amount_in_max:U256, path:Vec<ContractHash>, to:Key, deadline:U256
fn swap_tokens_for_exact_cspr()
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<ContractHash>  = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");

    let amounts : Vec<U256> = Uniswap::default().swap_tokens_for_exact_cspr(amount_out, amount_in_max, path, to);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact tokens for cspr.
/// 
/// Parameters-> amount_in:U256, amount_out_min:U256, path:Vec<ContractHash>, to:Key, deadline:U256
fn swap_exact_tokens_for_cspr()
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let path: Vec<ContractHash>  = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");

    let amounts : Vec<U256> = Uniswap::default().swap_exact_tokens_for_cspr(amount_in, amount_out_min, path, to);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap cspr for exact tokens
/// 
/// Parameters-> amount_out:U256, amount_in_max:U256, path:Vec<ContractHash>, to:Key, deadline:U256
fn swap_cspr_for_exact_tokens()
{
    let deadline: U256 = runtime::get_named_arg("deadline");
    if !(Uniswap::default().ensure(deadline))
    {
        runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
    }

    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<ContractHash>  = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");

    let amounts : Vec<U256> = Uniswap::default().swap_cspr_for_exact_tokens(amount_out, amount_in_max, path, to);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}


fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("factory", ContractHash::cl_type()),
            Parameter::new("weth", ContractHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("library_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("add_liquidity"),
        vec![
            Parameter::new("token_a", ContractHash::cl_type()),
            Parameter::new("token_b", ContractHash::cl_type()),
            Parameter::new("amount_a_desired", CLType::U256),
            Parameter::new("amount_b_desired", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            ],
            CLType::Tuple3([Box::new(CLType::U256), Box::new(CLType::U256), Box::new(CLType::U256)]),
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("add_liquidity_cspr"),
        vec![
            Parameter::new("token", ContractHash::cl_type()),
            Parameter::new("amount_token_desired", CLType::U256),
            Parameter::new("amount_cspr_desired", CLType::U256),          // we don't have msg.value in casperlabs, therefore get amount_cspr_desired from parameter
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            ],
            CLType::Tuple3([Box::new(CLType::U256), Box::new(CLType::U256), Box::new(CLType::U256)]),
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity"),
        vec![
            Parameter::new("token_a", ContractHash::cl_type()),
            Parameter::new("token_b", ContractHash::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            ],
            CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity_cspr"),
        vec![
            Parameter::new("token", ContractHash::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            ],
            CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity_with_permit"),
        vec![
            Parameter::new("token_a", ContractHash::cl_type()),
            Parameter::new("token_b", ContractHash::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("approve_max", CLType::Bool),
            Parameter::new("v", CLType::U8),
            Parameter::new("r", Bytes::cl_type()),                          // bytes32
            Parameter::new("s", Bytes::cl_type())                           // bytes32
            ],
            CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_liquidity_cspr_with_permit"),

        vec![
            Parameter::new("token", ContractHash::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("approve_max", CLType::Bool),
            Parameter::new("v", CLType::U8),
            Parameter::new("r", Bytes::cl_type()),                          // bytes32
            Parameter::new("s", Bytes::cl_type())                           // bytes32
            ],
            CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_exact_tokens_for_tokens"),

        vec![
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(ContractHash::cl_type()))),
            Parameter::new("to", Key::cl_type()),
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
            Parameter::new("path", CLType::List(Box::new(ContractHash::cl_type()))),
            Parameter::new("to", Key::cl_type()),
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
            Parameter::new("path", CLType::List(Box::new(ContractHash::cl_type()))),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            ],
            CLType::List(Box::new(CLType::U256)),
            EntryPointAccess::Public,
            EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("swap_tokens_for_exact_cspr"),

        vec![
            Parameter::new("amount_out", CLType::U256),
            Parameter::new("amount_in_max", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(ContractHash::cl_type()))),
            Parameter::new("to", Key::cl_type()),
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
            Parameter::new("path", CLType::List(Box::new(ContractHash::cl_type()))),
            Parameter::new("to", Key::cl_type()),
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
            Parameter::new("path", CLType::List(Box::new(ContractHash::cl_type()))),
            Parameter::new("to", Key::cl_type()),
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
fn call() {

    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) : (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    let factory: ContractHash = runtime::get_named_arg("factory");
    let wcspr: ContractHash = runtime::get_named_arg("wcspr");
    let library_hash: ContractHash = runtime::get_named_arg("library");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "factory" => factory,
        "wcspr" => wcspr,
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "library_hash" =>  library_hash,
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
    storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
        .unwrap_or_revert()
        .pop()
        .unwrap_or_revert();

    // Call the constructor entry point
    let _: () = runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

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