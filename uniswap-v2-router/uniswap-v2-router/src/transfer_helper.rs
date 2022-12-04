pub mod transfer_helper {

    extern crate alloc;

    use casper_contract::contract_api::runtime;
    use casper_types::{
         runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
    };

    pub fn safe_transfer(token: Key, to: Key, value: U256) -> Result<(), u32> {
        // Token must be approved for router to spend.
        let args: RuntimeArgs = runtime_args! {
            "recipient" => to,
            "amount" => value
        };

        let result: Result<(), u32> = runtime::call_versioned_contract(
            ContractPackageHash::from(token.into_hash().unwrap_or_default()),
            None,
            "transfer",
            args,
        );
        result
    }

    pub fn safe_transfer_from(token: Key, from: Key, to: Key, value: U256) -> Result<(), u32> {
        // Token must be approved for router to spend.
        let args: RuntimeArgs = runtime_args! {
            "owner" => from,
            "recipient" => to,
            "amount" => value
        };

        let result: Result<(), u32> = runtime::call_versioned_contract(
            ContractPackageHash::from(token.into_hash().unwrap_or_default()),
            None,
            "transfer_from",
            args,
        );
        result
    }
}
