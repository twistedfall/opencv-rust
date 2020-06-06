use std::ffi::c_void;

use crate::{
	traits::Boxed,
	features2d::ORB,
	Result,
	sys,
	types,
};

impl dyn ORB + '_ {
	pub fn default() -> Result<types::PtrOfORB> {
		extern "C" { fn cv_ORB_create() -> sys::Result<*mut c_void>; }
		unsafe { cv_ORB_create() }.into_result().map(|ptr| unsafe { types::PtrOfORB::from_raw(ptr) })
	}
}
