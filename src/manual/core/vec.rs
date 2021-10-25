use std::{
	ffi::c_void,
	ops::{Deref, DerefMut},
};

use crate::{
	core::{_InputArray, ToInputArray},
	input_array_ref_forward,
	Result,
	sys,
	traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
};

pub use self::{scalar_inner::ValidScalarType, vec_inner::ValidVecType};

// additional modules needed because valid_types! introduces module named "private"
mod vec_inner {
	valid_types!(ValidVecType: i8, u8, i16, u16, i32, f32, f64);
}

mod scalar_inner {
	valid_types!(ValidScalarType: i32, f64);
}

/// [docs.opencv.org](https://docs.opencv.org/master/d6/dcf/classcv_1_1Vec.html)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec<T, const N: usize> (pub [T; N]);

impl<T, const N: usize> Default for Vec<T, N>
	where
		[T; N]: Default
{
	#[inline]
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<T: Copy, const N: usize> Vec<T, N> {
	#[inline]
	pub fn all(v0: T) -> Self {
		Self::from([v0; N])
	}
}

impl<T, const N: usize> From<[T; N]> for Vec<T, N> {
	#[inline]
	fn from(s: [T; N]) -> Self {
		Self(s)
	}
}

impl<T, const N: usize> Deref for Vec<T, N> {
	type Target = [T; N];

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T, const N: usize> DerefMut for Vec<T, N> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T: ValidVecType, const N: usize> OpenCVType<'_> for Vec<T, N> {
	type Arg = Self;
	type ExternReceive = Self;
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self> { Ok(self) }

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self { self }

	#[inline]
	unsafe fn opencv_from_extern(s: Self) -> Self { s }
}

impl<T: ValidVecType, const N: usize> OpenCVTypeArg<'_> for Vec<T, N> {
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self> { Ok(self) }

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self { self }
}

impl<T: ValidVecType, const N: usize> OpenCVTypeExternContainer for Vec<T, N> {
	type ExternSend = *const Self;
	type ExternSendMut = *mut Self;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend { self }

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut { self }

	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut { &mut *std::mem::ManuallyDrop::new(self) as _ }
}

pub type Scalar_<T> = Vec<T, 4>;

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

input_array_ref_forward! { Scalar_<f64> }
