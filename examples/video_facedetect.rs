use std::thread;
use std::time::Duration;

use opencv::core::{Rect, Size, Vector};
use opencv::prelude::*;
use opencv::{core, highgui, imgproc, objdetect, videoio, Result};

fn main() -> Result<()> {
	const WINDOW: &str = "video capture";
	highgui::named_window_def(WINDOW)?;
	let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?;
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	if !cam.is_opened()? {
		panic!("Unable to open default camera!");
	}
	let mut face = objdetect::CascadeClassifier::new(&xml)?;
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width == 0 {
			thread::sleep(Duration::from_secs(50));
			continue;
		}
		let mut gray = Mat::default();
		imgproc::cvt_color_def(&frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;
		let mut reduced = Mat::default();
		imgproc::resize(&gray, &mut reduced, Size::new(0, 0), 0.25, 0.25, imgproc::INTER_LINEAR)?;
		let mut faces = Vector::new();
		face.detect_multi_scale(
			&reduced,
			&mut faces,
			1.1,
			2,
			objdetect::CASCADE_SCALE_IMAGE,
			Size::new(30, 30),
			Size::new(0, 0),
		)?;
		println!("faces: {}", faces.len());
		for face in faces {
			println!("face {face:?}");
			let scaled_face = Rect::new(face.x * 4, face.y * 4, face.width * 4, face.height * 4);
			imgproc::rectangle_def(&mut frame, scaled_face, (0, 255, 0).into())?;
		}
		highgui::imshow(WINDOW, &frame)?;
		if highgui::wait_key(10)? > 0 {
			break;
		}
	}
	Ok(())
}
