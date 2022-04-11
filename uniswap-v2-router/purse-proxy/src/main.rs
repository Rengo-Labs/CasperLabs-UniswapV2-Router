#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};

pub const DESTINATION_DEPOSIT: &str = "deposit";
pub const DESTINATION_WITHDRAW: &str = "withdraw";
pub const DESTINATION_GET_PURSE_BALANCE: &str = "get_purse_balance";
pub const DESTINATION_ADD_LIQUIDITY_CSPR: &str = "add_liquidity_cspr";
pub const AMOUNT_RUNTIME_ARG: &str = "amount";
pub const PURSE_RUNTIME_ARG: &str = "purse";
pub const TO_PURSE_RUNTIME_ARG: &str = "to_purse";

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
    let destination_package_hash: Key = runtime::get_named_arg("destination_package_hash");
    let destination_entrypoint: String = runtime::get_named_arg("destination_entrypoint");

    let main_purse: URef = account::get_main_purse();

    let ret: Result<(), u32> = match destination_entrypoint.as_str() {
        DESTINATION_ADD_LIQUIDITY_CSPR => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
                .unwrap_or_revert();

            // let router_address: Key = runtime::get_named_arg("router_hash");
            // let router_address: ContractPackageHash =
            //     ContractPackageHash::from(router_address.into_hash().unwrap_or_revert());

            // let self_hash: Key = runtime::get_named_arg("self_hash");
            // let self_hash: ContractPackageHash =
            //     ContractPackageHash::from(self_hash.into_hash().unwrap_or_revert());

            // let token: Key = runtime::get_named_arg("token");
            // let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
            // let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
            // let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
            // let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
            // let to: Key = runtime::get_named_arg("to");
            // let deadline: U256 = runtime::get_named_arg("deadline");
            // let pair: Option<Key> = runtime::get_named_arg("pair");

            // let router_package_hash: ContractPackageHash = runtime::call_versioned_contract(
            //     router_address,
            //     None,
            //     "package_hash",
            //     runtime_args! {},
            // );

            // // Approve contract
            // let _: () = runtime::call_versioned_contract(
            //     ContractPackageHash::from(token.into_hash().unwrap_or_revert()),
            //     None,
            //     "approve",
            //     runtime_args! {
            //         "spender" => Key::from(router_package_hash),
            //         "amount" => amount_token_desired
            //     },
            // );

            // let args: RuntimeArgs = runtime_args! {
            //     "token" => token,
            //     "amount_token_desired" => amount_token_desired,
            //     "amount_cspr_desired" => amount_cspr_desired,
            //     "amount_token_min" => amount_token_min,
            //     "amount_cspr_min" => amount_cspr_min,
            //     "to" => to,
            //     "deadline" => deadline,
            //     "pair" => pair,
            //     "purse" => secondary_purse
            // };

            // let (amount_token, amount_cspr, liquidity): (U256, U256, U256) =
            //     runtime::call_versioned_contract(
            //         router_address,
            //         None,
            //         DESTINATION_ADD_LIQUIDITY_CSPR,
            //         args,
            //     );

            // this entry points context is session therefore it can't access contract keys. Therefore to set the keys, it calls new entrypoint method.
            // runtime::call_versioned_contract(
            //     self_hash,
            //     None,
            //     "set_liquidity_cspr_keys",
            //     runtime_args! { "amount_token" => amount_token, "amount_cspr" => amount_cspr, "liquidity" => liquidity},
            // );

            // let () = runtime::call_versioned_contract(
            //     self_hash,
            //     None,
            //     "set_liquidity_cspr_keys",
            //     runtime_args! { "amount_token" => amount_token, "amount_cspr" => amount_cspr, "liquidity" => liquidity},
            // );

            // runtime::call_versioned_contract(
            //     ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
            //     None,
            //     DESTINATION_WITHDRAW,
            //     runtime_args! {
            //         AMOUNT_RUNTIME_ARG => amount,
            //         TO_PURSE_RUNTIME_ARG => main_purse
            //     },
            // )

            Ok(())
        }
        DESTINATION_DEPOSIT => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            let secondary_purse: URef = system::create_purse();
            system::transfer_from_purse_to_purse(main_purse, secondary_purse, amount, None)
                .unwrap_or_revert();

            runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_DEPOSIT,
                runtime_args! {
                    AMOUNT_RUNTIME_ARG => amount,
                    PURSE_RUNTIME_ARG => secondary_purse
                },
            )
        }
        DESTINATION_WITHDRAW => {
            let amount: U512 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG);
            runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_WITHDRAW,
                runtime_args! {
                    AMOUNT_RUNTIME_ARG => amount,
                    TO_PURSE_RUNTIME_ARG => main_purse
                },
            )
        }
        DESTINATION_GET_PURSE_BALANCE => {
            let () = runtime::call_versioned_contract(
                ContractPackageHash::from(destination_package_hash.into_hash().unwrap()),
                None,
                DESTINATION_GET_PURSE_BALANCE,
                runtime_args! {
                    PURSE_RUNTIME_ARG => main_purse
                },
            );
            Ok(())
        }
        _ => Err(ApiError::UnexpectedKeyVariant.into()),
    };
    ret.unwrap_or_revert();
}
