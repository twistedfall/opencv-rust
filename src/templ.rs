use std::ffi::{c_char, CStr};
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
		*$name = unsafe { $crate::templ::receive_string($name_via.cast()) };
	};
}

macro_rules! callback_arg {
	($tr_name: ident($($tr_arg_name: ident: $tr_arg_type: ty),*) -> $tr_ret: ty => $tr_userdata_name: ident in $callbacks_name: ident => $callback_name: ident($($fw_arg_name: ident: $fw_arg_type: ty),*) -> $fw_ret: ty) => {
		unsafe extern "C" fn trampoline($($tr_arg_name: $tr_arg_type),*) -> $tr_ret {
			let mut callback: Box<Box<dyn FnMut($($fw_arg_type),*) -> $fw_ret + Send + Sync>> = Box::from_raw($tr_userdata_name.cast());
			let out = callback($($fw_arg_name),*);
			Box::into_raw(callback);
			out
		}

		let $tr_name = if $callback_name.is_some() {
			Some(trampoline as unsafe extern "C" fn($($tr_arg_name: $tr_arg_type),*) -> $tr_ret)
		} else {
			None
		};
	}
}

macro_rules! userdata_arg {
	($userdata_name: ident in $callbacks_name: ident => $callback_name: ident) => {
		let $userdata_name = if let Some(callback) = $callback_name {
			Box::into_raw(Box::new(callback)).cast::<::std::ffi::c_void>()
		} else {
			::std::ptr::null_mut() // fixme, remove previous callback
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
		let $name = $name
			.iter()
			.map(|x| ::std::ffi::CString::new(*x).expect("invalid C string"))
			.collect::<::std::vec::Vec<_>>();
		let $name = $name.iter().map(|x| x.as_ptr()).collect::<::std::vec::Vec<_>>();
	};
}

#[allow(unused_macros)]
macro_rules! string_array_arg_mut {
	($name: ident) => {
		let $name = $name
			.iter()
			.map(|x| ::std::ffi::CString::new(*x).expect("invalid C string"))
			.collect::<::std::vec::Vec<_>>();
		// fixme: Casting to mut below trusts on undefined CString behavior.
		let mut $name = $name.iter().map(|x| x.as_ptr().cast_mut()).collect::<::std::vec::Vec<_>>();
	};
}

macro_rules! smart_ptr_option_arg {
	(unsafe ref $name: ident) => {
		let null = if $name.is_none() {
			Some(unsafe { $crate::core::Ptr::new_null() })
		} else {
			None
		};
		let $name = $name.or(null.as_ref()).expect("Nullability should have been checked");
	};
	(unsafe $name: ident) => {
		let $name = $name.unwrap_or_else(|| unsafe { $crate::core::Ptr::new_null() });
	};
	(ref $name: ident) => {
		let null = if $name.is_none() {
			Some($crate::core::Ptr::new_null())
		} else {
			None
		};
		let $name = $name.or(null.as_ref()).expect("Nullability should have been checked");
	};
	($name: ident) => {
		let $name = $name.unwrap_or_else(|| $crate::core::Ptr::new_null());
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

/// The return type of this function goes into `receive_string::<String>()`
#[inline(always)]
pub unsafe fn ocvrs_create_string(s: *const c_char) -> *mut String {
	let s = CStr::from_ptr(s).to_string_lossy().into_owned();
	Box::into_raw(Box::new(s))
}

/// The return type of this function goes into `receive_string::<Vec<u8>>()`
#[inline(always)]
pub unsafe fn ocvrs_create_byte_string(v: *const u8, len: size_t) -> *mut Vec<u8> {
	let byte_slice = if v.is_null() {
		&[]
	} else {
		slice::from_raw_parts(v, len)
	};
	let v = byte_slice.to_vec();
	Box::into_raw(Box::new(v))
}

/// Used for both regular `String` and byte string (`Vec<u8>`)
#[inline]
pub unsafe fn receive_string<T>(s: *mut T) -> T {
	if s.is_null() {
		panic!("Got null pointer for receive_string()");
	}
	*Box::from_raw(s)
}
