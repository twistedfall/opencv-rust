//! # 2D Features Framework
//!   # Feature Detection and Description
//!   # Descriptor Matchers
//!
//!   Matchers of keypoint descriptors in OpenCV have wrappers with a common interface that enables
//!   you to easily switch between different algorithms solving the same problem. This section is
//!   devoted to matching descriptors that are represented as vectors in a multidimensional space.
//!   All objects that implement vector descriptor matchers inherit the DescriptorMatcher interface.
//!
//!   # Drawing Function of Keypoints and Matches
//!   # Object Categorization
//!
//!   This section describes approaches based on local 2D features and used to categorize objects.
//!
//!   # Hardware Acceleration Layer
//!       # Interface
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{AKAZETrait, AKAZETraitConst, AffineFeatureTrait, AffineFeatureTraitConst, AgastFeatureDetectorTrait, AgastFeatureDetectorTraitConst, BFMatcherTrait, BFMatcherTraitConst, BOWImgDescriptorExtractorTrait, BOWImgDescriptorExtractorTraitConst, BOWKMeansTrainerTrait, BOWKMeansTrainerTraitConst, BOWTrainerTrait, BOWTrainerTraitConst, BRISKTrait, BRISKTraitConst, DescriptorMatcherTrait, DescriptorMatcherTraitConst, FastFeatureDetectorTrait, FastFeatureDetectorTraitConst, Feature2DTrait, Feature2DTraitConst, FlannBasedMatcherTrait, FlannBasedMatcherTraitConst, GFTTDetectorTrait, GFTTDetectorTraitConst, KAZETrait, KAZETraitConst, KeyPointsFilterTrait, KeyPointsFilterTraitConst, MSERTrait, MSERTraitConst, ORBTrait, ORBTraitConst, SIFTTrait, SIFTTraitConst, SimpleBlobDetectorTrait, SimpleBlobDetectorTraitConst};
}

// DESCRIPTOR_KAZE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:864
pub const AKAZE_DESCRIPTOR_KAZE: i32 = 3;
/// Upright descriptors, not invariant to rotation
// DESCRIPTOR_KAZE_UPRIGHT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:863
pub const AKAZE_DESCRIPTOR_KAZE_UPRIGHT: i32 = 2;
// DESCRIPTOR_MLDB /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:866
pub const AKAZE_DESCRIPTOR_MLDB: i32 = 5;
/// Upright descriptors, not invariant to rotation
// DESCRIPTOR_MLDB_UPRIGHT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:865
pub const AKAZE_DESCRIPTOR_MLDB_UPRIGHT: i32 = 4;
// AGAST_5_8 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:634
pub const AgastFeatureDetector_AGAST_5_8: i32 = 0;
// AGAST_7_12d /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:634
pub const AgastFeatureDetector_AGAST_7_12d: i32 = 1;
// AGAST_7_12s /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:634
pub const AgastFeatureDetector_AGAST_7_12s: i32 = 2;
// NONMAX_SUPPRESSION /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:639
pub const AgastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
// OAST_9_16 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:634
pub const AgastFeatureDetector_OAST_9_16: i32 = 3;
// THRESHOLD /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:639
pub const AgastFeatureDetector_THRESHOLD: i32 = 10000;
// BRUTEFORCE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1000
pub const DescriptorMatcher_BRUTEFORCE: i32 = 2;
// BRUTEFORCE_HAMMING /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1002
pub const DescriptorMatcher_BRUTEFORCE_HAMMING: i32 = 4;
// BRUTEFORCE_HAMMINGLUT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1003
pub const DescriptorMatcher_BRUTEFORCE_HAMMINGLUT: i32 = 5;
// BRUTEFORCE_L1 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1001
pub const DescriptorMatcher_BRUTEFORCE_L1: i32 = 3;
// BRUTEFORCE_SL2 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1004
pub const DescriptorMatcher_BRUTEFORCE_SL2: i32 = 6;
// FLANNBASED /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:999
pub const DescriptorMatcher_FLANNBASED: i32 = 1;
/// Output image matrix will be created (Mat::create),
/// i.e. existing memory of output image may be reused.
/// Two source image, matches and single keypoints will be drawn.
/// For each keypoint only the center point will be drawn (without
/// the circle around keypoint with keypoint size and orientation).
// DEFAULT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1344
pub const DrawMatchesFlags_DEFAULT: i32 = 0;
/// Output image matrix will not be created (Mat::create).
/// Matches will be drawn on existing content of output image.
// DRAW_OVER_OUTIMG /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1349
pub const DrawMatchesFlags_DRAW_OVER_OUTIMG: i32 = 1;
/// For each keypoint the circle around keypoint with keypoint size and
/// orientation will be drawn.
// DRAW_RICH_KEYPOINTS /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1352
pub const DrawMatchesFlags_DRAW_RICH_KEYPOINTS: i32 = 4;
/// Single keypoints will not be drawn.
// NOT_DRAW_SINGLE_POINTS /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1351
pub const DrawMatchesFlags_NOT_DRAW_SINGLE_POINTS: i32 = 2;
// FAST_N /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:582
pub const FastFeatureDetector_FAST_N: i32 = 10002;
// NONMAX_SUPPRESSION /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:582
pub const FastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
// THRESHOLD /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:582
pub const FastFeatureDetector_THRESHOLD: i32 = 10000;
// TYPE_5_8 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:578
pub const FastFeatureDetector_TYPE_5_8: i32 = 0;
// TYPE_7_12 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:578
pub const FastFeatureDetector_TYPE_7_12: i32 = 1;
// TYPE_9_16 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:578
pub const FastFeatureDetector_TYPE_9_16: i32 = 2;
// DIFF_CHARBONNIER /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:803
pub const KAZE_DIFF_CHARBONNIER: i32 = 3;
// DIFF_PM_G1 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:800
pub const KAZE_DIFF_PM_G1: i32 = 0;
// DIFF_PM_G2 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:801
pub const KAZE_DIFF_PM_G2: i32 = 1;
// DIFF_WEICKERT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:802
pub const KAZE_DIFF_WEICKERT: i32 = 2;
// FAST_SCORE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:426
pub const ORB_FAST_SCORE: i32 = 1;
// HARRIS_SCORE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:426
pub const ORB_HARRIS_SCORE: i32 = 0;
// DescriptorType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:861
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AKAZE_DescriptorType {
	/// Upright descriptors, not invariant to rotation
	DESCRIPTOR_KAZE_UPRIGHT = 2,
	DESCRIPTOR_KAZE = 3,
	/// Upright descriptors, not invariant to rotation
	DESCRIPTOR_MLDB_UPRIGHT = 4,
	DESCRIPTOR_MLDB = 5,
}

impl TryFrom<i32> for AKAZE_DescriptorType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			2 => Ok(Self::DESCRIPTOR_KAZE_UPRIGHT),
			3 => Ok(Self::DESCRIPTOR_KAZE),
			4 => Ok(Self::DESCRIPTOR_MLDB_UPRIGHT),
			5 => Ok(Self::DESCRIPTOR_MLDB),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features2d::AKAZE_DescriptorType"))),
		}
	}
}

opencv_type_enum! { crate::features2d::AKAZE_DescriptorType }

// DetectorType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:632
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AgastFeatureDetector_DetectorType {
	AGAST_5_8 = 0,
	AGAST_7_12d = 1,
	AGAST_7_12s = 2,
	OAST_9_16 = 3,
}

impl TryFrom<i32> for AgastFeatureDetector_DetectorType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::AGAST_5_8),
			1 => Ok(Self::AGAST_7_12d),
			2 => Ok(Self::AGAST_7_12s),
			3 => Ok(Self::OAST_9_16),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features2d::AgastFeatureDetector_DetectorType"))),
		}
	}
}

opencv_type_enum! { crate::features2d::AgastFeatureDetector_DetectorType }

// MatcherType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:997
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
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features2d::DescriptorMatcher_MatcherType"))),
		}
	}
}

opencv_type_enum! { crate::features2d::DescriptorMatcher_MatcherType }

// DrawMatchesFlags /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1342
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
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features2d::DrawMatchesFlags"))),
		}
	}
}

opencv_type_enum! { crate::features2d::DrawMatchesFlags }

// DetectorType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:576
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
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features2d::FastFeatureDetector_DetectorType"))),
		}
	}
}

opencv_type_enum! { crate::features2d::FastFeatureDetector_DetectorType }

// DiffusivityType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:798
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KAZE_DiffusivityType {
	DIFF_PM_G1 = 0,
	DIFF_PM_G2 = 1,
	DIFF_WEICKERT = 2,
	DIFF_CHARBONNIER = 3,
}

impl TryFrom<i32> for KAZE_DiffusivityType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DIFF_PM_G1),
			1 => Ok(Self::DIFF_PM_G2),
			2 => Ok(Self::DIFF_WEICKERT),
			3 => Ok(Self::DIFF_CHARBONNIER),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features2d::KAZE_DiffusivityType"))),
		}
	}
}

opencv_type_enum! { crate::features2d::KAZE_DiffusivityType }

// ScoreType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:426
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
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::features2d::ORB_ScoreType"))),
		}
	}
}

opencv_type_enum! { crate::features2d::ORB_ScoreType }

// AffineDescriptorExtractor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:260
pub type AffineDescriptorExtractor = crate::features2d::AffineFeature;
// AffineFeatureDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:259
pub type AffineFeatureDetector = crate::features2d::AffineFeature;
/// Extractors of keypoint descriptors in OpenCV have wrappers with a common interface that enables you
/// to easily switch between different algorithms solving the same problem. This section is devoted to
/// computing descriptors represented as vectors in a multidimensional space. All objects that implement
/// the vector descriptor extractors inherit the DescriptorExtractor interface.
// DescriptorExtractor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:235
pub type DescriptorExtractor = crate::features2d::Feature2D;
/// Feature detectors in OpenCV have wrappers with a common interface that enables you to easily switch
/// between different algorithms solving the same problem. All objects that implement keypoint detectors
/// inherit the FeatureDetector interface.
// FeatureDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:228
pub type FeatureDetector = crate::features2d::Feature2D;
// SiftDescriptorExtractor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:348
pub type SiftDescriptorExtractor = crate::features2d::SIFT;
// SiftFeatureDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:347
pub type SiftFeatureDetector = crate::features2d::SIFT;
/// @overload
///
/// ## Note
/// This alternative version of [agast] function uses the following default values for its arguments:
/// * nonmax_suppression: true
// cv::AGAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:658
// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
#[inline]
pub fn agast_def(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_mair2010_agast) .
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * nonmax_suppression: true
// AGAST(InputArray, std::vector<KeyPoint> &, int, bool)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:658
// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool"]), _)]),
#[inline]
pub fn agast(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
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
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_mair2010_agast) .
// AGAST(InputArray, std::vector<KeyPoint> &, int, bool, AgastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:679
// ("cv::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::AgastFeatureDetector::DetectorType"]), _)]),
#[inline]
pub fn agast_with_type(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// @overload
///
/// ## Note
/// This alternative version of [fast] function uses the following default values for its arguments:
/// * nonmax_suppression: true
// cv::FAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:602
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
#[inline]
pub fn fast_def(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
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
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
///
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Rosten06) .
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
// FAST(InputArray, std::vector<KeyPoint> &, int, bool)(InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:602
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool"]), _)]),
#[inline]
pub fn fast(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, ocvrs_return.as_mut_ptr()) };
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
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
///
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Rosten06) .
///
///
/// Note: In Python API, types are given as cv.FAST_FEATURE_DETECTOR_TYPE_5_8,
/// cv.FAST_FEATURE_DETECTOR_TYPE_7_12 and cv.FAST_FEATURE_DETECTOR_TYPE_9_16. For corner
/// detection, use cv.FAST.detect() method.
// FAST(InputArray, std::vector<KeyPoint> &, int, bool, FastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:623
// ("cv::FAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
#[inline]
pub fn fast_with_type(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// computeRecallPrecisionCurve(const std::vector<std::vector<DMatch>> &, const std::vector<std::vector<uchar>> &, std::vector<Point2f> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1431
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
// cv::drawKeypoints(InputArray, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1372
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
// drawKeypoints(InputArray, const std::vector<KeyPoint> &, InputOutputArray, const Scalar &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputOutputArray, SimpleClass, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1372
// ("cv::drawKeypoints", vec![(pred!(mut, ["image", "keypoints", "outImage", "color", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_keypoints(image: &impl ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut impl ToInputOutputArray, color: core::Scalar, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
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
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1397
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
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1397
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_matches(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
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
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1404
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
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<DMatch> &, InputOutputArray, const int, const Scalar &, const Scalar &, const std::vector<char> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, Primitive, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1404
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchesThickness", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "const cv::_InputOutputArray*", "const int", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_matches_with_thickness(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, matches_thickness: i32, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
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
// cv::drawMatches(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1411
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
// drawMatches(InputArray, const std::vector<KeyPoint> &, InputArray, const std::vector<KeyPoint> &, const std::vector<std::vector<DMatch>> &, InputOutputArray, const Scalar &, const Scalar &, const std::vector<std::vector<char>> &, DrawMatchesFlags)(InputArray, CppPassByVoidPtr, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, InputOutputArray, SimpleClass, SimpleClass, CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1411
// ("cv::drawMatches", vec![(pred!(mut, ["img1", "keypoints1", "img2", "keypoints2", "matches1to2", "outImg", "matchColor", "singlePointColor", "matchesMask", "flags"], ["const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const cv::_InputArray*", "const std::vector<cv::KeyPoint>*", "const std::vector<std::vector<cv::DMatch>>*", "const cv::_InputOutputArray*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<std::vector<char>>*", "cv::DrawMatchesFlags"]), _)]),
#[inline]
pub fn draw_matches_knn(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut impl ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<core::Vector<c_char>>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
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
// cv::evaluateFeatureDetector(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1426
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
// evaluateFeatureDetector(const Mat &, const Mat &, const Mat &, std::vector<KeyPoint> *, std::vector<KeyPoint> *, float &, int &, const Ptr<FeatureDetector> &)(TraitClass, TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Indirect, Indirect, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1426
// ("cv::evaluateFeatureDetector", vec![(pred!(mut, ["img1", "img2", "H1to2", "keypoints1", "keypoints2", "repeatability", "correspCount", "fdetector"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "std::vector<cv::KeyPoint>*", "std::vector<cv::KeyPoint>*", "float*", "int*", "const cv::Ptr<cv::Feature2D>*"]), _)]),
#[inline]
pub fn evaluate_feature_detector(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, h1to2: &impl core::MatTraitConst, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32, fdetector: &core::Ptr<crate::features2d::Feature2D>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR_const_PtrLFeature2DGR(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, fdetector.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getNearestPoint(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1436
// ("cv::getNearestPoint", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
#[inline]
pub fn get_nearest_point(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getNearestPoint_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// getRecall(const std::vector<Point2f> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1435
// ("cv::getRecall", vec![(pred!(mut, ["recallPrecisionCurve", "l_precision"], ["const std::vector<cv::Point2f>*", "float"]), _)]),
#[inline]
pub fn get_recall(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<f32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_getRecall_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::features2d::AKAZE]
// AKAZE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:857
pub trait AKAZETraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AKAZE(&self) -> *const c_void;

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:891
	// ("cv::AKAZE::getDescriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_type(&self) -> Result<crate::features2d::AKAZE_DescriptorType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDescriptorType_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:894
	// ("cv::AKAZE::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDescriptorSize_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDescriptorChannels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:897
	// ("cv::AKAZE::getDescriptorChannels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_channels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDescriptorChannels_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:900
	// ("cv::AKAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getThreshold_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:903
	// ("cv::AKAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getNOctaves_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:906
	// ("cv::AKAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getNOctaveLayers_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:909
	// ("cv::AKAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDiffusivity_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:910
	// ("cv::AKAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getDefaultName_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMaxPoints()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:913
	// ("cv::AKAZE::getMaxPoints", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_getMaxPoints_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::AKAZE]
pub trait AKAZETrait: crate::features2d::AKAZETraitConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_AKAZE(&mut self) -> *mut c_void;

	// setDescriptorType(AKAZE::DescriptorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:890
	// ("cv::AKAZE::setDescriptorType", vec![(pred!(mut, ["dtype"], ["cv::AKAZE::DescriptorType"]), _)]),
	#[inline]
	fn set_descriptor_type(&mut self, dtype: crate::features2d::AKAZE_DescriptorType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDescriptorType_DescriptorType(self.as_raw_mut_AKAZE(), dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDescriptorSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:893
	// ("cv::AKAZE::setDescriptorSize", vec![(pred!(mut, ["dsize"], ["int"]), _)]),
	#[inline]
	fn set_descriptor_size(&mut self, dsize: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDescriptorSize_int(self.as_raw_mut_AKAZE(), dsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDescriptorChannels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:896
	// ("cv::AKAZE::setDescriptorChannels", vec![(pred!(mut, ["dch"], ["int"]), _)]),
	#[inline]
	fn set_descriptor_channels(&mut self, dch: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDescriptorChannels_int(self.as_raw_mut_AKAZE(), dch, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:899
	// ("cv::AKAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setThreshold_double(self.as_raw_mut_AKAZE(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:902
	// ("cv::AKAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	#[inline]
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setNOctaves_int(self.as_raw_mut_AKAZE(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:905
	// ("cv::AKAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setNOctaveLayers_int(self.as_raw_mut_AKAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDiffusivity(KAZE::DiffusivityType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:908
	// ("cv::AKAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["cv::KAZE::DiffusivityType"]), _)]),
	#[inline]
	fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_AKAZE(), diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPoints(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:912
	// ("cv::AKAZE::setMaxPoints", vec![(pred!(mut, ["max_points"], ["int"]), _)]),
	#[inline]
	fn set_max_points(&mut self, max_points: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_setMaxPoints_int(self.as_raw_mut_AKAZE(), max_points, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the AKAZE keypoint detector and descriptor extractor, described in [ANB13](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_ANB13).
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
// AKAZE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:857
pub struct AKAZE {
	ptr: *mut c_void,
}

opencv_type_boxed! { AKAZE }

impl Drop for AKAZE {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_AKAZE_delete(self.as_raw_mut_AKAZE()) };
	}
}

unsafe impl Send for AKAZE {}

impl core::AlgorithmTraitConst for AKAZE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AKAZE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AKAZE, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features2d::Feature2DTraitConst for AKAZE {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for AKAZE {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AKAZE, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::AKAZETraitConst for AKAZE {
	#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::AKAZETrait for AKAZE {
	#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AKAZE, crate::features2d::AKAZETraitConst, as_raw_AKAZE, crate::features2d::AKAZETrait, as_raw_mut_AKAZE }

impl AKAZE {
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
	/// * max_points: Maximum amount of returned points. In case if image contains
	/// more features, then the features with highest response are returned.
	/// Negative value means no limitation.
	///
	/// ## C++ default parameters
	/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
	/// * descriptor_size: 0
	/// * descriptor_channels: 3
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	/// * max_points: -1
	// create(AKAZE::DescriptorType, int, int, float, int, int, KAZE::DiffusivityType, int)(Enum, Primitive, Primitive, Primitive, Primitive, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:884
	// ("cv::AKAZE::create", vec![(pred!(mut, ["descriptor_type", "descriptor_size", "descriptor_channels", "threshold", "nOctaves", "nOctaveLayers", "diffusivity", "max_points"], ["cv::AKAZE::DescriptorType", "int", "int", "float", "int", "int", "cv::KAZE::DiffusivityType", "int"]), _)]),
	#[inline]
	pub fn create(descriptor_type: crate::features2d::AKAZE_DescriptorType, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType, max_points: i32) -> Result<core::Ptr<crate::features2d::AKAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType_int(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity, max_points, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::AKAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}

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
	/// * max_points: Maximum amount of returned points. In case if image contains
	/// more features, then the features with highest response are returned.
	/// Negative value means no limitation.
	///
	/// ## Note
	/// This alternative version of [AKAZE::create] function uses the following default values for its arguments:
	/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
	/// * descriptor_size: 0
	/// * descriptor_channels: 3
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	/// * max_points: -1
	// cv::AKAZE::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:884
	// ("cv::AKAZE::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::AKAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AKAZE_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::AKAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AKAZE, core::Algorithm, cv_AKAZE_to_Algorithm }

boxed_cast_base! { AKAZE, crate::features2d::Feature2D, cv_AKAZE_to_Feature2D }

impl std::fmt::Debug for AKAZE {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AKAZE")
			.finish()
	}
}

/// Constant methods for [crate::features2d::AffineFeature]
// AffineFeature /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:241
pub trait AffineFeatureTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AffineFeature(&self) -> *const c_void;

	// getViewParams(std::vector<float> &, std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:255
	// ("cv::AffineFeature::getViewParams", vec![(pred!(const, ["tilts", "rolls"], ["std::vector<float>*", "std::vector<float>*"]), _)]),
	#[inline]
	fn get_view_params(&self, tilts: &mut core::Vector<f32>, rolls: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_getViewParams_const_vectorLfloatGR_vectorLfloatGR(self.as_raw_AffineFeature(), tilts.as_raw_mut_VectorOff32(), rolls.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:256
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

/// Mutable methods for [crate::features2d::AffineFeature]
pub trait AffineFeatureTrait: crate::features2d::AffineFeatureTraitConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void;

	// setViewParams(const std::vector<float> &, const std::vector<float> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:254
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
/// described as ASIFT in [YM11](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_YM11) .
// AffineFeature /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:241
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

impl crate::features2d::Feature2DTraitConst for AffineFeature {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for AffineFeature {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::AffineFeatureTraitConst for AffineFeature {
	#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::AffineFeatureTrait for AffineFeature {
	#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature, crate::features2d::AffineFeatureTraitConst, as_raw_AffineFeature, crate::features2d::AffineFeatureTrait, as_raw_mut_AffineFeature }

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
	// create(const Ptr<Feature2D> &, int, int, float, float)(CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:251
	// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend", "maxTilt", "minTilt", "tiltStep", "rotateStepBase"], ["const cv::Ptr<cv::Feature2D>*", "int", "int", "float", "float"]), _)]),
	#[inline]
	pub fn create(backend: &core::Ptr<crate::features2d::Feature2D>, max_tilt: i32, min_tilt: i32, tilt_step: f32, rotate_step_base: f32) -> Result<core::Ptr<crate::features2d::AffineFeature>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_create_const_PtrLFeature2DGR_int_int_float_float(backend.as_raw_PtrOfFeature2D(), max_tilt, min_tilt, tilt_step, rotate_step_base, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::AffineFeature>::opencv_from_extern(ret) };
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
	// cv::AffineFeature::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:251
	// ("cv::AffineFeature::create", vec![(pred!(mut, ["backend"], ["const cv::Ptr<cv::Feature2D>*"]), _)]),
	#[inline]
	pub fn create_def(backend: &core::Ptr<crate::features2d::Feature2D>) -> Result<core::Ptr<crate::features2d::AffineFeature>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineFeature_create_const_PtrLFeature2DGR(backend.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::AffineFeature>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AffineFeature, core::Algorithm, cv_AffineFeature_to_Algorithm }

boxed_cast_base! { AffineFeature, crate::features2d::Feature2D, cv_AffineFeature_to_Feature2D }

impl std::fmt::Debug for AffineFeature {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AffineFeature")
			.finish()
	}
}

/// Constant methods for [crate::features2d::AgastFeatureDetector]
// AgastFeatureDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:629
pub trait AgastFeatureDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AgastFeatureDetector(&self) -> *const c_void;

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:647
	// ("cv::AgastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getThreshold_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:650
	// ("cv::AgastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nonmax_suppression(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getNonmaxSuppression_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:653
	// ("cv::AgastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_type(&self) -> Result<crate::features2d::AgastFeatureDetector_DetectorType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getType_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:654
	// ("cv::AgastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_getDefaultName_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::AgastFeatureDetector]
pub trait AgastFeatureDetectorTrait: crate::features2d::AgastFeatureDetectorTraitConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void;

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:646
	// ("cv::AgastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_setThreshold_int(self.as_raw_mut_AgastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:649
	// ("cv::AgastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	#[inline]
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_AgastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setType(AgastFeatureDetector::DetectorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:652
	// ("cv::AgastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["cv::AgastFeatureDetector::DetectorType"]), _)]),
	#[inline]
	fn set_type(&mut self, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_setType_DetectorType(self.as_raw_mut_AgastFeatureDetector(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Wrapping class for feature detection using the AGAST method. :
// AgastFeatureDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:629
pub struct AgastFeatureDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { AgastFeatureDetector }

impl Drop for AgastFeatureDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_AgastFeatureDetector_delete(self.as_raw_mut_AgastFeatureDetector()) };
	}
}

unsafe impl Send for AgastFeatureDetector {}

impl core::AlgorithmTraitConst for AgastFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AgastFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AgastFeatureDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features2d::Feature2DTraitConst for AgastFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for AgastFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AgastFeatureDetector, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::AgastFeatureDetectorTraitConst for AgastFeatureDetector {
	#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::AgastFeatureDetectorTrait for AgastFeatureDetector {
	#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AgastFeatureDetector, crate::features2d::AgastFeatureDetectorTraitConst, as_raw_AgastFeatureDetector, crate::features2d::AgastFeatureDetectorTrait, as_raw_mut_AgastFeatureDetector }

impl AgastFeatureDetector {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: AgastFeatureDetector::OAST_9_16
	// create(int, bool, AgastFeatureDetector::DetectorType)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:642
	// ("cv::AgastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "cv::AgastFeatureDetector::DetectorType"]), _)]),
	#[inline]
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<core::Ptr<crate::features2d::AgastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::AgastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [AgastFeatureDetector::create] function uses the following default values for its arguments:
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: AgastFeatureDetector::OAST_9_16
	// cv::AgastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:642
	// ("cv::AgastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::AgastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AgastFeatureDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::AgastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AgastFeatureDetector, core::Algorithm, cv_AgastFeatureDetector_to_Algorithm }

boxed_cast_base! { AgastFeatureDetector, crate::features2d::Feature2D, cv_AgastFeatureDetector_to_Feature2D }

impl std::fmt::Debug for AgastFeatureDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AgastFeatureDetector")
			.finish()
	}
}

/// Constant methods for [crate::features2d::BFMatcher]
// BFMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1246
pub trait BFMatcherTraitConst: crate::features2d::DescriptorMatcherTraitConst {
	fn as_raw_BFMatcher(&self) -> *const c_void;

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1257
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
	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1273
	// ("cv::BFMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_clone_const_bool(self.as_raw_BFMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [BFMatcherTraitConst::clone] function uses the following default values for its arguments:
	/// * empty_train_data: false
	// cv::BFMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1273
	// ("cv::BFMatcher::clone", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn clone_def(&self) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_clone_const(self.as_raw_BFMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::BFMatcher]
pub trait BFMatcherTrait: crate::features2d::BFMatcherTraitConst + crate::features2d::DescriptorMatcherTrait {
	fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void;

}

/// Brute-force descriptor matcher.
///
/// For each descriptor in the first set, this matcher finds the closest descriptor in the second set
/// by trying each one. This descriptor matcher supports masking permissible matches of descriptor
/// sets.
// BFMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1246
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

impl crate::features2d::DescriptorMatcherTraitConst for BFMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::DescriptorMatcherTrait for BFMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BFMatcher, crate::features2d::DescriptorMatcherTraitConst, as_raw_DescriptorMatcher, crate::features2d::DescriptorMatcherTrait, as_raw_mut_DescriptorMatcher }

impl crate::features2d::BFMatcherTraitConst for BFMatcher {
	#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BFMatcherTrait for BFMatcher {
	#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BFMatcher, crate::features2d::BFMatcherTraitConst, as_raw_BFMatcher, crate::features2d::BFMatcherTrait, as_raw_mut_BFMatcher }

impl BFMatcher {
	/// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
	///
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	/// * cross_check: false
	// BFMatcher(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1253
	// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
	#[inline]
	pub fn new(norm_type: i32, cross_check: bool) -> Result<crate::features2d::BFMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_BFMatcher_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BFMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * norm_type: NORM_L2
	/// * cross_check: false
	// cv::BFMatcher::BFMatcher() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1253
	// ("cv::BFMatcher::BFMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::features2d::BFMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_BFMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BFMatcher::opencv_from_extern(ret) };
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
	// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1271
	// ("cv::BFMatcher::create", vec![(pred!(mut, ["normType", "crossCheck"], ["int", "bool"]), _)]),
	#[inline]
	pub fn create(norm_type: i32, cross_check: bool) -> Result<core::Ptr<crate::features2d::BFMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_create_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BFMatcher>::opencv_from_extern(ret) };
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
	// cv::BFMatcher::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1271
	// ("cv::BFMatcher::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::BFMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BFMatcher_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BFMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BFMatcher, core::Algorithm, cv_BFMatcher_to_Algorithm }

boxed_cast_base! { BFMatcher, crate::features2d::DescriptorMatcher, cv_BFMatcher_to_DescriptorMatcher }

impl std::fmt::Debug for BFMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BFMatcher")
			.finish()
	}
}

/// Constant methods for [crate::features2d::BOWImgDescriptorExtractor]
// BOWImgDescriptorExtractor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1531
pub trait BOWImgDescriptorExtractorTraitConst {
	fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void;

	/// Returns the set vocabulary.
	// getVocabulary()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1556
	// ("cv::BOWImgDescriptorExtractor::getVocabulary", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_vocabulary(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_getVocabulary_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns an image descriptor size if the vocabulary is set. Otherwise, it returns 0.
	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1586
	// ("cv::BOWImgDescriptorExtractor::descriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorSize_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns an image descriptor type.
	// descriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1590
	// ("cv::BOWImgDescriptorExtractor::descriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorType_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::BOWImgDescriptorExtractor]
pub trait BOWImgDescriptorExtractorTrait: crate::features2d::BOWImgDescriptorExtractorTraitConst {
	fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void;

	/// Sets a visual vocabulary.
	///
	/// ## Parameters
	/// * vocabulary: Vocabulary (can be trained using the inheritor of BOWTrainer ). Each row of the
	/// vocabulary is a visual word (cluster center).
	// setVocabulary(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1552
	// ("cv::BOWImgDescriptorExtractor::setVocabulary", vec![(pred!(mut, ["vocabulary"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_vocabulary(&mut self, vocabulary: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_setVocabulary_const_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), vocabulary.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// compute(InputArray, std::vector<KeyPoint> &, OutputArray, std::vector<std::vector<int>> *, Mat *)(InputArray, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1568
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor", "pointIdxsOfClusters", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*", "cv::Mat*"]), _)]),
	#[inline]
	fn compute_desc(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>, descriptors: &mut impl core::MatTrait) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_vectorLvectorLintGGX_MatX(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32(), descriptors.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [BOWImgDescriptorExtractorTrait::compute_desc] function uses the following default values for its arguments:
	/// * point_idxs_of_clusters: 0
	/// * descriptors: 0
	// cv::BOWImgDescriptorExtractor::compute(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1568
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_desc_def(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// pointIdxsOfClusters[i] are keypoint indices that belong to the i -th cluster (word of vocabulary)
	/// returned if it is non-zero.
	///
	/// ## C++ default parameters
	/// * point_idxs_of_clusters: 0
	// compute(InputArray, OutputArray, std::vector<std::vector<int>> *)(InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1577
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor", "pointIdxsOfClusters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*"]), _)]),
	#[inline]
	fn compute(&mut self, keypoint_descriptors: &impl ToInputArray, img_descriptor: &mut impl ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>) -> Result<()> {
		input_array_arg!(keypoint_descriptors);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vectorLvectorLintGGX(self.as_raw_mut_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw__InputArray(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// @overload
	/// ## Parameters
	/// * keypointDescriptors: Computed descriptors to match with vocabulary.
	/// * imgDescriptor: Computed output image descriptor.
	/// * pointIdxsOfClusters: Indices of keypoints that belong to the cluster. This means that
	/// pointIdxsOfClusters[i] are keypoint indices that belong to the i -th cluster (word of vocabulary)
	/// returned if it is non-zero.
	///
	/// ## Note
	/// This alternative version of [BOWImgDescriptorExtractorTrait::compute] function uses the following default values for its arguments:
	/// * point_idxs_of_clusters: 0
	// cv::BOWImgDescriptorExtractor::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1577
	// ("cv::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_def(&mut self, keypoint_descriptors: &impl ToInputArray, img_descriptor: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(keypoint_descriptors);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw__InputArray(), img_descriptor.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// compute2(const Mat &, std::vector<KeyPoint> &, Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1581
	// ("cv::BOWImgDescriptorExtractor::compute2", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::Mat*", "std::vector<cv::KeyPoint>*", "cv::Mat*"]), _)]),
	#[inline]
	fn compute2(&mut self, image: &impl core::MatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_compute2_const_MatR_vectorLKeyPointGR_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
// BOWImgDescriptorExtractor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1531
pub struct BOWImgDescriptorExtractor {
	ptr: *mut c_void,
}

opencv_type_boxed! { BOWImgDescriptorExtractor }

impl Drop for BOWImgDescriptorExtractor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BOWImgDescriptorExtractor_delete(self.as_raw_mut_BOWImgDescriptorExtractor()) };
	}
}

unsafe impl Send for BOWImgDescriptorExtractor {}

impl crate::features2d::BOWImgDescriptorExtractorTraitConst for BOWImgDescriptorExtractor {
	#[inline] fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BOWImgDescriptorExtractorTrait for BOWImgDescriptorExtractor {
	#[inline] fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWImgDescriptorExtractor, crate::features2d::BOWImgDescriptorExtractorTraitConst, as_raw_BOWImgDescriptorExtractor, crate::features2d::BOWImgDescriptorExtractorTrait, as_raw_mut_BOWImgDescriptorExtractor }

impl BOWImgDescriptorExtractor {
	/// The constructor.
	///
	/// ## Parameters
	/// * dextractor: Descriptor extractor that is used to compute descriptors for an input image and
	/// its keypoints.
	/// * dmatcher: Descriptor matcher that is used to find the nearest word of the trained vocabulary
	/// for each keypoint descriptor of the image.
	// BOWImgDescriptorExtractor(const Ptr<Feature2D> &, const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1541
	// ("cv::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dextractor", "dmatcher"], ["const cv::Ptr<cv::Feature2D>*", "const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	#[inline]
	pub fn new_with_extractor(dextractor: &core::Ptr<crate::features2d::Feature2D>, dmatcher: &core::Ptr<crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLFeature2DGR_const_PtrLDescriptorMatcherGR(dextractor.as_raw_PtrOfFeature2D(), dmatcher.as_raw_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BOWImgDescriptorExtractor::opencv_from_extern(ret) };
		Ok(ret)
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
	// BOWImgDescriptorExtractor(const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1544
	// ("cv::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dmatcher"], ["const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	#[inline]
	pub fn new(dmatcher: &core::Ptr<crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLDescriptorMatcherGR(dmatcher.as_raw_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BOWImgDescriptorExtractor::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for BOWImgDescriptorExtractor {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BOWImgDescriptorExtractor")
			.finish()
	}
}

/// Constant methods for [crate::features2d::BOWKMeansTrainer]
// BOWKMeansTrainer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1498
pub trait BOWKMeansTrainerTraitConst: crate::features2d::BOWTrainerTraitConst {
	fn as_raw_BOWKMeansTrainer(&self) -> *const c_void;

	// cluster()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1510
	// ("cv::BOWKMeansTrainer::cluster", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cluster(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWKMeansTrainer_cluster_const(self.as_raw_BOWKMeansTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1511
	// ("cv::BOWKMeansTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn cluster_with_descriptor(&self, descriptors: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWKMeansTrainer_cluster_const_const_MatR(self.as_raw_BOWKMeansTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::BOWKMeansTrainer]
pub trait BOWKMeansTrainerTrait: crate::features2d::BOWKMeansTrainerTraitConst + crate::features2d::BOWTrainerTrait {
	fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void;

}

/// kmeans -based class to train visual vocabulary using the *bag of visual words* approach. :
// BOWKMeansTrainer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1498
pub struct BOWKMeansTrainer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BOWKMeansTrainer }

impl Drop for BOWKMeansTrainer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BOWKMeansTrainer_delete(self.as_raw_mut_BOWKMeansTrainer()) };
	}
}

unsafe impl Send for BOWKMeansTrainer {}

impl crate::features2d::BOWTrainerTraitConst for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BOWTrainerTrait for BOWKMeansTrainer {
	#[inline] fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWKMeansTrainer, crate::features2d::BOWTrainerTraitConst, as_raw_BOWTrainer, crate::features2d::BOWTrainerTrait, as_raw_mut_BOWTrainer }

impl crate::features2d::BOWKMeansTrainerTraitConst for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWKMeansTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BOWKMeansTrainerTrait for BOWKMeansTrainer {
	#[inline] fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWKMeansTrainer, crate::features2d::BOWKMeansTrainerTraitConst, as_raw_BOWKMeansTrainer, crate::features2d::BOWKMeansTrainerTrait, as_raw_mut_BOWKMeansTrainer }

impl BOWKMeansTrainer {
	/// The constructor.
	/// ## See also
	/// cv::kmeans
	///
	/// ## C++ default parameters
	/// * termcrit: TermCriteria()
	/// * attempts: 3
	/// * flags: KMEANS_PP_CENTERS
	// BOWKMeansTrainer(int, const TermCriteria &, int, int)(Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1505
	// ("cv::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount", "termcrit", "attempts", "flags"], ["int", "const cv::TermCriteria*", "int", "int"]), _)]),
	#[inline]
	pub fn new(cluster_count: i32, termcrit: core::TermCriteria, attempts: i32, flags: i32) -> Result<crate::features2d::BOWKMeansTrainer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(cluster_count, &termcrit, attempts, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BOWKMeansTrainer::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// The constructor.
	/// ## See also
	/// cv::kmeans
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * termcrit: TermCriteria()
	/// * attempts: 3
	/// * flags: KMEANS_PP_CENTERS
	// cv::BOWKMeansTrainer::BOWKMeansTrainer(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1505
	// ("cv::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount"], ["int"]), _)]),
	#[inline]
	pub fn new_def(cluster_count: i32) -> Result<crate::features2d::BOWKMeansTrainer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWKMeansTrainer_BOWKMeansTrainer_int(cluster_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::BOWKMeansTrainer::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BOWKMeansTrainer, crate::features2d::BOWTrainer, cv_BOWKMeansTrainer_to_BOWTrainer }

impl std::fmt::Debug for BOWKMeansTrainer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BOWKMeansTrainer")
			.finish()
	}
}

/// Constant methods for [crate::features2d::BOWTrainer]
// BOWTrainer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1452
pub trait BOWTrainerTraitConst {
	fn as_raw_BOWTrainer(&self) -> *const c_void;

	/// Returns a training set of descriptors.
	// getDescriptors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1469
	// ("cv::BOWTrainer::getDescriptors", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptors(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_getDescriptors_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns the count of all descriptors stored in the training set.
	// descriptorsCount()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1473
	// ("cv::BOWTrainer::descriptorsCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptors_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_descriptorsCount_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// cluster()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1478
	// ("cv::BOWTrainer::cluster", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cluster(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_cluster_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
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
	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1489
	// ("cv::BOWTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn cluster_with_descriptors(&self, descriptors: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_cluster_const_const_MatR(self.as_raw_BOWTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::BOWTrainer]
pub trait BOWTrainerTrait: crate::features2d::BOWTrainerTraitConst {
	fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void;

	/// Adds descriptors to a training set.
	///
	/// ## Parameters
	/// * descriptors: Descriptors to add to a training set. Each row of the descriptors matrix is a
	/// descriptor.
	///
	/// The training set is clustered using clustermethod to construct the vocabulary.
	// add(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1465
	// ("cv::BOWTrainer::add", vec![(pred!(mut, ["descriptors"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn add(&mut self, descriptors: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_add_const_MatR(self.as_raw_mut_BOWTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1475
	// ("cv::BOWTrainer::clear", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BOWTrainer_clear(self.as_raw_mut_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for training the *bag of visual words* vocabulary from a set of descriptors.
///
/// For details, see, for example, *Visual Categorization with Bags of Keypoints* by Gabriella Csurka,
/// Christopher R. Dance, Lixin Fan, Jutta Willamowski, Cedric Bray, 2004. :
// BOWTrainer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1452
pub struct BOWTrainer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BOWTrainer }

impl Drop for BOWTrainer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BOWTrainer_delete(self.as_raw_mut_BOWTrainer()) };
	}
}

unsafe impl Send for BOWTrainer {}

impl crate::features2d::BOWTrainerTraitConst for BOWTrainer {
	#[inline] fn as_raw_BOWTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BOWTrainerTrait for BOWTrainer {
	#[inline] fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWTrainer, crate::features2d::BOWTrainerTraitConst, as_raw_BOWTrainer, crate::features2d::BOWTrainerTrait, as_raw_mut_BOWTrainer }

impl BOWTrainer {
}

boxed_cast_descendant! { BOWTrainer, crate::features2d::BOWKMeansTrainer, cv_BOWTrainer_to_BOWKMeansTrainer }

impl std::fmt::Debug for BOWTrainer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BOWTrainer")
			.finish()
	}
}

/// Constant methods for [crate::features2d::BRISK]
// BRISK /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:353
pub trait BRISKTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BRISK(&self) -> *const c_void;

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:395
	// ("cv::BRISK::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_getDefaultName_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:401
	// ("cv::BRISK::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_getThreshold_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:407
	// ("cv::BRISK::getOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_getOctaves_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPatternScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:413
	// ("cv::BRISK::getPatternScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pattern_scale(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_getPatternScale_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::BRISK]
pub trait BRISKTrait: crate::features2d::BRISKTraitConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_BRISK(&mut self) -> *mut c_void;

	/// Set detection threshold.
	/// ## Parameters
	/// * threshold: AGAST detection threshold score.
	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:400
	// ("cv::BRISK::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_setThreshold_int(self.as_raw_mut_BRISK(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set detection octaves.
	/// ## Parameters
	/// * octaves: detection octaves. Use 0 to do single scale.
	// setOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:406
	// ("cv::BRISK::setOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	#[inline]
	fn set_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_setOctaves_int(self.as_raw_mut_BRISK(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set detection patternScale.
	/// ## Parameters
	/// * patternScale: apply this scale to the pattern used for sampling the neighbourhood of a
	/// keypoint.
	// setPatternScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:412
	// ("cv::BRISK::setPatternScale", vec![(pred!(mut, ["patternScale"], ["float"]), _)]),
	#[inline]
	fn set_pattern_scale(&mut self, pattern_scale: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_setPatternScale_float(self.as_raw_mut_BRISK(), pattern_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the BRISK keypoint detector and descriptor extractor, described in [LCS11](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_LCS11) .
// BRISK /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:353
pub struct BRISK {
	ptr: *mut c_void,
}

opencv_type_boxed! { BRISK }

impl Drop for BRISK {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_BRISK_delete(self.as_raw_mut_BRISK()) };
	}
}

unsafe impl Send for BRISK {}

impl core::AlgorithmTraitConst for BRISK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BRISK {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BRISK, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features2d::Feature2DTraitConst for BRISK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for BRISK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BRISK, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::BRISKTraitConst for BRISK {
	#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::BRISKTrait for BRISK {
	#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BRISK, crate::features2d::BRISKTraitConst, as_raw_BRISK, crate::features2d::BRISKTrait, as_raw_mut_BRISK }

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
	// create(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:363
	// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "patternScale"], ["int", "int", "float"]), _)]),
	#[inline]
	pub fn create(thresh: i32, octaves: i32, pattern_scale: f32) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_int_int_float(thresh, octaves, pattern_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// The BRISK constructor
	///
	/// ## Parameters
	/// * thresh: AGAST detection threshold score.
	/// * octaves: detection octaves. Use 0 to do single scale.
	/// * patternScale: apply this scale to the pattern used for sampling the neighbourhood of a
	/// keypoint.
	///
	/// ## Note
	/// This alternative version of [BRISK::create] function uses the following default values for its arguments:
	/// * thresh: 30
	/// * octaves: 3
	/// * pattern_scale: 1.0f
	// cv::BRISK::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:363
	// ("cv::BRISK::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
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
	// create(const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:376
	// ("cv::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList", "dMax", "dMin", "indexChange"], ["const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_with_pattern(radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>, d_max: f32, d_min: f32, index_change: &core::Vector<i32>) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [BRISK::create_with_pattern] function uses the following default values for its arguments:
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	// cv::BRISK::create(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:376
	// ("cv::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList"], ["const std::vector<float>*", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_with_pattern_def(radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_const_vectorLfloatGR_const_vectorLintGR(radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
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
	// create(int, int, const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:392
	// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList", "dMax", "dMin", "indexChange"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_with_pattern_threshold_octaves(thresh: i32, octaves: i32, radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>, d_max: f32, d_min: f32, index_change: &core::Vector<i32>) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(thresh, octaves, radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [BRISK::create_with_pattern_threshold_octaves] function uses the following default values for its arguments:
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	// cv::BRISK::create(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:392
	// ("cv::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_with_pattern_threshold_octaves_def(thresh: i32, octaves: i32, radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>) -> Result<core::Ptr<crate::features2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR(thresh, octaves, radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BRISK, core::Algorithm, cv_BRISK_to_Algorithm }

boxed_cast_base! { BRISK, crate::features2d::Feature2D, cv_BRISK_to_Feature2D }

impl std::fmt::Debug for BRISK {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BRISK")
			.finish()
	}
}

/// Constant methods for [crate::features2d::DescriptorMatcher]
// DescriptorMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:994
pub trait DescriptorMatcherTraitConst: core::AlgorithmTraitConst {
	fn as_raw_DescriptorMatcher(&self) -> *const c_void;

	/// Returns a constant link to the train descriptor collection trainDescCollection .
	// getTrainDescriptors()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1021
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
	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1029
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
	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1033
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
	// match(InputArray, InputArray, std::vector<DMatch> &, InputArray)(InputArray, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1060
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
	// cv::DescriptorMatcher::match(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1060
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
	// knnMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, int, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1081
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
	// cv::DescriptorMatcher::knnMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1081
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
	// radiusMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, float, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1104
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
	// cv::DescriptorMatcher::radiusMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1104
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

	// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1146
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1161
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
	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1169
	// ("cv::DescriptorMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_clone_const_bool(self.as_raw_DescriptorMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
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
	// cv::DescriptorMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1169
	// ("cv::DescriptorMatcher::clone", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn clone_def(&self) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_clone_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1188
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

	// write(const Ptr<FileStorage> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1190
	// ("cv::DescriptorMatcher::write", vec![(pred!(const, ["fs", "name"], ["const cv::Ptr<cv::FileStorage>*", "const cv::String*"]), _)]),
	#[inline]
	fn write_to_storage_ptr_with_name(&self, fs: &core::Ptr<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_write_const_const_PtrLFileStorageGR_const_StringR(self.as_raw_DescriptorMatcher(), fs.as_raw_PtrOfFileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::DescriptorMatcher]
pub trait DescriptorMatcherTrait: core::AlgorithmTrait + crate::features2d::DescriptorMatcherTraitConst {
	fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void;

	/// Adds descriptors to train a CPU(trainDescCollectionis) or GPU(utrainDescCollectionis) descriptor
	/// collection.
	///
	/// If the collection is not empty, the new descriptors are added to existing train descriptors.
	///
	/// ## Parameters
	/// * descriptors: Descriptors to add. Each descriptors[i] is a set of descriptors from the same
	/// train image.
	// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1017
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
	// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1025
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
	// train()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1042
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
	// match(InputArray, std::vector<DMatch> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1115
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
	// cv::DescriptorMatcher::match(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1115
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
	// knnMatch(InputArray, std::vector<std::vector<DMatch>> &, int, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1128
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
	// cv::DescriptorMatcher::knnMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1128
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
	// radiusMatch(InputArray, std::vector<std::vector<DMatch>> &, float, InputArrayOfArrays, bool)(InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1142
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
	// cv::DescriptorMatcher::radiusMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1142
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

	// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1152
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1159
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
// DescriptorMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:994
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

impl crate::features2d::DescriptorMatcherTraitConst for DescriptorMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::DescriptorMatcherTrait for DescriptorMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DescriptorMatcher, crate::features2d::DescriptorMatcherTraitConst, as_raw_DescriptorMatcher, crate::features2d::DescriptorMatcherTrait, as_raw_mut_DescriptorMatcher }

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
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1182
	// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["descriptorMatcherType"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn create(descriptor_matcher_type: &str) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		extern_container_arg!(descriptor_matcher_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create(const DescriptorMatcher::MatcherType &)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1184
	// ("cv::DescriptorMatcher::create", vec![(pred!(mut, ["matcherType"], ["const cv::DescriptorMatcher::MatcherType*"]), _)]),
	#[inline]
	pub fn create_with_matcher_type(matcher_type: crate::features2d::DescriptorMatcher_MatcherType) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_DescriptorMatcher_create_const_MatcherTypeR(matcher_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { DescriptorMatcher, crate::features2d::BFMatcher, cv_DescriptorMatcher_to_BFMatcher }

boxed_cast_descendant! { DescriptorMatcher, crate::features2d::FlannBasedMatcher, cv_DescriptorMatcher_to_FlannBasedMatcher }

boxed_cast_base! { DescriptorMatcher, core::Algorithm, cv_DescriptorMatcher_to_Algorithm }

impl std::fmt::Debug for DescriptorMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DescriptorMatcher")
			.finish()
	}
}

/// Constant methods for [crate::features2d::FastFeatureDetector]
// FastFeatureDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:573
pub trait FastFeatureDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_FastFeatureDetector(&self) -> *const c_void;

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:591
	// ("cv::FastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getThreshold_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:594
	// ("cv::FastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nonmax_suppression(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getNonmaxSuppression_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:597
	// ("cv::FastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_type(&self) -> Result<crate::features2d::FastFeatureDetector_DetectorType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_getType_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:598
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

/// Mutable methods for [crate::features2d::FastFeatureDetector]
pub trait FastFeatureDetectorTrait: crate::features2d::FastFeatureDetectorTraitConst + crate::features2d::Feature2DTrait {
	fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void;

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:590
	// ("cv::FastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setThreshold_int(self.as_raw_mut_FastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:593
	// ("cv::FastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	#[inline]
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_FastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setType(FastFeatureDetector::DetectorType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:596
	// ("cv::FastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["cv::FastFeatureDetector::DetectorType"]), _)]),
	#[inline]
	fn set_type(&mut self, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_setType_DetectorType(self.as_raw_mut_FastFeatureDetector(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Wrapping class for feature detection using the FAST method. :
// FastFeatureDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:573
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

impl crate::features2d::Feature2DTraitConst for FastFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for FastFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastFeatureDetector, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::FastFeatureDetectorTraitConst for FastFeatureDetector {
	#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::FastFeatureDetectorTrait for FastFeatureDetector {
	#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastFeatureDetector, crate::features2d::FastFeatureDetectorTraitConst, as_raw_FastFeatureDetector, crate::features2d::FastFeatureDetectorTrait, as_raw_mut_FastFeatureDetector }

impl FastFeatureDetector {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: FastFeatureDetector::TYPE_9_16
	// create(int, bool, FastFeatureDetector::DetectorType)(Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:586
	// ("cv::FastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
	#[inline]
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<core::Ptr<crate::features2d::FastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::FastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [FastFeatureDetector::create] function uses the following default values for its arguments:
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: FastFeatureDetector::TYPE_9_16
	// cv::FastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:586
	// ("cv::FastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::FastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FastFeatureDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::FastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FastFeatureDetector, core::Algorithm, cv_FastFeatureDetector_to_Algorithm }

boxed_cast_base! { FastFeatureDetector, crate::features2d::Feature2D, cv_FastFeatureDetector_to_Feature2D }

impl std::fmt::Debug for FastFeatureDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FastFeatureDetector")
			.finish()
	}
}

/// Constant methods for [crate::features2d::Feature2D]
// Feature2D /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:137
pub trait Feature2DTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Feature2D(&self) -> *const c_void;

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:201
	// ("cv::Feature2D::descriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_descriptorSize_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// descriptorType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:202
	// ("cv::Feature2D::descriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_descriptorType_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:203
	// ("cv::Feature2D::defaultNorm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_defaultNorm_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:205
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:209
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
	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:215
	// ("cv::Feature2D::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_empty_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:216
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

	// write(FileStorage &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:219
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

	// write(const Ptr<FileStorage> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:221
	// ("cv::Feature2D::write", vec![(pred!(const, ["fs", "name"], ["const cv::Ptr<cv::FileStorage>*", "const cv::String*"]), _)]),
	#[inline]
	fn write_to_storage_ptr_with_name(&self, fs: &core::Ptr<core::FileStorage>, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Feature2D_write_const_const_PtrLFileStorageGR_const_StringR(self.as_raw_Feature2D(), fs.as_raw_PtrOfFileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::Feature2D]
pub trait Feature2DTrait: core::AlgorithmTrait + crate::features2d::Feature2DTraitConst {
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
	// detect(InputArray, std::vector<KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:151
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
	// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:151
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
	// detect(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, InputArrayOfArrays)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:162
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
	// cv::Feature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:162
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
	// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:177
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
	// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:191
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
	// detectAndCompute(InputArray, InputArray, std::vector<KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:196
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
	// cv::Feature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:196
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

	// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:207
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:212
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

// Feature2D /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:137
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

impl crate::features2d::Feature2DTraitConst for Feature2D {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for Feature2D {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Feature2D, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl Feature2D {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_Feature2D_defaultNew_const()) }
	}

}

boxed_cast_descendant! { Feature2D, crate::features2d::AKAZE, cv_Feature2D_to_AKAZE }

boxed_cast_descendant! { Feature2D, crate::features2d::AffineFeature, cv_Feature2D_to_AffineFeature }

boxed_cast_descendant! { Feature2D, crate::features2d::AgastFeatureDetector, cv_Feature2D_to_AgastFeatureDetector }

boxed_cast_descendant! { Feature2D, crate::features2d::BRISK, cv_Feature2D_to_BRISK }

boxed_cast_descendant! { Feature2D, crate::features2d::FastFeatureDetector, cv_Feature2D_to_FastFeatureDetector }

boxed_cast_descendant! { Feature2D, crate::features2d::GFTTDetector, cv_Feature2D_to_GFTTDetector }

boxed_cast_descendant! { Feature2D, crate::features2d::KAZE, cv_Feature2D_to_KAZE }

boxed_cast_descendant! { Feature2D, crate::features2d::MSER, cv_Feature2D_to_MSER }

boxed_cast_descendant! { Feature2D, crate::features2d::ORB, cv_Feature2D_to_ORB }

boxed_cast_descendant! { Feature2D, crate::features2d::SIFT, cv_Feature2D_to_SIFT }

boxed_cast_descendant! { Feature2D, crate::features2d::SimpleBlobDetector, cv_Feature2D_to_SimpleBlobDetector }

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

/// Constant methods for [crate::features2d::FlannBasedMatcher]
// FlannBasedMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1293
pub trait FlannBasedMatcherTraitConst: crate::features2d::DescriptorMatcherTraitConst {
	fn as_raw_FlannBasedMatcher(&self) -> *const c_void;

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1305
	// ("cv::FlannBasedMatcher::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_write_const_FileStorageR(self.as_raw_FlannBasedMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1308
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
	// clone(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1312
	// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, ["emptyTrainData"], ["bool"]), _)]),
	#[inline]
	#[must_use]
	fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clone_const_bool(self.as_raw_FlannBasedMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [FlannBasedMatcherTraitConst::clone] function uses the following default values for its arguments:
	/// * empty_train_data: false
	// cv::FlannBasedMatcher::clone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1312
	// ("cv::FlannBasedMatcher::clone", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn clone_def(&self) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clone_const(self.as_raw_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::FlannBasedMatcher]
pub trait FlannBasedMatcherTrait: crate::features2d::DescriptorMatcherTrait + crate::features2d::FlannBasedMatcherTraitConst {
	fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void;

	// add(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1299
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

	// clear()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1300
	// ("cv::FlannBasedMatcher::clear", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_clear(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1303
	// ("cv::FlannBasedMatcher::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_read_const_FileNodeR(self.as_raw_mut_FlannBasedMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// train()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1307
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
// FlannBasedMatcher /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1293
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

impl crate::features2d::DescriptorMatcherTraitConst for FlannBasedMatcher {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::DescriptorMatcherTrait for FlannBasedMatcher {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlannBasedMatcher, crate::features2d::DescriptorMatcherTraitConst, as_raw_DescriptorMatcher, crate::features2d::DescriptorMatcherTrait, as_raw_mut_DescriptorMatcher }

impl crate::features2d::FlannBasedMatcherTraitConst for FlannBasedMatcher {
	#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::FlannBasedMatcherTrait for FlannBasedMatcher {
	#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlannBasedMatcher, crate::features2d::FlannBasedMatcherTraitConst, as_raw_FlannBasedMatcher, crate::features2d::FlannBasedMatcherTrait, as_raw_mut_FlannBasedMatcher }

impl FlannBasedMatcher {
	/// ## C++ default parameters
	/// * index_params: makePtr<flann::KDTreeIndexParams>()
	/// * search_params: makePtr<flann::SearchParams>()
	// FlannBasedMatcher(const Ptr<flann::IndexParams> &, const Ptr<flann::SearchParams> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1296
	// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, ["indexParams", "searchParams"], ["const cv::Ptr<cv::flann::IndexParams>*", "const cv::Ptr<cv::flann::SearchParams>*"]), _)]),
	#[inline]
	pub fn new(index_params: &core::Ptr<crate::flann::IndexParams>, search_params: &core::Ptr<crate::flann::SearchParams>) -> Result<crate::features2d::FlannBasedMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher_const_PtrLIndexParamsGR_const_PtrLSearchParamsGR(index_params.as_raw_PtrOfIndexParams(), search_params.as_raw_PtrOfSearchParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::FlannBasedMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * index_params: makePtr<flann::KDTreeIndexParams>()
	/// * search_params: makePtr<flann::SearchParams>()
	// cv::FlannBasedMatcher::FlannBasedMatcher() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1296
	// ("cv::FlannBasedMatcher::FlannBasedMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::features2d::FlannBasedMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::FlannBasedMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:1310
	// ("cv::FlannBasedMatcher::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::features2d::FlannBasedMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FlannBasedMatcher_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::FlannBasedMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FlannBasedMatcher, core::Algorithm, cv_FlannBasedMatcher_to_Algorithm }

boxed_cast_base! { FlannBasedMatcher, crate::features2d::DescriptorMatcher, cv_FlannBasedMatcher_to_DescriptorMatcher }

impl std::fmt::Debug for FlannBasedMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FlannBasedMatcher")
			.finish()
	}
}

/// Constant methods for [crate::features2d::GFTTDetector]
// GFTTDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:684
pub trait GFTTDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_GFTTDetector(&self) -> *const c_void;

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:692
	// ("cv::GFTTDetector::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getMaxFeatures_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getQualityLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:695
	// ("cv::GFTTDetector::getQualityLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_quality_level(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getQualityLevel_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinDistance()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:698
	// ("cv::GFTTDetector::getMinDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_distance(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getMinDistance_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:701
	// ("cv::GFTTDetector::getBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getBlockSize_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getHarrisDetector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:707
	// ("cv::GFTTDetector::getHarrisDetector", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_harris_detector(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getHarrisDetector_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:710
	// ("cv::GFTTDetector::getK", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_k(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getK_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:711
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

/// Mutable methods for [crate::features2d::GFTTDetector]
pub trait GFTTDetectorTrait: crate::features2d::Feature2DTrait + crate::features2d::GFTTDetectorTraitConst {
	fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void;

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:691
	// ("cv::GFTTDetector::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	#[inline]
	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setMaxFeatures_int(self.as_raw_mut_GFTTDetector(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setQualityLevel(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:694
	// ("cv::GFTTDetector::setQualityLevel", vec![(pred!(mut, ["qlevel"], ["double"]), _)]),
	#[inline]
	fn set_quality_level(&mut self, qlevel: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setQualityLevel_double(self.as_raw_mut_GFTTDetector(), qlevel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDistance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:697
	// ("cv::GFTTDetector::setMinDistance", vec![(pred!(mut, ["minDistance"], ["double"]), _)]),
	#[inline]
	fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setMinDistance_double(self.as_raw_mut_GFTTDetector(), min_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:700
	// ("cv::GFTTDetector::setBlockSize", vec![(pred!(mut, ["blockSize"], ["int"]), _)]),
	#[inline]
	fn set_block_size(&mut self, block_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setBlockSize_int(self.as_raw_mut_GFTTDetector(), block_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setGradientSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:703
	// ("cv::GFTTDetector::setGradientSize", vec![(pred!(mut, ["gradientSize_"], ["int"]), _)]),
	#[inline]
	fn set_gradient_size(&mut self, gradient_size_: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setGradientSize_int(self.as_raw_mut_GFTTDetector(), gradient_size_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getGradientSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:704
	// ("cv::GFTTDetector::getGradientSize", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_gradient_size(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_getGradientSize(self.as_raw_mut_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setHarrisDetector(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:706
	// ("cv::GFTTDetector::setHarrisDetector", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_harris_detector(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_setHarrisDetector_bool(self.as_raw_mut_GFTTDetector(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setK(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:709
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
// GFTTDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:684
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

impl crate::features2d::Feature2DTraitConst for GFTTDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for GFTTDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GFTTDetector, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::GFTTDetectorTraitConst for GFTTDetector {
	#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::GFTTDetectorTrait for GFTTDetector {
	#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GFTTDetector, crate::features2d::GFTTDetectorTraitConst, as_raw_GFTTDetector, crate::features2d::GFTTDetectorTrait, as_raw_mut_GFTTDetector }

impl GFTTDetector {
	/// ## C++ default parameters
	/// * max_corners: 1000
	/// * quality_level: 0.01
	/// * min_distance: 1
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	// create(int, double, double, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:687
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "bool", "double"]), _)]),
	#[inline]
	pub fn create(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<crate::features2d::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners, quality_level, min_distance, block_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::GFTTDetector>::opencv_from_extern(ret) };
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
	// cv::GFTTDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:687
	// ("cv::GFTTDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_harris_detector: false
	/// * k: 0.04
	// create(int, double, double, int, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:689
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize", "useHarrisDetector", "k"], ["int", "double", "double", "int", "int", "bool", "double"]), _)]),
	#[inline]
	pub fn create_with_gradient(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<crate::features2d::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners, quality_level, min_distance, block_size, gradiant_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [GFTTDetector::create_with_gradient] function uses the following default values for its arguments:
	/// * use_harris_detector: false
	/// * k: 0.04
	// cv::GFTTDetector::create(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:689
	// ("cv::GFTTDetector::create", vec![(pred!(mut, ["maxCorners", "qualityLevel", "minDistance", "blockSize", "gradiantSize"], ["int", "double", "double", "int", "int"]), _)]),
	#[inline]
	pub fn create_with_gradient_def(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32) -> Result<core::Ptr<crate::features2d::GFTTDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int(max_corners, quality_level, min_distance, block_size, gradiant_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::GFTTDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { GFTTDetector, core::Algorithm, cv_GFTTDetector_to_Algorithm }

boxed_cast_base! { GFTTDetector, crate::features2d::Feature2D, cv_GFTTDetector_to_Feature2D }

impl std::fmt::Debug for GFTTDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GFTTDetector")
			.finish()
	}
}

/// Constant methods for [crate::features2d::KAZE]
// KAZE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:795
pub trait KAZETraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_KAZE(&self) -> *const c_void;

	// getExtended()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:822
	// ("cv::KAZE::getExtended", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_extended(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getExtended_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUpright()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:825
	// ("cv::KAZE::getUpright", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_upright(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getUpright_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:828
	// ("cv::KAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getThreshold_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:831
	// ("cv::KAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getNOctaves_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:834
	// ("cv::KAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getNOctaveLayers_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:837
	// ("cv::KAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getDiffusivity_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:838
	// ("cv::KAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_getDefaultName_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::features2d::KAZE]
pub trait KAZETrait: crate::features2d::Feature2DTrait + crate::features2d::KAZETraitConst {
	fn as_raw_mut_KAZE(&mut self) -> *mut c_void;

	// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:821
	// ("cv::KAZE::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
	#[inline]
	fn set_extended(&mut self, extended: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setExtended_bool(self.as_raw_mut_KAZE(), extended, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:824
	// ("cv::KAZE::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
	#[inline]
	fn set_upright(&mut self, upright: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setUpright_bool(self.as_raw_mut_KAZE(), upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:827
	// ("cv::KAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setThreshold_double(self.as_raw_mut_KAZE(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:830
	// ("cv::KAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	#[inline]
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setNOctaves_int(self.as_raw_mut_KAZE(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:833
	// ("cv::KAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setNOctaveLayers_int(self.as_raw_mut_KAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDiffusivity(KAZE::DiffusivityType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:836
	// ("cv::KAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["cv::KAZE::DiffusivityType"]), _)]),
	#[inline]
	fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_KAZE(), diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the KAZE keypoint detector and descriptor extractor, described in [ABD12](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_ABD12) .
///
///
/// Note: AKAZE descriptor can only be used with KAZE or AKAZE keypoints .. [ABD12] KAZE Features. Pablo
/// F. Alcantarilla, Adrien Bartoli and Andrew J. Davison. In European Conference on Computer Vision
/// (ECCV), Fiorenze, Italy, October 2012.
// KAZE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:795
pub struct KAZE {
	ptr: *mut c_void,
}

opencv_type_boxed! { KAZE }

impl Drop for KAZE {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_KAZE_delete(self.as_raw_mut_KAZE()) };
	}
}

unsafe impl Send for KAZE {}

impl core::AlgorithmTraitConst for KAZE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for KAZE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KAZE, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features2d::Feature2DTraitConst for KAZE {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for KAZE {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KAZE, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::KAZETraitConst for KAZE {
	#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::KAZETrait for KAZE {
	#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KAZE, crate::features2d::KAZETraitConst, as_raw_KAZE, crate::features2d::KAZETrait, as_raw_mut_KAZE }

impl KAZE {
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
	// create(bool, bool, float, int, int, KAZE::DiffusivityType)(Primitive, Primitive, Primitive, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:816
	// ("cv::KAZE::create", vec![(pred!(mut, ["extended", "upright", "threshold", "nOctaves", "nOctaveLayers", "diffusivity"], ["bool", "bool", "float", "int", "int", "cv::KAZE::DiffusivityType"]), _)]),
	#[inline]
	pub fn create(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType) -> Result<core::Ptr<crate::features2d::KAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_create_bool_bool_float_int_int_DiffusivityType(extended, upright, threshold, n_octaves, n_octave_layers, diffusivity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::KAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}

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
	/// ## Note
	/// This alternative version of [KAZE::create] function uses the following default values for its arguments:
	/// * extended: false
	/// * upright: false
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	// cv::KAZE::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:816
	// ("cv::KAZE::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::KAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KAZE_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::KAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { KAZE, core::Algorithm, cv_KAZE_to_Algorithm }

boxed_cast_base! { KAZE, crate::features2d::Feature2D, cv_KAZE_to_Feature2D }

impl std::fmt::Debug for KAZE {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KAZE")
			.finish()
	}
}

/// Constant methods for [crate::features2d::KeyPointsFilter]
// KeyPointsFilter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:92
pub trait KeyPointsFilterTraitConst {
	fn as_raw_KeyPointsFilter(&self) -> *const c_void;

}

/// Mutable methods for [crate::features2d::KeyPointsFilter]
pub trait KeyPointsFilterTrait: crate::features2d::KeyPointsFilterTraitConst {
	fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void;

}

/// A class filters a vector of keypoints.
///
/// Because now it is difficult to provide a convenient interface for all usage scenarios of the
/// keypoints filter class, it has only several needed by now static methods.
// KeyPointsFilter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:92
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

impl crate::features2d::KeyPointsFilterTraitConst for KeyPointsFilter {
	#[inline] fn as_raw_KeyPointsFilter(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::KeyPointsFilterTrait for KeyPointsFilter {
	#[inline] fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KeyPointsFilter, crate::features2d::KeyPointsFilterTraitConst, as_raw_KeyPointsFilter, crate::features2d::KeyPointsFilterTrait, as_raw_mut_KeyPointsFilter }

impl KeyPointsFilter {
	// KeyPointsFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:95
	// ("cv::KeyPointsFilter::KeyPointsFilter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::features2d::KeyPointsFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_KeyPointsFilter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::features2d::KeyPointsFilter::opencv_from_extern(ret) };
		Ok(ret)
	}

	// runByImageBorder(std::vector<KeyPoint> &, Size, int)(CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:100
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
	// runByKeypointSize(std::vector<KeyPoint> &, float, float)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:104
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
	// cv::KeyPointsFilter::runByKeypointSize(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:104
	// ("cv::KeyPointsFilter::runByKeypointSize", vec![(pred!(mut, ["keypoints", "minSize"], ["std::vector<cv::KeyPoint>*", "float"]), _)]),
	#[inline]
	pub fn run_by_keypoint_size_def(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// runByPixelsMask(std::vector<KeyPoint> &, const Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:109
	// ("cv::KeyPointsFilter::runByPixelsMask", vec![(pred!(mut, ["keypoints", "mask"], ["std::vector<cv::KeyPoint>*", "const cv::Mat*"]), _)]),
	#[inline]
	pub fn run_by_pixels_mask(keypoints: &mut core::Vector<core::KeyPoint>, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByPixelsMask_vectorLKeyPointGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// runByPixelsMask2VectorPoint(std::vector<KeyPoint> &, std::vector<std::vector<Point>> &, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:113
	// ("cv::KeyPointsFilter::runByPixelsMask2VectorPoint", vec![(pred!(mut, ["keypoints", "removeFrom", "mask"], ["std::vector<cv::KeyPoint>*", "std::vector<std::vector<cv::Point>>*", "const cv::Mat*"]), _)]),
	#[inline]
	pub fn run_by_pixels_mask2_vector_point(keypoints: &mut core::Vector<core::KeyPoint>, remove_from: &mut core::Vector<core::Vector<core::Point>>, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_runByPixelsMask2VectorPoint_vectorLKeyPointGR_vectorLvectorLPointGGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), remove_from.as_raw_mut_VectorOfVectorOfPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// removeDuplicated(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:117
	// ("cv::KeyPointsFilter::removeDuplicated", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	pub fn remove_duplicated(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_removeDuplicated_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// removeDuplicatedSorted(std::vector<KeyPoint> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:121
	// ("cv::KeyPointsFilter::removeDuplicatedSorted", vec![(pred!(mut, ["keypoints"], ["std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	pub fn remove_duplicated_sorted(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_KeyPointsFilter_removeDuplicatedSorted_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// retainBest(std::vector<KeyPoint> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:126
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

/// Constant methods for [crate::features2d::MSER]
// MSER /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:507
pub trait MSERTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_MSER(&self) -> *const c_void;

	// getDelta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:538
	// ("cv::MSER::getDelta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_delta(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getDelta_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinArea()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:541
	// ("cv::MSER::getMinArea", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMinArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxArea()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:544
	// ("cv::MSER::getMaxArea", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMaxArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxVariation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:547
	// ("cv::MSER::getMaxVariation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_variation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMaxVariation_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinDiversity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:550
	// ("cv::MSER::getMinDiversity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_diversity(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMinDiversity_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxEvolution()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:553
	// ("cv::MSER::getMaxEvolution", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_evolution(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMaxEvolution_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAreaThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:556
	// ("cv::MSER::getAreaThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_area_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getAreaThreshold_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinMargin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:559
	// ("cv::MSER::getMinMargin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_margin(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getMinMargin_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeBlurSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:562
	// ("cv::MSER::getEdgeBlurSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_edge_blur_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getEdgeBlurSize_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPass2Only()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:565
	// ("cv::MSER::getPass2Only", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pass2_only(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_getPass2Only_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:567
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

/// Mutable methods for [crate::features2d::MSER]
pub trait MSERTrait: crate::features2d::Feature2DTrait + crate::features2d::MSERTraitConst {
	fn as_raw_mut_MSER(&mut self) -> *mut c_void;

	/// Detect %MSER regions
	///
	/// ## Parameters
	/// * image: input image (8UC1, 8UC3 or 8UC4, must be greater or equal than 3x3)
	/// * msers: resulting list of point sets
	/// * bboxes: resulting bounding boxes
	// detectRegions(InputArray, std::vector<std::vector<Point>> &, std::vector<Rect> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:533
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

	// setDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:537
	// ("cv::MSER::setDelta", vec![(pred!(mut, ["delta"], ["int"]), _)]),
	#[inline]
	fn set_delta(&mut self, delta: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setDelta_int(self.as_raw_mut_MSER(), delta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:540
	// ("cv::MSER::setMinArea", vec![(pred!(mut, ["minArea"], ["int"]), _)]),
	#[inline]
	fn set_min_area(&mut self, min_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMinArea_int(self.as_raw_mut_MSER(), min_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:543
	// ("cv::MSER::setMaxArea", vec![(pred!(mut, ["maxArea"], ["int"]), _)]),
	#[inline]
	fn set_max_area(&mut self, max_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMaxArea_int(self.as_raw_mut_MSER(), max_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxVariation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:546
	// ("cv::MSER::setMaxVariation", vec![(pred!(mut, ["maxVariation"], ["double"]), _)]),
	#[inline]
	fn set_max_variation(&mut self, max_variation: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMaxVariation_double(self.as_raw_mut_MSER(), max_variation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDiversity(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:549
	// ("cv::MSER::setMinDiversity", vec![(pred!(mut, ["minDiversity"], ["double"]), _)]),
	#[inline]
	fn set_min_diversity(&mut self, min_diversity: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMinDiversity_double(self.as_raw_mut_MSER(), min_diversity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxEvolution(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:552
	// ("cv::MSER::setMaxEvolution", vec![(pred!(mut, ["maxEvolution"], ["int"]), _)]),
	#[inline]
	fn set_max_evolution(&mut self, max_evolution: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMaxEvolution_int(self.as_raw_mut_MSER(), max_evolution, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setAreaThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:555
	// ("cv::MSER::setAreaThreshold", vec![(pred!(mut, ["areaThreshold"], ["double"]), _)]),
	#[inline]
	fn set_area_threshold(&mut self, area_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setAreaThreshold_double(self.as_raw_mut_MSER(), area_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinMargin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:558
	// ("cv::MSER::setMinMargin", vec![(pred!(mut, ["min_margin"], ["double"]), _)]),
	#[inline]
	fn set_min_margin(&mut self, min_margin: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setMinMargin_double(self.as_raw_mut_MSER(), min_margin, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setEdgeBlurSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:561
	// ("cv::MSER::setEdgeBlurSize", vec![(pred!(mut, ["edge_blur_size"], ["int"]), _)]),
	#[inline]
	fn set_edge_blur_size(&mut self, edge_blur_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_setEdgeBlurSize_int(self.as_raw_mut_MSER(), edge_blur_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPass2Only(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:564
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
/// - the grey image algorithm is taken from: [nister2008linear](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_nister2008linear) ;  the paper claims to be faster
/// than union-find method; it actually get 1.5~2m/s on my centrino L7200 1.2GHz laptop.
///
/// - the color image algorithm is taken from: [forssen2007maximally](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_forssen2007maximally) ; it should be much slower
/// than grey image method ( 3~4 times )
///
/// - (Python) A complete example showing the use of the %MSER detector can be found at samples/python/mser.py
// MSER /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:507
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

impl crate::features2d::Feature2DTraitConst for MSER {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for MSER {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSER, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::MSERTraitConst for MSER {
	#[inline] fn as_raw_MSER(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::MSERTrait for MSER {
	#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSER, crate::features2d::MSERTraitConst, as_raw_MSER, crate::features2d::MSERTrait, as_raw_mut_MSER }

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
	// create(int, int, int, double, double, int, double, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:522
	// ("cv::MSER::create", vec![(pred!(mut, ["delta", "min_area", "max_area", "max_variation", "min_diversity", "max_evolution", "area_threshold", "min_margin", "edge_blur_size"], ["int", "int", "int", "double", "double", "int", "double", "double", "int"]), _)]),
	#[inline]
	pub fn create(delta: i32, min_area: i32, max_area: i32, max_variation: f64, min_diversity: f64, max_evolution: i32, area_threshold: f64, min_margin: f64, edge_blur_size: i32) -> Result<core::Ptr<crate::features2d::MSER>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_create_int_int_int_double_double_int_double_double_int(delta, min_area, max_area, max_variation, min_diversity, max_evolution, area_threshold, min_margin, edge_blur_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::MSER>::opencv_from_extern(ret) };
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
	// cv::MSER::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:522
	// ("cv::MSER::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::MSER>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MSER_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::MSER>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MSER, core::Algorithm, cv_MSER_to_Algorithm }

boxed_cast_base! { MSER, crate::features2d::Feature2D, cv_MSER_to_Feature2D }

impl std::fmt::Debug for MSER {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MSER")
			.finish()
	}
}

/// Constant methods for [crate::features2d::ORB]
// ORB /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:423
pub trait ORBTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_ORB(&self) -> *const c_void;

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:464
	// ("cv::ORB::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getMaxFeatures_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:467
	// ("cv::ORB::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getScaleFactor_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:470
	// ("cv::ORB::getNLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getNLevels_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:473
	// ("cv::ORB::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_edge_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getEdgeThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFirstLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:476
	// ("cv::ORB::getFirstLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_first_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getFirstLevel_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWTA_K()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:479
	// ("cv::ORB::getWTA_K", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_wta_k(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getWTA_K_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScoreType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:482
	// ("cv::ORB::getScoreType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_score_type(&self) -> Result<crate::features2d::ORB_ScoreType> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getScoreType_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPatchSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:485
	// ("cv::ORB::getPatchSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_patch_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getPatchSize_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFastThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:488
	// ("cv::ORB::getFastThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_fast_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_getFastThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:489
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

/// Mutable methods for [crate::features2d::ORB]
pub trait ORBTrait: crate::features2d::Feature2DTrait + crate::features2d::ORBTraitConst {
	fn as_raw_mut_ORB(&mut self) -> *mut c_void;

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:463
	// ("cv::ORB::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	#[inline]
	fn set_max_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setMaxFeatures_int(self.as_raw_mut_ORB(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:466
	// ("cv::ORB::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setScaleFactor_double(self.as_raw_mut_ORB(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:469
	// ("cv::ORB::setNLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
	#[inline]
	fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setNLevels_int(self.as_raw_mut_ORB(), nlevels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setEdgeThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:472
	// ("cv::ORB::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["int"]), _)]),
	#[inline]
	fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setEdgeThreshold_int(self.as_raw_mut_ORB(), edge_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFirstLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:475
	// ("cv::ORB::setFirstLevel", vec![(pred!(mut, ["firstLevel"], ["int"]), _)]),
	#[inline]
	fn set_first_level(&mut self, first_level: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setFirstLevel_int(self.as_raw_mut_ORB(), first_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWTA_K(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:478
	// ("cv::ORB::setWTA_K", vec![(pred!(mut, ["wta_k"], ["int"]), _)]),
	#[inline]
	fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setWTA_K_int(self.as_raw_mut_ORB(), wta_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScoreType(ORB::ScoreType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:481
	// ("cv::ORB::setScoreType", vec![(pred!(mut, ["scoreType"], ["cv::ORB::ScoreType"]), _)]),
	#[inline]
	fn set_score_type(&mut self, score_type: crate::features2d::ORB_ScoreType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setScoreType_ScoreType(self.as_raw_mut_ORB(), score_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:484
	// ("cv::ORB::setPatchSize", vec![(pred!(mut, ["patchSize"], ["int"]), _)]),
	#[inline]
	fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_setPatchSize_int(self.as_raw_mut_ORB(), patch_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFastThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:487
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
/// described in [RRKB11](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_RRKB11) . The algorithm uses FAST in pyramids to detect stable keypoints, selects
/// the strongest features using FAST or Harris response, finds their orientation using first-order
/// moments and computes the descriptors using BRIEF (where the coordinates of random point pairs (or
/// k-tuples) are rotated according to the measured orientation).
// ORB /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:423
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

impl crate::features2d::Feature2DTraitConst for ORB {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for ORB {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ORB, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::ORBTraitConst for ORB {
	#[inline] fn as_raw_ORB(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::ORBTrait for ORB {
	#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ORB, crate::features2d::ORBTraitConst, as_raw_ORB, crate::features2d::ORBTrait, as_raw_mut_ORB }

impl ORB {
	// kBytes /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:427
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
	// create(int, float, int, int, int, int, ORB::ScoreType, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Enum, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:460
	// ("cv::ORB::create", vec![(pred!(mut, ["nfeatures", "scaleFactor", "nlevels", "edgeThreshold", "firstLevel", "WTA_K", "scoreType", "patchSize", "fastThreshold"], ["int", "float", "int", "int", "int", "int", "cv::ORB::ScoreType", "int", "int"]), _)]),
	#[inline]
	pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: crate::features2d::ORB_ScoreType, patch_size: i32, fast_threshold: i32) -> Result<core::Ptr<crate::features2d::ORB>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::ORB>::opencv_from_extern(ret) };
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
	// cv::ORB::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:460
	// ("cv::ORB::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::ORB>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ORB_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::ORB>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ORB, core::Algorithm, cv_ORB_to_Algorithm }

boxed_cast_base! { ORB, crate::features2d::Feature2D, cv_ORB_to_Feature2D }

impl std::fmt::Debug for ORB {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ORB")
			.finish()
	}
}

/// Constant methods for [crate::features2d::SIFT]
// SIFT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:266
pub trait SIFTTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_SIFT(&self) -> *const c_void;

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:329
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

	// getNFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:332
	// ("cv::SIFT::getNFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_features(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getNFeatures_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:335
	// ("cv::SIFT::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getNOctaveLayers_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getContrastThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:338
	// ("cv::SIFT::getContrastThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_contrast_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getContrastThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:341
	// ("cv::SIFT::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_edge_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_getEdgeThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:344
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

/// Mutable methods for [crate::features2d::SIFT]
pub trait SIFTTrait: crate::features2d::Feature2DTrait + crate::features2d::SIFTTraitConst {
	fn as_raw_mut_SIFT(&mut self) -> *mut c_void;

	// setNFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:331
	// ("cv::SIFT::setNFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	#[inline]
	fn set_n_features(&mut self, max_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setNFeatures_int(self.as_raw_mut_SIFT(), max_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:334
	// ("cv::SIFT::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setNOctaveLayers_int(self.as_raw_mut_SIFT(), n_octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setContrastThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:337
	// ("cv::SIFT::setContrastThreshold", vec![(pred!(mut, ["contrastThreshold"], ["double"]), _)]),
	#[inline]
	fn set_contrast_threshold(&mut self, contrast_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setContrastThreshold_double(self.as_raw_mut_SIFT(), contrast_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setEdgeThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:340
	// ("cv::SIFT::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["double"]), _)]),
	#[inline]
	fn set_edge_threshold(&mut self, edge_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_setEdgeThreshold_double(self.as_raw_mut_SIFT(), edge_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:343
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
/// (SIFT) algorithm by D. Lowe [Lowe04](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Lowe04) .
// SIFT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:266
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

impl crate::features2d::Feature2DTraitConst for SIFT {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for SIFT {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SIFT, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::SIFTTraitConst for SIFT {
	#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::SIFTTrait for SIFT {
	#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SIFT, crate::features2d::SIFTTraitConst, as_raw_SIFT, crate::features2d::SIFTTrait, as_raw_mut_SIFT }

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
	// create(int, int, double, double, double, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:294
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "bool"]), _)]),
	#[inline]
	pub fn create(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, enable_precise_upscale: bool) -> Result<core::Ptr<crate::features2d::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double_bool(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, enable_precise_upscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SIFT>::opencv_from_extern(ret) };
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
	// cv::SIFT::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:294
	// ("cv::SIFT::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SIFT>::opencv_from_extern(ret) };
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
	// create(int, int, double, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:325
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType", "enable_precise_upscale"], ["int", "int", "double", "double", "double", "int", "bool"]), _)]),
	#[inline]
	pub fn create_with_descriptor_type(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32, enable_precise_upscale: bool) -> Result<core::Ptr<crate::features2d::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double_int_bool(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, descriptor_type, enable_precise_upscale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SIFT>::opencv_from_extern(ret) };
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
	// cv::SIFT::create(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:325
	// ("cv::SIFT::create", vec![(pred!(mut, ["nfeatures", "nOctaveLayers", "contrastThreshold", "edgeThreshold", "sigma", "descriptorType"], ["int", "int", "double", "double", "double", "int"]), _)]),
	#[inline]
	pub fn create_with_descriptor_type_def(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32) -> Result<core::Ptr<crate::features2d::SIFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SIFT_create_int_int_double_double_double_int(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, descriptor_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SIFT>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SIFT, core::Algorithm, cv_SIFT_to_Algorithm }

boxed_cast_base! { SIFT, crate::features2d::Feature2D, cv_SIFT_to_Feature2D }

impl std::fmt::Debug for SIFT {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SIFT")
			.finish()
	}
}

/// Constant methods for [crate::features2d::SimpleBlobDetector]
// SimpleBlobDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:745
pub trait SimpleBlobDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_SimpleBlobDetector(&self) -> *const c_void;

	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:782
	// ("cv::SimpleBlobDetector::getParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_params(&self) -> Result<crate::features2d::SimpleBlobDetector_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_getParams_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:784
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

	// getBlobContours()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:785
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

/// Mutable methods for [crate::features2d::SimpleBlobDetector]
pub trait SimpleBlobDetectorTrait: crate::features2d::Feature2DTrait + crate::features2d::SimpleBlobDetectorTraitConst {
	fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void;

	// setParams(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:781
	// ("cv::SimpleBlobDetector::setParams", vec![(pred!(mut, ["params"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
	#[inline]
	fn set_params(&mut self, params: crate::features2d::SimpleBlobDetector_Params) -> Result<()> {
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
// SimpleBlobDetector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:745
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

impl crate::features2d::Feature2DTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SimpleBlobDetector, crate::features2d::Feature2DTraitConst, as_raw_Feature2D, crate::features2d::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::features2d::SimpleBlobDetectorTraitConst for SimpleBlobDetector {
	#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::SimpleBlobDetectorTrait for SimpleBlobDetector {
	#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SimpleBlobDetector, crate::features2d::SimpleBlobDetectorTraitConst, as_raw_SimpleBlobDetector, crate::features2d::SimpleBlobDetectorTrait, as_raw_mut_SimpleBlobDetector }

impl SimpleBlobDetector {
	/// ## C++ default parameters
	/// * parameters: SimpleBlobDetector::Params()
	// create(const SimpleBlobDetector::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:779
	// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, ["parameters"], ["const cv::SimpleBlobDetector::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: crate::features2d::SimpleBlobDetector_Params) -> Result<core::Ptr<crate::features2d::SimpleBlobDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SimpleBlobDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [SimpleBlobDetector::create] function uses the following default values for its arguments:
	/// * parameters: SimpleBlobDetector::Params()
	// cv::SimpleBlobDetector::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:779
	// ("cv::SimpleBlobDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::features2d::SimpleBlobDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::SimpleBlobDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SimpleBlobDetector, core::Algorithm, cv_SimpleBlobDetector_to_Algorithm }

boxed_cast_base! { SimpleBlobDetector, crate::features2d::Feature2D, cv_SimpleBlobDetector_to_Feature2D }

impl std::fmt::Debug for SimpleBlobDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SimpleBlobDetector")
			.finish()
	}
}

// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:748
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

opencv_type_simple! { crate::features2d::SimpleBlobDetector_Params }

impl SimpleBlobDetector_Params {
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:775
	// ("cv::SimpleBlobDetector::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	pub fn write(self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_write_const_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:750
	// ("cv::SimpleBlobDetector::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::features2d::SimpleBlobDetector_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SimpleBlobDetector_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/features2d.hpp:774
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
