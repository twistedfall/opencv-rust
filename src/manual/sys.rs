// note to self, you can't use union here to store both result and error code because C++ side doesn't
// support non-POD types as union fields

use std::ffi::c_void;
use std::mem::MaybeUninit;

use crate::templ::receive_string;
use crate::Error;

#[repr(C)]
pub struct Result<T> {
	pub error_code: i32,
	pub error_msg: *mut c_void,
	pub result: MaybeUninit<T>,
}

impl<T> Result<T> {
	#[inline]
	pub fn into_result(self) -> crate::Result<T> {
		if self.error_msg.is_null() {
			Ok(unsafe { self.result.assume_init() })
		} else {
			let error_msg = if self.error_msg.is_null() {
				"Unable to receive error message".to_string()
			} else {
				unsafe { receive_string(self.error_msg.cast::<String>()) }
			};
			Err(Error::new(self.error_code, error_msg))
		}
	}
}
