#![allow(unused_parens)]
//! # 2D Features Framework
//!    # Feature Detection and Description
//!    # Descriptor Matchers
//! 
//! Matchers of keypoint descriptors in OpenCV have wrappers with a common interface that enables you to
//! easily switch between different algorithms solving the same problem. This section is devoted to
//! matching descriptors that are represented as vectors in a multidimensional space. All objects that
//! implement vector descriptor matchers inherit the DescriptorMatcher interface.
//! 
//! 
//! Note:
//!    *   An example explaining keypoint matching can be found at
//!        opencv_source_code/samples/cpp/descriptor_extractor_matcher.cpp
//!    *   An example on descriptor matching evaluation can be found at
//!        opencv_source_code/samples/cpp/detector_descriptor_matcher_evaluation.cpp
//!    *   An example on one to many image matching can be found at
//!        opencv_source_code/samples/cpp/matching_to_many_images.cpp
//! 
//!    # Drawing Function of Keypoints and Matches
//!    # Object Categorization
//! 
//! This section describes approaches based on local 2D features and used to categorize objects.
//! 
//! 
//! Note:
//!    *   A complete Bag-Of-Words sample can be found at
//!        opencv_source_code/samples/cpp/bagofwords_classification.cpp
//!    *   (Python) An example using the features2D framework to perform object categorization can be
//!        found at opencv_source_code/samples/python/find_obj.py
//! 
//!    # Hardware Acceleration Layer
//!        # Interface
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::KeyPointsFilterTrait, super::Feature2DTrait, super::BRISKTrait, super::ORB, super::MSER, super::FastFeatureDetector, super::AgastFeatureDetector, super::GFTTDetector, super::SimpleBlobDetectorTrait, super::KAZE, super::AKAZE, super::DescriptorMatcher, super::BFMatcherTrait, super::FlannBasedMatcherTrait, super::BOWTrainer, super::BOWKMeansTrainerTrait, super::BOWImgDescriptorExtractorTrait };
}

pub const AgastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
pub const AgastFeatureDetector_THRESHOLD: i32 = 10000;
/// Output image matrix will be created (Mat::create),
/// i.e. existing memory of output image may be reused.
/// Two source image, matches and single keypoints will be drawn.
/// For each keypoint only the center point will be drawn (without
/// the circle around keypoint with keypoint size and orientation).
pub const DrawMatchesFlags_DEFAULT: i32 = 0;
/// Output image matrix will not be created (Mat::create).
/// Matches will be drawn on existing content of output image.
pub const DrawMatchesFlags_DRAW_OVER_OUTIMG: i32 = 1;
/// For each keypoint the circle around keypoint with keypoint size and
/// orientation will be drawn.
pub const DrawMatchesFlags_DRAW_RICH_KEYPOINTS: i32 = 4;
/// Single keypoints will not be drawn.
pub const DrawMatchesFlags_NOT_DRAW_SINGLE_POINTS: i32 = 2;
pub const FastFeatureDetector_FAST_N: i32 = 10002;
pub const FastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
pub const FastFeatureDetector_THRESHOLD: i32 = 10000;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AKAZE_DescriptorType {
	/// Upright descriptors, not invariant to rotation
	DESCRIPTOR_KAZE_UPRIGHT = 2,
	DESCRIPTOR_KAZE = 3,
	/// Upright descriptors, not invariant to rotation
	DESCRIPTOR_MLDB_UPRIGHT = 4,
	DESCRIPTOR_MLDB = 5,
}

opencv_type_enum! { crate::features2d::AKAZE_DescriptorType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AgastFeatureDetector_DetectorType {
	AGAST_5_8 = 0,
	AGAST_7_12d = 1,
	AGAST_7_12s = 2,
	OAST_9_16 = 3,
}

opencv_type_enum! { crate::features2d::AgastFeatureDetector_DetectorType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DescriptorMatcher_MatcherType {
	FLANNBASED = 1,
	BRUTEFORCE = 2,
	BRUTEFORCE_L1 = 3,
	BRUTEFORCE_HAMMING = 4,
	BRUTEFORCE_HAMMINGLUT = 5,
	BRUTEFORCE_SL2 = 6,
}

opencv_type_enum! { crate::features2d::DescriptorMatcher_MatcherType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawMatchesFlags {
	/// Output image matrix will be created (Mat::create),
	/// i.e. existing memory of output image may be reused.
	/// Two source image, matches and single keypoints will be drawn.
	/// For each keypoint only the center point will be drawn (without
	/// the circle around keypoint with keypoint size and orientation).
	DEFAULT = 0,
	/// Output image matrix will not be created (Mat::create).
	/// Matches will be drawn on existing content of output image.
	DRAW_OVER_OUTIMG = 1,
	/// Single keypoints will not be drawn.
	NOT_DRAW_SINGLE_POINTS = 2,
	/// For each keypoint the circle around keypoint with keypoint size and
	/// orientation will be drawn.
	DRAW_RICH_KEYPOINTS = 4,
}

opencv_type_enum! { crate::features2d::DrawMatchesFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FastFeatureDetector_DetectorType {
	TYPE_5_8 = 0,
	TYPE_7_12 = 1,
	TYPE_9_16 = 2,
}

opencv_type_enum! { crate::features2d::FastFeatureDetector_DetectorType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KAZE_DiffusivityType {
	DIFF_PM_G1 = 0,
	DIFF_PM_G2 = 1,
	DIFF_WEICKERT = 2,
	DIFF_CHARBONNIER = 3,
}

opencv_type_enum! { crate::features2d::KAZE_DiffusivityType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ORB_ScoreType {
	HARRIS_SCORE = 0,
	FAST_SCORE = 1,
}

opencv_type_enum! { crate::features2d::ORB_ScoreType }

/// Extractors of keypoint descriptors in OpenCV have wrappers with a common interface that enables you
/// to easily switch between different algorithms solving the same problem. This section is devoted to
/// computing descriptors represented as vectors in a multidimensional space. All objects that implement
/// the vector descriptor extractors inherit the DescriptorExtractor interface.
pub type DescriptorExtractor = crate::features2d::Feature2D;
/// Feature detectors in OpenCV have wrappers with a common interface that enables you to easily switch
/// between different algorithms solving the same problem. All objects that implement keypoint detectors
/// inherit the FeatureDetector interface.
pub type FeatureDetector = crate::features2d::Feature2D;
/// Detects corners using the AGAST algorithm
/// 
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints detected on the image.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the four neighborhoods as defined in the paper:
/// AgastFeatureDetector::AGAST_5_8, AgastFeatureDetector::AGAST_7_12d,
/// AgastFeatureDetector::AGAST_7_12s, AgastFeatureDetector::OAST_9_16
/// 
/// For non-Intel platforms, there is a tree optimised variant of AGAST with same numerical results.
/// The 32-bit binary tree tables were generated automatically from original code using perl script.
/// The perl script and examples of tree generation are placed in features2d/doc folder.
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_mair2010_agast) .
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * nonmax_suppression: true
pub fn AGAST(image: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
	input_array_arg!(image);
	unsafe { sys::cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression) }.into_result()
}

/// Detects corners using the AGAST algorithm
/// 
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints detected on the image.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the four neighborhoods as defined in the paper:
/// AgastFeatureDetector::AGAST_5_8, AgastFeatureDetector::AGAST_7_12d,
/// AgastFeatureDetector::AGAST_7_12s, AgastFeatureDetector::OAST_9_16
/// 
/// For non-Intel platforms, there is a tree optimised variant of AGAST with same numerical results.
/// The 32-bit binary tree tables were generated automatically from original code using perl script.
/// The perl script and examples of tree generation are placed in features2d/doc folder.
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_mair2010_agast) .
pub fn AGAST_with_type(image: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	unsafe { sys::cv_AGAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ) }.into_result()
}

/// Detects corners using the FAST algorithm
/// 
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints detected on the image.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
/// 
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Rosten06) .
/// 
/// 
/// Note: In Python API, types are given as cv.FAST_FEATURE_DETECTOR_TYPE_5_8,
/// cv.FAST_FEATURE_DETECTOR_TYPE_7_12 and cv.FAST_FEATURE_DETECTOR_TYPE_9_16. For corner
/// detection, use cv.FAST.detect() method.
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * nonmax_suppression: true
pub fn FAST(image: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
	input_array_arg!(image);
	unsafe { sys::cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression) }.into_result()
}

/// Detects corners using the FAST algorithm
/// 
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints detected on the image.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
/// 
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Rosten06) .
/// 
/// 
/// Note: In Python API, types are given as cv.FAST_FEATURE_DETECTOR_TYPE_5_8,
/// cv.FAST_FEATURE_DETECTOR_TYPE_7_12 and cv.FAST_FEATURE_DETECTOR_TYPE_9_16. For corner
/// detection, use cv.FAST.detect() method.
pub fn FAST_with_type(image: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	unsafe { sys::cv_FAST_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ) }.into_result()
}

pub fn compute_recall_precision_curve(matches1to2: &core::Vector::<core::Vector::<core::DMatch>>, correct_matches1to2_mask: &core::Vector::<core::Vector::<u8>>, recall_precision_curve: &mut core::Vector::<core::Point2f>) -> Result<()> {
	unsafe { sys::cv_computeRecallPrecisionCurve_const_vector_vector_DMatch__R_const_vector_vector_unsigned_char__R_vector_Point2f_R(matches1to2.as_raw_VectorOfVectorOfDMatch(), correct_matches1to2_mask.as_raw_VectorOfVectorOfu8(), recall_precision_curve.as_raw_mut_VectorOfPoint2f()) }.into_result()
}

/// Draws keypoints.
/// 
/// ## Parameters
/// * image: Source image.
/// * keypoints: Keypoints from the source image.
/// * outImage: Output image. Its content depends on the flags value defining what is drawn in the
/// output image. See possible flags bit values below.
/// * color: Color of keypoints.
/// * flags: Flags setting drawing features. Possible flags bit values are defined by
/// DrawMatchesFlags. See details above in drawMatches .
/// 
/// 
/// Note:
/// For Python API, flags are modified as cv.DRAW_MATCHES_FLAGS_DEFAULT,
/// cv.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS, cv.DRAW_MATCHES_FLAGS_DRAW_OVER_OUTIMG,
/// cv.DRAW_MATCHES_FLAGS_NOT_DRAW_SINGLE_POINTS
/// 
/// ## C++ default parameters
/// * color: Scalar::all(-1)
/// * flags: DrawMatchesFlags::DEFAULT
pub fn draw_keypoints(image: &dyn core::ToInputArray, keypoints: &core::Vector::<core::KeyPoint>, out_image: &mut dyn core::ToInputOutputArray, color: core::Scalar, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(image);
	input_output_array_arg!(out_image);
	unsafe { sys::cv_drawKeypoints_const__InputArrayR_const_vector_KeyPoint_R_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), out_image.as_raw__InputOutputArray(), &color, flags) }.into_result()
}

/// Draws the found matches of keypoints from two images.
/// 
/// ## Parameters
/// * img1: First source image.
/// * keypoints1: Keypoints from the first source image.
/// * img2: Second source image.
/// * keypoints2: Keypoints from the second source image.
/// * matches1to2: Matches from the first image to the second one, which means that keypoints1[i]
/// has a corresponding point in keypoints2[matches[i]] .
/// * outImg: Output image. Its content depends on the flags value defining what is drawn in the
/// output image. See possible flags bit values below.
/// * matchColor: Color of matches (lines and connected keypoints). If matchColor==Scalar::all(-1)
/// , the color is generated randomly.
/// * singlePointColor: Color of single keypoints (circles), which means that keypoints do not
/// have the matches. If singlePointColor==Scalar::all(-1) , the color is generated randomly.
/// * matchesMask: Mask determining which matches are drawn. If the mask is empty, all matches are
/// drawn.
/// * flags: Flags setting drawing features. Possible flags bit values are defined by
/// DrawMatchesFlags.
/// 
/// This function draws matches of keypoints from two images in the output image. Match is a line
/// connecting two keypoints (circles). See cv::DrawMatchesFlags.
/// 
/// ## C++ default parameters
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawMatchesFlags::DEFAULT
pub fn draw_matches(img1: &dyn core::ToInputArray, keypoints1: &core::Vector::<core::KeyPoint>, img2: &dyn core::ToInputArray, keypoints2: &core::Vector::<core::KeyPoint>, matches1to2: &core::Vector::<core::DMatch>, out_img: &mut dyn core::ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector::<i8>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_DMatch_R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_char_R_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfi8(), flags) }.into_result()
}

/// Draws the found matches of keypoints from two images.
/// 
/// ## Parameters
/// * img1: First source image.
/// * keypoints1: Keypoints from the first source image.
/// * img2: Second source image.
/// * keypoints2: Keypoints from the second source image.
/// * matches1to2: Matches from the first image to the second one, which means that keypoints1[i]
/// has a corresponding point in keypoints2[matches[i]] .
/// * outImg: Output image. Its content depends on the flags value defining what is drawn in the
/// output image. See possible flags bit values below.
/// * matchColor: Color of matches (lines and connected keypoints). If matchColor==Scalar::all(-1)
/// , the color is generated randomly.
/// * singlePointColor: Color of single keypoints (circles), which means that keypoints do not
/// have the matches. If singlePointColor==Scalar::all(-1) , the color is generated randomly.
/// * matchesMask: Mask determining which matches are drawn. If the mask is empty, all matches are
/// drawn.
/// * flags: Flags setting drawing features. Possible flags bit values are defined by
/// DrawMatchesFlags.
/// 
/// This function draws matches of keypoints from two images in the output image. Match is a line
/// connecting two keypoints (circles). See cv::DrawMatchesFlags.
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<std::vector<char>>()
/// * flags: DrawMatchesFlags::DEFAULT
pub fn draw_matches_knn(img1: &dyn core::ToInputArray, keypoints1: &core::Vector::<core::KeyPoint>, img2: &dyn core::ToInputArray, keypoints2: &core::Vector::<core::KeyPoint>, matches1to2: &core::Vector::<core::Vector::<core::DMatch>>, out_img: &mut dyn core::ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector::<core::Vector::<i8>>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vector_KeyPoint_R_const__InputArrayR_const_vector_KeyPoint_R_const_vector_vector_DMatch__R_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vector_vector_char__R_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfVectorOfi8(), flags) }.into_result()
}

/// *************************************************************************************\
///    Functions to evaluate the feature detectors and [generic] descriptor extractors      *
/// \***************************************************************************************
/// 
/// ## C++ default parameters
/// * fdetector: Ptr<FeatureDetector>()
pub fn evaluate_feature_detector(img1: &core::Mat, img2: &core::Mat, h1to2: &core::Mat, keypoints1: &mut core::Vector::<core::KeyPoint>, keypoints2: &mut core::Vector::<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32, fdetector: &core::Ptr::<crate::features2d::Feature2D>) -> Result<()> {
	unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vector_KeyPoint_X_vector_KeyPoint_X_floatR_intR_const_Ptr_Feature2D_R(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, fdetector.as_raw_PtrOfFeature2D()) }.into_result()
}

pub fn get_nearest_point(recall_precision_curve: &core::Vector::<core::Point2f>, l_precision: f32) -> Result<i32> {
	unsafe { sys::cv_getNearestPoint_const_vector_Point2f_R_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision) }.into_result()
}

pub fn get_recall(recall_precision_curve: &core::Vector::<core::Point2f>, l_precision: f32) -> Result<f32> {
	unsafe { sys::cv_getRecall_const_vector_Point2f_R_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision) }.into_result()
}

/// Class implementing the AKAZE keypoint detector and descriptor extractor, described in [ANB13](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_ANB13).
/// 
/// @details AKAZE descriptors can only be used with KAZE or AKAZE keypoints. This class is thread-safe.
/// 
/// 
/// Note: When you need descriptors use Feature2D::detectAndCompute, which
/// provides better performance. When using Feature2D::detect followed by
/// Feature2D::compute scale space pyramid is computed twice.
/// 
/// 
/// Note: AKAZE implements T-API. When image is passed as UMat some parts of the algorithm
/// will use OpenCL.
/// 
/// 
/// Note: [ANB13] Fast Explicit Diffusion for Accelerated Features in Nonlinear
/// Scale Spaces. Pablo F. Alcantarilla, Jesús Nuevo and Adrien Bartoli. In
/// British Machine Vision Conference (BMVC), Bristol, UK, September 2013.
pub trait AKAZE: crate::features2d::Feature2DTrait {
	fn as_raw_AKAZE(&self) -> *const c_void;
	fn as_raw_mut_AKAZE(&mut self) -> *mut c_void;

	fn set_descriptor_type(&mut self, dtype: crate::features2d::AKAZE_DescriptorType) -> Result<()> {
		unsafe { sys::cv_AKAZE_setDescriptorType_DescriptorType(self.as_raw_mut_AKAZE(), dtype) }.into_result()
	}
	
	fn get_descriptor_type(&self) -> Result<crate::features2d::AKAZE_DescriptorType> {
		unsafe { sys::cv_AKAZE_getDescriptorType_const(self.as_raw_AKAZE()) }.into_result()
	}
	
	fn set_descriptor_size(&mut self, dsize: i32) -> Result<()> {
		unsafe { sys::cv_AKAZE_setDescriptorSize_int(self.as_raw_mut_AKAZE(), dsize) }.into_result()
	}
	
	fn get_descriptor_size(&self) -> Result<i32> {
		unsafe { sys::cv_AKAZE_getDescriptorSize_const(self.as_raw_AKAZE()) }.into_result()
	}
	
	fn set_descriptor_channels(&mut self, dch: i32) -> Result<()> {
		unsafe { sys::cv_AKAZE_setDescriptorChannels_int(self.as_raw_mut_AKAZE(), dch) }.into_result()
	}
	
	fn get_descriptor_channels(&self) -> Result<i32> {
		unsafe { sys::cv_AKAZE_getDescriptorChannels_const(self.as_raw_AKAZE()) }.into_result()
	}
	
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		unsafe { sys::cv_AKAZE_setThreshold_double(self.as_raw_mut_AKAZE(), threshold) }.into_result()
	}
	
	fn get_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_AKAZE_getThreshold_const(self.as_raw_AKAZE()) }.into_result()
	}
	
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		unsafe { sys::cv_AKAZE_setNOctaves_int(self.as_raw_mut_AKAZE(), octaves) }.into_result()
	}
	
	fn get_n_octaves(&self) -> Result<i32> {
		unsafe { sys::cv_AKAZE_getNOctaves_const(self.as_raw_AKAZE()) }.into_result()
	}
	
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		unsafe { sys::cv_AKAZE_setNOctaveLayers_int(self.as_raw_mut_AKAZE(), octave_layers) }.into_result()
	}
	
	fn get_n_octave_layers(&self) -> Result<i32> {
		unsafe { sys::cv_AKAZE_getNOctaveLayers_const(self.as_raw_AKAZE()) }.into_result()
	}
	
	fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
		unsafe { sys::cv_AKAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_AKAZE(), diff) }.into_result()
	}
	
	fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
		unsafe { sys::cv_AKAZE_getDiffusivity_const(self.as_raw_AKAZE()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_AKAZE_getDefaultName_const(self.as_raw_AKAZE()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn AKAZE + '_ {
	/// The AKAZE constructor
	/// 
	/// ## Parameters
	/// * descriptor_type: Type of the extracted descriptor: DESCRIPTOR_KAZE,
	/// DESCRIPTOR_KAZE_UPRIGHT, DESCRIPTOR_MLDB or DESCRIPTOR_MLDB_UPRIGHT.
	/// * descriptor_size: Size of the descriptor in bits. 0 -\> Full size
	/// * descriptor_channels: Number of channels in the descriptor (1, 2, 3)
	/// * threshold: Detector response threshold to accept point
	/// * nOctaves: Maximum octave evolution of the image
	/// * nOctaveLayers: Default number of sublevels per scale level
	/// * diffusivity: Diffusivity type. DIFF_PM_G1, DIFF_PM_G2, DIFF_WEICKERT or
	/// DIFF_CHARBONNIER
	/// 
	/// ## C++ default parameters
	/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
	/// * descriptor_size: 0
	/// * descriptor_channels: 3
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	pub fn create(descriptor_type: crate::features2d::AKAZE_DescriptorType, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType) -> Result<core::Ptr::<dyn crate::features2d::AKAZE>> {
		unsafe { sys::cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::AKAZE>::opencv_from_extern(r) } )
	}
	
}
/// Wrapping class for feature detection using the AGAST method. :
pub trait AgastFeatureDetector: crate::features2d::Feature2DTrait {
	fn as_raw_AgastFeatureDetector(&self) -> *const c_void;
	fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void;

	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		unsafe { sys::cv_AgastFeatureDetector_setThreshold_int(self.as_raw_mut_AgastFeatureDetector(), threshold) }.into_result()
	}
	
	fn get_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_AgastFeatureDetector_getThreshold_const(self.as_raw_AgastFeatureDetector()) }.into_result()
	}
	
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		unsafe { sys::cv_AgastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_AgastFeatureDetector(), f) }.into_result()
	}
	
	fn get_nonmax_suppression(&self) -> Result<bool> {
		unsafe { sys::cv_AgastFeatureDetector_getNonmaxSuppression_const(self.as_raw_AgastFeatureDetector()) }.into_result()
	}
	
	fn set_type(&mut self, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<()> {
		unsafe { sys::cv_AgastFeatureDetector_setType_DetectorType(self.as_raw_mut_AgastFeatureDetector(), typ) }.into_result()
	}
	
	fn get_type(&self) -> Result<crate::features2d::AgastFeatureDetector_DetectorType> {
		unsafe { sys::cv_AgastFeatureDetector_getType_const(self.as_raw_AgastFeatureDetector()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_AgastFeatureDetector_getDefaultName_const(self.as_raw_AgastFeatureDetector()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn AgastFeatureDetector + '_ {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: AgastFeatureDetector::OAST_9_16
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<core::Ptr::<dyn crate::features2d::AgastFeatureDetector>> {
		unsafe { sys::cv_AgastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::AgastFeatureDetector>::opencv_from_extern(r) } )
	}
	
}
/// Brute-force descriptor matcher.
/// 
/// For each descriptor in the first set, this matcher finds the closest descriptor in the second set
/// by trying each one. This descriptor matcher supports masking permissible matches of descriptor
/// sets.
pub trait BFMatcherTrait: crate::features2d::DescriptorMatcher {
	fn as_raw_BFMatcher(&self) -> *const c_void;
	fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void;

	fn is_mask_supported(&self) -> Result<bool> {
		unsafe { sys::cv_BFMatcher_isMaskSupported_const(self.as_raw_BFMatcher()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * empty_train_data: false
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr::<dyn crate::features2d::DescriptorMatcher>> {
		unsafe { sys::cv_BFMatcher_clone_const_bool(self.as_raw_BFMatcher(), empty_train_data) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(r) } )
	}
	
}

/// Brute-force descriptor matcher.
/// 
/// For each descriptor in the first set, this matcher finds the closest descriptor in the second set
/// by trying each one. This descriptor matcher supports masking permissible matches of descriptor
/// sets.
pub struct BFMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { BFMatcher }

impl Drop for BFMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_BFMatcher_delete(instance: *mut c_void); }
		unsafe { cv_BFMatcher_delete(self.as_raw_mut_BFMatcher()) };
	}
}

impl BFMatcher {
	#[inline] pub fn as_raw_BFMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for BFMatcher {}

impl core::AlgorithmTrait for BFMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BFMatcherTrait for BFMatcher {
	#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::DescriptorMatcher for BFMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BFMatcher {
	/// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
	/// 
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	/// * cross_check: false
	pub fn new(norm_type: i32, cross_check: bool) -> Result<crate::features2d::BFMatcher> {
		unsafe { sys::cv_BFMatcher_BFMatcher_int_bool(norm_type, cross_check) }.into_result().map(|r| unsafe { crate::features2d::BFMatcher::opencv_from_extern(r) } )
	}
	
	/// Brute-force matcher create method.
	/// ## Parameters
	/// * normType: One of NORM_L1, NORM_L2, NORM_HAMMING, NORM_HAMMING2. L1 and L2 norms are
	/// preferable choices for SIFT and SURF descriptors, NORM_HAMMING should be used with ORB, BRISK and
	/// BRIEF, NORM_HAMMING2 should be used with ORB when WTA_K==3 or 4 (see ORB::ORB constructor
	/// description).
	/// * crossCheck: If it is false, this is will be default BFMatcher behaviour when it finds the k
	/// nearest neighbors for each query descriptor. If crossCheck==true, then the knnMatch() method with
	/// k=1 will only return pairs (i,j) such that for i-th query descriptor the j-th descriptor in the
	/// matcher's collection is the nearest and vice versa, i.e. the BFMatcher will only return consistent
	/// pairs. Such technique usually produces best results with minimal number of outliers when there are
	/// enough matches. This is alternative to the ratio test, used by D. Lowe in SIFT paper.
	/// 
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	/// * cross_check: false
	pub fn create(norm_type: i32, cross_check: bool) -> Result<core::Ptr::<crate::features2d::BFMatcher>> {
		unsafe { sys::cv_BFMatcher_create_int_bool(norm_type, cross_check) }.into_result().map(|r| unsafe { core::Ptr::<crate::features2d::BFMatcher>::opencv_from_extern(r) } )
	}
	
}

/// Class to compute an image descriptor using the *bag of visual words*.
/// 
/// Such a computation consists of the following steps:
/// 
/// 1.  Compute descriptors for a given image and its keypoints set.
/// 2.  Find the nearest visual words from the vocabulary for each keypoint descriptor.
/// 3.  Compute the bag-of-words image descriptor as is a normalized histogram of vocabulary words
/// encountered in the image. The i-th bin of the histogram is a frequency of i-th word of the
/// vocabulary in the given image.
pub trait BOWImgDescriptorExtractorTrait {
	fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void;
	fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void;

	/// Sets a visual vocabulary.
	/// 
	/// ## Parameters
	/// * vocabulary: Vocabulary (can be trained using the inheritor of BOWTrainer ). Each row of the
	/// vocabulary is a visual word (cluster center).
	fn set_vocabulary(&mut self, vocabulary: &core::Mat) -> Result<()> {
		unsafe { sys::cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), vocabulary.as_raw_Mat()) }.into_result()
	}
	
	/// Returns the set vocabulary.
	fn get_vocabulary(&self) -> Result<core::Mat> {
		unsafe { sys::cv_BOWImgDescriptorExtractor_getVocabulary_const(self.as_raw_BOWImgDescriptorExtractor()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Computes an image descriptor using the set visual vocabulary.
	/// 
	/// ## Parameters
	/// * image: Image, for which the descriptor is computed.
	/// * keypoints: Keypoints detected in the input image.
	/// * imgDescriptor: Computed output image descriptor.
	/// * pointIdxsOfClusters: Indices of keypoints that belong to the cluster. This means that
	/// pointIdxsOfClusters[i] are keypoint indices that belong to the i -th cluster (word of vocabulary)
	/// returned if it is non-zero.
	/// * descriptors: Descriptors of the image keypoints that are returned if they are non-zero.
	/// 
	/// ## C++ default parameters
	/// * point_idxs_of_clusters: 0
	/// * descriptors: 0
	fn compute_desc(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, img_descriptor: &mut dyn core::ToOutputArray, point_idxs_of_clusters: &mut core::Vector::<core::Vector::<i32>>, descriptors: &mut core::Mat) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(img_descriptor);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_vector_vector_int__X_MatX(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32(), descriptors.as_raw_mut_Mat()) }.into_result()
	}
	
	/// Computes an image descriptor using the set visual vocabulary.
	/// 
	/// ## Parameters
	/// * image: Image, for which the descriptor is computed.
	/// * keypoints: Keypoints detected in the input image.
	/// * imgDescriptor: Computed output image descriptor.
	/// * pointIdxsOfClusters: Indices of keypoints that belong to the cluster. This means that
	/// pointIdxsOfClusters[i] are keypoint indices that belong to the i -th cluster (word of vocabulary)
	/// returned if it is non-zero.
	/// * descriptors: Descriptors of the image keypoints that are returned if they are non-zero.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * keypointDescriptors: Computed descriptors to match with vocabulary.
	/// * imgDescriptor: Computed output image descriptor.
	/// * pointIdxsOfClusters: Indices of keypoints that belong to the cluster. This means that
	///    pointIdxsOfClusters[i] are keypoint indices that belong to the i -th cluster (word of vocabulary)
	///    returned if it is non-zero.
	/// 
	/// ## C++ default parameters
	/// * point_idxs_of_clusters: 0
	fn compute(&mut self, keypoint_descriptors: &dyn core::ToInputArray, img_descriptor: &mut dyn core::ToOutputArray, point_idxs_of_clusters: &mut core::Vector::<core::Vector::<i32>>) -> Result<()> {
		input_array_arg!(keypoint_descriptors);
		output_array_arg!(img_descriptor);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vector_vector_int__X(self.as_raw_mut_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw__InputArray(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32()) }.into_result()
	}
	
	fn compute2(&mut self, image: &core::Mat, keypoints: &mut core::Vector::<core::KeyPoint>, img_descriptor: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute2_const_MatR_vector_KeyPoint_R_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw_mut_Mat()) }.into_result()
	}
	
	/// Returns an image descriptor size if the vocabulary is set. Otherwise, it returns 0.
	fn descriptor_size(&self) -> Result<i32> {
		unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorSize_const(self.as_raw_BOWImgDescriptorExtractor()) }.into_result()
	}
	
	/// Returns an image descriptor type.
	fn descriptor_type(&self) -> Result<i32> {
		unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorType_const(self.as_raw_BOWImgDescriptorExtractor()) }.into_result()
	}
	
}

/// Class to compute an image descriptor using the *bag of visual words*.
/// 
/// Such a computation consists of the following steps:
/// 
/// 1.  Compute descriptors for a given image and its keypoints set.
/// 2.  Find the nearest visual words from the vocabulary for each keypoint descriptor.
/// 3.  Compute the bag-of-words image descriptor as is a normalized histogram of vocabulary words
/// encountered in the image. The i-th bin of the histogram is a frequency of i-th word of the
/// vocabulary in the given image.
pub struct BOWImgDescriptorExtractor {
	ptr: *mut c_void
}

opencv_type_boxed! { BOWImgDescriptorExtractor }

impl Drop for BOWImgDescriptorExtractor {
	fn drop(&mut self) {
		extern "C" { fn cv_BOWImgDescriptorExtractor_delete(instance: *mut c_void); }
		unsafe { cv_BOWImgDescriptorExtractor_delete(self.as_raw_mut_BOWImgDescriptorExtractor()) };
	}
}

impl BOWImgDescriptorExtractor {
	#[inline] pub fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for BOWImgDescriptorExtractor {}

impl crate::features2d::BOWImgDescriptorExtractorTrait for BOWImgDescriptorExtractor {
	#[inline] fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BOWImgDescriptorExtractor {
	/// The constructor.
	/// 
	/// ## Parameters
	/// * dextractor: Descriptor extractor that is used to compute descriptors for an input image and
	/// its keypoints.
	/// * dmatcher: Descriptor matcher that is used to find the nearest word of the trained vocabulary
	/// for each keypoint descriptor of the image.
	pub fn new(dextractor: &core::Ptr::<crate::features2d::Feature2D>, dmatcher: &core::Ptr::<dyn crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
		unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_Feature2D_R_const_Ptr_DescriptorMatcher_R(dextractor.as_raw_PtrOfFeature2D(), dmatcher.as_raw_PtrOfDescriptorMatcher()) }.into_result().map(|r| unsafe { crate::features2d::BOWImgDescriptorExtractor::opencv_from_extern(r) } )
	}
	
	/// The constructor.
	/// 
	/// ## Parameters
	/// * dextractor: Descriptor extractor that is used to compute descriptors for an input image and
	/// its keypoints.
	/// * dmatcher: Descriptor matcher that is used to find the nearest word of the trained vocabulary
	/// for each keypoint descriptor of the image.
	/// 
	/// ## Overloaded parameters
	pub fn new_1(dmatcher: &core::Ptr::<dyn crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
		unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_Ptr_DescriptorMatcher_R(dmatcher.as_raw_PtrOfDescriptorMatcher()) }.into_result().map(|r| unsafe { crate::features2d::BOWImgDescriptorExtractor::opencv_from_extern(r) } )
	}
	
}

/// kmeans -based class to train visual vocabulary using the *bag of visual words* approach. :
pub trait BOWKMeansTrainerTrait: crate::features2d::BOWTrainer {
	fn as_raw_BOWKMeansTrainer(&self) -> *const c_void;
	fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void;

	fn cluster(&self) -> Result<core::Mat> {
		unsafe { sys::cv_BOWKMeansTrainer_cluster_const(self.as_raw_BOWKMeansTrainer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn cluster_1(&self, descriptors: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_BOWKMeansTrainer_cluster_const_const_MatR(self.as_raw_BOWKMeansTrainer(), descriptors.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// kmeans -based class to train visual vocabulary using the *bag of visual words* approach. :
pub struct BOWKMeansTrainer {
	ptr: *mut c_void
}

opencv_type_boxed! { BOWKMeansTrainer }

impl Drop for BOWKMeansTrainer {
	fn drop(&mut self) {
		extern "C" { fn cv_BOWKMeansTrainer_delete(instance: *mut c_void); }
		unsafe { cv_BOWKMeansTrainer_delete(self.as_raw_mut_BOWKMeansTrainer()) };
	}
}

impl BOWKMeansTrainer {
	#[inline] pub fn as_raw_BOWKMeansTrainer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for BOWKMeansTrainer {}

impl crate::features2d::BOWKMeansTrainerTrait for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWKMeansTrainer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BOWTrainer for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWTrainer(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BOWKMeansTrainer {
	/// The constructor.
	/// ## See also
	/// cv::kmeans
	/// 
	/// ## C++ default parameters
	/// * termcrit: TermCriteria()
	/// * attempts: 3
	/// * flags: KMEANS_PP_CENTERS
	pub fn new(cluster_count: i32, termcrit: core::TermCriteria, attempts: i32, flags: i32) -> Result<crate::features2d::BOWKMeansTrainer> {
		unsafe { sys::cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(cluster_count, &termcrit, attempts, flags) }.into_result().map(|r| unsafe { crate::features2d::BOWKMeansTrainer::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for training the *bag of visual words* vocabulary from a set of descriptors.
/// 
/// For details, see, for example, *Visual Categorization with Bags of Keypoints* by Gabriella Csurka,
/// Christopher R. Dance, Lixin Fan, Jutta Willamowski, Cedric Bray, 2004. :
pub trait BOWTrainer {
	fn as_raw_BOWTrainer(&self) -> *const c_void;
	fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void;

	/// Adds descriptors to a training set.
	/// 
	/// ## Parameters
	/// * descriptors: Descriptors to add to a training set. Each row of the descriptors matrix is a
	/// descriptor.
	/// 
	/// The training set is clustered using clustermethod to construct the vocabulary.
	fn add(&mut self, descriptors: &core::Mat) -> Result<()> {
		unsafe { sys::cv_BOWTrainer_add_const_MatR(self.as_raw_mut_BOWTrainer(), descriptors.as_raw_Mat()) }.into_result()
	}
	
	/// Returns a training set of descriptors.
	fn get_descriptors(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_BOWTrainer_getDescriptors_const(self.as_raw_BOWTrainer()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	/// Returns the count of all descriptors stored in the training set.
	fn descriptors_count(&self) -> Result<i32> {
		unsafe { sys::cv_BOWTrainer_descriptorsCount_const(self.as_raw_BOWTrainer()) }.into_result()
	}
	
	fn clear(&mut self) -> Result<()> {
		unsafe { sys::cv_BOWTrainer_clear(self.as_raw_mut_BOWTrainer()) }.into_result()
	}
	
	/// Clusters train descriptors.
	/// 
	/// ## Parameters
	/// * descriptors: Descriptors to cluster. Each row of the descriptors matrix is a descriptor.
	/// Descriptors are not added to the inner train descriptor set.
	/// 
	/// The vocabulary consists of cluster centers. So, this method returns the vocabulary. In the first
	/// variant of the method, train descriptors stored in the object are clustered. In the second variant,
	/// input descriptors are clustered.
	/// 
	/// ## Overloaded parameters
	fn cluster(&self) -> Result<core::Mat> {
		unsafe { sys::cv_BOWTrainer_cluster_const(self.as_raw_BOWTrainer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Clusters train descriptors.
	/// 
	/// ## Parameters
	/// * descriptors: Descriptors to cluster. Each row of the descriptors matrix is a descriptor.
	/// Descriptors are not added to the inner train descriptor set.
	/// 
	/// The vocabulary consists of cluster centers. So, this method returns the vocabulary. In the first
	/// variant of the method, train descriptors stored in the object are clustered. In the second variant,
	/// input descriptors are clustered.
	fn cluster_with_descriptors(&self, descriptors: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_BOWTrainer_cluster_const_const_MatR(self.as_raw_BOWTrainer(), descriptors.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Class implementing the BRISK keypoint detector and descriptor extractor, described in [LCS11](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_LCS11) .
pub trait BRISKTrait: crate::features2d::Feature2DTrait {
	fn as_raw_BRISK(&self) -> *const c_void;
	fn as_raw_mut_BRISK(&mut self) -> *mut c_void;

	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_BRISK_getDefaultName_const(self.as_raw_BRISK()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Set detection threshold.
	/// ## Parameters
	/// * threshold: AGAST detection threshold score.
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		unsafe { sys::cv_BRISK_setThreshold_int(self.as_raw_mut_BRISK(), threshold) }.into_result()
	}
	
	fn get_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_BRISK_getThreshold_const(self.as_raw_BRISK()) }.into_result()
	}
	
	/// Set detection octaves.
	/// ## Parameters
	/// * octaves: detection octaves. Use 0 to do single scale.
	fn set_octaves(&mut self, octaves: i32) -> Result<()> {
		unsafe { sys::cv_BRISK_setOctaves_int(self.as_raw_mut_BRISK(), octaves) }.into_result()
	}
	
	fn get_octaves(&self) -> Result<i32> {
		unsafe { sys::cv_BRISK_getOctaves_const(self.as_raw_BRISK()) }.into_result()
	}
	
}

/// Class implementing the BRISK keypoint detector and descriptor extractor, described in [LCS11](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_LCS11) .
pub struct BRISK {
	ptr: *mut c_void
}

opencv_type_boxed! { BRISK }

impl Drop for BRISK {
	fn drop(&mut self) {
		extern "C" { fn cv_BRISK_delete(instance: *mut c_void); }
		unsafe { cv_BRISK_delete(self.as_raw_mut_BRISK()) };
	}
}

impl BRISK {
	#[inline] pub fn as_raw_BRISK(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for BRISK {}

impl core::AlgorithmTrait for BRISK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::BRISKTrait for BRISK {
	#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTrait for BRISK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BRISK {
	/// The BRISK constructor
	/// 
	/// ## Parameters
	/// * thresh: AGAST detection threshold score.
	/// * octaves: detection octaves. Use 0 to do single scale.
	/// * patternScale: apply this scale to the pattern used for sampling the neighbourhood of a
	/// keypoint.
	/// 
	/// ## C++ default parameters
	/// * thresh: 30
	/// * octaves: 3
	/// * pattern_scale: 1.0f
	pub fn create(thresh: i32, octaves: i32, pattern_scale: f32) -> Result<core::Ptr::<crate::features2d::BRISK>> {
		unsafe { sys::cv_BRISK_create_int_int_float(thresh, octaves, pattern_scale) }.into_result().map(|r| unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(r) } )
	}
	
	/// The BRISK constructor for a custom pattern
	/// 
	/// ## Parameters
	/// * radiusList: defines the radii (in pixels) where the samples around a keypoint are taken (for
	/// keypoint scale 1).
	/// * numberList: defines the number of sampling points on the sampling circle. Must be the same
	/// size as radiusList..
	/// * dMax: threshold for the short pairings used for descriptor formation (in pixels for keypoint
	/// scale 1).
	/// * dMin: threshold for the long pairings used for orientation determination (in pixels for
	/// keypoint scale 1).
	/// * indexChange: index remapping of the bits.
	/// 
	/// ## C++ default parameters
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	pub fn create_with_pattern(radius_list: &core::Vector::<f32>, number_list: &core::Vector::<i32>, d_max: f32, d_min: f32, index_change: &core::Vector::<i32>) -> Result<core::Ptr::<crate::features2d::BRISK>> {
		unsafe { sys::cv_BRISK_create_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(r) } )
	}
	
	/// The BRISK constructor for a custom pattern, detection threshold and octaves
	/// 
	/// ## Parameters
	/// * thresh: AGAST detection threshold score.
	/// * octaves: detection octaves. Use 0 to do single scale.
	/// * radiusList: defines the radii (in pixels) where the samples around a keypoint are taken (for
	/// keypoint scale 1).
	/// * numberList: defines the number of sampling points on the sampling circle. Must be the same
	/// size as radiusList..
	/// * dMax: threshold for the short pairings used for descriptor formation (in pixels for keypoint
	/// scale 1).
	/// * dMin: threshold for the long pairings used for orientation determination (in pixels for
	/// keypoint scale 1).
	/// * indexChange: index remapping of the bits.
	/// 
	/// ## C++ default parameters
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	pub fn create_with_pattern_threshold_octaves(thresh: i32, octaves: i32, radius_list: &core::Vector::<f32>, number_list: &core::Vector::<i32>, d_max: f32, d_min: f32, index_change: &core::Vector::<i32>) -> Result<core::Ptr::<crate::features2d::BRISK>> {
		unsafe { sys::cv_BRISK_create_int_int_const_vector_float_R_const_vector_int_R_float_float_const_vector_int_R(thresh, octaves, radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for matching keypoint descriptors.
/// 
/// It has two groups of match methods: for matching descriptors of an image with another image or with
/// an image set.
pub trait DescriptorMatcher: core::AlgorithmTrait {
	fn as_raw_DescriptorMatcher(&self) -> *const c_void;
	fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void;

	/// Adds descriptors to train a CPU(trainDescCollectionis) or GPU(utrainDescCollectionis) descriptor
	/// collection.
	/// 
	/// If the collection is not empty, the new descriptors are added to existing train descriptors.
	/// 
	/// ## Parameters
	/// * descriptors: Descriptors to add. Each descriptors[i] is a set of descriptors from the same
	/// train image.
	fn add(&mut self, descriptors: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(descriptors);
		unsafe { sys::cv_DescriptorMatcher_add_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), descriptors.as_raw__InputArray()) }.into_result()
	}
	
	/// Returns a constant link to the train descriptor collection trainDescCollection .
	fn get_train_descriptors(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_DescriptorMatcher_getTrainDescriptors_const(self.as_raw_DescriptorMatcher()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	/// Clears the train descriptor collections.
	fn clear(&mut self) -> Result<()> {
		unsafe { sys::cv_DescriptorMatcher_clear(self.as_raw_mut_DescriptorMatcher()) }.into_result()
	}
	
	/// Returns true if there are no train descriptors in the both collections.
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_DescriptorMatcher_empty_const(self.as_raw_DescriptorMatcher()) }.into_result()
	}
	
	/// Returns true if the descriptor matcher supports masking permissible matches.
	fn is_mask_supported(&self) -> Result<bool> {
		unsafe { sys::cv_DescriptorMatcher_isMaskSupported_const(self.as_raw_DescriptorMatcher()) }.into_result()
	}
	
	/// Trains a descriptor matcher
	/// 
	/// Trains a descriptor matcher (for example, the flann index). In all methods to match, the method
	/// train() is run every time before matching. Some descriptor matchers (for example, BruteForceMatcher)
	/// have an empty implementation of this method. Other matchers really train their inner structures (for
	/// example, FlannBasedMatcher trains flann::Index ).
	fn train(&mut self) -> Result<()> {
		unsafe { sys::cv_DescriptorMatcher_train(self.as_raw_mut_DescriptorMatcher()) }.into_result()
	}
	
	/// Finds the best match for each descriptor from a query set.
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches. If a query descriptor is masked out in mask , no match is added for this
	/// descriptor. So, matches size may be smaller than the query descriptors count.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// 
	/// In the first variant of this method, the train descriptors are passed as an input argument. In the
	/// second variant of the method, train descriptors collection that was set by DescriptorMatcher::add is
	/// used. Optional mask (or masks) can be passed to specify which query and training descriptors can be
	/// matched. Namely, queryDescriptors[i] can be matched with trainDescriptors[j] only if
	/// mask.at\<uchar\>(i,j) is non-zero.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	fn train_match(&self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::DMatch>, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		unsafe { sys::cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vector_DMatch_R_const__InputArrayR(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// Finds the k best matches for each descriptor from a query set.
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * matches: Matches. Each matches[i] is k or less matches for the same query descriptor.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	/// less than k possible matches in total.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// These extended variants of DescriptorMatcher::match methods find several best matches for each query
	/// descriptor. The matches are returned in the distance increasing order. See DescriptorMatcher::match
	/// for the details about query and train descriptors.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * compact_result: false
	fn knn_train_match(&self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, k: i32, mask: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw__InputArray(), compact_result) }.into_result()
	}
	
	/// For each query descriptor, finds the training descriptors not farther than the specified distance.
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Found matches.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// 
	/// For each query descriptor, the methods find such training descriptors that the distance between the
	/// query descriptor and the training descriptor is equal or smaller than maxDistance. Found matches are
	/// returned in the distance increasing order.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * compact_result: false
	fn radius_train_match(&self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, max_distance: f32, mask: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw__InputArray(), compact_result) }.into_result()
	}
	
	/// Finds the best match for each descriptor from a query set.
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches. If a query descriptor is masked out in mask , no match is added for this
	/// descriptor. So, matches size may be smaller than the query descriptors count.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// 
	/// In the first variant of this method, the train descriptors are passed as an input argument. In the
	/// second variant of the method, train descriptors collection that was set by DescriptorMatcher::add is
	/// used. Optional mask (or masks) can be passed to specify which query and training descriptors can be
	/// matched. Namely, queryDescriptors[i] can be matched with trainDescriptors[j] only if
	/// mask.at\<uchar\>(i,j) is non-zero.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * queryDescriptors: Query set of descriptors.
	/// * matches: Matches. If a query descriptor is masked out in mask , no match is added for this
	///    descriptor. So, matches size may be smaller than the query descriptors count.
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	///    descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	/// 
	/// ## C++ default parameters
	/// * masks: noArray()
	fn match_(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::DMatch>, masks: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		unsafe { sys::cv_DescriptorMatcher_match_const__InputArrayR_vector_DMatch_R_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw__InputArray()) }.into_result()
	}
	
	/// Finds the k best matches for each descriptor from a query set.
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * matches: Matches. Each matches[i] is k or less matches for the same query descriptor.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	/// less than k possible matches in total.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// These extended variants of DescriptorMatcher::match methods find several best matches for each query
	/// descriptor. The matches are returned in the distance increasing order. See DescriptorMatcher::match
	/// for the details about query and train descriptors.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * queryDescriptors: Query set of descriptors.
	/// * matches: Matches. Each matches[i] is k or less matches for the same query descriptor.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	///    less than k possible matches in total.
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	///    descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	///    false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	///    the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// ## C++ default parameters
	/// * masks: noArray()
	/// * compact_result: false
	fn knn_match(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, k: i32, masks: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw__InputArray(), compact_result) }.into_result()
	}
	
	/// For each query descriptor, finds the training descriptors not farther than the specified distance.
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Found matches.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// 
	/// For each query descriptor, the methods find such training descriptors that the distance between the
	/// query descriptor and the training descriptor is equal or smaller than maxDistance. Found matches are
	/// returned in the distance increasing order.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * queryDescriptors: Query set of descriptors.
	/// * matches: Found matches.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	///    metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	///    in Pixels)!
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	///    descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	///    false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	///    the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// ## C++ default parameters
	/// * masks: noArray()
	/// * compact_result: false
	fn radius_match(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, max_distance: f32, masks: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw__InputArray(), compact_result) }.into_result()
	}
	
	fn write(&self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		unsafe { sys::cv_DescriptorMatcher_write_const_const_StringR(self.as_raw_DescriptorMatcher(), file_name.opencv_as_extern()) }.into_result()
	}
	
	fn read(&mut self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		unsafe { sys::cv_DescriptorMatcher_read_const_StringR(self.as_raw_mut_DescriptorMatcher(), file_name.opencv_as_extern()) }.into_result()
	}
	
	fn read_1(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_DescriptorMatcher_read_const_FileNodeR(self.as_raw_mut_DescriptorMatcher(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	fn write_1(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_DescriptorMatcher_write_const_FileStorageR(self.as_raw_DescriptorMatcher(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// Clones the matcher.
	/// 
	/// ## Parameters
	/// * emptyTrainData: If emptyTrainData is false, the method creates a deep copy of the object,
	/// that is, copies both parameters and train data. If emptyTrainData is true, the method creates an
	/// object copy with the current parameters but with empty train data.
	/// 
	/// ## C++ default parameters
	/// * empty_train_data: false
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr::<dyn crate::features2d::DescriptorMatcher>> {
		unsafe { sys::cv_DescriptorMatcher_clone_const_bool(self.as_raw_DescriptorMatcher(), empty_train_data) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * name: String()
	fn write_2(&self, fs: &core::Ptr::<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		unsafe { sys::cv_DescriptorMatcher_write_const_const_Ptr_FileStorage_R_const_StringR(self.as_raw_DescriptorMatcher(), fs.as_raw_PtrOfFileStorage(), name.opencv_as_extern()) }.into_result()
	}
	
}

impl dyn DescriptorMatcher + '_ {
	/// Creates a descriptor matcher of a given type with the default parameters (using default
	/// constructor).
	/// 
	/// ## Parameters
	/// * descriptorMatcherType: Descriptor matcher type. Now the following matcher types are
	/// supported:
	/// *   `BruteForce` (it uses L2 )
	/// *   `BruteForce-L1`
	/// *   `BruteForce-Hamming`
	/// *   `BruteForce-Hamming(2)`
	/// *   `FlannBased`
	pub fn create(descriptor_matcher_type: &str) -> Result<core::Ptr::<dyn crate::features2d::DescriptorMatcher>> {
		extern_container_arg!(descriptor_matcher_type);
		unsafe { sys::cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(r) } )
	}
	
	pub fn create_with_matcher_type(matcher_type: crate::features2d::DescriptorMatcher_MatcherType) -> Result<core::Ptr::<dyn crate::features2d::DescriptorMatcher>> {
		unsafe { sys::cv_DescriptorMatcher_create_const_MatcherTypeR(&matcher_type) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(r) } )
	}
	
}
/// Wrapping class for feature detection using the FAST method. :
pub trait FastFeatureDetector: crate::features2d::Feature2DTrait {
	fn as_raw_FastFeatureDetector(&self) -> *const c_void;
	fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void;

	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		unsafe { sys::cv_FastFeatureDetector_setThreshold_int(self.as_raw_mut_FastFeatureDetector(), threshold) }.into_result()
	}
	
	fn get_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_FastFeatureDetector_getThreshold_const(self.as_raw_FastFeatureDetector()) }.into_result()
	}
	
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		unsafe { sys::cv_FastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_FastFeatureDetector(), f) }.into_result()
	}
	
	fn get_nonmax_suppression(&self) -> Result<bool> {
		unsafe { sys::cv_FastFeatureDetector_getNonmaxSuppression_const(self.as_raw_FastFeatureDetector()) }.into_result()
	}
	
	fn set_type(&mut self, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
		unsafe { sys::cv_FastFeatureDetector_setType_DetectorType(self.as_raw_mut_FastFeatureDetector(), typ) }.into_result()
	}
	
	fn get_type(&self) -> Result<crate::features2d::FastFeatureDetector_DetectorType> {
		unsafe { sys::cv_FastFeatureDetector_getType_const(self.as_raw_FastFeatureDetector()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_FastFeatureDetector_getDefaultName_const(self.as_raw_FastFeatureDetector()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn FastFeatureDetector + '_ {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: FastFeatureDetector::TYPE_9_16
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<core::Ptr::<dyn crate::features2d::FastFeatureDetector>> {
		unsafe { sys::cv_FastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::FastFeatureDetector>::opencv_from_extern(r) } )
	}
	
}
pub trait Feature2DTrait: core::AlgorithmTrait {
	fn as_raw_Feature2D(&self) -> *const c_void;
	fn as_raw_mut_Feature2D(&mut self) -> *mut c_void;

	/// Detects keypoints in an image (first variant) or image set (second variant).
	/// 
	/// ## Parameters
	/// * image: Image.
	/// * keypoints: The detected keypoints. In the second variant of the method keypoints[i] is a set
	/// of keypoints detected in images[i] .
	/// * mask: Mask specifying where to look for keypoints (optional). It must be a 8-bit integer
	/// matrix with non-zero values in the region of interest.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	fn detect(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vector_KeyPoint_R_const__InputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// Detects keypoints in an image (first variant) or image set (second variant).
	/// 
	/// ## Parameters
	/// * image: Image.
	/// * keypoints: The detected keypoints. In the second variant of the method keypoints[i] is a set
	/// of keypoints detected in images[i] .
	/// * mask: Mask specifying where to look for keypoints (optional). It must be a 8-bit integer
	/// matrix with non-zero values in the region of interest.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * images: Image set.
	/// * keypoints: The detected keypoints. In the second variant of the method keypoints[i] is a set
	///    of keypoints detected in images[i] .
	/// * masks: Masks for each input image specifying where to look for keypoints (optional).
	///    masks[i] is a mask for images[i].
	/// 
	/// ## C++ default parameters
	/// * masks: noArray()
	fn detect_multiple(&mut self, images: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::Vector::<core::KeyPoint>>, masks: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(images);
		input_array_arg!(masks);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vector_vector_KeyPoint__R_const__InputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), masks.as_raw__InputArray()) }.into_result()
	}
	
	/// Computes the descriptors for a set of keypoints detected in an image (first variant) or image set
	/// (second variant).
	/// 
	/// ## Parameters
	/// * image: Image.
	/// * keypoints: Input collection of keypoints. Keypoints for which a descriptor cannot be
	/// computed are removed. Sometimes new keypoints can be added, for example: SIFT duplicates keypoint
	/// with several dominant orientations (for each orientation).
	/// * descriptors: Computed descriptors. In the second variant of the method descriptors[i] are
	/// descriptors computed for a keypoints[i]. Row j is the keypoints (or keypoints[i]) is the
	/// descriptor for keypoint j-th keypoint.
	fn compute(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray()) }.into_result()
	}
	
	/// Computes the descriptors for a set of keypoints detected in an image (first variant) or image set
	/// (second variant).
	/// 
	/// ## Parameters
	/// * image: Image.
	/// * keypoints: Input collection of keypoints. Keypoints for which a descriptor cannot be
	/// computed are removed. Sometimes new keypoints can be added, for example: SIFT duplicates keypoint
	/// with several dominant orientations (for each orientation).
	/// * descriptors: Computed descriptors. In the second variant of the method descriptors[i] are
	/// descriptors computed for a keypoints[i]. Row j is the keypoints (or keypoints[i]) is the
	/// descriptor for keypoint j-th keypoint.
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// * images: Image set.
	/// * keypoints: Input collection of keypoints. Keypoints for which a descriptor cannot be
	///    computed are removed. Sometimes new keypoints can be added, for example: SIFT duplicates keypoint
	///    with several dominant orientations (for each orientation).
	/// * descriptors: Computed descriptors. In the second variant of the method descriptors[i] are
	///    descriptors computed for a keypoints[i]. Row j is the keypoints (or keypoints[i]) is the
	///    descriptor for keypoint j-th keypoint.
	fn compute_multiple(&mut self, images: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::Vector::<core::KeyPoint>>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(descriptors);
		unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray()) }.into_result()
	}
	
	/// Detects keypoints and computes the descriptors
	/// 
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	fn detect_and_compute(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>, descriptors: &mut dyn core::ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		unsafe { sys::cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR_bool(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints) }.into_result()
	}
	
	fn descriptor_size(&self) -> Result<i32> {
		unsafe { sys::cv_Feature2D_descriptorSize_const(self.as_raw_Feature2D()) }.into_result()
	}
	
	fn descriptor_type(&self) -> Result<i32> {
		unsafe { sys::cv_Feature2D_descriptorType_const(self.as_raw_Feature2D()) }.into_result()
	}
	
	fn default_norm(&self) -> Result<i32> {
		unsafe { sys::cv_Feature2D_defaultNorm_const(self.as_raw_Feature2D()) }.into_result()
	}
	
	fn write(&self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		unsafe { sys::cv_Feature2D_write_const_const_StringR(self.as_raw_Feature2D(), file_name.opencv_as_extern()) }.into_result()
	}
	
	fn read(&mut self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		unsafe { sys::cv_Feature2D_read_const_StringR(self.as_raw_mut_Feature2D(), file_name.opencv_as_extern()) }.into_result()
	}
	
	fn write_1(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_Feature2D_write_const_FileStorageR(self.as_raw_Feature2D(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
	fn read_1(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_Feature2D_read_const_FileNodeR(self.as_raw_mut_Feature2D(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	/// Return true if detector object is empty
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_Feature2D_empty_const(self.as_raw_Feature2D()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_Feature2D_getDefaultName_const(self.as_raw_Feature2D()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * name: String()
	fn write_2(&self, fs: &core::Ptr::<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		unsafe { sys::cv_Feature2D_write_const_const_Ptr_FileStorage_R_const_StringR(self.as_raw_Feature2D(), fs.as_raw_PtrOfFileStorage(), name.opencv_as_extern()) }.into_result()
	}
	
}

pub struct Feature2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Feature2D }

impl Drop for Feature2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Feature2D_delete(instance: *mut c_void); }
		unsafe { cv_Feature2D_delete(self.as_raw_mut_Feature2D()) };
	}
}

impl Feature2D {
	#[inline] pub fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Feature2D {}

impl core::AlgorithmTrait for Feature2D {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTrait for Feature2D {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Feature2D {
}

/// Flann-based descriptor matcher.
/// 
/// This matcher trains cv::flann::Index on a train descriptor collection and calls its nearest search
/// methods to find the best matches. So, this matcher may be faster when matching a large train
/// collection than the brute force matcher. FlannBasedMatcher does not support masking permissible
/// matches of descriptor sets because flann::Index does not support this. :
pub trait FlannBasedMatcherTrait: crate::features2d::DescriptorMatcher {
	fn as_raw_FlannBasedMatcher(&self) -> *const c_void;
	fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void;

	fn add(&mut self, descriptors: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(descriptors);
		unsafe { sys::cv_FlannBasedMatcher_add_const__InputArrayR(self.as_raw_mut_FlannBasedMatcher(), descriptors.as_raw__InputArray()) }.into_result()
	}
	
	fn clear(&mut self) -> Result<()> {
		unsafe { sys::cv_FlannBasedMatcher_clear(self.as_raw_mut_FlannBasedMatcher()) }.into_result()
	}
	
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_FlannBasedMatcher_read_const_FileNodeR(self.as_raw_mut_FlannBasedMatcher(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_FlannBasedMatcher_write_const_FileStorageR(self.as_raw_FlannBasedMatcher(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
	fn train(&mut self) -> Result<()> {
		unsafe { sys::cv_FlannBasedMatcher_train(self.as_raw_mut_FlannBasedMatcher()) }.into_result()
	}
	
	fn is_mask_supported(&self) -> Result<bool> {
		unsafe { sys::cv_FlannBasedMatcher_isMaskSupported_const(self.as_raw_FlannBasedMatcher()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * empty_train_data: false
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr::<dyn crate::features2d::DescriptorMatcher>> {
		unsafe { sys::cv_FlannBasedMatcher_clone_const_bool(self.as_raw_FlannBasedMatcher(), empty_train_data) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::DescriptorMatcher>::opencv_from_extern(r) } )
	}
	
}

/// Flann-based descriptor matcher.
/// 
/// This matcher trains cv::flann::Index on a train descriptor collection and calls its nearest search
/// methods to find the best matches. So, this matcher may be faster when matching a large train
/// collection than the brute force matcher. FlannBasedMatcher does not support masking permissible
/// matches of descriptor sets because flann::Index does not support this. :
pub struct FlannBasedMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { FlannBasedMatcher }

impl Drop for FlannBasedMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_FlannBasedMatcher_delete(instance: *mut c_void); }
		unsafe { cv_FlannBasedMatcher_delete(self.as_raw_mut_FlannBasedMatcher()) };
	}
}

impl FlannBasedMatcher {
	#[inline] pub fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FlannBasedMatcher {}

impl core::AlgorithmTrait for FlannBasedMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::DescriptorMatcher for FlannBasedMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::FlannBasedMatcherTrait for FlannBasedMatcher {
	#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FlannBasedMatcher {
	/// ## C++ default parameters
	/// * index_params: makePtr<flann::KDTreeIndexParams>()
	/// * search_params: makePtr<flann::SearchParams>()
	pub fn new(index_params: &core::Ptr::<crate::flann::IndexParams>, search_params: &core::Ptr::<crate::flann::SearchParams>) -> Result<crate::features2d::FlannBasedMatcher> {
		unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher_const_Ptr_IndexParams_R_const_Ptr_SearchParams_R(index_params.as_raw_PtrOfIndexParams(), search_params.as_raw_PtrOfSearchParams()) }.into_result().map(|r| unsafe { crate::features2d::FlannBasedMatcher::opencv_from_extern(r) } )
	}
	
	pub fn create() -> Result<core::Ptr::<crate::features2d::FlannBasedMatcher>> {
		unsafe { sys::cv_FlannBasedMatcher_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::features2d::FlannBasedMatcher>::opencv_from_extern(r) } )
	}
	
}

/// Wrapping class for feature detection using the goodFeaturesToTrack function. :
pub trait GFTTDetector: crate::features2d::Feature2DTrait {
	fn as_raw_GFTTDetector(&self) -> *const c_void;
	fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void;

	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		unsafe { sys::cv_GFTTDetector_setMaxFeatures_int(self.as_raw_mut_GFTTDetector(), max_features) }.into_result()
	}
	
	fn get_max_features(&self) -> Result<i32> {
		unsafe { sys::cv_GFTTDetector_getMaxFeatures_const(self.as_raw_GFTTDetector()) }.into_result()
	}
	
	fn set_quality_level(&mut self, qlevel: f64) -> Result<()> {
		unsafe { sys::cv_GFTTDetector_setQualityLevel_double(self.as_raw_mut_GFTTDetector(), qlevel) }.into_result()
	}
	
	fn get_quality_level(&self) -> Result<f64> {
		unsafe { sys::cv_GFTTDetector_getQualityLevel_const(self.as_raw_GFTTDetector()) }.into_result()
	}
	
	fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
		unsafe { sys::cv_GFTTDetector_setMinDistance_double(self.as_raw_mut_GFTTDetector(), min_distance) }.into_result()
	}
	
	fn get_min_distance(&self) -> Result<f64> {
		unsafe { sys::cv_GFTTDetector_getMinDistance_const(self.as_raw_GFTTDetector()) }.into_result()
	}
	
	fn set_block_size(&mut self, block_size: i32) -> Result<()> {
		unsafe { sys::cv_GFTTDetector_setBlockSize_int(self.as_raw_mut_GFTTDetector(), block_size) }.into_result()
	}
	
	fn get_block_size(&self) -> Result<i32> {
		unsafe { sys::cv_GFTTDetector_getBlockSize_const(self.as_raw_GFTTDetector()) }.into_result()
	}
	
	fn set_harris_detector(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_GFTTDetector_setHarrisDetector_bool(self.as_raw_mut_GFTTDetector(), val) }.into_result()
	}
	
	fn get_harris_detector(&self) -> Result<bool> {
		unsafe { sys::cv_GFTTDetector_getHarrisDetector_const(self.as_raw_GFTTDetector()) }.into_result()
	}
	
	fn set_k(&mut self, k: f64) -> Result<()> {
		unsafe { sys::cv_GFTTDetector_setK_double(self.as_raw_mut_GFTTDetector(), k) }.into_result()
	}
	
	fn get_k(&self) -> Result<f64> {
		unsafe { sys::cv_GFTTDetector_getK_const(self.as_raw_GFTTDetector()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_GFTTDetector_getDefaultName_const(self.as_raw_GFTTDetector()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn GFTTDetector + '_ {
	/// ## C++ default parameters
	/// * max_corners: 1000
	/// * quality_level: 0.01
	/// * min_distance: 1
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	pub fn create(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr::<dyn crate::features2d::GFTTDetector>> {
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners, quality_level, min_distance, block_size, use_harris_detector, k) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::GFTTDetector>::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * use_harris_detector: false
	/// * k: 0.04
	pub fn create_with_gradient(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr::<dyn crate::features2d::GFTTDetector>> {
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners, quality_level, min_distance, block_size, gradiant_size, use_harris_detector, k) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::GFTTDetector>::opencv_from_extern(r) } )
	}
	
}
/// Class implementing the KAZE keypoint detector and descriptor extractor, described in [ABD12](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_ABD12) .
/// 
/// 
/// Note: AKAZE descriptor can only be used with KAZE or AKAZE keypoints .. [ABD12] KAZE Features. Pablo
/// F. Alcantarilla, Adrien Bartoli and Andrew J. Davison. In European Conference on Computer Vision
/// (ECCV), Fiorenze, Italy, October 2012.
pub trait KAZE: crate::features2d::Feature2DTrait {
	fn as_raw_KAZE(&self) -> *const c_void;
	fn as_raw_mut_KAZE(&mut self) -> *mut c_void;

	fn set_extended(&mut self, extended: bool) -> Result<()> {
		unsafe { sys::cv_KAZE_setExtended_bool(self.as_raw_mut_KAZE(), extended) }.into_result()
	}
	
	fn get_extended(&self) -> Result<bool> {
		unsafe { sys::cv_KAZE_getExtended_const(self.as_raw_KAZE()) }.into_result()
	}
	
	fn set_upright(&mut self, upright: bool) -> Result<()> {
		unsafe { sys::cv_KAZE_setUpright_bool(self.as_raw_mut_KAZE(), upright) }.into_result()
	}
	
	fn get_upright(&self) -> Result<bool> {
		unsafe { sys::cv_KAZE_getUpright_const(self.as_raw_KAZE()) }.into_result()
	}
	
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		unsafe { sys::cv_KAZE_setThreshold_double(self.as_raw_mut_KAZE(), threshold) }.into_result()
	}
	
	fn get_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_KAZE_getThreshold_const(self.as_raw_KAZE()) }.into_result()
	}
	
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		unsafe { sys::cv_KAZE_setNOctaves_int(self.as_raw_mut_KAZE(), octaves) }.into_result()
	}
	
	fn get_n_octaves(&self) -> Result<i32> {
		unsafe { sys::cv_KAZE_getNOctaves_const(self.as_raw_KAZE()) }.into_result()
	}
	
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		unsafe { sys::cv_KAZE_setNOctaveLayers_int(self.as_raw_mut_KAZE(), octave_layers) }.into_result()
	}
	
	fn get_n_octave_layers(&self) -> Result<i32> {
		unsafe { sys::cv_KAZE_getNOctaveLayers_const(self.as_raw_KAZE()) }.into_result()
	}
	
	fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
		unsafe { sys::cv_KAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_KAZE(), diff) }.into_result()
	}
	
	fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
		unsafe { sys::cv_KAZE_getDiffusivity_const(self.as_raw_KAZE()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_KAZE_getDefaultName_const(self.as_raw_KAZE()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn KAZE + '_ {
	/// The KAZE constructor
	/// 
	/// ## Parameters
	/// * extended: Set to enable extraction of extended (128-byte) descriptor.
	/// * upright: Set to enable use of upright descriptors (non rotation-invariant).
	/// * threshold: Detector response threshold to accept point
	/// * nOctaves: Maximum octave evolution of the image
	/// * nOctaveLayers: Default number of sublevels per scale level
	/// * diffusivity: Diffusivity type. DIFF_PM_G1, DIFF_PM_G2, DIFF_WEICKERT or
	/// DIFF_CHARBONNIER
	/// 
	/// ## C++ default parameters
	/// * extended: false
	/// * upright: false
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	pub fn create(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType) -> Result<core::Ptr::<dyn crate::features2d::KAZE>> {
		unsafe { sys::cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(extended, upright, threshold, n_octaves, n_octave_layers, diffusivity) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::KAZE>::opencv_from_extern(r) } )
	}
	
}
/// A class filters a vector of keypoints.
/// 
/// Because now it is difficult to provide a convenient interface for all usage scenarios of the
/// keypoints filter class, it has only several needed by now static methods.
pub trait KeyPointsFilterTrait {
	fn as_raw_KeyPointsFilter(&self) -> *const c_void;
	fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void;

}

/// A class filters a vector of keypoints.
/// 
/// Because now it is difficult to provide a convenient interface for all usage scenarios of the
/// keypoints filter class, it has only several needed by now static methods.
pub struct KeyPointsFilter {
	ptr: *mut c_void
}

opencv_type_boxed! { KeyPointsFilter }

impl Drop for KeyPointsFilter {
	fn drop(&mut self) {
		extern "C" { fn cv_KeyPointsFilter_delete(instance: *mut c_void); }
		unsafe { cv_KeyPointsFilter_delete(self.as_raw_mut_KeyPointsFilter()) };
	}
}

impl KeyPointsFilter {
	#[inline] pub fn as_raw_KeyPointsFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for KeyPointsFilter {}

impl crate::features2d::KeyPointsFilterTrait for KeyPointsFilter {
	#[inline] fn as_raw_KeyPointsFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KeyPointsFilter {
	pub fn default() -> Result<crate::features2d::KeyPointsFilter> {
		unsafe { sys::cv_KeyPointsFilter_KeyPointsFilter() }.into_result().map(|r| unsafe { crate::features2d::KeyPointsFilter::opencv_from_extern(r) } )
	}
	
	pub fn run_by_image_border(keypoints: &mut core::Vector::<core::KeyPoint>, image_size: core::Size, border_size: i32) -> Result<()> {
		unsafe { sys::cv_KeyPointsFilter_runByImageBorder_vector_KeyPoint_R_Size_int(keypoints.as_raw_mut_VectorOfKeyPoint(), image_size.opencv_as_extern(), border_size) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * max_size: FLT_MAX
	pub fn run_by_keypoint_size(keypoints: &mut core::Vector::<core::KeyPoint>, min_size: f32, max_size: f32) -> Result<()> {
		unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vector_KeyPoint_R_float_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, max_size) }.into_result()
	}
	
	pub fn run_by_pixels_mask(keypoints: &mut core::Vector::<core::KeyPoint>, mask: &core::Mat) -> Result<()> {
		unsafe { sys::cv_KeyPointsFilter_runByPixelsMask_vector_KeyPoint_R_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw_Mat()) }.into_result()
	}
	
	pub fn remove_duplicated(keypoints: &mut core::Vector::<core::KeyPoint>) -> Result<()> {
		unsafe { sys::cv_KeyPointsFilter_removeDuplicated_vector_KeyPoint_R(keypoints.as_raw_mut_VectorOfKeyPoint()) }.into_result()
	}
	
	pub fn remove_duplicated_sorted(keypoints: &mut core::Vector::<core::KeyPoint>) -> Result<()> {
		unsafe { sys::cv_KeyPointsFilter_removeDuplicatedSorted_vector_KeyPoint_R(keypoints.as_raw_mut_VectorOfKeyPoint()) }.into_result()
	}
	
	pub fn retain_best(keypoints: &mut core::Vector::<core::KeyPoint>, npoints: i32) -> Result<()> {
		unsafe { sys::cv_KeyPointsFilter_retainBest_vector_KeyPoint_R_int(keypoints.as_raw_mut_VectorOfKeyPoint(), npoints) }.into_result()
	}
	
}

/// Maximally stable extremal region extractor
/// 
/// The class encapsulates all the parameters of the %MSER extraction algorithm (see [wiki
/// article](http://en.wikipedia.org/wiki/Maximally_stable_extremal_regions)).
/// 
/// - there are two different implementation of %MSER: one for grey image, one for color image
/// 
/// - the grey image algorithm is taken from: [nister2008linear](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_nister2008linear) ;  the paper claims to be faster
/// than union-find method; it actually get 1.5~2m/s on my centrino L7200 1.2GHz laptop.
/// 
/// - the color image algorithm is taken from: [forssen2007maximally](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_forssen2007maximally) ; it should be much slower
/// than grey image method ( 3~4 times ); the chi_table.h file is taken directly from paper's source
/// code which is distributed under GPL.
/// 
/// - (Python) A complete example showing the use of the %MSER detector can be found at samples/python/mser.py
pub trait MSER: crate::features2d::Feature2DTrait {
	fn as_raw_MSER(&self) -> *const c_void;
	fn as_raw_mut_MSER(&mut self) -> *mut c_void;

	/// Detect %MSER regions
	/// 
	/// ## Parameters
	/// * image: input image (8UC1, 8UC3 or 8UC4, must be greater or equal than 3x3)
	/// * msers: resulting list of point sets
	/// * bboxes: resulting bounding boxes
	fn detect_regions(&mut self, image: &dyn core::ToInputArray, msers: &mut core::Vector::<core::Vector::<core::Point>>, bboxes: &mut core::Vector::<core::Rect>) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_MSER_detectRegions_const__InputArrayR_vector_vector_Point__R_vector_Rect_R(self.as_raw_mut_MSER(), image.as_raw__InputArray(), msers.as_raw_mut_VectorOfVectorOfPoint(), bboxes.as_raw_mut_VectorOfRect()) }.into_result()
	}
	
	fn set_delta(&mut self, delta: i32) -> Result<()> {
		unsafe { sys::cv_MSER_setDelta_int(self.as_raw_mut_MSER(), delta) }.into_result()
	}
	
	fn get_delta(&self) -> Result<i32> {
		unsafe { sys::cv_MSER_getDelta_const(self.as_raw_MSER()) }.into_result()
	}
	
	fn set_min_area(&mut self, min_area: i32) -> Result<()> {
		unsafe { sys::cv_MSER_setMinArea_int(self.as_raw_mut_MSER(), min_area) }.into_result()
	}
	
	fn get_min_area(&self) -> Result<i32> {
		unsafe { sys::cv_MSER_getMinArea_const(self.as_raw_MSER()) }.into_result()
	}
	
	fn set_max_area(&mut self, max_area: i32) -> Result<()> {
		unsafe { sys::cv_MSER_setMaxArea_int(self.as_raw_mut_MSER(), max_area) }.into_result()
	}
	
	fn get_max_area(&self) -> Result<i32> {
		unsafe { sys::cv_MSER_getMaxArea_const(self.as_raw_MSER()) }.into_result()
	}
	
	fn set_pass2_only(&mut self, f: bool) -> Result<()> {
		unsafe { sys::cv_MSER_setPass2Only_bool(self.as_raw_mut_MSER(), f) }.into_result()
	}
	
	fn get_pass2_only(&self) -> Result<bool> {
		unsafe { sys::cv_MSER_getPass2Only_const(self.as_raw_MSER()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_MSER_getDefaultName_const(self.as_raw_MSER()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn MSER + '_ {
	/// Full constructor for %MSER detector
	/// 
	/// ## Parameters
	/// * _delta: it compares ![inline formula](https://latex.codecogs.com/png.latex?%28size%5F%7Bi%7D%2Dsize%5F%7Bi%2Ddelta%7D%29%2Fsize%5F%7Bi%2Ddelta%7D)
	/// * _min_area: prune the area which smaller than minArea
	/// * _max_area: prune the area which bigger than maxArea
	/// * _max_variation: prune the area have similar size to its children
	/// * _min_diversity: for color image, trace back to cut off mser with diversity less than min_diversity
	/// * _max_evolution: for color image, the evolution steps
	/// * _area_threshold: for color image, the area threshold to cause re-initialize
	/// * _min_margin: for color image, ignore too small margin
	/// * _edge_blur_size: for color image, the aperture size for edge blur
	/// 
	/// ## C++ default parameters
	/// * _delta: 5
	/// * _min_area: 60
	/// * _max_area: 14400
	/// * _max_variation: 0.25
	/// * _min_diversity: .2
	/// * _max_evolution: 200
	/// * _area_threshold: 1.01
	/// * _min_margin: 0.003
	/// * _edge_blur_size: 5
	pub fn create(_delta: i32, _min_area: i32, _max_area: i32, _max_variation: f64, _min_diversity: f64, _max_evolution: i32, _area_threshold: f64, _min_margin: f64, _edge_blur_size: i32) -> Result<core::Ptr::<dyn crate::features2d::MSER>> {
		unsafe { sys::cv_MSER_create_int_int_int_double_double_int_double_double_int(_delta, _min_area, _max_area, _max_variation, _min_diversity, _max_evolution, _area_threshold, _min_margin, _edge_blur_size) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::MSER>::opencv_from_extern(r) } )
	}
	
}
/// Class implementing the ORB (*oriented BRIEF*) keypoint detector and descriptor extractor
/// 
/// described in [RRKB11](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_RRKB11) . The algorithm uses FAST in pyramids to detect stable keypoints, selects
/// the strongest features using FAST or Harris response, finds their orientation using first-order
/// moments and computes the descriptors using BRIEF (where the coordinates of random point pairs (or
/// k-tuples) are rotated according to the measured orientation).
pub trait ORB: crate::features2d::Feature2DTrait {
	fn as_raw_ORB(&self) -> *const c_void;
	fn as_raw_mut_ORB(&mut self) -> *mut c_void;

	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		unsafe { sys::cv_ORB_setMaxFeatures_int(self.as_raw_mut_ORB(), max_features) }.into_result()
	}
	
	fn get_max_features(&self) -> Result<i32> {
		unsafe { sys::cv_ORB_getMaxFeatures_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
		unsafe { sys::cv_ORB_setScaleFactor_double(self.as_raw_mut_ORB(), scale_factor) }.into_result()
	}
	
	fn get_scale_factor(&self) -> Result<f64> {
		unsafe { sys::cv_ORB_getScaleFactor_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
		unsafe { sys::cv_ORB_setNLevels_int(self.as_raw_mut_ORB(), nlevels) }.into_result()
	}
	
	fn get_n_levels(&self) -> Result<i32> {
		unsafe { sys::cv_ORB_getNLevels_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
		unsafe { sys::cv_ORB_setEdgeThreshold_int(self.as_raw_mut_ORB(), edge_threshold) }.into_result()
	}
	
	fn get_edge_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_ORB_getEdgeThreshold_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_first_level(&mut self, first_level: i32) -> Result<()> {
		unsafe { sys::cv_ORB_setFirstLevel_int(self.as_raw_mut_ORB(), first_level) }.into_result()
	}
	
	fn get_first_level(&self) -> Result<i32> {
		unsafe { sys::cv_ORB_getFirstLevel_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
		unsafe { sys::cv_ORB_setWTA_K_int(self.as_raw_mut_ORB(), wta_k) }.into_result()
	}
	
	fn get_wta_k(&self) -> Result<i32> {
		unsafe { sys::cv_ORB_getWTA_K_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_score_type(&mut self, score_type: crate::features2d::ORB_ScoreType) -> Result<()> {
		unsafe { sys::cv_ORB_setScoreType_ScoreType(self.as_raw_mut_ORB(), score_type) }.into_result()
	}
	
	fn get_score_type(&self) -> Result<crate::features2d::ORB_ScoreType> {
		unsafe { sys::cv_ORB_getScoreType_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
		unsafe { sys::cv_ORB_setPatchSize_int(self.as_raw_mut_ORB(), patch_size) }.into_result()
	}
	
	fn get_patch_size(&self) -> Result<i32> {
		unsafe { sys::cv_ORB_getPatchSize_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn set_fast_threshold(&mut self, fast_threshold: i32) -> Result<()> {
		unsafe { sys::cv_ORB_setFastThreshold_int(self.as_raw_mut_ORB(), fast_threshold) }.into_result()
	}
	
	fn get_fast_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_ORB_getFastThreshold_const(self.as_raw_ORB()) }.into_result()
	}
	
	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_ORB_getDefaultName_const(self.as_raw_ORB()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn ORB + '_ {
	pub const kBytes: i32 = 32;
	/// The ORB constructor
	/// 
	/// ## Parameters
	/// * nfeatures: The maximum number of features to retain.
	/// * scaleFactor: Pyramid decimation ratio, greater than 1. scaleFactor==2 means the classical
	/// pyramid, where each next level has 4x less pixels than the previous, but such a big scale factor
	/// will degrade feature matching scores dramatically. On the other hand, too close to 1 scale factor
	/// will mean that to cover certain scale range you will need more pyramid levels and so the speed
	/// will suffer.
	/// * nlevels: The number of pyramid levels. The smallest level will have linear size equal to
	/// input_image_linear_size/pow(scaleFactor, nlevels - firstLevel).
	/// * edgeThreshold: This is size of the border where the features are not detected. It should
	/// roughly match the patchSize parameter.
	/// * firstLevel: The level of pyramid to put source image to. Previous layers are filled
	/// with upscaled source image.
	/// * WTA_K: The number of points that produce each element of the oriented BRIEF descriptor. The
	/// default value 2 means the BRIEF where we take a random point pair and compare their brightnesses,
	/// so we get 0/1 response. Other possible values are 3 and 4. For example, 3 means that we take 3
	/// random points (of course, those point coordinates are random, but they are generated from the
	/// pre-defined seed, so each element of BRIEF descriptor is computed deterministically from the pixel
	/// rectangle), find point of maximum brightness and output index of the winner (0, 1 or 2). Such
	/// output will occupy 2 bits, and therefore it will need a special variant of Hamming distance,
	/// denoted as NORM_HAMMING2 (2 bits per bin). When WTA_K=4, we take 4 random points to compute each
	/// bin (that will also occupy 2 bits with possible values 0, 1, 2 or 3).
	/// * scoreType: The default HARRIS_SCORE means that Harris algorithm is used to rank features
	/// (the score is written to KeyPoint::score and is used to retain best nfeatures features);
	/// FAST_SCORE is alternative value of the parameter that produces slightly less stable keypoints,
	/// but it is a little faster to compute.
	/// * patchSize: size of the patch used by the oriented BRIEF descriptor. Of course, on smaller
	/// pyramid layers the perceived image area covered by a feature will be larger.
	/// * fastThreshold: the fast threshold
	/// 
	/// ## C++ default parameters
	/// * nfeatures: 500
	/// * scale_factor: 1.2f
	/// * nlevels: 8
	/// * edge_threshold: 31
	/// * first_level: 0
	/// * wta_k: 2
	/// * score_type: ORB::HARRIS_SCORE
	/// * patch_size: 31
	/// * fast_threshold: 20
	pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: crate::features2d::ORB_ScoreType, patch_size: i32, fast_threshold: i32) -> Result<core::Ptr::<dyn crate::features2d::ORB>> {
		unsafe { sys::cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::features2d::ORB>::opencv_from_extern(r) } )
	}
	
}
/// Class for extracting blobs from an image. :
/// 
/// The class implements a simple algorithm for extracting blobs from an image:
/// 
/// 1.  Convert the source image to binary images by applying thresholding with several thresholds from
///    minThreshold (inclusive) to maxThreshold (exclusive) with distance thresholdStep between
///    neighboring thresholds.
/// 2.  Extract connected components from every binary image by findContours and calculate their
///    centers.
/// 3.  Group centers from several binary images by their coordinates. Close centers form one group that
///    corresponds to one blob, which is controlled by the minDistBetweenBlobs parameter.
/// 4.  From the groups, estimate final centers of blobs and their radiuses and return as locations and
///    sizes of keypoints.
/// 
/// This class performs several filtrations of returned blobs. You should set filterBy\* to true/false
/// to turn on/off corresponding filtration. Available filtrations:
/// 
/// *   **By color**. This filter compares the intensity of a binary image at the center of a blob to
/// blobColor. If they differ, the blob is filtered out. Use blobColor = 0 to extract dark blobs
/// and blobColor = 255 to extract light blobs.
/// *   **By area**. Extracted blobs have an area between minArea (inclusive) and maxArea (exclusive).
/// *   **By circularity**. Extracted blobs have circularity
/// (![inline formula](https://latex.codecogs.com/png.latex?%5Cfrac%7B4%2A%5Cpi%2AArea%7D%7Bperimeter%20%2A%20perimeter%7D)) between minCircularity (inclusive) and
/// maxCircularity (exclusive).
/// *   **By ratio of the minimum inertia to maximum inertia**. Extracted blobs have this ratio
/// between minInertiaRatio (inclusive) and maxInertiaRatio (exclusive).
/// *   **By convexity**. Extracted blobs have convexity (area / area of blob convex hull) between
/// minConvexity (inclusive) and maxConvexity (exclusive).
/// 
/// Default values of parameters are tuned to extract dark circular blobs.
pub trait SimpleBlobDetectorTrait: crate::features2d::Feature2DTrait {
	fn as_raw_SimpleBlobDetector(&self) -> *const c_void;
	fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void;

	fn get_default_name(&self) -> Result<String> {
		unsafe { sys::cv_SimpleBlobDetector_getDefaultName_const(self.as_raw_SimpleBlobDetector()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

/// Class for extracting blobs from an image. :
/// 
/// The class implements a simple algorithm for extracting blobs from an image:
/// 
/// 1.  Convert the source image to binary images by applying thresholding with several thresholds from
///    minThreshold (inclusive) to maxThreshold (exclusive) with distance thresholdStep between
///    neighboring thresholds.
/// 2.  Extract connected components from every binary image by findContours and calculate their
///    centers.
/// 3.  Group centers from several binary images by their coordinates. Close centers form one group that
///    corresponds to one blob, which is controlled by the minDistBetweenBlobs parameter.
/// 4.  From the groups, estimate final centers of blobs and their radiuses and return as locations and
///    sizes of keypoints.
/// 
/// This class performs several filtrations of returned blobs. You should set filterBy\* to true/false
/// to turn on/off corresponding filtration. Available filtrations:
/// 
/// *   **By color**. This filter compares the intensity of a binary image at the center of a blob to
/// blobColor. If they differ, the blob is filtered out. Use blobColor = 0 to extract dark blobs
/// and blobColor = 255 to extract light blobs.
/// *   **By area**. Extracted blobs have an area between minArea (inclusive) and maxArea (exclusive).
/// *   **By circularity**. Extracted blobs have circularity
/// (![inline formula](https://latex.codecogs.com/png.latex?%5Cfrac%7B4%2A%5Cpi%2AArea%7D%7Bperimeter%20%2A%20perimeter%7D)) between minCircularity (inclusive) and
/// maxCircularity (exclusive).
/// *   **By ratio of the minimum inertia to maximum inertia**. Extracted blobs have this ratio
/// between minInertiaRatio (inclusive) and maxInertiaRatio (exclusive).
/// *   **By convexity**. Extracted blobs have convexity (area / area of blob convex hull) between
/// minConvexity (inclusive) and maxConvexity (exclusive).
/// 
/// Default values of parameters are tuned to extract dark circular blobs.
pub struct SimpleBlobDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { SimpleBlobDetector }

impl Drop for SimpleBlobDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_SimpleBlobDetector_delete(instance: *mut c_void); }
		unsafe { cv_SimpleBlobDetector_delete(self.as_raw_mut_SimpleBlobDetector()) };
	}
}

impl SimpleBlobDetector {
	#[inline] pub fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SimpleBlobDetector {}

impl core::AlgorithmTrait for SimpleBlobDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTrait for SimpleBlobDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::SimpleBlobDetectorTrait for SimpleBlobDetector {
	#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SimpleBlobDetector {
	/// ## C++ default parameters
	/// * parameters: SimpleBlobDetector::Params()
	pub fn create(parameters: crate::features2d::SimpleBlobDetector_Params) -> Result<core::Ptr::<crate::features2d::SimpleBlobDetector>> {
		unsafe { sys::cv_SimpleBlobDetector_create_const_ParamsR(&parameters) }.into_result().map(|r| unsafe { core::Ptr::<crate::features2d::SimpleBlobDetector>::opencv_from_extern(r) } )
	}
	
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SimpleBlobDetector_Params {
	pub threshold_step: f32,
	pub min_threshold: f32,
	pub max_threshold: f32,
	pub min_repeatability: size_t,
	pub min_dist_between_blobs: f32,
	pub filter_by_color: bool,
	pub blob_color: u8,
	pub filter_by_area: bool,
	pub min_area: f32,
	pub max_area: f32,
	pub filter_by_circularity: bool,
	pub min_circularity: f32,
	pub max_circularity: f32,
	pub filter_by_inertia: bool,
	pub min_inertia_ratio: f32,
	pub max_inertia_ratio: f32,
	pub filter_by_convexity: bool,
	pub min_convexity: f32,
	pub max_convexity: f32,
}

opencv_type_simple! { crate::features2d::SimpleBlobDetector_Params }

impl SimpleBlobDetector_Params {
	pub fn default() -> Result<crate::features2d::SimpleBlobDetector_Params> {
		unsafe { sys::cv_SimpleBlobDetector_Params_Params() }.into_result()
	}
	
	pub fn read(self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_SimpleBlobDetector_Params_read_const_FileNodeR(self.opencv_as_extern(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	pub fn write(self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_SimpleBlobDetector_Params_write_const_FileStorageR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}
pub use crate::manual::features2d::*;
