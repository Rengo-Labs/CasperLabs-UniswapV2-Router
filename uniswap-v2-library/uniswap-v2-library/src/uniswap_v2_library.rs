extern crate alloc;

use alloc::{vec, vec::Vec};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

use casper_types::{
    api_error::ApiError,
    bytesrepr::FromBytes,
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, Key, RuntimeArgs, U128, U256,
};
use contract_utils::{ContractContext, ContractStorage};

use crate::config::error::ErrorCode;
use crate::data::{self};

pub trait UniswapV2Library<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        data::set_self_hash(contract_hash);
        data::set_package_hash(package_hash);
    }

    fn sort_tokens(
        &mut self,
        token_a: ContractHash,
        token_b: ContractHash,
    ) -> (ContractHash, ContractHash) {
        if token_a == token_b {
            runtime::revert(ApiError::User(ErrorCode::IdenticalAddresses as u16));
        }
        let (token_0, token_1): (ContractHash, ContractHash);
        if token_a < token_b {
            token_0 = token_a;
            token_1 = token_b;
        } else {
            token_0 = token_b;
            token_1 = token_a;
        }
        if token_0.to_formatted_string()
            == "contract-hash-0000000000000000000000000000000000000000000000000000000000000000"
        {
            runtime::revert(ApiError::User(ErrorCode::ZeroAddress as u16));
        }
        (token_0, token_1)
    }

    fn pair_for(&mut self, factory: Key, token_a: Key, token_b: Key) -> Key {
        let args: RuntimeArgs = runtime_args! {
            "token0" => token_a,
            "token1" => token_b
        };

        let pair: Key = runtime::call_contract(
            ContractHash::from(factory.into_hash().unwrap_or_default()),
            "get_pair",
            args,
        );
        pair
    }

    fn get_reserves(
        &mut self,
        factory: ContractHash,
        token_a: ContractHash,
        token_b: ContractHash,
    ) -> (U128, U128) {
        let (token_0, _): (ContractHash, ContractHash) = self.sort_tokens(token_a, token_b);

        let pair: Key = self.pair_for(Key::from(factory), Key::from(token_a), Key::from(token_b));
        let pair: ContractHash = ContractHash::from(pair.into_hash().unwrap_or_default());

        let (reserve_0, reserve_1, _): (U128, U128, u64) =
            runtime::call_contract(pair, "get_reserves", runtime_args! {});
        let (reserve_a, reserve_b): (U128, U128);
        if token_a == token_0 {
            reserve_a = reserve_0;
            reserve_b = reserve_1;
        } else {
            reserve_a = reserve_1;
            reserve_b = reserve_0;
        }
        (reserve_a, reserve_b)
    }

    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn quote(&mut self, amount_a: U256, reserve_a: U128, reserve_b: U128) -> U256 {
        if amount_a <= 0.into() {
            runtime::revert(ApiError::User(ErrorCode::InsufficientAmount as u16));
        }
        if reserve_a <= 0.into() || reserve_b <= 0.into() {
            runtime::revert(ApiError::User(ErrorCode::InsufficientLiquidity as u16));
        }
        let amount_b: U256 =
            (amount_a * U256::from(reserve_b.as_u128())) / U256::from(reserve_a.as_u128());
        amount_b
    }

    // given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    fn get_amount_out(&mut self, amount_in: U256, reserve_in: U256, reserve_out: U256) -> U256 {
        if amount_in <= 0.into() {
            runtime::revert(ApiError::User(ErrorCode::InsufficientInputAmount as u16));
        }
        if reserve_in <= 0.into() || reserve_out <= 0.into() {
            runtime::revert(ApiError::User(ErrorCode::InsufficientLiquidity as u16));
        }

        let amount_in_with_fee: U256 = amount_in * 997;
        let numerator: U256 = amount_in_with_fee * reserve_out;
        let denominator: U256 = reserve_in
            .checked_mul(U256::from(1000))
            .unwrap_or_revert()
            .checked_add(amount_in_with_fee)
            .ok_or(ApiError::User(ErrorCode::Zero as u16))
            .unwrap_or_revert();
        let amount_out: U256 = numerator / denominator;

        amount_out
    }

    // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    fn get_amount_in(&mut self, amount_out: U256, reserve_in: U256, reserve_out: U256) -> U256 {
        if amount_out <= 0.into() {
            runtime::revert(ApiError::User(ErrorCode::InsufficientOutputAmount as u16));
        }
        if reserve_in <= 0.into() || reserve_out <= 0.into() {
            runtime::revert(ApiError::User(ErrorCode::InsufficientLiquidity as u16));
        }
        let numerator: U256 = reserve_in * amount_out * 1000;
        let denominator: U256 = (reserve_out - amount_out) * 997;
        let amount_in: U256 = (numerator / denominator)
            .checked_add(U256::from(1))
            .ok_or(ApiError::User(ErrorCode::Zero as u16))
            .unwrap_or_revert();

        amount_in
    }

    // performs chained getAmountOut calculations on any number of pairs
    fn get_amounts_out(
        &mut self,
        factory: ContractHash,
        amount_in: U256,
        path: Vec<ContractHash>,
    ) -> Vec<U256> {
        if path.len() < 2 {
            runtime::revert(ApiError::User(ErrorCode::InsufficientLiquidity as u16));
        }
        let mut amounts: Vec<U256> = vec![0.into(); path.len()];
        amounts[0] = amount_in;
        for i in 0..(path.len() - 1) {
            let (reserve_in, reserve_out): (U128, U128) =
                self.get_reserves(factory, path[i], path[i + 1]);

            let reserve_in: U256 = U256::from(reserve_in.as_u128());
            let reserve_out: U256 = U256::from(reserve_out.as_u128());

            amounts[i + 1] = self.get_amount_out(amounts[i], reserve_in, reserve_out);
        }
        amounts
    }

    // performs chained getAmountIn calculations on any number of pairs
    fn get_amounts_in(
        &mut self,
        factory: ContractHash,
        amount_out: U256,
        path: Vec<ContractHash>,
    ) -> Vec<U256> {
        if path.len() < 2 {
            runtime::revert(ApiError::User(ErrorCode::InvalidPath as u16));
        }
        let mut amounts: Vec<U256> = vec![0.into(); path.len()];
        let size = amounts.len();
        amounts[size - 1] = amount_out;
        for i in (1..path.len()).rev() {
            let (reserve_in, reserve_out): (U128, U128) =
                self.get_reserves(factory, path[i - 1], path[i]);

            let reserve_in: U256 = U256::from(reserve_in.as_u128());
            let reserve_out: U256 = U256::from(reserve_out.as_u128());

            amounts[i - 1] = self.get_amount_in(amounts[i], reserve_in, reserve_out);
        }
        amounts
    }

    fn call_contract<T: CLTyped + FromBytes>(
        contract_hash_str: &str,
        method: &str,
        args: RuntimeArgs,
    ) -> T {
        let contract_hash = ContractHash::from_formatted_str(contract_hash_str);
        runtime::call_contract(contract_hash.unwrap_or_default(), method, args)
    }
}
