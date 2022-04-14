#[repr(u16)]
pub enum ErrorCodes {
    /// 65,603 for (UniswapV2 Router Amount A Desired Is Zero)
    UniswapV2RouterAmountADesiredIsZero = 67,
    /// 65,604 for (UniswapV2 Router Amount B Desired Is Zero)
    UniswapV2RouterAmountBDesiredIsZero = 68,
    /// 65,605 for (UniswapV2 Router Amount Token Is Zero)
    UniswapV2RouterAmountTokenIsZero = 69,
    /// 65,606 for (UniswapV2 Router Abort1)
    UniswapV2RouterAbort1 = 70,
    /// 65,607 for (UniswapV2 Router Abort2)
    UniswapV2RouterAbort2 = 71,
    /// 65,608 for (UniswapV2 Router Abort3)
    UniswapV2RouterAbort3 = 72,
    /// 65,609 for (UniswapV2 Router Abort4)
    UniswapV2RouterAbort4 = 73,
    /// 65,610 for (UniswapV2 Router Abort5)
    UniswapV2RouterAbort5 = 74,
    /// 65,611 for (UniswapV2 Router Abort6)
    UniswapV2RouterAbort6 = 75,
    /// 65,612 for (UniswapV2 Router Abort7)
    UniswapV2RouterAbort7 = 76,
    /// 65,613 for (UniswapV2 Router Abort8)
    UniswapV2RouterAbort8 = 77,
    /// 65,614 for (UniswapV2 Router Abort9)
    UniswapV2RouterAbort9 = 78,
    /// 65,615 for (UniswapV2 Router Abort10)
    UniswapV2RouterAbort10 = 79,
    /// 65,616 for (UniswapV2 Router Abort11)
    UniswapV2RouterAbort11 = 80,
    /// 65,617 for (UniswapV2 Router Abort12)
    UniswapV2RouterAbort12 = 81,
    /// 65,618 for (UniswapV2 Router Abort13)
    UniswapV2RouterAbort13 = 82,
    /// 65,619 for (UniswapV2 Router TransferFailed1)
    UniswapV2RouterTransferFailed1 = 83,
    /// 65,620 for (UniswapV2 Router TransferFailed2)
    UniswapV2RouterTransferFailed2 = 84,
    /// 65,621 for (UniswapV2 Router TransferFailed3)
    UniswapV2RouterTransferFailed3 = 85,
    /// 65,622 for (UniswapV2 Router TransferFailed4)
    UniswapV2RouterTransferFailed4 = 86,
    /// 65,623 for (UniswapV2 Router TransferFailed5)
    UniswapV2RouterTransferFailed5 = 87,
    /// 65,624 for (UniswapV2 Router TransferFailed6)
    UniswapV2RouterTransferFailed6 = 88,
    /// 65,625 for (UniswapV2 Router TransferFailed7)
    UniswapV2RouterTransferFailed7 = 89,
    /// 65,626 for (UniswapV2 Router TransferFailed8)
    UniswapV2RouterTransferFailed8 = 90,
    /// 65,627 for (UniswapV2 Router TransferFailed9)
    UniswapV2RouterTransferFailed9 = 91,
    /// 65,628 for (UniswapV2 Router TransferFailed10)
    UniswapV2RouterTransferFailed10 = 92,
    /// 65,629 for (UniswapV2 Router TransferFailed11)
    UniswapV2RouterTransferFailed11 = 93,
    /// 65,630 for (UniswapV2 Router TransferFailed12)
    UniswapV2RouterTransferFailed12 = 94,
    /// 65,631 for (UniswapV2 Router TransferFailed13)
    UniswapV2RouterTransferFailed13 = 95,
    /// 65,632 for (UniswapV2 Router TransferFailed14)
    UniswapV2RouterTransferFailed14 = 96,
    /// 65,633 for (UniswapV2 Router TransferFailed15)
    UniswapV2RouterTransferFailed15 = 97,
    /// 65,634 for (UniswapV2 Router TransferFailed16)
    UniswapV2RouterTransferFailed16 = 98,
    /// 65,635 for (UniswapV2 Router TransferFailed17)
    UniswapV2RouterTransferFailed17 = 99,
    /// 65,636 for (UniswapV2 Router TransferFailed18)
    UniswapV2RouterTransferFailed18 = 100,
    /// 65,637 for (UniswapV2 Router TransferFailed19)
    UniswapV2RouterTransferFailed19 = 101,
    /// 65,638 for (UniswapV2 Router TransferFailed20)
    UniswapV2RouterTransferFailed20 = 102,
    /// 65,639 for (UniswapV2 Router TransferFailed21)
    UniswapV2RouterTransferFailed21 = 103,
    /// 65,640 for (UniswapV2 Router Zero Addr)
    UniswapV2RouterZeroAddr = 104,
    /// 65,641 for (UniswapV2 Router Invalid Arguments)
    UniswapV2RouterInvalidArguments = 105,
    /// 65,641 for (UniswapV2 Router Timed Out1)
    UniswapV2RouterTimedOut1 = 106,
    /// 65,641 for (UniswapV2 Router Timed Out2)
    UniswapV2RouterTimedOut2 = 107,
    /// 65,641 for (UniswapV2 Router Timed Out3)
    UniswapV2RouterTimedOut3 = 108,
    /// 65,641 for (UniswapV2 Router Timed Out4)
    UniswapV2RouterTimedOut4 = 109,
    /// 65,641 for (UniswapV2 Router Timed Out5)
    UniswapV2RouterTimedOut5 = 110,
    /// 65,641 for (UniswapV2 Router Timed Out6)
    UniswapV2RouterTimedOut6 = 111,
    /// 65,641 for (UniswapV2 Router Timed Out7)
    UniswapV2RouterTimedOut7 = 112,
    /// 65,641 for (UniswapV2 Router Timed Out8)
    UniswapV2RouterTimedOut8 = 113,
    /// 65,641 for (UniswapV2 Router Timed Out9)
    UniswapV2RouterTimedOut9 = 114,
    /// 65,641 for (UniswapV2 Router Timed Out10)
    UniswapV2RouterTimedOut10 = 115,
    /// 65,641 for (UniswapV2 Router Timed Out11)
    UniswapV2RouterTimedOut11 = 116,
    /// 65,641 for (UniswapV2 Router Timed Out12)
    UniswapV2RouterTimedOut12 = 117,
    /// 65,641 for (UniswapV2 Router Timed Out13)
    UniswapV2RouterTimedOut13 = 118,
    /// 65,641 for (UniswapV2 Router Timed Out14)
    UniswapV2RouterTimedOut14 = 119,
    /// 65,641 for (UniswapV2 Router Timed Out15)
    UniswapV2RouterTimedOut15 = 120,
    /// 65,641 for (UniswapV2 Router Timed Out16)
    UniswapV2RouterTimedOut16 = 121,
    /// 65,641 for (UniswapV2 Router Timed Out17)
    UniswapV2RouterTimedOut17 = 122,
    /// 65,641 for (UniswapV2 Router Timed Out18)
    UniswapV2RouterTimedOut18 = 123,
    /// 65,641 for (UniswapV2 Router Timed Out19)
    UniswapV2RouterTimedOut19 = 124,
    /// 65,641 for (UniswapV2 Router Timed Out20)
    UniswapV2RouterTimedOut20 = 125,
}

pub mod uniswapv2_contract_methods {

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

    // IWETH Contract methods
    pub const WCSPR_DEPOSIT: &str = "deposit";
    pub const WCSPR_TRANSFER: &str = "transfer";
    pub const WCSPR_TRANSFER_FROM: &str = "transfer_from";
    pub const WCSPR_WITHDRAW: &str = "withdraw";
}
