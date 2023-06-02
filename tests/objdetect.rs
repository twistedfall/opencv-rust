#![cfg(ocvrs_has_module_objdetect)]

use std::path::Path;

use opencv::{core, imgcodecs, objdetect, prelude::*, types::VectorOfPoint, Result};

#[test]
fn qr_code() -> Result<()> {
	let qr_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/qr.png");

	// workaround for the missing (and not runtime detectable) QUIRC support in repository OpenCV in Ubuntu 20.04
	let objdetect_missing_quirc = cfg!(target_os = "linux") && core::get_build_information()?.contains("/opencv-4.5.4+dfsg/");

	{
		#[allow(unused_mut)]
		let mut detector = objdetect::QRCodeDetector::default()?;
		let src = imgcodecs::imread(qr_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
		let mut pts = VectorOfPoint::new();
		let mut straight = Mat::default();
		let res = detector.detect_and_decode(&src, &mut pts, &mut straight)?;
		assert_eq!(4, pts.len());
		if objdetect_missing_quirc {
			assert_eq!(res, b"");
			assert!(straight.empty());
		} else {
			assert_eq!(res, b"https://crates.io/crates/opencv");
			assert!(!straight.empty());
		}
	}

	{
		#[allow(unused_mut)]
		let mut detector = objdetect::QRCodeDetector::default()?;
		let src = imgcodecs::imread(qr_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
		let mut pts = VectorOfPoint::new();
		let res = detector.detect(&src, &mut pts)?;
		assert!(res);
		assert_eq!(4, pts.len());
		let mut straight = Mat::default();
		let res = detector.decode(&src, &pts, &mut straight)?;
		if objdetect_missing_quirc {
			assert_eq!(res, b"");
			assert!(straight.empty());
		} else {
			assert_eq!(res, b"https://crates.io/crates/opencv");
			assert!(!straight.empty());
		}
	}

	let binary_qr_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/qr-binary.png");

	{
		#[allow(unused_mut)]
		let mut detector = objdetect::QRCodeDetector::default()?;
		let src = imgcodecs::imread(binary_qr_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
		let mut pts = VectorOfPoint::new();
		let mut straight = Mat::default();
		let res = detector.detect_and_decode(&src, &mut pts, &mut straight)?;
		assert_eq!(4, pts.len());
		if objdetect_missing_quirc {
			assert_eq!(res, b"");
			assert!(straight.empty());
		} else {
			assert_eq!(res, [0, 1, 2, 3, 4, 5]);
			assert!(!straight.empty());
		}
	}

	Ok(())
}

/// Return of string binary data into Vec<u8> via output argument
#[test]
#[cfg(ocvrs_opencv_branch_34)]
fn output_byte_string() -> Result<()> {
	let qr_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/qr.png");

	let src = imgcodecs::imread(qr_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
	let mut pts = VectorOfPoint::new();
	let res = objdetect::detect_qr_code(&src, &mut pts, 0.2, 0.1)?;
	assert!(res);
	assert_eq!(4, pts.len());
	let mut out = Vec::new();
	let mut straight = Mat::default();
	let res = objdetect::decode_qr_code(&src, &pts, &mut out, &mut straight)?;
	assert!(res);
	assert_eq!(out, b"https://crates.io/crates/opencv");
	assert!(!straight.empty());

	Ok(())
}
