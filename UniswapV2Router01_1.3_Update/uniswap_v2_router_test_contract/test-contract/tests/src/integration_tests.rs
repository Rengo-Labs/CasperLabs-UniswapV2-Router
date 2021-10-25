#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        Code, Error, SessionBuilder, TestContextBuilder, Value,
    };
    use casper_types::{runtime_args, RuntimeArgs, U512, account::AccountHash, PublicKey, SecretKey};

    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    // define KEY constant to match that in the contract
    const KEY: &str = "special_value";
    const VALUE: &str = "hello world";
    const ARG_MESSAGE: &str = "message";

    #[test]
    fn should_store_hello_world() {
        let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
        let public_key = PublicKey::from(&secret_key);
        let account_addr = AccountHash::from(&public_key);

        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .build();

        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
        let session_code = Code::from("contract.wasm");
        let session_args = runtime_args! {
            ARG_MESSAGE => VALUE,
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();

        let result_of_query: Result<Value, Error> =
            context.run(session).query(account_addr, &[KEY.to_string()]);

        let returned_value = result_of_query.expect("should be a value");

        let expected_value = Value::from_t(VALUE.to_string()).expect("should construct Value");
        assert_eq!(expected_value, returned_value);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
