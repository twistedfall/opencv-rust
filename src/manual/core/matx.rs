use num::{One, Zero};

use crate::{core, Error, Result};

valid_types!(ValidMatxType, f32, f64);

macro_rules! matx_impl {
	($type: ident, $rows: expr, $cols: expr, $type_trait: ident) => {
		/// [docs.opencv.org](https://docs.opencv.org/master/de/de1/classcv_1_1Matx.html)
		#[repr(C)]
		#[derive(Copy, Clone)]
		pub struct $type<T: $type_trait> {
			pub val: [T; $rows * $cols],
		}

		impl<T: $type_trait> $type<T> {
			pub const ROWS: usize = $rows;
			pub const COLS: usize = $cols;
			pub const CHANNELS: usize = $rows * $cols;
		}

		impl<T: $type_trait> $crate::core::MatxTrait for $type<T> {
			type ElemType = T;

			#[inline(always)] fn val(&self) -> &[Self::ElemType] { &self.val }
			#[inline(always)] fn val_mut(&mut self) -> &mut [Self::ElemType] { &mut self.val }
			#[inline(always)] fn rows(&self) -> usize { Self::ROWS }
			#[inline(always)] fn cols(&self) -> usize { Self::COLS }
			#[inline(always)] fn channels(&self) -> usize { Self::CHANNELS }

			fn all(alpha: Self::ElemType) -> Self {
				Self::from([alpha; $rows * $cols])
			}
		}

		impl<T: $type_trait> From<[T; $rows * $cols]> for $type<T> {
			fn from(s: [T; $rows * $cols]) -> Self {
				Self { val: s }
			}
		}

		impl<T: $type_trait> std::default::Default for $type<T> {
			fn default() -> Self {
				Self::all(T::default())
			}
		}

		impl<T: $type_trait> std::ops::Index<(usize, usize)> for $type<T> {
			type Output = T;

			fn index(&self, index: (usize, usize)) -> &Self::Output {
				self.get(index).expect("Index out of range")
			}
		}

		impl<T: $type_trait> std::ops::IndexMut<(usize, usize)> for $type<T> {
			fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
				self.get_mut(index).expect("Index out of range")
			}
		}

		impl<T: $type_trait> std::cmp::PartialEq for $type<T> {
			fn eq(&self, other: &$type<T>) -> bool {
				&self.val[..] == &other.val[..]
			}
		}

		impl<T: $type_trait> std::fmt::Debug for $type<T> {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct(stringify!($type))
				.field("rows", &self.rows())
				.field("cols", &self.cols())
				.field("channels", &self.channels())
				.field("shortdim", &self.shortdim())
				.finish()
			}
		}
	};
}

macro_rules! matx_input_output_array_impl {
	($type: ty, $extern_input: ident, $extern_output: ident, $extern_input_output: ident) => {
		impl $crate::core::ToInputArray for $type {
			fn input_array(&self) -> $crate::Result<$crate::core::_InputArray> {
				extern "C" { fn $extern_input(instance: *const $type) -> $crate::sys::Result<*mut std::ffi::c_void>; }
				unsafe { $extern_input(self) }
					.into_result()
					.map(|ptr| $crate::core::_InputArray { ptr })
			}
		}

		impl $crate::core::ToInputArray for &$type {
			#[inline]
			fn input_array(&self) -> $crate::Result<$crate::core::_InputArray> {
				(*self).input_array()
			}
		}

		impl $crate::core::ToOutputArray for $type {
				fn output_array(&mut self) -> $crate::Result<$crate::core::_OutputArray> {
				extern "C" { fn $extern_output(instance: *mut $type) -> $crate::sys::Result<*mut std::ffi::c_void>; }
				unsafe { $extern_output(self) }
					.into_result()
					.map(|ptr| $crate::core::_OutputArray { ptr })
			}
		}

		impl $crate::core::ToOutputArray for &mut $type {
			#[inline]
			fn output_array(&mut self) -> $crate::Result<$crate::core::_OutputArray> {
				(*self).output_array()
			}
		}

		impl $crate::core::ToInputOutputArray for $type {
				fn input_output_array(&mut self) -> $crate::Result<$crate::core::_InputOutputArray> {
				extern "C" { fn $extern_input_output(instance: *mut $type) -> $crate::sys::Result<*mut std::ffi::c_void>; }
				unsafe { $extern_input_output(self) }
					.into_result()
					.map(|ptr| $crate::core::_InputOutputArray { ptr })
			}
		}

		impl $crate::core::ToInputOutputArray for &mut $type {
			#[inline]
			fn input_output_array(&mut self) -> $crate::Result<$crate::core::_InputOutputArray> {
				(*self).input_output_array()
			}
		}
	}
}

matx_impl!(Matx12, 1, 2, ValidMatxType);
matx_input_output_array_impl!(Matx12<f32>, cv_Matx12f_input_array, cv_Matx12f_output_array, cv_Matx12f_input_output_array);
matx_input_output_array_impl!(Matx12<f64>, cv_Matx12d_input_array, cv_Matx12d_output_array, cv_Matx12d_input_output_array);
matx_impl!(Matx13, 1, 3, ValidMatxType);
matx_input_output_array_impl!(Matx13<f32>, cv_Matx13f_input_array, cv_Matx13f_output_array, cv_Matx13f_input_output_array);
matx_input_output_array_impl!(Matx13<f64>, cv_Matx13d_input_array, cv_Matx13d_output_array, cv_Matx13d_input_output_array);
matx_impl!(Matx14, 1, 4, ValidMatxType);
matx_input_output_array_impl!(Matx14<f32>, cv_Matx14f_input_array, cv_Matx14f_output_array, cv_Matx14f_input_output_array);
matx_input_output_array_impl!(Matx14<f64>, cv_Matx14d_input_array, cv_Matx14d_output_array, cv_Matx14d_input_output_array);
matx_impl!(Matx16, 1, 6, ValidMatxType);
matx_input_output_array_impl!(Matx16<f32>, cv_Matx16f_input_array, cv_Matx16f_output_array, cv_Matx16f_input_output_array);
matx_input_output_array_impl!(Matx16<f64>, cv_Matx16d_input_array, cv_Matx16d_output_array, cv_Matx16d_input_output_array);

matx_impl!(Matx21, 2, 1, ValidMatxType);
matx_input_output_array_impl!(Matx21<f32>, cv_Matx21f_input_array, cv_Matx21f_output_array, cv_Matx21f_input_output_array);
matx_input_output_array_impl!(Matx21<f64>, cv_Matx21d_input_array, cv_Matx21d_output_array, cv_Matx21d_input_output_array);
matx_impl!(Matx31, 3, 1, ValidMatxType);
matx_input_output_array_impl!(Matx31<f32>, cv_Matx31f_input_array, cv_Matx31f_output_array, cv_Matx31f_input_output_array);
matx_input_output_array_impl!(Matx31<f64>, cv_Matx31d_input_array, cv_Matx31d_output_array, cv_Matx31d_input_output_array);
matx_impl!(Matx41, 4, 1, ValidMatxType);
matx_input_output_array_impl!(Matx41<f32>, cv_Matx41f_input_array, cv_Matx41f_output_array, cv_Matx41f_input_output_array);
matx_input_output_array_impl!(Matx41<f64>, cv_Matx41d_input_array, cv_Matx41d_output_array, cv_Matx41d_input_output_array);
matx_impl!(Matx61, 6, 1, ValidMatxType);
matx_input_output_array_impl!(Matx61<f32>, cv_Matx61f_input_array, cv_Matx61f_output_array, cv_Matx61f_input_output_array);
matx_input_output_array_impl!(Matx61<f64>, cv_Matx61d_input_array, cv_Matx61d_output_array, cv_Matx61d_input_output_array);

matx_impl!(Matx22, 2, 2, ValidMatxType);
matx_input_output_array_impl!(Matx22<f32>, cv_Matx22f_input_array, cv_Matx22f_output_array, cv_Matx22f_input_output_array);
matx_input_output_array_impl!(Matx22<f64>, cv_Matx22d_input_array, cv_Matx22d_output_array, cv_Matx22d_input_output_array);
matx_impl!(Matx23, 2, 3, ValidMatxType);
matx_input_output_array_impl!(Matx23<f32>, cv_Matx23f_input_array, cv_Matx23f_output_array, cv_Matx23f_input_output_array);
matx_input_output_array_impl!(Matx23<f64>, cv_Matx23d_input_array, cv_Matx23d_output_array, cv_Matx23d_input_output_array);
matx_impl!(Matx32, 3, 2, ValidMatxType);
matx_input_output_array_impl!(Matx32<f32>, cv_Matx32f_input_array, cv_Matx32f_output_array, cv_Matx32f_input_output_array);
matx_input_output_array_impl!(Matx32<f64>, cv_Matx32d_input_array, cv_Matx32d_output_array, cv_Matx32d_input_output_array);

matx_impl!(Matx33, 3, 3, ValidMatxType);
matx_input_output_array_impl!(Matx33<f32>, cv_Matx33f_input_array, cv_Matx33f_output_array, cv_Matx33f_input_output_array);
matx_input_output_array_impl!(Matx33<f64>, cv_Matx33d_input_array, cv_Matx33d_output_array, cv_Matx33d_input_output_array);

matx_impl!(Matx34, 3, 4, ValidMatxType);
matx_input_output_array_impl!(Matx34<f32>, cv_Matx34f_input_array, cv_Matx34f_output_array, cv_Matx34f_input_output_array);
matx_input_output_array_impl!(Matx34<f64>, cv_Matx34d_input_array, cv_Matx34d_output_array, cv_Matx34d_input_output_array);
matx_impl!(Matx43, 4, 3, ValidMatxType);
matx_input_output_array_impl!(Matx43<f32>, cv_Matx43f_input_array, cv_Matx43f_output_array, cv_Matx43f_input_output_array);
matx_input_output_array_impl!(Matx43<f64>, cv_Matx43d_input_array, cv_Matx43d_output_array, cv_Matx43d_input_output_array);

matx_impl!(Matx44, 4, 4, ValidMatxType);
matx_input_output_array_impl!(Matx44<f32>, cv_Matx44f_input_array, cv_Matx44f_output_array, cv_Matx44f_input_output_array);
matx_input_output_array_impl!(Matx44<f64>, cv_Matx44d_input_array, cv_Matx44d_output_array, cv_Matx44d_input_output_array);
matx_impl!(Matx66, 6, 6, ValidMatxType);
matx_input_output_array_impl!(Matx66<f32>, cv_Matx66f_input_array, cv_Matx66f_output_array, cv_Matx66f_input_output_array);
matx_input_output_array_impl!(Matx66<f64>, cv_Matx66d_input_array, cv_Matx66d_output_array, cv_Matx66d_input_output_array);

#[inline(always)]
fn index_check(idx: (usize, usize), rows: usize, cols: usize) -> Result<()> {
	if idx.0 >= rows {
		Err(Error::new(core::StsOutOfRange, format!("Index: {} along dimension: rows out of bounds 0..{}", idx.0, rows)))
	} else if idx.1 >= cols {
		Err(Error::new(core::StsOutOfRange, format!("Index: {} along dimension: cols out of bounds 0..{}", idx.1, cols)))
	} else {
		Ok(())
	}
}

pub trait MatxTrait {
	type ElemType: ValidMatxType;

	fn rows(&self) -> usize;
	fn cols(&self) -> usize;
	fn val(&self) -> &[Self::ElemType];
	fn val_mut(&mut self) -> &mut [Self::ElemType];

	fn all(alpha: Self::ElemType) -> Self where Self: Sized;

	#[inline(always)]
	fn channels(&self) -> usize {
		self.rows() * self.cols()
	}

	#[inline(always)]
	fn shortdim(&self) -> usize {
		self.rows().min(self.cols())
	}

	fn zeros() -> Self where Self: Sized {
		Self::all(Self::ElemType::zero())
	}

	fn ones() -> Self where Self: Sized {
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

	fn eye() -> Self where Self: Sized {
		let mut out = Self::zeros();
		(0..out.shortdim()).for_each(|i| {
			unsafe { out.get_unchecked_mut((i, i)) }.set_one();
		});
		out
	}
}
