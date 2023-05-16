//! Port of https://github.com/opencv/opencv/blob/4.7.0/samples/cpp/tutorial_code/ImgTrans/HoughLines_Demo.cpp

use std::env::args;
use std::f64::consts::PI;
use std::sync::Mutex;

use opencv::core::{find_file, Point, Point2f, Scalar};
use opencv::highgui::{create_trackbar, imshow, named_window, wait_key, WINDOW_AUTOSIZE};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::imgproc::{canny, cvt_color, hough_lines, hough_lines_p, line, COLOR_BGR2GRAY, COLOR_GRAY2BGR};
use opencv::prelude::*;
use opencv::types::{VectorOfVec2f, VectorOfVec4i};
use opencv::Result;

opencv::opencv_branch_4! {
	use opencv::imgproc::LINE_AA;
}
opencv::not_opencv_branch_4! {
	use opencv::core::LINE_AA;
}

const STANDARD_NAME: &str = "Standard Hough Lines Demo";
const PROBABILISTIC_NAME: &str = "Probabilistic Hough Lines Demo";
const MIN_THRESHOLD: i32 = 50;
const MAX_TRACKBAR: i32 = 150;

/// Indications of how to run this program and why is it for
fn help() {
	println!("\t Hough Transform to detect lines");
	println!("\t---------------------------------");
	println!(" Usage: ./HoughLines_Demo <image_name>");
}

fn main() -> Result<()> {
	// Read the image
	let mut image_name = "building.jpg".to_string(); // by default
	if let Some(image_name_arg) = args().nth(1) {
		image_name = image_name_arg;
	}

	let src = imread(&find_file(&image_name, false, false)?, IMREAD_COLOR)?;
	if src.empty() {
		help();
		return Ok(());
	}

	// Pass the image to gray
	let mut src_gray = Mat::default();
	cvt_color(&src, &mut src_gray, COLOR_BGR2GRAY, 0)?;

	// Apply Canny edge detector
	let mut edges = Mat::default();
	canny(&src_gray, &mut edges, 50., 200., 3, false)?;

	// Create Trackbars for Thresholds
	let thresh_label = format!("Thres: {} + input", MIN_THRESHOLD);

	named_window(STANDARD_NAME, WINDOW_AUTOSIZE)?;
	let mut s_trackbar = MAX_TRACKBAR;
	create_trackbar(
		&thresh_label,
		STANDARD_NAME,
		Some(&mut s_trackbar),
		MAX_TRACKBAR,
		Some(Box::new({
			let edges = Mutex::new(edges.clone());
			move |s_trackbar| {
				standard_hough(&edges.lock().unwrap(), s_trackbar).unwrap();
			}
		})),
	)?;

	named_window(PROBABILISTIC_NAME, WINDOW_AUTOSIZE)?;
	let mut p_trackbar = MAX_TRACKBAR;
	create_trackbar(
		&thresh_label,
		PROBABILISTIC_NAME,
		Some(&mut p_trackbar),
		MAX_TRACKBAR,
		Some(Box::new({
			let edges = Mutex::new(edges.clone());
			move |p_trackbar| {
				probabilistic_hough(&edges.lock().unwrap(), p_trackbar).unwrap();
			}
		})),
	)?;

	standard_hough(&edges, s_trackbar)?;
	probabilistic_hough(&edges, p_trackbar)?;
	wait_key(0)?;
	Ok(())
}

fn standard_hough(edges: &Mat, s_trackbar: i32) -> Result<()> {
	let mut s_lines = VectorOfVec2f::new();
	let mut standard_hough = Mat::default();

	cvt_color(edges, &mut standard_hough, COLOR_GRAY2BGR, 0)?;

	// 1. Use Standard Hough Transform
	hough_lines(edges, &mut s_lines, 1., PI / 180., MIN_THRESHOLD + s_trackbar, 0., 0., 0., PI)?;

	// Show the result
	for s_line in s_lines {
		let [r, t] = *s_line;
		let cos_t = t.cos();
		let sin_t = t.sin();
		let x0 = r * cos_t;
		let y0 = r * sin_t;
		let alpha = 1000.;

		let pt1 = Point2f::new(x0 + alpha * -sin_t, y0 + alpha * cos_t).to::<i32>().unwrap();
		let pt2 = Point2f::new(x0 - alpha * -sin_t, y0 - alpha * cos_t).to::<i32>().unwrap();
		line(&mut standard_hough, pt1, pt2, Scalar::new(255., 0., 0., 0.), 3, LINE_AA, 0)?;
	}
	imshow(STANDARD_NAME, &standard_hough)?;
	Ok(())
}

fn probabilistic_hough(edges: &Mat, p_trackbar: i32) -> Result<()> {
	let mut p_lines = VectorOfVec4i::new();
	let mut probabalistic_hough = Mat::default();

	cvt_color(edges, &mut probabalistic_hough, COLOR_GRAY2BGR, 0)?;

	// 2. Use Probabilistic Hough Transform
	hough_lines_p(edges, &mut p_lines, 1., PI / 180., MIN_THRESHOLD + p_trackbar, 30., 10.)?;

	// Show the result
	for l in p_lines {
		line(
			&mut probabalistic_hough,
			Point::new(l[0], l[1]),
			Point::new(l[2], l[3]),
			Scalar::new(255., 0., 0., 0.),
			3,
			LINE_AA,
			0,
		)?;
	}
	imshow(PROBABILISTIC_NAME, &probabalistic_hough)?;
	Ok(())
}
