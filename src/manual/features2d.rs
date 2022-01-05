use std::ffi::c_void;

use crate::{
	features2d::ORB,
	Result,
	sys,
	traits::Boxed,
	types,
};

impl dyn ORB + '_ {
	pub fn default() -> Result<types::PtrOfORB> {
		extern "C" { fn cv_ORB_create(ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_ORB_create(ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { types::PtrOfORB::from_raw(ptr) })
	}
}
