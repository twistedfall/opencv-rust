use std::ffi::c_void;

pub trait Boxed: Sized {
	/// Wrap the specified raw pointer
	unsafe fn from_raw(ptr: *mut c_void) -> Self;

	/// Return an the underlying raw pointer while consuming this wrapper.
	///
	/// This will *not* free object referenced by this pointer so you can use this pointer indefinitely. Be sure
	/// to free it (by e.g. calling `from_raw()` with the same wrapper type) to avoid leaking memory.
	fn into_raw(self) -> *mut c_void;

	/// Return the underlying raw pointer.
	///
	/// You can use this pointer as long as the original object lives. Be careful not to double-free it.
	fn as_raw(&self) -> *const c_void;

	/// Return the underlying mutable raw pointer
	///
	/// You can use this pointer as long as the original object lives. Be careful not to double-free it. Note
	/// that ownership is still retained in the original object. Use `into_raw()` if you want to transfer
	/// ownership to another wrapper.
	fn as_raw_mut(&mut self) -> *mut c_void;
}

#[macro_export]
macro_rules! boxed_ptr {
	($type: ty) => {
		impl $crate::traits::Boxed for $type {
			#[inline]
			unsafe fn from_raw(ptr: *mut std::ffi::c_void) -> Self {
				Self { ptr }
			}

			#[inline]
			fn into_raw(self) -> *mut std::ffi::c_void {
				let out = self.ptr;
				std::mem::forget(self);
				out
			}

			#[inline]
			fn as_raw(&self) -> *const std::ffi::c_void {
				self.ptr
			}

			#[inline]
			fn as_raw_mut(&mut self) -> *mut std::ffi::c_void {
				self.ptr
			}
		}
	};
}
