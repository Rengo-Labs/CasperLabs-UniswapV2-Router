extern crate alloc;
use alloc::{vec::Vec, string::String, vec};

use casper_contract::{ contract_api::{runtime}};
use casper_types::{
    bytesrepr::{Bytes, FromBytes},
    contracts::{ContractHash, ContractPackageHash},Key, CLTyped, RuntimeArgs, runtime_args, BlockTime, ApiError, U256, U128};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};
use crate::config::*;
use crate::transfer_helper::transfer_helper;

pub trait UniswapV2Router<Storage: ContractStorage>: ContractContext<Storage> {
    
    // Will be called by constructor
    fn init(&mut self, factory: ContractHash, wcspr: ContractHash, library_hash: ContractHash, 
        contract_hash: Key, package_hash: ContractPackageHash, pair: ContractHash) {

        data::set_factory(factory);
        data::set_wcspr(wcspr);
        data::set_library_hash(library_hash);
        data::set_self_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_pair_hash(pair);
    }


    fn add_liquidity(&mut self, token_a: ContractHash, token_b: ContractHash, amount_a_desired: U256,
        amount_b_desired: U256, amount_a_min: U256, amount_b_min: U256, to: Key
    ) -> (U256, U256, U256)
    {
        
        let factory: ContractHash = data::factory();
        let (amount_a, amount_b) : (U256, U256) = Self::_add_liquidity(token_a, token_b, amount_a_desired, amount_b_desired, amount_a_min, amount_b_min);
        
        // call pair_for from library contract
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs= runtime_args! {
            "factory" => Key::from(factory),
            "token_a" => Key::from(token_a),
            "token_b" => Key::from(token_b)
        };
        let pair:Key = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        let pair:ContractHash = ContractHash::from(pair.into_hash().unwrap_or_default());               // convert key into ContractHash
        
        transfer_helper::safe_transfer_from(Key::from(token_a), Key::from(runtime::get_caller()), Key::from(pair), amount_a);
        transfer_helper::safe_transfer_from(Key::from(token_b), Key::from(runtime::get_caller()), Key::from(pair), amount_b);
        
        // call mint function from IUniswapV2Pair contract
        let args: RuntimeArgs= runtime_args!{
            "to" => to,
        };
        
        let liquidity:U256 = Self::call_contract(&pair.to_formatted_string(), uniswapv2_contract_methods::PAIR_MINT, args);
        (amount_a, amount_b, liquidity)
    }


    fn add_liquidity_cspr(&mut self, token: ContractHash, amount_token_desired: U256, amount_cspr_desired: U256,
        amount_token_min: U256, amount_cspr_min: U256, to: Key
    ) -> (U256, U256, U256)
    {
        let wcspr: ContractHash = data::wcspr();
        let factory: ContractHash = data::factory();
        
        let (amount_token, amount_cspr) : (U256, U256) = Self::_add_liquidity(token, wcspr, amount_token_desired, amount_cspr_desired, amount_token_min, amount_cspr_min);

        
        // call pair_for from library contract
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs= runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => Key::from(token),
            "token_b" => Key::from(wcspr)
        };
        let pair: Key = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        let pair: ContractHash = ContractHash::from(pair.into_hash().unwrap_or_default());               // convert key into ContractHash

        
        // call safe_transfer_from from TransferHelper
        transfer_helper::safe_transfer_from(Key::from(token), Key::from(runtime::get_caller()), Key::from(pair), amount_token);
        
        
        // call deposit method from wcspr
        let args: RuntimeArgs = runtime_args!{
            "to" => Key::from(data::package_hash()),               // returns package hash of this contract
            "amount" => amount_cspr
        };
        let () = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_DEPOSIT, args);
        
        // call transfer method from wcspr
        let args: RuntimeArgs = runtime_args!{
            "recipient" => Key::from(pair),
            "amount" => amount_cspr
        };
        let () = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_TRANSFER, args);
        
        // call mint function from pair contract
        let args: RuntimeArgs= runtime_args!{
            "to" => to,
        };        
        
        let liquidity:U256 = Self::call_contract(&pair.to_formatted_string(), uniswapv2_contract_methods::PAIR_MINT, args);
    
        if amount_cspr_desired > amount_cspr                         // refund left-over cspr, if any
        {
            transfer_helper::safe_transfer_cspr(Key::from(runtime::get_caller()), amount_cspr_desired - amount_cspr);
        }
        
        (amount_token, amount_cspr, liquidity)
    }


    fn remove_liquidity(&mut self, token_a: ContractHash, token_b: ContractHash, liquidity: U256, amount_a_min: U256,
        amount_b_min: U256, to: Key
    ) -> (U256, U256)
    {
        let factory: ContractHash = data::factory();

        // call pair_for from library contract
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs= runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => Key::from(token_a),
            "token_b" => Key::from(token_b)
        };
        let pair: Key = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        let pair: ContractHash = ContractHash::from(pair.into_hash().unwrap_or_default());               // convert key into ContractHash

        // call transferFrom from IUniSwapV2Pair
        let args: RuntimeArgs = runtime_args!{
            "owner" => Key::from(runtime::get_caller()),
            "recipient" => Key::from(pair),
            "amount" => liquidity
        };
        let _: () = Self::call_contract(&pair.to_formatted_string(), uniswapv2_contract_methods::PAIR_TRANSFER_FROM, args);

        
        // call burn from IUniSwapV2Pair
        let args: RuntimeArgs = runtime_args!{
            "to" => to,
        };
        let (amount0, amount1):(U256, U256) = Self::call_contract(&pair.to_formatted_string(), uniswapv2_contract_methods::PAIR_BURN, args);

        
        // call sortTokens from library contract
        let args: RuntimeArgs = runtime_args!{
            "token_a" => Key::from(token_a),
            "token_b" => Key::from(token_b)
        };

        let (token0, _):(ContractHash, ContractHash) = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_SORT_TOKENS, args);
        let (amount_a, amount_b):(U256, U256) = if token_a == token0 {(amount0, amount1)} else {(amount1, amount0)};

        if amount_a >= amount_a_min && amount_b >= amount_b_min {
            return (amount_a, amount_b)
        }else{
            return (0.into(),0.into())
        }
    }


    fn remove_liquidity_cspr(&mut self, token: ContractHash, liquidity: U256, amount_token_min: U256, amount_cspr_min: U256, 
        to: Key, deadline: U256) -> (U256, U256) {
        
        // calling self contract's removeLiquidity
        let self_hash: Key = data::self_hash();
        let wcspr: ContractHash = data::wcspr();

        let args: RuntimeArgs = runtime_args!{
            "token_a" => Key::from(token),
            "token_b" => Key::from(wcspr),
            "liquidity" => liquidity,
            "amount_a_min" => amount_token_min,
            "amount_b_min" => amount_cspr_min,
            "to" => self_hash,
            "deadline" => deadline
        };
        
        let package_hash = data::package_hash();
        let (amount_token, amount_cspr):(U256, U256) = runtime::call_versioned_contract(package_hash, None, "remove_liquidity", args);
        
        transfer_helper::safe_transfer(Key::from(token), to, amount_token);


        // call withdraw from WCSPR
        let args: RuntimeArgs = runtime_args!{
            "from" => Key::from(wcspr),
            "amount" => amount_cspr
        };
        let () = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_WITHDRAW, args);

        
        // call deposit from cspr
        transfer_helper::safe_transfer_cspr(to, amount_cspr);

        (amount_token, amount_cspr)
    }


    fn remove_liquidity_with_permit(&mut self, token_a: ContractHash, token_b: ContractHash, liquidity: U256, amount_a_min: U256, amount_b_min: U256, 
        to: Key, approve_max: bool, public_key: String, signature: String , deadline: U256) -> (U256, U256)
    {
        let factory: ContractHash = data::factory();

        // call pair_for method from uniswapv2Library
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => Key::from(token_a),
            "token_b" => Key::from(token_b)
        };
        let pair: Key = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        let zero_addr: Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        if pair == zero_addr {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

        let pair: ContractHash = ContractHash::from(pair.into_hash().unwrap_or_default());               // convert key into ContractHash
        let value: U256 = if approve_max {U256::MAX} else {liquidity};

        // call permit from uniswapv2pair
        let args: RuntimeArgs = runtime_args!{
            "public" => public_key,
            "signature" => signature,
            "owner" => Key::from(runtime::get_caller()),
            "spender" => Key::from(data::package_hash()),
            "value" => value,
            "deadline" => deadline.as_u64()
        };
        
        let () = Self::call_contract(&pair.to_formatted_string(), uniswapv2_contract_methods::PAIR_PERMIT, args);
        
        // call self remove_liquidity
        let args: RuntimeArgs = runtime_args!{
            "token_a" => Key::from(token_a),
            "token_b" => Key::from(token_b),
            "liquidity" => liquidity,
            "amount_a_min" => amount_a_min,
            "amount_b_min" => amount_b_min,
            "to" => to,
            "deadline" => deadline
        };
        
        let package_hash = data::package_hash();
        let (amount_a, amount_b):(U256, U256) = runtime::call_versioned_contract(package_hash, None, "remove_liquidity", args);
        (amount_a, amount_b)
    }


    fn remove_liquidity_cspr_with_permit(&mut self, token: ContractHash, liquidity: U256, amount_token_min: U256, amount_cspr_min: U256, 
        to: Key, approve_max: bool, public_key: String, signature: String , deadline: U256) -> (U256, U256)
    {
        let factory: ContractHash = data::factory();
        let wcspr: ContractHash = data::wcspr();

        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => Key::from(token),
            "token_b" => Key::from(wcspr)
        };
        let pair: Key = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        let zero_addr: Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        if pair == zero_addr {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

        let pair: ContractHash = ContractHash::from(pair.into_hash().unwrap_or_default());               // convert key into ContractHash
        let value: U256 = if approve_max {U256::MAX} else {liquidity};
        
        // call permit from uniswapv2pair
        let args: RuntimeArgs = runtime_args!{
            "public" => public_key,
            "signature" => signature,
            "owner" => Key::from(runtime::get_caller()),
            "spender" => Key::from(data::package_hash()),
            "value" => value,
            "deadline" => deadline.as_u64()
        };
        let () = Self::call_contract(&pair.to_formatted_string(), uniswapv2_contract_methods::PAIR_PERMIT, args);        
       
        // call remove_liquidity_cspr
        let args: RuntimeArgs = runtime_args!{
            "token" => Key::from(token),
            "liquidity" => liquidity,
            "amount_token_min" => amount_token_min,
            "amount_cspr_min" => amount_cspr_min,
            "to" => to,
            "deadline" => deadline
        };
        
        
        let package_hash = data::package_hash();
        let (amount_token, amount_cspr):(U256, U256) = runtime::call_versioned_contract(package_hash, None, "remove_liquidity_cspr", args);
        
        (amount_token, amount_cspr)
    }


    fn swap_exact_tokens_for_tokens(&mut self, amount_in: U256, amount_out_min: U256, path: Vec<Key>, to: Key) -> Vec<U256>
    {
        
        let factory: ContractHash = data::factory();

        // call getAmountOut from Library contract
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "amount_in" => amount_in,
            "path" => path.clone(),
        };
        let amounts: Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_OUT, args);

        if amounts[amounts.len() - 1] < amount_out_min
        {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

        // get pair
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => path[0],
            "token_b" => path[1],
        };
        let pair: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        
        // transfer_helper::safe_transfer_from(Key::from(path[0]), Key::from(runtime::get_caller()), Key::from(pair), amounts[0]);

        // // call _swap helper
        // Self::_swap(&amounts, &path, to);
        // amounts
        
        let amounts: Vec<U256> = vec![1.into(), 2.into()];
        amounts
    }

    fn swap_tokens_for_exact_tokens(&mut self, amount_out: U256, amount_in_max: U256, path: Vec<Key>, to: Key) -> Vec<U256>
    {
        let factory: ContractHash = data::factory();

        // call getAmountIn from Library contract

        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "amount_out" => amount_out,
            "path" => path.clone(),
        };
        let amounts: Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_IN, args);

        if amounts[0] > amount_in_max
        {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

        // Get pair
        let args: RuntimeArgs = runtime_args!{
            "factory" => factory,
            "token_a" => path[0],
            "token_b" => path[1],
        };
        let pair: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        transfer_helper::safe_transfer_from(Key::from(path[0]), Key::from(runtime::get_caller()), Key::from(pair), amounts[0]);

        Self::_swap(&amounts, &path, to);

        amounts
    }


    fn swap_exact_cspr_for_tokens(&mut self, amount_out_min: U256, amount_in: U256, path: Vec<Key>, to: Key) -> Vec<U256>
    {
        let wcspr: ContractHash = data::wcspr();
        let factory: ContractHash = data::factory();

        if !(path[0] == Key::from(wcspr)) {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

        // call get_amounts_out
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "amount_in" => amount_in,
            "path" => path.clone(),
        };
        let amounts: Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_OUT, args);
        
        if amounts[amounts.len() - 1] < amount_out_min {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

            
        // call deposit method from wcspr
        let args: RuntimeArgs = runtime_args!{
            "to" => Key::from(wcspr),
            "amount" => amounts[0]
        };
        let () = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_DEPOSIT, args);


        // call transfer method from IWETH
        
        // Get pair
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => path[0],
            "token_b" => path[1],
        };
        let pair: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        
        let args: RuntimeArgs = runtime_args!{
            "recipient" => Key::from(pair),
            "amount" => amounts[0]
        };
        let transfer_result: bool = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_TRANSFER, args);
        
        if !transfer_result {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }
        Self::_swap(&amounts, &path, to);

        amounts
    }


    fn swap_tokens_for_exact_cspr(&mut self, amount_out: U256, amount_in_max: U256, path: Vec<Key>, to: Key) -> Vec<U256>
    {
        let wcspr: ContractHash = data::wcspr();
        let factory: ContractHash = data::wcspr();
        let self_addr: Key = data::self_hash();

        if !(path[path.len() - 1] == Key::from(wcspr)){
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

        // call getAmountIn from Library contract
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "amount_out" => amount_out,
            "path" => path.clone(),
        };
        let amounts: Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_IN, args);

        if amounts[0] > amount_in_max
        {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }


        // call safeTransferFrom from TransferHelper

        // first need to get the pair
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => path[0],
            "token_b" => path[1],
        };
        let pair: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        transfer_helper::safe_transfer_from(Key::from(path[0]), Key::from(runtime::get_caller()), Key::from(pair), amounts[0]);
        Self::_swap(&amounts, &path, self_addr);


        // call withdraw from WCSPR
        let args: RuntimeArgs = runtime_args!{
            "from" => Key::from(wcspr),
            "amount" => amounts[0]
        };
        let _: () = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_WITHDRAW, args);
        

        /*
        // call safe_transfer_cspr from TransferHelper contract
        let args: RuntimeArgs = runtime_args!{
            "to" => to,
            "value" => amounts[amounts.len() - 1]
        };
        let () = Self::call_contract(&uniswapv2_transfer_helper_contract_hash, uniswapv2_contract_methods::TRANSFER_HELPER_SAFE_TRANSFER_CSPR, args);    
        */
        transfer_helper::safe_transfer_cspr(to, amounts[amounts.len() - 1]);
        amounts
    }


    fn swap_exact_tokens_for_cspr(&mut self, amount_in: U256, amount_out_min: U256, path: Vec<Key>, to: Key) -> Vec<U256>
    {
        let wcspr: ContractHash = data::wcspr();
        let factory: ContractHash = data::factory();
        let self_addr: Key = data::self_hash();

        if !(path[path.len() - 1] == Key::from(wcspr)){
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }
    
        // call get_amounts_out
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "amount_in" => amount_in,
            "path" => path.clone(),
        };
        let amounts: Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_OUT, args);
    
        if amounts[amounts.len() - 1] < amount_out_min {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }
        
    
        // call safeTransferFrom from TransferHelper
    
        // first need to get the pair
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => path[0],
            "token_b" => path[1],
        };
        let pair: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
    
        /*
        //let uniswapv2_transfer_helper_contract_hash: &str = uniswapv2_contracts_hash::TRANSFER_HELPER_HASH;
        let uniswapv2_transfer_helper_contract_hash = data::transfer_helper_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "token" => path[0],
            "from" => runtime::get_caller(),
            "to" => pair,
            "value" => amounts[0]
        };
        let () = Self::call_contract(&uniswapv2_transfer_helper_contract_hash, uniswapv2_contract_methods::TRANSFER_HELPER_SAFE_TRANSFER_FROM, args);
        */
        transfer_helper::safe_transfer_from(Key::from(path[0]), Key::from(runtime::get_caller()), Key::from(pair), amounts[0]);
        Self::_swap(&amounts, &path, self_addr);
    
        // call withdraw from WCSPR
        let args: RuntimeArgs = runtime_args!{
            "from" => Key::from(wcspr),
            "amount" => amounts[amounts.len() - 1]
        };
        let _: () = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_WITHDRAW, args);
    
        
        /*
        // call safe_transfer_cspr from TransferHelper contract
        let args: RuntimeArgs = runtime_args!{
            "to" => to,
            "value" => amounts[amounts.len() - 1]
        };
        let () = Self::call_contract(&uniswapv2_transfer_helper_contract_hash, uniswapv2_contract_methods::TRANSFER_HELPER_SAFE_TRANSFER_CSPR, args);
        */
        transfer_helper::safe_transfer_cspr(to, amounts[amounts.len() - 1]);
        amounts
    }

    fn swap_cspr_for_exact_tokens (&mut self, amount_out: U256, amount_in_max: U256, path: Vec<Key>, to: Key) -> Vec<U256>
    {
        let wcspr: ContractHash = data::wcspr();
        let factory: ContractHash = data::factory();

        if !(path[0] == Key::from(wcspr)){
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }

        // call get_amounts_out
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "amount_in" => amount_out,
            "path" => path.clone(),
        };
        let amounts: Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_IN, args);
        
        if amounts[0] > amount_in_max {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }


        // call deposit method from wcspr
        let args: RuntimeArgs = runtime_args!{
            "to" => Key::from(wcspr),
            "amount" => amounts[0]
        };
        let () = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_DEPOSIT, args);

        

        // call transfer method from wcspr

        // Get pair
        let args: RuntimeArgs = runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => path[0],
            "token_b" => path[1],
        };
        let pair: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
        
        let args: RuntimeArgs = runtime_args!{
            "recipient" => pair,
            "amount" => amounts[0]
        };
        let transfer_result: bool = Self::call_contract(&wcspr.to_formatted_string(), uniswapv2_contract_methods::WCSPR_TRANSFER, args);
        
        if !transfer_result {
            runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
        }
        Self::_swap(&amounts, &path, to);

        if amount_in_max > amounts[0] 
        {
            /*
            //let uniswapv2_transfer_helper_contract_hash: &str = uniswapv2_contracts_hash::TRANSFER_HELPER_HASH;
            let uniswapv2_transfer_helper_contract_hash = data::transfer_helper_hash().to_formatted_string();
            let args: RuntimeArgs = runtime_args!{
                "to" => runtime::get_caller(),
                "value" => amount_in_max - amounts[0]
            };

            let () = Self::call_contract(&uniswapv2_transfer_helper_contract_hash, uniswapv2_contract_methods::TRANSFER_HELPER_SAFE_TRANSFER_CSPR, args);
            */
            transfer_helper::safe_transfer_cspr(Key::from(runtime::get_caller()), amount_in_max - amounts[0]);
        }

        amounts
    }
    

    fn quote(amount_a: U256, reserve_a: U256, reserve_b: U256) -> U256
    {
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "amount_a" => amount_a,
            "reserve_a" => reserve_a,
            "reserve_b" => reserve_b
        };

        let amount_b : U256 = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_QUOTE, args);
        amount_b
    }

    fn get_amount_out(amount_in: U256, reserve_in: U256, reserve_out: U256) -> U256
    {
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "amount_in" => amount_in,
            "reserve_in" => reserve_in,
            "reserve_out" => reserve_out
        };

        let amount_out : U256 = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNT_OUT, args);
        amount_out
    }

    fn get_amount_in(amount_out: U256, reserve_in: U256, reserve_out: U256) -> U256
    {
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs = runtime_args!{
            "amount_out" => amount_out,
            "reserve_in" => reserve_in,
            "reserve_out" => reserve_out
        };

        let amount_in : U256 = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNT_IN, args);
        amount_in
    }

    fn get_amounts_out(amount_in: U256, path: Vec<ContractHash>) -> Vec<U256> 
    {
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let factory: ContractHash = data::factory();

        let args: RuntimeArgs = runtime_args!{
            "factory" => factory,
            "amount_in" => amount_in,
            "path" => path
        };

        let amounts_out : Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_OUT, args);
        amounts_out
    }

    fn get_amounts_in(amount_out: U256, path: Vec<ContractHash>) -> Vec<U256> 
    {
        //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let factory: ContractHash = data::factory();

        let args: RuntimeArgs = runtime_args!{
            "factory" => factory,
            "amount_out" => amount_out,
            "path" => path
        };

        let amounts_in : Vec<U256> = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_AMOUNTS_IN, args);
        amounts_in
    }

    // *************************************** Helper methods ****************************************

    fn _add_liquidity(token_a: ContractHash, token_b: ContractHash, amount_a_desired: U256, amount_b_desired: U256, amount_a_min: U256, amount_b_min: U256) -> (U256, U256)
    {
        let factory: ContractHash = data::factory();
        let args: RuntimeArgs = runtime_args! {
            "token0" => Key::from(token_a),
            "token1" => Key::from(token_b)
        };
        let pair: Key = Self::call_contract(&factory.to_formatted_string(), uniswapv2_contract_methods::FACTORY_GET_PAIR, args);
    
        let zero_addr: Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        if pair == zero_addr
        {
            // if pair doesnot exist, create one
            let args = runtime_args!{
                "token_a" => Key::from(token_a),
                "token_b" => Key::from(token_b),
                "pair_hash" => Key::from(data::pair_hash())
            };
            let _: () = Self::call_contract(&factory.to_formatted_string(), uniswapv2_contract_methods::FACTORY_CREATE_PAIR, args);
        }
    
        let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
        let args: RuntimeArgs= runtime_args!{
            "factory" => Key::from(factory),
            "token_a" => Key::from(token_a),
            "token_b" => Key::from(token_b),
        };
        let (reserve_a, reserve_b):(U128, U128) = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_GET_RESERVES, args);

        if reserve_a == 0.into() && reserve_b == 0.into()
        {
            return (amount_a_desired, amount_b_desired);
        }
        
        else
        {
            let args: RuntimeArgs = runtime_args!{
                "amount_a" => amount_a_desired,
                "reserve_a" => reserve_a,
                "reserve_b" => reserve_b
            };
            
            let amount_b_optimal: U256 = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_QUOTE, args);
            if amount_b_optimal <= amount_b_desired && amount_b_optimal >= amount_b_min
            {
                return (amount_a_desired, amount_b_optimal);
            }
            
            else
            {
                let args: RuntimeArgs = runtime_args!{
                    "amount_a" => amount_b_desired,
                    "reserve_a" => reserve_b,
                    "reserve_b" => reserve_a
                };
                let amount_a_optimal: U256 = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_QUOTE, args);
                
                if amount_a_optimal > amount_a_desired         // abort
                {
                    runtime::revert(ApiError::User(ErrorCodes::Abort as u16));
                }

                if amount_a_optimal >= amount_a_min
                {
                    return (amount_a_optimal, amount_b_desired);
                }

                else                                            // should never reach here because of revert()
                {
                    return (0.into(), 0.into());
                }
            }
        }
    }

    // a = accounts, p = paths, is paths contract or Account?
    fn _swap (amounts: &Vec<U256>, path: &Vec<Key>, _to: Key)
    {
        let factory = data::factory();
        for i in 0..(path.len() - 1)            // start ≤ x < end
        {
            let (input, output): (Key, Key) = (path[i], path[i+1]);
            let args: RuntimeArgs = runtime_args!{
                "token_a" => input,
                "token_b" => output
            };

            //let uniswapv2_library_contract_hash: &str = uniswapv2_contracts_hash::LIBRARY_HASH;
            let uniswapv2_library_contract_hash = data::library_hash().to_formatted_string();
            let (token0, _):(ContractHash, ContractHash) = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_SORT_TOKENS, args);

            let amount_out: U256 = amounts[i + 1];
            let (amount0_out, amount1_out) : (U256, U256) = if input == Key::from(token0) {(0.into(), amount_out)} else {(amount_out, 0.into())};
            let to: Key = {
                if i < path.len() - 2 
                {
                    let args: RuntimeArgs = runtime_args!{
                        "factory" => factory,
                        "token_a" => output,
                        "token_b" => path[i + 2]
                    };
                    let hash: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);
                    Key::from(hash)
                } 
                else {
                    _to
                }
            };
            
            // Call swap from UniswapV2Pair, but first need to call pair_for to get the pair
            let args: RuntimeArgs = runtime_args!{
                "factory" => factory,
                "token_a" => input,
                "token_b" => output
            };
            let pair: ContractHash = Self::call_contract(&uniswapv2_library_contract_hash, uniswapv2_contract_methods::LIBRARY_PAIR_FOR, args);


            let args: RuntimeArgs = runtime_args!{
                "amount0_out" => amount0_out,
                "amount1_out" => amount1_out,
                "to" => to,
                "data" => Bytes::new(),
            };
            let () = Self::call_contract(&pair.to_formatted_string(), uniswapv2_contract_methods::PAIR_SWAP, args);
        }
    }

    fn ensure(&self, deadline: U256) -> bool 
    {
        // shadowing the variable
        //let deadline = BlockTime::new(u64::from(deadline));
        let deadline = BlockTime::new(deadline.as_u64());
        let blocktime = runtime::get_blocktime();
    
        deadline >= blocktime
    }
    
    fn call_contract<T: CLTyped + FromBytes>(contract_hash_str: &str, method: &str, args: RuntimeArgs) -> T
    {
        let contract_hash = ContractHash::from_formatted_str(contract_hash_str);
        runtime::call_contract(contract_hash.unwrap_or_default(), method, args)    
    }
}