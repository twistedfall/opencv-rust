use std::{
	ffi::c_void,
	mem::transmute,
};

use opencv::{
	core::{Scalar, Vec4f},
	prelude::*,
	Result,
	types::VectorOfVec4f,
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
#[cfg(not(feature = "opencv-32"))]
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
