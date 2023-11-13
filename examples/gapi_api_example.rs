//! Port of https://github.com/opencv/opencv/blob/4.7.0/modules/gapi/samples/api_example.cpp

use std::env;

use opencv::core::{Point, Scalar, Size, Vector, BORDER_DEFAULT};
use opencv::gapi::GMat;
use opencv::imgproc::INTER_LINEAR;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::{gapi, highgui, Result};

fn main() -> Result<()> {
	let mut cap = VideoCapture::default()?;
	let args = env::args().collect::<Vec<_>>();
	if args.len() > 1 {
		cap.open_file(&args[0], CAP_ANY)?;
	} else {
		cap.open(0, CAP_ANY)?;
	}
	assert!(cap.is_opened()?);
	let input = GMat::default()?;
	let vga = gapi::resize(&input, Size::default(), 0.5, 0.5, INTER_LINEAR)?;
	let gray = gapi::bgr2_gray(&vga)?;
	let blurred = gapi::blur(&gray, Size::new(5, 5), Point::new(-1, -1), BORDER_DEFAULT, Scalar::all(0.))?;
	let edges = gapi::canny(&blurred, 32., 128., 3, false)?;
	let (b, g, r) = gapi::split3(&vga)?.into_tuple();
	let out = gapi::merge3(&b, &gapi::or_gmat_gmat(&g, &edges)?, &r)?;
	let mut ac = gapi::GComputation::new(input, out)?;
	let mut output_frame = Mat::default();
	loop {
		let mut input_frame = Mat::default();
		assert!(cap.read(&mut input_frame)?);
		ac.apply_2(input_frame, &mut output_frame, Vector::new())?;
		highgui::imshow("output", &output_frame)?;
		if highgui::wait_key(30)? >= 0 {
			break;
		}
	}
	Ok(())
}
