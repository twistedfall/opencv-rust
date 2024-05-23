//! # Extra Stereo Correspondance Algorithms
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{QuasiDenseStereoTrait, QuasiDenseStereoTraitConst};
}

// CV_CS_CENSUS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:14
pub const CV_CS_CENSUS: i32 = 2;
// CV_DENSE_CENSUS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:13
pub const CV_DENSE_CENSUS: i32 = 0;
// CV_MEAN_VARIATION /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:15
pub const CV_MEAN_VARIATION: i32 = 5;
// CV_MODIFIED_CENSUS_TRANSFORM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:14
pub const CV_MODIFIED_CENSUS_TRANSFORM: i32 = 4;
// CV_MODIFIED_CS_CENSUS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:14
pub const CV_MODIFIED_CS_CENSUS: i32 = 3;
// CV_QUADRATIC_INTERPOLATION /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo.hpp:70
pub const CV_QUADRATIC_INTERPOLATION: i32 = 0;
// CV_SIMETRICV_INTERPOLATION /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo.hpp:70
pub const CV_SIMETRICV_INTERPOLATION: i32 = 1;
// CV_SPARSE_CENSUS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:13
pub const CV_SPARSE_CENSUS: i32 = 1;
// CV_SPECKLE_REMOVAL_ALGORITHM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo.hpp:66
pub const CV_SPECKLE_REMOVAL_ALGORITHM: i32 = 0;
// CV_SPECKLE_REMOVAL_AVG_ALGORITHM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo.hpp:66
pub const CV_SPECKLE_REMOVAL_AVG_ALGORITHM: i32 = 1;
// CV_STAR_KERNEL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:15
pub const CV_STAR_KERNEL: i32 = 6;
/// Two variations of census applied on input images
/// Implementation of a census transform which is taking into account just the some pixels from the census kernel thus allowing for larger block sizes
///*
// censusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:22
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
// censusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:24
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
// cv::stereo::modifiedCensusTransform(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:29
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
// modifiedCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int, int, const Mat &, const Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:29
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
// cv::stereo::modifiedCensusTransform(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:31
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
// modifiedCensusTransform(const Mat &, int, Mat &, const int, int, const Mat &)(TraitClass, Primitive, TraitClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:31
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
// starCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:39
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
// starCensusTransform(const Mat &, int, Mat &)(TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:41
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
// symetricCensusTransform(const Mat &, const Mat &, int, Mat &, Mat &, const int)(TraitClass, TraitClass, Primitive, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:35
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
// symetricCensusTransform(const Mat &, int, Mat &, const int)(TraitClass, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/descriptor.hpp:37
// ("cv::stereo::symetricCensusTransform", vec![(pred!(mut, ["img1", "kernelSize", "dist1", "type"], ["const cv::Mat*", "int", "cv::Mat*", "const int"]), _)]),
#[inline]
pub fn symetric_census_transform_1(img1: &impl core::MatTraitConst, kernel_size: i32, dist1: &mut impl core::MatTrait, typ: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(img1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// \addtogroup xstereo
// MatchQuasiDense /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:27
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MatchQuasiDense {
	pub p0: core::Point2i,
	pub p1: core::Point2i,
	pub corr: f32,
}

opencv_type_simple! { crate::xstereo::MatchQuasiDense }

impl MatchQuasiDense {
	// operator<(const MatchQuasiDense &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:35
	// ("cv::stereo::MatchQuasiDense::operator<", vec![(pred!(const, ["rhs"], ["const cv::stereo::MatchQuasiDense*"]), _)]),
	#[inline]
	pub fn apply(self, rhs: crate::xstereo::MatchQuasiDense) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_MatchQuasiDense_operatorL_const_const_MatchQuasiDenseR(&self, &rhs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// MatchQuasiDense()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:33
	// ("cv::stereo::MatchQuasiDense::MatchQuasiDense", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::xstereo::MatchQuasiDense> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_MatchQuasiDense_MatchQuasiDense(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PropagationParameters /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:40
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PropagationParameters {
	pub corr_win_size_x: i32,
	pub corr_win_size_y: i32,
	pub border_x: i32,
	pub border_y: i32,
	pub correlation_threshold: f32,
	pub textrure_threshold: f32,
	pub neighborhood_size: i32,
	pub disparity_gradient: i32,
	pub lk_template_size: i32,
	pub lk_pyr_lvl: i32,
	pub lk_term_param1: i32,
	pub lk_term_param2: f32,
	pub gft_quality_thres: f32,
	pub gft_min_seperation_dist: i32,
	pub gft_max_num_features: i32,
}

opencv_type_simple! { crate::xstereo::PropagationParameters }

impl PropagationParameters {
}

/// Constant methods for [crate::xstereo::QuasiDenseStereo]
// QuasiDenseStereo /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:95
pub trait QuasiDenseStereoTraitConst {
	fn as_raw_QuasiDenseStereo(&self) -> *const c_void;

	// cv::stereo::QuasiDenseStereo::Param() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:190
	// ("cv::stereo::QuasiDenseStereo::Param", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn param(&self) -> crate::xstereo::PropagationParameters {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_propParam_const(self.as_raw_QuasiDenseStereo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

}

/// Mutable methods for [crate::xstereo::QuasiDenseStereo]
pub trait QuasiDenseStereoTrait: crate::xstereo::QuasiDenseStereoTraitConst {
	fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void;

	// cv::stereo::QuasiDenseStereo::setParam(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:190
	// ("cv::stereo::QuasiDenseStereo::setParam", vec![(pred!(mut, ["val"], ["const cv::stereo::PropagationParameters"]), _)]),
	#[inline]
	fn set_param(&mut self, val: crate::xstereo::PropagationParameters) {
		let ret = unsafe { sys::cv_stereo_QuasiDenseStereo_propParam_const_PropagationParameters(self.as_raw_mut_QuasiDenseStereo(), &val) };
		ret
	}

	/// Load a file containing the configuration parameters of the class.
	/// ## Parameters
	/// * filepath: The location of the .YAML file containing the configuration parameters.
	///
	/// Note: default value is an empty string in which case the default parameters will be loaded.
	/// @retval 1: If the path is not empty and the program loaded the parameters successfully.
	/// @retval 0: If the path is empty and the program loaded default parameters.
	/// @retval -1: If the file location is not valid or the program could not open the file and
	/// loaded default parameters from defaults.hpp.
	///
	/// Note: The method is automatically called in the constructor and configures the class.
	///
	/// Note: Loading different parameters will have an effect on the output. This is useful for tuning
	/// in case of video processing.
	/// ## See also
	/// loadParameters
	// loadParameters(cv::String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:118
	// ("cv::stereo::QuasiDenseStereo::loadParameters", vec![(pred!(mut, ["filepath"], ["cv::String"]), _)]),
	#[inline]
	fn load_parameters(&mut self, filepath: &str) -> Result<i32> {
		extern_container_arg!(filepath);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_loadParameters_String(self.as_raw_mut_QuasiDenseStereo(), filepath.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Save a file containing all the configuration parameters the class is currently set to.
	/// ## Parameters
	/// * filepath: The location to store the parameters file.
	///
	/// Note: Calling this method with no arguments will result in storing class parameters to a file
	/// names "qds_parameters.yaml" in the root project folder.
	///
	/// Note: This method can be used to generate a template file for tuning the class.
	/// ## See also
	/// loadParameters
	// saveParameters(cv::String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:129
	// ("cv::stereo::QuasiDenseStereo::saveParameters", vec![(pred!(mut, ["filepath"], ["cv::String"]), _)]),
	#[inline]
	fn save_parameters(&mut self, filepath: &str) -> Result<i32> {
		extern_container_arg!(filepath);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_saveParameters_String(self.as_raw_mut_QuasiDenseStereo(), filepath.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get The sparse corresponding points.
	/// ## Parameters
	/// * sMatches:[out] A vector containing all sparse correspondences.
	///
	/// Note: The method clears the sMatches vector.
	///
	/// Note: The returned Match elements inside the sMatches vector, do not use corr member.
	// getSparseMatches(std::vector<MatchQuasiDense> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:138
	// ("cv::stereo::QuasiDenseStereo::getSparseMatches", vec![(pred!(mut, ["sMatches"], ["std::vector<cv::stereo::MatchQuasiDense>*"]), _)]),
	#[inline]
	fn get_sparse_matches(&mut self, s_matches: &mut core::Vector<crate::xstereo::MatchQuasiDense>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getSparseMatches_vectorLMatchQuasiDenseGR(self.as_raw_mut_QuasiDenseStereo(), s_matches.as_raw_mut_VectorOfMatchQuasiDense(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get The dense corresponding points.
	/// ## Parameters
	/// * denseMatches:[out] A vector containing all dense matches.
	///
	/// Note: The method clears the denseMatches vector.
	///
	/// Note: The returned Match elements inside the sMatches vector, do not use corr member.
	// getDenseMatches(std::vector<MatchQuasiDense> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:147
	// ("cv::stereo::QuasiDenseStereo::getDenseMatches", vec![(pred!(mut, ["denseMatches"], ["std::vector<cv::stereo::MatchQuasiDense>*"]), _)]),
	#[inline]
	fn get_dense_matches(&mut self, dense_matches: &mut core::Vector<crate::xstereo::MatchQuasiDense>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getDenseMatches_vectorLMatchQuasiDenseGR(self.as_raw_mut_QuasiDenseStereo(), dense_matches.as_raw_mut_VectorOfMatchQuasiDense(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Main process of the algorithm. This method computes the sparse seeds and then densifies them.
	///
	/// Initially input images are converted to gray-scale and then the sparseMatching method
	/// is called to obtain the sparse stereo. Finally quasiDenseMatching is called to densify the corresponding
	/// points.
	/// ## Parameters
	/// * imgLeft: The left Channel of a stereo image pair.
	/// * imgRight: The right Channel of a stereo image pair.
	///
	/// Note: If input images are in color, the method assumes that are BGR and converts them to grayscale.
	/// ## See also
	/// sparseMatching
	/// quasiDenseMatching
	// process(const cv::Mat &, const cv::Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:163
	// ("cv::stereo::QuasiDenseStereo::process", vec![(pred!(mut, ["imgLeft", "imgRight"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn process(&mut self, img_left: &impl core::MatTraitConst, img_right: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(self.as_raw_mut_QuasiDenseStereo(), img_left.as_raw_Mat(), img_right.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Specify pixel coordinates in the left image and get its corresponding location in the right image.
	/// ## Parameters
	/// * x: The x pixel coordinate in the left image channel.
	/// * y: The y pixel coordinate in the left image channel.
	/// @retval cv::Point(x, y) The location of the corresponding pixel in the right image.
	/// @retval cv::Point(0, 0) (NO_MATCH)  if no match is found in the right image for the specified pixel location in the left image.
	///
	/// Note: This method should be always called after process, otherwise the matches will not be correct.
	// getMatch(const int, const int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:174
	// ("cv::stereo::QuasiDenseStereo::getMatch", vec![(pred!(mut, ["x", "y"], ["const int", "const int"]), _)]),
	#[inline]
	fn get_match(&mut self, x: i32, y: i32) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(self.as_raw_mut_QuasiDenseStereo(), x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compute and return the disparity map based on the correspondences found in the "process" method.
	///
	/// Note: Default level is 50
	/// ## Returns
	/// cv::Mat containing a the disparity image in grayscale.
	/// ## See also
	/// computeDisparity
	/// quantizeDisparity
	// getDisparity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:184
	// ("cv::stereo::QuasiDenseStereo::getDisparity", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_disparity(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_getDisparity(self.as_raw_mut_QuasiDenseStereo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Class containing the methods needed for Quasi Dense Stereo computation.
///
/// This module contains the code to perform quasi dense stereo matching.
/// The method initially starts with a sparse 3D reconstruction based on feature matching across a
/// stereo image pair and subsequently propagates the structure into neighboring image regions.
/// To obtain initial seed correspondences, the algorithm locates Shi and Tomashi features in the
/// left image of the stereo pair and then tracks them using pyramidal Lucas-Kanade in the right image.
/// To densify the sparse correspondences, the algorithm computes the zero-mean normalized
/// cross-correlation (ZNCC) in small patches around every seed pair and uses it as a quality metric
/// for each match. In this code, we introduce a custom structure to store the location and ZNCC value
/// of correspondences called "Match". Seed Matches are stored in a priority queue sorted according to
/// their ZNCC value, allowing for the best quality Match to be readily available. The algorithm pops
/// Matches and uses them to extract new matches around them. This is done by considering a small
/// neighboring area around each Seed and retrieving correspondences above a certain texture threshold
/// that are not previously computed. New matches are stored in the seed priority queue and used as seeds.
/// The propagation process ends when no additional matches can be retrieved.
/// ## See also
/// This code represents the work presented in [Stoyanov2010](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Stoyanov2010).
/// If this code is useful for your work please cite [Stoyanov2010](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Stoyanov2010).
///
/// Also the original growing scheme idea is described in [Lhuillier2000](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lhuillier2000)
// QuasiDenseStereo /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:95
pub struct QuasiDenseStereo {
	ptr: *mut c_void,
}

opencv_type_boxed! { QuasiDenseStereo }

impl Drop for QuasiDenseStereo {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_stereo_QuasiDenseStereo_delete(self.as_raw_mut_QuasiDenseStereo()) };
	}
}

unsafe impl Send for QuasiDenseStereo {}

impl crate::xstereo::QuasiDenseStereoTraitConst for QuasiDenseStereo {
	#[inline] fn as_raw_QuasiDenseStereo(&self) -> *const c_void { self.as_raw() }
}

impl crate::xstereo::QuasiDenseStereoTrait for QuasiDenseStereo {
	#[inline] fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { QuasiDenseStereo, crate::xstereo::QuasiDenseStereoTraitConst, as_raw_QuasiDenseStereo, crate::xstereo::QuasiDenseStereoTrait, as_raw_mut_QuasiDenseStereo }

impl QuasiDenseStereo {
	/// ## C++ default parameters
	/// * param_filepath: cv::String()
	// create(cv::Size, cv::String)(SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:187
	// ("cv::stereo::QuasiDenseStereo::create", vec![(pred!(mut, ["monoImgSize", "paramFilepath"], ["cv::Size", "cv::String"]), _)]),
	#[inline]
	pub fn create(mono_img_size: core::Size, param_filepath: &str) -> Result<core::Ptr<crate::xstereo::QuasiDenseStereo>> {
		extern_container_arg!(param_filepath);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_create_Size_String(&mono_img_size, param_filepath.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xstereo::QuasiDenseStereo>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [QuasiDenseStereo::create] function uses the following default values for its arguments:
	/// * param_filepath: cv::String()
	// cv::stereo::QuasiDenseStereo::create(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xstereo/quasi_dense_stereo.hpp:187
	// ("cv::stereo::QuasiDenseStereo::create", vec![(pred!(mut, ["monoImgSize"], ["cv::Size"]), _)]),
	#[inline]
	pub fn create_def(mono_img_size: core::Size) -> Result<core::Ptr<crate::xstereo::QuasiDenseStereo>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_stereo_QuasiDenseStereo_create_Size(&mono_img_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xstereo::QuasiDenseStereo>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for QuasiDenseStereo {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("QuasiDenseStereo")
			.field("param", &crate::xstereo::QuasiDenseStereoTraitConst::param(self))
			.finish()
	}
}
