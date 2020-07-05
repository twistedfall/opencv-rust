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
	#[doc(hidden)]	unsafe fn extern_new<'a>(val: <<T as OpenCVType<'a>>::ExternContainer as OpenCVTypeExternContainer>::ExternSendMut) -> *mut c_void;
}

#[macro_export]
macro_rules! ptr_extern {
	($type: ty, $extern_delete: ident, $extern_inner_as_ptr: ident $(,)?) => {
		impl $crate::manual::core::PtrExtern for $crate::manual::core::Ptr<$type> {
			#[inline(always)]
			unsafe fn extern_delete(&mut self) {
				extern "C" { fn $extern_delete(instance: *mut std::ffi::c_void); }
				$extern_delete(self.as_raw_mut())
			}

			#[inline(always)]
			unsafe fn extern_inner_as_ptr(&self) -> *const std::ffi::c_void {
				extern "C" { fn $extern_inner_as_ptr(instance: *const std::ffi::c_void) -> *mut std::ffi::c_void; }
				$extern_inner_as_ptr(self.as_raw())
			}

			#[inline(always)]
			unsafe fn extern_inner_as_ptr_mut(&mut self) -> *mut std::ffi::c_void {
				extern "C" { fn $extern_inner_as_ptr(instance: *mut std::ffi::c_void) -> *mut std::ffi::c_void; }
				$extern_inner_as_ptr(self.as_raw_mut())
			}
		}
	};
}

#[macro_export]
macro_rules! ptr_extern_ctor {
	($type: ty, $extern_new: ident $(,)?) => {
		impl $crate::manual::core::PtrExternCtor<$type> for $crate::manual::core::Ptr<$type> {
			#[inline(always)]
			unsafe fn extern_new<'a>(val: <<$type as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSendMut) -> *mut std::ffi::c_void {
				extern "C" { fn $extern_new<'a>(val: <<$type as $crate::traits::OpenCVType<'a>>::ExternContainer as $crate::traits::OpenCVTypeExternContainer>::ExternSendMut) -> *mut std::ffi::c_void; }
				$extern_new(val)
			}
		}
	};
}
