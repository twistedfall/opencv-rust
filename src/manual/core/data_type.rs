use crate::core;
use crate::core::{Point3_, Point_, Rect_, Size_, VecN};

#[inline]
pub const fn CV_MAT_DEPTH(flags: i32) -> i32 {
	#![allow(non_snake_case)]
	flags & core::CV_MAT_DEPTH_MASK
}

#[inline]
pub const fn CV_MAKETYPE(depth: i32, cn: i32) -> i32 {
	#![allow(non_snake_case)]
	CV_MAT_DEPTH(depth) + ((cn - 1) << core::CV_CN_SHIFT)
}

pub use CV_MAKETYPE as CV_MAKE_TYPE;

#[cfg(ocvrs_opencv_branch_5)]
#[inline]
pub const fn CV_IS_INT_TYPE(flags: i32) -> bool {
	#![allow(non_snake_case)]
	((1 << CV_MAT_DEPTH(flags)) & 0x1e1f) != 0
}

#[cfg(ocvrs_opencv_branch_5)]
#[inline]
pub const fn CV_IS_FLOAT_TYPE(flags: i32) -> bool {
	#![allow(non_snake_case)]
	((1 << CV_MAT_DEPTH(flags)) & 0x1e0) != 0
}

/// Implement this trait types that are valid to use as Mat elements.
///
/// # Safety
/// Types implementing this trait must adhere to the memory layout declared by the values returned
/// by `opencv_depth()` and `opencv_channels()` functions. In most cases that means that the type
/// must also be `#[repr(C)]`.
pub unsafe trait DataType: Copy {
	/// The shape of bytes occupied by the single layer/channel of the element. E.g. for an 8-bit BGR
	/// image it's `CV_8U` because a single channel for a pixel is unsigned 8 bits. You should use one
	/// of the depth constants for this like `CV_8U`, `CV_8S`, `CV_32F`, etc.
	fn opencv_depth() -> i32;

	/// Amount of layers/channels per element. E.g. for an 8-bit BGR image it's 3 because one pixel
	/// consists of 3 channels: B, G and R.
	fn opencv_channels() -> i32;

	/// OpenCV value for this type as produced by `CV_MAKETYPE()` function
	#[inline]
	fn opencv_type() -> i32 {
		CV_MAKETYPE(Self::opencv_depth(), Self::opencv_channels())
	}
}

macro_rules! data_type {
	($rust_type: ty, $mat_depth: expr, $channels: expr) => {
		unsafe impl $crate::core::DataType for $rust_type {
			#[inline]
			fn opencv_depth() -> i32 {
				$mat_depth
			}

			#[inline]
			fn opencv_channels() -> i32 {
				$channels
			}
		}
	};
}

macro_rules! mchan_fun {
	($name: ident, $depth: expr) => {
		#[inline]
		pub const fn $name(n: i32) -> i32 {
			#![allow(non_snake_case)]
			$crate::core::CV_MAKETYPE($depth, n)
		}
	};
}

// int
data_type!(u8, core::CV_8U, 1);
data_type!(i8, core::CV_8S, 1);
data_type!(u16, core::CV_16U, 1);
data_type!(i16, core::CV_16S, 1);
data_type!(i32, core::CV_32S, 1);

mchan_fun!(CV_8UC, core::CV_8U);
mchan_fun!(CV_8SC, core::CV_8S);
mchan_fun!(CV_16UC, core::CV_16U);
mchan_fun!(CV_16SC, core::CV_16S);
mchan_fun!(CV_32SC, core::CV_32S);

#[cfg(ocvrs_opencv_branch_5)]
mod opencv5 {
	use crate::core;

	data_type!(u32, core::CV_32U, 1);
	data_type!(u64, core::CV_64U, 1);
	data_type!(i64, core::CV_64S, 1);
	data_type!(bool, core::CV_Bool, 1);
	data_type!(core::bfloat, core::CV_16BF, 1);

	mchan_fun!(CV_32UC, core::CV_32U);
	mchan_fun!(CV_64SC, core::CV_64S);
	mchan_fun!(CV_64UC, core::CV_64U);
	mchan_fun!(CV_BoolC, core::CV_Bool);
	mchan_fun!(CV_16BFC, core::CV_16BF);
}
#[cfg(ocvrs_opencv_branch_5)]
pub use opencv5::*;

// float
data_type!(f32, core::CV_32F, 1);
data_type!(f64, core::CV_64F, 1);

mchan_fun!(CV_32FC, core::CV_32F);
mchan_fun!(CV_64FC, core::CV_64F);

#[cfg(not(ocvrs_opencv_branch_34))]
mod half_builtin {
	use crate::core;

	#[cfg(ocvrs_has_inherent_feature_hfloat)]
	data_type!(core::hfloat, core::CV_16F, 1);
	mchan_fun!(CV_16FC, core::CV_16F);
}
#[cfg(not(ocvrs_opencv_branch_34))]
pub use half_builtin::*;

#[cfg(all(feature = "f16", not(ocvrs_opencv_branch_34)))]
mod half {
	use half::f16;

	use crate::core;

	data_type!(f16, core::CV_16F, 1);

	#[cfg(ocvrs_opencv_branch_5)]
	mod opencv5 {
		use half::bf16;

		use crate::core;

		data_type!(bf16, core::CV_16BF, 1);
	}
}

#[cfg(feature = "rgb")]
mod rgb {
	use crate::core;

	data_type!(rgb::RGB8, core::CV_8U, 3);
	data_type!(rgb::RGBA8, core::CV_8U, 4);
	data_type!(rgb::alt::Gray<u8>, core::CV_8U, 1);
	data_type!(rgb::alt::GrayAlpha<u8>, core::CV_8U, 2);
	data_type!(rgb::alt::BGR8, core::CV_8U, 3);
	data_type!(rgb::alt::ARGB8, core::CV_8U, 4);
	data_type!(rgb::alt::BGRA8, core::CV_8U, 4);
	data_type!(rgb::alt::ABGR8, core::CV_8U, 4);
}

unsafe impl<T: DataType, const N: usize> DataType for VecN<T, N> {
	#[inline]
	fn opencv_depth() -> i32 {
		T::opencv_depth()
	}

	#[inline]
	fn opencv_channels() -> i32 {
		i32::try_from(N).expect("Number of channels is out of range for i32")
	}
}

unsafe impl<T: DataType> DataType for Point_<T> {
	#[inline]
	fn opencv_depth() -> i32 {
		T::opencv_depth()
	}

	#[inline]
	fn opencv_channels() -> i32 {
		2
	}
}

unsafe impl<T: DataType> DataType for Point3_<T> {
	#[inline]
	fn opencv_depth() -> i32 {
		T::opencv_depth()
	}

	#[inline]
	fn opencv_channels() -> i32 {
		3
	}
}

unsafe impl<T: DataType> DataType for Size_<T> {
	#[inline]
	fn opencv_depth() -> i32 {
		T::opencv_depth()
	}

	#[inline]
	fn opencv_channels() -> i32 {
		2
	}
}

unsafe impl<T: DataType> DataType for Rect_<T> {
	#[inline]
	fn opencv_depth() -> i32 {
		T::opencv_depth()
	}

	#[inline]
	fn opencv_channels() -> i32 {
		4
	}
}
