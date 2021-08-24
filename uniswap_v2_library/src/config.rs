pub mod config{
	use casper_types::api_error::ApiError;
	#[repr(u16)]
	pub enum ErrorCode {
		ZeroAddress = 0,
		IdenticalAddresses = 1,
		InsufficientAmount = 2,
		InsufficientInputAmount = 3,
		InsufficientOutputAmount = 4,
		InsufficientLiquidity = 5,
		InvalidPath = 6
	}
	impl From<ErrorCode> for ApiError {
	    fn from(code: ErrorCode) -> Self {
		ApiError::User(code as u16)
	    }
	}
}
