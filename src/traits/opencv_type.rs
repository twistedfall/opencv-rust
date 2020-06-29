use std::{
	ffi::{c_void, CString},
	os::raw::c_char,
};

use crate::Result;

#[doc(hidden)]
/// Common trait of all OpenCV related types, helps with generic handling of FFI marshalling
///
/// This trait is somewhat unnecessary complex because of the need of handling String, we need to be able to
/// pass &str as argument to functions that await String and do necessary conversion through CString.
pub trait OpenCVType<'a>: Sized {
	#[doc(hidden)]
	/// Owned version of the type, e.g. String for &str, for most other types it's Self
	type Owned: OpenCVType<'a>;
	#[doc(hidden)]
	/// Type when passed as argument to function, e.g. &str for String, for most other types it's either Self
	/// or &Self
	type Arg: OpenCVType<'a>;
	#[doc(hidden)]
	/// Return type when this type is returned over the FFI boundary from the C++ function, Self for simple
	/// types, *mut c_void for complex ones
	type ExternReceive;
	#[doc(hidden)]
	/// Container to help marshall type over FFI boundary, e.g. CString for String or &str, for most other
	/// types it's Self
	type ExternContainer: OpenCVTypeExternContainer;

	#[doc(hidden)]
	/// Convert Self into external container with possible error result, it shouldn't panic
	fn opencv_into_extern_container(self) -> Result<Self::ExternContainer>;
	#[doc(hidden)]
	/// Convert Self into external container in the nofail context, this can panic
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer;
	#[doc(hidden)]
	/// Construct the new Self::Owned from the data received from C++ function
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self::Owned;
}

#[doc(hidden)]
/// Common trait for the type that is used to help marshall OpenCV related type over the FFI boundary
pub trait OpenCVTypeExternContainer: Sized {
	#[doc(hidden)]
	/// Type when constant Self is sent to C++ function, usually it's Self for simple types or *const c_void
	/// for complex ones
	type ExternSend;
	#[doc(hidden)]
	/// Type when mutable Self is sent to C++ function, usually it's Self for simple types or *mut c_void for
	/// complex ones
	type ExternSendMut;

	#[doc(hidden)]
	fn opencv_as_extern(&self) -> Self::ExternSend;
	#[doc(hidden)]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut;
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

				#[inline] fn opencv_as_extern(&self) -> Self { *self }
				#[inline] fn opencv_as_extern_mut(&mut self) -> Self { *self }
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

			#[inline] fn opencv_as_extern(&self) -> Self::ExternSend { self }
			#[inline] fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut { self }
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
			bytes.drain(nul_pos..);
			unsafe { CString::from_vec_unchecked(bytes) }
		}
	}
}

impl<'a> OpenCVType<'a> for String {
	type Owned = Self;
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
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.as_ptr()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
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
