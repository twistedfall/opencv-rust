use crate::core::{self, Point3_, Point_, Rect_, Size_, ValidPoint3Type, ValidPointType, ValidRectType, ValidSizeType, VecN};

#[inline]
pub const fn CV_MAT_DEPTH(flags: i32) -> i32 {
	#![allow(non_snake_case)]
	flags & crate::core::Mat_DEPTH_MASK
}

#[inline]
pub const fn CV_MAKETYPE(depth: i32, cn: i32) -> i32 {
	#![allow(non_snake_case)]
	CV_MAT_DEPTH(depth) + ((cn - 1) << crate::core::CV_CN_SHIFT)
}

/// This sealed trait is implemented for types that are valid to use as Mat elements
pub trait DataType: Copy + private::Sealed {
	fn depth() -> i32;
	fn channels() -> i32;

	#[inline]
	fn typ() -> i32 {
		CV_MAKETYPE(Self::depth(), Self::channels())
	}
}

macro_rules! data_type {
	($rust_type: ty, $mat_depth: expr, $channels: expr) => {
		impl $crate::core::DataType for $rust_type {
			#[inline]
			fn depth() -> i32 { $mat_depth }

			#[inline]
			fn channels() -> i32 { $channels }
		}

		impl private::Sealed for $rust_type {}
	};
}

// int
data_type!(u8, core::CV_8U, 1);
data_type!(i8, core::CV_8S, 1);
data_type!(u16, core::CV_16U, 1);
data_type!(i16, core::CV_16S, 1);
data_type!(i32, core::CV_32S, 1);

// float
data_type!(f32, core::CV_32F, 1);
data_type!(f64, core::CV_64F, 1);

#[cfg(feature = "rgb")]
use rgb::*;
#[cfg(feature = "rgb")]
use rgb::alt::*;
#[cfg(feature = "rgb")]
data_type!(GRAY8, core::CV_8U, 1);
#[cfg(feature = "rgb")]
data_type!(GRAYA8, core::CV_8U, 2);
#[cfg(feature = "rgb")]
data_type!(RGB8, core::CV_8U, 3);
#[cfg(feature = "rgb")]
data_type!(BGR8, core::CV_8U, 3);
#[cfg(feature = "rgb")]
data_type!(RGBA8, core::CV_8U, 4);
#[cfg(feature = "rgb")]
data_type!(ARGB8, core::CV_8U, 4);
#[cfg(feature = "rgb")]
data_type!(BGRA8, core::CV_8U, 4);
#[cfg(feature = "rgb")]
data_type!(ABGR8, core::CV_8U, 4);

impl<T: DataType, const N: usize> DataType for VecN<T, N> {
	#[inline]
	fn depth() -> i32 { T::depth() }

	#[inline]
	fn channels() -> i32 { N as i32 }
}

impl<T: ValidPointType + DataType> DataType for Point_<T> {
	#[inline]
	fn depth() -> i32 { T::depth() }

	#[inline]
	fn channels() -> i32 { 2 }
}

impl<T: ValidPoint3Type + DataType> DataType for Point3_<T> {
	#[inline]
	fn depth() -> i32 { T::depth() }

	#[inline]
	fn channels() -> i32 { 3 }
}

impl<T: ValidSizeType + DataType> DataType for Size_<T> {
	#[inline]
	fn depth() -> i32 { T::depth() }

	#[inline]
	fn channels() -> i32 { 2 }
}

impl<T: ValidRectType + DataType> DataType for Rect_<T> {
	#[inline]
	fn depth() -> i32 { T::depth() }

	#[inline]
	fn channels() -> i32 { 4 }
}

mod private {
	pub trait Sealed {}
}

impl<T: DataType, const N: usize> private::Sealed for VecN<T, N> {}

impl<T: ValidPointType + DataType> private::Sealed for Point_<T> {}

impl<T: ValidPoint3Type + DataType> private::Sealed for Point3_<T> {}

impl<T: ValidSizeType + DataType> private::Sealed for Size_<T> {}

impl<T: ValidRectType + DataType> private::Sealed for Rect_<T> {}
