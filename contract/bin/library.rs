#![no_main]
#![no_std]
#![feature(slice_range)]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec, prelude::v1::Box};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess,
    Group, Key, Parameter, RuntimeArgs, URef, U256, EntryPointType,
    ContractHash, EntryPoints, api_error::{ApiError}
};
use crate::vec::Vec;
#[repr(u16)]
enum ErrorCode {
    ZeroAddress = 0,
    IdenticalAddresses = 1,
    InsufficientAmount = 2,
    InsufficientInputAmount = 3,
    InsufficientOutputAmount = 4,
    InsufficientLiquidity = 5,
    InvalidPath = 6
}

impl From<ErrorCode> for ApiError {
    fn from(code: ErrorCode) -> Self {
        ApiError::User(code as u16)
    }
}

use contract_utils::{ContractContext, OnChainContractStorage};
use erc20::{self, ERC20};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl ERC20<OnChainContractStorage> for Token {}

impl Token {
    fn constructor(&mut self, contract_hash:ContractHash) {
        ERC20::init(self, name, symbol, decimals, initial_supply, contract_hash);
        ERC20::mint(self, self.get_caller(), initial_supply);
    }
}

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");            // HERE...
    Token::default().constructor(contract_hash);
}

#[no_mangle]
fn sort_tokens() {
    let token_a:ContractHash = runtime::get_named_arg("token_a");
    let token_b:ContractHash = runtime::get_named_arg("token_b");
    if token_a == token_b {
        runtime::revert(ApiError::from(ErrorCode::IdenticalAddresses));
    }
    let (mut token_0, mut token_1):(ContractHash, ContractHash); 
    if token_a < token_b {
        token_0 = token_a;
        token_1 = token_b;
    }
    else{
        token_0 = token_b;
        token_1 = token_a;
    }
    if token_0.to_formatted_string() == "contract-hash-0000000000000000000000000000000000000000000000000000000000000000" {
        runtime::revert(ApiError::from(ErrorCode::ZeroAddress));
    }
    runtime::ret(CLValue::from_t((token_0, token_1)).unwrap_or_revert())
}

// calculates the CREATE2 address for a pair without making any external calls
fn pair_for() {
    let factory:ContractHash = runtime::get_named_arg("factory");
    let token_a:ContractHash = runtime::get_named_arg("token_a");
    let token_b:ContractHash = runtime::get_named_arg("token_b");
    let constructor_args = runtime_args! {
        "token_a" => token_a,
        "token_b" => token_b
    };
    let (token_0, token_1):(ContractHash, ContractHash) = 
        runtime::call_versioned_contract(Token::default().get_hash(), None, "sort_tokens", constructor_args);
    
    // let pair = address(uint(keccak256(abi.encodePacked( hex'ff', factory, keccak256(abi.encodePacked(token0, token1)), hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f' ))));
    // let hex = 
    // let pair:ContractHash = 
}

#[no_mangle]
fn get_reserves() {
    let factory:ContractHash = runtime::get_named_arg("factory");
    let token_a:ContractHash = runtime::get_named_arg("token_a");
    let token_b:ContractHash = runtime::get_named_arg("token_b");

    let constructor_args = runtime_args! {
        "token_a" => token_a,
        "token_b" => token_b
    };
    let (token_0, token_1):(ContractHash, ContractHash) = 
        runtime::call_versioned_contract(Token::default().get_hash(), None, "sort_tokens", constructor_args);
    let (reserve_0, reserve_1):(U256, U256) = (0.into(),0.into()); // IUniswapV2Pair(pairFor(factory, tokenA, tokenB)).getReserves();
    let (mut reserve_a, mut reserve_b):(U256, U256);
    if token_a == token_0 {
        reserve_a = reserve_0;
        reserve_b = reserve_1;
    }
    else{
        reserve_a = reserve_1;
        reserve_b = reserve_0;
    }
    runtime::ret(CLValue::from_t((reserve_a, reserve_b)).unwrap_or_revert())
}

// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
fn quote() {
    let amount_a: U256 = runtime::get_named_arg("amount_a");
    let reserve_a: U256 = runtime::get_named_arg("reserve_a");
    let reserve_b: U256 = runtime::get_named_arg("reserve_b");
    if amount_a <= 0.into() {
        runtime::revert(ApiError::from(ErrorCode::InsufficientAmount));        
    }
    if reserve_a <= 0.into() || reserve_b <= 0.into() {
        runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
    }
    let amount_b: U256 = (amount_a * reserve_b) / reserve_a;
    runtime::ret(CLValue::from_t(amount_b).unwrap_or_revert())
}

// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
fn get_amount_out(){
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let reserve_in: U256 = runtime::get_named_arg("reserve_in");
    let reserve_out: U256 = runtime::get_named_arg("reserve_out");
    if amount_in <= 0.into() {
        runtime::revert(ApiError::from(ErrorCode::InsufficientInputAmount)); 
    }
    if reserve_in <= 0.into() || reserve_out <= 0.into() {
        runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
    }
    let amount_in_with_fee: U256 = amount_in * 997;
    let numerator:U256 = amount_in_with_fee * reserve_out;
    let numerator:U256 = amount_in_with_fee * reserve_out;
    let denominator:U256 = (reserve_in * 1000) + amount_in_with_fee;
    let amount_out:U256 = numerator / denominator;
    runtime::ret(CLValue::from_t(amount_out).unwrap_or_revert())
}

// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
fn get_amount_in() {
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let reserve_in: U256 = runtime::get_named_arg("reserve_in");
    let reserve_out: U256 = runtime::get_named_arg("reserve_out");
    if amount_out <= 0.into() {
        runtime::revert(ApiError::from(ErrorCode::InsufficientOutputAmount));
    }
    if reserve_in <= 0.into() || reserve_out <= 0.into() {
        runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
    }
    let numerator:U256 = reserve_in * amount_out * 1000;
    let denominator:U256 = (reserve_out - amount_out) * 997;
    let amount_in:U256 = (numerator / denominator) + 1;
    runtime::ret(CLValue::from_t(amount_in).unwrap_or_revert())
}

// performs chained getAmountOut calculations on any number of pairs
fn get_amounts_out(){
    let factory: ContractHash = runtime::get_named_arg("factory");
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let path: Vec<ContractHash> = runtime::get_named_arg("path");
    if path.len() < 2 {
        // runtime::revert(error_codes::INVALID_PATH);
        runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
    }
    let mut amounts:Vec<U256> = vec![0.into(); path.len()];
    amounts[0] = amount_in;
    for i in 0..(path.len()-1) {
        let (reserve_in, reserve_out):(U256, U256) = 
            runtime::call_versioned_contract(Token::default().get_hash(), None, "get_reserves", runtime_args! {
                "factory" => factory,
                "token_a" => path[i],
                "token_b" => path[i+1]
            });
        amounts[i+1] = 
            runtime::call_versioned_contract(Token::default().get_hash(), None, "get_amount_out", runtime_args! {
                "amount_in" => amounts[i],
                "reserve_in" => reserve_in,
                "reserve_out" => reserve_out
            });
    }
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert())
}

// performs chained getAmountIn calculations on any number of pairs
fn get_amounts_in(){
    let factory: ContractHash = runtime::get_named_arg("factory");
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let path: Vec<ContractHash> = runtime::get_named_arg("path");
    if path.len() < 2 {
        runtime::revert(ApiError::from(ErrorCode::InvalidPath));
    }
    let amounts:Vec<U256> = vec![0.into(); path.len()];
    amounts[amounts.len()-1] = amount_out;
    for i in  (1..(path.len()-1)).rev() {
        let (reserve_in, reserve_out):(U256, U256) = 
            runtime::call_versioned_contract(Token::default().get_hash(), None, "get_reserves", runtime_args! {
                "factory" => factory,
                "token_a" => path[i-1],
                "token_b" => path[i]
            });
        amounts[i-1] = 
            runtime::call_versioned_contract(Token::default().get_hash(), None, "get_amount_in", runtime_args! {
                "amount_in" => amounts[i],
                "reserve_in" => reserve_in,
                "reserve_out" => reserve_out
            });
    }
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert())
}



fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "quote",
        vec![
            Parameter::new("amount_a", Key::cl_type()),
            Parameter::new("reserve_a", Key::cl_type()),
            Parameter::new("reserve_b", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amount_out",
        vec![
            Parameter::new("amount_in", Key::cl_type()),
            Parameter::new("reserve_in", Key::cl_type()),
            Parameter::new("reserve_out", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amount_in",
        vec![
            Parameter::new("amount_out", Key::cl_type()),
            Parameter::new("reserve_in", Key::cl_type()),
            Parameter::new("reserve_out", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amounts_out",
        vec![
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("amount_in", Key::cl_type()),
            Parameter::new("path", Key::cl_type()),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_amounts_in",
        vec![
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("amount_out", Key::cl_type()),
            Parameter::new("path", Key::cl_type()),
        ],
        CLType::List(Box::new(CLType::U256)),
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
        "contract_hash" => contract_hash          // USING THIS FOR INTERNAL FUNCTION CALLS...
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