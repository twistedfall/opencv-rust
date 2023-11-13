pub use abstract_ref::*;

use crate::{input_output_array, types};

mod abstract_ref;

#[doc(hidden)]
#[repr(C)]
/// needed because layout of () in repr(C) is not guaranteed
pub struct Unit([u8; 0]);

impl From<Unit> for () {
	fn from(_: Unit) -> Self {}
}

input_output_array! { types::VectorOfMat, from_mat_vec, from_mat_vec_mut }

input_output_array! { types::VectorOfUMat, from_umat_vec, from_umat_vec_mut }
