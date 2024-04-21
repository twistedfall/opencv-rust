use opencv::prelude::*;
use opencv::{highgui, imgproc, videoio, Result};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	if !cam.is_opened()? {
		panic!("Unable to open default camera!");
	}
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width > 0 {
			let mut gray = Mat::default();
			imgproc::cvt_color_def(&frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;
			highgui::imshow(window, &gray)?;
		}
		if highgui::wait_key(10)? > 0 {
			break;
		}
	}
	Ok(())
}
