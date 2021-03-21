#![cfg(feature = "contrib")]

use opencv::{
    core::Ptr,
    Result,
    videostab::{KeypointBasedMotionEstimator, MotionEstimatorRansacL2, MotionModel},
};

#[test]
fn motion_estimator() -> Result<()> {
    // just test the Ptr creation, conversion to base class and linking for now
    let est = MotionEstimatorRansacL2::new(MotionModel::MM_AFFINE).unwrap();
    let est_ptr = Ptr::new(est);
    let _estimator = KeypointBasedMotionEstimator::new(est_ptr.into()).unwrap();
    Ok(())
}
