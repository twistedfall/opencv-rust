//! Port of https://github.com/opencv/opencv/blob/4.7.0/samples/cpp/tutorial_code/ImgTrans/HoughCircle_Demo.cpp

use std::env::args;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use opencv::core::{find_file, AlgorithmHint, Point, Point2i, Size2i, Vec3f, Vector, BORDER_DEFAULT};
use opencv::highgui::{create_trackbar, imshow, named_window, wait_key, WINDOW_AUTOSIZE};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::imgproc::{circle, cvt_color_def, gaussian_blur, hough_circles, COLOR_BGR2GRAY, HOUGH_GRADIENT};
use opencv::prelude::*;
use opencv::Result;

opencv::not_opencv_branch_34! {
	 use opencv::imgproc::LINE_8;
}
opencv::opencv_branch_34! {
	 use opencv::core::LINE_8;
}

// Windows and trackbars names
const WINDOW_NAME: &str = "Hough Circle Detection Demo";
const CANNY_THRESHOLD_TRACKBAR_NAME: &str = "Canny Threshold";
const ACCUMULATOR_THRESHOLD_TRACKBAR_NAME: &str = "Accumulator Threshold";

// Initial and max values of the parameters of interest
const CANNY_THRESHOLD_INIT_VAL: i32 = 100;
const ACCUMULATOR_THRESHOLD_INIT_VAL: i32 = 50;
const MAX_CANNY_THRESHOLD: i32 = 255;
const MAX_ACCUMULATOR_THRESHOLD: i32 = 200;

fn hough_detection(src_gray: &Mat, src_display: &Mat, canny_threshold: i32, accumulator_threshold: i32) -> Result<()> {
	// Will hold the results of the detection
	let mut circles = Vector::<Vec3f>::new();

	// Runs the actual detection
	hough_circles(
		&src_gray,
		&mut circles,
		HOUGH_GRADIENT,
		1.0,
		(src_gray.rows() / 8).into(),
		canny_threshold.into(),
		accumulator_threshold.into(),
		0,
		0,
	)?;

	// Clone the color input image for display purposes
	let mut display: Mat = src_display.clone();

	// Show the result
	for c in circles {
		let center: Point2i = Point::new(c[0].round() as i32, c[1].round() as i32);
		let radius: i32 = c[2].round() as i32;

		// Circle center
		circle(&mut display, center, 3, (0.0, 255.0, 0.0).into(), -1, LINE_8, 0)?;

		// Circle outline
		circle(&mut display, center, radius, (0.0, 0.0, 255.0).into(), 3, LINE_8, 0)?;
	}

	// Show the results
	imshow(WINDOW_NAME, &display)?;
	Ok(())
}

fn main() -> Result<()> {
	// Read the image
	let mut image_name = "stuff.jpg".to_string(); // by default
	if let Some(image_name_arg) = args().nth(1) {
		image_name = image_name_arg;
	}

	let src = imread(&find_file(&image_name, true, false)?, IMREAD_COLOR)?;
	if src.empty() {
		eprintln!("Invalid input image");
		println!(
			"Usage: {} <path_to_input_image>",
			args().next().unwrap_or_else(|| "program".to_string())
		);
		return Ok(());
	}

	// Convert it to gray
	let mut src_gray = Mat::default();
	cvt_color_def(&src, &mut src_gray, COLOR_BGR2GRAY)?;

	// Reduce the noise so we avoid false circle detection
	let mut src_gray_blur = Mat::default();
	gaussian_blur(
		&src_gray,
		&mut src_gray_blur,
		Size2i::new(9, 9),
		2.0,
		2.0,
		BORDER_DEFAULT,
		AlgorithmHint::ALGO_HINT_DEFAULT,
	)?;

	// Declare and initialize both parameters that are subjects to change
	let canny_threshold: Arc<AtomicI32> = Arc::new(AtomicI32::new(CANNY_THRESHOLD_INIT_VAL));
	let accumulator_threshold: Arc<AtomicI32> = Arc::new(AtomicI32::new(ACCUMULATOR_THRESHOLD_INIT_VAL));

	// Create the main window, and attach the trackbars
	named_window(WINDOW_NAME, WINDOW_AUTOSIZE)?;

	create_trackbar(
		CANNY_THRESHOLD_TRACKBAR_NAME,
		WINDOW_NAME,
		None,
		MAX_CANNY_THRESHOLD,
		Some(Box::new({
			let canny_threshold = canny_threshold.clone();
			move |val| {
				canny_threshold.as_ref().store(val, Ordering::SeqCst);
			}
		})),
	)?;

	create_trackbar(
		ACCUMULATOR_THRESHOLD_TRACKBAR_NAME,
		WINDOW_NAME,
		None,
		MAX_ACCUMULATOR_THRESHOLD,
		Some(Box::new({
			let accumulator_threshold = accumulator_threshold.clone();
			move |val| {
				accumulator_threshold.as_ref().store(val, Ordering::SeqCst);
			}
		})),
	)?;

	// Infinite loop to display
	// and refresh the content of the output image
	// until the user presses q or Q
	let mut key: char = ' ';
	while key.to_ascii_lowercase() != 'q' {
		// Those parameters cannot be = 0, so we must check here
		let canny_threshold_val = canny_threshold.fetch_max(1, Ordering::SeqCst);
		let accumulator_threshold_val = accumulator_threshold.fetch_max(1, Ordering::SeqCst);

		// Runs the detection, and update the display
		hough_detection(&src_gray_blur, &src, canny_threshold_val, accumulator_threshold_val)?;

		// Get user key
		key = wait_key(10).unwrap() as u8 as char;
	}

	Ok(())
}
