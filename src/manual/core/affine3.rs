use std::fmt;

use num_traits::{One, Zero};

use crate::core::{Matx44, MatxTrait};
use crate::opencv_type_simple_generic;

/// [docs.opencv.org](https://docs.opencv.org/master/dd/d99/classcv_1_1Affine3.html)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Affine3<T: Copy> {
	pub matrix: Matx44<T>,
}

impl<T: Copy + Zero + One> Default for Affine3<T> {
	fn default() -> Self {
		Self { matrix: Matx44::eye() }
	}
}

impl<T: Copy + fmt::Debug> fmt::Debug for Affine3<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let rows_cols_data = [
			&self.matrix.val[0..4],
			&self.matrix.val[4..8],
			&self.matrix.val[8..12],
			&self.matrix.val[12..16],
		];
		f.debug_tuple("Affine3").field(&rows_cols_data).finish()
	}
}

opencv_type_simple_generic! { Affine3<Copy> }
