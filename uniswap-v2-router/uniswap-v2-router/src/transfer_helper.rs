pub mod transfer_helper {
    use common::{contract_api::runtime, *};

    pub fn safe_transfer(token: Key, to: Key, value: U256) {
        // Token must be approved for router to spend.
        runtime::call_versioned_contract::<()>(
            ContractPackageHash::from(token.into_hash().unwrap_or_default()),
            None,
            "transfer",
            runtime_args! {
                "recipient" => Address::from(to),
                "amount" => value
            },
        );
    }

    pub fn safe_transfer_from(token: Key, from: Key, to: Key, value: U256) {
        // Token must be approved for router to spend.
        runtime::call_versioned_contract::<()>(
            ContractPackageHash::from(token.into_hash().unwrap_or_default()),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => Address::from(from),
                "recipient" => Address::from(to),
                "amount" => value
            },
        );
    }
}
