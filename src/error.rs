use std::fmt;

#[derive(Debug)]
pub struct Error {
	pub code: i32,
	pub message: String,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} (code: {})", self.message, self.code)
	}
}

impl std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;
