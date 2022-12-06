// common keys
pub const PURSE: &str = "purse";

// session code router
pub const DESTINATION_ADD_LIQUIDITY: &str = "add_liquidity";
pub const DESTINATION_ADD_LIQUIDITY_CSPR: &str = "add_liquidity_cspr";
pub const DESTINATION_REMOVE_LIQUIDITY: &str = "remove_liquidity";
pub const DESTINATION_REMOVE_LIQUIDITY_CSPR: &str = "remove_liquidity_cspr";
pub const DESTINATION_SWAP_EXACT_TOKENS_FOR_TOKENS: &str = "swap_exact_tokens_for_tokens";
pub const DESTINATION_SWAP_TOKENS_FOR_EXACT_TOKENS: &str = "swap_tokens_for_exact_tokens";
pub const DESTINATION_SWAP_EXACT_CSPR_FOR_TOKENS: &str = "swap_exact_cspr_for_tokens";
pub const DESTINATION_SWAP_CSPR_FOR_EXACT_TOKENS: &str = "swap_cspr_for_exact_tokens";
pub const DESTINATION_SWAP_TOKENS_FOR_EXACT_CSPR: &str = "swap_tokens_for_exact_cspr";
pub const DESTINATION_SWAP_EXACT_TOKENS_FOR_CSPR: &str = "swap_exact_tokens_for_cspr";
pub const AMOUNT_RUNTIME_ARG: &str = "amount";

// router
pub const OWNER: &str = "owner";
pub const WHITELIST: &str = "whitelist";
pub const WCSPR: &str = "wcspr";
pub const FACTORY: &str = "factory";
pub const LIBRARY_HASH: &str = "library_hash";

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
pub const PAIR_SWAP: &str = "swap";

// IWETH Contract methods
pub const WCSPR_DEPOSIT: &str = "deposit";
pub const WCSPR_TRANSFER: &str = "transfer";
pub const WCSPR_TRANSFER_FROM: &str = "transfer_from";
pub const WCSPR_WITHDRAW: &str = "withdraw";
