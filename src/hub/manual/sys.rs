use std::{
    ffi::CStr,
    os::raw::c_char,
};
use std::marker::PhantomData;

use crate::{Error, Result};

#[repr(C)]
pub struct cv_return_value<S, O=S> {
    pub error_code: i32,
    pub error_msg: *mut c_char,
    pub result: S,
    _p: PhantomData<O>,
}

impl<S: Into<O>, O> cv_return_value<S, O> {
    #[inline]
    pub fn into_result(self) -> Result<O> {
        if self.error_msg.is_null() {
            Ok(self.result.into())
        } else {
            Err(Error::new(self.error_code, crate::templ::receive_string_mut(self.error_msg)))
        }
    }
}
