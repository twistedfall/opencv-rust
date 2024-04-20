#![cfg(ocvrs_has_module_features2d)]

use opencv::core::{no_array, Size, Vector, NORM_HAMMING};
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

#[test]
fn orb_bruteforce_match() -> Result<()> {
	let img_a = imgcodecs::imdecode(&BLOX, imgcodecs::IMREAD_GRAYSCALE)?;
	let img_b = img_a.clone(); // yep this is the same

	let mut orb = features2d::ORB::create_def()?;
	let mut kp_a = Vector::new();
	let mut des_a = Mat::default();
	orb.detect_and_compute(&img_a, &Mat::default(), &mut kp_a, &mut des_a, false)?;

	let mut kp_b = Vector::new();
	let mut des_b = Mat::default();
	orb.detect_and_compute(&img_b, &Mat::default(), &mut kp_b, &mut des_b, false)?;

	let size = 290;
	assert_eq!(size, kp_a.len());
	assert_eq!(Size::new(32, size as i32), des_a.size()?);
	assert_eq!(size, kp_b.len());
	assert_eq!(Size::new(32, size as i32), des_b.size()?);

	let bf_matcher = features2d::BFMatcher::create(NORM_HAMMING, true).unwrap();

	let mut matches = opencv::types::VectorOfDMatch::new();
	bf_matcher.train_match(&des_a, &des_b, &mut matches, &no_array()).unwrap();

	assert_ne!(matches.len(), 0); // expected many matches since images are equal
	Ok(())
}
