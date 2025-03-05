use std::ffi::c_void;

use crate::traits::{
	OpenCVFromExtern, OpenCVIntoExternContainer, OpenCVType, OpenCVTypeExternContainer, OpenCVTypeExternContainerMove,
};

/// Trait for structures that are created on the C++ side, usually only the raw void pointer is stored to point to the allocated
/// data on the heap.
pub trait Boxed: Sized {
	/// Wrap the specified raw pointer
	/// # Safety
	/// Caller must ensure that the passed pointer is pointing to a valid unowned object data
	unsafe fn from_raw(ptr: *mut c_void) -> Self;

	/// Return the underlying raw pointer while consuming this wrapper.
	///
	/// This will *not* free object referenced by this pointer, so you can use this pointer indefinitely. Be sure
	/// to free it (by e.g. calling `from_raw()` with the same wrapper type) to avoid leaking memory.
	#[must_use]
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

impl<T: Boxed> OpenCVType<'_> for T {
	type Arg = Self;
}

impl<T: Boxed> OpenCVFromExtern for T {
	type ExternReceive = *mut c_void;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		unsafe { Self::from_raw(s) }
	}
}

impl<T: Boxed> OpenCVIntoExternContainer for T {
	type ExternContainer = Self;

	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl<T: Boxed> OpenCVTypeExternContainer for T {
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

impl<T: Boxed> OpenCVTypeExternContainerMove for T {
	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut {
		self.into_raw()
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! opencv_type_boxed {
	($type: ty) => {
		impl $crate::traits::Boxed for $type {
			#[inline]
			unsafe fn from_raw(ptr: $crate::extern_receive!($type)) -> Self {
				Self { ptr }
			}

			#[inline]
			fn into_raw(self) -> $crate::extern_send!(mut $type) {
				::std::mem::ManuallyDrop::new(self).ptr
			}

			#[inline]
			fn as_raw(&self) -> $crate::extern_send!($type) {
				self.ptr
			}

			#[inline]
			fn as_raw_mut(&mut self) -> $crate::extern_send!(mut $type) {
				self.ptr
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! boxed_cast_base {
	($type: ty, $base: ty, $extern_convert: ident $(,)?) => {
		impl ::std::convert::From<$type> for $base {
			#[inline]
			fn from(s: $type) -> Self {
				unsafe { Self::from_raw($crate::sys::$extern_convert(s.into_raw())) }
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! boxed_cast_descendant {
	($type: ty, $descendant: ty, $extern_convert: ident $(,)?) => {
		impl ::std::convert::TryFrom<$type> for $descendant {
			type Error = $crate::Error;

			#[inline]
			fn try_from(s: $type) -> $crate::Result<Self> {
				let ret = unsafe { $crate::sys::$extern_convert(s.into_raw()) };
				if ret.is_null() {
					Err($crate::Error::new(
						$crate::core::StsBadArg,
						format!(
							"Unable to cast base class: {} to: {}",
							stringify!($type),
							stringify!($descendant)
						),
					))
				} else {
					Ok(unsafe { Self::from_raw(ret) })
				}
			}
		}
	};
}
