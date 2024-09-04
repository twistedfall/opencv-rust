//! Port of https://github.com/opencv/opencv/blob/4.9.0/samples/dnn/face_detect.cpp
//! Tutorial: https://docs.opencv.org/4.9.0/d0/dd4/tutorial_dnn_face.html

use std::env;

use objdetect::FaceDetectorYN;
use opencv::core::{CommandLineParser, Point, Point2f, Rect2f, Size, StsBadArg, StsError, TickMeter};
use opencv::objdetect::{FaceRecognizerSF, FaceRecognizerSF_DisType};
use opencv::prelude::*;
use opencv::{core, highgui, imgcodecs, imgproc, objdetect, videoio, Error, Result};

fn visualize(input: &mut Mat, frame: i32, faces: &Mat, fps: f64, thickness: i32) -> Result<()> {
	let fps_string = format!("FPS : {:.2}", fps);
	if frame >= 0 {
		println!("Frame {}, ", frame);
	}
	println!("FPS: {}", fps_string);
	for i in 0..faces.rows() {
		// Print results
		println!(
			"Face {i}, top-left coordinates: ({}, {}), box width: {}, box height: {}, score: {:.2}",
			faces.at_2d::<f32>(i, 0)?,
			faces.at_2d::<f32>(i, 1)?,
			faces.at_2d::<f32>(i, 2)?,
			faces.at_2d::<f32>(i, 3)?,
			faces.at_2d::<f32>(i, 14)?
		);

		// Draw bounding box
		let rect = Rect2f::new(
			*faces.at_2d::<f32>(i, 0)?,
			*faces.at_2d::<f32>(i, 1)?,
			*faces.at_2d::<f32>(i, 2)?,
			*faces.at_2d::<f32>(i, 3)?,
		)
		.to::<i32>()
		.ok_or_else(|| Error::new(StsBadArg, "Invalid rect"))?;
		imgproc::rectangle(input, rect, (0., 255., 0.).into(), thickness, imgproc::LINE_8, 0)?;
		// Draw landmarks
		imgproc::circle(
			input,
			Point2f::new(*faces.at_2d::<f32>(i, 4)?, *faces.at_2d::<f32>(i, 5)?)
				.to::<i32>()
				.ok_or_else(|| Error::new(StsBadArg, "Invalid point"))?,
			2,
			(255., 0., 0.).into(),
			thickness,
			imgproc::LINE_8,
			0,
		)?;
		imgproc::circle(
			input,
			Point2f::new(*faces.at_2d::<f32>(i, 6)?, *faces.at_2d::<f32>(i, 7)?)
				.to::<i32>()
				.ok_or_else(|| Error::new(StsBadArg, "Invalid point"))?,
			2,
			(0., 0., 255.).into(),
			thickness,
			imgproc::LINE_8,
			0,
		)?;
		imgproc::circle(
			input,
			Point2f::new(*faces.at_2d::<f32>(i, 8)?, *faces.at_2d::<f32>(i, 9)?)
				.to::<i32>()
				.ok_or_else(|| Error::new(StsBadArg, "Invalid point"))?,
			2,
			(0., 255., 0.).into(),
			thickness,
			imgproc::LINE_8,
			0,
		)?;
		imgproc::circle(
			input,
			Point2f::new(*faces.at_2d::<f32>(i, 10)?, *faces.at_2d::<f32>(i, 11)?)
				.to::<i32>()
				.ok_or_else(|| Error::new(StsBadArg, "Invalid point"))?,
			2,
			(255., 0., 255.).into(),
			thickness,
			imgproc::LINE_8,
			0,
		)?;
		imgproc::circle(
			input,
			Point2f::new(*faces.at_2d::<f32>(i, 12)?, *faces.at_2d::<f32>(i, 13)?)
				.to::<i32>()
				.ok_or_else(|| Error::new(StsBadArg, "Invalid point"))?,
			2,
			(0., 255., 255.).into(),
			thickness,
			imgproc::LINE_8,
			0,
		)?;
	}
	imgproc::put_text(
		input,
		&fps_string,
		Point::new(0, 15),
		imgproc::FONT_HERSHEY_SIMPLEX,
		0.5,
		(0., 255., 0.).into(),
		thickness,
		imgproc::LINE_8,
		false,
	)?;
	Ok(())
}

fn main() -> Result<()> {
	let args = env::args().collect::<Vec<_>>();
	let args = args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>();
	let parser = CommandLineParser::new(
		&args,
		concat!(
			"{help  h           |            | Print this message}",
			"{image1 i1         |            | Path to the input image1. Omit for detecting through VideoCapture}",
			"{image2 i2         |            | Path to the input image2. When image1 and image2 parameters given then the program try to find a face on both images and runs face recognition algorithm}",
			"{video v           | 0          | Path to the input video}",
			"{scale sc          | 1.0        | Scale factor used to resize input video frames}",
			"{fd_model fd       | face_detection_yunet_2021dec.onnx| Path to the model. Download yunet.onnx in https://github.com/opencv/opencv_zoo/tree/master/models/face_detection_yunet}",
			"{fr_model fr       | face_recognition_sface_2021dec.onnx | Path to the face recognition model. Download the model at https://github.com/opencv/opencv_zoo/tree/master/models/face_recognition_sface}",
			"{score_threshold   | 0.9        | Filter out faces of score < score_threshold}",
			"{nms_threshold     | 0.3        | Suppress bounding boxes of iou >= nms_threshold}",
			"{top_k             | 5000       | Keep top_k bounding boxes before NMS}",
			"{save s            | false      | Set true to save results. This flag is invalid when using camera}"),
	)?;
	if parser.has("help")? {
		parser.print_message()?;
		return Ok(());
	}

	let fd_model_path = parser.get_str_def("fd_model")?;
	let fr_model_path = parser.get_str_def("fr_model")?;

	let score_threshold = parser.get_f64_def("score_threshold")? as f32;
	let nms_threshold = parser.get_f64_def("nms_threshold")? as f32;
	let top_k = parser.get_i32_def("top_k")?;

	let save = parser.get_bool_def("save")?;
	let scale = parser.get_f64_def("scale")?;

	let cosine_similar_thresh = 0.363;
	let l2norm_similar_thresh = 1.128;

	// Initialize FaceDetectorYN
	let mut detector = FaceDetectorYN::create(
		&fd_model_path,
		"",
		Size::new(320, 320),
		score_threshold,
		nms_threshold,
		top_k,
		0,
		0,
	)?;

	let mut tm = TickMeter::default()?;

	// If input is an image
	if parser.has("image1")? {
		let input1 = parser.get_str_def("image1")?;
		let image1 = imgcodecs::imread_def(&input1)?;
		if image1.empty() {
			eprintln!("Cannot read image: {}", input1);
			return Err(Error::new(StsBadArg, "Cannot read image"));
		}

		let image_width = (f64::from(image1.cols()) * scale) as i32;
		let image_height = (f64::from(image1.rows()) * scale) as i32;
		let mut image1_out = Mat::default();
		imgproc::resize(
			&image1,
			&mut image1_out,
			Size::new(image_width, image_height),
			0.,
			0.,
			imgproc::INTER_LINEAR,
		)?;
		let mut image1 = image1_out;

		tm.start()?;

		// Set input size before inference
		detector.set_input_size(image1.size()?)?;

		let mut faces1 = Mat::default();
		detector.detect(&image1, &mut faces1)?;
		if faces1.rows() < 1 {
			eprintln!("Cannot find a face in {input1}");
			return Err(Error::new(StsError, "Cannot find a face"));
		}

		tm.stop()?;

		// Draw results on the input image
		visualize(&mut image1, -1, &faces1, tm.get_fps()?, 2)?;

		// Save results if save is true
		if save {
			println!("Saving result.jpg...");
			imgcodecs::imwrite_def("result.jpg", &image1)?;
		}

		// Visualize results
		highgui::imshow("image1", &image1)?;
		highgui::poll_key()?;

		if parser.has("image2")? {
			let input2 = parser.get_str_def("image2")?;
			let mut image2 = imgcodecs::imread_def(&input2)?;
			if image2.empty() {
				eprintln!("Cannot read image2: {input2}");
				return Err(Error::new(StsBadArg, "Cannot read image2"));
			}

			tm.reset()?;
			tm.start()?;
			detector.set_input_size(image2.size()?)?;

			let mut faces2 = Mat::default();
			detector.detect(&image2, &mut faces2)?;
			if faces2.rows() < 1 {
				eprintln!("Cannot find a face in {input2}");
				return Err(Error::new(StsError, "Cannot find a face"));
			}
			tm.stop()?;
			visualize(&mut image2, -1, &faces2, tm.get_fps()?, 2)?;
			if save {
				println!("Saving result2.jpg...");
				imgcodecs::imwrite_def("result2.jpg", &image2)?;
			}
			highgui::imshow("image2", &image2)?;
			highgui::poll_key()?;

			// Initialize FaceRecognizerSF
			let mut face_recognizer = FaceRecognizerSF::create_def(&fr_model_path, "")?;

			// Aligning and cropping facial image through the first face of faces detected.
			let mut aligned_face1 = Mat::default();
			let mut aligned_face2 = Mat::default();
			face_recognizer.align_crop(&image1, &faces1.row(0)?, &mut aligned_face1)?;
			face_recognizer.align_crop(&image2, &faces2.row(0)?, &mut aligned_face2)?;

			// Run feature extraction with given aligned_face
			let mut feature1 = Mat::default();
			let mut feature2 = Mat::default();
			face_recognizer.feature(&aligned_face1, &mut feature1)?;
			let feature1 = feature1.try_clone()?;
			face_recognizer.feature(&aligned_face2, &mut feature2)?;
			let feature2 = feature2.try_clone()?;

			// match
			let cos_score = face_recognizer.match_(&feature1, &feature2, FaceRecognizerSF_DisType::FR_COSINE.into())?;
			let l2_score = face_recognizer.match_(&feature1, &feature2, FaceRecognizerSF_DisType::FR_NORM_L2.into())?;

			if cos_score >= cosine_similar_thresh {
				println!("They have the same identity;");
			} else {
				println!("They have different identities;");
			}
			println!(
				"Cosine Similarity: {cos_score}, threshold: {cosine_similar_thresh}. (higher value means higher similarity, max 1.0)"
			);

			if l2_score <= l2norm_similar_thresh {
				println!("They have the same identity;");
			} else {
				println!("They have different identities.");
			}
			println!(
				"NormL2 Distance: {l2_score}, threshold: {l2norm_similar_thresh}. (lower value means higher similarity, min 0.0)"
			);
		}
		println!("Press any key to exit...");
		highgui::wait_key(0)?;
	} else {
		let frame_width;
		let frame_height;
		let mut capture = videoio::VideoCapture::default()?;
		let video = parser.get_str_def("video")?;
		if video.len() == 1 && video.chars().next().map_or(false, |c| c.is_ascii_digit()) {
			capture.open_def(parser.get_i32_def("video")?)?;
		} else {
			capture.open_file_def(&core::find_file_or_keep_def(&video)?)?;
		}
		if capture.is_opened()? {
			frame_width = (capture.get(videoio::CAP_PROP_FRAME_WIDTH)? * scale) as i32;
			frame_height = (capture.get(videoio::CAP_PROP_FRAME_HEIGHT)? * scale) as i32;
			println!("Video {video}: width={frame_width}, height={frame_height}");
		} else {
			println!("Could not initialize video capturing: {video}");
			return Err(Error::new(StsError, "Could not initialize video capturing"));
		}

		detector.set_input_size(Size::new(frame_width, frame_height))?;

		println!("Press 'SPACE' to save frame, any other key to exit...");
		let mut n_frame = 0;
		loop {
			// Get frame
			let mut frame = Mat::default();
			if !capture.read(&mut frame)? {
				eprintln!("Can't grab frame! Stop");
				break;
			}

			let mut frame_out = Mat::default();
			imgproc::resize_def(&frame, &mut frame_out, Size::new(frame_width, frame_height))?;
			let frame = frame_out;

			// Inference
			let mut faces = Mat::default();
			tm.start()?;
			detector.detect(&frame, &mut faces)?;
			tm.stop()?;

			let mut result = frame.try_clone()?;
			// Draw results on the input image
			visualize(&mut result, n_frame, &faces, tm.get_fps()?, 2)?;

			// Visualize results
			highgui::imshow("Live", &result)?;
			let mut key = highgui::wait_key(1)?;
			let mut save_frame = save;
			if key == ' ' as i32 {
				save_frame = true;
				key = 0; // handled
			}

			if save_frame {
				let frame_name = format!("frame_{:05}.png", n_frame);
				let result_name = format!("result_{:05}.jpg", n_frame);
				println!("Saving '{frame_name}' and '{result_name}' ...");
				imgcodecs::imwrite_def(&frame_name, &frame)?;
				imgcodecs::imwrite_def(&result_name, &result)?;
			}

			n_frame += 1;
			if key > 0 {
				break;
			}
		}
		println!("Processed {n_frame} frames");
	}
	println!("Done.");
	Ok(())
}
