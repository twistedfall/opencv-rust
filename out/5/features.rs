//! # Features Framework
//!   # Feature Detection and Description
//!   # Descriptor Matchers
//!
//!   Matchers of keypoint descriptors in OpenCV have wrappers with a common interface that enables
//!   you to easily switch between different algorithms solving the same problem. This section is
//!   devoted to matching descriptors that are represented as vectors in a multidimensional space.
//!   All objects that implement vector descriptor matchers inherit the DescriptorMatcher interface.
//!
//!   # Drawing Function of Keypoints and Matches
//!
//!   # Hardware Acceleration Layer
//!       # Interface
//!
//!   # Approximate Nearest Neighbors Search in Multi-Dimensional Spaces
//!
//!   This section documents OpenCV's interface to the Annoy. Annoy (Approximate Nearest Neighbors Oh Yeah)
//!   is a library to search for points in space that are close to a given query point. It also creates
//!   large read-only file-based data structures that are mmapped into memory so that many processes may
//!   share the same data.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{ANNIndexTrait, ANNIndexTraitConst, AffineFeatureTrait, AffineFeatureTraitConst, BFMatcherTrait, BFMatcherTraitConst, DescriptorMatcherTrait, DescriptorMatcherTraitConst, FastFeatureDetectorTrait, FastFeatureDetectorTraitConst, Feature2DTrait, Feature2DTraitConst, FlannBasedMatcherTrait, FlannBasedMatcherTraitConst, GFTTDetectorTrait, GFTTDetectorTraitConst, KeyPointsFilterTrait, KeyPointsFilterTraitConst, MSERTrait, MSERTraitConst, ORBTrait, ORBTraitConst, SIFTTrait, SIFTTraitConst, SimpleBlobDetectorTrait, SimpleBlobDetectorTraitConst};
}

// DIST_ANGULAR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1205
pub const ANNIndex_DIST_ANGULAR: i32 = 2;
// DIST_DOTPRODUCT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1207
pub const ANNIndex_DIST_DOTPRODUCT: i32 = 4;
// DIST_EUCLIDEAN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1203
pub const ANNIndex_DIST_EUCLIDEAN: i32 = 0;
// DIST_HAMMING /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1206
pub const ANNIndex_DIST_HAMMING: i32 = 3;
// DIST_MANHATTAN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1204
pub const ANNIndex_DIST_MANHATTAN: i32 = 1;
// BRUTEFORCE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:748
pub const DescriptorMatcher_BRUTEFORCE: i32 = 2;
// BRUTEFORCE_HAMMING /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:750
pub const DescriptorMatcher_BRUTEFORCE_HAMMING: i32 = 4;
// BRUTEFORCE_HAMMINGLUT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:751
pub const DescriptorMatcher_BRUTEFORCE_HAMMINGLUT: i32 = 5;
// BRUTEFORCE_L1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:749
pub const DescriptorMatcher_BRUTEFORCE_L1: i32 = 3;
// BRUTEFORCE_SL2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:752
pub const DescriptorMatcher_BRUTEFORCE_SL2: i32 = 6;
// FLANNBASED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:747
pub const DescriptorMatcher_FLANNBASED: i32 = 1;
/// Output image matrix will be created (Mat::create),
/// i.e. existing memory of output image may be reused.
/// Two source image, matches and single keypoints will be drawn.
/// For each keypoint only the center point will be drawn (without
/// the circle around keypoint with keypoint size and orientation).
// DEFAULT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1092
pub const DrawMatchesFlags_DEFAULT: i32 = 0;
/// Output image matrix will not be created (Mat::create).
/// Matches will be drawn on existing content of output image.
// DRAW_OVER_OUTIMG /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1097
pub const DrawMatchesFlags_DRAW_OVER_OUTIMG: i32 = 1;
/// For each keypoint the circle around keypoint with keypoint size and
/// orientation will be drawn.
// DRAW_RICH_KEYPOINTS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1100
pub const DrawMatchesFlags_DRAW_RICH_KEYPOINTS: i32 = 4;
/// Single keypoints will not be drawn.
// NOT_DRAW_SINGLE_POINTS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1099
pub const DrawMatchesFlags_NOT_DRAW_SINGLE_POINTS: i32 = 2;
// FAST_N /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:520
pub const FastFeatureDetector_FAST_N: i32 = 10002;
// NONMAX_SUPPRESSION /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:520
pub const FastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
// THRESHOLD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:520
pub const FastFeatureDetector_THRESHOLD: i32 = 10000;
// TYPE_5_8 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:516
pub const FastFeatureDetector_TYPE_5_8: i32 = 0;
// TYPE_7_12 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:516
pub const FastFeatureDetector_TYPE_7_12: i32 = 1;
// TYPE_9_16 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:516
pub const FastFeatureDetector_TYPE_9_16: i32 = 2;
// FAST_SCORE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:358
pub const ORB_FAST_SCORE: i32 = 1;
// HARRIS_SCORE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:358
pub const ORB_HARRIS_SCORE: i32 = 0;
/// Metrics used to calculate the distance between two feature vectors.
// Distance /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1201
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ANNIndex_Distance {
	DIST_EUCLIDEAN = 0,
	DIST_MANHATTAN = 1,
	DIST_ANGULAR = 2,
	DIST_HAMMING = 3,
	DIST_DOTPRODUCT = 4,
}

impl TryFrom<i32> for ANNIndex_Distance {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DIST_EUCLIDEAN),
			1 => Ok(Self::DIST_MANHATTAN),
			2 => Ok(Self::DIST_ANGULAR),
			3 => Ok(Self::DIST_HAMMING),
			4 => Ok(Self::DIST_DOTPRODUCT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features::ANNIndex_Distance"))),
		}
	}
}

opencv_type_enum! { crate::features::ANNIndex_Distance }

// MatcherType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:745
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DescriptorMatcher_MatcherType {
	FLANNBASED = 1,
	BRUTEFORCE = 2,
	BRUTEFORCE_L1 = 3,
	BRUTEFORCE_HAMMING = 4,
	BRUTEFORCE_HAMMINGLUT = 5,
	BRUTEFORCE_SL2 = 6,
}

impl TryFrom<i32> for DescriptorMatcher_MatcherType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::FLANNBASED),
			2 => Ok(Self::BRUTEFORCE),
			3 => Ok(Self::BRUTEFORCE_L1),
			4 => Ok(Self::BRUTEFORCE_HAMMING),
			5 => Ok(Self::BRUTEFORCE_HAMMINGLUT),
			6 => Ok(Self::BRUTEFORCE_SL2),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features::DescriptorMatcher_MatcherType"))),
		}
	}
}

opencv_type_enum! { crate::features::DescriptorMatcher_MatcherType }

// DrawMatchesFlags /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1090
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl TryFrom<i32> for DrawMatchesFlags {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DEFAULT),
			1 => Ok(Self::DRAW_OVER_OUTIMG),
			2 => Ok(Self::NOT_DRAW_SINGLE_POINTS),
			4 => Ok(Self::DRAW_RICH_KEYPOINTS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features::DrawMatchesFlags"))),
		}
	}
}

opencv_type_enum! { crate::features::DrawMatchesFlags }

// DetectorType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:514
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FastFeatureDetector_DetectorType {
	TYPE_5_8 = 0,
	TYPE_7_12 = 1,
	TYPE_9_16 = 2,
}

impl TryFrom<i32> for FastFeatureDetector_DetectorType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::TYPE_5_8),
			1 => Ok(Self::TYPE_7_12),
			2 => Ok(Self::TYPE_9_16),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features::FastFeatureDetector_DetectorType"))),
		}
	}
}

opencv_type_enum! { crate::features::FastFeatureDetector_DetectorType }

// ScoreType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:358
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ORB_ScoreType {
	HARRIS_SCORE = 0,
	FAST_SCORE = 1,
}

impl TryFrom<i32> for ORB_ScoreType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::HARRIS_SCORE),
			1 => Ok(Self::FAST_SCORE),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features::ORB_ScoreType"))),
		}
	}
}

opencv_type_enum! { crate::features::ORB_ScoreType }

// AffineDescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:257
pub type AffineDescriptorExtractor = crate::features::AffineFeature;
// AffineFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:256
pub type AffineFeatureDetector = crate::features::AffineFeature;
/// Extractors of keypoint descriptors in OpenCV have wrappers with a common interface that enables you
/// to easily switch between different algorithms solving the same problem. This section is devoted to
/// computing descriptors represented as vectors in a multidimensional space. All objects that implement
/// the vector descriptor extractors inherit the DescriptorExtractor interface.
// DescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:232
pub type DescriptorExtractor = crate::features::Feature2D;
/// Feature detectors in OpenCV have wrappers with a common interface that enables you to easily switch
/// between different algorithms solving the same problem. All objects that implement keypoint detectors
/// inherit the FeatureDetector interface.
// FeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:225
pub type FeatureDetector = crate::features::Feature2D;
// SiftDescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:345
pub type SiftDescriptorExtractor = crate::features::SIFT;
// SiftFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:344
pub type SiftFeatureDetector = crate::features::SIFT;
/// Detects corners using the FAST algorithm
///
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints detected on the image.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected keypoints (corners).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
///
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Rosten06) .
///
/// Check [tutorial_py_fast] "the corresponding tutorial" for more details.
///
/// ## Note
/// This alternative version of [fast_with_type] function uses the following default values for its arguments:
/// * nonmax_suppression: true
/// * typ: FastFeatureDetector::TYPE_9_16
// cv::FAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:554
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
#[inline]
pub fn fast_with_type_def(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_FAST_const__InputArrayR_vectorLKeyPointGR_int(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Detects corners using the FAST algorithm
///
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints detected on the image.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected keypoints (corners).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
///
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Rosten06) .
///
/// Check [tutorial_py_fast] "the corresponding tutorial" for more details.
///
/// ## C++ default parameters
/// * nonmax_suppression: true
/// * typ: FastFeatureDetector::TYPE_9_16
// FAST(InputArray, std::vector<KeyPoint> &, int, bool, FastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:554
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
#[inline]
pub fn fast_with_type(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features::FastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// computeRecallPrecisionCurve(const std::vector<std::vector<DMatch>> &, const std::vector<std::vector<uchar>> &, std::vector<Point2f> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1179
// ("cv::computeRecallPrecisionCurve", vec![(pred!(mut, ["matches1to2", "correctMatches1to2Mask", "recallPrecisionCurve"], ["const std::vector<std::vector<cv::DMatch>>*", "const std::vector<std::vector<unsigned char>>*", "std::vector<cv::Point2f>*"]), _)]),
#[inline]
pub fn compute_recall_precision_curve(matches1to2: &core::Vector<core::Vector<core::DMatch>>, correct_matches1to2_mask: &core::Vector<core::Vector<u8>>, recall_precision_curve: &mut core::Vector<core::Point2f>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_computeRecallPrecisionCurve_const_vectorLvectorLDMatchGGR_const_vectorLvectorLunsigned_charGGR_vectorLPoint2fGR(matches1to2.as_raw_VectorOfVectorOfDMatch(), correct_matches1to2_mask.as_raw_VectorOfVectorOfu8(), recall_precision_curve.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_keypoints] function uses the following default values for its arguments:
/// * color: Scalar::all(-1)
/// * flags: DrawMatchesFlags::DEFAULT
// cv::drawKeypoints(InputArray, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1120
// ("cv::drawKeypoints", vec![(pred!(mut, ["image", "keypoints", "outImage"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn draw_keypoints_def(image: &impl ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(image);
	input_output_array_arg!(out_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR(image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), out_image.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
// drawKeypoints(InputArray, const std::vector<KeyPoint> &, InputOutputArray, const Scalar &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputOutputArray, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1120
// ("cv::drawKeypoints", vec![(pred!(mut, ["image", "keypoints", "outImage", "color", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_keypoints(image: &impl ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut impl ToInputOutputArray, color: core::Scalar, flags: crate::features::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(image);
	input_output_array_arg!(out_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), out_image.as_raw__InputOutputArray(), &color, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// ## Note
/// This alternative version of [draw_matches] function uses the following default values for its arguments:
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawMatchesFlags::DEFAULT
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1145
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn draw_matches_def(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1145
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_matches(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [draw_matches_with_thickness] function uses the following default values for its arguments:
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawMatchesFlags::DEFAULT
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1152
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchesThickness"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const int"]), _)]),
#[inline]
pub fn draw_matches_with_thickness_def(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, matches_thickness: i32) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), matches_thickness, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// * matches_mask: std::vector<char>()
/// * flags: DrawMatchesFlags::DEFAULT
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const int, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1152
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchesThickness", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const int", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_matches_with_thickness(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, matches_thickness: i32, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), matches_thickness, &match_color, &single_point_color, matches_mask.as_raw_VectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [draw_matches_knn] function uses the following default values for its arguments:
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<std::vector<char>>()
/// * flags: DrawMatchesFlags::DEFAULT
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1159
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<std::vector<cv::DMatch>>*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn draw_matches_knn_def(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * match_color: Scalar::all(-1)
/// * single_point_color: Scalar::all(-1)
/// * matches_mask: std::vector<std::vector<char>>()
/// * flags: DrawMatchesFlags::DEFAULT
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<std::vector<DMatch>> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<std::vector<char>> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1159
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<std::vector<cv::DMatch>>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<std::vector<char>>*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_matches_knn(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut impl ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<core::Vector<c_char>>, flags: crate::features::DrawMatchesFlags) -> Result<()> {
	input_array_arg!(img1);
	input_array_arg!(img2);
	input_output_array_arg!(out_img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLvectorLcharGGR_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfVectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [evaluate_feature_detector] function uses the following default values for its arguments:
/// * fdetector: Ptr<FeatureDetector>()
// cv::evaluateFeatureDetector(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1174
// ("cv::evaluateFeatureDetector", vec![(pred!(mut, ["img1", "img2", "H1to2", "keypoints1", "keypoints2", "repeatability", "correspCount"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "std::vector<cv::KeyPoint>*", "std::vector<cv::KeyPoint>*", "float*", "int*"]), _)]),
#[inline]
pub fn evaluate_feature_detector_def(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, h1to2: &impl core::MatTraitConst, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * fdetector: Ptr<FeatureDetector>()
// evaluateFeatureDetector(const Mat &, const Mat &, const Mat &, std::vector<KeyPoint> *, std::vector<KeyPoint> *, float &, int &, const Ptr<FeatureDetector> &)(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1174
// ("cv::evaluateFeatureDetector", vec![(pred!(mut, ["img1", "img2", "H1to2", "keypoints1", "keypoints2", "repeatability", "correspCount", "fdetector"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "std::vector<cv::KeyPoint>*", "std::vector<cv::KeyPoint>*", "float*", "int*", "const cv::Ptr<cv::Feature2D>*"]), _)]),
#[inline]
pub fn evaluate_feature_detector(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, h1to2: &impl core::MatTraitConst, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32, fdetector: &core::Ptr<crate::features::Feature2D>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR_const_PtrLFeature2DGR(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, fdetector.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getNearestPoint(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1184
// ("cv::getNearestPoint", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
#[inline]
pub fn get_nearest_point(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getNearestPoint_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getRecall(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1183
// ("cv::getRecall", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
#[inline]
pub fn get_recall(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<f32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getRecall_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::features::ANNIndex]
// ANNIndex /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1196
pub trait ANNIndexTraitConst {
	fn as_raw_ANNIndex(&self) -> *const c_void;

}

/// Mutable methods for [crate::features::ANNIndex]
pub trait ANNIndexTrait: crate::features::ANNIndexTraitConst {
	fn as_raw_mut_ANNIndex(&mut self) -> *mut c_void;

	/// Add feature vectors to index.
	///
	/// ## Parameters
	/// * features: Matrix containing the feature vectors to index. The size of the matrix is
	///      num_features x feature_dimension.
	// addItems(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1217
	// ("cv::ANNIndex::addItems", vec![(pred!(mut, ["features"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn add_items(&mut self, features: &impl ToInputArray) -> Result<()> {
		input_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_addItems_const__InputArrayR(self.as_raw_mut_ANNIndex(), features.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Build the index.
	///
	/// ## Parameters
	/// * trees: Number of trees in the index. If not provided, the number is determined automatically
	/// in a way that at most 2x as much memory as the features vectors take is used.
	///
	/// ## C++ default parameters
	/// * trees: -1
	// build(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1224
	// ("cv::ANNIndex::build", vec![(pred!(mut, ["trees"], ["int"]), _)]),
	#[inline]
	fn build(&mut self, trees: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_build_int(self.as_raw_mut_ANNIndex(), trees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Build the index.
	///
	/// ## Parameters
	/// * trees: Number of trees in the index. If not provided, the number is determined automatically
	/// in a way that at most 2x as much memory as the features vectors take is used.
	///
	/// ## Note
	/// This alternative version of [ANNIndexTrait::build] function uses the following default values for its arguments:
	/// * trees: -1
	// cv::ANNIndex::build() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1224
	// ("cv::ANNIndex::build", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn build_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_build(self.as_raw_mut_ANNIndex(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs a K-nearest neighbor search for given query vector(s) using the index.
	///
	/// ## Parameters
	/// * query: The query vector(s).
	/// * indices: Matrix that will contain the indices of the K-nearest neighbors found, optional.
	/// * dists: Matrix that will contain the distances to the K-nearest neighbors found, optional.
	/// * knn: Number of nearest neighbors to search for.
	/// * search_k: The maximum number of nodes to inspect, which defaults to trees x knn if not provided.
	///
	/// ## C++ default parameters
	/// * search_k: -1
	// knnSearch(InputArray, OutputArray, OutputArray, int, int)(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1234
	// ("cv::ANNIndex::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn", "search_k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	#[inline]
	fn knn_search(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, knn: i32, search_k: i32) -> Result<()> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(self.as_raw_mut_ANNIndex(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, search_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs a K-nearest neighbor search for given query vector(s) using the index.
	///
	/// ## Parameters
	/// * query: The query vector(s).
	/// * indices: Matrix that will contain the indices of the K-nearest neighbors found, optional.
	/// * dists: Matrix that will contain the distances to the K-nearest neighbors found, optional.
	/// * knn: Number of nearest neighbors to search for.
	/// * search_k: The maximum number of nodes to inspect, which defaults to trees x knn if not provided.
	///
	/// ## Note
	/// This alternative version of [ANNIndexTrait::knn_search] function uses the following default values for its arguments:
	/// * search_k: -1
	// cv::ANNIndex::knnSearch(InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1234
	// ("cv::ANNIndex::knnSearch", vec![(pred!(mut, ["query", "indices", "dists", "knn"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn knn_search_def(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, knn: i32) -> Result<()> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(self.as_raw_mut_ANNIndex(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Save the index to disk and loads it. After saving, no more vectors can be added.
	///
	/// ## Parameters
	/// * filename: Filename of the index to be saved.
	/// * prefault: If prefault is set to true, it will pre-read the entire file into memory (using mmap
	/// with MAP_POPULATE). Default is false.
	///
	/// ## C++ default parameters
	/// * prefault: false
	// save(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1242
	// ("cv::ANNIndex::save", vec![(pred!(mut, ["filename", "prefault"], ["const cv::String*", "bool"]), _)]),
	#[inline]
	fn save(&mut self, filename: &str, prefault: bool) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_save_const_StringR_bool(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), prefault, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Save the index to disk and loads it. After saving, no more vectors can be added.
	///
	/// ## Parameters
	/// * filename: Filename of the index to be saved.
	/// * prefault: If prefault is set to true, it will pre-read the entire file into memory (using mmap
	/// with MAP_POPULATE). Default is false.
	///
	/// ## Note
	/// This alternative version of [ANNIndexTrait::save] function uses the following default values for its arguments:
	/// * prefault: false
	// cv::ANNIndex::save(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1242
	// ("cv::ANNIndex::save", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	fn save_def(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_save_const_StringR(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads (mmaps) an index from disk.
	///
	/// ## Parameters
	/// * filename: Filename of the index to be loaded.
	/// * prefault: If prefault is set to true, it will pre-read the entire file into memory (using mmap
	/// with MAP_POPULATE). Default is false.
	///
	/// ## C++ default parameters
	/// * prefault: false
	// load(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1250
	// ("cv::ANNIndex::load", vec![(pred!(mut, ["filename", "prefault"], ["const cv::String*", "bool"]), _)]),
	#[inline]
	fn load(&mut self, filename: &str, prefault: bool) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_load_const_StringR_bool(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), prefault, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads (mmaps) an index from disk.
	///
	/// ## Parameters
	/// * filename: Filename of the index to be loaded.
	/// * prefault: If prefault is set to true, it will pre-read the entire file into memory (using mmap
	/// with MAP_POPULATE). Default is false.
	///
	/// ## Note
	/// This alternative version of [ANNIndexTrait::load] function uses the following default values for its arguments:
	/// * prefault: false
	// cv::ANNIndex::load(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1250
	// ("cv::ANNIndex::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	fn load_def(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_load_const_StringR(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Return the number of trees in the index.
	// getTreeNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1254
	// ("cv::ANNIndex::getTreeNumber", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_tree_number(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_getTreeNumber(self.as_raw_mut_ANNIndex(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Return the number of feature vectors in the index.
	// getItemNumber()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1258
	// ("cv::ANNIndex::getItemNumber", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_item_number(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_getItemNumber(self.as_raw_mut_ANNIndex(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Prepare to build the index in the specified file instead of RAM (execute before adding
	/// items, no need to save after build)
	///
	/// ## Parameters
	/// * filename: Filename of the index to be built.
	// setOnDiskBuild(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1265
	// ("cv::ANNIndex::setOnDiskBuild", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	#[inline]
	fn set_on_disk_build(&mut self, filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_setOnDiskBuild_const_StringR(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Initialize the random number generator with the given seed. Only necessary to pass this
	/// before adding the items. Will have no effect after calling build() or load().
	///
	/// ## Parameters
	/// * seed: The given seed of the random number generator. Its value should be within the range of uint32_t.
	// setSeed(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1272
	// ("cv::ANNIndex::setSeed", vec![(pred!(mut, ["seed"], ["int"]), _)]),
	#[inline]
	fn set_seed(&mut self, seed: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_setSeed_int(self.as_raw_mut_ANNIndex(), seed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// ANNIndex /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1196
pub struct ANNIndex {
	ptr: *mut c_void,
}

opencv_type_boxed! { ANNIndex }

impl Drop for ANNIndex {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_ANNIndex_delete(self.as_raw_mut_ANNIndex()) };
	}
}

unsafe impl Send for ANNIndex {}

impl crate::features::ANNIndexTraitConst for ANNIndex {
	#[inline] fn as_raw_ANNIndex(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::ANNIndexTrait for ANNIndex {
	#[inline] fn as_raw_mut_ANNIndex(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ANNIndex, crate::features::ANNIndexTraitConst, as_raw_ANNIndex, crate::features::ANNIndexTrait, as_raw_mut_ANNIndex }

impl ANNIndex {
	/// Creates an instance of annoy index class with given parameters
	///
	/// ## Parameters
	/// * dim: The dimension of the feature vector.
	/// * distType: Metric to calculate the distance between two feature vectors, can be DIST_EUCLIDEAN,
	///    DIST_MANHATTAN, DIST_ANGULAR, DIST_HAMMING, or DIST_DOTPRODUCT.
	///
	/// ## C++ default parameters
	/// * dist_type: ANNIndex::DIST_EUCLIDEAN
	// create(int, ANNIndex::Distance)(Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1280
	// ("cv::ANNIndex::create", vec![(pred!(mut, ["dim", "distType"], ["int", "cv::ANNIndex::Distance"]), _)]),
	#[inline]
	pub fn create(dim: i32, dist_type: crate::features::ANNIndex_Distance) -> Result<core::Ptr<crate::features::ANNIndex>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_create_int_Distance(dim, dist_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::ANNIndex>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates an instance of annoy index class with given parameters
	///
	/// ## Parameters
	/// * dim: The dimension of the feature vector.
	/// * distType: Metric to calculate the distance between two feature vectors, can be DIST_EUCLIDEAN,
	///    DIST_MANHATTAN, DIST_ANGULAR, DIST_HAMMING, or DIST_DOTPRODUCT.
	///
	/// ## Note
	/// This alternative version of [ANNIndex::create] function uses the following default values for its arguments:
	/// * dist_type: ANNIndex::DIST_EUCLIDEAN
	// cv::ANNIndex::create(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1280
	// ("cv::ANNIndex::create", vec![(pred!(mut, ["dim"], ["int"]), _)]),
	#[inline]
	pub fn create_def(dim: i32) -> Result<core::Ptr<crate::features::ANNIndex>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ANNIndex_create_int(dim, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::ANNIndex>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for ANNIndex {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ANNIndex")
			.finish()
	}
}

/// Constant methods for [crate::features::AffineFeature]
// AffineFeature /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:238
pub trait AffineFeatureTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_AffineFeature(&self) -> *const c_void;

	// getViewParams(std::vector<float> &, std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:252
	// ("cv::AffineFeature::getViewParams", vec![(pred!(const, ["tilts", "rolls"], ["std::vector<float>*", "std::vector<float>*"]), _)]),
	#[inline]
	fn get_view_params(&self, tilts: &mut core::Vector<f32>, rolls: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_getViewParams_const_vectorLfloatGR_vectorLfloatGR(self.as_raw_AffineFeature(), tilts.as_raw_mut_VectorOff32(), rolls.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:253
	// ("cv::AffineFeature::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_getDefaultName_const(self.as_raw_AffineFeature(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::AffineFeature]
pub trait AffineFeatureTrait: crate::features::AffineFeatureTraitConst + crate::features::Feature2DTrait {
	fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void;

	// setViewParams(const std::vector<float> &, const std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:251
	// ("cv::AffineFeature::setViewParams", vec![(pred!(mut, ["tilts", "rolls"], ["const std::vector<float>*", "const std::vector<float>*"]), _)]),
	#[inline]
	fn set_view_params(&mut self, tilts: &core::Vector<f32>, rolls: &core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_setViewParams_const_vectorLfloatGR_const_vectorLfloatGR(self.as_raw_mut_AffineFeature(), tilts.as_raw_VectorOff32(), rolls.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for implementing the wrapper which makes detectors and extractors to be affine invariant,
/// described as ASIFT in [YM11](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_YM11) .
// AffineFeature /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:238
pub struct AffineFeature {
	ptr: *mut c_void,
}

opencv_type_boxed! { AffineFeature }

impl Drop for AffineFeature {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_AffineFeature_delete(self.as_raw_mut_AffineFeature()) };
	}
}

unsafe impl Send for AffineFeature {}

impl core::AlgorithmTraitConst for AffineFeature {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AffineFeature {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for AffineFeature {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for AffineFeature {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features::AffineFeatureTraitConst for AffineFeature {
	#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::AffineFeatureTrait for AffineFeature {
	#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature, crate::features::AffineFeatureTraitConst, as_raw_AffineFeature, crate::features::AffineFeatureTrait, as_raw_mut_AffineFeature }

impl AffineFeature {
	/// ## Parameters
	/// * backend: The detector/extractor you want to use as backend.
	/// * maxTilt: The highest power index of tilt factor. 5 is used in the paper as tilt sampling range n.
	/// * minTilt: The lowest power index of tilt factor. 0 is used in the paper.
	/// * tiltStep: Tilt sampling step ![inline formula](https://latex.codecogs.com/png.latex?%5Cdelta%5Ft) in Algorithm 1 in the paper.
	/// * rotateStepBase: Rotation sampling step factor b in Algorithm 1 in the paper.
	///
	/// ## C++ default parameters
	/// * max_tilt: 5
	/// * min_tilt: 0
	/// * tilt_step: 1.4142135623730951f
	/// * rotate_step_base: 72
	// create(const Ptr<Feature2D> &, int, int, float, float)(CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:248
	// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend", "maxTilt", "minTilt", "tiltStep", "rotateStepBase"], ["const cv::Ptr<cv::Feature2D>*", "int", "int", "float", "float"]), _)]),
	#[inline]
	pub fn create(backend: &core::Ptr<crate::features::Feature2D>, max_tilt: i32, min_tilt: i32, tilt_step: f32, rotate_step_base: f32) -> Result<core::Ptr<crate::features::AffineFeature>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_create_const_PtrLFeature2DGR_int_int_float_float(backend.as_raw_PtrOfFeature2D(), max_tilt, min_tilt, tilt_step, rotate_step_base, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::AffineFeature>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * backend: The detector/extractor you want to use as backend.
	/// * maxTilt: The highest power index of tilt factor. 5 is used in the paper as tilt sampling range n.
	/// * minTilt: The lowest power index of tilt factor. 0 is used in the paper.
	/// * tiltStep: Tilt sampling step ![inline formula](https://latex.codecogs.com/png.latex?%5Cdelta%5Ft) in Algorithm 1 in the paper.
	/// * rotateStepBase: Rotation sampling step factor b in Algorithm 1 in the paper.
	///
	/// ## Note
	/// This alternative version of [AffineFeature::create] function uses the following default values for its arguments:
	/// * max_tilt: 5
	/// * min_tilt: 0
	/// * tilt_step: 1.4142135623730951f
	/// * rotate_step_base: 72
	// cv::AffineFeature::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:248
	// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend"], ["const cv::Ptr<cv::Feature2D>*"]), _)]),
	#[inline]
	pub fn create_def(backend: &core::Ptr<crate::features::Feature2D>) -> Result<core::Ptr<crate::features::AffineFeature>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_create_const_PtrLFeature2DGR(backend.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::AffineFeature>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AffineFeature, core::Algorithm, cv_AffineFeature_to_Algorithm }

boxed_cast_base! { AffineFeature, crate::features::Feature2D, cv_AffineFeature_to_Feature2D }

impl std::fmt::Debug for AffineFeature {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AffineFeature")
			.finish()
	}
}

/// Constant methods for [crate::features::BFMatcher]
// BFMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:994
pub trait BFMatcherTraitConst: crate::features::DescriptorMatcherTraitConst {
	fn as_raw_BFMatcher(&self) -> *const c_void;

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1005
	// ("cv::BFMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_mask_supported(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_isMaskSupported_const(self.as_raw_BFMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * empty_train_data: false
	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1021
	// ("cv::BFMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_clone_const_bool(self.as_raw_BFMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [BFMatcherTraitConst::clone] function uses the following default values for its arguments:
	/// * empty_train_data: false
	// cv::BFMatcher::clone() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1021
	// ("cv::BFMatcher::clone", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn clone_def(&self) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_clone_const(self.as_raw_BFMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::BFMatcher]
pub trait BFMatcherTrait: crate::features::BFMatcherTraitConst + crate::features::DescriptorMatcherTrait {
	fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void;

}

/// Brute-force descriptor matcher.
///
/// For each descriptor in the first set, this matcher finds the closest descriptor in the second set
/// by trying each one. This descriptor matcher supports masking permissible matches of descriptor
/// sets.
// BFMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:994
pub struct BFMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { BFMatcher }

impl Drop for BFMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BFMatcher_delete(self.as_raw_mut_BFMatcher()) };
	}
}

unsafe impl Send for BFMatcher {}

impl core::AlgorithmTraitConst for BFMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BFMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BFMatcher, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::DescriptorMatcherTraitConst for BFMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::DescriptorMatcherTrait for BFMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BFMatcher, crate::features::DescriptorMatcherTraitConst, as_raw_DescriptorMatcher, crate::features::DescriptorMatcherTrait, as_raw_mut_DescriptorMatcher }

impl crate::features::BFMatcherTraitConst for BFMatcher {
	#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::BFMatcherTrait for BFMatcher {
	#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BFMatcher, crate::features::BFMatcherTraitConst, as_raw_BFMatcher, crate::features::BFMatcherTrait, as_raw_mut_BFMatcher }

impl BFMatcher {
	/// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
	///
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	/// * cross_check: false
	// BFMatcher(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1001
	// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
	#[inline]
	pub fn new(norm_type: i32, cross_check: bool) -> Result<crate::features::BFMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_BFMatcher_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features::BFMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * norm_type: NORM_L2
	/// * cross_check: false
	// cv::BFMatcher::BFMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1001
	// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::features::BFMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_BFMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features::BFMatcher::opencv_from_extern(ret) };
		Ok(ret)
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
	// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1019
	// ("cv::BFMatcher::create", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
	#[inline]
	pub fn create(norm_type: i32, cross_check: bool) -> Result<core::Ptr<crate::features::BFMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_create_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::BFMatcher>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [BFMatcher::create] function uses the following default values for its arguments:
	/// * norm_type: NORM_L2
	/// * cross_check: false
	// cv::BFMatcher::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1019
	// ("cv::BFMatcher::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features::BFMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::BFMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BFMatcher, core::Algorithm, cv_BFMatcher_to_Algorithm }

boxed_cast_base! { BFMatcher, crate::features::DescriptorMatcher, cv_BFMatcher_to_DescriptorMatcher }

impl std::fmt::Debug for BFMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BFMatcher")
			.finish()
	}
}

/// Constant methods for [crate::features::DescriptorMatcher]
// DescriptorMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:742
pub trait DescriptorMatcherTraitConst: core::AlgorithmTraitConst {
	fn as_raw_DescriptorMatcher(&self) -> *const c_void;

	/// Returns a constant link to the train descriptor collection trainDescCollection .
	// getTrainDescriptors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:769
	// ("cv::DescriptorMatcher::getTrainDescriptors", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_train_descriptors(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_getTrainDescriptors_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns true if there are no train descriptors in the both collections.
	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:777
	// ("cv::DescriptorMatcher::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_empty_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns true if the descriptor matcher supports masking permissible matches.
	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:781
	// ("cv::DescriptorMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_mask_supported(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_isMaskSupported_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// match(InputArray, InputArray, std::vector<DMatch> &, InputArray)(InputArray, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:808
	// ("cv::DescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn train_match(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [DescriptorMatcherTraitConst::train_match] function uses the following default values for its arguments:
	/// * mask: noArray()
	// cv::DescriptorMatcher::match(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:808
	// ("cv::DescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	#[inline]
	fn train_match_def(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// knnMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, int, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:829
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
	#[inline]
	fn knn_train_match(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, mask: &impl ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [DescriptorMatcherTraitConst::knn_train_match] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * compact_result: false
	// cv::DescriptorMatcher::knnMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:829
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	#[inline]
	fn knn_train_match_def(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// radiusMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, float, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:852
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
	#[inline]
	fn radius_train_match(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, mask: &impl ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [DescriptorMatcherTraitConst::radius_train_match] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * compact_result: false
	// cv::DescriptorMatcher::radiusMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:852
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	#[inline]
	fn radius_train_match_def(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:894
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fileName"], ["const cv::String*"]), _)]),
	#[inline]
	fn write(&self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_write_const_const_StringR(self.as_raw_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:909
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write_to_storage(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_write_const_FileStorageR(self.as_raw_DescriptorMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:917
	// ("cv::DescriptorMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_clone_const_bool(self.as_raw_DescriptorMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Clones the matcher.
	///
	/// ## Parameters
	/// * emptyTrainData: If emptyTrainData is false, the method creates a deep copy of the object,
	/// that is, copies both parameters and train data. If emptyTrainData is true, the method creates an
	/// object copy with the current parameters but with empty train data.
	///
	/// ## Note
	/// This alternative version of [DescriptorMatcherTraitConst::clone] function uses the following default values for its arguments:
	/// * empty_train_data: false
	// cv::DescriptorMatcher::clone() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:917
	// ("cv::DescriptorMatcher::clone", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn clone_def(&self) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_clone_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:936
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	#[inline]
	fn write_to_storage_with_name(&self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_write_const_FileStorageR_const_StringR(self.as_raw_DescriptorMatcher(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::DescriptorMatcher]
pub trait DescriptorMatcherTrait: core::AlgorithmTrait + crate::features::DescriptorMatcherTraitConst {
	fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void;

	/// Adds descriptors to train a CPU(trainDescCollectionis) or GPU(utrainDescCollectionis) descriptor
	/// collection.
	///
	/// If the collection is not empty, the new descriptors are added to existing train descriptors.
	///
	/// ## Parameters
	/// * descriptors: Descriptors to add. Each descriptors[i] is a set of descriptors from the same
	/// train image.
	// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:765
	// ("cv::DescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn add(&mut self, descriptors: &impl ToInputArray) -> Result<()> {
		input_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_add_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Clears the train descriptor collections.
	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:773
	// ("cv::DescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_clear(self.as_raw_mut_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Trains a descriptor matcher
	///
	/// Trains a descriptor matcher (for example, the flann index). In all methods to match, the method
	/// train() is run every time before matching. Some descriptor matchers (for example, BruteForceMatcher)
	/// have an empty implementation of this method. Other matchers really train their inner structures (for
	/// example, FlannBasedMatcher trains flann::Index ).
	// train()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:790
	// ("cv::DescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn train(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_train(self.as_raw_mut_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// descriptor. So, matches size may be smaller than the query descriptors count.
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	/// descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	///
	/// ## C++ default parameters
	/// * masks: noArray()
	// match(InputArray, std::vector<DMatch> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:863
	// ("cv::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn match_(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>, masks: &impl ToInputArray) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * matches: Matches. If a query descriptor is masked out in mask , no match is added for this
	/// descriptor. So, matches size may be smaller than the query descriptors count.
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	/// descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	///
	/// ## Note
	/// This alternative version of [DescriptorMatcherTrait::match_] function uses the following default values for its arguments:
	/// * masks: noArray()
	// cv::DescriptorMatcher::match(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:863
	// ("cv::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	#[inline]
	fn match__def(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
		input_array_arg!(query_descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// less than k possible matches in total.
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	/// descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	///
	/// ## C++ default parameters
	/// * masks: noArray()
	/// * compact_result: false
	// knnMatch(InputArray, std::vector<std::vector<DMatch>> &, int, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:876
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
	#[inline]
	fn knn_match(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, masks: &impl ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * matches: Matches. Each matches[i] is k or less matches for the same query descriptor.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	/// less than k possible matches in total.
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	/// descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	///
	/// ## Note
	/// This alternative version of [DescriptorMatcherTrait::knn_match] function uses the following default values for its arguments:
	/// * masks: noArray()
	/// * compact_result: false
	// cv::DescriptorMatcher::knnMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:876
	// ("cv::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	#[inline]
	fn knn_match_def(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
		input_array_arg!(query_descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	/// descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	///
	/// ## C++ default parameters
	/// * masks: noArray()
	/// * compact_result: false
	// radiusMatch(InputArray, std::vector<std::vector<DMatch>> &, float, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:890
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
	#[inline]
	fn radius_match(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, masks: &impl ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * matches: Found matches.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
	/// descriptors and stored train descriptors from the i-th image trainDescCollection[i].
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	///
	/// ## Note
	/// This alternative version of [DescriptorMatcherTrait::radius_match] function uses the following default values for its arguments:
	/// * masks: noArray()
	/// * compact_result: false
	// cv::DescriptorMatcher::radiusMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:890
	// ("cv::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	#[inline]
	fn radius_match_def(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
		input_array_arg!(query_descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:900
	// ("cv::DescriptorMatcher::read", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
	#[inline]
	fn read(&mut self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_read_const_StringR(self.as_raw_mut_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:907
	// ("cv::DescriptorMatcher::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read_from_node(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_read_const_FileNodeR(self.as_raw_mut_DescriptorMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for matching keypoint descriptors.
///
/// It has two groups of match methods: for matching descriptors of an image with another image or with
/// an image set.
// DescriptorMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:742
pub struct DescriptorMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { DescriptorMatcher }

impl Drop for DescriptorMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_DescriptorMatcher_delete(self.as_raw_mut_DescriptorMatcher()) };
	}
}

unsafe impl Send for DescriptorMatcher {}

impl core::AlgorithmTraitConst for DescriptorMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DescriptorMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DescriptorMatcher, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::DescriptorMatcherTraitConst for DescriptorMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::DescriptorMatcherTrait for DescriptorMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DescriptorMatcher, crate::features::DescriptorMatcherTraitConst, as_raw_DescriptorMatcher, crate::features::DescriptorMatcherTrait, as_raw_mut_DescriptorMatcher }

impl DescriptorMatcher {
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
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:930
	// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["descriptorMatcherType"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn create(descriptor_matcher_type: &str) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		extern_container_arg!(descriptor_matcher_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create(const DescriptorMatcher::MatcherType &)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:932
	// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["matcherType"], ["const cv::DescriptorMatcher::MatcherType*"]), _)]),
	#[inline]
	pub fn create_with_matcher_type(matcher_type: crate::features::DescriptorMatcher_MatcherType) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_create_const_MatcherTypeR(matcher_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { DescriptorMatcher, crate::features::BFMatcher, cv_DescriptorMatcher_to_BFMatcher }

boxed_cast_descendant! { DescriptorMatcher, crate::features::FlannBasedMatcher, cv_DescriptorMatcher_to_FlannBasedMatcher }

boxed_cast_base! { DescriptorMatcher, core::Algorithm, cv_DescriptorMatcher_to_Algorithm }

impl std::fmt::Debug for DescriptorMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DescriptorMatcher")
			.finish()
	}
}

/// Constant methods for [crate::features::FastFeatureDetector]
// FastFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:511
pub trait FastFeatureDetectorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_FastFeatureDetector(&self) -> *const c_void;

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:529
	// ("cv::FastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getThreshold_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:532
	// ("cv::FastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nonmax_suppression(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getNonmaxSuppression_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:535
	// ("cv::FastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_type(&self) -> Result<crate::features::FastFeatureDetector_DetectorType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getType_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:536
	// ("cv::FastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getDefaultName_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::FastFeatureDetector]
pub trait FastFeatureDetectorTrait: crate::features::FastFeatureDetectorTraitConst + crate::features::Feature2DTrait {
	fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void;

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:528
	// ("cv::FastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setThreshold_int(self.as_raw_mut_FastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:531
	// ("cv::FastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	#[inline]
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_FastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setType(FastFeatureDetector::DetectorType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:534
	// ("cv::FastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["cv::FastFeatureDetector::DetectorType"]), _)]),
	#[inline]
	fn set_type(&mut self, typ: crate::features::FastFeatureDetector_DetectorType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setType_DetectorType(self.as_raw_mut_FastFeatureDetector(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Wrapping class for feature detection using the FAST method.
///
/// Check [tutorial_py_fast] "the corresponding tutorial" for more details.
// FastFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:511
pub struct FastFeatureDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { FastFeatureDetector }

impl Drop for FastFeatureDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_FastFeatureDetector_delete(self.as_raw_mut_FastFeatureDetector()) };
	}
}

unsafe impl Send for FastFeatureDetector {}

impl core::AlgorithmTraitConst for FastFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FastFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastFeatureDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for FastFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for FastFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastFeatureDetector, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features::FastFeatureDetectorTraitConst for FastFeatureDetector {
	#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::FastFeatureDetectorTrait for FastFeatureDetector {
	#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastFeatureDetector, crate::features::FastFeatureDetectorTraitConst, as_raw_FastFeatureDetector, crate::features::FastFeatureDetectorTrait, as_raw_mut_FastFeatureDetector }

impl FastFeatureDetector {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: FastFeatureDetector::TYPE_9_16
	// create(int, bool, FastFeatureDetector::DetectorType)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:524
	// ("cv::FastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
	#[inline]
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features::FastFeatureDetector_DetectorType) -> Result<core::Ptr<crate::features::FastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::FastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [FastFeatureDetector::create] function uses the following default values for its arguments:
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: FastFeatureDetector::TYPE_9_16
	// cv::FastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:524
	// ("cv::FastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features::FastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::FastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FastFeatureDetector, core::Algorithm, cv_FastFeatureDetector_to_Algorithm }

boxed_cast_base! { FastFeatureDetector, crate::features::Feature2D, cv_FastFeatureDetector_to_Feature2D }

impl std::fmt::Debug for FastFeatureDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FastFeatureDetector")
			.finish()
	}
}

/// Constant methods for [crate::features::Feature2D]
// Feature2D /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:138
pub trait Feature2DTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Feature2D(&self) -> *const c_void;

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:201
	// ("cv::Feature2D::descriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_descriptorSize_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// descriptorType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:202
	// ("cv::Feature2D::descriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_descriptorType_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:203
	// ("cv::Feature2D::defaultNorm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_defaultNorm_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:205
	// ("cv::Feature2D::write", vec![(pred!(const, ["fileName"], ["const cv::String*"]), _)]),
	#[inline]
	fn write(&self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_write_const_const_StringR(self.as_raw_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:209
	// ("cv::Feature2D::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write_to_storage(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_write_const_FileStorageR(self.as_raw_Feature2D(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Return true if detector object is empty
	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:215
	// ("cv::Feature2D::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_empty_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:216
	// ("cv::Feature2D::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_getDefaultName_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:219
	// ("cv::Feature2D::write", vec![(pred!(const, ["fs", "name"], ["cv::FileStorage*", "const cv::String*"]), _)]),
	#[inline]
	fn write_to_storage_with_name(&self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_write_const_FileStorageR_const_StringR(self.as_raw_Feature2D(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::Feature2D]
pub trait Feature2DTrait: core::AlgorithmTrait + crate::features::Feature2DTraitConst {
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
	// detect(InputArray, std::vector<KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:151
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn detect(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR_const__InputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [Feature2DTrait::detect] function uses the following default values for its arguments:
	/// * mask: noArray()
	// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:151
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	fn detect_def(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// of keypoints detected in images[i] .
	/// * masks: Masks for each input image specifying where to look for keypoints (optional).
	/// masks[i] is a mask for images[i].
	///
	/// ## C++ default parameters
	/// * masks: noArray()
	// detect(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:162
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["images", "keypoints", "masks"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn detect_multiple(&mut self, images: &impl ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, masks: &impl ToInputArray) -> Result<()> {
		input_array_arg!(images);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR_const__InputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * images: Image set.
	/// * keypoints: The detected keypoints. In the second variant of the method keypoints[i] is a set
	/// of keypoints detected in images[i] .
	/// * masks: Masks for each input image specifying where to look for keypoints (optional).
	/// masks[i] is a mask for images[i].
	///
	/// ## Note
	/// This alternative version of [Feature2DTrait::detect_multiple] function uses the following default values for its arguments:
	/// * masks: noArray()
	// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:162
	// ("cv::Feature2D::detect", vec![(pred!(mut, ["images", "keypoints"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*"]), _)]),
	#[inline]
	fn detect_multiple_def(&mut self, images: &impl ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>) -> Result<()> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:177
	// ("cv::Feature2D::compute", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// computed are removed. Sometimes new keypoints can be added, for example: SIFT duplicates keypoint
	/// with several dominant orientations (for each orientation).
	/// * descriptors: Computed descriptors. In the second variant of the method descriptors[i] are
	/// descriptors computed for a keypoints[i]. Row j is the keypoints (or keypoints[i]) is the
	/// descriptor for keypoint j-th keypoint.
	// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:191
	// ("cv::Feature2D::compute", vec![(pred!(mut, ["images", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_multiple(&mut self, images: &impl ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Detects keypoints and computes the descriptors
	///
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// detectAndCompute(InputArray, InputArray, std::vector<KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:196
	// ("cv::Feature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "bool"]), _)]),
	#[inline]
	fn detect_and_compute(&mut self, image: &impl ToInputArray, mask: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_bool(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Detects keypoints and computes the descriptors
	///
	/// ## Note
	/// This alternative version of [Feature2DTrait::detect_and_compute] function uses the following default values for its arguments:
	/// * use_provided_keypoints: false
	// cv::Feature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:196
	// ("cv::Feature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn detect_and_compute_def(&mut self, image: &impl ToInputArray, mask: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:207
	// ("cv::Feature2D::read", vec![(pred!(mut, ["fileName"], ["const cv::String*"]), _)]),
	#[inline]
	fn read(&mut self, file_name: &str) -> Result<()> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_read_const_StringR(self.as_raw_mut_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:212
	// ("cv::Feature2D::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read_from_node(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_read_const_FileNodeR(self.as_raw_mut_Feature2D(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for 2D image feature detectors and descriptor extractors
// Feature2D /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:138
pub struct Feature2D {
	ptr: *mut c_void,
}

opencv_type_boxed! { Feature2D }

impl Drop for Feature2D {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_Feature2D_delete(self.as_raw_mut_Feature2D()) };
	}
}

unsafe impl Send for Feature2D {}

impl core::AlgorithmTraitConst for Feature2D {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Feature2D {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Feature2D, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for Feature2D {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for Feature2D {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Feature2D, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl Feature2D {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_Feature2D_defaultNew_const()) }
	}

}

boxed_cast_descendant! { Feature2D, crate::features::AffineFeature, cv_Feature2D_to_AffineFeature }

boxed_cast_descendant! { Feature2D, crate::features::FastFeatureDetector, cv_Feature2D_to_FastFeatureDetector }

boxed_cast_descendant! { Feature2D, crate::features::GFTTDetector, cv_Feature2D_to_GFTTDetector }

boxed_cast_descendant! { Feature2D, crate::features::MSER, cv_Feature2D_to_MSER }

boxed_cast_descendant! { Feature2D, crate::features::ORB, cv_Feature2D_to_ORB }

boxed_cast_descendant! { Feature2D, crate::features::SIFT, cv_Feature2D_to_SIFT }

boxed_cast_descendant! { Feature2D, crate::features::SimpleBlobDetector, cv_Feature2D_to_SimpleBlobDetector }

boxed_cast_base! { Feature2D, core::Algorithm, cv_Feature2D_to_Algorithm }

impl std::fmt::Debug for Feature2D {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Feature2D")
			.finish()
	}
}

impl Default for Feature2D {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::features::FlannBasedMatcher]
// FlannBasedMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1041
pub trait FlannBasedMatcherTraitConst: crate::features::DescriptorMatcherTraitConst {
	fn as_raw_FlannBasedMatcher(&self) -> *const c_void;

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1053
	// ("cv::FlannBasedMatcher::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_write_const_FileStorageR(self.as_raw_FlannBasedMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1056
	// ("cv::FlannBasedMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_mask_supported(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_isMaskSupported_const(self.as_raw_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * empty_train_data: false
	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1060
	// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clone_const_bool(self.as_raw_FlannBasedMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [FlannBasedMatcherTraitConst::clone] function uses the following default values for its arguments:
	/// * empty_train_data: false
	// cv::FlannBasedMatcher::clone() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1060
	// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn clone_def(&self) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clone_const(self.as_raw_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::FlannBasedMatcher]
pub trait FlannBasedMatcherTrait: crate::features::DescriptorMatcherTrait + crate::features::FlannBasedMatcherTraitConst {
	fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void;

	// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1047
	// ("cv::FlannBasedMatcher::add", vec![(pred!(mut, ["descriptors"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn add(&mut self, descriptors: &impl ToInputArray) -> Result<()> {
		input_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_add_const__InputArrayR(self.as_raw_mut_FlannBasedMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1048
	// ("cv::FlannBasedMatcher::clear", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clear(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1051
	// ("cv::FlannBasedMatcher::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_read_const_FileNodeR(self.as_raw_mut_FlannBasedMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// train()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1055
	// ("cv::FlannBasedMatcher::train", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn train(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_train(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Flann-based descriptor matcher.
///
/// This matcher trains cv::flann::Index on a train descriptor collection and calls its nearest search
/// methods to find the best matches. So, this matcher may be faster when matching a large train
/// collection than the brute force matcher. FlannBasedMatcher does not support masking permissible
/// matches of descriptor sets because flann::Index does not support this. :
// FlannBasedMatcher /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1041
pub struct FlannBasedMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { FlannBasedMatcher }

impl Drop for FlannBasedMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_FlannBasedMatcher_delete(self.as_raw_mut_FlannBasedMatcher()) };
	}
}

unsafe impl Send for FlannBasedMatcher {}

impl core::AlgorithmTraitConst for FlannBasedMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FlannBasedMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlannBasedMatcher, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::DescriptorMatcherTraitConst for FlannBasedMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::DescriptorMatcherTrait for FlannBasedMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlannBasedMatcher, crate::features::DescriptorMatcherTraitConst, as_raw_DescriptorMatcher, crate::features::DescriptorMatcherTrait, as_raw_mut_DescriptorMatcher }

impl crate::features::FlannBasedMatcherTraitConst for FlannBasedMatcher {
	#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::FlannBasedMatcherTrait for FlannBasedMatcher {
	#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlannBasedMatcher, crate::features::FlannBasedMatcherTraitConst, as_raw_FlannBasedMatcher, crate::features::FlannBasedMatcherTrait, as_raw_mut_FlannBasedMatcher }

impl FlannBasedMatcher {
	/// ## C++ default parameters
	/// * index_params: makePtr<flann::KDTreeIndexParams>()
	/// * search_params: makePtr<flann::SearchParams>()
	// FlannBasedMatcher(const Ptr<flann::IndexParams> &, const Ptr<flann::SearchParams> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1044
	// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, ["indexParams", "searchParams"], ["const cv::Ptr<cv::flann::IndexParams>*", "const cv::Ptr<cv::flann::SearchParams>*"]), _)]),
	#[inline]
	pub fn new(index_params: &core::Ptr<crate::flann::IndexParams>, search_params: &core::Ptr<crate::flann::SearchParams>) -> Result<crate::features::FlannBasedMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher_const_PtrLIndexParamsGR_const_PtrLSearchParamsGR(index_params.as_raw_PtrOfIndexParams(), search_params.as_raw_PtrOfSearchParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features::FlannBasedMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * index_params: makePtr<flann::KDTreeIndexParams>()
	/// * search_params: makePtr<flann::SearchParams>()
	// cv::FlannBasedMatcher::FlannBasedMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1044
	// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::features::FlannBasedMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features::FlannBasedMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:1058
	// ("cv::FlannBasedMatcher::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::features::FlannBasedMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::FlannBasedMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FlannBasedMatcher, core::Algorithm, cv_FlannBasedMatcher_to_Algorithm }

boxed_cast_base! { FlannBasedMatcher, crate::features::DescriptorMatcher, cv_FlannBasedMatcher_to_DescriptorMatcher }

impl std::fmt::Debug for FlannBasedMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FlannBasedMatcher")
			.finish()
	}
}

/// Constant methods for [crate::features::GFTTDetector]
// GFTTDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:560
pub trait GFTTDetectorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_GFTTDetector(&self) -> *const c_void;

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:568
	// ("cv::GFTTDetector::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getMaxFeatures_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getQualityLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:571
	// ("cv::GFTTDetector::getQualityLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_quality_level(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getQualityLevel_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:574
	// ("cv::GFTTDetector::getMinDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_distance(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getMinDistance_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:577
	// ("cv::GFTTDetector::getBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getBlockSize_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getHarrisDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:583
	// ("cv::GFTTDetector::getHarrisDetector", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_harris_detector(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getHarrisDetector_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:586
	// ("cv::GFTTDetector::getK", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_k(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getK_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:587
	// ("cv::GFTTDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getDefaultName_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::GFTTDetector]
pub trait GFTTDetectorTrait: crate::features::Feature2DTrait + crate::features::GFTTDetectorTraitConst {
	fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void;

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:567
	// ("cv::GFTTDetector::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	#[inline]
	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setMaxFeatures_int(self.as_raw_mut_GFTTDetector(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setQualityLevel(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:570
	// ("cv::GFTTDetector::setQualityLevel", vec![(pred!(mut, ["qlevel"], ["double"]), _)]),
	#[inline]
	fn set_quality_level(&mut self, qlevel: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setQualityLevel_double(self.as_raw_mut_GFTTDetector(), qlevel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDistance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:573
	// ("cv::GFTTDetector::setMinDistance", vec![(pred!(mut, ["minDistance"], ["double"]), _)]),
	#[inline]
	fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setMinDistance_double(self.as_raw_mut_GFTTDetector(), min_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:576
	// ("cv::GFTTDetector::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	#[inline]
	fn set_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setBlockSize_int(self.as_raw_mut_GFTTDetector(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setGradientSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:579
	// ("cv::GFTTDetector::setGradientSize", vec![(pred!(mut, ["gradientSize_"], ["int"]), _)]),
	#[inline]
	fn set_gradient_size(&mut self, gradient_size_: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setGradientSize_int(self.as_raw_mut_GFTTDetector(), gradient_size_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getGradientSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:580
	// ("cv::GFTTDetector::getGradientSize", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_gradient_size(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getGradientSize(self.as_raw_mut_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setHarrisDetector(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:582
	// ("cv::GFTTDetector::setHarrisDetector", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_harris_detector(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setHarrisDetector_bool(self.as_raw_mut_GFTTDetector(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setK(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:585
	// ("cv::GFTTDetector::setK", vec![(pred!(mut, ["k"], ["double"]), _)]),
	#[inline]
	fn set_k(&mut self, k: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setK_double(self.as_raw_mut_GFTTDetector(), k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Wrapping class for feature detection using the goodFeaturesToTrack function. :
// GFTTDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:560
pub struct GFTTDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { GFTTDetector }

impl Drop for GFTTDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_GFTTDetector_delete(self.as_raw_mut_GFTTDetector()) };
	}
}

unsafe impl Send for GFTTDetector {}

impl core::AlgorithmTraitConst for GFTTDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GFTTDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GFTTDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for GFTTDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for GFTTDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GFTTDetector, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features::GFTTDetectorTraitConst for GFTTDetector {
	#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::GFTTDetectorTrait for GFTTDetector {
	#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GFTTDetector, crate::features::GFTTDetectorTraitConst, as_raw_GFTTDetector, crate::features::GFTTDetectorTrait, as_raw_mut_GFTTDetector }

impl GFTTDetector {
	/// ## C++ default parameters
	/// * max_corners: 1000
	/// * quality_level: 0.01
	/// * min_distance: 1
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	// create(int, double, double, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:563
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "bool", "double"]), _)]),
	#[inline]
	pub fn create(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<crate::features::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners, quality_level, min_distance, block_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [GFTTDetector::create] function uses the following default values for its arguments:
	/// * max_corners: 1000
	/// * quality_level: 0.01
	/// * min_distance: 1
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	// cv::GFTTDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:563
	// ("cv::GFTTDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_harris_detector: false
	/// * k: 0.04
	// create(int, double, double, int, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:565
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "int", "bool", "double"]), _)]),
	#[inline]
	pub fn create_with_gradient(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<crate::features::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners, quality_level, min_distance, block_size, gradiant_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [GFTTDetector::create_with_gradient] function uses the following default values for its arguments:
	/// * use_harris_detector: false
	/// * k: 0.04
	// cv::GFTTDetector::create(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:565
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize"], ["int", "double", "double", "int", "int"]), _)]),
	#[inline]
	pub fn create_with_gradient_def(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32) -> Result<core::Ptr<crate::features::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int(max_corners, quality_level, min_distance, block_size, gradiant_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { GFTTDetector, core::Algorithm, cv_GFTTDetector_to_Algorithm }

boxed_cast_base! { GFTTDetector, crate::features::Feature2D, cv_GFTTDetector_to_Feature2D }

impl std::fmt::Debug for GFTTDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GFTTDetector")
			.finish()
	}
}

/// Constant methods for [crate::features::KeyPointsFilter]
// KeyPointsFilter /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:96
pub trait KeyPointsFilterTraitConst {
	fn as_raw_KeyPointsFilter(&self) -> *const c_void;

}

/// Mutable methods for [crate::features::KeyPointsFilter]
pub trait KeyPointsFilterTrait: crate::features::KeyPointsFilterTraitConst {
	fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void;

}

/// A class filters a vector of keypoints.
///
/// Because now it is difficult to provide a convenient interface for all usage scenarios of the
/// keypoints filter class, it has only several needed by now static methods.
// KeyPointsFilter /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:96
pub struct KeyPointsFilter {
	ptr: *mut c_void,
}

opencv_type_boxed! { KeyPointsFilter }

impl Drop for KeyPointsFilter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_KeyPointsFilter_delete(self.as_raw_mut_KeyPointsFilter()) };
	}
}

unsafe impl Send for KeyPointsFilter {}

impl crate::features::KeyPointsFilterTraitConst for KeyPointsFilter {
	#[inline] fn as_raw_KeyPointsFilter(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::KeyPointsFilterTrait for KeyPointsFilter {
	#[inline] fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KeyPointsFilter, crate::features::KeyPointsFilterTraitConst, as_raw_KeyPointsFilter, crate::features::KeyPointsFilterTrait, as_raw_mut_KeyPointsFilter }

impl KeyPointsFilter {
	// KeyPointsFilter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:99
	// ("cv::KeyPointsFilter::KeyPointsFilter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::features::KeyPointsFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_KeyPointsFilter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features::KeyPointsFilter::opencv_from_extern(ret) };
		Ok(ret)
	}

	// runByImageBorder(std::vector<KeyPoint> &, Size, int)(CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:104
	// ("cv::KeyPointsFilter::runByImageBorder", vec![(pred!(mut, ["keypoints", "imageSize", "borderSize"], ["std::vector<cv::KeyPoint>*", "cv::Size", "int"]), _)]),
	#[inline]
	pub fn run_by_image_border(keypoints: &mut core::Vector<core::KeyPoint>, image_size: core::Size, border_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByImageBorder_vectorLKeyPointGR_Size_int(keypoints.as_raw_mut_VectorOfKeyPoint(), &image_size, border_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * max_size: FLT_MAX
	// runByKeypointSize(std::vector<KeyPoint> &, float, float)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:108
	// ("cv::KeyPointsFilter::runByKeypointSize", vec![(pred!(mut, ["keypoints", "minSize", "maxSize"], ["std::vector<cv::KeyPoint>*", "float", "float"]), _)]),
	#[inline]
	pub fn run_by_keypoint_size(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32, max_size: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, max_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [KeyPointsFilter::run_by_keypoint_size] function uses the following default values for its arguments:
	/// * max_size: FLT_MAX
	// cv::KeyPointsFilter::runByKeypointSize(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:108
	// ("cv::KeyPointsFilter::runByKeypointSize", vec![(pred!(mut, ["keypoints", "minSize"], ["std::vector<cv::KeyPoint>*", "float"]), _)]),
	#[inline]
	pub fn run_by_keypoint_size_def(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// runByPixelsMask(std::vector<KeyPoint> &, const Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:113
	// ("cv::KeyPointsFilter::runByPixelsMask", vec![(pred!(mut, ["keypoints", "mask"], ["std::vector<cv::KeyPoint>*", "const cv::Mat*"]), _)]),
	#[inline]
	pub fn run_by_pixels_mask(keypoints: &mut core::Vector<core::KeyPoint>, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByPixelsMask_vectorLKeyPointGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// runByPixelsMask2VectorPoint(std::vector<KeyPoint> &, std::vector<std::vector<Point>> &, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:117
	// ("cv::KeyPointsFilter::runByPixelsMask2VectorPoint", vec![(pred!(mut, ["keypoints", "removeFrom", "mask"], ["std::vector<cv::KeyPoint>*", "std::vector<std::vector<cv::Point>>*", "const cv::Mat*"]), _)]),
	#[inline]
	pub fn run_by_pixels_mask2_vector_point(keypoints: &mut core::Vector<core::KeyPoint>, remove_from: &mut core::Vector<core::Vector<core::Point>>, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByPixelsMask2VectorPoint_vectorLKeyPointGR_vectorLvectorLPointGGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), remove_from.as_raw_mut_VectorOfVectorOfPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// removeDuplicated(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:121
	// ("cv::KeyPointsFilter::removeDuplicated", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	pub fn remove_duplicated(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_removeDuplicated_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// removeDuplicatedSorted(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:125
	// ("cv::KeyPointsFilter::removeDuplicatedSorted", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	pub fn remove_duplicated_sorted(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_removeDuplicatedSorted_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// retainBest(std::vector<KeyPoint> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:130
	// ("cv::KeyPointsFilter::retainBest", vec![(pred!(mut, ["keypoints", "npoints"], ["std::vector<cv::KeyPoint>*", "int"]), _)]),
	#[inline]
	pub fn retain_best(keypoints: &mut core::Vector<core::KeyPoint>, npoints: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_retainBest_vectorLKeyPointGR_int(keypoints.as_raw_mut_VectorOfKeyPoint(), npoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

impl std::fmt::Debug for KeyPointsFilter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KeyPointsFilter")
			.finish()
	}
}

/// Constant methods for [crate::features::MSER]
// MSER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:443
pub trait MSERTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_MSER(&self) -> *const c_void;

	// getDelta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:474
	// ("cv::MSER::getDelta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_delta(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getDelta_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinArea()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:477
	// ("cv::MSER::getMinArea", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMinArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxArea()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:480
	// ("cv::MSER::getMaxArea", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMaxArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxVariation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:483
	// ("cv::MSER::getMaxVariation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_variation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMaxVariation_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinDiversity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:486
	// ("cv::MSER::getMinDiversity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_diversity(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMinDiversity_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxEvolution()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:489
	// ("cv::MSER::getMaxEvolution", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_evolution(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMaxEvolution_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAreaThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:492
	// ("cv::MSER::getAreaThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_area_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getAreaThreshold_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinMargin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:495
	// ("cv::MSER::getMinMargin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_margin(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMinMargin_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeBlurSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:498
	// ("cv::MSER::getEdgeBlurSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_edge_blur_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getEdgeBlurSize_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPass2Only()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:501
	// ("cv::MSER::getPass2Only", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pass2_only(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getPass2Only_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:503
	// ("cv::MSER::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getDefaultName_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::MSER]
pub trait MSERTrait: crate::features::Feature2DTrait + crate::features::MSERTraitConst {
	fn as_raw_mut_MSER(&mut self) -> *mut c_void;

	/// Detect %MSER regions
	///
	/// ## Parameters
	/// * image: input image (8UC1, 8UC3 or 8UC4, must be greater or equal than 3x3)
	/// * msers: resulting list of point sets
	/// * bboxes: resulting bounding boxes
	// detectRegions(InputArray, std::vector<std::vector<Point>> &, std::vector<Rect> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:469
	// ("cv::MSER::detectRegions", vec![(pred!(mut, ["image", "msers", "bboxes"], ["const cv::_InputArray*", "std::vector<std::vector<cv::Point>>*", "std::vector<cv::Rect>*"]), _)]),
	#[inline]
	fn detect_regions(&mut self, image: &impl ToInputArray, msers: &mut core::Vector<core::Vector<core::Point>>, bboxes: &mut core::Vector<core::Rect>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_detectRegions_const__InputArrayR_vectorLvectorLPointGGR_vectorLRectGR(self.as_raw_mut_MSER(), image.as_raw__InputArray(), msers.as_raw_mut_VectorOfVectorOfPoint(), bboxes.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:473
	// ("cv::MSER::setDelta", vec![(pred!(mut, ["delta"], ["int"]), _)]),
	#[inline]
	fn set_delta(&mut self, delta: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setDelta_int(self.as_raw_mut_MSER(), delta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:476
	// ("cv::MSER::setMinArea", vec![(pred!(mut, ["minArea"], ["int"]), _)]),
	#[inline]
	fn set_min_area(&mut self, min_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMinArea_int(self.as_raw_mut_MSER(), min_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:479
	// ("cv::MSER::setMaxArea", vec![(pred!(mut, ["maxArea"], ["int"]), _)]),
	#[inline]
	fn set_max_area(&mut self, max_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMaxArea_int(self.as_raw_mut_MSER(), max_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxVariation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:482
	// ("cv::MSER::setMaxVariation", vec![(pred!(mut, ["maxVariation"], ["double"]), _)]),
	#[inline]
	fn set_max_variation(&mut self, max_variation: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMaxVariation_double(self.as_raw_mut_MSER(), max_variation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDiversity(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:485
	// ("cv::MSER::setMinDiversity", vec![(pred!(mut, ["minDiversity"], ["double"]), _)]),
	#[inline]
	fn set_min_diversity(&mut self, min_diversity: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMinDiversity_double(self.as_raw_mut_MSER(), min_diversity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxEvolution(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:488
	// ("cv::MSER::setMaxEvolution", vec![(pred!(mut, ["maxEvolution"], ["int"]), _)]),
	#[inline]
	fn set_max_evolution(&mut self, max_evolution: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMaxEvolution_int(self.as_raw_mut_MSER(), max_evolution, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setAreaThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:491
	// ("cv::MSER::setAreaThreshold", vec![(pred!(mut, ["areaThreshold"], ["double"]), _)]),
	#[inline]
	fn set_area_threshold(&mut self, area_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setAreaThreshold_double(self.as_raw_mut_MSER(), area_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinMargin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:494
	// ("cv::MSER::setMinMargin", vec![(pred!(mut, ["min_margin"], ["double"]), _)]),
	#[inline]
	fn set_min_margin(&mut self, min_margin: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMinMargin_double(self.as_raw_mut_MSER(), min_margin, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setEdgeBlurSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:497
	// ("cv::MSER::setEdgeBlurSize", vec![(pred!(mut, ["edge_blur_size"], ["int"]), _)]),
	#[inline]
	fn set_edge_blur_size(&mut self, edge_blur_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setEdgeBlurSize_int(self.as_raw_mut_MSER(), edge_blur_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPass2Only(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:500
	// ("cv::MSER::setPass2Only", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	#[inline]
	fn set_pass2_only(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setPass2Only_bool(self.as_raw_mut_MSER(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Maximally stable extremal region extractor
///
/// The class encapsulates all the parameters of the %MSER extraction algorithm (see [wiki
/// article](http://en.wikipedia.org/wiki/Maximally_stable_extremal_regions)).
///
/// - there are two different implementation of %MSER: one for grey image, one for color image
///
/// - the grey image algorithm is taken from: [nister2008linear](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_nister2008linear) ;  the paper claims to be faster
/// than union-find method; it actually get 1.5~2m/s on my centrino L7200 1.2GHz laptop.
///
/// - the color image algorithm is taken from: [forssen2007maximally](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_forssen2007maximally) ; it should be much slower
/// than grey image method ( 3~4 times )
///
/// - (Python) A complete example showing the use of the %MSER detector can be found at samples/python/mser.py
// MSER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:443
pub struct MSER {
	ptr: *mut c_void,
}

opencv_type_boxed! { MSER }

impl Drop for MSER {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_MSER_delete(self.as_raw_mut_MSER()) };
	}
}

unsafe impl Send for MSER {}

impl core::AlgorithmTraitConst for MSER {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MSER {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSER, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for MSER {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for MSER {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSER, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features::MSERTraitConst for MSER {
	#[inline] fn as_raw_MSER(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::MSERTrait for MSER {
	#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSER, crate::features::MSERTraitConst, as_raw_MSER, crate::features::MSERTrait, as_raw_mut_MSER }

impl MSER {
	/// Full constructor for %MSER detector
	///
	/// ## Parameters
	/// * delta: it compares ![inline formula](https://latex.codecogs.com/png.latex?%28size%5F%7Bi%7D%2Dsize%5F%7Bi%2Ddelta%7D%29%2Fsize%5F%7Bi%2Ddelta%7D)
	/// * min_area: prune the area which smaller than minArea
	/// * max_area: prune the area which bigger than maxArea
	/// * max_variation: prune the area have similar size to its children
	/// * min_diversity: for color image, trace back to cut off mser with diversity less than min_diversity
	/// * max_evolution: for color image, the evolution steps
	/// * area_threshold: for color image, the area threshold to cause re-initialize
	/// * min_margin: for color image, ignore too small margin
	/// * edge_blur_size: for color image, the aperture size for edge blur
	///
	/// ## C++ default parameters
	/// * delta: 5
	/// * min_area: 60
	/// * max_area: 14400
	/// * max_variation: 0.25
	/// * min_diversity: .2
	/// * max_evolution: 200
	/// * area_threshold: 1.01
	/// * min_margin: 0.003
	/// * edge_blur_size: 5
	// create(int, int, int, double, double, int, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:458
	// ("cv::MSER::create", vec![(pred!(mut, ["delta", "min_area", "max_area", "max_variation", "min_diversity", "max_evolution", "area_threshold", "min_margin", "edge_blur_size"], ["int", "int", "int", "double", "double", "int", "double", "double", "int"]), _)]),
	#[inline]
	pub fn create(delta: i32, min_area: i32, max_area: i32, max_variation: f64, min_diversity: f64, max_evolution: i32, area_threshold: f64, min_margin: f64, edge_blur_size: i32) -> Result<core::Ptr<crate::features::MSER>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_create_int_int_int_double_double_int_double_double_int(delta, min_area, max_area, max_variation, min_diversity, max_evolution, area_threshold, min_margin, edge_blur_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::MSER>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Full constructor for %MSER detector
	///
	/// ## Parameters
	/// * delta: it compares ![inline formula](https://latex.codecogs.com/png.latex?%28size%5F%7Bi%7D%2Dsize%5F%7Bi%2Ddelta%7D%29%2Fsize%5F%7Bi%2Ddelta%7D)
	/// * min_area: prune the area which smaller than minArea
	/// * max_area: prune the area which bigger than maxArea
	/// * max_variation: prune the area have similar size to its children
	/// * min_diversity: for color image, trace back to cut off mser with diversity less than min_diversity
	/// * max_evolution: for color image, the evolution steps
	/// * area_threshold: for color image, the area threshold to cause re-initialize
	/// * min_margin: for color image, ignore too small margin
	/// * edge_blur_size: for color image, the aperture size for edge blur
	///
	/// ## Note
	/// This alternative version of [MSER::create] function uses the following default values for its arguments:
	/// * delta: 5
	/// * min_area: 60
	/// * max_area: 14400
	/// * max_variation: 0.25
	/// * min_diversity: .2
	/// * max_evolution: 200
	/// * area_threshold: 1.01
	/// * min_margin: 0.003
	/// * edge_blur_size: 5
	// cv::MSER::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:458
	// ("cv::MSER::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features::MSER>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::MSER>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MSER, core::Algorithm, cv_MSER_to_Algorithm }

boxed_cast_base! { MSER, crate::features::Feature2D, cv_MSER_to_Feature2D }

impl std::fmt::Debug for MSER {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MSER")
			.finish()
	}
}

/// Constant methods for [crate::features::ORB]
// ORB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:355
pub trait ORBTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_ORB(&self) -> *const c_void;

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:396
	// ("cv::ORB::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getMaxFeatures_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:399
	// ("cv::ORB::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getScaleFactor_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:402
	// ("cv::ORB::getNLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getNLevels_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:405
	// ("cv::ORB::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_edge_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getEdgeThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFirstLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:408
	// ("cv::ORB::getFirstLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_first_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getFirstLevel_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWTA_K()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:411
	// ("cv::ORB::getWTA_K", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_wta_k(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getWTA_K_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScoreType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:414
	// ("cv::ORB::getScoreType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_score_type(&self) -> Result<crate::features::ORB_ScoreType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getScoreType_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPatchSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:417
	// ("cv::ORB::getPatchSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_patch_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getPatchSize_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFastThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:420
	// ("cv::ORB::getFastThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_fast_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getFastThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:421
	// ("cv::ORB::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getDefaultName_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::ORB]
pub trait ORBTrait: crate::features::Feature2DTrait + crate::features::ORBTraitConst {
	fn as_raw_mut_ORB(&mut self) -> *mut c_void;

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:395
	// ("cv::ORB::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	#[inline]
	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setMaxFeatures_int(self.as_raw_mut_ORB(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:398
	// ("cv::ORB::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setScaleFactor_double(self.as_raw_mut_ORB(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:401
	// ("cv::ORB::setNLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
	#[inline]
	fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setNLevels_int(self.as_raw_mut_ORB(), nlevels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setEdgeThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:404
	// ("cv::ORB::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["int"]), _)]),
	#[inline]
	fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setEdgeThreshold_int(self.as_raw_mut_ORB(), edge_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFirstLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:407
	// ("cv::ORB::setFirstLevel", vec![(pred!(mut, ["firstLevel"], ["int"]), _)]),
	#[inline]
	fn set_first_level(&mut self, first_level: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setFirstLevel_int(self.as_raw_mut_ORB(), first_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWTA_K(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:410
	// ("cv::ORB::setWTA_K", vec![(pred!(mut, ["wta_k"], ["int"]), _)]),
	#[inline]
	fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setWTA_K_int(self.as_raw_mut_ORB(), wta_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScoreType(ORB::ScoreType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:413
	// ("cv::ORB::setScoreType", vec![(pred!(mut, ["scoreType"], ["cv::ORB::ScoreType"]), _)]),
	#[inline]
	fn set_score_type(&mut self, score_type: crate::features::ORB_ScoreType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setScoreType_ScoreType(self.as_raw_mut_ORB(), score_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:416
	// ("cv::ORB::setPatchSize", vec![(pred!(mut, ["patchSize"], ["int"]), _)]),
	#[inline]
	fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setPatchSize_int(self.as_raw_mut_ORB(), patch_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFastThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:419
	// ("cv::ORB::setFastThreshold", vec![(pred!(mut, ["fastThreshold"], ["int"]), _)]),
	#[inline]
	fn set_fast_threshold(&mut self, fast_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setFastThreshold_int(self.as_raw_mut_ORB(), fast_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the ORB (*oriented BRIEF*) keypoint detector and descriptor extractor
///
/// described in [RRKB11](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_RRKB11) . The algorithm uses FAST in pyramids to detect stable keypoints, selects
/// the strongest features using FAST or Harris response, finds their orientation using first-order
/// moments and computes the descriptors using BRIEF (where the coordinates of random point pairs (or
/// k-tuples) are rotated according to the measured orientation).
// ORB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:355
pub struct ORB {
	ptr: *mut c_void,
}

opencv_type_boxed! { ORB }

impl Drop for ORB {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_ORB_delete(self.as_raw_mut_ORB()) };
	}
}

unsafe impl Send for ORB {}

impl core::AlgorithmTraitConst for ORB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ORB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ORB, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for ORB {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for ORB {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ORB, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features::ORBTraitConst for ORB {
	#[inline] fn as_raw_ORB(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::ORBTrait for ORB {
	#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ORB, crate::features::ORBTraitConst, as_raw_ORB, crate::features::ORBTrait, as_raw_mut_ORB }

impl ORB {
	// kBytes /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:359
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
	// create(int, float, int, int, int, int, ORB::ScoreType, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:392
	// ("cv::ORB::create", vec![(pred!(mut, ["nfeatures", "scaleFactor", "nlevels", "edgeThreshold", "firstLevel", "WTA_K", "scoreType", "patchSize", "fastThreshold"], ["int", "float", "int", "int", "int", "int", "cv::ORB::ScoreType", "int", "int"]), _)]),
	#[inline]
	pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: crate::features::ORB_ScoreType, patch_size: i32, fast_threshold: i32) -> Result<core::Ptr<crate::features::ORB>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::ORB>::opencv_from_extern(ret) };
		Ok(ret)
	}

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
	/// ## Note
	/// This alternative version of [ORB::create] function uses the following default values for its arguments:
	/// * nfeatures: 500
	/// * scale_factor: 1.2f
	/// * nlevels: 8
	/// * edge_threshold: 31
	/// * first_level: 0
	/// * wta_k: 2
	/// * score_type: ORB::HARRIS_SCORE
	/// * patch_size: 31
	/// * fast_threshold: 20
	// cv::ORB::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:392
	// ("cv::ORB::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features::ORB>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::ORB>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ORB, core::Algorithm, cv_ORB_to_Algorithm }

boxed_cast_base! { ORB, crate::features::Feature2D, cv_ORB_to_Feature2D }

impl std::fmt::Debug for ORB {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ORB")
			.finish()
	}
}

/// Constant methods for [crate::features::SIFT]
// SIFT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:263
pub trait SIFTTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_SIFT(&self) -> *const c_void;

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:326
	// ("cv::SIFT::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getDefaultName_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getNFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:329
	// ("cv::SIFT::getNFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getNFeatures_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:332
	// ("cv::SIFT::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getNOctaveLayers_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getContrastThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:335
	// ("cv::SIFT::getContrastThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_contrast_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getContrastThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:338
	// ("cv::SIFT::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_edge_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getEdgeThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:341
	// ("cv::SIFT::getSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getSigma_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::SIFT]
pub trait SIFTTrait: crate::features::Feature2DTrait + crate::features::SIFTTraitConst {
	fn as_raw_mut_SIFT(&mut self) -> *mut c_void;

	// setNFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:328
	// ("cv::SIFT::setNFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	#[inline]
	fn set_n_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setNFeatures_int(self.as_raw_mut_SIFT(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:331
	// ("cv::SIFT::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setNOctaveLayers_int(self.as_raw_mut_SIFT(), n_octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setContrastThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:334
	// ("cv::SIFT::setContrastThreshold", vec![(pred!(mut, ["contrastThreshold"], ["double"]), _)]),
	#[inline]
	fn set_contrast_threshold(&mut self, contrast_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setContrastThreshold_double(self.as_raw_mut_SIFT(), contrast_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setEdgeThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:337
	// ("cv::SIFT::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["double"]), _)]),
	#[inline]
	fn set_edge_threshold(&mut self, edge_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setEdgeThreshold_double(self.as_raw_mut_SIFT(), edge_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:340
	// ("cv::SIFT::setSigma", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
	#[inline]
	fn set_sigma(&mut self, sigma: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setSigma_double(self.as_raw_mut_SIFT(), sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for extracting keypoints and computing descriptors using the Scale Invariant Feature Transform
/// (SIFT) algorithm by D. Lowe [Lowe04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lowe04) .
// SIFT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:263
pub struct SIFT {
	ptr: *mut c_void,
}

opencv_type_boxed! { SIFT }

impl Drop for SIFT {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_SIFT_delete(self.as_raw_mut_SIFT()) };
	}
}

unsafe impl Send for SIFT {}

impl core::AlgorithmTraitConst for SIFT {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SIFT {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SIFT, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for SIFT {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for SIFT {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SIFT, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features::SIFTTraitConst for SIFT {
	#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::SIFTTrait for SIFT {
	#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SIFT, crate::features::SIFTTraitConst, as_raw_SIFT, crate::features::SIFTTrait, as_raw_mut_SIFT }

impl SIFT {
	/// ## Parameters
	/// * nfeatures: The number of best features to retain. The features are ranked by their scores
	/// (measured in SIFT algorithm as the local contrast)
	///
	/// * nOctaveLayers: The number of layers in each octave. 3 is the value used in D. Lowe paper. The
	/// number of octaves is computed automatically from the image resolution.
	///
	/// * contrastThreshold: The contrast threshold used to filter out weak features in semi-uniform
	/// (low-contrast) regions. The larger the threshold, the less features are produced by the detector.
	///
	///
	/// Note: The contrast threshold will be divided by nOctaveLayers when the filtering is applied. When
	/// nOctaveLayers is set to default and if you want to use the value used in D. Lowe paper, 0.03, set
	/// this argument to 0.09.
	///
	/// * edgeThreshold: The threshold used to filter out edge-like features. Note that the its meaning
	/// is different from the contrastThreshold, i.e. the larger the edgeThreshold, the less features are
	/// filtered out (more features are retained).
	///
	/// * sigma: The sigma of the Gaussian applied to the input image at the octave \#0. If your image
	/// is captured with a weak camera with soft lenses, you might want to reduce the number.
	///
	/// * enable_precise_upscale: Whether to enable precise upscaling in the scale pyramid, which maps
	/// index ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bx%7D) to ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B2x%7D). This prevents localization bias. The option
	/// is disabled by default.
	///
	/// ## C++ default parameters
	/// * nfeatures: 0
	/// * n_octave_layers: 3
	/// * contrast_threshold: 0.04
	/// * edge_threshold: 10
	/// * sigma: 1.6
	/// * enable_precise_upscale: false
	// create(int, int, double, double, double, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:291
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "bool"]), _)]),
	#[inline]
	pub fn create(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, enable_precise_upscale: bool) -> Result<core::Ptr<crate::features::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double_bool(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, enable_precise_upscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::SIFT>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * nfeatures: The number of best features to retain. The features are ranked by their scores
	/// (measured in SIFT algorithm as the local contrast)
	///
	/// * nOctaveLayers: The number of layers in each octave. 3 is the value used in D. Lowe paper. The
	/// number of octaves is computed automatically from the image resolution.
	///
	/// * contrastThreshold: The contrast threshold used to filter out weak features in semi-uniform
	/// (low-contrast) regions. The larger the threshold, the less features are produced by the detector.
	///
	///
	/// Note: The contrast threshold will be divided by nOctaveLayers when the filtering is applied. When
	/// nOctaveLayers is set to default and if you want to use the value used in D. Lowe paper, 0.03, set
	/// this argument to 0.09.
	///
	/// * edgeThreshold: The threshold used to filter out edge-like features. Note that the its meaning
	/// is different from the contrastThreshold, i.e. the larger the edgeThreshold, the less features are
	/// filtered out (more features are retained).
	///
	/// * sigma: The sigma of the Gaussian applied to the input image at the octave \#0. If your image
	/// is captured with a weak camera with soft lenses, you might want to reduce the number.
	///
	/// * enable_precise_upscale: Whether to enable precise upscaling in the scale pyramid, which maps
	/// index ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bx%7D) to ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B2x%7D). This prevents localization bias. The option
	/// is disabled by default.
	///
	/// ## Note
	/// This alternative version of [SIFT::create] function uses the following default values for its arguments:
	/// * nfeatures: 0
	/// * n_octave_layers: 3
	/// * contrast_threshold: 0.04
	/// * edge_threshold: 10
	/// * sigma: 1.6
	/// * enable_precise_upscale: false
	// cv::SIFT::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:291
	// ("cv::SIFT::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::SIFT>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create SIFT with specified descriptorType.
	/// ## Parameters
	/// * nfeatures: The number of best features to retain. The features are ranked by their scores
	/// (measured in SIFT algorithm as the local contrast)
	///
	/// * nOctaveLayers: The number of layers in each octave. 3 is the value used in D. Lowe paper. The
	/// number of octaves is computed automatically from the image resolution.
	///
	/// * contrastThreshold: The contrast threshold used to filter out weak features in semi-uniform
	/// (low-contrast) regions. The larger the threshold, the less features are produced by the detector.
	///
	///
	/// Note: The contrast threshold will be divided by nOctaveLayers when the filtering is applied. When
	/// nOctaveLayers is set to default and if you want to use the value used in D. Lowe paper, 0.03, set
	/// this argument to 0.09.
	///
	/// * edgeThreshold: The threshold used to filter out edge-like features. Note that the its meaning
	/// is different from the contrastThreshold, i.e. the larger the edgeThreshold, the less features are
	/// filtered out (more features are retained).
	///
	/// * sigma: The sigma of the Gaussian applied to the input image at the octave \#0. If your image
	/// is captured with a weak camera with soft lenses, you might want to reduce the number.
	///
	/// * descriptorType: The type of descriptors. Only CV_32F and CV_8U are supported.
	///
	/// * enable_precise_upscale: Whether to enable precise upscaling in the scale pyramid, which maps
	/// index ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bx%7D) to ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B2x%7D). This prevents localization bias. The option
	/// is disabled by default.
	///
	/// ## C++ default parameters
	/// * enable_precise_upscale: false
	// create(int, int, double, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:322
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "int", "bool"]), _)]),
	#[inline]
	pub fn create_with_descriptor_type(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32, enable_precise_upscale: bool) -> Result<core::Ptr<crate::features::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double_int_bool(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, descriptor_type, enable_precise_upscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::SIFT>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create SIFT with specified descriptorType.
	/// ## Parameters
	/// * nfeatures: The number of best features to retain. The features are ranked by their scores
	/// (measured in SIFT algorithm as the local contrast)
	///
	/// * nOctaveLayers: The number of layers in each octave. 3 is the value used in D. Lowe paper. The
	/// number of octaves is computed automatically from the image resolution.
	///
	/// * contrastThreshold: The contrast threshold used to filter out weak features in semi-uniform
	/// (low-contrast) regions. The larger the threshold, the less features are produced by the detector.
	///
	///
	/// Note: The contrast threshold will be divided by nOctaveLayers when the filtering is applied. When
	/// nOctaveLayers is set to default and if you want to use the value used in D. Lowe paper, 0.03, set
	/// this argument to 0.09.
	///
	/// * edgeThreshold: The threshold used to filter out edge-like features. Note that the its meaning
	/// is different from the contrastThreshold, i.e. the larger the edgeThreshold, the less features are
	/// filtered out (more features are retained).
	///
	/// * sigma: The sigma of the Gaussian applied to the input image at the octave \#0. If your image
	/// is captured with a weak camera with soft lenses, you might want to reduce the number.
	///
	/// * descriptorType: The type of descriptors. Only CV_32F and CV_8U are supported.
	///
	/// * enable_precise_upscale: Whether to enable precise upscaling in the scale pyramid, which maps
	/// index ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bx%7D) to ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B2x%7D). This prevents localization bias. The option
	/// is disabled by default.
	///
	/// ## Note
	/// This alternative version of [SIFT::create_with_descriptor_type] function uses the following default values for its arguments:
	/// * enable_precise_upscale: false
	// cv::SIFT::create(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:322
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType"], ["int", "int", "double", "double", "double", "int"]), _)]),
	#[inline]
	pub fn create_with_descriptor_type_def(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32) -> Result<core::Ptr<crate::features::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double_int(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, descriptor_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::SIFT>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SIFT, core::Algorithm, cv_SIFT_to_Algorithm }

boxed_cast_base! { SIFT, crate::features::Feature2D, cv_SIFT_to_Feature2D }

impl std::fmt::Debug for SIFT {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SIFT")
			.finish()
	}
}

/// Constant methods for [crate::features::SimpleBlobDetector]
// SimpleBlobDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:621
pub trait SimpleBlobDetectorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_SimpleBlobDetector(&self) -> *const c_void;

	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:658
	// ("cv::SimpleBlobDetector::getParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_params(&self) -> Result<crate::features::SimpleBlobDetector_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_getParams_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:660
	// ("cv::SimpleBlobDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_getDefaultName_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getBlobContours()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:661
	// ("cv::SimpleBlobDetector::getBlobContours", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_blob_contours(&self) -> Result<core::Vector<core::Vector<core::Point>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_getBlobContours_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Vector<core::Point>>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features::SimpleBlobDetector]
pub trait SimpleBlobDetectorTrait: crate::features::Feature2DTrait + crate::features::SimpleBlobDetectorTraitConst {
	fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void;

	// setParams(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:657
	// ("cv::SimpleBlobDetector::setParams", vec![(pred!(mut, ["params"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
	#[inline]
	fn set_params(&mut self, params: crate::features::SimpleBlobDetector_Params) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_setParams_const_ParamsR(self.as_raw_mut_SimpleBlobDetector(), &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
// SimpleBlobDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:621
pub struct SimpleBlobDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { SimpleBlobDetector }

impl Drop for SimpleBlobDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_SimpleBlobDetector_delete(self.as_raw_mut_SimpleBlobDetector()) };
	}
}

unsafe impl Send for SimpleBlobDetector {}

impl core::AlgorithmTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SimpleBlobDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SimpleBlobDetector, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features::SimpleBlobDetectorTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::SimpleBlobDetectorTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SimpleBlobDetector, crate::features::SimpleBlobDetectorTraitConst, as_raw_SimpleBlobDetector, crate::features::SimpleBlobDetectorTrait, as_raw_mut_SimpleBlobDetector }

impl SimpleBlobDetector {
	/// ## C++ default parameters
	/// * parameters: SimpleBlobDetector::Params()
	// create(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:655
	// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, ["parameters"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: crate::features::SimpleBlobDetector_Params) -> Result<core::Ptr<crate::features::SimpleBlobDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::SimpleBlobDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [SimpleBlobDetector::create] function uses the following default values for its arguments:
	/// * parameters: SimpleBlobDetector::Params()
	// cv::SimpleBlobDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:655
	// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features::SimpleBlobDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features::SimpleBlobDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SimpleBlobDetector, core::Algorithm, cv_SimpleBlobDetector_to_Algorithm }

boxed_cast_base! { SimpleBlobDetector, crate::features::Feature2D, cv_SimpleBlobDetector_to_Feature2D }

impl std::fmt::Debug for SimpleBlobDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SimpleBlobDetector")
			.finish()
	}
}

// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:624
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
	pub collect_contours: bool,
}

opencv_type_simple! { crate::features::SimpleBlobDetector_Params }

impl SimpleBlobDetector_Params {
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:651
	// ("cv::SimpleBlobDetector::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	pub fn write(self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_write_const_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:626
	// ("cv::SimpleBlobDetector::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::features::SimpleBlobDetector_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/features.hpp:650
	// ("cv::SimpleBlobDetector::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	pub fn read(self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_read_const_FileNodeR(&self, fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}
