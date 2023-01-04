use casper_types::ApiError;

#[repr(u16)]
pub enum Errors {
    /// 65,599 for (UniswapV2 Router Not In Whitelist)
    UniswapV2RouterNotInWhitelist = 63,
    /// 65,600 for (UniswapV2 Router Not Owner1)
    UniswapV2RouterNotOwner1 = 64,
    /// 65,601 for (UniswapV2 Router Not Owner2)
    UniswapV2RouterNotOwner2 = 65,
    /// 65,602 for (UniswapV2 Router Not Owner3)
    UniswapV2RouterNotOwner3 = 66,
    /// 65,603 for (UniswapV2 Router Amount A Desired Is Zero)
    UniswapV2RouterAmountADesiredIsZero = 67,
    /// 65,604 for (UniswapV2 Router Amount B Desired Is Zero)
    UniswapV2RouterAmountBDesiredIsZero = 68,
    /// 65,605 for (UniswapV2 Router Amount Token Is Zero)
    UniswapV2RouterAmountTokenIsZero = 69,
    /// 65,606 for (UniswapV2 Router Abort1)
    UniswapV2RouterAbort1 = 70,
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
    /// 65,640 for (UniswapV2 Router Zero Addr)
    UniswapV2RouterZeroAddr = 104,
    /// 65,641 for (UniswapV2 Router Invalid Arguments)
    UniswapV2RouterInvalidArguments = 105,
    /// 65,642 for (UniswapV2 Router Timed Out1)
    UniswapV2RouterTimedOut1 = 106,
    /// 65,644 for (UniswapV2 Router Timed Out3)
    UniswapV2RouterTimedOut3 = 108,
    /// 65,646 for (UniswapV2 Router Timed Out5)
    UniswapV2RouterTimedOut5 = 110,
    /// 65,648 for (UniswapV2 Router Timed Out7)
    UniswapV2RouterTimedOut7 = 112,
    /// 65,650 for (UniswapV2 Router Timed Out9)
    UniswapV2RouterTimedOut9 = 114,
    /// 65,652 for (UniswapV2 Router Timed Out11)
    UniswapV2RouterTimedOut11 = 116,
    /// 65,654 for (UniswapV2 Router Timed Out13)
    UniswapV2RouterTimedOut13 = 118,
    /// 65,656 for (UniswapV2 Router Timed Out15)
    UniswapV2RouterTimedOut15 = 120,
    /// 65,658 for (UniswapV2 Router Timed Out17)
    UniswapV2RouterTimedOut17 = 122,
    /// 65,660 for (UniswapV2 Router Timed Out19)
    UniswapV2RouterTimedOut19 = 124,

    /// 65,662 for (UniswapV2 Library Multiplication Overflow 1)
    MultiplicationOverflow1 = 126,
    /// 65,663 for (UniswapV2 Library Addition Overflow 1)
    AdditionOverflow1 = 127,
    /// 65,664 for (UniswapV2 Library Addition Overflow 2)
    AdditionOverflow2 = 128,
    /// 65,666 for (UniswapV2 Library Zero Address)
    ZeroAddress = 130,
    /// 65,667 for (UniswapV2 Library Identical Addresses)
    IdenticalAddresses = 131,
    /// 65,668 for (UniswapV2 Library Insufficient Amount)
    InsufficientAmount = 132,
    /// 65,669 for (UniswapV2 Library Insufficient Input Amount)
    InsufficientInputAmount = 133,
    /// 65,670 for (UniswapV2 Library Insufficient Output Amount)
    InsufficientOutputAmount = 134,
    /// 65,671 for (UniswapV2 Library Invalid Path)
    InvalidPath = 135,
    /// 65,672 for (UniswapV2 Library Insufficient Liquidity1)
    InsufficientLiquidity1 = 136,
    /// 65,672 for (UniswapV2 Library Insufficient Liquidity2)
    InsufficientLiquidity2 = 137,
    /// 65,672 for (UniswapV2 Library Insufficient Liquidity3)
    InsufficientLiquidity3 = 138,
    /// 65,672 for (UniswapV2 Library Insufficient Liquidity4)
    InsufficientLiquidity4 = 139,
}

impl From<Errors> for ApiError {
    fn from(error: Errors) -> ApiError {
        ApiError::User(error as u16)
    }
}
