use std::{env, time};

use opencv::core::{GpuMat, Size};
use opencv::prelude::*;
use opencv::{core, cudafilters, cudaimgproc, imgcodecs, imgproc, Result};

const ITERATIONS: usize = 100;

fn main() -> Result<()> {
	let img_file = env::args().nth(1).expect("Please supply image file name");

	let dev_count = core::get_cuda_enabled_device_count()?;
	let cuda_available = dev_count > 0;
	if cuda_available {
		for dev_num in 0..dev_count {
			core::print_short_cuda_device_info(dev_num)?;
		}
	}
	println!(
		"CUDA is {}",
		if cuda_available {
			"available"
		} else {
			"not available"
		}
	);
	println!("Timing CPU implementation... ");
	let img = imgcodecs::imread(&img_file, imgcodecs::IMREAD_COLOR)?;
	let start = time::Instant::now();
	for _ in 0..ITERATIONS {
		let mut gray = Mat::default();
		imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
		let mut blurred = Mat::default();
		imgproc::gaussian_blur(&gray, &mut blurred, core::Size::new(7, 7), 1.5, 0., core::BORDER_DEFAULT)?;
		let mut edges = Mat::default();
		imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;
	}
	println!("{:#?}", start.elapsed());
	if cuda_available {
		println!("Timing CUDA implementation... ");
		let img = imgcodecs::imread(&img_file, imgcodecs::IMREAD_COLOR)?;
		let mut img_gpu = GpuMat::default()?;
		img_gpu.upload(&img)?;
		let mut stream = core::Stream::default()?;
		let start = time::Instant::now();
		for _ in 0..ITERATIONS {
			let mut gray = GpuMat::default()?;
			cudaimgproc::cvt_color(&img_gpu, &mut gray, imgproc::COLOR_BGR2GRAY, 0, &mut stream)?;
			let mut blurred = GpuMat::default()?;
			let mut filter = cudafilters::create_gaussian_filter(
				gray.typ()?,
				blurred.typ()?,
				Size::new(7, 7),
				1.5,
				0.,
				core::BORDER_DEFAULT,
				core::BORDER_DEFAULT,
			)?;
			filter.apply(&gray, &mut blurred, &mut stream)?;
			let mut edges = GpuMat::default()?;
			let mut detector = cudaimgproc::create_canny_edge_detector(0., 50., 3, false)?;
			detector.detect(&blurred, &mut edges, &mut stream)?;
			stream.wait_for_completion()?;
		}
		println!("{:#?}", start.elapsed());
	}
	Ok(())
}
