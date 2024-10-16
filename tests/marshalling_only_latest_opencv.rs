//! Tests that will not be run in CI on OpenCV 4.2.0, 4.5.4 and 3.4.16 due to missing classes

/// Setting and getting fields through Ptr
#[test]
#[cfg(all(ocvrs_has_module_aruco, any(ocvrs_opencv_branch_34, ocvrs_opencv_branch_4)))]
#[allow(deprecated)]
fn field_access_on_ptr() -> opencv::Result<()> {
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
	use opencv::prelude::*;

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

#[test]
fn smart_ptr_optional() -> opencv::Result<()> {
	#![cfg(ocvrs_has_module_calib3d)]

	// findCirclesGrid doesn't accept an optional blobDetector in 4.2.0, but there is no mechanism to enable type hints only on
	// specific OpenCV versions. With 4.2.0 being that old it should be fine to just not run the test.

	use opencv::calib3d;
	use opencv::core::{Point2f, Size, Vector};

	let mut centers = Vector::<Point2f>::new();
	for i in 0..4 {
		for j in 0..4 {
			centers.push((i as f32 * 100.0, j as f32 * 100.0).into());
		}
	}

	let mut out_centers = Vector::<Point2f>::new();
	let r = calib3d::find_circles_grid_1(
		&centers,
		Size::new(4, 4),
		&mut out_centers,
		calib3d::CALIB_CB_SYMMETRIC_GRID,
		None,
	);
	assert!(matches!(r, Ok(true)));
	Ok(())
}
