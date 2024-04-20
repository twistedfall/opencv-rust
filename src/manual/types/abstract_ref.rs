use std::ffi::c_void;
use std::marker::PhantomData;

use crate::traits::Boxed;

pub struct AbstractRefMut<'r, T: ?Sized> {
	ptr: *mut c_void,
	_d: PhantomData<&'r mut T>,
}

impl<T: ?Sized> Boxed for AbstractRefMut<'_, T> {
	#[inline]
	unsafe fn from_raw(ptr: *mut c_void) -> Self {
		Self { ptr, _d: PhantomData }
	}

	#[inline]
	fn into_raw(self) -> *mut c_void {
		self.ptr
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
