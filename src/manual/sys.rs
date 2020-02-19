#![allow(non_camel_case_types)]

use std::{
	marker::PhantomData,
	ffi::c_void,
};

use crate::{Error, Result as CrateResult, types::Unit};

#[repr(C)]
pub struct Result<S, O = S> {
	pub error_code: i32,
	pub error_msg: *mut c_void,
	pub result: S,
	_p: PhantomData<O>,
}

impl<S: Into<O>, O> Result<S, O> {
	#[inline]
	pub fn into_result(self) -> CrateResult<O> {
		if self.error_msg.is_null() {
			Ok(self.result.into())
		} else {
			Err(Error::new(self.error_code, crate::templ::receive_string(self.error_msg)))
		}
	}
}

pub type Result_void = Result<Unit, ()>;
