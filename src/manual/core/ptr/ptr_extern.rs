use std::ffi::c_void;

use crate::traits::{OpenCVType, OpenCVTypeExternContainerMove};
use crate::{extern_receive, extern_send};

#[doc(hidden)]
pub trait PtrExtern {
	#[doc(hidden)]
	unsafe fn extern_delete(&mut self);
	#[doc(hidden)]
	unsafe fn extern_inner_as_ptr(&self) -> *const c_void;
	#[doc(hidden)]
	unsafe fn extern_inner_as_ptr_mut(&mut self) -> *mut c_void;
}

#[doc(hidden)]
pub trait PtrExternCtor<T: OpenCVTypeExternContainerMove> {
	#[doc(hidden)]
	unsafe fn extern_new<'a>(val: extern_send!(mut T)) -> extern_receive!(Self: 'a)
	where
		Self: OpenCVType<'a>;
}

#[doc(hidden)]
#[macro_export]
macro_rules! ptr_extern {
	($type: ty, $extern_delete: ident, $extern_inner_as_ptr: ident, $extern_inner_as_ptr_mut: ident $(,)?) => {
		impl $crate::core::PtrExtern for $crate::core::Ptr<$type> {
			#[inline]
			unsafe fn extern_delete(&mut self) {
				$crate::sys::$extern_delete(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_inner_as_ptr(&self) -> *const std::ffi::c_void {
				$crate::sys::$extern_inner_as_ptr(self.as_raw()) as *const _
			}

			#[inline]
			unsafe fn extern_inner_as_ptr_mut(&mut self) -> *mut std::ffi::c_void {
				$crate::sys::$extern_inner_as_ptr_mut(self.as_raw_mut()) as *mut _
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! ptr_extern_ctor {
	($type: ty, $extern_new: ident) => {
		impl $crate::core::PtrExternCtor<$type> for $crate::core::Ptr<$type> {
			// `clippy::needless_lifetimes` needed because of the support for Rust 1.59
			#[allow(clippy::needless_lifetimes)]
			#[inline]
			unsafe fn extern_new<'a>(val: extern_container_send!(mut $type: 'a)) -> extern_receive!(Self: 'a) {
				$crate::sys::$extern_new(val)
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! ptr_cast_base {
	($type: ty, $base: ty, $extern_convert: ident) => {
		impl ::std::convert::From<$type> for $base {
			#[inline]
			fn from(s: $type) -> Self {
				unsafe { Self::from_raw($crate::sys::$extern_convert(s.into_raw())) }
			}
		}
	};
}
