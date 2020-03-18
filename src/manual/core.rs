pub use CV_MAKETYPE as CV_MAKE_TYPE;

pub use affine3::*;
pub use input_output_array::*;
pub use mat::*;
pub use matx::*;
pub use point::*;
pub use point3::*;
pub use rect::*;
pub use size::*;
pub use sized::*;
pub use vec::*;

macro_rules! valid_types {
	($trait: ident, $($rust_type: ty),+) => {
		/// This sealed trait is implemented for types that are valid to use in corresponding context
		pub trait $trait: ::num::traits::NumAssign + PartialOrd + Default + Copy + private::Sealed {}

		mod private {
			pub trait Sealed {}
		}

		$(
			impl $trait for $rust_type {}
			impl private::Sealed for $rust_type {}
		)+
	};
}

mod affine3;
mod input_output_array;
mod mat;
mod matx;
mod point3;
mod point;
mod rect;
mod size;
mod sized;
mod vec;

#[inline(always)]
pub const fn CV_MAT_DEPTH(flags: i32) -> i32 {
	#![allow(non_snake_case)]
	flags & crate::core::Mat_DEPTH_MASK
}

#[inline(always)]
pub const fn CV_MAKETYPE(depth: i32, cn: i32) -> i32 {
	#![allow(non_snake_case)]
	CV_MAT_DEPTH(depth) + ((cn - 1) << crate::core::CV_CN_SHIFT)
}
