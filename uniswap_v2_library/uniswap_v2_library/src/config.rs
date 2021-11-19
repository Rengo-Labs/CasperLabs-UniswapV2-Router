pub mod error{
	
	use casper_types::api_error::ApiError;
	#[repr(u16)]
	pub enum ErrorCode {
		Zero = 0,				// Overflow
		One,					// Underflow
		ZeroAddress,
		IdenticalAddresses,
		InsufficientAmount,
		InsufficientInputAmount,
		InsufficientOutputAmount,
		InsufficientLiquidity,
		InvalidPath
	}

	impl From<ErrorCode> for ApiError {
	    fn from(code: ErrorCode) -> Self {
		ApiError::User(code as u16)
	    }
	}

}