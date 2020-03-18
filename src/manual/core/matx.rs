use std::{
	ffi::c_void,
	fmt,
};

use num::{One, Zero};

use crate::{
	core::{self, ToInputArray, ToInputOutputArray, ToOutputArray},
	Error,
	manual::core::sized::*,
	Result,
	sys::Result as SysResult,
};

fn index_check(idx: (usize, usize), rows: usize, cols: usize) -> Result<()> {
	if idx.0 >= rows {
		Err(Error::new(core::StsOutOfRange, format!("Index: {} along dimension: rows out of bounds 0..{}", idx.0, rows)))
	} else if idx.1 >= cols {
		Err(Error::new(core::StsOutOfRange, format!("Index: {} along dimension: cols out of bounds 0..{}", idx.1, cols)))
	} else {
		Ok(())
	}
}

valid_types!(ValidMatxType, f32, f64);

pub trait MatxTrait: Sized {
	type ElemType: ValidMatxType;

	fn rows(&self) -> usize;
	fn cols(&self) -> usize;
	fn val(&self) -> &[Self::ElemType];
	fn val_mut(&mut self) -> &mut [Self::ElemType];

	fn all(alpha: Self::ElemType) -> Self;

	fn channels(&self) -> usize {
		self.rows() * self.cols()
	}

	fn shortdim(&self) -> usize {
		self.rows().min(self.cols())
	}

	fn zeros() -> Self {
		Self::all(Self::ElemType::zero())
	}

	fn ones() -> Self {
		Self::all(Self::ElemType::one())
	}

	fn get(&self, idx: (usize, usize)) -> Option<&Self::ElemType> {
		index_check(idx, self.rows(), self.cols())
			.ok()
			.map(|_| unsafe { self.get_unchecked(idx) })
	}

	unsafe fn get_unchecked(&self, idx: (usize, usize)) -> &Self::ElemType {
		self.val().get_unchecked(idx.0 * self.cols() + idx.1)
	}

	fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut Self::ElemType> {
		index_check(idx, self.rows(), self.cols())
			.ok()?;
		Some(unsafe { self.get_unchecked_mut(idx) })
	}

	unsafe fn get_unchecked_mut(&mut self, idx: (usize, usize)) -> &mut Self::ElemType {
		let cols = self.cols();
		self.val_mut().get_unchecked_mut(idx.0 * cols + idx.1)
	}

	fn eye() -> Self {
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
pub struct Matx<T: ValidMatxType, A: SizedArray<T>> {
	pub val: A::Storage,
}

impl<T: ValidMatxType, A: SizedArray<T>> Matx<T, A> {
	pub fn from(s: A::Storage) -> Self {
		Self { val: s }
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> MatxTrait for Matx<T, A> {
	type ElemType = T;

	fn rows(&self) -> usize {
		A::ROWS
	}

	fn cols(&self) -> usize {
		A::COLS
	}

	fn val(&self) -> &[Self::ElemType] {
		self.val.as_ref()
	}

	fn val_mut(&mut self) -> &mut [Self::ElemType] {
		self.val.as_mut()
	}

	fn all(alpha: Self::ElemType) -> Self where Self: Sized {
		Self { val: A::all(alpha) }
	}

	fn channels(&self) -> usize {
		A::ROWS * A::COLS
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> Default for Matx<T, A> {
	fn default() -> Self {
		Self::all(T::default())
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> std::ops::Index<(usize, usize)> for Matx<T, A> {
	type Output = T;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		self.get(index).expect("Index out of range")
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> std::ops::IndexMut<(usize, usize)> for Matx<T, A> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		self.get_mut(index).expect("Index out of range")
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> std::cmp::PartialEq for Matx<T, A> {
	fn eq(&self, other: &Matx<T, A>) -> bool {
		&self.val() == &other.val()
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> fmt::Debug for Matx<T, A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Matx")
			.field("rows", &self.rows())
			.field("cols", &self.cols())
			.field("channels", &self.channels())
			.field("shortdim", &self.shortdim())
			.finish()
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> ToInputArray for Matx<T, A> where Self: MatxExtern<T, A> {
	fn input_array(&self) -> Result<core::_InputArray> {
		unsafe { self.extern_input_array() }
			.into_result()
			.map(|ptr| core::_InputArray { ptr })
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> ToInputArray for &Matx<T, A> where Matx<T, A>: MatxExtern<T, A> {
	fn input_array(&self) -> Result<core::_InputArray> {
		(*self).input_array()
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> ToOutputArray for Matx<T, A> where Self: MatxExtern<T, A> {
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		unsafe { self.extern_output_array() }
			.into_result()
			.map(|ptr| core::_OutputArray { ptr })
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> ToOutputArray for &mut Matx<T, A> where Matx<T, A>: MatxExtern<T, A> {
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		(*self).output_array()
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> ToInputOutputArray for Matx<T, A> where Self: MatxExtern<T, A> {
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		unsafe { self.extern_input_output_array() }
			.into_result()
			.map(|ptr| core::_InputOutputArray { ptr })
	}
}

impl<T: ValidMatxType, A: SizedArray<T>> ToInputOutputArray for &mut Matx<T, A> where Matx<T, A>: MatxExtern<T, A> {
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		(*self).input_output_array()
	}
}

#[doc(hidden)]
pub trait MatxExtern<T: ValidMatxType, A: SizedArray<T>> {
	unsafe fn extern_input_array(&self) -> SysResult<*mut c_void>;
	unsafe fn extern_output_array(&mut self) -> SysResult<*mut c_void>;
	unsafe fn extern_input_output_array(&mut self) -> SysResult<*mut c_void>;
}

macro_rules! matx_extern {
	($type: ty, $array: ty, $extern_input_array: ident, $extern_ouput_array: ident, $extern_input_array_output: ident) => {
		impl $crate::manual::core::MatxExtern<$type, $array> for $crate::manual::core::Matx<$type, $array> {
			unsafe fn extern_input_array(&self) -> $crate::sys::Result<*mut c_void> {
				extern "C" { fn $extern_input_array(instance: *const $crate::manual::core::Matx<$type, $array>) -> $crate::sys::Result<*mut c_void>; }
				$extern_input_array(self)
			}

			unsafe fn extern_output_array(&mut self) -> $crate::sys::Result<*mut c_void> {
				extern "C" { fn $extern_ouput_array(instance: *mut $crate::manual::core::Matx<$type, $array>) -> $crate::sys::Result<*mut c_void>; }
				$extern_ouput_array(self)
			}

			unsafe fn extern_input_output_array(&mut self) -> $crate::sys::Result<*mut c_void> {
				extern "C" { fn $extern_input_array_output(instance: *mut $crate::manual::core::Matx<$type, $array>) -> $crate::sys::Result<*mut c_void>; }
				$extern_input_array_output(self)
			}
		}
	}
}

pub type Matx12<T> = Matx<T, SizedArray12>;
matx_extern!(f32, SizedArray12, cv_Matx12f_input_array, cv_Matx12f_output_array, cv_Matx12f_input_output_array);
matx_extern!(f64, SizedArray12, cv_Matx12d_input_array, cv_Matx12d_output_array, cv_Matx12d_input_output_array);
pub type Matx13<T> = Matx<T, SizedArray13>;
matx_extern!(f32, SizedArray13, cv_Matx13f_input_array, cv_Matx13f_output_array, cv_Matx13f_input_output_array);
matx_extern!(f64, SizedArray13, cv_Matx13d_input_array, cv_Matx13d_output_array, cv_Matx13d_input_output_array);
pub type Matx14<T> = Matx<T, SizedArray14>;
matx_extern!(f32, SizedArray14, cv_Matx14f_input_array, cv_Matx14f_output_array, cv_Matx14f_input_output_array);
matx_extern!(f64, SizedArray14, cv_Matx14d_input_array, cv_Matx14d_output_array, cv_Matx14d_input_output_array);
pub type Matx16<T> = Matx<T, SizedArray16>;
matx_extern!(f32, SizedArray16, cv_Matx16f_input_array, cv_Matx16f_output_array, cv_Matx16f_input_output_array);
matx_extern!(f64, SizedArray16, cv_Matx16d_input_array, cv_Matx16d_output_array, cv_Matx16d_input_output_array);

pub type Matx21<T> = Matx<T, SizedArray21>;
matx_extern!(f32, SizedArray21, cv_Matx21f_input_array, cv_Matx21f_output_array, cv_Matx21f_input_output_array);
matx_extern!(f64, SizedArray21, cv_Matx21d_input_array, cv_Matx21d_output_array, cv_Matx21d_input_output_array);
pub type Matx31<T> = Matx<T, SizedArray31>;
matx_extern!(f32, SizedArray31, cv_Matx31f_input_array, cv_Matx31f_output_array, cv_Matx31f_input_output_array);
matx_extern!(f64, SizedArray31, cv_Matx31d_input_array, cv_Matx31d_output_array, cv_Matx31d_input_output_array);
pub type Matx41<T> = Matx<T, SizedArray41>;
matx_extern!(f32, SizedArray41, cv_Matx41f_input_array, cv_Matx41f_output_array, cv_Matx41f_input_output_array);
matx_extern!(f64, SizedArray41, cv_Matx41d_input_array, cv_Matx41d_output_array, cv_Matx41d_input_output_array);
pub type Matx61<T> = Matx<T, SizedArray61>;
matx_extern!(f32, SizedArray61, cv_Matx61f_input_array, cv_Matx61f_output_array, cv_Matx61f_input_output_array);
matx_extern!(f64, SizedArray61, cv_Matx61d_input_array, cv_Matx61d_output_array, cv_Matx61d_input_output_array);

pub type Matx22<T> = Matx<T, SizedArray22>;
matx_extern!(f32, SizedArray22, cv_Matx22f_input_array, cv_Matx22f_output_array, cv_Matx22f_input_output_array);
matx_extern!(f64, SizedArray22, cv_Matx22d_input_array, cv_Matx22d_output_array, cv_Matx22d_input_output_array);
pub type Matx23<T> = Matx<T, SizedArray23>;
matx_extern!(f32, SizedArray23, cv_Matx23f_input_array, cv_Matx23f_output_array, cv_Matx23f_input_output_array);
matx_extern!(f64, SizedArray23, cv_Matx23d_input_array, cv_Matx23d_output_array, cv_Matx23d_input_output_array);
pub type Matx32<T> = Matx<T, SizedArray32>;
matx_extern!(f32, SizedArray32, cv_Matx32f_input_array, cv_Matx32f_output_array, cv_Matx32f_input_output_array);
matx_extern!(f64, SizedArray32, cv_Matx32d_input_array, cv_Matx32d_output_array, cv_Matx32d_input_output_array);

pub type Matx33<T> = Matx<T, SizedArray33>;
matx_extern!(f32, SizedArray33, cv_Matx33f_input_array, cv_Matx33f_output_array, cv_Matx33f_input_output_array);
matx_extern!(f64, SizedArray33, cv_Matx33d_input_array, cv_Matx33d_output_array, cv_Matx33d_input_output_array);

pub type Matx34<T> = Matx<T, SizedArray34>;
matx_extern!(f32, SizedArray34, cv_Matx34f_input_array, cv_Matx34f_output_array, cv_Matx34f_input_output_array);
matx_extern!(f64, SizedArray34, cv_Matx34d_input_array, cv_Matx34d_output_array, cv_Matx34d_input_output_array);
pub type Matx43<T> = Matx<T, SizedArray43>;
matx_extern!(f32, SizedArray43, cv_Matx43f_input_array, cv_Matx43f_output_array, cv_Matx43f_input_output_array);
matx_extern!(f64, SizedArray43, cv_Matx43d_input_array, cv_Matx43d_output_array, cv_Matx43d_input_output_array);

pub type Matx44<T> = Matx<T, SizedArray44>;
matx_extern!(f32, SizedArray44, cv_Matx44f_input_array, cv_Matx44f_output_array, cv_Matx44f_input_output_array);
matx_extern!(f64, SizedArray44, cv_Matx44d_input_array, cv_Matx44d_output_array, cv_Matx44d_input_output_array);
pub type Matx66<T> = Matx<T, SizedArray66>;
matx_extern!(f32, SizedArray66, cv_Matx66f_input_array, cv_Matx66f_output_array, cv_Matx66f_input_output_array);
matx_extern!(f64, SizedArray66, cv_Matx66d_input_array, cv_Matx66d_output_array, cv_Matx66d_input_output_array);
