use std::ffi::c_void;
use std::mem;

use matches::assert_matches;

use opencv::core::{MatConstIterator, MatIter, Point, Point2d, Rect, Scalar, Size, Vec2b, Vec2s, Vec3d, Vec3f, Vec4w, Vector};
use opencv::prelude::*;
use opencv::{core, imgproc, Error, Result};
const PIXEL: &[u8] = include_bytes!("pixel.png");

#[test]
fn mat_default() -> Result<()> {
	let mat = Mat::default();
	assert_eq!(u8::opencv_type(), mat.typ());
	assert_eq!(u8::opencv_depth(), mat.depth());
	assert_eq!(u8::opencv_channels(), mat.channels());
	assert_eq!(Size::new(0, 0), mat.size()?);
	assert_eq!(0, mat.dims());
	assert!(!mat.is_allocated());
	assert!(mat.data().is_null());
	Ok(())
}

#[test]
fn mat_create() -> Result<()> {
	let mut mat = Mat::default();
	unsafe { mat.create_rows_cols(10, 10, u16::opencv_type())? };
	assert!(mat.is_allocated());
	assert!(!mat.data().is_null());
	mat.set_scalar(7.into())?;
	assert_eq!(7, *mat.at_2d::<u16>(0, 0)?);
	assert_eq!(7, *mat.at_2d::<u16>(3, 3)?);
	assert_eq!(7, *mat.at_2d::<u16>(9, 9)?);
	unsafe { mat.release()? };
	assert!(!mat.is_allocated());
	assert!(mat.data().is_null());
	assert_eq!(Size::new(0, 0), mat.size()?);
	Ok(())
}

#[test]
fn mat_from_iter() -> Result<()> {
	{
		let mut vec = Vector::<i32>::new();
		vec.push(1);
		vec.push(2);
		vec.push(3);
		let mat = Mat::from_exact_iter(vec.into_iter())?;
		assert_eq!(3, mat.rows());
		assert_eq!(1, mat.cols());
		assert_eq!(i32::opencv_type(), mat.typ());
		assert_eq!(1, *mat.at_2d::<i32>(0, 0)?);
		assert_eq!(2, *mat.at_2d::<i32>(1, 0)?);
		assert_eq!(3, *mat.at_2d::<i32>(2, 0)?);
	}

	{
		let vec: Vec<i32> = vec![1, 2, 3];
		let mat = Mat::from_exact_iter(vec.into_iter())?;
		assert_eq!(3, mat.rows());
		assert_eq!(1, mat.cols());
		assert_eq!(i32::opencv_type(), mat.typ());
		assert_eq!(1, *mat.at_2d::<i32>(0, 0)?);
		assert_eq!(2, *mat.at_2d::<i32>(1, 0)?);
		assert_eq!(3, *mat.at_2d::<i32>(2, 0)?);
	}
	Ok(())
}

#[test]
fn mat_for_rows_and_cols() -> Result<()> {
	let mat = unsafe { Mat::new_rows_cols(400, 300, Vec3d::opencv_type()) }?;
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
	assert_eq!(2, mat.mat_step().buf().len());
	assert_eq!(7200, mat.mat_step().buf()[0]);
	assert_eq!(24, mat.mat_step().buf()[1]);
	assert_eq!(24, mat.elem_size()?);
	assert_eq!(8, mat.elem_size1());
	assert_eq!(900, mat.step1(0)?);
	assert_eq!(3, mat.step1(1)?);
	assert_eq!(120000, mat.total());
	Ok(())
}

#[test]
fn mat_nd() -> Result<()> {
	{
		let mut mat = Mat::new_nd_with_default(&[3, 3, 3], Vec4w::opencv_type(), 0.into())?;
		assert_eq!(&Vec4w::new(0, 0, 0, 0), mat.at::<Vec4w>(1)?);
		assert_matches!(
			mat.at::<Vec4w>(27),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_eq!(0, mat.at_3d::<Vec4w>(1, 1, 1)?[0]);
		*mat.at_3d_mut::<Vec4w>(1, 1, 1)? = Vec4w::all(10);
		assert_eq!(10, mat.at_3d::<Vec4w>(1, 1, 1)?[0]);
		assert_eq!(0, mat.at_3d::<Vec4w>(1, 1, 2)?[2]);
		assert_eq!(3, mat.dims());
		assert_eq!([3, 3, 3], *mat.mat_size());
	}

	{
		let dims = Vector::<i32>::from_iter(vec![2, 3, 4, 5, 6, 7]);
		let mut mat = Mat::new_nd_vec_with_default(&dims, Vec4w::opencv_type(), 0.into())?;
		assert_eq!(-1, mat.rows());
		assert_eq!(-1, mat.cols());
		assert_matches!(
			mat.at_nd::<Vec4w>(&[1, 1, 1]),
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
		assert_matches!(
			mat.at_nd::<Vec4w>(&[1, 1, 1, 1, 1, 1, 1]),
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
		assert_matches!(
			mat.at_nd::<Vec4w>(&[10, 10, 10, 10, 10, 10]),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_nd::<Vec4w>(&[-1, 10, 10, 10, 10, 10]),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_nd::<Vec4w>(&[2, 3, 4, 5, 10, 10]),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_nd::<Vec4w>(&[2, 3, 4, 5, 6, 10]),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_eq!(Vec4w::default(), *mat.at_nd::<Vec4w>(&[1, 2, 3, 4, 5, 6])?);
		*mat.at_nd_mut::<Vec4w>(&[1, 2, 3, 4, 5, 6])? = Vec4w::from([5, 6, 7, 8]);
		assert_eq!(Vec4w::from([5, 6, 7, 8]), *mat.at_nd::<Vec4w>(&[1, 2, 3, 4, 5, 6])?);
	}
	Ok(())
}

#[test]
fn mat_at_1d() -> Result<()> {
	let s: Vec<Vec<f32>> = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];

	{
		let mut mat = Mat::from_slice_2d(&s)?;
		let mut mat = mat.reshape_mut(1, 1)?;
		assert_eq!(1, mat.rows());
		assert_eq!(9, mat.cols());
		assert_matches!(
			mat.at::<f32>(-1),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at::<f32>(10),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_mut::<f32>(-1),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_mut::<f32>(10),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_eq!(*mat.at::<f32>(0)?, 1.);
		assert_eq!(*mat.at::<f32>(5)?, 6.);
		assert_eq!(*mat.at::<f32>(8)?, 9.);
		*mat.at_mut::<f32>(4)? = 2.;
		assert_eq!(*mat.at::<f32>(4)?, 2.);
	}

	{
		let mut mat = Mat::from_slice_2d(&s)?;
		let mut mat = mat.reshape_mut(1, 9)?;
		assert_eq!(9, mat.rows());
		assert_eq!(1, mat.cols());
		assert_matches!(
			mat.at::<f32>(-1),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at::<f32>(10),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_mut::<f32>(-1),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_mut::<f32>(10),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_eq!(*mat.at::<f32>(0)?, 1.);
		assert_eq!(*mat.at::<f32>(4)?, 5.);
		assert_eq!(*mat.at::<f32>(8)?, 9.);
		*mat.at_mut::<f32>(4)? = 2.;
		assert_eq!(*mat.at::<f32>(4)?, 2.);
	}

	{
		let mut mat = Mat::from_slice_2d(&s)?;
		assert_matches!(
			mat.at::<f32>(-1),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at::<f32>(10),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_mut::<f32>(-1),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_matches!(
			mat.at_mut::<f32>(10),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_eq!(*mat.at::<f32>(0)?, 1.);
		assert_eq!(*mat.at::<f32>(6)?, 7.);
		assert_eq!(*mat.at::<f32>(8)?, 9.);
		*mat.at_mut::<f32>(4)? = 2.;
		assert_eq!(*mat.at::<f32>(4)?, 2.);
	}
	Ok(())
}

#[test]
fn mat_2d_i0_is_rows_i1_is_cols() -> Result<()> {
	// Just a sanity check about which Mat dimension corresponds to which in Size
	let mat = Mat::new_size_with_default(Size::new(6, 5), f32::opencv_type(), 1.23.into())?;
	let size = mat.size()?;
	assert_eq!(size.width, 6);
	assert_eq!(size.height, 5);
	Ok(())
}

#[test]
fn mat_at_2d() -> Result<()> {
	let mut mat = Mat::new_rows_cols_with_default(100, 100, f32::opencv_type(), 1.23.into())?;
	assert_eq!(*mat.at_2d::<f32>(0, 0)?, 1.23);
	*mat.at_2d_mut::<f32>(0, 0)? = 1.;
	assert_eq!(*mat.at_2d::<f32>(0, 0)?, 1.);
	assert_matches!(
		mat.at::<i32>(0),
		Err(Error {
			code: core::StsUnmatchedFormats,
			..
		})
	);
	assert_matches!(
		mat.at::<f32>(10000),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	Ok(())
}

#[test]
fn mat_at_2d_multichannel() -> Result<()> {
	let mut mat = Mat::new_rows_cols_with_default(100, 100, Vec3f::opencv_type(), Scalar::all(1.23))?;
	let pix = *mat.at_2d::<Vec3f>(0, 0)?;
	assert_eq!(pix[0], 1.23);
	assert_eq!(pix[1], 1.23);
	assert_eq!(pix[2], 1.23);

	*mat.at_2d_mut::<Vec3f>(0, 0)? = Vec3f::from([1.1, 2.2, 3.3]);

	let pix = *mat.at_2d::<Vec3f>(0, 0)?;
	assert_eq!(pix[0], 1.1);
	assert_eq!(pix[1], 2.2);
	assert_eq!(pix[2], 3.3);

	assert_matches!(
		mat.at_2d::<i32>(0, 0),
		Err(Error {
			code: core::StsUnmatchedFormats,
			..
		})
	);
	assert_matches!(
		mat.at_2d::<Vec3f>(100, 1),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	assert_matches!(
		mat.at_2d::<Vec3f>(1, 100),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	Ok(())
}

#[test]
fn mat_at_row() -> Result<()> {
	let mut mat = Mat::new_rows_cols_with_default(100, 100, f32::opencv_type(), 1.23.into())?;

	let row = mat.at_row::<f32>(0)?;
	assert_eq!(row.len(), 100);
	assert_eq!(row[0], 1.23);

	let row = mat.at_row_mut::<f32>(1)?;
	row[0..4].copy_from_slice(&[10., 20., 30., 40.]);

	let data = mat.data_typed::<f32>()?;
	assert_eq!(data[0], 1.23);
	assert_eq!(data[100], 10.);
	assert_eq!(data[101], 20.);
	assert_eq!(data[102], 30.);
	assert_eq!(data[103], 40.);

	assert_matches!(
		mat.at_row::<i32>(0),
		Err(Error {
			code: core::StsUnmatchedFormats,
			..
		})
	);
	assert_matches!(
		mat.at_row::<f32>(100),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	assert_matches!(
		mat.at_row_mut::<i32>(0),
		Err(Error {
			code: core::StsUnmatchedFormats,
			..
		})
	);
	assert_matches!(
		mat.at_row_mut::<f32>(100),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	Ok(())
}

#[test]
fn mat_at_pt() -> Result<()> {
	let s: Vec<Vec<f32>> = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];
	let mut m = Mat::from_slice_2d(&s)?;
	assert_eq!(5., *m.at_pt::<f32>(Point::new(1, 1))?);
	assert_eq!(4., *m.at_pt_mut::<f32>(Point::new(0, 1))?);
	assert_eq!(3., unsafe { *m.at_pt_unchecked::<f32>(Point::new(2, 0))? });
	assert_eq!(9., unsafe { *m.at_pt_unchecked_mut::<f32>(Point::new(2, 2))? });
	assert_matches!(
		m.at_pt::<f32>(Point::new(-1, -3)),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	assert_matches!(
		m.at_pt::<f32>(Point::new(3, -3)),
		Err(Error {
			code: core::StsOutOfRange,
			..
		})
	);
	Ok(())
}

#[test]
fn mat_vec() -> Result<()> {
	{
		let s: Vec<Vec<f32>> = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];

		let mat = Mat::from_slice_2d(&s)?;
		assert_eq!(mat.size()?, Size { width: 3, height: 3 });
		assert_eq!(*mat.at_2d::<f32>(1, 1)?, 5.);

		let v = mat.to_vec_2d::<f32>()?;
		assert_eq!(s, v);
	}

	{
		let mut dims = Vector::<i32>::new();
		dims.push(3);
		dims.push(3);
		dims.push(3);
		let mut mat = Mat::new_nd_vec_with_default(&dims, f64::opencv_type(), 2.into())?;
		*mat.at_3d_mut::<f64>(1, 1, 1)? = 10.;
		assert_eq!(3, mat.dims());
		if mat.to_vec_2d::<f64>().is_ok() {
			panic!("dims too high");
		}
	}

	Ok(())
}

#[test]
fn mat_continuous() -> Result<()> {
	let s: Vec<Vec<f32>> = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];

	let mut mat = Mat::from_slice_2d(&s)?;

	{
		let sub_mat_non_cont = Mat::roi(&mat, Rect::new(1, 1, 2, 2))?;
		assert_eq!(mat.typ(), sub_mat_non_cont.typ());
		assert_eq!(2, sub_mat_non_cont.rows());
		assert_eq!(2, sub_mat_non_cont.cols());
		assert!(sub_mat_non_cont.is_submatrix());
		assert!(!sub_mat_non_cont.is_continuous());
		assert_eq!(5., *sub_mat_non_cont.at_2d::<f32>(0, 0)?);
		assert_eq!(6., *sub_mat_non_cont.at_2d::<f32>(0, 1)?);
		assert_eq!(8., *sub_mat_non_cont.at_2d::<f32>(1, 0)?);
		assert_eq!(9., *sub_mat_non_cont.at_2d::<f32>(1, 1)?);

		let vec = sub_mat_non_cont.to_vec_2d::<f32>()?;
		assert_eq!(5., vec[0][0]);
		assert_eq!(6., vec[0][1]);
		assert_eq!(8., vec[1][0]);
		assert_eq!(9., vec[1][1]);

		let mat_clone = sub_mat_non_cont.try_clone()?;
		assert_eq!(mat.typ(), mat_clone.typ());
		assert_eq!(2, mat_clone.rows());
		assert_eq!(2, mat_clone.cols());
		assert!(!mat_clone.is_submatrix());
		assert!(mat_clone.is_continuous());
		assert_eq!(5., *mat_clone.at_2d::<f32>(0, 0)?);
		assert_eq!(6., *mat_clone.at_2d::<f32>(0, 1)?);
		assert_eq!(8., *mat_clone.at_2d::<f32>(1, 0)?);
		assert_eq!(9., *mat_clone.at_2d::<f32>(1, 1)?);
	}

	{
		let sub_mat_cont = Mat::roi(&mat, Rect::new(0, 1, 3, 2))?;
		assert_eq!(mat.typ(), sub_mat_cont.typ());
		assert_eq!(2, sub_mat_cont.rows());
		assert_eq!(3, sub_mat_cont.cols());
		assert!(sub_mat_cont.is_submatrix());
		assert!(sub_mat_cont.is_continuous());
		assert_eq!(4., *sub_mat_cont.at_2d::<f32>(0, 0)?);
		assert_eq!(6., *sub_mat_cont.at_2d::<f32>(0, 2)?);
		assert_eq!(7., *sub_mat_cont.at_2d::<f32>(1, 0)?);
		assert_eq!(9., *sub_mat_cont.at_2d::<f32>(1, 2)?);

		let vec = sub_mat_cont.to_vec_2d::<f32>()?;
		assert_eq!(4., vec[0][0]);
		assert_eq!(6., vec[0][2]);
		assert_eq!(7., vec[1][0]);
		assert_eq!(9., vec[1][2]);
	}

	{
		let mut sub_mat_non_cont = Mat::roi_mut(&mut mat, Rect::new(1, 1, 1, 2))?;
		assert!(!sub_mat_non_cont.is_continuous());
		assert_matches!(
			sub_mat_non_cont.data_typed::<f32>(),
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
		assert_matches!(
			sub_mat_non_cont.data_typed_mut::<f32>(),
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
	}

	Ok(())
}

#[test]
fn mat_merge_split() -> Result<()> {
	let mut src = Vector::<Mat>::new();
	src.push(Mat::new_rows_cols_with_default(1, 3, u8::opencv_type(), 1.into())?);
	src.push(Mat::new_rows_cols_with_default(1, 3, u8::opencv_type(), 2.into())?);
	let mut merged = Mat::default();
	core::merge(&src, &mut merged)?;
	assert_eq!(merged.typ(), Vec2b::opencv_type());
	assert_eq!(merged.at_2d::<Vec2b>(0, 1)?[0], 1);
	assert_eq!(merged.at_2d::<Vec2b>(0, 2)?[1], 2);

	let mut split = Vector::<Mat>::new();
	core::split(&merged, &mut split)?;
	assert_eq!(2, split.len());
	let mat = split.get(0)?;
	assert_eq!(u8::opencv_type(), mat.typ());
	assert_eq!(1, mat.channels());
	assert_eq!(1, *mat.at_2d::<u8>(0, 2)?);
	let mat = split.get(1)?;
	assert_eq!(u8::opencv_type(), mat.typ());
	assert_eq!(1, mat.channels());
	assert_eq!(2, *mat.at_2d::<u8>(0, 0)?);

	Ok(())
}

#[test]
fn mat_from_data() -> Result<()> {
	let mut bytes = PIXEL.to_vec();
	assert_eq!(90, bytes.len());

	{
		let src = unsafe {
			Mat::new_rows_cols_with_data_unsafe_def(
				1,
				PIXEL.len().try_into()?,
				u8::opencv_type(),
				bytes.as_mut_ptr().cast::<c_void>(),
			)?
		};
		assert_eq!(Size::new(PIXEL.len().try_into()?, 1), src.size()?);
		assert_eq!(PIXEL.len(), src.total());
		let row = src.at_row::<u8>(0)?;
		assert_eq!(0x89, row[0]);
		assert_eq!(0x50, row[1]);
		assert_eq!(0x1A, row[6]);
		assert_eq!(0x0D, row[11]);
		assert_eq!(0x82, row[89]);
	}

	{
		let src = unsafe { Mat::new_nd_with_data_unsafe_def(&[3, 5, 6], u8::opencv_type(), bytes.as_mut_ptr().cast::<c_void>())? };
		assert_eq!(Size::new(5, 3), src.size()?);
		assert_eq!(PIXEL.len(), src.total());
		assert_eq!(0x89, *src.at_3d::<u8>(0, 0, 0)?);
		assert_eq!(0x50, *src.at_3d::<u8>(0, 0, 1)?);
		assert_eq!(0x1A, *src.at_3d::<u8>(0, 1, 0)?);
		assert_eq!(0x0D, *src.at_3d::<u8>(0, 1, 5)?);
		assert_eq!(0x82, *src.at_3d::<u8>(2, 4, 5)?);
	}

	{
		let mut bytes = bytes.clone();
		let mut mat = unsafe {
			Mat::new_rows_cols_with_data_unsafe_def(
				1,
				bytes.len().try_into()?,
				u8::opencv_type(),
				bytes.as_mut_ptr().cast::<c_void>(),
			)?
		};
		assert_eq!(mat.data(), bytes.as_ptr());
		bytes[0] = 0xFF;
		assert_eq!(0xFF, *mat.at::<u8>(0)?);
		mat.resize_with_default(100, 0.into())?;
		assert_ne!(mat.data(), bytes.as_ptr());
		bytes[0] = 0xAA;
		let row = mat.at_row::<u8>(0)?;
		assert_eq!(0xFF, row[0]);
		assert_eq!(0x50, row[1]);
		assert_eq!(0x1A, row[6]);
		assert_eq!(0x0D, row[11]);
		assert_eq!(0x82, row[89]);
		let row = mat.at_row::<u8>(1)?;
		assert_eq!(0, row[1]);
		assert_eq!(0, row[6]);
		assert_eq!(0, row[89]);
	}
	Ok(())
}

#[test]
fn mat_from_matexpr() -> Result<()> {
	{
		let mat = Mat::zeros(3, 4, f32::opencv_type())?.to_mat()?;
		assert_eq!(4, mat.cols());
		assert_eq!(3, mat.rows());
		assert_eq!(0., *mat.at_2d::<f32>(0, 0)?);
		assert_eq!(0., *mat.at_2d::<f32>(1, 1)?);
		assert_eq!(0., *mat.at_2d::<f32>(2, 3)?);
	}

	{
		let mat = Mat::ones_nd(&[6, 5], f32::opencv_type())?.to_mat()?;
		assert_eq!(5, mat.cols());
		assert_eq!(6, mat.rows());
		assert_eq!(1., *mat.at_2d::<f32>(0, 0)?);
		assert_eq!(1., *mat.at_2d::<f32>(1, 1)?);
		assert_eq!(1., *mat.at_2d::<f32>(5, 4)?);
	}
	Ok(())
}

#[test]
fn mat_const_iterator() -> Result<()> {
	{
		let mat = Mat::from_slice(&[1, 2, 3, 4])?;
		let mut pos = [0, 0];
		let mut iter = MatConstIterator::over(&mat)?;
		unsafe {
			iter.pos_to(pos.as_mut_ptr())?;
		}
		assert_eq!(iter.typ(), mat.typ());
		assert_eq!(1, *iter.current::<i32>()?);
		assert_eq!(Point::new(0, 0), iter.pos()?);
		assert_eq!([0, 0], pos);
		assert!(iter.has_elements());
		iter.seek(1, true)?;
		unsafe {
			iter.pos_to(pos.as_mut_ptr())?;
		}
		assert_eq!(2, *iter.current::<i32>()?);
		assert_eq!(Point::new(1, 0), iter.pos()?);
		assert_eq!([0, 1], pos);
		assert!(iter.has_elements());
		iter.seek(1, true)?;
		unsafe {
			iter.pos_to(pos.as_mut_ptr())?;
		}
		assert_eq!(3, *iter.current::<i32>()?);
		assert_eq!(Point::new(2, 0), iter.pos()?);
		assert_eq!([0, 2], pos);
		assert!(iter.has_elements());
		iter.seek(1, true)?;
		unsafe {
			iter.pos_to(pos.as_mut_ptr())?;
		}
		assert_eq!(4, *iter.current::<i32>()?);
		assert_eq!(Point::new(3, 0), iter.pos()?);
		assert_eq!([0, 3], pos);
		assert!(iter.has_elements());
		iter.seek(1, true)?;
		unsafe {
			iter.pos_to(pos.as_mut_ptr())?;
		}
		assert_matches!(
			iter.current::<i32>(),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_eq!(Point::new(0, 1), iter.pos()?);
		assert_eq!([1, 0], pos);
		assert!(!iter.has_elements());
		iter.seek(1, true)?;
		unsafe {
			iter.pos_to(pos.as_mut_ptr())?;
		}
		assert_matches!(
			iter.current::<i32>(),
			Err(Error {
				code: core::StsOutOfRange,
				..
			})
		);
		assert_eq!(Point::new(0, 1), iter.pos()?);
		assert_eq!([1, 0], pos);
		assert!(!iter.has_elements());
	}

	Ok(())
}

#[test]
fn mat_iterator() -> Result<()> {
	{
		let mat = Mat::from_slice_2d(&[[1, 2], [3, 4]])?;
		for (pos, x) in mat.iter::<i32>()? {
			match pos {
				Point { x: 0, y: 0 } => assert_eq!(x, 1),
				Point { x: 1, y: 0 } => assert_eq!(x, 2),
				Point { x: 0, y: 1 } => assert_eq!(x, 3),
				Point { x: 1, y: 1 } => assert_eq!(x, 4),
				_ => panic!("Too many elements"),
			}
		}
		for (pos, x) in MatIter::<i32>::new(MatConstIterator::with_start(&mat, Point::new(1, 0))?)? {
			match pos {
				Point { x: 1, y: 0 } => assert_eq!(x, 2),
				Point { x: 0, y: 1 } => assert_eq!(x, 3),
				Point { x: 1, y: 1 } => assert_eq!(x, 4),
				_ => panic!("Too many elements"),
			}
		}
	}

	{
		let mat = Mat::from_slice_2d(&[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])?;
		let roi = Mat::roi(&mat, Rect::new(1, 1, 2, 2))?;
		for (pos, x) in roi.iter::<i32>()? {
			match pos {
				Point { x: 0, y: 0 } => assert_eq!(x, 6),
				Point { x: 1, y: 0 } => assert_eq!(x, 7),
				Point { x: 0, y: 1 } => assert_eq!(x, 10),
				Point { x: 1, y: 1 } => assert_eq!(x, 11),
				_ => panic!("Too many elements"),
			}
		}
	}

	{
		let mut mat = Mat::from_slice_2d(&[[1, 2], [3, 4]])?;
		for (pos, x) in mat.iter_mut::<i32>()? {
			*x *= pos.x + pos.y;
		}
		assert_eq!([0, 2, 3, 8], mat.data_typed::<i32>()?);
	}

	{
		let mut mat = Mat::from_slice_2d(&[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])?;
		let mut roi = Mat::roi_mut(&mut mat, Rect::new(1, 1, 2, 2))?;
		for (pos, x) in roi.iter_mut::<i32>()? {
			*x += pos.x + pos.y;
		}
		assert_eq!(
			[1, 2, 3, 4, 5, 6, 8, 8, 9, 11, 13, 12, 13, 14, 15, 16],
			mat.data_typed::<i32>()?
		);
	}

	{
		let mat = Mat::from_slice::<u8>(&[])?;
		#[allow(clippy::never_loop)]
		for _ in mat.iter::<u8>()? {
			panic!("Mat must be empty");
		}
	}
	Ok(())
}

#[test]
fn mat_locate_roi() -> Result<()> {
	let mat = Mat::from_slice(&[1, 2, 3, 4])?;
	let roi = Mat::roi(&mat, Rect::new(1, 0, 2, 1))?;
	let mut sz = Size::default();
	let mut ofs = Point::default();
	roi.locate_roi(&mut sz, &mut ofs)?;
	assert_eq!(sz, Size::new(4, 1));
	assert_eq!(ofs, Point::new(1, 0));
	Ok(())
}

#[test]
fn mat_roi() -> Result<()> {
	let mut mat = Mat::from_slice(&[1, 2, 3, 4])?.try_clone()?;
	let roi = Mat::roi(&mat, Rect::new(1, 0, 2, 1))?;
	assert_eq!(2, *roi.at(0)?);
	assert_eq!(3, *roi.at(1)?);

	let roi = mat.roi(Rect::new(1, 0, 2, 1))?;
	assert_eq!(2, *roi.at(0)?);
	assert_eq!(3, *roi.at(1)?);

	let mut roi = Mat::roi_mut(&mut mat, Rect::new(1, 0, 2, 1))?;
	assert_eq!(2, *roi.at(0)?);
	assert_eq!(3, *roi.at(1)?);
	*roi.at_mut(0)? = 10;
	*roi.at_mut(1)? = 11;
	assert_eq!(&[1, 10, 11, 4], mat.data_typed::<i32>()?);

	let mut roi = mat.roi_mut(Rect::new(1, 0, 2, 1))?;
	assert_eq!(10, *roi.at(0)?);
	assert_eq!(11, *roi.at(1)?);
	*roi.at_mut(0)? = 20;
	*roi.at_mut(1)? = 21;
	assert_eq!(&[1, 20, 21, 4], mat.data_typed::<i32>()?);

	Ok(())
}

#[test]
fn mat_roi_2() -> Result<()> {
	let mut mat = Mat::from_slice(&[1, 2, 3, 4])?.try_clone()?;
	let (mut roi1, mut roi2) = Mat::roi_2_mut(&mut mat, Rect::new(0, 0, 2, 1), Rect::new(2, 0, 2, 1))?;
	assert_eq!(1, *roi1.at(0)?);
	assert_eq!(2, *roi1.at(1)?);
	assert_eq!(3, *roi2.at(0)?);
	assert_eq!(4, *roi2.at(1)?);

	mem::swap(roi1.at_mut::<i32>(0)?, roi2.at_mut::<i32>(0)?);
	mem::swap(roi1.at_mut::<i32>(1)?, roi2.at_mut::<i32>(1)?);
	assert_eq!(3, *mat.at(0)?);
	assert_eq!(4, *mat.at(1)?);
	assert_eq!(1, *mat.at(2)?);
	assert_eq!(2, *mat.at(3)?);

	assert_matches!(
		Mat::roi_2_mut(&mut mat, Rect::new(0, 0, 3, 1), Rect::new(2, 0, 2, 1)),
		Err(Error {
			code: core::StsBadArg,
			..
		})
	);

	Ok(())
}

#[test]
fn mat_convert() -> Result<()> {
	let mat = Mat::from_slice(&[1, 2, 3, 4])?;
	let mut mat_ = mat.try_clone()?.try_into_typed::<i32>()?;
	assert_eq!(3, *mat_.at(2)?);
	*mat_.at_mut(3)? = 8;
	assert_eq!(8, *mat_.at(3)?);
	assert_eq!(mat.typ(), mat_.typ());
	assert_eq!(mat.size()?, mat_.size()?);
	let mat_back = mat_.into_untyped();
	assert_eq!(mat.size()?, mat_back.size()?);
	Ok(())
}

#[test]
fn mat_mul() -> Result<()> {
	{
		let expr = Mat::ones(1, 5, f64::opencv_type())?;
		let res = core::mul_matexpr_f64(&expr, 4.)?.to_mat()?;
		assert_eq!(res.typ(), f64::opencv_type());
		assert_eq!(res.data_typed::<f64>()?, &[4., 4., 4., 4., 4.]);
	}
	{
		let expr = Mat::new_rows_cols_with_default(2, 3, i32::opencv_type(), 9.into())?;
		let res = core::sub_mat_scalar(&expr, 5.into())?.to_mat()?;
		assert_eq!(res.typ(), i32::opencv_type());
		assert_eq!(res.data_typed::<i32>()?, &[4, 4, 4, 4, 4, 4]);
	}
	{
		let mat1 = Mat::from_slice_2d(&[[1f32, 2., 3.], [4., 5., 6.]])?;
		let mat2 = Mat::from_slice_2d(&[[7f32, 8.], [9., 10.], [11., 12.]])?;
		let res = core::mul_mat_mat(&mat1, &mat2)?.to_mat()?;
		assert_eq!(res.typ(), f32::opencv_type());
		assert_eq!(res.size()?, Size::new(2, 2));
		assert_eq!(res.data_typed::<f32>()?, &[58., 64., 139., 154.]);
	}
	{
		let mat = Mat::from_slice(&[1u8, 2, 3, 4, 5, 6])?;
		let res = core::div_mat_f64(&mat, 2.)?.to_mat()?;
		assert_eq!(res.typ(), u8::opencv_type());
		assert_eq!(res.data_typed::<u8>()?, &[0, 1, 2, 2, 2, 3]);
	}
	Ok(())
}

#[test]
fn mat_data() -> Result<()> {
	{
		let mat = Mat::from_slice(&[8, 13, 21, 39u8])?;
		assert_eq!(&[8, 13, 21, 39], mat.data_bytes()?);
		let roi = Mat::roi(&mat, Rect::new(1, 0, 2, 1))?;
		assert_eq!(&[13, 21], roi.data_bytes()?);
	}

	{
		let mat = Mat::from_slice_2d(&[[6, 7], [16, 17u8]])?;
		assert_eq!(&[6, 7, 16, 17], mat.data_bytes()?);
		let roi = Mat::roi(&mat, Rect::new(0, 0, 1, 2))?;
		assert_matches!(
			roi.data_bytes(),
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
	}

	{
		let mut mat = Mat::from_slice(&[Vec2b::from([5, 8]), Vec2b::from([13, 21])])?.try_clone()?;
		assert_eq!(&[5, 8, 13, 21], mat.data_bytes()?);
		let bytes = mat.data_bytes_mut()?;
		bytes[1] = 90;
		assert_eq!(&[5, 90, 13, 21], mat.data_bytes()?);
	}
	Ok(())
}

#[test]
fn mat_equals() -> Result<()> {
	let mat1 = Mat::new_rows_cols_with_default(3, 3, i32::opencv_type(), 0.into())?;
	let mat2 = Mat::new_rows_cols_with_default(3, 3, i32::opencv_type(), 0.into())?;
	let mat3 = Mat::new_rows_cols_with_default(3, 3, i32::opencv_type(), Scalar::all(1.))?;
	let res = core::equals_mat_mat(&mat1, &mat2)?.to_mat()?;
	assert!(res.data_typed::<u8>()?.iter().all(|&e| e != 0));
	let res = core::equals_mat_mat(&mat1, &mat3)?.to_mat()?;
	assert!(res.data_typed::<u8>()?.iter().all(|&e| e == 0));
	Ok(())
}

#[test]
fn mat_rgb() -> Result<()> {
	#![cfg(feature = "rgb")]
	let m = Mat::new_rows_cols_with_default(3, 3, rgb::RGBA8::opencv_type(), Scalar::all(1.))?;
	assert_eq!(rgb::RGBA::new(1, 1, 1, 1), *m.at_2d(1, 1)?);
	assert_matches!(
		m.at_2d::<rgb::RGB8>(1, 1),
		Err(Error {
			code: core::StsUnmatchedFormats,
			..
		})
	);
	Ok(())
}

#[test]
fn mat_from_slice() -> Result<()> {
	let src_u8 = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let src_point = [
		Point2d::new(10.1, 20.2),
		Point2d::new(30.3, 40.4),
		Point2d::new(50.5, 60.6),
		Point2d::new(70.7, 80.8),
	];

	{
		let mat = Mat::from_slice(&src_u8)?;
		assert_eq!(10, mat.total());
		assert_eq!(5u8, *mat.at(5)?);

		let mat = Mat::from_slice(&src_point)?;
		assert_eq!(4, mat.total());
		assert_eq!(Point2d::new(30.3, 40.4), *mat.at(1)?);
	}

	{
		let mat = Mat::new_rows_cols_with_data(2, 5, &src_u8)?;
		assert_eq!(10, mat.total());
		assert_eq!(2, mat.rows());
		assert_eq!(5, mat.cols());
		assert_eq!(6u8, *mat.at_2d(1, 1)?);

		let mat_sz = Mat::new_size_with_data(Size::new(5, 2), &src_u8)?;
		assert_eq!(mat.data_typed::<u8>()?, mat_sz.data_typed::<u8>()?);

		let mat_nd = Mat::new_nd_with_data(&[2, 5], &src_u8)?;
		assert_eq!(mat.rows(), mat_nd.rows());
		assert_eq!(mat.cols(), mat_nd.cols());
		assert_eq!(mat.data_typed::<u8>()?, mat_nd.data_typed::<u8>()?);

		let mat_3d = Mat::new_nd_with_data(&[1, 2, 3], &src_u8[..6])?;
		assert_eq!(3, mat_3d.dims());
		assert_eq!(-1, mat_3d.rows());
		assert_eq!(-1, mat_3d.cols());
		let mat_size = mat_3d.mat_size();
		assert_eq!(1, mat_size.get(0)?);
		assert_eq!(2, mat_size.get(1)?);
		assert_eq!(3, mat_size.get(2)?);
	}

	{
		let mut src_u8 = src_u8;

		let mut mat = Mat::new_rows_cols_with_data_mut(5, 2, &mut src_u8)?;
		*mat.at_2d_mut::<u8>(3, 0)? = 222;
		assert_eq!(222, src_u8[6]);

		let mut mat_sz = Mat::new_size_with_data_mut(Size::new(2, 5), &mut src_u8)?;
		*mat_sz.at_2d_mut::<u8>(3, 0)? = 211;
		assert_eq!(211, src_u8[6]);

		let mut mat = Mat::new_nd_with_data_mut(&[5, 2], &mut src_u8)?;
		*mat.at_nd_mut::<u8>(&[3, 0])? = 111;
		assert_eq!(111, src_u8[6]);
	}

	{
		let mut src_u8 = src_u8;
		let mut mat = Mat::from_slice_mut(&mut src_u8)?;
		core::divide_def(100., &[2; 10], &mut mat)?;
		assert_eq!([50; 10], src_u8);
	}

	{
		let src: &[u8] = &[];
		let mat_res = Mat::new_nd_with_data(&[], src);
		assert_matches!(
			mat_res,
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
	}

	{
		let src = [1u8; 20];
		let src_mat = Mat::new_rows_cols_with_data(4, 5, &src)?;
		let mut dst = [0u8; 10];
		let mut dst_mat = Mat::new_rows_cols_with_data_mut(2, 5, &mut dst)?;
		assert_eq!(Size::new(5, 2), dst_mat.size()?);
		// this reallocates a `Mat` and disconnects in from the `dst` internally, so it's left untouched
		imgproc::cvt_color_def(&src_mat, &mut dst_mat, imgproc::COLOR_GRAY2BGR)?;
		assert_eq!(3, dst_mat.channels());
		assert_eq!(Size::new(5, 4), dst_mat.size()?);
		assert_eq!([0; 10], dst);
	}

	{
		let mat_res = Mat::new_rows_cols_with_data(3, 3, &src_u8);
		assert_matches!(
			mat_res,
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);

		let mat_res = Mat::new_size_with_data(Size::new(3, 3), &src_u8);
		assert_matches!(
			mat_res,
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);

		let mat_res = Mat::new_nd_with_data(&[3, 3], &src_u8);
		assert_matches!(
			mat_res,
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
	}

	{
		let mat = Mat::new_rows_cols_with_data(2, 2, &src_point)?;
		assert_eq!(4, mat.total());
		assert_eq!(2, mat.rows());
		assert_eq!(2, mat.cols());
		assert_eq!(Point2d::new(50.5, 60.6), *mat.at_2d(1, 0)?);
	}

	Ok(())
}

#[test]
fn mat_from_slice_2d() -> Result<()> {
	{
		let src = [&[1, 2, 3, 4], &[5, 6, 7, 8], &[9, 10, 11, 12], &[13, 14, 15, 16u8]];
		let mat = Mat::from_slice_2d(&src)?;
		assert_eq!(4, mat.rows());
		assert_eq!(4, mat.cols());
		assert_eq!(16, mat.total());
	}

	{
		// first row is shorter
		let src = &[
			[1, 2].as_slice(),
			[5, 6, 7, 8].as_slice(),
			[9, 10, 11, 12].as_slice(),
			[13, 14, 15, 16u8].as_slice(),
		];
		let mat_res = Mat::from_slice_2d(src);
		assert_matches!(
			mat_res,
			Err(Error {
				code: core::StsUnmatchedSizes,
				..
			})
		);
	}

	Ok(())
}

#[test]
fn mat_custom_data_type() -> Result<()> {
	#[repr(C)]
	#[derive(Copy, Clone, Debug)]
	struct A {
		a: i16,
		b: i16,
	}

	unsafe impl DataType for A {
		fn opencv_depth() -> i32 {
			core::CV_16S
		}

		fn opencv_channels() -> i32 {
			2
		}
	}

	let m = Mat::new_rows_cols_with_default(10, 10, Vec2s::opencv_type(), (-10, 20).into())?;
	let data = m.data_typed::<A>()?;
	assert!(!data.is_empty());
	assert!(data.iter().all(|el| el.a == -10 && el.b == 20));
	Ok(())
}

#[test]
fn mat_reshape() -> Result<()> {
	let mut slice = [1, 2, 3, 4, 5, 6];
	let mut mat = Mat::from_slice_mut(&mut slice)?;

	let mut mat2 = mat.reshape_mut(1, 2)?;
	assert_eq!(mat2.cols(), 3);
	assert_eq!(mat2.rows(), 2);
	*mat2.at_2d_mut(1, 0)? = 10;

	assert_eq!(&[1, 2, 3, 10, 5, 6], mat2.data_typed::<i32>()?);

	Ok(())
}
