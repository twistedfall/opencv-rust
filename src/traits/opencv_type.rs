use std::ffi::c_void;

pub use enumeration::OpenCVBitfieldEnum;

use crate::Result;

mod enumeration;
mod string;

/// Common trait of all OpenCV related types, helps with generic handling of FFI marshalling
///
/// This trait is somewhat unnecessary complex because of the need of handling String, we need to be able to
/// pass &str as argument to functions that expect String and do necessary conversion through CString.
#[doc(hidden)]
pub trait OpenCVType<'a>: OpenCVIntoExternContainer + OpenCVFromExtern {
	/// Type when passed as argument to function, e.g. &str for String, for most other types it's Self
	#[doc(hidden)]
	type Arg: OpenCVIntoExternContainer;
}

/// Trail to create OpenCV type from the data received from C++ side
#[doc(hidden)]
pub trait OpenCVFromExtern {
	/// Return type when this type is returned over the FFI boundary from the C++ function, Self for simple
	/// types, *mut c_void for complex ones
	#[doc(hidden)]
	type ExternReceive;

	/// Construct the new Self from the data received from C++ function
	#[doc(hidden)]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self;
}

/// Common trait for types that can be used as argument that will be converted into a OpenCV type.
///
/// Mostly necessary to be able to pass `&str` argument for types that otherwise have `String` Rust representation.
#[doc(hidden)]
pub trait OpenCVIntoExternContainer: Sized {
	/// Container to help marshall type over FFI boundary, e.g. CString for String or &str, for most other
	/// types it's Self
	#[doc(hidden)]
	type ExternContainer: OpenCVTypeExternContainer;

	/// Convert Self into external container with possible error result, it shouldn't panic
	#[doc(hidden)]
	#[inline]
	fn opencv_into_extern_container(self) -> Result<Self::ExternContainer> {
		Ok(self.opencv_into_extern_container_nofail())
	}

	/// Convert Self into external container in the nofail context, this can panic
	#[doc(hidden)]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer;
}

/// Common trait for the type that is used to help marshall OpenCV related type over the FFI boundary
#[doc(hidden)]
pub trait OpenCVTypeExternContainer {
	/// Type when constant Self is sent to C++ function, usually it's Self for simple types or *const c_void
	/// for complex ones
	#[doc(hidden)]
	type ExternSend;
	/// Type when mutable Self is sent to C++ function, usually it's Self for simple types or *mut c_void for
	/// complex ones
	#[doc(hidden)]
	type ExternSendMut;

	#[doc(hidden)]
	fn opencv_as_extern(&self) -> Self::ExternSend;

	#[doc(hidden)]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut;
}

/// Common trait for those `OpenCVTypeExternContainer`s that can be moved. Currently used for moving a boxed class behind a `Ptr`.
#[doc(hidden)]
pub trait OpenCVTypeExternContainerMove: OpenCVTypeExternContainer {
	#[doc(hidden)]
	fn opencv_into_extern(self) -> Self::ExternSendMut;
}

/// Extern type to receive the OpenCVType over FFI boundary, used to improve readability
#[doc(hidden)]
#[macro_export]
macro_rules! extern_receive {
	($typ: ty) => {
		<$typ as $crate::traits::OpenCVFromExtern>::ExternReceive
	};
}

/// Extern type to send the OpenCVTypeExternContainer over FFI boundary, used to improve readability
#[doc(hidden)]
#[macro_export]
macro_rules! extern_send {
	(mut $typ: ty) => {
		<$typ as $crate::traits::OpenCVTypeExternContainer>::ExternSendMut
	};
	($typ: ty) => {
		<$typ as $crate::traits::OpenCVTypeExternContainer>::ExternSend
	};
}

/// Extern type to send the owned OpenCVType over FFI boundary, used to improve readability
#[doc(hidden)]
#[macro_export]
macro_rules! extern_container_send {
	(mut $typ: ty) => {
		$crate::extern_send!(mut <$typ as $crate::traits::OpenCVIntoExternContainer>::ExternContainer)
	};
	($typ: ty) => {
		$crate::extern_send!(<$typ as $crate::traits::OpenCVIntoExternContainer>::ExternContainer)
	};
}

/// Extern type to send the ::Arg of a OpenCVType over FFI boundary, used to improve readability
#[doc(hidden)]
#[macro_export]
macro_rules! extern_arg_send {
	(mut $typ: ty: $lt: lifetime) => {
		$crate::extern_container_send!(mut <$typ as $crate::traits::OpenCVType<$lt>>::Arg)
	};
	($typ: ty: $lt: lifetime) => {
		$crate::extern_container_send!(<$typ as $crate::traits::OpenCVType<$lt>>::Arg)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! opencv_type_copy {
	($($type: ty),+ $(,)?) => {
		$(
			impl $crate::traits::OpenCVType<'_> for $type {
				type Arg = Self;
			}

			impl $crate::traits::OpenCVFromExtern for $type {
				type ExternReceive = Self;

				#[inline] unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self { s }
			}

			impl $crate::traits::OpenCVIntoExternContainer for $type {
				type ExternContainer = Self;

				#[inline] fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer { self }
			}

			impl $crate::traits::OpenCVTypeExternContainer for $type {
				type ExternSend = Self;
				type ExternSendMut = Self;

				#[inline] fn opencv_as_extern(&self) -> Self::ExternSend { *self }
				#[inline] fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut { *self }
			}

			impl $crate::traits::OpenCVTypeExternContainerMove for $type {
				#[inline] fn opencv_into_extern(self) -> Self { self }
			}
		)+
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! opencv_type_simple {
	($type: ty) => {
		impl $crate::traits::OpenCVType<'_> for $type {
			type Arg = Self;
		}

		impl $crate::traits::OpenCVFromExtern for $type {
			type ExternReceive = Self;

			#[inline]
			unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
				s
			}
		}

		impl $crate::traits::OpenCVIntoExternContainer for $type {
			type ExternContainer = Self;

			#[inline]
			fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
				self
			}
		}

		impl $crate::traits::OpenCVTypeExternContainer for $type {
			type ExternSend = *const Self;
			type ExternSendMut = *mut Self;

			#[inline]
			fn opencv_as_extern(&self) -> Self::ExternSend {
				self
			}
			#[inline]
			fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
				self
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! opencv_type_simple_generic {
	($type: ident<$trait: ident $( + $more: ident)*>) => {
		impl<T: $trait$( + $more)*> $crate::traits::OpenCVType<'_> for $type<T> {
			type Arg = Self;
		}

		impl<T: $trait$( + $more)*> $crate::traits::OpenCVFromExtern for $type<T> {
			type ExternReceive = Self;

			#[inline] unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self { s }
		}

		impl<T: $trait$( + $more)*> $crate::traits::OpenCVIntoExternContainer for $type<T> {
			type ExternContainer = Self;

			#[inline] fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer { self }
		}

		impl<T: $trait$( + $more)*> $crate::traits::OpenCVTypeExternContainer for $type<T> {
			type ExternSend = *const Self;
			type ExternSendMut = *mut Self;

			#[inline] fn opencv_as_extern(&self) -> Self::ExternSend { self }
			#[inline] fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut { self }
		}
	};
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
