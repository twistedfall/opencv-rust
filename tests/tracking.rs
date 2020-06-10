#![cfg(feature = "contrib")]

use opencv::{
	prelude::*,
	Result,
	tracking::TrackerSamplerPF_Params,
};
use opencv::core::Scalar;

#[test]
fn typed_mat() -> Result<()> {
	let mut params = TrackerSamplerPF_Params::default()?;
	assert_eq!(&[15., 15., 15., 15.], params.std().data_typed()?);
	let mat = Mat::new_rows_cols_with_default(1, 4, f64::typ(), Scalar::all(8.))?.try_into_typed::<f64>()?;
	let mat_src = mat.clone()?.try_into_typed::<f64>()?;
	params.set_std(mat);
	let mat = params.std();
	assert_eq!(mat.size()?, mat_src.size()?);
	assert_eq!(mat.data_typed()?, mat_src.data_typed()?);
	Ok(())
}
