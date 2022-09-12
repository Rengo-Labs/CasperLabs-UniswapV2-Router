#![no_std]
#![no_main]

// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{string::String, vec::Vec};
use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};

pub const DESTINATION_ADD_LIQUIDITY_CSPR: &str = "add_liquidity_cspr";
pub const DESTINATION_REMOVE_LIQUIDITY_CSPR: &str = "remove_liquidity_cspr";
pub const DESTINATION_SWAP_EXACT_CSPR_FOR_TOKENS: &str = "swap_exact_cspr_for_tokens";
pub const DESTINATION_SWAP_CSPR_FOR_EXACT_TOKENS: &str = "swap_cspr_for_exact_tokens";
pub const DESTINATION_SWAP_TOKENS_FOR_EXACT_CSPR: &str = "swap_tokens_for_exact_cspr";
pub const DESTINATION_SWAP_EXACT_TOKENS_FOR_CSPR: &str = "swap_exact_tokens_for_cspr";

pub const AMOUNT_RUNTIME_ARG: &str = "amount";

#[repr(u32)]
pub enum Error {
    Abort = 0,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let destination_entrypoint: String = runtime::get_named_arg("destination_entrypoint");
    let main_purse: URef = account::get_main_purse();

    let ret: Result<(), u32> = match destination_entrypoint.as_str() {
        DESTINATION_ADD_LIQUIDITY_CSPR => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
                .unwrap_or_revert();

            let router_address: Key = runtime::get_named_arg("router_hash");
            let router_address: ContractPackageHash =
                ContractPackageHash::from(router_address.into_hash().unwrap_or_revert());

            let self_hash: Key = runtime::get_named_arg("self_hash");
            let self_hash: ContractPackageHash =
                ContractPackageHash::from(self_hash.into_hash().unwrap_or_revert());

            let token: Key = runtime::get_named_arg("token");
            let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
            let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
            let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
            let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let pair: Option<Key> = runtime::get_named_arg("pair");

            let args: RuntimeArgs = runtime_args! {
                "token" => token,
                "amount_token_desired" => amount_token_desired,
                "amount_cspr_desired" => amount_cspr_desired,
                "amount_token_min" => amount_token_min,
                "amount_cspr_min" => amount_cspr_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair,
                "purse" => secondary_purse
            };

            let (amount_token, amount_cspr, liquidity): (U256, U256, U256) =
                runtime::call_versioned_contract(
                    router_address,
                    None,
                    DESTINATION_ADD_LIQUIDITY_CSPR,
                    args,
                );

            let () = runtime::call_versioned_contract(
                self_hash,
                None,
                "set_liquidity_cspr_keys",
                runtime_args! { "amount_token" => amount_token, "amount_cspr" => amount_cspr, "liquidity" => liquidity},
            );
            Ok(())
        }
        DESTINATION_REMOVE_LIQUIDITY_CSPR => {
            let self_hash: Key = runtime::get_named_arg("self_hash");
            let self_hash: ContractPackageHash =
                ContractPackageHash::from(self_hash.into_hash().unwrap_or_revert());

            let router_address: Key = runtime::get_named_arg("router_hash");
            let router_address: ContractPackageHash =
                ContractPackageHash::from(router_address.into_hash().unwrap_or_revert());

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
                "deadline" => deadline,
                "to_purse" => main_purse
            };

            let (amount_token, amount_cspr): (U256, U256) = runtime::call_versioned_contract(
                router_address,
                None,
                "remove_liquidity_cspr",
                args,
            );

            let () = runtime::call_versioned_contract(
                self_hash,
                None,
                "set_remove_liquidity_cspr_keys",
                runtime_args! { "amount_a" => amount_token, "amount_b" => amount_cspr},
            );
            Ok(())
        }

        DESTINATION_SWAP_EXACT_CSPR_FOR_TOKENS => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
                .unwrap_or_revert();

            let router_address: Key = runtime::get_named_arg("router_hash");
            let router_address: ContractPackageHash =
                ContractPackageHash::from(router_address.into_hash().unwrap_or_revert());

            let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
            let amount_in: U256 = runtime::get_named_arg("amount_in");
            let _path: Vec<String> = runtime::get_named_arg("path");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let mut path: Vec<Key> = Vec::new();
            for i in 0..(_path.len()) {
                path.push(Key::from_formatted_str(&_path[i]).unwrap());
            }

            let args: RuntimeArgs = runtime_args! {
                "amount_out_min" => amount_out_min,
                "amount_in" => amount_in,
                "path" => _path,
                "to" => to,
                "deadline" => deadline,
                "purse" => secondary_purse
            };
            let _amounts: Vec<U256> = runtime::call_versioned_contract(
                router_address,
                None,
                DESTINATION_SWAP_EXACT_CSPR_FOR_TOKENS,
                args,
            );
            Ok(())
        }
        DESTINATION_SWAP_CSPR_FOR_EXACT_TOKENS => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
                .unwrap_or_revert();

            let router_address: Key = runtime::get_named_arg("router_hash");
            let router_address: ContractPackageHash =
                ContractPackageHash::from(router_address.into_hash().unwrap_or_revert());

            let amount_out: U256 = runtime::get_named_arg("amount_out");
            let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
            let _path: Vec<String> = runtime::get_named_arg("path");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");

            let mut path: Vec<Key> = Vec::new();
            for i in 0..(_path.len()) {
                path.push(Key::from_formatted_str(&_path[i]).unwrap());
            }

            let args: RuntimeArgs = runtime_args! {
                "amount_out" => amount_out,
                "amount_in_max" => amount_in_max,
                "path" => _path,
                "to" => to,
                "deadline" => deadline,
                "purse" => secondary_purse
            };
            let _amounts: Vec<U256> = runtime::call_versioned_contract(
                router_address,
                None,
                DESTINATION_SWAP_CSPR_FOR_EXACT_TOKENS,
                args,
            );
            Ok(())
        }
        DESTINATION_SWAP_TOKENS_FOR_EXACT_CSPR => {
            let router_address: Key = runtime::get_named_arg("router_hash");
            let router_address: ContractPackageHash =
                ContractPackageHash::from(router_address.into_hash().unwrap_or_revert());

            let amount_out: U256 = runtime::get_named_arg("amount_out");
            let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
            let _path: Vec<String> = runtime::get_named_arg("path");
            let deadline: U256 = runtime::get_named_arg("deadline");

            let mut path: Vec<Key> = Vec::new();
            for i in 0..(_path.len()) {
                path.push(Key::from_formatted_str(&_path[i]).unwrap());
            }

            let args: RuntimeArgs = runtime_args! {
                "amount_out" => amount_out,
                "amount_in_max" => amount_in_max,
                "path" => _path,
                "to" => main_purse,
                "deadline" => deadline
            };

            let _amounts: Vec<U256> = runtime::call_versioned_contract(
                router_address,
                None,
                DESTINATION_SWAP_TOKENS_FOR_EXACT_CSPR,
                args,
            );
            Ok(())
        }
        DESTINATION_SWAP_EXACT_TOKENS_FOR_CSPR => {
            let router_address: Key = runtime::get_named_arg("router_hash");
            let router_address: ContractPackageHash =
                ContractPackageHash::from(router_address.into_hash().unwrap_or_revert());

            let amount_in: U256 = runtime::get_named_arg("amount_in");
            let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
            let _path: Vec<String> = runtime::get_named_arg("path");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let mut path: Vec<Key> = Vec::new();
            for i in 0..(_path.len()) {
                path.push(Key::from_formatted_str(&_path[i]).unwrap());
            }

            let args: RuntimeArgs = runtime_args! {
                "amount_in" => amount_in,
                "amount_out_min" => amount_out_min,
                "path" => _path,
                "to" => main_purse,
                "deadline" => deadline
            };

            let _amounts: Vec<U256> = runtime::call_versioned_contract(
                router_address,
                None,
                DESTINATION_SWAP_EXACT_TOKENS_FOR_CSPR,
                args,
            );
            Ok(())
        }

        _ => Err(ApiError::UnexpectedKeyVariant.into()),
    };
    ret.unwrap_or_revert();
}
