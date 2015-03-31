#![feature(libc)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/core.rs"));
