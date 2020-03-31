use std::ffi::c_void;

use crate::{
	core::{_InputArray, Boxed, ToInputArray},
	Result,
	sys,
};

pub use self::{scalar::ValidScalarType, vec::ValidVecType};

// additional modules needed because valid_types! introduces module named "private"
mod vec {
	valid_types!(ValidVecType, i8, u8, i16, u16, i32, f32, f64);
}

mod scalar {
	valid_types!(ValidScalarType, f64);
}

macro_rules! vec_impl {
	($type: ident, $count: expr, $type_trait: ident) => {
		/// [docs.opencv.org](https://docs.opencv.org/master/d6/dcf/classcv_1_1Vec.html)
		#[repr(C)]
		#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
		pub struct $type<T: $type_trait>(pub [T; $count]);

		impl<T: $type_trait> $type<T> {
			pub fn all(v0: T) -> Self {
				Self::from([v0; $count])
			}
		}

		impl<T: $type_trait> From<[T; $count]> for $type<T> {
			fn from(s: [T; $count]) -> Self {
				Self(s)
			}
		}

		impl<T: $type_trait> std::ops::Deref for $type<T> {
			type Target = [T; $count];

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl<T: $type_trait> std::ops::DerefMut for $type<T> {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	};
}

vec_impl!(Vec2, 2, ValidVecType);
vec_impl!(Vec3, 3, ValidVecType);
vec_impl!(Vec4, 4, ValidVecType);
vec_impl!(Vec6, 6, ValidVecType);
vec_impl!(Vec8, 8, ValidVecType);
vec_impl!(Scalar_, 4, ValidScalarType);

impl<T: ValidScalarType> Scalar_<T> {
	pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self::from([v0, v1, v2, v3])
	}
}

impl<T: ValidScalarType> From<T> for Scalar_<T> {
	fn from(v0: T) -> Self {
		Self::from([v0, T::zero(), T::zero(), T::zero()])
	}
}

impl ToInputArray for Scalar_<f64> {
	fn input_array(&self) -> Result<_InputArray> {
		extern "C" { fn cv_Scalar_input_array(instance: *const Scalar_<f64>) -> sys::Result<*mut c_void>; }
		unsafe { cv_Scalar_input_array(self) }
			.into_result()
			.map(|ptr| unsafe { _InputArray::from_raw(ptr) })
	}
}

impl ToInputArray for &Scalar_<f64> {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		(*self).input_array()
	}
}
