use std::{
	env,
	error::Error,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, Mutex,
	},
};

use opencv::{
	calib3d,
	core::{self, Point, Scalar, Size},
	highgui, imgcodecs, imgproc,
	prelude::*,
	types::VectorOfPoint,
};

opencv::opencv_branch_4! {
	use self::imgproc::LINE_8;
}
opencv::not_opencv_branch_4! {
	use self::core::LINE_8;
}

fn help() {
	// print a welcome message, and the OpenCV version
	println!();
	println!("This is a demo program shows how perspective transformation applied on an image,");
	println!("Using OpenCV version {}", core::CV_VERSION);
	println!();
	println!(
		"Usage:\n{} [image_name -- Default data/right.jpg]",
		env::args().next().unwrap()
	);
	println!();
	println!("Hot keys:");
	println!("\tESC, q - quit the program");
	println!("\tr - change order of points to rotate transformation");
	println!("\tc - delete selected points");
	println!("\ti - change order of points to inverse transformation");
	println!("\nUse your mouse to select a point and move it to see transformation changes");
}

static WINDOW_TITLE: &str = "Perspective Transformation Demo";
static LABELS: [&str; 4] = ["TL", "TR", "BR", "BL"];

fn main() -> Result<(), Box<dyn Error>> {
	let roi_corners = Arc::new(Mutex::new(VectorOfPoint::new()));
	let mut dst_corners = VectorOfPoint::with_capacity(4);
	for _ in 0..4 {
		dst_corners.push(Point::default());
	}
	let validation_needed = Arc::new(AtomicBool::new(true));

	help();
	let filename = env::args().nth(1).unwrap_or_else(|| "data/right.jpg".to_string());
	let filename = core::find_file(&filename, true, false)?;
	let original_image = imgcodecs::imread(&filename, imgcodecs::IMREAD_COLOR)?;
	let mut image;
	let original_image_cols = original_image.cols() as f32;
	let original_image_rows = original_image.rows() as f32;
	{
		let mut roi_corners = roi_corners.lock().unwrap();
		roi_corners.push(Point::new(
			(original_image_cols / 1.7) as i32,
			(original_image_rows / 4.2) as i32,
		));
		roi_corners.push(Point::new(
			(original_image_cols / 1.15) as i32,
			(original_image_rows / 3.32) as i32,
		));
		roi_corners.push(Point::new(
			(original_image_cols / 1.33) as i32,
			(original_image_rows / 1.1) as i32,
		));
		roi_corners.push(Point::new(
			(original_image_cols / 1.93) as i32,
			(original_image_rows / 1.36) as i32,
		));
	}
	highgui::named_window(WINDOW_TITLE, highgui::WINDOW_NORMAL)?;
	highgui::named_window("Warped Image", highgui::WINDOW_AUTOSIZE)?;
	highgui::move_window("Warped Image", 20, 20)?;
	highgui::move_window(WINDOW_TITLE, 330, 20)?;
	highgui::set_mouse_callback(
		WINDOW_TITLE,
		Some(Box::new({
			let mut dragging = false;
			let mut selected_corner_index = 0;
			let roi_corners = Arc::clone(&roi_corners);
			let validation_needed = Arc::clone(&validation_needed);
			move |event, x, y, _flags| {
				{
					let mut roi_corners = roi_corners.lock().unwrap();
					// Action when left button is pressed
					if roi_corners.len() == 4 {
						for (i, roi_corner) in roi_corners.iter().enumerate() {
							if event == highgui::EVENT_LBUTTONDOWN && (roi_corner.x - x).abs() < 10 && (roi_corner.y - y).abs() < 10 {
								selected_corner_index = i;
								dragging = true;
							}
						}
					} else if event == highgui::EVENT_LBUTTONDOWN {
						roi_corners.push(Point::new(x, y));
						validation_needed.store(true, Ordering::Relaxed);
					}
					// Action when left button is released
					if event == highgui::EVENT_LBUTTONUP {
						dragging = false;
					}
					// Action when left button is pressed and mouse has moved over the window
					if event == highgui::EVENT_MOUSEMOVE && dragging {
						roi_corners.set(selected_corner_index, Point::new(x, y)).unwrap();
						validation_needed.store(true, Ordering::Relaxed);
					}
				}
			}
		})),
	)?;
	let mut end_program = false;
	while !end_program {
		let roi_corners_len = roi_corners.lock().unwrap().len();
		if validation_needed.load(Ordering::Relaxed) && roi_corners_len < 4 {
			validation_needed.store(false, Ordering::Relaxed);
			image = original_image.clone();
			{
				let roi_corners = roi_corners.lock().unwrap();
				for (i, roi_corner) in roi_corners.iter().enumerate() {
					imgproc::circle(&mut image, roi_corner, 5, Scalar::new(0., 255., 0., 0.), 3, LINE_8, 0)?;
					if i > 0 {
						imgproc::line(
							&mut image,
							roi_corners.get(i - 1)?,
							roi_corner,
							Scalar::new(0., 0., 255., 0.),
							2,
							LINE_8,
							0,
						)?;
						imgproc::circle(&mut image, roi_corner, 5, Scalar::new(0., 255., 0., 0.), 3, LINE_8, 0)?;
						imgproc::put_text(
							&mut image,
							LABELS[i],
							roi_corner,
							highgui::QT_FONT_NORMAL,
							0.8,
							Scalar::new(255., 0., 0., 0.),
							2,
							LINE_8,
							false,
						)?;
					}
				}
			}
			highgui::imshow(WINDOW_TITLE, &image)?;
		}
		if validation_needed.load(Ordering::Relaxed) && roi_corners_len == 4 {
			image = original_image.clone();
			{
				let roi_corners = roi_corners.lock().unwrap();
				for (i, roi_corner) in roi_corners.iter().enumerate() {
					imgproc::line(
						&mut image,
						roi_corner,
						roi_corners.get((i + 1) % 4)?,
						Scalar::new(0., 0., 255., 0.),
						2,
						LINE_8,
						0,
					)?;
					imgproc::circle(&mut image, roi_corner, 5, Scalar::new(0., 255., 0., 0.), 3, LINE_8, 0)?;
					imgproc::put_text(
						&mut image,
						LABELS[i],
						roi_corner,
						highgui::QT_FONT_NORMAL,
						0.8,
						Scalar::new(255., 0., 0., 0.),
						2,
						LINE_8,
						false,
					)?;
				}
				highgui::imshow(WINDOW_TITLE, &image)?;
				dst_corners.set(0, Point::new(0, 0))?;
				dst_corners.set(
					1,
					Point::new(
						(roi_corners.get(0)? - roi_corners.get(1)?)
							.norm()
							.max((roi_corners.get(2)? - roi_corners.get(3)?).norm()) as i32,
						0,
					),
				)?;
				dst_corners.set(
					2,
					Point::new(
						(roi_corners.get(0)? - roi_corners.get(1)?)
							.norm()
							.max((roi_corners.get(2)? - roi_corners.get(3)?).norm()) as i32,
						(roi_corners.get(1)? - roi_corners.get(2)?)
							.norm()
							.max((roi_corners.get(3)? - roi_corners.get(0)?).norm()) as i32,
					),
				)?;
				dst_corners.set(
					3,
					Point::new(
						0,
						(roi_corners.get(1)? - roi_corners.get(2)?)
							.norm()
							.max((roi_corners.get(3)? - roi_corners.get(0)?).norm()) as i32,
					),
				)?;
				let warped_image_size = Size::new(dst_corners.get(2)?.x, dst_corners.get(2)?.y);
				let roi_corners_mat = Mat::from_exact_iter(roi_corners.iter())?;
				let dst_corners_mat = Mat::from_exact_iter(dst_corners.iter())?;
				let h = calib3d::find_homography(&roi_corners_mat, &dst_corners_mat, &mut Mat::default(), 0, 3.)?; //get homography
				let mut warped_image = Mat::default();
				imgproc::warp_perspective(
					&original_image,
					&mut warped_image,
					&h,
					warped_image_size,
					imgproc::INTER_LINEAR,
					core::BORDER_CONSTANT,
					Scalar::default(),
				)?; // do perspective transformation
				highgui::imshow("Warped Image", &warped_image)?;
			}
		}
		let c = highgui::wait_key(10)? as u8 as char;
		if c == 'q' || c == 'Q' || c as u8 == 27 {
			end_program = true;
		}
		if c == 'c' || c == 'C' {
			roi_corners.lock().unwrap().clear();
		}
		if c == 'r' || c == 'R' {
			{
				let mut roi_corners = roi_corners.lock().unwrap();
				let t = roi_corners.get(0)?;
				roi_corners.push(t);
				roi_corners.remove(0)?;
			}
		}
		if c == 'i' || c == 'I' {
			{
				let mut roi_corners = roi_corners.lock().unwrap();
				roi_corners.swap(0, 1)?;
				roi_corners.swap(2, 3)?;
			}
		}
	}
	Ok(())
}
