use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

use crate::platform_types::size_t;

macro_rules! extern_container_arg {
	(nofail mut $name: ident) => {
		let mut $name = $name.opencv_into_extern_container_nofail();
	};
	(nofail $name: ident) => {
		let $name = $name.opencv_into_extern_container_nofail();
	};
	(mut $name: ident) => {
		let mut $name = $name.opencv_into_extern_container()?;
	};
	($name: ident) => {
		let $name = $name.opencv_into_extern_container()?;
	};
}

macro_rules! string_arg_output_send {
	(via $name_via: ident) => {
		let mut $name_via = ::std::ptr::null_mut();
	};
}

macro_rules! string_arg_output_receive {
	($name_via: ident => $name: ident) => {
		*$name = unsafe { $crate::templ::receive_string($name_via as *mut String) };
	};
}

// currently only used in objdetect::decodeQRCode function in OpenCV 3.4
#[allow(unused_macros)]
macro_rules! byte_string_arg_output_receive {
	($name_via: ident => $name: ident) => {
		*$name = unsafe { $crate::templ::receive_byte_string($name_via as *mut Vec<u8>) };
	};
}

macro_rules! callback_arg {
	($tr_name: ident($($tr_arg_name: ident: $tr_arg_type: ty),*) -> $tr_ret: ty => $tr_userdata_name: ident in $callbacks_name: ident => $callback_name: ident($($fw_arg_name: ident: $fw_arg_type: ty),*) -> $fw_ret: ty) => {
		unsafe extern "C" fn trampoline($($tr_arg_name: $tr_arg_type),*) -> $tr_ret {
			let mut callback: Box<Box<dyn FnMut($($fw_arg_type),*) -> $fw_ret + Send + Sync>> = Box::from_raw($tr_userdata_name as _);
			let out = callback($($fw_arg_name),*);
			Box::into_raw(callback);
			out
		}

		let $tr_name = if $callback_name.is_some() {
			Some(trampoline as _)
		} else {
			None
		};
	}
}

macro_rules! userdata_arg {
	($userdata_name: ident in $callbacks_name: ident => $callback_name: ident) => {
		let $userdata_name = if let Some(callback) = $callback_name {
			Box::into_raw(Box::new(callback)) as *mut ::std::ffi::c_void
		} else {
			0 as _ // fixme, remove previous callback
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

macro_rules! string_array_arg {
	($name: ident) => {
		let $name = $name.iter().map(|x| x.as_ptr() as _).collect::<::std::vec::Vec<_>>();
	};
}

#[allow(unused_macros)]
macro_rules! string_array_arg_mut {
	($name: ident) => {
		let mut $name = $name.iter().map(|x| x.as_ptr() as _).collect::<::std::vec::Vec<_>>();
	};
}

macro_rules! return_send {
	(via $name: ident) => {
		let mut $name = ::std::mem::MaybeUninit::uninit();
	};
}

macro_rules! return_receive {
	(unsafe $name_via: ident => $name: ident) => {
		let $name = unsafe { $name_via.assume_init() };
	};
	($name_via: ident => $name: ident) => {
		let $name = $name_via.assume_init();
	};
}

/// The return type of this function goes into `receive_string`
#[no_mangle]
unsafe extern "C" fn ocvrs_create_string(s: *const c_char) -> *mut String {
	let s = CStr::from_ptr(s).to_string_lossy().into_owned();
	Box::into_raw(Box::new(s))
}

/// The return type of this function goes into `receive_byte_string`
#[no_mangle]
unsafe extern "C" fn ocvrs_create_byte_string(v: *const u8, len: size_t) -> *mut Vec<u8> {
	let v = slice::from_raw_parts(v, len).to_vec();
	Box::into_raw(Box::new(v))
}

#[inline]
pub unsafe fn receive_string(s: *mut String) -> String {
	if s.is_null() {
		panic!("Got null pointer for receive_string()");
	}
	*Box::from_raw(s)
}

#[inline]
pub unsafe fn receive_byte_string(s: *mut Vec<u8>) -> Vec<u8> {
	if s.is_null() {
		panic!("Got null pointer for receive_byte_vec()");
	}
	*Box::from_raw(s)
}
