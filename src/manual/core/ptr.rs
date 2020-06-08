use std::{
	ffi::c_void,
	marker::PhantomData,
	mem,
};

pub use ptr_extern::PtrExtern;

use crate::core::Boxed;

#[cfg(not(feature = "opencv-32"))]
mod ptr_f32;
mod ptr_extern;

/// [docs.opencv.org 3.x](https://docs.opencv.org/3.4/d0/de7/structcv_1_1Ptr.html)
/// [docs.opencv.org 4.x](https://en.cppreference.com/w/cpp/memory/shared_ptr)
pub struct Ptr<T: ?Sized> where Self: PtrExtern {
	ptr: *mut c_void,
	_d: PhantomData<T>,
}

impl<T: ?Sized> Ptr<T> where Self: PtrExtern {
	/// Get raw pointer to the inner object
	pub fn inner_as_raw(&self) -> *const c_void {
		unsafe { self.extern_inner_as_ptr() }
	}

	/// Get mutable raw pointer to the inner object
	pub fn inner_as_raw_mut(&mut self) -> *mut c_void {
		unsafe { self.extern_inner_as_ptr_mut() }
	}
}

impl<T: ?Sized> Boxed for Ptr<T> where Self: PtrExtern {
	#[inline]
	unsafe fn from_raw(ptr: *mut c_void) -> Self {
		Self { ptr, _d: PhantomData }
	}

	#[inline]
	fn into_raw(self) -> *mut c_void {
		let out = self.ptr;
		mem::forget(self);
		out
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

impl<T: ?Sized> Drop for Ptr<T> where Self: PtrExtern {
	fn drop(&mut self) {
		unsafe { self.extern_delete() }
	}
}
