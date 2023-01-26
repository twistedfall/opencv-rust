use std::os::raw::c_void;

use crate::core::{_InputArray, _InputArrayTrait, _InputOutputArray, _InputOutputArrayTrait, _OutputArray, _OutputArrayTrait};
use crate::traits::Boxed;
use crate::{input_output_array, sys, Result};

/// Trait to serve as a replacement for `InputArray` in C++ OpenCV
///
/// You can pass references to the types implementing this trait everywhere where OpenCV API expects
/// `InputArray` or `InputArrayOfArrays`.
///
/// More info in [OpenCV docs](https://docs.opencv.org/master/d4/d32/classcv_1_1__InputArray.html#details).
pub trait ToInputArray {
	fn input_array(&self) -> Result<_InputArray>;
}

input_output_array! { f64, from_f64 }

/// Trait to serve as a replacement for `OutputArray` in C++ OpenCV
///
/// You can pass reference to the type implementing this trait everywhere where OpenCV API expects
/// `OutputArray` or `OutputArrayOfArrays`.
///
/// More info in [OpenCV docs](https://docs.opencv.org/master/d2/d9e/classcv_1_1__OutputArray.html#details).
pub trait ToOutputArray {
	fn output_array(&mut self) -> Result<_OutputArray>;
}

/// Trait to serve as a replacement for `InputOutputArray` in C++ OpenCV
///
/// You can pass reference to the type implementing this trait everywhere where OpenCV API expects
/// `InputOutputArray` or `InputOutputArrayOfArrays`.
///
/// For more info see comments for `ToInputArray` and `ToOutputArray`.
pub trait ToInputOutputArray {
	fn input_output_array(&mut self) -> Result<_InputOutputArray>;
}

impl<T: _InputArrayTrait> ToInputArray for T {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		extern "C" {
			fn cv_InputArray_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>);
		}
		return_send!(via ocvrs_return);
		unsafe { cv_InputArray_input_array(self.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result().map(|ptr| unsafe { _InputArray::from_raw(ptr) })
	}
}

impl<T: _OutputArrayTrait> ToOutputArray for T {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		extern "C" {
			fn cv_OutputArray_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>);
		}
		return_send!(via ocvrs_return);
		unsafe { cv_OutputArray_output_array(self.as_raw_mut__OutputArray(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result().map(|ptr| unsafe { _OutputArray::from_raw(ptr) })
	}
}

impl<T: _InputOutputArrayTrait> ToInputOutputArray for T {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		extern "C" {
			fn cv_InputOutputArray_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>);
		}
		return_send!(via ocvrs_return);
		unsafe { cv_InputOutputArray_input_output_array(self.as_raw_mut__InputOutputArray(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result().map(|ptr| unsafe { _InputOutputArray::from_raw(ptr) })
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! input_output_array {
	($type: ty, $const_cons: ident) => {
		impl $crate::core::ToInputArray for $type {
			#[inline]
			fn input_array(&self) -> $crate::Result<$crate::core::_InputArray> {
				$crate::core::_InputArray::$const_cons(self)
			}
		}

		$crate::input_array_ref_forward! { $type }
	};

	($type: ty, $const_cons: ident, $mut_cons: ident) => {
		$crate::input_output_array! { $type, $const_cons }

		impl $crate::core::ToOutputArray for $type {
			#[inline]
			fn output_array(&mut self) -> $crate::Result<$crate::core::_OutputArray> {
				$crate::core::_OutputArray::$mut_cons(self)
			}
		}

		impl $crate::core::ToInputOutputArray for $type {
			#[inline]
			fn input_output_array(&mut self) -> $crate::Result<$crate::core::_InputOutputArray> {
				$crate::core::_InputOutputArray::$mut_cons(self)
			}
		}

		$crate::output_array_ref_forward! { $type }
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! input_array_ref_forward {
	($type: ty) => {
		impl $crate::core::ToInputArray for &$type {
			#[inline]
			fn input_array(&self) -> $crate::Result<$crate::core::_InputArray> {
				(*self).input_array()
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! output_array_ref_forward {
	($type: ty) => {
		impl $crate::core::ToOutputArray for &mut $type {
			#[inline]
			fn output_array(&mut self) -> $crate::Result<$crate::core::_OutputArray> {
				(*self).output_array()
			}
		}

		impl $crate::core::ToInputOutputArray for &mut $type {
			#[inline]
			fn input_output_array(&mut self) -> $crate::Result<$crate::core::_InputOutputArray> {
				(*self).input_output_array()
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! input_output_array_ref_forward {
	($type: ty) => {
		$crate::input_array_ref_forward! { $type }
		$crate::output_array_ref_forward! { $type }
	};
}
