use std::ffi::{c_char, c_void, CString};

use crate::templ::receive_string;

use super::{OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer};

fn cstring_new_nofail(bytes: impl Into<Vec<u8>>) -> CString {
	CString::new(bytes).unwrap_or_else(|e| {
		let nul_pos = e.nul_position();
		let mut bytes = e.into_vec();
		bytes.drain(nul_pos..);
		unsafe { CString::from_vec_unchecked(bytes) }
	})
}

impl<'a> OpenCVType<'a> for String {
	type Arg = &'a str;
	type ExternReceive = *mut c_void;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		receive_string(s.cast::<String>())
	}
}

impl OpenCVTypeArg<'_> for String {
	type ExternContainer = CString;

	#[inline]
	fn opencv_into_extern_container(self) -> crate::Result<Self::ExternContainer> {
		CString::new(self).map_err(|e| e.into())
	}

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		cstring_new_nofail(self)
	}
}

impl OpenCVTypeArg<'_> for &str {
	type ExternContainer = CString;

	#[inline]
	fn opencv_into_extern_container(self) -> crate::Result<Self::ExternContainer> {
		CString::new(self).map_err(|e| e.into())
	}

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		cstring_new_nofail(self)
	}
}

impl OpenCVTypeExternContainer for CString {
	type ExternSend = *const c_char;
	type ExternSendMut = *mut c_char;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.as_ptr()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
		unimplemented!("Casting CString::as_ptr() to mut is UB")
	}
}

impl OpenCVType<'_> for Vec<u8> {
	type Arg = Self;
	type ExternReceive = *mut c_void;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		receive_string(s.cast::<Vec<u8>>())
	}
}

impl OpenCVTypeArg<'_> for Vec<u8> {
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl OpenCVTypeExternContainer for Vec<u8> {
	type ExternSend = *const u8;
	type ExternSendMut = *mut u8;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.as_ptr()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
		self.as_mut_ptr()
	}
}
