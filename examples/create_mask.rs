//! # Create Mask
//! Reference: [opencv/samples/cpp/create_mask.cpp](https://github.com/opencv/opencv/blob/4.x/samples/cpp/create_mask.cpp)

use opencv::{
	core::{bitwise_and, find_file, CommandLineParser, Point, Scalar, CV_8UC1, CV_8UC3},
	highgui::{self, *},
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
	let input_image = "lena.jpg";
	let input_image_path = find_file(input_image, true, false).expect("Cannot find input image");

	let [src, mut img1, mut mask, mut final_img]: [Mat; 4];
	src = imread(&input_image_path, IMREAD_COLOR).unwrap();
	if src.empty() {
		eprintln!("Error opening image: {}", input_image);
		process::exit(-1);
	}

	highgui::named_window(SOURCE_WINDOW, WINDOW_AUTOSIZE).unwrap();
	let mouse_event_data: (i32, i32, i32, i32) = Default::default();
	let (mouse_event_data, should_handle_mouse_event) = (Arc::new(Mutex::new(mouse_event_data)), Arc::new(AtomicBool::new(false)));

	let mouse_event_dispatcher = {
		let mouse_data = Arc::clone(&mouse_event_data);
		let should_handle_mouse_event = Arc::clone(&should_handle_mouse_event);

		move |event: i32, x: i32, y: i32, flags: i32| {
			// can intercept specific mouse events here to don't update the mouse_data
			if let Ok(mut mouse_data) = mouse_data.lock() {
				*mouse_data = (event, x, y, flags);
			}
			should_handle_mouse_event.store(true, atomic::Ordering::Relaxed);
		}
	};
	highgui::set_mouse_callback(SOURCE_WINDOW, Some(Box::new(mouse_event_dispatcher))).expect("Cannot set mouse callback");

	highgui::imshow(SOURCE_WINDOW, &src).unwrap();

	let (mut num_points_has_add, mut drag_left_button, mut right_button_down) = (0, false, false);
	let mut points = Vec::<Point>::new();

	img1 = Mat::new_size_with_default(src.size().unwrap(), CV_8UC3, Scalar::new(0.0, 0.0, 0.0, 0.0)).unwrap();

	loop {
		// Press Esc to exit
		if highgui::wait_key(10).unwrap() == 27 {
			break;
		}

		let (mouse_event, x, y, _) = {
			if !should_handle_mouse_event.load(atomic::Ordering::Relaxed) {
				continue;
			}
			should_handle_mouse_event.store(false, atomic::Ordering::Relaxed);

			if let Ok(mouse_event_data) = mouse_event_data.lock() {
				*mouse_event_data
			} else {
				continue;
			}
		};

		match mouse_event {
			EVENT_LBUTTONDOWN => {
				dbg!((mouse_event, x, y));

				if !drag_left_button && !right_button_down {
					if num_points_has_add == 0 {
						img1 = src.clone();
					}

					let point = Point::new(x, y);
					imgproc::circle(&mut img1, point, 2, Scalar::new(0., 0., 255., 0.), -1, imgproc::LINE_8, 0).unwrap();
					points.push(point);
					num_points_has_add += 1;
					drag_left_button = true;

					if num_points_has_add > 1 {
						imgproc::line(
							&mut img1,
							points[num_points_has_add - 2],
							point,
							Scalar::new(0., 0., 255., 0.),
							2,
							imgproc::LINE_8,
							0,
						)
						.unwrap();
					}

					imshow(SOURCE_WINDOW, &img1).unwrap();
				}
			}
			EVENT_LBUTTONUP => {
				dbg!((mouse_event, x, y));

				if drag_left_button {
					imshow(SOURCE_WINDOW, &img1).unwrap();
					drag_left_button = false;
				}
			}
			EVENT_RBUTTONDOWN => {
				dbg!((mouse_event, x, y));

				right_button_down = true;
				let mut img1 = src.clone();

				if num_points_has_add > 0 {
					let pts_mat = Mat::from_slice(points.as_slice()).unwrap();
					imgproc::polylines(&mut img1, &pts_mat, true, Scalar::new(0., 0., 0., 0.), 2, imgproc::LINE_8, 0).unwrap();
				}

				imshow(SOURCE_WINDOW, &img1).unwrap();
			}
			EVENT_RBUTTONUP => {
				dbg!((mouse_event, x, y));

				right_button_down = false;
				final_img = Mat::zeros_size(src.size().unwrap(), CV_8UC3).unwrap().to_mat().unwrap();

				mask = Mat::zeros_size(src.size().unwrap(), CV_8UC1).unwrap().to_mat().unwrap();

				let points_mat = Mat::from_slice(points.as_slice()).unwrap();
				imgproc::fill_poly(
					&mut mask,
					&points_mat,
					Scalar::new(255., 255., 255., 255.),
					imgproc::LINE_8,
					0,
					Point::default(),
				)
				.unwrap();

				bitwise_and(&src, &src, &mut final_img, &mask).unwrap();

				imshow("Mask", &mask).unwrap();
				imshow("Result", &final_img).unwrap();
				imshow(SOURCE_WINDOW, &img1).unwrap();
			}
			EVENT_MBUTTONDOWN => {
				dbg!((mouse_event, x, y));

				num_points_has_add = 0;
				points.clear();
				img1 = src.clone();
				imshow(SOURCE_WINDOW, &img1).unwrap();
			}
			_ => {}
		}
	}
}
