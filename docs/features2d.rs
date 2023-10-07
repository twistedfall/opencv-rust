pub mod features2d {
	//! # 2D Features Framework
	//!    # Feature Detection and Description
	//!    # Descriptor Matchers
	//! 
	//! Matchers of keypoint descriptors in OpenCV have wrappers with a common interface that enables you to
	//! easily switch between different algorithms solving the same problem. This section is devoted to
	//! matching descriptors that are represented as vectors in a multidimensional space. All objects that
	//! implement vector descriptor matchers inherit the DescriptorMatcher interface.
	//! 
	//!    # Drawing Function of Keypoints and Matches
	//!    # Object Categorization
	//! 
	//! This section describes approaches based on local 2D features and used to categorize objects.
	//! 
	//!    # Hardware Acceleration Layer
	//!        # Interface
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::KeyPointsFilterTraitConst, super::KeyPointsFilterTrait, super::Feature2DTraitConst, super::Feature2DTrait, super::AffineFeatureTraitConst, super::AffineFeatureTrait, super::SIFTTraitConst, super::SIFTTrait, super::BRISKTraitConst, super::BRISKTrait, super::ORBTraitConst, super::ORBTrait, super::MSERTraitConst, super::MSERTrait, super::FastFeatureDetectorTraitConst, super::FastFeatureDetectorTrait, super::AgastFeatureDetectorTraitConst, super::AgastFeatureDetectorTrait, super::GFTTDetectorTraitConst, super::GFTTDetectorTrait, super::SimpleBlobDetectorTraitConst, super::SimpleBlobDetectorTrait, super::KAZETraitConst, super::KAZETrait, super::AKAZETraitConst, super::AKAZETrait, super::DescriptorMatcherTraitConst, super::DescriptorMatcherTrait, super::BFMatcherTraitConst, super::BFMatcherTrait, super::FlannBasedMatcherTraitConst, super::FlannBasedMatcherTrait, super::BOWTrainerTraitConst, super::BOWTrainerTrait, super::BOWKMeansTrainerTraitConst, super::BOWKMeansTrainerTrait, super::BOWImgDescriptorExtractorTraitConst, super::BOWImgDescriptorExtractorTrait };
	}
	
	pub const AKAZE_DESCRIPTOR_KAZE: i32 = 3;
	/// Upright descriptors, not invariant to rotation
	pub const AKAZE_DESCRIPTOR_KAZE_UPRIGHT: i32 = 2;
	pub const AKAZE_DESCRIPTOR_MLDB: i32 = 5;
	/// Upright descriptors, not invariant to rotation
	pub const AKAZE_DESCRIPTOR_MLDB_UPRIGHT: i32 = 4;
	pub const AgastFeatureDetector_AGAST_5_8: i32 = 0;
	pub const AgastFeatureDetector_AGAST_7_12d: i32 = 1;
	pub const AgastFeatureDetector_AGAST_7_12s: i32 = 2;
	pub const AgastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
	pub const AgastFeatureDetector_OAST_9_16: i32 = 3;
	pub const AgastFeatureDetector_THRESHOLD: i32 = 10000;
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
	pub const KAZE_DIFF_CHARBONNIER: i32 = 3;
	pub const KAZE_DIFF_PM_G1: i32 = 0;
	pub const KAZE_DIFF_PM_G2: i32 = 1;
	pub const KAZE_DIFF_WEICKERT: i32 = 2;
	pub const ORB_FAST_SCORE: i32 = 1;
	pub const ORB_HARRIS_SCORE: i32 = 0;
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
	
	opencv_type_enum! { crate::features2d::AKAZE_DescriptorType }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum AgastFeatureDetector_DetectorType {
		AGAST_5_8 = 0,
		AGAST_7_12d = 1,
		AGAST_7_12s = 2,
		OAST_9_16 = 3,
	}
	
	opencv_type_enum! { crate::features2d::AgastFeatureDetector_DetectorType }
	
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
	
	opencv_type_enum! { crate::features2d::DescriptorMatcher_MatcherType }
	
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
	
	opencv_type_enum! { crate::features2d::DrawMatchesFlags }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum FastFeatureDetector_DetectorType {
		TYPE_5_8 = 0,
		TYPE_7_12 = 1,
		TYPE_9_16 = 2,
	}
	
	opencv_type_enum! { crate::features2d::FastFeatureDetector_DetectorType }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum KAZE_DiffusivityType {
		DIFF_PM_G1 = 0,
		DIFF_PM_G2 = 1,
		DIFF_WEICKERT = 2,
		DIFF_CHARBONNIER = 3,
	}
	
	opencv_type_enum! { crate::features2d::KAZE_DiffusivityType }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ORB_ScoreType {
		HARRIS_SCORE = 0,
		FAST_SCORE = 1,
	}
	
	opencv_type_enum! { crate::features2d::ORB_ScoreType }
	
	pub type AffineDescriptorExtractor = crate::features2d::AffineFeature;
	pub type AffineFeatureDetector = crate::features2d::AffineFeature;
	/// Extractors of keypoint descriptors in OpenCV have wrappers with a common interface that enables you
	/// to easily switch between different algorithms solving the same problem. This section is devoted to
	/// computing descriptors represented as vectors in a multidimensional space. All objects that implement
	/// the vector descriptor extractors inherit the DescriptorExtractor interface.
	pub type DescriptorExtractor = crate::features2d::Feature2D;
	/// Feature detectors in OpenCV have wrappers with a common interface that enables you to easily switch
	/// between different algorithms solving the same problem. All objects that implement keypoint detectors
	/// inherit the FeatureDetector interface.
	pub type FeatureDetector = crate::features2d::Feature2D;
	pub type SiftDescriptorExtractor = crate::features2d::SIFT;
	pub type SiftFeatureDetector = crate::features2d::SIFT;
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [agast] function uses the following default values for its arguments:
	/// * nonmax_suppression: true
	#[inline]
	pub fn agast_def(image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
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
	/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_mair2010_agast) .
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * nonmax_suppression: true
	#[inline]
	pub fn agast(image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
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
	/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_mair2010_agast) .
	#[inline]
	pub fn agast_with_type(image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::AgastFeatureDetector_DetectorType) -> Result<()> {
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
	#[inline]
	pub fn fast_def(image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
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
	/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Rosten06) .
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
	#[inline]
	pub fn fast(image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool) -> Result<()> {
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
	/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Rosten06) .
	/// 
	/// 
	/// Note: In Python API, types are given as cv.FAST_FEATURE_DETECTOR_TYPE_5_8,
	/// cv.FAST_FEATURE_DETECTOR_TYPE_7_12 and cv.FAST_FEATURE_DETECTOR_TYPE_9_16. For corner
	/// detection, use cv.FAST.detect() method.
	#[inline]
	pub fn fast_with_type(image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	pub fn draw_keypoints_def(image: &impl core::ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut impl core::ToInputOutputArray) -> Result<()> {
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
	#[inline]
	pub fn draw_keypoints(image: &impl core::ToInputArray, keypoints: &core::Vector<core::KeyPoint>, out_image: &mut impl core::ToInputOutputArray, color: core::Scalar, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
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
	#[inline]
	pub fn draw_matches_def(img1: &impl core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl core::ToInputOutputArray) -> Result<()> {
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
	#[inline]
	pub fn draw_matches(img1: &impl core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl core::ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
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
	/// This alternative version of [draw_matches_1] function uses the following default values for its arguments:
	/// * match_color: Scalar::all(-1)
	/// * single_point_color: Scalar::all(-1)
	/// * matches_mask: std::vector<char>()
	/// * flags: DrawMatchesFlags::DEFAULT
	#[inline]
	pub fn draw_matches_1_def(img1: &impl core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl core::ToInputOutputArray, matches_thickness: i32) -> Result<()> {
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
	#[inline]
	pub fn draw_matches_1(img1: &impl core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut impl core::ToInputOutputArray, matches_thickness: i32, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<c_char>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
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
	#[inline]
	pub fn draw_matches_knn_def(img1: &impl core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut impl core::ToInputOutputArray) -> Result<()> {
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
	#[inline]
	pub fn draw_matches_knn(img1: &impl core::ToInputArray, keypoints1: &core::Vector<core::KeyPoint>, img2: &impl core::ToInputArray, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::Vector<core::DMatch>>, out_img: &mut impl core::ToInputOutputArray, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &core::Vector<core::Vector<c_char>>, flags: crate::features2d::DrawMatchesFlags) -> Result<()> {
		input_array_arg!(img1);
		input_array_arg!(img2);
		input_output_array_arg!(out_img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_drawMatches_const__InputArrayR_const_vectorLKeyPointGR_const__InputArrayR_const_vectorLKeyPointGR_const_vectorLvectorLDMatchGGR_const__InputOutputArrayR_const_ScalarR_const_ScalarR_const_vectorLvectorLcharGGR_DrawMatchesFlags(img1.as_raw__InputArray(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw__InputArray(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw__InputOutputArray(), &match_color, &single_point_color, matches_mask.as_raw_VectorOfVectorOfc_char(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// *************************************************************************************\
	///    Functions to evaluate the feature detectors and [generic] descriptor extractors      *
	/// \***************************************************************************************
	/// 
	/// ## Note
	/// This alternative version of [evaluate_feature_detector] function uses the following default values for its arguments:
	/// * fdetector: Ptr<FeatureDetector>()
	#[inline]
	pub fn evaluate_feature_detector_def(img1: &core::Mat, img2: &core::Mat, h1to2: &core::Mat, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// *************************************************************************************\
	///    Functions to evaluate the feature detectors and [generic] descriptor extractors      *
	/// \***************************************************************************************
	/// 
	/// ## C++ default parameters
	/// * fdetector: Ptr<FeatureDetector>()
	#[inline]
	pub fn evaluate_feature_detector(img1: &core::Mat, img2: &core::Mat, h1to2: &core::Mat, keypoints1: &mut core::Vector<core::KeyPoint>, keypoints2: &mut core::Vector<core::KeyPoint>, repeatability: &mut f32, corresp_count: &mut i32, fdetector: &core::Ptr<crate::features2d::Feature2D>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_evaluateFeatureDetector_const_MatR_const_MatR_const_MatR_vectorLKeyPointGX_vectorLKeyPointGX_floatR_intR_const_PtrLFeature2DGR(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_mut_VectorOfKeyPoint(), keypoints2.as_raw_mut_VectorOfKeyPoint(), repeatability, corresp_count, fdetector.as_raw_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_nearest_point(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getNearestPoint_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_recall(recall_precision_curve: &core::Vector<core::Point2f>, l_precision: f32) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_getRecall_const_vectorLPoint2fGR_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::features2d::AKAZE]
	pub trait AKAZETraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_AKAZE(&self) -> *const c_void;
	
		#[inline]
		fn get_descriptor_type(&self) -> Result<crate::features2d::AKAZE_DescriptorType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getDescriptorType_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_descriptor_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getDescriptorSize_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_descriptor_channels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getDescriptorChannels_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getThreshold_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_n_octaves(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getNOctaves_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_n_octave_layers(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getNOctaveLayers_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getDiffusivity_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_getDefaultName_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::features2d::AKAZE]
	pub trait AKAZETrait: crate::features2d::AKAZETraitConst + crate::features2d::Feature2DTrait {
		fn as_raw_mut_AKAZE(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_descriptor_type(&mut self, dtype: crate::features2d::AKAZE_DescriptorType) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_setDescriptorType_DescriptorType(self.as_raw_mut_AKAZE(), dtype, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_descriptor_size(&mut self, dsize: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_setDescriptorSize_int(self.as_raw_mut_AKAZE(), dsize, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_descriptor_channels(&mut self, dch: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_setDescriptorChannels_int(self.as_raw_mut_AKAZE(), dch, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_threshold(&mut self, threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_setThreshold_double(self.as_raw_mut_AKAZE(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_setNOctaves_int(self.as_raw_mut_AKAZE(), octaves, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_setNOctaveLayers_int(self.as_raw_mut_AKAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_AKAZE(), diff, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class implementing the AKAZE keypoint detector and descriptor extractor, described in [ANB13](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ANB13).
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
	pub struct AKAZE {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for AKAZE {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for AKAZE {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AKAZETraitConst for AKAZE {
		#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::AKAZETrait for AKAZE {
		#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		/// 
		/// ## C++ default parameters
		/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
		/// * descriptor_size: 0
		/// * descriptor_channels: 3
		/// * threshold: 0.001f
		/// * n_octaves: 4
		/// * n_octave_layers: 4
		/// * diffusivity: KAZE::DIFF_PM_G2
		#[inline]
		pub fn create(descriptor_type: crate::features2d::AKAZE_DescriptorType, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: crate::features2d::KAZE_DiffusivityType) -> Result<core::Ptr<crate::features2d::AKAZE>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AKAZE_create_DescriptorType_int_int_float_int_int_DiffusivityType(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity, ocvrs_return.as_mut_ptr()) };
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
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
		/// * descriptor_size: 0
		/// * descriptor_channels: 3
		/// * threshold: 0.001f
		/// * n_octaves: 4
		/// * n_octave_layers: 4
		/// * diffusivity: KAZE::DIFF_PM_G2
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
	pub trait AffineFeatureTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_AffineFeature(&self) -> *const c_void;
	
		#[inline]
		fn get_view_params(&self, tilts: &mut core::Vector<f32>, rolls: &mut core::Vector<f32>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AffineFeature_getViewParams_const_vectorLfloatGR_vectorLfloatGR(self.as_raw_AffineFeature(), tilts.as_raw_mut_VectorOff32(), rolls.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	/// described as ASIFT in [YM11](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_YM11) .
	pub struct AffineFeature {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for AffineFeature {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for AffineFeature {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AffineFeatureTraitConst for AffineFeature {
		#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::AffineFeatureTrait for AffineFeature {
		#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * max_tilt: 5
		/// * min_tilt: 0
		/// * tilt_step: 1.4142135623730951f
		/// * rotate_step_base: 72
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
	pub trait AgastFeatureDetectorTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_AgastFeatureDetector(&self) -> *const c_void;
	
		#[inline]
		fn get_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AgastFeatureDetector_getThreshold_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nonmax_suppression(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AgastFeatureDetector_getNonmaxSuppression_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_type(&self) -> Result<crate::features2d::AgastFeatureDetector_DetectorType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AgastFeatureDetector_getType_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_threshold(&mut self, threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AgastFeatureDetector_setThreshold_int(self.as_raw_mut_AgastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_AgastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_AgastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct AgastFeatureDetector {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for AgastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for AgastFeatureDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AgastFeatureDetectorTraitConst for AgastFeatureDetector {
		#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::AgastFeatureDetectorTrait for AgastFeatureDetector {
		#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AgastFeatureDetector {
		/// ## C++ default parameters
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: AgastFeatureDetector::OAST_9_16
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: AgastFeatureDetector::OAST_9_16
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
	pub trait BFMatcherTraitConst: crate::features2d::DescriptorMatcherTraitConst {
		fn as_raw_BFMatcher(&self) -> *const c_void;
	
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
		/// This alternative version of [clone] function uses the following default values for its arguments:
		/// * empty_train_data: false
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
	pub struct BFMatcher {
		ptr: *mut c_void
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
	
	impl crate::features2d::DescriptorMatcherTraitConst for BFMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcherTrait for BFMatcher {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::BFMatcherTraitConst for BFMatcher {
		#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::BFMatcherTrait for BFMatcher {
		#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BFMatcher {
		/// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
		/// 
		/// ## C++ default parameters
		/// * norm_type: NORM_L2
		/// * cross_check: false
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * norm_type: NORM_L2
		/// * cross_check: false
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
	pub trait BOWImgDescriptorExtractorTraitConst {
		fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void;
	
		/// Returns the set vocabulary.
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
		#[inline]
		fn descriptor_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorSize_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns an image descriptor type.
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
		#[inline]
		fn set_vocabulary(&mut self, vocabulary: &core::Mat) -> Result<()> {
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
		#[inline]
		fn compute_desc(&mut self, image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl core::ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>, descriptors: &mut core::Mat) -> Result<()> {
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
		/// This alternative version of [compute_desc] function uses the following default values for its arguments:
		/// * point_idxs_of_clusters: 0
		/// * descriptors: 0
		#[inline]
		fn compute_desc_def(&mut self, image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl core::ToOutputArray) -> Result<()> {
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
		///    pointIdxsOfClusters[i] are keypoint indices that belong to the i -th cluster (word of vocabulary)
		///    returned if it is non-zero.
		/// 
		/// ## C++ default parameters
		/// * point_idxs_of_clusters: 0
		#[inline]
		fn compute(&mut self, keypoint_descriptors: &impl core::ToInputArray, img_descriptor: &mut impl core::ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>) -> Result<()> {
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
		/// This alternative version of [compute] function uses the following default values for its arguments:
		/// * point_idxs_of_clusters: 0
		#[inline]
		fn compute_def(&mut self, keypoint_descriptors: &impl core::ToInputArray, img_descriptor: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(keypoint_descriptors);
			output_array_arg!(img_descriptor);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw__InputArray(), img_descriptor.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn compute2(&mut self, image: &core::Mat, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut core::Mat) -> Result<()> {
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
	pub struct BOWImgDescriptorExtractor {
		ptr: *mut c_void
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
	
	impl BOWImgDescriptorExtractor {
		/// The constructor.
		/// 
		/// ## Parameters
		/// * dextractor: Descriptor extractor that is used to compute descriptors for an input image and
		/// its keypoints.
		/// * dmatcher: Descriptor matcher that is used to find the nearest word of the trained vocabulary
		/// for each keypoint descriptor of the image.
		#[inline]
		pub fn new(dextractor: &core::Ptr<crate::features2d::Feature2D>, dmatcher: &core::Ptr<crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
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
		#[inline]
		pub fn new_1(dmatcher: &core::Ptr<crate::features2d::DescriptorMatcher>) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
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
	pub trait BOWKMeansTrainerTraitConst: crate::features2d::BOWTrainerTraitConst {
		fn as_raw_BOWKMeansTrainer(&self) -> *const c_void;
	
		#[inline]
		fn cluster(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BOWKMeansTrainer_cluster_const(self.as_raw_BOWKMeansTrainer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn cluster_1(&self, descriptors: &core::Mat) -> Result<core::Mat> {
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
	pub struct BOWKMeansTrainer {
		ptr: *mut c_void
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
	
	impl crate::features2d::BOWKMeansTrainerTraitConst for BOWKMeansTrainer {
		#[inline] fn as_raw_BOWKMeansTrainer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::BOWKMeansTrainerTrait for BOWKMeansTrainer {
		#[inline] fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
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
	pub trait BOWTrainerTraitConst {
		fn as_raw_BOWTrainer(&self) -> *const c_void;
	
		/// Returns a training set of descriptors.
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
		#[inline]
		fn cluster_with_descriptors(&self, descriptors: &core::Mat) -> Result<core::Mat> {
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
		#[inline]
		fn add(&mut self, descriptors: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BOWTrainer_add_const_MatR(self.as_raw_mut_BOWTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct BOWTrainer {
		ptr: *mut c_void
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
	pub trait BRISKTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_BRISK(&self) -> *const c_void;
	
		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BRISK_getDefaultName_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BRISK_getThreshold_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_octaves(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BRISK_getOctaves_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn set_pattern_scale(&mut self, pattern_scale: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_BRISK_setPatternScale_float(self.as_raw_mut_BRISK(), pattern_scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class implementing the BRISK keypoint detector and descriptor extractor, described in [LCS11](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_LCS11) .
	pub struct BRISK {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for BRISK {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for BRISK {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::BRISKTraitConst for BRISK {
		#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::BRISKTrait for BRISK {
		#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * thresh: 30
		/// * octaves: 3
		/// * pattern_scale: 1.0f
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
		/// This alternative version of [create_with_pattern] function uses the following default values for its arguments:
		/// * d_max: 5.85f
		/// * d_min: 8.2f
		/// * index_change: std::vector<int>()
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
		/// This alternative version of [create_with_pattern_threshold_octaves] function uses the following default values for its arguments:
		/// * d_max: 5.85f
		/// * d_min: 8.2f
		/// * index_change: std::vector<int>()
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
	pub trait DescriptorMatcherTraitConst: core::AlgorithmTraitConst {
		fn as_raw_DescriptorMatcher(&self) -> *const c_void;
	
		/// Returns a constant link to the train descriptor collection trainDescCollection .
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
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_empty_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns true if the descriptor matcher supports masking permissible matches.
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
		#[inline]
		fn train_match(&self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>, mask: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [train_match] function uses the following default values for its arguments:
		/// * mask: noArray()
		#[inline]
		fn train_match_def(&self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
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
		#[inline]
		fn knn_train_match(&self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, mask: &impl core::ToInputArray, compact_result: bool) -> Result<()> {
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
		/// This alternative version of [knn_train_match] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * compact_result: false
		#[inline]
		fn knn_train_match_def(&self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
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
		#[inline]
		fn radius_train_match(&self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, mask: &impl core::ToInputArray, compact_result: bool) -> Result<()> {
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
		/// This alternative version of [radius_train_match] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * compact_result: false
		#[inline]
		fn radius_train_match_def(&self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn write(&self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_write_const_const_StringR(self.as_raw_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn write_1(&self, unnamed: &mut core::FileStorage) -> Result<()> {
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
		/// This alternative version of [clone] function uses the following default values for its arguments:
		/// * empty_train_data: false
		#[inline]
		fn clone_def(&self) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_clone_const(self.as_raw_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn write_2(&self, fs: &mut core::FileStorage, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_write_const_FileStorageR_const_StringR(self.as_raw_DescriptorMatcher(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn write_3(&self, fs: &core::Ptr<core::FileStorage>, name: &str) -> Result<()> {
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
		#[inline]
		fn add(&mut self, descriptors: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_add_const__InputArrayR(self.as_raw_mut_DescriptorMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Clears the train descriptor collections.
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
		///    descriptor. So, matches size may be smaller than the query descriptors count.
		/// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
		///    descriptors and stored train descriptors from the i-th image trainDescCollection[i].
		/// 
		/// ## C++ default parameters
		/// * masks: noArray()
		#[inline]
		fn match_(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>, masks: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [match_] function uses the following default values for its arguments:
		/// * masks: noArray()
		#[inline]
		fn match__def(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
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
		#[inline]
		fn knn_match(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, masks: &impl core::ToInputArray, compact_result: bool) -> Result<()> {
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
		/// This alternative version of [knn_match] function uses the following default values for its arguments:
		/// * masks: noArray()
		/// * compact_result: false
		#[inline]
		fn knn_match_def(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
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
		#[inline]
		fn radius_match(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, masks: &impl core::ToInputArray, compact_result: bool) -> Result<()> {
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
		/// This alternative version of [radius_match] function uses the following default values for its arguments:
		/// * masks: noArray()
		/// * compact_result: false
		#[inline]
		fn radius_match_def(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_mut_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn read(&mut self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_read_const_StringR(self.as_raw_mut_DescriptorMatcher(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn read_1(&mut self, unnamed: &core::FileNode) -> Result<()> {
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
	pub struct DescriptorMatcher {
		ptr: *mut c_void
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
	
	impl crate::features2d::DescriptorMatcherTraitConst for DescriptorMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcherTrait for DescriptorMatcher {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		pub fn create(descriptor_matcher_type: &str) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
			extern_container_arg!(descriptor_matcher_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_create_const_StringR(descriptor_matcher_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features2d::DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn create_with_matcher_type(matcher_type: crate::features2d::DescriptorMatcher_MatcherType) -> Result<core::Ptr<crate::features2d::DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_DescriptorMatcher_create_const_MatcherTypeR(&matcher_type, ocvrs_return.as_mut_ptr()) };
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
	pub trait FastFeatureDetectorTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_FastFeatureDetector(&self) -> *const c_void;
	
		#[inline]
		fn get_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_getThreshold_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nonmax_suppression(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_getNonmaxSuppression_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_type(&self) -> Result<crate::features2d::FastFeatureDetector_DetectorType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_getType_const(self.as_raw_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_threshold(&mut self, threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_setThreshold_int(self.as_raw_mut_FastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_FastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct FastFeatureDetector {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for FastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for FastFeatureDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::FastFeatureDetectorTraitConst for FastFeatureDetector {
		#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::FastFeatureDetectorTrait for FastFeatureDetector {
		#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FastFeatureDetector {
		/// ## C++ default parameters
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: FastFeatureDetector::TYPE_9_16
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: FastFeatureDetector::TYPE_9_16
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
	pub trait Feature2DTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Feature2D(&self) -> *const c_void;
	
		#[inline]
		fn descriptor_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_descriptorSize_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn descriptor_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_descriptorType_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn default_norm(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_defaultNorm_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn write(&self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_write_const_const_StringR(self.as_raw_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn write_1(&self, unnamed: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_write_const_FileStorageR(self.as_raw_Feature2D(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Return true if detector object is empty
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_empty_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_getDefaultName_const(self.as_raw_Feature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn write_2(&self, fs: &mut core::FileStorage, name: &str) -> Result<()> {
			extern_container_arg!(name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_write_const_FileStorageR_const_StringR(self.as_raw_Feature2D(), fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn write_3(&self, fs: &core::Ptr<core::FileStorage>, name: &str) -> Result<()> {
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
		#[inline]
		fn detect(&mut self, image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, mask: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [detect] function uses the following default values for its arguments:
		/// * mask: noArray()
		#[inline]
		fn detect_def(&mut self, image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
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
		///    of keypoints detected in images[i] .
		/// * masks: Masks for each input image specifying where to look for keypoints (optional).
		///    masks[i] is a mask for images[i].
		/// 
		/// ## C++ default parameters
		/// * masks: noArray()
		#[inline]
		fn detect_multiple(&mut self, images: &impl core::ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, masks: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [detect_multiple] function uses the following default values for its arguments:
		/// * masks: noArray()
		#[inline]
		fn detect_multiple_def(&mut self, images: &impl core::ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>) -> Result<()> {
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
		#[inline]
		fn compute(&mut self, image: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl core::ToOutputArray) -> Result<()> {
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
		///    computed are removed. Sometimes new keypoints can be added, for example: SIFT duplicates keypoint
		///    with several dominant orientations (for each orientation).
		/// * descriptors: Computed descriptors. In the second variant of the method descriptors[i] are
		///    descriptors computed for a keypoints[i]. Row j is the keypoints (or keypoints[i]) is the
		///    descriptor for keypoint j-th keypoint.
		#[inline]
		fn compute_multiple(&mut self, images: &impl core::ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, descriptors: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn detect_and_compute(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl core::ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
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
		/// This alternative version of [detect_and_compute] function uses the following default values for its arguments:
		/// * use_provided_keypoints: false
		#[inline]
		fn detect_and_compute_def(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(mask);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_Feature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn read(&mut self, file_name: &str) -> Result<()> {
			extern_container_arg!(file_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_read_const_StringR(self.as_raw_mut_Feature2D(), file_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn read_1(&mut self, unnamed: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Feature2D_read_const_FileNodeR(self.as_raw_mut_Feature2D(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Feature2D {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for Feature2D {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for Feature2D {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Feature2D {
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
	
	/// Constant methods for [crate::features2d::FlannBasedMatcher]
	pub trait FlannBasedMatcherTraitConst: crate::features2d::DescriptorMatcherTraitConst {
		fn as_raw_FlannBasedMatcher(&self) -> *const c_void;
	
		#[inline]
		fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_write_const_FileStorageR(self.as_raw_FlannBasedMatcher(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		/// This alternative version of [clone] function uses the following default values for its arguments:
		/// * empty_train_data: false
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
	
		#[inline]
		fn add(&mut self, descriptors: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_add_const__InputArrayR(self.as_raw_mut_FlannBasedMatcher(), descriptors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn clear(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_clear(self.as_raw_mut_FlannBasedMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_read_const_FileNodeR(self.as_raw_mut_FlannBasedMatcher(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct FlannBasedMatcher {
		ptr: *mut c_void
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
	
	impl crate::features2d::DescriptorMatcherTraitConst for FlannBasedMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcherTrait for FlannBasedMatcher {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::FlannBasedMatcherTraitConst for FlannBasedMatcher {
		#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::FlannBasedMatcherTrait for FlannBasedMatcher {
		#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FlannBasedMatcher {
		/// ## C++ default parameters
		/// * index_params: makePtr<flann::KDTreeIndexParams>()
		/// * search_params: makePtr<flann::SearchParams>()
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
		#[inline]
		pub fn new_def() -> Result<crate::features2d::FlannBasedMatcher> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_FlannBasedMatcher_FlannBasedMatcher(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::features2d::FlannBasedMatcher::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	pub trait GFTTDetectorTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_GFTTDetector(&self) -> *const c_void;
	
		#[inline]
		fn get_max_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getMaxFeatures_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_quality_level(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getQualityLevel_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_distance(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getMinDistance_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_block_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getBlockSize_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_harris_detector(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getHarrisDetector_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_k(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getK_const(self.as_raw_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_max_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setMaxFeatures_int(self.as_raw_mut_GFTTDetector(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_quality_level(&mut self, qlevel: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setQualityLevel_double(self.as_raw_mut_GFTTDetector(), qlevel, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setMinDistance_double(self.as_raw_mut_GFTTDetector(), min_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_block_size(&mut self, block_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setBlockSize_int(self.as_raw_mut_GFTTDetector(), block_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_gradient_size(&mut self, gradient_size_: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setGradientSize_int(self.as_raw_mut_GFTTDetector(), gradient_size_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_gradient_size(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_getGradientSize(self.as_raw_mut_GFTTDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_harris_detector(&mut self, val: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFTTDetector_setHarrisDetector_bool(self.as_raw_mut_GFTTDetector(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct GFTTDetector {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for GFTTDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for GFTTDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::GFTTDetectorTraitConst for GFTTDetector {
		#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::GFTTDetectorTrait for GFTTDetector {
		#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GFTTDetector {
		/// ## C++ default parameters
		/// * max_corners: 1000
		/// * quality_level: 0.01
		/// * min_distance: 1
		/// * block_size: 3
		/// * use_harris_detector: false
		/// * k: 0.04
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * max_corners: 1000
		/// * quality_level: 0.01
		/// * min_distance: 1
		/// * block_size: 3
		/// * use_harris_detector: false
		/// * k: 0.04
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
		/// This alternative version of [create_with_gradient] function uses the following default values for its arguments:
		/// * use_harris_detector: false
		/// * k: 0.04
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
	pub trait KAZETraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_KAZE(&self) -> *const c_void;
	
		#[inline]
		fn get_extended(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_getExtended_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_upright(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_getUpright_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_getThreshold_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_n_octaves(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_getNOctaves_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_n_octave_layers(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_getNOctaveLayers_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_diffusivity(&self) -> Result<crate::features2d::KAZE_DiffusivityType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_getDiffusivity_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_extended(&mut self, extended: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_setExtended_bool(self.as_raw_mut_KAZE(), extended, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_upright(&mut self, upright: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_setUpright_bool(self.as_raw_mut_KAZE(), upright, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_threshold(&mut self, threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_setThreshold_double(self.as_raw_mut_KAZE(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_setNOctaves_int(self.as_raw_mut_KAZE(), octaves, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_setNOctaveLayers_int(self.as_raw_mut_KAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_diffusivity(&mut self, diff: crate::features2d::KAZE_DiffusivityType) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KAZE_setDiffusivity_DiffusivityType(self.as_raw_mut_KAZE(), diff, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class implementing the KAZE keypoint detector and descriptor extractor, described in [ABD12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ABD12) .
	/// 
	/// 
	/// Note: AKAZE descriptor can only be used with KAZE or AKAZE keypoints .. [ABD12] KAZE Features. Pablo
	/// F. Alcantarilla, Adrien Bartoli and Andrew J. Davison. In European Conference on Computer Vision
	/// (ECCV), Fiorenze, Italy, October 2012.
	pub struct KAZE {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for KAZE {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for KAZE {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::KAZETraitConst for KAZE {
		#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::KAZETrait for KAZE {
		#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * extended: false
		/// * upright: false
		/// * threshold: 0.001f
		/// * n_octaves: 4
		/// * n_octave_layers: 4
		/// * diffusivity: KAZE::DIFF_PM_G2
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
	pub struct KeyPointsFilter {
		ptr: *mut c_void
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
	
	impl KeyPointsFilter {
		#[inline]
		pub fn default() -> Result<crate::features2d::KeyPointsFilter> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_KeyPointsFilter(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::features2d::KeyPointsFilter::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn run_by_image_border(keypoints: &mut core::Vector<core::KeyPoint>, image_size: core::Size, border_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByImageBorder_vectorLKeyPointGR_Size_int(keypoints.as_raw_mut_VectorOfKeyPoint(), image_size.opencv_as_extern(), border_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * max_size: FLT_MAX
		#[inline]
		pub fn run_by_keypoint_size(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32, max_size: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, max_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run_by_keypoint_size] function uses the following default values for its arguments:
		/// * max_size: FLT_MAX
		#[inline]
		pub fn run_by_keypoint_size_def(keypoints: &mut core::Vector<core::KeyPoint>, min_size: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_vectorLKeyPointGR_float(keypoints.as_raw_mut_VectorOfKeyPoint(), min_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn run_by_pixels_mask(keypoints: &mut core::Vector<core::KeyPoint>, mask: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByPixelsMask_vectorLKeyPointGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn run_by_pixels_mask2_vector_point(keypoints: &mut core::Vector<core::KeyPoint>, remove_from: &mut core::Vector<core::Vector<core::Point>>, mask: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_runByPixelsMask2VectorPoint_vectorLKeyPointGR_vectorLvectorLPointGGR_const_MatR(keypoints.as_raw_mut_VectorOfKeyPoint(), remove_from.as_raw_mut_VectorOfVectorOfPoint(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn remove_duplicated(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_removeDuplicated_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn remove_duplicated_sorted(keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_KeyPointsFilter_removeDuplicatedSorted_vectorLKeyPointGR(keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub trait MSERTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_MSER(&self) -> *const c_void;
	
		#[inline]
		fn get_delta(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getDelta_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_area(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMinArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_area(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMaxArea_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_variation(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMaxVariation_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_diversity(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMinDiversity_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_evolution(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMaxEvolution_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_area_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getAreaThreshold_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_margin(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getMinMargin_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_edge_blur_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getEdgeBlurSize_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_pass2_only(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_getPass2Only_const(self.as_raw_MSER(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn detect_regions(&mut self, image: &impl core::ToInputArray, msers: &mut core::Vector<core::Vector<core::Point>>, bboxes: &mut core::Vector<core::Rect>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_detectRegions_const__InputArrayR_vectorLvectorLPointGGR_vectorLRectGR(self.as_raw_mut_MSER(), image.as_raw__InputArray(), msers.as_raw_mut_VectorOfVectorOfPoint(), bboxes.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_delta(&mut self, delta: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setDelta_int(self.as_raw_mut_MSER(), delta, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_area(&mut self, min_area: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMinArea_int(self.as_raw_mut_MSER(), min_area, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_area(&mut self, max_area: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMaxArea_int(self.as_raw_mut_MSER(), max_area, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_variation(&mut self, max_variation: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMaxVariation_double(self.as_raw_mut_MSER(), max_variation, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_diversity(&mut self, min_diversity: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMinDiversity_double(self.as_raw_mut_MSER(), min_diversity, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_evolution(&mut self, max_evolution: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMaxEvolution_int(self.as_raw_mut_MSER(), max_evolution, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_area_threshold(&mut self, area_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setAreaThreshold_double(self.as_raw_mut_MSER(), area_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_margin(&mut self, min_margin: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setMinMargin_double(self.as_raw_mut_MSER(), min_margin, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_edge_blur_size(&mut self, edge_blur_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MSER_setEdgeBlurSize_int(self.as_raw_mut_MSER(), edge_blur_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	/// - the grey image algorithm is taken from: [nister2008linear](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_nister2008linear) ;  the paper claims to be faster
	/// than union-find method; it actually get 1.5~2m/s on my centrino L7200 1.2GHz laptop.
	/// 
	/// - the color image algorithm is taken from: [forssen2007maximally](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_forssen2007maximally) ; it should be much slower
	/// than grey image method ( 3~4 times )
	/// 
	/// - (Python) A complete example showing the use of the %MSER detector can be found at samples/python/mser.py
	pub struct MSER {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for MSER {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for MSER {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::MSERTraitConst for MSER {
		#[inline] fn as_raw_MSER(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::MSERTrait for MSER {
		#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		/// This alternative version of [create] function uses the following default values for its arguments:
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
	pub trait ORBTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_ORB(&self) -> *const c_void;
	
		#[inline]
		fn get_max_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getMaxFeatures_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_scale_factor(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getScaleFactor_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_n_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getNLevels_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_edge_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getEdgeThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_first_level(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getFirstLevel_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_wta_k(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getWTA_K_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_score_type(&self) -> Result<crate::features2d::ORB_ScoreType> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getScoreType_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_patch_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getPatchSize_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_fast_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_getFastThreshold_const(self.as_raw_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_max_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setMaxFeatures_int(self.as_raw_mut_ORB(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setScaleFactor_double(self.as_raw_mut_ORB(), scale_factor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setNLevels_int(self.as_raw_mut_ORB(), nlevels, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setEdgeThreshold_int(self.as_raw_mut_ORB(), edge_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_first_level(&mut self, first_level: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setFirstLevel_int(self.as_raw_mut_ORB(), first_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setWTA_K_int(self.as_raw_mut_ORB(), wta_k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_score_type(&mut self, score_type: crate::features2d::ORB_ScoreType) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setScoreType_ScoreType(self.as_raw_mut_ORB(), score_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ORB_setPatchSize_int(self.as_raw_mut_ORB(), patch_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	/// described in [RRKB11](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_RRKB11) . The algorithm uses FAST in pyramids to detect stable keypoints, selects
	/// the strongest features using FAST or Harris response, finds their orientation using first-order
	/// moments and computes the descriptors using BRIEF (where the coordinates of random point pairs (or
	/// k-tuples) are rotated according to the measured orientation).
	pub struct ORB {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for ORB {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for ORB {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::ORBTraitConst for ORB {
		#[inline] fn as_raw_ORB(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::ORBTrait for ORB {
		#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		/// This alternative version of [create] function uses the following default values for its arguments:
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
	pub trait SIFTTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_SIFT(&self) -> *const c_void;
	
		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getDefaultName_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_n_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getNFeatures_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_n_octave_layers(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getNOctaveLayers_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_contrast_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getContrastThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_edge_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_getEdgeThreshold_const(self.as_raw_SIFT(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_n_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setNFeatures_int(self.as_raw_mut_SIFT(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setNOctaveLayers_int(self.as_raw_mut_SIFT(), n_octave_layers, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_contrast_threshold(&mut self, contrast_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setContrastThreshold_double(self.as_raw_mut_SIFT(), contrast_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_edge_threshold(&mut self, edge_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SIFT_setEdgeThreshold_double(self.as_raw_mut_SIFT(), edge_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	/// (SIFT) algorithm by D. Lowe [Lowe04](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Lowe04) .
	pub struct SIFT {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for SIFT {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for SIFT {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::SIFTTraitConst for SIFT {
		#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::SIFTTrait for SIFT {
		#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * nfeatures: 0
		/// * n_octave_layers: 3
		/// * contrast_threshold: 0.04
		/// * edge_threshold: 10
		/// * sigma: 1.6
		/// * enable_precise_upscale: false
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
		#[inline]
		pub fn create_1(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32, enable_precise_upscale: bool) -> Result<core::Ptr<crate::features2d::SIFT>> {
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * enable_precise_upscale: false
		#[inline]
		pub fn create_def_1(nfeatures: i32, n_octave_layers: i32, contrast_threshold: f64, edge_threshold: f64, sigma: f64, descriptor_type: i32) -> Result<core::Ptr<crate::features2d::SIFT>> {
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
	pub trait SimpleBlobDetectorTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_SimpleBlobDetector(&self) -> *const c_void;
	
		#[inline]
		fn get_params(&self) -> Result<crate::features2d::SimpleBlobDetector_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_getParams_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_default_name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_getDefaultName_const(self.as_raw_SimpleBlobDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	pub struct SimpleBlobDetector {
		ptr: *mut c_void
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
	
	impl crate::features2d::Feature2DTraitConst for SimpleBlobDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for SimpleBlobDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTraitConst for SimpleBlobDetector {
		#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTrait for SimpleBlobDetector {
		#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SimpleBlobDetector {
		/// ## C++ default parameters
		/// * parameters: SimpleBlobDetector::Params()
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * parameters: SimpleBlobDetector::Params()
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
		#[inline]
		pub fn write(self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_Params_write_const_FileStorageR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::features2d::SimpleBlobDetector_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn read(self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SimpleBlobDetector_Params_read_const_FileNodeR(self.opencv_as_extern(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
}
