#[repr(u16)]
pub enum ErrorCodes
{
    Abort = 35,
    TimedOut
}

/*
pub mod uniswapv2_contracts_hash
{
    // Convention -> CONTRACT_NAME_HASH
    pub static PAIR_HASH: &str = "contract-14c9cac495c0531cc556a1973c77d29412810bac374fcd7f04b1a2b3927ba024";
    pub static LIBRARY_HASH: &str = "contract-14c9cac495c0531cc556a1973c77d29412810bac374fcd7f04b1a2b3927ba024";
    pub static TRANSFER_HELPER_HASH: &str = "contract-14c9cac495c0531cc556a1973c77d29412810bac374fcd7f04b1a2b3927ba024";
    pub static IWETH_HASH: &str = "contract-14c9cac495c0531cc556a1973c77d29412810bac374fcd7f04b1a2b3927ba024";
}
*/

pub mod uniswapv2_contract_methods
{
    // Convention -> CONTRACT_NAME_METHOD_NAME
    
    // FACTORY Contract Methods
    pub const FACTORY_GET_PAIR: &str = "get_pair";
    pub const FACTORY_CREATE_PAIR: &str = "create_pair";


    // Library Contract Methods
    pub const LIBRARY_GET_RESERVES: &str = "get_reserves";
    pub const LIBRARY_QUOTE: &str = "quote";
    pub const LIBRARY_PAIR_FOR: &str = "pair_for";
    pub const LIBRARY_SAFE_TRANSFER_FROM: &str = "safe_transfer_from";
    pub const LIBRARY_SORT_TOKENS: &str = "sort_tokens";
    pub const LIBRARY_GET_AMOUNTS_OUT: &str = "get_amounts_out";
    pub const LIBRARY_GET_AMOUNTS_IN: &str = "get_amounts_in";
    pub const LIBRARY_GET_AMOUNT_OUT: &str = "get_amount_out";
    pub const LIBRARY_GET_AMOUNT_IN: &str = "get_amount_in";

    // Pair Contract Methods
    pub const PAIR_MINT: &str = "mint";
    pub const PAIR_TRANSFER_FROM: &str = "transfer_from";
    pub const PAIR_BURN: &str = "burn";
    pub const PAIR_PERMIT: &str = "permit";
    pub const PAIR_SWAP: &str = "swap";

    // Transfer_helper Contract methods
    pub const TRANSFER_HELPER_SAFE_TRANSFER_FROM: &str = "safe_transfer_from";
    pub const TRANSFER_HELPER_SAFE_TRANSFER_CSPR: &str = "safe_transfer_cspr";
    pub const TRASNFER_HELPER_SAFE_SAFE_TRANSFER: &str = "safe_transfer";

    // IWETH Contract methods
    pub const IWETH_DEPOSIT: &str = "deposit";
    pub const IWETH_TRANSFER: &str = "transfer";
    pub const IWETH_WITHDRAW: &str = "withdraw";
}