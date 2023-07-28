//! Port of https://github.com/opencv/opencv/blob/master/samples/dnn/text_detection.cpp
//! Check the source cpp file for where to get the NN files.
//! This example requires at least OpenCV 4.5.1.

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use dnn::{TextDetectionModel_EAST, TextRecognitionModel};
use opencv::core::{CommandLineParser, Point, Point2f, Scalar, Size, Vector};
use opencv::prelude::*;
use opencv::{core, dnn, highgui, imgproc, videoio};
use videoio::VideoCapture;

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

const KEYS: &str = concat!(
	"{ help  h              | | Print help message. }",
	"{ input i              | | Path to input image or video file. Skip this argument to capture frames from a camera.}",
	"{ detModel dmp         | | Path to a binary .pb file contains trained detector network.}",
	"{ width                | 320 | Preprocess input image by resizing to a specific width. It should be multiple by 32. }",
	"{ height               | 320 | Preprocess input image by resizing to a specific height. It should be multiple by 32. }",
	"{ thr                  | 0.5 | Confidence threshold. }",
	"{ nms                  | 0.4 | Non-maximum suppression threshold. }",
	"{ recModel rmp         | | Path to a binary .onnx file contains trained CRNN text recognition model. ",
	"Download links are provided in doc/tutorials/dnn/dnn_text_spotting/dnn_text_spotting.markdown}",
	"{ RGBInput rgb         |0| 0: imread with flags=IMREAD_GRAYSCALE; 1: imread with flags=IMREAD_COLOR. }",
	"{ vocabularyPath vp    | alphabet_36.txt | Path to benchmarks for evaluation. ",
	"Download links are provided in doc/tutorials/dnn/dnn_text_spotting/dnn_text_spotting.markdown}",
);

pub fn main() -> Result<()> {
	let args = env::args().collect::<Vec<_>>();
	let args = args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>();
	let mut parser = CommandLineParser::new(i32::try_from(args.len()).expect("Too many arguments"), &args, KEYS)?;
	parser.about(
		"Use this script to run TensorFlow implementation (https://github.com/argman/EAST) of \
		EAST: An Efficient and Accurate Scene Text Detector (https://arxiv.org/abs/1704.03155v2)",
	)?;
	if args.len() == 1 || parser.has("help")? {
		parser.print_message()?;
		return Ok(());
	}

	// Parameters.
	let conf_threshold = parser.get_f64("thr", true)? as f32;
	let nms_threshold = parser.get_f64("nms", true)? as f32;
	let width = parser.get_i32("width", true)?;
	let height = parser.get_i32("height", true)?;
	let imread_rgb = parser.get_bool("RGBInput", true)?;
	let det_model_path = parser.get_str("detModel", true)?;
	let rec_model_path = parser.get_str("recModel", true)?;
	let voc_path = parser.get_str("vocabularyPath", true)?;

	// Load networks.
	let mut detector = TextDetectionModel_EAST::from_file(&det_model_path, "")?;
	detector
		.set_confidence_threshold(conf_threshold)?
		.set_nms_threshold(nms_threshold)?;
	let mut recognizer = TextRecognitionModel::from_file(&rec_model_path, "")?;

	// Load vocabulary
	let mut vocabulary = Vector::<String>::new();
	let voc_file = BufReader::new(File::open(voc_path)?);
	for voc_line in voc_file.lines() {
		vocabulary.push(&voc_line?);
	}
	recognizer.set_vocabulary(&vocabulary)?.set_decode_type("CTC-greedy")?;

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
	let mut cap = VideoCapture::new(0, videoio::CAP_ANY)?;
	let open_success = VideoCapture::is_opened(&cap)?;
	if !open_success {
		panic!("Unable to open default camera!");
	}
	let win_name = "EAST: An Efficient and Accurate Scene Text Detector";
	let mut frame = Mat::default();
	while highgui::wait_key(1)? < 0 {
		cap.read(&mut frame)?;
		if frame.empty() {
			highgui::wait_key(0)?;
			break;
		}
		println!("Frame size: {:?}", frame.size()?);
		// Detection
		let mut det_results = Vector::new();
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
			let mut contours = Vector::<Vector<Point>>::new();
			for quadrangle in &det_results {
				let mut quadrangle_2f = Vector::<Point2f>::new();
				for pt in &quadrangle {
					quadrangle_2f.push(Point2f::new(pt.x as f32, pt.y as f32))
				}
				let cropped = four_points_transform(rec_input.as_ref().unwrap_or(&frame), quadrangle_2f.as_slice())?;
				let recognition_result = recognizer.recognize(&cropped)?;
				println!("Recognition result: {recognition_result}");
				imgproc::put_text(
					&mut frame,
					&recognition_result,
					quadrangle.get(3)?,
					imgproc::FONT_HERSHEY_SIMPLEX,
					1.5,
					Scalar::from((0., 0., 255.)),
					2,
					imgproc::LINE_8,
					false,
				)?;
				contours.push(quadrangle);
			}
			imgproc::polylines(
				&mut frame,
				&contours,
				true,
				Scalar::from((0., 255., 0.)),
				2,
				imgproc::LINE_8,
				0,
			)?;
		}
		let mut big_frame = Mat::default();
		imgproc::resize(&frame, &mut big_frame, Size::default(), 3., 3., imgproc::INTER_NEAREST)?;
		highgui::imshow(win_name, &big_frame)?;
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
	let rotation_matrix = imgproc::get_perspective_transform_slice(vertices, &target_vertices, core::DECOMP_LU)?;
	let mut out = Mat::default();
	imgproc::warp_perspective(
		frame,
		&mut out,
		&rotation_matrix,
		output_size,
		imgproc::INTER_LINEAR,
		core::BORDER_CONSTANT,
		Scalar::default(),
	)?;
	Ok(out)
}
