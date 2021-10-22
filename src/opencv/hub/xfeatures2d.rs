#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Extra 2D Features Framework
//!    # Experimental 2D Features Algorithms
//! 
//! This section describes experimental algorithms for 2d feature detection.
//! 
//!    # Non-free 2D Features Algorithms
//! 
//! This section describes two popular algorithms for 2d feature detection, SIFT and SURF, that are
//! known to be patented. You need to set the OPENCV_ENABLE_NONFREE option in cmake to use those. Use them at your own risk.
//! 
//!    # Experimental 2D Features Matching Algorithm
//! 
//! This section describes the following matching strategies:
//!    - GMS: Grid-based Motion Statistics, [Bian2017gms](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Bian2017gms)
//!    - LOGOS: Local geometric support for high-outlier spatial verification, [Lowry2018LOGOSLG](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Lowry2018LOGOSLG)
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::SURFConst, super::SURF, super::FREAKTraitConst, super::FREAKTrait, super::StarDetectorTraitConst, super::StarDetectorTrait, super::BriefDescriptorExtractorTraitConst, super::BriefDescriptorExtractorTrait, super::LUCIDTraitConst, super::LUCIDTrait, super::LATCHTraitConst, super::LATCHTrait, super::BEBLIDTraitConst, super::BEBLIDTrait, super::DAISYConst, super::DAISY, super::MSDDetectorTraitConst, super::MSDDetectorTrait, super::VGGConst, super::VGG, super::BoostDescConst, super::BoostDesc, super::PCTSignaturesConst, super::PCTSignatures, super::PCTSignaturesSQFDConst, super::PCTSignaturesSQFD, super::Elliptic_KeyPointTraitConst, super::Elliptic_KeyPointTrait, super::HarrisLaplaceFeatureDetectorTraitConst, super::HarrisLaplaceFeatureDetectorTrait, super::AffineFeature2DConst, super::AffineFeature2D, super::TBMRConst, super::TBMR, super::SURF_CUDATraitConst, super::SURF_CUDATrait };
}

pub const BoostDesc_BGM: i32 = 100;
pub const BoostDesc_BGM_BILINEAR: i32 = 102;
pub const BoostDesc_BGM_HARD: i32 = 101;
pub const BoostDesc_BINBOOST_128: i32 = 301;
pub const BoostDesc_BINBOOST_256: i32 = 302;
pub const BoostDesc_BINBOOST_64: i32 = 300;
pub const BoostDesc_LBGM: i32 = 200;
pub const VGG_VGG_120: i32 = 100;
pub const VGG_VGG_48: i32 = 103;
pub const VGG_VGG_64: i32 = 102;
pub const VGG_VGG_80: i32 = 101;
///  Descriptor number of bits, each bit is a boosting weak-learner.
/// The user can choose between 512 or 256 bits.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BEBLID_BeblidSize {
	SIZE_512_BITS = 100,
	SIZE_256_BITS = 101,
}

opencv_type_enum! { crate::xfeatures2d::BEBLID_BeblidSize }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DAISY_NormalizationType {
	NRM_NONE = 100,
	NRM_PARTIAL = 101,
	NRM_FULL = 102,
	NRM_SIFT = 103,
}

opencv_type_enum! { crate::xfeatures2d::DAISY_NormalizationType }

/// Lp distance function selector.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCTSignatures_DistanceFunction {
	L0_25 = 0,
	L0_5 = 1,
	L1 = 2,
	L2 = 3,
	L2SQUARED = 4,
	L5 = 5,
	L_INFINITY = 6,
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_DistanceFunction }

/// Point distributions supported by random point generator.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCTSignatures_PointDistribution {
	/// Generate numbers uniformly.
	UNIFORM = 0,
	/// Generate points in a regular grid.
	REGULAR = 1,
	/// Generate points with normal (gaussian) distribution.
	NORMAL = 2,
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_PointDistribution }

/// Similarity function selector.
/// ## See also
/// Christian Beecks, Merih Seran Uysal, Thomas Seidl.
///       Signature quadratic form distance.
///       In Proceedings of the ACM International Conference on Image and Video Retrieval, pages 438-445.
///       ACM, 2010.
/// [BeecksUS10](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_BeecksUS10)
/// 
/// Note: For selected distance function: ![block formula](https://latex.codecogs.com/png.latex?%20d%28c%5Fi%2C%20c%5Fj%29%20)  and parameter: ![block formula](https://latex.codecogs.com/png.latex?%20%5Calpha%20)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCTSignatures_SimilarityFunction {
	/// ![block formula](https://latex.codecogs.com/png.latex?%20%2Dd%28c%5Fi%2C%20c%5Fj%29%20)
	MINUS = 0,
	/// ![block formula](https://latex.codecogs.com/png.latex?%20e%5E%7B%20%2D%5Calpha%20%2A%20d%5E2%28c%5Fi%2C%20c%5Fj%29%7D%20)
	GAUSSIAN = 1,
	/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B1%7D%7B%5Calpha%20%2B%20d%28c%5Fi%2C%20c%5Fj%29%7D%20)
	HEURISTIC = 2,
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_SimilarityFunction }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
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

opencv_type_enum! { crate::xfeatures2d::SURF_CUDA_KeypointLayout }

pub type SurfDescriptorExtractor = dyn crate::xfeatures2d::SURF;
pub type SurfFeatureDetector = dyn crate::xfeatures2d::SURF;
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
/// Detects corners using the FAST algorithm by [Rosten06](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Rosten06) .
/// 
/// ## C++ default parameters
/// * nonmax_suppression: true
/// * typ: FastFeatureDetector::TYPE_9_16
#[inline]
pub fn fast_for_point_set(image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	unsafe { sys::cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ) }.into_result()
}

/// GMS (Grid-based Motion Statistics) feature matching strategy described in [Bian2017gms](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Bian2017gms) .
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
#[inline]
pub fn match_gms(size1: core::Size, size2: core::Size, keypoints1: &core::Vector<core::KeyPoint>, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, matches_gms: &mut core::Vector<core::DMatch>, with_rotation: bool, with_scale: bool, threshold_factor: f64) -> Result<()> {
	unsafe { sys::cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vector_KeyPoint_R_const_vector_KeyPoint_R_const_vector_DMatch_R_vector_DMatch_R_const_bool_const_bool_const_double(&size1, &size2, keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), matches_gms.as_raw_mut_VectorOfDMatch(), with_rotation, with_scale, threshold_factor) }.into_result()
}

/// LOGOS (Local geometric support for high-outlier spatial verification) feature matching strategy described in [Lowry2018LOGOSLG](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Lowry2018LOGOSLG) .
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
#[inline]
pub fn match_logos(keypoints1: &core::Vector<core::KeyPoint>, keypoints2: &core::Vector<core::KeyPoint>, nn1: &core::Vector<i32>, nn2: &core::Vector<i32>, matches1to2: &mut core::Vector<core::DMatch>) -> Result<()> {
	unsafe { sys::cv_xfeatures2d_matchLOGOS_const_vector_KeyPoint_R_const_vector_KeyPoint_R_const_vector_int_R_const_vector_int_R_vector_DMatch_R(keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), nn1.as_raw_VectorOfi32(), nn2.as_raw_VectorOfi32(), matches1to2.as_raw_mut_VectorOfDMatch()) }.into_result()
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
pub trait SURF_CUDATraitConst {
	fn as_raw_SURF_CUDA(&self) -> *const c_void;

	#[inline]
	fn hessian_threshold(&self) -> f64 {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropHessianThreshold_const(self.as_raw_SURF_CUDA()) }.into_result().expect("Infallible function failed: hessian_threshold")
	}
	
	#[inline]
	fn n_octaves(&self) -> i32 {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropNOctaves_const(self.as_raw_SURF_CUDA()) }.into_result().expect("Infallible function failed: n_octaves")
	}
	
	#[inline]
	fn n_octave_layers(&self) -> i32 {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropNOctaveLayers_const(self.as_raw_SURF_CUDA()) }.into_result().expect("Infallible function failed: n_octave_layers")
	}
	
	#[inline]
	fn extended(&self) -> bool {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropExtended_const(self.as_raw_SURF_CUDA()) }.into_result().expect("Infallible function failed: extended")
	}
	
	#[inline]
	fn upright(&self) -> bool {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropUpright_const(self.as_raw_SURF_CUDA()) }.into_result().expect("Infallible function failed: upright")
	}
	
	/// max keypoints = min(keypointsRatio * img.size().area(), 65535)
	#[inline]
	fn keypoints_ratio(&self) -> f32 {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropKeypointsRatio_const(self.as_raw_SURF_CUDA()) }.into_result().expect("Infallible function failed: keypoints_ratio")
	}
	
	#[inline]
	fn sum(&self) -> core::GpuMat {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropSum_const(self.as_raw_SURF_CUDA()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } ).expect("Infallible function failed: sum")
	}
	
	#[inline]
	fn mask1(&self) -> core::GpuMat {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropMask1_const(self.as_raw_SURF_CUDA()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } ).expect("Infallible function failed: mask1")
	}
	
	#[inline]
	fn mask_sum(&self) -> core::GpuMat {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropMaskSum_const(self.as_raw_SURF_CUDA()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } ).expect("Infallible function failed: mask_sum")
	}
	
	#[inline]
	fn det(&self) -> core::GpuMat {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropDet_const(self.as_raw_SURF_CUDA()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } ).expect("Infallible function failed: det")
	}
	
	#[inline]
	fn trace(&self) -> core::GpuMat {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropTrace_const(self.as_raw_SURF_CUDA()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } ).expect("Infallible function failed: trace")
	}
	
	#[inline]
	fn max_pos_buffer(&self) -> core::GpuMat {
		unsafe { sys::cv_cuda_SURF_CUDA_getPropMaxPosBuffer_const(self.as_raw_SURF_CUDA()) }.into_result().map(|r| unsafe { core::GpuMat::opencv_from_extern(r) } ).expect("Infallible function failed: max_pos_buffer")
	}
	
	/// returns the descriptor size in float's (64 or 128)
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_SURF_CUDA_descriptorSize_const(self.as_raw_SURF_CUDA()) }.into_result()
	}
	
	/// returns the default norm type
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_SURF_CUDA_defaultNorm_const(self.as_raw_SURF_CUDA()) }.into_result()
	}
	
}

pub trait SURF_CUDATrait: crate::xfeatures2d::SURF_CUDATraitConst {
	fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void;

	#[inline]
	fn set_hessian_threshold(&mut self, val: f64) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropHessianThreshold_double(self.as_raw_mut_SURF_CUDA(), val) }.into_result().expect("Infallible function failed: set_hessian_threshold")
	}
	
	#[inline]
	fn set_n_octaves(&mut self, val: i32) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropNOctaves_int(self.as_raw_mut_SURF_CUDA(), val) }.into_result().expect("Infallible function failed: set_n_octaves")
	}
	
	#[inline]
	fn set_n_octave_layers(&mut self, val: i32) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropNOctaveLayers_int(self.as_raw_mut_SURF_CUDA(), val) }.into_result().expect("Infallible function failed: set_n_octave_layers")
	}
	
	#[inline]
	fn set_extended(&mut self, val: bool) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropExtended_bool(self.as_raw_mut_SURF_CUDA(), val) }.into_result().expect("Infallible function failed: set_extended")
	}
	
	#[inline]
	fn set_upright(&mut self, val: bool) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropUpright_bool(self.as_raw_mut_SURF_CUDA(), val) }.into_result().expect("Infallible function failed: set_upright")
	}
	
	/// max keypoints = min(keypointsRatio * img.size().area(), 65535)
	#[inline]
	fn set_keypoints_ratio(&mut self, val: f32) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropKeypointsRatio_float(self.as_raw_mut_SURF_CUDA(), val) }.into_result().expect("Infallible function failed: set_keypoints_ratio")
	}
	
	#[inline]
	fn set_sum(&mut self, mut val: core::GpuMat) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropSum_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) }.into_result().expect("Infallible function failed: set_sum")
	}
	
	#[inline]
	fn set_mask1(&mut self, mut val: core::GpuMat) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropMask1_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) }.into_result().expect("Infallible function failed: set_mask1")
	}
	
	#[inline]
	fn set_mask_sum(&mut self, mut val: core::GpuMat) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropMaskSum_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) }.into_result().expect("Infallible function failed: set_mask_sum")
	}
	
	#[inline]
	fn set_det(&mut self, mut val: core::GpuMat) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropDet_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) }.into_result().expect("Infallible function failed: set_det")
	}
	
	#[inline]
	fn set_trace(&mut self, mut val: core::GpuMat) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropTrace_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) }.into_result().expect("Infallible function failed: set_trace")
	}
	
	#[inline]
	fn set_max_pos_buffer(&mut self, mut val: core::GpuMat) {
		unsafe { sys::cv_cuda_SURF_CUDA_setPropMaxPosBuffer_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) }.into_result().expect("Infallible function failed: set_max_pos_buffer")
	}
	
	/// upload host keypoints to device memory
	#[inline]
	fn upload_keypoints(&mut self, keypoints: &core::Vector<core::KeyPoint>, keypoints_gpu: &mut core::GpuMat) -> Result<()> {
		unsafe { sys::cv_cuda_SURF_CUDA_uploadKeypoints_const_vector_KeyPoint_R_GpuMatR(self.as_raw_mut_SURF_CUDA(), keypoints.as_raw_VectorOfKeyPoint(), keypoints_gpu.as_raw_mut_GpuMat()) }.into_result()
	}
	
	/// download keypoints from device to host memory
	#[inline]
	fn download_keypoints(&mut self, keypoints_gpu: &core::GpuMat, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		unsafe { sys::cv_cuda_SURF_CUDA_downloadKeypoints_const_GpuMatR_vector_KeyPoint_R(self.as_raw_mut_SURF_CUDA(), keypoints_gpu.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint()) }.into_result()
	}
	
	/// download descriptors from device to host memory
	#[inline]
	fn download_descriptors(&mut self, descriptors_gpu: &core::GpuMat, descriptors: &mut core::Vector<f32>) -> Result<()> {
		unsafe { sys::cv_cuda_SURF_CUDA_downloadDescriptors_const_GpuMatR_vector_float_R(self.as_raw_mut_SURF_CUDA(), descriptors_gpu.as_raw_GpuMat(), descriptors.as_raw_mut_VectorOff32()) }.into_result()
	}
	
	/// Finds the keypoints using fast hessian detector used in SURF
	/// 
	/// ## Parameters
	/// * img: Source image, currently supports only CV_8UC1 images.
	/// * mask: A mask image same size as src and of type CV_8UC1.
	/// * keypoints: Detected keypoints.
	#[inline]
	fn detect(&mut self, img: &core::GpuMat, mask: &core::GpuMat, keypoints: &mut core::GpuMat) -> Result<()> {
		unsafe { sys::cv_cuda_SURF_CUDA_detect_const_GpuMatR_const_GpuMatR_GpuMatR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat()) }.into_result()
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
	#[inline]
	fn detect_with_descriptors(&mut self, img: &core::GpuMat, mask: &core::GpuMat, keypoints: &mut core::GpuMat, descriptors: &mut core::GpuMat, use_provided_keypoints: bool) -> Result<()> {
		unsafe { sys::cv_cuda_SURF_CUDA_detectWithDescriptors_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), descriptors.as_raw_mut_GpuMat(), use_provided_keypoints) }.into_result()
	}
	
	#[inline]
	fn release_memory(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_SURF_CUDA_releaseMemory(self.as_raw_mut_SURF_CUDA()) }.into_result()
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
pub struct SURF_CUDA {
	ptr: *mut c_void
}

opencv_type_boxed! { SURF_CUDA }

impl Drop for SURF_CUDA {
	fn drop(&mut self) {
		extern "C" { fn cv_SURF_CUDA_delete(instance: *mut c_void); }
		unsafe { cv_SURF_CUDA_delete(self.as_raw_mut_SURF_CUDA()) };
	}
}

unsafe impl Send for SURF_CUDA {}

impl crate::xfeatures2d::SURF_CUDATraitConst for SURF_CUDA {
	#[inline] fn as_raw_SURF_CUDA(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::SURF_CUDATrait for SURF_CUDA {
	#[inline] fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SURF_CUDA {
	/// the default constructor
	#[inline]
	pub fn default() -> Result<crate::xfeatures2d::SURF_CUDA> {
		unsafe { sys::cv_cuda_SURF_CUDA_SURF_CUDA() }.into_result().map(|r| unsafe { crate::xfeatures2d::SURF_CUDA::opencv_from_extern(r) } )
	}
	
	/// the full constructor taking all the necessary parameters
	/// 
	/// ## C++ default parameters
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	#[inline]
	pub fn new(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool) -> Result<crate::xfeatures2d::SURF_CUDA> {
		unsafe { sys::cv_cuda_SURF_CUDA_SURF_CUDA_double_int_int_bool_float_bool(_hessian_threshold, _n_octaves, _n_octave_layers, _extended, _keypoints_ratio, _upright) }.into_result().map(|r| unsafe { crate::xfeatures2d::SURF_CUDA::opencv_from_extern(r) } )
	}
	
	/// ## Parameters
	/// * _hessianThreshold: Threshold for hessian keypoint detector used in SURF.
	/// * _nOctaves: Number of pyramid octaves the keypoint detector will use.
	/// * _nOctaveLayers: Number of octave layers within each octave.
	/// * _extended: Extended descriptor flag (true - use extended 128-element descriptors; false - use
	/// 64-element descriptors).
	/// * _keypointsRatio: 
	/// * _upright: Up-right or rotated features flag (true - do not compute orientation of features;
	/// false - compute orientation).
	/// 
	/// ## C++ default parameters
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	#[inline]
	pub fn create(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool) -> Result<core::Ptr<crate::xfeatures2d::SURF_CUDA>> {
		unsafe { sys::cv_cuda_SURF_CUDA_create_double_int_int_bool_float_bool(_hessian_threshold, _n_octaves, _n_octave_layers, _extended, _keypoints_ratio, _upright) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::SURF_CUDA>::opencv_from_extern(r) } )
	}
	
}

/// Class implementing affine adaptation for key points.
/// 
/// A @ref FeatureDetector and a @ref DescriptorExtractor are wrapped to augment the
/// detected points with their affine invariant elliptic region and to compute
/// the feature descriptors on the regions after warping them into circles.
/// 
/// The interface is equivalent to @ref Feature2D, adding operations for
/// @ref Elliptic_KeyPoint "Elliptic_KeyPoints" instead of @ref KeyPoint "KeyPoints".
pub trait AffineFeature2DConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AffineFeature2D(&self) -> *const c_void;

}

pub trait AffineFeature2D: crate::features2d::Feature2DTrait + crate::xfeatures2d::AffineFeature2DConst {
	fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void;

	/// Detects keypoints in the image using the wrapped detector and
	/// performs affine adaptation to augment them with their elliptic regions.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	fn detect(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__InputArrayR(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// Detects keypoints and computes descriptors for their surrounding
	/// regions, after warping them into circles.
	/// 
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	#[inline]
	fn detect_and_compute(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>, descriptors: &mut dyn core::ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__OutputArrayR_bool(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints) }.into_result()
	}
	
}

impl dyn AffineFeature2D + '_ {
	/// Creates an instance wrapping the given keypoint detector and
	/// descriptor extractor.
	#[inline]
	pub fn create(mut keypoint_detector: core::Ptr<crate::features2d::Feature2D>, mut descriptor_extractor: core::Ptr<crate::features2d::Feature2D>) -> Result<core::Ptr<dyn crate::xfeatures2d::AffineFeature2D>> {
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D__Ptr_Feature2D_(keypoint_detector.as_raw_mut_PtrOfFeature2D(), descriptor_extractor.as_raw_mut_PtrOfFeature2D()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::AffineFeature2D>::opencv_from_extern(r) } )
	}
	
	/// Creates an instance where keypoint detector and descriptor
	/// extractor are identical.
	#[inline]
	pub fn create_1(mut keypoint_detector: core::Ptr<crate::features2d::Feature2D>) -> Result<core::Ptr<dyn crate::xfeatures2d::AffineFeature2D>> {
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D_(keypoint_detector.as_raw_mut_PtrOfFeature2D()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::AffineFeature2D>::opencv_from_extern(r) } )
	}
	
}
/// Class implementing BEBLID (Boosted Efficient Binary Local Image Descriptor),
///  described in [Suarez2020BEBLID](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Suarez2020BEBLID) .
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
/// \cite winder2007learning as described in the paper [Suarez2020BEBLID](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Suarez2020BEBLID).
/// You can check in the [AKAZE example](https://raw.githubusercontent.com/opencv/opencv/master/samples/cpp/tutorial_code/features2D/AKAZE_match.cpp)
/// how well BEBLID works. Detecting 10000 keypoints with ORB and describing with BEBLID obtains
/// 561 inliers (75%) whereas describing with ORB obtains only 493 inliers (63%).
pub trait BEBLIDTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BEBLID(&self) -> *const c_void;

}

pub trait BEBLIDTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::BEBLIDTraitConst {
	fn as_raw_mut_BEBLID(&mut self) -> *mut c_void;

}

/// Class implementing BEBLID (Boosted Efficient Binary Local Image Descriptor),
///  described in [Suarez2020BEBLID](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Suarez2020BEBLID) .
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
/// \cite winder2007learning as described in the paper [Suarez2020BEBLID](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Suarez2020BEBLID).
/// You can check in the [AKAZE example](https://raw.githubusercontent.com/opencv/opencv/master/samples/cpp/tutorial_code/features2D/AKAZE_match.cpp)
/// how well BEBLID works. Detecting 10000 keypoints with ORB and describing with BEBLID obtains
/// 561 inliers (75%) whereas describing with ORB obtains only 493 inliers (63%).
pub struct BEBLID {
	ptr: *mut c_void
}

opencv_type_boxed! { BEBLID }

impl Drop for BEBLID {
	fn drop(&mut self) {
		extern "C" { fn cv_BEBLID_delete(instance: *mut c_void); }
		unsafe { cv_BEBLID_delete(self.as_raw_mut_BEBLID()) };
	}
}

unsafe impl Send for BEBLID {}

impl core::AlgorithmTraitConst for BEBLID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BEBLID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for BEBLID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for BEBLID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::BEBLIDTraitConst for BEBLID {
	#[inline] fn as_raw_BEBLID(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BEBLIDTrait for BEBLID {
	#[inline] fn as_raw_mut_BEBLID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

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
	#[inline]
	pub fn create(scale_factor: f32, n_bits: i32) -> Result<core::Ptr<crate::xfeatures2d::BEBLID>> {
		unsafe { sys::cv_xfeatures2d_BEBLID_create_float_int(scale_factor, n_bits) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::BEBLID>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { BEBLID, core::Algorithm, cv_BEBLID_to_Algorithm }

boxed_cast_base! { BEBLID, crate::features2d::Feature2D, cv_BEBLID_to_Feature2D }

/// Class implementing BoostDesc (Learning Image Descriptors with Boosting), described in
/// [Trzcinski13a](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Trzcinski13a) and [Trzcinski13b](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Trzcinski13b).
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
pub trait BoostDescConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BoostDesc(&self) -> *const c_void;

	#[inline]
	fn get_use_scale_orientation(&self) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(self.as_raw_BoostDesc()) }.into_result()
	}
	
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_BoostDesc_getScaleFactor_const(self.as_raw_BoostDesc()) }.into_result()
	}
	
}

pub trait BoostDesc: crate::features2d::Feature2DTrait + crate::xfeatures2d::BoostDescConst {
	fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void;

	#[inline]
	fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(self.as_raw_mut_BoostDesc(), use_scale_orientation) }.into_result()
	}
	
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(self.as_raw_mut_BoostDesc(), scale_factor) }.into_result()
	}
	
}

impl dyn BoostDesc + '_ {
	/// ## C++ default parameters
	/// * desc: BoostDesc::BINBOOST_256
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	#[inline]
	pub fn create(desc: i32, use_scale_orientation: bool, scale_factor: f32) -> Result<core::Ptr<dyn crate::xfeatures2d::BoostDesc>> {
		unsafe { sys::cv_xfeatures2d_BoostDesc_create_int_bool_float(desc, use_scale_orientation, scale_factor) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::BoostDesc>::opencv_from_extern(r) } )
	}
	
}
/// Class for computing BRIEF descriptors described in [calon2010](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_calon2010) .
/// 
/// ## Parameters
/// * bytes: legth of the descriptor in bytes, valid values are: 16, 32 (default) or 64 .
/// * use_orientation: sample patterns using keypoints orientation, disabled by default.
pub trait BriefDescriptorExtractorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void;

}

pub trait BriefDescriptorExtractorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::BriefDescriptorExtractorTraitConst {
	fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void;

}

/// Class for computing BRIEF descriptors described in [calon2010](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_calon2010) .
/// 
/// ## Parameters
/// * bytes: legth of the descriptor in bytes, valid values are: 16, 32 (default) or 64 .
/// * use_orientation: sample patterns using keypoints orientation, disabled by default.
pub struct BriefDescriptorExtractor {
	ptr: *mut c_void
}

opencv_type_boxed! { BriefDescriptorExtractor }

impl Drop for BriefDescriptorExtractor {
	fn drop(&mut self) {
		extern "C" { fn cv_BriefDescriptorExtractor_delete(instance: *mut c_void); }
		unsafe { cv_BriefDescriptorExtractor_delete(self.as_raw_mut_BriefDescriptorExtractor()) };
	}
}

unsafe impl Send for BriefDescriptorExtractor {}

impl core::AlgorithmTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BriefDescriptorExtractor {
	/// ## C++ default parameters
	/// * bytes: 32
	/// * use_orientation: false
	#[inline]
	pub fn create(bytes: i32, use_orientation: bool) -> Result<core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>> {
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(bytes, use_orientation) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::BriefDescriptorExtractor>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { BriefDescriptorExtractor, core::Algorithm, cv_BriefDescriptorExtractor_to_Algorithm }

boxed_cast_base! { BriefDescriptorExtractor, crate::features2d::Feature2D, cv_BriefDescriptorExtractor_to_Feature2D }

/// Class implementing DAISY descriptor, described in [Tola10](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Tola10)
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
pub trait DAISYConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_DAISY(&self) -> *const c_void;

	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	#[inline]
	fn get_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(self.as_raw_DAISY(), y, x, orientation, descriptor) }.into_result()
	}
	
	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	/// * H: homography matrix for warped grid
	#[inline]
	fn get_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(self.as_raw_DAISY(), y, x, orientation, descriptor, h) }.into_result()
	}
	
	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	#[inline]
	fn get_unnormalized_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(self.as_raw_DAISY(), y, x, orientation, descriptor) }.into_result()
	}
	
	/// ## Parameters
	/// * y: position y on image
	/// * x: position x on image
	/// * orientation: orientation on image (0->360)
	/// * descriptor: supplied array for descriptor storage
	/// * H: homography matrix for warped grid
	#[inline]
	fn get_unnormalized_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(self.as_raw_DAISY(), y, x, orientation, descriptor, h) }.into_result()
	}
	
}

pub trait DAISY: crate::features2d::Feature2DTrait + crate::xfeatures2d::DAISYConst {
	fn as_raw_mut_DAISY(&mut self) -> *mut c_void;

	/// ## Parameters
	/// * image: image to extract descriptors
	/// * keypoints: of interest within image
	/// * descriptors: resulted descriptors array
	#[inline]
	fn compute(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray()) }.into_result()
	}
	
	#[inline]
	fn compute_1(&mut self, images: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(descriptors);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(self.as_raw_mut_DAISY(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray()) }.into_result()
	}
	
	/// ## Parameters
	/// * image: image to extract descriptors
	/// * roi: region of interest within image
	/// * descriptors: resulted descriptors array for roi image pixels
	#[inline]
	fn compute_2(&mut self, image: &dyn core::ToInputArray, roi: core::Rect, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), roi.opencv_as_extern(), descriptors.as_raw__OutputArray()) }.into_result()
	}
	
	/// ## Parameters
	/// * image: image to extract descriptors
	/// * descriptors: resulted descriptors array for all image pixels
	#[inline]
	fn compute_3(&mut self, image: &dyn core::ToInputArray, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), descriptors.as_raw__OutputArray()) }.into_result()
	}
	
}

impl dyn DAISY + '_ {
	/// ## C++ default parameters
	/// * radius: 15
	/// * q_radius: 3
	/// * q_theta: 8
	/// * q_hist: 8
	/// * norm: DAISY::NRM_NONE
	/// * h: noArray()
	/// * interpolation: true
	/// * use_orientation: false
	#[inline]
	pub fn create(radius: f32, q_radius: i32, q_theta: i32, q_hist: i32, norm: crate::xfeatures2d::DAISY_NormalizationType, h: &dyn core::ToInputArray, interpolation: bool, use_orientation: bool) -> Result<core::Ptr<dyn crate::xfeatures2d::DAISY>> {
		input_array_arg!(h);
		unsafe { sys::cv_xfeatures2d_DAISY_create_float_int_int_int_NormalizationType_const__InputArrayR_bool_bool(radius, q_radius, q_theta, q_hist, norm, h.as_raw__InputArray(), interpolation, use_orientation) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::DAISY>::opencv_from_extern(r) } )
	}
	
}
/// Elliptic region around an interest point.
pub trait Elliptic_KeyPointTraitConst {
	fn as_raw_Elliptic_KeyPoint(&self) -> *const c_void;

	/// the lengths of the major and minor ellipse axes
	#[inline]
	fn axes(&self) -> core::Size_<f32> {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_getPropAxes_const(self.as_raw_Elliptic_KeyPoint()) }.into_result().expect("Infallible function failed: axes")
	}
	
	/// the integration scale at which the parameters were estimated
	#[inline]
	fn si(&self) -> f32 {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_getPropSi_const(self.as_raw_Elliptic_KeyPoint()) }.into_result().expect("Infallible function failed: si")
	}
	
	/// the transformation between image space and local patch space
	#[inline]
	fn transf(&self) -> core::Matx23f {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_getPropTransf_const(self.as_raw_Elliptic_KeyPoint()) }.into_result().expect("Infallible function failed: transf")
	}
	
}

pub trait Elliptic_KeyPointTrait: crate::xfeatures2d::Elliptic_KeyPointTraitConst {
	fn as_raw_mut_Elliptic_KeyPoint(&mut self) -> *mut c_void;

	/// the lengths of the major and minor ellipse axes
	#[inline]
	fn set_axes(&mut self, val: core::Size_<f32>) {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_setPropAxes_Size__float_(self.as_raw_mut_Elliptic_KeyPoint(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_axes")
	}
	
	/// the integration scale at which the parameters were estimated
	#[inline]
	fn set_si(&mut self, val: f32) {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_setPropSi_float(self.as_raw_mut_Elliptic_KeyPoint(), val) }.into_result().expect("Infallible function failed: set_si")
	}
	
	/// the transformation between image space and local patch space
	#[inline]
	fn set_transf(&mut self, val: core::Matx23f) {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_setPropTransf_Matx23f(self.as_raw_mut_Elliptic_KeyPoint(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_transf")
	}
	
}

/// Elliptic region around an interest point.
pub struct Elliptic_KeyPoint {
	ptr: *mut c_void
}

opencv_type_boxed! { Elliptic_KeyPoint }

impl Drop for Elliptic_KeyPoint {
	fn drop(&mut self) {
		extern "C" { fn cv_Elliptic_KeyPoint_delete(instance: *mut c_void); }
		unsafe { cv_Elliptic_KeyPoint_delete(self.as_raw_mut_Elliptic_KeyPoint()) };
	}
}

unsafe impl Send for Elliptic_KeyPoint {}

impl crate::xfeatures2d::Elliptic_KeyPointTraitConst for Elliptic_KeyPoint {
	#[inline] fn as_raw_Elliptic_KeyPoint(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::Elliptic_KeyPointTrait for Elliptic_KeyPoint {
	#[inline] fn as_raw_mut_Elliptic_KeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Elliptic_KeyPoint {
	#[inline]
	pub fn default() -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint() }.into_result().map(|r| unsafe { crate::xfeatures2d::Elliptic_KeyPoint::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn new(pt: core::Point2f, angle: f32, axes: core::Size, size: f32, si: f32) -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(pt.opencv_as_extern(), angle, axes.opencv_as_extern(), size, si) }.into_result().map(|r| unsafe { crate::xfeatures2d::Elliptic_KeyPoint::opencv_from_extern(r) } )
	}
	
}

/// Class implementing the FREAK (*Fast Retina Keypoint*) keypoint descriptor, described in [AOV12](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_AOV12) .
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
pub trait FREAKTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_FREAK(&self) -> *const c_void;

}

pub trait FREAKTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::FREAKTraitConst {
	fn as_raw_mut_FREAK(&mut self) -> *mut c_void;

}

/// Class implementing the FREAK (*Fast Retina Keypoint*) keypoint descriptor, described in [AOV12](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_AOV12) .
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
pub struct FREAK {
	ptr: *mut c_void
}

opencv_type_boxed! { FREAK }

impl Drop for FREAK {
	fn drop(&mut self) {
		extern "C" { fn cv_FREAK_delete(instance: *mut c_void); }
		unsafe { cv_FREAK_delete(self.as_raw_mut_FREAK()) };
	}
}

unsafe impl Send for FREAK {}

impl core::AlgorithmTraitConst for FREAK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FREAK {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for FREAK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for FREAK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::FREAKTraitConst for FREAK {
	#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::FREAKTrait for FREAK {
	#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FREAK {
	pub const NB_SCALES: i32 = 64;
	pub const NB_PAIRS: i32 = 512;
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
	#[inline]
	pub fn create(orientation_normalized: bool, scale_normalized: bool, pattern_scale: f32, n_octaves: i32, selected_pairs: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::FREAK>> {
		unsafe { sys::cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vector_int_R(orientation_normalized, scale_normalized, pattern_scale, n_octaves, selected_pairs.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::FREAK>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { FREAK, core::Algorithm, cv_FREAK_to_Algorithm }

boxed_cast_base! { FREAK, crate::features2d::Feature2D, cv_FREAK_to_Feature2D }

/// Class implementing the Harris-Laplace feature detector as described in [Mikolajczyk2004](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Mikolajczyk2004).
pub trait HarrisLaplaceFeatureDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void;

}

pub trait HarrisLaplaceFeatureDetectorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst {
	fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void;

}

/// Class implementing the Harris-Laplace feature detector as described in [Mikolajczyk2004](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Mikolajczyk2004).
pub struct HarrisLaplaceFeatureDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { HarrisLaplaceFeatureDetector }

impl Drop for HarrisLaplaceFeatureDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_HarrisLaplaceFeatureDetector_delete(instance: *mut c_void); }
		unsafe { cv_HarrisLaplaceFeatureDetector_delete(self.as_raw_mut_HarrisLaplaceFeatureDetector()) };
	}
}

unsafe impl Send for HarrisLaplaceFeatureDetector {}

impl core::AlgorithmTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

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
	#[inline]
	pub fn create(num_octaves: i32, corn_thresh: f32, dog_thresh: f32, max_corners: i32, num_layers: i32) -> Result<core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>> {
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(num_octaves, corn_thresh, dog_thresh, max_corners, num_layers) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::HarrisLaplaceFeatureDetector>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { HarrisLaplaceFeatureDetector, core::Algorithm, cv_HarrisLaplaceFeatureDetector_to_Algorithm }

boxed_cast_base! { HarrisLaplaceFeatureDetector, crate::features2d::Feature2D, cv_HarrisLaplaceFeatureDetector_to_Feature2D }

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
pub trait LATCHTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_LATCH(&self) -> *const c_void;

}

pub trait LATCHTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::LATCHTraitConst {
	fn as_raw_mut_LATCH(&mut self) -> *mut c_void;

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
pub struct LATCH {
	ptr: *mut c_void
}

opencv_type_boxed! { LATCH }

impl Drop for LATCH {
	fn drop(&mut self) {
		extern "C" { fn cv_LATCH_delete(instance: *mut c_void); }
		unsafe { cv_LATCH_delete(self.as_raw_mut_LATCH()) };
	}
}

unsafe impl Send for LATCH {}

impl core::AlgorithmTraitConst for LATCH {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LATCH {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for LATCH {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for LATCH {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::LATCHTraitConst for LATCH {
	#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::LATCHTrait for LATCH {
	#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LATCH {
	/// ## C++ default parameters
	/// * bytes: 32
	/// * rotation_invariance: true
	/// * half_ssd_size: 3
	/// * sigma: 2.0
	#[inline]
	pub fn create(bytes: i32, rotation_invariance: bool, half_ssd_size: i32, sigma: f64) -> Result<core::Ptr<crate::xfeatures2d::LATCH>> {
		unsafe { sys::cv_xfeatures2d_LATCH_create_int_bool_int_double(bytes, rotation_invariance, half_ssd_size, sigma) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::LATCH>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { LATCH, core::Algorithm, cv_LATCH_to_Algorithm }

boxed_cast_base! { LATCH, crate::features2d::Feature2D, cv_LATCH_to_Feature2D }

/// Class implementing the locally uniform comparison image descriptor, described in [LUCID](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_LUCID)
/// 
/// An image descriptor that can be computed very fast, while being
/// about as robust as, for example, SURF or BRIEF.
/// 
/// 
/// Note: It requires a color image as input.
pub trait LUCIDTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_LUCID(&self) -> *const c_void;

}

pub trait LUCIDTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::LUCIDTraitConst {
	fn as_raw_mut_LUCID(&mut self) -> *mut c_void;

}

/// Class implementing the locally uniform comparison image descriptor, described in [LUCID](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_LUCID)
/// 
/// An image descriptor that can be computed very fast, while being
/// about as robust as, for example, SURF or BRIEF.
/// 
/// 
/// Note: It requires a color image as input.
pub struct LUCID {
	ptr: *mut c_void
}

opencv_type_boxed! { LUCID }

impl Drop for LUCID {
	fn drop(&mut self) {
		extern "C" { fn cv_LUCID_delete(instance: *mut c_void); }
		unsafe { cv_LUCID_delete(self.as_raw_mut_LUCID()) };
	}
}

unsafe impl Send for LUCID {}

impl core::AlgorithmTraitConst for LUCID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LUCID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for LUCID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for LUCID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::LUCIDTraitConst for LUCID {
	#[inline] fn as_raw_LUCID(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::LUCIDTrait for LUCID {
	#[inline] fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LUCID {
	/// ## Parameters
	/// * lucid_kernel: kernel for descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
	/// * blur_kernel: kernel for blurring image prior to descriptor construction, where 1=3x3, 2=5x5, 3=7x7 and so forth
	/// 
	/// ## C++ default parameters
	/// * lucid_kernel: 1
	/// * blur_kernel: 2
	#[inline]
	pub fn create(lucid_kernel: i32, blur_kernel: i32) -> Result<core::Ptr<crate::xfeatures2d::LUCID>> {
		unsafe { sys::cv_xfeatures2d_LUCID_create_const_int_const_int(lucid_kernel, blur_kernel) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::LUCID>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { LUCID, core::Algorithm, cv_LUCID_to_Algorithm }

boxed_cast_base! { LUCID, crate::features2d::Feature2D, cv_LUCID_to_Feature2D }

/// Class implementing the MSD (*Maximal Self-Dissimilarity*) keypoint detector, described in [Tombari14](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Tombari14).
/// 
/// The algorithm implements a novel interest point detector stemming from the intuition that image patches
/// which are highly dissimilar over a relatively large extent of their surroundings hold the property of
/// being repeatable and distinctive. This concept of "contextual self-dissimilarity" reverses the key
/// paradigm of recent successful techniques such as the Local Self-Similarity descriptor and the Non-Local
/// Means filter, which build upon the presence of similar - rather than dissimilar - patches. Moreover,
/// it extends to contextual information the local self-dissimilarity notion embedded in established
/// detectors of corner-like interest points, thereby achieving enhanced repeatability, distinctiveness and
/// localization accuracy.
pub trait MSDDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_MSDDetector(&self) -> *const c_void;

}

pub trait MSDDetectorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::MSDDetectorTraitConst {
	fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void;

}

/// Class implementing the MSD (*Maximal Self-Dissimilarity*) keypoint detector, described in [Tombari14](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Tombari14).
/// 
/// The algorithm implements a novel interest point detector stemming from the intuition that image patches
/// which are highly dissimilar over a relatively large extent of their surroundings hold the property of
/// being repeatable and distinctive. This concept of "contextual self-dissimilarity" reverses the key
/// paradigm of recent successful techniques such as the Local Self-Similarity descriptor and the Non-Local
/// Means filter, which build upon the presence of similar - rather than dissimilar - patches. Moreover,
/// it extends to contextual information the local self-dissimilarity notion embedded in established
/// detectors of corner-like interest points, thereby achieving enhanced repeatability, distinctiveness and
/// localization accuracy.
pub struct MSDDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { MSDDetector }

impl Drop for MSDDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_MSDDetector_delete(instance: *mut c_void); }
		unsafe { cv_MSDDetector_delete(self.as_raw_mut_MSDDetector()) };
	}
}

unsafe impl Send for MSDDetector {}

impl core::AlgorithmTraitConst for MSDDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MSDDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for MSDDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for MSDDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::MSDDetectorTraitConst for MSDDetector {
	#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::MSDDetectorTrait for MSDDetector {
	#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

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
	#[inline]
	pub fn create(m_patch_radius: i32, m_search_area_radius: i32, m_nms_radius: i32, m_nms_scale_radius: i32, m_th_saliency: f32, m_k_nn: i32, m_scale_factor: f32, m_n_scales: i32, m_compute_orientation: bool) -> Result<core::Ptr<crate::xfeatures2d::MSDDetector>> {
		unsafe { sys::cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_k_nn, m_scale_factor, m_n_scales, m_compute_orientation) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::MSDDetector>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { MSDDetector, core::Algorithm, cv_MSDDetector_to_Algorithm }

boxed_cast_base! { MSDDetector, crate::features2d::Feature2D, cv_MSDDetector_to_Feature2D }

/// Class implementing PCT (position-color-texture) signature extraction
///       as described in [KrulisLS16](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_KrulisLS16).
///       The algorithm is divided to a feature sampler and a clusterizer.
///       Feature sampler produces samples at given set of coordinates.
///       Clusterizer then produces clusters of these samples using k-means algorithm.
///       Resulting set of clusters is the signature of the input image.
/// 
///       A signature is an array of SIGNATURE_DIMENSION-dimensional points.
///       Used dimensions are:
///       weight, x, y position; lab color, contrast, entropy.
/// [KrulisLS16](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_KrulisLS16)
/// [BeecksUS10](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_BeecksUS10)
pub trait PCTSignaturesConst: core::AlgorithmTraitConst {
	fn as_raw_PCTSignatures(&self) -> *const c_void;

	/// Computes signature of given image.
	/// ## Parameters
	/// * image: Input image of CV_8U type.
	/// * signature: Output computed signature.
	#[inline]
	fn compute_signature(&self, image: &dyn core::ToInputArray, signature: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(signature);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(self.as_raw_PCTSignatures(), image.as_raw__InputArray(), signature.as_raw__OutputArray()) }.into_result()
	}
	
	/// Computes signatures for multiple images in parallel.
	/// ## Parameters
	/// * images: Vector of input images of CV_8U type.
	/// * signatures: Vector of computed signatures.
	#[inline]
	fn compute_signatures(&self, images: &core::Vector<core::Mat>, signatures: &mut core::Vector<core::Mat>) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vector_Mat_R_vector_Mat_R(self.as_raw_PCTSignatures(), images.as_raw_VectorOfMat(), signatures.as_raw_mut_VectorOfMat()) }.into_result()
	}
	
	/// Number of initial samples taken from the image.
	#[inline]
	fn get_sample_count(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getSampleCount_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Color resolution of the greyscale bitmap represented in allocated bits
	///       (i.e., value 4 means that 16 shades of grey are used).
	///       The greyscale bitmap is used for computing contrast and entropy values.
	#[inline]
	fn get_grayscale_bits(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Size of the texture sampling window used to compute contrast and entropy
	///       (center of the window is always in the pixel selected by x,y coordinates
	///       of the corresponding feature sample).
	#[inline]
	fn get_window_radius(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWindowRadius_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn get_weight_x(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightX_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn get_weight_y(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightY_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn get_weight_l(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightL_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn get_weight_a(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightA_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn get_weight_b(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightB_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn get_weight_contrast(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightContrast_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn get_weight_entropy(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Initial samples taken from the image.
	///       These sampled features become the input for clustering.
	#[inline]
	fn get_sampling_points(&self) -> Result<core::Vector<core::Point2f>> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(self.as_raw_PCTSignatures()) }.into_result().map(|r| unsafe { core::Vector::<core::Point2f>::opencv_from_extern(r) } )
	}
	
	/// ** clusterizer ***
	/// 
	/// * Initial seeds (initial number of clusters) for the k-means algorithm.
	#[inline]
	fn get_init_seed_indexes(&self) -> Result<core::Vector<i32>> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(self.as_raw_PCTSignatures()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
	}
	
	/// Number of initial seeds (initial number of clusters) for the k-means algorithm.
	#[inline]
	fn get_init_seed_count(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Number of iterations of the k-means clustering.
	///       We use fixed number of iterations, since the modified clustering is pruning clusters
	///       (not iteratively refining k clusters).
	#[inline]
	fn get_iteration_count(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getIterationCount_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Maximal number of generated clusters. If the number is exceeded,
	///       the clusters are sorted by their weights and the smallest clusters are cropped.
	#[inline]
	fn get_max_clusters_count(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// This parameter multiplied by the index of iteration gives lower limit for cluster size.
	///       Clusters containing fewer points than specified by the limit have their centroid dismissed
	///       and points are reassigned.
	#[inline]
	fn get_cluster_min_size(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Threshold euclidean distance between two centroids.
	///       If two cluster centers are closer than this distance,
	///       one of the centroid is dismissed and points are reassigned.
	#[inline]
	fn get_joining_distance(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Remove centroids in k-means whose weight is lesser or equal to given threshold.
	#[inline]
	fn get_drop_threshold(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getDropThreshold_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
	/// Distance function selector used for measuring distance between two points in k-means.
	#[inline]
	fn get_distance_function(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(self.as_raw_PCTSignatures()) }.into_result()
	}
	
}

pub trait PCTSignatures: core::AlgorithmTrait + crate::xfeatures2d::PCTSignaturesConst {
	fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void;

	/// Color resolution of the greyscale bitmap represented in allocated bits
	///       (i.e., value 4 means that 16 shades of grey are used).
	///       The greyscale bitmap is used for computing contrast and entropy values.
	#[inline]
	fn set_grayscale_bits(&mut self, grayscale_bits: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(self.as_raw_mut_PCTSignatures(), grayscale_bits) }.into_result()
	}
	
	/// Size of the texture sampling window used to compute contrast and entropy
	///       (center of the window is always in the pixel selected by x,y coordinates
	///       of the corresponding feature sample).
	#[inline]
	fn set_window_radius(&mut self, radius: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWindowRadius_int(self.as_raw_mut_PCTSignatures(), radius) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn set_weight_x(&mut self, weight: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightX_float(self.as_raw_mut_PCTSignatures(), weight) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn set_weight_y(&mut self, weight: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightY_float(self.as_raw_mut_PCTSignatures(), weight) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn set_weight_l(&mut self, weight: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightL_float(self.as_raw_mut_PCTSignatures(), weight) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn set_weight_a(&mut self, weight: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightA_float(self.as_raw_mut_PCTSignatures(), weight) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn set_weight_b(&mut self, weight: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightB_float(self.as_raw_mut_PCTSignatures(), weight) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn set_weight_contrast(&mut self, weight: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightContrast_float(self.as_raw_mut_PCTSignatures(), weight) }.into_result()
	}
	
	/// Weights (multiplicative constants) that linearly stretch individual axes of the feature space
	///       (x,y = position; L,a,b = color in CIE Lab space; c = contrast. e = entropy)
	#[inline]
	fn set_weight_entropy(&mut self, weight: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(self.as_raw_mut_PCTSignatures(), weight) }.into_result()
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
	#[inline]
	fn set_weight(&mut self, idx: i32, value: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeight_int_float(self.as_raw_mut_PCTSignatures(), idx, value) }.into_result()
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
	#[inline]
	fn set_weights(&mut self, weights: &core::Vector<f32>) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeights_const_vector_float_R(self.as_raw_mut_PCTSignatures(), weights.as_raw_VectorOff32()) }.into_result()
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
	#[inline]
	fn set_translation(&mut self, idx: i32, value: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslation_int_float(self.as_raw_mut_PCTSignatures(), idx, value) }.into_result()
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
	#[inline]
	fn set_translations(&mut self, translations: &core::Vector<f32>) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslations_const_vector_float_R(self.as_raw_mut_PCTSignatures(), translations.as_raw_VectorOff32()) }.into_result()
	}
	
	/// Sets sampling points used to sample the input image.
	/// ## Parameters
	/// * samplingPoints: Vector of sampling points in range [0..1)
	/// 
	/// Note: Number of sampling points must be greater or equal to clusterization seed count.
	#[inline]
	fn set_sampling_points(&mut self, mut sampling_points: core::Vector<core::Point2f>) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setSamplingPoints_vector_Point2f_(self.as_raw_mut_PCTSignatures(), sampling_points.as_raw_mut_VectorOfPoint2f()) }.into_result()
	}
	
	/// Initial seed indexes for the k-means algorithm.
	#[inline]
	fn set_init_seed_indexes(&mut self, mut init_seed_indexes: core::Vector<i32>) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vector_int_(self.as_raw_mut_PCTSignatures(), init_seed_indexes.as_raw_mut_VectorOfi32()) }.into_result()
	}
	
	/// Number of iterations of the k-means clustering.
	///       We use fixed number of iterations, since the modified clustering is pruning clusters
	///       (not iteratively refining k clusters).
	#[inline]
	fn set_iteration_count(&mut self, iteration_count: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setIterationCount_int(self.as_raw_mut_PCTSignatures(), iteration_count) }.into_result()
	}
	
	/// Maximal number of generated clusters. If the number is exceeded,
	///       the clusters are sorted by their weights and the smallest clusters are cropped.
	#[inline]
	fn set_max_clusters_count(&mut self, max_clusters_count: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(self.as_raw_mut_PCTSignatures(), max_clusters_count) }.into_result()
	}
	
	/// This parameter multiplied by the index of iteration gives lower limit for cluster size.
	///       Clusters containing fewer points than specified by the limit have their centroid dismissed
	///       and points are reassigned.
	#[inline]
	fn set_cluster_min_size(&mut self, cluster_min_size: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(self.as_raw_mut_PCTSignatures(), cluster_min_size) }.into_result()
	}
	
	/// Threshold euclidean distance between two centroids.
	///       If two cluster centers are closer than this distance,
	///       one of the centroid is dismissed and points are reassigned.
	#[inline]
	fn set_joining_distance(&mut self, joining_distance: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(self.as_raw_mut_PCTSignatures(), joining_distance) }.into_result()
	}
	
	/// Remove centroids in k-means whose weight is lesser or equal to given threshold.
	#[inline]
	fn set_drop_threshold(&mut self, drop_threshold: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setDropThreshold_float(self.as_raw_mut_PCTSignatures(), drop_threshold) }.into_result()
	}
	
	/// Distance function selector used for measuring distance between two points in k-means.
	///       Available: L0_25, L0_5, L1, L2, L2SQUARED, L5, L_INFINITY.
	#[inline]
	fn set_distance_function(&mut self, distance_function: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(self.as_raw_mut_PCTSignatures(), distance_function) }.into_result()
	}
	
}

impl dyn PCTSignatures + '_ {
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
	#[inline]
	pub fn create(init_sample_count: i32, init_seed_count: i32, point_distribution: i32) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignatures>> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(init_sample_count, init_seed_count, point_distribution) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>::opencv_from_extern(r) } )
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
	#[inline]
	pub fn create_1(init_sampling_points: &core::Vector<core::Point2f>, init_seed_count: i32) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignatures>> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_int(init_sampling_points.as_raw_VectorOfPoint2f(), init_seed_count) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>::opencv_from_extern(r) } )
	}
	
	/// Creates PCTSignatures algorithm using pre-generated sampling points
	///       and clusterization seeds indexes.
	/// ## Parameters
	/// * initSamplingPoints: Sampling points used in image sampling.
	/// * initClusterSeedIndexes: Indexes of initial clusterization seeds.
	///       Its size must be lower or equal to initSamplingPoints.size().
	/// ## Returns
	/// Created algorithm.
	#[inline]
	pub fn create_2(init_sampling_points: &core::Vector<core::Point2f>, init_cluster_seed_indexes: &core::Vector<i32>) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignatures>> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_vector_int_R(init_sampling_points.as_raw_VectorOfPoint2f(), init_cluster_seed_indexes.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>::opencv_from_extern(r) } )
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
	#[inline]
	pub fn draw_signature(source: &dyn core::ToInputArray, signature: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray, radius_to_shorter_side_ratio: f32, border_thickness: i32) -> Result<()> {
		input_array_arg!(source);
		input_array_arg!(signature);
		output_array_arg!(result);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(source.as_raw__InputArray(), signature.as_raw__InputArray(), result.as_raw__OutputArray(), radius_to_shorter_side_ratio, border_thickness) }.into_result()
	}
	
	/// Generates initial sampling points according to selected point distribution.
	/// ## Parameters
	/// * initPoints: Output vector where the generated points will be saved.
	/// * count: Number of points to generate.
	/// * pointDistribution: Point distribution selector.
	///       Available: UNIFORM, REGULAR, NORMAL.
	/// 
	/// Note: Generated coordinates are in range [0..1)
	#[inline]
	pub fn generate_init_points(init_points: &mut core::Vector<core::Point2f>, count: i32, point_distribution: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignatures_generateInitPoints_vector_Point2f_R_const_int_int(init_points.as_raw_mut_VectorOfPoint2f(), count, point_distribution) }.into_result()
	}
	
}
/// Class implementing Signature Quadratic Form Distance (SQFD).
/// ## See also
/// Christian Beecks, Merih Seran Uysal, Thomas Seidl.
///   Signature quadratic form distance.
///   In Proceedings of the ACM International Conference on Image and Video Retrieval, pages 438-445.
///   ACM, 2010.
/// [BeecksUS10](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_BeecksUS10)
pub trait PCTSignaturesSQFDConst: core::AlgorithmTraitConst {
	fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void;

	/// Computes Signature Quadratic Form Distance of two signatures.
	/// ## Parameters
	/// * _signature0: The first signature.
	/// * _signature1: The second signature.
	#[inline]
	fn compute_quadratic_form_distance(&self, _signature0: &dyn core::ToInputArray, _signature1: &dyn core::ToInputArray) -> Result<f32> {
		input_array_arg!(_signature0);
		input_array_arg!(_signature1);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(self.as_raw_PCTSignaturesSQFD(), _signature0.as_raw__InputArray(), _signature1.as_raw__InputArray()) }.into_result()
	}
	
	/// Computes Signature Quadratic Form Distance between the reference signature
	///       and each of the other image signatures.
	/// ## Parameters
	/// * sourceSignature: The signature to measure distance of other signatures from.
	/// * imageSignatures: Vector of signatures to measure distance from the source signature.
	/// * distances: Output vector of measured distances.
	#[inline]
	fn compute_quadratic_form_distances(&self, source_signature: &core::Mat, image_signatures: &core::Vector<core::Mat>, distances: &mut core::Vector<f32>) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatR_const_vector_Mat_R_vector_float_R(self.as_raw_PCTSignaturesSQFD(), source_signature.as_raw_Mat(), image_signatures.as_raw_VectorOfMat(), distances.as_raw_mut_VectorOff32()) }.into_result()
	}
	
}

pub trait PCTSignaturesSQFD: core::AlgorithmTrait + crate::xfeatures2d::PCTSignaturesSQFDConst {
	fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void;

}

impl dyn PCTSignaturesSQFD + '_ {
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
	#[inline]
	pub fn create(distance_function: i32, similarity_function: i32, similarity_parameter: f32) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignaturesSQFD>> {
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(distance_function, similarity_function, similarity_parameter) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignaturesSQFD>::opencv_from_extern(r) } )
	}
	
}
/// Class for extracting Speeded Up Robust Features from an image [Bay06](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Bay06) .
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
pub trait SURFConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_SURF(&self) -> *const c_void;

	#[inline]
	fn get_hessian_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_xfeatures2d_SURF_getHessianThreshold_const(self.as_raw_SURF()) }.into_result()
	}
	
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_SURF_getNOctaves_const(self.as_raw_SURF()) }.into_result()
	}
	
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_SURF_getNOctaveLayers_const(self.as_raw_SURF()) }.into_result()
	}
	
	#[inline]
	fn get_extended(&self) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_SURF_getExtended_const(self.as_raw_SURF()) }.into_result()
	}
	
	#[inline]
	fn get_upright(&self) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_SURF_getUpright_const(self.as_raw_SURF()) }.into_result()
	}
	
}

pub trait SURF: crate::features2d::Feature2DTrait + crate::xfeatures2d::SURFConst {
	fn as_raw_mut_SURF(&mut self) -> *mut c_void;

	#[inline]
	fn set_hessian_threshold(&mut self, hessian_threshold: f64) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_SURF_setHessianThreshold_double(self.as_raw_mut_SURF(), hessian_threshold) }.into_result()
	}
	
	#[inline]
	fn set_n_octaves(&mut self, n_octaves: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_SURF_setNOctaves_int(self.as_raw_mut_SURF(), n_octaves) }.into_result()
	}
	
	#[inline]
	fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_SURF_setNOctaveLayers_int(self.as_raw_mut_SURF(), n_octave_layers) }.into_result()
	}
	
	#[inline]
	fn set_extended(&mut self, extended: bool) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_SURF_setExtended_bool(self.as_raw_mut_SURF(), extended) }.into_result()
	}
	
	#[inline]
	fn set_upright(&mut self, upright: bool) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_SURF_setUpright_bool(self.as_raw_mut_SURF(), upright) }.into_result()
	}
	
}

impl dyn SURF + '_ {
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
	#[inline]
	pub fn create(hessian_threshold: f64, n_octaves: i32, n_octave_layers: i32, extended: bool, upright: bool) -> Result<core::Ptr<dyn crate::xfeatures2d::SURF>> {
		unsafe { sys::cv_xfeatures2d_SURF_create_double_int_int_bool_bool(hessian_threshold, n_octaves, n_octave_layers, extended, upright) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::SURF>::opencv_from_extern(r) } )
	}
	
}
/// The class implements the keypoint detector introduced by [Agrawal08](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Agrawal08), synonym of StarDetector. :
pub trait StarDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_StarDetector(&self) -> *const c_void;

}

pub trait StarDetectorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::StarDetectorTraitConst {
	fn as_raw_mut_StarDetector(&mut self) -> *mut c_void;

}

/// The class implements the keypoint detector introduced by [Agrawal08](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Agrawal08), synonym of StarDetector. :
pub struct StarDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { StarDetector }

impl Drop for StarDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_StarDetector_delete(instance: *mut c_void); }
		unsafe { cv_StarDetector_delete(self.as_raw_mut_StarDetector()) };
	}
}

unsafe impl Send for StarDetector {}

impl core::AlgorithmTraitConst for StarDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StarDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for StarDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for StarDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::StarDetectorTraitConst for StarDetector {
	#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::StarDetectorTrait for StarDetector {
	#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StarDetector {
	/// the full constructor
	/// 
	/// ## C++ default parameters
	/// * max_size: 45
	/// * response_threshold: 30
	/// * line_threshold_projected: 10
	/// * line_threshold_binarized: 8
	/// * suppress_nonmax_size: 5
	#[inline]
	pub fn create(max_size: i32, response_threshold: i32, line_threshold_projected: i32, line_threshold_binarized: i32, suppress_nonmax_size: i32) -> Result<core::Ptr<crate::xfeatures2d::StarDetector>> {
		unsafe { sys::cv_xfeatures2d_StarDetector_create_int_int_int_int_int(max_size, response_threshold, line_threshold_projected, line_threshold_binarized, suppress_nonmax_size) }.into_result().map(|r| unsafe { core::Ptr::<crate::xfeatures2d::StarDetector>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { StarDetector, core::Algorithm, cv_StarDetector_to_Algorithm }

boxed_cast_base! { StarDetector, crate::features2d::Feature2D, cv_StarDetector_to_Feature2D }

/// Class implementing the Tree Based Morse Regions (TBMR) as described in
/// [Najman2014](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Najman2014) extended with scaled extraction ability.
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
pub trait TBMRConst: crate::xfeatures2d::AffineFeature2DConst {
	fn as_raw_TBMR(&self) -> *const c_void;

	#[inline]
	fn get_min_area(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_TBMR_getMinArea_const(self.as_raw_TBMR()) }.into_result()
	}
	
	#[inline]
	fn get_max_area_relative(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_TBMR_getMaxAreaRelative_const(self.as_raw_TBMR()) }.into_result()
	}
	
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_TBMR_getScaleFactor_const(self.as_raw_TBMR()) }.into_result()
	}
	
	#[inline]
	fn get_n_scales(&self) -> Result<i32> {
		unsafe { sys::cv_xfeatures2d_TBMR_getNScales_const(self.as_raw_TBMR()) }.into_result()
	}
	
}

pub trait TBMR: crate::xfeatures2d::AffineFeature2D + crate::xfeatures2d::TBMRConst {
	fn as_raw_mut_TBMR(&mut self) -> *mut c_void;

	#[inline]
	fn set_min_area(&mut self, min_area: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_TBMR_setMinArea_int(self.as_raw_mut_TBMR(), min_area) }.into_result()
	}
	
	#[inline]
	fn set_max_area_relative(&mut self, max_area: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_TBMR_setMaxAreaRelative_float(self.as_raw_mut_TBMR(), max_area) }.into_result()
	}
	
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_TBMR_setScaleFactor_float(self.as_raw_mut_TBMR(), scale_factor) }.into_result()
	}
	
	#[inline]
	fn set_n_scales(&mut self, n_scales: i32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_TBMR_setNScales_int(self.as_raw_mut_TBMR(), n_scales) }.into_result()
	}
	
}

impl dyn TBMR + '_ {
	/// ## C++ default parameters
	/// * min_area: 60
	/// * max_area_relative: 0.01f
	/// * scale_factor: 1.25f
	/// * n_scales: -1
	#[inline]
	pub fn create(min_area: i32, max_area_relative: f32, scale_factor: f32, n_scales: i32) -> Result<core::Ptr<dyn crate::xfeatures2d::TBMR>> {
		unsafe { sys::cv_xfeatures2d_TBMR_create_int_float_float_int(min_area, max_area_relative, scale_factor, n_scales) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::TBMR>::opencv_from_extern(r) } )
	}
	
}
/// Class implementing VGG (Oxford Visual Geometry Group) descriptor trained end to end
/// using "Descriptor Learning Using Convex Optimisation" (DLCO) aparatus described in [Simonyan14](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_Simonyan14).
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
pub trait VGGConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_VGG(&self) -> *const c_void;

	#[inline]
	fn get_sigma(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_VGG_getSigma_const(self.as_raw_VGG()) }.into_result()
	}
	
	#[inline]
	fn get_use_normalize_image(&self) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeImage_const(self.as_raw_VGG()) }.into_result()
	}
	
	#[inline]
	fn get_use_scale_orientation(&self) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_VGG_getUseScaleOrientation_const(self.as_raw_VGG()) }.into_result()
	}
	
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		unsafe { sys::cv_xfeatures2d_VGG_getScaleFactor_const(self.as_raw_VGG()) }.into_result()
	}
	
	#[inline]
	fn get_use_normalize_descriptor(&self) -> Result<bool> {
		unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(self.as_raw_VGG()) }.into_result()
	}
	
}

pub trait VGG: crate::features2d::Feature2DTrait + crate::xfeatures2d::VGGConst {
	fn as_raw_mut_VGG(&mut self) -> *mut c_void;

	#[inline]
	fn set_sigma(&mut self, isigma: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_VGG_setSigma_const_float(self.as_raw_mut_VGG(), isigma) }.into_result()
	}
	
	#[inline]
	fn set_use_normalize_image(&mut self, img_normalize: bool) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(self.as_raw_mut_VGG(), img_normalize) }.into_result()
	}
	
	#[inline]
	fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(self.as_raw_mut_VGG(), use_scale_orientation) }.into_result()
	}
	
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_VGG_setScaleFactor_const_float(self.as_raw_mut_VGG(), scale_factor) }.into_result()
	}
	
	#[inline]
	fn set_use_normalize_descriptor(&mut self, dsc_normalize: bool) -> Result<()> {
		unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(self.as_raw_mut_VGG(), dsc_normalize) }.into_result()
	}
	
}

impl dyn VGG + '_ {
	/// ## C++ default parameters
	/// * desc: VGG::VGG_120
	/// * isigma: 1.4f
	/// * img_normalize: true
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	/// * dsc_normalize: false
	#[inline]
	pub fn create(desc: i32, isigma: f32, img_normalize: bool, use_scale_orientation: bool, scale_factor: f32, dsc_normalize: bool) -> Result<core::Ptr<dyn crate::xfeatures2d::VGG>> {
		unsafe { sys::cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::xfeatures2d::VGG>::opencv_from_extern(r) } )
	}
	
}