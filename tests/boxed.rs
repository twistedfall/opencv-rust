use std::ffi::c_void;
use std::mem::transmute;

use opencv::core::{Algorithm, KeyPoint, Scalar, Vec4f};
use opencv::prelude::*;
use opencv::types::{PtrOfFeature2D, VectorOfVec4f};
use opencv::Result;

#[test]
fn layout() -> Result<()> {
	let mat = Mat::new_rows_cols_with_default(1, 3, f32::opencv_type(), Scalar::all(10.))?;
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

		let a = Mat::new_rows_cols_with_default(10, 10, u16::opencv_type(), Scalar::all(9.))?;
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
		videostab::{KeypointBasedMotionEstimator, MotionEstimatorRansacL2},
	};

	let est = MotionEstimatorRansacL2::new_def().unwrap();
	let est_ptr = Ptr::new(est);
	let mut estimator = KeypointBasedMotionEstimator::new(est_ptr.into()).unwrap();
	#[cfg(ocvrs_opencv_branch_4)]
	let detector_ptr = FastFeatureDetector::create_def().unwrap();
	#[cfg(not(ocvrs_opencv_branch_4))]
	let detector_ptr = FastFeatureDetector::create_def().unwrap();
	let base_detector_ptr: Ptr<Feature2D> = detector_ptr.into();
	estimator.set_detector(base_detector_ptr).unwrap();

	Ok(())
}

#[test]
fn smart_ptr_cast_base() -> Result<()> {
	#![cfg(ocvrs_has_module_features2d)]
	#[cfg(ocvrs_opencv_branch_4)]
	use opencv::features2d::AKAZE;
	#[cfg(not(ocvrs_opencv_branch_4))]
	use opencv::features2d::AKAZE;

	let d = AKAZE::create_def()?;
	assert!(Feature2DTraitConst::empty(&d)?);
	assert_eq!("Feature2D.AKAZE", Feature2DTraitConst::get_default_name(&d)?);
	let a = PtrOfFeature2D::from(d);
	assert!(Feature2DTraitConst::empty(&a)?);
	assert_eq!("Feature2D.AKAZE", Feature2DTraitConst::get_default_name(&a)?);
	Ok(())
}

#[test]
fn cast_base() -> Result<()> {
	#![cfg(ocvrs_has_module_features2d)]
	use opencv::features2d::BFMatcher;

	let m = BFMatcher::new_def()?;
	assert!(<dyn AlgorithmTrait>::empty(&m)?);
	assert_eq!("my_object", &m.get_default_name()?);
	let a = Algorithm::from(m);
	assert!(a.empty()?);
	assert_eq!("my_object", &a.get_default_name()?);
	Ok(())
}

#[test]
fn cast_descendant() -> Result<()> {
	#![cfg(ocvrs_has_module_rgbd)]
	use opencv::rgbd::{OdometryFrame, RgbdFrame};
	use std::convert::TryFrom;

	let image = Mat::new_rows_cols_with_default(1, 2, i32::opencv_type(), Scalar::from(1.))?;
	let depth = Mat::default();
	let mask = Mat::default();
	let normals = Mat::default();
	let child = OdometryFrame::new(&image, &depth, &mask, &normals, 345)?;
	assert_eq!(345, child.id());
	assert_eq!(2, child.image().cols());
	let mut base = RgbdFrame::from(child);
	assert_eq!(345, base.id());
	assert_eq!(2, base.image().cols());
	base.set_image(Mat::new_rows_cols_with_default(10, 20, f64::opencv_type(), Scalar::from(2.))?);
	let child = OdometryFrame::try_from(base)?;
	assert_eq!(345, child.id());
	assert_eq!(20, child.image().cols());

	Ok(())
}

#[test]
fn cast_descendant_fail() -> Result<()> {
	#![cfg(ocvrs_has_module_stitching)]
	use opencv::{
		core,
		stitching::{Detail_Blender, Detail_FeatherBlender, Detail_MultiBandBlender},
		Error,
	};
	use std::convert::TryFrom;

	let child = Detail_FeatherBlender::new(43.)?;
	assert_eq!(43., child.sharpness()?);
	let base = Detail_Blender::from(child);
	let correct_child = Detail_FeatherBlender::try_from(base)?;
	let base = Detail_Blender::from(correct_child);
	let incorrect_child = Detail_MultiBandBlender::try_from(base);
	if !matches!(
		incorrect_child,
		Err(Error {
			code: core::StsBadArg,
			..
		})
	) {
		panic!("It shouldn't be possible to downcast to the incorrect descendant class");
	}
	Ok(())
}

#[test]
fn implicit_clone() -> Result<()> {
	let key_point = KeyPoint::new_coords(1., 2., 3., 4., 5., 6, 7)?;
	assert_eq!(4., key_point.angle());
	let mut key_point_clone = key_point.clone();
	assert_eq!(key_point.pt(), key_point_clone.pt());
	assert_eq!(key_point.size(), key_point_clone.size());
	assert_eq!(key_point.response(), key_point_clone.response());
	assert_eq!(key_point.octave(), key_point_clone.octave());
	assert_eq!(key_point.class_id(), key_point_clone.class_id());

	key_point_clone.set_octave(10);
	assert_eq!(6, key_point.octave());
	assert_eq!(10, key_point_clone.octave());
	Ok(())
}
