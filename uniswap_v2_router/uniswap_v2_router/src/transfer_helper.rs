pub mod transfer_helper {

    use casper_contract::{ contract_api::{runtime}};
    use casper_types::{ contracts::{ContractHash},Key, RuntimeArgs, runtime_args, U256};
    
    pub fn safe_transfer(token: Key, to: Key, value: U256)
    {
        let args: RuntimeArgs = runtime_args!{
            "recipient" => to.into_account().unwrap_or_default(),
            "amount" => value
        };
        runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer", args)
    }

    pub fn safe_transfer_from(token: Key, from: Key, to: Key, value: U256)
    {
        let args: RuntimeArgs = runtime_args!{
            "owner" => from.into_account().unwrap_or_default(),
            "recipient" => to.into_account().unwrap_or_default(),
            "amount" => value
        };
        runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer_from", args)
    }

    pub fn safe_transfer_cspr(to: Key, value: U256)
    {
        // calls transfer method of wcspr
        let args: RuntimeArgs = runtime_args!{
            "recipient" => to,
            "amount" => value
        };
        runtime::call_contract(ContractHash::from(to.into_hash().unwrap_or_default()), "transfer_from", args)
    }
}