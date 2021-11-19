pub mod transfer_helper {
    
    extern crate alloc;

    use casper_contract::{ contract_api::{runtime}};
    use casper_types::{ contracts::{ContractHash},Key, RuntimeArgs, runtime_args, U256};


    pub fn safe_transfer(token: Key, to: Key, value: U256)// -> Result<(), u32>
    {
        // Token must be approved for router to spend.
        let args: RuntimeArgs = runtime_args!{
            "recipient" => to,
            "amount" => value
        };

        //qsp2 fix, temporarily commented out
        //let result:Result<(), u32>  = runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer", args);
        //result

        let _ : () = runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer", args);
    }

    pub fn safe_transfer_from(token: Key, from: Key, to: Key, value: U256)// -> Result<(), u32>
    {
        // Token must be approved for router to spend.
        let args: RuntimeArgs = runtime_args!{
            "owner" => from,
            "recipient" => to,
            "amount" => value
        };

        //qsp2 fix, temporary commented out
        //let result:Result<(), u32> = runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer_from", args);
        //result

        let _ : () = runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer_from", args);
    }
}