use std::ffi::c_void;

#[doc(hidden)]
pub trait PtrExtern {
	unsafe fn extern_delete(&mut self);
	unsafe fn extern_inner_as_ptr(&self) -> *const c_void;
	unsafe fn extern_inner_as_ptr_mut(&mut self) -> *mut c_void;
}

#[macro_export]
macro_rules! ptr_extern {
	($type: ty, $extern_delete: ident, $extern_inner_as_ptr: ident) => {
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
