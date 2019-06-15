//! # 2D Features Framework
//! # Feature Detection and Description
//! # Descriptor Matchers
//!
//! Matchers of keypoint descriptors in OpenCV have wrappers with a common interface that enables you to
//! easily switch between different algorithms solving the same problem. This section is devoted to
//! matching descriptors that are represented as vectors in a multidimensional space. All objects that
//! implement vector descriptor matchers inherit the DescriptorMatcher interface.
//!
//!
//! Note:
//! *   An example explaining keypoint matching can be found at
//! opencv_source_code/samples/cpp/descriptor_extractor_matcher.cpp
//! *   An example on descriptor matching evaluation can be found at
//! opencv_source_code/samples/cpp/detector_descriptor_matcher_evaluation.cpp
//! *   An example on one to many image matching can be found at
//! opencv_source_code/samples/cpp/matching_to_many_images.cpp
//!
//! # Drawing Function of Keypoints and Matches
//! # Object Categorization
//!
//! This section describes approaches based on local 2D features and used to categorize objects.
//!
//!
//! Note:
//! *   A complete Bag-Of-Words sample can be found at
//! opencv_source_code/samples/cpp/bagofwords_classification.cpp
//! *   (Python) An example using the features2D framework to perform object categorization can be
//! found at opencv_source_code/samples/python/find_obj.py
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const AKAZE_DESCRIPTOR_KAZE: i32 = 3;
pub const AKAZE_DESCRIPTOR_KAZE_UPRIGHT: i32 = 2;
pub const AKAZE_DESCRIPTOR_MLDB: i32 = 5;
pub const AKAZE_DESCRIPTOR_MLDB_UPRIGHT: i32 = 4;
pub const AgastFeatureDetector_AGAST_5_8: i32 = 0;
pub const AgastFeatureDetector_AGAST_7_12d: i32 = 1;
pub const AgastFeatureDetector_AGAST_7_12s: i32 = 2;
pub const AgastFeatureDetector_OAST_9_16: i32 = 3;
pub const CV_HAL_TYPE_5_8: i32 = 0;
pub const CV_HAL_TYPE_7_12: i32 = 1;
pub const CV_HAL_TYPE_9_16: i32 = 2;
pub const DescriptorMatcher_BRUTEFORCE: i32 = 2;
pub const DescriptorMatcher_BRUTEFORCE_HAMMING: i32 = 4;
pub const DescriptorMatcher_BRUTEFORCE_HAMMINGLUT: i32 = 5;
pub const DescriptorMatcher_BRUTEFORCE_L1: i32 = 3;
pub const DescriptorMatcher_BRUTEFORCE_SL2: i32 = 6;
pub const DescriptorMatcher_FLANNBASED: i32 = 1;
pub const DrawMatchesFlags_DEFAULT: i32 = 0;
pub const DrawMatchesFlags_DRAW_OVER_OUTIMG: i32 = 1;
pub const DrawMatchesFlags_DRAW_RICH_KEYPOINTS: i32 = 4;
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
pub const ORB_kBytes: i32 = 32;


#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
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
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_mair2010_agast) .
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * nonmax_suppression: true
pub fn AGAST(image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, threshold: i32, nonmax_suppression: bool) -> Result<()> {
    unsafe { sys::cv_AGAST_Mat_VectorOfKeyPoint_int_bool(image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), threshold, nonmax_suppression) }.into_result()
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
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_mair2010_agast) .
pub fn AGAST_with_type(image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, threshold: i32, nonmax_suppression: bool, _type: i32) -> Result<()> {
    unsafe { sys::cv_AGAST_Mat_VectorOfKeyPoint_int_bool_int(image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), threshold, nonmax_suppression, _type) }.into_result()
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
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Rosten06) .
///
///
/// Note: In Python API, types are given as cv2.FAST_FEATURE_DETECTOR_TYPE_5_8,
/// cv2.FAST_FEATURE_DETECTOR_TYPE_7_12 and cv2.FAST_FEATURE_DETECTOR_TYPE_9_16. For corner
/// detection, use cv2.FAST.detect() method.
///
/// ## Overloaded parameters
///
/// ## C++ default parameters
/// * nonmax_suppression: true
pub fn FAST(image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, threshold: i32, nonmax_suppression: bool) -> Result<()> {
    unsafe { sys::cv_FAST_Mat_VectorOfKeyPoint_int_bool(image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), threshold, nonmax_suppression) }.into_result()
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
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Rosten06) .
///
///
/// Note: In Python API, types are given as cv2.FAST_FEATURE_DETECTOR_TYPE_5_8,
/// cv2.FAST_FEATURE_DETECTOR_TYPE_7_12 and cv2.FAST_FEATURE_DETECTOR_TYPE_9_16. For corner
/// detection, use cv2.FAST.detect() method.
pub fn FAST_with_type(image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, threshold: i32, nonmax_suppression: bool, _type: i32) -> Result<()> {
    unsafe { sys::cv_FAST_Mat_VectorOfKeyPoint_int_bool_int(image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), threshold, nonmax_suppression, _type) }.into_result()
}

pub fn compute_recall_precision_curve(matches1to2: &types::VectorOfVectorOfDMatch, correct_matches1to2_mask: &types::VectorOfVectorOfuchar, recall_precision_curve: &mut types::VectorOfPoint2f) -> Result<()> {
    unsafe { sys::cv_computeRecallPrecisionCurve_VectorOfVectorOfDMatch_VectorOfVectorOfuchar_VectorOfPoint2f(matches1to2.as_raw_VectorOfVectorOfDMatch(), correct_matches1to2_mask.as_raw_VectorOfVectorOfuchar(), recall_precision_curve.as_raw_VectorOfPoint2f()) }.into_result()
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
/// For Python API, flags are modified as cv2.DRAW_MATCHES_FLAGS_DEFAULT,
/// cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS, cv2.DRAW_MATCHES_FLAGS_DRAW_OVER_OUTIMG,
/// cv2.DRAW_MATCHES_FLAGS_NOT_DRAW_SINGLE_POINTS
///
/// ## C++ default parameters
/// * color: Scalar::all(-1)
/// * flags: DrawMatchesFlags::DEFAULT
pub fn draw_keypoints(image: &core::Mat, keypoints: &types::VectorOfKeyPoint, out_image: &mut core::Mat, color: core::Scalar, flags: i32) -> Result<()> {
    unsafe { sys::cv_drawKeypoints_Mat_VectorOfKeyPoint_Mat_Scalar_int(image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), out_image.as_raw_Mat(), color, flags) }.into_result()
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
pub fn draw_matches(img1: &core::Mat, keypoints1: &types::VectorOfKeyPoint, img2: &core::Mat, keypoints2: &types::VectorOfKeyPoint, matches1to2: &types::VectorOfDMatch, out_img: &mut core::Mat, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &types::VectorOfchar, flags: i32) -> Result<()> {
    unsafe { sys::cv_drawMatches_Mat_VectorOfKeyPoint_Mat_VectorOfKeyPoint_VectorOfDMatch_Mat_Scalar_Scalar_VectorOfchar_int(img1.as_raw_Mat(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw_Mat(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw_Mat(), match_color, single_point_color, matches_mask.as_raw_VectorOfchar(), flags) }.into_result()
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
/// * matches_mask: std::vector<std::vector<char> >()
/// * flags: DrawMatchesFlags::DEFAULT
pub fn draw_vector_matches(img1: &core::Mat, keypoints1: &types::VectorOfKeyPoint, img2: &core::Mat, keypoints2: &types::VectorOfKeyPoint, matches1to2: &types::VectorOfVectorOfDMatch, out_img: &mut core::Mat, match_color: core::Scalar, single_point_color: core::Scalar, matches_mask: &types::VectorOfVectorOfchar, flags: i32) -> Result<()> {
    unsafe { sys::cv_drawMatches_Mat_VectorOfKeyPoint_Mat_VectorOfKeyPoint_VectorOfVectorOfDMatch_Mat_Scalar_Scalar_VectorOfVectorOfchar_int(img1.as_raw_Mat(), keypoints1.as_raw_VectorOfKeyPoint(), img2.as_raw_Mat(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfVectorOfDMatch(), out_img.as_raw_Mat(), match_color, single_point_color, matches_mask.as_raw_VectorOfVectorOfchar(), flags) }.into_result()
}

/// \
///   Functions to evaluate the feature detectors and [generic] descriptor extractors      *
///
/// ## C++ default parameters
/// * fdetector: Ptr<FeatureDetector>()
pub fn evaluate_feature_detector(img1: &core::Mat, img2: &core::Mat, h1to2: &core::Mat, keypoints1: &mut types::VectorOfKeyPoint, keypoints2: &mut types::VectorOfKeyPoint, repeatability: &mut f32, corresp_count: &mut i32, fdetector: &types::PtrOfFeature2D) -> Result<()> {
    unsafe { sys::cv_evaluateFeatureDetector_Mat_Mat_Mat_VectorOfKeyPoint_VectorOfKeyPoint_float_int_PtrOfFeature2D(img1.as_raw_Mat(), img2.as_raw_Mat(), h1to2.as_raw_Mat(), keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), repeatability, corresp_count, fdetector.as_raw_PtrOfFeature2D()) }.into_result()
}

pub fn get_nearest_point(recall_precision_curve: &types::VectorOfPoint2f, l_precision: f32) -> Result<i32> {
    unsafe { sys::cv_getNearestPoint_VectorOfPoint2f_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision) }.into_result()
}

pub fn get_recall(recall_precision_curve: &types::VectorOfPoint2f, l_precision: f32) -> Result<f32> {
    unsafe { sys::cv_getRecall_VectorOfPoint2f_float(recall_precision_curve.as_raw_VectorOfPoint2f(), l_precision) }.into_result()
}

// Generating impl for trait cv::AKAZE (trait)
/// Class implementing the AKAZE keypoint detector and descriptor extractor, described in [ANB13](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_ANB13).
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
pub trait AKAZE: crate::features2d::Feature2D {
    #[inline(always)] fn as_raw_AKAZE(&self) -> *mut c_void;
    fn set_descriptor_type(&mut self, dtype: i32) -> Result<()> {
        unsafe { sys::cv_AKAZE_setDescriptorType_int(self.as_raw_AKAZE(), dtype) }.into_result()
    }
    
    fn get_descriptor_type(&self) -> Result<i32> {
        unsafe { sys::cv_AKAZE_getDescriptorType_const(self.as_raw_AKAZE()) }.into_result()
    }
    
    fn set_descriptor_size(&mut self, dsize: i32) -> Result<()> {
        unsafe { sys::cv_AKAZE_setDescriptorSize_int(self.as_raw_AKAZE(), dsize) }.into_result()
    }
    
    fn get_descriptor_size(&self) -> Result<i32> {
        unsafe { sys::cv_AKAZE_getDescriptorSize_const(self.as_raw_AKAZE()) }.into_result()
    }
    
    fn set_descriptor_channels(&mut self, dch: i32) -> Result<()> {
        unsafe { sys::cv_AKAZE_setDescriptorChannels_int(self.as_raw_AKAZE(), dch) }.into_result()
    }
    
    fn get_descriptor_channels(&self) -> Result<i32> {
        unsafe { sys::cv_AKAZE_getDescriptorChannels_const(self.as_raw_AKAZE()) }.into_result()
    }
    
    fn set_threshold(&mut self, threshold: f64) -> Result<()> {
        unsafe { sys::cv_AKAZE_setThreshold_double(self.as_raw_AKAZE(), threshold) }.into_result()
    }
    
    fn get_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_AKAZE_getThreshold_const(self.as_raw_AKAZE()) }.into_result()
    }
    
    fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
        unsafe { sys::cv_AKAZE_setNOctaves_int(self.as_raw_AKAZE(), octaves) }.into_result()
    }
    
    fn get_n_octaves(&self) -> Result<i32> {
        unsafe { sys::cv_AKAZE_getNOctaves_const(self.as_raw_AKAZE()) }.into_result()
    }
    
    fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
        unsafe { sys::cv_AKAZE_setNOctaveLayers_int(self.as_raw_AKAZE(), octave_layers) }.into_result()
    }
    
    fn get_n_octave_layers(&self) -> Result<i32> {
        unsafe { sys::cv_AKAZE_getNOctaveLayers_const(self.as_raw_AKAZE()) }.into_result()
    }
    
    fn set_diffusivity(&mut self, diff: i32) -> Result<()> {
        unsafe { sys::cv_AKAZE_setDiffusivity_int(self.as_raw_AKAZE(), diff) }.into_result()
    }
    
    fn get_diffusivity(&self) -> Result<i32> {
        unsafe { sys::cv_AKAZE_getDiffusivity_const(self.as_raw_AKAZE()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_AKAZE_getDefaultName_const(self.as_raw_AKAZE()) }.into_result().map(crate::templ::receive_string_mut)
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
    pub fn create(descriptor_type: i32, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32) -> Result<types::PtrOfAKAZE> {
        unsafe { sys::cv_AKAZE_create_int_int_int_float_int_int_int(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity) }.into_result().map(|ptr| types::PtrOfAKAZE { ptr })
    }
    
}

// Generating impl for trait cv::AgastFeatureDetector (trait)
/// Wrapping class for feature detection using the AGAST method. :
pub trait AgastFeatureDetector: crate::features2d::Feature2D {
    #[inline(always)] fn as_raw_AgastFeatureDetector(&self) -> *mut c_void;
    fn set_threshold(&mut self, threshold: i32) -> Result<()> {
        unsafe { sys::cv_AgastFeatureDetector_setThreshold_int(self.as_raw_AgastFeatureDetector(), threshold) }.into_result()
    }
    
    fn get_threshold(&self) -> Result<i32> {
        unsafe { sys::cv_AgastFeatureDetector_getThreshold_const(self.as_raw_AgastFeatureDetector()) }.into_result()
    }
    
    fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
        unsafe { sys::cv_AgastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_AgastFeatureDetector(), f) }.into_result()
    }
    
    fn get_nonmax_suppression(&self) -> Result<bool> {
        unsafe { sys::cv_AgastFeatureDetector_getNonmaxSuppression_const(self.as_raw_AgastFeatureDetector()) }.into_result()
    }
    
    fn set_type(&mut self, _type: i32) -> Result<()> {
        unsafe { sys::cv_AgastFeatureDetector_setType_int(self.as_raw_AgastFeatureDetector(), _type) }.into_result()
    }
    
    fn get_type(&self) -> Result<i32> {
        unsafe { sys::cv_AgastFeatureDetector_getType_const(self.as_raw_AgastFeatureDetector()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_AgastFeatureDetector_getDefaultName_const(self.as_raw_AgastFeatureDetector()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

impl dyn AgastFeatureDetector + '_ {

    ///
    /// ## C++ default parameters
    /// * threshold: 10
    /// * nonmax_suppression: true
    /// * _type: AgastFeatureDetector::OAST_9_16
    pub fn create(threshold: i32, nonmax_suppression: bool, _type: i32) -> Result<types::PtrOfAgastFeatureDetector> {
        unsafe { sys::cv_AgastFeatureDetector_create_int_bool_int(threshold, nonmax_suppression, _type) }.into_result().map(|ptr| types::PtrOfAgastFeatureDetector { ptr })
    }
    
}

// boxed class cv::BFMatcher
/// Brute-force descriptor matcher.
///
/// For each descriptor in the first set, this matcher finds the closest descriptor in the second set
/// by trying each one. This descriptor matcher supports masking permissible matches of descriptor
/// sets.
#[allow(dead_code)]
pub struct BFMatcher {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::BFMatcher {
    fn drop(&mut self) {
        unsafe { sys::cv_BFMatcher_delete(self.ptr) };
    }
}
impl crate::features2d::BFMatcher {
    #[inline(always)] pub fn as_raw_BFMatcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl core::Algorithm for BFMatcher {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::features2d::DescriptorMatcher for BFMatcher {
    #[inline(always)] fn as_raw_DescriptorMatcher(&self) -> *mut c_void { self.ptr }
}

impl BFMatcher {

    /// Brute-force matcher constructor (obsolete). Please use BFMatcher.create()
    ///
    /// ## C++ default parameters
    /// * norm_type: NORM_L2
    /// * cross_check: false
    pub fn new(norm_type: i32, cross_check: bool) -> Result<crate::features2d::BFMatcher> {
        unsafe { sys::cv_BFMatcher_BFMatcher_int_bool(norm_type, cross_check) }.into_result().map(|ptr| crate::features2d::BFMatcher { ptr })
    }
    
    pub fn is_mask_supported(&self) -> Result<bool> {
        unsafe { sys::cv_BFMatcher_isMaskSupported_const(self.as_raw_BFMatcher()) }.into_result()
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
    pub fn create(norm_type: i32, cross_check: bool) -> Result<types::PtrOfBFMatcher> {
        unsafe { sys::cv_BFMatcher_create_int_bool(norm_type, cross_check) }.into_result().map(|ptr| types::PtrOfBFMatcher { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * empty_train_data: false
    pub fn clone(&self, empty_train_data: bool) -> Result<types::PtrOfDescriptorMatcher> {
        unsafe { sys::cv_BFMatcher_clone_const_bool(self.as_raw_BFMatcher(), empty_train_data) }.into_result().map(|ptr| types::PtrOfDescriptorMatcher { ptr })
    }
    
}

// boxed class cv::BOWImgDescriptorExtractor
/// Class to compute an image descriptor using the *bag of visual words*.
///
/// Such a computation consists of the following steps:
///
/// 1.  Compute descriptors for a given image and its keypoints set.
/// 2.  Find the nearest visual words from the vocabulary for each keypoint descriptor.
/// 3.  Compute the bag-of-words image descriptor as is a normalized histogram of vocabulary words
/// encountered in the image. The i-th bin of the histogram is a frequency of i-th word of the
/// vocabulary in the given image.
#[allow(dead_code)]
pub struct BOWImgDescriptorExtractor {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::BOWImgDescriptorExtractor {
    fn drop(&mut self) {
        unsafe { sys::cv_BOWImgDescriptorExtractor_delete(self.ptr) };
    }
}
impl crate::features2d::BOWImgDescriptorExtractor {
    #[inline(always)] pub fn as_raw_BOWImgDescriptorExtractor(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl BOWImgDescriptorExtractor {

    /// The constructor.
    ///
    /// ## Parameters
    /// * dextractor: Descriptor extractor that is used to compute descriptors for an input image and
    /// its keypoints.
    /// * dmatcher: Descriptor matcher that is used to find the nearest word of the trained vocabulary
    /// for each keypoint descriptor of the image.
    pub fn new_with_dextractor(dextractor: &types::PtrOfFeature2D, dmatcher: &types::PtrOfDescriptorMatcher) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_PtrOfFeature2D_PtrOfDescriptorMatcher(dextractor.as_raw_PtrOfFeature2D(), dmatcher.as_raw_PtrOfDescriptorMatcher()) }.into_result().map(|ptr| crate::features2d::BOWImgDescriptorExtractor { ptr })
    }
    
    pub fn new(dmatcher: &types::PtrOfDescriptorMatcher) -> Result<crate::features2d::BOWImgDescriptorExtractor> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_PtrOfDescriptorMatcher(dmatcher.as_raw_PtrOfDescriptorMatcher()) }.into_result().map(|ptr| crate::features2d::BOWImgDescriptorExtractor { ptr })
    }
    
    /// Sets a visual vocabulary.
    ///
    /// ## Parameters
    /// * vocabulary: Vocabulary (can be trained using the inheritor of BOWTrainer ). Each row of the
    /// vocabulary is a visual word (cluster center).
    pub fn set_vocabulary(&mut self, vocabulary: &core::Mat) -> Result<()> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_setVocabulary_Mat(self.as_raw_BOWImgDescriptorExtractor(), vocabulary.as_raw_Mat()) }.into_result()
    }
    
    /// Returns the set vocabulary.
    pub fn get_vocabulary(&self) -> Result<core::Mat> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_getVocabulary_const(self.as_raw_BOWImgDescriptorExtractor()) }.into_result().map(|ptr| core::Mat { ptr })
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
    pub fn compute_desc(&mut self, image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, img_descriptor: &mut core::Mat, point_idxs_of_clusters: &mut types::VectorOfVectorOfint, descriptors: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_compute_Mat_VectorOfKeyPoint_Mat_VectorOfVectorOfint_Mat(self.as_raw_BOWImgDescriptorExtractor(), image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), img_descriptor.as_raw_Mat(), point_idxs_of_clusters.as_raw_VectorOfVectorOfint(), descriptors.as_raw_Mat()) }.into_result()
    }
    
    /// ## Parameters
    /// * keypointDescriptors: Computed descriptors to match with vocabulary.
    /// * imgDescriptor: Computed output image descriptor.
    /// * pointIdxsOfClusters: Indices of keypoints that belong to the cluster. This means that
    /// pointIdxsOfClusters[i] are keypoint indices that belong to the i -th cluster (word of vocabulary)
    /// returned if it is non-zero.
    ///
    /// ## C++ default parameters
    /// * point_idxs_of_clusters: 0
    pub fn compute(&mut self, keypoint_descriptors: &core::Mat, img_descriptor: &mut core::Mat, point_idxs_of_clusters: &mut types::VectorOfVectorOfint) -> Result<()> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_compute_Mat_Mat_VectorOfVectorOfint(self.as_raw_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw_Mat(), img_descriptor.as_raw_Mat(), point_idxs_of_clusters.as_raw_VectorOfVectorOfint()) }.into_result()
    }
    
    pub fn compute2(&mut self, image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, img_descriptor: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_compute2_Mat_VectorOfKeyPoint_Mat(self.as_raw_BOWImgDescriptorExtractor(), image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), img_descriptor.as_raw_Mat()) }.into_result()
    }
    
    /// Returns an image descriptor size if the vocabulary is set. Otherwise, it returns 0.
    pub fn descriptor_size(&self) -> Result<i32> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorSize_const(self.as_raw_BOWImgDescriptorExtractor()) }.into_result()
    }
    
    /// Returns an image descriptor type.
    pub fn descriptor_type(&self) -> Result<i32> {
        unsafe { sys::cv_BOWImgDescriptorExtractor_descriptorType_const(self.as_raw_BOWImgDescriptorExtractor()) }.into_result()
    }
    
}

// boxed class cv::BOWKMeansTrainer
/// kmeans -based class to train visual vocabulary using the *bag of visual words* approach. :
#[allow(dead_code)]
pub struct BOWKMeansTrainer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::BOWKMeansTrainer {
    fn drop(&mut self) {
        unsafe { sys::cv_BOWKMeansTrainer_delete(self.ptr) };
    }
}
impl crate::features2d::BOWKMeansTrainer {
    #[inline(always)] pub fn as_raw_BOWKMeansTrainer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl crate::features2d::BOWTrainer for BOWKMeansTrainer {
    #[inline(always)] fn as_raw_BOWTrainer(&self) -> *mut c_void { self.ptr }
}

impl BOWKMeansTrainer {

    /// The constructor.
    ///
    /// @see cv::kmeans
    ///
    /// ## C++ default parameters
    /// * termcrit: TermCriteria()
    /// * attempts: 3
    /// * flags: KMEANS_PP_CENTERS
    pub fn new_with_criteria(cluster_count: i32, termcrit: &core::TermCriteria, attempts: i32, flags: i32) -> Result<crate::features2d::BOWKMeansTrainer> {
        unsafe { sys::cv_BOWKMeansTrainer_BOWKMeansTrainer_int_TermCriteria_int_int(cluster_count, termcrit.as_raw_TermCriteria(), attempts, flags) }.into_result().map(|ptr| crate::features2d::BOWKMeansTrainer { ptr })
    }
    
    pub fn default(&self) -> Result<core::Mat> {
        unsafe { sys::cv_BOWKMeansTrainer_cluster_const(self.as_raw_BOWKMeansTrainer()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    pub fn new(&self, descriptors: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_BOWKMeansTrainer_cluster_const_Mat(self.as_raw_BOWKMeansTrainer(), descriptors.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// Generating impl for trait cv::BOWTrainer (trait)
/// Abstract base class for training the *bag of visual words* vocabulary from a set of descriptors.
///
/// For details, see, for example, *Visual Categorization with Bags of Keypoints* by Gabriella Csurka,
/// Christopher R. Dance, Lixin Fan, Jutta Willamowski, Cedric Bray, 2004. :
pub trait BOWTrainer {
    #[inline(always)] fn as_raw_BOWTrainer(&self) -> *mut c_void;
    /// Adds descriptors to a training set.
    ///
    /// ## Parameters
    /// * descriptors: Descriptors to add to a training set. Each row of the descriptors matrix is a
    /// descriptor.
    ///
    /// The training set is clustered using clustermethod to construct the vocabulary.
    fn add(&mut self, descriptors: &core::Mat) -> Result<()> {
        unsafe { sys::cv_BOWTrainer_add_Mat(self.as_raw_BOWTrainer(), descriptors.as_raw_Mat()) }.into_result()
    }
    
    /// Returns a training set of descriptors.
    fn get_descriptors(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_BOWTrainer_getDescriptors_const(self.as_raw_BOWTrainer()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    /// Returns the count of all descriptors stored in the training set.
    fn descriptors_count(&self) -> Result<i32> {
        unsafe { sys::cv_BOWTrainer_descriptorsCount_const(self.as_raw_BOWTrainer()) }.into_result()
    }
    
    fn clear(&mut self) -> Result<()> {
        unsafe { sys::cv_BOWTrainer_clear(self.as_raw_BOWTrainer()) }.into_result()
    }
    
    fn cluster(&self) -> Result<core::Mat> {
        unsafe { sys::cv_BOWTrainer_cluster_const(self.as_raw_BOWTrainer()) }.into_result().map(|ptr| core::Mat { ptr })
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
        unsafe { sys::cv_BOWTrainer_cluster_const_Mat(self.as_raw_BOWTrainer(), descriptors.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::BRISK
/// Class implementing the BRISK keypoint detector and descriptor extractor, described in [LCS11](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_LCS11) .
#[allow(dead_code)]
pub struct BRISK {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::BRISK {
    fn drop(&mut self) {
        unsafe { sys::cv_BRISK_delete(self.ptr) };
    }
}
impl crate::features2d::BRISK {
    #[inline(always)] pub fn as_raw_BRISK(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl core::Algorithm for BRISK {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::features2d::Feature2D for BRISK {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void { self.ptr }
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
    pub fn create(thresh: i32, octaves: i32, pattern_scale: f32) -> Result<types::PtrOfBRISK> {
        unsafe { sys::cv_BRISK_create_int_int_float(thresh, octaves, pattern_scale) }.into_result().map(|ptr| types::PtrOfBRISK { ptr })
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
    pub fn create_with_pattern(radius_list: &types::VectorOffloat, number_list: &types::VectorOfint, d_max: f32, d_min: f32, index_change: &types::VectorOfint) -> Result<types::PtrOfBRISK> {
        unsafe { sys::cv_BRISK_create_VectorOffloat_VectorOfint_float_float_VectorOfint(radius_list.as_raw_VectorOffloat(), number_list.as_raw_VectorOfint(), d_max, d_min, index_change.as_raw_VectorOfint()) }.into_result().map(|ptr| types::PtrOfBRISK { ptr })
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
    pub fn create_with_pattern_threshold_octaves(thresh: i32, octaves: i32, radius_list: &types::VectorOffloat, number_list: &types::VectorOfint, d_max: f32, d_min: f32, index_change: &types::VectorOfint) -> Result<types::PtrOfBRISK> {
        unsafe { sys::cv_BRISK_create_int_int_VectorOffloat_VectorOfint_float_float_VectorOfint(thresh, octaves, radius_list.as_raw_VectorOffloat(), number_list.as_raw_VectorOfint(), d_max, d_min, index_change.as_raw_VectorOfint()) }.into_result().map(|ptr| types::PtrOfBRISK { ptr })
    }
    
    pub fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_BRISK_getDefaultName_const(self.as_raw_BRISK()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

// Generating impl for trait cv::DescriptorMatcher (trait)
/// Abstract base class for matching keypoint descriptors.
///
/// It has two groups of match methods: for matching descriptors of an image with another image or with
/// an image set.
pub trait DescriptorMatcher: core::Algorithm {
    #[inline(always)] fn as_raw_DescriptorMatcher(&self) -> *mut c_void;
    /// Adds descriptors to train a CPU(trainDescCollectionis) or GPU(utrainDescCollectionis) descriptor
    /// collection.
    ///
    /// If the collection is not empty, the new descriptors are added to existing train descriptors.
    ///
    /// ## Parameters
    /// * descriptors: Descriptors to add. Each descriptors[i] is a set of descriptors from the same
    /// train image.
    fn add(&mut self, descriptors: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_add_VectorOfMat(self.as_raw_DescriptorMatcher(), descriptors.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Returns a constant link to the train descriptor collection trainDescCollection .
    fn get_train_descriptors(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_DescriptorMatcher_getTrainDescriptors_const(self.as_raw_DescriptorMatcher()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    /// Clears the train descriptor collections.
    fn clear(&mut self) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_clear(self.as_raw_DescriptorMatcher()) }.into_result()
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
        unsafe { sys::cv_DescriptorMatcher_train(self.as_raw_DescriptorMatcher()) }.into_result()
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
    fn train_matches(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut types::VectorOfDMatch, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_match_const_Mat_Mat_VectorOfDMatch_Mat(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_VectorOfDMatch(), mask.as_raw_Mat()) }.into_result()
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
    fn knn_train_matches(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, k: i32, mask: &core::Mat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_knnMatch_const_Mat_Mat_VectorOfVectorOfDMatch_int_Mat_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), k, mask.as_raw_Mat(), compact_result) }.into_result()
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
    fn train_radius_matches(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, max_distance: f32, mask: &core::Mat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_radiusMatch_const_Mat_Mat_VectorOfVectorOfDMatch_float_Mat_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), max_distance, mask.as_raw_Mat(), compact_result) }.into_result()
    }
    
    /// ## Parameters
    /// * queryDescriptors: Query set of descriptors.
    /// * matches: Matches. If a query descriptor is masked out in mask , no match is added for this
    /// descriptor. So, matches size may be smaller than the query descriptors count.
    /// * masks: Set of masks. Each masks[i] specifies permissible matches between the input query
    /// descriptors and stored train descriptors from the i-th image trainDescCollection[i].
    ///
    /// ## C++ default parameters
    /// * masks: noArray()
    fn matches(&mut self, query_descriptors: &core::Mat, matches: &mut types::VectorOfDMatch, masks: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_match_Mat_VectorOfDMatch_VectorOfMat(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_VectorOfDMatch(), masks.as_raw_VectorOfMat()) }.into_result()
    }
    
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
    /// ## C++ default parameters
    /// * masks: noArray()
    /// * compact_result: false
    fn knn_matches(&mut self, query_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, k: i32, masks: &types::VectorOfMat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_knnMatch_Mat_VectorOfVectorOfDMatch_int_VectorOfMat_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), k, masks.as_raw_VectorOfMat(), compact_result) }.into_result()
    }
    
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
    /// ## C++ default parameters
    /// * masks: noArray()
    /// * compact_result: false
    fn radius_matches(&mut self, query_descriptors: &core::Mat, matches: &mut types::VectorOfVectorOfDMatch, max_distance: f32, masks: &types::VectorOfMat, compact_result: bool) -> Result<()> {
        unsafe { sys::cv_DescriptorMatcher_radiusMatch_Mat_VectorOfVectorOfDMatch_float_VectorOfMat_bool(self.as_raw_DescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_VectorOfVectorOfDMatch(), max_distance, masks.as_raw_VectorOfMat(), compact_result) }.into_result()
    }
    
    fn write(&self, file_name: &str) -> Result<()> {
        string_arg!(file_name);
        unsafe { sys::cv_DescriptorMatcher_write_const_String(self.as_raw_DescriptorMatcher(), file_name.as_ptr()) }.into_result()
    }
    
    fn read(&mut self, file_name: &str) -> Result<()> {
        string_arg!(file_name);
        unsafe { sys::cv_DescriptorMatcher_read_String(self.as_raw_DescriptorMatcher(), file_name.as_ptr()) }.into_result()
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
    fn clone(&self, empty_train_data: bool) -> Result<types::PtrOfDescriptorMatcher> {
        unsafe { sys::cv_DescriptorMatcher_clone_const_bool(self.as_raw_DescriptorMatcher(), empty_train_data) }.into_result().map(|ptr| types::PtrOfDescriptorMatcher { ptr })
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
    pub fn create(descriptor_matcher_type: &str) -> Result<types::PtrOfDescriptorMatcher> {
        string_arg!(descriptor_matcher_type);
        unsafe { sys::cv_DescriptorMatcher_create_String(descriptor_matcher_type.as_ptr()) }.into_result().map(|ptr| types::PtrOfDescriptorMatcher { ptr })
    }
    
    pub fn create_with_matcher_type(matcher_type: i32) -> Result<types::PtrOfDescriptorMatcher> {
        unsafe { sys::cv_DescriptorMatcher_create_int(matcher_type) }.into_result().map(|ptr| types::PtrOfDescriptorMatcher { ptr })
    }
    
}

// boxed class cv::DrawMatchesFlags
/// \
///                                   Drawing functions                                    *
#[allow(dead_code)]
pub struct DrawMatchesFlags {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::DrawMatchesFlags {
    fn drop(&mut self) {
        unsafe { sys::cv_DrawMatchesFlags_delete(self.ptr) };
    }
}
impl crate::features2d::DrawMatchesFlags {
    #[inline(always)] pub fn as_raw_DrawMatchesFlags(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

// Generating impl for trait cv::FastFeatureDetector (trait)
/// Wrapping class for feature detection using the FAST method. :
pub trait FastFeatureDetector: crate::features2d::Feature2D {
    #[inline(always)] fn as_raw_FastFeatureDetector(&self) -> *mut c_void;
    fn set_threshold(&mut self, threshold: i32) -> Result<()> {
        unsafe { sys::cv_FastFeatureDetector_setThreshold_int(self.as_raw_FastFeatureDetector(), threshold) }.into_result()
    }
    
    fn get_threshold(&self) -> Result<i32> {
        unsafe { sys::cv_FastFeatureDetector_getThreshold_const(self.as_raw_FastFeatureDetector()) }.into_result()
    }
    
    fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
        unsafe { sys::cv_FastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_FastFeatureDetector(), f) }.into_result()
    }
    
    fn get_nonmax_suppression(&self) -> Result<bool> {
        unsafe { sys::cv_FastFeatureDetector_getNonmaxSuppression_const(self.as_raw_FastFeatureDetector()) }.into_result()
    }
    
    fn set_type(&mut self, _type: i32) -> Result<()> {
        unsafe { sys::cv_FastFeatureDetector_setType_int(self.as_raw_FastFeatureDetector(), _type) }.into_result()
    }
    
    fn get_type(&self) -> Result<i32> {
        unsafe { sys::cv_FastFeatureDetector_getType_const(self.as_raw_FastFeatureDetector()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_FastFeatureDetector_getDefaultName_const(self.as_raw_FastFeatureDetector()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

impl dyn FastFeatureDetector + '_ {

    ///
    /// ## C++ default parameters
    /// * threshold: 10
    /// * nonmax_suppression: true
    /// * _type: FastFeatureDetector::TYPE_9_16
    pub fn create(threshold: i32, nonmax_suppression: bool, _type: i32) -> Result<types::PtrOfFastFeatureDetector> {
        unsafe { sys::cv_FastFeatureDetector_create_int_bool_int(threshold, nonmax_suppression, _type) }.into_result().map(|ptr| types::PtrOfFastFeatureDetector { ptr })
    }
    
}

// Generating impl for trait cv::Feature2D (trait)
/// Abstract base class for 2D image feature detectors and descriptor extractors
pub trait Feature2D: core::Algorithm {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void;
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
    fn detect(&mut self, image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_Feature2D_detect_Mat_VectorOfKeyPoint_Mat(self.as_raw_Feature2D(), image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), mask.as_raw_Mat()) }.into_result()
    }
    
    /// ## Parameters
    /// * images: Image set.
    /// * keypoints: The detected keypoints. In the second variant of the method keypoints[i] is a set
    /// of keypoints detected in images[i] .
    /// * masks: Masks for each input image specifying where to look for keypoints (optional).
    /// masks[i] is a mask for images[i].
    ///
    /// ## C++ default parameters
    /// * masks: noArray()
    fn detect_multiple(&mut self, images: &types::VectorOfMat, keypoints: &mut types::VectorOfVectorOfKeyPoint, masks: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_Feature2D_detect_VectorOfMat_VectorOfVectorOfKeyPoint_VectorOfMat(self.as_raw_Feature2D(), images.as_raw_VectorOfMat(), keypoints.as_raw_VectorOfVectorOfKeyPoint(), masks.as_raw_VectorOfMat()) }.into_result()
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
    fn compute(&mut self, image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, descriptors: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_Feature2D_compute_Mat_VectorOfKeyPoint_Mat(self.as_raw_Feature2D(), image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), descriptors.as_raw_Mat()) }.into_result()
    }
    
    /// ## Parameters
    /// * images: Image set.
    /// * keypoints: Input collection of keypoints. Keypoints for which a descriptor cannot be
    /// computed are removed. Sometimes new keypoints can be added, for example: SIFT duplicates keypoint
    /// with several dominant orientations (for each orientation).
    /// * descriptors: Computed descriptors. In the second variant of the method descriptors[i] are
    /// descriptors computed for a keypoints[i]. Row j is the keypoints (or keypoints[i]) is the
    /// descriptor for keypoint j-th keypoint.
    fn compute_multiple(&mut self, images: &types::VectorOfMat, keypoints: &mut types::VectorOfVectorOfKeyPoint, descriptors: &mut types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_Feature2D_compute_VectorOfMat_VectorOfVectorOfKeyPoint_VectorOfMat(self.as_raw_Feature2D(), images.as_raw_VectorOfMat(), keypoints.as_raw_VectorOfVectorOfKeyPoint(), descriptors.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Detects keypoints and computes the descriptors
    ///
    /// ## C++ default parameters
    /// * use_provided_keypoints: false
    fn detect_and_compute(&mut self, image: &core::Mat, mask: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, descriptors: &mut core::Mat, use_provided_keypoints: bool) -> Result<()> {
        unsafe { sys::cv_Feature2D_detectAndCompute_Mat_Mat_VectorOfKeyPoint_Mat_bool(self.as_raw_Feature2D(), image.as_raw_Mat(), mask.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), descriptors.as_raw_Mat(), use_provided_keypoints) }.into_result()
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
        string_arg!(file_name);
        unsafe { sys::cv_Feature2D_write_const_String(self.as_raw_Feature2D(), file_name.as_ptr()) }.into_result()
    }
    
    fn read(&mut self, file_name: &str) -> Result<()> {
        string_arg!(file_name);
        unsafe { sys::cv_Feature2D_read_String(self.as_raw_Feature2D(), file_name.as_ptr()) }.into_result()
    }
    
    /// Return true if detector object is empty
    fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_Feature2D_empty_const(self.as_raw_Feature2D()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_Feature2D_getDefaultName_const(self.as_raw_Feature2D()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

// boxed class cv::FlannBasedMatcher
/// Flann-based descriptor matcher.
///
/// This matcher trains cv::flann::Index on a train descriptor collection and calls its nearest search
/// methods to find the best matches. So, this matcher may be faster when matching a large train
/// collection than the brute force matcher. FlannBasedMatcher does not support masking permissible
/// matches of descriptor sets because flann::Index does not support this. :
#[allow(dead_code)]
pub struct FlannBasedMatcher {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::FlannBasedMatcher {
    fn drop(&mut self) {
        unsafe { sys::cv_FlannBasedMatcher_delete(self.ptr) };
    }
}
impl crate::features2d::FlannBasedMatcher {
    #[inline(always)] pub fn as_raw_FlannBasedMatcher(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl core::Algorithm for FlannBasedMatcher {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::features2d::DescriptorMatcher for FlannBasedMatcher {
    #[inline(always)] fn as_raw_DescriptorMatcher(&self) -> *mut c_void { self.ptr }
}

impl FlannBasedMatcher {

    pub fn add(&mut self, descriptors: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_FlannBasedMatcher_add_VectorOfMat(self.as_raw_FlannBasedMatcher(), descriptors.as_raw_VectorOfMat()) }.into_result()
    }
    
    pub fn clear(&mut self) -> Result<()> {
        unsafe { sys::cv_FlannBasedMatcher_clear(self.as_raw_FlannBasedMatcher()) }.into_result()
    }
    
    pub fn train(&mut self) -> Result<()> {
        unsafe { sys::cv_FlannBasedMatcher_train(self.as_raw_FlannBasedMatcher()) }.into_result()
    }
    
    pub fn is_mask_supported(&self) -> Result<bool> {
        unsafe { sys::cv_FlannBasedMatcher_isMaskSupported_const(self.as_raw_FlannBasedMatcher()) }.into_result()
    }
    
    pub fn create() -> Result<types::PtrOfFlannBasedMatcher> {
        unsafe { sys::cv_FlannBasedMatcher_create() }.into_result().map(|ptr| types::PtrOfFlannBasedMatcher { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * empty_train_data: false
    pub fn clone(&self, empty_train_data: bool) -> Result<types::PtrOfDescriptorMatcher> {
        unsafe { sys::cv_FlannBasedMatcher_clone_const_bool(self.as_raw_FlannBasedMatcher(), empty_train_data) }.into_result().map(|ptr| types::PtrOfDescriptorMatcher { ptr })
    }
    
}

// Generating impl for trait cv::GFTTDetector (trait)
/// Wrapping class for feature detection using the goodFeaturesToTrack function. :
pub trait GFTTDetector: crate::features2d::Feature2D {
    #[inline(always)] fn as_raw_GFTTDetector(&self) -> *mut c_void;
    fn set_max_features(&mut self, max_features: i32) -> Result<()> {
        unsafe { sys::cv_GFTTDetector_setMaxFeatures_int(self.as_raw_GFTTDetector(), max_features) }.into_result()
    }
    
    fn get_max_features(&self) -> Result<i32> {
        unsafe { sys::cv_GFTTDetector_getMaxFeatures_const(self.as_raw_GFTTDetector()) }.into_result()
    }
    
    fn set_quality_level(&mut self, qlevel: f64) -> Result<()> {
        unsafe { sys::cv_GFTTDetector_setQualityLevel_double(self.as_raw_GFTTDetector(), qlevel) }.into_result()
    }
    
    fn get_quality_level(&self) -> Result<f64> {
        unsafe { sys::cv_GFTTDetector_getQualityLevel_const(self.as_raw_GFTTDetector()) }.into_result()
    }
    
    fn set_min_distance(&mut self, min_distance: f64) -> Result<()> {
        unsafe { sys::cv_GFTTDetector_setMinDistance_double(self.as_raw_GFTTDetector(), min_distance) }.into_result()
    }
    
    fn get_min_distance(&self) -> Result<f64> {
        unsafe { sys::cv_GFTTDetector_getMinDistance_const(self.as_raw_GFTTDetector()) }.into_result()
    }
    
    fn set_block_size(&mut self, block_size: i32) -> Result<()> {
        unsafe { sys::cv_GFTTDetector_setBlockSize_int(self.as_raw_GFTTDetector(), block_size) }.into_result()
    }
    
    fn get_block_size(&self) -> Result<i32> {
        unsafe { sys::cv_GFTTDetector_getBlockSize_const(self.as_raw_GFTTDetector()) }.into_result()
    }
    
    fn set_harris_detector(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_GFTTDetector_setHarrisDetector_bool(self.as_raw_GFTTDetector(), val) }.into_result()
    }
    
    fn get_harris_detector(&self) -> Result<bool> {
        unsafe { sys::cv_GFTTDetector_getHarrisDetector_const(self.as_raw_GFTTDetector()) }.into_result()
    }
    
    fn set_k(&mut self, k: f64) -> Result<()> {
        unsafe { sys::cv_GFTTDetector_setK_double(self.as_raw_GFTTDetector(), k) }.into_result()
    }
    
    fn get_k(&self) -> Result<f64> {
        unsafe { sys::cv_GFTTDetector_getK_const(self.as_raw_GFTTDetector()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_GFTTDetector_getDefaultName_const(self.as_raw_GFTTDetector()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

impl dyn GFTTDetector + '_ {

    ///
    /// ## C++ default parameters
    /// * max_corners: 1000
    /// * quality_level: 0.01
    /// * min_distance: 1
    /// * block_size: 3
    /// * use_harris_detector: false
    /// * k: 0.04
    pub fn create(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, use_harris_detector: bool, k: f64) -> Result<types::PtrOfGFTTDetector> {
        unsafe { sys::cv_GFTTDetector_create_int_double_double_int_bool_double(max_corners, quality_level, min_distance, block_size, use_harris_detector, k) }.into_result().map(|ptr| types::PtrOfGFTTDetector { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * use_harris_detector: false
    /// * k: 0.04
    pub fn create_with_gradient(max_corners: i32, quality_level: f64, min_distance: f64, block_size: i32, gradiant_size: i32, use_harris_detector: bool, k: f64) -> Result<types::PtrOfGFTTDetector> {
        unsafe { sys::cv_GFTTDetector_create_int_double_double_int_int_bool_double(max_corners, quality_level, min_distance, block_size, gradiant_size, use_harris_detector, k) }.into_result().map(|ptr| types::PtrOfGFTTDetector { ptr })
    }
    
}

// Generating impl for trait cv::KAZE (trait)
/// Class implementing the KAZE keypoint detector and descriptor extractor, described in [ABD12](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_ABD12) .
///
///
/// Note: AKAZE descriptor can only be used with KAZE or AKAZE keypoints .. [ABD12] KAZE Features. Pablo
/// F. Alcantarilla, Adrien Bartoli and Andrew J. Davison. In European Conference on Computer Vision
/// (ECCV), Fiorenze, Italy, October 2012.
pub trait KAZE: crate::features2d::Feature2D {
    #[inline(always)] fn as_raw_KAZE(&self) -> *mut c_void;
    fn set_extended(&mut self, extended: bool) -> Result<()> {
        unsafe { sys::cv_KAZE_setExtended_bool(self.as_raw_KAZE(), extended) }.into_result()
    }
    
    fn get_extended(&self) -> Result<bool> {
        unsafe { sys::cv_KAZE_getExtended_const(self.as_raw_KAZE()) }.into_result()
    }
    
    fn set_upright(&mut self, upright: bool) -> Result<()> {
        unsafe { sys::cv_KAZE_setUpright_bool(self.as_raw_KAZE(), upright) }.into_result()
    }
    
    fn get_upright(&self) -> Result<bool> {
        unsafe { sys::cv_KAZE_getUpright_const(self.as_raw_KAZE()) }.into_result()
    }
    
    fn set_threshold(&mut self, threshold: f64) -> Result<()> {
        unsafe { sys::cv_KAZE_setThreshold_double(self.as_raw_KAZE(), threshold) }.into_result()
    }
    
    fn get_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_KAZE_getThreshold_const(self.as_raw_KAZE()) }.into_result()
    }
    
    fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
        unsafe { sys::cv_KAZE_setNOctaves_int(self.as_raw_KAZE(), octaves) }.into_result()
    }
    
    fn get_n_octaves(&self) -> Result<i32> {
        unsafe { sys::cv_KAZE_getNOctaves_const(self.as_raw_KAZE()) }.into_result()
    }
    
    fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
        unsafe { sys::cv_KAZE_setNOctaveLayers_int(self.as_raw_KAZE(), octave_layers) }.into_result()
    }
    
    fn get_n_octave_layers(&self) -> Result<i32> {
        unsafe { sys::cv_KAZE_getNOctaveLayers_const(self.as_raw_KAZE()) }.into_result()
    }
    
    fn set_diffusivity(&mut self, diff: i32) -> Result<()> {
        unsafe { sys::cv_KAZE_setDiffusivity_int(self.as_raw_KAZE(), diff) }.into_result()
    }
    
    fn get_diffusivity(&self) -> Result<i32> {
        unsafe { sys::cv_KAZE_getDiffusivity_const(self.as_raw_KAZE()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_KAZE_getDefaultName_const(self.as_raw_KAZE()) }.into_result().map(crate::templ::receive_string_mut)
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
    pub fn create(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32) -> Result<types::PtrOfKAZE> {
        unsafe { sys::cv_KAZE_create_bool_bool_float_int_int_int(extended, upright, threshold, n_octaves, n_octave_layers, diffusivity) }.into_result().map(|ptr| types::PtrOfKAZE { ptr })
    }
    
}

// boxed class cv::KeyPointsFilter
/// A class filters a vector of keypoints.
///
/// Because now it is difficult to provide a convenient interface for all usage scenarios of the
/// keypoints filter class, it has only several needed by now static methods.
#[allow(dead_code)]
pub struct KeyPointsFilter {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::KeyPointsFilter {
    fn drop(&mut self) {
        unsafe { sys::cv_KeyPointsFilter_delete(self.ptr) };
    }
}
impl crate::features2d::KeyPointsFilter {
    #[inline(always)] pub fn as_raw_KeyPointsFilter(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl KeyPointsFilter {

    pub fn new() -> Result<crate::features2d::KeyPointsFilter> {
        unsafe { sys::cv_KeyPointsFilter_KeyPointsFilter() }.into_result().map(|ptr| crate::features2d::KeyPointsFilter { ptr })
    }
    
    pub fn run_by_image_border(keypoints: &mut types::VectorOfKeyPoint, image_size: core::Size, border_size: i32) -> Result<()> {
        unsafe { sys::cv_KeyPointsFilter_runByImageBorder_VectorOfKeyPoint_Size_int(keypoints.as_raw_VectorOfKeyPoint(), image_size, border_size) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * max_size: FLT_MAX
    pub fn run_by_keypoint_size(keypoints: &mut types::VectorOfKeyPoint, min_size: f32, max_size: f32) -> Result<()> {
        unsafe { sys::cv_KeyPointsFilter_runByKeypointSize_VectorOfKeyPoint_float_float(keypoints.as_raw_VectorOfKeyPoint(), min_size, max_size) }.into_result()
    }
    
    pub fn run_by_pixels_mask(keypoints: &mut types::VectorOfKeyPoint, mask: &core::Mat) -> Result<()> {
        unsafe { sys::cv_KeyPointsFilter_runByPixelsMask_VectorOfKeyPoint_Mat(keypoints.as_raw_VectorOfKeyPoint(), mask.as_raw_Mat()) }.into_result()
    }
    
    pub fn remove_duplicated(keypoints: &mut types::VectorOfKeyPoint) -> Result<()> {
        unsafe { sys::cv_KeyPointsFilter_removeDuplicated_VectorOfKeyPoint(keypoints.as_raw_VectorOfKeyPoint()) }.into_result()
    }
    
    pub fn remove_duplicated_sorted(keypoints: &mut types::VectorOfKeyPoint) -> Result<()> {
        unsafe { sys::cv_KeyPointsFilter_removeDuplicatedSorted_VectorOfKeyPoint(keypoints.as_raw_VectorOfKeyPoint()) }.into_result()
    }
    
    pub fn retain_best(keypoints: &mut types::VectorOfKeyPoint, npoints: i32) -> Result<()> {
        unsafe { sys::cv_KeyPointsFilter_retainBest_VectorOfKeyPoint_int(keypoints.as_raw_VectorOfKeyPoint(), npoints) }.into_result()
    }
    
}

// Generating impl for trait cv::MSER (trait)
/// Maximally stable extremal region extractor
///
/// The class encapsulates all the parameters of the %MSER extraction algorithm (see [wiki
/// article](http://en.wikipedia.org/wiki/Maximally_stable_extremal_regions)).
///
/// - there are two different implementation of %MSER: one for grey image, one for color image
///
/// - the grey image algorithm is taken from: [nister2008linear](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_nister2008linear) ;  the paper claims to be faster
/// than union-find method; it actually get 1.5~2m/s on my centrino L7200 1.2GHz laptop.
///
/// - the color image algorithm is taken from: [forssen2007maximally](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_forssen2007maximally) ; it should be much slower
/// than grey image method ( 3~4 times ); the chi_table.h file is taken directly from paper's source
/// code which is distributed under GPL.
///
/// - (Python) A complete example showing the use of the %MSER detector can be found at samples/python/mser.py
pub trait MSER: crate::features2d::Feature2D {
    #[inline(always)] fn as_raw_MSER(&self) -> *mut c_void;
    /// Detect %MSER regions
    ///
    /// ## Parameters
    /// * image: input image (8UC1, 8UC3 or 8UC4, must be greater or equal than 3x3)
    /// * msers: resulting list of point sets
    /// * bboxes: resulting bounding boxes
    fn detect_regions(&mut self, image: &core::Mat, msers: &mut types::VectorOfVectorOfPoint, bboxes: &mut types::VectorOfRect) -> Result<()> {
        unsafe { sys::cv_MSER_detectRegions_Mat_VectorOfVectorOfPoint_VectorOfRect(self.as_raw_MSER(), image.as_raw_Mat(), msers.as_raw_VectorOfVectorOfPoint(), bboxes.as_raw_VectorOfRect()) }.into_result()
    }
    
    fn set_delta(&mut self, delta: i32) -> Result<()> {
        unsafe { sys::cv_MSER_setDelta_int(self.as_raw_MSER(), delta) }.into_result()
    }
    
    fn get_delta(&self) -> Result<i32> {
        unsafe { sys::cv_MSER_getDelta_const(self.as_raw_MSER()) }.into_result()
    }
    
    fn set_min_area(&mut self, min_area: i32) -> Result<()> {
        unsafe { sys::cv_MSER_setMinArea_int(self.as_raw_MSER(), min_area) }.into_result()
    }
    
    fn get_min_area(&self) -> Result<i32> {
        unsafe { sys::cv_MSER_getMinArea_const(self.as_raw_MSER()) }.into_result()
    }
    
    fn set_max_area(&mut self, max_area: i32) -> Result<()> {
        unsafe { sys::cv_MSER_setMaxArea_int(self.as_raw_MSER(), max_area) }.into_result()
    }
    
    fn get_max_area(&self) -> Result<i32> {
        unsafe { sys::cv_MSER_getMaxArea_const(self.as_raw_MSER()) }.into_result()
    }
    
    fn set_pass2_only(&mut self, f: bool) -> Result<()> {
        unsafe { sys::cv_MSER_setPass2Only_bool(self.as_raw_MSER(), f) }.into_result()
    }
    
    fn get_pass2_only(&self) -> Result<bool> {
        unsafe { sys::cv_MSER_getPass2Only_const(self.as_raw_MSER()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_MSER_getDefaultName_const(self.as_raw_MSER()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

impl dyn MSER + '_ {

    /// Full consturctor for %MSER detector
    ///
    /// ## Parameters
    /// * _delta: it compares <span lang='latex'>(size_{i}-size_{i-delta})/size_{i-delta}</span>
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
    pub fn create(_delta: i32, _min_area: i32, _max_area: i32, _max_variation: f64, _min_diversity: f64, _max_evolution: i32, _area_threshold: f64, _min_margin: f64, _edge_blur_size: i32) -> Result<types::PtrOfMSER> {
        unsafe { sys::cv_MSER_create_int_int_int_double_double_int_double_double_int(_delta, _min_area, _max_area, _max_variation, _min_diversity, _max_evolution, _area_threshold, _min_margin, _edge_blur_size) }.into_result().map(|ptr| types::PtrOfMSER { ptr })
    }
    
}

// Generating impl for trait cv::ORB (trait)
/// Class implementing the ORB (*oriented BRIEF*) keypoint detector and descriptor extractor
///
/// described in [RRKB11](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_RRKB11) . The algorithm uses FAST in pyramids to detect stable keypoints, selects
/// the strongest features using FAST or Harris response, finds their orientation using first-order
/// moments and computes the descriptors using BRIEF (where the coordinates of random point pairs (or
/// k-tuples) are rotated according to the measured orientation).
pub trait ORB: crate::features2d::Feature2D {
    #[inline(always)] fn as_raw_ORB(&self) -> *mut c_void;
    fn set_max_features(&mut self, max_features: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setMaxFeatures_int(self.as_raw_ORB(), max_features) }.into_result()
    }
    
    fn get_max_features(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getMaxFeatures_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
        unsafe { sys::cv_ORB_setScaleFactor_double(self.as_raw_ORB(), scale_factor) }.into_result()
    }
    
    fn get_scale_factor(&self) -> Result<f64> {
        unsafe { sys::cv_ORB_getScaleFactor_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setNLevels_int(self.as_raw_ORB(), nlevels) }.into_result()
    }
    
    fn get_n_levels(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getNLevels_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setEdgeThreshold_int(self.as_raw_ORB(), edge_threshold) }.into_result()
    }
    
    fn get_edge_threshold(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getEdgeThreshold_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_first_level(&mut self, first_level: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setFirstLevel_int(self.as_raw_ORB(), first_level) }.into_result()
    }
    
    fn get_first_level(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getFirstLevel_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setWTA_K_int(self.as_raw_ORB(), wta_k) }.into_result()
    }
    
    fn get_wta_k(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getWTA_K_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_score_type(&mut self, score_type: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setScoreType_int(self.as_raw_ORB(), score_type) }.into_result()
    }
    
    fn get_score_type(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getScoreType_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setPatchSize_int(self.as_raw_ORB(), patch_size) }.into_result()
    }
    
    fn get_patch_size(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getPatchSize_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn set_fast_threshold(&mut self, fast_threshold: i32) -> Result<()> {
        unsafe { sys::cv_ORB_setFastThreshold_int(self.as_raw_ORB(), fast_threshold) }.into_result()
    }
    
    fn get_fast_threshold(&self) -> Result<i32> {
        unsafe { sys::cv_ORB_getFastThreshold_const(self.as_raw_ORB()) }.into_result()
    }
    
    fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_ORB_getDefaultName_const(self.as_raw_ORB()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

impl dyn ORB + '_ {

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
    /// * fastThreshold:
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
    pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: i32, patch_size: i32, fast_threshold: i32) -> Result<types::PtrOfORB> {
        unsafe { sys::cv_ORB_create_int_float_int_int_int_int_int_int_int(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold) }.into_result().map(|ptr| types::PtrOfORB { ptr })
    }
    
}

// boxed class cv::SimpleBlobDetector
/// Class for extracting blobs from an image. :
///
/// The class implements a simple algorithm for extracting blobs from an image:
///
/// 1.  Convert the source image to binary images by applying thresholding with several thresholds from
/// minThreshold (inclusive) to maxThreshold (exclusive) with distance thresholdStep between
/// neighboring thresholds.
/// 2.  Extract connected components from every binary image by findContours and calculate their
/// centers.
/// 3.  Group centers from several binary images by their coordinates. Close centers form one group that
/// corresponds to one blob, which is controlled by the minDistBetweenBlobs parameter.
/// 4.  From the groups, estimate final centers of blobs and their radiuses and return as locations and
/// sizes of keypoints.
///
/// This class performs several filtrations of returned blobs. You should set filterBy\* to true/false
/// to turn on/off corresponding filtration. Available filtrations:
///
/// *   **By color**. This filter compares the intensity of a binary image at the center of a blob to
/// blobColor. If they differ, the blob is filtered out. Use blobColor = 0 to extract dark blobs
/// and blobColor = 255 to extract light blobs.
/// *   **By area**. Extracted blobs have an area between minArea (inclusive) and maxArea (exclusive).
/// *   **By circularity**. Extracted blobs have circularity
/// (<span lang='latex'>\frac{4*\pi*Area}{perimeter * perimeter}</span>) between minCircularity (inclusive) and
/// maxCircularity (exclusive).
/// *   **By ratio of the minimum inertia to maximum inertia**. Extracted blobs have this ratio
/// between minInertiaRatio (inclusive) and maxInertiaRatio (exclusive).
/// *   **By convexity**. Extracted blobs have convexity (area / area of blob convex hull) between
/// minConvexity (inclusive) and maxConvexity (exclusive).
///
/// Default values of parameters are tuned to extract dark circular blobs.
#[allow(dead_code)]
pub struct SimpleBlobDetector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}
impl Drop for crate::features2d::SimpleBlobDetector {
    fn drop(&mut self) {
        unsafe { sys::cv_SimpleBlobDetector_delete(self.ptr) };
    }
}
impl crate::features2d::SimpleBlobDetector {
    #[inline(always)] pub fn as_raw_SimpleBlobDetector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl core::Algorithm for SimpleBlobDetector {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::features2d::Feature2D for SimpleBlobDetector {
    #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void { self.ptr }
}

impl SimpleBlobDetector {

    ///
    /// ## C++ default parameters
    /// * parameters: SimpleBlobDetector::Params()
    pub fn create(parameters: crate::features2d::SimpleBlobDetector_Params) -> Result<types::PtrOfSimpleBlobDetector> {
        unsafe { sys::cv_SimpleBlobDetector_create_SimpleBlobDetector_Params(parameters) }.into_result().map(|ptr| types::PtrOfSimpleBlobDetector { ptr })
    }
    
    pub fn get_default_name(&self) -> Result<String> {
        unsafe { sys::cv_SimpleBlobDetector_getDefaultName_const(self.as_raw_SimpleBlobDetector()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

impl SimpleBlobDetector_Params {

    pub fn new() -> Result<crate::features2d::SimpleBlobDetector_Params> {
        unsafe { sys::cv_SimpleBlobDetector_Params_Params() }.into_result()
    }
    
}

pub use crate::hub::manual::features2d::*;
