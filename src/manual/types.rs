pub use abstract_ref::*;

use crate::{input_output_array, types};

mod abstract_ref;

#[doc(hidden)]
#[repr(C)]
/// Needed to prevent the following error:
/// `extern` block uses type `()`, which is not FFI-safe
/// That warning is not triggered in rust 1.74.0, so drop when MSRV is high enough
pub struct Unit([u8; 0]);

impl From<Unit> for () {
	fn from(_: Unit) -> Self {}
}

input_output_array! { types::VectorOfMat, from_mat_vec, from_mat_vec_mut }
input_output_array! { types::VectorOfUMat, from_umat_vec, from_umat_vec_mut }
