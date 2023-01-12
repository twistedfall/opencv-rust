use std::ffi::c_void;

use crate::traits::{OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer};
use crate::{extern_container_send, extern_receive, extern_send};

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
pub trait PtrExternCtor<T: for<'a> OpenCVTypeArg<'a>> {
	#[doc(hidden)]
	unsafe fn extern_new<'a>(val: extern_container_send!(mut T: 'a)) -> extern_receive!(Self: 'a)
	where
		Self: OpenCVType<'a>;
}

#[macro_export]
macro_rules! ptr_extern {
	($type: ty, $extern_delete: ident, $extern_inner_as_ptr: ident, $extern_inner_as_ptr_mut: ident $(,)?) => {
		extern "C" {
	fn $extern_delete(instance: extern_send!(mut $crate::core::Ptr<$type>));
			fn $extern_inner_as_ptr(instance: extern_send!($crate::core::Ptr<$type>)) -> *const std::ffi::c_void;
	fn $extern_inner_as_ptr_mut(instance: extern_send!(mut $crate::core::Ptr<$type>)) -> *mut std::ffi::c_void;
		}

		impl $crate::core::PtrExtern for $crate::core::Ptr<$type> {
			#[inline]
			unsafe fn extern_delete(&mut self) {
				$extern_delete(self.as_raw_mut())
			}

			#[inline]
			unsafe fn extern_inner_as_ptr(&self) -> *const std::ffi::c_void {
				$extern_inner_as_ptr(self.as_raw())
			}

			#[inline]
			unsafe fn extern_inner_as_ptr_mut(&mut self) -> *mut std::ffi::c_void {
				$extern_inner_as_ptr_mut(self.as_raw_mut())
			}
		}
	};
}

#[macro_export]
macro_rules! ptr_extern_ctor {
	($type: ty, $extern_new: ident) => {
		extern "C" {
	fn $extern_new<'a>(val: extern_container_send!(mut $type: 'a)) -> extern_receive!($crate::core::Ptr<$type>: 'a);
		}

		impl $crate::core::PtrExternCtor<$type> for $crate::core::Ptr<$type> {
			#[inline]
			unsafe fn extern_new<'a>(val: extern_container_send!(mut $type: 'a)) -> extern_receive!(Self: 'a) {
				$extern_new(val)
			}
		}
	};
}

#[macro_export]
macro_rules! ptr_cast_base {
	($type: ty, $base: ty, $extern_convert: ident) => {
		extern "C" {
	fn $extern_convert<'a>(val: extern_send!(mut $type)) -> extern_receive!($base: 'a);
		}

		impl ::std::convert::From<$type> for $base {
			#[inline]
			fn from(s: $type) -> Self {
				unsafe { Self::from_raw($extern_convert(s.into_raw())) }
			}
		}
	};
}
