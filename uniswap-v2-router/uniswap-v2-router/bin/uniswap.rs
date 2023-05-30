#![no_main]

use std::collections::BTreeSet;

use uniswap_v2_router_crate::{
    contract_api::{runtime, storage, system},
    functions::get_purse,
    unwrap_or_revert::UnwrapOrRevert,
    *,
};

#[derive(Default)]
struct Uniswap(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Uniswap {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl UniswapV2Router<OnChainContractStorage> for Uniswap {}

impl Uniswap {
    fn constructor(
        &self,
        factory: Key,
        wcspr: Key,
        library_hash: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        UniswapV2Router::init(
            self,
            factory.into_hash().unwrap_or_default().into(),
            wcspr.into_hash().unwrap_or_default().into(),
            library_hash.into_hash().unwrap_or_default().into(),
            contract_hash,
            package_hash,
        );
    }
}

/// Constructor to initialize required key pairs
#[no_mangle]
fn constructor() {
    let factory: Key = runtime::get_named_arg("factory");
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let library_hash: Key = runtime::get_named_arg("library_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Uniswap::default().constructor(factory, wcspr, library_hash, contract_hash, package_hash);
}

#[inline(always)]
#[no_mangle]
/// Add tokens to liquidity pool.
///
/// Parameters-> token_a:Key, token_b:Key, amount_a_desired:U256, amount_b_desired:U256, amount_a_min:U256, amount_b_min:U256, to:Key, deadline: U256, pair:Option<Key> , purse:URef
fn add_liquidity() {
    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let amount_a_desired: U256 = runtime::get_named_arg("amount_a_desired");
    let amount_b_desired: U256 = runtime::get_named_arg("amount_b_desired");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let pair: Option<Key> = runtime::get_named_arg("pair");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let (amount_a, amount_b, liquidity): (U256, U256, U256) = Uniswap::default().add_liquidity(
        token_a.into_hash().unwrap_or_revert().into(),
        token_b.into_hash().unwrap_or_revert().into(),
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        to,
        pair,
        deadline,
    );
    runtime::ret(CLValue::from_t((amount_a, amount_b, liquidity)).unwrap_or_revert());
}

#[inline(always)]
#[no_mangle]
/// Add cspr to liquidity pool.
///
/// Parameters-> token:Key, amount_token_desired:U256, amount_cspr_desired:U256, amount_token_min:U256, amount_cspr_min:U256, to:Key, deadline:U256, pair:Option<Key> , purse:URef
fn add_liquidity_cspr() {
    let token: Key = runtime::get_named_arg("token");
    let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
    let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let pair: Option<Key> = runtime::get_named_arg("pair");
    let purse: URef = runtime::get_named_arg("purse");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) = Uniswap::default()
        .add_liquidity_cspr(
            token.into_hash().unwrap_or_revert().into(),
            amount_token_desired,
            amount_cspr_desired,
            amount_token_min,
            amount_cspr_min,
            to,
            pair,
            purse,
            deadline,
        );
    runtime::ret(CLValue::from_t((amount_token, amount_cspr, liquidity)).unwrap_or_revert());
}

#[inline(always)]
#[no_mangle]
/// Remove from liquidity pool.
///
/// Parameters-> token_a:Key, token_b:Key, liquidity:U256, amount_a_min:U256, amount_b_min:U256, to:Key, deadline:U256
fn remove_liquidity() {
    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let (amount_a, amount_b): (U256, U256) = Uniswap::default().remove_liquidity(
        token_a.into_hash().unwrap_or_revert().into(),
        token_b.into_hash().unwrap_or_revert().into(),
        liquidity,
        amount_a_min,
        amount_b_min,
        to,
        deadline,
    );
    runtime::ret(CLValue::from_t((amount_a, amount_b)).unwrap_or_revert());
}

#[inline(always)]
#[no_mangle]
/// Remove cspr from liquidity pool.
///
/// Parameters-> token:Key, liquidity:U256, amount_token_min:U256, amount_cspr_min:U256, to:Key, deadline:U256
fn remove_liquidity_cspr() {
    let token: Key = runtime::get_named_arg("token");
    let liquidity: U256 = runtime::get_named_arg("liquidity");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let to_purse: URef = runtime::get_named_arg("to_purse");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let (amount_token, amount_cspr): (U256, U256) = Uniswap::default().remove_liquidity_cspr(
        token.into_hash().unwrap_or_revert().into(),
        liquidity,
        amount_token_min,
        amount_cspr_min,
        to,
        to_purse,
        deadline,
    );
    runtime::ret(CLValue::from_t((amount_token, amount_cspr)).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact tokens for tokens.
///
/// Parameters-> amount_in:U256, amount_out_min:U256, path:Vec<Key>, to:Key, deadline:U256
fn swap_exact_tokens_for_tokens() {
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amounts: Vec<U256> = Uniswap::default().swap_exact_tokens_for_tokens(
        amount_in,
        amount_out_min,
        path,
        to,
        deadline,
    );
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap tokens for exact tokens.
///
/// Parameters-> amount_out:U256, amount_in_max:U256, path:Vec<Key>, to:Key, deadline:U256
fn swap_tokens_for_exact_tokens() {
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amounts: Vec<U256> = Uniswap::default().swap_tokens_for_exact_tokens(
        amount_out,
        amount_in_max,
        path,
        to,
        deadline,
    );
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact cspr for tokens.
///
/// Parameters-> amount_out_min:U256, amount_in:U256, path:Vec<Key>, to:Key, deadline:U256, purse:URef
fn swap_exact_cspr_for_tokens() {
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let purse: URef = runtime::get_named_arg("purse");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amounts: Vec<U256> = Uniswap::default().swap_exact_cspr_for_tokens(
        amount_out_min,
        amount_in,
        path,
        to,
        purse,
        deadline,
    );
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

/// Swap cspr for exact tokens
///
/// Parameters-> amount_out:U256, amount_in_max:U256, path:Vec<Key>, to:Key, deadline:U256, purse:URef

#[no_mangle]
fn swap_cspr_for_exact_tokens() {
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: Key = runtime::get_named_arg("to");
    let purse: URef = runtime::get_named_arg("purse");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amounts: Vec<U256> = Uniswap::default().swap_cspr_for_exact_tokens(
        amount_out,
        amount_in_max,
        path,
        to,
        purse,
        deadline,
    );
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap tokens for exact cspr.
///
/// Parameters-> amount_out:U256, amount_in_max:U256, path:Vec<Key>, to:Key, deadline:U256
fn swap_tokens_for_exact_cspr() {
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: URef = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amounts: Vec<U256> = Uniswap::default().swap_tokens_for_exact_cspr(
        amount_out,
        amount_in_max,
        path,
        to,
        deadline,
    );
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
/// Swap exact tokens for cspr.
///
/// Parameters-> amount_in:U256, amount_out_min:U256, path:Vec<Key>, to:Key, deadline:U256
fn swap_exact_tokens_for_cspr() {
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
    let path: Vec<String> = runtime::get_named_arg("path");
    let to: URef = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let amounts: Vec<U256> = Uniswap::default().swap_exact_tokens_for_cspr(
        amount_in,
        amount_out_min,
        path,
        to,
        deadline,
    );
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert());
}

#[no_mangle]
// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
fn quote() {
    let amount_a: U256 = runtime::get_named_arg("amount_a");
    let reserve_a: U256 = runtime::get_named_arg("reserve_a");
    let reserve_b: U256 = runtime::get_named_arg("reserve_b");

    let amount_b: U256 = Uniswap::quote(amount_a, reserve_a, reserve_b);
    runtime::ret(CLValue::from_t(amount_b).unwrap_or_revert())
}

#[no_mangle]
// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
fn get_amount_out() {
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let reserve_in: U256 = runtime::get_named_arg("reserve_in");
    let reserve_out: U256 = runtime::get_named_arg("reserve_out");
    let amount_out: U256 = Uniswap::get_amount_out(amount_in, reserve_in, reserve_out);
    runtime::ret(CLValue::from_t(amount_out).unwrap_or_revert())
}

#[no_mangle]
// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
fn get_amount_in() {
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let reserve_in: U256 = runtime::get_named_arg("reserve_in");
    let reserve_out: U256 = runtime::get_named_arg("reserve_out");
    let amount_in: U256 = Uniswap::get_amount_in(amount_out, reserve_in, reserve_out);
    runtime::ret(CLValue::from_t(amount_in).unwrap_or_revert())
}

#[no_mangle]
// performs chained getAmountOut calculations on any number of pairs
fn get_amounts_out() {
    let amount_in: U256 = runtime::get_named_arg("amount_in");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let amounts: Vec<U256> = Uniswap::get_amounts_out(amount_in, path);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert())
}

#[no_mangle]
// performs chained getAmountIn calculations on any number of pairs
fn get_amounts_in() {
    let amount_out: U256 = runtime::get_named_arg("amount_out");
    let path: Vec<Key> = runtime::get_named_arg("path");
    let amounts: Vec<U256> = Uniswap::get_amounts_in(amount_out, path);
    runtime::ret(CLValue::from_t(amounts).unwrap_or_revert())
}

#[no_mangle]
// only accept CSPR via from the WCSPR contract
fn receive() {
    let amount: U512 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");
    system::transfer_from_purse_to_purse(purse, get_purse(), amount, None).unwrap_or_revert();
}

#[no_mangle]
// change the owner for whitelisting
fn change_owner() {
    let owner: Key = runtime::get_named_arg("owner");
    Uniswap::default().change_owner(owner);
}

#[no_mangle]
// add a user to whitelist
fn add_to_whitelist() {
    let user: Key = runtime::get_named_arg("user");
    Uniswap::default().add_to_whitelist(user);
}

#[no_mangle]
// remove a user from whitelist
fn remove_from_whitelist() {
    let user: Key = runtime::get_named_arg("user");
    Uniswap::default().remove_from_whitelist(user);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("factory", CLType::Key),
            Parameter::new("wcspr", CLType::Key),
            Parameter::new("library_hash", CLType::Key),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_liquidity",
        vec![
            Parameter::new("token_a", CLType::Key),
            Parameter::new("token_b", CLType::Key),
            Parameter::new("amount_a_desired", CLType::U256),
            Parameter::new("amount_b_desired", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key))),
        ],
        CLType::Tuple3([
            Box::new(CLType::U256),
            Box::new(CLType::U256),
            Box::new(CLType::U256),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_liquidity_cspr",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("amount_token_desired", CLType::U256),
            Parameter::new("amount_cspr_desired", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("purse", CLType::URef),
        ],
        CLType::Tuple3([
            Box::new(CLType::U256),
            Box::new(CLType::U256),
            Box::new(CLType::U256),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "remove_liquidity",
        vec![
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
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
        "remove_liquidity_cspr",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("liquidity", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("to_purse", CLType::URef),
        ],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_exact_tokens_for_tokens",
        vec![
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_tokens_for_exact_tokens",
        vec![
            Parameter::new("amount_out", CLType::U256),
            Parameter::new("amount_in_max", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_exact_cspr_for_tokens",
        vec![
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("purse", CLType::URef),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_tokens_for_exact_cspr",
        vec![
            Parameter::new("amount_out", CLType::U256),
            Parameter::new("amount_in_max", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::URef),
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_exact_tokens_for_cspr",
        vec![
            Parameter::new("amount_in", CLType::U256),
            Parameter::new("amount_out_min", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::URef), // purse to transfer cspr to
            Parameter::new("deadline", CLType::U256),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "swap_cspr_for_exact_tokens",
        vec![
            Parameter::new("amount_out", CLType::U256),
            Parameter::new("amount_in_max", CLType::U256),
            Parameter::new("path", CLType::List(Box::new(String::cl_type()))),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("purse", CLType::URef),
        ],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "quote",
        vec![
            Parameter::new("amount_a", U256::cl_type()),
            Parameter::new("reserve_a", U256::cl_type()),
            Parameter::new("reserve_b", U256::cl_type()),
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
            Parameter::new("amount_out", U256::cl_type()),
            Parameter::new("path", CLType::List(Box::new(Key::cl_type()))),
        ],
        CLType::List(Box::new(U256::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "receive",
        vec![
            Parameter::new("amount", U512::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "change_owner",
        vec![Parameter::new("owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_to_whitelist",
        vec![Parameter::new("user", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "remove_from_whitelist",
        vec![Parameter::new("user", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

// All session code must have a `call` entrypoint.
#[no_mangle]
fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let factory: Key = runtime::get_named_arg("factory");
        let wcspr: Key = runtime::get_named_arg("wcspr");
        let library_hash: Key = runtime::get_named_arg("library");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "factory" => factory,
            "wcspr" => wcspr,
            "library_hash" =>  library_hash,
            "contract_hash" => contract_hash,
            "package_hash" => package_hash,
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
    } else {
        // this is a contract upgrade
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
