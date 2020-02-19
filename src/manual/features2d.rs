use std::ffi::c_void;

use crate::{
	features2d::ORB,
	Result,
	sys,
	types,
};

impl dyn ORB + '_ {
	pub fn default() -> Result<types::PtrOfORB> {
		extern "C" { fn cv_ORB_create() -> sys::Result<*mut c_void>; }
		unsafe { cv_ORB_create() }.into_result().map(|ptr| types::PtrOfORB { ptr })
	}
}
