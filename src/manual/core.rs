pub use affine3::*;
pub use CV_MAKETYPE as CV_MAKE_TYPE;
pub use data_type::*;
pub use gpumat::*;
pub use input_output_array::*;
pub use mat::*;
pub use mat_ops::*;
pub use matx::*;
pub use point::*;
pub use point3::*;
pub use ptr::*;
pub use rect::*;
pub use scalar::*;
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

mod affine3;
mod data_type;
mod gpumat;
mod input_output_array;
mod mat;
mod mat_ops;
mod matx;
mod point3;
mod point;
pub(crate) mod ptr;
mod rect;
mod scalar;
mod size;
mod sized;
mod vec;
mod vector;
