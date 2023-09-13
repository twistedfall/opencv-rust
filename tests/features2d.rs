#![cfg(ocvrs_has_module_features2d)]

use std::path::PathBuf;

use opencv::core::{ Size, NORM_HAMMING, no_array };
use opencv::prelude::*;
use opencv::types::VectorOfKeyPoint;
use opencv::{features2d, imgcodecs, Result};

#[test]
fn orb() -> Result<()> {
	let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/blox.jpg");
	let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
	let mut orb = features2d::ORB::default()?;
	let mut kp = VectorOfKeyPoint::new();
	let mut des = Mat::default();
	orb.detect_and_compute(&img, &Mat::default(), &mut kp, &mut des, false)?;
	let size = 290;
	assert_eq!(size, kp.len());
	assert_eq!(Size::new(32, size as i32), des.size()?);
	Ok(())
}


/// cargo test --package opencv --test features2d -- orb_bruteforce_match --exact --nocapture 
#[test]
fn orb_bruteforce_match() -> Result<()> {
	let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/blox.jpg");
	let img_a = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_GRAYSCALE)?;
	let img_b = img_a.clone();  // yep this is the same 

	let mut orb = features2d::ORB::default()?;
	let mut kp_a = VectorOfKeyPoint::new();
	let mut des_a = Mat::default();
	orb.detect_and_compute(&img_a, &Mat::default(), &mut kp_a, &mut des_a, false)?;

	let mut kp_b = VectorOfKeyPoint::new();
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

	assert_ne!(matches.len(), 0);  // expected many matches since images are equal
	Ok(())
}
