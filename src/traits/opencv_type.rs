use std::{
	ffi::{c_void, CString},
	os::raw::c_char,
};

use crate::Result;

pub trait OpenCVType<'a>: Sized {
	type Owned: OpenCVType<'a>;
	type Arg: OpenCVType<'a>;
	type ExternReceive; // return type when this type is returned from the C++ function
	type ExternContainer: OpenCVTypeExternContainer;

	fn opencv_into_extern_container(self) -> Result<Self::ExternContainer>;
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer;
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self::Owned;
}

pub trait OpenCVTypeExternContainer: Sized {
	type ExternSend;
	type ExternSendMut;

	fn opencv_to_extern(&self) -> Self::ExternSend;
	fn opencv_to_extern_mut(&mut self) -> Self::ExternSendMut;
}

#[macro_export]
macro_rules! opencv_type_copy {
	($($type: ty),+ $(,)?) => {
		$(
			impl $crate::traits::OpenCVType<'_> for $type {
				type Owned = Self;
				type Arg = Self;
				type ExternReceive = Self;
				type ExternContainer = Self;

				#[inline] fn opencv_into_extern_container(self) -> $crate::Result<Self> { Ok(self) }
				#[inline] fn opencv_into_extern_container_nofail(self) -> Self { self }
				#[inline] unsafe fn opencv_from_extern(s: Self) -> Self { s }
			}

			impl $crate::traits::OpenCVTypeExternContainer for $type {
				type ExternSend = Self;
				type ExternSendMut = Self;

				#[inline] fn opencv_to_extern(&self) -> Self { *self }
				#[inline] fn opencv_to_extern_mut(&mut self) -> Self { *self }
			}
		)+
	};
}

#[macro_export]
macro_rules! opencv_type_enum {
	($type: ty) => {
		$crate::opencv_type_copy! { $type }
	};
}

#[macro_export]
macro_rules! opencv_type_simple {
	($type: ty) => {
		impl $crate::traits::OpenCVType<'_> for $type {
			type Owned = Self;
			type Arg = Self;
			type ExternReceive = Self;
			type ExternContainer = Self;

			#[inline] fn opencv_into_extern_container(self) -> $crate::Result<Self> { Ok(self) }
			#[inline] fn opencv_into_extern_container_nofail(self) -> Self { self }
			#[inline] unsafe fn opencv_from_extern(s: Self) -> Self { s }
		}

		impl $crate::traits::OpenCVTypeExternContainer for $type {
			type ExternSend = *const Self;
			type ExternSendMut = *mut Self;

			#[inline] fn opencv_to_extern(&self) -> Self::ExternSend { self }
			#[inline] fn opencv_to_extern_mut(&mut self) -> Self::ExternSendMut { self }
		}
	};
}

pub fn cstring_new_nofail(bytes: impl Into<Vec<u8>>) -> CString {
	match CString::new(bytes) {
		Ok(s) => {
			s
		}
		Err(e) => {
			let nul_pos = e.nul_position();
			let mut bytes = e.into_vec();
			bytes.drain(nul_pos + 1..);
			unsafe { CString::from_vec_unchecked(bytes) }
		}
	}
}

impl<'a> OpenCVType<'a> for String {
	type Owned = String;
	type Arg = &'a str;
	type ExternReceive = *mut c_void;
	type ExternContainer = CString;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self::ExternContainer> {
		CString::new(self).map_err(|e| e.into())
	}

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		cstring_new_nofail(self)
	}

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self::Owned {
		crate::templ::receive_string(s as *mut String)
	}
}

impl OpenCVType<'_> for &str {
	type Owned = String;
	type Arg = Self;
	type ExternReceive = *mut c_void;
	type ExternContainer = CString;

	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self::ExternContainer> {
		CString::new(self).map_err(|e| e.into())
	}

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		cstring_new_nofail(self)
	}

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self::Owned {
		crate::templ::receive_string(s as *mut String)
	}
}

impl OpenCVTypeExternContainer for CString {
	type ExternSend = *const c_char;
	type ExternSendMut = *mut c_char;

	#[inline]
	fn opencv_to_extern(&self) -> Self::ExternSend {
		self.as_ptr()
	}

	#[inline]
	fn opencv_to_extern_mut(&mut self) -> Self::ExternSendMut {
		self.as_ptr() as _  // fixme: use as_mut_ptr() when it's stabilized
	}
}

opencv_type_copy! {
	(),
	bool,
	i8, u8,
	i16, u16,
	i32, u32,
	i64, u64,
	f32, f64,
	isize, usize,
	*const c_void, *mut c_void,
}
