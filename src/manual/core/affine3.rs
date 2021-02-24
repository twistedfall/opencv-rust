use crate::{
	core::{Matx44, MatxTrait, ValidMatxType},
	opencv_type_simple_generic,
};

/// [docs.opencv.org](https://docs.opencv.org/master/dd/d99/classcv_1_1Affine3.html)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Affine3<T: ValidMatxType> {
	pub matrix: Matx44<T>,
}

impl<T: ValidMatxType> Default for Affine3<T> {
	fn default() -> Self {
		Self { matrix: Matx44::eye() }
	}
}

opencv_type_simple_generic! { Affine3<ValidMatxType> }
