use std::convert::TryFrom;
use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;

use crate::boxed_ref::{BoxedRef, BoxedRefMut};
use crate::core::{
	Mat, MatTrait, MatTraitConst, MatTraitConstManual, MatTraitManual, Point, ToInputArray, ToInputOutputArray, ToOutputArray,
	_InputArray, _InputOutputArray, _OutputArray,
};
use crate::traits::Boxed;
use crate::{Error, Result};

use super::{match_format, match_indices, match_is_continuous, match_total, DataType};

/// [docs.opencv.org](https://docs.opencv.org/master/df/dfc/classcv_1_1Mat__.html)
///
/// This struct is freely convertible into and from `Mat` using `into` and `try_from` methods. You might want
/// to convert `Mat` to `Mat_` before calling typed methods (like `at`, `data_typed`) when more performance is
/// required because this way you will skip the data type checks.
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

	/// See [Mat::at]
	#[inline]
	pub fn at(&self, i0: i32) -> Result<&T> {
		match_total(self, i0).and_then(|_| unsafe { self.at_unchecked(i0) })
	}

	/// See [Mat::at_2d]
	#[inline]
	pub fn at_2d(&self, row: i32, col: i32) -> Result<&T> {
		match_indices(self, &[row, col]).and_then(|_| unsafe { self.at_2d_unchecked(row, col) })
	}

	/// See [Mat::at_3d]
	#[inline]
	pub fn at_3d(&self, i0: i32, i1: i32, i2: i32) -> Result<&T> {
		match_indices(self, &[i0, i1, i2]).and_then(|_| unsafe { self.at_3d_unchecked(i0, i1, i2) })
	}

	/// See [Mat::at_nd]
	#[inline]
	pub fn at_nd(&self, idx: &[i32]) -> Result<&T> {
		match_indices(self, idx).and_then(|_| unsafe { self.at_nd_unchecked(idx) })
	}

	/// See [Mat::at_pt]
	#[inline]
	pub fn at_pt(&self, pt: Point) -> Result<&T> {
		self.at_2d(pt.y, pt.x)
	}

	/// See [Mat::at_row]
	#[inline]
	pub fn at_row(&self, row: i32) -> Result<&[T]> {
		match_indices(self, &[row, 0]).and_then(|_| unsafe { self.at_row_unchecked(row) })
	}

	/// See [Mat::at_mut]
	#[inline]
	pub fn at_mut(&mut self, i0: i32) -> Result<&mut T> {
		match_total(self, i0)?;
		unsafe { self.at_unchecked_mut(i0) }
	}

	/// See [Mat::at_2d_mut]
	#[inline]
	pub fn at_2d_mut(&mut self, row: i32, col: i32) -> Result<&mut T> {
		match_indices(self, &[row, col])?;
		unsafe { self.at_2d_unchecked_mut(row, col) }
	}

	/// See [Mat::at_3d_mut]
	#[inline]
	pub fn at_3d_mut(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut T> {
		match_indices(self, &[i0, i1, i2])?;
		unsafe { self.at_3d_unchecked_mut(i0, i1, i2) }
	}

	/// See [Mat::at_nd_mut]
	#[inline]
	pub fn at_nd_mut(&mut self, idx: &[i32]) -> Result<&mut T> {
		match_indices(self, idx)?;
		unsafe { self.at_nd_unchecked_mut(idx) }
	}

	/// See [Mat::at_pt_mut]
	#[inline]
	pub fn at_pt_mut(&mut self, pt: Point) -> Result<&mut T> {
		self.at_2d_mut(pt.y, pt.x)
	}

	/// See [Mat::at_row_mut]
	#[inline]
	pub fn at_row_mut(&mut self, row: i32) -> Result<&mut [T]> {
		match_indices(self, &[row, 0])?;
		unsafe { self.at_row_unchecked_mut(row) }
	}

	/// See [Mat::data_typed]
	#[inline]
	pub fn data_typed(&self) -> Result<&[T]> {
		match_is_continuous(self).and_then(|_| unsafe { self.data_typed_unchecked() })
	}

	/// See [Mat::data_typed_mut]
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
	fn input_array(&self) -> Result<BoxedRef<_InputArray>> {
		self.inner.input_array()
	}
}

impl<T> ToOutputArray for Mat_<T> {
	#[inline]
	fn output_array(&mut self) -> Result<BoxedRefMut<_OutputArray>> {
		self.inner.output_array()
	}
}

impl<T> ToInputOutputArray for Mat_<T> {
	#[inline]
	fn input_output_array(&mut self) -> Result<BoxedRefMut<_InputOutputArray>> {
		self.inner.input_output_array()
	}
}

impl<T> fmt::Debug for Mat_<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f)
	}
}
