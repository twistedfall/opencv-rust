#![allow(unused_parens)]
//! # Stereo Correspondance Algorithms
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

pub const CV_CS_CENSUS: i32 = 2;
pub const CV_DENSE_CENSUS: i32 = 0;
pub const CV_MEAN_VARIATION: i32 = 5;
pub const CV_MODIFIED_CENSUS_TRANSFORM: i32 = 4;
pub const CV_MODIFIED_CS_CENSUS: i32 = 3;
pub const CV_QUADRATIC_INTERPOLATION: i32 = 0;
pub const CV_SIMETRICV_INTERPOLATION: i32 = 1;
pub const CV_SPARSE_CENSUS: i32 = 1;
pub const CV_SPECKLE_REMOVAL_ALGORITHM: i32 = 0;
pub const CV_SPECKLE_REMOVAL_AVG_ALGORITHM: i32 = 1;
pub const CV_STAR_KERNEL: i32 = 6;
pub fn census_transform(image1: &core::Mat, image2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_int(image1.as_raw_Mat(), image2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ) }.into_result()
}

pub fn census_transform_1(image1: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_censusTransform_const_MatR_int_MatR_int(image1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ) }.into_result()
}

/// STANDARD_MCT - Modified census which is memorizing for each pixel 2 bits and includes a tolerance to the pixel comparison
/// MCT_MEAN_VARIATION - Implementation of a modified census transform which is also taking into account the variation to the mean of the window not just the center pixel
/// *
/// 
/// ## C++ default parameters
/// * t: 0
/// * integral_image1: cv::Mat::zeros(100,100,CV_8UC1)
/// * integral_image2: cv::Mat::zeros(100,100,CV_8UC1)
pub fn modified_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32, t: i32, integral_image1: &core::Mat, integral_image2: &core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_int_int_const_MatR_const_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, t, integral_image1.as_raw_Mat(), integral_image2.as_raw_Mat()) }.into_result()
}

/// ## C++ default parameters
/// * t: 0
/// * integral_image: cv::Mat::zeros(100,100,CV_8UC1)
pub fn modified_census_transform_1(img1: &core::Mat, kernel_size: i32, dist: &mut core::Mat, typ: i32, t: i32, integral_image: &core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_int_int_const_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat(), typ, t, integral_image.as_raw_Mat()) }.into_result()
}

pub fn star_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat()) }.into_result()
}

pub fn star_census_transform_1(img1: &core::Mat, kernel_size: i32, dist: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_int_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat()) }.into_result()
}

/// The classical center symetric census
/// A modified version of cs census which is comparing a pixel with its correspondent after the center
/// *
pub fn symetric_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_int(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ) }.into_result()
}

pub fn symetric_census_transform_1(img1: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_int_MatR_int(img1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ) }.into_result()
}
