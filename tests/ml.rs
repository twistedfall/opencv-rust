#![cfg(ocvrs_has_module_ml)]

use opencv::{
	core::{Scalar, Size},
	ml,
	prelude::*,
	Result,
};

#[test]
fn knn() -> Result<()> {
	let mut knn = <dyn KNearest>::create()?;
	assert!(StatModelConst::empty(&knn)?);
	let samp = Mat::new_rows_cols_with_default(1, 1, f32::typ(), Scalar::all(1.))?;
	let resp = Mat::new_rows_cols_with_default(1, 1, f32::typ(), Scalar::all(2.))?;
	knn.train(&samp, ml::ROW_SAMPLE, &resp)?;
	let mut resp = Mat::default();
	let mut neigh = Mat::default();
	let mut dist = Mat::default();
	knn.find_nearest(&samp, 3, &mut resp, &mut neigh, &mut dist)?;
	assert_eq!(2., *resp.at_2d::<f32>(0, 0)?);
	assert_eq!(Size::new(1, 1), resp.size()?);
	let width = if cfg!(ocvrs_opencv_branch_32) { 3 } else { 1 };
	assert_eq!(2., *neigh.at_2d::<f32>(0, 0)?);
	assert_eq!(Size::new(width, 1), neigh.size()?);
	assert_eq!(0., *dist.at_2d::<f32>(0, 0)?);
	assert_eq!(Size::new(width, 1), dist.size()?);
	Ok(())
}
