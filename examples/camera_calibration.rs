//! Port of code from the tutorial at: https://docs.opencv.org/4.x/dc/dbb/tutorial_py_calibration.html

use std::error::Error;
use std::fs;

use opencv::core::{
	no_array, Point2f, Point3f, Size, TermCriteria, TermCriteria_EPS, TermCriteria_MAX_ITER, TermCriteria_Type, Vector,
};
use opencv::prelude::*;
use opencv::{highgui, imgcodecs, imgproc, opencv_branch_34, opencv_branch_4, opencv_branch_5};

opencv_branch_5! {
	use opencv::calib::{find_chessboard_corners_def, draw_chessboard_corners, calibrate_camera};
	use opencv::mod_3d::{undistort, init_undistort_rectify_map};
}

opencv_branch_4! {
	use opencv::calib3d::{find_chessboard_corners_def, draw_chessboard_corners, calibrate_camera, undistort, init_undistort_rectify_map};
}

opencv_branch_34! {
	use opencv::calib3d::{find_chessboard_corners_def, draw_chessboard_corners, calibrate_camera};
	use opencv::imgproc::{undistort, init_undistort_rectify_map};
}

fn main() -> Result<(), Box<dyn Error>> {
	// termination criteria
	let criteria = TermCriteria {
		typ: TermCriteria_EPS + TermCriteria_MAX_ITER,
		max_count: 30,
		epsilon: 0.001,
	};

	// prepare object points, like (0,0,0), (1,0,0), (2,0,0) ....,(6,5,0)
	let objp_len = 6 * 7;
	let objp = Vector::from_iter((0..objp_len).map(|i| Point3f::new((i % 7) as f32, (i / 7) as f32, 0.)));

	let images = fs::read_dir(".")?
		.flatten()
		.filter(|entry| entry.path().extension().is_some_and(|ext| ext == "jpg"));

	for image in images {
		// Arrays to store object points and image points from all the images.
		let mut objpoints = Vector::<Vector<Point3f>>::new(); // 3d point in real world space
		let mut imgpoints = Vector::<Vector<Point2f>>::new(); // 2d points in image plane.

		let mut img = imgcodecs::imread_def(image.path().to_string_lossy().as_ref())?;
		let mut gray = Mat::default();
		imgproc::cvt_color_def(&img, &mut gray, imgproc::COLOR_BGR2GRAY)?;

		let mut corners = Vector::<Point2f>::default();
		let ret = find_chessboard_corners_def(&gray, Size::new(7, 6), &mut corners)?;
		if ret {
			objpoints.push(objp.clone());

			imgproc::corner_sub_pix(&gray, &mut corners, Size::new(11, 11), Size::new(-1, -1), criteria)?;

			// Draw and display the corners
			draw_chessboard_corners(&mut img, Size::new(7, 6), &corners, ret)?;
			highgui::imshow("Source", &img)?;

			imgpoints.push(corners);

			// Calibration
			let mut mtx = Mat::default();
			let mut dist = Mat::default();
			let mut rvecs = Vector::<Mat>::new();
			let mut tvecs = Vector::<Mat>::new();
			calibrate_camera(
				&objpoints,
				&imgpoints,
				gray.size()?,
				&mut mtx,
				&mut dist,
				&mut rvecs,
				&mut tvecs,
				0,
				TermCriteria::new(
					i32::from(TermCriteria_Type::COUNT) + i32::from(TermCriteria_Type::EPS),
					30,
					f64::EPSILON,
				)?,
			)?;

			// Using cv.undistort()
			let mut dst_undistort = Mat::default();
			undistort(&img, &mut dst_undistort, &mtx, &dist, &no_array())?;
			highgui::imshow("Result using undistort", &dst_undistort)?;

			// Using remapping
			let mut mapx = Mat::default();
			let mut mapy = Mat::default();
			init_undistort_rectify_map(
				&mtx,
				&dist,
				&no_array(),
				&no_array(),
				img.size()?,
				f32::opencv_type(),
				&mut mapx,
				&mut mapy,
			)?;
			let mut dst_remap = Mat::default();
			imgproc::remap_def(&img, &mut dst_remap, &mapx, &mapy, imgproc::INTER_LINEAR)?;
			highgui::imshow("Result using remap", &dst_undistort)?;

			highgui::wait_key_def()?;
		}
	}
	highgui::destroy_all_windows()?;
	Ok(())
}
