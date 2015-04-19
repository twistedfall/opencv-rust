use std::mem::transmute;
use libc::c_uint;

extern "C" {
    pub fn cv_core_Mat_create() -> *mut i8;
    pub fn cv_core_Mat_create_III(width:c_uint, height:c_uint, typ:c_uint) -> *mut i8;
    pub fn cv_core_Mat_size(mat:*mut i8) -> ::sys::core::Size2i;
}

#[allow(non_camel_case_types)]
pub enum ChannelDepth {
    CV_8U=0, CV_8S=1, CV_16U=2, CV_16S=3, CV_32S=4,
    CV_32F=5, CV_64F=6, CV_USRTYPE1=7
}

pub type MatType = u32;

#[repr(C)]#[allow(dead_code)]
pub struct Mat {
    ptr: *mut i8
}

impl Mat {
    pub fn make_type(depth:ChannelDepth, count:u32) -> MatType {
        depth as u32 | ((count-1) << 3)
    }
    pub fn new() -> Mat {
        unsafe { return transmute(cv_core_Mat_create()); }
    }
    pub fn for_dims(width:u32, height:u32, typ:u32) -> Mat {
        unsafe { return transmute(cv_core_Mat_create_III(width, height, typ)); }
    }

    pub fn size(&self) -> ::sys::core::Size2i {
        unsafe { return transmute(cv_core_Mat_size(self.ptr)); }
    }
}
