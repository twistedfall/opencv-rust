#![feature(libc)]

extern crate libc;

mod cv;

use libc::c_char;

include!(concat!(env!("OUT_DIR"), "/hub.rs"));
