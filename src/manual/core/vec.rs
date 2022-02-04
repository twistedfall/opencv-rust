use std::{
	array,
	ffi::c_void,
	ops::{Deref, DerefMut, MulAssign},
};

use num_traits::Float;

use crate::{
	core::{_InputArray, _InputOutputArray, _OutputArray, ToInputArray, ToInputOutputArray, ToOutputArray},
	Result,
	sys,
	traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer},
};

mod operations;

/// [docs.opencv.org](https://docs.opencv.org/master/d6/dcf/classcv_1_1Vec.html)
/// Named `VecN` to avoid name clash with std's `Vec`.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct VecN<T, const N: usize> (pub [T; N]);

impl<T, const N: usize> Default for VecN<T, N>
	where
		[T; N]: Default
{
	#[inline]
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<T: Copy, const N: usize> VecN<T, N> {
	#[inline]
	pub fn all(v0: T) -> Self {
		Self::from([v0; N])
	}

	/// per-element multiplication
	#[inline]
	pub fn mul(&self, v: Self) -> Self where T: MulAssign {
		let mut out = *self;
		out.iter_mut()
			.zip(v.into_iter())
			.for_each(|(dest, m)| *dest *= m);
		out
	}
}

impl<T, const N: usize> From<[T; N]> for VecN<T, N> {
	#[inline]
	fn from(s: [T; N]) -> Self {
		Self(s)
	}
}

impl<T, const N: usize> Deref for VecN<T, N> {
	type Target = [T; N];

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T, const N: usize> DerefMut for VecN<T, N> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T, const N: usize> IntoIterator for VecN<T, N> {
	type Item = T;
	type IntoIter = array::IntoIter<T, N>;

	#[inline]
	fn into_iter(self) -> array::IntoIter<T, N> {
		IntoIterator::into_iter(self.0)
	}
}

impl<F: Float> VecN<F, 2> {
	/// conjugation (makes sense for complex numbers and quaternions)
	#[inline]
	pub fn conj(&self) -> Self {
		Self([self[0], -self[1]])
	}
}

impl<F: Float> VecN<F, 3> {
	/// cross product of the two 3D vectors
	#[inline]
	pub fn cross(&self, v: Self) -> Self {
		Self([
			self[1] * v[2] - self[2] * v[1],
			self[2] * v[0] - self[0] * v[2],
			self[0] * v[1] - self[1] * v[0]
		])
	}
}

impl<F: Float> VecN<F, 4> {
	/// conjugation (makes sense for complex numbers and quaternions)
	#[inline]
	pub fn conj(&self) -> Self {
		Self([self[0], -self[1], -self[2], -self[3]])
	}
}

impl<T, const N: usize> OpenCVType<'_> for VecN<T, N> {
	type Arg = Self;
	type ExternReceive = Self;
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self { self }

	#[inline]
	unsafe fn opencv_from_extern(s: Self) -> Self { s }
}

impl<T, const N: usize> OpenCVTypeArg<'_> for VecN<T, N> {
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self { self }
}

impl<T, const N: usize> OpenCVTypeExternContainer for VecN<T, N> {
	type ExternSend = *const Self;
	type ExternSendMut = *mut Self;

	#[inline]
	fn opencv_as_extern(&self) -> Self::ExternSend { self }

	#[inline]
	fn opencv_as_extern_mut(&mut self) -> Self::ExternSendMut { self }

	#[inline]
	fn opencv_into_extern(self) -> Self::ExternSendMut { &mut *std::mem::ManuallyDrop::new(self) as _ }
}

impl<T, const N: usize> ToInputArray for VecN<T, N> where Self: VecExtern<T, N> {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		unsafe { self.extern_input_array() }
			.into_result()
			.map(|ptr| unsafe { _InputArray::from_raw(ptr) })
	}
}

impl<T, const N: usize> ToInputArray for &VecN<T, N> where VecN<T, N>: VecExtern<T, N> {
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		(*self).input_array()
	}
}

impl<T, const N: usize> ToOutputArray for VecN<T, N> where Self: VecExtern<T, N> {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		unsafe { self.extern_output_array() }
			.into_result()
			.map(|ptr| unsafe { _OutputArray::from_raw(ptr) })
	}
}

impl<T, const N: usize> ToOutputArray for &mut VecN<T, N> where VecN<T, N>: VecExtern<T, N> {
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		(*self).output_array()
	}
}

impl<T, const N: usize> ToInputOutputArray for VecN<T, N> where Self: VecExtern<T, N> {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		unsafe { self.extern_input_output_array() }
			.into_result()
			.map(|ptr| unsafe { _InputOutputArray::from_raw(ptr) })
	}
}

impl<T, const N: usize> ToInputOutputArray for &mut VecN<T, N> where VecN<T, N>: VecExtern<T, N> {
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		(*self).input_output_array()
	}
}

#[doc(hidden)]
pub trait VecExtern<T, const N: usize> {
	#[doc(hidden)] unsafe fn extern_input_array(&self) -> sys::Result<*mut c_void>;
	#[doc(hidden)] unsafe fn extern_output_array(&mut self) -> sys::Result<*mut c_void>;
	#[doc(hidden)] unsafe fn extern_input_output_array(&mut self) -> sys::Result<*mut c_void>;
}

macro_rules! vecn_extern {
	($type: ty, $len: expr, $extern_input_array: ident, $extern_ouput_array: ident, $extern_input_array_output: ident) => {
		impl $crate::manual::core::VecExtern<$type, $len> for $crate::manual::core::VecN<$type, $len> {
			#[inline]
			unsafe fn extern_input_array(&self) -> $crate::sys::Result<*mut ::std::ffi::c_void> {
				extern "C" { fn $extern_input_array(instance: *const $crate::manual::core::VecN<$type, $len>, ocvrs_return: *mut $crate::sys::Result<*mut ::std::ffi::c_void>); }
				return_send!(via ocvrs_return);
				$extern_input_array(self, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}

			#[inline]
			unsafe fn extern_output_array(&mut self) -> $crate::sys::Result<*mut ::std::ffi::c_void> {
				extern "C" { fn $extern_ouput_array(instance: *mut $crate::manual::core::VecN<$type, $len>, ocvrs_return: *mut $crate::sys::Result<*mut ::std::ffi::c_void>); }
				return_send!(via ocvrs_return);
				$extern_ouput_array(self, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}

			#[inline]
			unsafe fn extern_input_output_array(&mut self) -> $crate::sys::Result<*mut ::std::ffi::c_void> {
				extern "C" { fn $extern_input_array_output(instance: *mut $crate::manual::core::VecN<$type, $len>, ocvrs_return: *mut $crate::sys::Result<*mut ::std::ffi::c_void>); }
				return_send!(via ocvrs_return);
				$extern_input_array_output(self, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}
		}
	}
}

vecn_extern!(u8, 2, cv_Vec2b_input_array, cv_Vec2b_output_array, cv_Vec2b_input_output_array);
vecn_extern!(f64, 2, cv_Vec2d_input_array, cv_Vec2d_output_array, cv_Vec2d_input_output_array);
vecn_extern!(f32, 2, cv_Vec2f_input_array, cv_Vec2f_output_array, cv_Vec2f_input_output_array);
vecn_extern!(i32, 2, cv_Vec2i_input_array, cv_Vec2i_output_array, cv_Vec2i_input_output_array);
vecn_extern!(i16, 2, cv_Vec2s_input_array, cv_Vec2s_output_array, cv_Vec2s_input_output_array);
vecn_extern!(u16, 2, cv_Vec2w_input_array, cv_Vec2w_output_array, cv_Vec2w_input_output_array);

vecn_extern!(u8, 3, cv_Vec3b_input_array, cv_Vec3b_output_array, cv_Vec3b_input_output_array);
vecn_extern!(f64, 3, cv_Vec3d_input_array, cv_Vec3d_output_array, cv_Vec3d_input_output_array);
vecn_extern!(f32, 3, cv_Vec3f_input_array, cv_Vec3f_output_array, cv_Vec3f_input_output_array);
vecn_extern!(i32, 3, cv_Vec3i_input_array, cv_Vec3i_output_array, cv_Vec3i_input_output_array);
vecn_extern!(i16, 3, cv_Vec3s_input_array, cv_Vec3s_output_array, cv_Vec3s_input_output_array);
vecn_extern!(u16, 3, cv_Vec3w_input_array, cv_Vec3w_output_array, cv_Vec3w_input_output_array);

vecn_extern!(u8, 4, cv_Vec4b_input_array, cv_Vec4b_output_array, cv_Vec4b_input_output_array);
vecn_extern!(f64, 4, cv_Vec4d_input_array, cv_Vec4d_output_array, cv_Vec4d_input_output_array);
vecn_extern!(f32, 4, cv_Vec4f_input_array, cv_Vec4f_output_array, cv_Vec4f_input_output_array);
vecn_extern!(i32, 4, cv_Vec4i_input_array, cv_Vec4i_output_array, cv_Vec4i_input_output_array);
vecn_extern!(i16, 4, cv_Vec4s_input_array, cv_Vec4s_output_array, cv_Vec4s_input_output_array);
vecn_extern!(u16, 4, cv_Vec4w_input_array, cv_Vec4w_output_array, cv_Vec4w_input_output_array);

vecn_extern!(f64, 6, cv_Vec6d_input_array, cv_Vec6d_output_array, cv_Vec6d_input_output_array);
vecn_extern!(f32, 6, cv_Vec6f_input_array, cv_Vec6f_output_array, cv_Vec6f_input_output_array);
vecn_extern!(i32, 6, cv_Vec6i_input_array, cv_Vec6i_output_array, cv_Vec6i_input_output_array);

vecn_extern!(i32, 8, cv_Vec8i_input_array, cv_Vec8i_output_array, cv_Vec8i_input_output_array);

vecn_extern!(f64, 18, cv_Vec18d_input_array, cv_Vec18d_output_array, cv_Vec18d_input_output_array);
