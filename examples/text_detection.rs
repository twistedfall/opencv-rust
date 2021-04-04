//! Port of https://github.com/opencv/opencv/blob/master/samples/dnn/text_detection.cpp
//! Check the source cpp file for where to get the NN files.
//! This example requires at least OpenCV 4.5.1.

use std::{
	error::Error,
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
};

use opencv::{
	core::{self, Point2f, Scalar, Size},
	dnn,
	highgui,
	imgproc,
	prelude::*,
	types::{VectorOfPoint2f, VectorOfString, VectorOfVectorOfPoint},
	videoio,
};

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
	// Parameters.
	let conf_threshold = 0.5;
	let nms_threshold = 0.4;
	let width = 320;
	let height = 320;
	let imread_rgb = false;
	let det_model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frozen_east_text_detection.pb");
	let rec_model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("crnn.onnx");
	let voc_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("alphabet_36.txt");

	// Load networks.
	let mut detector = dnn::TextDetectionModel_EAST::from_file(det_model_path.to_str().unwrap(), "")?;
	detector.set_confidence_threshold(conf_threshold)?
		.set_nms_threshold(nms_threshold)?;
	let mut recognizer = dnn::TextRecognitionModel::from_file(rec_model_path.to_str().unwrap(), "")?;

	// Load vocabulary
	let mut vocabulary = VectorOfString::new();
	let voc_file = BufReader::new(File::open(voc_path)?);
	for voc_line in voc_file.lines() {
		vocabulary.push(&voc_line?);
	}
	recognizer.set_vocabulary(&vocabulary)?
		.set_decode_type("CTC-greedy")?;

	// Parameters for Recognition
	let rec_scale = 1. / 127.5;
	let rec_mean = Scalar::from((127.5, 127.5, 127.5));
	let rec_input_size = Size::new(100, 32);
	recognizer.set_input_params(rec_scale, rec_input_size, rec_mean, false, false)?;

	// Parameters for Detection
	let det_scale = 1.;
	let det_input_size = Size::new(width, height);
	let det_mean = Scalar::from((123.68, 116.78, 103.94));
	let swap_rb = true;
	detector.set_input_params(det_scale, det_input_size, det_mean, swap_rb, false)?;

	// Open a video file or an image file or a camera stream.
	let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
	let open_success = videoio::VideoCapture::is_opened(&cap)?;
	if !open_success {
		panic!("Unable to open default camera!");
	}
	let win_name = "EAST: An Efficient and Accurate Scene Text Detector";
	let mut frame = Mat::default();
	while highgui::wait_key(1)? < 0 {
		cap.read(&mut frame)?;
		if frame.empty()? {
			highgui::wait_key(0)?;
			break;
		}
		println!("Frame size: {:?}", frame.size()?);
		// Detection
		let mut det_results = VectorOfVectorOfPoint::new();
		detector.detect(&frame, &mut det_results)?;

		if !det_results.is_empty() {
			// Text Recognition
			let rec_input = if !imread_rgb {
				let mut rec_input = Mat::default();
				imgproc::cvt_color(&frame, &mut rec_input, imgproc::COLOR_BGR2GRAY, 0)?;
				Some(rec_input)
			} else {
				None
			};
			let mut contours = VectorOfVectorOfPoint::new();
			for quadrangle in &det_results {
				let mut quadrangle_2f = VectorOfPoint2f::new();
				for pt in &quadrangle {
					quadrangle_2f.push(Point2f::new(pt.x as f32, pt.y as f32))
				}
				let cropped = four_points_transform(rec_input.as_ref().unwrap_or(&frame), quadrangle_2f.as_slice())?;
				let recognition_result = recognizer.recognize(&cropped)?;
				println!("Recognition result: {}", recognition_result);
				imgproc::put_text(&mut frame, &recognition_result, quadrangle.get(3)?, imgproc::FONT_HERSHEY_SIMPLEX, 1.5, Scalar::from((0., 0., 255.)), 2, imgproc::LINE_8, false)?;
				contours.push(quadrangle);
			}
			imgproc::polylines(&mut frame, &contours, true, Scalar::from((0., 255., 0.)), 2, imgproc::LINE_8, 0)?;
		}
		let mut big_frame = Mat::default();
		imgproc::resize(&frame, &mut big_frame, Size::default(), 3., 3., imgproc::INTER_NEAREST)?;
		highgui::imshow(win_name, &mut big_frame)?;
	}
	Ok(())
}

fn four_points_transform(frame: &Mat, vertices: &[Point2f]) -> Result<Mat> {
	let output_size = Size::new(100, 32);
	let target_vertices = [
		Point2f::new(0., (output_size.height - 1) as f32),
		Point2f::new(0., 0.),
		Point2f::new((output_size.width - 1) as f32, 0.),
		Point2f::new((output_size.width - 1) as f32, (output_size.height - 1) as f32),
	];
	let rotation_matrix = imgproc::get_perspective_transform_slice(&vertices, &target_vertices, core::DECOMP_LU)?;
	let mut out = Mat::default();
	imgproc::warp_perspective(frame, &mut out, &rotation_matrix, output_size, imgproc::INTER_LINEAR, core::BORDER_CONSTANT, Scalar::default())?;
	Ok(out)
}
