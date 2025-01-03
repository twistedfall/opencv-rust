use std::ffi::c_void;
use std::mem::{transmute, ManuallyDrop};

use opencv::core::{
	Algorithm, ConjGradSolver, KeyPoint, MinProblemSolver, Ptr, Scalar, TermCriteria, TermCriteria_COUNT, Vec4f, Vector,
};
use opencv::features2d::Feature2D;
use opencv::prelude::*;
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
		fn into_raw(a: Vector<Vec4f>) -> *mut c_void {
			a.into_raw()
		}

		let mut a = Vector::<Vec4f>::new();
		a.push(Vec4f::all(1.));
		a.push(Vec4f::all(2.));
		a.push(Vec4f::all(3.));
		let ptr = into_raw(a);
		let b = unsafe { Vector::<Vec4f>::from_raw(ptr) };
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
	use opencv::core::Ptr;
	use opencv::features2d::{FastFeatureDetector, Feature2D};
	use opencv::videostab::{KeypointBasedMotionEstimator, MotionEstimatorRansacL2};

	let est = MotionEstimatorRansacL2::new_def().unwrap();
	let est_ptr = Ptr::new(est);
	let mut estimator = KeypointBasedMotionEstimator::new(est_ptr.into()).unwrap();
	#[cfg(not(ocvrs_opencv_branch_34))]
	let detector_ptr = FastFeatureDetector::create_def().unwrap();
	#[cfg(ocvrs_opencv_branch_34)]
	let detector_ptr = FastFeatureDetector::create_def().unwrap();
	let base_detector_ptr: Ptr<Feature2D> = detector_ptr.into();
	estimator.set_detector(base_detector_ptr).unwrap();

	Ok(())
}

#[test]
fn smart_ptr_cast_base() -> Result<()> {
	#![cfg(any(ocvrs_has_module_features2d, ocvrs_has_module_xfeatures2d))]
	#[cfg(not(ocvrs_opencv_branch_5))]
	use opencv::features2d::AKAZE;
	#[cfg(ocvrs_opencv_branch_5)]
	use opencv::xfeatures2d::AKAZE;

	let d = AKAZE::create_def()?;
	assert!(Feature2DTraitConst::empty(&d)?);
	assert_eq!("Feature2D.AKAZE", Feature2DTraitConst::get_default_name(&d)?);
	let a = Ptr::<Feature2D>::from(d);
	assert!(Feature2DTraitConst::empty(&a)?);
	assert_eq!("Feature2D.AKAZE", Feature2DTraitConst::get_default_name(&a)?);
	Ok(())
}

#[test]
fn cast_base() -> Result<()> {
	#![cfg(any(ocvrs_has_module_features2d, ocvrs_has_module_features))]
	use opencv::features2d::BFMatcher;

	let m = BFMatcher::new_def()?;
	assert!(AlgorithmTraitConst::empty(&m)?);
	assert_eq!("my_object", &m.get_default_name()?);
	let a = Algorithm::from(m);
	assert!(a.empty()?);
	assert_eq!("my_object", &a.get_default_name()?);
	Ok(())
}

#[test]
fn cast_descendant() -> Result<()> {
	let term_crit1 = TermCriteria::new(TermCriteria_COUNT, 2, 3.)?;
	let term_crit2 = TermCriteria::new(TermCriteria_COUNT, 4, 6.12)?;
	let mut solver = ConjGradSolver::create_def()?;
	solver.set_term_criteria(term_crit1)?;
	assert_eq!(term_crit1, solver.get_term_criteria()?);
	{
		// there is no way to create a simple, non `Ptr` `ConjGradSolver` object so we need to employ an unsafe workaround
		let solver_boxed = unsafe { ConjGradSolver::from_raw(solver.inner_as_raw_mut()) };
		assert_eq!(term_crit1, solver_boxed.get_term_criteria()?);
		// cast to the base class
		let mut min_solver_boxed = MinProblemSolver::from(solver_boxed);
		assert_eq!(term_crit1, min_solver_boxed.get_term_criteria()?);
		min_solver_boxed.set_term_criteria(term_crit2)?;
		// cast back to the descendant class
		let solver_boxed = ManuallyDrop::new(ConjGradSolver::try_from(min_solver_boxed)?);
		assert_eq!(term_crit2, solver_boxed.get_term_criteria()?);
	}
	assert_eq!(term_crit2, solver.get_term_criteria()?);

	Ok(())
}

#[test]
fn cast_descendant_fail() -> Result<()> {
	#![cfg(ocvrs_has_module_stitching)]
	use std::convert::TryFrom;

	use opencv::stitching::{Detail_Blender, Detail_FeatherBlender, Detail_MultiBandBlender};
	use opencv::{core, Error};

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
