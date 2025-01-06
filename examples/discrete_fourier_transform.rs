//! Port of https://github.com/opencv/opencv/blob/master/samples/cpp/tutorial_code/core/discrete_fourier_transform/discrete_fourier_transform.cpp

use std::env;

use opencv::core::{self, Rect, Scalar};
use opencv::prelude::*;
use opencv::{highgui, imgcodecs};

fn main() -> opencv::Result<()> {
	#![allow(non_snake_case)]
	let filename = env::args().nth(1).expect("Must supply image filename");
	let I = imgcodecs::imread(&filename, imgcodecs::IMREAD_GRAYSCALE)?;
	if I.empty() {
		panic!("Error opening image: {filename}");
	}
	let mut padded = Mat::default();
	let m = core::get_optimal_dft_size(I.rows())?;
	let n = core::get_optimal_dft_size(I.cols())?;
	core::copy_make_border_def(&I, &mut padded, 0, m - I.rows(), 0, n - I.cols(), core::BORDER_CONSTANT)?;
	let plane_size = padded.size()?;
	let mut planes = core::Vector::<Mat>::new();
	let mut padded_f32 = Mat::default();
	padded.convert_to_def(&mut padded_f32, f32::opencv_type())?;
	planes.push(padded_f32);
	planes.push(Mat::zeros_size(plane_size, f32::opencv_type())?.to_mat()?);
	let mut complexI = Mat::default();
	core::merge(&planes, &mut complexI)?;
	let mut complexI_tmp = Mat::default();
	core::dft_def(&complexI, &mut complexI_tmp)?;
	complexI = complexI_tmp;
	core::split(&complexI, &mut planes)?;
	let mut magI = Mat::default();
	core::magnitude(&planes.get(0)?, &planes.get(1)?, &mut magI)?;
	let mut magI_tmp = Mat::default();
	core::add_def(&magI, &Scalar::all(1.), &mut magI_tmp)?;
	magI = magI_tmp;
	let mut magI_log = Mat::default();
	core::log(&magI, &mut magI_log)?;
	let magI_log_rect = Rect::new(0, 0, magI_log.cols() & -2, magI_log.rows() & -2);
	let mut magI = Mat::roi_mut(&mut magI_log, magI_log_rect)?;
	let cx = magI.cols() / 2;
	let cy = magI.rows() / 2;
	let (mut top, mut bottom) = Mat::roi_2_mut(
		&mut magI,
		Rect::new(0, 0, magI_log_rect.width, cy),
		Rect::new(cx, 0, magI_log_rect.width, cy),
	)?;
	let (mut q0, mut q1) = Mat::roi_2_mut(&mut top, Rect::new(0, 0, cx, cy), Rect::new(cx, 0, cx, cy))?;
	let (mut q2, mut q3) = Mat::roi_2_mut(&mut bottom, Rect::new(0, 0, cx, cy), Rect::new(cx, 0, cx, cy))?;
	let mut tmp = Mat::default();
	q0.copy_to(&mut tmp)?;
	q3.copy_to(&mut q0)?;
	tmp.copy_to(&mut q3)?;
	q1.copy_to(&mut tmp)?;
	q2.copy_to(&mut q1)?;
	tmp.copy_to(&mut q2)?;
	let mut magI_tmp = Mat::default();
	core::normalize(&magI, &mut magI_tmp, 0., 1., core::NORM_MINMAX, -1, &core::no_array())?;
	let magI = magI_tmp;
	highgui::imshow("Input Image", &I)?;
	highgui::imshow("spectrum magnitude", &magI)?;
	highgui::wait_key_def()?;
	Ok(())
}
