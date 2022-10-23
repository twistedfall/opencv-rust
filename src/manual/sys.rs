#![allow(non_camel_case_types)]

// note to self, you can't use union here to store both result and error code because C++ side doesn't
// support non-POD types as union fields

use std::{ffi::c_void, marker::PhantomData, mem::MaybeUninit};

use crate::{types::Unit, Error, Result as CrateResult};

#[repr(C)]
pub struct Result<S, O = S> {
	pub error_code: i32,
	pub error_msg: *mut c_void,
	pub result: MaybeUninit<S>,
	_p: PhantomData<O>,
}

impl<S: Into<O>, O> Result<S, O> {
	#[inline]
	pub fn into_result(self) -> CrateResult<O> {
		if self.error_msg.is_null() {
			Ok(unsafe { self.result.assume_init() }.into())
		} else {
			Err(Error::new(self.error_code, unsafe {
				crate::templ::receive_string(self.error_msg as *mut String)
			}))
		}
	}
}

pub type Result_void = Result<Unit, ()>;
