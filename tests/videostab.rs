#![cfg(ocvrs_has_module_videostab)]

use opencv::{
	core::Ptr,
	features2d::FastFeatureDetector,
	prelude::*,
	Result,
	videostab::{KeypointBasedMotionEstimator, MotionEstimatorRansacL2, MotionModel},
};
#[cfg(feature = "opencv-4")]
use opencv::features2d::FastFeatureDetector_DetectorType;

#[test]
fn motion_estimator() -> Result<()> {
	// just test the Ptr creation, conversion to base class and linking for now
	let est = MotionEstimatorRansacL2::new(MotionModel::MM_AFFINE).unwrap();
	let est_ptr = Ptr::new(est);
	let mut estimator = KeypointBasedMotionEstimator::new(est_ptr.into()).unwrap();
	#[cfg(feature = "opencv-4")]
	let detector_ptr = <dyn FastFeatureDetector>::create(10, true, FastFeatureDetector_DetectorType::TYPE_9_16).unwrap();
	#[cfg(not(feature = "opencv-4"))]
	let detector_ptr = <dyn FastFeatureDetector>::create(10, true, 2).unwrap();
	estimator.set_detector(detector_ptr.into()).unwrap();
	Ok(())
}
