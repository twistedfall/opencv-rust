use std::ffi::c_void;

use crate::traits::{OpenCVType, OpenCVTypeExternContainer};

#[doc(hidden)]
pub trait PtrExtern {
	#[doc(hidden)]	unsafe fn extern_delete(&mut self);
	#[doc(hidden)]	unsafe fn extern_inner_as_ptr(&self) -> *const c_void;
	#[doc(hidden)]	unsafe fn extern_inner_as_ptr_mut(&mut self) -> *mut c_void;
}

#[doc(hidden)]
pub trait PtrExternCtor<T: for<'a> OpenCVType<'a>>: Sized {
	#[doc(hidden)]	unsafe fn extern_new(val: <<T as OpenCVType>::ExternContainer as OpenCVTypeExternContainer>::ExternSendMut) -> *mut c_void;
}

#[macro_export]
macro_rules! ptr_extern {
	($type: ty, $extern_delete: ident, $extern_inner_as_ptr: ident, $extern_inner_as_ptr_mut: ident $(,)?) => {
		extern "C" {
			fn $extern_delete(instance: *mut std::ffi::c_void);
			fn $extern_inner_as_ptr(instance: *const std::ffi::c_void) -> *const std::ffi::c_void;
			fn $extern_inner_as_ptr_mut(instance: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
		}

		impl $crate::manual::core::PtrExtern for $crate::manual::core::Ptr<$type> {
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
	($type: ty, $extern_new: ident $(,)?) => {
		extern "C" { fn $extern_new(val: <<$type as $crate::traits::OpenCVType>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSendMut) -> *mut std::ffi::c_void; }

		impl $crate::manual::core::PtrExternCtor<$type> for $crate::manual::core::Ptr<$type> {
			#[inline]
			unsafe fn extern_new(val: <<$type as $crate::traits::OpenCVType>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSendMut) -> *mut std::ffi::c_void {
				$extern_new(val)
			}
		}
	};
}

#[macro_export]
macro_rules! ptr_cast_base {
	($type: ty, $base: ty, $extern_convert: ident $(,)?) => {
		extern "C" { fn $extern_convert(val: *mut std::ffi::c_void) -> *mut std::ffi::c_void; }

		impl ::std::convert::From<$type> for $base {
			#[inline]
			fn from(s: $type) -> Self {
				unsafe { Self::from_raw($extern_convert(s.into_raw())) }
			}
		}
	};
}
