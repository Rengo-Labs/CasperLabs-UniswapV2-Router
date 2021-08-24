extern crate alloc;
use alloc::{vec, vec::Vec};

use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::FromBytes, U256, api_error::ApiError,
    contracts::ContractHash, CLTyped, RuntimeArgs, runtime_args
};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self, self_hash};
use crate::config::config::ErrorCode;

pub trait UniswapV2Library<Storage: ContractStorage>: ContractContext<Storage> {
    
    // Will be called by constructor
    fn init(&mut self, contract_hash:ContractHash) {
        data::set_self_hash(contract_hash);
    }

    fn sort_tokens(&mut self, token_a:ContractHash, token_b:ContractHash) -> (ContractHash, ContractHash) {
        
        if token_a == token_b {
            runtime::revert(ApiError::from(ErrorCode::IdenticalAddresses));
        }
        let (token_0, token_1):(ContractHash, ContractHash); 
        if token_a < token_b {
            token_0 = token_a;
            token_1 = token_b;
        }
        else{
            token_0 = token_b;
            token_1 = token_a;
        }
        if token_0.to_formatted_string() == "contract-hash-0000000000000000000000000000000000000000000000000000000000000000" {
            runtime::revert(ApiError::from(ErrorCode::ZeroAddress));
        }
        (token_0, token_1)
    }
    
    // calculates the CREATE2 address for a pair without making any external calls
    fn pair_for(&mut self, factory:ContractHash, token_a:ContractHash, token_b:ContractHash) -> ContractHash {
        
        let args = runtime_args! {
            "token_a" => token_a,
            "token_b" => token_b
        };
        let (token_0, token_1):(ContractHash, ContractHash) = 
            runtime::call_contract(self_hash(), "sort_tokens", args);
        
        // let pair = address(uint(keccak256(abi.encodePacked( hex'ff', factory, keccak256(abi.encodePacked(token0, token1)), hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f' ))));
        // let hex = 
        // let pair:ContractHash;
        // pair
        token_0
    }
    
    fn get_reserves(&mut self, factory:ContractHash, token_a:ContractHash, token_b:ContractHash) -> (U256, U256) {
        
        let args = runtime_args! {
            "token_a" => token_a,
            "token_b" => token_b
        };
        let (token_0, token_1):(ContractHash, ContractHash) = 
            runtime::call_contract(self_hash(), "sort_tokens", args);
        let (reserve_0, reserve_1):(U256, U256) = (0.into(),0.into()); // IUniswapV2Pair(pairFor(factory, tokenA, tokenB)).getReserves();
        let (reserve_a, reserve_b):(U256, U256);
        if token_a == token_0 {
            reserve_a = reserve_0;
            reserve_b = reserve_1;
        }
        else{
            reserve_a = reserve_1;
            reserve_b = reserve_0;
        }
        (reserve_a, reserve_b)
    }
    
    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn quote(&mut self, amount_a:U256, reserve_a:U256, reserve_b:U256) -> U256 {
        
        if amount_a <= 0.into() {
            runtime::revert(ApiError::from(ErrorCode::InsufficientAmount));        
        }
        if reserve_a <= 0.into() || reserve_b <= 0.into() {
            runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
        }
        let amount_b: U256 = (amount_a * reserve_b) / reserve_a;
        amount_b
    }
    
    // given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    fn get_amount_out(&mut self, amount_in:U256, reserve_in:U256, reserve_out:U256) -> U256 {
        
        if amount_in <= 0.into() {
            runtime::revert(ApiError::from(ErrorCode::InsufficientInputAmount)); 
        }
        if reserve_in <= 0.into() || reserve_out <= 0.into() {
            runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
        }
        let amount_in_with_fee: U256 = amount_in * 997;
        let numerator:U256 = amount_in_with_fee * reserve_out;
        let denominator:U256 = (reserve_in * 1000) + amount_in_with_fee;
        let amount_out:U256 = numerator / denominator;
        amount_out
    }
    
    // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    fn get_amount_in(&mut self, amount_out:U256, reserve_in:U256, reserve_out:U256) -> U256 {
        
        if amount_out <= 0.into() {
            runtime::revert(ApiError::from(ErrorCode::InsufficientOutputAmount));
        }
        if reserve_in <= 0.into() || reserve_out <= 0.into() {
            runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
        }
        let numerator:U256 = reserve_in * amount_out * 1000;
        let denominator:U256 = (reserve_out - amount_out) * 997;
        let amount_in:U256 = (numerator / denominator) + 1;
        amount_in
    }
    
    // performs chained getAmountOut calculations on any number of pairs
    fn get_amounts_out(&mut self, factory:ContractHash, amount_in:U256, path: Vec<ContractHash>) -> Vec<U256> {
        
        if path.len() < 2 {
            // runtime::revert(error_codes::INVALID_PATH);
            runtime::revert(ApiError::from(ErrorCode::InsufficientLiquidity));
        }
        let mut amounts:Vec<U256> = vec![0.into(); path.len()];
        amounts[0] = amount_in;
        for i in 0..(path.len()-1) {
            let (reserve_in, reserve_out):(U256, U256) = 
                runtime::call_contract(self_hash(), "get_reserves", runtime_args! {
                    "factory" => factory,
                    "token_a" => path[i],
                    "token_b" => path[i+1]
                });
            amounts[i+1] = 
                runtime::call_contract(self_hash(), "get_amount_out", runtime_args! {
                    "amount_in" => amounts[i],
                    "reserve_in" => reserve_in,
                    "reserve_out" => reserve_out
                });
        }
        amounts
    }
    
    // performs chained getAmountIn calculations on any number of pairs
    fn get_amounts_in(&mut self, factory:ContractHash, amount_out:U256, path: Vec<ContractHash>) -> Vec<U256> {
        
        if path.len() < 2 {
            runtime::revert(ApiError::from(ErrorCode::InvalidPath));
        }
        let mut amounts:Vec<U256> = vec![0.into(); path.len()];
        let size = amounts.len();
        amounts[size-1] = amount_out;
        for i in  (1..(path.len()-1)).rev() {
            let (reserve_in, reserve_out):(U256, U256) = 
                runtime::call_contract(self_hash(), "get_reserves", runtime_args! {
                    "factory" => factory,
                    "token_a" => path[i-1],
                    "token_b" => path[i]
                });
            amounts[i-1] = 
                runtime::call_contract(self_hash(), "get_amount_in", runtime_args! {
                    "amount_in" => amounts[i],
                    "reserve_in" => reserve_in,
                    "reserve_out" => reserve_out
                });
        }
        amounts
    }    

    fn call_contract<T: CLTyped + FromBytes>(contract_hash_str: &str, method: &str, args: RuntimeArgs) -> T {
        let contract_hash = ContractHash::from_formatted_str(contract_hash_str);
        runtime::call_contract(contract_hash.unwrap_or_default(), method, args)    
    }
}
