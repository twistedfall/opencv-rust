#![cfg(ocvrs_has_module_features2d)]

use std::path::PathBuf;

use opencv::{
	core::Size,
	features2d::ORB,
	imgcodecs,
	prelude::*,
	Result,
	types::VectorOfKeyPoint,
};

#[test]
fn orb() -> Result<()> {
	let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/blox.jpg");
	let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
	let mut orb = <dyn ORB>::default()?;
	let mut kp = VectorOfKeyPoint::new();
	let mut des = Mat::default();
	orb.detect_and_compute(&img, &Mat::default(), &mut kp, &mut des, false)?;
	let size = if cfg!(ocvrs_opencv_branch_32) { 296 } else { 290 };
	assert_eq!(size, kp.len());
	assert_eq!(Size::new(32, size as i32), des.size()?);
	Ok(())
}
