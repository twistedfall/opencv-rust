//! Crate error type
//!
//! Provides the [`Error`] type that encapsulates the error code and message from OpenCV and the corresponding [`Result`] alias.

use std::ffi::NulError;

use ::core::char::TryFromCharError;
use ::core::fmt;
use ::core::num::TryFromIntError;

use crate::core;

/// Error type from OpenCV with the code and the message
///
/// The code is expected to be one of the [`core::Code`] constants, but it's never actually enforced in OpenCV so the code is exposed
/// as plain [`i32`], to get the enum variant call [`Self::code_as_enum()`].
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

	/// Return the corresponding [`core::Code`] variant of the error code, or [`None`] if no match has been found
	///
	/// The no match case shouldn't actually happen in reality.
	#[inline]
	pub fn code_as_enum(&self) -> Option<core::Code> {
		error_code_as_enum(self.code)
	}

	fn format_code(&self) -> String {
		if let Some(code) = self.code_as_enum() {
			format!("{code:?}, {}", self.code)
		} else {
			format!("{}", self.code)
		}
	}
}

impl fmt::Display for Error {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} (code: {})", self.message, self.format_code())
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Error")
			.field("code", &self.format_code())
			.field("message", &self.message)
			.finish()
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

impl From<TryFromIntError> for Error {
	fn from(_: TryFromIntError) -> Self {
		Self::new(
			core::StsBadArg,
			"Passed slice argument is too long, likely length doesn't fit i32",
		)
	}
}

impl std::error::Error for Error {}

/// Alias to the [`std::result::Result`] with the crate specific [`Error`] type
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Given the OpenCV error code, returns the corresponding [`core::Code`] enum variant.
pub fn error_code_as_enum(code: i32) -> Option<core::Code> {
	match code {
		core::StsOk => Some(core::Code::StsOk),
		core::StsBackTrace => Some(core::Code::StsBackTrace),
		core::StsError => Some(core::Code::StsError),
		core::StsInternal => Some(core::Code::StsInternal),
		core::StsNoMem => Some(core::Code::StsNoMem),
		core::StsBadArg => Some(core::Code::StsBadArg),
		core::StsBadFunc => Some(core::Code::StsBadFunc),
		core::StsNoConv => Some(core::Code::StsNoConv),
		core::StsAutoTrace => Some(core::Code::StsAutoTrace),
		core::HeaderIsNull => Some(core::Code::HeaderIsNull),
		core::BadImageSize => Some(core::Code::BadImageSize),
		core::BadOffset => Some(core::Code::BadOffset),
		core::BadDataPtr => Some(core::Code::BadDataPtr),
		core::BadStep => Some(core::Code::BadStep),
		core::BadModelOrChSeq => Some(core::Code::BadModelOrChSeq),
		core::BadNumChannels => Some(core::Code::BadNumChannels),
		core::BadNumChannel1U => Some(core::Code::BadNumChannel1U),
		core::BadDepth => Some(core::Code::BadDepth),
		core::BadAlphaChannel => Some(core::Code::BadAlphaChannel),
		core::BadOrder => Some(core::Code::BadOrder),
		core::BadOrigin => Some(core::Code::BadOrigin),
		core::BadAlign => Some(core::Code::BadAlign),
		core::BadCallBack => Some(core::Code::BadCallBack),
		core::BadTileSize => Some(core::Code::BadTileSize),
		core::BadCOI => Some(core::Code::BadCOI),
		core::BadROISize => Some(core::Code::BadROISize),
		core::MaskIsTiled => Some(core::Code::MaskIsTiled),
		core::StsNullPtr => Some(core::Code::StsNullPtr),
		core::StsVecLengthErr => Some(core::Code::StsVecLengthErr),
		core::StsFilterStructContentErr => Some(core::Code::StsFilterStructContentErr),
		core::StsKernelStructContentErr => Some(core::Code::StsKernelStructContentErr),
		core::StsFilterOffsetErr => Some(core::Code::StsFilterOffsetErr),
		core::StsBadSize => Some(core::Code::StsBadSize),
		core::StsDivByZero => Some(core::Code::StsDivByZero),
		core::StsInplaceNotSupported => Some(core::Code::StsInplaceNotSupported),
		core::StsObjectNotFound => Some(core::Code::StsObjectNotFound),
		core::StsUnmatchedFormats => Some(core::Code::StsUnmatchedFormats),
		core::StsBadFlag => Some(core::Code::StsBadFlag),
		core::StsBadPoint => Some(core::Code::StsBadPoint),
		core::StsBadMask => Some(core::Code::StsBadMask),
		core::StsUnmatchedSizes => Some(core::Code::StsUnmatchedSizes),
		core::StsUnsupportedFormat => Some(core::Code::StsUnsupportedFormat),
		core::StsOutOfRange => Some(core::Code::StsOutOfRange),
		core::StsParseError => Some(core::Code::StsParseError),
		core::StsNotImplemented => Some(core::Code::StsNotImplemented),
		core::StsBadMemBlock => Some(core::Code::StsBadMemBlock),
		core::StsAssert => Some(core::Code::StsAssert),
		core::GpuNotSupported => Some(core::Code::GpuNotSupported),
		core::GpuApiCallError => Some(core::Code::GpuApiCallError),
		core::OpenGlNotSupported => Some(core::Code::OpenGlNotSupported),
		core::OpenGlApiCallError => Some(core::Code::OpenGlApiCallError),
		core::OpenCLApiCallError => Some(core::Code::OpenCLApiCallError),
		core::OpenCLDoubleNotSupported => Some(core::Code::OpenCLDoubleNotSupported),
		core::OpenCLInitError => Some(core::Code::OpenCLInitError),
		core::OpenCLNoAMDBlasFft => Some(core::Code::OpenCLNoAMDBlasFft),
		_ => None,
	}
}
