use crate::traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer, OpenCVTypeExternContainerMove};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

/// Wrapper for C++ [std::tupe](https://en.cppreference.com/w/cpp/utility/tuple) and
/// [std::pair](https://en.cppreference.com/w/cpp/utility/pair)
pub struct Tuple<T>
where
	Self: TupleExtern,
{
	ptr: *mut c_void,
	_d: PhantomData<T>,
}

impl<T> Boxed for Tuple<T>
where
	Self: TupleExtern,
{
	#[inline]
	unsafe fn from_raw(ptr: *mut c_void) -> Self {
		Self { ptr, _d: PhantomData }
	}

	#[inline]
	fn into_raw(self) -> *mut c_void {
		ManuallyDrop::new(self).ptr
	}

	#[inline]
	fn as_raw(&self) -> *const c_void {
		self.ptr
	}

	#[inline]
	fn as_raw_mut(&mut self) -> *mut c_void {
		self.ptr
	}
}

impl<T> OpenCVType<'_> for Tuple<T>
where
	Self: TupleExtern,
{
	type Arg = Self;
	type ExternReceive = *mut c_void;

	#[inline]
	unsafe fn opencv_from_extern(s: Self::ExternReceive) -> Self {
		Self::from_raw(s)
	}
}

impl<T> OpenCVTypeArg<'_> for Tuple<T>
where
	Tuple<T>: TupleExtern,
{
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl<T> OpenCVTypeExternContainer for Tuple<T>
where
	Tuple<T>: TupleExtern,
{
	type ExternSend = *const c_void;
	type ExternSendMut = *mut c_void;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend {
		self.as_raw()
	}

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut {
		self.as_raw_mut()
	}
}

impl<T> OpenCVTypeExternContainerMove for Tuple<T>
where
	Tuple<T>: TupleExtern,
{
	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut {
		self.into_raw()
	}
}

impl<T> Drop for Tuple<T>
where
	Tuple<T>: TupleExtern,
{
	fn drop(&mut self) {
		unsafe { self.extern_delete() }
	}
}

pub trait TupleExtern {
	#[doc(hidden)]
	unsafe fn extern_delete(&mut self);
}

#[doc(hidden)]
#[macro_export]
macro_rules! tuple_extern {
	(
		$type: ty,
		$extern_new: ident, $extern_delete: ident,
		$($num: tt = $arg: ident: $element_type: ty, $element_get: ident via $extern_element_get: ident),+
	) => {
		impl $crate::core::TupleExtern for $crate::core::Tuple<$type> {
			#[inline]
			unsafe fn extern_delete(&mut self) {
				$crate::sys::$extern_delete(self.as_raw_mut())
			}
		}

		impl $crate::core::Tuple<$type> {
			$(
			#[inline]
			pub fn $element_get(&self) -> $element_type {
				return_send!(via ocvrs_return);
				unsafe {
					$crate::sys::$extern_element_get(self.as_raw(), ocvrs_return.as_mut_ptr());
					return_receive!(ocvrs_return => ret);
					<$element_type>::opencv_from_extern(ret)
				}
			}
			)+

			/// Create new from a Rust tuple
			#[inline]
			pub fn new(s: $type) -> Self {
				$( let mut $arg = s.$num.opencv_into_extern_container_nofail(); )+
				unsafe { Self::opencv_from_extern($crate::sys::$extern_new($( $arg.opencv_as_extern_mut() ),+)) }
			}

			/// Convert into a Rust tuple
			#[inline]
			pub fn into_tuple(self) -> $type {
				($( self.$element_get() ),+)
			}
		}
	};
}
