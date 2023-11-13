use std::borrow::Borrow;
use std::ffi::c_void;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::{fmt, slice};

pub use iter::{VectorIterator, VectorRefIterator};
pub use vector_extern::{VectorElement, VectorExtern, VectorExternCopyNonBool};

use crate::platform_types::size_t;
use crate::traits::OpenCVTypeExternContainerMove;
use crate::traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer};
use crate::Result;

mod iter;
mod vector_extern;

/// Wrapper for C++ [std::vector](https://en.cppreference.com/w/cpp/container/vector)
pub struct Vector<T: VectorElement>
where
	Self: VectorExtern<T>,
{
	ptr: *mut c_void,
	_d: PhantomData<T>,
}

impl<T: VectorElement> Vector<T>
where
	Self: VectorExtern<T>,
{
	/// Create a new Vector
	#[inline]
	pub fn new() -> Self {
		unsafe { Self::from_raw(Self::extern_new()) }
	}

	/// Create a Vector with pre-defined capacity
	#[inline]
	pub fn with_capacity(capacity: size_t) -> Self {
		let mut out = Self::new();
		out.reserve(capacity);
		out
	}

	/// Create a Vector from iterator
	#[inline]
	pub fn from_iter<'a>(s: impl IntoIterator<Item = <T as OpenCVType<'a>>::Arg>) -> Self {
		#![allow(clippy::should_implement_trait)]
		let mut out = Self::new();
		out.extend(s);
		out
	}

	/// Create a Vector from slice, the element type needs to be Copy (and not bool)
	#[inline]
	pub fn from_slice(s: &[T]) -> Self
	where
		Self: VectorExternCopyNonBool<T>,
	{
		unsafe { Self::from_raw(Self::extern_from_slice(s.as_ptr(), s.len())) }
	}

	#[inline]
	pub fn from_elem<'a>(elem: <T as OpenCVType<'a>>::Arg, n: size_t) -> Self
	where
		<T as OpenCVType<'a>>::Arg: Clone,
	{
		let mut out = Self::with_capacity(n);
		for _ in 0..n {
			out.push(elem.clone());
		}
		out
	}

	/// Return Vector length
	#[inline]
	pub fn len(&self) -> size_t {
		unsafe { self.extern_len() }
	}

	/// Return true if Vector is empty
	#[inline]
	pub fn is_empty(&self) -> bool {
		unsafe { self.extern_is_empty() }
	}

	/// Return Vector current capacity
	#[inline]
	pub fn capacity(&self) -> size_t {
		unsafe { self.extern_capacity() }
	}

	/// Free extra capacity
	#[inline]
	pub fn shrink_to_fit(&mut self) {
		unsafe { self.extern_shrink_to_fit() }
	}

	/// Reserve capacity for `additional` new elements
	#[inline]
	pub fn reserve(&mut self, additional: size_t) {
		unsafe { self.extern_reserve(additional) }
	}

	/// Remove all elements
	#[inline]
	pub fn clear(&mut self) {
		unsafe { self.extern_clear() }
	}

	/// Remove the element at the specified `index`
	#[inline]
	pub fn remove(&mut self, index: size_t) -> Result<()> {
		vector_index_check(index, self.len())?;
		unsafe { self.extern_remove(index) }
		Ok(())
	}

	/// Swap 2 elements in the Vector
	#[inline]
	pub fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
		let len = self.len();
		vector_index_check(index1, len)?;
		vector_index_check(index2, len)?;
		if index1 != index2 {
			unsafe { self.extern_swap(index1, index2) }
		}
		Ok(())
	}

	/// Add new element
	#[inline]
	pub fn push(&mut self, val: <T as OpenCVType>::Arg) {
		let val = val.opencv_into_extern_container_nofail();
		unsafe { self.extern_push(val.opencv_as_extern()) }
	}

	#[inline]
	pub(crate) fn push_owned(&mut self, val: T) {
		let val = val.opencv_into_extern_container_nofail();
		unsafe { self.extern_push_owned(val.opencv_as_extern()) }
	}

	/// Insert a new element at the specified `index`
	#[inline]
	pub fn insert(&mut self, index: size_t, val: <T as OpenCVType>::Arg) -> Result<()> {
		vector_index_check(index, self.len() + 1)?;
		let val = val.opencv_into_extern_container()?;
		unsafe { self.extern_insert(index, val.opencv_as_extern()) }
		Ok(())
	}

	/// Set element at the specified `index`
	#[inline]
	pub fn set(&mut self, index: size_t, val: <T as OpenCVType>::Arg) -> Result<()> {
		vector_index_check(index, self.len())?;
		let val = val.opencv_into_extern_container()?;
		unsafe { self.extern_set(index, val.opencv_as_extern()) }
		Ok(())
	}

	/// Same as `set()` but without bounds checking
	/// # Safety
	/// Caller must ensure that the specified `index` is within the `Vector` bounds
	#[inline]
	pub unsafe fn set_unchecked(&mut self, index: size_t, val: <T as OpenCVType>::Arg) {
		let val = val.opencv_into_extern_container_nofail();
		self.extern_set(index, val.opencv_as_extern())
	}

	/// Get element at the specified `index`
	#[inline]
	pub fn get(&self, index: size_t) -> Result<T> {
		vector_index_check(index, self.len())?;
		Ok(unsafe { self.get_unchecked(index) })
	}

	/// Same as `get()` but without bounds checking
	/// # Safety
	/// Caller must ensure that the specified `index` is within the `Vector` bounds
	#[inline]
	pub unsafe fn get_unchecked(&self, index: size_t) -> T {
		let val = self.extern_get(index);
		T::opencv_from_extern(val)
	}

	#[inline]
	pub fn iter(&self) -> VectorRefIterator<T> {
		VectorRefIterator::new(self)
	}

	/// Return slice to the elements of the array.
	///
	/// This method is only available for OpenCV types that are Copy, with the exception of bool
	/// because bool is handled in a special way on the C++ side.
	#[inline]
	pub fn as_slice(&self) -> &[T]
	where
		Self: VectorExternCopyNonBool<T>,
	{
		unsafe { slice::from_raw_parts(self.extern_data(), self.len()) }
	}

	/// Return mutable slice to the elements of the array.
	///
	/// This method is only available for OpenCV types that are Copy, with the exception of bool
	/// because bool is handled in a special way on the C++ side.
	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut [T]
	where
		Self: VectorExternCopyNonBool<T>,
	{
		unsafe { slice::from_raw_parts_mut(self.extern_data_mut(), self.len()) }
	}

	#[inline]
	pub fn to_vec(&self) -> Vec<T> {
		T::opencv_vector_to_vec(self)
	}
}

impl<T: VectorElement> Default for Vector<T>
where
	Self: VectorExtern<T>,
{
	#[inline]
	fn default() -> Vector<T> {
		Vector::new()
	}
}

impl<T: VectorElement> From<Vector<T>> for Vec<T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	fn from(from: Vector<T>) -> Self {
		from.to_vec()
	}
}

impl<T: VectorElement> From<Vec<<T as OpenCVType<'_>>::Arg>> for Vector<T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	fn from(from: Vec<<T as OpenCVType<'_>>::Arg>) -> Self {
		Self::from_iter(from)
	}
}

impl<'a, T: VectorElement> FromIterator<<T as OpenCVType<'a>>::Arg> for Vector<T>
where
	Self: VectorExtern<T>,
{
	#[inline]
	fn from_iter<I: IntoIterator<Item = <T as OpenCVType<'a>>::Arg>>(s: I) -> Vector<T> {
		Self::from_iter(s)
	}
}

impl<T: VectorElement> AsRef<[T]> for Vector<T>
where
	Self: VectorExtern<T> + VectorExternCopyNonBool<T>,
{
	#[inline]
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T: VectorElement> Borrow<[T]> for Vector<T>
where
	Self: VectorExtern<T> + VectorExternCopyNonBool<T>,
{
	#[inline]
	fn borrow(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T: VectorElement + fmt::Debug> fmt::Debug for Vector<T>
where
	Self: VectorExtern<T>,
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

impl<T: VectorElement> Drop for Vector<T>
where
	Self: VectorExtern<T>,
{
	fn drop(&mut self) {
		unsafe { self.extern_delete() }
	}
}

unsafe impl<T: Send + VectorElement> Send for Vector<T> where Self: VectorExtern<T> {}

unsafe impl<T: Sync + VectorElement> Sync for Vector<T> where Self: VectorExtern<T> {}

impl<'a, T: VectorElement> Extend<<T as OpenCVType<'a>>::Arg> for Vector<T>
where
	Self: VectorExtern<T>,
{
	fn extend<I: IntoIterator<Item = <T as OpenCVType<'a>>::Arg>>(&mut self, s: I) {
		let s = s.into_iter();
		let (lo, hi) = s.size_hint();
		self.reserve(hi.unwrap_or(lo));
		s.for_each(|elem| {
			self.push(elem);
		});
	}
}

impl<T: VectorElement> Boxed for Vector<T>
where
	Self: VectorExtern<T>,
{
	#[inline]
	unsafe fn from_raw(ptr: *mut c_void) -> Self {
		Self { ptr, _d: PhantomData }
	}

	#[inline]
	fn into_raw(self) -> *mut c_void {
		ManuallyDrop::new(self).ptr
	}

	#[inline]
	fn as_raw(&self) -> *const c_void {
		self.ptr
	}

	#[inline]
	fn as_raw_mut(&mut self) -> *mut c_void {
		self.ptr
	}
}

impl<T: VectorElement> OpenCVType<'_> for Vector<T>
where
	Self: VectorExtern<T>,
{
	type Arg = Self;
	type ExternReceive = *mut c_void;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		Self::from_raw(s)
	}
}

impl<T: VectorElement> OpenCVTypeArg<'_> for Vector<T>
where
	Self: VectorExtern<T>,
{
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl<T: VectorElement> OpenCVTypeExternContainer for Vector<T>
where
	Self: VectorExtern<T>,
{
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

impl<T: VectorElement> OpenCVTypeExternContainerMove for Vector<T>
where
	Self: VectorExtern<T>,
{
	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut {
		self.into_raw()
	}
}

#[inline(always)]
fn vector_index_check(index: size_t, len: size_t) -> Result<()> {
	if index >= len {
		Err(crate::Error::new(
			crate::core::StsOutOfRange,
			format!("Index: {index} out of bounds: 0..{len}"),
		))
	} else {
		Ok(())
	}
}
