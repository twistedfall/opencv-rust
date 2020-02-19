use std::{ffi::c_void, mem::size_of};

use crate::{
	core::Mat,
	sys::{Result, Result_void},
};

#[test]
fn cv_return_type() {
	assert_eq!(size_of::<Result<i32>>(), 24);
	assert_eq!(size_of::<Result<u8>>(), 24);
	assert_eq!(size_of::<Result_void>(), 16);
}

#[test]
fn mat_layout() {
	assert_eq!(size_of::<Mat>(), size_of::<*mut c_void>());
}
