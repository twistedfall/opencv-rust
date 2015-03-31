#![feature(libc)]

extern crate opencv_sys;
extern crate libc;

use std::ffi::CStr;

#[test]
fn test_trivial_call() {
    let ticks = unsafe { opencv_sys::cv_getTickCount() };
    assert!(ticks > 10000);
}

#[test]
fn test_return_string() {
    unsafe {
        let bi = opencv_sys::cv_getBuildInformation();
        let info = std::str::from_utf8(CStr::from_ptr(bi).to_bytes()).unwrap();
        assert!(info.contains("\nGeneral configuration for OpenCV"));
        libc::free(bi as *mut libc::types::common::c95::c_void);
     };
}
