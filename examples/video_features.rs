use opencv::{
	core,
	features2d,
	highgui,
	imgproc,
	prelude::*,
	Result,
	videoio,
};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;
	opencv::opencv_branch_32! {
		let mut cam = videoio::VideoCapture::new_default(0)?; // 0 is the default camera
	}
	opencv::not_opencv_branch_32! {
		let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	}
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
	let mut orb = <dyn features2d::ORB>::default()?;
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width > 0 {
			let mut gray = Mat::default();
			imgproc::cvt_color(
				&frame,
				&mut gray,
				imgproc::COLOR_BGR2GRAY,
				0,
			)?;
			let mut kps = opencv::types::VectorOfKeyPoint::new();
			let mask = Mat::default();
			orb.detect(&gray, &mut kps, &mask)?;
			let mut display = Mat::default();
			opencv::opencv_branch_4! {
				let default_draw_matches_flags = features2d::DrawMatchesFlags::DEFAULT;
			}
			opencv::not_opencv_branch_4! {
				let default_draw_matches_flags = features2d::DrawMatchesFlags_DEFAULT;
			}
			features2d::draw_keypoints(
				&gray,
				&kps,
				&mut display,
				core::Scalar::all(-1f64),
				default_draw_matches_flags,
			)?;
			highgui::imshow(window, &display)?;
		}
		if highgui::wait_key(10)? > 0 {
			break;
		}
	}
	Ok(())
}
