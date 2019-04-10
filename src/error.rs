use std::fmt;

#[derive(Debug)]
pub struct Error {
	pub code: i32,
	pub message: String,
}

impl Error {
	pub fn new(code: i32, message: String) -> Self {
		Self { code, message }
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} (code: {})", self.message, self.code)
	}
}

impl std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;
