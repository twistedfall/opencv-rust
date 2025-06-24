use opencv::{opencv_has_inherent_feature_opencl, Result};

fn main() -> Result<()> {
	use std::{env, time};

	use opencv::core::{Size, UMat};
	use opencv::prelude::*;
	use opencv::{imgcodecs, imgproc};

	opencv::not_opencv_branch_34! {
		use opencv::core::AccessFlag::ACCESS_READ;
	}
	opencv::opencv_branch_34! {
		use opencv::core::ACCESS_READ;
	}

	const ITERATIONS: usize = 100;
	let img_file = env::args().nth(1).expect("Please supply image file name");
	let opencl_have = opencv_has_inherent_feature_opencl! {
		{
			opencv::core::have_opencl()?
		} else {
			false
		}
	};
	opencv_has_inherent_feature_opencl! {
		if opencl_have {
			opencv::core::set_use_opencl(true)?;
			let mut platforms = opencv::core::Vector::new();
			opencv::core::get_platfoms_info(&mut platforms)?;
			for (platf_num, platform) in platforms.into_iter().enumerate() {
				println!("Platform #{}: {}", platf_num, platform.name()?);
				for dev_num in 0..platform.device_number()? {
					let mut dev = opencv::core::Device::default();
					platform.get_device(&mut dev, dev_num)?;
					println!("  OpenCL device #{dev_num}: {}", dev.name()?);
					println!("    vendor:  {}", dev.vendor_name()?);
					println!("    version: {}", dev.version()?);
				}
			}
		}
	}
	let opencl_use = opencv_has_inherent_feature_opencl! {
		{
			opencv::core::use_opencl()?
		} else {
			false
		}
	};
	println!(
		"OpenCL is {} and {}",
		if opencl_have {
			"available"
		} else {
			"not available"
		},
		if opencl_use {
			"enabled"
		} else {
			"disabled"
		},
	);
	println!("Timing CPU implementation... ");
	let img = imgcodecs::imread_def(&img_file)?;
	let start = time::Instant::now();
	for _ in 0..ITERATIONS {
		let mut gray = Mat::default();
		imgproc::cvt_color_def(&img, &mut gray, imgproc::COLOR_BGR2GRAY)?;
		let mut blurred = Mat::default();
		imgproc::gaussian_blur_def(&gray, &mut blurred, Size::new(7, 7), 1.5)?;
		let mut edges = Mat::default();
		imgproc::canny_def(&blurred, &mut edges, 0., 50.)?;
	}
	println!("{:#?}", start.elapsed());
	if opencl_use {
		println!("Timing OpenCL implementation... ");
		let mat = imgcodecs::imread_def(&img_file)?;
		let img = mat.get_umat_def(ACCESS_READ)?;
		let start = time::Instant::now();
		for _ in 0..ITERATIONS {
			let mut gray = UMat::new_def();
			imgproc::cvt_color_def(&img, &mut gray, imgproc::COLOR_BGR2GRAY)?;
			let mut blurred = UMat::new_def();
			imgproc::gaussian_blur_def(&gray, &mut blurred, Size::new(7, 7), 1.5)?;
			let mut edges = UMat::new_def();
			imgproc::canny_def(&blurred, &mut edges, 0., 50.)?;
		}
		println!("{:#?}", start.elapsed());
	}
	Ok(())
}
