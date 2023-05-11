#![no_main]

use common::{
    bytesrepr::ToBytes,
    contract_api::{account, runtime, storage, system},
    keys::*,
    unwrap_or_revert::UnwrapOrRevert,
    *,
};

// Key is the same a destination
fn store<T: CLTyped + ToBytes>(key: &str, value: T) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(key, value_key);
}

fn purse(amount: U512) -> URef {
    let main_purse: URef = account::get_main_purse();
    let secondary_purse: URef = system::create_purse();
    system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
        .unwrap_or_revert();
    secondary_purse
}

#[no_mangle]
pub extern "C" fn call() {
    let entrypoint: String = runtime::get_named_arg("entrypoint");
    let package_hash: Key = runtime::get_named_arg("package_hash");
    let package_hash: ContractPackageHash = package_hash.into_hash().unwrap_or_revert().into();
    match entrypoint.as_str() {
        DESTINATION_ADD_LIQUIDITY => {
            let token_a: Key = runtime::get_named_arg("token_a");
            let token_b: Key = runtime::get_named_arg("token_b");
            let amount_a_desired: U256 = runtime::get_named_arg("amount_a_desired");
            let amount_b_desired: U256 = runtime::get_named_arg("amount_b_desired");
            let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
            let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
            let to: Key = runtime::get_named_arg("to");
            let pair: Option<Key> = runtime::get_named_arg("pair");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: (U256, U256, U256) = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_ADD_LIQUIDITY,
                runtime_args! {
                    "token_a" => token_a,
                    "token_b" => token_b,
                    "amount_a_desired" => amount_a_desired,
                    "amount_b_desired" => amount_b_desired,
                    "amount_a_min" => amount_a_min,
                    "amount_b_min" => amount_b_min,
                    "to" => to,
                    "pair" => pair,
                    "deadline" => deadline,
                },
            );
            store(DESTINATION_ADD_LIQUIDITY, ret);
        }
        DESTINATION_ADD_LIQUIDITY_CSPR => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let token: Key = runtime::get_named_arg("token");
            let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
            let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
            let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
            let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let pair: Option<Key> = runtime::get_named_arg("pair");
            let ret: (U256, U256, U256) = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_ADD_LIQUIDITY_CSPR,
                runtime_args! {
                    "token" => token,
                    "amount_token_desired" => amount_token_desired,
                    "amount_cspr_desired" => amount_cspr_desired,
                    "amount_token_min" => amount_token_min,
                    "amount_cspr_min" => amount_cspr_min,
                    "to" => to,
                    "deadline" => deadline,
                    "pair" => pair,
                    "purse" => purse(amount)
                },
            );
            store(DESTINATION_ADD_LIQUIDITY_CSPR, ret);
        }
        DESTINATION_REMOVE_LIQUIDITY => {
            let token_a: Key = runtime::get_named_arg("token_a");
            let token_b: Key = runtime::get_named_arg("token_b");
            let liquidity: U256 = runtime::get_named_arg("liquidity");
            let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
            let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: (U256, U256) = runtime::call_versioned_contract(
                package_hash,
                None,
                "remove_liquidity",
                runtime_args! {
                    "token_a" => token_a,
                    "token_b" => token_b,
                    "liquidity" => liquidity,
                    "amount_a_min" => amount_a_min,
                    "amount_b_min" => amount_b_min,
                    "to" => to,
                    "deadline" => deadline
                },
            );
            store(DESTINATION_REMOVE_LIQUIDITY, ret);
        }
        DESTINATION_REMOVE_LIQUIDITY_CSPR => {
            let token: Key = runtime::get_named_arg("token");
            let liquidity: U256 = runtime::get_named_arg("liquidity");
            let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
            let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            // let to_purse: URef = runtime::get_named_arg("to_purse");
            let ret: (U256, U256) = runtime::call_versioned_contract(
                package_hash,
                None,
                "remove_liquidity_cspr",
                runtime_args! {
                    "token" => token,
                    "liquidity" => liquidity,
                    "amount_token_min" => amount_token_min,
                    "amount_cspr_min" => amount_cspr_min,
                    "to" => to,
                    "deadline" => deadline,
                    // "to_purse" => to_purse,
                    "to_purse" => account::get_main_purse(),
                },
            );
            store(DESTINATION_REMOVE_LIQUIDITY_CSPR, ret);
        }
        DESTINATION_SWAP_EXACT_TOKENS_FOR_TOKENS => {
            let amount_in: U256 = runtime::get_named_arg("amount_in");
            let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
            let path: Vec<String> = runtime::get_named_arg("path");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: Vec<U256> = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_SWAP_EXACT_TOKENS_FOR_TOKENS,
                runtime_args! {
                    "amount_in" => amount_in,
                    "amount_out_min" => amount_out_min,
                    "path" => path,
                    "to" => to,
                    "deadline" => deadline,
                },
            );
            store(DESTINATION_SWAP_EXACT_TOKENS_FOR_TOKENS, ret);
        }
        DESTINATION_SWAP_TOKENS_FOR_EXACT_TOKENS => {
            let amount_out: U256 = runtime::get_named_arg("amount_out");
            let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
            let path: Vec<String> = runtime::get_named_arg("path");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: Vec<U256> = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_SWAP_TOKENS_FOR_EXACT_TOKENS,
                runtime_args! {
                    "amount_out" => amount_out,
                    "amount_in_max" => amount_in_max,
                    "path" => path,
                    "to" => to,
                    "deadline" => deadline,
                },
            );
            store(DESTINATION_SWAP_TOKENS_FOR_EXACT_TOKENS, ret);
        }
        DESTINATION_SWAP_EXACT_CSPR_FOR_TOKENS => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
            let amount_in: U256 = runtime::get_named_arg("amount_in");
            let path: Vec<String> = runtime::get_named_arg("path");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: Vec<U256> = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_SWAP_EXACT_CSPR_FOR_TOKENS,
                runtime_args! {
                    "amount_out_min" => amount_out_min,
                    "amount_in" => amount_in,
                    "path" => path,
                    "to" => to,
                    "deadline" => deadline,
                    "purse" => purse(amount)
                },
            );
            store(DESTINATION_SWAP_EXACT_CSPR_FOR_TOKENS, ret);
        }
        DESTINATION_SWAP_CSPR_FOR_EXACT_TOKENS => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let amount_out: U256 = runtime::get_named_arg("amount_out");
            let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
            let path: Vec<String> = runtime::get_named_arg("path");
            let to: Key = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: Vec<U256> = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_SWAP_CSPR_FOR_EXACT_TOKENS,
                runtime_args! {
                    "amount_out" => amount_out,
                    "amount_in_max" => amount_in_max,
                    "path" => path,
                    "to" => to,
                    "deadline" => deadline,
                    "purse" => purse(amount)
                },
            );
            store(DESTINATION_SWAP_CSPR_FOR_EXACT_TOKENS, ret);
        }
        DESTINATION_SWAP_EXACT_TOKENS_FOR_CSPR => {
            let amount_in: U256 = runtime::get_named_arg("amount_in");
            let amount_out_min: U256 = runtime::get_named_arg("amount_out_min");
            let path: Vec<String> = runtime::get_named_arg("path");
            // let to: URef = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: Vec<U256> = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_SWAP_EXACT_TOKENS_FOR_CSPR,
                runtime_args! {
                    "amount_in" => amount_in,
                    "amount_out_min" => amount_out_min,
                    "path" => path,
                    // "to" => to,
                    "to" => account::get_main_purse(),
                    "deadline" => deadline
                },
            );
            store(DESTINATION_SWAP_EXACT_TOKENS_FOR_CSPR, ret);
        }
        DESTINATION_SWAP_TOKENS_FOR_EXACT_CSPR => {
            let amount_out: U256 = runtime::get_named_arg("amount_out");
            let amount_in_max: U256 = runtime::get_named_arg("amount_in_max");
            let path: Vec<String> = runtime::get_named_arg("path");
            // let to: URef = runtime::get_named_arg("to");
            let deadline: U256 = runtime::get_named_arg("deadline");
            let ret: Vec<U256> = runtime::call_versioned_contract(
                package_hash,
                None,
                DESTINATION_SWAP_TOKENS_FOR_EXACT_CSPR,
                runtime_args! {
                    "amount_out" => amount_out,
                    "amount_in_max" => amount_in_max,
                    "path" => path,
                    // "to" => to,
                    "to" => account::get_main_purse(),
                    "deadline" => deadline
                },
            );
            store(DESTINATION_SWAP_TOKENS_FOR_EXACT_CSPR, ret);
        }
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
}
