use std::any::type_name;

use crate::{core, Error, Result};

/// Common trait for all enums that represent flags, e.g. can be combined using bitwise operations
pub trait OpenCVBitfieldEnum: Sized + Copy + 'static {
	/// List of all valid separate flags for this enum, mostly for internal use
	const ALL_FLAGS: &'static [Self];

	/// Construct the enum from an `i32` value without checking its validity
	unsafe fn from_i32_unchecked(value: i32) -> Self;

	/// Get the `i32` value of this enum
	fn to_i32(self) -> i32;

	/// Construct the enum from an `i32` value, returning an error if any of the set bits do not have associated flags
	fn try_from_i32(value: i32) -> Result<Self> {
		let mut rem = value;
		for flag in Self::ALL_FLAGS {
			rem &= !flag.to_i32();
			if rem == 0 {
				break;
			}
		}
		if rem == 0 {
			Ok(unsafe { Self::from_i32_unchecked(value) })
		} else {
			Err(Error::new(
				core::StsBadArg,
				format!("Value: {value} is not valid for enum: {}", type_name::<Self>()),
			))
		}
	}

	/// Check whether the specified flag or flags are set.
	#[inline]
	fn is_set(self, flag: Self) -> bool {
		let s = self.to_i32();
		let f = flag.to_i32();
		if f == 0 {
			s == 0
		} else {
			s & f == f
		}
	}

	/// Set the specified flag or flags
	///
	/// If the specified `flag` has `0` value, then it replaces the current value completely as it's the only way to "set"
	/// `0`-value flag.
	#[inline]
	fn set(&mut self, flag: Self) {
		if flag.to_i32() == 0 {
			*self = flag;
		} else {
			let v = self.to_i32() | flag.to_i32();
			*self = unsafe { Self::from_i32_unchecked(v) };
		}
	}

	/// Clear the specified flag or flags
	///
	/// If the specified `flag` has `0` value, then it does nothing.
	#[inline]
	fn clear(&mut self, flag: Self) {
		let v = self.to_i32() & !flag.to_i32();
		*self = unsafe { Self::from_i32_unchecked(v) };
	}

	/// Toggle the specified flag or flags
	#[inline]
	fn toggle(&mut self, flag: Self) {
		let v = self.to_i32() ^ flag.to_i32();
		*self = unsafe { Self::from_i32_unchecked(v) };
	}

	/// Return a new value with the specified flag or flags set in addition to the current ones
	#[inline]
	fn with(mut self, flags: Self) -> Self {
		self.set(flags);
		self
	}

	/// Return a new value with the specified flag or flags cleared from the current ones
	#[inline]
	fn without(mut self, flags: Self) -> Self {
		self.clear(flags);
		self
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! opencv_type_enum {
	($enum: ty { $($variant: ident),+ }) => {
		$crate::opencv_type_copy! { $enum }

		impl TryFrom<i32> for $enum {
			type Error = $crate::Error;

			fn try_from(value: i32) -> Result<Self, Self::Error> {
				match value {
					$(
						_ if value == i32::from(Self::$variant) => Ok(Self::$variant),
					)+
					_ => Err(Self::Error::new(
						$crate::core::StsBadArg,
						format!("Value: {value} is not valid for enum: {}", stringify!($enum))
					)),
				}
			}
		}

		impl From<$enum> for i32 {
			#[inline]
			fn from(v: $enum) -> Self {
				// safe because our enums are repr(i32)
				v as Self
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! opencv_type_bitfield_enum {
	($enum: ty { $($variant: ident),+ }) => {
		$crate::opencv_type_copy! { $enum }

		impl $crate::traits::OpenCVBitfieldEnum for $enum {
			const ALL_FLAGS: &'static [Self] = &[$(Self::$variant),+];

			#[inline]
			unsafe fn from_i32_unchecked(value: i32) -> Self {
				Self(value)
			}

			#[inline]
			fn to_i32(self) -> i32 {
				self.0
			}
		}

		impl TryFrom<i32> for $enum {
			type Error = $crate::Error;

			#[inline]
			fn try_from(value: i32) -> Result<Self, Self::Error> {
				Self::try_from_i32(value)
			}
		}

		impl From<$enum> for i32 {
			#[inline]
			fn from(v: $enum) -> Self {
				v.to_i32()
			}
		}
	};
}
