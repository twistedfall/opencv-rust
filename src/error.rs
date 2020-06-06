use std::{
	ffi::NulError,
	fmt,
};

use crate::core;

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

impl From<NulError> for Error {
	fn from(_: NulError) -> Self {
		Self::new(core::StsBadArg, "Passed Rust string contains nul byte".into())
	}
}

impl std::error::Error for Error {}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
