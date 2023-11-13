use std::thread;
use std::time::Duration;

use opencv::prelude::*;
use opencv::{core, highgui, imgproc, objdetect, types, videoio, Result};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window_def(window)?;
	let (xml, mut cam) = {
		(
			core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?,
			videoio::VideoCapture::new(0, videoio::CAP_ANY)?, // 0 is the default camera
		)
	};
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
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
		imgproc::resize(
			&gray,
			&mut reduced,
			core::Size { width: 0, height: 0 },
			0.25f64,
			0.25f64,
			imgproc::INTER_LINEAR,
		)?;
		let mut faces = types::VectorOfRect::new();
		face.detect_multi_scale(
			&reduced,
			&mut faces,
			1.1,
			2,
			objdetect::CASCADE_SCALE_IMAGE,
			core::Size { width: 30, height: 30 },
			core::Size { width: 0, height: 0 },
		)?;
		println!("faces: {}", faces.len());
		for face in faces {
			println!("face {face:?}");
			let scaled_face = core::Rect::new(face.x * 4, face.y * 4, face.width * 4, face.height * 4);
			imgproc::rectangle_def(&mut frame, scaled_face, (0, 255, 0).into())?;
		}
		highgui::imshow(window, &frame)?;
		if highgui::wait_key(10)? > 0 {
			break;
		}
	}
	Ok(())
}
