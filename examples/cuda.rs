use std::env;
use std::time::Instant;

use opencv::core::Size;
use opencv::prelude::*;
use opencv::{imgcodecs, imgproc, opencv_has_inherent_feature_cuda, opencv_has_module_cudafilters, Result};

const ITERATIONS: usize = 100;

fn main() -> Result<()> {
	let img_file = env::args().nth(1).expect("Please supply image file name");

	let dev_count = opencv_has_inherent_feature_cuda! {
		{
			opencv::core::get_cuda_enabled_device_count()?
		} else {
			0
		}
	};
	let cuda_available = dev_count > 0;
	if cuda_available {
		for dev_num in 0..dev_count {
			opencv_has_inherent_feature_cuda! {
				{
					opencv::core::print_short_cuda_device_info(dev_num)?;
				} else {
					println!("CUDA device {dev_num} is not available");
				}
			}
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
	let img = imgcodecs::imread_def(&img_file)?;
	let start = Instant::now();
	for _ in 0..ITERATIONS {
		let mut gray = Mat::default();
		imgproc::cvt_color_def(&img, &mut gray, imgproc::COLOR_BGR2GRAY)?;
		let mut blurred = Mat::default();
		imgproc::gaussian_blur_def(&gray, &mut blurred, Size::new(7, 7), 1.5)?;
		let mut edges = Mat::default();
		imgproc::canny_def(&blurred, &mut edges, 0., 50.)?;
	}
	println!("{:#?}", start.elapsed());
	if cuda_available {
		opencv_has_module_cudafilters! {
			opencv::opencv_has_module_cudaimgproc! {
				use opencv::core::{GpuMat, Stream};
				use opencv::{cudafilters, cudaimgproc};

				println!("Timing CUDA implementation... ");
				let img = imgcodecs::imread_def(&img_file)?;
				let mut img_gpu = GpuMat::new_def()?;
				img_gpu.upload(&img)?;
				let mut stream = Stream::default()?;
				let start = Instant::now();
				for _ in 0..ITERATIONS {
					let mut gray = GpuMat::new_def()?;
					cudaimgproc::cvt_color(&img_gpu, &mut gray, imgproc::COLOR_BGR2GRAY, 0, &mut stream)?;
					let mut blurred = GpuMat::new_def()?;
					let mut filter = cudafilters::create_gaussian_filter_def(gray.typ()?, blurred.typ()?, Size::new(7, 7), 1.5)?;
					filter.apply(&gray, &mut blurred, &mut stream)?;
					let mut edges = GpuMat::new_def()?;
					let mut detector = cudaimgproc::create_canny_edge_detector_def(0., 50.)?;
					detector.detect(&blurred, &mut edges, &mut stream)?;
					stream.wait_for_completion()?;
				}
				println!("{:#?}", start.elapsed());
			}
		}
	}
	Ok(())
}
