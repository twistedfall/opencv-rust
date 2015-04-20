#![allow(unused_imports,non_snake_case,dead_code)]

extern crate libc;
pub use sys::core;
pub use sys::highgui;

include!(concat!(env!("OUT_DIR"), "/hub.rs"));
