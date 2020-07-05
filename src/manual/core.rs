pub use affine3::*;
pub use CV_MAKETYPE as CV_MAKE_TYPE;
pub use gpumat::*;
pub use input_output_array::*;
pub use mat::*;
pub use matx::*;
pub use point::*;
pub use point3::*;
pub use ptr::*;
pub use rect::*;
pub use size::*;
pub use sized::*;
pub use vec::*;
pub use vector::*;

macro_rules! valid_types {
	($trait: ident: $($rust_type: ty),+) => {
		/// This sealed trait is implemented for types that are valid to use in corresponding context
		pub trait $trait: ::num_traits::NumAssign + PartialOrd + Default + Copy + private::Sealed {}

		mod private {
			pub trait Sealed {}
		}

		$(
			impl $trait for $rust_type {}
			impl private::Sealed for $rust_type {}
		)+
	};
}

macro_rules! opencv_type_simple_generic {
	($type: ident<$trait: ident>) => {
		impl<T: $trait> $crate::traits::OpenCVType<'_> for $type<T> {
			type Arg = Self;
			type ExternReceive = Self;
			type ExternContainer = Self;

			#[inline] fn opencv_into_extern_container(self) -> $crate::Result<Self> { Ok(self) }
			#[inline] fn opencv_into_extern_container_nofail(self) -> Self { self }
			#[inline] unsafe fn opencv_from_extern(s: Self) -> Self { s }
		}

		impl<T: $trait> $crate::traits::OpenCVTypeArg<'_> for $type<T> {
			type ExternContainer = Self;

			#[inline] fn opencv_into_extern_container(self) -> $crate::Result<Self> { Ok(self) }
			#[inline] fn opencv_into_extern_container_nofail(self) -> Self { self }
		}

		impl<T: $trait> $crate::traits::OpenCVTypeExternContainer for $type<T> {
			type ExternSend = *const Self;
			type ExternSendMut = *mut Self;

			#[inline] fn opencv_as_extern(&self) -> Self::ExternSend { self }
			#[inline] fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut { self }
			#[inline] fn opencv_into_extern(self) -> Self::ExternSendMut { &mut *std::mem::ManuallyDrop::new(self) as _ }
		}
	};
}

mod affine3;
mod gpumat;
mod input_output_array;
mod mat;
mod matx;
mod point3;
mod point;
pub(crate) mod ptr;
mod rect;
mod size;
mod sized;
mod vec;
mod vector;

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
