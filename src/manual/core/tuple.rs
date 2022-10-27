use crate::traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

pub struct Tuple<T>
where
	Self: TupleExtern<T>,
{
	ptr: *mut c_void,
	_d: PhantomData<T>,
}

impl<T> Boxed for Tuple<T>
where
	Self: TupleExtern<T>,
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
	Self: TupleExtern<T>,
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
	Tuple<T>: TupleExtern<T>,
{
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl<T> OpenCVTypeExternContainer for Tuple<T>
where
	Tuple<T>: TupleExtern<T>,
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

	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut {
		self.into_raw()
	}
}

impl<T> Drop for Tuple<T>
where
	Tuple<T>: TupleExtern<T>,
{
	fn drop(&mut self) {
		unsafe { self.extern_delete() }
	}
}

pub trait TupleExtern<T> {
	#[doc(hidden)]
	unsafe fn extern_delete(&mut self);
}

#[macro_export]
macro_rules! tuple_extern {
	(
		$type: ty,
		$extern_const: ty,
		$extern_mut: ty,
		$extern_new: ident, $extern_delete: ident,
		$($arg: ident: $extern_element_type: ty = $num: tt, $extern_element_get: ident: $extern_element_return: ty => $element_get: ident: $element_type: ty),+
	) => {
		extern "C" {
			fn $extern_new($( $arg: $extern_element_type ),+) -> $extern_mut;
			fn $extern_delete(instance: $extern_mut);
			$(
				fn $extern_element_get(instance: $extern_const, ocvrs_return: *mut <$extern_element_return as $crate::traits::OpenCVType<'_>>::ExternReceive);
			)+
		}

		impl $crate::manual::core::TupleExtern<$type> for $crate::manual::core::Tuple<$type> {
			#[inline]
			unsafe fn extern_delete(&mut self) {
				$extern_delete(self.as_raw_mut())
			}
		}

		impl $crate::manual::core::Tuple<$type> {
			$(
			#[inline]
			pub fn $element_get(&self) -> $element_type {
				return_send!(via ocvrs_return);
				unsafe {
					$extern_element_get(self.as_raw(), ocvrs_return.as_mut_ptr());
					return_receive!(ocvrs_return => ret);
					<$element_type>::opencv_from_extern(ret)
				}
			}
			)+

			/// Create new from a Rust tuple
			#[inline]
			pub fn new(s: $type) -> Self {
				$( let mut $arg = s.$num.opencv_into_extern_container_nofail(); )+
				unsafe { Self::opencv_from_extern($extern_new($( $arg.opencv_as_extern_mut() ),+)) }
			}

			/// Convert into a Rust tuple
			#[inline]
			pub fn into_tuple(self) -> $type {
				($( self.$element_get() ),+)
			}
		}
	};
}
