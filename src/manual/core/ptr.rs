use core::ffi::c_void;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

pub use ptr_extern::{PtrExtern, PtrExternCtor};

use crate::traits::{Boxed, OpenCVTypeExternContainerMove};

mod ptr_extern;
mod ptr_f32;

/// This is similar to Rust [`std::sync::Arc`], but handled by the C++. Some OpenCV functions insist on accepting [`Ptr`] instead
/// of a heap allocated object, so we need to satisfy those.
///
/// [docs.opencv.org 4.x](https://docs.opencv.org/4.13.0/dc/d84/group__core__basic.html#ga524e5e94ebf48db273a71ab275eaf5b5)
/// [docs.opencv.org 5.x](https://docs.opencv.org/5.0/main_modules/core_basic.html#ptr)
pub struct Ptr<T: ?Sized>
where
	Self: PtrExtern,
{
	ptr: *mut c_void,
	_d: PhantomData<T>,
}

impl<T: ?Sized> Ptr<T>
where
	Self: PtrExtern,
{
	/// Create a new [`Ptr`] from the object
	pub fn new(val: T) -> Self
	where
		T: OpenCVTypeExternContainerMove + Sized,
		Self: PtrExternCtor<T>,
	{
		unsafe { Self::from_raw(Self::extern_new(val.opencv_into_extern())) }
	}

	/// Create a new [`Ptr`] which points to `NULL`
	///
	/// Not generally useful, mostly for internal use.
	///
	/// # Safety
	/// Should only be used as an argument to OpenCV functions that accept [`Ptr`] to `NULL`.
	pub unsafe fn new_null() -> Self {
		unsafe { Self::from_raw(Self::extern_new_null()) }
	}

	/// Get raw pointer to the inner object
	pub fn inner_as_raw(&self) -> *const c_void {
		unsafe { self.extern_inner_as_ptr() }
	}

	/// Get mutable raw pointer to the inner object
	pub fn inner_as_raw_mut(&mut self) -> *mut c_void {
		unsafe { self.extern_inner_as_ptr_mut() }
	}
}

impl<T: ?Sized> Boxed for Ptr<T>
where
	Self: PtrExtern,
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

impl<T: ?Sized> Drop for Ptr<T>
where
	Self: PtrExtern,
{
	fn drop(&mut self) {
		unsafe { self.extern_delete() }
	}
}

unsafe impl<T: Send> Send for Ptr<T> where Self: PtrExtern {}

unsafe impl<T: Sync> Sync for Ptr<T> where Self: PtrExtern {}
