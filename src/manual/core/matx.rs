use std::fmt;

use num_traits::{One, Zero};

use crate::core::{ToInputArray, ToInputOutputArray, ToOutputArray, _InputArray, _InputOutputArray, _OutputArray};
use crate::manual::core::sized::*;
use crate::traits::{Boxed, OpenCVType, OpenCVTypeArg, OpenCVTypeExternContainer};
use crate::{core, extern_receive, extern_send, sys, Error, Result};

fn index_check(idx: (usize, usize), rows: usize, cols: usize) -> Result<()> {
	if idx.0 >= rows {
		Err(Error::new(
			core::StsOutOfRange,
			format!("Index: {} along dimension: rows out of bounds 0..{}", idx.0, rows),
		))
	} else if idx.1 >= cols {
		Err(Error::new(
			core::StsOutOfRange,
			format!("Index: {} along dimension: cols out of bounds 0..{}", idx.1, cols),
		))
	} else {
		Ok(())
	}
}

pub trait MatxTrait: Sized {
	type ElemType;

	fn rows(&self) -> usize;
	fn cols(&self) -> usize;
	fn val(&self) -> &[Self::ElemType];
	fn val_mut(&mut self) -> &mut [Self::ElemType];

	fn all(alpha: Self::ElemType) -> Self;

	#[inline]
	fn channels(&self) -> usize {
		self.rows() * self.cols()
	}

	#[inline]
	fn shortdim(&self) -> usize {
		self.rows().min(self.cols())
	}

	#[inline]
	fn zeros() -> Self
	where
		Self::ElemType: Zero,
	{
		Self::all(Self::ElemType::zero())
	}

	#[inline]
	fn ones() -> Self
	where
		Self::ElemType: One,
	{
		Self::all(Self::ElemType::one())
	}

	#[inline]
	fn get(&self, idx: (usize, usize)) -> Option<&Self::ElemType> {
		index_check(idx, self.rows(), self.cols())
			.ok()
			.map(|_| unsafe { self.get_unchecked(idx) })
	}

	/// # Safety
	/// Caller must ensure that the specified `idx` is within the `Matx` bounds
	#[inline]
	unsafe fn get_unchecked(&self, idx: (usize, usize)) -> &Self::ElemType {
		self.val().get_unchecked(idx.0 * self.cols() + idx.1)
	}

	#[inline]
	fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut Self::ElemType> {
		index_check(idx, self.rows(), self.cols()).ok()?;
		Some(unsafe { self.get_unchecked_mut(idx) })
	}

	/// # Safety
	/// Caller must ensure that the specified `idx` is within the `Matx` bounds
	#[inline]
	unsafe fn get_unchecked_mut(&mut self, idx: (usize, usize)) -> &mut Self::ElemType {
		let cols = self.cols();
		self.val_mut().get_unchecked_mut(idx.0 * cols + idx.1)
	}

	#[inline]
	fn eye() -> Self
	where
		Self::ElemType: One + Zero,
	{
		let mut out = Self::zeros();
		(0..out.shortdim()).for_each(|i| {
			unsafe { out.get_unchecked_mut((i, i)) }.set_one();
		});
		out
	}
}

/// [docs.opencv.org](https://docs.opencv.org/master/de/de1/classcv_1_1Matx.html)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Matx<T, A: SizedArray<T>> {
	pub val: A::Storage,
}

impl<T, A: SizedArray<T>> Matx<T, A> {
	#[inline]
	pub fn from_array(s: A::Storage) -> Self {
		Self { val: s }
	}
}

impl<T, A: SizedArray<T>> MatxTrait for Matx<T, A> {
	type ElemType = T;

	#[inline]
	fn rows(&self) -> usize {
		A::ROWS
	}

	#[inline]
	fn cols(&self) -> usize {
		A::COLS
	}

	#[inline]
	fn val(&self) -> &[Self::ElemType] {
		self.val.as_ref()
	}

	#[inline]
	fn val_mut(&mut self) -> &mut [Self::ElemType] {
		self.val.as_mut()
	}

	#[inline]
	fn all(alpha: Self::ElemType) -> Self
	where
		Self: Sized,
	{
		Self { val: A::all(alpha) }
	}

	#[inline]
	fn channels(&self) -> usize {
		A::ROWS * A::COLS
	}
}

impl<T: Default, A: SizedArray<T>> Default for Matx<T, A> {
	#[inline]
	fn default() -> Self {
		Self::all(T::default())
	}
}

impl<T, A: SizedArray<T>> std::ops::Index<(usize, usize)> for Matx<T, A> {
	type Output = T;

	#[inline]
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		self.get(index).expect("Index out of range")
	}
}

impl<T, A: SizedArray<T>> std::ops::IndexMut<(usize, usize)> for Matx<T, A> {
	#[inline]
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		self.get_mut(index).expect("Index out of range")
	}
}

impl<T, A: SizedArray<T>> OpenCVType<'_> for Matx<T, A> {
	type Arg = Self;
	type ExternReceive = Self;

	#[inline]
	unsafe fn opencv_from_extern(s: Self) -> Self {
		s
	}
}

impl<T, A: SizedArray<T>> OpenCVTypeArg<'_> for Matx<T, A> {
	type ExternContainer = Self;

	#[inline]
	fn opencv_into_extern_container_nofail(self) -> Self::ExternContainer {
		self
	}
}

impl<T, A: SizedArray<T>> OpenCVTypeExternContainer for Matx<T, A> {
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

impl<T: PartialEq, A: SizedArray<T>> PartialEq for Matx<T, A> {
	#[inline]
	fn eq(&self, other: &Matx<T, A>) -> bool {
		self.val() == other.val()
	}
}

impl<T, A: SizedArray<T>> fmt::Debug for Matx<T, A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Matx")
			.field("rows", &self.rows())
			.field("cols", &self.cols())
			.field("channels", &self.channels())
			.field("shortdim", &self.shortdim())
			.finish()
	}
}

impl<T, A: SizedArray<T>> ToInputArray for Matx<T, A>
where
	Self: MatxExtern,
{
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		unsafe { self.extern_input_array() }
			.into_result()
			.map(|ptr| unsafe { _InputArray::from_raw(ptr) })
	}
}

impl<T, A: SizedArray<T>> ToInputArray for &Matx<T, A>
where
	Matx<T, A>: MatxExtern,
{
	#[inline]
	fn input_array(&self) -> Result<_InputArray> {
		(*self).input_array()
	}
}

impl<T, A: SizedArray<T>> ToOutputArray for Matx<T, A>
where
	Self: MatxExtern,
{
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		unsafe { self.extern_output_array() }
			.into_result()
			.map(|ptr| unsafe { _OutputArray::from_raw(ptr) })
	}
}

impl<T, A: SizedArray<T>> ToOutputArray for &mut Matx<T, A>
where
	Matx<T, A>: MatxExtern,
{
	#[inline]
	fn output_array(&mut self) -> Result<_OutputArray> {
		(*self).output_array()
	}
}

impl<T, A: SizedArray<T>> ToInputOutputArray for Matx<T, A>
where
	Self: MatxExtern,
{
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		unsafe { self.extern_input_output_array() }
			.into_result()
			.map(|ptr| unsafe { _InputOutputArray::from_raw(ptr) })
	}
}

impl<T, A: SizedArray<T>> ToInputOutputArray for &mut Matx<T, A>
where
	Matx<T, A>: MatxExtern,
{
	#[inline]
	fn input_output_array(&mut self) -> Result<_InputOutputArray> {
		(*self).input_output_array()
	}
}

#[doc(hidden)]
pub trait MatxExtern {
	#[doc(hidden)]
	unsafe fn extern_input_array(&self) -> sys::Result<extern_receive!(_InputArray)>;
	#[doc(hidden)]
	unsafe fn extern_output_array(&mut self) -> sys::Result<extern_receive!(_OutputArray)>;
	#[doc(hidden)]
	unsafe fn extern_input_output_array(&mut self) -> sys::Result<extern_receive!(_InputOutputArray)>;
}

macro_rules! matx_extern {
	($type: ty, $array: ty, $extern_input_array: ident, $extern_ouput_array: ident, $extern_input_array_output: ident) => {
		extern "C" {
			fn $extern_input_array(instance: extern_send!($crate::core::Matx<$type, $array>), ocvrs_return: *mut $crate::sys::Result<extern_receive!($crate::core::_InputArray)>);
			fn $extern_ouput_array(instance: extern_send!(mut $crate::core::Matx<$type, $array>), ocvrs_return: *mut $crate::sys::Result<extern_receive!($crate::core::_OutputArray)>);
			fn $extern_input_array_output(instance: extern_send!(mut $crate::core::Matx<$type, $array>), ocvrs_return: *mut $crate::sys::Result<extern_receive!($crate::core::_InputOutputArray)>);
		}

		impl $crate::core::MatxExtern for $crate::core::Matx<$type, $array> {
			#[inline]
			unsafe fn extern_input_array(&self) -> $crate::sys::Result<extern_receive!($crate::core::_InputArray)> {
				return_send!(via ocvrs_return);
				$extern_input_array(self, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}

			#[inline]
			unsafe fn extern_output_array(&mut self) -> $crate::sys::Result<extern_receive!($crate::core::_OutputArray)> {
				return_send!(via ocvrs_return);
				$extern_ouput_array(self, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}

			#[inline]
			unsafe fn extern_input_output_array(&mut self) -> $crate::sys::Result<extern_receive!($crate::core::_InputOutputArray)> {
				return_send!(via ocvrs_return);
				$extern_input_array_output(self, ocvrs_return.as_mut_ptr());
				return_receive!(ocvrs_return => ret);
				ret
			}
		}
	}
}

pub type Matx12<T> = Matx<T, SizedArray12>;
matx_extern!(
	f32,
	SizedArray12,
	cv_Matx12f_input_array,
	cv_Matx12f_output_array,
	cv_Matx12f_input_output_array
);
matx_extern!(
	f64,
	SizedArray12,
	cv_Matx12d_input_array,
	cv_Matx12d_output_array,
	cv_Matx12d_input_output_array
);
pub type Matx13<T> = Matx<T, SizedArray13>;
matx_extern!(
	f32,
	SizedArray13,
	cv_Matx13f_input_array,
	cv_Matx13f_output_array,
	cv_Matx13f_input_output_array
);
matx_extern!(
	f64,
	SizedArray13,
	cv_Matx13d_input_array,
	cv_Matx13d_output_array,
	cv_Matx13d_input_output_array
);
pub type Matx14<T> = Matx<T, SizedArray14>;
matx_extern!(
	f32,
	SizedArray14,
	cv_Matx14f_input_array,
	cv_Matx14f_output_array,
	cv_Matx14f_input_output_array
);
matx_extern!(
	f64,
	SizedArray14,
	cv_Matx14d_input_array,
	cv_Matx14d_output_array,
	cv_Matx14d_input_output_array
);
pub type Matx16<T> = Matx<T, SizedArray16>;
matx_extern!(
	f32,
	SizedArray16,
	cv_Matx16f_input_array,
	cv_Matx16f_output_array,
	cv_Matx16f_input_output_array
);
matx_extern!(
	f64,
	SizedArray16,
	cv_Matx16d_input_array,
	cv_Matx16d_output_array,
	cv_Matx16d_input_output_array
);

pub type Matx21<T> = Matx<T, SizedArray21>;
matx_extern!(
	f32,
	SizedArray21,
	cv_Matx21f_input_array,
	cv_Matx21f_output_array,
	cv_Matx21f_input_output_array
);
matx_extern!(
	f64,
	SizedArray21,
	cv_Matx21d_input_array,
	cv_Matx21d_output_array,
	cv_Matx21d_input_output_array
);
pub type Matx31<T> = Matx<T, SizedArray31>;
matx_extern!(
	f32,
	SizedArray31,
	cv_Matx31f_input_array,
	cv_Matx31f_output_array,
	cv_Matx31f_input_output_array
);
matx_extern!(
	f64,
	SizedArray31,
	cv_Matx31d_input_array,
	cv_Matx31d_output_array,
	cv_Matx31d_input_output_array
);
pub type Matx41<T> = Matx<T, SizedArray41>;
matx_extern!(
	f32,
	SizedArray41,
	cv_Matx41f_input_array,
	cv_Matx41f_output_array,
	cv_Matx41f_input_output_array
);
matx_extern!(
	f64,
	SizedArray41,
	cv_Matx41d_input_array,
	cv_Matx41d_output_array,
	cv_Matx41d_input_output_array
);
pub type Matx61<T> = Matx<T, SizedArray61>;
matx_extern!(
	f32,
	SizedArray61,
	cv_Matx61f_input_array,
	cv_Matx61f_output_array,
	cv_Matx61f_input_output_array
);
matx_extern!(
	f64,
	SizedArray61,
	cv_Matx61d_input_array,
	cv_Matx61d_output_array,
	cv_Matx61d_input_output_array
);

pub type Matx22<T> = Matx<T, SizedArray22>;
matx_extern!(
	f32,
	SizedArray22,
	cv_Matx22f_input_array,
	cv_Matx22f_output_array,
	cv_Matx22f_input_output_array
);
matx_extern!(
	f64,
	SizedArray22,
	cv_Matx22d_input_array,
	cv_Matx22d_output_array,
	cv_Matx22d_input_output_array
);
pub type Matx23<T> = Matx<T, SizedArray23>;
matx_extern!(
	f32,
	SizedArray23,
	cv_Matx23f_input_array,
	cv_Matx23f_output_array,
	cv_Matx23f_input_output_array
);
matx_extern!(
	f64,
	SizedArray23,
	cv_Matx23d_input_array,
	cv_Matx23d_output_array,
	cv_Matx23d_input_output_array
);
pub type Matx32<T> = Matx<T, SizedArray32>;
matx_extern!(
	f32,
	SizedArray32,
	cv_Matx32f_input_array,
	cv_Matx32f_output_array,
	cv_Matx32f_input_output_array
);
matx_extern!(
	f64,
	SizedArray32,
	cv_Matx32d_input_array,
	cv_Matx32d_output_array,
	cv_Matx32d_input_output_array
);

pub type Matx33<T> = Matx<T, SizedArray33>;
matx_extern!(
	f32,
	SizedArray33,
	cv_Matx33f_input_array,
	cv_Matx33f_output_array,
	cv_Matx33f_input_output_array
);
matx_extern!(
	f64,
	SizedArray33,
	cv_Matx33d_input_array,
	cv_Matx33d_output_array,
	cv_Matx33d_input_output_array
);

pub type Matx34<T> = Matx<T, SizedArray34>;
matx_extern!(
	f32,
	SizedArray34,
	cv_Matx34f_input_array,
	cv_Matx34f_output_array,
	cv_Matx34f_input_output_array
);
matx_extern!(
	f64,
	SizedArray34,
	cv_Matx34d_input_array,
	cv_Matx34d_output_array,
	cv_Matx34d_input_output_array
);
pub type Matx43<T> = Matx<T, SizedArray43>;
matx_extern!(
	f32,
	SizedArray43,
	cv_Matx43f_input_array,
	cv_Matx43f_output_array,
	cv_Matx43f_input_output_array
);
matx_extern!(
	f64,
	SizedArray43,
	cv_Matx43d_input_array,
	cv_Matx43d_output_array,
	cv_Matx43d_input_output_array
);

pub type Matx44<T> = Matx<T, SizedArray44>;
matx_extern!(
	f32,
	SizedArray44,
	cv_Matx44f_input_array,
	cv_Matx44f_output_array,
	cv_Matx44f_input_output_array
);
matx_extern!(
	f64,
	SizedArray44,
	cv_Matx44d_input_array,
	cv_Matx44d_output_array,
	cv_Matx44d_input_output_array
);
pub type Matx66<T> = Matx<T, SizedArray66>;
matx_extern!(
	f32,
	SizedArray66,
	cv_Matx66f_input_array,
	cv_Matx66f_output_array,
	cv_Matx66f_input_output_array
);
matx_extern!(
	f64,
	SizedArray66,
	cv_Matx66d_input_array,
	cv_Matx66d_output_array,
	cv_Matx66d_input_output_array
);
