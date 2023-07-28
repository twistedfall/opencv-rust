use std::{env, time};

use opencv::core::{UMat, UMatUsageFlags};
use opencv::prelude::*;
use opencv::{core, imgcodecs, imgproc, types, Result};

opencv::opencv_branch_4! {
	use opencv::core::AccessFlag::ACCESS_READ;
}
opencv::not_opencv_branch_4! {
	use opencv::core::ACCESS_READ;
}

const ITERATIONS: usize = 100;

fn main() -> Result<()> {
	let img_file = env::args().nth(1).expect("Please supply image file name");
	let opencl_have = core::have_opencl()?;
	if opencl_have {
		core::set_use_opencl(true)?;
		let mut platforms = types::VectorOfPlatformInfo::new();
		core::get_platfoms_info(&mut platforms)?;
		for (platf_num, platform) in platforms.into_iter().enumerate() {
			println!("Platform #{}: {}", platf_num, platform.name()?);
			for dev_num in 0..platform.device_number()? {
				let mut dev = core::Device::default();
				platform.get_device(&mut dev, dev_num)?;
				println!("  OpenCL device #{}: {}", dev_num, dev.name()?);
				println!("    vendor:  {}", dev.vendor_name()?);
				println!("    version: {}", dev.version()?);
			}
		}
	}
	let opencl_use = core::use_opencl()?;
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
	if opencl_use {
		println!("Timing OpenCL implementation... ");
		let mat = imgcodecs::imread(&img_file, imgcodecs::IMREAD_COLOR)?;
		let img = mat.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
		let start = time::Instant::now();
		for _ in 0..ITERATIONS {
			let mut gray = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
			imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
			let mut blurred = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
			imgproc::gaussian_blur(&gray, &mut blurred, core::Size::new(7, 7), 1.5, 0., core::BORDER_DEFAULT)?;
			let mut edges = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
			imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;
		}
		println!("{:#?}", start.elapsed());
	}
	Ok(())
}
