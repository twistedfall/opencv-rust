//! Tests that will not be run in CI on OpenCV 4.5.4 and 3.4.16 due to missing classes

use opencv::prelude::*;
use opencv::Result;

/// Setting and getting fields through Ptr
#[test]
fn field_access_on_ptr() -> Result<()> {
	#![cfg(all(ocvrs_has_module_aruco, any(ocvrs_opencv_branch_34, ocvrs_opencv_branch_4)))]
	use opencv::aruco::EstimateParameters;
	// the location and parameters are wildly different between even the minor release in the OpenCV branches, so for now
	// let's just limit to only those fields that are stable
	// #[cfg(ocvrs_opencv_branch_34)]
	// use opencv::aruco::PatternPos::CW_top_left_corner as LeftTop;
	// #[cfg(ocvrs_opencv_branch_4)]
	// use opencv::aruco::PatternPositionType::ARUCO_CW_TOP_LEFT_CORNER as LeftTop;
	// #[cfg(ocvrs_opencv_branch_34)]
	// use opencv::calib3d::SolvePnPMethod::{SOLVEPNP_EPNP, SOLVEPNP_MAX_COUNT};
	// #[cfg(ocvrs_opencv_branch_4)]
	// use opencv::calib3d::{SOLVEPNP_EPNP, SOLVEPNP_MAX_COUNT};
	use opencv::core::Ptr;

	let mut params = EstimateParameters::default()?;
	assert!(!params.use_extrinsic_guess());
	params.set_use_extrinsic_guess(true);
	let params_ptr = Ptr::<EstimateParameters>::new(params);
	assert!(params_ptr.use_extrinsic_guess());

	let plain = EstimateParameters::default()?;
	let mut ptr = Ptr::<EstimateParameters>::new(EstimateParameters::default()?);

	// assert_eq!(plain.pattern(), ptr.pattern());
	// ptr.set_pattern(LeftTop);
	// assert_eq!(LeftTop, ptr.pattern());

	assert_eq!(plain.use_extrinsic_guess(), ptr.use_extrinsic_guess());
	ptr.set_use_extrinsic_guess(true);
	assert!(ptr.use_extrinsic_guess());

	// assert_eq!(plain.solve_pnp_method(), ptr.solve_pnp_method());
	// ptr.set_solve_pnp_method(SOLVEPNP_MAX_COUNT);
	// assert_eq!(ptr.solve_pnp_method(), SOLVEPNP_MAX_COUNT);

	Ok(())
}
