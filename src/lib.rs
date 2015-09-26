#![allow(unused_imports,non_snake_case,dead_code)]
#![allow(non_upper_case_globals,overflowing_literals)]
#![allow(non_camel_case_types)]
extern crate libc;

use std::ffi::{ CStr, CString };
use libc::{ c_void, c_char, size_t };

include!(concat!(env!("OUT_DIR"), "/hub.rs"));

pub fn mat() -> ::core::Mat {
    ::core::Mat::new().unwrap()
}

