use std::{char::TryFromCharError, ffi::NulError, fmt};

use crate::core;

#[derive(Debug)]
pub struct Error {
	pub code: i32,
	pub message: String,
}

impl Error {
	#[inline]
	pub fn new(code: i32, message: impl Into<String>) -> Self {
		Self {
			code,
			message: message.into(),
		}
	}
}

impl fmt::Display for Error {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} (code: {})", self.message, self.code)
	}
}

impl From<NulError> for Error {
	#[inline]
	fn from(_: NulError) -> Self {
		Self::new(core::StsBadArg, "Passed Rust string contains nul byte")
	}
}

impl From<TryFromCharError> for Error {
	#[inline]
	fn from(_: TryFromCharError) -> Self {
		Self::new(core::StsBadArg, "Passed Rust char can't be converted to C++ char")
	}
}

impl std::error::Error for Error {}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
