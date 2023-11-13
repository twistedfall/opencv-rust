use std::convert::TryFrom;
use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;

use crate::core::{
	Mat, MatTrait, MatTraitConst, MatTraitConstManual, MatTraitManual, ToInputArray, ToInputOutputArray, ToOutputArray,
	_InputArray, _InputOutputArray, _OutputArray,
};
use crate::traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer, OpenCVTypeExternContainerMove};
use crate::{Error, Result};

use super::{match_dims, match_format, match_is_continuous, match_total, DataType};

/// [docs.opencv.org](https://docs.opencv.org/master/df/dfc/classcv_1_1Mat__.html)
///
/// This struct is freely convertible into and from `Mat` using `into` and `try_from` methods. You might want
/// to convert `Mat` to `Mat_` before calling typed methods (like `at`, `data_typed`) when more performance is
/// required because this way you will skip the data type checks (still WIP, not all methods are covered).
pub struct Mat_<T> {
	inner: Mat,
	_type: PhantomData<T>,
}

impl<T: DataType> TryFrom<Mat> for Mat_<T> {
	type Error = Error;

	#[inline]
	fn try_from(mat: Mat) -> Result<Self, Self::Error> {
		match_format::<T>(mat.typ()).map(|_| Self {
			inner: mat,
			_type: PhantomData,
		})
	}
}

impl<T: DataType> From<Mat_<T>> for Mat {
	#[inline]
	fn from(s: Mat_<T>) -> Self {
		s.inner
	}
}

impl<T: DataType> Mat_<T> {
	#[inline]
	pub fn into_untyped(self) -> Mat {
		self.into()
	}

	#[inline]
	pub fn as_untyped(&self) -> &Mat {
		&self.inner
	}

	#[inline]
	pub fn as_raw_Mat_(&self) -> *const c_void {
		#![allow(non_snake_case)]
		self.as_raw_Mat()
	}

	#[inline]
	pub fn as_raw_mut_Mat_(&mut self) -> *mut c_void {
		#![allow(non_snake_case)]
		self.as_raw_mut_Mat()
	}

	#[inline]
	pub fn at(&self, i0: i32) -> Result<&T> {
		match_dims(self, 2)
			.and_then(|_| match_total(self, i0))
			.and_then(|_| unsafe { self.at_unchecked(i0) })
	}

	#[inline]
	pub fn at_mut(&mut self, i0: i32) -> Result<&mut T> {
		match_dims(self, 2).and_then(|_| match_total(self, i0))?;
		unsafe { self.at_unchecked_mut(i0) }
	}

	#[inline]
	pub fn data_typed(&self) -> Result<&[T]> {
		match_is_continuous(self).and_then(|_| unsafe { self.data_typed_unchecked() })
	}

	#[inline]
	pub fn data_typed_mut(&mut self) -> Result<&mut [T]> {
		match_is_continuous(self)?;
		unsafe { self.data_typed_unchecked_mut() }
	}
}

impl<T> MatTraitConst for Mat_<T> {
	#[inline]
	fn as_raw_Mat(&self) -> *const c_void {
		self.inner.as_raw_Mat()
	}
}

impl<T> MatTrait for Mat_<T> {
	#[inline]
	fn as_raw_mut_Mat(&mut self) -> *mut c_void {
		self.inner.as_raw_mut_Mat()
	}
}

impl<T> Boxed for Mat_<T> {
	#[inline]
	unsafe fn from_raw(ptr: *mut c_void) -> Self {
		Self {
			inner: Mat::from_raw(ptr),
			_type: PhantomData,
		}
	}

	#[inline]
	fn into_raw(self) -> *mut c_void {
		self.inner.into_raw()
	}

	#[inline]
	fn as_raw(&self) -> *const c_void {
		self.inner.as_raw()
	}

	#[inline]
	fn as_raw_mut(&mut self) -> *mut c_void {
		self.inner.as_raw_mut()
	}
}

impl<T> ToInputArray for Mat_<T> {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		self.inner.input_array()
	}
}

impl<T> ToOutputArray for Mat_<T> {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		self.inner.output_array()
	}
}

impl<T> ToInputOutputArray for Mat_<T> {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		self.inner.input_output_array()
	}
}

impl<T> OpenCVType<'_> for Mat_<T> {
	type Arg = Self;
	type ExternReceive = *mut c_void;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		Self::from_raw(s)
	}
}

impl<T> OpenCVTypeArg<'_> for Mat_<T> {
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl<T> OpenCVTypeExternContainer for Mat_<T> {
	type ExternSend = *const c_void;
	type ExternSendMut = *mut c_void;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.as_raw()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
		self.as_raw_mut()
	}
}

impl<T> OpenCVTypeExternContainerMove for Mat_<T> {
	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut {
		self.into_raw()
	}
}

impl<T> fmt::Debug for Mat_<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f)
	}
}
