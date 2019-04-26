use std::{
    ffi::CStr,
    os::raw::c_char,
};

macro_rules! string_arg {
    (mut $name: ident) => {
        let $name = ::std::ffi::CString::new($name).map_err(|e| $crate::Error::new($crate::core::StsBadArg, format!("{}: {}", stringify!($name), e)))?;
    };
    ($name: ident) => {
        let $name = ::std::ffi::CString::new($name).map_err(|e| $crate::Error::new($crate::core::StsBadArg, format!("{}: {}", stringify!($name), e)))?;
    };
}

#[inline]
pub fn receive_string(s: *const c_char) -> String {
    let out = String::from_utf8_lossy(unsafe { CStr::from_ptr(s as _) }.to_bytes()).into_owned();
    unsafe { ::libc::free(s as _); }
    out
}

#[inline]
pub fn receive_string_mut(s: *mut c_char) -> String {
    let out = String::from_utf8_lossy(unsafe { CStr::from_ptr(s as _) }.to_bytes()).into_owned();
    unsafe { ::libc::free(s as _); }
    out
}
