use std::convert::TryInto;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;
use std::{fmt, slice};

pub use mat_::*;

use crate::core::{MatConstIterator, MatExpr, MatSize, MatStep, Point, Scalar, UMat};
use crate::platform_types::size_t;
use crate::prelude::*;
use crate::{core, input_output_array, Error, Result};

mod mat_;

#[inline(always)]
unsafe fn convert_ptr<'r, T>(r: *const u8) -> &'r T {
	&*(r as *const T)
}

#[inline(always)]
unsafe fn convert_ptr_mut<'r, T>(r: *mut u8) -> &'r mut T {
	&mut *(r as *mut T)
}

#[inline]
fn match_format<T: DataType>(mat_type: i32) -> Result<()> {
	let out_type = T::opencv_type();
	if mat_type == out_type {
		Ok(())
	} else {
		#[cfg(not(ocvrs_opencv_branch_32))]
		let mat_type = core::type_to_string(mat_type)?;
		#[cfg(not(ocvrs_opencv_branch_32))]
		let out_type = core::type_to_string(out_type)?;
		Err(Error::new(
			core::StsUnmatchedFormats,
			format!("Mat type is: {mat_type}, but requested type is: {out_type}"),
		))
	}
}

#[inline]
fn match_dims(mat: &(impl MatTraitConst + ?Sized), dims: usize) -> Result<()> {
	let mat_dims = mat.dims() as usize;
	if mat_dims == dims {
		Ok(())
	} else {
		Err(Error::new(
			core::StsUnmatchedSizes,
			format!("Mat dims is: {mat_dims}, but requested dims is: {dims}"),
		))
	}
}

fn match_indices(mat: &(impl MatTraitConst + ?Sized), idx: &[i32]) -> Result<()> {
	let size = mat.mat_size();
	match_dims(mat, idx.len())?;
	let mut out_of_bounds = size.iter().enumerate().filter(|&(i, &x)| idx[i] < 0 || idx[i] >= x);
	if let Some((out_dim, out_size)) = out_of_bounds.next() {
		Err(Error::new(
			core::StsOutOfRange,
			format!(
				"Index: {} along dimension: {} out of bounds 0..{}",
				idx[out_dim], out_dim, out_size
			),
		))
	} else {
		Ok(())
	}
}

#[inline]
fn match_total(mat: &(impl MatTraitConst + ?Sized), idx: i32) -> Result<()> {
	let size = mat.total();
	if 0 <= idx && (idx as usize) < size {
		Ok(())
	} else {
		Err(Error::new(
			core::StsOutOfRange,
			format!("Index: {idx} out of bounds: 0..{size}"),
		))
	}
}

#[inline]
fn match_is_continuous(mat: &(impl MatTraitConst + ?Sized)) -> Result<()> {
	if mat.is_continuous() {
		Ok(())
	} else {
		Err(Error::new(
			core::StsUnmatchedSizes,
			"Mat is not continuous, operation is not applicable",
		))
	}
}

#[inline(always)]
fn idx_to_row_col(mat: &(impl MatTraitConst + ?Sized), i0: i32) -> Result<(i32, i32)> {
	Ok(if mat.is_continuous() {
		(0, i0)
	} else {
		let mat_size = mat.size()?;
		if mat_size.width == 1 {
			(0, i0)
		} else if mat_size.height == 1 {
			(i0, 0)
		} else {
			let i = i0 / mat_size.height;
			(i, i0 - i * mat_size.height)
		}
	})
}

#[inline]
fn row_count_i32(row_count: usize) -> Result<i32> {
	i32::try_from(row_count).map_err(|_| Error::new(core::StsBadArg, format!("Row count: {row_count} is too high")))
}

#[inline]
fn col_count_i32(col_count: usize) -> Result<i32> {
	i32::try_from(col_count).map_err(|_| Error::new(core::StsBadArg, format!("Column count: {col_count} is too high")))
}

impl Mat {
	/// Create new `Mat` from the iterator of known size
	pub fn from_exact_iter<T: DataType>(s: impl ExactSizeIterator<Item = T>) -> Result<Self> {
		let mut out = unsafe { Self::new_rows_cols(row_count_i32(s.len())?, 1, T::opencv_type()) }?;
		for (i, x) in s.enumerate() {
			// safe because `row_count_i32` ensures that len of `s` fits `i32`
			let i = i as i32;
			unsafe { ({ out.at_unchecked_mut::<T>(i) }? as *mut T).write(x) };
		}
		Ok(out)
	}

	/// Create a new `Mat` by copying the data from a single-dimensional slice
	#[inline]
	pub fn from_slice<T: DataType>(s: &[T]) -> Result<Self> {
		Self::from_slice_2d(&[s])
	}

	/// Create a new `Mat` by copying the data from a 2-dimensional slice (slice of slices)
	pub fn from_slice_2d<T: DataType>(s: &[impl AsRef<[T]>]) -> Result<Self> {
		let col_count = if let Some(first_row) = s.first() {
			col_count_i32(first_row.as_ref().len())?
		} else {
			0
		};
		let row_count = if col_count > 0 {
			row_count_i32(s.len())?
		} else {
			0
		};
		let mut out = Self::new_rows_cols_with_default(row_count, col_count, T::opencv_type(), Scalar::all(0.))?;
		if row_count > 0 && col_count > 0 {
			for (row_n, row) in s.iter().enumerate() {
				// safe because `row_count_i32` ensures that len of `s` fits `i32`
				let row_n = row_n as i32;
				let trg = out.at_row_mut(row_n)?;
				let src = row.as_ref();
				if trg.len() != src.len() {
					return Err(Error::new(
						core::StsUnmatchedSizes,
						format!(
							"Unexpected number of items: {} in a row index: {row_n}, expected: {}",
							trg.len(),
							src.len()
						),
					));
				}
				trg.copy_from_slice(src);
			}
		}
		Ok(out)
	}

	/// Create a new `Mat` by copying the data from a single-dimensional slice with custom shape
	#[inline]
	pub fn from_slice_rows_cols<T: DataType>(s: &[T], row_count: usize, col_count: usize) -> Result<Self> {
		if row_count * col_count != s.len() {
			return Err(Error::new(
				core::StsUnmatchedSizes,
				format!(
					"The length of the slice: {} must match the passed row count: {row_count} and column count: {col_count} exactly",
					s.len()
				),
			));
		}
		unsafe {
			Self::new_rows_cols_with_data(
				row_count_i32(row_count)?,
				col_count_i32(col_count)?,
				T::opencv_type(),
				s.as_ptr() as *mut c_void,
				core::Mat_AUTO_STEP,
			)
		}?
		.try_clone()
	}

	#[inline]
	pub fn try_into_typed<T: DataType>(self) -> Result<Mat_<T>>
	where
		Self: Sized,
	{
		self.try_into()
	}

	/// Returns iterator over Mat elements and their positions
	#[inline]
	pub fn iter<T: DataType>(&self) -> Result<MatIter<T>> {
		match_format::<T>(self.typ())?;
		Ok(if self.empty() {
			MatIter {
				iter: None,
				_d: PhantomData,
			}
		} else {
			MatIter {
				iter: Some(MatConstIterator::over(self)?),
				_d: PhantomData,
			}
		})
	}
}

pub struct MatIter<'m, T> {
	iter: Option<MatConstIterator>,
	_d: PhantomData<&'m T>,
}

impl<T: DataType> Iterator for MatIter<'_, T> {
	type Item = (Point, T);

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.as_mut().and_then(|iter| {
			if iter.has_elements() {
				let cur = iter.current().ok()?;
				let out = (iter.pos().ok()?, *cur);
				iter.seek(1, true).ok()?;
				Some(out)
			} else {
				None
			}
		})
	}
}

pub(crate) mod mat_forward {
	use super::*;

	#[inline]
	pub fn at<T: DataType>(mat: &(impl MatTraitConst + ?Sized), i0: i32) -> Result<&T> {
		match_format::<T>(mat.typ())
			.and_then(|_| match_dims(mat, 2))
			.and_then(|_| match_total(mat, i0))
			.and_then(|_| unsafe { mat.at_unchecked(i0) })
	}

	#[inline]
	pub fn at_mut<T: DataType>(mat: &mut (impl MatTrait + ?Sized), i0: i32) -> Result<&mut T> {
		match_format::<T>(mat.typ())
			.and_then(|_| match_dims(mat, 2))
			.and_then(|_| match_total(mat, i0))?;
		unsafe { mat.at_unchecked_mut(i0) }
	}

	#[inline]
	pub fn at_2d<T: DataType>(mat: &(impl MatTraitConst + ?Sized), row: i32, col: i32) -> Result<&T> {
		match_format::<T>(mat.typ())
			.and_then(|_| match_indices(mat, &[row, col]))
			.and_then(|_| unsafe { mat.at_2d_unchecked(row, col) })
	}

	#[inline]
	pub fn at_2d_mut<T: DataType>(mat: &mut (impl MatTrait + ?Sized), row: i32, col: i32) -> Result<&mut T> {
		match_format::<T>(mat.typ()).and_then(|_| match_indices(mat, &[row, col]))?;
		unsafe { mat.at_2d_unchecked_mut(row, col) }
	}

	#[inline]
	pub fn at_pt<T: DataType>(mat: &(impl MatTraitConst + ?Sized), pt: Point) -> Result<&T> {
		at_2d(mat, pt.y, pt.x)
	}

	#[inline]
	pub fn at_pt_mut<T: DataType>(mat: &mut (impl MatTrait + ?Sized), pt: Point) -> Result<&mut T> {
		at_2d_mut(mat, pt.y, pt.x)
	}

	#[inline]
	pub fn at_3d<T: DataType>(mat: &(impl MatTraitConst + ?Sized), i0: i32, i1: i32, i2: i32) -> Result<&T> {
		match_format::<T>(mat.typ())
			.and_then(|_| match_indices(mat, &[i0, i1, i2]))
			.and_then(|_| unsafe { mat.at_3d_unchecked(i0, i1, i2) })
	}

	#[inline]
	pub fn at_3d_mut<T: DataType>(mat: &mut (impl MatTrait + ?Sized), i0: i32, i1: i32, i2: i32) -> Result<&mut T> {
		match_format::<T>(mat.typ()).and_then(|_| match_indices(mat, &[i0, i1, i2]))?;
		unsafe { mat.at_3d_unchecked_mut(i0, i1, i2) }
	}

	#[inline]
	pub fn at_nd<'s, T: DataType>(mat: &'s (impl MatTraitConst + ?Sized), idx: &[i32]) -> Result<&'s T> {
		match_format::<T>(mat.typ())
			.and_then(|_| match_indices(mat, idx))
			.and_then(|_| unsafe { mat.at_nd_unchecked(idx) })
	}

	#[inline]
	pub fn at_nd_mut<'s, T: DataType>(mat: &'s mut (impl MatTrait + ?Sized), idx: &[i32]) -> Result<&'s mut T> {
		match_format::<T>(mat.typ()).and_then(|_| match_indices(mat, idx))?;
		unsafe { mat.at_nd_unchecked_mut(idx) }
	}
}

pub trait MatTraitConstManual: MatTraitConst {
	/// Like `Mat::at()` but performs no bounds or type checks
	///
	/// # Safety
	/// Caller must ensure that index is within Mat bounds
	#[inline]
	unsafe fn at_unchecked<T: DataType>(&self, i0: i32) -> Result<&T> {
		let mat_size = self.size()?;
		let (i, j) = if self.is_continuous() || mat_size.width == 1 {
			(0, i0)
		} else if mat_size.height == 1 {
			(i0, 0)
		} else {
			let i = i0 / mat_size.height;
			(i, i0 - i * mat_size.height)
		};
		self.ptr_2d(i, j).map(|ptr| convert_ptr(ptr))
	}

	/// Like `Mat::at_2d()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_2d_unchecked<T: DataType>(&self, row: i32, col: i32) -> Result<&T> {
		self.ptr_2d(row, col).map(|ptr| convert_ptr(ptr))
	}

	/// Like `Mat::at_pt()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that point is within Mat bounds
	#[inline]
	unsafe fn at_pt_unchecked<T: DataType>(&self, pt: Point) -> Result<&T> {
		self.at_2d_unchecked(pt.y, pt.x)
	}

	/// Like `Mat::at_3d()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_3d_unchecked<T: DataType>(&self, i0: i32, i1: i32, i2: i32) -> Result<&T> {
		self.ptr_3d(i0, i1, i2).map(|ptr| convert_ptr(ptr))
	}

	/// Like `Mat::at_nd()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_nd_unchecked<T: DataType>(&self, idx: &[i32]) -> Result<&T> {
		self.ptr_nd(idx).map(|ptr| convert_ptr(ptr))
	}

	/// Return a complete read-only row
	#[inline]
	fn at_row<T: DataType>(&self, row: i32) -> Result<&[T]> {
		match_format::<T>(self.typ())
			.and_then(|_| match_indices(self, &[row, 0]))
			.and_then(|_| unsafe { self.at_row_unchecked(row) })
	}

	/// Like `Mat::at_row()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that index is within Mat bounds
	#[inline]
	unsafe fn at_row_unchecked<T: DataType>(&self, row: i32) -> Result<&[T]> {
		let width = self.size()?.width as usize;
		self.ptr(row).map(|x| slice::from_raw_parts(convert_ptr(x), width))
	}

	#[inline]
	fn is_allocated(&self) -> bool {
		!self.data().is_null()
	}

	/// Returns underlying data array as byte slice, Mat must be continuous.
	#[inline]
	fn data_bytes(&self) -> Result<&[u8]> {
		match_is_continuous(self).and_then(|_| {
			let data = self.data();
			if data.is_null() {
				Err(Error::new(core::StsNullPtr, "Function returned null pointer"))
			} else {
				Ok(unsafe { slice::from_raw_parts(data, self.total() * self.elem_size()?) })
			}
		})
	}

	#[inline]
	fn data_typed<T: DataType>(&self) -> Result<&[T]> {
		match_format::<T>(self.typ())
			.and_then(|_| match_is_continuous(self))
			.and_then(|_| unsafe { self.data_typed_unchecked() })
	}

	/// # Safety
	/// Caller must ensure that the `T` type argument corresponds to the data stored in the `Mat`
	#[inline]
	unsafe fn data_typed_unchecked<T: DataType>(&self) -> Result<&[T]> {
		let data = self.data();
		if data.is_null() {
			Err(Error::new(core::StsNullPtr, "Function returned null pointer"))
		} else {
			Ok(slice::from_raw_parts(data as *const T, self.total()))
		}
	}

	fn to_vec_2d<T: DataType>(&self) -> Result<Vec<Vec<T>>> {
		match_format::<T>(self.typ()).and_then(|_| match_dims(self, 2)).and_then(|_| {
			let size = Self::size(self)?;
			let width = size.width as usize;
			if self.is_continuous() {
				let data = self.data_typed()?;
				Ok((0..size.height)
					.map(|row_n| {
						let row_n = row_n as usize;
						let mut row = Vec::with_capacity(width);
						row.extend_from_slice(&data[row_n * width..(row_n + 1) * width]);
						row
					})
					.collect())
			} else {
				Ok((0..size.height)
					.map(|row_n| {
						self.at_row(row_n).map(|src_row| {
							let mut row = Vec::with_capacity(width);
							row.extend_from_slice(src_row);
							row
						})
					})
					.collect::<Result<_>>()?)
			}
		})
	}
}

pub trait MatTraitManual: MatTraitConstManual + MatTrait {
	/// Like `Mat::at_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that index is within Mat bounds
	#[inline]
	unsafe fn at_unchecked_mut<T: DataType>(&mut self, i0: i32) -> Result<&mut T> {
		let (i, j) = idx_to_row_col(self, i0)?;
		self.ptr_2d_mut(i, j).map(|ptr| convert_ptr_mut(ptr))
	}

	/// Like `Mat::at_2d_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_2d_unchecked_mut<T: DataType>(&mut self, row: i32, col: i32) -> Result<&mut T> {
		self.ptr_2d_mut(row, col).map(|ptr| convert_ptr_mut(ptr))
	}

	/// Like `Mat::at_pt_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that point is within Mat bounds
	#[inline]
	unsafe fn at_pt_unchecked_mut<T: DataType>(&mut self, pt: Point) -> Result<&mut T> {
		self.at_2d_unchecked_mut(pt.y, pt.x)
	}

	/// Like `Mat::at_3d_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_3d_unchecked_mut<T: DataType>(&mut self, i0: i32, i1: i32, i2: i32) -> Result<&mut T> {
		self.ptr_3d_mut(i0, i1, i2).map(|ptr| convert_ptr_mut(ptr))
	}

	/// Like `Mat::at_nd_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_nd_unchecked_mut<T: DataType>(&mut self, idx: &[i32]) -> Result<&mut T> {
		self.ptr_nd_mut(idx).map(|ptr| convert_ptr_mut(ptr))
	}

	/// Return a complete writeable row
	#[inline]
	fn at_row_mut<T: DataType>(&mut self, row: i32) -> Result<&mut [T]> {
		match_format::<T>(self.typ()).and_then(|_| match_indices(self, &[row, 0]))?;
		unsafe { self.at_row_unchecked_mut(row) }
	}

	/// Like `Mat::at_row_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that index is within Mat bounds
	#[inline]
	unsafe fn at_row_unchecked_mut<T: DataType>(&mut self, row: i32) -> Result<&mut [T]> {
		let width = self.size()?.width as usize;
		self
			.ptr_mut(row)
			.map(|x| slice::from_raw_parts_mut(convert_ptr_mut(x), width))
	}

	/// Returns underlying data array as mutable byte slice, Mat must be continuous.
	#[inline]
	fn data_bytes_mut(&mut self) -> Result<&mut [u8]> {
		match_is_continuous(self)
			.and_then(|_| Ok(unsafe { slice::from_raw_parts_mut(self.data_mut(), self.total() * self.elem_size()?) }))
	}

	#[inline]
	fn data_typed_mut<T: DataType>(&mut self) -> Result<&mut [T]> {
		match_format::<T>(self.typ()).and_then(|_| match_is_continuous(self))?;
		unsafe { self.data_typed_unchecked_mut() }
	}

	/// # Safety
	/// Caller must ensure that the `T` type argument corresponds to the data stored in the `Mat`
	#[inline]
	unsafe fn data_typed_unchecked_mut<T: DataType>(&mut self) -> Result<&mut [T]> {
		let total = self.total();
		Ok(slice::from_raw_parts_mut(self.data_mut() as *mut T, total))
	}
}

impl<T: MatTraitConst + ?Sized> MatTraitConstManual for T {}

impl<T: MatTrait + ?Sized> MatTraitManual for T {}

input_output_array! { Mat, from_mat, from_mat_mut }

impl fmt::Debug for Mat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let typ = self.typ();
		let depth = self.depth();
		#[cfg(not(ocvrs_opencv_branch_32))]
		let typ = core::type_to_string(typ).map_err(|_| fmt::Error)?;
		#[cfg(not(ocvrs_opencv_branch_32))]
		let depth = core::depth_to_string(depth).map_err(|_| fmt::Error)?;
		f.debug_struct("Mat")
			.field("type", &typ)
			.field("flags", &self.flags())
			.field("channels", &self.channels())
			.field("depth", &depth)
			.field("dims", &self.dims())
			.field("size", &self.size().map_err(|_| fmt::Error)?)
			.field("rows", &self.rows())
			.field("cols", &self.cols())
			.field("elem_size", &self.elem_size().map_err(|_| fmt::Error)?)
			.field("elem_size1", &self.elem_size1())
			.field("total", &self.total())
			.field("is_continuous", &self.is_continuous())
			.field("is_submatrix", &self.is_submatrix())
			.finish()
	}
}

input_output_array! { UMat, from_umat, from_umat_mut }

impl Deref for MatSize {
	type Target = [i32];

	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { slice::from_raw_parts(self.to_xconst_i32(), self.dims() as usize) }
	}
}

impl fmt::Debug for MatSize {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{:#?}", self.deref())
	}
}

impl Deref for MatStep {
	type Target = [size_t];

	#[inline]
	fn deref(&self) -> &Self::Target {
		extern "C" {
			fn cv_manual_MatStep_deref(instance: *const c_void) -> *const size_t;
		}
		let ptr = unsafe { cv_manual_MatStep_deref(self.as_raw_MatStep()) };
		unsafe { slice::from_raw_parts(ptr, 2) }
	}
}

impl fmt::Debug for MatStep {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{:#?}", self.deref())
	}
}

pub trait MatConstIteratorTraitManual: MatConstIteratorTrait {
	#[inline]
	fn current<T: DataType>(&self) -> Result<&T> {
		match_format::<T>(self.typ())?;
		if self.has_elements() {
			self.try_deref().map(|ptr| unsafe { convert_ptr(ptr) })
		} else {
			Err(Error::new(
				core::StsOutOfRange,
				"MatConstIterator doesn't have any more elements",
			))
		}
	}
}

impl<T: MatConstIteratorTrait> MatConstIteratorTraitManual for T {}

input_output_array! { MatExpr, from_matexpr }
