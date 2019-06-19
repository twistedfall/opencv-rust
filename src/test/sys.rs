use std::{ffi::c_void, mem::size_of};

use crate::{core::Mat, sys::cv_return_value};

#[test]
fn cv_return_type() {
    assert_eq!(size_of::<cv_return_value<i32>>(), 24);
    assert_eq!(size_of::<cv_return_value<u8>>(), 24);
    assert_eq!(size_of::<cv_return_value<crate::types::Unit, ()>>(), 16);
}

#[test]
fn mat_layout() {
    assert_eq!(size_of::<Mat>(), size_of::<*mut c_void>());
}
