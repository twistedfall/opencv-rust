//! # Extra 2D Features Framework
//!    # Experimental 2D Features Algorithms
//!
//!    This section describes experimental algorithms for 2d feature detection.
//!
//!    # Non-free 2D Features Algorithms
//!
//!    This section describes two popular algorithms for 2d feature detection, SIFT and SURF, that are
//!    known to be patented. You need to set the OPENCV_ENABLE_NONFREE option in cmake to use those. Use them at your own risk.
//!
//!    # Experimental 2D Features Matching Algorithm
//!
//!    This section describes the following matching strategies:
//!        - GMS: Grid-based Motion Statistics, [Bian2017gms](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Bian2017gms)
//!        - LOGOS: Local geometric support for high-outlier spatial verification, [Lowry2018LOGOSLG](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lowry2018LOGOSLG)
//!
//!    # Object Categorization
//!
//!    This section describes approaches based on local 2D features and used to categorize objects.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{AKAZETrait, AKAZETraitConst, AffineFeature2DTrait, AffineFeature2DTraitConst, AgastFeatureDetectorTrait, AgastFeatureDetectorTraitConst, BEBLIDTrait, BEBLIDTraitConst, BOWImgDescriptorExtractorTrait, BOWImgDescriptorExtractorTraitConst, BOWKMeansTrainerTrait, BOWKMeansTrainerTraitConst, BOWTrainerTrait, BOWTrainerTraitConst, BRISKTrait, BRISKTraitConst, BoostDescTrait, BoostDescTraitConst, BriefDescriptorExtractorTrait, BriefDescriptorExtractorTraitConst, DAISYTrait, DAISYTraitConst, Elliptic_KeyPointTrait, Elliptic_KeyPointTraitConst, FREAKTrait, FREAKTraitConst, HarrisLaplaceFeatureDetectorTrait, HarrisLaplaceFeatureDetectorTraitConst, KAZETrait, KAZETraitConst, LATCHTrait, LATCHTraitConst, LUCIDTrait, LUCIDTraitConst, MSDDetectorTrait, MSDDetectorTraitConst, PCTSignaturesSQFDTrait, PCTSignaturesSQFDTraitConst, PCTSignaturesTrait, PCTSignaturesTraitConst, SURFTrait, SURFTraitConst, SURF_CUDATrait, SURF_CUDATraitConst, StarDetectorTrait, StarDetectorTraitConst, TBMRTrait, TBMRTraitConst, TEBLIDTrait, TEBLIDTraitConst, VGGTrait, VGGTraitConst};
}

// DESCRIPTOR_KAZE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1412
pub const AKAZE_DESCRIPTOR_KAZE: i32 = 3;
/// Upright descriptors, not invariant to rotation
// DESCRIPTOR_KAZE_UPRIGHT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1411
pub const AKAZE_DESCRIPTOR_KAZE_UPRIGHT: i32 = 2;
// DESCRIPTOR_MLDB /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1414
pub const AKAZE_DESCRIPTOR_MLDB: i32 = 5;
/// Upright descriptors, not invariant to rotation
// DESCRIPTOR_MLDB_UPRIGHT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1413
pub const AKAZE_DESCRIPTOR_MLDB_UPRIGHT: i32 = 4;
// AGAST_5_8 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1292
pub const AgastFeatureDetector_AGAST_5_8: i32 = 0;
// AGAST_7_12d /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1292
pub const AgastFeatureDetector_AGAST_7_12d: i32 = 1;
// AGAST_7_12s /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1292
pub const AgastFeatureDetector_AGAST_7_12s: i32 = 2;
// NONMAX_SUPPRESSION /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1298
pub const AgastFeatureDetector_NONMAX_SUPPRESSION: i32 = 10001;
// OAST_9_16 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1292
pub const AgastFeatureDetector_OAST_9_16: i32 = 3;
// THRESHOLD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1298
pub const AgastFeatureDetector_THRESHOLD: i32 = 10000;
// SIZE_256_BITS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:277
pub const BEBLID_SIZE_256_BITS: i32 = 101;
// SIZE_512_BITS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:277
pub const BEBLID_SIZE_512_BITS: i32 = 100;
// BGM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:586
pub const BoostDesc_BGM: i32 = 100;
// BGM_BILINEAR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:586
pub const BoostDesc_BGM_BILINEAR: i32 = 102;
// BGM_HARD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:586
pub const BoostDesc_BGM_HARD: i32 = 101;
// BINBOOST_128 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:587
pub const BoostDesc_BINBOOST_128: i32 = 301;
// BINBOOST_256 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:587
pub const BoostDesc_BINBOOST_256: i32 = 302;
// BINBOOST_64 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:587
pub const BoostDesc_BINBOOST_64: i32 = 300;
// LBGM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:586
pub const BoostDesc_LBGM: i32 = 200;
// NRM_FULL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:362
pub const DAISY_NRM_FULL: i32 = 102;
// NRM_NONE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:362
pub const DAISY_NRM_NONE: i32 = 100;
// NRM_PARTIAL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:362
pub const DAISY_NRM_PARTIAL: i32 = 101;
// NRM_SIFT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:362
pub const DAISY_NRM_SIFT: i32 = 103;
// DIFF_CHARBONNIER /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1350
pub const KAZE_DIFF_CHARBONNIER: i32 = 3;
// DIFF_PM_G1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1347
pub const KAZE_DIFF_PM_G1: i32 = 0;
// DIFF_PM_G2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1348
pub const KAZE_DIFF_PM_G2: i32 = 1;
// DIFF_WEICKERT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1349
pub const KAZE_DIFF_WEICKERT: i32 = 2;
/// ![block formula](https://latex.codecogs.com/png.latex?%20e%5E%7B%20%2D%5Calpha%20%2A%20d%5E2%28c%5Fi%2C%20c%5Fj%29%7D%20)
// GAUSSIAN /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:655
pub const PCTSignatures_GAUSSIAN: i32 = 1;
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B1%7D%7B%5Calpha%20%2B%20d%28c%5Fi%2C%20c%5Fj%29%7D%20)
// HEURISTIC /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:656
pub const PCTSignatures_HEURISTIC: i32 = 2;
// L0_25 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:629
pub const PCTSignatures_L0_25: i32 = 0;
// L0_5 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:629
pub const PCTSignatures_L0_5: i32 = 1;
// L1 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:629
pub const PCTSignatures_L1: i32 = 2;
// L2 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:629
pub const PCTSignatures_L2: i32 = 3;
// L2SQUARED /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:629
pub const PCTSignatures_L2SQUARED: i32 = 4;
// L5 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:629
pub const PCTSignatures_L5: i32 = 5;
// L_INFINITY /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:629
pub const PCTSignatures_L_INFINITY: i32 = 6;
/// ![block formula](https://latex.codecogs.com/png.latex?%20%2Dd%28c%5Fi%2C%20c%5Fj%29%20)
// MINUS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:654
pub const PCTSignatures_MINUS: i32 = 0;
/// Generate points with normal (gaussian) distribution.
// NORMAL /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:639
pub const PCTSignatures_NORMAL: i32 = 2;
/// Generate points in a regular grid.
// REGULAR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:638
pub const PCTSignatures_REGULAR: i32 = 1;
/// Generate numbers uniformly.
// UNIFORM /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:637
pub const PCTSignatures_UNIFORM: i32 = 0;
// ANGLE_ROW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:96
pub const SURF_CUDA_ANGLE_ROW: i32 = 5;
// HESSIAN_ROW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:97
pub const SURF_CUDA_HESSIAN_ROW: i32 = 6;
// LAPLACIAN_ROW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:93
pub const SURF_CUDA_LAPLACIAN_ROW: i32 = 2;
// OCTAVE_ROW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:94
pub const SURF_CUDA_OCTAVE_ROW: i32 = 3;
// ROWS_COUNT /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:98
pub const SURF_CUDA_ROWS_COUNT: i32 = 7;
// SIZE_ROW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:95
pub const SURF_CUDA_SIZE_ROW: i32 = 4;
// X_ROW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:91
pub const SURF_CUDA_X_ROW: i32 = 0;
// Y_ROW /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:92
pub const SURF_CUDA_Y_ROW: i32 = 1;
// SIZE_256_BITS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:325
pub const TEBLID_SIZE_256_BITS: i32 = 102;
// SIZE_512_BITS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:325
pub const TEBLID_SIZE_512_BITS: i32 = 103;
// VGG_120 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:528
pub const VGG_VGG_120: i32 = 100;
// VGG_48 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:528
pub const VGG_VGG_48: i32 = 103;
// VGG_64 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:528
pub const VGG_VGG_64: i32 = 102;
// VGG_80 /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:528
pub const VGG_VGG_80: i32 = 101;
/// Descriptor number of bits, each bit is a boosting weak-learner.
/// The user can choose between 512 or 256 bits.
// BeblidSize /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:275
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BEBLID_BeblidSize {
	SIZE_512_BITS = 100,
	SIZE_256_BITS = 101,
}

impl TryFrom<i32> for BEBLID_BeblidSize {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			100 => Ok(Self::SIZE_512_BITS),
			101 => Ok(Self::SIZE_256_BITS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::xfeatures2d::BEBLID_BeblidSize"))),
		}
	}
}

opencv_type_enum! { crate::xfeatures2d::BEBLID_BeblidSize }

// NormalizationType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:360
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DAISY_NormalizationType {
	NRM_NONE = 100,
	NRM_PARTIAL = 101,
	NRM_FULL = 102,
	NRM_SIFT = 103,
}

impl TryFrom<i32> for DAISY_NormalizationType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			100 => Ok(Self::NRM_NONE),
			101 => Ok(Self::NRM_PARTIAL),
			102 => Ok(Self::NRM_FULL),
			103 => Ok(Self::NRM_SIFT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::xfeatures2d::DAISY_NormalizationType"))),
		}
	}
}

opencv_type_enum! { crate::xfeatures2d::DAISY_NormalizationType }

/// Lp distance function selector.
// DistanceFunction /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:627
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PCTSignatures_DistanceFunction {
	L0_25 = 0,
	L0_5 = 1,
	L1 = 2,
	L2 = 3,
	L2SQUARED = 4,
	L5 = 5,
	L_INFINITY = 6,
}

impl TryFrom<i32> for PCTSignatures_DistanceFunction {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::L0_25),
			1 => Ok(Self::L0_5),
			2 => Ok(Self::L1),
			3 => Ok(Self::L2),
			4 => Ok(Self::L2SQUARED),
			5 => Ok(Self::L5),
			6 => Ok(Self::L_INFINITY),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::xfeatures2d::PCTSignatures_DistanceFunction"))),
		}
	}
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_DistanceFunction }

/// Point distributions supported by random point generator.
// PointDistribution /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:635
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PCTSignatures_PointDistribution {
	/// Generate numbers uniformly.
	UNIFORM = 0,
	/// Generate points in a regular grid.
	REGULAR = 1,
	/// Generate points with normal (gaussian) distribution.
	NORMAL = 2,
}

impl TryFrom<i32> for PCTSignatures_PointDistribution {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::UNIFORM),
			1 => Ok(Self::REGULAR),
			2 => Ok(Self::NORMAL),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::xfeatures2d::PCTSignatures_PointDistribution"))),
		}
	}
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_PointDistribution }

/// Similarity function selector.
/// ## See also
/// Christian Beecks, Merih Seran Uysal, Thomas Seidl.
///       Signature quadratic form distance.
///       In Proceedings of the ACM International Conference on Image and Video Retrieval, pages 438-445.
///       ACM, 2010.
/// [BeecksUS10](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BeecksUS10)
///
/// Note: For selected distance function: ![block formula](https://latex.codecogs.com/png.latex?%20d%28c%5Fi%2C%20c%5Fj%29%20)  and parameter: ![block formula](https://latex.codecogs.com/png.latex?%20%5Calpha%20)
// SimilarityFunction /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:652
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PCTSignatures_SimilarityFunction {
	/// ![block formula](https://latex.codecogs.com/png.latex?%20%2Dd%28c%5Fi%2C%20c%5Fj%29%20)
	MINUS = 0,
	/// ![block formula](https://latex.codecogs.com/png.latex?%20e%5E%7B%20%2D%5Calpha%20%2A%20d%5E2%28c%5Fi%2C%20c%5Fj%29%7D%20)
	GAUSSIAN = 1,
	/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B1%7D%7B%5Calpha%20%2B%20d%28c%5Fi%2C%20c%5Fj%29%7D%20)
	HEURISTIC = 2,
}

impl TryFrom<i32> for PCTSignatures_SimilarityFunction {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::MINUS),
			1 => Ok(Self::GAUSSIAN),
			2 => Ok(Self::HEURISTIC),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::xfeatures2d::PCTSignatures_SimilarityFunction"))),
		}
	}
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_SimilarityFunction }

// KeypointLayout /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:89
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SURF_CUDA_KeypointLayout {
	X_ROW = 0,
	Y_ROW = 1,
	LAPLACIAN_ROW = 2,
	OCTAVE_ROW = 3,
	SIZE_ROW = 4,
	ANGLE_ROW = 5,
	HESSIAN_ROW = 6,
	ROWS_COUNT = 7,
}

impl TryFrom<i32> for SURF_CUDA_KeypointLayout {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::X_ROW),
			1 => Ok(Self::Y_ROW),
			2 => Ok(Self::LAPLACIAN_ROW),
			3 => Ok(Self::OCTAVE_ROW),
			4 => Ok(Self::SIZE_ROW),
			5 => Ok(Self::ANGLE_ROW),
			6 => Ok(Self::HESSIAN_ROW),
			7 => Ok(Self::ROWS_COUNT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::xfeatures2d::SURF_CUDA_KeypointLayout"))),
		}
	}
}

opencv_type_enum! { crate::xfeatures2d::SURF_CUDA_KeypointLayout }

/// Descriptor number of bits, each bit is a box average difference.
/// The user can choose between 256 or 512 bits.
// TeblidSize /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:323
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TEBLID_TeblidSize {
	SIZE_256_BITS = 102,
	SIZE_512_BITS = 103,
}

impl TryFrom<i32> for TEBLID_TeblidSize {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			102 => Ok(Self::SIZE_256_BITS),
			103 => Ok(Self::SIZE_512_BITS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::xfeatures2d::TEBLID_TeblidSize"))),
		}
	}
}

opencv_type_enum! { crate::xfeatures2d::TEBLID_TeblidSize }

// DescriptorType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1416
pub type AKAZE_DescriptorType = i32;
// DetectorType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1294
pub type AgastFeatureDetector_DetectorType = i32;
// DiffusivityType /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1352
pub type KAZE_DiffusivityType = i32;
// SurfDescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:120
pub type SurfDescriptorExtractor = crate::xfeatures2d::SURF;
// SurfFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:119
pub type SurfFeatureDetector = crate::xfeatures2d::SURF;
/// Detects corners using the AGAST algorithm
///
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints detected on the image.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected keypoints (corners).
/// * type: one of the four neighborhoods as defined in the paper:
/// AgastFeatureDetector::AGAST_5_8, AgastFeatureDetector::AGAST_7_12d,
/// AgastFeatureDetector::AGAST_7_12s, AgastFeatureDetector::OAST_9_16
///
/// For non-Intel platforms, there is a tree optimised variant of AGAST with same numerical results.
/// The 32-bit binary tree tables were generated automatically from original code using perl script.
/// The perl script and examples of tree generation are placed in features2d/doc folder.
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_mair2010_agast) .
///
/// ## Note
/// This alternative version of [agast] function uses the following default values for its arguments:
/// * nonmax_suppression: true
/// * typ: AgastFeatureDetector::OAST_9_16
// cv::xfeatures2d::AGAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1333
// ("cv::xfeatures2d::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
#[inline]
pub fn agast_def(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_AGAST_const__InputArrayR_vectorLKeyPointGR_int(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, ocvrs_return.as_mut_ptr()) };
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
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected keypoints (corners).
/// * type: one of the four neighborhoods as defined in the paper:
/// AgastFeatureDetector::AGAST_5_8, AgastFeatureDetector::AGAST_7_12d,
/// AgastFeatureDetector::AGAST_7_12s, AgastFeatureDetector::OAST_9_16
///
/// For non-Intel platforms, there is a tree optimised variant of AGAST with same numerical results.
/// The 32-bit binary tree tables were generated automatically from original code using perl script.
/// The perl script and examples of tree generation are placed in features2d/doc folder.
/// Detects corners using the AGAST algorithm by [mair2010_agast](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_mair2010_agast) .
///
/// ## C++ default parameters
/// * nonmax_suppression: true
/// * typ: AgastFeatureDetector::OAST_9_16
// AGAST(InputArray, std::vector<KeyPoint> &, int, bool, AgastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1333
// ("cv::xfeatures2d::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::xfeatures2d::AgastFeatureDetector::DetectorType"]), _)]),
#[inline]
pub fn agast(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::xfeatures2d::AgastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimates cornerness for prespecified KeyPoints using the FAST algorithm
///
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints which should be tested to fit the FAST criteria. Keypoints not being
/// detected as corners are removed.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
///
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Rosten06) .
///
/// ## Note
/// This alternative version of [fast_for_point_set] function uses the following default values for its arguments:
/// * nonmax_suppression: true
/// * typ: FastFeatureDetector::TYPE_9_16
// cv::xfeatures2d::FASTForPointSet(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1480
// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
#[inline]
pub fn fast_for_point_set_def(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Estimates cornerness for prespecified KeyPoints using the FAST algorithm
///
/// ## Parameters
/// * image: grayscale image where keypoints (corners) are detected.
/// * keypoints: keypoints which should be tested to fit the FAST criteria. Keypoints not being
/// detected as corners are removed.
/// * threshold: threshold on difference between intensity of the central pixel and pixels of a
/// circle around this pixel.
/// * nonmaxSuppression: if true, non-maximum suppression is applied to detected corners
/// (keypoints).
/// * type: one of the three neighborhoods as defined in the paper:
/// FastFeatureDetector::TYPE_9_16, FastFeatureDetector::TYPE_7_12,
/// FastFeatureDetector::TYPE_5_8
///
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Rosten06) .
///
/// ## C++ default parameters
/// * nonmax_suppression: true
/// * typ: FastFeatureDetector::TYPE_9_16
// FASTForPointSet(InputArray, std::vector<KeyPoint> &, int, bool, cv::FastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1480
// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
#[inline]
pub fn fast_for_point_set(image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features::FastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// GMS (Grid-based Motion Statistics) feature matching strategy described in [Bian2017gms](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Bian2017gms) .
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
///    Since GMS works well when the number of features is large, we recommend to use the ORB feature and set FastThreshold to 0 to get as many as possible features quickly.
///    If matching results are not satisfying, please add more features. (We use 10000 for images with 640 X 480).
///    If your images have big rotation and scale changes, please set withRotation or withScale to true.
///
/// ## Note
/// This alternative version of [match_gms] function uses the following default values for its arguments:
/// * with_rotation: false
/// * with_scale: false
/// * threshold_factor: 6.0
// cv::xfeatures2d::matchGMS(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1505
// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*"]), _)]),
#[inline]
pub fn match_gms_def(size1: core::Size, size2: core::Size, keypoints1: &core::Vector<core::KeyPoint>, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, matches_gms: &mut core::Vector<core::DMatch>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR(&size1, &size2, keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), matches_gms.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// GMS (Grid-based Motion Statistics) feature matching strategy described in [Bian2017gms](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Bian2017gms) .
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
///    Since GMS works well when the number of features is large, we recommend to use the ORB feature and set FastThreshold to 0 to get as many as possible features quickly.
///    If matching results are not satisfying, please add more features. (We use 10000 for images with 640 X 480).
///    If your images have big rotation and scale changes, please set withRotation or withScale to true.
///
/// ## C++ default parameters
/// * with_rotation: false
/// * with_scale: false
/// * threshold_factor: 6.0
// matchGMS(const Size &, const Size &, const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<DMatch> &, std::vector<DMatch> &, const bool, const bool, const double)(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1505
// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS", "withRotation", "withScale", "thresholdFactor"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*", "const bool", "const bool", "const double"]), _)]),
#[inline]
pub fn match_gms(size1: core::Size, size2: core::Size, keypoints1: &core::Vector<core::KeyPoint>, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, matches_gms: &mut core::Vector<core::DMatch>, with_rotation: bool, with_scale: bool, threshold_factor: f64) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR_const_bool_const_bool_const_double(&size1, &size2, keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), matches_gms.as_raw_mut_VectorOfDMatch(), with_rotation, with_scale, threshold_factor, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// LOGOS (Local geometric support for high-outlier spatial verification) feature matching strategy described in [Lowry2018LOGOSLG](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Lowry2018LOGOSLG) .
/// ## Parameters
/// * keypoints1: Input keypoints of image1.
/// * keypoints2: Input keypoints of image2.
/// * nn1: Index to the closest BoW centroid for each descriptors of image1.
/// * nn2: Index to the closest BoW centroid for each descriptors of image2.
/// * matches1to2: Matches returned by the LOGOS matching strategy.
///
/// Note:
///    This matching strategy is suitable for features matching against large scale database.
///    First step consists in constructing the bag-of-words (BoW) from a representative image database.
///    Image descriptors are then represented by their closest codevector (nearest BoW centroid).
// matchLOGOS(const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<int> &, const std::vector<int> &, std::vector<DMatch> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1520
// ("cv::xfeatures2d::matchLOGOS", vec![(pred!(mut, ["keypoints1", "keypoints2", "nn1", "nn2", "matches1to2"], ["const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<int>*", "const std::vector<int>*", "std::vector<cv::DMatch>*"]), _)]),
#[inline]
pub fn match_logos(keypoints1: &core::Vector<core::KeyPoint>, keypoints2: &core::Vector<core::KeyPoint>, nn1: &core::Vector<i32>, nn2: &core::Vector<i32>, matches1to2: &mut core::Vector<core::DMatch>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_matchLOGOS_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLintGR_const_vectorLintGR_vectorLDMatchGR(keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), nn1.as_raw_VectorOfi32(), nn2.as_raw_VectorOfi32(), matches1to2.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::xfeatures2d::SURF_CUDA]
// SURF_CUDA /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:86
pub trait SURF_CUDATraitConst {
	fn as_raw_SURF_CUDA(&self) -> *const c_void;

	// cv::cuda::SURF_CUDA::hessianThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:182
	// ("cv::cuda::SURF_CUDA::hessianThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn hessian_threshold(&self) -> f64 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propHessianThreshold_const(self.as_raw_SURF_CUDA()) };
		ret
	}

	// cv::cuda::SURF_CUDA::nOctaves() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:183
	// ("cv::cuda::SURF_CUDA::nOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn n_octaves(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propNOctaves_const(self.as_raw_SURF_CUDA()) };
		ret
	}

	// cv::cuda::SURF_CUDA::nOctaveLayers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:184
	// ("cv::cuda::SURF_CUDA::nOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn n_octave_layers(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propNOctaveLayers_const(self.as_raw_SURF_CUDA()) };
		ret
	}

	// cv::cuda::SURF_CUDA::extended() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:185
	// ("cv::cuda::SURF_CUDA::extended", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn extended(&self) -> bool {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propExtended_const(self.as_raw_SURF_CUDA()) };
		ret
	}

	// cv::cuda::SURF_CUDA::upright() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:186
	// ("cv::cuda::SURF_CUDA::upright", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn upright(&self) -> bool {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propUpright_const(self.as_raw_SURF_CUDA()) };
		ret
	}

	/// max keypoints = min(keypointsRatio * img.size().area(), 65535)
	// cv::cuda::SURF_CUDA::keypointsRatio() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:189
	// ("cv::cuda::SURF_CUDA::keypointsRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn keypoints_ratio(&self) -> f32 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propKeypointsRatio_const(self.as_raw_SURF_CUDA()) };
		ret
	}

	// cv::cuda::SURF_CUDA::sum() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::sum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sum(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propSum_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}

	// cv::cuda::SURF_CUDA::mask1() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::mask1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn mask1(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propMask1_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}

	// cv::cuda::SURF_CUDA::maskSum() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::maskSum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn mask_sum(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propMaskSum_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}

	// cv::cuda::SURF_CUDA::det() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::det", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn det(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propDet_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}

	// cv::cuda::SURF_CUDA::trace() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::trace", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn trace(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propTrace_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}

	// cv::cuda::SURF_CUDA::maxPosBuffer() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:195
	// ("cv::cuda::SURF_CUDA::maxPosBuffer", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_pos_buffer(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propMaxPosBuffer_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}

	/// returns the descriptor size in float's (64 or 128)
	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:121
	// ("cv::cuda::SURF_CUDA::descriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_descriptorSize_const(self.as_raw_SURF_CUDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// returns the default norm type
	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:123
	// ("cv::cuda::SURF_CUDA::defaultNorm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_defaultNorm_const(self.as_raw_SURF_CUDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::SURF_CUDA]
pub trait SURF_CUDATrait: crate::xfeatures2d::SURF_CUDATraitConst {
	fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void;

	// cv::cuda::SURF_CUDA::setHessianThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:182
	// ("cv::cuda::SURF_CUDA::setHessianThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_hessian_threshold(&mut self, val: f64) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propHessianThreshold_const_double(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}

	// cv::cuda::SURF_CUDA::setNOctaves(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:183
	// ("cv::cuda::SURF_CUDA::setNOctaves", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_n_octaves(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propNOctaves_const_int(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}

	// cv::cuda::SURF_CUDA::setNOctaveLayers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:184
	// ("cv::cuda::SURF_CUDA::setNOctaveLayers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propNOctaveLayers_const_int(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}

	// cv::cuda::SURF_CUDA::setExtended(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:185
	// ("cv::cuda::SURF_CUDA::setExtended", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_extended(&mut self, val: bool) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propExtended_const_bool(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}

	// cv::cuda::SURF_CUDA::setUpright(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:186
	// ("cv::cuda::SURF_CUDA::setUpright", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_upright(&mut self, val: bool) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propUpright_const_bool(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}

	/// max keypoints = min(keypointsRatio * img.size().area(), 65535)
	// cv::cuda::SURF_CUDA::setKeypointsRatio(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:189
	// ("cv::cuda::SURF_CUDA::setKeypointsRatio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_keypoints_ratio(&mut self, val: f32) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propKeypointsRatio_const_float(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}

	// cv::cuda::SURF_CUDA::setSum(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::setSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	#[inline]
	fn set_sum(&mut self, val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propSum_const_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_GpuMat()) };
		ret
	}

	// cv::cuda::SURF_CUDA::setMask1(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::setMask1", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	#[inline]
	fn set_mask1(&mut self, val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propMask1_const_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_GpuMat()) };
		ret
	}

	// cv::cuda::SURF_CUDA::setMaskSum(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::setMaskSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	#[inline]
	fn set_mask_sum(&mut self, val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propMaskSum_const_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_GpuMat()) };
		ret
	}

	// cv::cuda::SURF_CUDA::setDet(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::setDet", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	#[inline]
	fn set_det(&mut self, val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propDet_const_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_GpuMat()) };
		ret
	}

	// cv::cuda::SURF_CUDA::setTrace(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::setTrace", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	#[inline]
	fn set_trace(&mut self, val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propTrace_const_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_GpuMat()) };
		ret
	}

	// cv::cuda::SURF_CUDA::setMaxPosBuffer(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:195
	// ("cv::cuda::SURF_CUDA::setMaxPosBuffer", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	#[inline]
	fn set_max_pos_buffer(&mut self, val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_propMaxPosBuffer_const_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_GpuMat()) };
		ret
	}

	/// upload host keypoints to device memory
	// uploadKeypoints(const std::vector<KeyPoint> &, GpuMat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:126
	// ("cv::cuda::SURF_CUDA::uploadKeypoints", vec![(pred!(mut, ["keypoints", "keypointsGPU"], ["const std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn upload_keypoints(&mut self, keypoints: &core::Vector<core::KeyPoint>, keypoints_gpu: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_uploadKeypoints_const_vectorLKeyPointGR_GpuMatR(self.as_raw_mut_SURF_CUDA(), keypoints.as_raw_VectorOfKeyPoint(), keypoints_gpu.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// download keypoints from device to host memory
	// downloadKeypoints(const GpuMat &, std::vector<KeyPoint> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:128
	// ("cv::cuda::SURF_CUDA::downloadKeypoints", vec![(pred!(mut, ["keypointsGPU", "keypoints"], ["const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	fn download_keypoints(&mut self, keypoints_gpu: &impl core::GpuMatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_downloadKeypoints_const_GpuMatR_vectorLKeyPointGR(self.as_raw_mut_SURF_CUDA(), keypoints_gpu.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// download descriptors from device to host memory
	// downloadDescriptors(const GpuMat &, std::vector<float> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:131
	// ("cv::cuda::SURF_CUDA::downloadDescriptors", vec![(pred!(mut, ["descriptorsGPU", "descriptors"], ["const cv::cuda::GpuMat*", "std::vector<float>*"]), _)]),
	#[inline]
	fn download_descriptors(&mut self, descriptors_gpu: &impl core::GpuMatTraitConst, descriptors: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_downloadDescriptors_const_GpuMatR_vectorLfloatGR(self.as_raw_mut_SURF_CUDA(), descriptors_gpu.as_raw_GpuMat(), descriptors.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// finds the keypoints using fast hessian detector used in SURF
	/// supports CV_8UC1 images
	/// keypoints will have nFeature cols and 6 rows
	/// keypoints.ptr<float>(X_ROW)[i] will contain x coordinate of i'th feature
	/// keypoints.ptr<float>(Y_ROW)[i] will contain y coordinate of i'th feature
	/// keypoints.ptr<float>(LAPLACIAN_ROW)[i] will contain laplacian sign of i'th feature
	/// keypoints.ptr<float>(OCTAVE_ROW)[i] will contain octave of i'th feature
	/// keypoints.ptr<float>(SIZE_ROW)[i] will contain size of i'th feature
	/// keypoints.ptr<float>(ANGLE_ROW)[i] will contain orientation of i'th feature
	/// keypoints.ptr<float>(HESSIAN_ROW)[i] will contain response of i'th feature
	// operator()(const GpuMat &, const GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:143
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn apply(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// finds the keypoints and computes their descriptors.
	/// Optionally it can compute descriptors for the user-provided keypoints and recompute keypoints direction
	///
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// operator()(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, bool)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:146
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "bool"]), _)]),
	#[inline]
	fn apply_1(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut impl core::GpuMatTrait, descriptors: &mut impl core::GpuMatTrait, use_provided_keypoints: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), descriptors.as_raw_mut_GpuMat(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// finds the keypoints and computes their descriptors.
	/// Optionally it can compute descriptors for the user-provided keypoints and recompute keypoints direction
	///
	/// ## Note
	/// This alternative version of [SURF_CUDATrait::apply] function uses the following default values for its arguments:
	/// * use_provided_keypoints: false
	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:146
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn apply_def(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut impl core::GpuMatTrait, descriptors: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), descriptors.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the keypoints using fast hessian detector used in SURF
	///
	/// ## Parameters
	/// * img: Source image, currently supports only CV_8UC1 images.
	/// * mask: A mask image same size as src and of type CV_8UC1.
	/// * keypoints: Detected keypoints.
	// detect(const GpuMat &, const GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::detect", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn detect(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_detect_const_GpuMatR_const_GpuMatR_GpuMatR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:159
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
	#[inline]
	fn apply_2(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, GpuMat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:160
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*", "bool"]), _)]),
	#[inline]
	fn apply_3(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl core::GpuMatTrait, use_provided_keypoints: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR_bool(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw_mut_GpuMat(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [SURF_CUDATrait::apply] function uses the following default values for its arguments:
	/// * use_provided_keypoints: false
	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:160
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn apply_def_1(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the keypoints and computes their descriptors using fast hessian detector used in SURF
	///
	/// ## Parameters
	/// * img: Source image, currently supports only CV_8UC1 images.
	/// * mask: A mask image same size as src and of type CV_8UC1.
	/// * keypoints: Detected keypoints.
	/// * descriptors: Keypoint descriptors.
	/// * useProvidedKeypoints: Compute descriptors for the user-provided keypoints and recompute keypoints direction.
	///
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// detectWithDescriptors(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, bool)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:171
	// ("cv::cuda::SURF_CUDA::detectWithDescriptors", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "bool"]), _)]),
	#[inline]
	fn detect_with_descriptors(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut impl core::GpuMatTrait, descriptors: &mut impl core::GpuMatTrait, use_provided_keypoints: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_detectWithDescriptors_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), descriptors.as_raw_mut_GpuMat(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the keypoints and computes their descriptors using fast hessian detector used in SURF
	///
	/// ## Parameters
	/// * img: Source image, currently supports only CV_8UC1 images.
	/// * mask: A mask image same size as src and of type CV_8UC1.
	/// * keypoints: Detected keypoints.
	/// * descriptors: Keypoint descriptors.
	/// * useProvidedKeypoints: Compute descriptors for the user-provided keypoints and recompute keypoints direction.
	///
	/// ## Note
	/// This alternative version of [SURF_CUDATrait::detect_with_descriptors] function uses the following default values for its arguments:
	/// * use_provided_keypoints: false
	// cv::cuda::SURF_CUDA::detectWithDescriptors(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:171
	// ("cv::cuda::SURF_CUDA::detectWithDescriptors", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn detect_with_descriptors_def(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut impl core::GpuMatTrait, descriptors: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_detectWithDescriptors_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), descriptors.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, std::vector<float> &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:176
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*", "bool"]), _)]),
	#[inline]
	fn apply_4(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut core::Vector<f32>, use_provided_keypoints: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR_bool(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw_mut_VectorOff32(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [SURF_CUDATrait::apply] function uses the following default values for its arguments:
	/// * use_provided_keypoints: false
	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:176
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*"]), _)]),
	#[inline]
	fn apply_def_2(&mut self, img: &impl core::GpuMatTraitConst, mask: &impl core::GpuMatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// releaseMemory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:179
	// ("cv::cuda::SURF_CUDA::releaseMemory", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release_memory(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_releaseMemory(self.as_raw_mut_SURF_CUDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class used for extracting Speeded Up Robust Features (SURF) from an image. :
///
/// The class SURF_CUDA implements Speeded Up Robust Features descriptor. There is a fast multi-scale
/// Hessian keypoint detector that can be used to find the keypoints (which is the default option). But
/// the descriptors can also be computed for the user-specified keypoints. Only 8-bit grayscale images
/// are supported.
///
/// The class SURF_CUDA can store results in the GPU and CPU memory. It provides functions to convert
/// results between CPU and GPU version ( uploadKeypoints, downloadKeypoints, downloadDescriptors ). The
/// format of CPU results is the same as SURF results. GPU results are stored in GpuMat. The keypoints
/// matrix is ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BnFeatures%7D%20%5Ctimes%207) matrix with the CV_32FC1 type.
///
/// *   keypoints.ptr\<float\>(X_ROW)[i] contains x coordinate of the i-th feature.
/// *   keypoints.ptr\<float\>(Y_ROW)[i] contains y coordinate of the i-th feature.
/// *   keypoints.ptr\<float\>(LAPLACIAN_ROW)[i] contains the laplacian sign of the i-th feature.
/// *   keypoints.ptr\<float\>(OCTAVE_ROW)[i] contains the octave of the i-th feature.
/// *   keypoints.ptr\<float\>(SIZE_ROW)[i] contains the size of the i-th feature.
/// *   keypoints.ptr\<float\>(ANGLE_ROW)[i] contain orientation of the i-th feature.
/// *   keypoints.ptr\<float\>(HESSIAN_ROW)[i] contains the response of the i-th feature.
///
/// The descriptors matrix is ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BnFeatures%7D%20%5Ctimes%20%5Ctexttt%7BdescriptorSize%7D) matrix with the
/// CV_32FC1 type.
///
/// The class SURF_CUDA uses some buffers and provides access to it. All buffers can be safely released
/// between function calls.
/// ## See also
/// SURF
///
///
/// Note:
///    *   An example for using the SURF keypoint matcher on GPU can be found at
///        opencv_source_code/samples/gpu/surf_keypoint_matcher.cpp
// SURF_CUDA /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:86
pub struct SURF_CUDA {
	ptr: *mut c_void,
}

opencv_type_boxed! { SURF_CUDA }

impl Drop for SURF_CUDA {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_cuda_SURF_CUDA_delete(self.as_raw_mut_SURF_CUDA()) };
	}
}

unsafe impl Send for SURF_CUDA {}

impl crate::xfeatures2d::SURF_CUDATraitConst for SURF_CUDA {
	#[inline] fn as_raw_SURF_CUDA(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::SURF_CUDATrait for SURF_CUDA {
	#[inline] fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SURF_CUDA, crate::xfeatures2d::SURF_CUDATraitConst, as_raw_SURF_CUDA, crate::xfeatures2d::SURF_CUDATrait, as_raw_mut_SURF_CUDA }

impl SURF_CUDA {
	/// the default constructor
	// SURF_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:102
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::xfeatures2d::SURF_CUDA> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_SURF_CUDA(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::SURF_CUDA::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// the full constructor taking all the necessary parameters
	///
	/// ## C++ default parameters
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	// SURF_CUDA(double, int, int, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:104
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold", "_nOctaves", "_nOctaveLayers", "_extended", "_keypointsRatio", "_upright"], ["double", "int", "int", "bool", "float", "bool"]), _)]),
	#[inline]
	pub fn new(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool) -> Result<crate::xfeatures2d::SURF_CUDA> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_SURF_CUDA_double_int_int_bool_float_bool(_hessian_threshold, _n_octaves, _n_octave_layers, _extended, _keypoints_ratio, _upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::SURF_CUDA::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// the full constructor taking all the necessary parameters
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	// cv::cuda::SURF_CUDA::SURF_CUDA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:104
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold"], ["double"]), _)]),
	#[inline]
	pub fn new_def(_hessian_threshold: f64) -> Result<crate::xfeatures2d::SURF_CUDA> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_SURF_CUDA_double(_hessian_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::SURF_CUDA::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * _hessianThreshold: Threshold for hessian keypoint detector used in SURF.
	/// * _nOctaves: Number of pyramid octaves the keypoint detector will use.
	/// * _nOctaveLayers: Number of octave layers within each octave.
	/// * _extended: Extended descriptor flag (true - use extended 128-element descriptors; false - use
	/// 64-element descriptors).
	/// * _keypointsRatio: Limits a maximum number of features
	/// * _upright: Up-right or rotated features flag (true - do not compute orientation of features;
	/// false - compute orientation).
	///
	/// ## C++ default parameters
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	// create(double, int, int, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:117
	// ("cv::cuda::SURF_CUDA::create", vec![(pred!(mut, ["_hessianThreshold", "_nOctaves", "_nOctaveLayers", "_extended", "_keypointsRatio", "_upright"], ["double", "int", "int", "bool", "float", "bool"]), _)]),
	#[inline]
	pub fn create(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool) -> Result<core::Ptr<crate::xfeatures2d::SURF_CUDA>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_create_double_int_int_bool_float_bool(_hessian_threshold, _n_octaves, _n_octave_layers, _extended, _keypoints_ratio, _upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::SURF_CUDA>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * _hessianThreshold: Threshold for hessian keypoint detector used in SURF.
	/// * _nOctaves: Number of pyramid octaves the keypoint detector will use.
	/// * _nOctaveLayers: Number of octave layers within each octave.
	/// * _extended: Extended descriptor flag (true - use extended 128-element descriptors; false - use
	/// 64-element descriptors).
	/// * _keypointsRatio: Limits a maximum number of features
	/// * _upright: Up-right or rotated features flag (true - do not compute orientation of features;
	/// false - compute orientation).
	///
	/// ## Note
	/// This alternative version of [SURF_CUDA::create] function uses the following default values for its arguments:
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	// cv::cuda::SURF_CUDA::create(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:117
	// ("cv::cuda::SURF_CUDA::create", vec![(pred!(mut, ["_hessianThreshold"], ["double"]), _)]),
	#[inline]
	pub fn create_def(_hessian_threshold: f64) -> Result<core::Ptr<crate::xfeatures2d::SURF_CUDA>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_create_double(_hessian_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::SURF_CUDA>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for SURF_CUDA {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SURF_CUDA")
			.field("hessian_threshold", &crate::xfeatures2d::SURF_CUDATraitConst::hessian_threshold(self))
			.field("n_octaves", &crate::xfeatures2d::SURF_CUDATraitConst::n_octaves(self))
			.field("n_octave_layers", &crate::xfeatures2d::SURF_CUDATraitConst::n_octave_layers(self))
			.field("extended", &crate::xfeatures2d::SURF_CUDATraitConst::extended(self))
			.field("upright", &crate::xfeatures2d::SURF_CUDATraitConst::upright(self))
			.field("keypoints_ratio", &crate::xfeatures2d::SURF_CUDATraitConst::keypoints_ratio(self))
			.field("sum", &crate::xfeatures2d::SURF_CUDATraitConst::sum(self))
			.field("mask1", &crate::xfeatures2d::SURF_CUDATraitConst::mask1(self))
			.field("mask_sum", &crate::xfeatures2d::SURF_CUDATraitConst::mask_sum(self))
			.field("det", &crate::xfeatures2d::SURF_CUDATraitConst::det(self))
			.field("trace", &crate::xfeatures2d::SURF_CUDATraitConst::trace(self))
			.field("max_pos_buffer", &crate::xfeatures2d::SURF_CUDATraitConst::max_pos_buffer(self))
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::AKAZE]
// AKAZE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1405
pub trait AKAZETraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_AKAZE(&self) -> *const c_void;

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1440
	// ("cv::xfeatures2d::AKAZE::getDescriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getDescriptorType_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1443
	// ("cv::xfeatures2d::AKAZE::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getDescriptorSize_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDescriptorChannels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1446
	// ("cv::xfeatures2d::AKAZE::getDescriptorChannels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_channels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getDescriptorChannels_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1449
	// ("cv::xfeatures2d::AKAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getThreshold_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1452
	// ("cv::xfeatures2d::AKAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getNOctaves_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1455
	// ("cv::xfeatures2d::AKAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getNOctaveLayers_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1458
	// ("cv::xfeatures2d::AKAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_diffusivity(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getDiffusivity_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1459
	// ("cv::xfeatures2d::AKAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getDefaultName_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMaxPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1462
	// ("cv::xfeatures2d::AKAZE::getMaxPoints", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_getMaxPoints_const(self.as_raw_AKAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::AKAZE]
pub trait AKAZETrait: crate::features::Feature2DTrait + crate::xfeatures2d::AKAZETraitConst {
	fn as_raw_mut_AKAZE(&mut self) -> *mut c_void;

	// setDescriptorType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1439
	// ("cv::xfeatures2d::AKAZE::setDescriptorType", vec![(pred!(mut, ["dtype"], ["int"]), _)]),
	#[inline]
	fn set_descriptor_type(&mut self, dtype: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setDescriptorType_int(self.as_raw_mut_AKAZE(), dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDescriptorSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1442
	// ("cv::xfeatures2d::AKAZE::setDescriptorSize", vec![(pred!(mut, ["dsize"], ["int"]), _)]),
	#[inline]
	fn set_descriptor_size(&mut self, dsize: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setDescriptorSize_int(self.as_raw_mut_AKAZE(), dsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDescriptorChannels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1445
	// ("cv::xfeatures2d::AKAZE::setDescriptorChannels", vec![(pred!(mut, ["dch"], ["int"]), _)]),
	#[inline]
	fn set_descriptor_channels(&mut self, dch: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setDescriptorChannels_int(self.as_raw_mut_AKAZE(), dch, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1448
	// ("cv::xfeatures2d::AKAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setThreshold_double(self.as_raw_mut_AKAZE(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1451
	// ("cv::xfeatures2d::AKAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	#[inline]
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setNOctaves_int(self.as_raw_mut_AKAZE(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1454
	// ("cv::xfeatures2d::AKAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setNOctaveLayers_int(self.as_raw_mut_AKAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDiffusivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1457
	// ("cv::xfeatures2d::AKAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["int"]), _)]),
	#[inline]
	fn set_diffusivity(&mut self, diff: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setDiffusivity_int(self.as_raw_mut_AKAZE(), diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPoints(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1461
	// ("cv::xfeatures2d::AKAZE::setMaxPoints", vec![(pred!(mut, ["max_points"], ["int"]), _)]),
	#[inline]
	fn set_max_points(&mut self, max_points: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_setMaxPoints_int(self.as_raw_mut_AKAZE(), max_points, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the AKAZE keypoint detector and descriptor extractor, described in [ANB13](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_ANB13).
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
// AKAZE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1405
pub struct AKAZE {
	ptr: *mut c_void,
}

opencv_type_boxed! { AKAZE }

impl Drop for AKAZE {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_AKAZE_delete(self.as_raw_mut_AKAZE()) };
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

impl crate::features::Feature2DTraitConst for AKAZE {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for AKAZE {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AKAZE, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::AKAZETraitConst for AKAZE {
	#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::AKAZETrait for AKAZE {
	#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AKAZE, crate::xfeatures2d::AKAZETraitConst, as_raw_AKAZE, crate::xfeatures2d::AKAZETrait, as_raw_mut_AKAZE }

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
	// create(int, int, int, float, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1433
	// ("cv::xfeatures2d::AKAZE::create", vec![(pred!(mut, ["descriptor_type", "descriptor_size", "descriptor_channels", "threshold", "nOctaves", "nOctaveLayers", "diffusivity", "max_points"], ["int", "int", "int", "float", "int", "int", "int", "int"]), _)]),
	#[inline]
	pub fn create(descriptor_type: i32, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32, max_points: i32) -> Result<core::Ptr<crate::xfeatures2d::AKAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_create_int_int_int_float_int_int_int_int(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity, max_points, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::AKAZE>::opencv_from_extern(ret) };
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
	// cv::xfeatures2d::AKAZE::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1433
	// ("cv::xfeatures2d::AKAZE::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::AKAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AKAZE_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::AKAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AKAZE, core::Algorithm, cv_xfeatures2d_AKAZE_to_Algorithm }

boxed_cast_base! { AKAZE, crate::features::Feature2D, cv_xfeatures2d_AKAZE_to_Feature2D }

impl std::fmt::Debug for AKAZE {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AKAZE")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::AffineFeature2D]
// AffineFeature2D /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1139
pub trait AffineFeature2DTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_AffineFeature2D(&self) -> *const c_void;

}

/// Mutable methods for [crate::xfeatures2d::AffineFeature2D]
pub trait AffineFeature2DTrait: crate::features::Feature2DTrait + crate::xfeatures2d::AffineFeature2DTraitConst {
	fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void;

	/// Detects keypoints in the image using the wrapped detector and
	/// performs affine adaptation to augment them with their elliptic regions.
	///
	/// ## C++ default parameters
	/// * mask: noArray()
	// detect(InputArray, std::vector<Elliptic_KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1165
	// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn detect(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR_const__InputArrayR(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Detects keypoints in the image using the wrapped detector and
	/// performs affine adaptation to augment them with their elliptic regions.
	///
	/// ## Note
	/// This alternative version of [AffineFeature2DTrait::detect] function uses the following default values for its arguments:
	/// * mask: noArray()
	// cv::xfeatures2d::AffineFeature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1165
	// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*"]), _)]),
	#[inline]
	fn detect_def(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Detects keypoints and computes descriptors for their surrounding
	/// regions, after warping them into circles.
	///
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// detectAndCompute(InputArray, InputArray, std::vector<Elliptic_KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1175
	// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*", "bool"]), _)]),
	#[inline]
	fn detect_and_compute(&mut self, image: &impl ToInputArray, mask: &impl ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>, descriptors: &mut impl ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR_bool(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Detects keypoints and computes descriptors for their surrounding
	/// regions, after warping them into circles.
	///
	/// ## Note
	/// This alternative version of [AffineFeature2DTrait::detect_and_compute] function uses the following default values for its arguments:
	/// * use_provided_keypoints: false
	// cv::xfeatures2d::AffineFeature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1175
	// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn detect_and_compute_def(&mut self, image: &impl ToInputArray, mask: &impl ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing affine adaptation for key points.
///
/// A [FeatureDetector] and a [DescriptorExtractor] are wrapped to augment the
/// detected points with their affine invariant elliptic region and to compute
/// the feature descriptors on the regions after warping them into circles.
///
/// The interface is equivalent to [Feature2D], adding operations for
/// [Elliptic_KeyPoint] "Elliptic_KeyPoints" instead of [KeyPoint] "KeyPoints".
// AffineFeature2D /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1139
pub struct AffineFeature2D {
	ptr: *mut c_void,
}

opencv_type_boxed! { AffineFeature2D }

impl Drop for AffineFeature2D {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_delete(self.as_raw_mut_AffineFeature2D()) };
	}
}

unsafe impl Send for AffineFeature2D {}

impl core::AlgorithmTraitConst for AffineFeature2D {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AffineFeature2D {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature2D, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for AffineFeature2D {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for AffineFeature2D {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature2D, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::AffineFeature2DTraitConst for AffineFeature2D {
	#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::AffineFeature2DTrait for AffineFeature2D {
	#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineFeature2D, crate::xfeatures2d::AffineFeature2DTraitConst, as_raw_AffineFeature2D, crate::xfeatures2d::AffineFeature2DTrait, as_raw_mut_AffineFeature2D }

impl AffineFeature2D {
	/// Creates an instance wrapping the given keypoint detector and
	/// descriptor extractor.
	// create(Ptr<FeatureDetector>, Ptr<DescriptorExtractor>)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1146
	// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector", "descriptor_extractor"], ["cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>"]), _)]),
	#[inline]
	pub fn create(mut keypoint_detector: core::Ptr<crate::features::Feature2D>, mut descriptor_extractor: core::Ptr<crate::features::Feature2D>) -> Result<core::Ptr<crate::xfeatures2d::AffineFeature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG_PtrLFeature2DG(keypoint_detector.as_raw_mut_PtrOfFeature2D(), descriptor_extractor.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::AffineFeature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates an instance where keypoint detector and descriptor
	/// extractor are identical.
	// create(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1154
	// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	#[inline]
	pub fn create_1(mut keypoint_detector: core::Ptr<crate::features::Feature2D>) -> Result<core::Ptr<crate::xfeatures2d::AffineFeature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG(keypoint_detector.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::AffineFeature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { AffineFeature2D, crate::xfeatures2d::TBMR, cv_xfeatures2d_AffineFeature2D_to_TBMR }

boxed_cast_base! { AffineFeature2D, core::Algorithm, cv_xfeatures2d_AffineFeature2D_to_Algorithm }

boxed_cast_base! { AffineFeature2D, crate::features::Feature2D, cv_xfeatures2d_AffineFeature2D_to_Feature2D }

impl std::fmt::Debug for AffineFeature2D {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AffineFeature2D")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::AgastFeatureDetector]
// AgastFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1287
pub trait AgastFeatureDetectorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_AgastFeatureDetector(&self) -> *const c_void;

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1306
	// ("cv::xfeatures2d::AgastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_getThreshold_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1309
	// ("cv::xfeatures2d::AgastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nonmax_suppression(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_getNonmaxSuppression_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1312
	// ("cv::xfeatures2d::AgastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_getType_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1313
	// ("cv::xfeatures2d::AgastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_getDefaultName_const(self.as_raw_AgastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::AgastFeatureDetector]
pub trait AgastFeatureDetectorTrait: crate::features::Feature2DTrait + crate::xfeatures2d::AgastFeatureDetectorTraitConst {
	fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void;

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1305
	// ("cv::xfeatures2d::AgastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_setThreshold_int(self.as_raw_mut_AgastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1308
	// ("cv::xfeatures2d::AgastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	#[inline]
	fn set_nonmax_suppression(&mut self, f: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_setNonmaxSuppression_bool(self.as_raw_mut_AgastFeatureDetector(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1311
	// ("cv::xfeatures2d::AgastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["int"]), _)]),
	#[inline]
	fn set_type(&mut self, typ: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_setType_int(self.as_raw_mut_AgastFeatureDetector(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Wrapping class for feature detection using the AGAST method. :
// AgastFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1287
pub struct AgastFeatureDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { AgastFeatureDetector }

impl Drop for AgastFeatureDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_delete(self.as_raw_mut_AgastFeatureDetector()) };
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

impl crate::features::Feature2DTraitConst for AgastFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for AgastFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AgastFeatureDetector, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::AgastFeatureDetectorTraitConst for AgastFeatureDetector {
	#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::AgastFeatureDetectorTrait for AgastFeatureDetector {
	#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AgastFeatureDetector, crate::xfeatures2d::AgastFeatureDetectorTraitConst, as_raw_AgastFeatureDetector, crate::xfeatures2d::AgastFeatureDetectorTrait, as_raw_mut_AgastFeatureDetector }

impl AgastFeatureDetector {
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: AgastFeatureDetector::OAST_9_16
	// create(int, bool, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1301
	// ("cv::xfeatures2d::AgastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "int"]), _)]),
	#[inline]
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: i32) -> Result<core::Ptr<crate::xfeatures2d::AgastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_create_int_bool_int(threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::AgastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [AgastFeatureDetector::create] function uses the following default values for its arguments:
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: AgastFeatureDetector::OAST_9_16
	// cv::xfeatures2d::AgastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1301
	// ("cv::xfeatures2d::AgastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::AgastFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AgastFeatureDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::AgastFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AgastFeatureDetector, core::Algorithm, cv_xfeatures2d_AgastFeatureDetector_to_Algorithm }

boxed_cast_base! { AgastFeatureDetector, crate::features::Feature2D, cv_xfeatures2d_AgastFeatureDetector_to_Feature2D }

impl std::fmt::Debug for AgastFeatureDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AgastFeatureDetector")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::BEBLID]
// BEBLID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:268
pub trait BEBLIDTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_BEBLID(&self) -> *const c_void;

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:291
	// ("cv::xfeatures2d::BEBLID::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BEBLID_getScaleFactor_const(self.as_raw_BEBLID(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:293
	// ("cv::xfeatures2d::BEBLID::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BEBLID_getDefaultName_const(self.as_raw_BEBLID(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::BEBLID]
pub trait BEBLIDTrait: crate::features::Feature2DTrait + crate::xfeatures2d::BEBLIDTraitConst {
	fn as_raw_mut_BEBLID(&mut self) -> *mut c_void;

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:290
	// ("cv::xfeatures2d::BEBLID::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BEBLID_setScaleFactor_float(self.as_raw_mut_BEBLID(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing BEBLID (Boosted Efficient Binary Local Image Descriptor),
///  described in [Suarez2020BEBLID](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Suarez2020BEBLID) .
///
/// BEBLID \cite Suarez2020BEBLID is a efficient binary descriptor learned with boosting.
/// It is able to describe keypoints from any detector just by changing the scale_factor parameter.
/// In several benchmarks it has proved to largely improve other binary descriptors like ORB or
/// BRISK with the same efficiency. BEBLID describes using the difference of mean gray values in
/// different regions of the image around the KeyPoint, the descriptor is specifically optimized for
/// image matching and patch retrieval addressing the asymmetries of these problems.
///
/// If you find this code useful, please add a reference to the following paper:
/// <BLOCKQUOTE> Iago Suárez, Ghesn Sfeir, José M. Buenaposada, and Luis Baumela.
/// BEBLID: Boosted efficient binary local image descriptor.
/// Pattern Recognition Letters, 133:366–372, 2020. </BLOCKQUOTE>
///
/// The descriptor was trained using 1 million of randomly sampled pairs of patches
/// (20% positives and 80% negatives) from the Liberty split of the UBC datasets
/// \cite winder2007learning as described in the paper [Suarez2020BEBLID](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Suarez2020BEBLID).
/// You can check in the [AKAZE example](https://raw.githubusercontent.com/opencv/opencv/master/samples/cpp/tutorial_code/features2D/AKAZE_match.cpp)
/// how well BEBLID works. Detecting 10000 keypoints with ORB and describing with BEBLID obtains
/// 561 inliers (75%) whereas describing with ORB obtains only 493 inliers (63%).
// BEBLID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:268
pub struct BEBLID {
	ptr: *mut c_void,
}

opencv_type_boxed! { BEBLID }

impl Drop for BEBLID {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_BEBLID_delete(self.as_raw_mut_BEBLID()) };
	}
}

unsafe impl Send for BEBLID {}

impl core::AlgorithmTraitConst for BEBLID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BEBLID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BEBLID, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for BEBLID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for BEBLID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BEBLID, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::BEBLIDTraitConst for BEBLID {
	#[inline] fn as_raw_BEBLID(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BEBLIDTrait for BEBLID {
	#[inline] fn as_raw_mut_BEBLID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BEBLID, crate::xfeatures2d::BEBLIDTraitConst, as_raw_BEBLID, crate::xfeatures2d::BEBLIDTrait, as_raw_mut_BEBLID }

impl BEBLID {
	/// Creates the BEBLID descriptor.
	/// ## Parameters
	/// * scale_factor: Adjust the sampling window around detected keypoints:
	/// - <b> 1.00f </b> should be the scale for ORB keypoints
	/// - <b> 6.75f </b> should be the scale for SIFT detected keypoints
	/// - <b> 6.25f </b> is default and fits for KAZE, SURF detected keypoints
	/// - <b> 5.00f </b> should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints
	/// * n_bits: Determine the number of bits in the descriptor. Should be either
	///  BEBLID::SIZE_512_BITS or BEBLID::SIZE_256_BITS.
	///
	/// ## C++ default parameters
	/// * n_bits: BEBLID::SIZE_512_BITS
	// create(float, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:288
	// ("cv::xfeatures2d::BEBLID::create", vec![(pred!(mut, ["scale_factor", "n_bits"], ["float", "int"]), _)]),
	#[inline]
	pub fn create(scale_factor: f32, n_bits: i32) -> Result<core::Ptr<crate::xfeatures2d::BEBLID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BEBLID_create_float_int(scale_factor, n_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BEBLID>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates the BEBLID descriptor.
	/// ## Parameters
	/// * scale_factor: Adjust the sampling window around detected keypoints:
	/// - <b> 1.00f </b> should be the scale for ORB keypoints
	/// - <b> 6.75f </b> should be the scale for SIFT detected keypoints
	/// - <b> 6.25f </b> is default and fits for KAZE, SURF detected keypoints
	/// - <b> 5.00f </b> should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints
	/// * n_bits: Determine the number of bits in the descriptor. Should be either
	///  BEBLID::SIZE_512_BITS or BEBLID::SIZE_256_BITS.
	///
	/// ## Note
	/// This alternative version of [BEBLID::create] function uses the following default values for its arguments:
	/// * n_bits: BEBLID::SIZE_512_BITS
	// cv::xfeatures2d::BEBLID::create(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:288
	// ("cv::xfeatures2d::BEBLID::create", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	#[inline]
	pub fn create_def(scale_factor: f32) -> Result<core::Ptr<crate::xfeatures2d::BEBLID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BEBLID_create_float(scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BEBLID>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BEBLID, core::Algorithm, cv_xfeatures2d_BEBLID_to_Algorithm }

boxed_cast_base! { BEBLID, crate::features::Feature2D, cv_xfeatures2d_BEBLID_to_Feature2D }

impl std::fmt::Debug for BEBLID {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BEBLID")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::BOWImgDescriptorExtractor]
// BOWImgDescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1617
pub trait BOWImgDescriptorExtractorTraitConst {
	fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void;

	/// Returns the set vocabulary.
	// getVocabulary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1642
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::getVocabulary", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_vocabulary(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_getVocabulary_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns an image descriptor size if the vocabulary is set. Otherwise, it returns 0.
	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1672
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::descriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_descriptorSize_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns an image descriptor type.
	// descriptorType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1676
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::descriptorType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_descriptorType_const(self.as_raw_BOWImgDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::BOWImgDescriptorExtractor]
pub trait BOWImgDescriptorExtractorTrait: crate::xfeatures2d::BOWImgDescriptorExtractorTraitConst {
	fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void;

	/// Sets a visual vocabulary.
	///
	/// ## Parameters
	/// * vocabulary: Vocabulary (can be trained using the inheritor of BOWTrainer ). Each row of the
	/// vocabulary is a visual word (cluster center).
	// setVocabulary(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1638
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::setVocabulary", vec![(pred!(mut, ["vocabulary"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_vocabulary(&mut self, vocabulary: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_setVocabulary_const_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), vocabulary.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
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
	// compute(InputArray, std::vector<KeyPoint> &, OutputArray, std::vector<std::vector<int>> *, Mat *)(InputArray, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1654
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor", "pointIdxsOfClusters", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*", "cv::Mat*"]), _)]),
	#[inline]
	fn compute(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>, descriptors: &mut impl core::MatTrait) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_vectorLvectorLintGGX_MatX(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32(), descriptors.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
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
	/// This alternative version of [BOWImgDescriptorExtractorTrait::compute] function uses the following default values for its arguments:
	/// * point_idxs_of_clusters: 0
	/// * descriptors: 0
	// cv::xfeatures2d::BOWImgDescriptorExtractor::compute(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1654
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_def(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
	// compute(InputArray, OutputArray, std::vector<std::vector<int>> *)(InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1663
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor", "pointIdxsOfClusters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*"]), _)]),
	#[inline]
	fn compute_1(&mut self, keypoint_descriptors: &impl ToInputArray, img_descriptor: &mut impl ToOutputArray, point_idxs_of_clusters: &mut core::Vector<core::Vector<i32>>) -> Result<()> {
		input_array_arg!(keypoint_descriptors);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vectorLvectorLintGGX(self.as_raw_mut_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw__InputArray(), img_descriptor.as_raw__OutputArray(), point_idxs_of_clusters.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
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
	// cv::xfeatures2d::BOWImgDescriptorExtractor::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1663
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_def_1(&mut self, keypoint_descriptors: &impl ToInputArray, img_descriptor: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(keypoint_descriptors);
		output_array_arg!(img_descriptor);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_BOWImgDescriptorExtractor(), keypoint_descriptors.as_raw__InputArray(), img_descriptor.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// compute2(const Mat &, std::vector<KeyPoint> &, Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1667
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute2", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::Mat*", "std::vector<cv::KeyPoint>*", "cv::Mat*"]), _)]),
	#[inline]
	fn compute_2(&mut self, image: &impl core::MatTraitConst, keypoints: &mut core::Vector<core::KeyPoint>, img_descriptor: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_compute2_const_MatR_vectorLKeyPointGR_MatR(self.as_raw_mut_BOWImgDescriptorExtractor(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyPoint(), img_descriptor.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
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
// BOWImgDescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1617
pub struct BOWImgDescriptorExtractor {
	ptr: *mut c_void,
}

opencv_type_boxed! { BOWImgDescriptorExtractor }

impl Drop for BOWImgDescriptorExtractor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_delete(self.as_raw_mut_BOWImgDescriptorExtractor()) };
	}
}

unsafe impl Send for BOWImgDescriptorExtractor {}

impl crate::xfeatures2d::BOWImgDescriptorExtractorTraitConst for BOWImgDescriptorExtractor {
	#[inline] fn as_raw_BOWImgDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BOWImgDescriptorExtractorTrait for BOWImgDescriptorExtractor {
	#[inline] fn as_raw_mut_BOWImgDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWImgDescriptorExtractor, crate::xfeatures2d::BOWImgDescriptorExtractorTraitConst, as_raw_BOWImgDescriptorExtractor, crate::xfeatures2d::BOWImgDescriptorExtractorTrait, as_raw_mut_BOWImgDescriptorExtractor }

impl BOWImgDescriptorExtractor {
	/// The constructor.
	///
	/// ## Parameters
	/// * dextractor: Descriptor extractor that is used to compute descriptors for an input image and
	/// its keypoints.
	/// * dmatcher: Descriptor matcher that is used to find the nearest word of the trained vocabulary
	/// for each keypoint descriptor of the image.
	// BOWImgDescriptorExtractor(const Ptr<Feature2D> &, const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1627
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dextractor", "dmatcher"], ["const cv::Ptr<cv::Feature2D>*", "const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	#[inline]
	pub fn new(dextractor: &core::Ptr<crate::features::Feature2D>, dmatcher: &core::Ptr<crate::features::DescriptorMatcher>) -> Result<crate::xfeatures2d::BOWImgDescriptorExtractor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLFeature2DGR_const_PtrLDescriptorMatcherGR(dextractor.as_raw_PtrOfFeature2D(), dmatcher.as_raw_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::BOWImgDescriptorExtractor::opencv_from_extern(ret) };
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
	// BOWImgDescriptorExtractor(const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1630
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dmatcher"], ["const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	#[inline]
	pub fn new_1(dmatcher: &core::Ptr<crate::features::DescriptorMatcher>) -> Result<crate::xfeatures2d::BOWImgDescriptorExtractor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLDescriptorMatcherGR(dmatcher.as_raw_PtrOfDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::BOWImgDescriptorExtractor::opencv_from_extern(ret) };
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

/// Constant methods for [crate::xfeatures2d::BOWKMeansTrainer]
// BOWKMeansTrainer /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1584
pub trait BOWKMeansTrainerTraitConst: crate::xfeatures2d::BOWTrainerTraitConst {
	fn as_raw_BOWKMeansTrainer(&self) -> *const c_void;

	// cluster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1596
	// ("cv::xfeatures2d::BOWKMeansTrainer::cluster", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cluster(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWKMeansTrainer_cluster_const(self.as_raw_BOWKMeansTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1597
	// ("cv::xfeatures2d::BOWKMeansTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn cluster_1(&self, descriptors: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWKMeansTrainer_cluster_const_const_MatR(self.as_raw_BOWKMeansTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::BOWKMeansTrainer]
pub trait BOWKMeansTrainerTrait: crate::xfeatures2d::BOWKMeansTrainerTraitConst + crate::xfeatures2d::BOWTrainerTrait {
	fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void;

}

/// kmeans -based class to train visual vocabulary using the *bag of visual words* approach. :
// BOWKMeansTrainer /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1584
pub struct BOWKMeansTrainer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BOWKMeansTrainer }

impl Drop for BOWKMeansTrainer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_BOWKMeansTrainer_delete(self.as_raw_mut_BOWKMeansTrainer()) };
	}
}

unsafe impl Send for BOWKMeansTrainer {}

impl crate::xfeatures2d::BOWTrainerTraitConst for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BOWTrainerTrait for BOWKMeansTrainer {
	#[inline] fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWKMeansTrainer, crate::xfeatures2d::BOWTrainerTraitConst, as_raw_BOWTrainer, crate::xfeatures2d::BOWTrainerTrait, as_raw_mut_BOWTrainer }

impl crate::xfeatures2d::BOWKMeansTrainerTraitConst for BOWKMeansTrainer {
	#[inline] fn as_raw_BOWKMeansTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BOWKMeansTrainerTrait for BOWKMeansTrainer {
	#[inline] fn as_raw_mut_BOWKMeansTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWKMeansTrainer, crate::xfeatures2d::BOWKMeansTrainerTraitConst, as_raw_BOWKMeansTrainer, crate::xfeatures2d::BOWKMeansTrainerTrait, as_raw_mut_BOWKMeansTrainer }

impl BOWKMeansTrainer {
	/// The constructor.
	/// ## See also
	/// cv::kmeans
	///
	/// ## C++ default parameters
	/// * termcrit: TermCriteria()
	/// * attempts: 3
	/// * flags: KMEANS_PP_CENTERS
	// BOWKMeansTrainer(int, const TermCriteria &, int, int)(Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1591
	// ("cv::xfeatures2d::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount", "termcrit", "attempts", "flags"], ["int", "const cv::TermCriteria*", "int", "int"]), _)]),
	#[inline]
	pub fn new(cluster_count: i32, termcrit: core::TermCriteria, attempts: i32, flags: i32) -> Result<crate::xfeatures2d::BOWKMeansTrainer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(cluster_count, &termcrit, attempts, flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::BOWKMeansTrainer::opencv_from_extern(ret) };
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
	// cv::xfeatures2d::BOWKMeansTrainer::BOWKMeansTrainer(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1591
	// ("cv::xfeatures2d::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount"], ["int"]), _)]),
	#[inline]
	pub fn new_def(cluster_count: i32) -> Result<crate::xfeatures2d::BOWKMeansTrainer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWKMeansTrainer_BOWKMeansTrainer_int(cluster_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::BOWKMeansTrainer::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BOWKMeansTrainer, crate::xfeatures2d::BOWTrainer, cv_xfeatures2d_BOWKMeansTrainer_to_BOWTrainer }

impl std::fmt::Debug for BOWKMeansTrainer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BOWKMeansTrainer")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::BOWTrainer]
// BOWTrainer /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1538
pub trait BOWTrainerTraitConst {
	fn as_raw_BOWTrainer(&self) -> *const c_void;

	/// Returns a training set of descriptors.
	// getDescriptors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1555
	// ("cv::xfeatures2d::BOWTrainer::getDescriptors", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptors(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWTrainer_getDescriptors_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns the count of all descriptors stored in the training set.
	// descriptorsCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1559
	// ("cv::xfeatures2d::BOWTrainer::descriptorsCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptors_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWTrainer_descriptorsCount_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
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
	// cluster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1564
	// ("cv::xfeatures2d::BOWTrainer::cluster", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cluster(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWTrainer_cluster_const(self.as_raw_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
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
	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1575
	// ("cv::xfeatures2d::BOWTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn cluster_1(&self, descriptors: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWTrainer_cluster_const_const_MatR(self.as_raw_BOWTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::BOWTrainer]
pub trait BOWTrainerTrait: crate::xfeatures2d::BOWTrainerTraitConst {
	fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void;

	/// Adds descriptors to a training set.
	///
	/// ## Parameters
	/// * descriptors: Descriptors to add to a training set. Each row of the descriptors matrix is a
	/// descriptor.
	///
	/// The training set is clustered using clustermethod to construct the vocabulary.
	// add(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1551
	// ("cv::xfeatures2d::BOWTrainer::add", vec![(pred!(mut, ["descriptors"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn add(&mut self, descriptors: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWTrainer_add_const_MatR(self.as_raw_mut_BOWTrainer(), descriptors.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1561
	// ("cv::xfeatures2d::BOWTrainer::clear", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BOWTrainer_clear(self.as_raw_mut_BOWTrainer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for training the *bag of visual words* vocabulary from a set of descriptors.
///
/// For details, see, for example, *Visual Categorization with Bags of Keypoints* by Gabriella Csurka,
/// Christopher R. Dance, Lixin Fan, Jutta Willamowski, Cedric Bray, 2004. :
// BOWTrainer /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1538
pub struct BOWTrainer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BOWTrainer }

impl Drop for BOWTrainer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_BOWTrainer_delete(self.as_raw_mut_BOWTrainer()) };
	}
}

unsafe impl Send for BOWTrainer {}

impl crate::xfeatures2d::BOWTrainerTraitConst for BOWTrainer {
	#[inline] fn as_raw_BOWTrainer(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BOWTrainerTrait for BOWTrainer {
	#[inline] fn as_raw_mut_BOWTrainer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BOWTrainer, crate::xfeatures2d::BOWTrainerTraitConst, as_raw_BOWTrainer, crate::xfeatures2d::BOWTrainerTrait, as_raw_mut_BOWTrainer }

impl BOWTrainer {
}

boxed_cast_descendant! { BOWTrainer, crate::xfeatures2d::BOWKMeansTrainer, cv_xfeatures2d_BOWTrainer_to_BOWKMeansTrainer }

impl std::fmt::Debug for BOWTrainer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BOWTrainer")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::BRISK]
// BRISK /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1222
pub trait BRISKTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_BRISK(&self) -> *const c_void;

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1264
	// ("cv::xfeatures2d::BRISK::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_getDefaultName_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1270
	// ("cv::xfeatures2d::BRISK::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_getThreshold_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1276
	// ("cv::xfeatures2d::BRISK::getOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_getOctaves_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPatternScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1282
	// ("cv::xfeatures2d::BRISK::getPatternScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pattern_scale(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_getPatternScale_const(self.as_raw_BRISK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::BRISK]
pub trait BRISKTrait: crate::features::Feature2DTrait + crate::xfeatures2d::BRISKTraitConst {
	fn as_raw_mut_BRISK(&mut self) -> *mut c_void;

	/// Set detection threshold.
	/// ## Parameters
	/// * threshold: AGAST detection threshold score.
	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1269
	// ("cv::xfeatures2d::BRISK::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_setThreshold_int(self.as_raw_mut_BRISK(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set detection octaves.
	/// ## Parameters
	/// * octaves: detection octaves. Use 0 to do single scale.
	// setOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1275
	// ("cv::xfeatures2d::BRISK::setOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	#[inline]
	fn set_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_setOctaves_int(self.as_raw_mut_BRISK(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set detection patternScale.
	/// ## Parameters
	/// * patternScale: apply this scale to the pattern used for sampling the neighbourhood of a
	/// keypoint.
	// setPatternScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1281
	// ("cv::xfeatures2d::BRISK::setPatternScale", vec![(pred!(mut, ["patternScale"], ["float"]), _)]),
	#[inline]
	fn set_pattern_scale(&mut self, pattern_scale: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_setPatternScale_float(self.as_raw_mut_BRISK(), pattern_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the BRISK keypoint detector and descriptor extractor, described in [LCS11](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_LCS11) .
// BRISK /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1222
pub struct BRISK {
	ptr: *mut c_void,
}

opencv_type_boxed! { BRISK }

impl Drop for BRISK {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_BRISK_delete(self.as_raw_mut_BRISK()) };
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

impl crate::features::Feature2DTraitConst for BRISK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for BRISK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BRISK, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::BRISKTraitConst for BRISK {
	#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BRISKTrait for BRISK {
	#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BRISK, crate::xfeatures2d::BRISKTraitConst, as_raw_BRISK, crate::xfeatures2d::BRISKTrait, as_raw_mut_BRISK }

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
	// create(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1232
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "patternScale"], ["int", "int", "float"]), _)]),
	#[inline]
	pub fn create(thresh: i32, octaves: i32, pattern_scale: f32) -> Result<core::Ptr<crate::xfeatures2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_create_int_int_float(thresh, octaves, pattern_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BRISK>::opencv_from_extern(ret) };
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
	// cv::xfeatures2d::BRISK::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1232
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BRISK>::opencv_from_extern(ret) };
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
	// create(const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1245
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList", "dMax", "dMin", "indexChange"], ["const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_1(radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>, d_max: f32, d_min: f32, index_change: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_create_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BRISK>::opencv_from_extern(ret) };
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
	/// This alternative version of [BRISK::create] function uses the following default values for its arguments:
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	// cv::xfeatures2d::BRISK::create(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1245
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList"], ["const std::vector<float>*", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_def_1(radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_create_const_vectorLfloatGR_const_vectorLintGR(radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BRISK>::opencv_from_extern(ret) };
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
	// create(int, int, const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1261
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList", "dMax", "dMin", "indexChange"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_2(thresh: i32, octaves: i32, radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>, d_max: f32, d_min: f32, index_change: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(thresh, octaves, radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), d_max, d_min, index_change.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BRISK>::opencv_from_extern(ret) };
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
	/// This alternative version of [BRISK::create] function uses the following default values for its arguments:
	/// * d_max: 5.85f
	/// * d_min: 8.2f
	/// * index_change: std::vector<int>()
	// cv::xfeatures2d::BRISK::create(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1261
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_def_2(thresh: i32, octaves: i32, radius_list: &core::Vector<f32>, number_list: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::BRISK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR(thresh, octaves, radius_list.as_raw_VectorOff32(), number_list.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BRISK>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BRISK, core::Algorithm, cv_xfeatures2d_BRISK_to_Algorithm }

boxed_cast_base! { BRISK, crate::features::Feature2D, cv_xfeatures2d_BRISK_to_Feature2D }

impl std::fmt::Debug for BRISK {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BRISK")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::BoostDesc]
// BoostDesc /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:580
pub trait BoostDescTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_BoostDesc(&self) -> *const c_void;

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:593
	// ("cv::xfeatures2d::BoostDesc::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_getDefaultName_const(self.as_raw_BoostDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:596
	// ("cv::xfeatures2d::BoostDesc::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_scale_orientation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(self.as_raw_BoostDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:599
	// ("cv::xfeatures2d::BoostDesc::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_getScaleFactor_const(self.as_raw_BoostDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::BoostDesc]
pub trait BoostDescTrait: crate::features::Feature2DTrait + crate::xfeatures2d::BoostDescTraitConst {
	fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void;

	// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:595
	// ("cv::xfeatures2d::BoostDesc::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
	#[inline]
	fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(self.as_raw_mut_BoostDesc(), use_scale_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:598
	// ("cv::xfeatures2d::BoostDesc::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(self.as_raw_mut_BoostDesc(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing BoostDesc (Learning Image Descriptors with Boosting), described in
/// [Trzcinski13a](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Trzcinski13a) and [Trzcinski13b](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Trzcinski13b).
///
/// ## Parameters
/// * desc: type of descriptor to use, BoostDesc::BINBOOST_256 is default (256 bit long dimension)
/// Available types are: BoostDesc::BGM, BoostDesc::BGM_HARD, BoostDesc::BGM_BILINEAR, BoostDesc::LBGM,
/// BoostDesc::BINBOOST_64, BoostDesc::BINBOOST_128, BoostDesc::BINBOOST_256
/// * use_orientation: sample patterns using keypoints orientation, enabled by default
/// * scale_factor: adjust the sampling window of detected keypoints
/// 6.25f is default and fits for KAZE, SURF detected keypoints window ratio
/// 6.75f should be the scale for SIFT detected keypoints window ratio
/// 5.00f should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints window ratio
/// 0.75f should be the scale for ORB keypoints ratio
/// 1.50f was the default in original implementation
///
///
/// Note: BGM is the base descriptor where each binary dimension is computed as the output of a single weak learner.
/// BGM_HARD and BGM_BILINEAR refers to same BGM but use different type of gradient binning. In the BGM_HARD that
/// use ASSIGN_HARD binning type the gradient is assigned to the nearest orientation bin. In the BGM_BILINEAR that use
/// ASSIGN_BILINEAR binning type the gradient is assigned to the two neighbouring bins. In the BGM and all other modes that use
/// ASSIGN_SOFT binning type the gradient is assigned to 8 nearest bins according to the cosine value between the gradient
/// angle and the bin center. LBGM (alias FP-Boost) is the floating point extension where each dimension is computed
/// as a linear combination of the weak learner responses. BINBOOST and subvariants are the binary extensions of LBGM
/// where each bit is computed as a thresholded linear combination of a set of weak learners.
/// BoostDesc header files (boostdesc_*.i) was exported from original binaries with export-boostdesc.py script from
/// samples subfolder.
// BoostDesc /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:580
pub struct BoostDesc {
	ptr: *mut c_void,
}

opencv_type_boxed! { BoostDesc }

impl Drop for BoostDesc {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_BoostDesc_delete(self.as_raw_mut_BoostDesc()) };
	}
}

unsafe impl Send for BoostDesc {}

impl core::AlgorithmTraitConst for BoostDesc {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BoostDesc {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BoostDesc, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for BoostDesc {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for BoostDesc {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BoostDesc, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::BoostDescTraitConst for BoostDesc {
	#[inline] fn as_raw_BoostDesc(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BoostDescTrait for BoostDesc {
	#[inline] fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BoostDesc, crate::xfeatures2d::BoostDescTraitConst, as_raw_BoostDesc, crate::xfeatures2d::BoostDescTrait, as_raw_mut_BoostDesc }

impl BoostDesc {
	/// ## C++ default parameters
	/// * desc: BoostDesc::BINBOOST_256
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	// create(int, bool, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:590
	// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, ["desc", "use_scale_orientation", "scale_factor"], ["int", "bool", "float"]), _)]),
	#[inline]
	pub fn create(desc: i32, use_scale_orientation: bool, scale_factor: f32) -> Result<core::Ptr<crate::xfeatures2d::BoostDesc>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_create_int_bool_float(desc, use_scale_orientation, scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BoostDesc>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [BoostDesc::create] function uses the following default values for its arguments:
	/// * desc: BoostDesc::BINBOOST_256
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	// cv::xfeatures2d::BoostDesc::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:590
	// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::BoostDesc>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BoostDesc>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BoostDesc, core::Algorithm, cv_xfeatures2d_BoostDesc_to_Algorithm }

boxed_cast_base! { BoostDesc, crate::features::Feature2D, cv_xfeatures2d_BoostDesc_to_Feature2D }

impl std::fmt::Debug for BoostDesc {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BoostDesc")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::BriefDescriptorExtractor]
// BriefDescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:164
pub trait BriefDescriptorExtractorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void;

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:170
	// ("cv::xfeatures2d::BriefDescriptorExtractor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_getDescriptorSize_const(self.as_raw_BriefDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:173
	// ("cv::xfeatures2d::BriefDescriptorExtractor::getUseOrientation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_orientation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_getUseOrientation_const(self.as_raw_BriefDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:175
	// ("cv::xfeatures2d::BriefDescriptorExtractor::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_getDefaultName_const(self.as_raw_BriefDescriptorExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::BriefDescriptorExtractor]
pub trait BriefDescriptorExtractorTrait: crate::features::Feature2DTrait + crate::xfeatures2d::BriefDescriptorExtractorTraitConst {
	fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void;

	// setDescriptorSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:169
	// ("cv::xfeatures2d::BriefDescriptorExtractor::setDescriptorSize", vec![(pred!(mut, ["bytes"], ["int"]), _)]),
	#[inline]
	fn set_descriptor_size(&mut self, bytes: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_setDescriptorSize_int(self.as_raw_mut_BriefDescriptorExtractor(), bytes, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:172
	// ("cv::xfeatures2d::BriefDescriptorExtractor::setUseOrientation", vec![(pred!(mut, ["use_orientation"], ["bool"]), _)]),
	#[inline]
	fn set_use_orientation(&mut self, use_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_setUseOrientation_bool(self.as_raw_mut_BriefDescriptorExtractor(), use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for computing BRIEF descriptors described in [calon2010](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_calon2010) .
///
/// ## Parameters
/// * bytes: legth of the descriptor in bytes, valid values are: 16, 32 (default) or 64 .
/// * use_orientation: sample patterns using keypoints orientation, disabled by default.
// BriefDescriptorExtractor /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:164
pub struct BriefDescriptorExtractor {
	ptr: *mut c_void,
}

opencv_type_boxed! { BriefDescriptorExtractor }

impl Drop for BriefDescriptorExtractor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_delete(self.as_raw_mut_BriefDescriptorExtractor()) };
	}
}

unsafe impl Send for BriefDescriptorExtractor {}

impl core::AlgorithmTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BriefDescriptorExtractor, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BriefDescriptorExtractor, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::BriefDescriptorExtractorTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BriefDescriptorExtractor, crate::xfeatures2d::BriefDescriptorExtractorTraitConst, as_raw_BriefDescriptorExtractor, crate::xfeatures2d::BriefDescriptorExtractorTrait, as_raw_mut_BriefDescriptorExtractor }

impl BriefDescriptorExtractor {
	/// ## C++ default parameters
	/// * bytes: 32
	/// * use_orientation: false
	// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:167
	// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, ["bytes", "use_orientation"], ["int", "bool"]), _)]),
	#[inline]
	pub fn create(bytes: i32, use_orientation: bool) -> Result<core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(bytes, use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BriefDescriptorExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [BriefDescriptorExtractor::create] function uses the following default values for its arguments:
	/// * bytes: 32
	/// * use_orientation: false
	// cv::xfeatures2d::BriefDescriptorExtractor::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:167
	// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BriefDescriptorExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BriefDescriptorExtractor, core::Algorithm, cv_xfeatures2d_BriefDescriptorExtractor_to_Algorithm }

boxed_cast_base! { BriefDescriptorExtractor, crate::features::Feature2D, cv_xfeatures2d_BriefDescriptorExtractor_to_Feature2D }

impl std::fmt::Debug for BriefDescriptorExtractor {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BriefDescriptorExtractor")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::DAISY]
// DAISY /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:357
pub trait DAISYTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_DAISY(&self) -> *const c_void;

	// getRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:369
	// ("cv::xfeatures2d::DAISY::getRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_radius(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getRadius_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getQRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:372
	// ("cv::xfeatures2d::DAISY::getQRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_q_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getQRadius_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getQTheta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:375
	// ("cv::xfeatures2d::DAISY::getQTheta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_q_theta(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getQTheta_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getQHist()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:378
	// ("cv::xfeatures2d::DAISY::getQHist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_q_hist(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getQHist_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNorm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:381
	// ("cv::xfeatures2d::DAISY::getNorm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getNorm_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getH()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:384
	// ("cv::xfeatures2d::DAISY::getH", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_h(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getH_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getInterpolation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:387
	// ("cv::xfeatures2d::DAISY::getInterpolation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_interpolation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getInterpolation_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:390
	// ("cv::xfeatures2d::DAISY::getUseOrientation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_orientation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getUseOrientation_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:392
	// ("cv::xfeatures2d::DAISY::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_getDefaultName_const(self.as_raw_DAISY(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	// GetDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:424
	// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
	#[inline]
	fn get_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(self.as_raw_DAISY(), y, x, orientation, descriptor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	/// * H: homography matrix for warped grid
	// GetDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:433
	// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor", "H"], ["double", "double", "int", "float*", "double*"]), _)]),
	#[inline]
	fn get_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(self.as_raw_DAISY(), y, x, orientation, descriptor, h, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	// GetUnnormalizedDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:441
	// ("cv::xfeatures2d::DAISY::GetUnnormalizedDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
	#[inline]
	fn get_unnormalized_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(self.as_raw_DAISY(), y, x, orientation, descriptor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	/// * H: homography matrix for warped grid
	// GetUnnormalizedDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:450
	// ("cv::xfeatures2d::DAISY::GetUnnormalizedDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor", "H"], ["double", "double", "int", "float*", "double*"]), _)]),
	#[inline]
	fn get_unnormalized_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(self.as_raw_DAISY(), y, x, orientation, descriptor, h, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::DAISY]
pub trait DAISYTrait: crate::features::Feature2DTrait + crate::xfeatures2d::DAISYTraitConst {
	fn as_raw_mut_DAISY(&mut self) -> *mut c_void;

	// setRadius(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:368
	// ("cv::xfeatures2d::DAISY::setRadius", vec![(pred!(mut, ["radius"], ["float"]), _)]),
	#[inline]
	fn set_radius(&mut self, radius: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setRadius_float(self.as_raw_mut_DAISY(), radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setQRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:371
	// ("cv::xfeatures2d::DAISY::setQRadius", vec![(pred!(mut, ["q_radius"], ["int"]), _)]),
	#[inline]
	fn set_q_radius(&mut self, q_radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setQRadius_int(self.as_raw_mut_DAISY(), q_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setQTheta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:374
	// ("cv::xfeatures2d::DAISY::setQTheta", vec![(pred!(mut, ["q_theta"], ["int"]), _)]),
	#[inline]
	fn set_q_theta(&mut self, q_theta: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setQTheta_int(self.as_raw_mut_DAISY(), q_theta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setQHist(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:377
	// ("cv::xfeatures2d::DAISY::setQHist", vec![(pred!(mut, ["q_hist"], ["int"]), _)]),
	#[inline]
	fn set_q_hist(&mut self, q_hist: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setQHist_int(self.as_raw_mut_DAISY(), q_hist, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNorm(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:380
	// ("cv::xfeatures2d::DAISY::setNorm", vec![(pred!(mut, ["norm"], ["int"]), _)]),
	#[inline]
	fn set_norm(&mut self, norm: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setNorm_int(self.as_raw_mut_DAISY(), norm, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setH(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:383
	// ("cv::xfeatures2d::DAISY::setH", vec![(pred!(mut, ["H"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_h(&mut self, h: &impl ToInputArray) -> Result<()> {
		input_array_arg!(h);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setH_const__InputArrayR(self.as_raw_mut_DAISY(), h.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setInterpolation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:386
	// ("cv::xfeatures2d::DAISY::setInterpolation", vec![(pred!(mut, ["interpolation"], ["bool"]), _)]),
	#[inline]
	fn set_interpolation(&mut self, interpolation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setInterpolation_bool(self.as_raw_mut_DAISY(), interpolation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:389
	// ("cv::xfeatures2d::DAISY::setUseOrientation", vec![(pred!(mut, ["use_orientation"], ["bool"]), _)]),
	#[inline]
	fn set_use_orientation(&mut self, use_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_setUseOrientation_bool(self.as_raw_mut_DAISY(), use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// ## Parameters
	/// * image: image to extract descriptors
	/// * keypoints: of interest within image
	/// * descriptors: resulted descriptors array
	// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:399
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute(&mut self, image: &impl ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:401
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["images", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_1(&mut self, images: &impl ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(self.as_raw_mut_DAISY(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// ## Parameters
	/// * image: image to extract descriptors
	/// * roi: region of interest within image
	/// * descriptors: resulted descriptors array for roi image pixels
	// compute(InputArray, Rect, OutputArray)(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:410
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "roi", "descriptors"], ["const cv::_InputArray*", "cv::Rect", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_2(&mut self, image: &impl ToInputArray, roi: core::Rect, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), &roi, descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// ## Parameters
	/// * image: image to extract descriptors
	/// * descriptors: resulted descriptors array for all image pixels
	// compute(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:416
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_3(&mut self, image: &impl ToInputArray, descriptors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing DAISY descriptor, described in [Tola10](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Tola10)
///
/// ## Parameters
/// * radius: radius of the descriptor at the initial scale
/// * q_radius: amount of radial range division quantity
/// * q_theta: amount of angular range division quantity
/// * q_hist: amount of gradient orientations range division quantity
/// * norm: choose descriptors normalization type, where
/// DAISY::NRM_NONE will not do any normalization (default),
/// DAISY::NRM_PARTIAL mean that histograms are normalized independently for L2 norm equal to 1.0,
/// DAISY::NRM_FULL mean that descriptors are normalized for L2 norm equal to 1.0,
/// DAISY::NRM_SIFT mean that descriptors are normalized for L2 norm equal to 1.0 but no individual one is bigger than 0.154 as in SIFT
/// * H: optional 3x3 homography matrix used to warp the grid of daisy but sampling keypoints remains unwarped on image
/// * interpolation: switch to disable interpolation for speed improvement at minor quality loss
/// * use_orientation: sample patterns using keypoints orientation, disabled by default.
// DAISY /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:357
pub struct DAISY {
	ptr: *mut c_void,
}

opencv_type_boxed! { DAISY }

impl Drop for DAISY {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_DAISY_delete(self.as_raw_mut_DAISY()) };
	}
}

unsafe impl Send for DAISY {}

impl core::AlgorithmTraitConst for DAISY {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DAISY {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DAISY, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for DAISY {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for DAISY {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DAISY, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::DAISYTraitConst for DAISY {
	#[inline] fn as_raw_DAISY(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::DAISYTrait for DAISY {
	#[inline] fn as_raw_mut_DAISY(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DAISY, crate::xfeatures2d::DAISYTraitConst, as_raw_DAISY, crate::xfeatures2d::DAISYTrait, as_raw_mut_DAISY }

impl DAISY {
	/// ## C++ default parameters
	/// * radius: 15
	/// * q_radius: 3
	/// * q_theta: 8
	/// * q_hist: 8
	/// * norm: DAISY::NRM_NONE
	/// * h: noArray()
	/// * interpolation: true
	/// * use_orientation: false
	// create(float, int, int, int, DAISY::NormalizationType, InputArray, bool, bool)(Primitive, Primitive, Primitive, Primitive, Enum, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:364
	// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, ["radius", "q_radius", "q_theta", "q_hist", "norm", "H", "interpolation", "use_orientation"], ["float", "int", "int", "int", "cv::xfeatures2d::DAISY::NormalizationType", "const cv::_InputArray*", "bool", "bool"]), _)]),
	#[inline]
	pub fn create(radius: f32, q_radius: i32, q_theta: i32, q_hist: i32, norm: crate::xfeatures2d::DAISY_NormalizationType, h: &impl ToInputArray, interpolation: bool, use_orientation: bool) -> Result<core::Ptr<crate::xfeatures2d::DAISY>> {
		input_array_arg!(h);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_create_float_int_int_int_NormalizationType_const__InputArrayR_bool_bool(radius, q_radius, q_theta, q_hist, norm, h.as_raw__InputArray(), interpolation, use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::DAISY>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DAISY::create] function uses the following default values for its arguments:
	/// * radius: 15
	/// * q_radius: 3
	/// * q_theta: 8
	/// * q_hist: 8
	/// * norm: DAISY::NRM_NONE
	/// * h: noArray()
	/// * interpolation: true
	/// * use_orientation: false
	// cv::xfeatures2d::DAISY::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:364
	// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::DAISY>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::DAISY>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DAISY, core::Algorithm, cv_xfeatures2d_DAISY_to_Algorithm }

boxed_cast_base! { DAISY, crate::features::Feature2D, cv_xfeatures2d_DAISY_to_Feature2D }

impl std::fmt::Debug for DAISY {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DAISY")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::Elliptic_KeyPoint]
// Elliptic_KeyPoint /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1078
pub trait Elliptic_KeyPointTraitConst: core::KeyPointTraitConst {
	fn as_raw_Elliptic_KeyPoint(&self) -> *const c_void;

	/// the lengths of the major and minor ellipse axes
	// cv::xfeatures2d::Elliptic_KeyPoint::axes() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1081
	// ("cv::xfeatures2d::Elliptic_KeyPoint::axes", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn axes(&self) -> core::Size_<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const(self.as_raw_Elliptic_KeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// the integration scale at which the parameters were estimated
	// cv::xfeatures2d::Elliptic_KeyPoint::si() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1082
	// ("cv::xfeatures2d::Elliptic_KeyPoint::si", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn si(&self) -> f32 {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_propSi_const(self.as_raw_Elliptic_KeyPoint()) };
		ret
	}

	/// the transformation between image space and local patch space
	// cv::xfeatures2d::Elliptic_KeyPoint::transf() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1083
	// ("cv::xfeatures2d::Elliptic_KeyPoint::transf", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn transf(&self) -> core::Matx23f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_propTransf_const(self.as_raw_Elliptic_KeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

}

/// Mutable methods for [crate::xfeatures2d::Elliptic_KeyPoint]
pub trait Elliptic_KeyPointTrait: core::KeyPointTrait + crate::xfeatures2d::Elliptic_KeyPointTraitConst {
	fn as_raw_mut_Elliptic_KeyPoint(&mut self) -> *mut c_void;

	/// the lengths of the major and minor ellipse axes
	// cv::xfeatures2d::Elliptic_KeyPoint::setAxes(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1081
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setAxes", vec![(pred!(mut, ["val"], ["const cv::Size_<float>"]), _)]),
	#[inline]
	fn set_axes(&mut self, val: core::Size_<f32>) {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const_Size_LfloatG(self.as_raw_mut_Elliptic_KeyPoint(), &val) };
		ret
	}

	/// the integration scale at which the parameters were estimated
	// cv::xfeatures2d::Elliptic_KeyPoint::setSi(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1082
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setSi", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_si(&mut self, val: f32) {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_propSi_const_float(self.as_raw_mut_Elliptic_KeyPoint(), val) };
		ret
	}

	/// the transformation between image space and local patch space
	// cv::xfeatures2d::Elliptic_KeyPoint::setTransf(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1083
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setTransf", vec![(pred!(mut, ["val"], ["const cv::Matx23f"]), _)]),
	#[inline]
	fn set_transf(&mut self, val: core::Matx23f) {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_propTransf_const_Matx23f(self.as_raw_mut_Elliptic_KeyPoint(), &val) };
		ret
	}

}

/// Elliptic region around an interest point.
// Elliptic_KeyPoint /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1078
pub struct Elliptic_KeyPoint {
	ptr: *mut c_void,
}

opencv_type_boxed! { Elliptic_KeyPoint }

impl Drop for Elliptic_KeyPoint {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_delete(self.as_raw_mut_Elliptic_KeyPoint()) };
	}
}

unsafe impl Send for Elliptic_KeyPoint {}

impl core::KeyPointTraitConst for Elliptic_KeyPoint {
	#[inline] fn as_raw_KeyPoint(&self) -> *const c_void { self.as_raw() }
}

impl core::KeyPointTrait for Elliptic_KeyPoint {
	#[inline] fn as_raw_mut_KeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Elliptic_KeyPoint, core::KeyPointTraitConst, as_raw_KeyPoint, core::KeyPointTrait, as_raw_mut_KeyPoint }

impl crate::xfeatures2d::Elliptic_KeyPointTraitConst for Elliptic_KeyPoint {
	#[inline] fn as_raw_Elliptic_KeyPoint(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::Elliptic_KeyPointTrait for Elliptic_KeyPoint {
	#[inline] fn as_raw_mut_Elliptic_KeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Elliptic_KeyPoint, crate::xfeatures2d::Elliptic_KeyPointTraitConst, as_raw_Elliptic_KeyPoint, crate::xfeatures2d::Elliptic_KeyPointTrait, as_raw_mut_Elliptic_KeyPoint }

impl Elliptic_KeyPoint {
	// Elliptic_KeyPoint()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1084
	// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::Elliptic_KeyPoint::opencv_from_extern(ret) };
		Ok(ret)
	}

	// Elliptic_KeyPoint(Point2f, float, Size, float, float)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1085
	// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, ["pt", "angle", "axes", "size", "si"], ["cv::Point2f", "float", "cv::Size", "float", "float"]), _)]),
	#[inline]
	pub fn new(pt: core::Point2f, angle: f32, axes: core::Size, size: f32, si: f32) -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(&pt, angle, &axes, size, si, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::Elliptic_KeyPoint::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Elliptic_KeyPoint, core::KeyPoint, cv_xfeatures2d_Elliptic_KeyPoint_to_KeyPoint }

impl std::fmt::Debug for Elliptic_KeyPoint {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Elliptic_KeyPoint")
			.field("axes", &crate::xfeatures2d::Elliptic_KeyPointTraitConst::axes(self))
			.field("si", &crate::xfeatures2d::Elliptic_KeyPointTraitConst::si(self))
			.field("transf", &crate::xfeatures2d::Elliptic_KeyPointTraitConst::transf(self))
			.field("pt", &core::KeyPointTraitConst::pt(self))
			.field("size", &core::KeyPointTraitConst::size(self))
			.field("angle", &core::KeyPointTraitConst::angle(self))
			.field("response", &core::KeyPointTraitConst::response(self))
			.field("octave", &core::KeyPointTraitConst::octave(self))
			.field("class_id", &core::KeyPointTraitConst::class_id(self))
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::FREAK]
// FREAK /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:88
pub trait FREAKTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_FREAK(&self) -> *const c_void;

	// getOrientationNormalized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:110
	// ("cv::xfeatures2d::FREAK::getOrientationNormalized", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_orientation_normalized(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_getOrientationNormalized_const(self.as_raw_FREAK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleNormalized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:113
	// ("cv::xfeatures2d::FREAK::getScaleNormalized", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_normalized(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_getScaleNormalized_const(self.as_raw_FREAK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPatternScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:116
	// ("cv::xfeatures2d::FREAK::getPatternScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pattern_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_getPatternScale_const(self.as_raw_FREAK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:119
	// ("cv::xfeatures2d::FREAK::getNOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_getNOctaves_const(self.as_raw_FREAK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:121
	// ("cv::xfeatures2d::FREAK::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_getDefaultName_const(self.as_raw_FREAK(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::FREAK]
pub trait FREAKTrait: crate::features::Feature2DTrait + crate::xfeatures2d::FREAKTraitConst {
	fn as_raw_mut_FREAK(&mut self) -> *mut c_void;

	// setOrientationNormalized(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:109
	// ("cv::xfeatures2d::FREAK::setOrientationNormalized", vec![(pred!(mut, ["orientationNormalized"], ["bool"]), _)]),
	#[inline]
	fn set_orientation_normalized(&mut self, orientation_normalized: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_setOrientationNormalized_bool(self.as_raw_mut_FREAK(), orientation_normalized, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleNormalized(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:112
	// ("cv::xfeatures2d::FREAK::setScaleNormalized", vec![(pred!(mut, ["scaleNormalized"], ["bool"]), _)]),
	#[inline]
	fn set_scale_normalized(&mut self, scale_normalized: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_setScaleNormalized_bool(self.as_raw_mut_FREAK(), scale_normalized, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPatternScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:115
	// ("cv::xfeatures2d::FREAK::setPatternScale", vec![(pred!(mut, ["patternScale"], ["double"]), _)]),
	#[inline]
	fn set_pattern_scale(&mut self, pattern_scale: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_setPatternScale_double(self.as_raw_mut_FREAK(), pattern_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:118
	// ("cv::xfeatures2d::FREAK::setNOctaves", vec![(pred!(mut, ["nOctaves"], ["int"]), _)]),
	#[inline]
	fn set_n_octaves(&mut self, n_octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_setNOctaves_int(self.as_raw_mut_FREAK(), n_octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the FREAK (*Fast Retina Keypoint*) keypoint descriptor, described in [AOV12](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_AOV12) .
///
/// The algorithm propose a novel keypoint descriptor inspired by the human visual system and more
/// precisely the retina, coined Fast Retina Key- point (FREAK). A cascade of binary strings is
/// computed by efficiently comparing image intensities over a retinal sampling pattern. FREAKs are in
/// general faster to compute with lower memory load and also more robust than SIFT, SURF or BRISK.
/// They are competitive alternatives to existing keypoints in particular for embedded applications.
///
///
/// Note:
///    *   An example on how to use the FREAK descriptor can be found at
///        opencv_source_code/samples/cpp/freak_demo.cpp
// FREAK /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:88
pub struct FREAK {
	ptr: *mut c_void,
}

opencv_type_boxed! { FREAK }

impl Drop for FREAK {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_FREAK_delete(self.as_raw_mut_FREAK()) };
	}
}

unsafe impl Send for FREAK {}

impl core::AlgorithmTraitConst for FREAK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FREAK {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FREAK, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for FREAK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for FREAK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FREAK, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::FREAKTraitConst for FREAK {
	#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::FREAKTrait for FREAK {
	#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FREAK, crate::xfeatures2d::FREAKTraitConst, as_raw_FREAK, crate::xfeatures2d::FREAKTrait, as_raw_mut_FREAK }

impl FREAK {
	// NB_SCALES /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:92
	pub const NB_SCALES: i32 = 64;
	// NB_PAIRS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:93
	pub const NB_PAIRS: i32 = 512;
	// NB_ORIENPAIRS /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:94
	pub const NB_ORIENPAIRS: i32 = 45;
	/// ## Parameters
	/// * orientationNormalized: Enable orientation normalization.
	/// * scaleNormalized: Enable scale normalization.
	/// * patternScale: Scaling of the description pattern.
	/// * nOctaves: Number of octaves covered by the detected keypoints.
	/// * selectedPairs: (Optional) user defined selected pairs indexes,
	///
	/// ## C++ default parameters
	/// * orientation_normalized: true
	/// * scale_normalized: true
	/// * pattern_scale: 22.0f
	/// * n_octaves: 4
	/// * selected_pairs: std::vector<int>()
	// create(bool, bool, float, int, const std::vector<int> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:103
	// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, ["orientationNormalized", "scaleNormalized", "patternScale", "nOctaves", "selectedPairs"], ["bool", "bool", "float", "int", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create(orientation_normalized: bool, scale_normalized: bool, pattern_scale: f32, n_octaves: i32, selected_pairs: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::FREAK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vectorLintGR(orientation_normalized, scale_normalized, pattern_scale, n_octaves, selected_pairs.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::FREAK>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * orientationNormalized: Enable orientation normalization.
	/// * scaleNormalized: Enable scale normalization.
	/// * patternScale: Scaling of the description pattern.
	/// * nOctaves: Number of octaves covered by the detected keypoints.
	/// * selectedPairs: (Optional) user defined selected pairs indexes,
	///
	/// ## Note
	/// This alternative version of [FREAK::create] function uses the following default values for its arguments:
	/// * orientation_normalized: true
	/// * scale_normalized: true
	/// * pattern_scale: 22.0f
	/// * n_octaves: 4
	/// * selected_pairs: std::vector<int>()
	// cv::xfeatures2d::FREAK::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:103
	// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::FREAK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::FREAK>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FREAK, core::Algorithm, cv_xfeatures2d_FREAK_to_Algorithm }

boxed_cast_base! { FREAK, crate::features::Feature2D, cv_xfeatures2d_FREAK_to_Feature2D }

impl std::fmt::Debug for FREAK {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FREAK")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::HarrisLaplaceFeatureDetector]
// HarrisLaplaceFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1092
pub trait HarrisLaplaceFeatureDetectorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void;

	// getNumOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1112
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getNumOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_getNumOctaves_const(self.as_raw_HarrisLaplaceFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCornThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1115
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getCornThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_corn_thresh(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_getCornThresh_const(self.as_raw_HarrisLaplaceFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDOGThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1118
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getDOGThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_dog_thresh(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_getDOGThresh_const(self.as_raw_HarrisLaplaceFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxCorners()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1121
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getMaxCorners", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_corners(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_getMaxCorners_const(self.as_raw_HarrisLaplaceFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNumLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1124
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getNumLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_getNumLayers_const(self.as_raw_HarrisLaplaceFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1126
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_getDefaultName_const(self.as_raw_HarrisLaplaceFeatureDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::HarrisLaplaceFeatureDetector]
pub trait HarrisLaplaceFeatureDetectorTrait: crate::features::Feature2DTrait + crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst {
	fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void;

	// setNumOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1111
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setNumOctaves", vec![(pred!(mut, ["numOctaves_"], ["int"]), _)]),
	#[inline]
	fn set_num_octaves(&mut self, num_octaves_: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_setNumOctaves_int(self.as_raw_mut_HarrisLaplaceFeatureDetector(), num_octaves_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCornThresh(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1114
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setCornThresh", vec![(pred!(mut, ["corn_thresh_"], ["float"]), _)]),
	#[inline]
	fn set_corn_thresh(&mut self, corn_thresh_: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_setCornThresh_float(self.as_raw_mut_HarrisLaplaceFeatureDetector(), corn_thresh_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDOGThresh(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1117
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setDOGThresh", vec![(pred!(mut, ["DOG_thresh_"], ["float"]), _)]),
	#[inline]
	fn set_dog_thresh(&mut self, dog_thresh_: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_setDOGThresh_float(self.as_raw_mut_HarrisLaplaceFeatureDetector(), dog_thresh_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxCorners(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1120
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setMaxCorners", vec![(pred!(mut, ["maxCorners_"], ["int"]), _)]),
	#[inline]
	fn set_max_corners(&mut self, max_corners_: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_setMaxCorners_int(self.as_raw_mut_HarrisLaplaceFeatureDetector(), max_corners_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNumLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1123
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setNumLayers", vec![(pred!(mut, ["num_layers_"], ["int"]), _)]),
	#[inline]
	fn set_num_layers(&mut self, num_layers_: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_setNumLayers_int(self.as_raw_mut_HarrisLaplaceFeatureDetector(), num_layers_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the Harris-Laplace feature detector as described in [Mikolajczyk2004](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Mikolajczyk2004).
// HarrisLaplaceFeatureDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1092
pub struct HarrisLaplaceFeatureDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { HarrisLaplaceFeatureDetector }

impl Drop for HarrisLaplaceFeatureDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_delete(self.as_raw_mut_HarrisLaplaceFeatureDetector()) };
	}
}

unsafe impl Send for HarrisLaplaceFeatureDetector {}

impl core::AlgorithmTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { HarrisLaplaceFeatureDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { HarrisLaplaceFeatureDetector, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { HarrisLaplaceFeatureDetector, crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst, as_raw_HarrisLaplaceFeatureDetector, crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait, as_raw_mut_HarrisLaplaceFeatureDetector }

impl HarrisLaplaceFeatureDetector {
	/// Creates a new implementation instance.
	///
	/// ## Parameters
	/// * numOctaves: the number of octaves in the scale-space pyramid
	/// * corn_thresh: the threshold for the Harris cornerness measure
	/// * DOG_thresh: the threshold for the Difference-of-Gaussians scale selection
	/// * maxCorners: the maximum number of corners to consider
	/// * num_layers: the number of intermediate scales per octave
	///
	/// ## C++ default parameters
	/// * num_octaves: 6
	/// * corn_thresh: 0.01f
	/// * dog_thresh: 0.01f
	/// * max_corners: 5000
	/// * num_layers: 4
	// create(int, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1104
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, ["numOctaves", "corn_thresh", "DOG_thresh", "maxCorners", "num_layers"], ["int", "float", "float", "int", "int"]), _)]),
	#[inline]
	pub fn create(num_octaves: i32, corn_thresh: f32, dog_thresh: f32, max_corners: i32, num_layers: i32) -> Result<core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(num_octaves, corn_thresh, dog_thresh, max_corners, num_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::HarrisLaplaceFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates a new implementation instance.
	///
	/// ## Parameters
	/// * numOctaves: the number of octaves in the scale-space pyramid
	/// * corn_thresh: the threshold for the Harris cornerness measure
	/// * DOG_thresh: the threshold for the Difference-of-Gaussians scale selection
	/// * maxCorners: the maximum number of corners to consider
	/// * num_layers: the number of intermediate scales per octave
	///
	/// ## Note
	/// This alternative version of [HarrisLaplaceFeatureDetector::create] function uses the following default values for its arguments:
	/// * num_octaves: 6
	/// * corn_thresh: 0.01f
	/// * dog_thresh: 0.01f
	/// * max_corners: 5000
	/// * num_layers: 4
	// cv::xfeatures2d::HarrisLaplaceFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1104
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::HarrisLaplaceFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { HarrisLaplaceFeatureDetector, core::Algorithm, cv_xfeatures2d_HarrisLaplaceFeatureDetector_to_Algorithm }

boxed_cast_base! { HarrisLaplaceFeatureDetector, crate::features::Feature2D, cv_xfeatures2d_HarrisLaplaceFeatureDetector_to_Feature2D }

impl std::fmt::Debug for HarrisLaplaceFeatureDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("HarrisLaplaceFeatureDetector")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::KAZE]
// KAZE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1342
pub trait KAZETraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_KAZE(&self) -> *const c_void;

	// getExtended()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1370
	// ("cv::xfeatures2d::KAZE::getExtended", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_extended(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_getExtended_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUpright()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1373
	// ("cv::xfeatures2d::KAZE::getUpright", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_upright(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_getUpright_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1376
	// ("cv::xfeatures2d::KAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_getThreshold_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1379
	// ("cv::xfeatures2d::KAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_getNOctaves_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1382
	// ("cv::xfeatures2d::KAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_getNOctaveLayers_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1385
	// ("cv::xfeatures2d::KAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_diffusivity(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_getDiffusivity_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1386
	// ("cv::xfeatures2d::KAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_getDefaultName_const(self.as_raw_KAZE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::KAZE]
pub trait KAZETrait: crate::features::Feature2DTrait + crate::xfeatures2d::KAZETraitConst {
	fn as_raw_mut_KAZE(&mut self) -> *mut c_void;

	// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1369
	// ("cv::xfeatures2d::KAZE::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
	#[inline]
	fn set_extended(&mut self, extended: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_setExtended_bool(self.as_raw_mut_KAZE(), extended, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1372
	// ("cv::xfeatures2d::KAZE::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
	#[inline]
	fn set_upright(&mut self, upright: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_setUpright_bool(self.as_raw_mut_KAZE(), upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1375
	// ("cv::xfeatures2d::KAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	#[inline]
	fn set_threshold(&mut self, threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_setThreshold_double(self.as_raw_mut_KAZE(), threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1378
	// ("cv::xfeatures2d::KAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	#[inline]
	fn set_n_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_setNOctaves_int(self.as_raw_mut_KAZE(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1381
	// ("cv::xfeatures2d::KAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_setNOctaveLayers_int(self.as_raw_mut_KAZE(), octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDiffusivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1384
	// ("cv::xfeatures2d::KAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["int"]), _)]),
	#[inline]
	fn set_diffusivity(&mut self, diff: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_setDiffusivity_int(self.as_raw_mut_KAZE(), diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the KAZE keypoint detector and descriptor extractor, described in [ABD12](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_ABD12) .
///
///
/// Note: AKAZE descriptor can only be used with KAZE or AKAZE keypoints .. [ABD12] KAZE Features. Pablo
/// F. Alcantarilla, Adrien Bartoli and Andrew J. Davison. In European Conference on Computer Vision
/// (ECCV), Fiorenze, Italy, October 2012.
// KAZE /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1342
pub struct KAZE {
	ptr: *mut c_void,
}

opencv_type_boxed! { KAZE }

impl Drop for KAZE {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_KAZE_delete(self.as_raw_mut_KAZE()) };
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

impl crate::features::Feature2DTraitConst for KAZE {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for KAZE {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KAZE, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::KAZETraitConst for KAZE {
	#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::KAZETrait for KAZE {
	#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KAZE, crate::xfeatures2d::KAZETraitConst, as_raw_KAZE, crate::xfeatures2d::KAZETrait, as_raw_mut_KAZE }

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
	// create(bool, bool, float, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1364
	// ("cv::xfeatures2d::KAZE::create", vec![(pred!(mut, ["extended", "upright", "threshold", "nOctaves", "nOctaveLayers", "diffusivity"], ["bool", "bool", "float", "int", "int", "int"]), _)]),
	#[inline]
	pub fn create(extended: bool, upright: bool, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32) -> Result<core::Ptr<crate::xfeatures2d::KAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_create_bool_bool_float_int_int_int(extended, upright, threshold, n_octaves, n_octave_layers, diffusivity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::KAZE>::opencv_from_extern(ret) };
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
	// cv::xfeatures2d::KAZE::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1364
	// ("cv::xfeatures2d::KAZE::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::KAZE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_KAZE_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::KAZE>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { KAZE, core::Algorithm, cv_xfeatures2d_KAZE_to_Algorithm }

boxed_cast_base! { KAZE, crate::features::Feature2D, cv_xfeatures2d_KAZE_to_Feature2D }

impl std::fmt::Debug for KAZE {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KAZE")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::LATCH]
// LATCH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:226
pub trait LATCHTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_LATCH(&self) -> *const c_void;

	// getBytes()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:232
	// ("cv::xfeatures2d::LATCH::getBytes", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_bytes(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_getBytes_const(self.as_raw_LATCH(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getRotationInvariance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:235
	// ("cv::xfeatures2d::LATCH::getRotationInvariance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_rotation_invariance(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_getRotationInvariance_const(self.as_raw_LATCH(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getHalfSSDsize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:238
	// ("cv::xfeatures2d::LATCH::getHalfSSDsize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_half_ss_dsize(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_getHalfSSDsize_const(self.as_raw_LATCH(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:241
	// ("cv::xfeatures2d::LATCH::getSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sigma(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_getSigma_const(self.as_raw_LATCH(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:243
	// ("cv::xfeatures2d::LATCH::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_getDefaultName_const(self.as_raw_LATCH(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::LATCH]
pub trait LATCHTrait: crate::features::Feature2DTrait + crate::xfeatures2d::LATCHTraitConst {
	fn as_raw_mut_LATCH(&mut self) -> *mut c_void;

	// setBytes(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:231
	// ("cv::xfeatures2d::LATCH::setBytes", vec![(pred!(mut, ["bytes"], ["int"]), _)]),
	#[inline]
	fn set_bytes(&mut self, bytes: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_setBytes_int(self.as_raw_mut_LATCH(), bytes, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setRotationInvariance(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:234
	// ("cv::xfeatures2d::LATCH::setRotationInvariance", vec![(pred!(mut, ["rotationInvariance"], ["bool"]), _)]),
	#[inline]
	fn set_rotation_invariance(&mut self, rotation_invariance: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_setRotationInvariance_bool(self.as_raw_mut_LATCH(), rotation_invariance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setHalfSSDsize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:237
	// ("cv::xfeatures2d::LATCH::setHalfSSDsize", vec![(pred!(mut, ["half_ssd_size"], ["int"]), _)]),
	#[inline]
	fn set_half_ss_dsize(&mut self, half_ssd_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_setHalfSSDsize_int(self.as_raw_mut_LATCH(), half_ssd_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:240
	// ("cv::xfeatures2d::LATCH::setSigma", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
	#[inline]
	fn set_sigma(&mut self, sigma: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_setSigma_double(self.as_raw_mut_LATCH(), sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// latch Class for computing the LATCH descriptor.
/// If you find this code useful, please add a reference to the following paper in your work:
/// Gil Levi and Tal Hassner, "LATCH: Learned Arrangements of Three Patch Codes", arXiv preprint arXiv:1501.03719, 15 Jan. 2015
///
/// LATCH is a binary descriptor based on learned comparisons of triplets of image patches.
///
/// * bytes is the size of the descriptor - can be 64, 32, 16, 8, 4, 2 or 1
/// * rotationInvariance - whether or not the descriptor should compansate for orientation changes.
/// * half_ssd_size - the size of half of the mini-patches size. For example, if we would like to compare triplets of patches of size 7x7x
///    then the half_ssd_size should be (7-1)/2 = 3.
/// * sigma - sigma value for GaussianBlur smoothing of the source image. Source image will be used without smoothing in case sigma value is 0.
///
/// Note: the descriptor can be coupled with any keypoint extractor. The only demand is that if you use set rotationInvariance = True then
///    you will have to use an extractor which estimates the patch orientation (in degrees). Examples for such extractors are ORB and SIFT.
///
/// Note: a complete example can be found under /samples/cpp/tutorial_code/xfeatures2D/latch_match.cpp
// LATCH /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:226
pub struct LATCH {
	ptr: *mut c_void,
}

opencv_type_boxed! { LATCH }

impl Drop for LATCH {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_LATCH_delete(self.as_raw_mut_LATCH()) };
	}
}

unsafe impl Send for LATCH {}

impl core::AlgorithmTraitConst for LATCH {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LATCH {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LATCH, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for LATCH {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for LATCH {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LATCH, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::LATCHTraitConst for LATCH {
	#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::LATCHTrait for LATCH {
	#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LATCH, crate::xfeatures2d::LATCHTraitConst, as_raw_LATCH, crate::xfeatures2d::LATCHTrait, as_raw_mut_LATCH }

impl LATCH {
	/// ## C++ default parameters
	/// * bytes: 32
	/// * rotation_invariance: true
	/// * half_ssd_size: 3
	/// * sigma: 2.0
	// create(int, bool, int, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:229
	// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, ["bytes", "rotationInvariance", "half_ssd_size", "sigma"], ["int", "bool", "int", "double"]), _)]),
	#[inline]
	pub fn create(bytes: i32, rotation_invariance: bool, half_ssd_size: i32, sigma: f64) -> Result<core::Ptr<crate::xfeatures2d::LATCH>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_create_int_bool_int_double(bytes, rotation_invariance, half_ssd_size, sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::LATCH>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [LATCH::create] function uses the following default values for its arguments:
	/// * bytes: 32
	/// * rotation_invariance: true
	/// * half_ssd_size: 3
	/// * sigma: 2.0
	// cv::xfeatures2d::LATCH::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:229
	// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::LATCH>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::LATCH>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LATCH, core::Algorithm, cv_xfeatures2d_LATCH_to_Algorithm }

boxed_cast_base! { LATCH, crate::features::Feature2D, cv_xfeatures2d_LATCH_to_Feature2D }

impl std::fmt::Debug for LATCH {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LATCH")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::LUCID]
// LUCID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:185
pub trait LUCIDTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_LUCID(&self) -> *const c_void;

	// getLucidKernel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:195
	// ("cv::xfeatures2d::LUCID::getLucidKernel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_lucid_kernel(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_getLucidKernel_const(self.as_raw_LUCID(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBlurKernel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:198
	// ("cv::xfeatures2d::LUCID::getBlurKernel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_blur_kernel(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_getBlurKernel_const(self.as_raw_LUCID(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:200
	// ("cv::xfeatures2d::LUCID::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_getDefaultName_const(self.as_raw_LUCID(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::LUCID]
pub trait LUCIDTrait: crate::features::Feature2DTrait + crate::xfeatures2d::LUCIDTraitConst {
	fn as_raw_mut_LUCID(&mut self) -> *mut c_void;

	// setLucidKernel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:194
	// ("cv::xfeatures2d::LUCID::setLucidKernel", vec![(pred!(mut, ["lucid_kernel"], ["int"]), _)]),
	#[inline]
	fn set_lucid_kernel(&mut self, lucid_kernel: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_setLucidKernel_int(self.as_raw_mut_LUCID(), lucid_kernel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBlurKernel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:197
	// ("cv::xfeatures2d::LUCID::setBlurKernel", vec![(pred!(mut, ["blur_kernel"], ["int"]), _)]),
	#[inline]
	fn set_blur_kernel(&mut self, blur_kernel: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_setBlurKernel_int(self.as_raw_mut_LUCID(), blur_kernel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the locally uniform comparison image descriptor, described in [LUCID](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_LUCID)
///
/// An image descriptor that can be computed very fast, while being
/// about as robust as, for example, SURF or BRIEF.
///
///
/// Note: It requires a color image as input.
// LUCID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:185
pub struct LUCID {
	ptr: *mut c_void,
}

opencv_type_boxed! { LUCID }

impl Drop for LUCID {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_LUCID_delete(self.as_raw_mut_LUCID()) };
	}
}

unsafe impl Send for LUCID {}

impl core::AlgorithmTraitConst for LUCID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LUCID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LUCID, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for LUCID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for LUCID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LUCID, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::LUCIDTraitConst for LUCID {
	#[inline] fn as_raw_LUCID(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::LUCIDTrait for LUCID {
	#[inline] fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LUCID, crate::xfeatures2d::LUCIDTraitConst, as_raw_LUCID, crate::xfeatures2d::LUCIDTrait, as_raw_mut_LUCID }

impl LUCID {
	/// ## Parameters
	/// * lucid_kernel: kernel for descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
	/// * blur_kernel: kernel for blurring image prior to descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
	///
	/// ## C++ default parameters
	/// * lucid_kernel: 1
	/// * blur_kernel: 2
	// create(const int, const int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:192
	// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, ["lucid_kernel", "blur_kernel"], ["const int", "const int"]), _)]),
	#[inline]
	pub fn create(lucid_kernel: i32, blur_kernel: i32) -> Result<core::Ptr<crate::xfeatures2d::LUCID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_create_const_int_const_int(lucid_kernel, blur_kernel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::LUCID>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * lucid_kernel: kernel for descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
	/// * blur_kernel: kernel for blurring image prior to descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
	///
	/// ## Note
	/// This alternative version of [LUCID::create] function uses the following default values for its arguments:
	/// * lucid_kernel: 1
	/// * blur_kernel: 2
	// cv::xfeatures2d::LUCID::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:192
	// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::LUCID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::LUCID>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LUCID, core::Algorithm, cv_xfeatures2d_LUCID_to_Algorithm }

boxed_cast_base! { LUCID, crate::features::Feature2D, cv_xfeatures2d_LUCID_to_Feature2D }

impl std::fmt::Debug for LUCID {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LUCID")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::MSDDetector]
// MSDDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:467
pub trait MSDDetectorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_MSDDetector(&self) -> *const c_void;

	// getPatchRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:476
	// ("cv::xfeatures2d::MSDDetector::getPatchRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_patch_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getPatchRadius_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSearchAreaRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:479
	// ("cv::xfeatures2d::MSDDetector::getSearchAreaRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_search_area_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getSearchAreaRadius_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNmsRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:482
	// ("cv::xfeatures2d::MSDDetector::getNmsRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nms_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getNmsRadius_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNmsScaleRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:485
	// ("cv::xfeatures2d::MSDDetector::getNmsScaleRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nms_scale_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getNmsScaleRadius_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getThSaliency()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:488
	// ("cv::xfeatures2d::MSDDetector::getThSaliency", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_th_saliency(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getThSaliency_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getKNN()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:491
	// ("cv::xfeatures2d::MSDDetector::getKNN", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_knn(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getKNN_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:494
	// ("cv::xfeatures2d::MSDDetector::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getScaleFactor_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNScales()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:497
	// ("cv::xfeatures2d::MSDDetector::getNScales", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_scales(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getNScales_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getComputeOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:500
	// ("cv::xfeatures2d::MSDDetector::getComputeOrientation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_compute_orientation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getComputeOrientation_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:502
	// ("cv::xfeatures2d::MSDDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_getDefaultName_const(self.as_raw_MSDDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::MSDDetector]
pub trait MSDDetectorTrait: crate::features::Feature2DTrait + crate::xfeatures2d::MSDDetectorTraitConst {
	fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void;

	// setPatchRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:475
	// ("cv::xfeatures2d::MSDDetector::setPatchRadius", vec![(pred!(mut, ["patch_radius"], ["int"]), _)]),
	#[inline]
	fn set_patch_radius(&mut self, patch_radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setPatchRadius_int(self.as_raw_mut_MSDDetector(), patch_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSearchAreaRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:478
	// ("cv::xfeatures2d::MSDDetector::setSearchAreaRadius", vec![(pred!(mut, ["use_orientation"], ["int"]), _)]),
	#[inline]
	fn set_search_area_radius(&mut self, use_orientation: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setSearchAreaRadius_int(self.as_raw_mut_MSDDetector(), use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNmsRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:481
	// ("cv::xfeatures2d::MSDDetector::setNmsRadius", vec![(pred!(mut, ["nms_radius"], ["int"]), _)]),
	#[inline]
	fn set_nms_radius(&mut self, nms_radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setNmsRadius_int(self.as_raw_mut_MSDDetector(), nms_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNmsScaleRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:484
	// ("cv::xfeatures2d::MSDDetector::setNmsScaleRadius", vec![(pred!(mut, ["nms_scale_radius"], ["int"]), _)]),
	#[inline]
	fn set_nms_scale_radius(&mut self, nms_scale_radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setNmsScaleRadius_int(self.as_raw_mut_MSDDetector(), nms_scale_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setThSaliency(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:487
	// ("cv::xfeatures2d::MSDDetector::setThSaliency", vec![(pred!(mut, ["th_saliency"], ["float"]), _)]),
	#[inline]
	fn set_th_saliency(&mut self, th_saliency: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setThSaliency_float(self.as_raw_mut_MSDDetector(), th_saliency, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setKNN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:490
	// ("cv::xfeatures2d::MSDDetector::setKNN", vec![(pred!(mut, ["kNN"], ["int"]), _)]),
	#[inline]
	fn set_knn(&mut self, k_nn: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setKNN_int(self.as_raw_mut_MSDDetector(), k_nn, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:493
	// ("cv::xfeatures2d::MSDDetector::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setScaleFactor_float(self.as_raw_mut_MSDDetector(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNScales(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:496
	// ("cv::xfeatures2d::MSDDetector::setNScales", vec![(pred!(mut, ["use_orientation"], ["int"]), _)]),
	#[inline]
	fn set_n_scales(&mut self, use_orientation: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setNScales_int(self.as_raw_mut_MSDDetector(), use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setComputeOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:499
	// ("cv::xfeatures2d::MSDDetector::setComputeOrientation", vec![(pred!(mut, ["compute_orientation"], ["bool"]), _)]),
	#[inline]
	fn set_compute_orientation(&mut self, compute_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_setComputeOrientation_bool(self.as_raw_mut_MSDDetector(), compute_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the MSD (*Maximal Self-Dissimilarity*) keypoint detector, described in [Tombari14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Tombari14).
///
/// The algorithm implements a novel interest point detector stemming from the intuition that image patches
/// which are highly dissimilar over a relatively large extent of their surroundings hold the property of
/// being repeatable and distinctive. This concept of "contextual self-dissimilarity" reverses the key
/// paradigm of recent successful techniques such as the Local Self-Similarity descriptor and the Non-Local
/// Means filter, which build upon the presence of similar - rather than dissimilar - patches. Moreover,
/// it extends to contextual information the local self-dissimilarity notion embedded in established
/// detectors of corner-like interest points, thereby achieving enhanced repeatability, distinctiveness and
/// localization accuracy.
// MSDDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:467
pub struct MSDDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { MSDDetector }

impl Drop for MSDDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_MSDDetector_delete(self.as_raw_mut_MSDDetector()) };
	}
}

unsafe impl Send for MSDDetector {}

impl core::AlgorithmTraitConst for MSDDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MSDDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSDDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for MSDDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for MSDDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSDDetector, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::MSDDetectorTraitConst for MSDDetector {
	#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::MSDDetectorTrait for MSDDetector {
	#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MSDDetector, crate::xfeatures2d::MSDDetectorTraitConst, as_raw_MSDDetector, crate::xfeatures2d::MSDDetectorTrait, as_raw_mut_MSDDetector }

impl MSDDetector {
	/// ## C++ default parameters
	/// * m_patch_radius: 3
	/// * m_search_area_radius: 5
	/// * m_nms_radius: 5
	/// * m_nms_scale_radius: 0
	/// * m_th_saliency: 250.0f
	/// * m_k_nn: 4
	/// * m_scale_factor: 1.25f
	/// * m_n_scales: -1
	/// * m_compute_orientation: false
	// create(int, int, int, int, float, int, float, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:471
	// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, ["m_patch_radius", "m_search_area_radius", "m_nms_radius", "m_nms_scale_radius", "m_th_saliency", "m_kNN", "m_scale_factor", "m_n_scales", "m_compute_orientation"], ["int", "int", "int", "int", "float", "int", "float", "int", "bool"]), _)]),
	#[inline]
	pub fn create(m_patch_radius: i32, m_search_area_radius: i32, m_nms_radius: i32, m_nms_scale_radius: i32, m_th_saliency: f32, m_k_nn: i32, m_scale_factor: f32, m_n_scales: i32, m_compute_orientation: bool) -> Result<core::Ptr<crate::xfeatures2d::MSDDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_k_nn, m_scale_factor, m_n_scales, m_compute_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::MSDDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [MSDDetector::create] function uses the following default values for its arguments:
	/// * m_patch_radius: 3
	/// * m_search_area_radius: 5
	/// * m_nms_radius: 5
	/// * m_nms_scale_radius: 0
	/// * m_th_saliency: 250.0f
	/// * m_k_nn: 4
	/// * m_scale_factor: 1.25f
	/// * m_n_scales: -1
	/// * m_compute_orientation: false
	// cv::xfeatures2d::MSDDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:471
	// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::MSDDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::MSDDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MSDDetector, core::Algorithm, cv_xfeatures2d_MSDDetector_to_Algorithm }

boxed_cast_base! { MSDDetector, crate::features::Feature2D, cv_xfeatures2d_MSDDetector_to_Feature2D }

impl std::fmt::Debug for MSDDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MSDDetector")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::PCTSignatures]
// PCTSignatures /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:621
pub trait PCTSignaturesTraitConst: core::AlgorithmTraitConst {
	fn as_raw_PCTSignatures(&self) -> *const c_void;

	/// Computes signature of given image.
	/// ## Parameters
	/// * image: Input image of CV_8U type.
	/// * signature: Output computed signature.
	// computeSignature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:707
	// ("cv::xfeatures2d::PCTSignatures::computeSignature", vec![(pred!(const, ["image", "signature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_signature(&self, image: &impl ToInputArray, signature: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(signature);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(self.as_raw_PCTSignatures(), image.as_raw__InputArray(), signature.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes signatures for multiple images in parallel.
	/// ## Parameters
	/// * images: Vector of input images of CV_8U type.
	/// * signatures: Vector of computed signatures.
	// computeSignatures(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:716
	// ("cv::xfeatures2d::PCTSignatures::computeSignatures", vec![(pred!(const, ["images", "signatures"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn compute_signatures(&self, images: &core::Vector<core::Mat>, signatures: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vectorLMatGR_vectorLMatGR(self.as_raw_PCTSignatures(), images.as_raw_VectorOfMat(), signatures.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of initial samples taken from the image.
	// getSampleCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:758
	// ("cv::xfeatures2d::PCTSignatures::getSampleCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sample_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getSampleCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Color resolution of the greyscale bitmap represented in allocated bits
	///       (i.e., value 4 means that 16 shades of grey are used).
	///       The greyscale bitmap is used for computing contrast and entropy values.
	// getGrayscaleBits()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:765
	// ("cv::xfeatures2d::PCTSignatures::getGrayscaleBits", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_grayscale_bits(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Size of the texture sampling window used to compute contrast and entropy
	///       (center of the window is always in the pixel selected by x,y coordinates
	///       of the corresponding feature sample).
	// getWindowRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:778
	// ("cv::xfeatures2d::PCTSignatures::getWindowRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWindowRadius_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// getWeightX()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:791
	// ("cv::xfeatures2d::PCTSignatures::getWeightX", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weight_x(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightX_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// getWeightY()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:802
	// ("cv::xfeatures2d::PCTSignatures::getWeightY", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weight_y(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightY_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// getWeightL()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:813
	// ("cv::xfeatures2d::PCTSignatures::getWeightL", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weight_l(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightL_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// getWeightA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:824
	// ("cv::xfeatures2d::PCTSignatures::getWeightA", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weight_a(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightA_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// getWeightB()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:835
	// ("cv::xfeatures2d::PCTSignatures::getWeightB", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weight_b(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightB_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// getWeightContrast()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:846
	// ("cv::xfeatures2d::PCTSignatures::getWeightContrast", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weight_contrast(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightContrast_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// getWeightEntropy()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:857
	// ("cv::xfeatures2d::PCTSignatures::getWeightEntropy", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_weight_entropy(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Initial samples taken from the image.
	///       These sampled features become the input for clustering.
	// getSamplingPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:868
	// ("cv::xfeatures2d::PCTSignatures::getSamplingPoints", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sampling_points(&self) -> Result<core::Vector<core::Point2f>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ** clusterizer ***
	///
	/// * Initial seeds (initial number of clusters) for the k-means algorithm.
	// getInitSeedIndexes()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:945
	// ("cv::xfeatures2d::PCTSignatures::getInitSeedIndexes", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_init_seed_indexes(&self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Number of initial seeds (initial number of clusters) for the k-means algorithm.
	// getInitSeedCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:953
	// ("cv::xfeatures2d::PCTSignatures::getInitSeedCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_init_seed_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of iterations of the k-means clustering.
	///       We use fixed number of iterations, since the modified clustering is pruning clusters
	///       (not iteratively refining k clusters).
	// getIterationCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:960
	// ("cv::xfeatures2d::PCTSignatures::getIterationCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iteration_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getIterationCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Maximal number of generated clusters. If the number is exceeded,
	///       the clusters are sorted by their weights and the smallest clusters are cropped.
	// getMaxClustersCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:972
	// ("cv::xfeatures2d::PCTSignatures::getMaxClustersCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_clusters_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This parameter multiplied by the index of iteration gives lower limit for cluster size.
	///       Clusters containing fewer points than specified by the limit have their centroid dismissed
	///       and points are reassigned.
	// getClusterMinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:984
	// ("cv::xfeatures2d::PCTSignatures::getClusterMinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_cluster_min_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Threshold euclidean distance between two centroids.
	///       If two cluster centers are closer than this distance,
	///       one of the centroid is dismissed and points are reassigned.
	// getJoiningDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:997
	// ("cv::xfeatures2d::PCTSignatures::getJoiningDistance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_joining_distance(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Remove centroids in k-means whose weight is lesser or equal to given threshold.
	// getDropThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1008
	// ("cv::xfeatures2d::PCTSignatures::getDropThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_drop_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getDropThreshold_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Distance function selector used for measuring distance between two points in k-means.
	// getDistanceFunction()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1017
	// ("cv::xfeatures2d::PCTSignatures::getDistanceFunction", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_distance_function(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::PCTSignatures]
pub trait PCTSignaturesTrait: core::AlgorithmTrait + crate::xfeatures2d::PCTSignaturesTraitConst {
	fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void;

	/// Color resolution of the greyscale bitmap represented in allocated bits
	///       (i.e., value 4 means that 16 shades of grey are used).
	///       The greyscale bitmap is used for computing contrast and entropy values.
	// setGrayscaleBits(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:771
	// ("cv::xfeatures2d::PCTSignatures::setGrayscaleBits", vec![(pred!(mut, ["grayscaleBits"], ["int"]), _)]),
	#[inline]
	fn set_grayscale_bits(&mut self, grayscale_bits: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(self.as_raw_mut_PCTSignatures(), grayscale_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Size of the texture sampling window used to compute contrast and entropy
	///       (center of the window is always in the pixel selected by x,y coordinates
	///       of the corresponding feature sample).
	// setWindowRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:784
	// ("cv::xfeatures2d::PCTSignatures::setWindowRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	#[inline]
	fn set_window_radius(&mut self, radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWindowRadius_int(self.as_raw_mut_PCTSignatures(), radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// setWeightX(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:796
	// ("cv::xfeatures2d::PCTSignatures::setWeightX", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	#[inline]
	fn set_weight_x(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightX_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// setWeightY(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:807
	// ("cv::xfeatures2d::PCTSignatures::setWeightY", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	#[inline]
	fn set_weight_y(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightY_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// setWeightL(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:818
	// ("cv::xfeatures2d::PCTSignatures::setWeightL", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	#[inline]
	fn set_weight_l(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightL_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// setWeightA(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:829
	// ("cv::xfeatures2d::PCTSignatures::setWeightA", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	#[inline]
	fn set_weight_a(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightA_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// setWeightB(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:840
	// ("cv::xfeatures2d::PCTSignatures::setWeightB", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	#[inline]
	fn set_weight_b(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightB_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// setWeightContrast(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:851
	// ("cv::xfeatures2d::PCTSignatures::setWeightContrast", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	#[inline]
	fn set_weight_contrast(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightContrast_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	// setWeightEntropy(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:862
	// ("cv::xfeatures2d::PCTSignatures::setWeightEntropy", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	#[inline]
	fn set_weight_entropy(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space.
	/// ## Parameters
	/// * idx: ID of the weight
	/// * value: Value of the weight
	///
	/// Note:
	///       WEIGHT_IDX = 0;
	///       X_IDX = 1;
	///       Y_IDX = 2;
	///       L_IDX = 3;
	///       A_IDX = 4;
	///       B_IDX = 5;
	///       CONTRAST_IDX = 6;
	///       ENTROPY_IDX = 7;
	// setWeight(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:886
	// ("cv::xfeatures2d::PCTSignatures::setWeight", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
	#[inline]
	fn set_weight(&mut self, idx: i32, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeight_int_float(self.as_raw_mut_PCTSignatures(), idx, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space.
	/// ## Parameters
	/// * weights: Values of all weights.
	///
	/// Note:
	///       WEIGHT_IDX = 0;
	///       X_IDX = 1;
	///       Y_IDX = 2;
	///       L_IDX = 3;
	///       A_IDX = 4;
	///       B_IDX = 5;
	///       CONTRAST_IDX = 6;
	///       ENTROPY_IDX = 7;
	// setWeights(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:900
	// ("cv::xfeatures2d::PCTSignatures::setWeights", vec![(pred!(mut, ["weights"], ["const std::vector<float>*"]), _)]),
	#[inline]
	fn set_weights(&mut self, weights: &core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeights_const_vectorLfloatGR(self.as_raw_mut_PCTSignatures(), weights.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Translations of the individual axes of the feature space.
	/// ## Parameters
	/// * idx: ID of the translation
	/// * value: Value of the translation
	///
	/// Note:
	///       WEIGHT_IDX = 0;
	///       X_IDX = 1;
	///       Y_IDX = 2;
	///       L_IDX = 3;
	///       A_IDX = 4;
	///       B_IDX = 5;
	///       CONTRAST_IDX = 6;
	///       ENTROPY_IDX = 7;
	// setTranslation(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:916
	// ("cv::xfeatures2d::PCTSignatures::setTranslation", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
	#[inline]
	fn set_translation(&mut self, idx: i32, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslation_int_float(self.as_raw_mut_PCTSignatures(), idx, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Translations of the individual axes of the feature space.
	/// ## Parameters
	/// * translations: Values of all translations.
	///
	/// Note:
	///       WEIGHT_IDX = 0;
	///       X_IDX = 1;
	///       Y_IDX = 2;
	///       L_IDX = 3;
	///       A_IDX = 4;
	///       B_IDX = 5;
	///       CONTRAST_IDX = 6;
	///       ENTROPY_IDX = 7;
	// setTranslations(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:930
	// ("cv::xfeatures2d::PCTSignatures::setTranslations", vec![(pred!(mut, ["translations"], ["const std::vector<float>*"]), _)]),
	#[inline]
	fn set_translations(&mut self, translations: &core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslations_const_vectorLfloatGR(self.as_raw_mut_PCTSignatures(), translations.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets sampling points used to sample the input image.
	/// ## Parameters
	/// * samplingPoints: Vector of sampling points in range [0..1)
	///
	/// Note: Number of sampling points must be greater or equal to clusterization seed count.
	// setSamplingPoints(std::vector<Point2f>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:937
	// ("cv::xfeatures2d::PCTSignatures::setSamplingPoints", vec![(pred!(mut, ["samplingPoints"], ["std::vector<cv::Point2f>"]), _)]),
	#[inline]
	fn set_sampling_points(&mut self, mut sampling_points: core::Vector<core::Point2f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setSamplingPoints_vectorLPoint2fG(self.as_raw_mut_PCTSignatures(), sampling_points.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Initial seed indexes for the k-means algorithm.
	// setInitSeedIndexes(std::vector<int>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:949
	// ("cv::xfeatures2d::PCTSignatures::setInitSeedIndexes", vec![(pred!(mut, ["initSeedIndexes"], ["std::vector<int>"]), _)]),
	#[inline]
	fn set_init_seed_indexes(&mut self, mut init_seed_indexes: core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vectorLintG(self.as_raw_mut_PCTSignatures(), init_seed_indexes.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Number of iterations of the k-means clustering.
	///       We use fixed number of iterations, since the modified clustering is pruning clusters
	///       (not iteratively refining k clusters).
	// setIterationCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:966
	// ("cv::xfeatures2d::PCTSignatures::setIterationCount", vec![(pred!(mut, ["iterationCount"], ["int"]), _)]),
	#[inline]
	fn set_iteration_count(&mut self, iteration_count: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setIterationCount_int(self.as_raw_mut_PCTSignatures(), iteration_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Maximal number of generated clusters. If the number is exceeded,
	///       the clusters are sorted by their weights and the smallest clusters are cropped.
	// setMaxClustersCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:977
	// ("cv::xfeatures2d::PCTSignatures::setMaxClustersCount", vec![(pred!(mut, ["maxClustersCount"], ["int"]), _)]),
	#[inline]
	fn set_max_clusters_count(&mut self, max_clusters_count: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(self.as_raw_mut_PCTSignatures(), max_clusters_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This parameter multiplied by the index of iteration gives lower limit for cluster size.
	///       Clusters containing fewer points than specified by the limit have their centroid dismissed
	///       and points are reassigned.
	// setClusterMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:990
	// ("cv::xfeatures2d::PCTSignatures::setClusterMinSize", vec![(pred!(mut, ["clusterMinSize"], ["int"]), _)]),
	#[inline]
	fn set_cluster_min_size(&mut self, cluster_min_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(self.as_raw_mut_PCTSignatures(), cluster_min_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Threshold euclidean distance between two centroids.
	///       If two cluster centers are closer than this distance,
	///       one of the centroid is dismissed and points are reassigned.
	// setJoiningDistance(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1003
	// ("cv::xfeatures2d::PCTSignatures::setJoiningDistance", vec![(pred!(mut, ["joiningDistance"], ["float"]), _)]),
	#[inline]
	fn set_joining_distance(&mut self, joining_distance: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(self.as_raw_mut_PCTSignatures(), joining_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Remove centroids in k-means whose weight is lesser or equal to given threshold.
	// setDropThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1012
	// ("cv::xfeatures2d::PCTSignatures::setDropThreshold", vec![(pred!(mut, ["dropThreshold"], ["float"]), _)]),
	#[inline]
	fn set_drop_threshold(&mut self, drop_threshold: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setDropThreshold_float(self.as_raw_mut_PCTSignatures(), drop_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Distance function selector used for measuring distance between two points in k-means.
	///       Available: L0_25, L0_5, L1, L2, L2SQUARED, L5, L_INFINITY.
	// setDistanceFunction(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1022
	// ("cv::xfeatures2d::PCTSignatures::setDistanceFunction", vec![(pred!(mut, ["distanceFunction"], ["int"]), _)]),
	#[inline]
	fn set_distance_function(&mut self, distance_function: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(self.as_raw_mut_PCTSignatures(), distance_function, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing PCT (position-color-texture) signature extraction
///       as described in [KrulisLS16](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_KrulisLS16).
///       The algorithm is divided to a feature sampler and a clusterizer.
///       Feature sampler produces samples at given set of coordinates.
///       Clusterizer then produces clusters of these samples using k-means algorithm.
///       Resulting set of clusters is the signature of the input image.
///
///       A signature is an array of SIGNATURE_DIMENSION-dimensional points.
///       Used dimensions are:
///       weight, x, y position; lab color, contrast, entropy.
/// [KrulisLS16](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_KrulisLS16)
/// [BeecksUS10](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BeecksUS10)
// PCTSignatures /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:621
pub struct PCTSignatures {
	ptr: *mut c_void,
}

opencv_type_boxed! { PCTSignatures }

impl Drop for PCTSignatures {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_delete(self.as_raw_mut_PCTSignatures()) };
	}
}

unsafe impl Send for PCTSignatures {}

impl core::AlgorithmTraitConst for PCTSignatures {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PCTSignatures {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PCTSignatures, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::xfeatures2d::PCTSignaturesTraitConst for PCTSignatures {
	#[inline] fn as_raw_PCTSignatures(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::PCTSignaturesTrait for PCTSignatures {
	#[inline] fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PCTSignatures, crate::xfeatures2d::PCTSignaturesTraitConst, as_raw_PCTSignatures, crate::xfeatures2d::PCTSignaturesTrait, as_raw_mut_PCTSignatures }

impl PCTSignatures {
	/// Creates PCTSignatures algorithm using sample and seed count.
	///       It generates its own sets of sampling points and clusterization seed indexes.
	/// ## Parameters
	/// * initSampleCount: Number of points used for image sampling.
	/// * initSeedCount: Number of initial clusterization seeds.
	///       Must be lower or equal to initSampleCount
	/// * pointDistribution: Distribution of generated points. Default: UNIFORM.
	///       Available: UNIFORM, REGULAR, NORMAL.
	/// ## Returns
	/// Created algorithm.
	///
	/// ## C++ default parameters
	/// * init_sample_count: 2000
	/// * init_seed_count: 400
	/// * point_distribution: 0
	// create(const int, const int, const int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:670
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSampleCount", "initSeedCount", "pointDistribution"], ["const int", "const int", "const int"]), _)]),
	#[inline]
	pub fn create(init_sample_count: i32, init_seed_count: i32, point_distribution: i32) -> Result<core::Ptr<crate::xfeatures2d::PCTSignatures>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(init_sample_count, init_seed_count, point_distribution, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::PCTSignatures>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates PCTSignatures algorithm using sample and seed count.
	///       It generates its own sets of sampling points and clusterization seed indexes.
	/// ## Parameters
	/// * initSampleCount: Number of points used for image sampling.
	/// * initSeedCount: Number of initial clusterization seeds.
	///       Must be lower or equal to initSampleCount
	/// * pointDistribution: Distribution of generated points. Default: UNIFORM.
	///       Available: UNIFORM, REGULAR, NORMAL.
	/// ## Returns
	/// Created algorithm.
	///
	/// ## Note
	/// This alternative version of [PCTSignatures::create] function uses the following default values for its arguments:
	/// * init_sample_count: 2000
	/// * init_seed_count: 400
	/// * point_distribution: 0
	// cv::xfeatures2d::PCTSignatures::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:670
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::PCTSignatures>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::PCTSignatures>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates PCTSignatures algorithm using pre-generated sampling points
	///       and number of clusterization seeds. It uses the provided
	///       sampling points and generates its own clusterization seed indexes.
	/// ## Parameters
	/// * initSamplingPoints: Sampling points used in image sampling.
	/// * initSeedCount: Number of initial clusterization seeds.
	///       Must be lower or equal to initSamplingPoints.size().
	/// ## Returns
	/// Created algorithm.
	// create(const std::vector<Point2f> &, const int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:684
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initSeedCount"], ["const std::vector<cv::Point2f>*", "const int"]), _)]),
	#[inline]
	pub fn create_1(init_sampling_points: &core::Vector<core::Point2f>, init_seed_count: i32) -> Result<core::Ptr<crate::xfeatures2d::PCTSignatures>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_int(init_sampling_points.as_raw_VectorOfPoint2f(), init_seed_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::PCTSignatures>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates PCTSignatures algorithm using pre-generated sampling points
	///       and clusterization seeds indexes.
	/// ## Parameters
	/// * initSamplingPoints: Sampling points used in image sampling.
	/// * initClusterSeedIndexes: Indexes of initial clusterization seeds.
	///       Its size must be lower or equal to initSamplingPoints.size().
	/// ## Returns
	/// Created algorithm.
	// create(const std::vector<Point2f> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:696
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initClusterSeedIndexes"], ["const std::vector<cv::Point2f>*", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create_2(init_sampling_points: &core::Vector<core::Point2f>, init_cluster_seed_indexes: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::PCTSignatures>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_vectorLintGR(init_sampling_points.as_raw_VectorOfPoint2f(), init_cluster_seed_indexes.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::PCTSignatures>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Draws signature in the source image and outputs the result.
	///       Signatures are visualized as a circle
	///       with radius based on signature weight
	///       and color based on signature color.
	///       Contrast and entropy are not visualized.
	/// ## Parameters
	/// * source: Source image.
	/// * signature: Image signature.
	/// * result: Output result.
	/// * radiusToShorterSideRatio: Determines maximal radius of signature in the output image.
	/// * borderThickness: Border thickness of the visualized signature.
	///
	/// ## C++ default parameters
	/// * radius_to_shorter_side_ratio: 1.0/8
	/// * border_thickness: 1
	// drawSignature(InputArray, InputArray, OutputArray, float, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:732
	// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result", "radiusToShorterSideRatio", "borderThickness"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "int"]), _)]),
	#[inline]
	pub fn draw_signature(source: &impl ToInputArray, signature: &impl ToInputArray, result: &mut impl ToOutputArray, radius_to_shorter_side_ratio: f32, border_thickness: i32) -> Result<()> {
		input_array_arg!(source);
		input_array_arg!(signature);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(source.as_raw__InputArray(), signature.as_raw__InputArray(), result.as_raw__OutputArray(), radius_to_shorter_side_ratio, border_thickness, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Draws signature in the source image and outputs the result.
	///       Signatures are visualized as a circle
	///       with radius based on signature weight
	///       and color based on signature color.
	///       Contrast and entropy are not visualized.
	/// ## Parameters
	/// * source: Source image.
	/// * signature: Image signature.
	/// * result: Output result.
	/// * radiusToShorterSideRatio: Determines maximal radius of signature in the output image.
	/// * borderThickness: Border thickness of the visualized signature.
	///
	/// ## Note
	/// This alternative version of [PCTSignatures::draw_signature] function uses the following default values for its arguments:
	/// * radius_to_shorter_side_ratio: 1.0/8
	/// * border_thickness: 1
	// cv::xfeatures2d::PCTSignatures::drawSignature(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:732
	// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	pub fn draw_signature_def(source: &impl ToInputArray, signature: &impl ToInputArray, result: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(source);
		input_array_arg!(signature);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR(source.as_raw__InputArray(), signature.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Generates initial sampling points according to selected point distribution.
	/// ## Parameters
	/// * initPoints: Output vector where the generated points will be saved.
	/// * count: Number of points to generate.
	/// * pointDistribution: Point distribution selector.
	///       Available: UNIFORM, REGULAR, NORMAL.
	///
	/// Note: Generated coordinates are in range [0..1)
	// generateInitPoints(std::vector<Point2f> &, const int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:747
	// ("cv::xfeatures2d::PCTSignatures::generateInitPoints", vec![(pred!(mut, ["initPoints", "count", "pointDistribution"], ["std::vector<cv::Point2f>*", "const int", "int"]), _)]),
	#[inline]
	pub fn generate_init_points(init_points: &mut core::Vector<core::Point2f>, count: i32, point_distribution: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_generateInitPoints_vectorLPoint2fGR_const_int_int(init_points.as_raw_mut_VectorOfPoint2f(), count, point_distribution, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

boxed_cast_base! { PCTSignatures, core::Algorithm, cv_xfeatures2d_PCTSignatures_to_Algorithm }

impl std::fmt::Debug for PCTSignatures {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PCTSignatures")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::PCTSignaturesSQFD]
// PCTSignaturesSQFD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1034
pub trait PCTSignaturesSQFDTraitConst: core::AlgorithmTraitConst {
	fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void;

	/// Computes Signature Quadratic Form Distance of two signatures.
	/// ## Parameters
	/// * _signature0: The first signature.
	/// * _signature1: The second signature.
	// computeQuadraticFormDistance(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1057
	// ("cv::xfeatures2d::PCTSignaturesSQFD::computeQuadraticFormDistance", vec![(pred!(const, ["_signature0", "_signature1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn compute_quadratic_form_distance(&self, _signature0: &impl ToInputArray, _signature1: &impl ToInputArray) -> Result<f32> {
		input_array_arg!(_signature0);
		input_array_arg!(_signature1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(self.as_raw_PCTSignaturesSQFD(), _signature0.as_raw__InputArray(), _signature1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes Signature Quadratic Form Distance between the reference signature
	///       and each of the other image signatures.
	/// ## Parameters
	/// * sourceSignature: The signature to measure distance of other signatures from.
	/// * imageSignatures: Vector of signatures to measure distance from the source signature.
	/// * distances: Output vector of measured distances.
	// computeQuadraticFormDistances(const Mat &, const std::vector<Mat> &, std::vector<float> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1068
	// ("cv::xfeatures2d::PCTSignaturesSQFD::computeQuadraticFormDistances", vec![(pred!(const, ["sourceSignature", "imageSignatures", "distances"], ["const cv::Mat*", "const std::vector<cv::Mat>*", "std::vector<float>*"]), _)]),
	#[inline]
	fn compute_quadratic_form_distances(&self, source_signature: &impl core::MatTraitConst, image_signatures: &core::Vector<core::Mat>, distances: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatR_const_vectorLMatGR_vectorLfloatGR(self.as_raw_PCTSignaturesSQFD(), source_signature.as_raw_Mat(), image_signatures.as_raw_VectorOfMat(), distances.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::PCTSignaturesSQFD]
pub trait PCTSignaturesSQFDTrait: core::AlgorithmTrait + crate::xfeatures2d::PCTSignaturesSQFDTraitConst {
	fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void;

}

/// Class implementing Signature Quadratic Form Distance (SQFD).
/// ## See also
/// Christian Beecks, Merih Seran Uysal, Thomas Seidl.
///   Signature quadratic form distance.
///   In Proceedings of the ACM International Conference on Image and Video Retrieval, pages 438-445.
///   ACM, 2010.
/// [BeecksUS10](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_BeecksUS10)
// PCTSignaturesSQFD /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1034
pub struct PCTSignaturesSQFD {
	ptr: *mut c_void,
}

opencv_type_boxed! { PCTSignaturesSQFD }

impl Drop for PCTSignaturesSQFD {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_delete(self.as_raw_mut_PCTSignaturesSQFD()) };
	}
}

unsafe impl Send for PCTSignaturesSQFD {}

impl core::AlgorithmTraitConst for PCTSignaturesSQFD {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PCTSignaturesSQFD {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PCTSignaturesSQFD, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::xfeatures2d::PCTSignaturesSQFDTraitConst for PCTSignaturesSQFD {
	#[inline] fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::PCTSignaturesSQFDTrait for PCTSignaturesSQFD {
	#[inline] fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PCTSignaturesSQFD, crate::xfeatures2d::PCTSignaturesSQFDTraitConst, as_raw_PCTSignaturesSQFD, crate::xfeatures2d::PCTSignaturesSQFDTrait, as_raw_mut_PCTSignaturesSQFD }

impl PCTSignaturesSQFD {
	/// Creates the algorithm instance using selected distance function,
	///       similarity function and similarity function parameter.
	/// ## Parameters
	/// * distanceFunction: Distance function selector. Default: L2
	///       Available: L0_25, L0_5, L1, L2, L2SQUARED, L5, L_INFINITY
	/// * similarityFunction: Similarity function selector. Default: HEURISTIC
	///       Available: MINUS, GAUSSIAN, HEURISTIC
	/// * similarityParameter: Parameter of the similarity function.
	///
	/// ## C++ default parameters
	/// * distance_function: 3
	/// * similarity_function: 2
	/// * similarity_parameter: 1.0f
	// create(const int, const int, const float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1047
	// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, ["distanceFunction", "similarityFunction", "similarityParameter"], ["const int", "const int", "const float"]), _)]),
	#[inline]
	pub fn create(distance_function: i32, similarity_function: i32, similarity_parameter: f32) -> Result<core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(distance_function, similarity_function, similarity_parameter, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::PCTSignaturesSQFD>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates the algorithm instance using selected distance function,
	///       similarity function and similarity function parameter.
	/// ## Parameters
	/// * distanceFunction: Distance function selector. Default: L2
	///       Available: L0_25, L0_5, L1, L2, L2SQUARED, L5, L_INFINITY
	/// * similarityFunction: Similarity function selector. Default: HEURISTIC
	///       Available: MINUS, GAUSSIAN, HEURISTIC
	/// * similarityParameter: Parameter of the similarity function.
	///
	/// ## Note
	/// This alternative version of [PCTSignaturesSQFD::create] function uses the following default values for its arguments:
	/// * distance_function: 3
	/// * similarity_function: 2
	/// * similarity_parameter: 1.0f
	// cv::xfeatures2d::PCTSignaturesSQFD::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1047
	// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::PCTSignaturesSQFD>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PCTSignaturesSQFD, core::Algorithm, cv_xfeatures2d_PCTSignaturesSQFD_to_Algorithm }

impl std::fmt::Debug for PCTSignaturesSQFD {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PCTSignaturesSQFD")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::SURF]
// SURF /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:85
pub trait SURFTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_SURF(&self) -> *const c_void;

	// getHessianThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:102
	// ("cv::xfeatures2d::SURF::getHessianThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_hessian_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getHessianThreshold_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:105
	// ("cv::xfeatures2d::SURF::getNOctaves", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getNOctaves_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:108
	// ("cv::xfeatures2d::SURF::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getNOctaveLayers_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getExtended()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:111
	// ("cv::xfeatures2d::SURF::getExtended", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_extended(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getExtended_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUpright()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:114
	// ("cv::xfeatures2d::SURF::getUpright", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_upright(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getUpright_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:116
	// ("cv::xfeatures2d::SURF::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getDefaultName_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::SURF]
pub trait SURFTrait: crate::features::Feature2DTrait + crate::xfeatures2d::SURFTraitConst {
	fn as_raw_mut_SURF(&mut self) -> *mut c_void;

	// setHessianThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:101
	// ("cv::xfeatures2d::SURF::setHessianThreshold", vec![(pred!(mut, ["hessianThreshold"], ["double"]), _)]),
	#[inline]
	fn set_hessian_threshold(&mut self, hessian_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setHessianThreshold_double(self.as_raw_mut_SURF(), hessian_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:104
	// ("cv::xfeatures2d::SURF::setNOctaves", vec![(pred!(mut, ["nOctaves"], ["int"]), _)]),
	#[inline]
	fn set_n_octaves(&mut self, n_octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setNOctaves_int(self.as_raw_mut_SURF(), n_octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:107
	// ("cv::xfeatures2d::SURF::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
	#[inline]
	fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setNOctaveLayers_int(self.as_raw_mut_SURF(), n_octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:110
	// ("cv::xfeatures2d::SURF::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
	#[inline]
	fn set_extended(&mut self, extended: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setExtended_bool(self.as_raw_mut_SURF(), extended, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:113
	// ("cv::xfeatures2d::SURF::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
	#[inline]
	fn set_upright(&mut self, upright: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setUpright_bool(self.as_raw_mut_SURF(), upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class for extracting Speeded Up Robust Features from an image [Bay06](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Bay06) .
///
/// The algorithm parameters:
/// *   member int extended
///    *   0 means that the basic descriptors (64 elements each) shall be computed
///    *   1 means that the extended descriptors (128 elements each) shall be computed
/// *   member int upright
///    *   0 means that detector computes orientation of each feature.
///    *   1 means that the orientation is not computed (which is much, much faster). For example,
/// if you match images from a stereo pair, or do image stitching, the matched features
/// likely have very similar angles, and you can speed up feature extraction by setting
/// upright=1.
/// *   member double hessianThreshold
/// Threshold for the keypoint detector. Only features, whose hessian is larger than
/// hessianThreshold are retained by the detector. Therefore, the larger the value, the less
/// keypoints you will get. A good default value could be from 300 to 500, depending from the
/// image contrast.
/// *   member int nOctaves
/// The number of a gaussian pyramid octaves that the detector uses. It is set to 4 by default.
/// If you want to get very large features, use the larger value. If you want just small
/// features, decrease it.
/// *   member int nOctaveLayers
/// The number of images within each octave of a gaussian pyramid. It is set to 2 by default.
///
/// Note:
///    *   An example using the SURF feature detector can be found at
///        opencv_source_code/samples/cpp/generic_descriptor_match.cpp
///    *   Another example using the SURF feature detector, extractor and matcher can be found at
///        opencv_source_code/samples/cpp/matcher_simple.cpp
// SURF /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:85
pub struct SURF {
	ptr: *mut c_void,
}

opencv_type_boxed! { SURF }

impl Drop for SURF {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_SURF_delete(self.as_raw_mut_SURF()) };
	}
}

unsafe impl Send for SURF {}

impl core::AlgorithmTraitConst for SURF {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SURF {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SURF, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for SURF {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for SURF {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SURF, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::SURFTraitConst for SURF {
	#[inline] fn as_raw_SURF(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::SURFTrait for SURF {
	#[inline] fn as_raw_mut_SURF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SURF, crate::xfeatures2d::SURFTraitConst, as_raw_SURF, crate::xfeatures2d::SURFTrait, as_raw_mut_SURF }

impl SURF {
	/// ## Parameters
	/// * hessianThreshold: Threshold for hessian keypoint detector used in SURF.
	/// * nOctaves: Number of pyramid octaves the keypoint detector will use.
	/// * nOctaveLayers: Number of octave layers within each octave.
	/// * extended: Extended descriptor flag (true - use extended 128-element descriptors; false - use
	/// 64-element descriptors).
	/// * upright: Up-right or rotated features flag (true - do not compute orientation of features;
	/// false - compute orientation).
	///
	/// ## C++ default parameters
	/// * hessian_threshold: 100
	/// * n_octaves: 4
	/// * n_octave_layers: 3
	/// * extended: false
	/// * upright: false
	// create(double, int, int, bool, bool)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:97
	// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, ["hessianThreshold", "nOctaves", "nOctaveLayers", "extended", "upright"], ["double", "int", "int", "bool", "bool"]), _)]),
	#[inline]
	pub fn create(hessian_threshold: f64, n_octaves: i32, n_octave_layers: i32, extended: bool, upright: bool) -> Result<core::Ptr<crate::xfeatures2d::SURF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_create_double_int_int_bool_bool(hessian_threshold, n_octaves, n_octave_layers, extended, upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::SURF>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * hessianThreshold: Threshold for hessian keypoint detector used in SURF.
	/// * nOctaves: Number of pyramid octaves the keypoint detector will use.
	/// * nOctaveLayers: Number of octave layers within each octave.
	/// * extended: Extended descriptor flag (true - use extended 128-element descriptors; false - use
	/// 64-element descriptors).
	/// * upright: Up-right or rotated features flag (true - do not compute orientation of features;
	/// false - compute orientation).
	///
	/// ## Note
	/// This alternative version of [SURF::create] function uses the following default values for its arguments:
	/// * hessian_threshold: 100
	/// * n_octaves: 4
	/// * n_octave_layers: 3
	/// * extended: false
	/// * upright: false
	// cv::xfeatures2d::SURF::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:97
	// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::SURF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::SURF>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SURF, core::Algorithm, cv_xfeatures2d_SURF_to_Algorithm }

boxed_cast_base! { SURF, crate::features::Feature2D, cv_xfeatures2d_SURF_to_Feature2D }

impl std::fmt::Debug for SURF {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SURF")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::StarDetector]
// StarDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:127
pub trait StarDetectorTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_StarDetector(&self) -> *const c_void;

	// getMaxSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:137
	// ("cv::xfeatures2d::StarDetector::getMaxSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_getMaxSize_const(self.as_raw_StarDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getResponseThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:140
	// ("cv::xfeatures2d::StarDetector::getResponseThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_response_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_getResponseThreshold_const(self.as_raw_StarDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getLineThresholdProjected()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:143
	// ("cv::xfeatures2d::StarDetector::getLineThresholdProjected", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_line_threshold_projected(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_getLineThresholdProjected_const(self.as_raw_StarDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getLineThresholdBinarized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:146
	// ("cv::xfeatures2d::StarDetector::getLineThresholdBinarized", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_line_threshold_binarized(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_getLineThresholdBinarized_const(self.as_raw_StarDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSuppressNonmaxSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:149
	// ("cv::xfeatures2d::StarDetector::getSuppressNonmaxSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_suppress_nonmax_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_getSuppressNonmaxSize_const(self.as_raw_StarDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:151
	// ("cv::xfeatures2d::StarDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_getDefaultName_const(self.as_raw_StarDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::StarDetector]
pub trait StarDetectorTrait: crate::features::Feature2DTrait + crate::xfeatures2d::StarDetectorTraitConst {
	fn as_raw_mut_StarDetector(&mut self) -> *mut c_void;

	// setMaxSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:136
	// ("cv::xfeatures2d::StarDetector::setMaxSize", vec![(pred!(mut, ["_maxSize"], ["int"]), _)]),
	#[inline]
	fn set_max_size(&mut self, _max_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_setMaxSize_int(self.as_raw_mut_StarDetector(), _max_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setResponseThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:139
	// ("cv::xfeatures2d::StarDetector::setResponseThreshold", vec![(pred!(mut, ["_responseThreshold"], ["int"]), _)]),
	#[inline]
	fn set_response_threshold(&mut self, _response_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_setResponseThreshold_int(self.as_raw_mut_StarDetector(), _response_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setLineThresholdProjected(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:142
	// ("cv::xfeatures2d::StarDetector::setLineThresholdProjected", vec![(pred!(mut, ["_lineThresholdProjected"], ["int"]), _)]),
	#[inline]
	fn set_line_threshold_projected(&mut self, _line_threshold_projected: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_setLineThresholdProjected_int(self.as_raw_mut_StarDetector(), _line_threshold_projected, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setLineThresholdBinarized(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:145
	// ("cv::xfeatures2d::StarDetector::setLineThresholdBinarized", vec![(pred!(mut, ["_lineThresholdBinarized"], ["int"]), _)]),
	#[inline]
	fn set_line_threshold_binarized(&mut self, _line_threshold_binarized: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_setLineThresholdBinarized_int(self.as_raw_mut_StarDetector(), _line_threshold_binarized, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSuppressNonmaxSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:148
	// ("cv::xfeatures2d::StarDetector::setSuppressNonmaxSize", vec![(pred!(mut, ["_suppressNonmaxSize"], ["int"]), _)]),
	#[inline]
	fn set_suppress_nonmax_size(&mut self, _suppress_nonmax_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_setSuppressNonmaxSize_int(self.as_raw_mut_StarDetector(), _suppress_nonmax_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// The class implements the keypoint detector introduced by [Agrawal08](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Agrawal08), synonym of StarDetector. :
// StarDetector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:127
pub struct StarDetector {
	ptr: *mut c_void,
}

opencv_type_boxed! { StarDetector }

impl Drop for StarDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_StarDetector_delete(self.as_raw_mut_StarDetector()) };
	}
}

unsafe impl Send for StarDetector {}

impl core::AlgorithmTraitConst for StarDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StarDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StarDetector, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for StarDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for StarDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StarDetector, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::StarDetectorTraitConst for StarDetector {
	#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::StarDetectorTrait for StarDetector {
	#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StarDetector, crate::xfeatures2d::StarDetectorTraitConst, as_raw_StarDetector, crate::xfeatures2d::StarDetectorTrait, as_raw_mut_StarDetector }

impl StarDetector {
	/// the full constructor
	///
	/// ## C++ default parameters
	/// * max_size: 45
	/// * response_threshold: 30
	/// * line_threshold_projected: 10
	/// * line_threshold_binarized: 8
	/// * suppress_nonmax_size: 5
	// create(int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:131
	// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, ["maxSize", "responseThreshold", "lineThresholdProjected", "lineThresholdBinarized", "suppressNonmaxSize"], ["int", "int", "int", "int", "int"]), _)]),
	#[inline]
	pub fn create(max_size: i32, response_threshold: i32, line_threshold_projected: i32, line_threshold_binarized: i32, suppress_nonmax_size: i32) -> Result<core::Ptr<crate::xfeatures2d::StarDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_create_int_int_int_int_int(max_size, response_threshold, line_threshold_projected, line_threshold_binarized, suppress_nonmax_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::StarDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// the full constructor
	///
	/// ## Note
	/// This alternative version of [StarDetector::create] function uses the following default values for its arguments:
	/// * max_size: 45
	/// * response_threshold: 30
	/// * line_threshold_projected: 10
	/// * line_threshold_binarized: 8
	/// * suppress_nonmax_size: 5
	// cv::xfeatures2d::StarDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:131
	// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::StarDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::StarDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { StarDetector, core::Algorithm, cv_xfeatures2d_StarDetector_to_Algorithm }

boxed_cast_base! { StarDetector, crate::features::Feature2D, cv_xfeatures2d_StarDetector_to_Feature2D }

impl std::fmt::Debug for StarDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StarDetector")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::TBMR]
// TBMR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1200
pub trait TBMRTraitConst: crate::xfeatures2d::AffineFeature2DTraitConst {
	fn as_raw_TBMR(&self) -> *const c_void;

	// getMinArea()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1209
	// ("cv::xfeatures2d::TBMR::getMinArea", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getMinArea_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxAreaRelative()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1211
	// ("cv::xfeatures2d::TBMR::getMaxAreaRelative", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_area_relative(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getMaxAreaRelative_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1213
	// ("cv::xfeatures2d::TBMR::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getScaleFactor_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNScales()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1215
	// ("cv::xfeatures2d::TBMR::getNScales", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_n_scales(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getNScales_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1217
	// ("cv::xfeatures2d::TBMR::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getDefaultName_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::TBMR]
pub trait TBMRTrait: crate::xfeatures2d::AffineFeature2DTrait + crate::xfeatures2d::TBMRTraitConst {
	fn as_raw_mut_TBMR(&mut self) -> *mut c_void;

	// setMinArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1208
	// ("cv::xfeatures2d::TBMR::setMinArea", vec![(pred!(mut, ["minArea"], ["int"]), _)]),
	#[inline]
	fn set_min_area(&mut self, min_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setMinArea_int(self.as_raw_mut_TBMR(), min_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxAreaRelative(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1210
	// ("cv::xfeatures2d::TBMR::setMaxAreaRelative", vec![(pred!(mut, ["maxArea"], ["float"]), _)]),
	#[inline]
	fn set_max_area_relative(&mut self, max_area: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setMaxAreaRelative_float(self.as_raw_mut_TBMR(), max_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1212
	// ("cv::xfeatures2d::TBMR::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setScaleFactor_float(self.as_raw_mut_TBMR(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNScales(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1214
	// ("cv::xfeatures2d::TBMR::setNScales", vec![(pred!(mut, ["n_scales"], ["int"]), _)]),
	#[inline]
	fn set_n_scales(&mut self, n_scales: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setNScales_int(self.as_raw_mut_TBMR(), n_scales, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing the Tree Based Morse Regions (TBMR) as described in
/// [Najman2014](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Najman2014) extended with scaled extraction ability.
///
/// ## Parameters
/// * min_area: prune areas smaller than minArea
/// * max_area_relative: prune areas bigger than maxArea = max_area_relative *
/// input_image_size
/// * scale_factor: scale factor for scaled extraction.
/// * n_scales: number of applications of the scale factor (octaves).
///
///
/// Note: This algorithm is based on Component Tree (Min/Max) as well as MSER but
/// uses a Morse-theory approach to extract features.
///
/// Features are ellipses (similar to MSER, however a MSER feature can never be a
/// TBMR feature and vice versa).
// TBMR /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1200
pub struct TBMR {
	ptr: *mut c_void,
}

opencv_type_boxed! { TBMR }

impl Drop for TBMR {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_TBMR_delete(self.as_raw_mut_TBMR()) };
	}
}

unsafe impl Send for TBMR {}

impl crate::xfeatures2d::AffineFeature2DTraitConst for TBMR {
	#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::AffineFeature2DTrait for TBMR {
	#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TBMR, crate::xfeatures2d::AffineFeature2DTraitConst, as_raw_AffineFeature2D, crate::xfeatures2d::AffineFeature2DTrait, as_raw_mut_AffineFeature2D }

impl core::AlgorithmTraitConst for TBMR {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TBMR {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TBMR, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for TBMR {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for TBMR {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TBMR, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::TBMRTraitConst for TBMR {
	#[inline] fn as_raw_TBMR(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::TBMRTrait for TBMR {
	#[inline] fn as_raw_mut_TBMR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TBMR, crate::xfeatures2d::TBMRTraitConst, as_raw_TBMR, crate::xfeatures2d::TBMRTrait, as_raw_mut_TBMR }

impl TBMR {
	/// ## C++ default parameters
	/// * min_area: 60
	/// * max_area_relative: 0.01f
	/// * scale_factor: 1.25f
	/// * n_scales: -1
	// create(int, float, float, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1203
	// ("cv::xfeatures2d::TBMR::create", vec![(pred!(mut, ["min_area", "max_area_relative", "scale_factor", "n_scales"], ["int", "float", "float", "int"]), _)]),
	#[inline]
	pub fn create(min_area: i32, max_area_relative: f32, scale_factor: f32, n_scales: i32) -> Result<core::Ptr<crate::xfeatures2d::TBMR>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_create_int_float_float_int(min_area, max_area_relative, scale_factor, n_scales, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::TBMR>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [TBMR::create] function uses the following default values for its arguments:
	/// * min_area: 60
	/// * max_area_relative: 0.01f
	/// * scale_factor: 1.25f
	/// * n_scales: -1
	// cv::xfeatures2d::TBMR::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1203
	// ("cv::xfeatures2d::TBMR::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::TBMR>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::TBMR>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TBMR, crate::xfeatures2d::AffineFeature2D, cv_xfeatures2d_TBMR_to_AffineFeature2D }

boxed_cast_base! { TBMR, core::Algorithm, cv_xfeatures2d_TBMR_to_Algorithm }

boxed_cast_base! { TBMR, crate::features::Feature2D, cv_xfeatures2d_TBMR_to_Feature2D }

impl std::fmt::Debug for TBMR {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TBMR")
			.finish()
	}
}

/// Constant methods for [crate::xfeatures2d::TEBLID]
// TEBLID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:316
pub trait TEBLIDTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_TEBLID(&self) -> *const c_void;

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:338
	// ("cv::xfeatures2d::TEBLID::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TEBLID_getDefaultName_const(self.as_raw_TEBLID(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::TEBLID]
pub trait TEBLIDTrait: crate::features::Feature2DTrait + crate::xfeatures2d::TEBLIDTraitConst {
	fn as_raw_mut_TEBLID(&mut self) -> *mut c_void;

}

/// Class implementing TEBLID (Triplet-based Efficient Binary Local Image Descriptor),
///  described in [Suarez2021TEBLID](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Suarez2021TEBLID).
///
/// TEBLID stands for Triplet-based Efficient Binary Local Image Descriptor, although originally it was called BAD
/// \cite Suarez2021TEBLID. It is an improvement over BEBLID \cite Suarez2020BEBLID, that uses triplet loss,
/// hard negative mining, and anchor swap to improve the image matching results.
/// It is able to describe keypoints from any detector just by changing the scale_factor parameter.
/// TEBLID is as efficient as ORB, BEBLID or BRISK, but the triplet-based training objective selected more
/// discriminative features that explain the accuracy gain. It is also more compact than BEBLID,
/// when running the [AKAZE example](https://github.com/opencv/opencv/blob/5.x/samples/cpp/tutorial_code/features2D/AKAZE_match.cpp)
/// with 10000 keypoints detected by ORB, BEBLID obtains 561 inliers (75%) with 512 bits, whereas
/// TEBLID obtains 621 (75.2%) with 256 bits. ORB obtains only 493 inliers (63%).
///
/// If you find this code useful, please add a reference to the following paper:
/// <BLOCKQUOTE> Iago Suárez, José M. Buenaposada, and Luis Baumela.
/// Revisiting Binary Local Image Description for Resource Limited Devices.
/// IEEE Robotics and Automation Letters, vol. 6, no. 4, pp. 8317-8324, Oct. 2021. </BLOCKQUOTE>
///
/// The descriptor was trained in Liberty split of the UBC datasets \cite winder2007learning .
// TEBLID /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:316
pub struct TEBLID {
	ptr: *mut c_void,
}

opencv_type_boxed! { TEBLID }

impl Drop for TEBLID {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_TEBLID_delete(self.as_raw_mut_TEBLID()) };
	}
}

unsafe impl Send for TEBLID {}

impl core::AlgorithmTraitConst for TEBLID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TEBLID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TEBLID, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for TEBLID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for TEBLID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TEBLID, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::TEBLIDTraitConst for TEBLID {
	#[inline] fn as_raw_TEBLID(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::TEBLIDTrait for TEBLID {
	#[inline] fn as_raw_mut_TEBLID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TEBLID, crate::xfeatures2d::TEBLIDTraitConst, as_raw_TEBLID, crate::xfeatures2d::TEBLIDTrait, as_raw_mut_TEBLID }

impl TEBLID {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_xfeatures2d_TEBLID_defaultNew_const()) }
	}

	/// Creates the TEBLID descriptor.
	/// ## Parameters
	/// * scale_factor: Adjust the sampling window around detected keypoints:
	/// - <b> 1.00f </b> should be the scale for ORB keypoints
	/// - <b> 6.75f </b> should be the scale for SIFT detected keypoints
	/// - <b> 6.25f </b> is default and fits for KAZE, SURF detected keypoints
	/// - <b> 5.00f </b> should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints
	/// * n_bits: Determine the number of bits in the descriptor. Should be either
	///  TEBLID::SIZE_256_BITS or TEBLID::SIZE_512_BITS.
	///
	/// ## C++ default parameters
	/// * n_bits: TEBLID::SIZE_256_BITS
	// create(float, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:336
	// ("cv::xfeatures2d::TEBLID::create", vec![(pred!(mut, ["scale_factor", "n_bits"], ["float", "int"]), _)]),
	#[inline]
	pub fn create(scale_factor: f32, n_bits: i32) -> Result<core::Ptr<crate::xfeatures2d::TEBLID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TEBLID_create_float_int(scale_factor, n_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::TEBLID>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates the TEBLID descriptor.
	/// ## Parameters
	/// * scale_factor: Adjust the sampling window around detected keypoints:
	/// - <b> 1.00f </b> should be the scale for ORB keypoints
	/// - <b> 6.75f </b> should be the scale for SIFT detected keypoints
	/// - <b> 6.25f </b> is default and fits for KAZE, SURF detected keypoints
	/// - <b> 5.00f </b> should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints
	/// * n_bits: Determine the number of bits in the descriptor. Should be either
	///  TEBLID::SIZE_256_BITS or TEBLID::SIZE_512_BITS.
	///
	/// ## Note
	/// This alternative version of [TEBLID::create] function uses the following default values for its arguments:
	/// * n_bits: TEBLID::SIZE_256_BITS
	// cv::xfeatures2d::TEBLID::create(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:336
	// ("cv::xfeatures2d::TEBLID::create", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	#[inline]
	pub fn create_def(scale_factor: f32) -> Result<core::Ptr<crate::xfeatures2d::TEBLID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TEBLID_create_float(scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::TEBLID>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TEBLID, core::Algorithm, cv_xfeatures2d_TEBLID_to_Algorithm }

boxed_cast_base! { TEBLID, crate::features::Feature2D, cv_xfeatures2d_TEBLID_to_Feature2D }

impl std::fmt::Debug for TEBLID {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TEBLID")
			.finish()
	}
}

impl Default for TEBLID {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::xfeatures2d::VGG]
// VGG /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:522
pub trait VGGTraitConst: crate::features::Feature2DTraitConst {
	fn as_raw_VGG(&self) -> *const c_void;

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:535
	// ("cv::xfeatures2d::VGG::getDefaultName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_default_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getDefaultName_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:538
	// ("cv::xfeatures2d::VGG::getSigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sigma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getSigma_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseNormalizeImage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:541
	// ("cv::xfeatures2d::VGG::getUseNormalizeImage", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_normalize_image(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeImage_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:544
	// ("cv::xfeatures2d::VGG::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_scale_orientation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getUseScaleOrientation_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:547
	// ("cv::xfeatures2d::VGG::getScaleFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getScaleFactor_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getUseNormalizeDescriptor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:550
	// ("cv::xfeatures2d::VGG::getUseNormalizeDescriptor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_use_normalize_descriptor(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::xfeatures2d::VGG]
pub trait VGGTrait: crate::features::Feature2DTrait + crate::xfeatures2d::VGGTraitConst {
	fn as_raw_mut_VGG(&mut self) -> *mut c_void;

	// setSigma(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:537
	// ("cv::xfeatures2d::VGG::setSigma", vec![(pred!(mut, ["isigma"], ["const float"]), _)]),
	#[inline]
	fn set_sigma(&mut self, isigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setSigma_const_float(self.as_raw_mut_VGG(), isigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseNormalizeImage(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:540
	// ("cv::xfeatures2d::VGG::setUseNormalizeImage", vec![(pred!(mut, ["img_normalize"], ["const bool"]), _)]),
	#[inline]
	fn set_use_normalize_image(&mut self, img_normalize: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(self.as_raw_mut_VGG(), img_normalize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:543
	// ("cv::xfeatures2d::VGG::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
	#[inline]
	fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(self.as_raw_mut_VGG(), use_scale_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:546
	// ("cv::xfeatures2d::VGG::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setScaleFactor_const_float(self.as_raw_mut_VGG(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setUseNormalizeDescriptor(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:549
	// ("cv::xfeatures2d::VGG::setUseNormalizeDescriptor", vec![(pred!(mut, ["dsc_normalize"], ["const bool"]), _)]),
	#[inline]
	fn set_use_normalize_descriptor(&mut self, dsc_normalize: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(self.as_raw_mut_VGG(), dsc_normalize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class implementing VGG (Oxford Visual Geometry Group) descriptor trained end to end
/// using "Descriptor Learning Using Convex Optimisation" (DLCO) aparatus described in [Simonyan14](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_Simonyan14).
///
/// ## Parameters
/// * desc: type of descriptor to use, VGG::VGG_120 is default (120 dimensions float)
/// Available types are VGG::VGG_120, VGG::VGG_80, VGG::VGG_64, VGG::VGG_48
/// * isigma: gaussian kernel value for image blur (default is 1.4f)
/// * img_normalize: use image sample intensity normalization (enabled by default)
/// * use_orientation: sample patterns using keypoints orientation, enabled by default
/// * scale_factor: adjust the sampling window of detected keypoints to 64.0f (VGG sampling window)
/// 6.25f is default and fits for KAZE, SURF detected keypoints window ratio
/// 6.75f should be the scale for SIFT detected keypoints window ratio
/// 5.00f should be the scale for AKAZE, MSD, AGAST, FAST, BRISK keypoints window ratio
/// 0.75f should be the scale for ORB keypoints ratio
///
/// * dsc_normalize: clamp descriptors to 255 and convert to uchar CV_8UC1 (disabled by default)
// VGG /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:522
pub struct VGG {
	ptr: *mut c_void,
}

opencv_type_boxed! { VGG }

impl Drop for VGG {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xfeatures2d_VGG_delete(self.as_raw_mut_VGG()) };
	}
}

unsafe impl Send for VGG {}

impl core::AlgorithmTraitConst for VGG {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for VGG {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VGG, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::features::Feature2DTraitConst for VGG {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features::Feature2DTrait for VGG {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VGG, crate::features::Feature2DTraitConst, as_raw_Feature2D, crate::features::Feature2DTrait, as_raw_mut_Feature2D }

impl crate::xfeatures2d::VGGTraitConst for VGG {
	#[inline] fn as_raw_VGG(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::VGGTrait for VGG {
	#[inline] fn as_raw_mut_VGG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VGG, crate::xfeatures2d::VGGTraitConst, as_raw_VGG, crate::xfeatures2d::VGGTrait, as_raw_mut_VGG }

impl VGG {
	/// ## C++ default parameters
	/// * desc: VGG::VGG_120
	/// * isigma: 1.4f
	/// * img_normalize: true
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	/// * dsc_normalize: false
	// create(int, float, bool, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:531
	// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, ["desc", "isigma", "img_normalize", "use_scale_orientation", "scale_factor", "dsc_normalize"], ["int", "float", "bool", "bool", "float", "bool"]), _)]),
	#[inline]
	pub fn create(desc: i32, isigma: f32, img_normalize: bool, use_scale_orientation: bool, scale_factor: f32, dsc_normalize: bool) -> Result<core::Ptr<crate::xfeatures2d::VGG>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::VGG>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [VGG::create] function uses the following default values for its arguments:
	/// * desc: VGG::VGG_120
	/// * isigma: 1.4f
	/// * img_normalize: true
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	/// * dsc_normalize: false
	// cv::xfeatures2d::VGG::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:531
	// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::xfeatures2d::VGG>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::VGG>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { VGG, core::Algorithm, cv_xfeatures2d_VGG_to_Algorithm }

boxed_cast_base! { VGG, crate::features::Feature2D, cv_xfeatures2d_VGG_to_Feature2D }

impl std::fmt::Debug for VGG {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("VGG")
			.finish()
	}
}
