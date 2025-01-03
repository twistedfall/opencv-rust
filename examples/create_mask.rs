//! # Create Mask
//! Reference: [opencv/samples/cpp/create_mask.cpp](https://github.com/opencv/opencv/blob/4.9.0/samples/cpp/create_mask.cpp)

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{env, process};

use opencv::core::{bitwise_and, find_file, CommandLineParser, Point, Scalar, Vec3b};
use opencv::highgui::imshow;
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::prelude::*;
use opencv::{highgui, imgproc, not_opencv_branch_34, opencv_branch_34, Result};

not_opencv_branch_34! {
	use opencv::imgproc::LINE_8;
}
opencv_branch_34! {
	use opencv::core::LINE_8;
}

const SOURCE_WINDOW: &str = "Source image";

#[derive(Debug)]
enum DrawingState {
	Init,
	DrawingMarkerPoint,
	DrawingMarkerPointFinished,
	DrawingMask,
	DrawingMaskFinished,
	Resetting,
}

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	let argv = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

	let mut parser = CommandLineParser::new(&argv, "{@input | lena.jpg | input image}")?;
	parser.about("This program demonstrates using mouse events\n")?;
	parser.print_message()?;
	println!(
		"\n\tleft mouse button - set a point to create mask shape\n\
            \tright mouse button - create mask from points\n\
            \tmiddle mouse button - reset"
	);

	// HACK: replace with parser.get<String>("@input")
	let input_image = argv.into_iter().nth(2).unwrap_or("lena.jpg");
	let input_image_path = find_file(input_image, true, false)
		.map(|path| {
			println!("find input_image {} in : {}", input_image, path);
			path
		})
		.unwrap_or_else(|_| panic!("Cannot find input_image: {}", input_image));

	let [src, mut next_frame, mut mask, mut final_img]: [Mat; 4];
	src = imread(&input_image_path, IMREAD_COLOR)?;
	if src.empty() {
		eprintln!("Error opening image: {}", input_image);
		process::exit(-1);
	}

	highgui::named_window(SOURCE_WINDOW, highgui::WINDOW_AUTOSIZE)?;
	let mouse_event_data = (highgui::MouseEventTypes::EVENT_MOUSEWHEEL, 0, 0, 0);
	let (mouse_event_data, should_handle_mouse_event) = (Arc::new(Mutex::new(mouse_event_data)), Arc::new(AtomicBool::new(false)));

	let mouse_event_dispatcher = {
		let mouse_data = Arc::clone(&mouse_event_data);
		let should_handle_mouse_event = Arc::clone(&should_handle_mouse_event);

		move |event: i32, x: i32, y: i32, flags: i32| {
			// can intercept specific mouse events here to don't update the mouse_data
			if let Ok(mouse_event) = highgui::MouseEventTypes::try_from(event) {
				if let Ok(mut mouse_data) = mouse_data.lock() {
					*mouse_data = (mouse_event, x, y, flags);
				}
			}
			should_handle_mouse_event.store(true, Ordering::Relaxed);
		}
	};
	highgui::set_mouse_callback(SOURCE_WINDOW, Some(Box::new(mouse_event_dispatcher))).expect("Cannot set mouse callback");

	imshow(SOURCE_WINDOW, &src)?;

	let (mut marker_points, mut drawing_state) = (Vec::<Point>::new(), DrawingState::Init);

	next_frame = Mat::zeros_size(src.size()?, Vec3b::opencv_type())?.to_mat()?;

	loop {
		// Press Esc to exit
		if highgui::wait_key(10)? == 27 {
			break Ok(());
		}

		let (mouse_event, x, y, _) = {
			if !should_handle_mouse_event.load(Ordering::Relaxed) {
				continue;
			} else {
				should_handle_mouse_event.store(false, Ordering::Relaxed);

				if let Ok(mouse_event_data) = mouse_event_data.lock() {
					*mouse_event_data
				} else {
					continue;
				}
			}
		};

		drawing_state = state_transform(drawing_state, mouse_event);

		match drawing_state {
			DrawingState::Init | DrawingState::DrawingMarkerPointFinished => { /* do nothing */ }
			DrawingState::DrawingMarkerPoint => {
				if marker_points.is_empty() {
					next_frame = src.clone();
				}

				let point = Point::new(x, y);
				imgproc::circle(&mut next_frame, point, 2, Scalar::new(0., 0., 255., 0.), -1, LINE_8, 0)?;
				marker_points.push(point);

				if marker_points.len() > 1 {
					imgproc::line(
						&mut next_frame,
						marker_points[marker_points.len() - 2],
						point,
						Scalar::new(0., 0., 255., 0.),
						2,
						LINE_8,
						0,
					)?;
				}

				imshow(SOURCE_WINDOW, &next_frame)?;
			}
			DrawingState::DrawingMask => {
				if !marker_points.is_empty() {
					next_frame = src.clone();

					imgproc::polylines(
						&mut next_frame,
						&Mat::from_slice(marker_points.as_slice())?,
						true,
						Scalar::new(0., 0., 0., 0.),
						2,
						LINE_8,
						0,
					)?;

					imshow(SOURCE_WINDOW, &next_frame)?;
				}
			}
			DrawingState::DrawingMaskFinished => {
				if !marker_points.is_empty() {
					final_img = Mat::zeros_size(src.size()?, Vec3b::opencv_type())?.to_mat()?;
					mask = Mat::zeros_size(src.size()?, u8::opencv_type())?.to_mat()?;

					imgproc::fill_poly_def(&mut mask, &Mat::from_slice(marker_points.as_slice())?, Scalar::all(255.))?;

					bitwise_and(&src, &src, &mut final_img, &mask)?;

					imshow("Mask", &mask)?;
					imshow("Result", &final_img)?;
					imshow(SOURCE_WINDOW, &next_frame)?;
				}
			}
			DrawingState::Resetting => {
				if !marker_points.is_empty() {
					marker_points.clear();
					next_frame = src.clone();

					imshow(SOURCE_WINDOW, &next_frame)?;
				}
			}
		}
	}
}

fn state_transform(drawing_state: DrawingState, mouse_event: highgui::MouseEventTypes) -> DrawingState {
	use opencv::highgui::MouseEventTypes::*;

	use self::DrawingState::*;

	match (&drawing_state, mouse_event) {
		(Init, EVENT_LBUTTONDOWN) => DrawingMarkerPoint,
		(DrawingMarkerPoint, EVENT_LBUTTONUP) => DrawingMarkerPointFinished,
		(DrawingMarkerPointFinished, EVENT_LBUTTONDOWN) => DrawingMarkerPoint,
		(DrawingMarkerPointFinished, EVENT_RBUTTONDOWN) => DrawingMask,
		(DrawingMask, EVENT_RBUTTONUP) => DrawingMaskFinished,
		(Init | DrawingMarkerPointFinished | DrawingMaskFinished, EVENT_MBUTTONDOWN) => Resetting,
		(Resetting, EVENT_MBUTTONUP) => Init,
		_ => {
			println!(
				"Invalid state transition from {:?} with event {:?}",
				&drawing_state, mouse_event
			);
			drawing_state
		}
	}
}
