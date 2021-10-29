use std::{
	convert::TryInto,
	ffi::c_void,
	fmt,
	ops::Deref,
	slice,
};

pub use mat_::*;

use crate::{
	core::{self, MatExpr, MatSize, MatStep, Point, Scalar, UMat},
	Error,
	input_output_array,
	platform_types::size_t,
	prelude::*,
	Result,
	sys,
};

mod mat_;

#[inline(always)]
unsafe fn convert_ptr<T>(r: &u8) -> &T {
	&*(r as *const u8 as *const T)
}

#[inline(always)]
unsafe fn convert_ptr_mut<T>(r: &mut u8) -> &mut T {
	&mut *(r as *mut u8 as *mut T)
}

#[inline]
fn match_format<T: DataType>(mat_type: i32) -> Result<()> {
	let out_type = T::typ();
	if mat_type == out_type {
		Ok(())
	} else {
		#[cfg(not(ocvrs_opencv_branch_32))]
		let mat_type = core::type_to_string(mat_type)?;
		#[cfg(not(ocvrs_opencv_branch_32))]
		let out_type = core::type_to_string(out_type)?;
		Err(Error::new(core::StsUnmatchedFormats, format!("Mat type is: {}, but requested type is: {}", mat_type, out_type)))
	}
}

#[inline]
fn match_dims(mat: &(impl MatTraitConst + ?Sized), dims: usize) -> Result<()> {
	let mat_dims = mat.dims() as usize;
	if mat_dims == dims {
		Ok(())
	} else {
		Err(Error::new(core::StsUnmatchedSizes, format!("Mat dims is: {}, but requested dims is: {}", mat_dims, dims)))
	}
}

fn match_indices(mat: &(impl MatTraitConst + ?Sized), idx: &[i32]) -> Result<()> {
	let size = mat.mat_size();
	match_dims(mat, idx.len())?;
	let mut out_of_bounds = size.iter()
		.enumerate()
		.filter(|&(i, &x)| idx[i] < 0 || idx[i] >= x);
	if let Some((out_dim, out_size)) = out_of_bounds.next() {
		Err(Error::new(core::StsOutOfRange, format!("Index: {} along dimension: {} out of bounds 0..{}", idx[out_dim], out_dim, out_size)))
	} else {
		Ok(())
	}
}

#[inline]
fn match_total(mat: &(impl MatTraitConst + ?Sized), idx: i32) -> Result<()> {
	let size = mat.total()?;
	if 0 <= idx && (idx as usize) < size {
		Ok(())
	} else {
		Err(Error::new(core::StsOutOfRange, format!("Index: {} out of bounds: 0..{}", idx, size)))
	}
}

#[inline]
fn match_is_continuous(mat: &(impl MatTraitConst + ?Sized)) -> Result<()> {
	if mat.is_continuous()? {
		Ok(())
	} else {
		Err(Error::new(core::StsUnmatchedSizes, "Mat is not continuous, operation is not applicable".to_string()))
	}
}

#[inline(always)]
fn idx_to_row_col(mat: &(impl MatTraitConst + ?Sized), i0: i32) -> Result<(i32, i32)> {
	Ok(if mat.is_continuous()? {
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

impl Mat {
	/// Create new `Mat` from the iterator of known size
	pub fn from_exact_iter<T: DataType>(s: impl ExactSizeIterator<Item=T>) -> Result<Self> {
		let mut out = unsafe { Self::new_rows_cols(s.len() as _, 1, T::typ()) }?;
		for (i, x) in s.enumerate() {
			unsafe { ({ out.at_unchecked_mut::<T>(i as _) }? as *mut T).write(x) };
		}
		Ok(out)
	}

	#[inline]
	pub fn from_slice<T: DataType>(s: &[T]) -> Result<Self> {
		Self::from_slice_2d(&[s])
	}

	pub fn from_slice_2d<T: DataType>(s: &[impl AsRef<[T]>]) -> Result<Self> {
		let row_count: i32 = s.len() as _;
		let col_count: i32 = if row_count > 0 {
			s[0].as_ref().len() as _
		} else {
			0
		};
		let mut out = unsafe { Self::new_rows_cols(row_count, col_count, T::typ()) }?;
		for (row_n, row) in s.iter().enumerate() {
			let trg = out.at_row_mut(row_n as _)?;
			let src = row.as_ref();
			if trg.len() != src.len() {
				return Err(Error::new(core::StsUnmatchedSizes, format!("Unexpected number of items: {} in a row index: {}, expected: {}", trg.len(), row_n, src.len())));
			}
			trg.copy_from_slice(src);
		}
		Ok(out)
	}

	pub fn try_into_typed<T: DataType>(self) -> Result<Mat_<T>> where Self: Sized {
		self.try_into()
	}
}

pub(crate) mod mat_forward {
	use super::*;

	#[inline]
	pub fn at<T: DataType>(mat: &(impl MatTraitConst + ?Sized), i0: i32) -> Result<&T> {
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_dims(mat, 2))
			.and_then(|_| match_total(mat, i0))
			.and_then(|_| unsafe { mat.at_unchecked(i0) })
	}

	#[inline]
	pub fn at_mut<T: DataType>(mat: &mut (impl MatTrait + ?Sized), i0: i32) -> Result<&mut T> {
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_dims(mat, 2))
			.and_then(|_| match_total(mat, i0))?;
		unsafe { mat.at_unchecked_mut(i0) }
	}

	#[inline]
	pub fn at_2d<T: DataType>(mat: &(impl MatTraitConst + ?Sized), row: i32, col: i32) -> Result<&T> {
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_indices(mat, &[row, col]))
			.and_then(|_| unsafe { mat.at_2d_unchecked(row, col) })
	}

	#[inline]
	pub fn at_2d_mut<T: DataType>(mat: &mut (impl MatTrait + ?Sized), row: i32, col: i32) -> Result<&mut T> {
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_indices(mat, &[row, col]))?;
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
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_indices(mat, &[i0, i1, i2]))
			.and_then(|_| unsafe { mat.at_3d_unchecked(i0, i1, i2) })
	}

	#[inline]
	pub fn at_3d_mut<T: DataType>(mat: &mut (impl MatTrait + ?Sized), i0: i32, i1: i32, i2: i32) -> Result<&mut T> {
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_indices(mat, &[i0, i1, i2]))?;
		unsafe { mat.at_3d_unchecked_mut(i0, i1, i2) }
	}

	#[inline]
	pub fn at_nd<'s, T: core::DataType>(mat: &'s (impl MatTraitConst + ?Sized), idx: &[i32]) -> Result<&'s T> {
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_indices(mat, idx))
			.and_then(|_| unsafe { mat.at_nd_unchecked(idx) })
	}

	#[inline]
	pub fn at_nd_mut<'s, T: core::DataType>(mat: &'s mut (impl MatTrait + ?Sized), idx: &[i32]) -> Result<&'s mut T> {
		match_format::<T>(mat.typ()?)
			.and_then(|_| match_indices(mat, idx))?;
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
		let (i, j) = if self.is_continuous()? || mat_size.width == 1 {
			(0, i0)
		} else if mat_size.height == 1 {
			(i0, 0)
		} else {
			let i = i0 / mat_size.height;
			(i, i0 - i * mat_size.height)
		};
		self.ptr_2d(i, j)
			.map(|ptr| convert_ptr(ptr))
	}

	/// Like `Mat::at_2d()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_2d_unchecked<T: DataType>(&self, row: i32, col: i32) -> Result<&T> {
		self.ptr_2d(row, col)
			.map(|ptr| convert_ptr(ptr))
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
		self.ptr_3d(i0, i1, i2)
			.map(|ptr| convert_ptr(ptr))
	}

	/// Like `Mat::at_nd()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_nd_unchecked<T: core::DataType>(&self, idx: &[i32]) -> Result<&T> {
		self.ptr_nd(idx)
			.map(|ptr| convert_ptr(ptr))
	}

	/// Return a complete read-only row
	#[inline]
	fn at_row<T: DataType>(&self, row: i32) -> Result<&[T]> {
		match_format::<T>(self.typ()?)
			.and_then(|_| match_indices(self, &[row, 0]))
			.and_then(|_| unsafe { self.at_row_unchecked(row) })
	}

	/// Like `Mat::at_row()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that index is within Mat bounds
	#[inline]
	unsafe fn at_row_unchecked<T: DataType>(&self, row: i32) -> Result<&[T]> {
		let width = self.size()?.width as usize;
		self.ptr(row)
			.map(|x| slice::from_raw_parts(convert_ptr(x), width))
	}

	#[inline]
	fn size(&self) -> Result<core::Size> {
		extern "C" { fn cv_manual_Mat_size(instance: *const c_void) -> sys::Result<core::Size>; }
		unsafe { cv_manual_Mat_size(self.as_raw_Mat()) }
			.into_result()
	}

	#[inline]
	fn is_allocated(&self) -> bool {
		!self.data().is_null()
	}

	/// Raw pointer to the underlying data array, can be NULL
	#[inline]
	fn data(&self) -> *const u8 {
		extern "C" { fn cv_manual_Mat_data(instance: *const c_void) -> *const u8; }
		unsafe { cv_manual_Mat_data(self.as_raw_Mat()) }
	}

	/// Returns underlying data array as byte slice, Mat must be continuous.
	#[inline]
	fn data_bytes(&self) -> Result<&[u8]> {
		match_is_continuous(self)
			.and_then(|_| {
				let data = self.data();
				if data.is_null() {
					Err(Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))
				} else {
					Ok(unsafe { slice::from_raw_parts(data, self.total()? * self.elem_size()?) })
				}
			})
	}

	#[inline]
	fn data_typed<T: DataType>(&self) -> Result<&[T]> {
		match_format::<T>(self.typ()?)
			.and_then(|_| match_is_continuous(self))
			.and_then(|_| unsafe { self.data_typed_unchecked() })
	}

	/// # Safety
	/// Caller must ensure that the `T` type argument corresponds to the data stored in the `Mat`
	#[inline]
	unsafe fn data_typed_unchecked<T: DataType>(&self) -> Result<&[T]> {
		let data = self.data();
		if data.is_null() {
			Err(Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))
		} else {
			Ok(slice::from_raw_parts(data as *const T, self.total()?))
		}
	}

	fn to_vec_2d<T: DataType>(&self) -> Result<Vec<Vec<T>>> {
		match_format::<T>(self.typ()?)
			.and_then(|_| match_dims(self, 2))
			.and_then(|_| {
				let size = Self::size(self)?;
				let width = size.width as usize;
				if self.is_continuous()? {
					let data = self.data_typed()?;
					Ok((0..size.height)
						.map(|row_n| {
							let row_n = row_n as usize;
							let mut row = Vec::with_capacity(width);
							row.extend_from_slice(&data[row_n * width..(row_n + 1) * width]);
							row
						})
						.collect()
					)
				} else {
					Ok((0..size.height)
						.map(|row_n| {
							self.at_row(row_n).map(|src_row| {
								let mut row = Vec::with_capacity(width);
								row.extend_from_slice(src_row);
								row
							})
						})
						.collect::<Result<_>>()?
					)
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
		self.ptr_2d_mut(i, j)
			.map(|ptr| convert_ptr_mut(ptr))
	}

	/// Like `Mat::at_2d_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_2d_unchecked_mut<T: DataType>(&mut self, row: i32, col: i32) -> Result<&mut T> {
		self.ptr_2d_mut(row, col)
			.map(|ptr| convert_ptr_mut(ptr))
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
		self.ptr_3d_mut(i0, i1, i2)
			.map(|ptr| convert_ptr_mut(ptr))
	}

	/// Like `Mat::at_nd_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that indices are within Mat bounds
	#[inline]
	unsafe fn at_nd_unchecked_mut<T: core::DataType>(&mut self, idx: &[i32]) -> Result<&mut T> {
		self.ptr_nd_mut(idx)
			.map(|ptr| convert_ptr_mut(ptr))
	}

	/// Return a complete writeable row
	#[inline]
	fn at_row_mut<T: DataType>(&mut self, row: i32) -> Result<&mut [T]> {
		match_format::<T>(self.typ()?)
			.and_then(|_| match_indices(self, &[row, 0]))?;
		unsafe { self.at_row_unchecked_mut(row) }
	}

	/// Like `Mat::at_row_mut()` but performs no bounds or type checks
	/// # Safety
	/// Caller must ensure that index is within Mat bounds
	#[inline]
	unsafe fn at_row_unchecked_mut<T: DataType>(&mut self, row: i32) -> Result<&mut [T]> {
		let width = self.size()?.width as usize;
		self.ptr_mut(row)
			.map(|x| slice::from_raw_parts_mut(convert_ptr_mut(x), width))
	}

	/// Sets all or some of the array elements to the specified value.
	///
	/// ## Parameters
	/// * s: Assigned scalar converted to the actual array type.
	#[inline]
	fn set(&mut self, s: Scalar) -> Result<()> {
		extern "C" { fn cv_manual_Mat_set(instance: *mut c_void, s: *const Scalar) -> sys::Result_void; }
		unsafe { cv_manual_Mat_set(self.as_raw_mut_Mat(), &s) }
			.into_result()
	}

	/// Returns underlying data array as mutable byte slice, Mat must be continuous.
	#[inline]
	fn data_bytes_mut(&mut self) -> Result<&mut [u8]> {
		match_is_continuous(self)
			.and_then(|_| {
				Ok(unsafe { slice::from_raw_parts_mut(self.data_mut(), self.total()? * self.elem_size()?) })
			})
	}

	#[inline]
	fn data_typed_mut<T: DataType>(&mut self) -> Result<&mut [T]> {
		match_format::<T>(self.typ()?)
			.and_then(|_| match_is_continuous(self))?;
		unsafe { self.data_typed_unchecked_mut() }
	}

	/// # Safety
	/// Caller must ensure that the `T` type argument corresponds to the data stored in the `Mat`
	#[inline]
	unsafe fn data_typed_unchecked_mut<T: DataType>(&mut self) -> Result<&mut [T]> {
		let total = self.total()?;
		Ok(slice::from_raw_parts_mut(self.data_mut() as *mut u8 as *mut T, total))
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
		let typ = typ.and_then(core::type_to_string);
		#[cfg(not(ocvrs_opencv_branch_32))]
		let depth = depth.and_then(core::depth_to_string);
		f.debug_struct("Mat")
			.field("type", &typ.map_err(|_| fmt::Error)?)
			.field("flags", &self.flags())
			.field("channels", &self.channels().map_err(|_| fmt::Error)?)
			.field("depth", &depth.map_err(|_| fmt::Error)?)
			.field("dims", &self.dims())
			.field("size", &self.size().map_err(|_| fmt::Error)?)
			.field("rows", &self.rows())
			.field("cols", &self.cols())
			.field("elem_size", &self.elem_size().map_err(|_| fmt::Error)?)
			.field("elem_size1", &self.elem_size1().map_err(|_| fmt::Error)?)
			.field("total", &self.total().map_err(|_| fmt::Error)?)
			.field("is_continuous", &self.is_continuous().map_err(|_| fmt::Error)?)
			.field("is_submatrix", &self.is_submatrix().map_err(|_| fmt::Error)?)
			.finish()
	}
}

pub trait UMatTraitConstManual: UMatTraitConst {
	#[inline]
	fn size(&self) -> Result<core::Size> {
		extern "C" { fn cv_manual_UMat_size(instance: *const c_void) -> sys::Result<core::Size>; }
		unsafe { cv_manual_UMat_size(self.as_raw_UMat()) }
			.into_result()
	}
}

impl<T: UMatTraitConst> UMatTraitConstManual for T {}

input_output_array!{ UMat, from_umat, from_umat_mut }

#[cfg(ocvrs_opencv_branch_32)]
pub trait MatSizeTraitConstManual: MatSizeTraitConst {
	#[inline]
	fn dims(&self) -> i32 {
		extern "C" { fn cv_manual_MatSize_dims(instance: *const c_void) -> i32; }
		unsafe { cv_manual_MatSize_dims(self.as_raw_MatSize()) }
	}
}

#[cfg(ocvrs_opencv_branch_32)]
impl<T: MatSizeTraitConst> MatSizeTraitConstManual for T {}

impl Deref for MatSize {
	type Target = [i32];

	#[inline]
	fn deref(&self) -> &Self::Target {
		let ptr = self.to_ri32() as *const i32;
		unsafe { slice::from_raw_parts(ptr, self.dims() as usize) }
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
		extern "C" { fn cv_manual_MatStep_deref(instance: *const c_void) -> *const size_t; }
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
	fn typ(&self) -> i32 {
		extern "C" { fn cv_manual_MatConstIterator_type(instance: *const c_void) -> i32; }
		unsafe { cv_manual_MatConstIterator_type(self.as_raw_MatConstIterator()) }
	}

	#[inline]
	fn has_elements(&self) -> bool {
		extern "C" { fn cv_manual_MatConstIterator_has_elements(instance: *const c_void) -> bool; }
		unsafe { cv_manual_MatConstIterator_has_elements(self.as_raw_MatConstIterator()) }
	}

	#[inline]
	fn current<T: DataType>(&self) -> Result<&T> {
		match_format::<T>(self.typ())?;
		if self.has_elements() {
			self.try_deref().map(|ptr| unsafe { convert_ptr(ptr) })
		} else {
			Err(Error::new(core::StsOutOfRange, "MatConstIterator doesn't have any more elements".to_owned()))
		}
	}
}

impl<T: MatConstIteratorTrait> MatConstIteratorTraitManual for T {}

input_output_array! { MatExpr, from_matexpr }
