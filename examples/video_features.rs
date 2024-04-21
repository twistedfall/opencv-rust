use opencv::core::Vector;
use opencv::prelude::*;
use opencv::{features2d, highgui, imgproc, videoio, Result};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	if !cam.is_opened()? {
		panic!("Unable to open default camera!");
	}
	let mut orb = features2d::ORB::create_def()?;
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.cols() > 0 {
			let mut gray = Mat::default();
			imgproc::cvt_color_def(&frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;
			let mut kps = Vector::new();
			orb.detect_def(&gray, &mut kps)?;
			let mut display = Mat::default();
			features2d::draw_keypoints_def(&gray, &kps, &mut display)?;
			highgui::imshow(window, &display)?;
		}
		if highgui::wait_key(10)? > 0 {
			break;
		}
	}
	Ok(())
}
