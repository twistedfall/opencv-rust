#![cfg(ocvrs_has_module_features2d)]

use std::path::PathBuf;

use opencv::core::Size;
use opencv::prelude::*;
use opencv::types::VectorOfKeyPoint;
use opencv::{features2d, imgcodecs, Result};

#[test]
fn orb() -> Result<()> {
	let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/blox.jpg");
	let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
	let mut orb = features2d::ORB::create_def()?;
	let mut kp = VectorOfKeyPoint::new();
	let mut des = Mat::default();
	orb.detect_and_compute_def(&img, &Mat::default(), &mut kp, &mut des)?;
	let size = 290;
	assert_eq!(size, kp.len());
	assert_eq!(Size::new(32, size as i32), des.size()?);
	Ok(())
}
