#![cfg(ocvrs_has_module_features2d)]

use opencv::core::{Size, Vector};
use opencv::prelude::*;
use opencv::{features2d, imgcodecs, Result};

static BLOX: &[u8] = include_bytes!("../tests/blox.jpg");

#[test]
fn orb() -> Result<()> {
	let img = imgcodecs::imdecode(&BLOX, imgcodecs::IMREAD_COLOR)?;
	let mut orb = features2d::ORB::create_def()?;
	let mut kp = Vector::new();
	let mut des = Mat::default();
	orb.detect_and_compute_def(&img, &Mat::default(), &mut kp, &mut des)?;
	let size = 290;
	assert_eq!(size, kp.len());
	assert_eq!(Size::new(32, i32::try_from(size)?), des.size()?);
	Ok(())
}
