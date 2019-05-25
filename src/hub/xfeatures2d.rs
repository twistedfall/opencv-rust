//! # Extra 2D Features Framework
//! # Experimental 2D Features Algorithms
//! 
//! This section describes experimental algorithms for 2d feature detection.
//! 
//! # Non-free 2D Features Algorithms
//! 
//! This section describes two popular algorithms for 2d feature detection, SIFT and SURF, that are
//! known to be patented. You need to set the OPENCV_ENABLE_NONFREE option in cmake to use those. Use them at your own risk.
//! 
//! # Experimental 2D Features Matching Algorithm
//! 
//! This section describes the GMS (Grid-based Motion Statistics) matching strategy.
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const DAISY_NRM_FULL: i32 = 102;
pub const DAISY_NRM_NONE: i32 = 100;
pub const DAISY_NRM_PARTIAL: i32 = 101;
pub const DAISY_NRM_SIFT: i32 = 103;
pub const FREAK_NB_ORIENPAIRS: i32 = 45;
pub const FREAK_NB_PAIRS: i32 = 512;
pub const FREAK_NB_SCALES: i32 = 64;
pub const PCTSignatures_GAUSSIAN: i32 = 1;
pub const PCTSignatures_HEURISTIC: i32 = 2;
pub const PCTSignatures_L0_25: i32 = 0;
pub const PCTSignatures_L0_5: i32 = 1;
pub const PCTSignatures_L1: i32 = 2;
pub const PCTSignatures_L2: i32 = 3;
pub const PCTSignatures_L2SQUARED: i32 = 4;
pub const PCTSignatures_L5: i32 = 5;
pub const PCTSignatures_L_INFINITY: i32 = 6;
pub const PCTSignatures_MINUS: i32 = 0;
pub const PCTSignatures_NORMAL: i32 = 2;
pub const PCTSignatures_REGULAR: i32 = 1;
pub const PCTSignatures_UNIFORM: i32 = 0;

/// Estimates cornerness for prespecified KeyPoints using the FAST algorithm
/// 
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints which should be tested to fit the FAST criteria. Keypoints not beeing
/// detected as corners are removed.
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
/// ## C++ default parameters
/// * nonmax_suppression: true
/// * _type: FastFeatureDetector::TYPE_9_16
pub fn fast_for_point_set(image: &core::Mat, keypoints: &mut types::VectorOfKeyPoint, threshold: i32, nonmax_suppression: bool, _type: i32) -> Result<()> {
    unsafe { sys::cv_xfeatures2d_FASTForPointSet_Mat_VectorOfKeyPoint_int_bool_int(image.as_raw_Mat(), keypoints.as_raw_VectorOfKeyPoint(), threshold, nonmax_suppression, _type) }.into_result()
}

/// GMS  (Grid-based Motion Statistics) feature matching strategy by [Bian2017gms](https://docs.opencv.org/3.4.6/d0/de3/citelist.html#CITEREF_Bian2017gms) .
/// ## Parameters
/// * size1: Input size of image1.
/// * size2: Input size of image2.
/// * keypoints1: Input keypoints of image1.
/// * keypoints2: Input keypoints of image2.
/// * matches1to2: Input 1-nearest neighbor matches.
/// * matchesGMS: Matches returned by the GMS matching strategy.
/// * withRotation: Take rotation transformation into account.
/// * withScale: Take scale transformation into account.
/// * thresholdFactor: The higher, the less matches.
/// 
/// Note:
/// Since GMS works well when the number of features is large, we recommend to use the ORB feature and set FastThreshold to 0 to get as many as possible features quickly.
/// If matching results are not satisfying, please add more features. (We use 10000 for images with 640 X 480).
/// If your images have big rotation and scale changes, please set withRotation or withScale to true.
///
/// ## C++ default parameters
/// * with_rotation: false
/// * with_scale: false
/// * threshold_factor: 6.0
pub fn match_gms(size1: core::Size, size2: core::Size, keypoints1: &types::VectorOfKeyPoint, keypoints2: &types::VectorOfKeyPoint, matches1to2: &types::VectorOfDMatch, matches_gms: &mut types::VectorOfDMatch, with_rotation: bool, with_scale: bool, threshold_factor: f64) -> Result<()> {
    unsafe { sys::cv_xfeatures2d_matchGMS_Size_Size_VectorOfKeyPoint_VectorOfKeyPoint_VectorOfDMatch_VectorOfDMatch_bool_bool_double(size1, size2, keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), matches_gms.as_raw_VectorOfDMatch(), with_rotation, with_scale, threshold_factor) }.into_result()
}

