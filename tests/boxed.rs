use std::{
	ffi::c_void,
	mem::transmute,
};

use opencv::{
	core::{Algorithm, NORM_L2, Scalar, Vec4f},
	features2d::BFMatcher,
	flann::IndexParams,
	prelude::*,
	Result,
	types::{PtrOfFeature2D, PtrOfIndexParams, VectorOfVec4f},
};

#[test]
fn layout() -> Result<()> {
	let mat = Mat::new_rows_cols_with_default(1, 3, f32::typ(), Scalar::all(10.))?;
	let mut mat_ptr = mat.as_raw_Mat();
	let mat_ref: &mut Mat = unsafe { transmute(&mut mat_ptr) };
	assert_eq!(mat.size()?, mat_ref.size()?);
	assert_eq!(mat.typ()?, mat_ref.typ()?);
	assert_eq!(mat.rows(), mat_ref.rows());
	assert_eq!(mat.cols(), mat_ref.cols());
	assert_eq!(mat.at_2d::<f32>(0, 1)?, mat_ref.at_2d::<f32>(0, 1)?);
	Ok(())
}

#[test]
fn into_raw() -> Result<()> {
	{
		#[inline(never)]
		fn into_raw(a: VectorOfVec4f) -> *mut c_void {
			a.into_raw()
		}

		let mut a = VectorOfVec4f::new();
		a.push(Vec4f::all(1.));
		a.push(Vec4f::all(2.));
		a.push(Vec4f::all(3.));
		let ptr = into_raw(a);
		let b = unsafe { VectorOfVec4f::from_raw(ptr) };
		assert_eq!(3, b.len());
		assert_eq!(Vec4f::all(1.), b.get(0)?);
		assert_eq!(Vec4f::all(2.), b.get(1)?);
		assert_eq!(Vec4f::all(3.), b.get(2)?);
	}

	{
		#[inline(never)]
		fn into_raw(a: Mat) -> *mut c_void {
			a.into_raw()
		}

		let a = Mat::new_rows_cols_with_default(10, 10, u16::typ(), Scalar::all(9.))?;
		let ptr = into_raw(a);
		let b = unsafe { Mat::from_raw(ptr) };
		assert_eq!(100, b.total()?);
		assert_eq!(9, *b.at_2d::<u16>(9, 9)?);
	}

	Ok(())
}

#[test]
fn into_raw_ptroff32() -> Result<()> {
	use opencv::types::PtrOff32;

	#[inline(never)]
	fn into_raw(a: PtrOff32) -> *mut c_void {
		a.into_raw()
	}

	let a = PtrOff32::new(87.);
	let ptr = into_raw(a);
	let b = unsafe { PtrOff32::from_raw(ptr) };
	assert_eq!(87., *b);
	Ok(())
}

#[test]
fn into_raw_ptrofboxed() -> Result<()> {
	#[inline(never)]
	fn into_raw(a: PtrOfIndexParams) -> *mut c_void {
		a.into_raw()
	}

	let mut params = IndexParams::default()?;
	params.set_int("int", 87)?;
	params.set_double("double", 12.34)?;
	params.set_string("string", "my string")?;
	let a = PtrOfIndexParams::new(params);
	assert_eq!(87, a.get_int("int", 3)?);
	assert_eq!(3, a.get_int("int_non_existent", 3)?);
	assert_eq!(12.34, a.get_double("double", 99.99)?);
	assert_eq!("my string", a.get_string("string", "-")?);
	let ptr = into_raw(a);
	let b = unsafe { PtrOfIndexParams::from_raw(ptr) };
	assert_eq!(87, b.get_int("int", 3)?);
	assert_eq!(3, b.get_int("int_non_existent", 3)?);
	assert_eq!(12.34, b.get_double("double", 99.99)?);
	assert_eq!("my string", b.get_string("string", "-")?);
	Ok(())
}

#[test]
fn smart_ptr_cast_base() -> Result<()> {
	#[cfg(ocvrs_opencv_branch_4)]
	use opencv::features2d::{AKAZE_DescriptorType::DESCRIPTOR_MLDB, KAZE_DiffusivityType::DIFF_PM_G2};
	#[cfg(not(ocvrs_opencv_branch_4))]
	use opencv::features2d::{AKAZE_DESCRIPTOR_MLDB as DESCRIPTOR_MLDB, KAZE_DIFF_PM_G2 as DIFF_PM_G2};
	let d = <dyn AKAZE>::create(DESCRIPTOR_MLDB, 0, 3, 0.001, 4, 4, DIFF_PM_G2)?;
	assert_eq!(true, Feature2DTrait::empty(&d)?);
	if !cfg!(ocvrs_opencv_branch_32) {
		assert_eq!("Feature2D.AKAZE", Feature2DTrait::get_default_name(&d)?);
	} else {
		assert_eq!("my_object", Feature2DTrait::get_default_name(&d)?);
	}
	let a = PtrOfFeature2D::from(d);
	assert_eq!(true, Feature2DTrait::empty(&a)?);
	if !cfg!(ocvrs_opencv_branch_32) {
		assert_eq!("Feature2D.AKAZE", Feature2DTrait::get_default_name(&a)?);
	} else {
		assert_eq!("my_object", Feature2DTrait::get_default_name(&a)?);
	}
	Ok(())
}

#[test]
fn cast_base() -> Result<()> {
	let m = BFMatcher::new(NORM_L2, false)?;
	assert_eq!(true, AlgorithmTrait::empty(&m)?);
	assert_eq!("my_object", &m.get_default_name()?);
	let a = Algorithm::from(m);
	assert_eq!(true, a.empty()?);
	assert_eq!("my_object", &a.get_default_name()?);
	Ok(())
}
