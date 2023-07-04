//! # Create Mask
//! Reference: [opencv/samples/cpp/create_mask.cpp](https://github.com/opencv/opencv/blob/4.x/samples/cpp/create_mask.cpp)

use opencv::{
	core::{bitwise_and, find_file, CommandLineParser, Point, Scalar, CV_8UC1, CV_8UC3},
	highgui::{self, imshow},
	imgcodecs::{imread, IMREAD_COLOR},
	imgproc,
	prelude::*,
};

use std::{
	env, process,
	sync::{
		atomic::{self, AtomicBool},
		Arc, Mutex,
	},
};

const SOURCE_WINDOW: &str = "Source image";

#[derive(Debug, Clone, Copy)]
enum DrawingState {
	Init,
	DrawingMarkerPoint,
	DrawingMarkerPointFinished,
	DrawingMask,
	DrawingMaskFinished,
	Resetting,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let (argc, argv) = (args.len() as i32, args.iter().map(|s| s.as_str()).collect::<Vec<&str>>());

	let mut parser = CommandLineParser::new(argc, &argv, "{@input | lena.jpg | input image}").unwrap();
	parser.about("This program demonstrates using mouse events\n").unwrap();
	parser.print_message().unwrap();
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
	src = imread(&input_image_path, IMREAD_COLOR).unwrap();
	if src.empty() {
		eprintln!("Error opening image: {}", input_image);
		process::exit(-1);
	}

	highgui::named_window(SOURCE_WINDOW, highgui::WINDOW_AUTOSIZE).unwrap();
	let mouse_event_data = (highgui::MouseEventTypes::EVENT_MOUSEWHEEL, 0, 0, 0);
	let (mouse_event_data, should_handle_mouse_event) = (Arc::new(Mutex::new(mouse_event_data)), Arc::new(AtomicBool::new(false)));

	let mouse_event_dispatcher = {
		let mouse_data = Arc::clone(&mouse_event_data);
		let should_handle_mouse_event = Arc::clone(&should_handle_mouse_event);

		move |event: i32, x: i32, y: i32, flags: i32| {
			// can intercept specific mouse events here to don't update the mouse_data
			if let Ok(mut mouse_data) = mouse_data.lock() {
				*mouse_data = (mouse_event_from_i32(event), x, y, flags);
			}
			should_handle_mouse_event.store(true, atomic::Ordering::Relaxed);
		}
	};
	highgui::set_mouse_callback(SOURCE_WINDOW, Some(Box::new(mouse_event_dispatcher))).expect("Cannot set mouse callback");

	highgui::imshow(SOURCE_WINDOW, &src).unwrap();

	let (mut marker_points, mut drawing_state) = (Vec::<Point>::new(), DrawingState::Init);

	next_frame = Mat::zeros_size(src.size().unwrap(), CV_8UC3).unwrap().to_mat().unwrap();

	loop {
		// Press Esc to exit
		if highgui::wait_key(10).unwrap() == 27 {
			break;
		}

		let (mouse_event, x, y, _) = {
			if should_handle_mouse_event.load(atomic::Ordering::Relaxed) {
				continue;
			} else {
				should_handle_mouse_event.store(false, atomic::Ordering::Relaxed);

				if let Ok(mouse_event_data) = mouse_event_data.lock() {
					*mouse_event_data
				} else {
					continue;
				}
			}
		};

		drawing_state = self::state_transform(drawing_state, mouse_event);

		match drawing_state {
			DrawingState::Init | DrawingState::DrawingMarkerPointFinished => { /* do nothing */ }
			DrawingState::DrawingMarkerPoint => {
				if marker_points.is_empty() {
					next_frame = src.clone();
				}

				let point = Point::new(x, y);
				imgproc::circle(
					&mut next_frame,
					point,
					2,
					Scalar::new(0., 0., 255., 0.),
					-1,
					imgproc::LINE_8,
					0,
				)
				.unwrap();
				marker_points.push(point);

				if marker_points.len() > 1 {
					imgproc::line(
						&mut next_frame,
						marker_points[marker_points.len() - 2],
						point,
						Scalar::new(0., 0., 255., 0.),
						2,
						imgproc::LINE_8,
						0,
					)
					.unwrap();
				}

				imshow(SOURCE_WINDOW, &next_frame).unwrap();
			}
			DrawingState::DrawingMask => {
				if !marker_points.is_empty() {
					next_frame = src.clone();

					let pts_mat = Mat::from_slice(marker_points.as_slice()).unwrap();
					imgproc::polylines(
						&mut next_frame,
						&pts_mat,
						true,
						Scalar::new(0., 0., 0., 0.),
						2,
						imgproc::LINE_8,
						0,
					)
					.unwrap();

					imshow(SOURCE_WINDOW, &next_frame).unwrap();
				}
			}
			DrawingState::DrawingMaskFinished => {
				if !marker_points.is_empty() {
					final_img = Mat::zeros_size(src.size().unwrap(), CV_8UC3).unwrap().to_mat().unwrap();
					mask = Mat::zeros_size(src.size().unwrap(), CV_8UC1).unwrap().to_mat().unwrap();

					imgproc::fill_poly(
						&mut mask,
						&Mat::from_slice(marker_points.as_slice()).unwrap(),
						Scalar::new(255., 255., 255., 255.),
						imgproc::LINE_8,
						0,
						Point::default(),
					)
					.unwrap();

					bitwise_and(&src, &src, &mut final_img, &mask).unwrap();

					imshow("Mask", &mask).unwrap();
					imshow("Result", &final_img).unwrap();
					imshow(SOURCE_WINDOW, &next_frame).unwrap();
				}
			}
			DrawingState::Resetting => {
				if !marker_points.is_empty() {
					marker_points.clear();
					next_frame = src.clone();

					imshow(SOURCE_WINDOW, &next_frame).unwrap();
				}
			}
		}
	}
}

/// Converts an `i32` to a `opencv::highgui::MouseEventTypes`
///
/// # Panics
///
/// Panics if the argument less than 0 or greater than 11.
fn mouse_event_from_i32(value: i32) -> opencv::highgui::MouseEventTypes {
	(value.gt(&(opencv::highgui::MouseEventTypes::EVENT_MOUSEHWHEEL as i32) /* 11 */)
		|| (value.lt(&(opencv::highgui::MouseEventTypes::EVENT_MOUSEMOVE as i32) /* 0 */)))
	.then(|| panic!("Invalid cv::highgui::MouseEventTypes value: {}", value));

	// Safe because of the previous check
	unsafe { std::mem::transmute(value) }
}

fn state_transform(drawing_state: DrawingState, mouse_event: highgui::MouseEventTypes) -> DrawingState {
	use self::DrawingState::*;
	use opencv::highgui::MouseEventTypes::*;

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
				drawing_state, mouse_event
			);
			drawing_state
		}
	}
}
