use opencv::boxed_ref::{BoxedRef, BoxedRefMut};
use opencv::core::{no_array, Rect, Vec2i, Vector};
use opencv::prelude::*;
use opencv::{core, Result};

#[test]
fn boxed_ref_from_roi() -> Result<()> {
	let mut mat = Mat::from_slice(&[1, 2, 3, 4])?.try_clone()?;

	// const
	{
		let boxed_ref: BoxedRef<Mat> = mat.roi(Rect::new(1, 0, 2, 1))?;
		assert_eq!(&[2, 3], boxed_ref.data_typed::<i32>()?);

		let more_boxed_ref: BoxedRef<Mat> = boxed_ref.roi(Rect::new(1, 0, 1, 1))?;
		assert_eq!(&[3], more_boxed_ref.data_typed::<i32>()?);
	}

	// mut
	{
		let mut boxed_ref_mut: BoxedRefMut<Mat> = mat.roi_mut(Rect::new(1, 0, 2, 1))?;
		assert_eq!(&[2, 3], boxed_ref_mut.data_typed::<i32>()?);

		boxed_ref_mut.data_typed_mut::<i32>()?.copy_from_slice(&[4, 5]);

		let mut more_boxed_ref_mut: BoxedRefMut<Mat> = boxed_ref_mut.roi_mut(Rect::new(1, 0, 1, 1))?;
		assert_eq!(&[5], more_boxed_ref_mut.data_typed::<i32>()?);

		*more_boxed_ref_mut.at_mut::<i32>(0)? = 6;
	}

	assert_eq!(&[1, 4, 6, 4], mat.data_typed::<i32>()?);

	Ok(())
}

#[test]
fn boxed_ref_clone_pointee() -> Result<()> {
	let mut mat = Mat::from_slice(&[1, 2, 3, 4])?.try_clone()?;

	let boxed_ref: BoxedRefMut<Mat> = mat.roi_mut(Rect::new(1, 0, 2, 1))?;
	let mut mat2: Mat = boxed_ref.clone_pointee();
	mat2.data_typed_mut::<i32>()?.copy_from_slice(&[4, 5]);
	assert_eq!(&[4, 5], mat2.data_typed::<i32>()?);
	assert_eq!(&[1, 2, 3, 4], mat.data_typed::<i32>()?);

	Ok(())
}

#[test]
fn boxed_ref_pass() -> Result<()> {
	let mat = Mat::from_slice(&[1, 2, 3, 4])?.try_clone()?;

	// pass as InputArray
	{
		let boxed_ref: BoxedRef<Mat> = mat.roi(Rect::new(1, 0, 2, 1))?;
		let mut min = 0.;
		let mut max = 0.;
		core::min_max_loc(&boxed_ref, Some(&mut min), Some(&mut max), None, None, &no_array())?;
		assert_eq!(2., min);
		assert_eq!(3., max);
	}

	// pass as OutputArray
	{
		let mut mat = mat.clone();
		let mut boxed_ref: BoxedRefMut<Mat> = mat.roi_mut(Rect::new(1, 0, 2, 1))?;
		let src = Mat::from_slice(&[5, 6])?;
		src.copy_to(&mut boxed_ref)?;
		assert_eq!(&[1, 5, 6, 4], mat.data_typed::<i32>()?);
	}

	// pass as InputOutputArray
	{
		let mut mat = Mat::from_slice_2d(&[[1, 2], [3, 4], [5, 6]])?;
		let mut boxed_ref: BoxedRefMut<Mat> = mat.roi_mut(Rect::new(0, 1, 2, 2))?;
		core::complete_symm_def(&mut boxed_ref)?;
		assert_eq!(
			mat.data_typed::<i32>()?,
			Mat::from_slice_2d(&[[1, 2], [3, 4], [4, 6]])?.data_typed()?
		);
	}

	// pass as const Mat
	{
		let mat2 = Mat::from_slice(&[4, 3, 2, 1])?;
		let boxed_ref: BoxedRef<Mat> = mat.roi(Rect::new(0, 0, 4, 1))?;
		let mut out = Mat::default();
		core::min_mat_to(&boxed_ref, &mat2, &mut out)?;
		assert_eq!(&[1, 2, 2, 1], out.data_typed::<i32>()?);
	}

	// pass as mut Mat
	{
		let mat2 = Mat::from_slice(&[4, 3, 2, 1])?;
		let mut out = Mat::from_slice(&[0, 0, 0, 0])?.try_clone()?;
		let mut boxed_ref: BoxedRefMut<Mat> = out.roi_mut(Rect::new(0, 0, 4, 1))?;
		core::min_mat_to(&mat, &mat2, &mut boxed_ref)?;
		assert_eq!(&[1, 2, 2, 1], out.data_typed::<i32>()?);
	}

	Ok(())
}

#[test]
fn vector_boxed_ref() -> Result<()> {
	let src = Mat::from_slice(&[1, 2, 3, 4])?;

	let roi1 = src.roi(Rect::new(0, 0, 2, 1))?;
	let roi2 = src.roi(Rect::new(2, 0, 2, 1))?;
	let mut roi_vec = Vector::<BoxedRef<Mat>>::new();
	roi_vec.push(roi1);
	roi_vec.push(roi2);

	let mut dst = Mat::default();
	core::merge(&&roi_vec, &mut dst)?;

	assert_eq!(2, dst.channels());
	assert_eq!(2, dst.cols());
	assert_eq!(1, dst.rows());
	assert_eq!([Vec2i::from([1, 3]), Vec2i::from([2, 4])], dst.data_typed::<Vec2i>()?);

	Ok(())
}
