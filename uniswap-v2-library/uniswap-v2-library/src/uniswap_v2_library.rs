use common::{
    contract_api::runtime, errors::Errors, functions::zero_address,
    unwrap_or_revert::UnwrapOrRevert, *,
};

pub trait UniswapV2Library<Storage: ContractStorage>: ContractContext<Storage> {
    /// Will be called by constructor
    fn init(&self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    fn sort_tokens(
        &self,
        token_a: ContractPackageHash,
        token_b: ContractPackageHash,
    ) -> (ContractPackageHash, ContractPackageHash) {
        if token_a == token_b {
            runtime::revert(Errors::IdenticalAddresses);
        }
        let (token_0, token_1): (ContractPackageHash, ContractPackageHash);
        if token_a < token_b {
            token_0 = token_a;
            token_1 = token_b;
        } else {
            token_0 = token_b;
            token_1 = token_a;
        }
        if Key::from(token_0) == zero_address() {
            runtime::revert(Errors::ZeroAddress);
        }
        (token_0, token_1)
    }

    fn pair_for(&self, factory: Key, token_a: Key, token_b: Key) -> Key {
        runtime::call_versioned_contract(
            ContractPackageHash::from(factory.into_hash().unwrap_or_default()),
            None,
            "get_pair",
            runtime_args! {
                "token0" => token_a,
                "token1" => token_b
            },
        )
    }

    fn get_reserves(
        &self,
        factory: ContractPackageHash,
        token_a: ContractPackageHash,
        token_b: ContractPackageHash,
    ) -> (U128, U128) {
        let (token_0, _): (ContractPackageHash, ContractPackageHash) =
            self.sort_tokens(token_a, token_b);
        let pair: Key = self.pair_for(Key::from(factory), Key::from(token_a), Key::from(token_b));
        let (reserve_0, reserve_1, _): (U128, U128, u64) = runtime::call_versioned_contract(
            pair.into_hash().unwrap_or_revert().into(),
            None,
            "get_reserves",
            runtime_args! {},
        );
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

    /// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn quote(&self, amount_a: U256, reserve_a: U128, reserve_b: U128) -> U256 {
        if amount_a <= 0.into() {
            runtime::revert(Errors::InsufficientAmount);
        }
        if reserve_a <= 0.into() || reserve_b <= 0.into() {
            runtime::revert(Errors::InsufficientLiquidity);
        }
        (amount_a * U256::from(reserve_b.as_u128())) / U256::from(reserve_a.as_u128())
    }

    /// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    fn get_amount_out(&self, amount_in: U256, reserve_in: U256, reserve_out: U256) -> U256 {
        if amount_in <= 0.into() {
            runtime::revert(Errors::InsufficientInputAmount);
        }
        if reserve_in <= 0.into() || reserve_out <= 0.into() {
            runtime::revert(Errors::InsufficientLiquidity);
        }
        let amount_in_with_fee: U256 = amount_in * 997;
        let numerator: U256 = amount_in_with_fee * reserve_out;
        let denominator: U256 = reserve_in
            .checked_mul(U256::from(1000))
            .unwrap_or_revert_with(Errors::MultiplicationOverflow1)
            .checked_add(amount_in_with_fee)
            .unwrap_or_revert_with(Errors::AdditionOverflow1);
        numerator / denominator
    }

    /// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    fn get_amount_in(&self, amount_out: U256, reserve_in: U256, reserve_out: U256) -> U256 {
        if amount_out <= 0.into() {
            runtime::revert(Errors::InsufficientOutputAmount);
        }
        if reserve_in <= 0.into() || reserve_out <= 0.into() {
            runtime::revert(Errors::InsufficientLiquidity);
        }
        let numerator: U256 = reserve_in * amount_out * 1000;
        let denominator: U256 = (reserve_out - amount_out) * 997;
        (numerator / denominator)
            .checked_add(U256::from(1))
            .unwrap_or_revert_with(Errors::AdditionOverflow2)
    }

    /// performs chained getAmountOut calculations on any number of pairs
    fn get_amounts_out(
        &self,
        factory: ContractPackageHash,
        amount_in: U256,
        path: Vec<ContractPackageHash>,
    ) -> Vec<U256> {
        if path.len() < 2 {
            runtime::revert(Errors::InsufficientLiquidity);
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

    /// performs chained getAmountIn calculations on any number of pairs
    fn get_amounts_in(
        &self,
        factory: ContractPackageHash,
        amount_out: U256,
        path: Vec<ContractPackageHash>,
    ) -> Vec<U256> {
        if path.len() < 2 {
            runtime::revert(Errors::InvalidPath);
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
}
