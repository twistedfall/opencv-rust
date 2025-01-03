//! Tests that will not be run in CI on OpenCV 4.2.0, 4.5.4 and 3.4.16 due to missing classes

#[test]
fn smart_ptr_optional() -> opencv::Result<()> {
	#![cfg(ocvrs_has_module_calib3d)]

	// findCirclesGrid doesn't accept an optional blobDetector in 4.2.0, but there is no mechanism to enable type hints only on
	// specific OpenCV versions. With 4.2.0 being that old it should be fine to just not run the test.

	#[cfg(ocvrs_opencv_branch_5)]
	use opencv::calib::{find_circles_grid_1, CALIB_CB_SYMMETRIC_GRID};
	#[cfg(not(ocvrs_opencv_branch_5))]
	use opencv::calib3d::{find_circles_grid_1, CALIB_CB_SYMMETRIC_GRID};
	use opencv::core::{Point2f, Size, Vector};

	let mut centers = Vector::<Point2f>::new();
	for i in 0..4 {
		for j in 0..4 {
			centers.push((i as f32 * 100.0, j as f32 * 100.0).into());
		}
	}

	let mut out_centers = Vector::<Point2f>::new();
	let r = find_circles_grid_1(&centers, Size::new(4, 4), &mut out_centers, CALIB_CB_SYMMETRIC_GRID, None);
	assert!(matches!(r, Ok(true)));
	Ok(())
}
