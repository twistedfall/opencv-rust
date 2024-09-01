#![cfg(ocvrs_has_module_video)]

use std::cmp::Ordering;

use opencv as cv;
use opencv::core as cv_core;
use cv::{features2d::*,  Result};
use cv_core::{ Size, TermCriteria, VectorToVec, KeyPoint, Mat, Ptr, Vector, Point2f};
use cv::imgcodecs::*;

#[test]
pub fn check_optical_flow_points() -> Result<()>{
    //let mut detector = ORBFeatureDetector::create();
    let mut detector = AKAZE::create_def()?;

    let prev_img = imread("tests/data/optflow/1080p_00.png", ImreadModes::IMREAD_GRAYSCALE.into()).unwrap();
    let cur_img = imread("tests/data/optflow/1080p_01.png", ImreadModes::IMREAD_GRAYSCALE.into()).unwrap();

    let mut prev_kps = Vector::<KeyPoint>::new();
    let mut prev_desc = Mat::default();
	let mask = Mat::default();
    let _ = detector.detect_and_compute(&prev_img, &mask,&mut prev_kps, &mut prev_desc, false);

    let mut ref_pts = Vector::<Point2f>::new();
    let _ = KeyPoint::convert_def(&prev_kps, &mut ref_pts); //Converts the KeyPoints to raw Points2f

   	debug_keypoints(&ref_pts);

    let next_kps = track_pyr_lk(&prev_img, &cur_img, &ref_pts);

	println!("dumping key_points: ");
	debug_keypoints(&next_kps);

    let detects = next_kps.len();
    println!("detects: {}", detects);

    // This should fail 
    next_kps.to_vec().iter().for_each(|k| assert!( k.x.partial_cmp(&0.0) == Some(Ordering::Equal) && k.y.partial_cmp(&0.0) == Some(Ordering::Equal) ));

	Ok(())
}

fn track_pyr_lk(prev_img: &Mat, cur_img: &Mat, ref_pts: &Vector<Point2f>) -> Vector::<Point2f>{

    let mut cur_key_points = Vector::<Point2f>::new();
	let min_eig = 1e-4;
	let flags = 0; // OPTFLOW_LK_GET_MIN_EIGENVALS vs OPTFLOW_USE_INITIAL_FLOW
	let size = Size::new(21,21);
		let crit = TermCriteria::default().unwrap(); // TermCriteria::new(typ, 30 , 0.01).unwrap();
	let max_levels=3;
	let mut err = Vector::<f64>::new();
	let mut status = Vector::<u8>::new();

	let _ =cv::video::calc_optical_flow_pyr_lk(
		&prev_img,
		&cur_img,
		&ref_pts,
		&mut cur_key_points,
		&mut status,
		&mut err,
		size,
		max_levels,
		crit,
		flags,
		min_eig,
	);

    cur_key_points

}

pub fn debug_keypoints(kps: &Vector<Point2f>){
	let points = kps.to_vec();
	println!("[debug] keypoints len:{}" , kps.len());

	if kps.len() < 3 { return ; };

	points[0..3].iter().for_each(|v| println!("{}, {}", v.x, v.y));
	println!("....,");
	points[points.len()-3..].iter().for_each(|v| println!("{}, {}", v.x, v.y));
}