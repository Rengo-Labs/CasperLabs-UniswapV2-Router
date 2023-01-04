use std::collections::BTreeMap;

use crate::{data::*, transfer_helper::transfer_helper_mod};
use common::{
    contract_api::{runtime, storage, system},
    errors::Errors,
    functions::*,
    keys::*,
    unwrap_or_revert::UnwrapOrRevert,
    *,
};

pub enum ROUTEREvent {
    AddReserves {
        user: Key,
        reserve0: U256,
        reserve1: U256,
        pair_contract_hash: ContractPackageHash,
    },
    RemoveReserves {
        user: Key,
        reserve0: U256,
        reserve1: U256,
        pair_contract_hash: ContractPackageHash,
    },
}

impl ROUTEREvent {
    pub fn type_name(&self) -> String {
        match self {
            ROUTEREvent::AddReserves {
                user: _,
                reserve0: _,
                reserve1: _,
                pair_contract_hash: _,
            } => "addreserves",
            ROUTEREvent::RemoveReserves {
                user: _,
                reserve0: _,
                reserve1: _,
                pair_contract_hash: _,
            } => "removereserves",
        }
        .to_string()
    }
}

pub trait UniswapV2Router<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &self,
        factory: ContractPackageHash,
        wcspr: ContractPackageHash,
        library_hash: ContractPackageHash,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        set_factory(factory);
        set_wcspr(wcspr);
        set_library_hash(library_hash);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
        Whitelist::init();
        set_owner(self.get_caller());
        set_purse(system::create_purse());
    }

    fn change_owner(&self, owner: Key) {
        if self.get_caller() != get_owner() {
            runtime::revert(Errors::UniswapV2RouterNotOwner1);
        }
        set_owner(owner);
    }

    fn add_to_whitelist(&self, user: Key) {
        if self.get_caller() != get_owner() {
            runtime::revert(Errors::UniswapV2RouterNotOwner2);
        }
        Whitelist::instance().set(&user, true);
    }

    fn remove_from_whitelist(&self, user: Key) {
        if self.get_caller() != get_owner() {
            runtime::revert(Errors::UniswapV2RouterNotOwner3);
        }
        Whitelist::instance().set(&user, false);
    }

    #[allow(clippy::too_many_arguments)]
    fn add_liquidity(
        &self,
        token_a: ContractPackageHash,
        token_b: ContractPackageHash,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        pair: Option<Key>,
        deadline: U256,
    ) -> (U256, U256, U256) {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut1 as u16));
        }
        if amount_a_desired <= 0.into() {
            runtime::revert(Errors::UniswapV2RouterAmountADesiredIsZero);
        }
        if amount_b_desired <= 0.into() {
            runtime::revert(Errors::UniswapV2RouterAmountBDesiredIsZero);
        }
        let (amount_a, amount_b): (U256, U256) = self._add_liquidity(
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            pair,
        );
        // call pair_for from library contract
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => Key::from(token_a),
                "token_b" => Key::from(token_b)
            },
        );
        transfer_helper_mod::safe_transfer_from(
            Key::from(token_a),
            self.get_caller(),
            pair,
            amount_a,
        );
        transfer_helper_mod::safe_transfer_from(
            Key::from(token_b),
            self.get_caller(),
            pair,
            amount_b,
        );
        // call mint function from IUniswapV2Pair contract
        let liquidity: U256 = runtime::call_versioned_contract(
            pair.into_hash().unwrap_or_revert().into(),
            None,
            PAIR_MINT,
            runtime_args! {
                "to" => to,
            },
        );
        self.emit(&ROUTEREvent::AddReserves {
            user: to,
            reserve0: amount_a,
            reserve1: amount_b,
            pair_contract_hash: pair.into_hash().unwrap_or_revert().into(),
        });
        (amount_a, amount_b, liquidity)
    }

    #[allow(clippy::too_many_arguments)]
    fn add_liquidity_cspr(
        &self,
        token: ContractPackageHash,
        amount_token_desired: U256,
        amount_cspr_desired: U256,
        amount_token_min: U256,
        amount_cspr_min: U256,
        to: Key,
        pair: Option<Key>,
        caller_purse: URef,
        deadline: U256,
    ) -> (U256, U256, U256) {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut3 as u16));
        }
        let (amount_token, amount_cspr): (U256, U256) = self._add_liquidity(
            token,
            wcspr(),
            amount_token_desired,
            amount_cspr_desired,
            amount_token_min,
            amount_cspr_min,
            pair,
        );
        // call pair_for from library contract
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => Key::from(token),
                "token_b" => Key::from(wcspr())
            },
        );
        if amount_token <= 0.into() {
            runtime::revert(Errors::UniswapV2RouterAmountTokenIsZero);
        }
        // call safe_transfer_from from TransferHelper
        transfer_helper_mod::safe_transfer_from(
            Key::from(token),
            self.get_caller(),
            pair,
            amount_token,
        );
        system::transfer_from_purse_to_purse(
            caller_purse,
            get_purse(),
            u256_to_u512(amount_cspr),
            None,
        )
        .unwrap_or_revert();
        // this call will submit cspr to the wcspr contract and in return get wcspr tokens which will be sent to pair
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_DEPOSIT,
            runtime_args! {
                "amount" => u256_to_u512(amount_cspr),
                "purse" => get_purse()
            },
        );
        // call transfer method from wcspr
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_TRANSFER,
            runtime_args! {
                "recipient" => pair,
                "amount" => amount_cspr
            },
        );
        // call mint function from pair contract
        let liquidity: U256 = runtime::call_versioned_contract(
            pair.into_hash().unwrap_or_revert().into(),
            None,
            PAIR_MINT,
            runtime_args! {
                "to" => to,
            },
        );
        self.emit(&ROUTEREvent::AddReserves {
            user: to,
            reserve0: amount_token,
            reserve1: amount_cspr,
            pair_contract_hash: pair.into_hash().unwrap_or_revert().into(),
        });
        // No need to transfer the leftover cspr, because we are already taking the exact amount out from the caller purse
        (amount_token, amount_cspr, liquidity)
    }

    #[allow(clippy::too_many_arguments)]
    fn remove_liquidity(
        &self,
        token_a: ContractPackageHash,
        token_b: ContractPackageHash,
        liquidity: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        deadline: U256,
    ) -> (U256, U256) {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut5 as u16));
        }
        // call pair_for from library contract
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => Key::from(token_a),
                "token_b" => Key::from(token_b)
            },
        );
        // call transferFrom from IUniSwapV2Pair
        runtime::call_versioned_contract::<()>(
            pair.into_hash().unwrap_or_revert().into(),
            None,
            PAIR_TRANSFER_FROM,
            runtime_args! {
                "owner" => self.get_caller(),
                "recipient" => pair,
                "amount" => liquidity
            },
        );
        // call burn from IUniSwapV2Pair
        let (amount0, amount1): (U256, U256) = runtime::call_versioned_contract(
            pair.into_hash().unwrap_or_revert().into(),
            None,
            PAIR_BURN,
            runtime_args! {
                "to" => to,
            },
        );
        // call sortTokens from library contract
        let (token0, _): (ContractPackageHash, ContractPackageHash) =
            runtime::call_versioned_contract(
                library_hash(),
                None,
                LIBRARY_SORT_TOKENS,
                runtime_args! {
                    "token_a" => Key::from(token_a),
                    "token_b" => Key::from(token_b)
                },
            );
        let (amount_a, amount_b): (U256, U256) = if token_a == token0 {
            (amount0, amount1)
        } else {
            (amount1, amount0)
        };
        if amount_a < amount_a_min || amount_b < amount_b_min {
            runtime::revert(Errors::UniswapV2RouterAbort1);
        }
        self.emit(&ROUTEREvent::RemoveReserves {
            user: Key::from(runtime::get_caller()),
            reserve0: amount_a,
            reserve1: amount_b,
            pair_contract_hash: pair.into_hash().unwrap_or_revert().into(),
        });
        (amount_a, amount_b)
    }

    #[allow(clippy::too_many_arguments)]
    fn remove_liquidity_cspr(
        &self,
        token: ContractPackageHash,
        liquidity: U256,
        amount_token_min: U256,
        amount_cspr_min: U256,
        to: Key,        // to's key to transfer back token
        to_purse: URef, // to's purse to transfer back cspr
        deadline: U256,
    ) -> (U256, U256) {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut7 as u16));
        }
        // calling self contract's removeLiquidity
        let (amount_token, amount_cspr): (U256, U256) = self.remove_liquidity(
            token,
            wcspr(),
            liquidity,
            amount_token_min,
            amount_cspr_min,
            Key::from(get_package_hash()),
            deadline,
        );
        // transfer token to 'to'
        transfer_helper_mod::safe_transfer(Key::from(token), to, amount_token);
        // call withdraw and transfer cspr to 'to'
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_WITHDRAW,
            runtime_args! {
                "purse" => to_purse,
                "amount" => u256_to_u512(amount_cspr)
            },
        );
        (amount_token, amount_cspr)
    }

    fn swap_exact_tokens_for_tokens(
        &self,
        amount_in: U256,
        amount_out_min: U256,
        _path: Vec<String>,
        to: Key,
        deadline: U256,
    ) -> Vec<U256> {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut9 as u16));
        }
        let mut path: Vec<Key> = Vec::new();
        for i in &_path {
            path.push(Key::from_formatted_str(i).unwrap());
        }
        // call getAmountsOut from Library contract
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_OUT,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_in" => amount_in,
                "path" => path.clone(),
            },
        );
        if amounts[amounts.len() - 1] < amount_out_min {
            runtime::revert(Errors::UniswapV2RouterAbort4);
        }
        // get pair
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => path[0],
                "token_b" => path[1],
            },
        );
        transfer_helper_mod::safe_transfer_from(path[0], self.get_caller(), pair, amounts[0]);
        Self::_swap(&amounts, &path, to);
        amounts
    }

    fn swap_tokens_for_exact_tokens(
        &self,
        amount_out: U256,
        amount_in_max: U256,
        _path: Vec<String>,
        to: Key,
        deadline: U256,
    ) -> Vec<U256> {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut11 as u16));
        }
        let mut path: Vec<Key> = Vec::new();
        for i in &_path {
            path.push(Key::from_formatted_str(i).unwrap());
        }
        // call getAmountIn from Library contract
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_IN,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_out" => amount_out,
                "path" => path.clone(),
            },
        );
        if amounts[0] > amount_in_max {
            runtime::revert(Errors::UniswapV2RouterAbort5);
        }
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => path[0],
                "token_b" => path[1],
            },
        );
        transfer_helper_mod::safe_transfer_from(path[0], self.get_caller(), pair, amounts[0]);
        Self::_swap(&amounts, &path, to);
        amounts
    }

    fn swap_exact_cspr_for_tokens(
        &self,
        amount_out_min: U256,
        amount_in: U256,
        _path: Vec<String>,
        to: Key,
        caller_purse: URef,
        deadline: U256,
    ) -> Vec<U256> {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut13 as u16));
        }
        let mut path: Vec<Key> = Vec::new();
        for i in &_path {
            path.push(Key::from_formatted_str(i).unwrap());
        }
        if path[0] != Key::from(wcspr()) {
            runtime::revert(Errors::UniswapV2RouterAbort6);
        }
        // call get_amounts_out
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_OUT,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_in" => amount_in,
                "path" => path.clone(),
            },
        );
        if amounts[amounts.len() - 1] < amount_out_min {
            runtime::revert(Errors::UniswapV2RouterAbort7);
        }
        system::transfer_from_purse_to_purse(
            caller_purse,
            get_purse(),
            u256_to_u512(amounts[0]),
            None,
        )
        .unwrap_or_revert();
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_DEPOSIT,
            runtime_args! {
                "amount" => u256_to_u512(amounts[0]),
                "purse" => get_purse()
            },
        );
        // call transfer method from IWETH, get pair
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => path[0],
                "token_b" => path[1]
            },
        );
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_TRANSFER,
            runtime_args! {
                "recipient" => pair,
                "amount" => amounts[0]
            },
        );
        Self::_swap(&amounts, &path, to);
        amounts
    }

    fn swap_tokens_for_exact_cspr(
        &self,
        amount_out: U256,
        amount_in_max: U256,
        _path: Vec<String>,
        to: URef, // recipient of cspr, must be a purse
        deadline: U256,
    ) -> Vec<U256> {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut15 as u16));
        }
        let mut path: Vec<Key> = Vec::new();
        for i in &_path {
            path.push(Key::from_formatted_str(i).unwrap());
        }
        if path[path.len() - 1] != Key::from(wcspr()) {
            runtime::revert(Errors::UniswapV2RouterAbort8);
        }
        // call getAmountIn from Library contract
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_IN,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_out" => amount_out,
                "path" => path.clone(),
            },
        );
        if amounts[0] > amount_in_max {
            runtime::revert(Errors::UniswapV2RouterAbort9);
        }
        // call safeTransferFrom from TransferHelper, first need to get the pair
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => path[0],
                "token_b" => path[1],
            },
        );
        transfer_helper_mod::safe_transfer_from(path[0], self.get_caller(), pair, amounts[0]);
        Self::_swap(&amounts, &path, Key::from(get_package_hash()));
        // call withdraw from WCSPR and transfer cspr to 'to'
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_WITHDRAW,
            runtime_args! {
                "purse" => to,
                "amount" => u256_to_u512(amounts[amounts.len() - 1])
            },
        );
        amounts
    }

    fn swap_exact_tokens_for_cspr(
        &self,
        amount_in: U256,
        amount_out_min: U256,
        _path: Vec<String>,
        to: URef, // recipient of cspr, must be a purse
        deadline: U256,
    ) -> Vec<U256> {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut17 as u16));
        }
        let mut path: Vec<Key> = Vec::new();
        for i in &_path {
            path.push(Key::from_formatted_str(i).unwrap());
        }
        if path[path.len() - 1] != Key::from(wcspr()) {
            runtime::revert(Errors::UniswapV2RouterAbort10);
        }
        // call get_amounts_out
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_OUT,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_in" => amount_in,
                "path" => path.clone(),
            },
        );
        if amounts[amounts.len() - 1] < amount_out_min {
            runtime::revert(Errors::UniswapV2RouterAbort11);
        }
        // call safeTransferFrom from TransferHelper, first need to get the pair
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => path[0],
                "token_b" => path[1],
            },
        );
        transfer_helper_mod::safe_transfer_from(path[0], self.get_caller(), pair, amounts[0]);
        Self::_swap(&amounts, &path, Key::from(get_package_hash()));
        // call withdraw from WCSPR and transfer cspr to 'to'
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_WITHDRAW,
            runtime_args! {
                "purse" => to,
                "amount" => u256_to_u512(amounts[amounts.len() - 1])
            },
        );
        amounts
    }

    fn swap_cspr_for_exact_tokens(
        &self,
        amount_out: U256,
        amount_in_max: U256,
        _path: Vec<String>,
        to: Key,
        caller_purse: URef,
        deadline: U256,
    ) -> Vec<U256> {
        if !(self.ensure(deadline)) {
            runtime::revert(ApiError::User(Errors::UniswapV2RouterTimedOut19 as u16));
        }
        let mut path: Vec<Key> = Vec::new();
        for i in &_path {
            path.push(Key::from_formatted_str(i).unwrap());
        }
        if path[0] != Key::from(wcspr()) {
            runtime::revert(Errors::UniswapV2RouterAbort12);
        }
        // call get_amounts_out
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_IN,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_out" => amount_out,
                "path" => path.clone(),
            },
        );
        if amounts[0] > amount_in_max {
            runtime::revert(Errors::UniswapV2RouterAbort13);
        }
        system::transfer_from_purse_to_purse(
            caller_purse,
            get_purse(),
            u256_to_u512(amounts[0]),
            None,
        )
        .unwrap_or_revert();
        // call deposit method from wcspr
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_DEPOSIT,
            runtime_args! {
                "amount" => u256_to_u512(amounts[0]),
                "purse" => get_purse()
            },
        );
        // call transfer method from wcspr
        // Get pair
        let pair: Key = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_PAIR_FOR,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => path[0],
                "token_b" => path[1],
            },
        );
        runtime::call_versioned_contract::<()>(
            wcspr(),
            None,
            WCSPR_TRANSFER,
            runtime_args! {
                "recipient" => pair,
                "amount" => amounts[0]
            },
        );
        Self::_swap(&amounts, &path, to);
        // No need to refund extra cspr because we are already getting the exact required amount from the purse
        amounts
    }

    fn quote(amount_a: U256, reserve_a: U256, reserve_b: U256) -> U256 {
        runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_QUOTE,
            runtime_args! {
                "amount_a" => amount_a,
                "reserve_a" => U128::from(reserve_a.as_u128()),
                "reserve_b" => U128::from(reserve_b.as_u128())
            },
        )
    }

    fn get_amount_out(amount_in: U256, reserve_in: U256, reserve_out: U256) -> U256 {
        runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNT_OUT,
            runtime_args! {
                "amount_in" => amount_in,
                "reserve_in" => reserve_in,
                "reserve_out" => reserve_out
            },
        )
    }

    fn get_amount_in(amount_out: U256, reserve_in: U256, reserve_out: U256) -> U256 {
        runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNT_IN,
            runtime_args! {
                "amount_out" => amount_out,
                "reserve_in" => reserve_in,
                "reserve_out" => reserve_out
            },
        )
    }

    fn get_amounts_out(amount_in: U256, path: Vec<Key>) -> Vec<U256> {
        runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_OUT,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_in" => amount_in,
                "path" => path
            },
        )
    }

    fn get_amounts_in(amount_out: U256, path: Vec<Key>) -> Vec<U256> {
        runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_AMOUNTS_IN,
            runtime_args! {
                "factory" => Key::from(factory()),
                "amount_out" => amount_out,
                "path" => path
            },
        )
    }

    // *************************************** Helper methods ****************************************

    #[allow(clippy::too_many_arguments)]
    fn _add_liquidity(
        &self,
        token_a: ContractPackageHash,
        token_b: ContractPackageHash,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        pair_received: Option<Key>,
    ) -> (U256, U256) {
        let pair: Key = runtime::call_versioned_contract(
            factory(),
            None,
            FACTORY_GET_PAIR,
            runtime_args! {
                "token0" => Key::from(token_a),
                "token1" => Key::from(token_b)
            },
        );
        let mut pair_already_exist: bool = false;
        // If a pair is not passed, check if it already exists, if it doesnot, revert
        if pair_received.is_none() {
            if pair == zero_address() {
                // if pair is none and it doesnot already exist, revert
                runtime::revert(Errors::UniswapV2RouterZeroAddr);
            } else {
                pair_already_exist = true;
            }
        }
        // If a pair is passed in, check if it exists already, if it does, no need to call factory's create_pair
        if pair_received.is_some() && pair != zero_address() {
            pair_already_exist = true;
        }
        if !pair_already_exist {
            if !Whitelist::instance().get(&self.get_caller()) {
                runtime::revert(Errors::UniswapV2RouterNotInWhitelist);
            }
            // need to call create_pair only once for each pair. If a same pair is passed again, no need to call this again
            runtime::call_versioned_contract::<()>(
                factory(),
                None,
                FACTORY_CREATE_PAIR, // this create_pair method DOES NOT create a new pair, instead it initializes the pair passed in
                runtime_args! {
                    "token_a" => Key::from(token_a),
                    "token_b" => Key::from(token_b),
                    "pair_hash" => pair_received.unwrap()
                },
            );
        }
        let (reserve_a, reserve_b): (U128, U128) = runtime::call_versioned_contract(
            library_hash(),
            None,
            LIBRARY_GET_RESERVES,
            runtime_args! {
                "factory" => Key::from(factory()),
                "token_a" => Key::from(token_a),
                "token_b" => Key::from(token_b),
            },
        );
        if reserve_a == 0.into() && reserve_b == 0.into() {
            (amount_a_desired, amount_b_desired)
        } else {
            let amount_b_optimal: U256 = runtime::call_versioned_contract(
                library_hash(),
                None,
                LIBRARY_QUOTE,
                runtime_args! {
                    "amount_a" => amount_a_desired,
                    "reserve_a" => reserve_a,
                    "reserve_b" => reserve_b
                },
            );
            if amount_b_optimal <= amount_b_desired && amount_b_optimal >= amount_b_min {
                (amount_a_desired, amount_b_optimal)
            } else {
                let amount_a_optimal: U256 = runtime::call_versioned_contract(
                    library_hash(),
                    None,
                    LIBRARY_QUOTE,
                    runtime_args! {
                        "amount_a" => amount_b_desired,
                        "reserve_a" => reserve_b,
                        "reserve_b" => reserve_a
                    },
                );
                if amount_a_optimal > amount_a_desired {
                    runtime::revert(Errors::UniswapV2RouterInvalidArguments);
                }
                if amount_a_optimal >= amount_a_min {
                    (amount_a_optimal, amount_b_desired)
                } else {
                    (0.into(), 0.into())
                }
            }
        }
    }

    fn _swap(amounts: &[U256], path: &Vec<Key>, _to: Key) {
        for i in 0..(path.len() - 1)
        // start ≤ x < end - 1
        {
            let (input, output): (Key, Key) = (path[i], path[i + 1]);
            let (token0, _): (ContractPackageHash, ContractPackageHash) =
                runtime::call_versioned_contract(
                    library_hash(),
                    None,
                    LIBRARY_SORT_TOKENS,
                    runtime_args! {
                        "token_a" => input,
                        "token_b" => output
                    },
                );
            let amount_out: U256 = amounts[i + 1];
            let (amount0_out, amount1_out): (U256, U256) = if input == Key::from(token0) {
                (0.into(), amount_out)
            } else {
                (amount_out, 0.into())
            };
            let to: Key = {
                if i < path.len() - 2 {
                    runtime::call_versioned_contract(
                        library_hash(),
                        None,
                        LIBRARY_PAIR_FOR,
                        runtime_args! {
                            "factory" => Key::from(factory()),
                            "token_a" => output,
                            "token_b" => path[i + 2]
                        },
                    )
                } else {
                    _to
                }
            };
            // Call swap from UniswapV2Pair, but first need to call pair_for to get the pair
            let pair: Key = runtime::call_versioned_contract(
                library_hash(),
                None,
                LIBRARY_PAIR_FOR,
                runtime_args! {
                    "factory" => Key::from(factory()),
                    "token_a" => input,
                    "token_b" => output
                },
            );
            runtime::call_versioned_contract::<()>(
                pair.into_hash().unwrap_or_revert().into(),
                None,
                PAIR_SWAP,
                runtime_args! {
                    "amount0_out" => amount0_out,
                    "amount1_out" => amount1_out,
                    "to" => to,
                    "data" => "",
                },
            );
        }
    }

    fn ensure(&self, deadline: U256) -> bool {
        BlockTime::new(deadline.as_u64()) >= runtime::get_blocktime()
    }

    fn emit(&self, router_event: &ROUTEREvent) {
        match router_event {
            ROUTEREvent::AddReserves {
                user,
                reserve0,
                reserve1,
                pair_contract_hash,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", router_event.type_name());
                event.insert("user", user.to_string());
                event.insert("reserve0", reserve0.to_string());
                event.insert("reserve1", reserve1.to_string());
                event.insert("pair_contract_hash", pair_contract_hash.to_string());
                storage::new_uref(event);
            }
            ROUTEREvent::RemoveReserves {
                user,
                reserve0,
                reserve1,
                pair_contract_hash,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", router_event.type_name());
                event.insert("user", user.to_string());
                event.insert("reserve0", reserve0.to_string());
                event.insert("reserve1", reserve1.to_string());
                event.insert("pair_contract_hash", pair_contract_hash.to_string());
                storage::new_uref(event);
            }
        };
    }
}
