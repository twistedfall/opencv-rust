#[cfg(ocvrs_opencv_branch_4)]
use opencv::core::AccessFlag::ACCESS_READ;
#[cfg(not(ocvrs_opencv_branch_4))]
use opencv::core::ACCESS_READ;
use opencv::{
	core::{Rect, Size, UMat, UMatUsageFlags, Vec3d},
	prelude::*,
	types::VectorOfi32,
	Result,
};

#[test]
fn umat_default() -> Result<()> {
	let mat = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
	assert_eq!(u8::opencv_type(), mat.typ());
	assert_eq!(u8::opencv_depth(), mat.depth());
	assert_eq!(u8::opencv_channels(), mat.channels());
	assert_eq!(Size::new(0, 0), mat.size()?);
	assert_eq!(0, mat.dims());
	Ok(())
}

#[test]
fn umat_create() -> Result<()> {
	let mut mat = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
	unsafe { mat.create_rows_cols(10, 10, u16::opencv_type(), UMatUsageFlags::USAGE_DEFAULT)? };
	assert_eq!(Size::new(10, 10), mat.size()?);
	assert_eq!(2, mat.dims());
	Ok(())
}

#[test]
fn umat_to_mat() -> Result<()> {
	{
		let mut vec = VectorOfi32::new();
		vec.push(1);
		vec.push(2);
		vec.push(3);
		let mat = Mat::from_exact_iter(vec.into_iter())?;
		let umat = mat.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
		assert_eq!(3, umat.rows());
		assert_eq!(1, umat.cols());
		assert_eq!(i32::opencv_type(), umat.typ());
		let mat = umat.get_mat(ACCESS_READ)?;
		assert_eq!(1, *mat.at_2d::<i32>(0, 0)?);
		assert_eq!(2, *mat.at_2d::<i32>(1, 0)?);
		assert_eq!(3, *mat.at_2d::<i32>(2, 0)?);
	}

	{
		let vec: Vec<i32> = vec![1, 2, 3];
		let mat = Mat::from_exact_iter(vec.into_iter())?;
		let umat = mat.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
		assert_eq!(3, umat.rows());
		assert_eq!(1, umat.cols());
		assert_eq!(i32::opencv_type(), umat.typ());
		let mat = umat.get_mat(ACCESS_READ)?;
		assert_eq!(1, *mat.at_2d::<i32>(0, 0)?);
		assert_eq!(2, *mat.at_2d::<i32>(1, 0)?);
		assert_eq!(3, *mat.at_2d::<i32>(2, 0)?);
	}
	Ok(())
}

#[test]
fn umat_for_rows_and_cols() -> Result<()> {
	let mat = unsafe { UMat::new_rows_cols(400, 300, Vec3d::opencv_type(), UMatUsageFlags::USAGE_DEFAULT) }?;
	assert_eq!(Vec3d::opencv_type(), mat.typ());
	assert_eq!(Vec3d::opencv_depth(), mat.depth());
	assert_eq!(Vec3d::opencv_channels(), mat.channels());
	assert!(mat.is_continuous());
	assert!(!mat.is_submatrix());
	assert_eq!(Size::new(300, 400), mat.size()?);
	assert_eq!(400, mat.rows());
	assert_eq!(300, mat.cols());
	assert_eq!(2, mat.mat_size().len());
	assert_eq!(400, mat.mat_size()[0]);
	assert_eq!(300, mat.mat_size()[1]);
	assert_eq!(2, mat.dims());
	assert_eq!(2, mat.mat_step().len());
	assert_eq!(7200, mat.mat_step()[0]);
	assert_eq!(24, mat.mat_step()[1]);
	assert_eq!(24, mat.elem_size()?);
	assert_eq!(8, mat.elem_size1());
	assert_eq!(900, mat.step1(0)?);
	assert_eq!(3, mat.step1(1)?);
	assert_eq!(120000, mat.total());
	Ok(())
}

#[test]
fn umat_continuous() -> Result<()> {
	let s: Vec<Vec<f32>> = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];

	let mat = Mat::from_slice_2d(&s)?;
	let umat = mat.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;

	{
		let sub_umat_non_cont = UMat::roi(&umat, Rect::new(1, 1, 2, 2))?;
		assert_eq!(umat.typ(), sub_umat_non_cont.typ());
		assert_eq!(2, sub_umat_non_cont.rows());
		assert_eq!(2, sub_umat_non_cont.cols());
		assert!(sub_umat_non_cont.is_submatrix());
		assert!(!sub_umat_non_cont.is_continuous());

		let umat_clone = sub_umat_non_cont.try_clone()?;
		assert_eq!(umat.typ(), umat_clone.typ());
		assert_eq!(2, umat_clone.rows());
		assert_eq!(2, umat_clone.cols());
		assert!(!umat_clone.is_submatrix());
		assert!(umat_clone.is_continuous());
	}

	{
		let sub_umat_cont = UMat::roi(&umat, Rect::new(0, 1, 3, 2))?;
		assert_eq!(umat.typ(), sub_umat_cont.typ());
		assert_eq!(2, sub_umat_cont.rows());
		assert_eq!(3, sub_umat_cont.cols());
		assert!(sub_umat_cont.is_submatrix());
		assert!(sub_umat_cont.is_continuous());
	}

	Ok(())
}
