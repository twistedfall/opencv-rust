use matches::assert_matches;

#[cfg(ocvrs_opencv_branch_4)]
use opencv::core::AccessFlag::ACCESS_READ;
#[cfg(not(ocvrs_opencv_branch_4))]
use opencv::core::ACCESS_READ;
use opencv::{
	core::{self, Matx12d, Scalar, ToInputArray, ToInputOutputArray, ToOutputArray, UMat, Vec2b, VecN},
	prelude::*,
	types::{VectorOff64, VectorOfu8},
	Result,
};

#[test]
fn input_array() -> Result<()> {
	let mat = Mat::new_rows_cols_with_default(10, 10, u8::opencv_type(), Scalar::all(1.))?;
	let input_array = mat.input_array()?;
	assert_eq!(Scalar::from(100.), core::sum_elems(&input_array)?);
	Ok(())
}

#[test]
fn input_output_array_types() -> Result<()> {
	assert!(Mat::default().input_array()?.is_mat()?);
	assert!(Mat::default().output_array()?.is_mat()?);
	assert!(Mat::default().input_output_array()?.is_mat()?);

	assert!(VectorOfu8::new().input_array()?.is_vector()?);
	assert!(VectorOfu8::new().output_array()?.is_vector()?);
	assert!(VectorOfu8::new().input_output_array()?.is_vector()?);

	assert!(Matx12d::default().input_array()?.is_matx()?);
	assert!(Matx12d::default().output_array()?.is_matx()?);
	assert!(Matx12d::default().input_output_array()?.is_matx()?);

	assert!(
		UMat::new_rows_cols_with_default_def(1, 1, u8::opencv_type(), Scalar::from(8.))?
			.input_array()?
			.is_umat()?
	);
	assert!(
		UMat::new_rows_cols_with_default_def(1, 1, u8::opencv_type(), Scalar::from(8.))?
			.output_array()?
			.is_umat()?
	);
	assert!(
		UMat::new_rows_cols_with_default_def(1, 1, u8::opencv_type(), Scalar::from(8.))?
			.input_output_array()?
			.is_umat()?
	);

	assert!(Scalar::default().input_array()?.is_matx()?);
	assert!(Scalar::default().output_array()?.is_matx()?);
	assert!(Scalar::default().input_output_array()?.is_matx()?);

	assert!(Vec2b::default().input_array()?.is_matx()?);
	assert!(Vec2b::default().output_array()?.is_matx()?);
	assert!(Vec2b::default().input_output_array()?.is_matx()?);

	assert!(VecN::<f64, 18>::default().input_array()?.is_matx()?);
	assert!(VecN::<f64, 18>::default().output_array()?.is_matx()?);
	assert!(VecN::<f64, 18>::default().input_output_array()?.is_matx()?);
	Ok(())
}

#[test]
fn input_output_array() -> Result<()> {
	{
		let mat_expr = Mat::ones(1, 3, u8::opencv_type())?;
		let mat = Mat::new_rows_cols_with_default(1, 3, u8::opencv_type(), Scalar::all(3.))?;
		let umat = mat.get_umat_def(ACCESS_READ)?;
		{
			let mut trg = VectorOfu8::new();
			core::add(&mat_expr, &umat, &mut trg, &core::no_array(), -1)?;
			assert_eq!(3, trg.len());
			assert_eq!(4, trg.get(0)?);
			assert_eq!(4, trg.get(1)?);
			assert_eq!(4, trg.get(2)?);
		}

		{
			let mut trg = VectorOfu8::new();
			core::add(&&mat_expr, &&umat, &mut &mut trg, &core::no_array(), -1)?;
			assert_eq!(3, trg.len());
			assert_eq!(4, trg.get(0)?);
			assert_eq!(4, trg.get(1)?);
			assert_eq!(4, trg.get(2)?);
		}
	}

	{
		let mut t = VectorOff64::new();
		core::add(&2.5, &4., &mut t, &core::no_array(), -1)?;
		assert_eq!(6.5, t.get(0)?);
	}

	{
		let mut mat = Mat::from_slice_2d(&[&[1, 2, 3, 4], &[5, 6, 7, 8], &[9, 10, 11, 12], &[13, 14, 15, 16u8]])?;
		core::complete_symm(&mut mat, false)?;
		let expected = Mat::from_slice_2d(&[&[1, 2, 3, 4], &[2, 6, 7, 8], &[3, 7, 11, 12], &[4, 8, 12, 16u8]])?;
		assert_eq!(mat.to_vec_2d::<u8>()?, expected.to_vec_2d()?);
	}

	Ok(())
}

#[test]
fn no_array() -> Result<()> {
	use self::core::no_array;

	assert!(no_array().empty()?);

	{
		let m = Mat::new_rows_cols_with_default(1, 1, u16::opencv_type(), Scalar::all(0.))?;
		assert_matches!(core::mean_std_dev(&m, &mut no_array(), &mut no_array(), &no_array()), Ok(()));
	}
	Ok(())
}
