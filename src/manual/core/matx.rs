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

		impl<T: $type_trait> From<[T; $rows * $cols]> for $type<T> {
			fn from(s: [T; $rows * $cols]) -> Self {
				Self { val: s }
			}
		}

		impl<T: $type_trait> $type<T> {
			#![allow(non_upper_case_globals)]
			pub const rows: usize = $rows;
			pub const cols: usize = $cols;
			pub const channels: usize = $rows * $cols;
		}

		impl<T: $type_trait> $crate::core::MatxTrait for $type<T> {
			type ElemType = T;

			#[inline(always)] fn val(&self) -> &[Self::ElemType] { &self.val }
			#[inline(always)] fn val_mut(&mut self) -> &mut [Self::ElemType] { &mut self.val }
			#[inline(always)] fn rows() -> usize { Self::rows }
			#[inline(always)] fn cols() -> usize { Self::cols }
			#[inline(always)] fn channels() -> usize { Self::channels }

			fn all(alpha: T) -> Self {
				Self::from([alpha; $rows * $cols])
			}
		}

		impl<T: $type_trait> std::default::Default for $type<T> {
			fn default() -> Self {
				Self::from([T::default(); $rows * $cols])
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
	};
}

matx_impl!(Matx12, 1, 2, ValidMatxType);
matx_impl!(Matx13, 1, 3, ValidMatxType);
matx_impl!(Matx14, 1, 4, ValidMatxType);
matx_impl!(Matx16, 1, 6, ValidMatxType);

matx_impl!(Matx21, 2, 1, ValidMatxType);
matx_impl!(Matx31, 3, 1, ValidMatxType);
matx_impl!(Matx41, 4, 1, ValidMatxType);
matx_impl!(Matx61, 6, 1, ValidMatxType);

matx_impl!(Matx22, 2, 2, ValidMatxType);
matx_impl!(Matx23, 2, 3, ValidMatxType);
matx_impl!(Matx32, 3, 2, ValidMatxType);

matx_impl!(Matx33, 3, 3, ValidMatxType);

matx_impl!(Matx34, 3, 4, ValidMatxType);
matx_impl!(Matx43, 4, 3, ValidMatxType);

matx_impl!(Matx44, 4, 4, ValidMatxType);
matx_impl!(Matx66, 6, 6, ValidMatxType);

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

pub trait MatxTrait: Default {
	type ElemType: ValidMatxType;

	fn rows() -> usize;
	fn cols() -> usize;
	fn val(&self) -> &[Self::ElemType];
	fn val_mut(&mut self) -> &mut [Self::ElemType];

	fn all(alpha: Self::ElemType) -> Self where Self: Sized;

	#[inline(always)]
	fn channels() -> usize {
		Self::rows() * Self::cols()
	}

	#[inline(always)]
	fn shortdim() -> usize {
		Self::rows().min(Self::cols())
	}

	fn zeros() -> Self where Self: Sized, Self::ElemType: Zero {
		Self::all(Zero::zero())
	}

	fn ones() -> Self where Self: Sized, Self::ElemType: One {
		Self::all(One::one())
	}

	fn get(&self, idx: (usize, usize)) -> Option<&Self::ElemType> {
		index_check(idx, Self::rows(), Self::cols())
			.ok()
			.map(|_| unsafe { self.get_unchecked(idx) })
	}

	unsafe fn get_unchecked(&self, idx: (usize, usize)) -> &Self::ElemType {
		self.val().get_unchecked(idx.0 * Self::cols() + idx.1)
	}

	fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut Self::ElemType> {
		index_check(idx, Self::rows(), Self::cols())
			.ok()?;
		Some(unsafe { self.get_unchecked_mut(idx) })
	}

	unsafe fn get_unchecked_mut(&mut self, idx: (usize, usize)) -> &mut Self::ElemType {
		let cols = Self::cols();
		self.val_mut().get_unchecked_mut(idx.0 * cols + idx.1)
	}

	fn eye() -> Self where Self: Sized, Self::ElemType: One {
		let mut out = Self::default();
		(0..Self::shortdim()).for_each(|i| {
			unsafe { out.get_unchecked_mut((i, i)) }.set_one();
		});
		out
	}
}

