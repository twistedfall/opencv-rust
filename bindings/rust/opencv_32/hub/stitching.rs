#![allow(unused_parens)]
//! # Images stitching
//! 
//! This figure illustrates the stitching module pipeline implemented in the Stitcher class. Using that
//! class it's possible to configure/remove some steps, i.e. adjust the stitching pipeline according to
//! the particular needs. All building blocks from the pipeline are available in the detail namespace,
//! one can combine and use them separately.
//! 
//! The implemented stitching pipeline is very similar to the one proposed in [BL07](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_BL07) .
//! 
//! ![stitching pipeline](https://docs.opencv.org/3.2.0/StitchingPipeline.jpg)
//! 
//! Camera models
//! -------------
//! 
//! There are currently 2 camera models implemented in stitching pipeline.
//! 
//! - _Homography model_ expecting perspective transformations between images
//!   implemented in @ref cv::detail::BestOf2NearestMatcher cv::detail::HomographyBasedEstimator
//!   cv::detail::BundleAdjusterReproj cv::detail::BundleAdjusterRay
//! - _Affine model_ expecting affine transformation with 6 DOF or 4 DOF implemented in
//!   @ref cv::detail::AffineBestOf2NearestMatcher cv::detail::AffineBasedEstimator
//!   cv::detail::BundleAdjusterAffine cv::detail::BundleAdjusterAffinePartial cv::AffineWarper
//! 
//! Homography model is useful for creating photo panoramas captured by camera,
//! while affine-based model can be used to stitch scans and object captured by
//! specialized devices. Use @ref cv::Stitcher::create to get preconfigured pipeline for one
//! of those models.
//! 
//! 
//! Note:
//! Certain detailed settings of @ref cv::Stitcher might not make sense. Especially
//! you should not mix classes implementing affine model and classes implementing
//! Homography model, as they work with different transformations.
//!    # Features Finding and Images Matching
//!    # Rotation Estimation
//!    # Autocalibration
//!    # Images Warping
//!    # Seam Estimation
//!    # Exposure Compensation
//!    # Image Blenders
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::Detail_RotationWarper, super::Detail_ProjectorBaseTrait, super::Detail_PlaneProjectorTrait, super::Detail_PlaneWarperTrait, super::Detail_AffineWarperTrait, super::Detail_SphericalProjectorTrait, super::Detail_SphericalWarperTrait, super::Detail_CylindricalProjectorTrait, super::Detail_CylindricalWarperTrait, super::Detail_FisheyeProjectorTrait, super::Detail_FisheyeWarperTrait, super::Detail_StereographicProjectorTrait, super::Detail_StereographicWarperTrait, super::Detail_CompressedRectilinearProjectorTrait, super::Detail_CompressedRectilinearWarperTrait, super::Detail_CompressedRectilinearPortraitProjectorTrait, super::Detail_CompressedRectilinearPortraitWarperTrait, super::Detail_PaniniProjectorTrait, super::Detail_PaniniWarperTrait, super::Detail_PaniniPortraitProjectorTrait, super::Detail_PaniniPortraitWarperTrait, super::Detail_MercatorProjectorTrait, super::Detail_MercatorWarperTrait, super::Detail_TransverseMercatorProjectorTrait, super::Detail_TransverseMercatorWarperTrait, super::Detail_PlaneWarperGpuTrait, super::Detail_SphericalWarperGpuTrait, super::Detail_CylindricalWarperGpuTrait, super::Detail_SphericalPortraitWarperTrait, super::Detail_CylindricalPortraitWarperTrait, super::Detail_PlanePortraitWarperTrait, super::WarperCreator, super::PlaneWarperTrait, super::AffineWarperTrait, super::CylindricalWarperTrait, super::SphericalWarperTrait, super::FisheyeWarperTrait, super::StereographicWarperTrait, super::CompressedRectilinearWarperTrait, super::CompressedRectilinearPortraitWarperTrait, super::PaniniWarperTrait, super::PaniniPortraitWarperTrait, super::MercatorWarperTrait, super::TransverseMercatorWarperTrait, super::Detail_ImageFeaturesTrait, super::Detail_FeaturesFinder, super::Detail_SurfFeaturesFinderTrait, super::Detail_OrbFeaturesFinderTrait, super::Detail_AKAZEFeaturesFinderTrait, super::Detail_SurfFeaturesFinderGpuTrait, super::Detail_MatchesInfoTrait, super::Detail_FeaturesMatcher, super::Detail_BestOf2NearestMatcherTrait, super::Detail_BestOf2NearestRangeMatcherTrait, super::Detail_AffineBestOf2NearestMatcherTrait, super::Detail_DisjointSetsTrait, super::Detail_GraphEdgeTrait, super::Detail_GraphTrait, super::Detail_CameraParamsTrait, super::Detail_Estimator, super::Detail_HomographyBasedEstimatorTrait, super::Detail_AffineBasedEstimatorTrait, super::Detail_BundleAdjusterBase, super::Detail_NoBundleAdjusterTrait, super::Detail_BundleAdjusterReprojTrait, super::Detail_BundleAdjusterRayTrait, super::Detail_BundleAdjusterAffineTrait, super::Detail_BundleAdjusterAffinePartialTrait, super::Detail_ExposureCompensator, super::Detail_NoExposureCompensatorTrait, super::Detail_GainCompensatorTrait, super::Detail_BlocksGainCompensatorTrait, super::Detail_SeamFinder, super::Detail_NoSeamFinderTrait, super::Detail_PairwiseSeamFinder, super::Detail_VoronoiSeamFinderTrait, super::Detail_DpSeamFinderTrait, super::Detail_GraphCutSeamFinderBaseTrait, super::Detail_GraphCutSeamFinderTrait, super::Detail_BlenderTrait, super::Detail_FeatherBlenderTrait, super::Detail_MultiBandBlenderTrait, super::StitcherTrait };
}

pub const Detail_Blender_FEATHER: i32 = 1;
pub const Detail_Blender_MULTI_BAND: i32 = 2;
pub const Detail_Blender_NO: i32 = 0;
pub const Detail_ExposureCompensator_GAIN: i32 = 1;
pub const Detail_ExposureCompensator_GAIN_BLOCKS: i32 = 2;
pub const Detail_ExposureCompensator_NO: i32 = 0;
pub const Detail_WAVE_CORRECT_HORIZ: i32 = 0;
pub const Detail_WAVE_CORRECT_VERT: i32 = 1;
pub const Stitcher_ORIG_RESOL: i32 = -1;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Detail_DpSeamFinder_CostFunction {
	COLOR = 0,
	COLOR_GRAD = 1,
}

opencv_type_enum! { crate::stitching::Detail_DpSeamFinder_CostFunction }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Detail_GraphCutSeamFinderBase_CostType {
	COST_COLOR = 0,
	COST_COLOR_GRAD = 1,
}

opencv_type_enum! { crate::stitching::Detail_GraphCutSeamFinderBase_CostType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Detail_WaveCorrectKind {
	WAVE_CORRECT_HORIZ = 0,
	WAVE_CORRECT_VERT = 1,
}

opencv_type_enum! { crate::stitching::Detail_WaveCorrectKind }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Stitcher_Mode {
	/// Mode for creating photo panoramas. Expects images under perspective
	/// transformation and projects resulting pano to sphere.
	/// ## See also
	/// detail::BestOf2NearestMatcher SphericalWarper
	PANORAMA = 0,
	/// Mode for composing scans. Expects images under affine transformation does
	/// not compensate exposure by default.
	/// ## See also
	/// detail::AffineBestOf2NearestMatcher AffineWarper
	SCANS = 1,
}

opencv_type_enum! { crate::stitching::Stitcher_Mode }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Stitcher_Status {
	OK = 0,
	ERR_NEED_MORE_IMGS = 1,
	ERR_HOMOGRAPHY_EST_FAIL = 2,
	ERR_CAMERA_PARAMS_ADJUST_FAIL = 3,
}

opencv_type_enum! { crate::stitching::Stitcher_Status }

pub fn create_laplace_pyr_gpu(img: &dyn core::ToInputArray, num_levels: i32, pyr: &mut core::Vector::<core::UMat>) -> Result<()> {
	input_array_arg!(img);
	unsafe { sys::cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vector_UMat_R(img.as_raw__InputArray(), num_levels, pyr.as_raw_mut_VectorOfUMat()) }.into_result()
}

pub fn create_laplace_pyr(img: &dyn core::ToInputArray, num_levels: i32, pyr: &mut core::Vector::<core::UMat>) -> Result<()> {
	input_array_arg!(img);
	unsafe { sys::cv_detail_createLaplacePyr_const__InputArrayR_int_vector_UMat_R(img.as_raw__InputArray(), num_levels, pyr.as_raw_mut_VectorOfUMat()) }.into_result()
}

pub fn create_weight_map(mask: &dyn core::ToInputArray, sharpness: f32, weight: &mut dyn core::ToInputOutputArray) -> Result<()> {
	input_array_arg!(mask);
	input_output_array_arg!(weight);
	unsafe { sys::cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(mask.as_raw__InputArray(), sharpness, weight.as_raw__InputOutputArray()) }.into_result()
}

pub fn find_max_spanning_tree(num_images: i32, pairwise_matches: &core::Vector::<crate::stitching::Detail_MatchesInfo>, span_tree: &mut crate::stitching::Detail_Graph, centers: &mut core::Vector::<i32>) -> Result<()> {
	unsafe { sys::cv_detail_findMaxSpanningTree_int_const_vector_MatchesInfo_R_GraphR_vector_int_R(num_images, pairwise_matches.as_raw_VectorOfDetail_MatchesInfo(), span_tree.as_raw_mut_Detail_Graph(), centers.as_raw_mut_VectorOfi32()) }.into_result()
}

pub fn leave_biggest_component(features: &mut core::Vector::<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector::<crate::stitching::Detail_MatchesInfo>, conf_threshold: f32) -> Result<core::Vector::<i32>> {
	unsafe { sys::cv_detail_leaveBiggestComponent_vector_ImageFeatures_R_vector_MatchesInfo_R_float(features.as_raw_mut_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), conf_threshold) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
}

/// ///////////////////////////////////////////////////////////////////////////
pub fn matches_graph_as_string(pathes: &mut core::Vector::<String>, pairwise_matches: &mut core::Vector::<crate::stitching::Detail_MatchesInfo>, conf_threshold: f32) -> Result<String> {
	unsafe { sys::cv_detail_matchesGraphAsString_vector_String_R_vector_MatchesInfo_R_float(pathes.as_raw_mut_VectorOfString(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), conf_threshold) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
}

/// ///////////////////////////////////////////////////////////////////////////
pub fn normalize_using_weight_map(weight: &dyn core::ToInputArray, src: &mut dyn core::ToInputOutputArray) -> Result<()> {
	input_array_arg!(weight);
	input_output_array_arg!(src);
	unsafe { sys::cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(weight.as_raw__InputArray(), src.as_raw__InputOutputArray()) }.into_result()
}

/// ///////////////////////////////////////////////////////////////////////////
pub fn overlap_roi(tl1: core::Point, tl2: core::Point, sz1: core::Size, sz2: core::Size, roi: &mut core::Rect) -> Result<bool> {
	unsafe { sys::cv_detail_overlapRoi_Point_Point_Size_Size_RectR(tl1.opencv_as_extern(), tl2.opencv_as_extern(), sz1.opencv_as_extern(), sz2.opencv_as_extern(), roi) }.into_result()
}

pub fn restore_image_from_laplace_pyr_gpu(pyr: &mut core::Vector::<core::UMat>) -> Result<()> {
	unsafe { sys::cv_detail_restoreImageFromLaplacePyrGpu_vector_UMat_R(pyr.as_raw_mut_VectorOfUMat()) }.into_result()
}

pub fn restore_image_from_laplace_pyr(pyr: &mut core::Vector::<core::UMat>) -> Result<()> {
	unsafe { sys::cv_detail_restoreImageFromLaplacePyr_vector_UMat_R(pyr.as_raw_mut_VectorOfUMat()) }.into_result()
}

pub fn result_roi_intersection(corners: &core::Vector::<core::Point>, sizes: &core::Vector::<core::Size>) -> Result<core::Rect> {
	unsafe { sys::cv_detail_resultRoiIntersection_const_vector_Point_R_const_vector_Size_R(corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize()) }.into_result()
}

pub fn result_roi_1(corners: &core::Vector::<core::Point>, sizes: &core::Vector::<core::Size>) -> Result<core::Rect> {
	unsafe { sys::cv_detail_resultRoi_const_vector_Point_R_const_vector_Size_R(corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize()) }.into_result()
}

pub fn result_roi(corners: &core::Vector::<core::Point>, images: &core::Vector::<core::UMat>) -> Result<core::Rect> {
	unsafe { sys::cv_detail_resultRoi_const_vector_Point_R_const_vector_UMat_R(corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat()) }.into_result()
}

pub fn result_tl(corners: &core::Vector::<core::Point>) -> Result<core::Point> {
	unsafe { sys::cv_detail_resultTl_const_vector_Point_R(corners.as_raw_VectorOfPoint()) }.into_result()
}

pub fn select_random_subset(count: i32, size: i32, subset: &mut core::Vector::<i32>) -> Result<()> {
	unsafe { sys::cv_detail_selectRandomSubset_int_int_vector_int_R(count, size, subset.as_raw_mut_VectorOfi32()) }.into_result()
}

pub fn stitching_log_level() -> Result<i32> {
	unsafe { sys::cv_detail_stitchingLogLevel() }.into_result()
}

/// Tries to make panorama more horizontal (or vertical).
/// 
/// ## Parameters
/// * rmats: Camera rotation matrices.
/// * kind: Correction kind, see detail::WaveCorrectKind.
pub fn wave_correct(rmats: &mut core::Vector::<core::Mat>, kind: crate::stitching::Detail_WaveCorrectKind) -> Result<()> {
	unsafe { sys::cv_detail_waveCorrect_vector_Mat_R_WaveCorrectKind(rmats.as_raw_mut_VectorOfMat(), kind) }.into_result()
}

/// Affine warper factory class.
/// ## See also
/// detail::AffineWarper
pub trait AffineWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_AffineWarper(&self) -> *const c_void;
	fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_AffineWarper_create_const_float(self.as_raw_AffineWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

/// Affine warper factory class.
/// ## See also
/// detail::AffineWarper
pub struct AffineWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { AffineWarper }

impl Drop for AffineWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_AffineWarper_delete(instance: *mut c_void); }
		unsafe { cv_AffineWarper_delete(self.as_raw_mut_AffineWarper()) };
	}
}

impl AffineWarper {
	#[inline] pub fn as_raw_AffineWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for AffineWarper {}

impl crate::stitching::AffineWarperTrait for AffineWarper {
	#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for AffineWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AffineWarper {
}

pub trait CompressedRectilinearPortraitWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void;
	fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_CompressedRectilinearPortraitWarper_create_const_float(self.as_raw_CompressedRectilinearPortraitWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct CompressedRectilinearPortraitWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { CompressedRectilinearPortraitWarper }

impl Drop for CompressedRectilinearPortraitWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_CompressedRectilinearPortraitWarper_delete(instance: *mut c_void); }
		unsafe { cv_CompressedRectilinearPortraitWarper_delete(self.as_raw_mut_CompressedRectilinearPortraitWarper()) };
	}
}

impl CompressedRectilinearPortraitWarper {
	#[inline] pub fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CompressedRectilinearPortraitWarper {}

impl crate::stitching::CompressedRectilinearPortraitWarperTrait for CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CompressedRectilinearPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearPortraitWarper> {
		unsafe { sys::cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(a, b) }.into_result().map(|r| unsafe { crate::stitching::CompressedRectilinearPortraitWarper::opencv_from_extern(r) } )
	}
	
}

pub trait CompressedRectilinearWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void;
	fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_CompressedRectilinearWarper_create_const_float(self.as_raw_CompressedRectilinearWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct CompressedRectilinearWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { CompressedRectilinearWarper }

impl Drop for CompressedRectilinearWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_CompressedRectilinearWarper_delete(instance: *mut c_void); }
		unsafe { cv_CompressedRectilinearWarper_delete(self.as_raw_mut_CompressedRectilinearWarper()) };
	}
}

impl CompressedRectilinearWarper {
	#[inline] pub fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CompressedRectilinearWarper {}

impl crate::stitching::CompressedRectilinearWarperTrait for CompressedRectilinearWarper {
	#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for CompressedRectilinearWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CompressedRectilinearWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearWarper> {
		unsafe { sys::cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(a, b) }.into_result().map(|r| unsafe { crate::stitching::CompressedRectilinearWarper::opencv_from_extern(r) } )
	}
	
}

/// Cylindrical warper factory class.
/// ## See also
/// detail::CylindricalWarper
pub trait CylindricalWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_CylindricalWarper(&self) -> *const c_void;
	fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_CylindricalWarper_create_const_float(self.as_raw_CylindricalWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

/// Cylindrical warper factory class.
/// ## See also
/// detail::CylindricalWarper
pub struct CylindricalWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { CylindricalWarper }

impl Drop for CylindricalWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_CylindricalWarper_delete(instance: *mut c_void); }
		unsafe { cv_CylindricalWarper_delete(self.as_raw_mut_CylindricalWarper()) };
	}
}

impl CylindricalWarper {
	#[inline] pub fn as_raw_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CylindricalWarper {}

impl crate::stitching::CylindricalWarperTrait for CylindricalWarper {
	#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for CylindricalWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CylindricalWarper {
}

pub trait FisheyeWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_FisheyeWarper(&self) -> *const c_void;
	fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_FisheyeWarper_create_const_float(self.as_raw_FisheyeWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct FisheyeWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { FisheyeWarper }

impl Drop for FisheyeWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_FisheyeWarper_delete(instance: *mut c_void); }
		unsafe { cv_FisheyeWarper_delete(self.as_raw_mut_FisheyeWarper()) };
	}
}

impl FisheyeWarper {
	#[inline] pub fn as_raw_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FisheyeWarper {}

impl crate::stitching::FisheyeWarperTrait for FisheyeWarper {
	#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for FisheyeWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FisheyeWarper {
}

pub trait MercatorWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_MercatorWarper(&self) -> *const c_void;
	fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_MercatorWarper_create_const_float(self.as_raw_MercatorWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct MercatorWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { MercatorWarper }

impl Drop for MercatorWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_MercatorWarper_delete(instance: *mut c_void); }
		unsafe { cv_MercatorWarper_delete(self.as_raw_mut_MercatorWarper()) };
	}
}

impl MercatorWarper {
	#[inline] pub fn as_raw_MercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MercatorWarper {}

impl crate::stitching::MercatorWarperTrait for MercatorWarper {
	#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for MercatorWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MercatorWarper {
}

pub trait PaniniPortraitWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_PaniniPortraitWarper(&self) -> *const c_void;
	fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_PaniniPortraitWarper_create_const_float(self.as_raw_PaniniPortraitWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct PaniniPortraitWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { PaniniPortraitWarper }

impl Drop for PaniniPortraitWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_PaniniPortraitWarper_delete(instance: *mut c_void); }
		unsafe { cv_PaniniPortraitWarper_delete(self.as_raw_mut_PaniniPortraitWarper()) };
	}
}

impl PaniniPortraitWarper {
	#[inline] pub fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PaniniPortraitWarper {}

impl crate::stitching::PaniniPortraitWarperTrait for PaniniPortraitWarper {
	#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for PaniniPortraitWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PaniniPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniPortraitWarper> {
		unsafe { sys::cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(a, b) }.into_result().map(|r| unsafe { crate::stitching::PaniniPortraitWarper::opencv_from_extern(r) } )
	}
	
}

pub trait PaniniWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_PaniniWarper(&self) -> *const c_void;
	fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_PaniniWarper_create_const_float(self.as_raw_PaniniWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct PaniniWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { PaniniWarper }

impl Drop for PaniniWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_PaniniWarper_delete(instance: *mut c_void); }
		unsafe { cv_PaniniWarper_delete(self.as_raw_mut_PaniniWarper()) };
	}
}

impl PaniniWarper {
	#[inline] pub fn as_raw_PaniniWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PaniniWarper {}

impl crate::stitching::PaniniWarperTrait for PaniniWarper {
	#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for PaniniWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PaniniWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniWarper> {
		unsafe { sys::cv_PaniniWarper_PaniniWarper_float_float(a, b) }.into_result().map(|r| unsafe { crate::stitching::PaniniWarper::opencv_from_extern(r) } )
	}
	
}

/// Plane warper factory class.
/// ## See also
/// detail::PlaneWarper
pub trait PlaneWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_PlaneWarper(&self) -> *const c_void;
	fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_PlaneWarper_create_const_float(self.as_raw_PlaneWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

/// Plane warper factory class.
/// ## See also
/// detail::PlaneWarper
pub struct PlaneWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { PlaneWarper }

impl Drop for PlaneWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_PlaneWarper_delete(instance: *mut c_void); }
		unsafe { cv_PlaneWarper_delete(self.as_raw_mut_PlaneWarper()) };
	}
}

impl PlaneWarper {
	#[inline] pub fn as_raw_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PlaneWarper {}

impl crate::stitching::PlaneWarperTrait for PlaneWarper {
	#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for PlaneWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PlaneWarper {
}

/// Spherical warper factory class
pub trait SphericalWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_SphericalWarper(&self) -> *const c_void;
	fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_SphericalWarper_create_const_float(self.as_raw_SphericalWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

/// Spherical warper factory class
pub struct SphericalWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { SphericalWarper }

impl Drop for SphericalWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_SphericalWarper_delete(instance: *mut c_void); }
		unsafe { cv_SphericalWarper_delete(self.as_raw_mut_SphericalWarper()) };
	}
}

impl SphericalWarper {
	#[inline] pub fn as_raw_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SphericalWarper {}

impl crate::stitching::SphericalWarperTrait for SphericalWarper {
	#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for SphericalWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SphericalWarper {
}

pub trait StereographicWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_StereographicWarper(&self) -> *const c_void;
	fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_StereographicWarper_create_const_float(self.as_raw_StereographicWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct StereographicWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { StereographicWarper }

impl Drop for StereographicWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_StereographicWarper_delete(instance: *mut c_void); }
		unsafe { cv_StereographicWarper_delete(self.as_raw_mut_StereographicWarper()) };
	}
}

impl StereographicWarper {
	#[inline] pub fn as_raw_StereographicWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for StereographicWarper {}

impl crate::stitching::StereographicWarperTrait for StereographicWarper {
	#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for StereographicWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StereographicWarper {
}

/// High level image stitcher.
/// 
/// It's possible to use this class without being aware of the entire stitching pipeline. However, to
/// be able to achieve higher stitching stability and quality of the final images at least being
/// familiar with the theory is recommended.
/// 
/// 
/// Note:
///    *   A basic example on image stitching can be found at
///        opencv_source_code/samples/cpp/stitching.cpp
///    *   A detailed example on image stitching can be found at
///        opencv_source_code/samples/cpp/stitching_detailed.cpp
pub trait StitcherTrait {
	fn as_raw_Stitcher(&self) -> *const c_void;
	fn as_raw_mut_Stitcher(&mut self) -> *mut c_void;

	fn registration_resol(&self) -> Result<f64> {
		unsafe { sys::cv_Stitcher_registrationResol_const(self.as_raw_Stitcher()) }.into_result()
	}
	
	fn set_registration_resol(&mut self, resol_mpx: f64) -> Result<()> {
		unsafe { sys::cv_Stitcher_setRegistrationResol_double(self.as_raw_mut_Stitcher(), resol_mpx) }.into_result()
	}
	
	fn seam_estimation_resol(&self) -> Result<f64> {
		unsafe { sys::cv_Stitcher_seamEstimationResol_const(self.as_raw_Stitcher()) }.into_result()
	}
	
	fn set_seam_estimation_resol(&mut self, resol_mpx: f64) -> Result<()> {
		unsafe { sys::cv_Stitcher_setSeamEstimationResol_double(self.as_raw_mut_Stitcher(), resol_mpx) }.into_result()
	}
	
	fn compositing_resol(&self) -> Result<f64> {
		unsafe { sys::cv_Stitcher_compositingResol_const(self.as_raw_Stitcher()) }.into_result()
	}
	
	fn set_compositing_resol(&mut self, resol_mpx: f64) -> Result<()> {
		unsafe { sys::cv_Stitcher_setCompositingResol_double(self.as_raw_mut_Stitcher(), resol_mpx) }.into_result()
	}
	
	fn pano_confidence_thresh(&self) -> Result<f64> {
		unsafe { sys::cv_Stitcher_panoConfidenceThresh_const(self.as_raw_Stitcher()) }.into_result()
	}
	
	fn set_pano_confidence_thresh(&mut self, conf_thresh: f64) -> Result<()> {
		unsafe { sys::cv_Stitcher_setPanoConfidenceThresh_double(self.as_raw_mut_Stitcher(), conf_thresh) }.into_result()
	}
	
	fn wave_correction(&self) -> Result<bool> {
		unsafe { sys::cv_Stitcher_waveCorrection_const(self.as_raw_Stitcher()) }.into_result()
	}
	
	fn set_wave_correction(&mut self, flag: bool) -> Result<()> {
		unsafe { sys::cv_Stitcher_setWaveCorrection_bool(self.as_raw_mut_Stitcher(), flag) }.into_result()
	}
	
	fn wave_correct_kind(&self) -> Result<crate::stitching::Detail_WaveCorrectKind> {
		unsafe { sys::cv_Stitcher_waveCorrectKind_const(self.as_raw_Stitcher()) }.into_result()
	}
	
	fn set_wave_correct_kind(&mut self, kind: crate::stitching::Detail_WaveCorrectKind) -> Result<()> {
		unsafe { sys::cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(self.as_raw_mut_Stitcher(), kind) }.into_result()
	}
	
	fn features_finder(&mut self) -> Result<core::Ptr::<dyn crate::stitching::Detail_FeaturesFinder>> {
		unsafe { sys::cv_Stitcher_featuresFinder(self.as_raw_mut_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_FeaturesFinder>::opencv_from_extern(r) } )
	}
	
	fn features_finder_1(&self) -> Result<core::Ptr::<dyn crate::stitching::Detail_FeaturesFinder>> {
		unsafe { sys::cv_Stitcher_featuresFinder_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_FeaturesFinder>::opencv_from_extern(r) } )
	}
	
	fn set_features_finder(&mut self, mut features_finder: core::Ptr::<dyn crate::stitching::Detail_FeaturesFinder>) -> Result<()> {
		unsafe { sys::cv_Stitcher_setFeaturesFinder_Ptr_FeaturesFinder_(self.as_raw_mut_Stitcher(), features_finder.as_raw_mut_PtrOfDetail_FeaturesFinder()) }.into_result()
	}
	
	fn features_matcher(&mut self) -> Result<core::Ptr::<dyn crate::stitching::Detail_FeaturesMatcher>> {
		unsafe { sys::cv_Stitcher_featuresMatcher(self.as_raw_mut_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_FeaturesMatcher>::opencv_from_extern(r) } )
	}
	
	fn features_matcher_1(&self) -> Result<core::Ptr::<dyn crate::stitching::Detail_FeaturesMatcher>> {
		unsafe { sys::cv_Stitcher_featuresMatcher_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_FeaturesMatcher>::opencv_from_extern(r) } )
	}
	
	fn set_features_matcher(&mut self, mut features_matcher: core::Ptr::<dyn crate::stitching::Detail_FeaturesMatcher>) -> Result<()> {
		unsafe { sys::cv_Stitcher_setFeaturesMatcher_Ptr_FeaturesMatcher_(self.as_raw_mut_Stitcher(), features_matcher.as_raw_mut_PtrOfDetail_FeaturesMatcher()) }.into_result()
	}
	
	fn matching_mask(&self) -> Result<core::UMat> {
		unsafe { sys::cv_Stitcher_matchingMask_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } )
	}
	
	fn set_matching_mask(&mut self, mask: &core::UMat) -> Result<()> {
		unsafe { sys::cv_Stitcher_setMatchingMask_const_UMatR(self.as_raw_mut_Stitcher(), mask.as_raw_UMat()) }.into_result()
	}
	
	fn bundle_adjuster(&mut self) -> Result<core::Ptr::<dyn crate::stitching::Detail_BundleAdjusterBase>> {
		unsafe { sys::cv_Stitcher_bundleAdjuster(self.as_raw_mut_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_BundleAdjusterBase>::opencv_from_extern(r) } )
	}
	
	fn bundle_adjuster_1(&self) -> Result<core::Ptr::<dyn crate::stitching::Detail_BundleAdjusterBase>> {
		unsafe { sys::cv_Stitcher_bundleAdjuster_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_BundleAdjusterBase>::opencv_from_extern(r) } )
	}
	
	fn set_bundle_adjuster(&mut self, mut bundle_adjuster: core::Ptr::<dyn crate::stitching::Detail_BundleAdjusterBase>) -> Result<()> {
		unsafe { sys::cv_Stitcher_setBundleAdjuster_Ptr_BundleAdjusterBase_(self.as_raw_mut_Stitcher(), bundle_adjuster.as_raw_mut_PtrOfDetail_BundleAdjusterBase()) }.into_result()
	}
	
	fn warper(&mut self) -> Result<core::Ptr::<dyn crate::stitching::WarperCreator>> {
		unsafe { sys::cv_Stitcher_warper(self.as_raw_mut_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::WarperCreator>::opencv_from_extern(r) } )
	}
	
	fn warper_1(&self) -> Result<core::Ptr::<dyn crate::stitching::WarperCreator>> {
		unsafe { sys::cv_Stitcher_warper_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::WarperCreator>::opencv_from_extern(r) } )
	}
	
	fn set_warper(&mut self, mut creator: core::Ptr::<dyn crate::stitching::WarperCreator>) -> Result<()> {
		unsafe { sys::cv_Stitcher_setWarper_Ptr_WarperCreator_(self.as_raw_mut_Stitcher(), creator.as_raw_mut_PtrOfWarperCreator()) }.into_result()
	}
	
	fn exposure_compensator(&mut self) -> Result<core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>> {
		unsafe { sys::cv_Stitcher_exposureCompensator(self.as_raw_mut_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(r) } )
	}
	
	fn exposure_compensator_1(&self) -> Result<core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>> {
		unsafe { sys::cv_Stitcher_exposureCompensator_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(r) } )
	}
	
	fn set_exposure_compensator(&mut self, mut exposure_comp: core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>) -> Result<()> {
		unsafe { sys::cv_Stitcher_setExposureCompensator_Ptr_ExposureCompensator_(self.as_raw_mut_Stitcher(), exposure_comp.as_raw_mut_PtrOfDetail_ExposureCompensator()) }.into_result()
	}
	
	fn seam_finder(&mut self) -> Result<core::Ptr::<dyn crate::stitching::Detail_SeamFinder>> {
		unsafe { sys::cv_Stitcher_seamFinder(self.as_raw_mut_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_SeamFinder>::opencv_from_extern(r) } )
	}
	
	fn seam_finder_1(&self) -> Result<core::Ptr::<dyn crate::stitching::Detail_SeamFinder>> {
		unsafe { sys::cv_Stitcher_seamFinder_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_SeamFinder>::opencv_from_extern(r) } )
	}
	
	fn set_seam_finder(&mut self, mut seam_finder: core::Ptr::<dyn crate::stitching::Detail_SeamFinder>) -> Result<()> {
		unsafe { sys::cv_Stitcher_setSeamFinder_Ptr_SeamFinder_(self.as_raw_mut_Stitcher(), seam_finder.as_raw_mut_PtrOfDetail_SeamFinder()) }.into_result()
	}
	
	fn blender(&mut self) -> Result<core::Ptr::<crate::stitching::Detail_Blender>> {
		unsafe { sys::cv_Stitcher_blender(self.as_raw_mut_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(r) } )
	}
	
	fn blender_1(&self) -> Result<core::Ptr::<crate::stitching::Detail_Blender>> {
		unsafe { sys::cv_Stitcher_blender_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(r) } )
	}
	
	fn set_blender(&mut self, mut b: core::Ptr::<crate::stitching::Detail_Blender>) -> Result<()> {
		unsafe { sys::cv_Stitcher_setBlender_Ptr_Blender_(self.as_raw_mut_Stitcher(), b.as_raw_mut_PtrOfDetail_Blender()) }.into_result()
	}
	
	/// These functions try to match the given images and to estimate rotations of each camera.
	/// 
	/// 
	/// Note: Use the functions only if you're aware of the stitching pipeline, otherwise use
	/// Stitcher::stitch.
	/// 
	/// ## Parameters
	/// * images: Input images.
	/// * rois: Region of interest rectangles.
	/// ## Returns
	/// Status code.
	/// 
	/// ## Overloaded parameters
	fn estimate_transform(&mut self, images: &dyn core::ToInputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		unsafe { sys::cv_Stitcher_estimateTransform_const__InputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray()) }.into_result()
	}
	
	/// These functions try to match the given images and to estimate rotations of each camera.
	/// 
	/// 
	/// Note: Use the functions only if you're aware of the stitching pipeline, otherwise use
	/// Stitcher::stitch.
	/// 
	/// ## Parameters
	/// * images: Input images.
	/// * rois: Region of interest rectangles.
	/// ## Returns
	/// Status code.
	fn estimate_transform_1(&mut self, images: &dyn core::ToInputArray, rois: &core::Vector::<core::Vector::<core::Rect>>) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		unsafe { sys::cv_Stitcher_estimateTransform_const__InputArrayR_const_vector_vector_Rect__R(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), rois.as_raw_VectorOfVectorOfRect()) }.into_result()
	}
	
	/// These functions try to compose the given images (or images stored internally from the other function
	/// calls) into the final pano under the assumption that the image transformations were estimated
	/// before.
	/// 
	/// 
	/// Note: Use the functions only if you're aware of the stitching pipeline, otherwise use
	/// Stitcher::stitch.
	/// 
	/// ## Parameters
	/// * images: Input images.
	/// * pano: Final pano.
	/// ## Returns
	/// Status code.
	/// 
	/// ## Overloaded parameters
	fn compose_panorama(&mut self, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		output_array_arg!(pano);
		unsafe { sys::cv_Stitcher_composePanorama_const__OutputArrayR(self.as_raw_mut_Stitcher(), pano.as_raw__OutputArray()) }.into_result()
	}
	
	/// These functions try to compose the given images (or images stored internally from the other function
	/// calls) into the final pano under the assumption that the image transformations were estimated
	/// before.
	/// 
	/// 
	/// Note: Use the functions only if you're aware of the stitching pipeline, otherwise use
	/// Stitcher::stitch.
	/// 
	/// ## Parameters
	/// * images: Input images.
	/// * pano: Final pano.
	/// ## Returns
	/// Status code.
	fn compose_panorama_images(&mut self, images: &dyn core::ToInputArray, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		output_array_arg!(pano);
		unsafe { sys::cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), pano.as_raw__OutputArray()) }.into_result()
	}
	
	/// These functions try to stitch the given images.
	/// 
	/// ## Parameters
	/// * images: Input images.
	/// * rois: Region of interest rectangles.
	/// * pano: Final pano.
	/// ## Returns
	/// Status code.
	/// 
	/// ## Overloaded parameters
	fn stitch(&mut self, images: &dyn core::ToInputArray, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		output_array_arg!(pano);
		unsafe { sys::cv_Stitcher_stitch_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), pano.as_raw__OutputArray()) }.into_result()
	}
	
	/// These functions try to stitch the given images.
	/// 
	/// ## Parameters
	/// * images: Input images.
	/// * rois: Region of interest rectangles.
	/// * pano: Final pano.
	/// ## Returns
	/// Status code.
	fn stitch_rois(&mut self, images: &dyn core::ToInputArray, rois: &core::Vector::<core::Vector::<core::Rect>>, pano: &mut dyn core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		output_array_arg!(pano);
		unsafe { sys::cv_Stitcher_stitch_const__InputArrayR_const_vector_vector_Rect__R_const__OutputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), rois.as_raw_VectorOfVectorOfRect(), pano.as_raw__OutputArray()) }.into_result()
	}
	
	fn component(&self) -> Result<core::Vector::<i32>> {
		unsafe { sys::cv_Stitcher_component_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
	}
	
	fn cameras(&self) -> Result<core::Vector::<crate::stitching::Detail_CameraParams>> {
		unsafe { sys::cv_Stitcher_cameras_const(self.as_raw_Stitcher()) }.into_result().map(|r| unsafe { core::Vector::<crate::stitching::Detail_CameraParams>::opencv_from_extern(r) } )
	}
	
	fn work_scale(&self) -> Result<f64> {
		unsafe { sys::cv_Stitcher_workScale_const(self.as_raw_Stitcher()) }.into_result()
	}
	
}

/// High level image stitcher.
/// 
/// It's possible to use this class without being aware of the entire stitching pipeline. However, to
/// be able to achieve higher stitching stability and quality of the final images at least being
/// familiar with the theory is recommended.
/// 
/// 
/// Note:
///    *   A basic example on image stitching can be found at
///        opencv_source_code/samples/cpp/stitching.cpp
///    *   A detailed example on image stitching can be found at
///        opencv_source_code/samples/cpp/stitching_detailed.cpp
pub struct Stitcher {
	ptr: *mut c_void
}

opencv_type_boxed! { Stitcher }

impl Drop for Stitcher {
	fn drop(&mut self) {
		extern "C" { fn cv_Stitcher_delete(instance: *mut c_void); }
		unsafe { cv_Stitcher_delete(self.as_raw_mut_Stitcher()) };
	}
}

impl Stitcher {
	#[inline] pub fn as_raw_Stitcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Stitcher {}

impl crate::stitching::StitcherTrait for Stitcher {
	#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Stitcher {
	/// Creates a stitcher with the default parameters.
	/// 
	/// ## Parameters
	/// * try_use_gpu: Flag indicating whether GPU should be used whenever it's possible.
	/// ## Returns
	/// Stitcher class instance.
	/// 
	/// ## C++ default parameters
	/// * try_use_gpu: false
	pub fn create_default(try_use_gpu: bool) -> Result<crate::stitching::Stitcher> {
		unsafe { sys::cv_Stitcher_createDefault_bool(try_use_gpu) }.into_result().map(|r| unsafe { crate::stitching::Stitcher::opencv_from_extern(r) } )
	}
	
	/// Creates a Stitcher configured in one of the stitching modes.
	/// 
	/// ## Parameters
	/// * mode: Scenario for stitcher operation. This is usually determined by source of images
	/// to stitch and their transformation. Default parameters will be chosen for operation in given
	/// scenario.
	/// * try_use_gpu: Flag indicating whether GPU should be used whenever it's possible.
	/// ## Returns
	/// Stitcher class instance.
	/// 
	/// ## C++ default parameters
	/// * mode: PANORAMA
	/// * try_use_gpu: false
	pub fn create(mode: crate::stitching::Stitcher_Mode, try_use_gpu: bool) -> Result<core::Ptr::<crate::stitching::Stitcher>> {
		unsafe { sys::cv_Stitcher_create_Mode_bool(mode, try_use_gpu) }.into_result().map(|r| unsafe { core::Ptr::<crate::stitching::Stitcher>::opencv_from_extern(r) } )
	}
	
}

pub trait TransverseMercatorWarperTrait: crate::stitching::WarperCreator {
	fn as_raw_TransverseMercatorWarper(&self) -> *const c_void;
	fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_TransverseMercatorWarper_create_const_float(self.as_raw_TransverseMercatorWarper(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

pub struct TransverseMercatorWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { TransverseMercatorWarper }

impl Drop for TransverseMercatorWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_TransverseMercatorWarper_delete(instance: *mut c_void); }
		unsafe { cv_TransverseMercatorWarper_delete(self.as_raw_mut_TransverseMercatorWarper()) };
	}
}

impl TransverseMercatorWarper {
	#[inline] pub fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TransverseMercatorWarper {}

impl crate::stitching::TransverseMercatorWarperTrait for TransverseMercatorWarper {
	#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::WarperCreator for TransverseMercatorWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TransverseMercatorWarper {
}

/// Image warper factories base class.
pub trait WarperCreator {
	fn as_raw_WarperCreator(&self) -> *const c_void;
	fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void;

	fn create(&self, scale: f32) -> Result<core::Ptr::<dyn crate::stitching::Detail_RotationWarper>> {
		unsafe { sys::cv_WarperCreator_create_const_float(self.as_raw_WarperCreator(), scale) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_RotationWarper>::opencv_from_extern(r) } )
	}
	
}

/// AKAZE features finder. :
/// ## See also
/// detail::FeaturesFinder, AKAZE
pub trait Detail_AKAZEFeaturesFinderTrait: crate::stitching::Detail_FeaturesFinder {
	fn as_raw_Detail_AKAZEFeaturesFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_AKAZEFeaturesFinder(&mut self) -> *mut c_void;

}

/// AKAZE features finder. :
/// ## See also
/// detail::FeaturesFinder, AKAZE
pub struct Detail_AKAZEFeaturesFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_AKAZEFeaturesFinder }

impl Drop for Detail_AKAZEFeaturesFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_AKAZEFeaturesFinder_delete(instance: *mut c_void); }
		unsafe { cv_Detail_AKAZEFeaturesFinder_delete(self.as_raw_mut_Detail_AKAZEFeaturesFinder()) };
	}
}

impl Detail_AKAZEFeaturesFinder {
	#[inline] pub fn as_raw_Detail_AKAZEFeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_AKAZEFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_AKAZEFeaturesFinder {}

impl crate::stitching::Detail_AKAZEFeaturesFinderTrait for Detail_AKAZEFeaturesFinder {
	#[inline] fn as_raw_Detail_AKAZEFeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_AKAZEFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesFinder for Detail_AKAZEFeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_AKAZEFeaturesFinder {
	/// ## C++ default parameters
	/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
	/// * descriptor_size: 0
	/// * descriptor_channels: 3
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	pub fn new(descriptor_type: i32, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32) -> Result<crate::stitching::Detail_AKAZEFeaturesFinder> {
		unsafe { sys::cv_detail_AKAZEFeaturesFinder_AKAZEFeaturesFinder_int_int_int_float_int_int_int(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity) }.into_result().map(|r| unsafe { crate::stitching::Detail_AKAZEFeaturesFinder::opencv_from_extern(r) } )
	}
	
}

/// Affine transformation based estimator.
/// 
/// This estimator uses pairwise tranformations estimated by matcher to estimate
/// final transformation for each camera.
/// ## See also
/// cv::detail::HomographyBasedEstimator
pub trait Detail_AffineBasedEstimatorTrait: crate::stitching::Detail_Estimator {
	fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void;
	fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void;

}

/// Affine transformation based estimator.
/// 
/// This estimator uses pairwise tranformations estimated by matcher to estimate
/// final transformation for each camera.
/// ## See also
/// cv::detail::HomographyBasedEstimator
pub struct Detail_AffineBasedEstimator {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_AffineBasedEstimator }

impl Drop for Detail_AffineBasedEstimator {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_AffineBasedEstimator_delete(instance: *mut c_void); }
		unsafe { cv_Detail_AffineBasedEstimator_delete(self.as_raw_mut_Detail_AffineBasedEstimator()) };
	}
}

impl Detail_AffineBasedEstimator {
	#[inline] pub fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_AffineBasedEstimator {}

impl crate::stitching::Detail_AffineBasedEstimatorTrait for Detail_AffineBasedEstimator {
	#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_Estimator for Detail_AffineBasedEstimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_AffineBasedEstimator {
}

/// Features matcher similar to cv::detail::BestOf2NearestMatcher which
/// finds two best matches for each feature and leaves the best one only if the
/// ratio between descriptor distances is greater than the threshold match_conf.
/// 
/// Unlike cv::detail::BestOf2NearestMatcher this matcher uses affine
/// transformation (affine trasformation estimate will be placed in matches_info).
/// ## See also
/// cv::detail::FeaturesMatcher cv::detail::BestOf2NearestMatcher
pub trait Detail_AffineBestOf2NearestMatcherTrait: crate::stitching::Detail_BestOf2NearestMatcherTrait {
	fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void;
	fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void;

}

/// Features matcher similar to cv::detail::BestOf2NearestMatcher which
/// finds two best matches for each feature and leaves the best one only if the
/// ratio between descriptor distances is greater than the threshold match_conf.
/// 
/// Unlike cv::detail::BestOf2NearestMatcher this matcher uses affine
/// transformation (affine trasformation estimate will be placed in matches_info).
/// ## See also
/// cv::detail::FeaturesMatcher cv::detail::BestOf2NearestMatcher
pub struct Detail_AffineBestOf2NearestMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_AffineBestOf2NearestMatcher }

impl Drop for Detail_AffineBestOf2NearestMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_AffineBestOf2NearestMatcher_delete(instance: *mut c_void); }
		unsafe { cv_Detail_AffineBestOf2NearestMatcher_delete(self.as_raw_mut_Detail_AffineBestOf2NearestMatcher()) };
	}
}

impl Detail_AffineBestOf2NearestMatcher {
	#[inline] pub fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_AffineBestOf2NearestMatcher {}

impl crate::stitching::Detail_AffineBestOf2NearestMatcherTrait for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesMatcher for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_AffineBestOf2NearestMatcher {
	/// Constructs a "best of 2 nearest" matcher that expects affine trasformation
	/// between images
	/// 
	/// ## Parameters
	/// * full_affine: whether to use full affine transformation with 6 degress of freedom or reduced
	/// transformation with 4 degrees of freedom using only rotation, translation and uniform scaling
	/// * try_use_gpu: Should try to use GPU or not
	/// * match_conf: Match distances ration threshold
	/// * num_matches_thresh1: Minimum number of matches required for the 2D affine transform
	/// estimation used in the inliers classification step
	/// ## See also
	/// cv::estimateAffine2D cv::estimateAffinePartial2D
	/// 
	/// ## C++ default parameters
	/// * full_affine: false
	/// * try_use_gpu: false
	/// * match_conf: 0.3f
	/// * num_matches_thresh1: 6
	pub fn new(full_affine: bool, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32) -> Result<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		unsafe { sys::cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(full_affine, try_use_gpu, match_conf, num_matches_thresh1) }.into_result().map(|r| unsafe { crate::stitching::Detail_AffineBestOf2NearestMatcher::opencv_from_extern(r) } )
	}
	
}

/// Affine warper that uses rotations and translations
/// 
/// Uses affine transformation in homogeneous coordinates to represent both rotation and
/// translation in camera rotation matrix.
pub trait Detail_AffineWarperTrait: crate::stitching::Detail_PlaneWarperTrait {
	fn as_raw_Detail_AffineWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_AffineWarper(&mut self) -> *mut c_void;

	fn warp_point(&mut self, pt: core::Point2f, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_AffineWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
	}
	
	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_AffineWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_AffineWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_AffineWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp_roi(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_AffineWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
	}
	
}

/// Affine warper that uses rotations and translations
/// 
/// Uses affine transformation in homogeneous coordinates to represent both rotation and
/// translation in camera rotation matrix.
pub struct Detail_AffineWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_AffineWarper }

impl Drop for Detail_AffineWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_AffineWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_AffineWarper_delete(self.as_raw_mut_Detail_AffineWarper()) };
	}
}

impl Detail_AffineWarper {
	#[inline] pub fn as_raw_Detail_AffineWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_AffineWarper {}

impl crate::stitching::Detail_AffineWarperTrait for Detail_AffineWarper {
	#[inline] fn as_raw_Detail_AffineWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_PlaneWarperTrait for Detail_AffineWarper {
	#[inline] fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_AffineWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_AffineWarper {
	/// Construct an instance of the affine warper class.
	/// 
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	/// 
	/// ## C++ default parameters
	/// * scale: 1.f
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_AffineWarper> {
		unsafe { sys::cv_detail_AffineWarper_AffineWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_AffineWarper::opencv_from_extern(r) } )
	}
	
}

/// Features matcher which finds two best matches for each feature and leaves the best one only if the
/// ratio between descriptor distances is greater than the threshold match_conf
/// ## See also
/// detail::FeaturesMatcher
pub trait Detail_BestOf2NearestMatcherTrait: crate::stitching::Detail_FeaturesMatcher {
	fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void;
	fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void;

	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_detail_BestOf2NearestMatcher_collectGarbage(self.as_raw_mut_Detail_BestOf2NearestMatcher()) }.into_result()
	}
	
}

/// Features matcher which finds two best matches for each feature and leaves the best one only if the
/// ratio between descriptor distances is greater than the threshold match_conf
/// ## See also
/// detail::FeaturesMatcher
pub struct Detail_BestOf2NearestMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_BestOf2NearestMatcher }

impl Drop for Detail_BestOf2NearestMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_BestOf2NearestMatcher_delete(instance: *mut c_void); }
		unsafe { cv_Detail_BestOf2NearestMatcher_delete(self.as_raw_mut_Detail_BestOf2NearestMatcher()) };
	}
}

impl Detail_BestOf2NearestMatcher {
	#[inline] pub fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_BestOf2NearestMatcher {}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for Detail_BestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesMatcher for Detail_BestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_BestOf2NearestMatcher {
	/// Constructs a "best of 2 nearest" matcher.
	/// 
	/// ## Parameters
	/// * try_use_gpu: Should try to use GPU or not
	/// * match_conf: Match distances ration threshold
	/// * num_matches_thresh1: Minimum number of matches required for the 2D projective transform
	/// estimation used in the inliers classification step
	/// * num_matches_thresh2: Minimum number of matches required for the 2D projective transform
	/// re-estimation on inliers
	/// 
	/// ## C++ default parameters
	/// * try_use_gpu: false
	/// * match_conf: 0.3f
	/// * num_matches_thresh1: 6
	/// * num_matches_thresh2: 6
	pub fn new(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32) -> Result<crate::stitching::Detail_BestOf2NearestMatcher> {
		unsafe { sys::cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2) }.into_result().map(|r| unsafe { crate::stitching::Detail_BestOf2NearestMatcher::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_BestOf2NearestRangeMatcherTrait: crate::stitching::Detail_BestOf2NearestMatcherTrait {
	fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void;
	fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void;

}

pub struct Detail_BestOf2NearestRangeMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_BestOf2NearestRangeMatcher }

impl Drop for Detail_BestOf2NearestRangeMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_BestOf2NearestRangeMatcher_delete(instance: *mut c_void); }
		unsafe { cv_Detail_BestOf2NearestRangeMatcher_delete(self.as_raw_mut_Detail_BestOf2NearestRangeMatcher()) };
	}
}

impl Detail_BestOf2NearestRangeMatcher {
	#[inline] pub fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_BestOf2NearestRangeMatcher {}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BestOf2NearestRangeMatcherTrait for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesMatcher for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_BestOf2NearestRangeMatcher {
	/// ## C++ default parameters
	/// * range_width: 5
	/// * try_use_gpu: false
	/// * match_conf: 0.3f
	/// * num_matches_thresh1: 6
	/// * num_matches_thresh2: 6
	pub fn new(range_width: i32, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32) -> Result<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		unsafe { sys::cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(range_width, try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2) }.into_result().map(|r| unsafe { crate::stitching::Detail_BestOf2NearestRangeMatcher::opencv_from_extern(r) } )
	}
	
}

/// Base class for all blenders.
/// 
/// Simple blender which puts one image over another
pub trait Detail_BlenderTrait {
	fn as_raw_Detail_Blender(&self) -> *const c_void;
	fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void;

	/// Prepares the blender for blending.
	/// 
	/// ## Parameters
	/// * corners: Source images top-left corners
	/// * sizes: Source image sizes
	fn prepare(&mut self, corners: &core::Vector::<core::Point>, sizes: &core::Vector::<core::Size>) -> Result<()> {
		unsafe { sys::cv_detail_Blender_prepare_const_vector_Point_R_const_vector_Size_R(self.as_raw_mut_Detail_Blender(), corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize()) }.into_result()
	}
	
	/// Prepares the blender for blending.
	/// 
	/// ## Parameters
	/// * corners: Source images top-left corners
	/// * sizes: Source image sizes
	/// 
	/// ## Overloaded parameters
	fn prepare_1(&mut self, dst_roi: core::Rect) -> Result<()> {
		unsafe { sys::cv_detail_Blender_prepare_Rect(self.as_raw_mut_Detail_Blender(), dst_roi.opencv_as_extern()) }.into_result()
	}
	
	/// Processes the image.
	/// 
	/// ## Parameters
	/// * img: Source image
	/// * mask: Source image mask
	/// * tl: Source image top-left corners
	fn feed(&mut self, img: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, tl: core::Point) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(mask);
		unsafe { sys::cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_Blender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), tl.opencv_as_extern()) }.into_result()
	}
	
	/// Blends and returns the final pano.
	/// 
	/// ## Parameters
	/// * dst: Final pano
	/// * dst_mask: Final pano mask
	fn blend(&mut self, dst: &mut dyn core::ToInputOutputArray, dst_mask: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(dst);
		input_output_array_arg!(dst_mask);
		unsafe { sys::cv_detail_Blender_blend_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Detail_Blender(), dst.as_raw__InputOutputArray(), dst_mask.as_raw__InputOutputArray()) }.into_result()
	}
	
}

/// Base class for all blenders.
/// 
/// Simple blender which puts one image over another
pub struct Detail_Blender {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_Blender }

impl Drop for Detail_Blender {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_Blender_delete(instance: *mut c_void); }
		unsafe { cv_Detail_Blender_delete(self.as_raw_mut_Detail_Blender()) };
	}
}

impl Detail_Blender {
	#[inline] pub fn as_raw_Detail_Blender(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_Blender {}

impl crate::stitching::Detail_BlenderTrait for Detail_Blender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_Blender {
	/// ## C++ default parameters
	/// * try_gpu: false
	pub fn create_default(typ: i32, try_gpu: bool) -> Result<core::Ptr::<crate::stitching::Detail_Blender>> {
		unsafe { sys::cv_detail_Blender_createDefault_int_bool(typ, try_gpu) }.into_result().map(|r| unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(r) } )
	}
	
}

/// Exposure compensator which tries to remove exposure related artifacts by adjusting image block
/// intensities, see [UES01](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_UES01) for details.
pub trait Detail_BlocksGainCompensatorTrait: crate::stitching::Detail_ExposureCompensator {
	fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void;
	fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void;

	fn apply(&mut self, index: i32, corner: core::Point, image: &mut dyn core::ToInputOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_BlocksGainCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
}

/// Exposure compensator which tries to remove exposure related artifacts by adjusting image block
/// intensities, see [UES01](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_UES01) for details.
pub struct Detail_BlocksGainCompensator {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_BlocksGainCompensator }

impl Drop for Detail_BlocksGainCompensator {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_BlocksGainCompensator_delete(instance: *mut c_void); }
		unsafe { cv_Detail_BlocksGainCompensator_delete(self.as_raw_mut_Detail_BlocksGainCompensator()) };
	}
}

impl Detail_BlocksGainCompensator {
	#[inline] pub fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_BlocksGainCompensator {}

impl crate::stitching::Detail_BlocksGainCompensatorTrait for Detail_BlocksGainCompensator {
	#[inline] fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensator for Detail_BlocksGainCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_BlocksGainCompensator {
	/// ## C++ default parameters
	/// * bl_width: 32
	/// * bl_height: 32
	pub fn new(bl_width: i32, bl_height: i32) -> Result<crate::stitching::Detail_BlocksGainCompensator> {
		unsafe { sys::cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(bl_width, bl_height) }.into_result().map(|r| unsafe { crate::stitching::Detail_BlocksGainCompensator::opencv_from_extern(r) } )
	}
	
}

/// Bundle adjuster that expects affine transformation
/// represented in homogeneous coordinates in R for each camera param. Implements
/// camera parameters refinement algorithm which minimizes sum of the reprojection
/// error squares
/// 
/// It estimates all transformation parameters. Refinement mask is ignored.
/// ## See also
/// AffineBasedEstimator AffineBestOf2NearestMatcher BundleAdjusterAffinePartial
pub trait Detail_BundleAdjusterAffineTrait: crate::stitching::Detail_BundleAdjusterBase {
	fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void;
	fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void;

}

/// Bundle adjuster that expects affine transformation
/// represented in homogeneous coordinates in R for each camera param. Implements
/// camera parameters refinement algorithm which minimizes sum of the reprojection
/// error squares
/// 
/// It estimates all transformation parameters. Refinement mask is ignored.
/// ## See also
/// AffineBasedEstimator AffineBestOf2NearestMatcher BundleAdjusterAffinePartial
pub struct Detail_BundleAdjusterAffine {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_BundleAdjusterAffine }

impl Drop for Detail_BundleAdjusterAffine {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_BundleAdjusterAffine_delete(instance: *mut c_void); }
		unsafe { cv_Detail_BundleAdjusterAffine_delete(self.as_raw_mut_Detail_BundleAdjusterAffine()) };
	}
}

impl Detail_BundleAdjusterAffine {
	#[inline] pub fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_BundleAdjusterAffine {}

impl crate::stitching::Detail_BundleAdjusterAffineTrait for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBase for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_Estimator for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_BundleAdjusterAffine {
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterAffine> {
		unsafe { sys::cv_detail_BundleAdjusterAffine_BundleAdjusterAffine() }.into_result().map(|r| unsafe { crate::stitching::Detail_BundleAdjusterAffine::opencv_from_extern(r) } )
	}
	
}

/// Bundle adjuster that expects affine transformation with 4 DOF
/// represented in homogeneous coordinates in R for each camera param. Implements
/// camera parameters refinement algorithm which minimizes sum of the reprojection
/// error squares
/// 
/// It estimates all transformation parameters. Refinement mask is ignored.
/// ## See also
/// AffineBasedEstimator AffineBestOf2NearestMatcher BundleAdjusterAffine
pub trait Detail_BundleAdjusterAffinePartialTrait: crate::stitching::Detail_BundleAdjusterBase {
	fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void;
	fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void;

}

/// Bundle adjuster that expects affine transformation with 4 DOF
/// represented in homogeneous coordinates in R for each camera param. Implements
/// camera parameters refinement algorithm which minimizes sum of the reprojection
/// error squares
/// 
/// It estimates all transformation parameters. Refinement mask is ignored.
/// ## See also
/// AffineBasedEstimator AffineBestOf2NearestMatcher BundleAdjusterAffine
pub struct Detail_BundleAdjusterAffinePartial {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_BundleAdjusterAffinePartial }

impl Drop for Detail_BundleAdjusterAffinePartial {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_BundleAdjusterAffinePartial_delete(instance: *mut c_void); }
		unsafe { cv_Detail_BundleAdjusterAffinePartial_delete(self.as_raw_mut_Detail_BundleAdjusterAffinePartial()) };
	}
}

impl Detail_BundleAdjusterAffinePartial {
	#[inline] pub fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_BundleAdjusterAffinePartial {}

impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBase for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_Estimator for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_BundleAdjusterAffinePartial {
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		unsafe { sys::cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial() }.into_result().map(|r| unsafe { crate::stitching::Detail_BundleAdjusterAffinePartial::opencv_from_extern(r) } )
	}
	
}

/// Base class for all camera parameters refinement methods.
pub trait Detail_BundleAdjusterBase: crate::stitching::Detail_Estimator {
	fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void;
	fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void;

	fn refinement_mask(&self) -> Result<core::Mat> {
		unsafe { sys::cv_detail_BundleAdjusterBase_refinementMask_const(self.as_raw_Detail_BundleAdjusterBase()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn set_refinement_mask(&mut self, mask: &core::Mat) -> Result<()> {
		unsafe { sys::cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(self.as_raw_mut_Detail_BundleAdjusterBase(), mask.as_raw_Mat()) }.into_result()
	}
	
	fn conf_thresh(&self) -> Result<f64> {
		unsafe { sys::cv_detail_BundleAdjusterBase_confThresh_const(self.as_raw_Detail_BundleAdjusterBase()) }.into_result()
	}
	
	fn set_conf_thresh(&mut self, conf_thresh: f64) -> Result<()> {
		unsafe { sys::cv_detail_BundleAdjusterBase_setConfThresh_double(self.as_raw_mut_Detail_BundleAdjusterBase(), conf_thresh) }.into_result()
	}
	
	fn term_criteria(&mut self) -> Result<core::TermCriteria> {
		unsafe { sys::cv_detail_BundleAdjusterBase_termCriteria(self.as_raw_mut_Detail_BundleAdjusterBase()) }.into_result()
	}
	
	fn set_term_criteria(&mut self, term_criteria: core::TermCriteria) -> Result<()> {
		unsafe { sys::cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaR(self.as_raw_mut_Detail_BundleAdjusterBase(), &term_criteria) }.into_result()
	}
	
}

/// Implementation of the camera parameters refinement algorithm which minimizes sum of the distances
/// between the rays passing through the camera center and a feature. :
/// 
/// It can estimate focal length. It ignores the refinement mask for now.
pub trait Detail_BundleAdjusterRayTrait: crate::stitching::Detail_BundleAdjusterBase {
	fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void;
	fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void;

}

/// Implementation of the camera parameters refinement algorithm which minimizes sum of the distances
/// between the rays passing through the camera center and a feature. :
/// 
/// It can estimate focal length. It ignores the refinement mask for now.
pub struct Detail_BundleAdjusterRay {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_BundleAdjusterRay }

impl Drop for Detail_BundleAdjusterRay {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_BundleAdjusterRay_delete(instance: *mut c_void); }
		unsafe { cv_Detail_BundleAdjusterRay_delete(self.as_raw_mut_Detail_BundleAdjusterRay()) };
	}
}

impl Detail_BundleAdjusterRay {
	#[inline] pub fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_BundleAdjusterRay {}

impl crate::stitching::Detail_BundleAdjusterBase for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterRayTrait for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_Estimator for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_BundleAdjusterRay {
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterRay> {
		unsafe { sys::cv_detail_BundleAdjusterRay_BundleAdjusterRay() }.into_result().map(|r| unsafe { crate::stitching::Detail_BundleAdjusterRay::opencv_from_extern(r) } )
	}
	
}

/// Implementation of the camera parameters refinement algorithm which minimizes sum of the reprojection
/// error squares
/// 
/// It can estimate focal length, aspect ratio, principal point.
/// You can affect only on them via the refinement mask.
pub trait Detail_BundleAdjusterReprojTrait: crate::stitching::Detail_BundleAdjusterBase {
	fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void;
	fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void;

}

/// Implementation of the camera parameters refinement algorithm which minimizes sum of the reprojection
/// error squares
/// 
/// It can estimate focal length, aspect ratio, principal point.
/// You can affect only on them via the refinement mask.
pub struct Detail_BundleAdjusterReproj {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_BundleAdjusterReproj }

impl Drop for Detail_BundleAdjusterReproj {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_BundleAdjusterReproj_delete(instance: *mut c_void); }
		unsafe { cv_Detail_BundleAdjusterReproj_delete(self.as_raw_mut_Detail_BundleAdjusterReproj()) };
	}
}

impl Detail_BundleAdjusterReproj {
	#[inline] pub fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_BundleAdjusterReproj {}

impl crate::stitching::Detail_BundleAdjusterBase for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterReprojTrait for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_Estimator for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_BundleAdjusterReproj {
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterReproj> {
		unsafe { sys::cv_detail_BundleAdjusterReproj_BundleAdjusterReproj() }.into_result().map(|r| unsafe { crate::stitching::Detail_BundleAdjusterReproj::opencv_from_extern(r) } )
	}
	
}

/// Describes camera parameters.
/// 
/// 
/// Note: Translation is assumed to be zero during the whole stitching pipeline. :
pub trait Detail_CameraParamsTrait {
	fn as_raw_Detail_CameraParams(&self) -> *const c_void;
	fn as_raw_mut_Detail_CameraParams(&mut self) -> *mut c_void;

	fn focal(&self) -> f64 {
		unsafe { sys::cv_detail_CameraParams_getPropFocal_const(self.as_raw_Detail_CameraParams()) }.into_result().expect("Infallible function failed: focal")
	}
	
	fn set_focal(&mut self, val: f64) -> () {
		unsafe { sys::cv_detail_CameraParams_setPropFocal_double(self.as_raw_mut_Detail_CameraParams(), val) }.into_result().expect("Infallible function failed: set_focal")
	}
	
	fn aspect(&self) -> f64 {
		unsafe { sys::cv_detail_CameraParams_getPropAspect_const(self.as_raw_Detail_CameraParams()) }.into_result().expect("Infallible function failed: aspect")
	}
	
	fn set_aspect(&mut self, val: f64) -> () {
		unsafe { sys::cv_detail_CameraParams_setPropAspect_double(self.as_raw_mut_Detail_CameraParams(), val) }.into_result().expect("Infallible function failed: set_aspect")
	}
	
	fn ppx(&self) -> f64 {
		unsafe { sys::cv_detail_CameraParams_getPropPpx_const(self.as_raw_Detail_CameraParams()) }.into_result().expect("Infallible function failed: ppx")
	}
	
	fn set_ppx(&mut self, val: f64) -> () {
		unsafe { sys::cv_detail_CameraParams_setPropPpx_double(self.as_raw_mut_Detail_CameraParams(), val) }.into_result().expect("Infallible function failed: set_ppx")
	}
	
	fn ppy(&self) -> f64 {
		unsafe { sys::cv_detail_CameraParams_getPropPpy_const(self.as_raw_Detail_CameraParams()) }.into_result().expect("Infallible function failed: ppy")
	}
	
	fn set_ppy(&mut self, val: f64) -> () {
		unsafe { sys::cv_detail_CameraParams_setPropPpy_double(self.as_raw_mut_Detail_CameraParams(), val) }.into_result().expect("Infallible function failed: set_ppy")
	}
	
	fn r(&mut self) -> core::Mat {
		unsafe { sys::cv_detail_CameraParams_getPropR(self.as_raw_mut_Detail_CameraParams()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: r")
	}
	
	fn set_r(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_detail_CameraParams_setPropR_Mat(self.as_raw_mut_Detail_CameraParams(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_r")
	}
	
	fn t(&mut self) -> core::Mat {
		unsafe { sys::cv_detail_CameraParams_getPropT(self.as_raw_mut_Detail_CameraParams()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: t")
	}
	
	fn set_t(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_detail_CameraParams_setPropT_Mat(self.as_raw_mut_Detail_CameraParams(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_t")
	}
	
	fn k(&self) -> Result<core::Mat> {
		unsafe { sys::cv_detail_CameraParams_K_const(self.as_raw_Detail_CameraParams()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Describes camera parameters.
/// 
/// 
/// Note: Translation is assumed to be zero during the whole stitching pipeline. :
pub struct Detail_CameraParams {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CameraParams }

impl Drop for Detail_CameraParams {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CameraParams_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CameraParams_delete(self.as_raw_mut_Detail_CameraParams()) };
	}
}

impl Detail_CameraParams {
	#[inline] pub fn as_raw_Detail_CameraParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CameraParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CameraParams {}

impl crate::stitching::Detail_CameraParamsTrait for Detail_CameraParams {
	#[inline] fn as_raw_Detail_CameraParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CameraParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CameraParams {
	pub fn default() -> Result<crate::stitching::Detail_CameraParams> {
		unsafe { sys::cv_detail_CameraParams_CameraParams() }.into_result().map(|r| unsafe { crate::stitching::Detail_CameraParams::opencv_from_extern(r) } )
	}
	
	pub fn copy(other: &crate::stitching::Detail_CameraParams) -> Result<crate::stitching::Detail_CameraParams> {
		unsafe { sys::cv_detail_CameraParams_CameraParams_const_CameraParamsR(other.as_raw_Detail_CameraParams()) }.into_result().map(|r| unsafe { crate::stitching::Detail_CameraParams::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_CompressedRectilinearPortraitProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_CompressedRectilinearPortraitProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_CompressedRectilinearPortraitProjector(&mut self) -> *mut c_void;

	fn a(&self) -> f32 {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_getPropA_const(self.as_raw_Detail_CompressedRectilinearPortraitProjector()) }.into_result().expect("Infallible function failed: a")
	}
	
	fn set_a(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_setPropA_float(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), val) }.into_result().expect("Infallible function failed: set_a")
	}
	
	fn b(&self) -> f32 {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_getPropB_const(self.as_raw_Detail_CompressedRectilinearPortraitProjector()) }.into_result().expect("Infallible function failed: b")
	}
	
	fn set_b(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_setPropB_float(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), val) }.into_result().expect("Infallible function failed: set_b")
	}
	
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_CompressedRectilinearPortraitProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CompressedRectilinearPortraitProjector }

impl Drop for Detail_CompressedRectilinearPortraitProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CompressedRectilinearPortraitProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CompressedRectilinearPortraitProjector_delete(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector()) };
	}
}

impl Detail_CompressedRectilinearPortraitProjector {
	#[inline] pub fn as_raw_Detail_CompressedRectilinearPortraitProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CompressedRectilinearPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CompressedRectilinearPortraitProjector {}

impl crate::stitching::Detail_CompressedRectilinearPortraitProjectorTrait for Detail_CompressedRectilinearPortraitProjector {
	#[inline] fn as_raw_Detail_CompressedRectilinearPortraitProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_CompressedRectilinearPortraitProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CompressedRectilinearPortraitProjector {
}

pub trait Detail_CompressedRectilinearPortraitWarperTrait {
	fn as_raw_Detail_CompressedRectilinearPortraitWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void;

}

pub struct Detail_CompressedRectilinearPortraitWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CompressedRectilinearPortraitWarper }

impl Drop for Detail_CompressedRectilinearPortraitWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CompressedRectilinearPortraitWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CompressedRectilinearPortraitWarper_delete(self.as_raw_mut_Detail_CompressedRectilinearPortraitWarper()) };
	}
}

impl Detail_CompressedRectilinearPortraitWarper {
	#[inline] pub fn as_raw_Detail_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CompressedRectilinearPortraitWarper {}

impl crate::stitching::Detail_CompressedRectilinearPortraitWarperTrait for Detail_CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_Detail_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CompressedRectilinearPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_CompressedRectilinearPortraitWarper> {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(scale, a, b) }.into_result().map(|r| unsafe { crate::stitching::Detail_CompressedRectilinearPortraitWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_CompressedRectilinearProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_CompressedRectilinearProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_CompressedRectilinearProjector(&mut self) -> *mut c_void;

	fn a(&self) -> f32 {
		unsafe { sys::cv_detail_CompressedRectilinearProjector_getPropA_const(self.as_raw_Detail_CompressedRectilinearProjector()) }.into_result().expect("Infallible function failed: a")
	}
	
	fn set_a(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_CompressedRectilinearProjector_setPropA_float(self.as_raw_mut_Detail_CompressedRectilinearProjector(), val) }.into_result().expect("Infallible function failed: set_a")
	}
	
	fn b(&self) -> f32 {
		unsafe { sys::cv_detail_CompressedRectilinearProjector_getPropB_const(self.as_raw_Detail_CompressedRectilinearProjector()) }.into_result().expect("Infallible function failed: b")
	}
	
	fn set_b(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_CompressedRectilinearProjector_setPropB_float(self.as_raw_mut_Detail_CompressedRectilinearProjector(), val) }.into_result().expect("Infallible function failed: set_b")
	}
	
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_CompressedRectilinearProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CompressedRectilinearProjector }

impl Drop for Detail_CompressedRectilinearProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CompressedRectilinearProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CompressedRectilinearProjector_delete(self.as_raw_mut_Detail_CompressedRectilinearProjector()) };
	}
}

impl Detail_CompressedRectilinearProjector {
	#[inline] pub fn as_raw_Detail_CompressedRectilinearProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CompressedRectilinearProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CompressedRectilinearProjector {}

impl crate::stitching::Detail_CompressedRectilinearProjectorTrait for Detail_CompressedRectilinearProjector {
	#[inline] fn as_raw_Detail_CompressedRectilinearProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_CompressedRectilinearProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CompressedRectilinearProjector {
}

pub trait Detail_CompressedRectilinearWarperTrait {
	fn as_raw_Detail_CompressedRectilinearWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_CompressedRectilinearWarper(&mut self) -> *mut c_void;

}

pub struct Detail_CompressedRectilinearWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CompressedRectilinearWarper }

impl Drop for Detail_CompressedRectilinearWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CompressedRectilinearWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CompressedRectilinearWarper_delete(self.as_raw_mut_Detail_CompressedRectilinearWarper()) };
	}
}

impl Detail_CompressedRectilinearWarper {
	#[inline] pub fn as_raw_Detail_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CompressedRectilinearWarper {}

impl crate::stitching::Detail_CompressedRectilinearWarperTrait for Detail_CompressedRectilinearWarper {
	#[inline] fn as_raw_Detail_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_CompressedRectilinearWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CompressedRectilinearWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_CompressedRectilinearWarper> {
		unsafe { sys::cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(scale, a, b) }.into_result().map(|r| unsafe { crate::stitching::Detail_CompressedRectilinearWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_CylindricalPortraitWarperTrait {
	fn as_raw_Detail_CylindricalPortraitWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_CylindricalPortraitWarper(&mut self) -> *mut c_void;

}

pub struct Detail_CylindricalPortraitWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CylindricalPortraitWarper }

impl Drop for Detail_CylindricalPortraitWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CylindricalPortraitWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CylindricalPortraitWarper_delete(self.as_raw_mut_Detail_CylindricalPortraitWarper()) };
	}
}

impl Detail_CylindricalPortraitWarper {
	#[inline] pub fn as_raw_Detail_CylindricalPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CylindricalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CylindricalPortraitWarper {}

impl crate::stitching::Detail_CylindricalPortraitWarperTrait for Detail_CylindricalPortraitWarper {
	#[inline] fn as_raw_Detail_CylindricalPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CylindricalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_CylindricalPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CylindricalPortraitWarper {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_CylindricalPortraitWarper> {
		unsafe { sys::cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_CylindricalPortraitWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_CylindricalProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_CylindricalProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_CylindricalProjector(&mut self) -> *mut c_void;

	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_CylindricalProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CylindricalProjector }

impl Drop for Detail_CylindricalProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CylindricalProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CylindricalProjector_delete(self.as_raw_mut_Detail_CylindricalProjector()) };
	}
}

impl Detail_CylindricalProjector {
	#[inline] pub fn as_raw_Detail_CylindricalProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CylindricalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CylindricalProjector {}

impl crate::stitching::Detail_CylindricalProjectorTrait for Detail_CylindricalProjector {
	#[inline] fn as_raw_Detail_CylindricalProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CylindricalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_CylindricalProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CylindricalProjector {
}

/// Warper that maps an image onto the x\*x + z\*z = 1 cylinder.
pub trait Detail_CylindricalWarperTrait {
	fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void;

	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_CylindricalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
}

/// Warper that maps an image onto the x\*x + z\*z = 1 cylinder.
pub struct Detail_CylindricalWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CylindricalWarper }

impl Drop for Detail_CylindricalWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CylindricalWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CylindricalWarper_delete(self.as_raw_mut_Detail_CylindricalWarper()) };
	}
}

impl Detail_CylindricalWarper {
	#[inline] pub fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CylindricalWarper {}

impl crate::stitching::Detail_CylindricalWarperTrait for Detail_CylindricalWarper {
	#[inline] fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_CylindricalWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CylindricalWarper {
	/// Construct an instance of the cylindrical warper class.
	/// 
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_CylindricalWarper> {
		unsafe { sys::cv_detail_CylindricalWarper_CylindricalWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_CylindricalWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_CylindricalWarperGpuTrait: crate::stitching::Detail_CylindricalWarperTrait {
	fn as_raw_Detail_CylindricalWarperGpu(&self) -> *const c_void;
	fn as_raw_mut_Detail_CylindricalWarperGpu(&mut self) -> *mut c_void;

	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_CylindricalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn build_maps_1(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat()) }.into_result()
	}
	
	fn warp_1(&mut self, src: &core::GpuMat, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat()) }.into_result()
	}
	
}

pub struct Detail_CylindricalWarperGpu {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_CylindricalWarperGpu }

impl Drop for Detail_CylindricalWarperGpu {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_CylindricalWarperGpu_delete(instance: *mut c_void); }
		unsafe { cv_Detail_CylindricalWarperGpu_delete(self.as_raw_mut_Detail_CylindricalWarperGpu()) };
	}
}

impl Detail_CylindricalWarperGpu {
	#[inline] pub fn as_raw_Detail_CylindricalWarperGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_CylindricalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_CylindricalWarperGpu {}

impl crate::stitching::Detail_CylindricalWarperTrait for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_CylindricalWarperGpuTrait for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_Detail_CylindricalWarperGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_CylindricalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_CylindricalWarperGpu {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_CylindricalWarperGpu> {
		unsafe { sys::cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_CylindricalWarperGpu::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_DisjointSetsTrait {
	fn as_raw_Detail_DisjointSets(&self) -> *const c_void;
	fn as_raw_mut_Detail_DisjointSets(&mut self) -> *mut c_void;

	fn parent(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_detail_DisjointSets_getPropParent(self.as_raw_mut_Detail_DisjointSets()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: parent")
	}
	
	fn set_parent(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_detail_DisjointSets_setPropParent_vector_int_(self.as_raw_mut_Detail_DisjointSets(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_parent")
	}
	
	fn size(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_detail_DisjointSets_getPropSize(self.as_raw_mut_Detail_DisjointSets()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: size")
	}
	
	fn set_size(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_detail_DisjointSets_setPropSize_vector_int_(self.as_raw_mut_Detail_DisjointSets(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_size")
	}
	
	fn create_one_elem_sets(&mut self, elem_count: i32) -> Result<()> {
		unsafe { sys::cv_detail_DisjointSets_createOneElemSets_int(self.as_raw_mut_Detail_DisjointSets(), elem_count) }.into_result()
	}
	
	fn find_set_by_elem(&mut self, elem: i32) -> Result<i32> {
		unsafe { sys::cv_detail_DisjointSets_findSetByElem_int(self.as_raw_mut_Detail_DisjointSets(), elem) }.into_result()
	}
	
	fn merge_sets(&mut self, set1: i32, set2: i32) -> Result<i32> {
		unsafe { sys::cv_detail_DisjointSets_mergeSets_int_int(self.as_raw_mut_Detail_DisjointSets(), set1, set2) }.into_result()
	}
	
}

pub struct Detail_DisjointSets {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_DisjointSets }

impl Drop for Detail_DisjointSets {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_DisjointSets_delete(instance: *mut c_void); }
		unsafe { cv_Detail_DisjointSets_delete(self.as_raw_mut_Detail_DisjointSets()) };
	}
}

impl Detail_DisjointSets {
	#[inline] pub fn as_raw_Detail_DisjointSets(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_DisjointSets(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_DisjointSets {}

impl crate::stitching::Detail_DisjointSetsTrait for Detail_DisjointSets {
	#[inline] fn as_raw_Detail_DisjointSets(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_DisjointSets(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_DisjointSets {
	/// ## C++ default parameters
	/// * elem_count: 0
	pub fn new(elem_count: i32) -> Result<crate::stitching::Detail_DisjointSets> {
		unsafe { sys::cv_detail_DisjointSets_DisjointSets_int(elem_count) }.into_result().map(|r| unsafe { crate::stitching::Detail_DisjointSets::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_DpSeamFinderTrait: crate::stitching::Detail_SeamFinder {
	fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void;

	fn cost_function(&self) -> Result<crate::stitching::Detail_DpSeamFinder_CostFunction> {
		unsafe { sys::cv_detail_DpSeamFinder_costFunction_const(self.as_raw_Detail_DpSeamFinder()) }.into_result()
	}
	
	fn set_cost_function(&mut self, val: crate::stitching::Detail_DpSeamFinder_CostFunction) -> Result<()> {
		unsafe { sys::cv_detail_DpSeamFinder_setCostFunction_CostFunction(self.as_raw_mut_Detail_DpSeamFinder(), val) }.into_result()
	}
	
	fn find(&mut self, src: &core::Vector::<core::UMat>, corners: &core::Vector::<core::Point>, masks: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_DpSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_DpSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
}

pub struct Detail_DpSeamFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_DpSeamFinder }

impl Drop for Detail_DpSeamFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_DpSeamFinder_delete(instance: *mut c_void); }
		unsafe { cv_Detail_DpSeamFinder_delete(self.as_raw_mut_Detail_DpSeamFinder()) };
	}
}

impl Detail_DpSeamFinder {
	#[inline] pub fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_DpSeamFinder {}

impl crate::stitching::Detail_DpSeamFinderTrait for Detail_DpSeamFinder {
	#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinder for Detail_DpSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_DpSeamFinder {
	/// ## C++ default parameters
	/// * cost_func: COLOR
	pub fn new(cost_func: crate::stitching::Detail_DpSeamFinder_CostFunction) -> Result<crate::stitching::Detail_DpSeamFinder> {
		unsafe { sys::cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cost_func) }.into_result().map(|r| unsafe { crate::stitching::Detail_DpSeamFinder::opencv_from_extern(r) } )
	}
	
}

/// Rotation estimator base class.
/// 
/// It takes features of all images, pairwise matches between all images and estimates rotations of all
/// cameras.
/// 
/// 
/// Note: The coordinate system origin is implementation-dependent, but you can always normalize the
/// rotations in respect to the first camera, for instance. :
pub trait Detail_Estimator {
	fn as_raw_Detail_Estimator(&self) -> *const c_void;
	fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void;

}

/// Base class for all exposure compensators.
pub trait Detail_ExposureCompensator {
	fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void;
	fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void;

	/// ## Parameters
	/// * corners: Source image top-left corners
	/// * images: Source images
	/// * masks: Image masks to update (second value in pair specifies the value which should be used
	/// to detect where image is)
	fn feed(&mut self, corners: &core::Vector::<core::Point>, images: &core::Vector::<core::UMat>, masks: &core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_ExposureCompensator_feed_const_vector_Point_R_const_vector_UMat_R_const_vector_UMat_R(self.as_raw_mut_Detail_ExposureCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfUMat()) }.into_result()
	}
	
	/// Compensate exposure in the specified image.
	/// 
	/// ## Parameters
	/// * index: Image index
	/// * corner: Image top-left corner
	/// * image: Image to process
	/// * mask: Image mask
	fn apply(&mut self, index: i32, corner: core::Point, image: &mut dyn core::ToInputOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_ExposureCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
}

impl dyn Detail_ExposureCompensator + '_ {
	pub fn create_default(typ: i32) -> Result<core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>> {
		unsafe { sys::cv_detail_ExposureCompensator_createDefault_int(typ) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(r) } )
	}
	
}
/// Simple blender which mixes images at its borders.
pub trait Detail_FeatherBlenderTrait: crate::stitching::Detail_BlenderTrait {
	fn as_raw_Detail_FeatherBlender(&self) -> *const c_void;
	fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void;

	fn sharpness(&self) -> Result<f32> {
		unsafe { sys::cv_detail_FeatherBlender_sharpness_const(self.as_raw_Detail_FeatherBlender()) }.into_result()
	}
	
	fn set_sharpness(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_detail_FeatherBlender_setSharpness_float(self.as_raw_mut_Detail_FeatherBlender(), val) }.into_result()
	}
	
	fn prepare(&mut self, dst_roi: core::Rect) -> Result<()> {
		unsafe { sys::cv_detail_FeatherBlender_prepare_Rect(self.as_raw_mut_Detail_FeatherBlender(), dst_roi.opencv_as_extern()) }.into_result()
	}
	
	fn feed(&mut self, img: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, tl: core::Point) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(mask);
		unsafe { sys::cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_FeatherBlender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), tl.opencv_as_extern()) }.into_result()
	}
	
	fn blend(&mut self, dst: &mut dyn core::ToInputOutputArray, dst_mask: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(dst);
		input_output_array_arg!(dst_mask);
		unsafe { sys::cv_detail_FeatherBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Detail_FeatherBlender(), dst.as_raw__InputOutputArray(), dst_mask.as_raw__InputOutputArray()) }.into_result()
	}
	
	/// Creates weight maps for fixed set of source images by their masks and top-left corners.
	/// Final image can be obtained by simple weighting of the source images.
	fn create_weight_maps(&mut self, masks: &core::Vector::<core::UMat>, corners: &core::Vector::<core::Point>, weight_maps: &mut core::Vector::<core::UMat>) -> Result<core::Rect> {
		unsafe { sys::cv_detail_FeatherBlender_createWeightMaps_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_FeatherBlender(), masks.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), weight_maps.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
}

/// Simple blender which mixes images at its borders.
pub struct Detail_FeatherBlender {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_FeatherBlender }

impl Drop for Detail_FeatherBlender {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_FeatherBlender_delete(instance: *mut c_void); }
		unsafe { cv_Detail_FeatherBlender_delete(self.as_raw_mut_Detail_FeatherBlender()) };
	}
}

impl Detail_FeatherBlender {
	#[inline] pub fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_FeatherBlender {}

impl crate::stitching::Detail_BlenderTrait for Detail_FeatherBlender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeatherBlenderTrait for Detail_FeatherBlender {
	#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_FeatherBlender {
	/// ## C++ default parameters
	/// * sharpness: 0.02f
	pub fn new(sharpness: f32) -> Result<crate::stitching::Detail_FeatherBlender> {
		unsafe { sys::cv_detail_FeatherBlender_FeatherBlender_float(sharpness) }.into_result().map(|r| unsafe { crate::stitching::Detail_FeatherBlender::opencv_from_extern(r) } )
	}
	
}

/// Feature finders base class
pub trait Detail_FeaturesFinder {
	fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void;

	/// Frees unused memory allocated before if there is any.
	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_detail_FeaturesFinder_collectGarbage(self.as_raw_mut_Detail_FeaturesFinder()) }.into_result()
	}
	
}

/// Feature matchers base class.
pub trait Detail_FeaturesMatcher {
	fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void;
	fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void;

	/// ## Returns
	/// True, if it's possible to use the same matcher instance in parallel, false otherwise
	fn is_thread_safe(&self) -> Result<bool> {
		unsafe { sys::cv_detail_FeaturesMatcher_isThreadSafe_const(self.as_raw_Detail_FeaturesMatcher()) }.into_result()
	}
	
	/// Frees unused memory allocated before if there is any.
	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_detail_FeaturesMatcher_collectGarbage(self.as_raw_mut_Detail_FeaturesMatcher()) }.into_result()
	}
	
}

pub trait Detail_FisheyeProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_FisheyeProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_FisheyeProjector(&mut self) -> *mut c_void;

	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_FisheyeProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_FisheyeProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_FisheyeProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_FisheyeProjector }

impl Drop for Detail_FisheyeProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_FisheyeProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_FisheyeProjector_delete(self.as_raw_mut_Detail_FisheyeProjector()) };
	}
}

impl Detail_FisheyeProjector {
	#[inline] pub fn as_raw_Detail_FisheyeProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_FisheyeProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_FisheyeProjector {}

impl crate::stitching::Detail_FisheyeProjectorTrait for Detail_FisheyeProjector {
	#[inline] fn as_raw_Detail_FisheyeProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FisheyeProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_FisheyeProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_FisheyeProjector {
}

pub trait Detail_FisheyeWarperTrait {
	fn as_raw_Detail_FisheyeWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_FisheyeWarper(&mut self) -> *mut c_void;

}

pub struct Detail_FisheyeWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_FisheyeWarper }

impl Drop for Detail_FisheyeWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_FisheyeWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_FisheyeWarper_delete(self.as_raw_mut_Detail_FisheyeWarper()) };
	}
}

impl Detail_FisheyeWarper {
	#[inline] pub fn as_raw_Detail_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_FisheyeWarper {}

impl crate::stitching::Detail_FisheyeWarperTrait for Detail_FisheyeWarper {
	#[inline] fn as_raw_Detail_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_FisheyeWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_FisheyeWarper {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_FisheyeWarper> {
		unsafe { sys::cv_detail_FisheyeWarper_FisheyeWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_FisheyeWarper::opencv_from_extern(r) } )
	}
	
}

/// Exposure compensator which tries to remove exposure related artifacts by adjusting image
/// intensities, see [BL07](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_BL07) and [WJ10](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_WJ10) for details.
pub trait Detail_GainCompensatorTrait: crate::stitching::Detail_ExposureCompensator {
	fn as_raw_Detail_GainCompensator(&self) -> *const c_void;
	fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void;

	fn apply(&mut self, index: i32, corner: core::Point, image: &mut dyn core::ToInputOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_GainCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray()) }.into_result()
	}
	
	fn gains(&self) -> Result<core::Vector::<f64>> {
		unsafe { sys::cv_detail_GainCompensator_gains_const(self.as_raw_Detail_GainCompensator()) }.into_result().map(|r| unsafe { core::Vector::<f64>::opencv_from_extern(r) } )
	}
	
}

/// Exposure compensator which tries to remove exposure related artifacts by adjusting image
/// intensities, see [BL07](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_BL07) and [WJ10](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_WJ10) for details.
pub struct Detail_GainCompensator {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_GainCompensator }

impl Drop for Detail_GainCompensator {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_GainCompensator_delete(instance: *mut c_void); }
		unsafe { cv_Detail_GainCompensator_delete(self.as_raw_mut_Detail_GainCompensator()) };
	}
}

impl Detail_GainCompensator {
	#[inline] pub fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_GainCompensator {}

impl crate::stitching::Detail_ExposureCompensator for Detail_GainCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_GainCompensatorTrait for Detail_GainCompensator {
	#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_GainCompensator {
}

pub trait Detail_GraphTrait {
	fn as_raw_Detail_Graph(&self) -> *const c_void;
	fn as_raw_mut_Detail_Graph(&mut self) -> *mut c_void;

	fn create(&mut self, num_vertices: i32) -> Result<()> {
		unsafe { sys::cv_detail_Graph_create_int(self.as_raw_mut_Detail_Graph(), num_vertices) }.into_result()
	}
	
	fn num_vertices(&self) -> Result<i32> {
		unsafe { sys::cv_detail_Graph_numVertices_const(self.as_raw_Detail_Graph()) }.into_result()
	}
	
	fn add_edge(&mut self, from: i32, to: i32, weight: f32) -> Result<()> {
		unsafe { sys::cv_detail_Graph_addEdge_int_int_float(self.as_raw_mut_Detail_Graph(), from, to, weight) }.into_result()
	}
	
}

pub struct Detail_Graph {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_Graph }

impl Drop for Detail_Graph {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_Graph_delete(instance: *mut c_void); }
		unsafe { cv_Detail_Graph_delete(self.as_raw_mut_Detail_Graph()) };
	}
}

impl Detail_Graph {
	#[inline] pub fn as_raw_Detail_Graph(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_Graph(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_Graph {}

impl crate::stitching::Detail_GraphTrait for Detail_Graph {
	#[inline] fn as_raw_Detail_Graph(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Graph(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_Graph {
	/// ## C++ default parameters
	/// * num_vertices: 0
	pub fn new(num_vertices: i32) -> Result<crate::stitching::Detail_Graph> {
		unsafe { sys::cv_detail_Graph_Graph_int(num_vertices) }.into_result().map(|r| unsafe { crate::stitching::Detail_Graph::opencv_from_extern(r) } )
	}
	
}

/// Minimum graph cut-based seam estimator. See details in [V03](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_V03) .
pub trait Detail_GraphCutSeamFinderTrait: crate::stitching::Detail_GraphCutSeamFinderBaseTrait + crate::stitching::Detail_SeamFinder {
	fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void;

	fn find(&mut self, src: &core::Vector::<core::UMat>, corners: &core::Vector::<core::Point>, masks: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_GraphCutSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_GraphCutSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
}

/// Minimum graph cut-based seam estimator. See details in [V03](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_V03) .
pub struct Detail_GraphCutSeamFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_GraphCutSeamFinder }

impl Drop for Detail_GraphCutSeamFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_GraphCutSeamFinder_delete(instance: *mut c_void); }
		unsafe { cv_Detail_GraphCutSeamFinder_delete(self.as_raw_mut_Detail_GraphCutSeamFinder()) };
	}
}

impl Detail_GraphCutSeamFinder {
	#[inline] pub fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_GraphCutSeamFinder {}

impl crate::stitching::Detail_GraphCutSeamFinderTrait for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinder for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_GraphCutSeamFinder {
	/// ## C++ default parameters
	/// * cost_type: COST_COLOR_GRAD
	/// * terminal_cost: 10000.f
	/// * bad_region_penalty: 1000.f
	pub fn new(cost_type: i32, terminal_cost: f32, bad_region_penalty: f32) -> Result<crate::stitching::Detail_GraphCutSeamFinder> {
		unsafe { sys::cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(cost_type, terminal_cost, bad_region_penalty) }.into_result().map(|r| unsafe { crate::stitching::Detail_GraphCutSeamFinder::opencv_from_extern(r) } )
	}
	
}

/// Base class for all minimum graph-cut-based seam estimators.
pub trait Detail_GraphCutSeamFinderBaseTrait {
	fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void;
	fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void;

}

/// Base class for all minimum graph-cut-based seam estimators.
pub struct Detail_GraphCutSeamFinderBase {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_GraphCutSeamFinderBase }

impl Drop for Detail_GraphCutSeamFinderBase {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_GraphCutSeamFinderBase_delete(instance: *mut c_void); }
		unsafe { cv_Detail_GraphCutSeamFinderBase_delete(self.as_raw_mut_Detail_GraphCutSeamFinderBase()) };
	}
}

impl Detail_GraphCutSeamFinderBase {
	#[inline] pub fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_GraphCutSeamFinderBase {}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for Detail_GraphCutSeamFinderBase {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_GraphCutSeamFinderBase {
}

pub trait Detail_GraphEdgeTrait {
	fn as_raw_Detail_GraphEdge(&self) -> *const c_void;
	fn as_raw_mut_Detail_GraphEdge(&mut self) -> *mut c_void;

	fn from(&self) -> i32 {
		unsafe { sys::cv_detail_GraphEdge_getPropFrom_const(self.as_raw_Detail_GraphEdge()) }.into_result().expect("Infallible function failed: from")
	}
	
	fn set_from(&mut self, val: i32) -> () {
		unsafe { sys::cv_detail_GraphEdge_setPropFrom_int(self.as_raw_mut_Detail_GraphEdge(), val) }.into_result().expect("Infallible function failed: set_from")
	}
	
	fn to(&self) -> i32 {
		unsafe { sys::cv_detail_GraphEdge_getPropTo_const(self.as_raw_Detail_GraphEdge()) }.into_result().expect("Infallible function failed: to")
	}
	
	fn set_to(&mut self, val: i32) -> () {
		unsafe { sys::cv_detail_GraphEdge_setPropTo_int(self.as_raw_mut_Detail_GraphEdge(), val) }.into_result().expect("Infallible function failed: set_to")
	}
	
	fn weight(&self) -> f32 {
		unsafe { sys::cv_detail_GraphEdge_getPropWeight_const(self.as_raw_Detail_GraphEdge()) }.into_result().expect("Infallible function failed: weight")
	}
	
	fn set_weight(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_GraphEdge_setPropWeight_float(self.as_raw_mut_Detail_GraphEdge(), val) }.into_result().expect("Infallible function failed: set_weight")
	}
	
}

pub struct Detail_GraphEdge {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_GraphEdge }

impl Drop for Detail_GraphEdge {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_GraphEdge_delete(instance: *mut c_void); }
		unsafe { cv_Detail_GraphEdge_delete(self.as_raw_mut_Detail_GraphEdge()) };
	}
}

impl Detail_GraphEdge {
	#[inline] pub fn as_raw_Detail_GraphEdge(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_GraphEdge(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_GraphEdge {}

impl crate::stitching::Detail_GraphEdgeTrait for Detail_GraphEdge {
	#[inline] fn as_raw_Detail_GraphEdge(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_GraphEdge(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_GraphEdge {
	pub fn new(from: i32, to: i32, weight: f32) -> Result<crate::stitching::Detail_GraphEdge> {
		unsafe { sys::cv_detail_GraphEdge_GraphEdge_int_int_float(from, to, weight) }.into_result().map(|r| unsafe { crate::stitching::Detail_GraphEdge::opencv_from_extern(r) } )
	}
	
}

/// Homography based rotation estimator.
pub trait Detail_HomographyBasedEstimatorTrait: crate::stitching::Detail_Estimator {
	fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void;
	fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void;

}

/// Homography based rotation estimator.
pub struct Detail_HomographyBasedEstimator {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_HomographyBasedEstimator }

impl Drop for Detail_HomographyBasedEstimator {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_HomographyBasedEstimator_delete(instance: *mut c_void); }
		unsafe { cv_Detail_HomographyBasedEstimator_delete(self.as_raw_mut_Detail_HomographyBasedEstimator()) };
	}
}

impl Detail_HomographyBasedEstimator {
	#[inline] pub fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_HomographyBasedEstimator {}

impl crate::stitching::Detail_Estimator for Detail_HomographyBasedEstimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_HomographyBasedEstimatorTrait for Detail_HomographyBasedEstimator {
	#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_HomographyBasedEstimator {
	/// ## C++ default parameters
	/// * is_focals_estimated: false
	pub fn new(is_focals_estimated: bool) -> Result<crate::stitching::Detail_HomographyBasedEstimator> {
		unsafe { sys::cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(is_focals_estimated) }.into_result().map(|r| unsafe { crate::stitching::Detail_HomographyBasedEstimator::opencv_from_extern(r) } )
	}
	
}

/// Structure containing image keypoints and descriptors.
pub trait Detail_ImageFeaturesTrait {
	fn as_raw_Detail_ImageFeatures(&self) -> *const c_void;
	fn as_raw_mut_Detail_ImageFeatures(&mut self) -> *mut c_void;

	fn img_idx(&self) -> i32 {
		unsafe { sys::cv_detail_ImageFeatures_getPropImg_idx_const(self.as_raw_Detail_ImageFeatures()) }.into_result().expect("Infallible function failed: img_idx")
	}
	
	fn set_img_idx(&mut self, val: i32) -> () {
		unsafe { sys::cv_detail_ImageFeatures_setPropImg_idx_int(self.as_raw_mut_Detail_ImageFeatures(), val) }.into_result().expect("Infallible function failed: set_img_idx")
	}
	
	fn img_size(&self) -> core::Size {
		unsafe { sys::cv_detail_ImageFeatures_getPropImg_size_const(self.as_raw_Detail_ImageFeatures()) }.into_result().expect("Infallible function failed: img_size")
	}
	
	fn set_img_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_detail_ImageFeatures_setPropImg_size_Size(self.as_raw_mut_Detail_ImageFeatures(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_img_size")
	}
	
	fn keypoints(&mut self) -> core::Vector::<core::KeyPoint> {
		unsafe { sys::cv_detail_ImageFeatures_getPropKeypoints(self.as_raw_mut_Detail_ImageFeatures()) }.into_result().map(|r| unsafe { core::Vector::<core::KeyPoint>::opencv_from_extern(r) } ).expect("Infallible function failed: keypoints")
	}
	
	fn set_keypoints(&mut self, mut val: core::Vector::<core::KeyPoint>) -> () {
		unsafe { sys::cv_detail_ImageFeatures_setPropKeypoints_vector_KeyPoint_(self.as_raw_mut_Detail_ImageFeatures(), val.as_raw_mut_VectorOfKeyPoint()) }.into_result().expect("Infallible function failed: set_keypoints")
	}
	
	fn descriptors(&mut self) -> core::UMat {
		unsafe { sys::cv_detail_ImageFeatures_getPropDescriptors(self.as_raw_mut_Detail_ImageFeatures()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } ).expect("Infallible function failed: descriptors")
	}
	
	fn set_descriptors(&mut self, mut val: core::UMat) -> () {
		unsafe { sys::cv_detail_ImageFeatures_setPropDescriptors_UMat(self.as_raw_mut_Detail_ImageFeatures(), val.as_raw_mut_UMat()) }.into_result().expect("Infallible function failed: set_descriptors")
	}
	
}

/// Structure containing image keypoints and descriptors.
pub struct Detail_ImageFeatures {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_ImageFeatures }

impl Drop for Detail_ImageFeatures {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_ImageFeatures_delete(instance: *mut c_void); }
		unsafe { cv_Detail_ImageFeatures_delete(self.as_raw_mut_Detail_ImageFeatures()) };
	}
}

impl Detail_ImageFeatures {
	#[inline] pub fn as_raw_Detail_ImageFeatures(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_ImageFeatures(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_ImageFeatures {}

impl crate::stitching::Detail_ImageFeaturesTrait for Detail_ImageFeatures {
	#[inline] fn as_raw_Detail_ImageFeatures(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ImageFeatures(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_ImageFeatures {
}

/// Structure containing information about matches between two images.
/// 
/// It's assumed that there is a transformation between those images. Transformation may be
/// homography or affine transformation based on selected matcher.
/// ## See also
/// detail::FeaturesMatcher
pub trait Detail_MatchesInfoTrait {
	fn as_raw_Detail_MatchesInfo(&self) -> *const c_void;
	fn as_raw_mut_Detail_MatchesInfo(&mut self) -> *mut c_void;

	/// Images indices (optional)
	fn src_img_idx(&self) -> i32 {
		unsafe { sys::cv_detail_MatchesInfo_getPropSrc_img_idx_const(self.as_raw_Detail_MatchesInfo()) }.into_result().expect("Infallible function failed: src_img_idx")
	}
	
	/// Images indices (optional)
	fn set_src_img_idx(&mut self, val: i32) -> () {
		unsafe { sys::cv_detail_MatchesInfo_setPropSrc_img_idx_int(self.as_raw_mut_Detail_MatchesInfo(), val) }.into_result().expect("Infallible function failed: set_src_img_idx")
	}
	
	/// Images indices (optional)
	fn dst_img_idx(&self) -> i32 {
		unsafe { sys::cv_detail_MatchesInfo_getPropDst_img_idx_const(self.as_raw_Detail_MatchesInfo()) }.into_result().expect("Infallible function failed: dst_img_idx")
	}
	
	/// Images indices (optional)
	fn set_dst_img_idx(&mut self, val: i32) -> () {
		unsafe { sys::cv_detail_MatchesInfo_setPropDst_img_idx_int(self.as_raw_mut_Detail_MatchesInfo(), val) }.into_result().expect("Infallible function failed: set_dst_img_idx")
	}
	
	fn matches(&mut self) -> core::Vector::<core::DMatch> {
		unsafe { sys::cv_detail_MatchesInfo_getPropMatches(self.as_raw_mut_Detail_MatchesInfo()) }.into_result().map(|r| unsafe { core::Vector::<core::DMatch>::opencv_from_extern(r) } ).expect("Infallible function failed: matches")
	}
	
	fn set_matches(&mut self, mut val: core::Vector::<core::DMatch>) -> () {
		unsafe { sys::cv_detail_MatchesInfo_setPropMatches_vector_DMatch_(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_mut_VectorOfDMatch()) }.into_result().expect("Infallible function failed: set_matches")
	}
	
	/// Geometrically consistent matches mask
	fn inliers_mask(&mut self) -> core::Vector::<u8> {
		unsafe { sys::cv_detail_MatchesInfo_getPropInliers_mask(self.as_raw_mut_Detail_MatchesInfo()) }.into_result().map(|r| unsafe { core::Vector::<u8>::opencv_from_extern(r) } ).expect("Infallible function failed: inliers_mask")
	}
	
	/// Geometrically consistent matches mask
	fn set_inliers_mask(&mut self, mut val: core::Vector::<u8>) -> () {
		unsafe { sys::cv_detail_MatchesInfo_setPropInliers_mask_vector_unsigned_char_(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_mut_VectorOfu8()) }.into_result().expect("Infallible function failed: set_inliers_mask")
	}
	
	/// Number of geometrically consistent matches
	fn num_inliers(&self) -> i32 {
		unsafe { sys::cv_detail_MatchesInfo_getPropNum_inliers_const(self.as_raw_Detail_MatchesInfo()) }.into_result().expect("Infallible function failed: num_inliers")
	}
	
	/// Number of geometrically consistent matches
	fn set_num_inliers(&mut self, val: i32) -> () {
		unsafe { sys::cv_detail_MatchesInfo_setPropNum_inliers_int(self.as_raw_mut_Detail_MatchesInfo(), val) }.into_result().expect("Infallible function failed: set_num_inliers")
	}
	
	/// Estimated transformation
	fn h(&mut self) -> core::Mat {
		unsafe { sys::cv_detail_MatchesInfo_getPropH(self.as_raw_mut_Detail_MatchesInfo()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: h")
	}
	
	/// Estimated transformation
	fn set_h(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_detail_MatchesInfo_setPropH_Mat(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_h")
	}
	
	/// Confidence two images are from the same panorama
	fn confidence(&self) -> f64 {
		unsafe { sys::cv_detail_MatchesInfo_getPropConfidence_const(self.as_raw_Detail_MatchesInfo()) }.into_result().expect("Infallible function failed: confidence")
	}
	
	/// Confidence two images are from the same panorama
	fn set_confidence(&mut self, val: f64) -> () {
		unsafe { sys::cv_detail_MatchesInfo_setPropConfidence_double(self.as_raw_mut_Detail_MatchesInfo(), val) }.into_result().expect("Infallible function failed: set_confidence")
	}
	
}

/// Structure containing information about matches between two images.
/// 
/// It's assumed that there is a transformation between those images. Transformation may be
/// homography or affine transformation based on selected matcher.
/// ## See also
/// detail::FeaturesMatcher
pub struct Detail_MatchesInfo {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_MatchesInfo }

impl Drop for Detail_MatchesInfo {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_MatchesInfo_delete(instance: *mut c_void); }
		unsafe { cv_Detail_MatchesInfo_delete(self.as_raw_mut_Detail_MatchesInfo()) };
	}
}

impl Detail_MatchesInfo {
	#[inline] pub fn as_raw_Detail_MatchesInfo(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_MatchesInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_MatchesInfo {}

impl crate::stitching::Detail_MatchesInfoTrait for Detail_MatchesInfo {
	#[inline] fn as_raw_Detail_MatchesInfo(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_MatchesInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_MatchesInfo {
	pub fn default() -> Result<crate::stitching::Detail_MatchesInfo> {
		unsafe { sys::cv_detail_MatchesInfo_MatchesInfo() }.into_result().map(|r| unsafe { crate::stitching::Detail_MatchesInfo::opencv_from_extern(r) } )
	}
	
	pub fn copy(other: &crate::stitching::Detail_MatchesInfo) -> Result<crate::stitching::Detail_MatchesInfo> {
		unsafe { sys::cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(other.as_raw_Detail_MatchesInfo()) }.into_result().map(|r| unsafe { crate::stitching::Detail_MatchesInfo::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_MercatorProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_MercatorProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_MercatorProjector(&mut self) -> *mut c_void;

	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_MercatorProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_MercatorProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_MercatorProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_MercatorProjector }

impl Drop for Detail_MercatorProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_MercatorProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_MercatorProjector_delete(self.as_raw_mut_Detail_MercatorProjector()) };
	}
}

impl Detail_MercatorProjector {
	#[inline] pub fn as_raw_Detail_MercatorProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_MercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_MercatorProjector {}

impl crate::stitching::Detail_MercatorProjectorTrait for Detail_MercatorProjector {
	#[inline] fn as_raw_Detail_MercatorProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_MercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_MercatorProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_MercatorProjector {
}

pub trait Detail_MercatorWarperTrait {
	fn as_raw_Detail_MercatorWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_MercatorWarper(&mut self) -> *mut c_void;

}

pub struct Detail_MercatorWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_MercatorWarper }

impl Drop for Detail_MercatorWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_MercatorWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_MercatorWarper_delete(self.as_raw_mut_Detail_MercatorWarper()) };
	}
}

impl Detail_MercatorWarper {
	#[inline] pub fn as_raw_Detail_MercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_MercatorWarper {}

impl crate::stitching::Detail_MercatorWarperTrait for Detail_MercatorWarper {
	#[inline] fn as_raw_Detail_MercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_MercatorWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_MercatorWarper {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_MercatorWarper> {
		unsafe { sys::cv_detail_MercatorWarper_MercatorWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_MercatorWarper::opencv_from_extern(r) } )
	}
	
}

/// Blender which uses multi-band blending algorithm (see [BA83](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_BA83)).
pub trait Detail_MultiBandBlenderTrait: crate::stitching::Detail_BlenderTrait {
	fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void;
	fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void;

	fn num_bands(&self) -> Result<i32> {
		unsafe { sys::cv_detail_MultiBandBlender_numBands_const(self.as_raw_Detail_MultiBandBlender()) }.into_result()
	}
	
	fn set_num_bands(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_detail_MultiBandBlender_setNumBands_int(self.as_raw_mut_Detail_MultiBandBlender(), val) }.into_result()
	}
	
	fn prepare(&mut self, dst_roi: core::Rect) -> Result<()> {
		unsafe { sys::cv_detail_MultiBandBlender_prepare_Rect(self.as_raw_mut_Detail_MultiBandBlender(), dst_roi.opencv_as_extern()) }.into_result()
	}
	
	fn feed(&mut self, img: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, tl: core::Point) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(mask);
		unsafe { sys::cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_MultiBandBlender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), tl.opencv_as_extern()) }.into_result()
	}
	
	fn blend(&mut self, dst: &mut dyn core::ToInputOutputArray, dst_mask: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(dst);
		input_output_array_arg!(dst_mask);
		unsafe { sys::cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Detail_MultiBandBlender(), dst.as_raw__InputOutputArray(), dst_mask.as_raw__InputOutputArray()) }.into_result()
	}
	
}

/// Blender which uses multi-band blending algorithm (see [BA83](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_BA83)).
pub struct Detail_MultiBandBlender {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_MultiBandBlender }

impl Drop for Detail_MultiBandBlender {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_MultiBandBlender_delete(instance: *mut c_void); }
		unsafe { cv_Detail_MultiBandBlender_delete(self.as_raw_mut_Detail_MultiBandBlender()) };
	}
}

impl Detail_MultiBandBlender {
	#[inline] pub fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_MultiBandBlender {}

impl crate::stitching::Detail_BlenderTrait for Detail_MultiBandBlender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_MultiBandBlenderTrait for Detail_MultiBandBlender {
	#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_MultiBandBlender {
	/// ## C++ default parameters
	/// * try_gpu: false
	/// * num_bands: 5
	/// * weight_type: CV_32F
	pub fn new(try_gpu: i32, num_bands: i32, weight_type: i32) -> Result<crate::stitching::Detail_MultiBandBlender> {
		unsafe { sys::cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(try_gpu, num_bands, weight_type) }.into_result().map(|r| unsafe { crate::stitching::Detail_MultiBandBlender::opencv_from_extern(r) } )
	}
	
}

/// Stub bundle adjuster that does nothing.
pub trait Detail_NoBundleAdjusterTrait: crate::stitching::Detail_BundleAdjusterBase {
	fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void;
	fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void;

}

/// Stub bundle adjuster that does nothing.
pub struct Detail_NoBundleAdjuster {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_NoBundleAdjuster }

impl Drop for Detail_NoBundleAdjuster {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_NoBundleAdjuster_delete(instance: *mut c_void); }
		unsafe { cv_Detail_NoBundleAdjuster_delete(self.as_raw_mut_Detail_NoBundleAdjuster()) };
	}
}

impl Detail_NoBundleAdjuster {
	#[inline] pub fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_NoBundleAdjuster {}

impl crate::stitching::Detail_BundleAdjusterBase for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_Estimator for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoBundleAdjusterTrait for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_NoBundleAdjuster {
	pub fn default() -> Result<crate::stitching::Detail_NoBundleAdjuster> {
		unsafe { sys::cv_detail_NoBundleAdjuster_NoBundleAdjuster() }.into_result().map(|r| unsafe { crate::stitching::Detail_NoBundleAdjuster::opencv_from_extern(r) } )
	}
	
}

/// Stub exposure compensator which does nothing.
pub trait Detail_NoExposureCompensatorTrait: crate::stitching::Detail_ExposureCompensator {
	fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void;
	fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void;

	fn apply(&mut self, unnamed: i32, unnamed_1: core::Point, unnamed_2: &mut dyn core::ToInputOutputArray, unnamed_3: &dyn core::ToInputArray) -> Result<()> {
		input_output_array_arg!(unnamed_2);
		input_array_arg!(unnamed_3);
		unsafe { sys::cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_NoExposureCompensator(), unnamed, unnamed_1.opencv_as_extern(), unnamed_2.as_raw__InputOutputArray(), unnamed_3.as_raw__InputArray()) }.into_result()
	}
	
}

/// Stub exposure compensator which does nothing.
pub struct Detail_NoExposureCompensator {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_NoExposureCompensator }

impl Drop for Detail_NoExposureCompensator {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_NoExposureCompensator_delete(instance: *mut c_void); }
		unsafe { cv_Detail_NoExposureCompensator_delete(self.as_raw_mut_Detail_NoExposureCompensator()) };
	}
}

impl Detail_NoExposureCompensator {
	#[inline] pub fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_NoExposureCompensator {}

impl crate::stitching::Detail_ExposureCompensator for Detail_NoExposureCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoExposureCompensatorTrait for Detail_NoExposureCompensator {
	#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_NoExposureCompensator {
}

/// Stub seam estimator which does nothing.
pub trait Detail_NoSeamFinderTrait: crate::stitching::Detail_SeamFinder {
	fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void;

	fn find(&mut self, unnamed: &core::Vector::<core::UMat>, unnamed_1: &core::Vector::<core::Point>, unnamed_2: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_NoSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_NoSeamFinder(), unnamed.as_raw_VectorOfUMat(), unnamed_1.as_raw_VectorOfPoint(), unnamed_2.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
}

/// Stub seam estimator which does nothing.
pub struct Detail_NoSeamFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_NoSeamFinder }

impl Drop for Detail_NoSeamFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_NoSeamFinder_delete(instance: *mut c_void); }
		unsafe { cv_Detail_NoSeamFinder_delete(self.as_raw_mut_Detail_NoSeamFinder()) };
	}
}

impl Detail_NoSeamFinder {
	#[inline] pub fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_NoSeamFinder {}

impl crate::stitching::Detail_NoSeamFinderTrait for Detail_NoSeamFinder {
	#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinder for Detail_NoSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_NoSeamFinder {
}

/// ORB features finder. :
/// ## See also
/// detail::FeaturesFinder, ORB
pub trait Detail_OrbFeaturesFinderTrait: crate::stitching::Detail_FeaturesFinder {
	fn as_raw_Detail_OrbFeaturesFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_OrbFeaturesFinder(&mut self) -> *mut c_void;

}

/// ORB features finder. :
/// ## See also
/// detail::FeaturesFinder, ORB
pub struct Detail_OrbFeaturesFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_OrbFeaturesFinder }

impl Drop for Detail_OrbFeaturesFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_OrbFeaturesFinder_delete(instance: *mut c_void); }
		unsafe { cv_Detail_OrbFeaturesFinder_delete(self.as_raw_mut_Detail_OrbFeaturesFinder()) };
	}
}

impl Detail_OrbFeaturesFinder {
	#[inline] pub fn as_raw_Detail_OrbFeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_OrbFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_OrbFeaturesFinder {}

impl crate::stitching::Detail_FeaturesFinder for Detail_OrbFeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_OrbFeaturesFinderTrait for Detail_OrbFeaturesFinder {
	#[inline] fn as_raw_Detail_OrbFeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_OrbFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_OrbFeaturesFinder {
	/// ## C++ default parameters
	/// * _grid_size: Size(3,1)
	/// * nfeatures: 1500
	/// * scale_factor: 1.3f
	/// * nlevels: 5
	pub fn new(_grid_size: core::Size, nfeatures: i32, scale_factor: f32, nlevels: i32) -> Result<crate::stitching::Detail_OrbFeaturesFinder> {
		unsafe { sys::cv_detail_OrbFeaturesFinder_OrbFeaturesFinder_Size_int_float_int(_grid_size.opencv_as_extern(), nfeatures, scale_factor, nlevels) }.into_result().map(|r| unsafe { crate::stitching::Detail_OrbFeaturesFinder::opencv_from_extern(r) } )
	}
	
}

/// Base class for all pairwise seam estimators.
pub trait Detail_PairwiseSeamFinder: crate::stitching::Detail_SeamFinder {
	fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void;

	fn find(&mut self, src: &core::Vector::<core::UMat>, corners: &core::Vector::<core::Point>, masks: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_PairwiseSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_PairwiseSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
}

pub trait Detail_PaniniPortraitProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_PaniniPortraitProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_PaniniPortraitProjector(&mut self) -> *mut c_void;

	fn a(&self) -> f32 {
		unsafe { sys::cv_detail_PaniniPortraitProjector_getPropA_const(self.as_raw_Detail_PaniniPortraitProjector()) }.into_result().expect("Infallible function failed: a")
	}
	
	fn set_a(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_PaniniPortraitProjector_setPropA_float(self.as_raw_mut_Detail_PaniniPortraitProjector(), val) }.into_result().expect("Infallible function failed: set_a")
	}
	
	fn b(&self) -> f32 {
		unsafe { sys::cv_detail_PaniniPortraitProjector_getPropB_const(self.as_raw_Detail_PaniniPortraitProjector()) }.into_result().expect("Infallible function failed: b")
	}
	
	fn set_b(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_PaniniPortraitProjector_setPropB_float(self.as_raw_mut_Detail_PaniniPortraitProjector(), val) }.into_result().expect("Infallible function failed: set_b")
	}
	
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniPortraitProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniPortraitProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_PaniniPortraitProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PaniniPortraitProjector }

impl Drop for Detail_PaniniPortraitProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PaniniPortraitProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PaniniPortraitProjector_delete(self.as_raw_mut_Detail_PaniniPortraitProjector()) };
	}
}

impl Detail_PaniniPortraitProjector {
	#[inline] pub fn as_raw_Detail_PaniniPortraitProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PaniniPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PaniniPortraitProjector {}

impl crate::stitching::Detail_PaniniPortraitProjectorTrait for Detail_PaniniPortraitProjector {
	#[inline] fn as_raw_Detail_PaniniPortraitProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PaniniPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_PaniniPortraitProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PaniniPortraitProjector {
}

pub trait Detail_PaniniPortraitWarperTrait {
	fn as_raw_Detail_PaniniPortraitWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_PaniniPortraitWarper(&mut self) -> *mut c_void;

}

pub struct Detail_PaniniPortraitWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PaniniPortraitWarper }

impl Drop for Detail_PaniniPortraitWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PaniniPortraitWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PaniniPortraitWarper_delete(self.as_raw_mut_Detail_PaniniPortraitWarper()) };
	}
}

impl Detail_PaniniPortraitWarper {
	#[inline] pub fn as_raw_Detail_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PaniniPortraitWarper {}

impl crate::stitching::Detail_PaniniPortraitWarperTrait for Detail_PaniniPortraitWarper {
	#[inline] fn as_raw_Detail_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_PaniniPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PaniniPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_PaniniPortraitWarper> {
		unsafe { sys::cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(scale, a, b) }.into_result().map(|r| unsafe { crate::stitching::Detail_PaniniPortraitWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_PaniniProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_PaniniProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_PaniniProjector(&mut self) -> *mut c_void;

	fn a(&self) -> f32 {
		unsafe { sys::cv_detail_PaniniProjector_getPropA_const(self.as_raw_Detail_PaniniProjector()) }.into_result().expect("Infallible function failed: a")
	}
	
	fn set_a(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_PaniniProjector_setPropA_float(self.as_raw_mut_Detail_PaniniProjector(), val) }.into_result().expect("Infallible function failed: set_a")
	}
	
	fn b(&self) -> f32 {
		unsafe { sys::cv_detail_PaniniProjector_getPropB_const(self.as_raw_Detail_PaniniProjector()) }.into_result().expect("Infallible function failed: b")
	}
	
	fn set_b(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_PaniniProjector_setPropB_float(self.as_raw_mut_Detail_PaniniProjector(), val) }.into_result().expect("Infallible function failed: set_b")
	}
	
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_PaniniProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PaniniProjector }

impl Drop for Detail_PaniniProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PaniniProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PaniniProjector_delete(self.as_raw_mut_Detail_PaniniProjector()) };
	}
}

impl Detail_PaniniProjector {
	#[inline] pub fn as_raw_Detail_PaniniProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PaniniProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PaniniProjector {}

impl crate::stitching::Detail_PaniniProjectorTrait for Detail_PaniniProjector {
	#[inline] fn as_raw_Detail_PaniniProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PaniniProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_PaniniProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PaniniProjector {
}

pub trait Detail_PaniniWarperTrait {
	fn as_raw_Detail_PaniniWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_PaniniWarper(&mut self) -> *mut c_void;

}

pub struct Detail_PaniniWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PaniniWarper }

impl Drop for Detail_PaniniWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PaniniWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PaniniWarper_delete(self.as_raw_mut_Detail_PaniniWarper()) };
	}
}

impl Detail_PaniniWarper {
	#[inline] pub fn as_raw_Detail_PaniniWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PaniniWarper {}

impl crate::stitching::Detail_PaniniWarperTrait for Detail_PaniniWarper {
	#[inline] fn as_raw_Detail_PaniniWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_PaniniWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PaniniWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_PaniniWarper> {
		unsafe { sys::cv_detail_PaniniWarper_PaniniWarper_float_float_float(scale, a, b) }.into_result().map(|r| unsafe { crate::stitching::Detail_PaniniWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_PlanePortraitWarperTrait {
	fn as_raw_Detail_PlanePortraitWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_PlanePortraitWarper(&mut self) -> *mut c_void;

}

pub struct Detail_PlanePortraitWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PlanePortraitWarper }

impl Drop for Detail_PlanePortraitWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PlanePortraitWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PlanePortraitWarper_delete(self.as_raw_mut_Detail_PlanePortraitWarper()) };
	}
}

impl Detail_PlanePortraitWarper {
	#[inline] pub fn as_raw_Detail_PlanePortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PlanePortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PlanePortraitWarper {}

impl crate::stitching::Detail_PlanePortraitWarperTrait for Detail_PlanePortraitWarper {
	#[inline] fn as_raw_Detail_PlanePortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PlanePortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_PlanePortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PlanePortraitWarper {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_PlanePortraitWarper> {
		unsafe { sys::cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_PlanePortraitWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_PlaneProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_PlaneProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_PlaneProjector(&mut self) -> *mut c_void;

	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlaneProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlaneProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_PlaneProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PlaneProjector }

impl Drop for Detail_PlaneProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PlaneProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PlaneProjector_delete(self.as_raw_mut_Detail_PlaneProjector()) };
	}
}

impl Detail_PlaneProjector {
	#[inline] pub fn as_raw_Detail_PlaneProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PlaneProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PlaneProjector {}

impl crate::stitching::Detail_PlaneProjectorTrait for Detail_PlaneProjector {
	#[inline] fn as_raw_Detail_PlaneProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PlaneProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_PlaneProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PlaneProjector {
}

/// Warper that maps an image onto the z = 1 plane.
pub trait Detail_PlaneWarperTrait {
	fn as_raw_Detail_PlaneWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void;

	fn warp_point(&mut self, pt: core::Point2f, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
	}
	
	fn warp_point_1(&mut self, pt: core::Point2f, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		unsafe { sys::cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray()) }.into_result()
	}
	
	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn build_maps_1(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp_1(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp_roi(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
	}
	
	fn warp_roi_1(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		unsafe { sys::cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray()) }.into_result()
	}
	
}

/// Warper that maps an image onto the z = 1 plane.
pub struct Detail_PlaneWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PlaneWarper }

impl Drop for Detail_PlaneWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PlaneWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PlaneWarper_delete(self.as_raw_mut_Detail_PlaneWarper()) };
	}
}

impl Detail_PlaneWarper {
	#[inline] pub fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PlaneWarper {}

impl crate::stitching::Detail_PlaneWarperTrait for Detail_PlaneWarper {
	#[inline] fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_PlaneWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PlaneWarper {
	/// Construct an instance of the plane warper class.
	/// 
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	/// 
	/// ## C++ default parameters
	/// * scale: 1.f
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_PlaneWarper> {
		unsafe { sys::cv_detail_PlaneWarper_PlaneWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_PlaneWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_PlaneWarperGpuTrait: crate::stitching::Detail_PlaneWarperTrait {
	fn as_raw_Detail_PlaneWarperGpu(&self) -> *const c_void;
	fn as_raw_mut_Detail_PlaneWarperGpu(&mut self) -> *mut c_void;

	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn build_maps_1(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp_1(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn build_maps_2(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat()) }.into_result()
	}
	
	fn build_maps_3(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat()) }.into_result()
	}
	
	fn warp_2(&mut self, src: &core::GpuMat, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat()) }.into_result()
	}
	
	fn warp_3(&mut self, src: &core::GpuMat, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat()) }.into_result()
	}
	
}

pub struct Detail_PlaneWarperGpu {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_PlaneWarperGpu }

impl Drop for Detail_PlaneWarperGpu {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_PlaneWarperGpu_delete(instance: *mut c_void); }
		unsafe { cv_Detail_PlaneWarperGpu_delete(self.as_raw_mut_Detail_PlaneWarperGpu()) };
	}
}

impl Detail_PlaneWarperGpu {
	#[inline] pub fn as_raw_Detail_PlaneWarperGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_PlaneWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_PlaneWarperGpu {}

impl crate::stitching::Detail_PlaneWarperTrait for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_PlaneWarperGpuTrait for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_Detail_PlaneWarperGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PlaneWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_RotationWarper for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_PlaneWarperGpu {
	/// ## C++ default parameters
	/// * scale: 1.f
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_PlaneWarperGpu> {
		unsafe { sys::cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_PlaneWarperGpu::opencv_from_extern(r) } )
	}
	
}

/// Base class for warping logic implementation.
pub trait Detail_ProjectorBaseTrait {
	fn as_raw_Detail_ProjectorBase(&self) -> *const c_void;
	fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void;

	fn scale(&self) -> f32 {
		unsafe { sys::cv_detail_ProjectorBase_getPropScale_const(self.as_raw_Detail_ProjectorBase()) }.into_result().expect("Infallible function failed: scale")
	}
	
	fn set_scale(&mut self, val: f32) -> () {
		unsafe { sys::cv_detail_ProjectorBase_setPropScale_float(self.as_raw_mut_Detail_ProjectorBase(), val) }.into_result().expect("Infallible function failed: set_scale")
	}
	
	fn k(&mut self) -> &mut [f32; 9] {
		unsafe { sys::cv_detail_ProjectorBase_getPropK(self.as_raw_mut_Detail_ProjectorBase()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: k")
	}
	
	fn rinv(&mut self) -> &mut [f32; 9] {
		unsafe { sys::cv_detail_ProjectorBase_getPropRinv(self.as_raw_mut_Detail_ProjectorBase()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: rinv")
	}
	
	fn r_kinv(&mut self) -> &mut [f32; 9] {
		unsafe { sys::cv_detail_ProjectorBase_getPropR_kinv(self.as_raw_mut_Detail_ProjectorBase()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: r_kinv")
	}
	
	fn k_rinv(&mut self) -> &mut [f32; 9] {
		unsafe { sys::cv_detail_ProjectorBase_getPropK_rinv(self.as_raw_mut_Detail_ProjectorBase()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: k_rinv")
	}
	
	fn t(&mut self) -> &mut [f32; 3] {
		unsafe { sys::cv_detail_ProjectorBase_getPropT(self.as_raw_mut_Detail_ProjectorBase()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: t")
	}
	
	/// ## C++ default parameters
	/// * k: Mat::eye(3,3,CV_32F)
	/// * r: Mat::eye(3,3,CV_32F)
	/// * t: Mat::zeros(3,1,CV_32F)
	fn set_camera_params(&mut self, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		unsafe { sys::cv_detail_ProjectorBase_setCameraParams_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_ProjectorBase(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray()) }.into_result()
	}
	
}

/// Base class for warping logic implementation.
pub struct Detail_ProjectorBase {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_ProjectorBase }

impl Drop for Detail_ProjectorBase {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_ProjectorBase_delete(instance: *mut c_void); }
		unsafe { cv_Detail_ProjectorBase_delete(self.as_raw_mut_Detail_ProjectorBase()) };
	}
}

impl Detail_ProjectorBase {
	#[inline] pub fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_ProjectorBase {}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_ProjectorBase {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_ProjectorBase {
}

/// Rotation-only model image warper interface.
pub trait Detail_RotationWarper {
	fn as_raw_Detail_RotationWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void;

	/// Projects the image point.
	/// 
	/// ## Parameters
	/// * pt: Source point
	/// * K: Camera intrinsic parameters
	/// * R: Camera rotation matrix
	/// ## Returns
	/// Projected point
	fn warp_point(&mut self, pt: core::Point2f, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_RotationWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
	}
	
	/// Builds the projection maps according to the given camera data.
	/// 
	/// ## Parameters
	/// * src_size: Source image size
	/// * K: Camera intrinsic parameters
	/// * R: Camera rotation matrix
	/// * xmap: Projection map for the x axis
	/// * ymap: Projection map for the y axis
	/// ## Returns
	/// Projected image minimum bounding box
	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	/// Projects the image.
	/// 
	/// ## Parameters
	/// * src: Source image
	/// * K: Camera intrinsic parameters
	/// * R: Camera rotation matrix
	/// * interp_mode: Interpolation mode
	/// * border_mode: Border extrapolation mode
	/// * dst: Projected image
	/// ## Returns
	/// Project image top-left corner
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_RotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	/// Projects the image backward.
	/// 
	/// ## Parameters
	/// * src: Projected image
	/// * K: Camera intrinsic parameters
	/// * R: Camera rotation matrix
	/// * interp_mode: Interpolation mode
	/// * border_mode: Border extrapolation mode
	/// * dst_size: Backward-projected image size
	/// * dst: Backward-projected image
	fn warp_backward(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst_size: core::Size, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst_size.opencv_as_extern(), dst.as_raw__OutputArray()) }.into_result()
	}
	
	/// ## Parameters
	/// * src_size: Source image bounding box
	/// * K: Camera intrinsic parameters
	/// * R: Camera rotation matrix
	/// ## Returns
	/// Projected image minimum bounding box
	fn warp_roi(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_RotationWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray()) }.into_result()
	}
	
	fn get_scale(&self) -> Result<f32> {
		unsafe { sys::cv_detail_RotationWarper_getScale_const(self.as_raw_Detail_RotationWarper()) }.into_result()
	}
	
	fn set_scale(&mut self, unnamed: f32) -> Result<()> {
		unsafe { sys::cv_detail_RotationWarper_setScale_float(self.as_raw_mut_Detail_RotationWarper(), unnamed) }.into_result()
	}
	
}

/// Base class for a seam estimator.
pub trait Detail_SeamFinder {
	fn as_raw_Detail_SeamFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void;

	/// Estimates seams.
	/// 
	/// ## Parameters
	/// * src: Source images
	/// * corners: Source image top-left corners
	/// * masks: Source image masks to update
	fn find(&mut self, src: &core::Vector::<core::UMat>, corners: &core::Vector::<core::Point>, masks: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_SeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_SeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
}

pub trait Detail_SphericalPortraitWarperTrait {
	fn as_raw_Detail_SphericalPortraitWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_SphericalPortraitWarper(&mut self) -> *mut c_void;

}

pub struct Detail_SphericalPortraitWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_SphericalPortraitWarper }

impl Drop for Detail_SphericalPortraitWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_SphericalPortraitWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_SphericalPortraitWarper_delete(self.as_raw_mut_Detail_SphericalPortraitWarper()) };
	}
}

impl Detail_SphericalPortraitWarper {
	#[inline] pub fn as_raw_Detail_SphericalPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_SphericalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_SphericalPortraitWarper {}

impl crate::stitching::Detail_RotationWarper for Detail_SphericalPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SphericalPortraitWarperTrait for Detail_SphericalPortraitWarper {
	#[inline] fn as_raw_Detail_SphericalPortraitWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SphericalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_SphericalPortraitWarper {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_SphericalPortraitWarper> {
		unsafe { sys::cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_SphericalPortraitWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_SphericalProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_SphericalProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_SphericalProjector(&mut self) -> *mut c_void;

	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_SphericalProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_SphericalProjector }

impl Drop for Detail_SphericalProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_SphericalProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_SphericalProjector_delete(self.as_raw_mut_Detail_SphericalProjector()) };
	}
}

impl Detail_SphericalProjector {
	#[inline] pub fn as_raw_Detail_SphericalProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_SphericalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_SphericalProjector {}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_SphericalProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SphericalProjectorTrait for Detail_SphericalProjector {
	#[inline] fn as_raw_Detail_SphericalProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SphericalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_SphericalProjector {
}

/// Warper that maps an image onto the unit sphere located at the origin.
/// 
/// Projects image onto unit sphere with origin at (0, 0, 0) and radius scale, measured in pixels.
/// A 360° panorama would therefore have a resulting width of 2 * scale * PI pixels.
/// Poles are located at (0, -1, 0) and (0, 1, 0) points.
pub trait Detail_SphericalWarperTrait {
	fn as_raw_Detail_SphericalWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void;

	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_SphericalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
}

/// Warper that maps an image onto the unit sphere located at the origin.
/// 
/// Projects image onto unit sphere with origin at (0, 0, 0) and radius scale, measured in pixels.
/// A 360° panorama would therefore have a resulting width of 2 * scale * PI pixels.
/// Poles are located at (0, -1, 0) and (0, 1, 0) points.
pub struct Detail_SphericalWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_SphericalWarper }

impl Drop for Detail_SphericalWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_SphericalWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_SphericalWarper_delete(self.as_raw_mut_Detail_SphericalWarper()) };
	}
}

impl Detail_SphericalWarper {
	#[inline] pub fn as_raw_Detail_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_SphericalWarper {}

impl crate::stitching::Detail_RotationWarper for Detail_SphericalWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SphericalWarperTrait for Detail_SphericalWarper {
	#[inline] fn as_raw_Detail_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_SphericalWarper {
	/// Construct an instance of the spherical warper class.
	/// 
	/// ## Parameters
	/// * scale: Radius of the projected sphere, in pixels. An image spanning the
	///              whole sphere will have a width of 2 * scale * PI pixels.
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_SphericalWarper> {
		unsafe { sys::cv_detail_SphericalWarper_SphericalWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_SphericalWarper::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_SphericalWarperGpuTrait: crate::stitching::Detail_SphericalWarperTrait {
	fn as_raw_Detail_SphericalWarperGpu(&self) -> *const c_void;
	fn as_raw_mut_Detail_SphericalWarperGpu(&mut self) -> *mut c_void;

	fn build_maps(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut dyn core::ToOutputArray, ymap: &mut dyn core::ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		unsafe { sys::cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray()) }.into_result()
	}
	
	fn warp(&mut self, src: &dyn core::ToInputArray, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut dyn core::ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		unsafe { sys::cv_detail_SphericalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray()) }.into_result()
	}
	
	fn build_maps_1(&mut self, src_size: core::Size, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_SphericalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat()) }.into_result()
	}
	
	fn warp_1(&mut self, src: &core::GpuMat, k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		unsafe { sys::cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_SphericalWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat()) }.into_result()
	}
	
}

pub struct Detail_SphericalWarperGpu {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_SphericalWarperGpu }

impl Drop for Detail_SphericalWarperGpu {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_SphericalWarperGpu_delete(instance: *mut c_void); }
		unsafe { cv_Detail_SphericalWarperGpu_delete(self.as_raw_mut_Detail_SphericalWarperGpu()) };
	}
}

impl Detail_SphericalWarperGpu {
	#[inline] pub fn as_raw_Detail_SphericalWarperGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_SphericalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_SphericalWarperGpu {}

impl crate::stitching::Detail_RotationWarper for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SphericalWarperTrait for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_Detail_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SphericalWarperGpuTrait for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_Detail_SphericalWarperGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SphericalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_SphericalWarperGpu {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_SphericalWarperGpu> {
		unsafe { sys::cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_SphericalWarperGpu::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_StereographicProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_StereographicProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_StereographicProjector(&mut self) -> *mut c_void;

	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_StereographicProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_StereographicProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_StereographicProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_StereographicProjector }

impl Drop for Detail_StereographicProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_StereographicProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_StereographicProjector_delete(self.as_raw_mut_Detail_StereographicProjector()) };
	}
}

impl Detail_StereographicProjector {
	#[inline] pub fn as_raw_Detail_StereographicProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_StereographicProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_StereographicProjector {}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_StereographicProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_StereographicProjectorTrait for Detail_StereographicProjector {
	#[inline] fn as_raw_Detail_StereographicProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_StereographicProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_StereographicProjector {
}

pub trait Detail_StereographicWarperTrait {
	fn as_raw_Detail_StereographicWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_StereographicWarper(&mut self) -> *mut c_void;

}

pub struct Detail_StereographicWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_StereographicWarper }

impl Drop for Detail_StereographicWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_StereographicWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_StereographicWarper_delete(self.as_raw_mut_Detail_StereographicWarper()) };
	}
}

impl Detail_StereographicWarper {
	#[inline] pub fn as_raw_Detail_StereographicWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_StereographicWarper {}

impl crate::stitching::Detail_RotationWarper for Detail_StereographicWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_StereographicWarperTrait for Detail_StereographicWarper {
	#[inline] fn as_raw_Detail_StereographicWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_StereographicWarper {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_StereographicWarper> {
		unsafe { sys::cv_detail_StereographicWarper_StereographicWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_StereographicWarper::opencv_from_extern(r) } )
	}
	
}

/// SURF features finder.
/// ## See also
/// detail::FeaturesFinder, SURF
pub trait Detail_SurfFeaturesFinderTrait: crate::stitching::Detail_FeaturesFinder {
	fn as_raw_Detail_SurfFeaturesFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_SurfFeaturesFinder(&mut self) -> *mut c_void;

}

/// SURF features finder.
/// ## See also
/// detail::FeaturesFinder, SURF
pub struct Detail_SurfFeaturesFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_SurfFeaturesFinder }

impl Drop for Detail_SurfFeaturesFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_SurfFeaturesFinder_delete(instance: *mut c_void); }
		unsafe { cv_Detail_SurfFeaturesFinder_delete(self.as_raw_mut_Detail_SurfFeaturesFinder()) };
	}
}

impl Detail_SurfFeaturesFinder {
	#[inline] pub fn as_raw_Detail_SurfFeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_SurfFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_SurfFeaturesFinder {}

impl crate::stitching::Detail_FeaturesFinder for Detail_SurfFeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SurfFeaturesFinderTrait for Detail_SurfFeaturesFinder {
	#[inline] fn as_raw_Detail_SurfFeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SurfFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_SurfFeaturesFinder {
	/// ## C++ default parameters
	/// * hess_thresh: 300.
	/// * num_octaves: 3
	/// * num_layers: 4
	/// * num_octaves_descr: /*4*/3
	/// * num_layers_descr: /*2*/4
	pub fn new(hess_thresh: f64, num_octaves: i32, num_layers: i32, num_octaves_descr: i32, num_layers_descr: i32) -> Result<crate::stitching::Detail_SurfFeaturesFinder> {
		unsafe { sys::cv_detail_SurfFeaturesFinder_SurfFeaturesFinder_double_int_int_int_int(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr) }.into_result().map(|r| unsafe { crate::stitching::Detail_SurfFeaturesFinder::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_SurfFeaturesFinderGpuTrait: crate::stitching::Detail_FeaturesFinder {
	fn as_raw_Detail_SurfFeaturesFinderGpu(&self) -> *const c_void;
	fn as_raw_mut_Detail_SurfFeaturesFinderGpu(&mut self) -> *mut c_void;

	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_detail_SurfFeaturesFinderGpu_collectGarbage(self.as_raw_mut_Detail_SurfFeaturesFinderGpu()) }.into_result()
	}
	
}

pub struct Detail_SurfFeaturesFinderGpu {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_SurfFeaturesFinderGpu }

impl Drop for Detail_SurfFeaturesFinderGpu {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_SurfFeaturesFinderGpu_delete(instance: *mut c_void); }
		unsafe { cv_Detail_SurfFeaturesFinderGpu_delete(self.as_raw_mut_Detail_SurfFeaturesFinderGpu()) };
	}
}

impl Detail_SurfFeaturesFinderGpu {
	#[inline] pub fn as_raw_Detail_SurfFeaturesFinderGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_SurfFeaturesFinderGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_SurfFeaturesFinderGpu {}

impl crate::stitching::Detail_FeaturesFinder for Detail_SurfFeaturesFinderGpu {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SurfFeaturesFinderGpuTrait for Detail_SurfFeaturesFinderGpu {
	#[inline] fn as_raw_Detail_SurfFeaturesFinderGpu(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SurfFeaturesFinderGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_SurfFeaturesFinderGpu {
	/// ## C++ default parameters
	/// * hess_thresh: 300.
	/// * num_octaves: 3
	/// * num_layers: 4
	/// * num_octaves_descr: 4
	/// * num_layers_descr: 2
	pub fn new(hess_thresh: f64, num_octaves: i32, num_layers: i32, num_octaves_descr: i32, num_layers_descr: i32) -> Result<crate::stitching::Detail_SurfFeaturesFinderGpu> {
		unsafe { sys::cv_detail_SurfFeaturesFinderGpu_SurfFeaturesFinderGpu_double_int_int_int_int(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr) }.into_result().map(|r| unsafe { crate::stitching::Detail_SurfFeaturesFinderGpu::opencv_from_extern(r) } )
	}
	
}

pub trait Detail_TransverseMercatorProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_Detail_TransverseMercatorProjector(&self) -> *const c_void;
	fn as_raw_mut_Detail_TransverseMercatorProjector(&mut self) -> *mut c_void;

	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_TransverseMercatorProjector(), x, y, u, v) }.into_result()
	}
	
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		unsafe { sys::cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_TransverseMercatorProjector(), u, v, x, y) }.into_result()
	}
	
}

pub struct Detail_TransverseMercatorProjector {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_TransverseMercatorProjector }

impl Drop for Detail_TransverseMercatorProjector {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_TransverseMercatorProjector_delete(instance: *mut c_void); }
		unsafe { cv_Detail_TransverseMercatorProjector_delete(self.as_raw_mut_Detail_TransverseMercatorProjector()) };
	}
}

impl Detail_TransverseMercatorProjector {
	#[inline] pub fn as_raw_Detail_TransverseMercatorProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_TransverseMercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_TransverseMercatorProjector {}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_TransverseMercatorProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_TransverseMercatorProjectorTrait for Detail_TransverseMercatorProjector {
	#[inline] fn as_raw_Detail_TransverseMercatorProjector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_TransverseMercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_TransverseMercatorProjector {
}

pub trait Detail_TransverseMercatorWarperTrait {
	fn as_raw_Detail_TransverseMercatorWarper(&self) -> *const c_void;
	fn as_raw_mut_Detail_TransverseMercatorWarper(&mut self) -> *mut c_void;

}

pub struct Detail_TransverseMercatorWarper {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_TransverseMercatorWarper }

impl Drop for Detail_TransverseMercatorWarper {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_TransverseMercatorWarper_delete(instance: *mut c_void); }
		unsafe { cv_Detail_TransverseMercatorWarper_delete(self.as_raw_mut_Detail_TransverseMercatorWarper()) };
	}
}

impl Detail_TransverseMercatorWarper {
	#[inline] pub fn as_raw_Detail_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_TransverseMercatorWarper {}

impl crate::stitching::Detail_RotationWarper for Detail_TransverseMercatorWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_TransverseMercatorWarperTrait for Detail_TransverseMercatorWarper {
	#[inline] fn as_raw_Detail_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_TransverseMercatorWarper {
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_TransverseMercatorWarper> {
		unsafe { sys::cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(scale) }.into_result().map(|r| unsafe { crate::stitching::Detail_TransverseMercatorWarper::opencv_from_extern(r) } )
	}
	
}

/// Voronoi diagram-based seam estimator.
pub trait Detail_VoronoiSeamFinderTrait: crate::stitching::Detail_PairwiseSeamFinder {
	fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void;
	fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void;

	fn find(&mut self, src: &core::Vector::<core::UMat>, corners: &core::Vector::<core::Point>, masks: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_VoronoiSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_VoronoiSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
	fn find_1(&mut self, size: &core::Vector::<core::Size>, corners: &core::Vector::<core::Point>, masks: &mut core::Vector::<core::UMat>) -> Result<()> {
		unsafe { sys::cv_detail_VoronoiSeamFinder_find_const_vector_Size_R_const_vector_Point_R_vector_UMat_R(self.as_raw_mut_Detail_VoronoiSeamFinder(), size.as_raw_VectorOfSize(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat()) }.into_result()
	}
	
}

/// Voronoi diagram-based seam estimator.
pub struct Detail_VoronoiSeamFinder {
	ptr: *mut c_void
}

opencv_type_boxed! { Detail_VoronoiSeamFinder }

impl Drop for Detail_VoronoiSeamFinder {
	fn drop(&mut self) {
		extern "C" { fn cv_Detail_VoronoiSeamFinder_delete(instance: *mut c_void); }
		unsafe { cv_Detail_VoronoiSeamFinder_delete(self.as_raw_mut_Detail_VoronoiSeamFinder()) };
	}
}

impl Detail_VoronoiSeamFinder {
	#[inline] pub fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Detail_VoronoiSeamFinder {}

impl crate::stitching::Detail_PairwiseSeamFinder for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinder for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_VoronoiSeamFinderTrait for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Detail_VoronoiSeamFinder {
}
