use std::ffi::c_void;

use crate::{
	core::{_InputArray, ToInputArray},
	opencv_type_simple_generic,
	Result,
	sys,
	traits::Boxed,
};

pub use self::{scalar_inner::ValidScalarType, vec_inner::ValidVecType};

// additional modules needed because valid_types! introduces module named "private"
mod vec_inner {
	valid_types!(ValidVecType: i8, u8, i16, u16, i32, f32, f64);
}

mod scalar_inner {
	valid_types!(ValidScalarType: i32, f64);
}

macro_rules! vec_impl {
	($type: ident, $count: expr, $type_trait: ident) => {
		/// [docs.opencv.org](https://docs.opencv.org/master/d6/dcf/classcv_1_1Vec.html)
		#[repr(C)]
		#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
		pub struct $type<T: $type_trait>(pub [T; $count]);

		impl<T: $type_trait> $type<T> {
			#[inline]
			pub fn all(v0: T) -> Self {
				Self::from([v0; $count])
			}
		}

		impl<T: $type_trait> From<[T; $count]> for $type<T> {
			#[inline]
			fn from(s: [T; $count]) -> Self {
				Self(s)
			}
		}

		impl<T: $type_trait> std::ops::Deref for $type<T> {
			type Target = [T; $count];

			#[inline]
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl<T: $type_trait> std::ops::DerefMut for $type<T> {
			#[inline]
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

		opencv_type_simple_generic! { $type<$type_trait> }
	};
}

vec_impl!(Vec2, 2, ValidVecType);
vec_impl!(Vec3, 3, ValidVecType);
vec_impl!(Vec4, 4, ValidVecType);
vec_impl!(Vec6, 6, ValidVecType);
vec_impl!(Vec8, 8, ValidVecType);
vec_impl!(Vec18, 18, ValidVecType);
vec_impl!(Scalar_, 4, ValidScalarType);

impl<T: ValidScalarType> Scalar_<T> {
	#[inline]
	pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self::from([v0, v1, v2, v3])
	}
}

impl<T: ValidScalarType> From<T> for Scalar_<T> {
	#[inline]
	fn from(v0: T) -> Self {
		Self::from([v0, T::zero(), T::zero(), T::zero()])
	}
}

impl<T: ValidScalarType> From<(T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T)) -> Self {
		Self::from([v.0, v.1, T::zero(), T::zero()])
	}
}

impl<T: ValidScalarType> From<(T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T)) -> Self {
		Self::from([v.0, v.1, v.2, T::zero()])
	}
}

impl<T: ValidScalarType> From<(T, T, T, T)> for Scalar_<T> {
	#[inline]
	fn from(v: (T, T, T, T)) -> Self {
		Self::from([v.0, v.1, v.2, v.3])
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
