use std::{
    ffi::CStr,
    os::raw::c_char,
};

pub use vector::*;

mod vector;

macro_rules! string_arg {
    (mut $name: ident) => {
        let $name = ::std::ffi::CString::new($name).map_err(|e| $crate::Error::new($crate::core::StsBadArg, format!("{}: {}", stringify!($name), e)))?;
    };
    ($name: ident) => {
        let $name = ::std::ffi::CString::new($name).map_err(|e| $crate::Error::new($crate::core::StsBadArg, format!("{}: {}", stringify!($name), e)))?;
    };
}

macro_rules! string_arg_output_send {
    (via $name_via: ident) => {
        let mut $name_via = ::std::ptr::null_mut();
    };
}

macro_rules! string_arg_output_receive {
    ($name_via: ident => $name: ident) => {
        *$name = $crate::templ::receive_string_mut($name_via);
    };
}

macro_rules! callback_arg {
    ($callback_name: ident($($tr_arg_name: ident: $tr_arg_type: ty),*) via $userdata_name: ident => ($($fw_arg_name: ident: $fw_arg_type: ty),*)) => {
        static callbacks: ::once_cell::sync::Lazy<::std::sync::Mutex<::slab::Slab<Box<dyn FnMut($($fw_arg_type),*) + Send + Sync>>>> = ::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new(::slab::Slab::with_capacity(1)));

        extern "C" fn trampoline($($tr_arg_name: $tr_arg_type),*) {
            if let Some(callback) = callbacks.lock().unwrap().get_mut($userdata_name as _) {
                callback($($fw_arg_name),*);
            }
        }

        let ($userdata_name, $callback_name) = if let Some(callback) = $callback_name {
            (callbacks.lock().unwrap().insert(callback) as _, Some(trampoline as _))
        } else {
            (0 as _, None) // fixme, remove previous callback
        };
    };
}

macro_rules! input_array_arg {
    ($name: ident) => {
        let $name = $name.input_array()?;
    };
}

macro_rules! output_array_arg {
    ($name: ident) => {
        let $name = $name.output_array()?;
    };
}

macro_rules! input_output_array_arg {
    ($name: ident) => {
        let $name = $name.input_output_array()?;
    };
}

#[inline]
pub fn receive_string(s: *const c_char) -> String {
    let out = unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned();
    unsafe { ::libc::free(s as _); }
    out
}

#[inline]
pub fn receive_string_mut(s: *mut c_char) -> String {
    let out = unsafe { CStr::from_ptr(s as _) }.to_string_lossy().into_owned();
    unsafe { ::libc::free(s as _); }
    out
}
