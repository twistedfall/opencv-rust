use opencv::{
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
			highgui::imshow(window, &gray)?;
		}
		if highgui::wait_key(10)? > 0 {
			break;
		}
	}
	Ok(())
}
