use std::{
	ffi::c_void,
	mem::transmute,
};

use opencv::{
	core::{Algorithm, Scalar, Vec4f},
	prelude::*,
	Result,
	types::{PtrOfFeature2D, VectorOfVec4f},
};

#[test]
fn layout() -> Result<()> {
	let mat = Mat::new_rows_cols_with_default(1, 3, f32::typ(), Scalar::all(10.))?;
	let mut mat_ptr = mat.as_raw_Mat();
	let mat_ref: &mut Mat = unsafe { transmute(&mut mat_ptr) };
	assert_eq!(mat.size()?, mat_ref.size()?);
	assert_eq!(mat.typ(), mat_ref.typ());
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
		assert_eq!(100, b.total());
		assert_eq!(9, *b.at_2d::<u16>(9, 9)?);
	}

	Ok(())
}

/// Ptr creation, conversion to base class and linking
#[test]
fn smart_ptr_crate_and_cast_to_base_class() -> Result<()> {
	#![cfg(ocvrs_has_module_videostab)]
	use opencv::{
		core::Ptr,
		features2d::{FastFeatureDetector, Feature2D},
		videostab::{KeypointBasedMotionEstimator, MotionEstimatorRansacL2, MotionModel},
	};
	#[cfg(ocvrs_opencv_branch_4)]
	use opencv::features2d::FastFeatureDetector_DetectorType;

	let est = MotionEstimatorRansacL2::new(MotionModel::MM_AFFINE).unwrap();
	let est_ptr = Ptr::new(est);
	let mut estimator = KeypointBasedMotionEstimator::new(est_ptr.into()).unwrap();
	#[cfg(ocvrs_opencv_branch_4)]
	let detector_ptr = <dyn FastFeatureDetector>::create(10, true, FastFeatureDetector_DetectorType::TYPE_9_16).unwrap();
	#[cfg(not(ocvrs_opencv_branch_4))]
	let detector_ptr = <dyn FastFeatureDetector>::create(10, true, 2).unwrap();
	let base_detector_ptr: Ptr<Feature2D> = detector_ptr.into();
	estimator.set_detector(base_detector_ptr).unwrap();

	Ok(())
}

#[test]
fn smart_ptr_cast_base() -> Result<()> {
	#![cfg(ocvrs_has_module_features2d)]
	#[cfg(ocvrs_opencv_branch_4)]
	use opencv::features2d::{AKAZE_DescriptorType::DESCRIPTOR_MLDB, KAZE_DiffusivityType::DIFF_PM_G2};
	#[cfg(not(ocvrs_opencv_branch_4))]
	use opencv::features2d::{AKAZE_DESCRIPTOR_MLDB as DESCRIPTOR_MLDB, KAZE_DIFF_PM_G2 as DIFF_PM_G2};

	let d = <dyn AKAZE>::create(DESCRIPTOR_MLDB, 0, 3, 0.001, 4, 4, DIFF_PM_G2)?;
	assert_eq!(true, Feature2DTraitConst::empty(&d)?);
	if !cfg!(ocvrs_opencv_branch_32) {
		assert_eq!("Feature2D.AKAZE", Feature2DTraitConst::get_default_name(&d)?);
	} else {
		assert_eq!("my_object", Feature2DTraitConst::get_default_name(&d)?);
	}
	let a = PtrOfFeature2D::from(d);
	assert_eq!(true, Feature2DTraitConst::empty(&a)?);
	if !cfg!(ocvrs_opencv_branch_32) {
		assert_eq!("Feature2D.AKAZE", Feature2DTraitConst::get_default_name(&a)?);
	} else {
		assert_eq!("my_object", Feature2DTraitConst::get_default_name(&a)?);
	}
	Ok(())
}

#[test]
fn cast_base() -> Result<()> {
	#![cfg(ocvrs_has_module_features2d)]
	use opencv::{features2d::BFMatcher, core::NORM_L2};

	let m = BFMatcher::new(NORM_L2, false)?;
	assert_eq!(true, <dyn AlgorithmTrait>::empty(&m)?);
	assert_eq!("my_object", &m.get_default_name()?);
	let a = Algorithm::from(m);
	assert_eq!(true, a.empty()?);
	assert_eq!("my_object", &a.get_default_name()?);
	Ok(())
}

#[test]
fn cast_descendant() -> Result<()> {
	#![cfg(ocvrs_has_module_rgbd)]
	use std::convert::TryFrom;
	use opencv::rgbd::{OdometryFrame, RgbdFrame};

	let image = Mat::new_rows_cols_with_default(1, 2, i32::typ(), Scalar::from(1.))?;
	let depth = Mat::default();
	let mask = Mat::default();
	let normals = Mat::default();
	let child = OdometryFrame::new(&image, &depth, &mask, &normals, 345)?;
	assert_eq!(345, child.id());
	assert_eq!(2, child.image().cols());
	let mut base = RgbdFrame::from(child);
	assert_eq!(345, base.id());
	assert_eq!(2, base.image().cols());
	base.set_image(Mat::new_rows_cols_with_default(10, 20, f64::typ(), Scalar::from(2.))?);
	let child = OdometryFrame::try_from(base)?;
	assert_eq!(345, child.id());
	assert_eq!(20, child.image().cols());

	Ok(())
}

#[test]
fn cast_descendant_fail() -> Result<()> {
	#![cfg(ocvrs_has_module_stitching)]
	use std::convert::TryFrom;
	use opencv::{
		core,
		stitching::{Detail_FeatherBlender, Detail_MultiBandBlender, Detail_Blender},
		Error,
	};

	let child = Detail_FeatherBlender::new(43.)?;
	assert_eq!(43., child.sharpness()?);
	let base = Detail_Blender::from(child);
	let correct_child = Detail_FeatherBlender::try_from(base)?;
	let base = Detail_Blender::from(correct_child);
	let incorrect_child = Detail_MultiBandBlender::try_from(base);
	if !matches!(incorrect_child, Err(Error { code: core::StsBadArg, .. })) {
		panic!("It shouldn't be possible to downcast to the incorrect descendant class");
	}
	Ok(())
}
