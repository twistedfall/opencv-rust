//! # Stereo Correspondance Algorithms
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
}

// CV_CS_CENSUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:14
pub const CV_CS_CENSUS: i32 = 2;
// CV_DENSE_CENSUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:13
pub const CV_DENSE_CENSUS: i32 = 0;
// CV_MEAN_VARIATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:15
pub const CV_MEAN_VARIATION: i32 = 5;
// CV_MODIFIED_CENSUS_TRANSFORM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:14
pub const CV_MODIFIED_CENSUS_TRANSFORM: i32 = 4;
// CV_MODIFIED_CS_CENSUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:14
pub const CV_MODIFIED_CS_CENSUS: i32 = 3;
// CV_QUADRATIC_INTERPOLATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo.hpp:117
pub const CV_QUADRATIC_INTERPOLATION: i32 = 0;
// CV_SIMETRICV_INTERPOLATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo.hpp:117
pub const CV_SIMETRICV_INTERPOLATION: i32 = 1;
// CV_SPARSE_CENSUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:13
pub const CV_SPARSE_CENSUS: i32 = 1;
// CV_SPECKLE_REMOVAL_ALGORITHM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo.hpp:113
pub const CV_SPECKLE_REMOVAL_ALGORITHM: i32 = 0;
// CV_SPECKLE_REMOVAL_AVG_ALGORITHM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo.hpp:113
pub const CV_SPECKLE_REMOVAL_AVG_ALGORITHM: i32 = 1;
// CV_STAR_KERNEL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:15
pub const CV_STAR_KERNEL: i32 = 6;
/// Two variations of census applied on input images
/// Implementation of a census transform which is taking into account just the some pixels from the census kernel thus allowing for larger block sizes
///*
// censusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:22
// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "image2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
#[inline]
pub fn census_transform(image1: &impl core::MatTraitConst, image2: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, dist2: &mut impl core::MatTrait, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(image1.as_raw_Mat(), image2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// single image census transform
// censusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:24
// ("cv::stereo::censusTransform", vec![(pred!(mut, ["image1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
#[inline]
pub fn census_transform_1(image1: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_censusTransform_const_MatR_int_MatR_const_int(image1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// STANDARD_MCT - Modified census which is memorizing for each pixel 2 bits and includes a tolerance to the pixel comparison
/// MCT_MEAN_VARIATION - Implementation of a modified census transform which is also taking into account the variation to the mean of the window not just the center pixel
/// *
///
/// ## Note
/// This alternative version of [modified_census_transform] function uses the following default values for its arguments:
/// * t: 0
/// * integral_image1: Mat()
/// * integral_image2: Mat()
// cv::stereo::modifiedCensusTransform(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:29
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
#[inline]
pub fn modified_census_transform_def(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, dist2: &mut impl core::MatTrait, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// STANDARD_MCT - Modified census which is memorizing for each pixel 2 bits and includes a tolerance to the pixel comparison
/// MCT_MEAN_VARIATION - Implementation of a modified census transform which is also taking into account the variation to the mean of the window not just the center pixel
/// *
///
/// ## C++ default parameters
/// * t: 0
/// * integral_image1: Mat()
/// * integral_image2: Mat()
// modifiedCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int, int, const Mat &, const Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:29
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type", "t", "integralImage1", "integralImage2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int", "int", "const cv::Mat*", "const cv::Mat*"]), _)]),
#[inline]
pub fn modified_census_transform(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, dist2: &mut impl core::MatTrait, typ: i32, t: i32, integral_image1: &impl core::MatTraitConst, integral_image2: &impl core::MatTraitConst) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, t, integral_image1.as_raw_Mat(), integral_image2.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// single version of modified census transform descriptor
///
/// ## Note
/// This alternative version of [modified_census_transform_1] function uses the following default values for its arguments:
/// * t: 0
/// * integral_image: Mat()
// cv::stereo::modifiedCensusTransform(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:31
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
#[inline]
pub fn modified_census_transform_1_def(img1: &impl core::MatTraitConst, kernel_size: i32, dist: &mut impl core::MatTrait, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// single version of modified census transform descriptor
///
/// ## C++ default parameters
/// * t: 0
/// * integral_image: Mat()
// modifiedCensusTransform(const Mat &, int, Mat &, const int, int, const Mat &)(TraitClass, Primitive, TraitClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:31
// ("cv::stereo::modifiedCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist", "type", "t", "integralImage"], ["const cv::Mat*", "int", "cv::Mat*", "const int", "int", "const cv::Mat*"]), _)]),
#[inline]
pub fn modified_census_transform_1(img1: &impl core::MatTraitConst, kernel_size: i32, dist: &mut impl core::MatTrait, typ: i32, t: i32, integral_image: &impl core::MatTraitConst) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat(), typ, t, integral_image.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// in a 9x9 kernel only certain positions are choosen
// starCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:39
// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*"]), _)]),
#[inline]
pub fn star_census_transform(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, dist2: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// single image version of star kernel
// starCensusTransform(const Mat &, int, Mat &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:41
// ("cv::stereo::starCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist"], ["const cv::Mat*", "int", "cv::Mat*"]), _)]),
#[inline]
pub fn star_census_transform_1(img1: &impl core::MatTraitConst, kernel_size: i32, dist: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_int_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// The classical center symetric census
/// A modified version of cs census which is comparing a pixel with its correspondent after the center
///*
// symetricCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:35
// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "img2", "kernelSize", "dist1", "dist2", "type"], ["const cv::Mat*", "const cv::Mat*", "int", "cv::Mat*", "cv::Mat*", "const int"]), _)]),
#[inline]
pub fn symetric_census_transform(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, dist2: &mut impl core::MatTrait, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// single version of census transform
// symetricCensusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stereo/descriptor.hpp:37
// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
#[inline]
pub fn symetric_census_transform_1(img1: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(img1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}
