#![allow(unused_parens)]
//! # Stereo Correspondance Algorithms
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::MatchTrait, super::PropagationParametersTrait, super::QuasiDenseStereo };
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
/// Two variations of census applied on input images
/// Implementation of a census transform which is taking into account just the some pixels from the census kernel thus allowing for larger block sizes
/// *
pub fn census_transform(image1: &core::Mat, image2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_censusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(image1.as_raw_Mat(), image2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ) }.into_result()
}

/// single image census transform
pub fn census_transform_1(image1: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_censusTransform_const_MatR_int_MatR_const_int(image1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ) }.into_result()
}

/// STANDARD_MCT - Modified census which is memorizing for each pixel 2 bits and includes a tolerance to the pixel comparison
/// MCT_MEAN_VARIATION - Implementation of a modified census transform which is also taking into account the variation to the mean of the window not just the center pixel
/// *
/// 
/// ## C++ default parameters
/// * t: 0
/// * integral_image1: Mat()
/// * integral_image2: Mat()
pub fn modified_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32, t: i32, integral_image1: &core::Mat, integral_image2: &core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int_int_const_MatR_const_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ, t, integral_image1.as_raw_Mat(), integral_image2.as_raw_Mat()) }.into_result()
}

/// single version of modified census transform descriptor
/// 
/// ## C++ default parameters
/// * t: 0
/// * integral_image: Mat()
pub fn modified_census_transform_1(img1: &core::Mat, kernel_size: i32, dist: &mut core::Mat, typ: i32, t: i32, integral_image: &core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_modifiedCensusTransform_const_MatR_int_MatR_const_int_int_const_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat(), typ, t, integral_image.as_raw_Mat()) }.into_result()
}

/// in a 9x9 kernel only certain positions are choosen
pub fn star_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_const_MatR_int_MatR_MatR(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat()) }.into_result()
}

/// single image version of star kernel
pub fn star_census_transform_1(img1: &core::Mat, kernel_size: i32, dist: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_stereo_starCensusTransform_const_MatR_int_MatR(img1.as_raw_Mat(), kernel_size, dist.as_raw_mut_Mat()) }.into_result()
}

/// The classical center symetric census
/// A modified version of cs census which is comparing a pixel with its correspondent after the center
/// *
pub fn symetric_census_transform(img1: &core::Mat, img2: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, dist2: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_const_MatR_int_MatR_MatR_const_int(img1.as_raw_Mat(), img2.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), dist2.as_raw_mut_Mat(), typ) }.into_result()
}

/// single version of census transform
pub fn symetric_census_transform_1(img1: &core::Mat, kernel_size: i32, dist1: &mut core::Mat, typ: i32) -> Result<()> {
	unsafe { sys::cv_stereo_symetricCensusTransform_const_MatR_int_MatR_const_int(img1.as_raw_Mat(), kernel_size, dist1.as_raw_mut_Mat(), typ) }.into_result()
}

/// \addtogroup stereo
pub trait MatchTrait {
	fn as_raw_Match(&self) -> *const c_void;
	fn as_raw_mut_Match(&mut self) -> *mut c_void;

	fn p0(&self) -> core::Point2i {
		unsafe { sys::cv_stereo_Match_getPropP0_const(self.as_raw_Match()) }.into_result().expect("Infallible function failed: p0")
	}
	
	fn set_p0(&mut self, val: core::Point2i) -> () {
		unsafe { sys::cv_stereo_Match_setPropP0_Point2i(self.as_raw_mut_Match(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_p0")
	}
	
	fn p1(&self) -> core::Point2i {
		unsafe { sys::cv_stereo_Match_getPropP1_const(self.as_raw_Match()) }.into_result().expect("Infallible function failed: p1")
	}
	
	fn set_p1(&mut self, val: core::Point2i) -> () {
		unsafe { sys::cv_stereo_Match_setPropP1_Point2i(self.as_raw_mut_Match(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_p1")
	}
	
	fn corr(&self) -> f32 {
		unsafe { sys::cv_stereo_Match_getPropCorr_const(self.as_raw_Match()) }.into_result().expect("Infallible function failed: corr")
	}
	
	fn set_corr(&mut self, val: f32) -> () {
		unsafe { sys::cv_stereo_Match_setPropCorr_float(self.as_raw_mut_Match(), val) }.into_result().expect("Infallible function failed: set_corr")
	}
	
}

/// \addtogroup stereo
pub struct Match {
	ptr: *mut c_void
}

opencv_type_boxed! { Match }

impl Drop for Match {
	fn drop(&mut self) {
		extern "C" { fn cv_Match_delete(instance: *mut c_void); }
		unsafe { cv_Match_delete(self.as_raw_mut_Match()) };
	}
}

impl Match {
	#[inline] pub fn as_raw_Match(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Match {}

impl crate::stereo::MatchTrait for Match {
	#[inline] fn as_raw_Match(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Match {
}

pub trait PropagationParametersTrait {
	fn as_raw_PropagationParameters(&self) -> *const c_void;
	fn as_raw_mut_PropagationParameters(&mut self) -> *mut c_void;

	fn corr_win_size_x(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropCorrWinSizeX_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: corr_win_size_x")
	}
	
	fn set_corr_win_size_x(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropCorrWinSizeX_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_corr_win_size_x")
	}
	
	fn corr_win_size_y(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropCorrWinSizeY_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: corr_win_size_y")
	}
	
	fn set_corr_win_size_y(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropCorrWinSizeY_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_corr_win_size_y")
	}
	
	fn border_x(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropBorderX_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: border_x")
	}
	
	fn set_border_x(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropBorderX_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_border_x")
	}
	
	fn border_y(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropBorderY_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: border_y")
	}
	
	fn set_border_y(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropBorderY_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_border_y")
	}
	
	fn correlation_threshold(&self) -> f32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropCorrelationThreshold_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: correlation_threshold")
	}
	
	fn set_correlation_threshold(&mut self, val: f32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropCorrelationThreshold_float(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_correlation_threshold")
	}
	
	fn textrure_threshold(&self) -> f32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropTextrureThreshold_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: textrure_threshold")
	}
	
	fn set_textrure_threshold(&mut self, val: f32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropTextrureThreshold_float(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_textrure_threshold")
	}
	
	fn neighborhood_size(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropNeighborhoodSize_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: neighborhood_size")
	}
	
	fn set_neighborhood_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropNeighborhoodSize_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_neighborhood_size")
	}
	
	fn disparity_gradient(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropDisparityGradient_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: disparity_gradient")
	}
	
	fn set_disparity_gradient(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropDisparityGradient_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_disparity_gradient")
	}
	
	fn lk_template_size(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropLkTemplateSize_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: lk_template_size")
	}
	
	fn set_lk_template_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropLkTemplateSize_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_lk_template_size")
	}
	
	fn lk_pyr_lvl(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropLkPyrLvl_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: lk_pyr_lvl")
	}
	
	fn set_lk_pyr_lvl(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropLkPyrLvl_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_lk_pyr_lvl")
	}
	
	fn lk_term_param1(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropLkTermParam1_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: lk_term_param1")
	}
	
	fn set_lk_term_param1(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropLkTermParam1_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_lk_term_param1")
	}
	
	fn lk_term_param2(&self) -> f32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropLkTermParam2_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: lk_term_param2")
	}
	
	fn set_lk_term_param2(&mut self, val: f32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropLkTermParam2_float(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_lk_term_param2")
	}
	
	fn gft_quality_thres(&self) -> f32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropGftQualityThres_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: gft_quality_thres")
	}
	
	fn set_gft_quality_thres(&mut self, val: f32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropGftQualityThres_float(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_gft_quality_thres")
	}
	
	fn gft_min_seperation_dist(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropGftMinSeperationDist_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: gft_min_seperation_dist")
	}
	
	fn set_gft_min_seperation_dist(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropGftMinSeperationDist_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_gft_min_seperation_dist")
	}
	
	fn gft_max_num_features(&self) -> i32 {
		unsafe { sys::cv_stereo_PropagationParameters_getPropGftMaxNumFeatures_const(self.as_raw_PropagationParameters()) }.into_result().expect("Infallible function failed: gft_max_num_features")
	}
	
	fn set_gft_max_num_features(&mut self, val: i32) -> () {
		unsafe { sys::cv_stereo_PropagationParameters_setPropGftMaxNumFeatures_int(self.as_raw_mut_PropagationParameters(), val) }.into_result().expect("Infallible function failed: set_gft_max_num_features")
	}
	
}

pub struct PropagationParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { PropagationParameters }

impl Drop for PropagationParameters {
	fn drop(&mut self) {
		extern "C" { fn cv_PropagationParameters_delete(instance: *mut c_void); }
		unsafe { cv_PropagationParameters_delete(self.as_raw_mut_PropagationParameters()) };
	}
}

impl PropagationParameters {
	#[inline] pub fn as_raw_PropagationParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PropagationParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PropagationParameters {}

impl crate::stereo::PropagationParametersTrait for PropagationParameters {
	#[inline] fn as_raw_PropagationParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PropagationParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PropagationParameters {
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
/// This code represents the work presented in [Stoyanov2010](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Stoyanov2010).
/// If this code is useful for your work please cite [Stoyanov2010](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Stoyanov2010).
/// 
/// Also the original growing scheme idea is described in [Lhuillier2000](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Lhuillier2000)
pub trait QuasiDenseStereo {
	fn as_raw_QuasiDenseStereo(&self) -> *const c_void;
	fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void;

	fn param(&mut self) -> crate::stereo::PropagationParameters {
		unsafe { sys::cv_stereo_QuasiDenseStereo_getPropParam(self.as_raw_mut_QuasiDenseStereo()) }.into_result().map(|r| unsafe { crate::stereo::PropagationParameters::opencv_from_extern(r) } ).expect("Infallible function failed: param")
	}
	
	fn set_param(&mut self, mut val: crate::stereo::PropagationParameters) -> () {
		unsafe { sys::cv_stereo_QuasiDenseStereo_setPropParam_PropagationParameters(self.as_raw_mut_QuasiDenseStereo(), val.as_raw_mut_PropagationParameters()) }.into_result().expect("Infallible function failed: set_param")
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
	fn load_parameters(&mut self, filepath: &str) -> Result<i32> {
		extern_container_arg!(mut filepath);
		unsafe { sys::cv_stereo_QuasiDenseStereo_loadParameters_String(self.as_raw_mut_QuasiDenseStereo(), filepath.opencv_as_extern_mut()) }.into_result()
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
	fn save_parameters(&mut self, filepath: &str) -> Result<i32> {
		extern_container_arg!(mut filepath);
		unsafe { sys::cv_stereo_QuasiDenseStereo_saveParameters_String(self.as_raw_mut_QuasiDenseStereo(), filepath.opencv_as_extern_mut()) }.into_result()
	}
	
	/// Get The sparse corresponding points.
	/// ## Parameters
	/// * sMatches:[out] A vector containing all sparse correspondences.
	/// 
	/// Note: The method clears the sMatches vector.
	/// 
	/// Note: The returned Match elements inside the sMatches vector, do not use corr member.
	fn get_sparse_matches(&mut self, s_matches: &mut core::Vector::<crate::stereo::Match>) -> Result<()> {
		unsafe { sys::cv_stereo_QuasiDenseStereo_getSparseMatches_vector_Match_R(self.as_raw_mut_QuasiDenseStereo(), s_matches.as_raw_mut_VectorOfMatch()) }.into_result()
	}
	
	/// Get The dense corresponding points.
	/// ## Parameters
	/// * denseMatches:[out] A vector containing all dense matches.
	/// 
	/// Note: The method clears the denseMatches vector.
	/// 
	/// Note: The returned Match elements inside the sMatches vector, do not use corr member.
	fn get_dense_matches(&mut self, dense_matches: &mut core::Vector::<crate::stereo::Match>) -> Result<()> {
		unsafe { sys::cv_stereo_QuasiDenseStereo_getDenseMatches_vector_Match_R(self.as_raw_mut_QuasiDenseStereo(), dense_matches.as_raw_mut_VectorOfMatch()) }.into_result()
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
	fn process(&mut self, img_left: &core::Mat, img_right: &core::Mat) -> Result<()> {
		unsafe { sys::cv_stereo_QuasiDenseStereo_process_const_MatR_const_MatR(self.as_raw_mut_QuasiDenseStereo(), img_left.as_raw_Mat(), img_right.as_raw_Mat()) }.into_result()
	}
	
	/// Specify pixel coordinates in the left image and get its corresponding location in the right image.
	/// ## Parameters
	/// * x: The x pixel coordinate in the left image channel.
	/// * y: The y pixel coordinate in the left image channel.
	/// @retval cv::Point(x, y) The location of the corresponding pixel in the right image.
	/// @retval cv::Point(0, 0) (NO_MATCH)  if no match is found in the right image for the specified pixel location in the left image.
	/// 
	/// Note: This method should be always called after process, otherwise the matches will not be correct.
	fn get_match(&mut self, x: i32, y: i32) -> Result<core::Point2f> {
		unsafe { sys::cv_stereo_QuasiDenseStereo_getMatch_const_int_const_int(self.as_raw_mut_QuasiDenseStereo(), x, y) }.into_result()
	}
	
	/// Compute and return the disparity map based on the correspondences found in the "process" method.
	/// ## Parameters
	/// * disparityLvls: The level of detail in output disparity image.
	/// 
	/// Note: Default level is 50
	/// ## Returns
	/// cv::Mat containing a the disparity image in grayscale.
	/// ## See also
	/// computeDisparity
	/// quantizeDisparity
	/// 
	/// ## C++ default parameters
	/// * disparity_lvls: 50
	fn get_disparity(&mut self, disparity_lvls: u8) -> Result<core::Mat> {
		unsafe { sys::cv_stereo_QuasiDenseStereo_getDisparity_uint8_t(self.as_raw_mut_QuasiDenseStereo(), disparity_lvls) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

impl dyn QuasiDenseStereo + '_ {
	/// ## C++ default parameters
	/// * param_filepath: cv::String()
	pub fn create(mono_img_size: core::Size, param_filepath: &str) -> Result<core::Ptr::<dyn crate::stereo::QuasiDenseStereo>> {
		extern_container_arg!(mut param_filepath);
		unsafe { sys::cv_stereo_QuasiDenseStereo_create_Size_String(mono_img_size.opencv_as_extern(), param_filepath.opencv_as_extern_mut()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stereo::QuasiDenseStereo>::opencv_from_extern(r) } )
	}
	
}