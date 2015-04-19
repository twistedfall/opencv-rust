extern crate opencv_sys;

use opencv_sys::*;

use std::ffi::CString;

#[allow(dead_code)]
fn main() {
    let mat = core::Mat::for_dims(400,300, core::Mat::make_type(core::ChannelDepth::CV_8U, 3));
    let size = mat.size();
    println!("{:?}", size);
}
