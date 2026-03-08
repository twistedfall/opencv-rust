pub mod features {
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
		pub use super::{ALIKEDTrait, ALIKEDTraitConst, ANNIndexTrait, ANNIndexTraitConst, AffineFeatureTrait, AffineFeatureTraitConst, BFMatcherTrait, BFMatcherTraitConst, DISKTrait, DISKTraitConst, DescriptorMatcherTrait, DescriptorMatcherTraitConst, FastFeatureDetectorTrait, FastFeatureDetectorTraitConst, Feature2DTrait, Feature2DTraitConst, FlannBasedMatcherTrait, FlannBasedMatcherTraitConst, GFTTDetectorTrait, GFTTDetectorTraitConst, KeyPointsFilterTrait, KeyPointsFilterTraitConst, LightGlueMatcherTrait, LightGlueMatcherTraitConst, MSERTrait, MSERTraitConst, ORBTrait, ORBTraitConst, SIFTTrait, SIFTTraitConst, SimpleBlobDetectorTrait, SimpleBlobDetectorTraitConst};
	}

	pub const ANNIndex_DIST_ANGULAR: i32 = 2;
	pub const ANNIndex_DIST_DOTPRODUCT: i32 = 4;
	pub const ANNIndex_DIST_EUCLIDEAN: i32 = 0;
	pub const ANNIndex_DIST_HAMMING: i32 = 3;
	pub const ANNIndex_DIST_MANHATTAN: i32 = 1;
	pub const DescriptorMatcher_BRUTEFORCE: i32 = 2;
	pub const DescriptorMatcher_BRUTEFORCE_HAMMING: i32 = 4;
	pub const DescriptorMatcher_BRUTEFORCE_HAMMINGLUT: i32 = 5;
	pub const DescriptorMatcher_BRUTEFORCE_L1: i32 = 3;
	pub const DescriptorMatcher_BRUTEFORCE_SL2: i32 = 6;
	pub const DescriptorMatcher_FLANNBASED: i32 = 1;
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
	pub const FastFeatureDetector_TYPE_5_8: i32 = 0;
	pub const FastFeatureDetector_TYPE_7_12: i32 = 1;
	pub const FastFeatureDetector_TYPE_9_16: i32 = 2;
	pub const ORB_FAST_SCORE: i32 = 1;
	pub const ORB_HARRIS_SCORE: i32 = 0;
	/// Metrics used to calculate the distance between two feature vectors.
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ANNIndex_Distance {
		DIST_EUCLIDEAN = 0,
		DIST_MANHATTAN = 1,
		DIST_ANGULAR = 2,
		DIST_HAMMING = 3,
		DIST_DOTPRODUCT = 4,
	}

	opencv_type_enum! { crate::features::ANNIndex_Distance { DIST_EUCLIDEAN, DIST_MANHATTAN, DIST_ANGULAR, DIST_HAMMING, DIST_DOTPRODUCT } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum DescriptorMatcher_MatcherType {
		FLANNBASED = 1,
		BRUTEFORCE = 2,
		BRUTEFORCE_L1 = 3,
		BRUTEFORCE_HAMMING = 4,
		BRUTEFORCE_HAMMINGLUT = 5,
		BRUTEFORCE_SL2 = 6,
	}

	opencv_type_enum! { crate::features::DescriptorMatcher_MatcherType { FLANNBASED, BRUTEFORCE, BRUTEFORCE_L1, BRUTEFORCE_HAMMING, BRUTEFORCE_HAMMINGLUT, BRUTEFORCE_SL2 } }

	#[repr(transparent)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub struct DrawMatchesFlags(i32);

	impl DrawMatchesFlags {
		/// Output image matrix will be created (Mat::create),
		/// i.e. existing memory of output image may be reused.
		/// Two source image, matches and single keypoints will be drawn.
		/// For each keypoint only the center point will be drawn (without
		/// the circle around keypoint with keypoint size and orientation).
		pub const DEFAULT: Self = Self(0);
		/// Output image matrix will not be created (Mat::create).
		/// Matches will be drawn on existing content of output image.
		pub const DRAW_OVER_OUTIMG: Self = Self(1);
		/// Single keypoints will not be drawn.
		pub const NOT_DRAW_SINGLE_POINTS: Self = Self(2);
		/// For each keypoint the circle around keypoint with keypoint size and
		/// orientation will be drawn.
		pub const DRAW_RICH_KEYPOINTS: Self = Self(4);
	}

	opencv_type_bitfield_enum! { crate::features::DrawMatchesFlags { DEFAULT, DRAW_OVER_OUTIMG, NOT_DRAW_SINGLE_POINTS, DRAW_RICH_KEYPOINTS } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum FastFeatureDetector_DetectorType {
		TYPE_5_8 = 0,
		TYPE_7_12 = 1,
		TYPE_9_16 = 2,
	}

	opencv_type_enum! { crate::features::FastFeatureDetector_DetectorType { TYPE_5_8, TYPE_7_12, TYPE_9_16 } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ORB_ScoreType {
		HARRIS_SCORE = 0,
		FAST_SCORE = 1,
	}

	opencv_type_enum! { crate::features::ORB_ScoreType { HARRIS_SCORE, FAST_SCORE } }

	pub type AffineDescriptorExtractor = crate::features::AffineFeature;
	pub type AffineFeatureDetector = crate::features::AffineFeature;
	/// Extractors of keypoint descriptors in OpenCV have wrappers with a common interface that enables you
	/// to easily switch between different algorithms solving the same problem. This section is devoted to
	/// computing descriptors represented as vectors in a multidimensional space. All objects that implement
	/// the vector descriptor extractors inherit the DescriptorExtractor interface.
	pub type DescriptorExtractor = crate::features::Feature2D;
	/// Feature detectors in OpenCV have wrappers with a common interface that enables you to easily switch
	/// between different algorithms solving the same problem. All objects that implement keypoint detectors
	/// inherit the FeatureDetector interface.
	pub type FeatureDetector = crate::features::Feature2D;
	pub type SiftDescriptorExtractor = crate::features::SIFT;
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
	#[inline]
	pub fn fast_with_type_def(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FAST_const__InputArrayR_vectorLKeyPointGR_int(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn fast_with_type(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features::FastFeatureDetector_DetectorType) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	#[inline]
	pub fn compute_recall_precision_curve(matches1to2: &core::Vector<core::Vector<core::DMatch>>, correct_matches1to2_mask: &core::Vector<core::Vector<u8>>, recall_precision_curve: &mut core::Vector<core::Point2f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_computeRecallPrecisionCurve_const_vectorLvectorLDMatchGGR_const_vectorLvectorLunsigned_charGGR_vectorLPoint2fGR(matches1to2.as_raw_VectorOfVectorOfDMatch(), correct_matches1to2_mask.as_raw_VectorOfVectorOfu8(), recall_precision_curve.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_keypoints_def(image: &impl ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(image);
		input_output_array_arg!(out_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR(image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), out_image.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_keypoints(image: &impl ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut impl ToInputOutputArray, color: core::Scalar, flags: crate::features::DrawMatchesFlags) -> Result<()> {
		input_array_arg!(image);
		input_output_array_arg!(out_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawKeypoints_const__InputArrayR_const_vectorLKeyPointGR_const__InputOutputArrayR_const_ScalarR_DrawMatchesFlags(image.as_raw__InputArray(), keypoints.as_raw_VectorOfKeyPoint(), out_image.as_raw__InputOutputArray(), &color, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_matches_def(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_output_array_arg!(out_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_matches(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features::DrawMatchesFlags) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_output_array_arg!(out_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	///
	/// ## Note
	/// This alternative version of [draw_matches_with_thickness] function uses the following default values for its arguments:
	/// * match_color: Scalar::all(-1)
	/// * single_point_color: Scalar::all(-1)
	/// * matches_mask: std::vector<char>()
	/// * flags: DrawMatchesFlags::DEFAULT
	#[inline]
	pub fn draw_matches_with_thickness_def(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, matches_thickness: i32) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_output_array_arg!(out_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), matches_thickness, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	#[inline]
	pub fn draw_matches_with_thickness(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl ToInputOutputArray, matches_thickness: i32, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features::DrawMatchesFlags) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_output_array_arg!(out_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLDMatchGR_const__InputOutputArrayR_const_int_const_ScalarR_const_ScalarR_const_vectorLcharGR_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw__InputOutputArray(), matches_thickness, &match_color, &single_point_color, matches_mask.as_raw_VectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [draw_matches_knn] function uses the following default values for its arguments:
	/// * match_color: Scalar::all(-1)
	/// * single_point_color: Scalar::all(-1)
	/// * matches_mask: std::vector<std::vector<char>>()
	/// * flags: DrawMatchesFlags::DEFAULT
	#[inline]
	pub fn draw_matches_knn_def(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut impl ToInputOutputArray) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_output_array_arg!(out_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * match_color: Scalar::all(-1)
	/// * single_point_color: Scalar::all(-1)
	/// * matches_mask: std::vector<std::vector<char>>()
	/// * flags: DrawMatchesFlags::DEFAULT
	#[inline]
	pub fn draw_matches_knn(img1: &impl ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut impl ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<core::Vector<c_char>>, flags: crate::features::DrawMatchesFlags) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_output_array_arg!(out_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLvectorLcharGGR_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfVectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [evaluate_feature_detector] function uses the following default values for its arguments:
	/// * fdetector: Ptr<FeatureDetector>()
	#[inline]
	pub fn evaluate_feature_detector_def(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, h1to2: &impl core::MatTraitConst, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * fdetector: Ptr<FeatureDetector>()
	#[inline]
	pub fn evaluate_feature_detector(img1: &impl core::MatTraitConst, img2: &impl core::MatTraitConst, h1to2: &impl core::MatTraitConst, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32, fdetector: &core::Ptr<crate::features::Feature2D>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR_const_PtrLFeature2DGR(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, fdetector.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	#[inline]
	pub fn get_nearest_point(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getNearestPoint_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	#[inline]
	pub fn get_recall(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getRecall_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Determines strong corners on an image.
	///
	/// The function finds the most prominent corners in the image or in the specified image region, as
	/// described in [Shi94](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Shi94)
	///
	/// *   Function calculates the corner quality measure at every source image pixel using the
	///    [corner_min_eigen_val] or [corner_harris] .
	/// *   Function performs a non-maximum suppression (the local maximums in *3 x 3* neighborhood are
	///    retained).
	/// *   The corners with the minimal eigenvalue less than
	///    ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BqualityLevel%7D%20%5Ccdot%20%5Cmax%5F%7Bx%2Cy%7D%20qualityMeasureMap%28x%2Cy%29) are rejected.
	/// *   The remaining corners are sorted by the quality measure in the descending order.
	/// *   Function throws away each corner for which there is a stronger corner at a distance less than
	///    maxDistance.
	///
	/// The function can be used to initialize a point-based tracker of an object.
	///
	///
	/// Note: If the function is called with different values A and B of the parameter qualityLevel , and
	/// A \> B, the vector of returned corners with qualityLevel=A will be the prefix of the output vector
	/// with qualityLevel=B .
	///
	/// ## Parameters
	/// * image: Input 8-bit or floating-point 32-bit, single-channel image.
	/// * corners: Output vector of detected corners.
	/// * maxCorners: Maximum number of corners to return. If there are more corners than are found,
	/// the strongest of them is returned. `maxCorners <= 0` implies that no limit on the maximum is set
	/// and all detected corners are returned.
	/// * qualityLevel: Parameter characterizing the minimal accepted quality of image corners. The
	/// parameter value is multiplied by the best corner quality measure, which is the minimal eigenvalue
	/// (see [corner_min_eigen_val] ) or the Harris function response (see [corner_harris] ). The corners with the
	/// quality measure less than the product are rejected. For example, if the best corner has the
	/// quality measure = 1500, and the qualityLevel=0.01 , then all the corners with the quality measure
	/// less than 15 are rejected.
	/// * minDistance: Minimum possible Euclidean distance between the returned corners.
	/// * mask: Optional region of interest. If the image is not empty (it needs to have the type
	/// CV_8UC1 and the same size as image ), it specifies the region in which the corners are detected.
	/// * blockSize: Size of an average block for computing a derivative covariation matrix over each
	/// pixel neighborhood. See cornerEigenValsAndVecs .
	/// * useHarrisDetector: Parameter indicating whether to use a Harris detector (see #cornerHarris)
	/// or #cornerMinEigenVal.
	/// * k: Free parameter of the Harris detector.
	/// ## See also
	/// cornerMinEigenVal, cornerHarris, calcOpticalFlowPyrLK, estimateRigidTransform,
	///
	/// ## Note
	/// This alternative version of [good_features_to_track] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn good_features_to_track_def(image: &impl ToInputArray, corners: &mut impl ToOutputArray, max_corners: i32, quality_level: f64, min_distance: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double(image.as_raw__InputArray(), corners.as_raw__OutputArray(), max_corners, quality_level, min_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Same as above, but returns also quality measure of the detected corners.
	///
	/// ## Parameters
	/// * image: Input 8-bit or floating-point 32-bit, single-channel image.
	/// * corners: Output vector of detected corners.
	/// * maxCorners: Maximum number of corners to return. If there are more corners than are found,
	/// the strongest of them is returned. `maxCorners <= 0` implies that no limit on the maximum is set
	/// and all detected corners are returned.
	/// * qualityLevel: Parameter characterizing the minimal accepted quality of image corners. The
	/// parameter value is multiplied by the best corner quality measure, which is the minimal eigenvalue
	/// (see [corner_min_eigen_val] ) or the Harris function response (see [corner_harris] ). The corners with the
	/// quality measure less than the product are rejected. For example, if the best corner has the
	/// quality measure = 1500, and the qualityLevel=0.01 , then all the corners with the quality measure
	/// less than 15 are rejected.
	/// * minDistance: Minimum possible Euclidean distance between the returned corners.
	/// * mask: Region of interest. If the image is not empty (it needs to have the type
	/// CV_8UC1 and the same size as image ), it specifies the region in which the corners are detected.
	/// * cornersQuality: Output vector of quality measure of the detected corners.
	/// * blockSize: Size of an average block for computing a derivative covariation matrix over each
	/// pixel neighborhood. See cornerEigenValsAndVecs .
	/// * gradientSize: Aperture parameter for the Sobel operator used for derivatives computation.
	/// See cornerEigenValsAndVecs .
	/// * useHarrisDetector: Parameter indicating whether to use a Harris detector (see #cornerHarris)
	/// or #cornerMinEigenVal.
	/// * k: Free parameter of the Harris detector.
	///
	/// ## Note
	/// This alternative version of [good_features_to_track_with_quality] function uses the following default values for its arguments:
	/// * block_size: 3
	/// * gradient_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn good_features_to_track_with_quality_def(image: &impl ToInputArray, corners: &mut impl ToOutputArray, max_corners: i32, quality_level: f64, min_distance: f64, mask: &impl ToInputArray, corners_quality: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		input_array_arg!(mask);
		output_array_arg!(corners_quality);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), corners.as_raw__OutputArray(), max_corners, quality_level, min_distance, mask.as_raw__InputArray(), corners_quality.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Same as above, but returns also quality measure of the detected corners.
	///
	/// ## Parameters
	/// * image: Input 8-bit or floating-point 32-bit, single-channel image.
	/// * corners: Output vector of detected corners.
	/// * maxCorners: Maximum number of corners to return. If there are more corners than are found,
	/// the strongest of them is returned. `maxCorners <= 0` implies that no limit on the maximum is set
	/// and all detected corners are returned.
	/// * qualityLevel: Parameter characterizing the minimal accepted quality of image corners. The
	/// parameter value is multiplied by the best corner quality measure, which is the minimal eigenvalue
	/// (see [corner_min_eigen_val] ) or the Harris function response (see [corner_harris] ). The corners with the
	/// quality measure less than the product are rejected. For example, if the best corner has the
	/// quality measure = 1500, and the qualityLevel=0.01 , then all the corners with the quality measure
	/// less than 15 are rejected.
	/// * minDistance: Minimum possible Euclidean distance between the returned corners.
	/// * mask: Region of interest. If the image is not empty (it needs to have the type
	/// CV_8UC1 and the same size as image ), it specifies the region in which the corners are detected.
	/// * cornersQuality: Output vector of quality measure of the detected corners.
	/// * blockSize: Size of an average block for computing a derivative covariation matrix over each
	/// pixel neighborhood. See cornerEigenValsAndVecs .
	/// * gradientSize: Aperture parameter for the Sobel operator used for derivatives computation.
	/// See cornerEigenValsAndVecs .
	/// * useHarrisDetector: Parameter indicating whether to use a Harris detector (see #cornerHarris)
	/// or #cornerMinEigenVal.
	/// * k: Free parameter of the Harris detector.
	///
	/// ## C++ default parameters
	/// * block_size: 3
	/// * gradient_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn good_features_to_track_with_quality(image: &impl ToInputArray, corners: &mut impl ToOutputArray, max_corners: i32, quality_level: f64, min_distance: f64, mask: &impl ToInputArray, corners_quality: &mut impl ToOutputArray, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		input_array_arg!(mask);
		output_array_arg!(corners_quality);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR_int_int_bool_double(image.as_raw__InputArray(), corners.as_raw__OutputArray(), max_corners, quality_level, min_distance, mask.as_raw__InputArray(), corners_quality.as_raw__OutputArray(), block_size, gradient_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Determines strong corners on an image.
	///
	/// The function finds the most prominent corners in the image or in the specified image region, as
	/// described in [Shi94](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Shi94)
	///
	/// *   Function calculates the corner quality measure at every source image pixel using the
	///    [corner_min_eigen_val] or [corner_harris] .
	/// *   Function performs a non-maximum suppression (the local maximums in *3 x 3* neighborhood are
	///    retained).
	/// *   The corners with the minimal eigenvalue less than
	///    ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BqualityLevel%7D%20%5Ccdot%20%5Cmax%5F%7Bx%2Cy%7D%20qualityMeasureMap%28x%2Cy%29) are rejected.
	/// *   The remaining corners are sorted by the quality measure in the descending order.
	/// *   Function throws away each corner for which there is a stronger corner at a distance less than
	///    maxDistance.
	///
	/// The function can be used to initialize a point-based tracker of an object.
	///
	///
	/// Note: If the function is called with different values A and B of the parameter qualityLevel , and
	/// A \> B, the vector of returned corners with qualityLevel=A will be the prefix of the output vector
	/// with qualityLevel=B .
	///
	/// ## Parameters
	/// * image: Input 8-bit or floating-point 32-bit, single-channel image.
	/// * corners: Output vector of detected corners.
	/// * maxCorners: Maximum number of corners to return. If there are more corners than are found,
	/// the strongest of them is returned. `maxCorners <= 0` implies that no limit on the maximum is set
	/// and all detected corners are returned.
	/// * qualityLevel: Parameter characterizing the minimal accepted quality of image corners. The
	/// parameter value is multiplied by the best corner quality measure, which is the minimal eigenvalue
	/// (see [corner_min_eigen_val] ) or the Harris function response (see [corner_harris] ). The corners with the
	/// quality measure less than the product are rejected. For example, if the best corner has the
	/// quality measure = 1500, and the qualityLevel=0.01 , then all the corners with the quality measure
	/// less than 15 are rejected.
	/// * minDistance: Minimum possible Euclidean distance between the returned corners.
	/// * mask: Optional region of interest. If the image is not empty (it needs to have the type
	/// CV_8UC1 and the same size as image ), it specifies the region in which the corners are detected.
	/// * blockSize: Size of an average block for computing a derivative covariation matrix over each
	/// pixel neighborhood. See cornerEigenValsAndVecs .
	/// * useHarrisDetector: Parameter indicating whether to use a Harris detector (see #cornerHarris)
	/// or #cornerMinEigenVal.
	/// * k: Free parameter of the Harris detector.
	/// ## See also
	/// cornerMinEigenVal, cornerHarris, calcOpticalFlowPyrLK, estimateRigidTransform,
	///
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * block_size: 3
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn good_features_to_track(image: &impl ToInputArray, corners: &mut impl ToOutputArray, max_corners: i32, quality_level: f64, min_distance: f64, mask: &impl ToInputArray, block_size: i32, use_harris_detector: bool, k: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_bool_double(image.as_raw__InputArray(), corners.as_raw__OutputArray(), max_corners, quality_level, min_distance, mask.as_raw__InputArray(), block_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [good_features_to_track_1] function uses the following default values for its arguments:
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn good_features_to_track_1_def(image: &impl ToInputArray, corners: &mut impl ToOutputArray, max_corners: i32, quality_level: f64, min_distance: f64, mask: &impl ToInputArray, block_size: i32, gradient_size: i32) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int(image.as_raw__InputArray(), corners.as_raw__OutputArray(), max_corners, quality_level, min_distance, mask.as_raw__InputArray(), block_size, gradient_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_harris_detector: false
	/// * k: 0.04
	#[inline]
	pub fn good_features_to_track_1(image: &impl ToInputArray, corners: &mut impl ToOutputArray, max_corners: i32, quality_level: f64, min_distance: f64, mask: &impl ToInputArray, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(corners);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int_bool_double(image.as_raw__InputArray(), corners.as_raw__OutputArray(), max_corners, quality_level, min_distance, mask.as_raw__InputArray(), block_size, gradient_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ALIKED feature detector and descriptor extractor.
	///
	/// ALIKED (A Lightweight Image KEYpoint Detector) is a CNN-based feature detector and descriptor
	/// extractor, as described in [Zhao23](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Zhao23) . It produces 128-dimensional float descriptors and
	/// keypoints with sub-pixel accuracy.
	///
	/// The model expects RGB input [1,3,H,W] and internally converts BGR images to RGB.
	pub struct ALIKED {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { ALIKED }

	impl Drop for ALIKED {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ALIKED_delete(self.as_raw_mut_ALIKED()) };
		}
	}

	unsafe impl Send for ALIKED {}

	impl ALIKED {
		/// Creates ALIKED from a model file path.
		/// ## Parameters
		/// * modelPath: Path to the ONNX model file.
		/// * params: ALIKED parameters.
		///
		/// ## C++ default parameters
		/// * params: ALIKED::Params()
		#[inline]
		pub fn create(model_path: &str, params: crate::features::ALIKED_Params) -> Result<core::Ptr<crate::features::ALIKED>> {
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ALIKED_create_const_StringR_const_ParamsR(model_path.opencv_as_extern(), &params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::ALIKED>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates ALIKED from a model file path.
		/// ## Parameters
		/// * modelPath: Path to the ONNX model file.
		/// * params: ALIKED parameters.
		///
		/// ## Note
		/// This alternative version of [ALIKED::create] function uses the following default values for its arguments:
		/// * params: ALIKED::Params()
		#[inline]
		pub fn create_def(model_path: &str) -> Result<core::Ptr<crate::features::ALIKED>> {
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ALIKED_create_const_StringR(model_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::ALIKED>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates ALIKED from in-memory model data.
		/// ## Parameters
		/// * modelData: Buffer containing the model data.
		/// * params: ALIKED parameters.
		///
		/// ## C++ default parameters
		/// * params: ALIKED::Params()
		#[inline]
		pub fn create_1(model_data: &core::Vector<u8>, params: crate::features::ALIKED_Params) -> Result<core::Ptr<crate::features::ALIKED>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ALIKED_create_const_vectorLunsigned_charGR_const_ParamsR(model_data.as_raw_VectorOfu8(), &params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::ALIKED>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates ALIKED from in-memory model data.
		/// ## Parameters
		/// * modelData: Buffer containing the model data.
		/// * params: ALIKED parameters.
		///
		/// ## Note
		/// This alternative version of [ALIKED::create] function uses the following default values for its arguments:
		/// * params: ALIKED::Params()
		#[inline]
		pub fn create_def_1(model_data: &core::Vector<u8>) -> Result<core::Ptr<crate::features::ALIKED>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ALIKED_create_const_vectorLunsigned_charGR(model_data.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::ALIKED>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::ALIKED]
	pub trait ALIKEDTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_ALIKED(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::features::ALIKED]
	pub trait ALIKEDTrait: crate::features::ALIKEDTraitConst + crate::features::Feature2DTrait {
		fn as_raw_mut_ALIKED(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for ALIKED {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ALIKED")
				.finish()
		}
	}

	boxed_cast_base! { ALIKED, core::Algorithm, cv_ALIKED_to_Algorithm }

	boxed_cast_base! { ALIKED, crate::features::Feature2D, cv_ALIKED_to_Feature2D }

	impl core::AlgorithmTraitConst for ALIKED {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for ALIKED {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ALIKED, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::features::Feature2DTraitConst for ALIKED {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::Feature2DTrait for ALIKED {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ALIKED, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

	impl crate::features::ALIKEDTraitConst for ALIKED {
		#[inline] fn as_raw_ALIKED(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::ALIKEDTrait for ALIKED {
		#[inline] fn as_raw_mut_ALIKED(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ALIKED, crate::features::ALIKEDTraitConst, as_raw_ALIKED, crate::features::ALIKEDTrait, as_raw_mut_ALIKED }

	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct ALIKED_Params {
		/// Input image size for the network, default 640x640
		pub input_size: core::Size,
		/// Whether to L2-normalize descriptors, default true
		pub normalize_descriptors: bool,
		/// DNN engine type (dnn::EngineType), default ENGINE_NEW
		pub engine: i32,
		/// DNN backend, default DNN_BACKEND_DEFAULT
		pub backend: i32,
		/// DNN target, default DNN_TARGET_CPU
		pub target: i32,
	}

	opencv_type_simple! { crate::features::ALIKED_Params }

	impl ALIKED_Params {
		#[inline]
		pub fn default() -> Result<crate::features::ALIKED_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ALIKED_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

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
		#[inline]
		pub fn create(dim: i32, dist_type: crate::features::ANNIndex_Distance) -> Result<core::Ptr<crate::features::ANNIndex>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_create_int_Distance(dim, dist_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def(dim: i32) -> Result<core::Ptr<crate::features::ANNIndex>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_create_int(dim, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::ANNIndex>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::ANNIndex]
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
		#[inline]
		fn add_items(&mut self, features: &impl ToInputArray) -> Result<()> {
			input_array_arg!(features);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_addItems_const__InputArrayR(self.as_raw_mut_ANNIndex(), features.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn build(&mut self, trees: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_build_int(self.as_raw_mut_ANNIndex(), trees, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn build_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_build(self.as_raw_mut_ANNIndex(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn knn_search(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, knn: i32, search_k: i32) -> Result<()> {
			input_array_arg!(query);
			output_array_arg!(indices);
			output_array_arg!(dists);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(self.as_raw_mut_ANNIndex(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, search_k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn knn_search_def(&mut self, query: &impl ToInputArray, indices: &mut impl ToOutputArray, dists: &mut impl ToOutputArray, knn: i32) -> Result<()> {
			input_array_arg!(query);
			output_array_arg!(indices);
			output_array_arg!(dists);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(self.as_raw_mut_ANNIndex(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn save(&mut self, filename: &str, prefault: bool) -> Result<()> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_save_const_StringR_bool(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), prefault, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn save_def(&mut self, filename: &str) -> Result<()> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_save_const_StringR(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn load(&mut self, filename: &str, prefault: bool) -> Result<()> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_load_const_StringR_bool(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), prefault, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn load_def(&mut self, filename: &str) -> Result<()> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_load_const_StringR(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Return the number of trees in the index.
		#[inline]
		fn get_tree_number(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_getTreeNumber(self.as_raw_mut_ANNIndex(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Return the number of feature vectors in the index.
		#[inline]
		fn get_item_number(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_getItemNumber(self.as_raw_mut_ANNIndex(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Prepare to build the index in the specified file instead of RAM (execute before adding
		/// items, no need to save after build)
		///
		/// ## Parameters
		/// * filename: Filename of the index to be built.
		#[inline]
		fn set_on_disk_build(&mut self, filename: &str) -> Result<bool> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_setOnDiskBuild_const_StringR(self.as_raw_mut_ANNIndex(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Initialize the random number generator with the given seed. Only necessary to pass this
		/// before adding the items. Will have no effect after calling build() or load().
		///
		/// ## Parameters
		/// * seed: The given seed of the random number generator. Its value should be within the range of uint32_t.
		#[inline]
		fn set_seed(&mut self, seed: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ANNIndex_setSeed_int(self.as_raw_mut_ANNIndex(), seed, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
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

	impl crate::features::ANNIndexTraitConst for ANNIndex {
		#[inline] fn as_raw_ANNIndex(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::ANNIndexTrait for ANNIndex {
		#[inline] fn as_raw_mut_ANNIndex(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ANNIndex, crate::features::ANNIndexTraitConst, as_raw_ANNIndex, crate::features::ANNIndexTrait, as_raw_mut_ANNIndex }

	/// Class for implementing the wrapper which makes detectors and extractors to be affine invariant,
	/// described as ASIFT in [YM11](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_YM11) .
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
		#[inline]
		pub fn create(backend: &core::Ptr<crate::features::Feature2D>, max_tilt: i32, min_tilt: i32, tilt_step: f32, rotate_step_base: f32) -> Result<core::Ptr<crate::features::AffineFeature>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AffineFeature_create_const_PtrLFeature2DGR_int_int_float_float(backend.as_raw_PtrOfFeature2D(), max_tilt, min_tilt, tilt_step, rotate_step_base, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def(backend: &core::Ptr<crate::features::Feature2D>) -> Result<core::Ptr<crate::features::AffineFeature>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AffineFeature_create_const_PtrLFeature2DGR(backend.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::AffineFeature>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::AffineFeature]
	pub trait AffineFeatureTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_AffineFeature(&self) -> *const c_void;

		#[inline]
		fn get_view_params(&self, tilts: &mut core::Vector<f32>, rolls: &mut core::Vector<f32>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AffineFeature_getViewParams_const_vectorLfloatGR_vectorLfloatGR(self.as_raw_AffineFeature(), tilts.as_raw_mut_VectorOff32(), rolls.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AffineFeature_getDefaultName_const(self.as_raw_AffineFeature(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::AffineFeature]
	pub trait AffineFeatureTrait: crate::features::AffineFeatureTraitConst + crate::features::Feature2DTrait {
		fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void;

		#[inline]
		fn set_view_params(&mut self, tilts: &core::Vector<f32>, rolls: &core::Vector<f32>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AffineFeature_setViewParams_const_vectorLfloatGR_const_vectorLfloatGR(self.as_raw_mut_AffineFeature(), tilts.as_raw_VectorOff32(), rolls.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for AffineFeature {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AffineFeature")
				.finish()
		}
	}

	boxed_cast_base! { AffineFeature, core::Algorithm, cv_AffineFeature_to_Algorithm }

	boxed_cast_base! { AffineFeature, crate::features::Feature2D, cv_AffineFeature_to_Feature2D }

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

	/// Brute-force descriptor matcher.
	///
	/// For each descriptor in the first set, this matcher finds the closest descriptor in the second set
	/// by trying each one. This descriptor matcher supports masking permissible matches of descriptor
	/// sets.
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

	impl BFMatcher {
		/// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
		///
		/// ## C++ default parameters
		/// * norm_type: NORM_L2
		/// * cross_check: false
		#[inline]
		pub fn new(norm_type: i32, cross_check: bool) -> Result<crate::features::BFMatcher> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BFMatcher_BFMatcher_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn new_def() -> Result<crate::features::BFMatcher> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BFMatcher_BFMatcher(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create(norm_type: i32, cross_check: bool) -> Result<core::Ptr<crate::features::BFMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BFMatcher_create_int_bool(norm_type, cross_check, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::features::BFMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BFMatcher_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::BFMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::BFMatcher]
	pub trait BFMatcherTraitConst: crate::features::DescriptorMatcherTraitConst {
		fn as_raw_BFMatcher(&self) -> *const c_void;

		#[inline]
		fn is_mask_supported(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BFMatcher_isMaskSupported_const(self.as_raw_BFMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## C++ default parameters
		/// * empty_train_data: false
		#[inline]
		#[must_use]
		fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BFMatcher_clone_const_bool(self.as_raw_BFMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [BFMatcherTraitConst::clone] function uses the following default values for its arguments:
		/// * empty_train_data: false
		#[inline]
		fn clone_def(&self) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BFMatcher_clone_const(self.as_raw_BFMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::BFMatcher]
	pub trait BFMatcherTrait: crate::features::BFMatcherTraitConst + crate::features::DescriptorMatcherTrait {
		fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for BFMatcher {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BFMatcher")
				.finish()
		}
	}

	boxed_cast_base! { BFMatcher, core::Algorithm, cv_BFMatcher_to_Algorithm }

	boxed_cast_base! { BFMatcher, crate::features::DescriptorMatcher, cv_BFMatcher_to_DescriptorMatcher }

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

	/// DISK feature detector and descriptor, based on a DNN model.
	///
	/// DISK (Deep Image Structure and Keypoints) is a learned local-feature pipeline that produces
	/// keypoints and 128-D L2-normalized descriptors via a single forward pass through a fully
	/// convolutional network. This class wraps an ONNX export of the pre-trained DISK model through
	/// cv::dnn::Net and exposes it under the standard cv::Feature2D interface so it can be used as
	/// a drop-in alternative to SIFT/ORB.
	///
	/// The class assumes the ONNX model has a single input named `image` taking an N×3×H×W float32
	/// tensor in [0, 1] (RGB channel order) and three outputs named `keypoints` (N×2), `scores` (N)
	/// and `descriptors` (N×128).
	pub struct DISK {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { DISK }

	impl Drop for DISK {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_DISK_delete(self.as_raw_mut_DISK()) };
		}
	}

	unsafe impl Send for DISK {}

	impl DISK {
		/// Creates a DISK detector.
		/// ## Parameters
		/// * modelPath: Path to the DISK ONNX model.
		/// * maxKeypoints: Maximum number of keypoints to return per image. The strongest
		///                    responses (by network score) are kept; -1 keeps all detections.
		/// * scoreThreshold: Discard keypoints with network score strictly below this value.
		/// * imageSize: Target input size (width, height) fed to the network. Use Size()
		///                  (the default) to fall back to the network's expected fixed input
		///                  shape of 1024x1024. When overriding, both dimensions must be
		///                  positive multiples of 16, since DISK downsamples by a factor of 16.
		/// * backendId: DNN backend identifier (see cv::dnn::Backend); 0 = DNN_BACKEND_DEFAULT.
		/// * targetId: DNN target identifier (see cv::dnn::Target);  0 = DNN_TARGET_CPU.
		///
		/// ## C++ default parameters
		/// * max_keypoints: -1
		/// * score_threshold: 0.0f
		/// * image_size: Size()
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create(model_path: &str, max_keypoints: i32, score_threshold: f32, image_size: core::Size, backend_id: i32, target_id: i32) -> Result<core::Ptr<crate::features::DISK>> {
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_create_const_StringR_int_float_const_SizeR_int_int(model_path.opencv_as_extern(), max_keypoints, score_threshold, &image_size, backend_id, target_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DISK>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates a DISK detector.
		/// ## Parameters
		/// * modelPath: Path to the DISK ONNX model.
		/// * maxKeypoints: Maximum number of keypoints to return per image. The strongest
		///                    responses (by network score) are kept; -1 keeps all detections.
		/// * scoreThreshold: Discard keypoints with network score strictly below this value.
		/// * imageSize: Target input size (width, height) fed to the network. Use Size()
		///                  (the default) to fall back to the network's expected fixed input
		///                  shape of 1024x1024. When overriding, both dimensions must be
		///                  positive multiples of 16, since DISK downsamples by a factor of 16.
		/// * backendId: DNN backend identifier (see cv::dnn::Backend); 0 = DNN_BACKEND_DEFAULT.
		/// * targetId: DNN target identifier (see cv::dnn::Target);  0 = DNN_TARGET_CPU.
		///
		/// ## Note
		/// This alternative version of [DISK::create] function uses the following default values for its arguments:
		/// * max_keypoints: -1
		/// * score_threshold: 0.0f
		/// * image_size: Size()
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_def(model_path: &str) -> Result<core::Ptr<crate::features::DISK>> {
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_create_const_StringR(model_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DISK>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates a DISK detector from an in-memory model buffer.
		///
		/// This overload loads the DISK ONNX model from a buffer instead of a file on disk. It is
		/// intended for cases where the model is read from application resources (for example Android
		/// assets) and is not available as a path on the filesystem.
		///
		/// ## Parameters
		/// * bufferModel: A buffer containing the contents of the DISK ONNX model.
		/// * maxKeypoints: Maximum number of keypoints to return per image. The strongest
		///                    responses (by network score) are kept; -1 keeps all detections.
		/// * scoreThreshold: Discard keypoints with network score strictly below this value.
		/// * imageSize: Target input size (width, height) fed to the network. Use Size()
		///                  (the default) to fall back to the network's expected fixed input
		///                  shape of 1024x1024. When overriding, both dimensions must be
		///                  positive multiples of 16, since DISK downsamples by a factor of 16.
		/// * backendId: DNN backend identifier (see cv::dnn::Backend); 0 = DNN_BACKEND_DEFAULT.
		/// * targetId: DNN target identifier (see cv::dnn::Target);  0 = DNN_TARGET_CPU.
		///
		///
		/// Note: In C++ this is an overload of [create]. The Python/Java/Objective-C bindings expose
		///       it as `createFromMemory`, because Objective-C selectors are not disambiguated by argument
		///       type and would otherwise clash with the file-path [create].
		///
		/// ## C++ default parameters
		/// * max_keypoints: -1
		/// * score_threshold: 0.0f
		/// * image_size: Size()
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_from_memory(buffer_model: &core::Vector<u8>, max_keypoints: i32, score_threshold: f32, image_size: core::Size, backend_id: i32, target_id: i32) -> Result<core::Ptr<crate::features::DISK>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_create_const_vectorLunsigned_charGR_int_float_const_SizeR_int_int(buffer_model.as_raw_VectorOfu8(), max_keypoints, score_threshold, &image_size, backend_id, target_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DISK>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates a DISK detector from an in-memory model buffer.
		///
		/// This overload loads the DISK ONNX model from a buffer instead of a file on disk. It is
		/// intended for cases where the model is read from application resources (for example Android
		/// assets) and is not available as a path on the filesystem.
		///
		/// ## Parameters
		/// * bufferModel: A buffer containing the contents of the DISK ONNX model.
		/// * maxKeypoints: Maximum number of keypoints to return per image. The strongest
		///                    responses (by network score) are kept; -1 keeps all detections.
		/// * scoreThreshold: Discard keypoints with network score strictly below this value.
		/// * imageSize: Target input size (width, height) fed to the network. Use Size()
		///                  (the default) to fall back to the network's expected fixed input
		///                  shape of 1024x1024. When overriding, both dimensions must be
		///                  positive multiples of 16, since DISK downsamples by a factor of 16.
		/// * backendId: DNN backend identifier (see cv::dnn::Backend); 0 = DNN_BACKEND_DEFAULT.
		/// * targetId: DNN target identifier (see cv::dnn::Target);  0 = DNN_TARGET_CPU.
		///
		///
		/// Note: In C++ this is an overload of [create]. The Python/Java/Objective-C bindings expose
		///       it as `createFromMemory`, because Objective-C selectors are not disambiguated by argument
		///       type and would otherwise clash with the file-path [create].
		///
		/// ## Note
		/// This alternative version of [DISK::create_from_memory] function uses the following default values for its arguments:
		/// * max_keypoints: -1
		/// * score_threshold: 0.0f
		/// * image_size: Size()
		/// * backend_id: 0
		/// * target_id: 0
		#[inline]
		pub fn create_from_memory_def(buffer_model: &core::Vector<u8>) -> Result<core::Ptr<crate::features::DISK>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_create_const_vectorLunsigned_charGR(buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DISK>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::DISK]
	pub trait DISKTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_DISK(&self) -> *const c_void;

		#[inline]
		fn get_max_keypoints(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_getMaxKeypoints_const(self.as_raw_DISK(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_score_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_getScoreThreshold_const(self.as_raw_DISK(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_image_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_getImageSize_const(self.as_raw_DISK(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_getDefaultName_const(self.as_raw_DISK(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::DISK]
	pub trait DISKTrait: crate::features::DISKTraitConst + crate::features::Feature2DTrait {
		fn as_raw_mut_DISK(&mut self) -> *mut c_void;

		#[inline]
		fn set_max_keypoints(&mut self, max_keypoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_setMaxKeypoints_int(self.as_raw_mut_DISK(), max_keypoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_score_threshold(&mut self, threshold: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_setScoreThreshold_float(self.as_raw_mut_DISK(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_image_size(&mut self, size: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DISK_setImageSize_const_SizeR(self.as_raw_mut_DISK(), &size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for DISK {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DISK")
				.finish()
		}
	}

	boxed_cast_base! { DISK, core::Algorithm, cv_DISK_to_Algorithm }

	boxed_cast_base! { DISK, crate::features::Feature2D, cv_DISK_to_Feature2D }

	impl core::AlgorithmTraitConst for DISK {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for DISK {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { DISK, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::features::Feature2DTraitConst for DISK {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::Feature2DTrait for DISK {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { DISK, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

	impl crate::features::DISKTraitConst for DISK {
		#[inline] fn as_raw_DISK(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::DISKTrait for DISK {
		#[inline] fn as_raw_mut_DISK(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { DISK, crate::features::DISKTraitConst, as_raw_DISK, crate::features::DISKTrait, as_raw_mut_DISK }

	/// Abstract base class for matching keypoint descriptors.
	///
	/// It has two groups of match methods: for matching descriptors of an image with another image or with
	/// an image set.
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
		#[inline]
		pub fn create(descriptor_matcher_type: &str) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			extern_container_arg!(descriptor_matcher_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create_with_matcher_type(matcher_type: crate::features::DescriptorMatcher_MatcherType) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_create_const_MatcherTypeR(matcher_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::DescriptorMatcher]
	pub trait DescriptorMatcherTraitConst: core::AlgorithmTraitConst {
		fn as_raw_DescriptorMatcher(&self) -> *const c_void;

		/// Returns a constant link to the train descriptor collection trainDescCollection .
		#[inline]
		fn get_train_descriptors(&self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_getTrainDescriptors_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Returns true if there are no train descriptors in the both collections.
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_empty_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns true if the descriptor matcher supports masking permissible matches.
		#[inline]
		fn is_mask_supported(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_isMaskSupported_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn train_match(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>, mask: &impl ToInputArray) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn train_match_def(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_match_const_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn knn_train_match(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, mask: &impl ToInputArray, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn knn_train_match_def(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_knnMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn radius_train_match(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, mask: &impl ToInputArray, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn radius_train_match_def(&self, query_descriptors: &impl ToInputArray, train_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn write(&self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_write_const_const_StringR(self.as_raw_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn write_to_storage(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_write_const_FileStorageR(self.as_raw_DescriptorMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		#[must_use]
		fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_clone_const_bool(self.as_raw_DescriptorMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn clone_def(&self) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_clone_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn write_to_storage_with_name(&self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_write_const_FileStorageR_const_StringR(self.as_raw_DescriptorMatcher(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn add(&mut self, descriptors: &impl ToInputArray) -> Result<()> {
			input_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_add_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Clears the train descriptor collections.
		#[inline]
		fn clear(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_clear(self.as_raw_mut_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Trains a descriptor matcher
		///
		/// Trains a descriptor matcher (for example, the flann index). In all methods to match, the method
		/// train() is run every time before matching. Some descriptor matchers (for example, BruteForceMatcher)
		/// have an empty implementation of this method. Other matchers really train their inner structures (for
		/// example, FlannBasedMatcher trains flann::Index ).
		#[inline]
		fn train(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_train(self.as_raw_mut_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn match_(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>, masks: &impl ToInputArray) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(masks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		/// ## Note
		/// This alternative version of [DescriptorMatcherTrait::match_] function uses the following default values for its arguments:
		/// * masks: noArray()
		#[inline]
		fn match__def(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn knn_match(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, masks: &impl ToInputArray, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(masks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		/// ## Note
		/// This alternative version of [DescriptorMatcherTrait::knn_match] function uses the following default values for its arguments:
		/// * masks: noArray()
		/// * compact_result: false
		#[inline]
		fn knn_match_def(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn radius_match(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, masks: &impl ToInputArray, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(masks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		/// ## Note
		/// This alternative version of [DescriptorMatcherTrait::radius_match] function uses the following default values for its arguments:
		/// * masks: noArray()
		/// * compact_result: false
		#[inline]
		fn radius_match_def(&mut self, query_descriptors: &impl ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn read(&mut self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_read_const_StringR(self.as_raw_mut_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn read_from_node(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_read_const_FileNodeR(self.as_raw_mut_DescriptorMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for DescriptorMatcher {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DescriptorMatcher")
				.finish()
		}
	}

	boxed_cast_base! { DescriptorMatcher, core::Algorithm, cv_DescriptorMatcher_to_Algorithm }

	boxed_cast_descendant! { DescriptorMatcher, crate::features::BFMatcher, cv_DescriptorMatcher_to_BFMatcher }

	boxed_cast_descendant! { DescriptorMatcher, crate::features::FlannBasedMatcher, cv_DescriptorMatcher_to_FlannBasedMatcher }

	boxed_cast_descendant! { DescriptorMatcher, crate::features::LightGlueMatcher, cv_DescriptorMatcher_to_LightGlueMatcher }

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

	/// Wrapping class for feature detection using the FAST method.
	///
	/// Check [tutorial_py_fast] "the corresponding tutorial" for more details.
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

	impl FastFeatureDetector {
		/// ## C++ default parameters
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: FastFeatureDetector::TYPE_9_16
		#[inline]
		pub fn create(threshold: i32, nonmax_suppression: bool, typ: crate::features::FastFeatureDetector_DetectorType) -> Result<core::Ptr<crate::features::FastFeatureDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_create_int_bool_DetectorType(threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::FastFeatureDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [FastFeatureDetector::create] function uses the following default values for its arguments:
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: FastFeatureDetector::TYPE_9_16
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::features::FastFeatureDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::FastFeatureDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::FastFeatureDetector]
	pub trait FastFeatureDetectorTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_FastFeatureDetector(&self) -> *const c_void;

		#[inline]
		fn get_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_getThreshold_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_nonmax_suppression(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_getNonmaxSuppression_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_type(&self) -> Result<crate::features::FastFeatureDetector_DetectorType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_getType_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_getDefaultName_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::FastFeatureDetector]
	pub trait FastFeatureDetectorTrait: crate::features::FastFeatureDetectorTraitConst + crate::features::Feature2DTrait {
		fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void;

		#[inline]
		fn set_threshold(&mut self, threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_setThreshold_int(self.as_raw_mut_FastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_FastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_type(&mut self, typ: crate::features::FastFeatureDetector_DetectorType) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_setType_DetectorType(self.as_raw_mut_FastFeatureDetector(), typ, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for FastFeatureDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FastFeatureDetector")
				.finish()
		}
	}

	boxed_cast_base! { FastFeatureDetector, core::Algorithm, cv_FastFeatureDetector_to_Algorithm }

	boxed_cast_base! { FastFeatureDetector, crate::features::Feature2D, cv_FastFeatureDetector_to_Feature2D }

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

	/// Abstract base class for 2D image feature detectors and descriptor extractors
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

	impl Feature2D {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::features::Feature2D {
			let ret = unsafe { sys::cv_Feature2D_defaultNew_const() };
			let ret = unsafe { crate::features::Feature2D::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::features::Feature2D]
	pub trait Feature2DTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Feature2D(&self) -> *const c_void;

		#[inline]
		fn descriptor_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_descriptorSize_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn descriptor_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_descriptorType_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn default_norm(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_defaultNorm_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn write(&self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_write_const_const_StringR(self.as_raw_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn write_to_storage(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_write_const_FileStorageR(self.as_raw_Feature2D(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Return true if detector object is empty
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_empty_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_getDefaultName_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn write_to_storage_with_name(&self, fs: &mut impl core::FileStorageTrait, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_write_const_FileStorageR_const_StringR(self.as_raw_Feature2D(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn detect(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, mask: &impl ToInputArray) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR_const__InputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn detect_def(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLKeyPointGR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn detect_multiple(&mut self, images: &impl ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, masks: &impl ToInputArray) -> Result<()> {
			input_array_arg!(images);
			input_array_arg!(masks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR_const__InputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		/// ## Note
		/// This alternative version of [Feature2DTrait::detect_multiple] function uses the following default values for its arguments:
		/// * masks: noArray()
		#[inline]
		fn detect_multiple_def(&mut self, images: &impl ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>) -> Result<()> {
			input_array_arg!(images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_detect_const__InputArrayR_vectorLvectorLKeyPointGGR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn compute(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn compute_multiple(&mut self, images: &impl ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, descriptors: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(images);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(self.as_raw_mut_Feature2D(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Detects keypoints and computes the descriptors
		///
		/// ## C++ default parameters
		/// * use_provided_keypoints: false
		#[inline]
		fn detect_and_compute(&mut self, image: &impl ToInputArray, mask: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(mask);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_bool(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Detects keypoints and computes the descriptors
		///
		/// ## Note
		/// This alternative version of [Feature2DTrait::detect_and_compute] function uses the following default values for its arguments:
		/// * use_provided_keypoints: false
		#[inline]
		fn detect_and_compute_def(&mut self, image: &impl ToInputArray, mask: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(mask);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn read(&mut self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_read_const_StringR(self.as_raw_mut_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn read_from_node(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_read_const_FileNodeR(self.as_raw_mut_Feature2D(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for Feature2D {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for Feature2D {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Feature2D")
				.finish()
		}
	}

	boxed_cast_base! { Feature2D, core::Algorithm, cv_Feature2D_to_Algorithm }

	boxed_cast_descendant! { Feature2D, crate::features::ALIKED, cv_Feature2D_to_ALIKED }

	boxed_cast_descendant! { Feature2D, crate::features::AffineFeature, cv_Feature2D_to_AffineFeature }

	boxed_cast_descendant! { Feature2D, crate::features::DISK, cv_Feature2D_to_DISK }

	boxed_cast_descendant! { Feature2D, crate::features::FastFeatureDetector, cv_Feature2D_to_FastFeatureDetector }

	boxed_cast_descendant! { Feature2D, crate::features::GFTTDetector, cv_Feature2D_to_GFTTDetector }

	boxed_cast_descendant! { Feature2D, crate::features::MSER, cv_Feature2D_to_MSER }

	boxed_cast_descendant! { Feature2D, crate::features::ORB, cv_Feature2D_to_ORB }

	boxed_cast_descendant! { Feature2D, crate::features::SIFT, cv_Feature2D_to_SIFT }

	boxed_cast_descendant! { Feature2D, crate::features::SimpleBlobDetector, cv_Feature2D_to_SimpleBlobDetector }

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

	/// Flann-based descriptor matcher.
	///
	/// This matcher trains cv::flann::Index on a train descriptor collection and calls its nearest search
	/// methods to find the best matches. So, this matcher may be faster when matching a large train
	/// collection than the brute force matcher. FlannBasedMatcher does not support masking permissible
	/// matches of descriptor sets because flann::Index does not support this. :
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

	impl FlannBasedMatcher {
		/// ## C++ default parameters
		/// * index_params: makePtr<flann::KDTreeIndexParams>()
		/// * search_params: makePtr<flann::SearchParams>()
		#[inline]
		pub fn new(index_params: &core::Ptr<crate::flann::IndexParams>, search_params: &core::Ptr<crate::flann::SearchParams>) -> Result<crate::features::FlannBasedMatcher> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher_const_PtrLIndexParamsGR_const_PtrLSearchParamsGR(index_params.as_raw_PtrOfIndexParams(), search_params.as_raw_PtrOfSearchParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::features::FlannBasedMatcher::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * index_params: makePtr<flann::KDTreeIndexParams>()
		/// * search_params: makePtr<flann::SearchParams>()
		#[inline]
		pub fn new_def() -> Result<crate::features::FlannBasedMatcher> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::features::FlannBasedMatcher::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create() -> Result<core::Ptr<crate::features::FlannBasedMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::FlannBasedMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::FlannBasedMatcher]
	pub trait FlannBasedMatcherTraitConst: crate::features::DescriptorMatcherTraitConst {
		fn as_raw_FlannBasedMatcher(&self) -> *const c_void;

		#[inline]
		fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_write_const_FileStorageR(self.as_raw_FlannBasedMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn is_mask_supported(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_isMaskSupported_const(self.as_raw_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## C++ default parameters
		/// * empty_train_data: false
		#[inline]
		#[must_use]
		fn clone(&self, empty_train_data: bool) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_clone_const_bool(self.as_raw_FlannBasedMatcher(), empty_train_data, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [FlannBasedMatcherTraitConst::clone] function uses the following default values for its arguments:
		/// * empty_train_data: false
		#[inline]
		fn clone_def(&self) -> Result<core::Ptr<crate::features::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_clone_const(self.as_raw_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::FlannBasedMatcher]
	pub trait FlannBasedMatcherTrait: crate::features::DescriptorMatcherTrait + crate::features::FlannBasedMatcherTraitConst {
		fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void;

		#[inline]
		fn add(&mut self, descriptors: &impl ToInputArray) -> Result<()> {
			input_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_add_const__InputArrayR(self.as_raw_mut_FlannBasedMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn clear(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_clear(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_read_const_FileNodeR(self.as_raw_mut_FlannBasedMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn train(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_train(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for FlannBasedMatcher {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FlannBasedMatcher")
				.finish()
		}
	}

	boxed_cast_base! { FlannBasedMatcher, core::Algorithm, cv_FlannBasedMatcher_to_Algorithm }

	boxed_cast_base! { FlannBasedMatcher, crate::features::DescriptorMatcher, cv_FlannBasedMatcher_to_DescriptorMatcher }

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

	/// Wrapping class for feature detection using the goodFeaturesToTrack function. :
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

	impl GFTTDetector {
		/// ## C++ default parameters
		/// * max_corners: 1000
		/// * quality_level: 0.01
		/// * min_distance: 1
		/// * block_size: 3
		/// * use_harris_detector: false
		/// * k: 0.04
		#[inline]
		pub fn create(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<crate::features::GFTTDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners, quality_level, min_distance, block_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::features::GFTTDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::GFTTDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## C++ default parameters
		/// * use_harris_detector: false
		/// * k: 0.04
		#[inline]
		pub fn create_with_gradient(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradient_size: i32, use_harris_detector: bool, k: f64) -> Result<core::Ptr<crate::features::GFTTDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners, quality_level, min_distance, block_size, gradient_size, use_harris_detector, k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::GFTTDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [GFTTDetector::create_with_gradient] function uses the following default values for its arguments:
		/// * use_harris_detector: false
		/// * k: 0.04
		#[inline]
		pub fn create_with_gradient_def(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradient_size: i32) -> Result<core::Ptr<crate::features::GFTTDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int(max_corners, quality_level, min_distance, block_size, gradient_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::GFTTDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::GFTTDetector]
	pub trait GFTTDetectorTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_GFTTDetector(&self) -> *const c_void;

		#[inline]
		fn get_max_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getMaxFeatures_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_quality_level(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getQualityLevel_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_distance(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getMinDistance_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_block_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getBlockSize_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_harris_detector(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getHarrisDetector_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_k(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getK_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getDefaultName_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::GFTTDetector]
	pub trait GFTTDetectorTrait: crate::features::Feature2DTrait + crate::features::GFTTDetectorTraitConst {
		fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void;

		#[inline]
		fn set_max_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setMaxFeatures_int(self.as_raw_mut_GFTTDetector(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_quality_level(&mut self, qlevel: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setQualityLevel_double(self.as_raw_mut_GFTTDetector(), qlevel, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setMinDistance_double(self.as_raw_mut_GFTTDetector(), min_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_block_size(&mut self, block_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setBlockSize_int(self.as_raw_mut_GFTTDetector(), block_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_gradient_size(&mut self, gradient_size_: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setGradientSize_int(self.as_raw_mut_GFTTDetector(), gradient_size_, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_gradient_size(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getGradientSize(self.as_raw_mut_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_harris_detector(&mut self, val: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setHarrisDetector_bool(self.as_raw_mut_GFTTDetector(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_k(&mut self, k: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setK_double(self.as_raw_mut_GFTTDetector(), k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for GFTTDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GFTTDetector")
				.finish()
		}
	}

	boxed_cast_base! { GFTTDetector, core::Algorithm, cv_GFTTDetector_to_Algorithm }

	boxed_cast_base! { GFTTDetector, crate::features::Feature2D, cv_GFTTDetector_to_Feature2D }

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

	/// A class filters a vector of keypoints.
	///
	/// Because now it is difficult to provide a convenient interface for all usage scenarios of the
	/// keypoints filter class, it has only several needed by now static methods.
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

	impl KeyPointsFilter {
		#[inline]
		pub fn default() -> Result<crate::features::KeyPointsFilter> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_KeyPointsFilter(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::features::KeyPointsFilter::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn run_by_image_border(keypoints: &mut core::Vector<core::KeyPoint>, image_size: core::Size, border_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByImageBorder_vectorLKeyPointGR_Size_int(keypoints.as_raw_mut_VectorOfKeyPoint(), &image_size, border_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## C++ default parameters
		/// * max_size: FLT_MAX
		#[inline]
		pub fn run_by_keypoint_size(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32, max_size: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, max_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [KeyPointsFilter::run_by_keypoint_size] function uses the following default values for its arguments:
		/// * max_size: FLT_MAX
		#[inline]
		pub fn run_by_keypoint_size_def(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn run_by_pixels_mask(keypoints: &mut core::Vector<core::KeyPoint>, mask: &impl core::MatTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByPixelsMask_vectorLKeyPointGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn run_by_pixels_mask2_vector_point(keypoints: &mut core::Vector<core::KeyPoint>, remove_from: &mut core::Vector<core::Vector<core::Point>>, mask: &impl core::MatTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByPixelsMask2VectorPoint_vectorLKeyPointGR_vectorLvectorLPointGGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), remove_from.as_raw_mut_VectorOfVectorOfPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn remove_duplicated(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_removeDuplicated_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn remove_duplicated_sorted(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_removeDuplicatedSorted_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn retain_best(keypoints: &mut core::Vector<core::KeyPoint>, npoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_retainBest_vectorLKeyPointGR_int(keypoints.as_raw_mut_VectorOfKeyPoint(), npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::KeyPointsFilter]
	pub trait KeyPointsFilterTraitConst {
		fn as_raw_KeyPointsFilter(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::features::KeyPointsFilter]
	pub trait KeyPointsFilterTrait: crate::features::KeyPointsFilterTraitConst {
		fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for KeyPointsFilter {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("KeyPointsFilter")
				.finish()
		}
	}

	impl crate::features::KeyPointsFilterTraitConst for KeyPointsFilter {
		#[inline] fn as_raw_KeyPointsFilter(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::KeyPointsFilterTrait for KeyPointsFilter {
		#[inline] fn as_raw_mut_KeyPointsFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { KeyPointsFilter, crate::features::KeyPointsFilterTraitConst, as_raw_KeyPointsFilter, crate::features::KeyPointsFilterTrait, as_raw_mut_KeyPointsFilter }

	/// LightGlue feature matcher.
	///
	/// LightGlue is a CNN-based feature matcher, as described in [Lindenberger23](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lindenberger23) . It takes
	/// keypoint locations and descriptors from two images and directly predicts match pairs. Unlike
	/// traditional matchers that compute descriptor distances, LightGlue uses attention mechanisms
	/// to produce confidence scores for each potential match pair.
	///
	/// The matcher extends DescriptorMatcher and supports the standard match(), knnMatch(), and
	/// radiusMatch() interfaces. Context (keypoints and image sizes) must be provided via
	/// setPairInfo() before matching.
	pub struct LightGlueMatcher {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { LightGlueMatcher }

	impl Drop for LightGlueMatcher {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_LightGlueMatcher_delete(self.as_raw_mut_LightGlueMatcher()) };
		}
	}

	unsafe impl Send for LightGlueMatcher {}

	impl LightGlueMatcher {
		/// Creates LightGlueMatcher from a model file path.
		/// ## Parameters
		/// * modelPath: Path to the ONNX model file.
		/// * scoreThreshold: Match confidence threshold.
		/// * backend: DNN backend
		/// * target: DNN target
		///
		/// ## C++ default parameters
		/// * score_threshold: 0.0f
		/// * backend: 0
		/// * target: 0
		#[inline]
		pub fn create(model_path: &str, score_threshold: f32, backend: i32, target: i32) -> Result<core::Ptr<crate::features::LightGlueMatcher>> {
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LightGlueMatcher_create_const_StringR_float_int_int(model_path.opencv_as_extern(), score_threshold, backend, target, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::LightGlueMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates LightGlueMatcher from a model file path.
		/// ## Parameters
		/// * modelPath: Path to the ONNX model file.
		/// * scoreThreshold: Match confidence threshold.
		/// * backend: DNN backend
		/// * target: DNN target
		///
		/// ## Note
		/// This alternative version of [LightGlueMatcher::create] function uses the following default values for its arguments:
		/// * score_threshold: 0.0f
		/// * backend: 0
		/// * target: 0
		#[inline]
		pub fn create_def(model_path: &str) -> Result<core::Ptr<crate::features::LightGlueMatcher>> {
			extern_container_arg!(model_path);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LightGlueMatcher_create_const_StringR(model_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::LightGlueMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates LightGlueMatcher from in-memory model data.
		/// ## Parameters
		/// * modelData: Buffer containing the model data.
		/// * scoreThreshold: Match confidence threshold.
		/// * backend: DNN backend
		/// * target: DNN target
		///
		/// ## C++ default parameters
		/// * score_threshold: 0.0f
		/// * backend: 0
		/// * target: 0
		#[inline]
		pub fn create_from_memory(model_data: &core::Vector<u8>, score_threshold: f32, backend: i32, target: i32) -> Result<core::Ptr<crate::features::LightGlueMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LightGlueMatcher_create_const_vectorLunsigned_charGR_float_int_int(model_data.as_raw_VectorOfu8(), score_threshold, backend, target, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::LightGlueMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates LightGlueMatcher from in-memory model data.
		/// ## Parameters
		/// * modelData: Buffer containing the model data.
		/// * scoreThreshold: Match confidence threshold.
		/// * backend: DNN backend
		/// * target: DNN target
		///
		/// ## Note
		/// This alternative version of [LightGlueMatcher::create_from_memory] function uses the following default values for its arguments:
		/// * score_threshold: 0.0f
		/// * backend: 0
		/// * target: 0
		#[inline]
		pub fn create_from_memory_def(model_data: &core::Vector<u8>) -> Result<core::Ptr<crate::features::LightGlueMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LightGlueMatcher_create_const_vectorLunsigned_charGR(model_data.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::LightGlueMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::LightGlueMatcher]
	pub trait LightGlueMatcherTraitConst: crate::features::DescriptorMatcherTraitConst {
		fn as_raw_LightGlueMatcher(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::features::LightGlueMatcher]
	pub trait LightGlueMatcherTrait: crate::features::DescriptorMatcherTrait + crate::features::LightGlueMatcherTraitConst {
		fn as_raw_mut_LightGlueMatcher(&mut self) -> *mut c_void;

		/// Sets the keypoint and image size context for the next match() call.
		///
		/// This provides the spatial context that LightGlue needs in addition to descriptors.
		/// Must be called before match()/knnMatch()/radiusMatch() unless using automatic context
		/// from in-process ALIKED instances.
		///
		/// ## Parameters
		/// * queryKpts: Query image keypoints (Nx2 float matrix with x,y coordinates).
		/// * trainKpts: Train image keypoints (Nx2 float matrix with x,y coordinates).
		/// * queryImageSize: Size of the query image (width, height).
		/// * trainImageSize: Size of the train image (width, height).
		///
		/// ## C++ default parameters
		/// * query_image_size: Size()
		/// * train_image_size: Size()
		#[inline]
		fn set_pair_info(&mut self, query_kpts: &impl ToInputArray, train_kpts: &impl ToInputArray, query_image_size: core::Size, train_image_size: core::Size) -> Result<()> {
			input_array_arg!(query_kpts);
			input_array_arg!(train_kpts);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LightGlueMatcher_setPairInfo_const__InputArrayR_const__InputArrayR_Size_Size(self.as_raw_mut_LightGlueMatcher(), query_kpts.as_raw__InputArray(), train_kpts.as_raw__InputArray(), &query_image_size, &train_image_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the keypoint and image size context for the next match() call.
		///
		/// This provides the spatial context that LightGlue needs in addition to descriptors.
		/// Must be called before match()/knnMatch()/radiusMatch() unless using automatic context
		/// from in-process ALIKED instances.
		///
		/// ## Parameters
		/// * queryKpts: Query image keypoints (Nx2 float matrix with x,y coordinates).
		/// * trainKpts: Train image keypoints (Nx2 float matrix with x,y coordinates).
		/// * queryImageSize: Size of the query image (width, height).
		/// * trainImageSize: Size of the train image (width, height).
		///
		/// ## Note
		/// This alternative version of [LightGlueMatcherTrait::set_pair_info] function uses the following default values for its arguments:
		/// * query_image_size: Size()
		/// * train_image_size: Size()
		#[inline]
		fn set_pair_info_def(&mut self, query_kpts: &impl ToInputArray, train_kpts: &impl ToInputArray) -> Result<()> {
			input_array_arg!(query_kpts);
			input_array_arg!(train_kpts);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LightGlueMatcher_setPairInfo_const__InputArrayR_const__InputArrayR(self.as_raw_mut_LightGlueMatcher(), query_kpts.as_raw__InputArray(), train_kpts.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Clears stored pair context information.
		#[inline]
		fn clear_pair_info(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_LightGlueMatcher_clearPairInfo(self.as_raw_mut_LightGlueMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for LightGlueMatcher {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LightGlueMatcher")
				.finish()
		}
	}

	boxed_cast_base! { LightGlueMatcher, core::Algorithm, cv_LightGlueMatcher_to_Algorithm }

	boxed_cast_base! { LightGlueMatcher, crate::features::DescriptorMatcher, cv_LightGlueMatcher_to_DescriptorMatcher }

	impl core::AlgorithmTraitConst for LightGlueMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for LightGlueMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LightGlueMatcher, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::features::DescriptorMatcherTraitConst for LightGlueMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::DescriptorMatcherTrait for LightGlueMatcher {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LightGlueMatcher, crate::features::DescriptorMatcherTraitConst, as_raw_DescriptorMatcher, crate::features::DescriptorMatcherTrait, as_raw_mut_DescriptorMatcher }

	impl crate::features::LightGlueMatcherTraitConst for LightGlueMatcher {
		#[inline] fn as_raw_LightGlueMatcher(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::features::LightGlueMatcherTrait for LightGlueMatcher {
		#[inline] fn as_raw_mut_LightGlueMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LightGlueMatcher, crate::features::LightGlueMatcherTraitConst, as_raw_LightGlueMatcher, crate::features::LightGlueMatcherTrait, as_raw_mut_LightGlueMatcher }

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
		#[inline]
		pub fn create(delta: i32, min_area: i32, max_area: i32, max_variation: f64, min_diversity: f64, max_evolution: i32, area_threshold: f64, min_margin: f64, edge_blur_size: i32) -> Result<core::Ptr<crate::features::MSER>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_create_int_int_int_double_double_int_double_double_int(delta, min_area, max_area, max_variation, min_diversity, max_evolution, area_threshold, min_margin, edge_blur_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::features::MSER>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::MSER>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::MSER]
	pub trait MSERTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_MSER(&self) -> *const c_void;

		#[inline]
		fn get_delta(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getDelta_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_area(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMinArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_area(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMaxArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_variation(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMaxVariation_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_diversity(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMinDiversity_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_evolution(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMaxEvolution_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_area_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getAreaThreshold_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_margin(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMinMargin_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_edge_blur_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getEdgeBlurSize_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_pass2_only(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getPass2Only_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getDefaultName_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn detect_regions(&mut self, image: &impl ToInputArray, msers: &mut core::Vector<core::Vector<core::Point>>, bboxes: &mut core::Vector<core::Rect>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_detectRegions_const__InputArrayR_vectorLvectorLPointGGR_vectorLRectGR(self.as_raw_mut_MSER(), image.as_raw__InputArray(), msers.as_raw_mut_VectorOfVectorOfPoint(), bboxes.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_delta(&mut self, delta: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setDelta_int(self.as_raw_mut_MSER(), delta, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_area(&mut self, min_area: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMinArea_int(self.as_raw_mut_MSER(), min_area, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_area(&mut self, max_area: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMaxArea_int(self.as_raw_mut_MSER(), max_area, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_variation(&mut self, max_variation: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMaxVariation_double(self.as_raw_mut_MSER(), max_variation, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_diversity(&mut self, min_diversity: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMinDiversity_double(self.as_raw_mut_MSER(), min_diversity, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_evolution(&mut self, max_evolution: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMaxEvolution_int(self.as_raw_mut_MSER(), max_evolution, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_area_threshold(&mut self, area_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setAreaThreshold_double(self.as_raw_mut_MSER(), area_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_margin(&mut self, min_margin: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMinMargin_double(self.as_raw_mut_MSER(), min_margin, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_edge_blur_size(&mut self, edge_blur_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setEdgeBlurSize_int(self.as_raw_mut_MSER(), edge_blur_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_pass2_only(&mut self, f: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setPass2Only_bool(self.as_raw_mut_MSER(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for MSER {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("MSER")
				.finish()
		}
	}

	boxed_cast_base! { MSER, core::Algorithm, cv_MSER_to_Algorithm }

	boxed_cast_base! { MSER, crate::features::Feature2D, cv_MSER_to_Feature2D }

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

	/// Class implementing the ORB (*oriented BRIEF*) keypoint detector and descriptor extractor
	///
	/// described in [RRKB11](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_RRKB11) . The algorithm uses FAST in pyramids to detect stable keypoints, selects
	/// the strongest features using FAST or Harris response, finds their orientation using first-order
	/// moments and computes the descriptors using BRIEF (where the coordinates of random point pairs (or
	/// k-tuples) are rotated according to the measured orientation).
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

	impl ORB {
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
		#[inline]
		pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: crate::features::ORB_ScoreType, patch_size: i32, fast_threshold: i32) -> Result<core::Ptr<crate::features::ORB>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_create_int_float_int_int_int_int_ScoreType_int_int(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::features::ORB>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::ORB>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::ORB]
	pub trait ORBTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_ORB(&self) -> *const c_void;

		#[inline]
		fn get_max_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getMaxFeatures_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_scale_factor(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getScaleFactor_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_n_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getNLevels_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_edge_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getEdgeThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_first_level(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getFirstLevel_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_wta_k(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getWTA_K_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_score_type(&self) -> Result<crate::features::ORB_ScoreType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getScoreType_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_patch_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getPatchSize_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_fast_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getFastThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getDefaultName_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::ORB]
	pub trait ORBTrait: crate::features::Feature2DTrait + crate::features::ORBTraitConst {
		fn as_raw_mut_ORB(&mut self) -> *mut c_void;

		#[inline]
		fn set_max_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setMaxFeatures_int(self.as_raw_mut_ORB(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setScaleFactor_double(self.as_raw_mut_ORB(), scale_factor, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setNLevels_int(self.as_raw_mut_ORB(), nlevels, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setEdgeThreshold_int(self.as_raw_mut_ORB(), edge_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_first_level(&mut self, first_level: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setFirstLevel_int(self.as_raw_mut_ORB(), first_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setWTA_K_int(self.as_raw_mut_ORB(), wta_k, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_score_type(&mut self, score_type: crate::features::ORB_ScoreType) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setScoreType_ScoreType(self.as_raw_mut_ORB(), score_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setPatchSize_int(self.as_raw_mut_ORB(), patch_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_fast_threshold(&mut self, fast_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setFastThreshold_int(self.as_raw_mut_ORB(), fast_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for ORB {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ORB")
				.finish()
		}
	}

	boxed_cast_base! { ORB, core::Algorithm, cv_ORB_to_Algorithm }

	boxed_cast_base! { ORB, crate::features::Feature2D, cv_ORB_to_Feature2D }

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

	/// Class for extracting keypoints and computing descriptors using the Scale Invariant Feature Transform
	/// (SIFT) algorithm by D. Lowe [Lowe04](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lowe04) .
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
		#[inline]
		pub fn create(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, enable_precise_upscale: bool) -> Result<core::Ptr<crate::features::SIFT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_create_int_int_double_double_double_bool(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, enable_precise_upscale, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::features::SIFT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_with_descriptor_type(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32, enable_precise_upscale: bool) -> Result<core::Ptr<crate::features::SIFT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_create_int_int_double_double_double_int_bool(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, descriptor_type, enable_precise_upscale, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		pub fn create_with_descriptor_type_def(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32) -> Result<core::Ptr<crate::features::SIFT>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_create_int_int_double_double_double_int(nfeatures, n_octave_layers, contrast_threshold, edge_threshold, sigma, descriptor_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::SIFT>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::SIFT]
	pub trait SIFTTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_SIFT(&self) -> *const c_void;

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getDefaultName_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn get_n_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getNFeatures_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_n_octave_layers(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getNOctaveLayers_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_contrast_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getContrastThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_edge_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getEdgeThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_sigma(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getSigma_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::SIFT]
	pub trait SIFTTrait: crate::features::Feature2DTrait + crate::features::SIFTTraitConst {
		fn as_raw_mut_SIFT(&mut self) -> *mut c_void;

		#[inline]
		fn set_n_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setNFeatures_int(self.as_raw_mut_SIFT(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setNOctaveLayers_int(self.as_raw_mut_SIFT(), n_octave_layers, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_contrast_threshold(&mut self, contrast_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setContrastThreshold_double(self.as_raw_mut_SIFT(), contrast_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_edge_threshold(&mut self, edge_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setEdgeThreshold_double(self.as_raw_mut_SIFT(), edge_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_sigma(&mut self, sigma: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setSigma_double(self.as_raw_mut_SIFT(), sigma, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for SIFT {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SIFT")
				.finish()
		}
	}

	boxed_cast_base! { SIFT, core::Algorithm, cv_SIFT_to_Algorithm }

	boxed_cast_base! { SIFT, crate::features::Feature2D, cv_SIFT_to_Feature2D }

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

	impl SimpleBlobDetector {
		/// ## C++ default parameters
		/// * parameters: SimpleBlobDetector::Params()
		#[inline]
		pub fn create(parameters: crate::features::SimpleBlobDetector_Params) -> Result<core::Ptr<crate::features::SimpleBlobDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_create_const_ParamsR(&parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::SimpleBlobDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [SimpleBlobDetector::create] function uses the following default values for its arguments:
		/// * parameters: SimpleBlobDetector::Params()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::features::SimpleBlobDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features::SimpleBlobDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::features::SimpleBlobDetector]
	pub trait SimpleBlobDetectorTraitConst: crate::features::Feature2DTraitConst {
		fn as_raw_SimpleBlobDetector(&self) -> *const c_void;

		#[inline]
		fn get_params(&self) -> Result<crate::features::SimpleBlobDetector_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_getParams_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_getDefaultName_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Returns the contours of the blobs detected during the last call to detect().
		///
		/// Note: The [Params::collectContours] parameter must be set to true before calling
		/// detect() for this method to return any data.
		#[inline]
		fn get_blob_contours(&self) -> Result<core::Vector<core::Vector<core::Point>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_getBlobContours_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Vector<core::Point>>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::features::SimpleBlobDetector]
	pub trait SimpleBlobDetectorTrait: crate::features::Feature2DTrait + crate::features::SimpleBlobDetectorTraitConst {
		fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void;

		#[inline]
		fn set_params(&mut self, params: crate::features::SimpleBlobDetector_Params) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_setParams_const_ParamsR(self.as_raw_mut_SimpleBlobDetector(), &params, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for SimpleBlobDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SimpleBlobDetector")
				.finish()
		}
	}

	boxed_cast_base! { SimpleBlobDetector, core::Algorithm, cv_SimpleBlobDetector_to_Algorithm }

	boxed_cast_base! { SimpleBlobDetector, crate::features::Feature2D, cv_SimpleBlobDetector_to_Feature2D }

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
		/// Flag to enable contour collection.
		/// If set to true, the detector will store the contours of the detected blobs in memory,
		/// which can be retrieved after the detect() call using getBlobContours().
		///
		/// Note: Default value is false.
		pub collect_contours: bool,
	}

	opencv_type_simple! { crate::features::SimpleBlobDetector_Params }

	impl SimpleBlobDetector_Params {
		#[inline]
		pub fn write(self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_Params_write_const_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn default() -> Result<crate::features::SimpleBlobDetector_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn read(self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_Params_read_const_FileNodeR(&self, fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

}
