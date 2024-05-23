//! # Images stitching
//!
//! This figure illustrates the stitching module pipeline implemented in the Stitcher class. Using that
//! class it's possible to configure/remove some steps, i.e. adjust the stitching pipeline according to
//! the particular needs. All building blocks from the pipeline are available in the detail namespace,
//! one can combine and use them separately.
//!
//! The implemented stitching pipeline is very similar to the one proposed in [BL07](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BL07) .
//!
//! ![stitching pipeline](https://docs.opencv.org/3.4.20/StitchingPipeline.jpg)
//!
//! Camera models
//! -------------
//!
//! There are currently 2 camera models implemented in stitching pipeline.
//!
//! - _Homography model_ expecting perspective transformations between images
//!   implemented in [cv::detail::BestOf2NearestMatcher] cv::detail::HomographyBasedEstimator
//!   cv::detail::BundleAdjusterReproj cv::detail::BundleAdjusterRay
//! - _Affine model_ expecting affine transformation with 6 DOF or 4 DOF implemented in
//!   [cv::detail::AffineBestOf2NearestMatcher] cv::detail::AffineBasedEstimator
//!   cv::detail::BundleAdjusterAffine cv::detail::BundleAdjusterAffinePartial cv::AffineWarper
//!
//! Homography model is useful for creating photo panoramas captured by camera,
//! while affine-based model can be used to stitch scans and object captured by
//! specialized devices. Use [cv::Stitcher::create] to get preconfigured pipeline for one
//! of those models.
//!
//!
//! Note:
//! Certain detailed settings of [cv::Stitcher] might not make sense. Especially
//! you should not mix classes implementing affine model and classes implementing
//! Homography model, as they work with different transformations.
//!    # Features Finding and Images Matching
//!    # Rotation Estimation
//!    # Autocalibration
//!    # Images Warping
//!    # Seam Estimation
//!    # Exposure Compensation
//!    # Image Blenders
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{AffineWarperTrait, AffineWarperTraitConst, CompressedRectilinearPortraitWarperTrait, CompressedRectilinearPortraitWarperTraitConst, CompressedRectilinearWarperTrait, CompressedRectilinearWarperTraitConst, CylindricalWarperTrait, CylindricalWarperTraitConst, Detail_AKAZEFeaturesFinderTrait, Detail_AKAZEFeaturesFinderTraitConst, Detail_AffineBasedEstimatorTrait, Detail_AffineBasedEstimatorTraitConst, Detail_AffineBestOf2NearestMatcherTrait, Detail_AffineBestOf2NearestMatcherTraitConst, Detail_AffineWarperTrait, Detail_AffineWarperTraitConst, Detail_BestOf2NearestMatcherTrait, Detail_BestOf2NearestMatcherTraitConst, Detail_BestOf2NearestRangeMatcherTrait, Detail_BestOf2NearestRangeMatcherTraitConst, Detail_BlenderTrait, Detail_BlenderTraitConst, Detail_BlocksGainCompensatorTrait, Detail_BlocksGainCompensatorTraitConst, Detail_BundleAdjusterAffinePartialTrait, Detail_BundleAdjusterAffinePartialTraitConst, Detail_BundleAdjusterAffineTrait, Detail_BundleAdjusterAffineTraitConst, Detail_BundleAdjusterBaseTrait, Detail_BundleAdjusterBaseTraitConst, Detail_BundleAdjusterRayTrait, Detail_BundleAdjusterRayTraitConst, Detail_BundleAdjusterReprojTrait, Detail_BundleAdjusterReprojTraitConst, Detail_CameraParamsTrait, Detail_CameraParamsTraitConst, Detail_CompressedRectilinearPortraitProjectorTrait, Detail_CompressedRectilinearPortraitProjectorTraitConst, Detail_CompressedRectilinearPortraitWarperTrait, Detail_CompressedRectilinearPortraitWarperTraitConst, Detail_CompressedRectilinearProjectorTrait, Detail_CompressedRectilinearProjectorTraitConst, Detail_CompressedRectilinearWarperTrait, Detail_CompressedRectilinearWarperTraitConst, Detail_CylindricalPortraitProjectorTrait, Detail_CylindricalPortraitProjectorTraitConst, Detail_CylindricalPortraitWarperTrait, Detail_CylindricalPortraitWarperTraitConst, Detail_CylindricalProjectorTrait, Detail_CylindricalProjectorTraitConst, Detail_CylindricalWarperGpuTrait, Detail_CylindricalWarperGpuTraitConst, Detail_CylindricalWarperTrait, Detail_CylindricalWarperTraitConst, Detail_DisjointSetsTrait, Detail_DisjointSetsTraitConst, Detail_DpSeamFinderTrait, Detail_DpSeamFinderTraitConst, Detail_EstimatorTrait, Detail_EstimatorTraitConst, Detail_ExposureCompensatorTrait, Detail_ExposureCompensatorTraitConst, Detail_FeatherBlenderTrait, Detail_FeatherBlenderTraitConst, Detail_FeaturesFinderTrait, Detail_FeaturesFinderTraitConst, Detail_FeaturesMatcherTrait, Detail_FeaturesMatcherTraitConst, Detail_FisheyeProjectorTrait, Detail_FisheyeProjectorTraitConst, Detail_FisheyeWarperTrait, Detail_FisheyeWarperTraitConst, Detail_GainCompensatorTrait, Detail_GainCompensatorTraitConst, Detail_GraphCutSeamFinderBaseTrait, Detail_GraphCutSeamFinderBaseTraitConst, Detail_GraphCutSeamFinderTrait, Detail_GraphCutSeamFinderTraitConst, Detail_GraphEdgeTrait, Detail_GraphEdgeTraitConst, Detail_GraphTrait, Detail_GraphTraitConst, Detail_HomographyBasedEstimatorTrait, Detail_HomographyBasedEstimatorTraitConst, Detail_ImageFeaturesTrait, Detail_ImageFeaturesTraitConst, Detail_MatchesInfoTrait, Detail_MatchesInfoTraitConst, Detail_MercatorProjectorTrait, Detail_MercatorProjectorTraitConst, Detail_MercatorWarperTrait, Detail_MercatorWarperTraitConst, Detail_MultiBandBlenderTrait, Detail_MultiBandBlenderTraitConst, Detail_NoBundleAdjusterTrait, Detail_NoBundleAdjusterTraitConst, Detail_NoExposureCompensatorTrait, Detail_NoExposureCompensatorTraitConst, Detail_NoSeamFinderTrait, Detail_NoSeamFinderTraitConst, Detail_OrbFeaturesFinderTrait, Detail_OrbFeaturesFinderTraitConst, Detail_PairwiseSeamFinderTrait, Detail_PairwiseSeamFinderTraitConst, Detail_PaniniPortraitProjectorTrait, Detail_PaniniPortraitProjectorTraitConst, Detail_PaniniPortraitWarperTrait, Detail_PaniniPortraitWarperTraitConst, Detail_PaniniProjectorTrait, Detail_PaniniProjectorTraitConst, Detail_PaniniWarperTrait, Detail_PaniniWarperTraitConst, Detail_PlanePortraitProjectorTrait, Detail_PlanePortraitProjectorTraitConst, Detail_PlanePortraitWarperTrait, Detail_PlanePortraitWarperTraitConst, Detail_PlaneProjectorTrait, Detail_PlaneProjectorTraitConst, Detail_PlaneWarperGpuTrait, Detail_PlaneWarperGpuTraitConst, Detail_PlaneWarperTrait, Detail_PlaneWarperTraitConst, Detail_ProjectorBaseTrait, Detail_ProjectorBaseTraitConst, Detail_RotationWarperTrait, Detail_RotationWarperTraitConst, Detail_SeamFinderTrait, Detail_SeamFinderTraitConst, Detail_SiftFeaturesFinderTrait, Detail_SiftFeaturesFinderTraitConst, Detail_SphericalPortraitProjectorTrait, Detail_SphericalPortraitProjectorTraitConst, Detail_SphericalPortraitWarperTrait, Detail_SphericalPortraitWarperTraitConst, Detail_SphericalProjectorTrait, Detail_SphericalProjectorTraitConst, Detail_SphericalWarperGpuTrait, Detail_SphericalWarperGpuTraitConst, Detail_SphericalWarperTrait, Detail_SphericalWarperTraitConst, Detail_StereographicProjectorTrait, Detail_StereographicProjectorTraitConst, Detail_StereographicWarperTrait, Detail_StereographicWarperTraitConst, Detail_SurfFeaturesFinderGpuTrait, Detail_SurfFeaturesFinderGpuTraitConst, Detail_SurfFeaturesFinderTrait, Detail_SurfFeaturesFinderTraitConst, Detail_TransverseMercatorProjectorTrait, Detail_TransverseMercatorProjectorTraitConst, Detail_TransverseMercatorWarperTrait, Detail_TransverseMercatorWarperTraitConst, Detail_VoronoiSeamFinderTrait, Detail_VoronoiSeamFinderTraitConst, FisheyeWarperTrait, FisheyeWarperTraitConst, MercatorWarperTrait, MercatorWarperTraitConst, PaniniPortraitWarperTrait, PaniniPortraitWarperTraitConst, PaniniWarperTrait, PaniniWarperTraitConst, PlaneWarperTrait, PlaneWarperTraitConst, SphericalWarperTrait, SphericalWarperTraitConst, StereographicWarperTrait, StereographicWarperTraitConst, StitcherTrait, StitcherTraitConst, TransverseMercatorWarperTrait, TransverseMercatorWarperTraitConst, WarperCreatorTrait, WarperCreatorTraitConst};
}

// FEATHER /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:68
pub const Detail_Blender_FEATHER: i32 = 1;
// MULTI_BAND /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:68
pub const Detail_Blender_MULTI_BAND: i32 = 2;
// NO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:68
pub const Detail_Blender_NO: i32 = 0;
// COLOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:121
pub const Detail_DpSeamFinder_COLOR: i32 = 0;
// COLOR_GRAD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:121
pub const Detail_DpSeamFinder_COLOR_GRAD: i32 = 1;
// GAIN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:65
pub const Detail_ExposureCompensator_GAIN: i32 = 1;
// GAIN_BLOCKS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:65
pub const Detail_ExposureCompensator_GAIN_BLOCKS: i32 = 2;
// NO /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:65
pub const Detail_ExposureCompensator_NO: i32 = 0;
// COST_COLOR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:231
pub const Detail_GraphCutSeamFinderBase_COST_COLOR: i32 = 0;
// COST_COLOR_GRAD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:231
pub const Detail_GraphCutSeamFinderBase_COST_COLOR_GRAD: i32 = 1;
// WAVE_CORRECT_HORIZ /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:326
pub const Detail_WAVE_CORRECT_HORIZ: i32 = 0;
// WAVE_CORRECT_VERT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:327
pub const Detail_WAVE_CORRECT_VERT: i32 = 1;
// ERR_CAMERA_PARAMS_ADJUST_FAIL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:141
pub const Stitcher_ERR_CAMERA_PARAMS_ADJUST_FAIL: i32 = 3;
// ERR_HOMOGRAPHY_EST_FAIL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:140
pub const Stitcher_ERR_HOMOGRAPHY_EST_FAIL: i32 = 2;
// ERR_NEED_MORE_IMGS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:139
pub const Stitcher_ERR_NEED_MORE_IMGS: i32 = 1;
// OK /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:138
pub const Stitcher_OK: i32 = 0;
// ORIG_RESOL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:135
pub const Stitcher_ORIG_RESOL: i32 = -1;
/// Mode for creating photo panoramas. Expects images under perspective
/// transformation and projects resulting pano to sphere.
/// ## See also
/// detail::BestOf2NearestMatcher SphericalWarper
// PANORAMA /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:150
pub const Stitcher_PANORAMA: i32 = 0;
/// Mode for composing scans. Expects images under affine transformation does
/// not compensate exposure by default.
/// ## See also
/// detail::AffineBestOf2NearestMatcher AffineWarper
// SCANS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:156
pub const Stitcher_SCANS: i32 = 1;
// CostFunction /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:121
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Detail_DpSeamFinder_CostFunction {
	COLOR = 0,
	COLOR_GRAD = 1,
}

impl TryFrom<i32> for Detail_DpSeamFinder_CostFunction {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::COLOR),
			1 => Ok(Self::COLOR_GRAD),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::stitching::Detail_DpSeamFinder_CostFunction"))),
		}
	}
}

opencv_type_enum! { crate::stitching::Detail_DpSeamFinder_CostFunction }

// CostType /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:231
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Detail_GraphCutSeamFinderBase_CostType {
	COST_COLOR = 0,
	COST_COLOR_GRAD = 1,
}

impl TryFrom<i32> for Detail_GraphCutSeamFinderBase_CostType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::COST_COLOR),
			1 => Ok(Self::COST_COLOR_GRAD),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::stitching::Detail_GraphCutSeamFinderBase_CostType"))),
		}
	}
}

opencv_type_enum! { crate::stitching::Detail_GraphCutSeamFinderBase_CostType }

// WaveCorrectKind /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:324
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Detail_WaveCorrectKind {
	WAVE_CORRECT_HORIZ = 0,
	WAVE_CORRECT_VERT = 1,
}

impl TryFrom<i32> for Detail_WaveCorrectKind {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::WAVE_CORRECT_HORIZ),
			1 => Ok(Self::WAVE_CORRECT_VERT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::stitching::Detail_WaveCorrectKind"))),
		}
	}
}

opencv_type_enum! { crate::stitching::Detail_WaveCorrectKind }

// Mode /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:143
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl TryFrom<i32> for Stitcher_Mode {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::PANORAMA),
			1 => Ok(Self::SCANS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::stitching::Stitcher_Mode"))),
		}
	}
}

opencv_type_enum! { crate::stitching::Stitcher_Mode }

// Status /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:136
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Stitcher_Status {
	OK = 0,
	ERR_NEED_MORE_IMGS = 1,
	ERR_HOMOGRAPHY_EST_FAIL = 2,
	ERR_CAMERA_PARAMS_ADJUST_FAIL = 3,
}

impl TryFrom<i32> for Stitcher_Status {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::OK),
			1 => Ok(Self::ERR_NEED_MORE_IMGS),
			2 => Ok(Self::ERR_HOMOGRAPHY_EST_FAIL),
			3 => Ok(Self::ERR_CAMERA_PARAMS_ADJUST_FAIL),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::stitching::Stitcher_Status"))),
		}
	}
}

opencv_type_enum! { crate::stitching::Stitcher_Status }

// createLaplacePyrGpu(InputArray, int, std::vector<UMat> &)(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:173
// ("cv::detail::createLaplacePyrGpu", vec![(pred!(mut, ["img", "num_levels", "pyr"], ["const cv::_InputArray*", "int", "std::vector<cv::UMat>*"]), _)]),
#[inline]
pub fn create_laplace_pyr_gpu(img: &impl ToInputArray, num_levels: i32, pyr: &mut core::Vector<core::UMat>) -> Result<()> {
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vectorLUMatGR(img.as_raw__InputArray(), num_levels, pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// createLaplacePyr(InputArray, int, std::vector<UMat> &)(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:172
// ("cv::detail::createLaplacePyr", vec![(pred!(mut, ["img", "num_levels", "pyr"], ["const cv::_InputArray*", "int", "std::vector<cv::UMat>*"]), _)]),
#[inline]
pub fn create_laplace_pyr(img: &impl ToInputArray, num_levels: i32, pyr: &mut core::Vector<core::UMat>) -> Result<()> {
	input_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_createLaplacePyr_const__InputArrayR_int_vectorLUMatGR(img.as_raw__InputArray(), num_levels, pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// createWeightMap(InputArray, float, InputOutputArray)(InputArray, Primitive, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:170
// ("cv::detail::createWeightMap", vec![(pred!(mut, ["mask", "sharpness", "weight"], ["const cv::_InputArray*", "float", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn create_weight_map(mask: &impl ToInputArray, sharpness: f32, weight: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(mask);
	input_output_array_arg!(weight);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(mask.as_raw__InputArray(), sharpness, weight.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// findMaxSpanningTree(int, const std::vector<MatchesInfo> &, Graph &, std::vector<int> &)(Primitive, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:350
// ("cv::detail::findMaxSpanningTree", vec![(pred!(mut, ["num_images", "pairwise_matches", "span_tree", "centers"], ["int", "const std::vector<cv::detail::MatchesInfo>*", "cv::detail::Graph*", "std::vector<int>*"]), _)]),
#[inline]
pub fn find_max_spanning_tree(num_images: i32, pairwise_matches: &core::Vector<crate::stitching::Detail_MatchesInfo>, span_tree: &mut impl crate::stitching::Detail_GraphTrait, centers: &mut core::Vector<i32>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_findMaxSpanningTree_int_const_vectorLMatchesInfoGR_GraphR_vectorLintGR(num_images, pairwise_matches.as_raw_VectorOfDetail_MatchesInfo(), span_tree.as_raw_mut_Detail_Graph(), centers.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// leaveBiggestComponent(std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, float)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:345
// ("cv::detail::leaveBiggestComponent", vec![(pred!(mut, ["features", "pairwise_matches", "conf_threshold"], ["std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*", "float"]), _)]),
#[inline]
pub fn leave_biggest_component(features: &mut core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>, conf_threshold: f32) -> Result<core::Vector<i32>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_leaveBiggestComponent_vectorLImageFeaturesGR_vectorLMatchesInfoGR_float(features.as_raw_mut_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), conf_threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ///////////////////////////////////////////////////////////////////////////
// matchesGraphAsString(std::vector<String> &, std::vector<MatchesInfo> &, float)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:342
// ("cv::detail::matchesGraphAsString", vec![(pred!(mut, ["pathes", "pairwise_matches", "conf_threshold"], ["std::vector<cv::String>*", "std::vector<cv::detail::MatchesInfo>*", "float"]), _)]),
#[inline]
pub fn matches_graph_as_string(pathes: &mut core::Vector<String>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>, conf_threshold: f32) -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_matchesGraphAsString_vectorLStringGR_vectorLMatchesInfoGR_float(pathes.as_raw_mut_VectorOfString(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), conf_threshold, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// ///////////////////////////////////////////////////////////////////////////
// normalizeUsingWeightMap(InputArray, InputOutputArray)(InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:168
// ("cv::detail::normalizeUsingWeightMap", vec![(pred!(mut, ["weight", "src"], ["const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn normalize_using_weight_map(weight: &impl ToInputArray, src: &mut impl ToInputOutputArray) -> Result<()> {
	input_array_arg!(weight);
	input_output_array_arg!(src);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(weight.as_raw__InputArray(), src.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ///////////////////////////////////////////////////////////////////////////
// overlapRoi(Point, Point, Size, Size, Rect &)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:103
// ("cv::detail::overlapRoi", vec![(pred!(mut, ["tl1", "tl2", "sz1", "sz2", "roi"], ["cv::Point", "cv::Point", "cv::Size", "cv::Size", "cv::Rect*"]), _)]),
#[inline]
pub fn overlap_roi(tl1: core::Point, tl2: core::Point, sz1: core::Size, sz2: core::Size, roi: &mut core::Rect) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_overlapRoi_Point_Point_Size_Size_RectR(&tl1, &tl2, &sz1, &sz2, roi, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// restoreImageFromLaplacePyrGpu(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:177
// ("cv::detail::restoreImageFromLaplacePyrGpu", vec![(pred!(mut, ["pyr"], ["std::vector<cv::UMat>*"]), _)]),
#[inline]
pub fn restore_image_from_laplace_pyr_gpu(pyr: &mut core::Vector<core::UMat>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_restoreImageFromLaplacePyrGpu_vectorLUMatGR(pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// restoreImageFromLaplacePyr(std::vector<UMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:176
// ("cv::detail::restoreImageFromLaplacePyr", vec![(pred!(mut, ["pyr"], ["std::vector<cv::UMat>*"]), _)]),
#[inline]
pub fn restore_image_from_laplace_pyr(pyr: &mut core::Vector<core::UMat>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_restoreImageFromLaplacePyr_vectorLUMatGR(pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// resultRoiIntersection(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:106
// ("cv::detail::resultRoiIntersection", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
#[inline]
pub fn result_roi_intersection(corners: &core::Vector<core::Point>, sizes: &core::Vector<core::Size>) -> Result<core::Rect> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_resultRoiIntersection_const_vectorLPointGR_const_vectorLSizeGR(corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// resultRoi(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:105
// ("cv::detail::resultRoi", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
#[inline]
pub fn result_roi_1(corners: &core::Vector<core::Point>, sizes: &core::Vector<core::Size>) -> Result<core::Rect> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_resultRoi_const_vectorLPointGR_const_vectorLSizeGR(corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// resultRoi(const std::vector<Point> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:104
// ("cv::detail::resultRoi", vec![(pred!(mut, ["corners", "images"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*"]), _)]),
#[inline]
pub fn result_roi(corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>) -> Result<core::Rect> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_resultRoi_const_vectorLPointGR_const_vectorLUMatGR(corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// resultTl(const std::vector<Point> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:107
// ("cv::detail::resultTl", vec![(pred!(mut, ["corners"], ["const std::vector<cv::Point>*"]), _)]),
#[inline]
pub fn result_tl(corners: &core::Vector<core::Point>) -> Result<core::Point> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_resultTl_const_vectorLPointGR(corners.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// selectRandomSubset(int, int, std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:110
// ("cv::detail::selectRandomSubset", vec![(pred!(mut, ["count", "size", "subset"], ["int", "int", "std::vector<int>*"]), _)]),
#[inline]
pub fn select_random_subset(count: i32, size: i32, subset: &mut core::Vector<i32>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_selectRandomSubset_int_int_vectorLintGR(count, size, subset.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// stitchingLogLevel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:112
// ("cv::detail::stitchingLogLevel", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn stitching_log_level() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_stitchingLogLevel(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Tries to make panorama more horizontal (or vertical).
///
/// ## Parameters
/// * rmats: Camera rotation matrices.
/// * kind: Correction kind, see detail::WaveCorrectKind.
// waveCorrect(std::vector<Mat> &, WaveCorrectKind)(CppPassByVoidPtr, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:335
// ("cv::detail::waveCorrect", vec![(pred!(mut, ["rmats", "kind"], ["std::vector<cv::Mat>*", "cv::detail::WaveCorrectKind"]), _)]),
#[inline]
pub fn wave_correct(rmats: &mut core::Vector<core::Mat>, kind: crate::stitching::Detail_WaveCorrectKind) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_detail_waveCorrect_vectorLMatGR_WaveCorrectKind(rmats.as_raw_mut_VectorOfMat(), kind, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::stitching::AffineWarper]
// AffineWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:74
pub trait AffineWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_AffineWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:77
	// ("cv::AffineWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineWarper_create_const_float(self.as_raw_AffineWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::AffineWarper]
pub trait AffineWarperTrait: crate::stitching::AffineWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void;

}

/// Affine warper factory class.
/// ## See also
/// detail::AffineWarper
// AffineWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:74
pub struct AffineWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { AffineWarper }

impl Drop for AffineWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_AffineWarper_delete(self.as_raw_mut_AffineWarper()) };
	}
}

unsafe impl Send for AffineWarper {}

impl crate::stitching::WarperCreatorTraitConst for AffineWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for AffineWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::AffineWarperTraitConst for AffineWarper {
	#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::AffineWarperTrait for AffineWarper {
	#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AffineWarper, crate::stitching::AffineWarperTraitConst, as_raw_AffineWarper, crate::stitching::AffineWarperTrait, as_raw_mut_AffineWarper }

impl AffineWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_AffineWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { AffineWarper, crate::stitching::WarperCreator, cv_AffineWarper_to_WarperCreator }

impl std::fmt::Debug for AffineWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AffineWarper")
			.finish()
	}
}

impl Default for AffineWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::CompressedRectilinearPortraitWarper]
// CompressedRectilinearPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:119
pub trait CompressedRectilinearPortraitWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:127
	// ("cv::CompressedRectilinearPortraitWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CompressedRectilinearPortraitWarper_create_const_float(self.as_raw_CompressedRectilinearPortraitWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::CompressedRectilinearPortraitWarper]
pub trait CompressedRectilinearPortraitWarperTrait: crate::stitching::CompressedRectilinearPortraitWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void;

}

// CompressedRectilinearPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:119
pub struct CompressedRectilinearPortraitWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { CompressedRectilinearPortraitWarper }

impl Drop for CompressedRectilinearPortraitWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CompressedRectilinearPortraitWarper_delete(self.as_raw_mut_CompressedRectilinearPortraitWarper()) };
	}
}

unsafe impl Send for CompressedRectilinearPortraitWarper {}

impl crate::stitching::WarperCreatorTraitConst for CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CompressedRectilinearPortraitWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::CompressedRectilinearPortraitWarperTraitConst for CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::CompressedRectilinearPortraitWarperTrait for CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CompressedRectilinearPortraitWarper, crate::stitching::CompressedRectilinearPortraitWarperTraitConst, as_raw_CompressedRectilinearPortraitWarper, crate::stitching::CompressedRectilinearPortraitWarperTrait, as_raw_mut_CompressedRectilinearPortraitWarper }

impl CompressedRectilinearPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// CompressedRectilinearPortraitWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:123
	// ("cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	#[inline]
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::CompressedRectilinearPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:123
	// ("cv::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::CompressedRectilinearPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::CompressedRectilinearPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CompressedRectilinearPortraitWarper, crate::stitching::WarperCreator, cv_CompressedRectilinearPortraitWarper_to_WarperCreator }

impl std::fmt::Debug for CompressedRectilinearPortraitWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CompressedRectilinearPortraitWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::CompressedRectilinearWarper]
// CompressedRectilinearWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:108
pub trait CompressedRectilinearWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:116
	// ("cv::CompressedRectilinearWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CompressedRectilinearWarper_create_const_float(self.as_raw_CompressedRectilinearWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::CompressedRectilinearWarper]
pub trait CompressedRectilinearWarperTrait: crate::stitching::CompressedRectilinearWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void;

}

// CompressedRectilinearWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:108
pub struct CompressedRectilinearWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { CompressedRectilinearWarper }

impl Drop for CompressedRectilinearWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CompressedRectilinearWarper_delete(self.as_raw_mut_CompressedRectilinearWarper()) };
	}
}

unsafe impl Send for CompressedRectilinearWarper {}

impl crate::stitching::WarperCreatorTraitConst for CompressedRectilinearWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for CompressedRectilinearWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CompressedRectilinearWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::CompressedRectilinearWarperTraitConst for CompressedRectilinearWarper {
	#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::CompressedRectilinearWarperTrait for CompressedRectilinearWarper {
	#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CompressedRectilinearWarper, crate::stitching::CompressedRectilinearWarperTraitConst, as_raw_CompressedRectilinearWarper, crate::stitching::CompressedRectilinearWarperTrait, as_raw_mut_CompressedRectilinearWarper }

impl CompressedRectilinearWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// CompressedRectilinearWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:112
	// ("cv::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	#[inline]
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::CompressedRectilinearWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::CompressedRectilinearWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::CompressedRectilinearWarper::CompressedRectilinearWarper() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:112
	// ("cv::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::CompressedRectilinearWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CompressedRectilinearWarper_CompressedRectilinearWarper(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::CompressedRectilinearWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CompressedRectilinearWarper, crate::stitching::WarperCreator, cv_CompressedRectilinearWarper_to_WarperCreator }

impl std::fmt::Debug for CompressedRectilinearWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CompressedRectilinearWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::CylindricalWarper]
// CylindricalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:83
pub trait CylindricalWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_CylindricalWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:86
	// ("cv::CylindricalWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CylindricalWarper_create_const_float(self.as_raw_CylindricalWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::CylindricalWarper]
pub trait CylindricalWarperTrait: crate::stitching::CylindricalWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void;

}

/// Cylindrical warper factory class.
/// ## See also
/// detail::CylindricalWarper
// CylindricalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:83
pub struct CylindricalWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { CylindricalWarper }

impl Drop for CylindricalWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CylindricalWarper_delete(self.as_raw_mut_CylindricalWarper()) };
	}
}

unsafe impl Send for CylindricalWarper {}

impl crate::stitching::WarperCreatorTraitConst for CylindricalWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for CylindricalWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CylindricalWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::CylindricalWarperTraitConst for CylindricalWarper {
	#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::CylindricalWarperTrait for CylindricalWarper {
	#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CylindricalWarper, crate::stitching::CylindricalWarperTraitConst, as_raw_CylindricalWarper, crate::stitching::CylindricalWarperTrait, as_raw_mut_CylindricalWarper }

impl CylindricalWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_CylindricalWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { CylindricalWarper, crate::stitching::WarperCreator, cv_CylindricalWarper_to_WarperCreator }

impl std::fmt::Debug for CylindricalWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CylindricalWarper")
			.finish()
	}
}

impl Default for CylindricalWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::FisheyeWarper]
// FisheyeWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:96
pub trait FisheyeWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_FisheyeWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:99
	// ("cv::FisheyeWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_FisheyeWarper_create_const_float(self.as_raw_FisheyeWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::FisheyeWarper]
pub trait FisheyeWarperTrait: crate::stitching::FisheyeWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void;

}

// FisheyeWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:96
pub struct FisheyeWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { FisheyeWarper }

impl Drop for FisheyeWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_FisheyeWarper_delete(self.as_raw_mut_FisheyeWarper()) };
	}
}

unsafe impl Send for FisheyeWarper {}

impl crate::stitching::WarperCreatorTraitConst for FisheyeWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for FisheyeWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FisheyeWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::FisheyeWarperTraitConst for FisheyeWarper {
	#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::FisheyeWarperTrait for FisheyeWarper {
	#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FisheyeWarper, crate::stitching::FisheyeWarperTraitConst, as_raw_FisheyeWarper, crate::stitching::FisheyeWarperTrait, as_raw_mut_FisheyeWarper }

impl FisheyeWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_FisheyeWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { FisheyeWarper, crate::stitching::WarperCreator, cv_FisheyeWarper_to_WarperCreator }

impl std::fmt::Debug for FisheyeWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FisheyeWarper")
			.finish()
	}
}

impl Default for FisheyeWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::MercatorWarper]
// MercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:152
pub trait MercatorWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_MercatorWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:155
	// ("cv::MercatorWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MercatorWarper_create_const_float(self.as_raw_MercatorWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::MercatorWarper]
pub trait MercatorWarperTrait: crate::stitching::MercatorWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void;

}

// MercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:152
pub struct MercatorWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { MercatorWarper }

impl Drop for MercatorWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_MercatorWarper_delete(self.as_raw_mut_MercatorWarper()) };
	}
}

unsafe impl Send for MercatorWarper {}

impl crate::stitching::WarperCreatorTraitConst for MercatorWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for MercatorWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MercatorWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::MercatorWarperTraitConst for MercatorWarper {
	#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::MercatorWarperTrait for MercatorWarper {
	#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MercatorWarper, crate::stitching::MercatorWarperTraitConst, as_raw_MercatorWarper, crate::stitching::MercatorWarperTrait, as_raw_mut_MercatorWarper }

impl MercatorWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_MercatorWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { MercatorWarper, crate::stitching::WarperCreator, cv_MercatorWarper_to_WarperCreator }

impl std::fmt::Debug for MercatorWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MercatorWarper")
			.finish()
	}
}

impl Default for MercatorWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::PaniniPortraitWarper]
// PaniniPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:141
pub trait PaniniPortraitWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_PaniniPortraitWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:149
	// ("cv::PaniniPortraitWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PaniniPortraitWarper_create_const_float(self.as_raw_PaniniPortraitWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::PaniniPortraitWarper]
pub trait PaniniPortraitWarperTrait: crate::stitching::PaniniPortraitWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void;

}

// PaniniPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:141
pub struct PaniniPortraitWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { PaniniPortraitWarper }

impl Drop for PaniniPortraitWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_PaniniPortraitWarper_delete(self.as_raw_mut_PaniniPortraitWarper()) };
	}
}

unsafe impl Send for PaniniPortraitWarper {}

impl crate::stitching::WarperCreatorTraitConst for PaniniPortraitWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for PaniniPortraitWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PaniniPortraitWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::PaniniPortraitWarperTraitConst for PaniniPortraitWarper {
	#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::PaniniPortraitWarperTrait for PaniniPortraitWarper {
	#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PaniniPortraitWarper, crate::stitching::PaniniPortraitWarperTraitConst, as_raw_PaniniPortraitWarper, crate::stitching::PaniniPortraitWarperTrait, as_raw_mut_PaniniPortraitWarper }

impl PaniniPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// PaniniPortraitWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:145
	// ("cv::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	#[inline]
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::PaniniPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::PaniniPortraitWarper::PaniniPortraitWarper() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:145
	// ("cv::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::PaniniPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PaniniPortraitWarper_PaniniPortraitWarper(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::PaniniPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PaniniPortraitWarper, crate::stitching::WarperCreator, cv_PaniniPortraitWarper_to_WarperCreator }

impl std::fmt::Debug for PaniniPortraitWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PaniniPortraitWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::PaniniWarper]
// PaniniWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:130
pub trait PaniniWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_PaniniWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:138
	// ("cv::PaniniWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PaniniWarper_create_const_float(self.as_raw_PaniniWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::PaniniWarper]
pub trait PaniniWarperTrait: crate::stitching::PaniniWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void;

}

// PaniniWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:130
pub struct PaniniWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { PaniniWarper }

impl Drop for PaniniWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_PaniniWarper_delete(self.as_raw_mut_PaniniWarper()) };
	}
}

unsafe impl Send for PaniniWarper {}

impl crate::stitching::WarperCreatorTraitConst for PaniniWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for PaniniWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PaniniWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::PaniniWarperTraitConst for PaniniWarper {
	#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::PaniniWarperTrait for PaniniWarper {
	#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PaniniWarper, crate::stitching::PaniniWarperTraitConst, as_raw_PaniniWarper, crate::stitching::PaniniWarperTrait, as_raw_mut_PaniniWarper }

impl PaniniWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// PaniniWarper(float, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:134
	// ("cv::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["A", "B"], ["float", "float"]), _)]),
	#[inline]
	pub fn new(a: f32, b: f32) -> Result<crate::stitching::PaniniWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PaniniWarper_PaniniWarper_float_float(a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::PaniniWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::PaniniWarper::PaniniWarper() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:134
	// ("cv::PaniniWarper::PaniniWarper", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::PaniniWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PaniniWarper_PaniniWarper(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::PaniniWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PaniniWarper, crate::stitching::WarperCreator, cv_PaniniWarper_to_WarperCreator }

impl std::fmt::Debug for PaniniWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PaniniWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::PlaneWarper]
// PlaneWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:65
pub trait PlaneWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_PlaneWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:68
	// ("cv::PlaneWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_PlaneWarper_create_const_float(self.as_raw_PlaneWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::PlaneWarper]
pub trait PlaneWarperTrait: crate::stitching::PlaneWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void;

}

/// Plane warper factory class.
/// ## See also
/// detail::PlaneWarper
// PlaneWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:65
pub struct PlaneWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { PlaneWarper }

impl Drop for PlaneWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_PlaneWarper_delete(self.as_raw_mut_PlaneWarper()) };
	}
}

unsafe impl Send for PlaneWarper {}

impl crate::stitching::WarperCreatorTraitConst for PlaneWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for PlaneWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PlaneWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::PlaneWarperTraitConst for PlaneWarper {
	#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::PlaneWarperTrait for PlaneWarper {
	#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PlaneWarper, crate::stitching::PlaneWarperTraitConst, as_raw_PlaneWarper, crate::stitching::PlaneWarperTrait, as_raw_mut_PlaneWarper }

impl PlaneWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_PlaneWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { PlaneWarper, crate::stitching::WarperCreator, cv_PlaneWarper_to_WarperCreator }

impl std::fmt::Debug for PlaneWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PlaneWarper")
			.finish()
	}
}

impl Default for PlaneWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::SphericalWarper]
// SphericalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:90
pub trait SphericalWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_SphericalWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:93
	// ("cv::SphericalWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_SphericalWarper_create_const_float(self.as_raw_SphericalWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::SphericalWarper]
pub trait SphericalWarperTrait: crate::stitching::SphericalWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void;

}

/// Spherical warper factory class
// SphericalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:90
pub struct SphericalWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { SphericalWarper }

impl Drop for SphericalWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_SphericalWarper_delete(self.as_raw_mut_SphericalWarper()) };
	}
}

unsafe impl Send for SphericalWarper {}

impl crate::stitching::WarperCreatorTraitConst for SphericalWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for SphericalWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SphericalWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::SphericalWarperTraitConst for SphericalWarper {
	#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::SphericalWarperTrait for SphericalWarper {
	#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SphericalWarper, crate::stitching::SphericalWarperTraitConst, as_raw_SphericalWarper, crate::stitching::SphericalWarperTrait, as_raw_mut_SphericalWarper }

impl SphericalWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_SphericalWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { SphericalWarper, crate::stitching::WarperCreator, cv_SphericalWarper_to_WarperCreator }

impl std::fmt::Debug for SphericalWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SphericalWarper")
			.finish()
	}
}

impl Default for SphericalWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::StereographicWarper]
// StereographicWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:102
pub trait StereographicWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_StereographicWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:105
	// ("cv::StereographicWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_StereographicWarper_create_const_float(self.as_raw_StereographicWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::StereographicWarper]
pub trait StereographicWarperTrait: crate::stitching::StereographicWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void;

}

// StereographicWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:102
pub struct StereographicWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { StereographicWarper }

impl Drop for StereographicWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_StereographicWarper_delete(self.as_raw_mut_StereographicWarper()) };
	}
}

unsafe impl Send for StereographicWarper {}

impl crate::stitching::WarperCreatorTraitConst for StereographicWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for StereographicWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereographicWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::StereographicWarperTraitConst for StereographicWarper {
	#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::StereographicWarperTrait for StereographicWarper {
	#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StereographicWarper, crate::stitching::StereographicWarperTraitConst, as_raw_StereographicWarper, crate::stitching::StereographicWarperTrait, as_raw_mut_StereographicWarper }

impl StereographicWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_StereographicWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { StereographicWarper, crate::stitching::WarperCreator, cv_StereographicWarper_to_WarperCreator }

impl std::fmt::Debug for StereographicWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StereographicWarper")
			.finish()
	}
}

impl Default for StereographicWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Stitcher]
// Stitcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:132
pub trait StitcherTraitConst {
	fn as_raw_Stitcher(&self) -> *const c_void;

	// registrationResol()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:177
	// ("cv::Stitcher::registrationResol", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn registration_resol(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_registrationResol_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// seamEstimationResol()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:180
	// ("cv::Stitcher::seamEstimationResol", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn seam_estimation_resol(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_seamEstimationResol_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// compositingResol()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:183
	// ("cv::Stitcher::compositingResol", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn compositing_resol(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_compositingResol_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// panoConfidenceThresh()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:186
	// ("cv::Stitcher::panoConfidenceThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pano_confidence_thresh(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_panoConfidenceThresh_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// waveCorrection()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:189
	// ("cv::Stitcher::waveCorrection", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn wave_correction(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_waveCorrection_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// waveCorrectKind()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:192
	// ("cv::Stitcher::waveCorrectKind", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn wave_correct_kind(&self) -> Result<crate::stitching::Detail_WaveCorrectKind> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_waveCorrectKind_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// featuresFinder()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:196
	// ("cv::Stitcher::featuresFinder", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn features_finder(&self) -> Result<core::Ptr<crate::stitching::Detail_FeaturesFinder>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_featuresFinder_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_FeaturesFinder>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// featuresMatcher()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:201
	// ("cv::Stitcher::featuresMatcher", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn features_matcher(&self) -> Result<core::Ptr<crate::stitching::Detail_FeaturesMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_featuresMatcher_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_FeaturesMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// matchingMask()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:205
	// ("cv::Stitcher::matchingMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn matching_mask(&self) -> Result<core::UMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_matchingMask_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// bundleAdjuster()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:213
	// ("cv::Stitcher::bundleAdjuster", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bundle_adjuster(&self) -> Result<core::Ptr<crate::stitching::Detail_BundleAdjusterBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_bundleAdjuster_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_BundleAdjusterBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// warper()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:225
	// ("cv::Stitcher::warper", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn warper(&self) -> Result<core::Ptr<crate::stitching::WarperCreator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_warper_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::WarperCreator>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// exposureCompensator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:229
	// ("cv::Stitcher::exposureCompensator", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn exposure_compensator(&self) -> Result<core::Ptr<crate::stitching::Detail_ExposureCompensator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_exposureCompensator_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// seamFinder()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:234
	// ("cv::Stitcher::seamFinder", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn seam_finder(&self) -> Result<core::Ptr<crate::stitching::Detail_SeamFinder>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_seamFinder_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_SeamFinder>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// blender()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:238
	// ("cv::Stitcher::blender", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn blender(&self) -> Result<core::Ptr<crate::stitching::Detail_Blender>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_blender_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// component()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:280
	// ("cv::Stitcher::component", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn component(&self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_component_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// cameras()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:281
	// ("cv::Stitcher::cameras", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cameras(&self) -> Result<core::Vector<crate::stitching::Detail_CameraParams>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_cameras_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::stitching::Detail_CameraParams>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// workScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:282
	// ("cv::Stitcher::workScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn work_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_workScale_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Stitcher]
pub trait StitcherTrait: crate::stitching::StitcherTraitConst {
	fn as_raw_mut_Stitcher(&mut self) -> *mut c_void;

	// setRegistrationResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:178
	// ("cv::Stitcher::setRegistrationResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
	#[inline]
	fn set_registration_resol(&mut self, resol_mpx: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setRegistrationResol_double(self.as_raw_mut_Stitcher(), resol_mpx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSeamEstimationResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:181
	// ("cv::Stitcher::setSeamEstimationResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
	#[inline]
	fn set_seam_estimation_resol(&mut self, resol_mpx: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setSeamEstimationResol_double(self.as_raw_mut_Stitcher(), resol_mpx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCompositingResol(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:184
	// ("cv::Stitcher::setCompositingResol", vec![(pred!(mut, ["resol_mpx"], ["double"]), _)]),
	#[inline]
	fn set_compositing_resol(&mut self, resol_mpx: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setCompositingResol_double(self.as_raw_mut_Stitcher(), resol_mpx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPanoConfidenceThresh(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:187
	// ("cv::Stitcher::setPanoConfidenceThresh", vec![(pred!(mut, ["conf_thresh"], ["double"]), _)]),
	#[inline]
	fn set_pano_confidence_thresh(&mut self, conf_thresh: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setPanoConfidenceThresh_double(self.as_raw_mut_Stitcher(), conf_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWaveCorrection(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:190
	// ("cv::Stitcher::setWaveCorrection", vec![(pred!(mut, ["flag"], ["bool"]), _)]),
	#[inline]
	fn set_wave_correction(&mut self, flag: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setWaveCorrection_bool(self.as_raw_mut_Stitcher(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWaveCorrectKind(detail::WaveCorrectKind)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:193
	// ("cv::Stitcher::setWaveCorrectKind", vec![(pred!(mut, ["kind"], ["cv::detail::WaveCorrectKind"]), _)]),
	#[inline]
	fn set_wave_correct_kind(&mut self, kind: crate::stitching::Detail_WaveCorrectKind) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(self.as_raw_mut_Stitcher(), kind, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// featuresFinder()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:195
	// ("cv::Stitcher::featuresFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn features_finder_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_FeaturesFinder>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_featuresFinder(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_FeaturesFinder>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setFeaturesFinder(Ptr<detail::FeaturesFinder>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:197
	// ("cv::Stitcher::setFeaturesFinder", vec![(pred!(mut, ["features_finder"], ["cv::Ptr<cv::detail::FeaturesFinder>"]), _)]),
	#[inline]
	fn set_features_finder(&mut self, mut features_finder: core::Ptr<crate::stitching::Detail_FeaturesFinder>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setFeaturesFinder_PtrLFeaturesFinderG(self.as_raw_mut_Stitcher(), features_finder.as_raw_mut_PtrOfDetail_FeaturesFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// featuresMatcher()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:200
	// ("cv::Stitcher::featuresMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn features_matcher_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_FeaturesMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_featuresMatcher(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_FeaturesMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setFeaturesMatcher(Ptr<detail::FeaturesMatcher>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:202
	// ("cv::Stitcher::setFeaturesMatcher", vec![(pred!(mut, ["features_matcher"], ["cv::Ptr<cv::detail::FeaturesMatcher>"]), _)]),
	#[inline]
	fn set_features_matcher(&mut self, mut features_matcher: core::Ptr<crate::stitching::Detail_FeaturesMatcher>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setFeaturesMatcher_PtrLFeaturesMatcherG(self.as_raw_mut_Stitcher(), features_matcher.as_raw_mut_PtrOfDetail_FeaturesMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMatchingMask(const cv::UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:206
	// ("cv::Stitcher::setMatchingMask", vec![(pred!(mut, ["mask"], ["const cv::UMat*"]), _)]),
	#[inline]
	fn set_matching_mask(&mut self, mask: &impl core::UMatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setMatchingMask_const_UMatR(self.as_raw_mut_Stitcher(), mask.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// bundleAdjuster()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:212
	// ("cv::Stitcher::bundleAdjuster", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn bundle_adjuster_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_BundleAdjusterBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_bundleAdjuster(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_BundleAdjusterBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setBundleAdjuster(Ptr<detail::BundleAdjusterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:214
	// ("cv::Stitcher::setBundleAdjuster", vec![(pred!(mut, ["bundle_adjuster"], ["cv::Ptr<cv::detail::BundleAdjusterBase>"]), _)]),
	#[inline]
	fn set_bundle_adjuster(&mut self, mut bundle_adjuster: core::Ptr<crate::stitching::Detail_BundleAdjusterBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setBundleAdjuster_PtrLBundleAdjusterBaseG(self.as_raw_mut_Stitcher(), bundle_adjuster.as_raw_mut_PtrOfDetail_BundleAdjusterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warper()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:224
	// ("cv::Stitcher::warper", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn warper_1(&mut self) -> Result<core::Ptr<crate::stitching::WarperCreator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_warper(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::WarperCreator>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setWarper(Ptr<WarperCreator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:226
	// ("cv::Stitcher::setWarper", vec![(pred!(mut, ["creator"], ["cv::Ptr<cv::WarperCreator>"]), _)]),
	#[inline]
	fn set_warper(&mut self, mut creator: core::Ptr<crate::stitching::WarperCreator>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setWarper_PtrLWarperCreatorG(self.as_raw_mut_Stitcher(), creator.as_raw_mut_PtrOfWarperCreator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// exposureCompensator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:228
	// ("cv::Stitcher::exposureCompensator", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn exposure_compensator_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_ExposureCompensator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_exposureCompensator(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setExposureCompensator(Ptr<detail::ExposureCompensator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:230
	// ("cv::Stitcher::setExposureCompensator", vec![(pred!(mut, ["exposure_comp"], ["cv::Ptr<cv::detail::ExposureCompensator>"]), _)]),
	#[inline]
	fn set_exposure_compensator(&mut self, mut exposure_comp: core::Ptr<crate::stitching::Detail_ExposureCompensator>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setExposureCompensator_PtrLExposureCompensatorG(self.as_raw_mut_Stitcher(), exposure_comp.as_raw_mut_PtrOfDetail_ExposureCompensator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// seamFinder()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:233
	// ("cv::Stitcher::seamFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn seam_finder_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_SeamFinder>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_seamFinder(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_SeamFinder>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setSeamFinder(Ptr<detail::SeamFinder>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:235
	// ("cv::Stitcher::setSeamFinder", vec![(pred!(mut, ["seam_finder"], ["cv::Ptr<cv::detail::SeamFinder>"]), _)]),
	#[inline]
	fn set_seam_finder(&mut self, mut seam_finder: core::Ptr<crate::stitching::Detail_SeamFinder>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setSeamFinder_PtrLSeamFinderG(self.as_raw_mut_Stitcher(), seam_finder.as_raw_mut_PtrOfDetail_SeamFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// blender()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:237
	// ("cv::Stitcher::blender", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn blender_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_Blender>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_blender(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// setBlender(Ptr<detail::Blender>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:239
	// ("cv::Stitcher::setBlender", vec![(pred!(mut, ["b"], ["cv::Ptr<cv::detail::Blender>"]), _)]),
	#[inline]
	fn set_blender(&mut self, mut b: core::Ptr<crate::stitching::Detail_Blender>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_setBlender_PtrLBlenderG(self.as_raw_mut_Stitcher(), b.as_raw_mut_PtrOfDetail_Blender(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// estimateTransform(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:242
	// ("cv::Stitcher::estimateTransform", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn estimate_transform(&mut self, images: &impl ToInputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_estimateTransform_const__InputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// estimateTransform(InputArrayOfArrays, const std::vector<std::vector<Rect>> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:252
	// ("cv::Stitcher::estimateTransform", vec![(pred!(mut, ["images", "rois"], ["const cv::_InputArray*", "const std::vector<std::vector<cv::Rect>>*"]), _)]),
	#[inline]
	fn estimate_transform_1(&mut self, images: &impl ToInputArray, rois: &core::Vector<core::Vector<core::Rect>>) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_estimateTransform_const__InputArrayR_const_vectorLvectorLRectGGR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), rois.as_raw_VectorOfVectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// composePanorama(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:255
	// ("cv::Stitcher::composePanorama", vec![(pred!(mut, ["pano"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compose_panorama(&mut self, pano: &mut impl ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		output_array_arg!(pano);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_composePanorama_const__OutputArrayR(self.as_raw_mut_Stitcher(), pano.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// composePanorama(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:267
	// ("cv::Stitcher::composePanorama", vec![(pred!(mut, ["images", "pano"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compose_panorama_images(&mut self, images: &impl ToInputArray, pano: &mut impl ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		output_array_arg!(pano);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), pano.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// stitch(InputArrayOfArrays, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:270
	// ("cv::Stitcher::stitch", vec![(pred!(mut, ["images", "pano"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn stitch(&mut self, images: &impl ToInputArray, pano: &mut impl ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		output_array_arg!(pano);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_stitch_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), pano.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// These functions try to stitch the given images.
	///
	/// ## Parameters
	/// * images: Input images.
	/// * rois: Region of interest rectangles.
	/// * pano: Final pano.
	/// ## Returns
	/// Status code.
	// stitch(InputArrayOfArrays, const std::vector<std::vector<Rect>> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:278
	// ("cv::Stitcher::stitch", vec![(pred!(mut, ["images", "rois", "pano"], ["const cv::_InputArray*", "const std::vector<std::vector<cv::Rect>>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn stitch_1(&mut self, images: &impl ToInputArray, rois: &core::Vector<core::Vector<core::Rect>>, pano: &mut impl ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
		input_array_arg!(images);
		output_array_arg!(pano);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_stitch_const__InputArrayR_const_vectorLvectorLRectGGR_const__OutputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), rois.as_raw_VectorOfVectorOfRect(), pano.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
// Stitcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:132
pub struct Stitcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { Stitcher }

impl Drop for Stitcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_Stitcher_delete(self.as_raw_mut_Stitcher()) };
	}
}

unsafe impl Send for Stitcher {}

impl crate::stitching::StitcherTraitConst for Stitcher {
	#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::StitcherTrait for Stitcher {
	#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Stitcher, crate::stitching::StitcherTraitConst, as_raw_Stitcher, crate::stitching::StitcherTrait, as_raw_mut_Stitcher }

impl Stitcher {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_Stitcher_defaultNew_const()) }
	}

	/// Creates a stitcher with the default parameters.
	///
	/// ## Parameters
	/// * try_use_gpu: Flag indicating whether GPU should be used whenever it's possible.
	/// ## Returns
	/// Stitcher class instance.
	///
	/// ## C++ default parameters
	/// * try_use_gpu: false
	// createDefault(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:166
	// ("cv::Stitcher::createDefault", vec![(pred!(mut, ["try_use_gpu"], ["bool"]), _)]),
	#[inline]
	pub fn create_default(try_use_gpu: bool) -> Result<crate::stitching::Stitcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_createDefault_bool(try_use_gpu, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Stitcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates a stitcher with the default parameters.
	///
	/// ## Parameters
	/// * try_use_gpu: Flag indicating whether GPU should be used whenever it's possible.
	/// ## Returns
	/// Stitcher class instance.
	///
	/// ## Note
	/// This alternative version of [Stitcher::create_default] function uses the following default values for its arguments:
	/// * try_use_gpu: false
	// cv::Stitcher::createDefault() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:166
	// ("cv::Stitcher::createDefault", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_default_def() -> Result<crate::stitching::Stitcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_createDefault(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Stitcher::opencv_from_extern(ret) };
		Ok(ret)
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
	// create(Mode, bool)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:175
	// ("cv::Stitcher::create", vec![(pred!(mut, ["mode", "try_use_gpu"], ["cv::Stitcher::Mode", "bool"]), _)]),
	#[inline]
	pub fn create(mode: crate::stitching::Stitcher_Mode, try_use_gpu: bool) -> Result<core::Ptr<crate::stitching::Stitcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_create_Mode_bool(mode, try_use_gpu, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Stitcher>::opencv_from_extern(ret) };
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [Stitcher::create] function uses the following default values for its arguments:
	/// * mode: PANORAMA
	/// * try_use_gpu: false
	// cv::Stitcher::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching.hpp:175
	// ("cv::Stitcher::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::stitching::Stitcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Stitcher_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Stitcher>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Stitcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Stitcher")
			.finish()
	}
}

impl Default for Stitcher {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::TransverseMercatorWarper]
// TransverseMercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:158
pub trait TransverseMercatorWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_TransverseMercatorWarper(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:161
	// ("cv::TransverseMercatorWarper::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TransverseMercatorWarper_create_const_float(self.as_raw_TransverseMercatorWarper(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::TransverseMercatorWarper]
pub trait TransverseMercatorWarperTrait: crate::stitching::TransverseMercatorWarperTraitConst + crate::stitching::WarperCreatorTrait {
	fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void;

}

// TransverseMercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:158
pub struct TransverseMercatorWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { TransverseMercatorWarper }

impl Drop for TransverseMercatorWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TransverseMercatorWarper_delete(self.as_raw_mut_TransverseMercatorWarper()) };
	}
}

unsafe impl Send for TransverseMercatorWarper {}

impl crate::stitching::WarperCreatorTraitConst for TransverseMercatorWarper {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for TransverseMercatorWarper {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TransverseMercatorWarper, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl crate::stitching::TransverseMercatorWarperTraitConst for TransverseMercatorWarper {
	#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::TransverseMercatorWarperTrait for TransverseMercatorWarper {
	#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TransverseMercatorWarper, crate::stitching::TransverseMercatorWarperTraitConst, as_raw_TransverseMercatorWarper, crate::stitching::TransverseMercatorWarperTrait, as_raw_mut_TransverseMercatorWarper }

impl TransverseMercatorWarper {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_TransverseMercatorWarper_defaultNew_const()) }
	}

}

boxed_cast_base! { TransverseMercatorWarper, crate::stitching::WarperCreator, cv_TransverseMercatorWarper_to_WarperCreator }

impl std::fmt::Debug for TransverseMercatorWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TransverseMercatorWarper")
			.finish()
	}
}

impl Default for TransverseMercatorWarper {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::WarperCreator]
// WarperCreator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:55
pub trait WarperCreatorTraitConst {
	fn as_raw_WarperCreator(&self) -> *const c_void;

	// create(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:59
	// ("cv::WarperCreator::create", vec![(pred!(const, ["scale"], ["float"]), _)]),
	#[inline]
	fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_WarperCreator_create_const_float(self.as_raw_WarperCreator(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::WarperCreator]
pub trait WarperCreatorTrait: crate::stitching::WarperCreatorTraitConst {
	fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void;

}

/// Image warper factories base class.
// WarperCreator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/warpers.hpp:55
pub struct WarperCreator {
	ptr: *mut c_void,
}

opencv_type_boxed! { WarperCreator }

impl Drop for WarperCreator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_WarperCreator_delete(self.as_raw_mut_WarperCreator()) };
	}
}

unsafe impl Send for WarperCreator {}

impl crate::stitching::WarperCreatorTraitConst for WarperCreator {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::WarperCreatorTrait for WarperCreator {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { WarperCreator, crate::stitching::WarperCreatorTraitConst, as_raw_WarperCreator, crate::stitching::WarperCreatorTrait, as_raw_mut_WarperCreator }

impl WarperCreator {
}

boxed_cast_descendant! { WarperCreator, crate::stitching::AffineWarper, cv_WarperCreator_to_AffineWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::CompressedRectilinearPortraitWarper, cv_WarperCreator_to_CompressedRectilinearPortraitWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::CompressedRectilinearWarper, cv_WarperCreator_to_CompressedRectilinearWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::CylindricalWarper, cv_WarperCreator_to_CylindricalWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::FisheyeWarper, cv_WarperCreator_to_FisheyeWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::MercatorWarper, cv_WarperCreator_to_MercatorWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::PaniniPortraitWarper, cv_WarperCreator_to_PaniniPortraitWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::PaniniWarper, cv_WarperCreator_to_PaniniWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::PlaneWarper, cv_WarperCreator_to_PlaneWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::SphericalWarper, cv_WarperCreator_to_SphericalWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::StereographicWarper, cv_WarperCreator_to_StereographicWarper }

boxed_cast_descendant! { WarperCreator, crate::stitching::TransverseMercatorWarper, cv_WarperCreator_to_TransverseMercatorWarper }

impl std::fmt::Debug for WarperCreator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("WarperCreator")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_AKAZEFeaturesFinder]
// AKAZEFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:175
pub trait Detail_AKAZEFeaturesFinderTraitConst: crate::stitching::Detail_FeaturesFinderTraitConst {
	fn as_raw_Detail_AKAZEFeaturesFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_AKAZEFeaturesFinder]
pub trait Detail_AKAZEFeaturesFinderTrait: crate::stitching::Detail_AKAZEFeaturesFinderTraitConst + crate::stitching::Detail_FeaturesFinderTrait {
	fn as_raw_mut_Detail_AKAZEFeaturesFinder(&mut self) -> *mut c_void;

}

/// AKAZE features finder. :
/// ## See also
/// detail::FeaturesFinder, AKAZE
// AKAZEFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:175
pub struct Detail_AKAZEFeaturesFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_AKAZEFeaturesFinder }

impl Drop for Detail_AKAZEFeaturesFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_AKAZEFeaturesFinder_delete(self.as_raw_mut_Detail_AKAZEFeaturesFinder()) };
	}
}

unsafe impl Send for Detail_AKAZEFeaturesFinder {}

impl crate::stitching::Detail_FeaturesFinderTraitConst for Detail_AKAZEFeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for Detail_AKAZEFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AKAZEFeaturesFinder, crate::stitching::Detail_FeaturesFinderTraitConst, as_raw_Detail_FeaturesFinder, crate::stitching::Detail_FeaturesFinderTrait, as_raw_mut_Detail_FeaturesFinder }

impl crate::stitching::Detail_AKAZEFeaturesFinderTraitConst for Detail_AKAZEFeaturesFinder {
	#[inline] fn as_raw_Detail_AKAZEFeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_AKAZEFeaturesFinderTrait for Detail_AKAZEFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_AKAZEFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AKAZEFeaturesFinder, crate::stitching::Detail_AKAZEFeaturesFinderTraitConst, as_raw_Detail_AKAZEFeaturesFinder, crate::stitching::Detail_AKAZEFeaturesFinderTrait, as_raw_mut_Detail_AKAZEFeaturesFinder }

impl Detail_AKAZEFeaturesFinder {
	/// ## C++ default parameters
	/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
	/// * descriptor_size: 0
	/// * descriptor_channels: 3
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	// AKAZEFeaturesFinder(int, int, int, float, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:178
	// ("cv::detail::AKAZEFeaturesFinder::AKAZEFeaturesFinder", vec![(pred!(mut, ["descriptor_type", "descriptor_size", "descriptor_channels", "threshold", "nOctaves", "nOctaveLayers", "diffusivity"], ["int", "int", "int", "float", "int", "int", "int"]), _)]),
	#[inline]
	pub fn new(descriptor_type: i32, descriptor_size: i32, descriptor_channels: i32, threshold: f32, n_octaves: i32, n_octave_layers: i32, diffusivity: i32) -> Result<crate::stitching::Detail_AKAZEFeaturesFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AKAZEFeaturesFinder_AKAZEFeaturesFinder_int_int_int_float_int_int_int(descriptor_type, descriptor_size, descriptor_channels, threshold, n_octaves, n_octave_layers, diffusivity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_AKAZEFeaturesFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * descriptor_type: AKAZE::DESCRIPTOR_MLDB
	/// * descriptor_size: 0
	/// * descriptor_channels: 3
	/// * threshold: 0.001f
	/// * n_octaves: 4
	/// * n_octave_layers: 4
	/// * diffusivity: KAZE::DIFF_PM_G2
	// cv::detail::AKAZEFeaturesFinder::AKAZEFeaturesFinder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:178
	// ("cv::detail::AKAZEFeaturesFinder::AKAZEFeaturesFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_AKAZEFeaturesFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AKAZEFeaturesFinder_AKAZEFeaturesFinder(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_AKAZEFeaturesFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_AKAZEFeaturesFinder, crate::stitching::Detail_FeaturesFinder, cv_detail_AKAZEFeaturesFinder_to_Detail_FeaturesFinder }

impl std::fmt::Debug for Detail_AKAZEFeaturesFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_AKAZEFeaturesFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_AffineBasedEstimator]
// AffineBasedEstimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:119
pub trait Detail_AffineBasedEstimatorTraitConst: crate::stitching::Detail_EstimatorTraitConst {
	fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_AffineBasedEstimator]
pub trait Detail_AffineBasedEstimatorTrait: crate::stitching::Detail_AffineBasedEstimatorTraitConst + crate::stitching::Detail_EstimatorTrait {
	fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void;

}

/// Affine transformation based estimator.
///
/// This estimator uses pairwise transformations estimated by matcher to estimate
/// final transformation for each camera.
/// ## See also
/// cv::detail::HomographyBasedEstimator
// AffineBasedEstimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:119
pub struct Detail_AffineBasedEstimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_AffineBasedEstimator }

impl Drop for Detail_AffineBasedEstimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_AffineBasedEstimator_delete(self.as_raw_mut_Detail_AffineBasedEstimator()) };
	}
}

unsafe impl Send for Detail_AffineBasedEstimator {}

impl crate::stitching::Detail_EstimatorTraitConst for Detail_AffineBasedEstimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_AffineBasedEstimator {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineBasedEstimator, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_AffineBasedEstimatorTraitConst for Detail_AffineBasedEstimator {
	#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_AffineBasedEstimatorTrait for Detail_AffineBasedEstimator {
	#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineBasedEstimator, crate::stitching::Detail_AffineBasedEstimatorTraitConst, as_raw_Detail_AffineBasedEstimator, crate::stitching::Detail_AffineBasedEstimatorTrait, as_raw_mut_Detail_AffineBasedEstimator }

impl Detail_AffineBasedEstimator {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_AffineBasedEstimator_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_AffineBasedEstimator, crate::stitching::Detail_Estimator, cv_detail_AffineBasedEstimator_to_Detail_Estimator }

impl std::fmt::Debug for Detail_AffineBasedEstimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_AffineBasedEstimator")
			.finish()
	}
}

impl Default for Detail_AffineBasedEstimator {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_AffineBestOf2NearestMatcher]
// AffineBestOf2NearestMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:339
pub trait Detail_AffineBestOf2NearestMatcherTraitConst: crate::stitching::Detail_BestOf2NearestMatcherTraitConst {
	fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_AffineBestOf2NearestMatcher]
pub trait Detail_AffineBestOf2NearestMatcherTrait: crate::stitching::Detail_AffineBestOf2NearestMatcherTraitConst + crate::stitching::Detail_BestOf2NearestMatcherTrait {
	fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void;

}

/// Features matcher similar to cv::detail::BestOf2NearestMatcher which
/// finds two best matches for each feature and leaves the best one only if the
/// ratio between descriptor distances is greater than the threshold match_conf.
///
/// Unlike cv::detail::BestOf2NearestMatcher this matcher uses affine
/// transformation (affine transformation estimate will be placed in matches_info).
/// ## See also
/// cv::detail::FeaturesMatcher cv::detail::BestOf2NearestMatcher
// AffineBestOf2NearestMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:339
pub struct Detail_AffineBestOf2NearestMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_AffineBestOf2NearestMatcher }

impl Drop for Detail_AffineBestOf2NearestMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_AffineBestOf2NearestMatcher_delete(self.as_raw_mut_Detail_AffineBestOf2NearestMatcher()) };
	}
}

unsafe impl Send for Detail_AffineBestOf2NearestMatcher {}

impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineBestOf2NearestMatcher, crate::stitching::Detail_BestOf2NearestMatcherTraitConst, as_raw_Detail_BestOf2NearestMatcher, crate::stitching::Detail_BestOf2NearestMatcherTrait, as_raw_mut_Detail_BestOf2NearestMatcher }

impl crate::stitching::Detail_FeaturesMatcherTraitConst for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcherTrait for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineBestOf2NearestMatcher, crate::stitching::Detail_FeaturesMatcherTraitConst, as_raw_Detail_FeaturesMatcher, crate::stitching::Detail_FeaturesMatcherTrait, as_raw_mut_Detail_FeaturesMatcher }

impl crate::stitching::Detail_AffineBestOf2NearestMatcherTraitConst for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_AffineBestOf2NearestMatcherTrait for Detail_AffineBestOf2NearestMatcher {
	#[inline] fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineBestOf2NearestMatcher, crate::stitching::Detail_AffineBestOf2NearestMatcherTraitConst, as_raw_Detail_AffineBestOf2NearestMatcher, crate::stitching::Detail_AffineBestOf2NearestMatcherTrait, as_raw_mut_Detail_AffineBestOf2NearestMatcher }

impl Detail_AffineBestOf2NearestMatcher {
	/// Constructs a "best of 2 nearest" matcher that expects affine transformation
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
	// AffineBestOf2NearestMatcher(bool, bool, float, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:354
	// ("cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher", vec![(pred!(mut, ["full_affine", "try_use_gpu", "match_conf", "num_matches_thresh1"], ["bool", "bool", "float", "int"]), _)]),
	#[inline]
	pub fn new(full_affine: bool, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32) -> Result<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(full_affine, try_use_gpu, match_conf, num_matches_thresh1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_AffineBestOf2NearestMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructs a "best of 2 nearest" matcher that expects affine transformation
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
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * full_affine: false
	/// * try_use_gpu: false
	/// * match_conf: 0.3f
	/// * num_matches_thresh1: 6
	// cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:354
	// ("cv::detail::AffineBestOf2NearestMatcher::AffineBestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_AffineBestOf2NearestMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_AffineBestOf2NearestMatcher, crate::stitching::Detail_BestOf2NearestMatcher, cv_detail_AffineBestOf2NearestMatcher_to_Detail_BestOf2NearestMatcher }

boxed_cast_base! { Detail_AffineBestOf2NearestMatcher, crate::stitching::Detail_FeaturesMatcher, cv_detail_AffineBestOf2NearestMatcher_to_Detail_FeaturesMatcher }

impl std::fmt::Debug for Detail_AffineBestOf2NearestMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_AffineBestOf2NearestMatcher")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_AffineWarper]
// AffineWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:213
pub trait Detail_AffineWarperTraitConst: crate::stitching::Detail_PlaneWarperTraitConst {
	fn as_raw_Detail_AffineWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_AffineWarper]
pub trait Detail_AffineWarperTrait: crate::stitching::Detail_AffineWarperTraitConst + crate::stitching::Detail_PlaneWarperTrait {
	fn as_raw_mut_Detail_AffineWarper(&mut self) -> *mut c_void;

	/// Projects the image point.
	///
	/// ## Parameters
	/// * pt: Source point
	/// * K: Camera intrinsic parameters
	/// * H: Camera extrinsic parameters
	/// ## Returns
	/// Projected point
	// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:229
	// ("cv::detail::AffineWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "H"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_point(&mut self, pt: core::Point2f, k: &impl ToInputArray, h: &impl ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(h);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_AffineWarper(), &pt, k.as_raw__InputArray(), h.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Builds the projection maps according to the given camera data.
	///
	/// ## Parameters
	/// * src_size: Source image size
	/// * K: Camera intrinsic parameters
	/// * H: Camera extrinsic parameters
	/// * xmap: Projection map for the x axis
	/// * ymap: Projection map for the y axis
	/// ## Returns
	/// Projected image minimum bounding box
	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:240
	// ("cv::detail::AffineWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "H", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, h: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(h);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_AffineWarper(), &src_size, k.as_raw__InputArray(), h.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Projects the image.
	///
	/// ## Parameters
	/// * src: Source image
	/// * K: Camera intrinsic parameters
	/// * H: Camera extrinsic parameters
	/// * interp_mode: Interpolation mode
	/// * border_mode: Border extrapolation mode
	/// * dst: Projected image
	/// ## Returns
	/// Project image top-left corner
	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:252
	// ("cv::detail::AffineWarper::warp", vec![(pred!(mut, ["src", "K", "H", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, h: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(h);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_AffineWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), h.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Parameters
	/// * src_size: Source image bounding box
	/// * K: Camera intrinsic parameters
	/// * H: Camera extrinsic parameters
	/// ## Returns
	/// Projected image minimum bounding box
	// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:261
	// ("cv::detail::AffineWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "H"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_roi(&mut self, src_size: core::Size, k: &impl ToInputArray, h: &impl ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(h);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_AffineWarper(), &src_size, k.as_raw__InputArray(), h.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Affine warper that uses rotations and translations
///
/// Uses affine transformation in homogeneous coordinates to represent both rotation and
/// translation in camera rotation matrix.
// AffineWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:213
pub struct Detail_AffineWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_AffineWarper }

impl Drop for Detail_AffineWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_AffineWarper_delete(self.as_raw_mut_Detail_AffineWarper()) };
	}
}

unsafe impl Send for Detail_AffineWarper {}

impl crate::stitching::Detail_PlaneWarperTraitConst for Detail_AffineWarper {
	#[inline] fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PlaneWarperTrait for Detail_AffineWarper {
	#[inline] fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineWarper, crate::stitching::Detail_PlaneWarperTraitConst, as_raw_Detail_PlaneWarper, crate::stitching::Detail_PlaneWarperTrait, as_raw_mut_Detail_PlaneWarper }

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_AffineWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_AffineWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_AffineWarperTraitConst for Detail_AffineWarper {
	#[inline] fn as_raw_Detail_AffineWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_AffineWarperTrait for Detail_AffineWarper {
	#[inline] fn as_raw_mut_Detail_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_AffineWarper, crate::stitching::Detail_AffineWarperTraitConst, as_raw_Detail_AffineWarper, crate::stitching::Detail_AffineWarperTrait, as_raw_mut_Detail_AffineWarper }

impl Detail_AffineWarper {
	/// Construct an instance of the affine warper class.
	///
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	///
	/// ## C++ default parameters
	/// * scale: 1.f
	// AffineWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:220
	// ("cv::detail::AffineWarper::AffineWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_AffineWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineWarper_AffineWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_AffineWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Construct an instance of the affine warper class.
	///
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * scale: 1.f
	// cv::detail::AffineWarper::AffineWarper() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:220
	// ("cv::detail::AffineWarper::AffineWarper", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_AffineWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_AffineWarper_AffineWarper(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_AffineWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_AffineWarper, crate::stitching::Detail_PlaneWarper, cv_detail_AffineWarper_to_Detail_PlaneWarper }

boxed_cast_base! { Detail_AffineWarper, crate::stitching::Detail_RotationWarper, cv_detail_AffineWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_AffineWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_AffineWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_BestOf2NearestMatcher]
// BestOf2NearestMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:291
pub trait Detail_BestOf2NearestMatcherTraitConst: crate::stitching::Detail_FeaturesMatcherTraitConst {
	fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_BestOf2NearestMatcher]
pub trait Detail_BestOf2NearestMatcherTrait: crate::stitching::Detail_BestOf2NearestMatcherTraitConst + crate::stitching::Detail_FeaturesMatcherTrait {
	fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void;

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:306
	// ("cv::detail::BestOf2NearestMatcher::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BestOf2NearestMatcher_collectGarbage(self.as_raw_mut_Detail_BestOf2NearestMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Features matcher which finds two best matches for each feature and leaves the best one only if the
/// ratio between descriptor distances is greater than the threshold match_conf
/// ## See also
/// detail::FeaturesMatcher
// BestOf2NearestMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:291
pub struct Detail_BestOf2NearestMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BestOf2NearestMatcher }

impl Drop for Detail_BestOf2NearestMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BestOf2NearestMatcher_delete(self.as_raw_mut_Detail_BestOf2NearestMatcher()) };
	}
}

unsafe impl Send for Detail_BestOf2NearestMatcher {}

impl crate::stitching::Detail_FeaturesMatcherTraitConst for Detail_BestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcherTrait for Detail_BestOf2NearestMatcher {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BestOf2NearestMatcher, crate::stitching::Detail_FeaturesMatcherTraitConst, as_raw_Detail_FeaturesMatcher, crate::stitching::Detail_FeaturesMatcherTrait, as_raw_mut_Detail_FeaturesMatcher }

impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for Detail_BestOf2NearestMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for Detail_BestOf2NearestMatcher {
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BestOf2NearestMatcher, crate::stitching::Detail_BestOf2NearestMatcherTraitConst, as_raw_Detail_BestOf2NearestMatcher, crate::stitching::Detail_BestOf2NearestMatcherTrait, as_raw_mut_Detail_BestOf2NearestMatcher }

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
	// BestOf2NearestMatcher(bool, float, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:303
	// ("cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher", vec![(pred!(mut, ["try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2"], ["bool", "float", "int", "int"]), _)]),
	#[inline]
	pub fn new(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32) -> Result<crate::stitching::Detail_BestOf2NearestMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BestOf2NearestMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

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
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * try_use_gpu: false
	/// * match_conf: 0.3f
	/// * num_matches_thresh1: 6
	/// * num_matches_thresh2: 6
	// cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:303
	// ("cv::detail::BestOf2NearestMatcher::BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_BestOf2NearestMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BestOf2NearestMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Detail_BestOf2NearestMatcher, crate::stitching::Detail_AffineBestOf2NearestMatcher, cv_detail_BestOf2NearestMatcher_to_Detail_AffineBestOf2NearestMatcher }

boxed_cast_descendant! { Detail_BestOf2NearestMatcher, crate::stitching::Detail_BestOf2NearestRangeMatcher, cv_detail_BestOf2NearestMatcher_to_Detail_BestOf2NearestRangeMatcher }

boxed_cast_base! { Detail_BestOf2NearestMatcher, crate::stitching::Detail_FeaturesMatcher, cv_detail_BestOf2NearestMatcher_to_Detail_FeaturesMatcher }

impl std::fmt::Debug for Detail_BestOf2NearestMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BestOf2NearestMatcher")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_BestOf2NearestRangeMatcher]
// BestOf2NearestRangeMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:316
pub trait Detail_BestOf2NearestRangeMatcherTraitConst: crate::stitching::Detail_BestOf2NearestMatcherTraitConst {
	fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_BestOf2NearestRangeMatcher]
pub trait Detail_BestOf2NearestRangeMatcherTrait: crate::stitching::Detail_BestOf2NearestMatcherTrait + crate::stitching::Detail_BestOf2NearestRangeMatcherTraitConst {
	fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * mask: cv::UMat()
	// operator()(const std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, const cv::UMat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:322
	// ("cv::detail::BestOf2NearestRangeMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches", "mask"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*", "const cv::UMat*"]), _)]),
	#[inline]
	fn apply(&mut self, features: &core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>, mask: &impl core::UMatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BestOf2NearestRangeMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR_const_UMatR(self.as_raw_mut_Detail_BestOf2NearestRangeMatcher(), features.as_raw_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), mask.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Detail_BestOf2NearestRangeMatcherTrait::apply] function uses the following default values for its arguments:
	/// * mask: cv::UMat()
	// cv::detail::BestOf2NearestRangeMatcher::operator()(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:322
	// ("cv::detail::BestOf2NearestRangeMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*"]), _)]),
	#[inline]
	fn apply_def(&mut self, features: &core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BestOf2NearestRangeMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR(self.as_raw_mut_Detail_BestOf2NearestRangeMatcher(), features.as_raw_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// BestOf2NearestRangeMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:316
pub struct Detail_BestOf2NearestRangeMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BestOf2NearestRangeMatcher }

impl Drop for Detail_BestOf2NearestRangeMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BestOf2NearestRangeMatcher_delete(self.as_raw_mut_Detail_BestOf2NearestRangeMatcher()) };
	}
}

unsafe impl Send for Detail_BestOf2NearestRangeMatcher {}

impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BestOf2NearestMatcherTrait for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BestOf2NearestRangeMatcher, crate::stitching::Detail_BestOf2NearestMatcherTraitConst, as_raw_Detail_BestOf2NearestMatcher, crate::stitching::Detail_BestOf2NearestMatcherTrait, as_raw_mut_Detail_BestOf2NearestMatcher }

impl crate::stitching::Detail_FeaturesMatcherTraitConst for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcherTrait for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BestOf2NearestRangeMatcher, crate::stitching::Detail_FeaturesMatcherTraitConst, as_raw_Detail_FeaturesMatcher, crate::stitching::Detail_FeaturesMatcherTrait, as_raw_mut_Detail_FeaturesMatcher }

impl crate::stitching::Detail_BestOf2NearestRangeMatcherTraitConst for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BestOf2NearestRangeMatcherTrait for Detail_BestOf2NearestRangeMatcher {
	#[inline] fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BestOf2NearestRangeMatcher, crate::stitching::Detail_BestOf2NearestRangeMatcherTraitConst, as_raw_Detail_BestOf2NearestRangeMatcher, crate::stitching::Detail_BestOf2NearestRangeMatcherTrait, as_raw_mut_Detail_BestOf2NearestRangeMatcher }

impl Detail_BestOf2NearestRangeMatcher {
	/// ## C++ default parameters
	/// * range_width: 5
	/// * try_use_gpu: false
	/// * match_conf: 0.3f
	/// * num_matches_thresh1: 6
	/// * num_matches_thresh2: 6
	// BestOf2NearestRangeMatcher(int, bool, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:319
	// ("cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher", vec![(pred!(mut, ["range_width", "try_use_gpu", "match_conf", "num_matches_thresh1", "num_matches_thresh2"], ["int", "bool", "float", "int", "int"]), _)]),
	#[inline]
	pub fn new(range_width: i32, try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32) -> Result<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(range_width, try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BestOf2NearestRangeMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * range_width: 5
	/// * try_use_gpu: false
	/// * match_conf: 0.3f
	/// * num_matches_thresh1: 6
	/// * num_matches_thresh2: 6
	// cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:319
	// ("cv::detail::BestOf2NearestRangeMatcher::BestOf2NearestRangeMatcher", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BestOf2NearestRangeMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_BestOf2NearestRangeMatcher, crate::stitching::Detail_BestOf2NearestMatcher, cv_detail_BestOf2NearestRangeMatcher_to_Detail_BestOf2NearestMatcher }

boxed_cast_base! { Detail_BestOf2NearestRangeMatcher, crate::stitching::Detail_FeaturesMatcher, cv_detail_BestOf2NearestRangeMatcher_to_Detail_FeaturesMatcher }

impl std::fmt::Debug for Detail_BestOf2NearestRangeMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BestOf2NearestRangeMatcher")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_Blender]
// Blender /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:63
pub trait Detail_BlenderTraitConst {
	fn as_raw_Detail_Blender(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_Blender]
pub trait Detail_BlenderTrait: crate::stitching::Detail_BlenderTraitConst {
	fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void;

	/// Prepares the blender for blending.
	///
	/// ## Parameters
	/// * corners: Source images top-left corners
	/// * sizes: Source image sizes
	// prepare(const std::vector<Point> &, const std::vector<Size> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:76
	// ("cv::detail::Blender::prepare", vec![(pred!(mut, ["corners", "sizes"], ["const std::vector<cv::Point>*", "const std::vector<cv::Size>*"]), _)]),
	#[inline]
	fn prepare(&mut self, corners: &core::Vector<core::Point>, sizes: &core::Vector<core::Size>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Blender_prepare_const_vectorLPointGR_const_vectorLSizeGR(self.as_raw_mut_Detail_Blender(), corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Prepares the blender for blending.
	///
	/// ## Parameters
	/// * corners: Source images top-left corners
	/// * sizes: Source image sizes
	///
	/// ## Overloaded parameters
	// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:78
	// ("cv::detail::Blender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
	#[inline]
	fn prepare_1(&mut self, dst_roi: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Blender_prepare_Rect(self.as_raw_mut_Detail_Blender(), &dst_roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Processes the image.
	///
	/// ## Parameters
	/// * img: Source image
	/// * mask: Source image mask
	/// * tl: Source image top-left corners
	// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:85
	// ("cv::detail::Blender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	#[inline]
	fn feed(&mut self, img: &impl ToInputArray, mask: &impl ToInputArray, tl: core::Point) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_Blender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), &tl, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Blends and returns the final pano.
	///
	/// ## Parameters
	/// * dst: Final pano
	/// * dst_mask: Final pano mask
	// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:91
	// ("cv::detail::Blender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn blend(&mut self, dst: &mut impl ToInputOutputArray, dst_mask: &mut impl ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(dst);
		input_output_array_arg!(dst_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Blender_blend_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Detail_Blender(), dst.as_raw__InputOutputArray(), dst_mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for all blenders.
///
/// Simple blender which puts one image over another
// Blender /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:63
pub struct Detail_Blender {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_Blender }

impl Drop for Detail_Blender {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_Blender_delete(self.as_raw_mut_Detail_Blender()) };
	}
}

unsafe impl Send for Detail_Blender {}

impl crate::stitching::Detail_BlenderTraitConst for Detail_Blender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for Detail_Blender {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_Blender, crate::stitching::Detail_BlenderTraitConst, as_raw_Detail_Blender, crate::stitching::Detail_BlenderTrait, as_raw_mut_Detail_Blender }

impl Detail_Blender {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_Blender_defaultNew_const()) }
	}

	/// ## C++ default parameters
	/// * try_gpu: false
	// createDefault(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:69
	// ("cv::detail::Blender::createDefault", vec![(pred!(mut, ["type", "try_gpu"], ["int", "bool"]), _)]),
	#[inline]
	pub fn create_default(typ: i32, try_gpu: bool) -> Result<core::Ptr<crate::stitching::Detail_Blender>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Blender_createDefault_int_bool(typ, try_gpu, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Detail_Blender::create_default] function uses the following default values for its arguments:
	/// * try_gpu: false
	// cv::detail::Blender::createDefault(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:69
	// ("cv::detail::Blender::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
	#[inline]
	pub fn create_default_def(typ: i32) -> Result<core::Ptr<crate::stitching::Detail_Blender>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Blender_createDefault_int(typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Detail_Blender, crate::stitching::Detail_FeatherBlender, cv_detail_Blender_to_Detail_FeatherBlender }

boxed_cast_descendant! { Detail_Blender, crate::stitching::Detail_MultiBandBlender, cv_detail_Blender_to_Detail_MultiBandBlender }

impl std::fmt::Debug for Detail_Blender {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_Blender")
			.finish()
	}
}

impl Default for Detail_Blender {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_BlocksGainCompensator]
// BlocksGainCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:117
pub trait Detail_BlocksGainCompensatorTraitConst: crate::stitching::Detail_ExposureCompensatorTraitConst {
	fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_BlocksGainCompensator]
pub trait Detail_BlocksGainCompensatorTrait: crate::stitching::Detail_BlocksGainCompensatorTraitConst + crate::stitching::Detail_ExposureCompensatorTrait {
	fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void;

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:122
	// ("cv::detail::BlocksGainCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	#[inline]
	fn feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BlocksGainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_BlocksGainCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:124
	// ("cv::detail::BlocksGainCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl ToInputOutputArray, mask: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_BlocksGainCompensator(), index, &corner, image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Exposure compensator which tries to remove exposure related artifacts by adjusting image block
/// intensities, see [UES01](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_UES01) for details.
// BlocksGainCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:117
pub struct Detail_BlocksGainCompensator {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BlocksGainCompensator }

impl Drop for Detail_BlocksGainCompensator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BlocksGainCompensator_delete(self.as_raw_mut_Detail_BlocksGainCompensator()) };
	}
}

unsafe impl Send for Detail_BlocksGainCompensator {}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_BlocksGainCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_BlocksGainCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BlocksGainCompensator, crate::stitching::Detail_ExposureCompensatorTraitConst, as_raw_Detail_ExposureCompensator, crate::stitching::Detail_ExposureCompensatorTrait, as_raw_mut_Detail_ExposureCompensator }

impl crate::stitching::Detail_BlocksGainCompensatorTraitConst for Detail_BlocksGainCompensator {
	#[inline] fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BlocksGainCompensatorTrait for Detail_BlocksGainCompensator {
	#[inline] fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BlocksGainCompensator, crate::stitching::Detail_BlocksGainCompensatorTraitConst, as_raw_Detail_BlocksGainCompensator, crate::stitching::Detail_BlocksGainCompensatorTrait, as_raw_mut_Detail_BlocksGainCompensator }

impl Detail_BlocksGainCompensator {
	/// ## C++ default parameters
	/// * bl_width: 32
	/// * bl_height: 32
	// BlocksGainCompensator(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:120
	// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, ["bl_width", "bl_height"], ["int", "int"]), _)]),
	#[inline]
	pub fn new(bl_width: i32, bl_height: i32) -> Result<crate::stitching::Detail_BlocksGainCompensator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(bl_width, bl_height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BlocksGainCompensator::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * bl_width: 32
	/// * bl_height: 32
	// cv::detail::BlocksGainCompensator::BlocksGainCompensator() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:120
	// ("cv::detail::BlocksGainCompensator::BlocksGainCompensator", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_BlocksGainCompensator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BlocksGainCompensator_BlocksGainCompensator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BlocksGainCompensator::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_BlocksGainCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_BlocksGainCompensator_to_Detail_ExposureCompensator }

impl std::fmt::Debug for Detail_BlocksGainCompensator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BlocksGainCompensator")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_BundleAdjusterAffine]
// BundleAdjusterAffine /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:285
pub trait Detail_BundleAdjusterAffineTraitConst: crate::stitching::Detail_BundleAdjusterBaseTraitConst {
	fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_BundleAdjusterAffine]
pub trait Detail_BundleAdjusterAffineTrait: crate::stitching::Detail_BundleAdjusterAffineTraitConst + crate::stitching::Detail_BundleAdjusterBaseTrait {
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
// BundleAdjusterAffine /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:285
pub struct Detail_BundleAdjusterAffine {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BundleAdjusterAffine }

impl Drop for Detail_BundleAdjusterAffine {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BundleAdjusterAffine_delete(self.as_raw_mut_Detail_BundleAdjusterAffine()) };
	}
}

unsafe impl Send for Detail_BundleAdjusterAffine {}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterAffine, crate::stitching::Detail_BundleAdjusterBaseTraitConst, as_raw_Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterBaseTrait, as_raw_mut_Detail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterAffine, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_BundleAdjusterAffineTraitConst for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterAffineTrait for Detail_BundleAdjusterAffine {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterAffine, crate::stitching::Detail_BundleAdjusterAffineTraitConst, as_raw_Detail_BundleAdjusterAffine, crate::stitching::Detail_BundleAdjusterAffineTrait, as_raw_mut_Detail_BundleAdjusterAffine }

impl Detail_BundleAdjusterAffine {
	// BundleAdjusterAffine()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:288
	// ("cv::detail::BundleAdjusterAffine::BundleAdjusterAffine", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterAffine> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterAffine_BundleAdjusterAffine(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BundleAdjusterAffine::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_BundleAdjusterAffine, crate::stitching::Detail_BundleAdjusterBase, cv_detail_BundleAdjusterAffine_to_Detail_BundleAdjusterBase }

boxed_cast_base! { Detail_BundleAdjusterAffine, crate::stitching::Detail_Estimator, cv_detail_BundleAdjusterAffine_to_Detail_Estimator }

impl std::fmt::Debug for Detail_BundleAdjusterAffine {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BundleAdjusterAffine")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_BundleAdjusterAffinePartial]
// BundleAdjusterAffinePartial /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:309
pub trait Detail_BundleAdjusterAffinePartialTraitConst: crate::stitching::Detail_BundleAdjusterBaseTraitConst {
	fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_BundleAdjusterAffinePartial]
pub trait Detail_BundleAdjusterAffinePartialTrait: crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst + crate::stitching::Detail_BundleAdjusterBaseTrait {
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
// BundleAdjusterAffinePartial /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:309
pub struct Detail_BundleAdjusterAffinePartial {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BundleAdjusterAffinePartial }

impl Drop for Detail_BundleAdjusterAffinePartial {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BundleAdjusterAffinePartial_delete(self.as_raw_mut_Detail_BundleAdjusterAffinePartial()) };
	}
}

unsafe impl Send for Detail_BundleAdjusterAffinePartial {}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterAffinePartial, crate::stitching::Detail_BundleAdjusterBaseTraitConst, as_raw_Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterBaseTrait, as_raw_mut_Detail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterAffinePartial, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for Detail_BundleAdjusterAffinePartial {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterAffinePartial, crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst, as_raw_Detail_BundleAdjusterAffinePartial, crate::stitching::Detail_BundleAdjusterAffinePartialTrait, as_raw_mut_Detail_BundleAdjusterAffinePartial }

impl Detail_BundleAdjusterAffinePartial {
	// BundleAdjusterAffinePartial()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:312
	// ("cv::detail::BundleAdjusterAffinePartial::BundleAdjusterAffinePartial", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BundleAdjusterAffinePartial::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_BundleAdjusterAffinePartial, crate::stitching::Detail_BundleAdjusterBase, cv_detail_BundleAdjusterAffinePartial_to_Detail_BundleAdjusterBase }

boxed_cast_base! { Detail_BundleAdjusterAffinePartial, crate::stitching::Detail_Estimator, cv_detail_BundleAdjusterAffinePartial_to_Detail_Estimator }

impl std::fmt::Debug for Detail_BundleAdjusterAffinePartial {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BundleAdjusterAffinePartial")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_BundleAdjusterBase]
// BundleAdjusterBase /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:129
pub trait Detail_BundleAdjusterBaseTraitConst: crate::stitching::Detail_EstimatorTraitConst {
	fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void;

	// refinementMask()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:132
	// ("cv::detail::BundleAdjusterBase::refinementMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn refinement_mask(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterBase_refinementMask_const(self.as_raw_Detail_BundleAdjusterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// confThresh()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:139
	// ("cv::detail::BundleAdjusterBase::confThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn conf_thresh(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterBase_confThresh_const(self.as_raw_Detail_BundleAdjusterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_BundleAdjusterBase]
pub trait Detail_BundleAdjusterBaseTrait: crate::stitching::Detail_BundleAdjusterBaseTraitConst + crate::stitching::Detail_EstimatorTrait {
	fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void;

	// setRefinementMask(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:133
	// ("cv::detail::BundleAdjusterBase::setRefinementMask", vec![(pred!(mut, ["mask"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_refinement_mask(&mut self, mask: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(self.as_raw_mut_Detail_BundleAdjusterBase(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setConfThresh(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:140
	// ("cv::detail::BundleAdjusterBase::setConfThresh", vec![(pred!(mut, ["conf_thresh"], ["double"]), _)]),
	#[inline]
	fn set_conf_thresh(&mut self, conf_thresh: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterBase_setConfThresh_double(self.as_raw_mut_Detail_BundleAdjusterBase(), conf_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// termCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:142
	// ("cv::detail::BundleAdjusterBase::termCriteria", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn term_criteria(&mut self) -> Result<core::TermCriteria> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterBase_termCriteria(self.as_raw_mut_Detail_BundleAdjusterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:143
	// ("cv::detail::BundleAdjusterBase::setTermCriteria", vec![(pred!(mut, ["term_criteria"], ["const cv::TermCriteria*"]), _)]),
	#[inline]
	fn set_term_criteria(&mut self, term_criteria: core::TermCriteria) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaR(self.as_raw_mut_Detail_BundleAdjusterBase(), &term_criteria, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for all camera parameters refinement methods.
// BundleAdjusterBase /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:129
pub struct Detail_BundleAdjusterBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BundleAdjusterBase }

impl Drop for Detail_BundleAdjusterBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BundleAdjusterBase_delete(self.as_raw_mut_Detail_BundleAdjusterBase()) };
	}
}

unsafe impl Send for Detail_BundleAdjusterBase {}

impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterBase {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterBase {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterBase, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for Detail_BundleAdjusterBase {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for Detail_BundleAdjusterBase {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterBaseTraitConst, as_raw_Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterBaseTrait, as_raw_mut_Detail_BundleAdjusterBase }

impl Detail_BundleAdjusterBase {
}

boxed_cast_descendant! { Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterAffine, cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterAffine }

boxed_cast_descendant! { Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterAffinePartial, cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterAffinePartial }

boxed_cast_descendant! { Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterRay, cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterRay }

boxed_cast_descendant! { Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterReproj, cv_detail_BundleAdjusterBase_to_Detail_BundleAdjusterReproj }

boxed_cast_descendant! { Detail_BundleAdjusterBase, crate::stitching::Detail_NoBundleAdjuster, cv_detail_BundleAdjusterBase_to_Detail_NoBundleAdjuster }

boxed_cast_base! { Detail_BundleAdjusterBase, crate::stitching::Detail_Estimator, cv_detail_BundleAdjusterBase_to_Detail_Estimator }

impl std::fmt::Debug for Detail_BundleAdjusterBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BundleAdjusterBase")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_BundleAdjusterRay]
// BundleAdjusterRay /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:261
pub trait Detail_BundleAdjusterRayTraitConst: crate::stitching::Detail_BundleAdjusterBaseTraitConst {
	fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_BundleAdjusterRay]
pub trait Detail_BundleAdjusterRayTrait: crate::stitching::Detail_BundleAdjusterBaseTrait + crate::stitching::Detail_BundleAdjusterRayTraitConst {
	fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void;

}

/// Implementation of the camera parameters refinement algorithm which minimizes sum of the distances
/// between the rays passing through the camera center and a feature. :
///
/// It can estimate focal length. It ignores the refinement mask for now.
// BundleAdjusterRay /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:261
pub struct Detail_BundleAdjusterRay {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BundleAdjusterRay }

impl Drop for Detail_BundleAdjusterRay {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BundleAdjusterRay_delete(self.as_raw_mut_Detail_BundleAdjusterRay()) };
	}
}

unsafe impl Send for Detail_BundleAdjusterRay {}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterRay, crate::stitching::Detail_BundleAdjusterBaseTraitConst, as_raw_Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterBaseTrait, as_raw_mut_Detail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterRay, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_BundleAdjusterRayTraitConst for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterRayTrait for Detail_BundleAdjusterRay {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterRay, crate::stitching::Detail_BundleAdjusterRayTraitConst, as_raw_Detail_BundleAdjusterRay, crate::stitching::Detail_BundleAdjusterRayTrait, as_raw_mut_Detail_BundleAdjusterRay }

impl Detail_BundleAdjusterRay {
	// BundleAdjusterRay()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:264
	// ("cv::detail::BundleAdjusterRay::BundleAdjusterRay", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterRay> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterRay_BundleAdjusterRay(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BundleAdjusterRay::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_BundleAdjusterRay, crate::stitching::Detail_BundleAdjusterBase, cv_detail_BundleAdjusterRay_to_Detail_BundleAdjusterBase }

boxed_cast_base! { Detail_BundleAdjusterRay, crate::stitching::Detail_Estimator, cv_detail_BundleAdjusterRay_to_Detail_Estimator }

impl std::fmt::Debug for Detail_BundleAdjusterRay {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BundleAdjusterRay")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_BundleAdjusterReproj]
// BundleAdjusterReproj /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:241
pub trait Detail_BundleAdjusterReprojTraitConst: crate::stitching::Detail_BundleAdjusterBaseTraitConst {
	fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_BundleAdjusterReproj]
pub trait Detail_BundleAdjusterReprojTrait: crate::stitching::Detail_BundleAdjusterBaseTrait + crate::stitching::Detail_BundleAdjusterReprojTraitConst {
	fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void;

}

/// Implementation of the camera parameters refinement algorithm which minimizes sum of the reprojection
/// error squares
///
/// It can estimate focal length, aspect ratio, principal point.
/// You can affect only on them via the refinement mask.
// BundleAdjusterReproj /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:241
pub struct Detail_BundleAdjusterReproj {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_BundleAdjusterReproj }

impl Drop for Detail_BundleAdjusterReproj {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_BundleAdjusterReproj_delete(self.as_raw_mut_Detail_BundleAdjusterReproj()) };
	}
}

unsafe impl Send for Detail_BundleAdjusterReproj {}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterReproj, crate::stitching::Detail_BundleAdjusterBaseTraitConst, as_raw_Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterBaseTrait, as_raw_mut_Detail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterReproj, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_BundleAdjusterReprojTraitConst for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterReprojTrait for Detail_BundleAdjusterReproj {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_BundleAdjusterReproj, crate::stitching::Detail_BundleAdjusterReprojTraitConst, as_raw_Detail_BundleAdjusterReproj, crate::stitching::Detail_BundleAdjusterReprojTrait, as_raw_mut_Detail_BundleAdjusterReproj }

impl Detail_BundleAdjusterReproj {
	// BundleAdjusterReproj()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:244
	// ("cv::detail::BundleAdjusterReproj::BundleAdjusterReproj", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_BundleAdjusterReproj> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_BundleAdjusterReproj_BundleAdjusterReproj(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_BundleAdjusterReproj::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_BundleAdjusterReproj, crate::stitching::Detail_BundleAdjusterBase, cv_detail_BundleAdjusterReproj_to_Detail_BundleAdjusterBase }

boxed_cast_base! { Detail_BundleAdjusterReproj, crate::stitching::Detail_Estimator, cv_detail_BundleAdjusterReproj_to_Detail_Estimator }

impl std::fmt::Debug for Detail_BundleAdjusterReproj {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_BundleAdjusterReproj")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_CameraParams]
// CameraParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:58
pub trait Detail_CameraParamsTraitConst {
	fn as_raw_Detail_CameraParams(&self) -> *const c_void;

	// cv::detail::CameraParams::focal() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:65
	// ("cv::detail::CameraParams::focal", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn focal(&self) -> f64 {
		let ret = unsafe { sys::cv_detail_CameraParams_propFocal_const(self.as_raw_Detail_CameraParams()) };
		ret
	}

	// cv::detail::CameraParams::aspect() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:66
	// ("cv::detail::CameraParams::aspect", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn aspect(&self) -> f64 {
		let ret = unsafe { sys::cv_detail_CameraParams_propAspect_const(self.as_raw_Detail_CameraParams()) };
		ret
	}

	// cv::detail::CameraParams::ppx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:67
	// ("cv::detail::CameraParams::ppx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ppx(&self) -> f64 {
		let ret = unsafe { sys::cv_detail_CameraParams_propPpx_const(self.as_raw_Detail_CameraParams()) };
		ret
	}

	// cv::detail::CameraParams::ppy() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:68
	// ("cv::detail::CameraParams::ppy", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ppy(&self) -> f64 {
		let ret = unsafe { sys::cv_detail_CameraParams_propPpy_const(self.as_raw_Detail_CameraParams()) };
		ret
	}

	// cv::detail::CameraParams::R() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:69
	// ("cv::detail::CameraParams::R", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn r(&self) -> core::Mat {
		let ret = unsafe { sys::cv_detail_CameraParams_propR_const(self.as_raw_Detail_CameraParams()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::detail::CameraParams::t() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:70
	// ("cv::detail::CameraParams::t", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn t(&self) -> core::Mat {
		let ret = unsafe { sys::cv_detail_CameraParams_propT_const(self.as_raw_Detail_CameraParams()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// K()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:63
	// ("cv::detail::CameraParams::K", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn k(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CameraParams_K_const(self.as_raw_Detail_CameraParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_CameraParams]
pub trait Detail_CameraParamsTrait: crate::stitching::Detail_CameraParamsTraitConst {
	fn as_raw_mut_Detail_CameraParams(&mut self) -> *mut c_void;

	// cv::detail::CameraParams::setFocal(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:65
	// ("cv::detail::CameraParams::setFocal", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_focal(&mut self, val: f64) {
		let ret = unsafe { sys::cv_detail_CameraParams_propFocal_const_double(self.as_raw_mut_Detail_CameraParams(), val) };
		ret
	}

	// cv::detail::CameraParams::setAspect(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:66
	// ("cv::detail::CameraParams::setAspect", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_aspect(&mut self, val: f64) {
		let ret = unsafe { sys::cv_detail_CameraParams_propAspect_const_double(self.as_raw_mut_Detail_CameraParams(), val) };
		ret
	}

	// cv::detail::CameraParams::setPpx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:67
	// ("cv::detail::CameraParams::setPpx", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_ppx(&mut self, val: f64) {
		let ret = unsafe { sys::cv_detail_CameraParams_propPpx_const_double(self.as_raw_mut_Detail_CameraParams(), val) };
		ret
	}

	// cv::detail::CameraParams::setPpy(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:68
	// ("cv::detail::CameraParams::setPpy", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_ppy(&mut self, val: f64) {
		let ret = unsafe { sys::cv_detail_CameraParams_propPpy_const_double(self.as_raw_mut_Detail_CameraParams(), val) };
		ret
	}

	// cv::detail::CameraParams::setR(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:69
	// ("cv::detail::CameraParams::setR", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_r(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_detail_CameraParams_propR_const_Mat(self.as_raw_mut_Detail_CameraParams(), val.as_raw_Mat()) };
		ret
	}

	// cv::detail::CameraParams::setT(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:70
	// ("cv::detail::CameraParams::setT", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_t(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_detail_CameraParams_propT_const_Mat(self.as_raw_mut_Detail_CameraParams(), val.as_raw_Mat()) };
		ret
	}

	// operator=(const CameraParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:62
	// ("cv::detail::CameraParams::operator=", vec![(pred!(mut, ["other"], ["const cv::detail::CameraParams*"]), _)]),
	#[inline]
	fn set(&mut self, other: &impl crate::stitching::Detail_CameraParamsTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CameraParams_operatorST_const_CameraParamsR(self.as_raw_mut_Detail_CameraParams(), other.as_raw_Detail_CameraParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Describes camera parameters.
///
///
/// Note: Translation is assumed to be zero during the whole stitching pipeline. :
// CameraParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:58
pub struct Detail_CameraParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CameraParams }

impl Drop for Detail_CameraParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CameraParams_delete(self.as_raw_mut_Detail_CameraParams()) };
	}
}

unsafe impl Send for Detail_CameraParams {}

impl crate::stitching::Detail_CameraParamsTraitConst for Detail_CameraParams {
	#[inline] fn as_raw_Detail_CameraParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CameraParamsTrait for Detail_CameraParams {
	#[inline] fn as_raw_mut_Detail_CameraParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CameraParams, crate::stitching::Detail_CameraParamsTraitConst, as_raw_Detail_CameraParams, crate::stitching::Detail_CameraParamsTrait, as_raw_mut_Detail_CameraParams }

impl Detail_CameraParams {
	// CameraParams()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:60
	// ("cv::detail::CameraParams::CameraParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_CameraParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CameraParams_CameraParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CameraParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	// CameraParams(const CameraParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/camera.hpp:61
	// ("cv::detail::CameraParams::CameraParams", vec![(pred!(mut, ["other"], ["const cv::detail::CameraParams*"]), _)]),
	#[inline]
	pub fn copy(other: &impl crate::stitching::Detail_CameraParamsTraitConst) -> Result<crate::stitching::Detail_CameraParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CameraParams_CameraParams_const_CameraParamsR(other.as_raw_Detail_CameraParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CameraParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Detail_CameraParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CameraParams")
			.field("focal", &crate::stitching::Detail_CameraParamsTraitConst::focal(self))
			.field("aspect", &crate::stitching::Detail_CameraParamsTraitConst::aspect(self))
			.field("ppx", &crate::stitching::Detail_CameraParamsTraitConst::ppx(self))
			.field("ppy", &crate::stitching::Detail_CameraParamsTraitConst::ppy(self))
			.field("r", &crate::stitching::Detail_CameraParamsTraitConst::r(self))
			.field("t", &crate::stitching::Detail_CameraParamsTraitConst::t(self))
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_CompressedRectilinearPortraitProjector]
// CompressedRectilinearPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:378
pub trait Detail_CompressedRectilinearPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_CompressedRectilinearPortraitProjector(&self) -> *const c_void;

	// cv::detail::CompressedRectilinearPortraitProjector::a() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:380
	// ("cv::detail::CompressedRectilinearPortraitProjector::a", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn a(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propA_const(self.as_raw_Detail_CompressedRectilinearPortraitProjector()) };
		ret
	}

	// cv::detail::CompressedRectilinearPortraitProjector::b() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:380
	// ("cv::detail::CompressedRectilinearPortraitProjector::b", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn b(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propB_const(self.as_raw_Detail_CompressedRectilinearPortraitProjector()) };
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearPortraitProjector]
pub trait Detail_CompressedRectilinearPortraitProjectorTrait: crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_CompressedRectilinearPortraitProjector(&mut self) -> *mut c_void;

	// cv::detail::CompressedRectilinearPortraitProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:380
	// ("cv::detail::CompressedRectilinearPortraitProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_a(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propA_const_float(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), val) };
		ret
	}

	// cv::detail::CompressedRectilinearPortraitProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:380
	// ("cv::detail::CompressedRectilinearPortraitProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_b(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propB_const_float(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), val) };
		ret
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:382
	// ("cv::detail::CompressedRectilinearPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:383
	// ("cv::detail::CompressedRectilinearPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CompressedRectilinearPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:378
pub struct Detail_CompressedRectilinearPortraitProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CompressedRectilinearPortraitProjector }

impl Drop for Detail_CompressedRectilinearPortraitProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_delete(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector()) };
	}
}

unsafe impl Send for Detail_CompressedRectilinearPortraitProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_CompressedRectilinearPortraitProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_CompressedRectilinearPortraitProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearPortraitProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst for Detail_CompressedRectilinearPortraitProjector {
	#[inline] fn as_raw_Detail_CompressedRectilinearPortraitProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CompressedRectilinearPortraitProjectorTrait for Detail_CompressedRectilinearPortraitProjector {
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearPortraitProjector, crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst, as_raw_Detail_CompressedRectilinearPortraitProjector, crate::stitching::Detail_CompressedRectilinearPortraitProjectorTrait, as_raw_mut_Detail_CompressedRectilinearPortraitProjector }

impl Detail_CompressedRectilinearPortraitProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_CompressedRectilinearPortraitProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_CompressedRectilinearPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CompressedRectilinearPortraitProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_CompressedRectilinearPortraitProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CompressedRectilinearPortraitProjector")
			.field("a", &crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst::a(self))
			.field("b", &crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst::b(self))
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_CompressedRectilinearPortraitProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_CompressedRectilinearPortraitWarper]
// CompressedRectilinearPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:387
pub trait Detail_CompressedRectilinearPortraitWarperTraitConst {
	fn as_raw_Detail_CompressedRectilinearPortraitWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearPortraitWarper]
pub trait Detail_CompressedRectilinearPortraitWarperTrait: crate::stitching::Detail_CompressedRectilinearPortraitWarperTraitConst {
	fn as_raw_mut_Detail_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void;

}

// CompressedRectilinearPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:387
pub struct Detail_CompressedRectilinearPortraitWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CompressedRectilinearPortraitWarper }

impl Drop for Detail_CompressedRectilinearPortraitWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CompressedRectilinearPortraitWarper_delete(self.as_raw_mut_Detail_CompressedRectilinearPortraitWarper()) };
	}
}

unsafe impl Send for Detail_CompressedRectilinearPortraitWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearPortraitWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_CompressedRectilinearPortraitWarperTraitConst for Detail_CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_Detail_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CompressedRectilinearPortraitWarperTrait for Detail_CompressedRectilinearPortraitWarper {
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearPortraitWarper, crate::stitching::Detail_CompressedRectilinearPortraitWarperTraitConst, as_raw_Detail_CompressedRectilinearPortraitWarper, crate::stitching::Detail_CompressedRectilinearPortraitWarperTrait, as_raw_mut_Detail_CompressedRectilinearPortraitWarper }

impl Detail_CompressedRectilinearPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// CompressedRectilinearPortraitWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:390
	// ("cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	#[inline]
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_CompressedRectilinearPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(scale, a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CompressedRectilinearPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:390
	// ("cv::detail::CompressedRectilinearPortraitWarper::CompressedRectilinearPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new_def(scale: f32) -> Result<crate::stitching::Detail_CompressedRectilinearPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CompressedRectilinearPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_CompressedRectilinearPortraitWarper, crate::stitching::Detail_RotationWarper, cv_detail_CompressedRectilinearPortraitWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_CompressedRectilinearPortraitWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CompressedRectilinearPortraitWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_CompressedRectilinearProjector]
// CompressedRectilinearProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:357
pub trait Detail_CompressedRectilinearProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_CompressedRectilinearProjector(&self) -> *const c_void;

	// cv::detail::CompressedRectilinearProjector::a() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:359
	// ("cv::detail::CompressedRectilinearProjector::a", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn a(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propA_const(self.as_raw_Detail_CompressedRectilinearProjector()) };
		ret
	}

	// cv::detail::CompressedRectilinearProjector::b() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:359
	// ("cv::detail::CompressedRectilinearProjector::b", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn b(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propB_const(self.as_raw_Detail_CompressedRectilinearProjector()) };
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearProjector]
pub trait Detail_CompressedRectilinearProjectorTrait: crate::stitching::Detail_CompressedRectilinearProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_CompressedRectilinearProjector(&mut self) -> *mut c_void;

	// cv::detail::CompressedRectilinearProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:359
	// ("cv::detail::CompressedRectilinearProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_a(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propA_const_float(self.as_raw_mut_Detail_CompressedRectilinearProjector(), val) };
		ret
	}

	// cv::detail::CompressedRectilinearProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:359
	// ("cv::detail::CompressedRectilinearProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_b(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propB_const_float(self.as_raw_mut_Detail_CompressedRectilinearProjector(), val) };
		ret
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:361
	// ("cv::detail::CompressedRectilinearProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:362
	// ("cv::detail::CompressedRectilinearProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CompressedRectilinearProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:357
pub struct Detail_CompressedRectilinearProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CompressedRectilinearProjector }

impl Drop for Detail_CompressedRectilinearProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CompressedRectilinearProjector_delete(self.as_raw_mut_Detail_CompressedRectilinearProjector()) };
	}
}

unsafe impl Send for Detail_CompressedRectilinearProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_CompressedRectilinearProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_CompressedRectilinearProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_CompressedRectilinearProjectorTraitConst for Detail_CompressedRectilinearProjector {
	#[inline] fn as_raw_Detail_CompressedRectilinearProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CompressedRectilinearProjectorTrait for Detail_CompressedRectilinearProjector {
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearProjector, crate::stitching::Detail_CompressedRectilinearProjectorTraitConst, as_raw_Detail_CompressedRectilinearProjector, crate::stitching::Detail_CompressedRectilinearProjectorTrait, as_raw_mut_Detail_CompressedRectilinearProjector }

impl Detail_CompressedRectilinearProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_CompressedRectilinearProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_CompressedRectilinearProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CompressedRectilinearProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_CompressedRectilinearProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CompressedRectilinearProjector")
			.field("a", &crate::stitching::Detail_CompressedRectilinearProjectorTraitConst::a(self))
			.field("b", &crate::stitching::Detail_CompressedRectilinearProjectorTraitConst::b(self))
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_CompressedRectilinearProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_CompressedRectilinearWarper]
// CompressedRectilinearWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:366
pub trait Detail_CompressedRectilinearWarperTraitConst {
	fn as_raw_Detail_CompressedRectilinearWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearWarper]
pub trait Detail_CompressedRectilinearWarperTrait: crate::stitching::Detail_CompressedRectilinearWarperTraitConst {
	fn as_raw_mut_Detail_CompressedRectilinearWarper(&mut self) -> *mut c_void;

}

// CompressedRectilinearWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:366
pub struct Detail_CompressedRectilinearWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CompressedRectilinearWarper }

impl Drop for Detail_CompressedRectilinearWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CompressedRectilinearWarper_delete(self.as_raw_mut_Detail_CompressedRectilinearWarper()) };
	}
}

unsafe impl Send for Detail_CompressedRectilinearWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_CompressedRectilinearWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_CompressedRectilinearWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_CompressedRectilinearWarperTraitConst for Detail_CompressedRectilinearWarper {
	#[inline] fn as_raw_Detail_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CompressedRectilinearWarperTrait for Detail_CompressedRectilinearWarper {
	#[inline] fn as_raw_mut_Detail_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CompressedRectilinearWarper, crate::stitching::Detail_CompressedRectilinearWarperTraitConst, as_raw_Detail_CompressedRectilinearWarper, crate::stitching::Detail_CompressedRectilinearWarperTrait, as_raw_mut_Detail_CompressedRectilinearWarper }

impl Detail_CompressedRectilinearWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// CompressedRectilinearWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:369
	// ("cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	#[inline]
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_CompressedRectilinearWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(scale, a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CompressedRectilinearWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:369
	// ("cv::detail::CompressedRectilinearWarper::CompressedRectilinearWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new_def(scale: f32) -> Result<crate::stitching::Detail_CompressedRectilinearWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CompressedRectilinearWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_CompressedRectilinearWarper, crate::stitching::Detail_RotationWarper, cv_detail_CompressedRectilinearWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_CompressedRectilinearWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CompressedRectilinearWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_CylindricalPortraitProjector]
// CylindricalPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:630
pub trait Detail_CylindricalPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_CylindricalPortraitProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_CylindricalPortraitProjector]
pub trait Detail_CylindricalPortraitProjectorTrait: crate::stitching::Detail_CylindricalPortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_CylindricalPortraitProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:632
	// ("cv::detail::CylindricalPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:633
	// ("cv::detail::CylindricalPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CylindricalPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:630
pub struct Detail_CylindricalPortraitProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CylindricalPortraitProjector }

impl Drop for Detail_CylindricalPortraitProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CylindricalPortraitProjector_delete(self.as_raw_mut_Detail_CylindricalPortraitProjector()) };
	}
}

unsafe impl Send for Detail_CylindricalPortraitProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_CylindricalPortraitProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_CylindricalPortraitProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalPortraitProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_CylindricalPortraitProjectorTraitConst for Detail_CylindricalPortraitProjector {
	#[inline] fn as_raw_Detail_CylindricalPortraitProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CylindricalPortraitProjectorTrait for Detail_CylindricalPortraitProjector {
	#[inline] fn as_raw_mut_Detail_CylindricalPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalPortraitProjector, crate::stitching::Detail_CylindricalPortraitProjectorTraitConst, as_raw_Detail_CylindricalPortraitProjector, crate::stitching::Detail_CylindricalPortraitProjectorTrait, as_raw_mut_Detail_CylindricalPortraitProjector }

impl Detail_CylindricalPortraitProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_CylindricalPortraitProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_CylindricalPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CylindricalPortraitProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_CylindricalPortraitProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CylindricalPortraitProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_CylindricalPortraitProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_CylindricalPortraitWarper]
// CylindricalPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:637
pub trait Detail_CylindricalPortraitWarperTraitConst {
	fn as_raw_Detail_CylindricalPortraitWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_CylindricalPortraitWarper]
pub trait Detail_CylindricalPortraitWarperTrait: crate::stitching::Detail_CylindricalPortraitWarperTraitConst {
	fn as_raw_mut_Detail_CylindricalPortraitWarper(&mut self) -> *mut c_void;

}

// CylindricalPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:637
pub struct Detail_CylindricalPortraitWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CylindricalPortraitWarper }

impl Drop for Detail_CylindricalPortraitWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CylindricalPortraitWarper_delete(self.as_raw_mut_Detail_CylindricalPortraitWarper()) };
	}
}

unsafe impl Send for Detail_CylindricalPortraitWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_CylindricalPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_CylindricalPortraitWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalPortraitWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_CylindricalPortraitWarperTraitConst for Detail_CylindricalPortraitWarper {
	#[inline] fn as_raw_Detail_CylindricalPortraitWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CylindricalPortraitWarperTrait for Detail_CylindricalPortraitWarper {
	#[inline] fn as_raw_mut_Detail_CylindricalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalPortraitWarper, crate::stitching::Detail_CylindricalPortraitWarperTraitConst, as_raw_Detail_CylindricalPortraitWarper, crate::stitching::Detail_CylindricalPortraitWarperTrait, as_raw_mut_Detail_CylindricalPortraitWarper }

impl Detail_CylindricalPortraitWarper {
	// CylindricalPortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:640
	// ("cv::detail::CylindricalPortraitWarper::CylindricalPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_CylindricalPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CylindricalPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_CylindricalPortraitWarper, crate::stitching::Detail_RotationWarper, cv_detail_CylindricalPortraitWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_CylindricalPortraitWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CylindricalPortraitWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_CylindricalProjector]
// CylindricalProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:301
pub trait Detail_CylindricalProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_CylindricalProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_CylindricalProjector]
pub trait Detail_CylindricalProjectorTrait: crate::stitching::Detail_CylindricalProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_CylindricalProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:303
	// ("cv::detail::CylindricalProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:304
	// ("cv::detail::CylindricalProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CylindricalProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:301
pub struct Detail_CylindricalProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CylindricalProjector }

impl Drop for Detail_CylindricalProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CylindricalProjector_delete(self.as_raw_mut_Detail_CylindricalProjector()) };
	}
}

unsafe impl Send for Detail_CylindricalProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_CylindricalProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_CylindricalProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_CylindricalProjectorTraitConst for Detail_CylindricalProjector {
	#[inline] fn as_raw_Detail_CylindricalProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CylindricalProjectorTrait for Detail_CylindricalProjector {
	#[inline] fn as_raw_mut_Detail_CylindricalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalProjector, crate::stitching::Detail_CylindricalProjectorTraitConst, as_raw_Detail_CylindricalProjector, crate::stitching::Detail_CylindricalProjectorTrait, as_raw_mut_Detail_CylindricalProjector }

impl Detail_CylindricalProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_CylindricalProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_CylindricalProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CylindricalProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_CylindricalProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CylindricalProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_CylindricalProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_CylindricalWarper]
// CylindricalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:310
pub trait Detail_CylindricalWarperTraitConst {
	fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_CylindricalWarper]
pub trait Detail_CylindricalWarperTrait: crate::stitching::Detail_CylindricalWarperTraitConst {
	fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void;

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:319
	// ("cv::detail::CylindricalWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:320
	// ("cv::detail::CylindricalWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Warper that maps an image onto the x\*x + z\*z = 1 cylinder.
// CylindricalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:310
pub struct Detail_CylindricalWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CylindricalWarper }

impl Drop for Detail_CylindricalWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CylindricalWarper_delete(self.as_raw_mut_Detail_CylindricalWarper()) };
	}
}

unsafe impl Send for Detail_CylindricalWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_CylindricalWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_CylindricalWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_CylindricalWarperTraitConst for Detail_CylindricalWarper {
	#[inline] fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CylindricalWarperTrait for Detail_CylindricalWarper {
	#[inline] fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalWarper, crate::stitching::Detail_CylindricalWarperTraitConst, as_raw_Detail_CylindricalWarper, crate::stitching::Detail_CylindricalWarperTrait, as_raw_mut_Detail_CylindricalWarper }

impl Detail_CylindricalWarper {
	/// Construct an instance of the cylindrical warper class.
	///
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	// CylindricalWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:317
	// ("cv::detail::CylindricalWarper::CylindricalWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_CylindricalWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarper_CylindricalWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CylindricalWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Detail_CylindricalWarper, crate::stitching::Detail_CylindricalWarperGpu, cv_detail_CylindricalWarper_to_Detail_CylindricalWarperGpu }

boxed_cast_base! { Detail_CylindricalWarper, crate::stitching::Detail_RotationWarper, cv_detail_CylindricalWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_CylindricalWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CylindricalWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_CylindricalWarperGpu]
// CylindricalWarperGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:572
pub trait Detail_CylindricalWarperGpuTraitConst: crate::stitching::Detail_CylindricalWarperTraitConst {
	fn as_raw_Detail_CylindricalWarperGpu(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_CylindricalWarperGpu]
pub trait Detail_CylindricalWarperGpuTrait: crate::stitching::Detail_CylindricalWarperGpuTraitConst + crate::stitching::Detail_CylindricalWarperTrait {
	fn as_raw_mut_Detail_CylindricalWarperGpu(&mut self) -> *mut c_void;

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:582
	// ("cv::detail::CylindricalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:590
	// ("cv::detail::CylindricalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:602
	// ("cv::detail::CylindricalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn build_maps_1(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_CylindricalWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:604
	// ("cv::detail::CylindricalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn warp_1(&mut self, src: &impl core::GpuMatTraitConst, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::GpuMatTrait) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CylindricalWarperGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:572
pub struct Detail_CylindricalWarperGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_CylindricalWarperGpu }

impl Drop for Detail_CylindricalWarperGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_CylindricalWarperGpu_delete(self.as_raw_mut_Detail_CylindricalWarperGpu()) };
	}
}

unsafe impl Send for Detail_CylindricalWarperGpu {}

impl crate::stitching::Detail_CylindricalWarperTraitConst for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CylindricalWarperTrait for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalWarperGpu, crate::stitching::Detail_CylindricalWarperTraitConst, as_raw_Detail_CylindricalWarper, crate::stitching::Detail_CylindricalWarperTrait, as_raw_mut_Detail_CylindricalWarper }

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalWarperGpu, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_CylindricalWarperGpuTraitConst for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_Detail_CylindricalWarperGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_CylindricalWarperGpuTrait for Detail_CylindricalWarperGpu {
	#[inline] fn as_raw_mut_Detail_CylindricalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_CylindricalWarperGpu, crate::stitching::Detail_CylindricalWarperGpuTraitConst, as_raw_Detail_CylindricalWarperGpu, crate::stitching::Detail_CylindricalWarperGpuTrait, as_raw_mut_Detail_CylindricalWarperGpu }

impl Detail_CylindricalWarperGpu {
	// CylindricalWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:575
	// ("cv::detail::CylindricalWarperGpu::CylindricalWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_CylindricalWarperGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_CylindricalWarperGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_CylindricalWarperGpu, crate::stitching::Detail_CylindricalWarper, cv_detail_CylindricalWarperGpu_to_Detail_CylindricalWarper }

boxed_cast_base! { Detail_CylindricalWarperGpu, crate::stitching::Detail_RotationWarper, cv_detail_CylindricalWarperGpu_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_CylindricalWarperGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_CylindricalWarperGpu")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_DisjointSets]
// DisjointSets /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:55
pub trait Detail_DisjointSetsTraitConst {
	fn as_raw_Detail_DisjointSets(&self) -> *const c_void;

	// cv::detail::DisjointSets::parent() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:64
	// ("cv::detail::DisjointSets::parent", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn parent(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_detail_DisjointSets_propParent_const(self.as_raw_Detail_DisjointSets()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}

	// cv::detail::DisjointSets::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:65
	// ("cv::detail::DisjointSets::size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn size(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_detail_DisjointSets_propSize_const(self.as_raw_Detail_DisjointSets()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_DisjointSets]
pub trait Detail_DisjointSetsTrait: crate::stitching::Detail_DisjointSetsTraitConst {
	fn as_raw_mut_Detail_DisjointSets(&mut self) -> *mut c_void;

	// cv::detail::DisjointSets::setParent(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:64
	// ("cv::detail::DisjointSets::setParent", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_parent(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_detail_DisjointSets_propParent_const_vectorLintG(self.as_raw_mut_Detail_DisjointSets(), val.as_raw_VectorOfi32()) };
		ret
	}

	// cv::detail::DisjointSets::setSize(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:65
	// ("cv::detail::DisjointSets::setSize", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_size(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_detail_DisjointSets_propSize_const_vectorLintG(self.as_raw_mut_Detail_DisjointSets(), val.as_raw_VectorOfi32()) };
		ret
	}

	// createOneElemSets(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:60
	// ("cv::detail::DisjointSets::createOneElemSets", vec![(pred!(mut, ["elem_count"], ["int"]), _)]),
	#[inline]
	fn create_one_elem_sets(&mut self, elem_count: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DisjointSets_createOneElemSets_int(self.as_raw_mut_Detail_DisjointSets(), elem_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// findSetByElem(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:61
	// ("cv::detail::DisjointSets::findSetByElem", vec![(pred!(mut, ["elem"], ["int"]), _)]),
	#[inline]
	fn find_set_by_elem(&mut self, elem: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DisjointSets_findSetByElem_int(self.as_raw_mut_Detail_DisjointSets(), elem, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mergeSets(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:62
	// ("cv::detail::DisjointSets::mergeSets", vec![(pred!(mut, ["set1", "set2"], ["int", "int"]), _)]),
	#[inline]
	fn merge_sets(&mut self, set1: i32, set2: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DisjointSets_mergeSets_int_int(self.as_raw_mut_Detail_DisjointSets(), set1, set2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DisjointSets /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:55
pub struct Detail_DisjointSets {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_DisjointSets }

impl Drop for Detail_DisjointSets {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_DisjointSets_delete(self.as_raw_mut_Detail_DisjointSets()) };
	}
}

unsafe impl Send for Detail_DisjointSets {}

impl crate::stitching::Detail_DisjointSetsTraitConst for Detail_DisjointSets {
	#[inline] fn as_raw_Detail_DisjointSets(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_DisjointSetsTrait for Detail_DisjointSets {
	#[inline] fn as_raw_mut_Detail_DisjointSets(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_DisjointSets, crate::stitching::Detail_DisjointSetsTraitConst, as_raw_Detail_DisjointSets, crate::stitching::Detail_DisjointSetsTrait, as_raw_mut_Detail_DisjointSets }

impl Detail_DisjointSets {
	/// ## C++ default parameters
	/// * elem_count: 0
	// DisjointSets(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:58
	// ("cv::detail::DisjointSets::DisjointSets", vec![(pred!(mut, ["elem_count"], ["int"]), _)]),
	#[inline]
	pub fn new(elem_count: i32) -> Result<crate::stitching::Detail_DisjointSets> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DisjointSets_DisjointSets_int(elem_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_DisjointSets::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * elem_count: 0
	// cv::detail::DisjointSets::DisjointSets() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:58
	// ("cv::detail::DisjointSets::DisjointSets", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_DisjointSets> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DisjointSets_DisjointSets(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_DisjointSets::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Detail_DisjointSets {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_DisjointSets")
			.field("parent", &crate::stitching::Detail_DisjointSetsTraitConst::parent(self))
			.field("size", &crate::stitching::Detail_DisjointSetsTraitConst::size(self))
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_DpSeamFinder]
// DpSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:118
pub trait Detail_DpSeamFinderTraitConst: crate::stitching::Detail_SeamFinderTraitConst {
	fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void;

	// costFunction()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:125
	// ("cv::detail::DpSeamFinder::costFunction", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cost_function(&self) -> Result<crate::stitching::Detail_DpSeamFinder_CostFunction> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DpSeamFinder_costFunction_const(self.as_raw_Detail_DpSeamFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_DpSeamFinder]
pub trait Detail_DpSeamFinderTrait: crate::stitching::Detail_DpSeamFinderTraitConst + crate::stitching::Detail_SeamFinderTrait {
	fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void;

	// setCostFunction(CostFunction)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:126
	// ("cv::detail::DpSeamFinder::setCostFunction", vec![(pred!(mut, ["val"], ["cv::detail::DpSeamFinder::CostFunction"]), _)]),
	#[inline]
	fn set_cost_function(&mut self, val: crate::stitching::Detail_DpSeamFinder_CostFunction) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DpSeamFinder_setCostFunction_CostFunction(self.as_raw_mut_Detail_DpSeamFinder(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:128
	// ("cv::detail::DpSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DpSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_DpSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DpSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:118
pub struct Detail_DpSeamFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_DpSeamFinder }

impl Drop for Detail_DpSeamFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_DpSeamFinder_delete(self.as_raw_mut_Detail_DpSeamFinder()) };
	}
}

unsafe impl Send for Detail_DpSeamFinder {}

impl crate::stitching::Detail_SeamFinderTraitConst for Detail_DpSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for Detail_DpSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_DpSeamFinder, crate::stitching::Detail_SeamFinderTraitConst, as_raw_Detail_SeamFinder, crate::stitching::Detail_SeamFinderTrait, as_raw_mut_Detail_SeamFinder }

impl crate::stitching::Detail_DpSeamFinderTraitConst for Detail_DpSeamFinder {
	#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_DpSeamFinderTrait for Detail_DpSeamFinder {
	#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_DpSeamFinder, crate::stitching::Detail_DpSeamFinderTraitConst, as_raw_Detail_DpSeamFinder, crate::stitching::Detail_DpSeamFinderTrait, as_raw_mut_Detail_DpSeamFinder }

impl Detail_DpSeamFinder {
	/// ## C++ default parameters
	/// * cost_func: COLOR
	// DpSeamFinder(CostFunction)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:123
	// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, ["costFunc"], ["cv::detail::DpSeamFinder::CostFunction"]), _)]),
	#[inline]
	pub fn new(cost_func: crate::stitching::Detail_DpSeamFinder_CostFunction) -> Result<crate::stitching::Detail_DpSeamFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cost_func, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_DpSeamFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * cost_func: COLOR
	// cv::detail::DpSeamFinder::DpSeamFinder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:123
	// ("cv::detail::DpSeamFinder::DpSeamFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_DpSeamFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_DpSeamFinder_DpSeamFinder(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_DpSeamFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_DpSeamFinder, crate::stitching::Detail_SeamFinder, cv_detail_DpSeamFinder_to_Detail_SeamFinder }

impl std::fmt::Debug for Detail_DpSeamFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_DpSeamFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_Estimator]
// Estimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:65
pub trait Detail_EstimatorTraitConst {
	fn as_raw_Detail_Estimator(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_Estimator]
pub trait Detail_EstimatorTrait: crate::stitching::Detail_EstimatorTraitConst {
	fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void;

	/// Estimates camera parameters.
	///
	/// ## Parameters
	/// * features: Features of images
	/// * pairwise_matches: Pairwise matches of images
	/// * cameras: Estimated camera parameters
	/// ## Returns
	/// True in case of success, false otherwise
	// operator()(const std::vector<ImageFeatures> &, const std::vector<MatchesInfo> &, std::vector<CameraParams> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:77
	// ("cv::detail::Estimator::operator()", vec![(pred!(mut, ["features", "pairwise_matches", "cameras"], ["const std::vector<cv::detail::ImageFeatures>*", "const std::vector<cv::detail::MatchesInfo>*", "std::vector<cv::detail::CameraParams>*"]), _)]),
	#[inline]
	fn apply(&mut self, features: &core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &core::Vector<crate::stitching::Detail_MatchesInfo>, cameras: &mut core::Vector<crate::stitching::Detail_CameraParams>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Estimator_operator___const_vectorLImageFeaturesGR_const_vectorLMatchesInfoGR_vectorLCameraParamsGR(self.as_raw_mut_Detail_Estimator(), features.as_raw_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_VectorOfDetail_MatchesInfo(), cameras.as_raw_mut_VectorOfDetail_CameraParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
// Estimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:65
pub struct Detail_Estimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_Estimator }

impl Drop for Detail_Estimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_Estimator_delete(self.as_raw_mut_Detail_Estimator()) };
	}
}

unsafe impl Send for Detail_Estimator {}

impl crate::stitching::Detail_EstimatorTraitConst for Detail_Estimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_Estimator {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_Estimator, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl Detail_Estimator {
}

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_AffineBasedEstimator, cv_detail_Estimator_to_Detail_AffineBasedEstimator }

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_BundleAdjusterAffine, cv_detail_Estimator_to_Detail_BundleAdjusterAffine }

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_BundleAdjusterAffinePartial, cv_detail_Estimator_to_Detail_BundleAdjusterAffinePartial }

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_BundleAdjusterBase, cv_detail_Estimator_to_Detail_BundleAdjusterBase }

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_BundleAdjusterRay, cv_detail_Estimator_to_Detail_BundleAdjusterRay }

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_BundleAdjusterReproj, cv_detail_Estimator_to_Detail_BundleAdjusterReproj }

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_HomographyBasedEstimator, cv_detail_Estimator_to_Detail_HomographyBasedEstimator }

boxed_cast_descendant! { Detail_Estimator, crate::stitching::Detail_NoBundleAdjuster, cv_detail_Estimator_to_Detail_NoBundleAdjuster }

impl std::fmt::Debug for Detail_Estimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_Estimator")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_ExposureCompensator]
// ExposureCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:60
pub trait Detail_ExposureCompensatorTraitConst {
	fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_ExposureCompensator]
pub trait Detail_ExposureCompensatorTrait: crate::stitching::Detail_ExposureCompensatorTraitConst {
	fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void;

	/// ## Parameters
	/// * corners: Source image top-left corners
	/// * images: Source images
	/// * masks: Image masks to update (second value in pair specifies the value which should be used
	/// to detect where image is)
	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:74
	// ("cv::detail::ExposureCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_ExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLUMatGR(self.as_raw_mut_Detail_ExposureCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Parameters
	/// * corners: Source image top-left corners
	/// * images: Source images
	/// * masks: Image masks to update (second value in pair specifies the value which should be used
	/// to detect where image is)
	///
	/// ## Overloaded parameters
	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:77
	// ("cv::detail::ExposureCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	#[inline]
	fn feed_1(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_ExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_ExposureCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compensate exposure in the specified image.
	///
	/// ## Parameters
	/// * index: Image index
	/// * corner: Image top-left corner
	/// * image: Image to process
	/// * mask: Image mask
	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:86
	// ("cv::detail::ExposureCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl ToInputOutputArray, mask: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_ExposureCompensator(), index, &corner, image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for all exposure compensators.
// ExposureCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:60
pub struct Detail_ExposureCompensator {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_ExposureCompensator }

impl Drop for Detail_ExposureCompensator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_ExposureCompensator_delete(self.as_raw_mut_Detail_ExposureCompensator()) };
	}
}

unsafe impl Send for Detail_ExposureCompensator {}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_ExposureCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_ExposureCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_ExposureCompensator, crate::stitching::Detail_ExposureCompensatorTraitConst, as_raw_Detail_ExposureCompensator, crate::stitching::Detail_ExposureCompensatorTrait, as_raw_mut_Detail_ExposureCompensator }

impl Detail_ExposureCompensator {
	// createDefault(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:66
	// ("cv::detail::ExposureCompensator::createDefault", vec![(pred!(mut, ["type"], ["int"]), _)]),
	#[inline]
	pub fn create_default(typ: i32) -> Result<core::Ptr<crate::stitching::Detail_ExposureCompensator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_ExposureCompensator_createDefault_int(typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Detail_ExposureCompensator, crate::stitching::Detail_BlocksGainCompensator, cv_detail_ExposureCompensator_to_Detail_BlocksGainCompensator }

boxed_cast_descendant! { Detail_ExposureCompensator, crate::stitching::Detail_GainCompensator, cv_detail_ExposureCompensator_to_Detail_GainCompensator }

boxed_cast_descendant! { Detail_ExposureCompensator, crate::stitching::Detail_NoExposureCompensator, cv_detail_ExposureCompensator_to_Detail_NoExposureCompensator }

impl std::fmt::Debug for Detail_ExposureCompensator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_ExposureCompensator")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_FeatherBlender]
// FeatherBlender /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:100
pub trait Detail_FeatherBlenderTraitConst: crate::stitching::Detail_BlenderTraitConst {
	fn as_raw_Detail_FeatherBlender(&self) -> *const c_void;

	// sharpness()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:105
	// ("cv::detail::FeatherBlender::sharpness", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sharpness(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_sharpness_const(self.as_raw_Detail_FeatherBlender(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_FeatherBlender]
pub trait Detail_FeatherBlenderTrait: crate::stitching::Detail_BlenderTrait + crate::stitching::Detail_FeatherBlenderTraitConst {
	fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void;

	// setSharpness(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:106
	// ("cv::detail::FeatherBlender::setSharpness", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_sharpness(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_setSharpness_float(self.as_raw_mut_Detail_FeatherBlender(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:108
	// ("cv::detail::FeatherBlender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
	#[inline]
	fn prepare(&mut self, dst_roi: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_prepare_Rect(self.as_raw_mut_Detail_FeatherBlender(), &dst_roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:109
	// ("cv::detail::FeatherBlender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	#[inline]
	fn feed(&mut self, img: &impl ToInputArray, mask: &impl ToInputArray, tl: core::Point) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_FeatherBlender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), &tl, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:110
	// ("cv::detail::FeatherBlender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn blend(&mut self, dst: &mut impl ToInputOutputArray, dst_mask: &mut impl ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(dst);
		input_output_array_arg!(dst_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Detail_FeatherBlender(), dst.as_raw__InputOutputArray(), dst_mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Creates weight maps for fixed set of source images by their masks and top-left corners.
	/// Final image can be obtained by simple weighting of the source images.
	// createWeightMaps(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:114
	// ("cv::detail::FeatherBlender::createWeightMaps", vec![(pred!(mut, ["masks", "corners", "weight_maps"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn create_weight_maps(&mut self, masks: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, weight_maps: &mut core::Vector<core::UMat>) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_createWeightMaps_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_FeatherBlender(), masks.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), weight_maps.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Simple blender which mixes images at its borders.
// FeatherBlender /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:100
pub struct Detail_FeatherBlender {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_FeatherBlender }

impl Drop for Detail_FeatherBlender {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_FeatherBlender_delete(self.as_raw_mut_Detail_FeatherBlender()) };
	}
}

unsafe impl Send for Detail_FeatherBlender {}

impl crate::stitching::Detail_BlenderTraitConst for Detail_FeatherBlender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for Detail_FeatherBlender {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FeatherBlender, crate::stitching::Detail_BlenderTraitConst, as_raw_Detail_Blender, crate::stitching::Detail_BlenderTrait, as_raw_mut_Detail_Blender }

impl crate::stitching::Detail_FeatherBlenderTraitConst for Detail_FeatherBlender {
	#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeatherBlenderTrait for Detail_FeatherBlender {
	#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FeatherBlender, crate::stitching::Detail_FeatherBlenderTraitConst, as_raw_Detail_FeatherBlender, crate::stitching::Detail_FeatherBlenderTrait, as_raw_mut_Detail_FeatherBlender }

impl Detail_FeatherBlender {
	/// ## C++ default parameters
	/// * sharpness: 0.02f
	// FeatherBlender(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:103
	// ("cv::detail::FeatherBlender::FeatherBlender", vec![(pred!(mut, ["sharpness"], ["float"]), _)]),
	#[inline]
	pub fn new(sharpness: f32) -> Result<crate::stitching::Detail_FeatherBlender> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_FeatherBlender_float(sharpness, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_FeatherBlender::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * sharpness: 0.02f
	// cv::detail::FeatherBlender::FeatherBlender() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:103
	// ("cv::detail::FeatherBlender::FeatherBlender", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_FeatherBlender> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeatherBlender_FeatherBlender(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_FeatherBlender::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_FeatherBlender, crate::stitching::Detail_Blender, cv_detail_FeatherBlender_to_Detail_Blender }

impl std::fmt::Debug for Detail_FeatherBlender {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_FeatherBlender")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_FeaturesFinder]
// FeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:71
pub trait Detail_FeaturesFinderTraitConst {
	fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_FeaturesFinder]
pub trait Detail_FeaturesFinderTrait: crate::stitching::Detail_FeaturesFinderTraitConst {
	fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void;

	/// Finds features in the given image.
	///
	/// ## Parameters
	/// * image: Source image
	/// * features: Found features
	/// * rois: Regions of interest
	/// ## See also
	/// detail::ImageFeatures, Rect_
	///
	/// ## Overloaded parameters
	// operator()(InputArray, ImageFeatures &)(InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:76
	// ("cv::detail::FeaturesFinder::operator()", vec![(pred!(mut, ["image", "features"], ["const cv::_InputArray*", "cv::detail::ImageFeatures*"]), _)]),
	#[inline]
	fn apply(&mut self, image: &impl ToInputArray, features: &mut impl crate::stitching::Detail_ImageFeaturesTrait) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesFinder_operator___const__InputArrayR_ImageFeaturesR(self.as_raw_mut_Detail_FeaturesFinder(), image.as_raw__InputArray(), features.as_raw_mut_Detail_ImageFeatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds features in the given image.
	///
	/// ## Parameters
	/// * image: Source image
	/// * features: Found features
	/// * rois: Regions of interest
	/// ## See also
	/// detail::ImageFeatures, Rect_
	// operator()(InputArray, ImageFeatures &, const std::vector<cv::Rect> &)(InputArray, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:85
	// ("cv::detail::FeaturesFinder::operator()", vec![(pred!(mut, ["image", "features", "rois"], ["const cv::_InputArray*", "cv::detail::ImageFeatures*", "const std::vector<cv::Rect>*"]), _)]),
	#[inline]
	fn apply_1(&mut self, image: &impl ToInputArray, features: &mut impl crate::stitching::Detail_ImageFeaturesTrait, rois: &core::Vector<core::Rect>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesFinder_operator___const__InputArrayR_ImageFeaturesR_const_vectorLRectGR(self.as_raw_mut_Detail_FeaturesFinder(), image.as_raw__InputArray(), features.as_raw_mut_Detail_ImageFeatures(), rois.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds features in the given images in parallel.
	///
	/// ## Parameters
	/// * images: Source images
	/// * features: Found features for each image
	/// * rois: Regions of interest for each image
	/// ## See also
	/// detail::ImageFeatures, Rect_
	// operator()(InputArrayOfArrays, std::vector<ImageFeatures> &, const std::vector<std::vector<cv::Rect>> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:94
	// ("cv::detail::FeaturesFinder::operator()", vec![(pred!(mut, ["images", "features", "rois"], ["const cv::_InputArray*", "std::vector<cv::detail::ImageFeatures>*", "const std::vector<std::vector<cv::Rect>>*"]), _)]),
	#[inline]
	fn apply_2(&mut self, images: &impl ToInputArray, features: &mut core::Vector<crate::stitching::Detail_ImageFeatures>, rois: &core::Vector<core::Vector<core::Rect>>) -> Result<()> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesFinder_operator___const__InputArrayR_vectorLImageFeaturesGR_const_vectorLvectorLRectGGR(self.as_raw_mut_Detail_FeaturesFinder(), images.as_raw__InputArray(), features.as_raw_mut_VectorOfDetail_ImageFeatures(), rois.as_raw_VectorOfVectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds features in the given images in parallel.
	///
	/// ## Parameters
	/// * images: Source images
	/// * features: Found features for each image
	/// * rois: Regions of interest for each image
	/// ## See also
	/// detail::ImageFeatures, Rect_
	///
	/// ## Overloaded parameters
	// operator()(InputArrayOfArrays, std::vector<ImageFeatures> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:97
	// ("cv::detail::FeaturesFinder::operator()", vec![(pred!(mut, ["images", "features"], ["const cv::_InputArray*", "std::vector<cv::detail::ImageFeatures>*"]), _)]),
	#[inline]
	fn apply_3(&mut self, images: &impl ToInputArray, features: &mut core::Vector<crate::stitching::Detail_ImageFeatures>) -> Result<()> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesFinder_operator___const__InputArrayR_vectorLImageFeaturesGR(self.as_raw_mut_Detail_FeaturesFinder(), images.as_raw__InputArray(), features.as_raw_mut_VectorOfDetail_ImageFeatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Frees unused memory allocated before if there is any.
	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:99
	// ("cv::detail::FeaturesFinder::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesFinder_collectGarbage(self.as_raw_mut_Detail_FeaturesFinder(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Feature finders base class
// FeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:71
pub struct Detail_FeaturesFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_FeaturesFinder }

impl Drop for Detail_FeaturesFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_FeaturesFinder_delete(self.as_raw_mut_Detail_FeaturesFinder()) };
	}
}

unsafe impl Send for Detail_FeaturesFinder {}

impl crate::stitching::Detail_FeaturesFinderTraitConst for Detail_FeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for Detail_FeaturesFinder {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FeaturesFinder, crate::stitching::Detail_FeaturesFinderTraitConst, as_raw_Detail_FeaturesFinder, crate::stitching::Detail_FeaturesFinderTrait, as_raw_mut_Detail_FeaturesFinder }

impl Detail_FeaturesFinder {
}

boxed_cast_descendant! { Detail_FeaturesFinder, crate::stitching::Detail_AKAZEFeaturesFinder, cv_detail_FeaturesFinder_to_Detail_AKAZEFeaturesFinder }

boxed_cast_descendant! { Detail_FeaturesFinder, crate::stitching::Detail_OrbFeaturesFinder, cv_detail_FeaturesFinder_to_Detail_OrbFeaturesFinder }

boxed_cast_descendant! { Detail_FeaturesFinder, crate::stitching::Detail_SiftFeaturesFinder, cv_detail_FeaturesFinder_to_Detail_SiftFeaturesFinder }

boxed_cast_descendant! { Detail_FeaturesFinder, crate::stitching::Detail_SurfFeaturesFinder, cv_detail_FeaturesFinder_to_Detail_SurfFeaturesFinder }

boxed_cast_descendant! { Detail_FeaturesFinder, crate::stitching::Detail_SurfFeaturesFinderGpu, cv_detail_FeaturesFinder_to_Detail_SurfFeaturesFinderGpu }

impl std::fmt::Debug for Detail_FeaturesFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_FeaturesFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_FeaturesMatcher]
// FeaturesMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:236
pub trait Detail_FeaturesMatcherTraitConst {
	fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void;

	/// ## Returns
	/// True, if it's possible to use the same matcher instance in parallel, false otherwise
	// isThreadSafe()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:264
	// ("cv::detail::FeaturesMatcher::isThreadSafe", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_thread_safe(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesMatcher_isThreadSafe_const(self.as_raw_Detail_FeaturesMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_FeaturesMatcher]
pub trait Detail_FeaturesMatcherTrait: crate::stitching::Detail_FeaturesMatcherTraitConst {
	fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void;

	/// Performs images matching.
	///
	/// ## Parameters
	/// * features: Features of the source images
	/// * pairwise_matches: Found pairwise matches
	/// * mask: Mask indicating which image pairs must be matched
	///
	/// The function is parallelized with the TBB library.
	/// ## See also
	/// detail::MatchesInfo
	///
	/// ## Overloaded parameters
	///
	/// * features1: First image features
	/// * features2: Second image features
	/// * matches_info: Found matches
	// operator()(const ImageFeatures &, const ImageFeatures &, MatchesInfo &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:246
	// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features1", "features2", "matches_info"], ["const cv::detail::ImageFeatures*", "const cv::detail::ImageFeatures*", "cv::detail::MatchesInfo*"]), _)]),
	#[inline]
	fn apply(&mut self, features1: &impl crate::stitching::Detail_ImageFeaturesTraitConst, features2: &impl crate::stitching::Detail_ImageFeaturesTraitConst, matches_info: &mut impl crate::stitching::Detail_MatchesInfoTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesMatcher_operator___const_ImageFeaturesR_const_ImageFeaturesR_MatchesInfoR(self.as_raw_mut_Detail_FeaturesMatcher(), features1.as_raw_Detail_ImageFeatures(), features2.as_raw_Detail_ImageFeatures(), matches_info.as_raw_mut_Detail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs images matching.
	///
	/// ## Parameters
	/// * features: Features of the source images
	/// * pairwise_matches: Found pairwise matches
	/// * mask: Mask indicating which image pairs must be matched
	///
	/// The function is parallelized with the TBB library.
	/// ## See also
	/// detail::MatchesInfo
	///
	/// ## C++ default parameters
	/// * mask: cv::UMat()
	// operator()(const std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, const cv::UMat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:259
	// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches", "mask"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*", "const cv::UMat*"]), _)]),
	#[inline]
	fn apply_1(&mut self, features: &core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>, mask: &impl core::UMatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR_const_UMatR(self.as_raw_mut_Detail_FeaturesMatcher(), features.as_raw_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), mask.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Performs images matching.
	///
	/// ## Parameters
	/// * features: Features of the source images
	/// * pairwise_matches: Found pairwise matches
	/// * mask: Mask indicating which image pairs must be matched
	///
	/// The function is parallelized with the TBB library.
	/// ## See also
	/// detail::MatchesInfo
	///
	/// ## Note
	/// This alternative version of [Detail_FeaturesMatcherTrait::apply] function uses the following default values for its arguments:
	/// * mask: cv::UMat()
	// cv::detail::FeaturesMatcher::operator()(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:259
	// ("cv::detail::FeaturesMatcher::operator()", vec![(pred!(mut, ["features", "pairwise_matches"], ["const std::vector<cv::detail::ImageFeatures>*", "std::vector<cv::detail::MatchesInfo>*"]), _)]),
	#[inline]
	fn apply_def(&mut self, features: &core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR(self.as_raw_mut_Detail_FeaturesMatcher(), features.as_raw_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Frees unused memory allocated before if there is any.
	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:268
	// ("cv::detail::FeaturesMatcher::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FeaturesMatcher_collectGarbage(self.as_raw_mut_Detail_FeaturesMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Feature matchers base class.
// FeaturesMatcher /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:236
pub struct Detail_FeaturesMatcher {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_FeaturesMatcher }

impl Drop for Detail_FeaturesMatcher {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_FeaturesMatcher_delete(self.as_raw_mut_Detail_FeaturesMatcher()) };
	}
}

unsafe impl Send for Detail_FeaturesMatcher {}

impl crate::stitching::Detail_FeaturesMatcherTraitConst for Detail_FeaturesMatcher {
	#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesMatcherTrait for Detail_FeaturesMatcher {
	#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FeaturesMatcher, crate::stitching::Detail_FeaturesMatcherTraitConst, as_raw_Detail_FeaturesMatcher, crate::stitching::Detail_FeaturesMatcherTrait, as_raw_mut_Detail_FeaturesMatcher }

impl Detail_FeaturesMatcher {
}

boxed_cast_descendant! { Detail_FeaturesMatcher, crate::stitching::Detail_AffineBestOf2NearestMatcher, cv_detail_FeaturesMatcher_to_Detail_AffineBestOf2NearestMatcher }

boxed_cast_descendant! { Detail_FeaturesMatcher, crate::stitching::Detail_BestOf2NearestMatcher, cv_detail_FeaturesMatcher_to_Detail_BestOf2NearestMatcher }

boxed_cast_descendant! { Detail_FeaturesMatcher, crate::stitching::Detail_BestOf2NearestRangeMatcher, cv_detail_FeaturesMatcher_to_Detail_BestOf2NearestRangeMatcher }

impl std::fmt::Debug for Detail_FeaturesMatcher {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_FeaturesMatcher")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_FisheyeProjector]
// FisheyeProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:329
pub trait Detail_FisheyeProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_FisheyeProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_FisheyeProjector]
pub trait Detail_FisheyeProjectorTrait: crate::stitching::Detail_FisheyeProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_FisheyeProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:331
	// ("cv::detail::FisheyeProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_FisheyeProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:332
	// ("cv::detail::FisheyeProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_FisheyeProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// FisheyeProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:329
pub struct Detail_FisheyeProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_FisheyeProjector }

impl Drop for Detail_FisheyeProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_FisheyeProjector_delete(self.as_raw_mut_Detail_FisheyeProjector()) };
	}
}

unsafe impl Send for Detail_FisheyeProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_FisheyeProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_FisheyeProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FisheyeProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_FisheyeProjectorTraitConst for Detail_FisheyeProjector {
	#[inline] fn as_raw_Detail_FisheyeProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FisheyeProjectorTrait for Detail_FisheyeProjector {
	#[inline] fn as_raw_mut_Detail_FisheyeProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FisheyeProjector, crate::stitching::Detail_FisheyeProjectorTraitConst, as_raw_Detail_FisheyeProjector, crate::stitching::Detail_FisheyeProjectorTrait, as_raw_mut_Detail_FisheyeProjector }

impl Detail_FisheyeProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_FisheyeProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_FisheyeProjector, crate::stitching::Detail_ProjectorBase, cv_detail_FisheyeProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_FisheyeProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_FisheyeProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_FisheyeProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_FisheyeWarper]
// FisheyeWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:336
pub trait Detail_FisheyeWarperTraitConst {
	fn as_raw_Detail_FisheyeWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_FisheyeWarper]
pub trait Detail_FisheyeWarperTrait: crate::stitching::Detail_FisheyeWarperTraitConst {
	fn as_raw_mut_Detail_FisheyeWarper(&mut self) -> *mut c_void;

}

// FisheyeWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:336
pub struct Detail_FisheyeWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_FisheyeWarper }

impl Drop for Detail_FisheyeWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_FisheyeWarper_delete(self.as_raw_mut_Detail_FisheyeWarper()) };
	}
}

unsafe impl Send for Detail_FisheyeWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_FisheyeWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_FisheyeWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FisheyeWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_FisheyeWarperTraitConst for Detail_FisheyeWarper {
	#[inline] fn as_raw_Detail_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FisheyeWarperTrait for Detail_FisheyeWarper {
	#[inline] fn as_raw_mut_Detail_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_FisheyeWarper, crate::stitching::Detail_FisheyeWarperTraitConst, as_raw_Detail_FisheyeWarper, crate::stitching::Detail_FisheyeWarperTrait, as_raw_mut_Detail_FisheyeWarper }

impl Detail_FisheyeWarper {
	// FisheyeWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:339
	// ("cv::detail::FisheyeWarper::FisheyeWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_FisheyeWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_FisheyeWarper_FisheyeWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_FisheyeWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_FisheyeWarper, crate::stitching::Detail_RotationWarper, cv_detail_FisheyeWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_FisheyeWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_FisheyeWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_GainCompensator]
// GainCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:102
pub trait Detail_GainCompensatorTraitConst: crate::stitching::Detail_ExposureCompensatorTraitConst {
	fn as_raw_Detail_GainCompensator(&self) -> *const c_void;

	// gains()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:108
	// ("cv::detail::GainCompensator::gains", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn gains(&self) -> Result<core::Vector<f64>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GainCompensator_gains_const(self.as_raw_Detail_GainCompensator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<f64>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_GainCompensator]
pub trait Detail_GainCompensatorTrait: crate::stitching::Detail_ExposureCompensatorTrait + crate::stitching::Detail_GainCompensatorTraitConst {
	fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void;

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:105
	// ("cv::detail::GainCompensator::feed", vec![(pred!(mut, ["corners", "images", "masks"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	#[inline]
	fn feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_GainCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:107
	// ("cv::detail::GainCompensator::apply", vec![(pred!(mut, ["index", "corner", "image", "mask"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl ToInputOutputArray, mask: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_GainCompensator(), index, &corner, image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Exposure compensator which tries to remove exposure related artifacts by adjusting image
/// intensities, see [BL07](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BL07) and [WJ10](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_WJ10) for details.
// GainCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:102
pub struct Detail_GainCompensator {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_GainCompensator }

impl Drop for Detail_GainCompensator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_GainCompensator_delete(self.as_raw_mut_Detail_GainCompensator()) };
	}
}

unsafe impl Send for Detail_GainCompensator {}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_GainCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_GainCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_GainCompensator, crate::stitching::Detail_ExposureCompensatorTraitConst, as_raw_Detail_ExposureCompensator, crate::stitching::Detail_ExposureCompensatorTrait, as_raw_mut_Detail_ExposureCompensator }

impl crate::stitching::Detail_GainCompensatorTraitConst for Detail_GainCompensator {
	#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_GainCompensatorTrait for Detail_GainCompensator {
	#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_GainCompensator, crate::stitching::Detail_GainCompensatorTraitConst, as_raw_Detail_GainCompensator, crate::stitching::Detail_GainCompensatorTrait, as_raw_mut_Detail_GainCompensator }

impl Detail_GainCompensator {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_GainCompensator_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_GainCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_GainCompensator_to_Detail_ExposureCompensator }

impl std::fmt::Debug for Detail_GainCompensator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_GainCompensator")
			.finish()
	}
}

impl Default for Detail_GainCompensator {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_Graph]
// Graph /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:85
pub trait Detail_GraphTraitConst {
	fn as_raw_Detail_Graph(&self) -> *const c_void;

	// numVertices()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:90
	// ("cv::detail::Graph::numVertices", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_vertices(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Graph_numVertices_const(self.as_raw_Detail_Graph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_Graph]
pub trait Detail_GraphTrait: crate::stitching::Detail_GraphTraitConst {
	fn as_raw_mut_Detail_Graph(&mut self) -> *mut c_void;

	// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:89
	// ("cv::detail::Graph::create", vec![(pred!(mut, ["num_vertices"], ["int"]), _)]),
	#[inline]
	fn create(&mut self, num_vertices: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Graph_create_int(self.as_raw_mut_Detail_Graph(), num_vertices, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// addEdge(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:91
	// ("cv::detail::Graph::addEdge", vec![(pred!(mut, ["from", "to", "weight"], ["int", "int", "float"]), _)]),
	#[inline]
	fn add_edge(&mut self, from: i32, to: i32, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Graph_addEdge_int_int_float(self.as_raw_mut_Detail_Graph(), from, to, weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Graph /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:85
pub struct Detail_Graph {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_Graph }

impl Drop for Detail_Graph {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_Graph_delete(self.as_raw_mut_Detail_Graph()) };
	}
}

unsafe impl Send for Detail_Graph {}

impl crate::stitching::Detail_GraphTraitConst for Detail_Graph {
	#[inline] fn as_raw_Detail_Graph(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_GraphTrait for Detail_Graph {
	#[inline] fn as_raw_mut_Detail_Graph(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_Graph, crate::stitching::Detail_GraphTraitConst, as_raw_Detail_Graph, crate::stitching::Detail_GraphTrait, as_raw_mut_Detail_Graph }

impl Detail_Graph {
	/// ## C++ default parameters
	/// * num_vertices: 0
	// Graph(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:88
	// ("cv::detail::Graph::Graph", vec![(pred!(mut, ["num_vertices"], ["int"]), _)]),
	#[inline]
	pub fn new(num_vertices: i32) -> Result<crate::stitching::Detail_Graph> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Graph_Graph_int(num_vertices, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_Graph::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * num_vertices: 0
	// cv::detail::Graph::Graph() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:88
	// ("cv::detail::Graph::Graph", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_Graph> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_Graph_Graph(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_Graph::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Detail_Graph {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_Graph")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_GraphCutSeamFinder]
// GraphCutSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:236
pub trait Detail_GraphCutSeamFinderTraitConst: crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst + crate::stitching::Detail_SeamFinderTraitConst {
	fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_GraphCutSeamFinder]
pub trait Detail_GraphCutSeamFinderTrait: crate::stitching::Detail_GraphCutSeamFinderBaseTrait + crate::stitching::Detail_GraphCutSeamFinderTraitConst + crate::stitching::Detail_SeamFinderTrait {
	fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void;

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:244
	// ("cv::detail::GraphCutSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GraphCutSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_GraphCutSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Minimum graph cut-based seam estimator. See details in [V03](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_V03) .
// GraphCutSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:236
pub struct Detail_GraphCutSeamFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_GraphCutSeamFinder }

impl Drop for Detail_GraphCutSeamFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_GraphCutSeamFinder_delete(self.as_raw_mut_Detail_GraphCutSeamFinder()) };
	}
}

unsafe impl Send for Detail_GraphCutSeamFinder {}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_GraphCutSeamFinder, crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst, as_raw_Detail_GraphCutSeamFinderBase, crate::stitching::Detail_GraphCutSeamFinderBaseTrait, as_raw_mut_Detail_GraphCutSeamFinderBase }

impl crate::stitching::Detail_SeamFinderTraitConst for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_GraphCutSeamFinder, crate::stitching::Detail_SeamFinderTraitConst, as_raw_Detail_SeamFinder, crate::stitching::Detail_SeamFinderTrait, as_raw_mut_Detail_SeamFinder }

impl crate::stitching::Detail_GraphCutSeamFinderTraitConst for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderTrait for Detail_GraphCutSeamFinder {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_GraphCutSeamFinder, crate::stitching::Detail_GraphCutSeamFinderTraitConst, as_raw_Detail_GraphCutSeamFinder, crate::stitching::Detail_GraphCutSeamFinderTrait, as_raw_mut_Detail_GraphCutSeamFinder }

impl Detail_GraphCutSeamFinder {
	/// ## C++ default parameters
	/// * cost_type: COST_COLOR_GRAD
	/// * terminal_cost: 10000.f
	/// * bad_region_penalty: 1000.f
	// GraphCutSeamFinder(int, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:239
	// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, ["cost_type", "terminal_cost", "bad_region_penalty"], ["int", "float", "float"]), _)]),
	#[inline]
	pub fn new(cost_type: i32, terminal_cost: f32, bad_region_penalty: f32) -> Result<crate::stitching::Detail_GraphCutSeamFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(cost_type, terminal_cost, bad_region_penalty, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_GraphCutSeamFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * cost_type: COST_COLOR_GRAD
	/// * terminal_cost: 10000.f
	/// * bad_region_penalty: 1000.f
	// cv::detail::GraphCutSeamFinder::GraphCutSeamFinder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:239
	// ("cv::detail::GraphCutSeamFinder::GraphCutSeamFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_GraphCutSeamFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GraphCutSeamFinder_GraphCutSeamFinder(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_GraphCutSeamFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_GraphCutSeamFinder, crate::stitching::Detail_GraphCutSeamFinderBase, cv_detail_GraphCutSeamFinder_to_Detail_GraphCutSeamFinderBase }

boxed_cast_base! { Detail_GraphCutSeamFinder, crate::stitching::Detail_SeamFinder, cv_detail_GraphCutSeamFinder_to_Detail_SeamFinder }

impl std::fmt::Debug for Detail_GraphCutSeamFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_GraphCutSeamFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_GraphCutSeamFinderBase]
// GraphCutSeamFinderBase /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:228
pub trait Detail_GraphCutSeamFinderBaseTraitConst {
	fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_GraphCutSeamFinderBase]
pub trait Detail_GraphCutSeamFinderBaseTrait: crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst {
	fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void;

}

/// Base class for all minimum graph-cut-based seam estimators.
// GraphCutSeamFinderBase /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:228
pub struct Detail_GraphCutSeamFinderBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_GraphCutSeamFinderBase }

impl Drop for Detail_GraphCutSeamFinderBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_GraphCutSeamFinderBase_delete(self.as_raw_mut_Detail_GraphCutSeamFinderBase()) };
	}
}

unsafe impl Send for Detail_GraphCutSeamFinderBase {}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for Detail_GraphCutSeamFinderBase {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for Detail_GraphCutSeamFinderBase {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_GraphCutSeamFinderBase, crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst, as_raw_Detail_GraphCutSeamFinderBase, crate::stitching::Detail_GraphCutSeamFinderBaseTrait, as_raw_mut_Detail_GraphCutSeamFinderBase }

impl Detail_GraphCutSeamFinderBase {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_GraphCutSeamFinderBase_defaultNew_const()) }
	}

}

impl std::fmt::Debug for Detail_GraphCutSeamFinderBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_GraphCutSeamFinderBase")
			.finish()
	}
}

impl Default for Detail_GraphCutSeamFinderBase {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_GraphEdge]
// GraphEdge /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:72
pub trait Detail_GraphEdgeTraitConst {
	fn as_raw_Detail_GraphEdge(&self) -> *const c_void;

	// cv::detail::GraphEdge::from() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::from", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn from(&self) -> i32 {
		let ret = unsafe { sys::cv_detail_GraphEdge_propFrom_const(self.as_raw_Detail_GraphEdge()) };
		ret
	}

	// cv::detail::GraphEdge::to() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::to", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn to(&self) -> i32 {
		let ret = unsafe { sys::cv_detail_GraphEdge_propTo_const(self.as_raw_Detail_GraphEdge()) };
		ret
	}

	// cv::detail::GraphEdge::weight() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:79
	// ("cv::detail::GraphEdge::weight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weight(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_GraphEdge_propWeight_const(self.as_raw_Detail_GraphEdge()) };
		ret
	}

	// operator<(const GraphEdge &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:75
	// ("cv::detail::GraphEdge::operator<", vec![(pred!(const, ["other"], ["const cv::detail::GraphEdge*"]), _)]),
	#[inline]
	fn less_than(&self, other: &impl crate::stitching::Detail_GraphEdgeTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GraphEdge_operatorL_const_const_GraphEdgeR(self.as_raw_Detail_GraphEdge(), other.as_raw_Detail_GraphEdge(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator>(const GraphEdge &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:76
	// ("cv::detail::GraphEdge::operator>", vec![(pred!(const, ["other"], ["const cv::detail::GraphEdge*"]), _)]),
	#[inline]
	fn greater_than(&self, other: &impl crate::stitching::Detail_GraphEdgeTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GraphEdge_operatorG_const_const_GraphEdgeR(self.as_raw_Detail_GraphEdge(), other.as_raw_Detail_GraphEdge(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_GraphEdge]
pub trait Detail_GraphEdgeTrait: crate::stitching::Detail_GraphEdgeTraitConst {
	fn as_raw_mut_Detail_GraphEdge(&mut self) -> *mut c_void;

	// cv::detail::GraphEdge::setFrom(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::setFrom", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_from(&mut self, val: i32) {
		let ret = unsafe { sys::cv_detail_GraphEdge_propFrom_const_int(self.as_raw_mut_Detail_GraphEdge(), val) };
		ret
	}

	// cv::detail::GraphEdge::setTo(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:78
	// ("cv::detail::GraphEdge::setTo", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_to(&mut self, val: i32) {
		let ret = unsafe { sys::cv_detail_GraphEdge_propTo_const_int(self.as_raw_mut_Detail_GraphEdge(), val) };
		ret
	}

	// cv::detail::GraphEdge::setWeight(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:79
	// ("cv::detail::GraphEdge::setWeight", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_weight(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_GraphEdge_propWeight_const_float(self.as_raw_mut_Detail_GraphEdge(), val) };
		ret
	}

}

// GraphEdge /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:72
pub struct Detail_GraphEdge {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_GraphEdge }

impl Drop for Detail_GraphEdge {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_GraphEdge_delete(self.as_raw_mut_Detail_GraphEdge()) };
	}
}

unsafe impl Send for Detail_GraphEdge {}

impl crate::stitching::Detail_GraphEdgeTraitConst for Detail_GraphEdge {
	#[inline] fn as_raw_Detail_GraphEdge(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_GraphEdgeTrait for Detail_GraphEdge {
	#[inline] fn as_raw_mut_Detail_GraphEdge(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_GraphEdge, crate::stitching::Detail_GraphEdgeTraitConst, as_raw_Detail_GraphEdge, crate::stitching::Detail_GraphEdgeTrait, as_raw_mut_Detail_GraphEdge }

impl Detail_GraphEdge {
	// GraphEdge(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/util.hpp:74
	// ("cv::detail::GraphEdge::GraphEdge", vec![(pred!(mut, ["from", "to", "weight"], ["int", "int", "float"]), _)]),
	#[inline]
	pub fn new(from: i32, to: i32, weight: f32) -> Result<crate::stitching::Detail_GraphEdge> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_GraphEdge_GraphEdge_int_int_float(from, to, weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_GraphEdge::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Detail_GraphEdge {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_GraphEdge")
			.field("from", &crate::stitching::Detail_GraphEdgeTraitConst::from(self))
			.field("to", &crate::stitching::Detail_GraphEdgeTraitConst::to(self))
			.field("weight", &crate::stitching::Detail_GraphEdgeTraitConst::weight(self))
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_HomographyBasedEstimator]
// HomographyBasedEstimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:98
pub trait Detail_HomographyBasedEstimatorTraitConst: crate::stitching::Detail_EstimatorTraitConst {
	fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_HomographyBasedEstimator]
pub trait Detail_HomographyBasedEstimatorTrait: crate::stitching::Detail_EstimatorTrait + crate::stitching::Detail_HomographyBasedEstimatorTraitConst {
	fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void;

}

/// Homography based rotation estimator.
// HomographyBasedEstimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:98
pub struct Detail_HomographyBasedEstimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_HomographyBasedEstimator }

impl Drop for Detail_HomographyBasedEstimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_HomographyBasedEstimator_delete(self.as_raw_mut_Detail_HomographyBasedEstimator()) };
	}
}

unsafe impl Send for Detail_HomographyBasedEstimator {}

impl crate::stitching::Detail_EstimatorTraitConst for Detail_HomographyBasedEstimator {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_HomographyBasedEstimator {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_HomographyBasedEstimator, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_HomographyBasedEstimatorTraitConst for Detail_HomographyBasedEstimator {
	#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_HomographyBasedEstimatorTrait for Detail_HomographyBasedEstimator {
	#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_HomographyBasedEstimator, crate::stitching::Detail_HomographyBasedEstimatorTraitConst, as_raw_Detail_HomographyBasedEstimator, crate::stitching::Detail_HomographyBasedEstimatorTrait, as_raw_mut_Detail_HomographyBasedEstimator }

impl Detail_HomographyBasedEstimator {
	/// ## C++ default parameters
	/// * is_focals_estimated: false
	// HomographyBasedEstimator(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:101
	// ("cv::detail::HomographyBasedEstimator::HomographyBasedEstimator", vec![(pred!(mut, ["is_focals_estimated"], ["bool"]), _)]),
	#[inline]
	pub fn new(is_focals_estimated: bool) -> Result<crate::stitching::Detail_HomographyBasedEstimator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(is_focals_estimated, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_HomographyBasedEstimator::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * is_focals_estimated: false
	// cv::detail::HomographyBasedEstimator::HomographyBasedEstimator() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:101
	// ("cv::detail::HomographyBasedEstimator::HomographyBasedEstimator", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_HomographyBasedEstimator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_HomographyBasedEstimator_HomographyBasedEstimator(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_HomographyBasedEstimator::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_HomographyBasedEstimator, crate::stitching::Detail_Estimator, cv_detail_HomographyBasedEstimator_to_Detail_Estimator }

impl std::fmt::Debug for Detail_HomographyBasedEstimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_HomographyBasedEstimator")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_ImageFeatures]
// ImageFeatures /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:62
pub trait Detail_ImageFeaturesTraitConst {
	fn as_raw_Detail_ImageFeatures(&self) -> *const c_void;

	// cv::detail::ImageFeatures::img_idx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:64
	// ("cv::detail::ImageFeatures::img_idx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn img_idx(&self) -> i32 {
		let ret = unsafe { sys::cv_detail_ImageFeatures_propImg_idx_const(self.as_raw_Detail_ImageFeatures()) };
		ret
	}

	// cv::detail::ImageFeatures::img_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:65
	// ("cv::detail::ImageFeatures::img_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn img_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_ImageFeatures_propImg_size_const(self.as_raw_Detail_ImageFeatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::detail::ImageFeatures::keypoints() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:66
	// ("cv::detail::ImageFeatures::keypoints", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn keypoints(&self) -> core::Vector<core::KeyPoint> {
		let ret = unsafe { sys::cv_detail_ImageFeatures_propKeypoints_const(self.as_raw_Detail_ImageFeatures()) };
		let ret = unsafe { core::Vector::<core::KeyPoint>::opencv_from_extern(ret) };
		ret
	}

	// cv::detail::ImageFeatures::descriptors() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:67
	// ("cv::detail::ImageFeatures::descriptors", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn descriptors(&self) -> core::UMat {
		let ret = unsafe { sys::cv_detail_ImageFeatures_propDescriptors_const(self.as_raw_Detail_ImageFeatures()) };
		let ret = unsafe { core::UMat::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_ImageFeatures]
pub trait Detail_ImageFeaturesTrait: crate::stitching::Detail_ImageFeaturesTraitConst {
	fn as_raw_mut_Detail_ImageFeatures(&mut self) -> *mut c_void;

	// cv::detail::ImageFeatures::setImg_idx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:64
	// ("cv::detail::ImageFeatures::setImg_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_img_idx(&mut self, val: i32) {
		let ret = unsafe { sys::cv_detail_ImageFeatures_propImg_idx_const_int(self.as_raw_mut_Detail_ImageFeatures(), val) };
		ret
	}

	// cv::detail::ImageFeatures::setImg_size(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:65
	// ("cv::detail::ImageFeatures::setImg_size", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_img_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_detail_ImageFeatures_propImg_size_const_Size(self.as_raw_mut_Detail_ImageFeatures(), &val) };
		ret
	}

	// cv::detail::ImageFeatures::setKeypoints(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:66
	// ("cv::detail::ImageFeatures::setKeypoints", vec![(pred!(mut, ["val"], ["const std::vector<cv::KeyPoint>"]), _)]),
	#[inline]
	fn set_keypoints(&mut self, val: core::Vector<core::KeyPoint>) {
		let ret = unsafe { sys::cv_detail_ImageFeatures_propKeypoints_const_vectorLKeyPointG(self.as_raw_mut_Detail_ImageFeatures(), val.as_raw_VectorOfKeyPoint()) };
		ret
	}

	// cv::detail::ImageFeatures::setDescriptors(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:67
	// ("cv::detail::ImageFeatures::setDescriptors", vec![(pred!(mut, ["val"], ["const cv::UMat"]), _)]),
	#[inline]
	fn set_descriptors(&mut self, val: core::UMat) {
		let ret = unsafe { sys::cv_detail_ImageFeatures_propDescriptors_const_UMat(self.as_raw_mut_Detail_ImageFeatures(), val.as_raw_UMat()) };
		ret
	}

}

/// Structure containing image keypoints and descriptors.
// ImageFeatures /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:62
pub struct Detail_ImageFeatures {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_ImageFeatures }

impl Drop for Detail_ImageFeatures {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_ImageFeatures_delete(self.as_raw_mut_Detail_ImageFeatures()) };
	}
}

unsafe impl Send for Detail_ImageFeatures {}

impl crate::stitching::Detail_ImageFeaturesTraitConst for Detail_ImageFeatures {
	#[inline] fn as_raw_Detail_ImageFeatures(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ImageFeaturesTrait for Detail_ImageFeatures {
	#[inline] fn as_raw_mut_Detail_ImageFeatures(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_ImageFeatures, crate::stitching::Detail_ImageFeaturesTraitConst, as_raw_Detail_ImageFeatures, crate::stitching::Detail_ImageFeaturesTrait, as_raw_mut_Detail_ImageFeatures }

impl Detail_ImageFeatures {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_ImageFeatures_defaultNew_const()) }
	}

}

impl Clone for Detail_ImageFeatures {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_detail_ImageFeatures_implicitClone_const(self.as_raw_Detail_ImageFeatures())) }
	}
}

impl std::fmt::Debug for Detail_ImageFeatures {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_ImageFeatures")
			.field("img_idx", &crate::stitching::Detail_ImageFeaturesTraitConst::img_idx(self))
			.field("img_size", &crate::stitching::Detail_ImageFeaturesTraitConst::img_size(self))
			.field("keypoints", &crate::stitching::Detail_ImageFeaturesTraitConst::keypoints(self))
			.field("descriptors", &crate::stitching::Detail_ImageFeaturesTraitConst::descriptors(self))
			.finish()
	}
}

impl Default for Detail_ImageFeatures {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_MatchesInfo]
// MatchesInfo /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:221
pub trait Detail_MatchesInfoTraitConst {
	fn as_raw_Detail_MatchesInfo(&self) -> *const c_void;

	/// Images indices (optional)
	// cv::detail::MatchesInfo::src_img_idx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:227
	// ("cv::detail::MatchesInfo::src_img_idx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn src_img_idx(&self) -> i32 {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propSrc_img_idx_const(self.as_raw_Detail_MatchesInfo()) };
		ret
	}

	/// Images indices (optional)
	// cv::detail::MatchesInfo::dst_img_idx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:227
	// ("cv::detail::MatchesInfo::dst_img_idx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn dst_img_idx(&self) -> i32 {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propDst_img_idx_const(self.as_raw_Detail_MatchesInfo()) };
		ret
	}

	// cv::detail::MatchesInfo::matches() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:228
	// ("cv::detail::MatchesInfo::matches", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn matches(&self) -> core::Vector<core::DMatch> {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propMatches_const(self.as_raw_Detail_MatchesInfo()) };
		let ret = unsafe { core::Vector::<core::DMatch>::opencv_from_extern(ret) };
		ret
	}

	/// Geometrically consistent matches mask
	// cv::detail::MatchesInfo::inliers_mask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:229
	// ("cv::detail::MatchesInfo::inliers_mask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn inliers_mask(&self) -> core::Vector<u8> {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propInliers_mask_const(self.as_raw_Detail_MatchesInfo()) };
		let ret = unsafe { core::Vector::<u8>::opencv_from_extern(ret) };
		ret
	}

	/// Number of geometrically consistent matches
	// cv::detail::MatchesInfo::num_inliers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:230
	// ("cv::detail::MatchesInfo::num_inliers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_inliers(&self) -> i32 {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propNum_inliers_const(self.as_raw_Detail_MatchesInfo()) };
		ret
	}

	/// Estimated transformation
	// cv::detail::MatchesInfo::H() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:231
	// ("cv::detail::MatchesInfo::H", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn h(&self) -> core::Mat {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propH_const(self.as_raw_Detail_MatchesInfo()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	/// Confidence two images are from the same panorama
	// cv::detail::MatchesInfo::confidence() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:232
	// ("cv::detail::MatchesInfo::confidence", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn confidence(&self) -> f64 {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propConfidence_const(self.as_raw_Detail_MatchesInfo()) };
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_MatchesInfo]
pub trait Detail_MatchesInfoTrait: crate::stitching::Detail_MatchesInfoTraitConst {
	fn as_raw_mut_Detail_MatchesInfo(&mut self) -> *mut c_void;

	/// Images indices (optional)
	// cv::detail::MatchesInfo::setSrc_img_idx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:227
	// ("cv::detail::MatchesInfo::setSrc_img_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_src_img_idx(&mut self, val: i32) {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propSrc_img_idx_const_int(self.as_raw_mut_Detail_MatchesInfo(), val) };
		ret
	}

	/// Images indices (optional)
	// cv::detail::MatchesInfo::setDst_img_idx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:227
	// ("cv::detail::MatchesInfo::setDst_img_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_dst_img_idx(&mut self, val: i32) {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propDst_img_idx_const_int(self.as_raw_mut_Detail_MatchesInfo(), val) };
		ret
	}

	// cv::detail::MatchesInfo::setMatches(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:228
	// ("cv::detail::MatchesInfo::setMatches", vec![(pred!(mut, ["val"], ["const std::vector<cv::DMatch>"]), _)]),
	#[inline]
	fn set_matches(&mut self, val: core::Vector<core::DMatch>) {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propMatches_const_vectorLDMatchG(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_VectorOfDMatch()) };
		ret
	}

	/// Geometrically consistent matches mask
	// cv::detail::MatchesInfo::setInliers_mask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:229
	// ("cv::detail::MatchesInfo::setInliers_mask", vec![(pred!(mut, ["val"], ["const std::vector<unsigned char>"]), _)]),
	#[inline]
	fn set_inliers_mask(&mut self, val: core::Vector<u8>) {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propInliers_mask_const_vectorLunsigned_charG(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_VectorOfu8()) };
		ret
	}

	/// Number of geometrically consistent matches
	// cv::detail::MatchesInfo::setNum_inliers(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:230
	// ("cv::detail::MatchesInfo::setNum_inliers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_inliers(&mut self, val: i32) {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propNum_inliers_const_int(self.as_raw_mut_Detail_MatchesInfo(), val) };
		ret
	}

	/// Estimated transformation
	// cv::detail::MatchesInfo::setH(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:231
	// ("cv::detail::MatchesInfo::setH", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_h(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propH_const_Mat(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_Mat()) };
		ret
	}

	/// Confidence two images are from the same panorama
	// cv::detail::MatchesInfo::setConfidence(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:232
	// ("cv::detail::MatchesInfo::setConfidence", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_confidence(&mut self, val: f64) {
		let ret = unsafe { sys::cv_detail_MatchesInfo_propConfidence_const_double(self.as_raw_mut_Detail_MatchesInfo(), val) };
		ret
	}

	// operator=(const MatchesInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:225
	// ("cv::detail::MatchesInfo::operator=", vec![(pred!(mut, ["other"], ["const cv::detail::MatchesInfo*"]), _)]),
	#[inline]
	fn set(&mut self, other: &impl crate::stitching::Detail_MatchesInfoTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MatchesInfo_operatorST_const_MatchesInfoR(self.as_raw_mut_Detail_MatchesInfo(), other.as_raw_Detail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Structure containing information about matches between two images.
///
/// It's assumed that there is a transformation between those images. Transformation may be
/// homography or affine transformation based on selected matcher.
/// ## See also
/// detail::FeaturesMatcher
// MatchesInfo /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:221
pub struct Detail_MatchesInfo {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_MatchesInfo }

impl Drop for Detail_MatchesInfo {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_MatchesInfo_delete(self.as_raw_mut_Detail_MatchesInfo()) };
	}
}

unsafe impl Send for Detail_MatchesInfo {}

impl crate::stitching::Detail_MatchesInfoTraitConst for Detail_MatchesInfo {
	#[inline] fn as_raw_Detail_MatchesInfo(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_MatchesInfoTrait for Detail_MatchesInfo {
	#[inline] fn as_raw_mut_Detail_MatchesInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_MatchesInfo, crate::stitching::Detail_MatchesInfoTraitConst, as_raw_Detail_MatchesInfo, crate::stitching::Detail_MatchesInfoTrait, as_raw_mut_Detail_MatchesInfo }

impl Detail_MatchesInfo {
	// MatchesInfo()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:223
	// ("cv::detail::MatchesInfo::MatchesInfo", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_MatchesInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MatchesInfo_MatchesInfo(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_MatchesInfo::opencv_from_extern(ret) };
		Ok(ret)
	}

	// MatchesInfo(const MatchesInfo &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:224
	// ("cv::detail::MatchesInfo::MatchesInfo", vec![(pred!(mut, ["other"], ["const cv::detail::MatchesInfo*"]), _)]),
	#[inline]
	pub fn copy(other: &impl crate::stitching::Detail_MatchesInfoTraitConst) -> Result<crate::stitching::Detail_MatchesInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(other.as_raw_Detail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_MatchesInfo::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Detail_MatchesInfo {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_MatchesInfo")
			.field("src_img_idx", &crate::stitching::Detail_MatchesInfoTraitConst::src_img_idx(self))
			.field("dst_img_idx", &crate::stitching::Detail_MatchesInfoTraitConst::dst_img_idx(self))
			.field("matches", &crate::stitching::Detail_MatchesInfoTraitConst::matches(self))
			.field("inliers_mask", &crate::stitching::Detail_MatchesInfoTraitConst::inliers_mask(self))
			.field("num_inliers", &crate::stitching::Detail_MatchesInfoTraitConst::num_inliers(self))
			.field("h", &crate::stitching::Detail_MatchesInfoTraitConst::h(self))
			.field("confidence", &crate::stitching::Detail_MatchesInfoTraitConst::confidence(self))
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_MercatorProjector]
// MercatorProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:442
pub trait Detail_MercatorProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_MercatorProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_MercatorProjector]
pub trait Detail_MercatorProjectorTrait: crate::stitching::Detail_MercatorProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_MercatorProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:444
	// ("cv::detail::MercatorProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_MercatorProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:445
	// ("cv::detail::MercatorProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_MercatorProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MercatorProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:442
pub struct Detail_MercatorProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_MercatorProjector }

impl Drop for Detail_MercatorProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_MercatorProjector_delete(self.as_raw_mut_Detail_MercatorProjector()) };
	}
}

unsafe impl Send for Detail_MercatorProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_MercatorProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_MercatorProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_MercatorProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_MercatorProjectorTraitConst for Detail_MercatorProjector {
	#[inline] fn as_raw_Detail_MercatorProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_MercatorProjectorTrait for Detail_MercatorProjector {
	#[inline] fn as_raw_mut_Detail_MercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_MercatorProjector, crate::stitching::Detail_MercatorProjectorTraitConst, as_raw_Detail_MercatorProjector, crate::stitching::Detail_MercatorProjectorTrait, as_raw_mut_Detail_MercatorProjector }

impl Detail_MercatorProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_MercatorProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_MercatorProjector, crate::stitching::Detail_ProjectorBase, cv_detail_MercatorProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_MercatorProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_MercatorProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_MercatorProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_MercatorWarper]
// MercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:449
pub trait Detail_MercatorWarperTraitConst {
	fn as_raw_Detail_MercatorWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_MercatorWarper]
pub trait Detail_MercatorWarperTrait: crate::stitching::Detail_MercatorWarperTraitConst {
	fn as_raw_mut_Detail_MercatorWarper(&mut self) -> *mut c_void;

}

// MercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:449
pub struct Detail_MercatorWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_MercatorWarper }

impl Drop for Detail_MercatorWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_MercatorWarper_delete(self.as_raw_mut_Detail_MercatorWarper()) };
	}
}

unsafe impl Send for Detail_MercatorWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_MercatorWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_MercatorWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_MercatorWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_MercatorWarperTraitConst for Detail_MercatorWarper {
	#[inline] fn as_raw_Detail_MercatorWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_MercatorWarperTrait for Detail_MercatorWarper {
	#[inline] fn as_raw_mut_Detail_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_MercatorWarper, crate::stitching::Detail_MercatorWarperTraitConst, as_raw_Detail_MercatorWarper, crate::stitching::Detail_MercatorWarperTrait, as_raw_mut_Detail_MercatorWarper }

impl Detail_MercatorWarper {
	// MercatorWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:452
	// ("cv::detail::MercatorWarper::MercatorWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_MercatorWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MercatorWarper_MercatorWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_MercatorWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_MercatorWarper, crate::stitching::Detail_RotationWarper, cv_detail_MercatorWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_MercatorWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_MercatorWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_MultiBandBlender]
// MultiBandBlender /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:127
pub trait Detail_MultiBandBlenderTraitConst: crate::stitching::Detail_BlenderTraitConst {
	fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void;

	// numBands()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:132
	// ("cv::detail::MultiBandBlender::numBands", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_bands(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MultiBandBlender_numBands_const(self.as_raw_Detail_MultiBandBlender(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_MultiBandBlender]
pub trait Detail_MultiBandBlenderTrait: crate::stitching::Detail_BlenderTrait + crate::stitching::Detail_MultiBandBlenderTraitConst {
	fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void;

	// setNumBands(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:133
	// ("cv::detail::MultiBandBlender::setNumBands", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_num_bands(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MultiBandBlender_setNumBands_int(self.as_raw_mut_Detail_MultiBandBlender(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// prepare(Rect)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:135
	// ("cv::detail::MultiBandBlender::prepare", vec![(pred!(mut, ["dst_roi"], ["cv::Rect"]), _)]),
	#[inline]
	fn prepare(&mut self, dst_roi: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MultiBandBlender_prepare_Rect(self.as_raw_mut_Detail_MultiBandBlender(), &dst_roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// feed(InputArray, InputArray, Point)(InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:136
	// ("cv::detail::MultiBandBlender::feed", vec![(pred!(mut, ["img", "mask", "tl"], ["const cv::_InputArray*", "const cv::_InputArray*", "cv::Point"]), _)]),
	#[inline]
	fn feed(&mut self, img: &impl ToInputArray, mask: &impl ToInputArray, tl: core::Point) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_MultiBandBlender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), &tl, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// blend(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:137
	// ("cv::detail::MultiBandBlender::blend", vec![(pred!(mut, ["dst", "dst_mask"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	#[inline]
	fn blend(&mut self, dst: &mut impl ToInputOutputArray, dst_mask: &mut impl ToInputOutputArray) -> Result<()> {
		input_output_array_arg!(dst);
		input_output_array_arg!(dst_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Detail_MultiBandBlender(), dst.as_raw__InputOutputArray(), dst_mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Blender which uses multi-band blending algorithm (see [BA83](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BA83)).
// MultiBandBlender /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:127
pub struct Detail_MultiBandBlender {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_MultiBandBlender }

impl Drop for Detail_MultiBandBlender {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_MultiBandBlender_delete(self.as_raw_mut_Detail_MultiBandBlender()) };
	}
}

unsafe impl Send for Detail_MultiBandBlender {}

impl crate::stitching::Detail_BlenderTraitConst for Detail_MultiBandBlender {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for Detail_MultiBandBlender {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_MultiBandBlender, crate::stitching::Detail_BlenderTraitConst, as_raw_Detail_Blender, crate::stitching::Detail_BlenderTrait, as_raw_mut_Detail_Blender }

impl crate::stitching::Detail_MultiBandBlenderTraitConst for Detail_MultiBandBlender {
	#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_MultiBandBlenderTrait for Detail_MultiBandBlender {
	#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_MultiBandBlender, crate::stitching::Detail_MultiBandBlenderTraitConst, as_raw_Detail_MultiBandBlender, crate::stitching::Detail_MultiBandBlenderTrait, as_raw_mut_Detail_MultiBandBlender }

impl Detail_MultiBandBlender {
	/// ## C++ default parameters
	/// * try_gpu: false
	/// * num_bands: 5
	/// * weight_type: CV_32F
	// MultiBandBlender(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:130
	// ("cv::detail::MultiBandBlender::MultiBandBlender", vec![(pred!(mut, ["try_gpu", "num_bands", "weight_type"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn new(try_gpu: i32, num_bands: i32, weight_type: i32) -> Result<crate::stitching::Detail_MultiBandBlender> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(try_gpu, num_bands, weight_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_MultiBandBlender::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * try_gpu: false
	/// * num_bands: 5
	/// * weight_type: CV_32F
	// cv::detail::MultiBandBlender::MultiBandBlender() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/blenders.hpp:130
	// ("cv::detail::MultiBandBlender::MultiBandBlender", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_MultiBandBlender> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_MultiBandBlender_MultiBandBlender(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_MultiBandBlender::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_MultiBandBlender, crate::stitching::Detail_Blender, cv_detail_MultiBandBlender_to_Detail_Blender }

impl std::fmt::Debug for Detail_MultiBandBlender {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_MultiBandBlender")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_NoBundleAdjuster]
// NoBundleAdjuster /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:217
pub trait Detail_NoBundleAdjusterTraitConst: crate::stitching::Detail_BundleAdjusterBaseTraitConst {
	fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_NoBundleAdjuster]
pub trait Detail_NoBundleAdjusterTrait: crate::stitching::Detail_BundleAdjusterBaseTrait + crate::stitching::Detail_NoBundleAdjusterTraitConst {
	fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void;

}

/// Stub bundle adjuster that does nothing.
// NoBundleAdjuster /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:217
pub struct Detail_NoBundleAdjuster {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_NoBundleAdjuster }

impl Drop for Detail_NoBundleAdjuster {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_NoBundleAdjuster_delete(self.as_raw_mut_Detail_NoBundleAdjuster()) };
	}
}

unsafe impl Send for Detail_NoBundleAdjuster {}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_NoBundleAdjuster, crate::stitching::Detail_BundleAdjusterBaseTraitConst, as_raw_Detail_BundleAdjusterBase, crate::stitching::Detail_BundleAdjusterBaseTrait, as_raw_mut_Detail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_NoBundleAdjuster, crate::stitching::Detail_EstimatorTraitConst, as_raw_Detail_Estimator, crate::stitching::Detail_EstimatorTrait, as_raw_mut_Detail_Estimator }

impl crate::stitching::Detail_NoBundleAdjusterTraitConst for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_NoBundleAdjusterTrait for Detail_NoBundleAdjuster {
	#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_NoBundleAdjuster, crate::stitching::Detail_NoBundleAdjusterTraitConst, as_raw_Detail_NoBundleAdjuster, crate::stitching::Detail_NoBundleAdjusterTrait, as_raw_mut_Detail_NoBundleAdjuster }

impl Detail_NoBundleAdjuster {
	// NoBundleAdjuster()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/motion_estimators.hpp:220
	// ("cv::detail::NoBundleAdjuster::NoBundleAdjuster", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_NoBundleAdjuster> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_NoBundleAdjuster_NoBundleAdjuster(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_NoBundleAdjuster::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_NoBundleAdjuster, crate::stitching::Detail_BundleAdjusterBase, cv_detail_NoBundleAdjuster_to_Detail_BundleAdjusterBase }

boxed_cast_base! { Detail_NoBundleAdjuster, crate::stitching::Detail_Estimator, cv_detail_NoBundleAdjuster_to_Detail_Estimator }

impl std::fmt::Debug for Detail_NoBundleAdjuster {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_NoBundleAdjuster")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_NoExposureCompensator]
// NoExposureCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:91
pub trait Detail_NoExposureCompensatorTraitConst: crate::stitching::Detail_ExposureCompensatorTraitConst {
	fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_NoExposureCompensator]
pub trait Detail_NoExposureCompensatorTrait: crate::stitching::Detail_ExposureCompensatorTrait + crate::stitching::Detail_NoExposureCompensatorTraitConst {
	fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void;

	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<std::pair<UMat, uchar>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:94
	// ("cv::detail::NoExposureCompensator::feed", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["const std::vector<cv::Point>*", "const std::vector<cv::UMat>*", "const std::vector<std::pair<cv::UMat, unsigned char>>*"]), _)]),
	#[inline]
	fn feed(&mut self, unnamed: &core::Vector<core::Point>, unnamed_1: &core::Vector<core::UMat>, unnamed_2: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_NoExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_NoExposureCompensator(), unnamed.as_raw_VectorOfPoint(), unnamed_1.as_raw_VectorOfUMat(), unnamed_2.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// apply(int, Point, InputOutputArray, InputArray)(Primitive, SimpleClass, InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:96
	// ("cv::detail::NoExposureCompensator::apply", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed", "unnamed"], ["int", "cv::Point", "const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn apply(&mut self, unnamed: i32, unnamed_1: core::Point, unnamed_2: &mut impl ToInputOutputArray, unnamed_3: &impl ToInputArray) -> Result<()> {
		input_output_array_arg!(unnamed_2);
		input_array_arg!(unnamed_3);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_NoExposureCompensator(), unnamed, &unnamed_1, unnamed_2.as_raw__InputOutputArray(), unnamed_3.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Stub exposure compensator which does nothing.
// NoExposureCompensator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/exposure_compensate.hpp:91
pub struct Detail_NoExposureCompensator {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_NoExposureCompensator }

impl Drop for Detail_NoExposureCompensator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_NoExposureCompensator_delete(self.as_raw_mut_Detail_NoExposureCompensator()) };
	}
}

unsafe impl Send for Detail_NoExposureCompensator {}

impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_NoExposureCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_NoExposureCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_NoExposureCompensator, crate::stitching::Detail_ExposureCompensatorTraitConst, as_raw_Detail_ExposureCompensator, crate::stitching::Detail_ExposureCompensatorTrait, as_raw_mut_Detail_ExposureCompensator }

impl crate::stitching::Detail_NoExposureCompensatorTraitConst for Detail_NoExposureCompensator {
	#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_NoExposureCompensatorTrait for Detail_NoExposureCompensator {
	#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_NoExposureCompensator, crate::stitching::Detail_NoExposureCompensatorTraitConst, as_raw_Detail_NoExposureCompensator, crate::stitching::Detail_NoExposureCompensatorTrait, as_raw_mut_Detail_NoExposureCompensator }

impl Detail_NoExposureCompensator {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_NoExposureCompensator_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_NoExposureCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_NoExposureCompensator_to_Detail_ExposureCompensator }

impl std::fmt::Debug for Detail_NoExposureCompensator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_NoExposureCompensator")
			.finish()
	}
}

impl Default for Detail_NoExposureCompensator {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_NoSeamFinder]
// NoSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:74
pub trait Detail_NoSeamFinderTraitConst: crate::stitching::Detail_SeamFinderTraitConst {
	fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_NoSeamFinder]
pub trait Detail_NoSeamFinderTrait: crate::stitching::Detail_NoSeamFinderTraitConst + crate::stitching::Detail_SeamFinderTrait {
	fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void;

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:77
	// ("cv::detail::NoSeamFinder::find", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn find(&mut self, unnamed: &core::Vector<core::UMat>, unnamed_1: &core::Vector<core::Point>, unnamed_2: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_NoSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_NoSeamFinder(), unnamed.as_raw_VectorOfUMat(), unnamed_1.as_raw_VectorOfPoint(), unnamed_2.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Stub seam estimator which does nothing.
// NoSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:74
pub struct Detail_NoSeamFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_NoSeamFinder }

impl Drop for Detail_NoSeamFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_NoSeamFinder_delete(self.as_raw_mut_Detail_NoSeamFinder()) };
	}
}

unsafe impl Send for Detail_NoSeamFinder {}

impl crate::stitching::Detail_SeamFinderTraitConst for Detail_NoSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for Detail_NoSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_NoSeamFinder, crate::stitching::Detail_SeamFinderTraitConst, as_raw_Detail_SeamFinder, crate::stitching::Detail_SeamFinderTrait, as_raw_mut_Detail_SeamFinder }

impl crate::stitching::Detail_NoSeamFinderTraitConst for Detail_NoSeamFinder {
	#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_NoSeamFinderTrait for Detail_NoSeamFinder {
	#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_NoSeamFinder, crate::stitching::Detail_NoSeamFinderTraitConst, as_raw_Detail_NoSeamFinder, crate::stitching::Detail_NoSeamFinderTrait, as_raw_mut_Detail_NoSeamFinder }

impl Detail_NoSeamFinder {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_NoSeamFinder_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_NoSeamFinder, crate::stitching::Detail_SeamFinder, cv_detail_NoSeamFinder_to_Detail_SeamFinder }

impl std::fmt::Debug for Detail_NoSeamFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_NoSeamFinder")
			.finish()
	}
}

impl Default for Detail_NoSeamFinder {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_OrbFeaturesFinder]
// OrbFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:159
pub trait Detail_OrbFeaturesFinderTraitConst: crate::stitching::Detail_FeaturesFinderTraitConst {
	fn as_raw_Detail_OrbFeaturesFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_OrbFeaturesFinder]
pub trait Detail_OrbFeaturesFinderTrait: crate::stitching::Detail_FeaturesFinderTrait + crate::stitching::Detail_OrbFeaturesFinderTraitConst {
	fn as_raw_mut_Detail_OrbFeaturesFinder(&mut self) -> *mut c_void;

}

/// ORB features finder. :
/// ## See also
/// detail::FeaturesFinder, ORB
// OrbFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:159
pub struct Detail_OrbFeaturesFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_OrbFeaturesFinder }

impl Drop for Detail_OrbFeaturesFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_OrbFeaturesFinder_delete(self.as_raw_mut_Detail_OrbFeaturesFinder()) };
	}
}

unsafe impl Send for Detail_OrbFeaturesFinder {}

impl crate::stitching::Detail_FeaturesFinderTraitConst for Detail_OrbFeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for Detail_OrbFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_OrbFeaturesFinder, crate::stitching::Detail_FeaturesFinderTraitConst, as_raw_Detail_FeaturesFinder, crate::stitching::Detail_FeaturesFinderTrait, as_raw_mut_Detail_FeaturesFinder }

impl crate::stitching::Detail_OrbFeaturesFinderTraitConst for Detail_OrbFeaturesFinder {
	#[inline] fn as_raw_Detail_OrbFeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_OrbFeaturesFinderTrait for Detail_OrbFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_OrbFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_OrbFeaturesFinder, crate::stitching::Detail_OrbFeaturesFinderTraitConst, as_raw_Detail_OrbFeaturesFinder, crate::stitching::Detail_OrbFeaturesFinderTrait, as_raw_mut_Detail_OrbFeaturesFinder }

impl Detail_OrbFeaturesFinder {
	/// ## C++ default parameters
	/// * _grid_size: Size(3,1)
	/// * nfeatures: 1500
	/// * scale_factor: 1.3f
	/// * nlevels: 5
	// OrbFeaturesFinder(Size, int, float, int)(SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:162
	// ("cv::detail::OrbFeaturesFinder::OrbFeaturesFinder", vec![(pred!(mut, ["_grid_size", "nfeatures", "scaleFactor", "nlevels"], ["cv::Size", "int", "float", "int"]), _)]),
	#[inline]
	pub fn new(_grid_size: core::Size, nfeatures: i32, scale_factor: f32, nlevels: i32) -> Result<crate::stitching::Detail_OrbFeaturesFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_OrbFeaturesFinder_OrbFeaturesFinder_Size_int_float_int(&_grid_size, nfeatures, scale_factor, nlevels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_OrbFeaturesFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _grid_size: Size(3,1)
	/// * nfeatures: 1500
	/// * scale_factor: 1.3f
	/// * nlevels: 5
	// cv::detail::OrbFeaturesFinder::OrbFeaturesFinder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:162
	// ("cv::detail::OrbFeaturesFinder::OrbFeaturesFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_OrbFeaturesFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_OrbFeaturesFinder_OrbFeaturesFinder(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_OrbFeaturesFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_OrbFeaturesFinder, crate::stitching::Detail_FeaturesFinder, cv_detail_OrbFeaturesFinder_to_Detail_FeaturesFinder }

impl std::fmt::Debug for Detail_OrbFeaturesFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_OrbFeaturesFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_PairwiseSeamFinder]
// PairwiseSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:82
pub trait Detail_PairwiseSeamFinderTraitConst: crate::stitching::Detail_SeamFinderTraitConst {
	fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PairwiseSeamFinder]
pub trait Detail_PairwiseSeamFinderTrait: crate::stitching::Detail_PairwiseSeamFinderTraitConst + crate::stitching::Detail_SeamFinderTrait {
	fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void;

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:85
	// ("cv::detail::PairwiseSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PairwiseSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_PairwiseSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for all pairwise seam estimators.
// PairwiseSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:82
pub struct Detail_PairwiseSeamFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PairwiseSeamFinder }

impl Drop for Detail_PairwiseSeamFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PairwiseSeamFinder_delete(self.as_raw_mut_Detail_PairwiseSeamFinder()) };
	}
}

unsafe impl Send for Detail_PairwiseSeamFinder {}

impl crate::stitching::Detail_SeamFinderTraitConst for Detail_PairwiseSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for Detail_PairwiseSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PairwiseSeamFinder, crate::stitching::Detail_SeamFinderTraitConst, as_raw_Detail_SeamFinder, crate::stitching::Detail_SeamFinderTrait, as_raw_mut_Detail_SeamFinder }

impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for Detail_PairwiseSeamFinder {
	#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PairwiseSeamFinderTrait for Detail_PairwiseSeamFinder {
	#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PairwiseSeamFinder, crate::stitching::Detail_PairwiseSeamFinderTraitConst, as_raw_Detail_PairwiseSeamFinder, crate::stitching::Detail_PairwiseSeamFinderTrait, as_raw_mut_Detail_PairwiseSeamFinder }

impl Detail_PairwiseSeamFinder {
}

boxed_cast_descendant! { Detail_PairwiseSeamFinder, crate::stitching::Detail_VoronoiSeamFinder, cv_detail_PairwiseSeamFinder_to_Detail_VoronoiSeamFinder }

boxed_cast_base! { Detail_PairwiseSeamFinder, crate::stitching::Detail_SeamFinder, cv_detail_PairwiseSeamFinder_to_Detail_SeamFinder }

impl std::fmt::Debug for Detail_PairwiseSeamFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PairwiseSeamFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_PaniniPortraitProjector]
// PaniniPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:420
pub trait Detail_PaniniPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_PaniniPortraitProjector(&self) -> *const c_void;

	// cv::detail::PaniniPortraitProjector::a() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:422
	// ("cv::detail::PaniniPortraitProjector::a", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn a(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propA_const(self.as_raw_Detail_PaniniPortraitProjector()) };
		ret
	}

	// cv::detail::PaniniPortraitProjector::b() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:422
	// ("cv::detail::PaniniPortraitProjector::b", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn b(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propB_const(self.as_raw_Detail_PaniniPortraitProjector()) };
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_PaniniPortraitProjector]
pub trait Detail_PaniniPortraitProjectorTrait: crate::stitching::Detail_PaniniPortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_PaniniPortraitProjector(&mut self) -> *mut c_void;

	// cv::detail::PaniniPortraitProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:422
	// ("cv::detail::PaniniPortraitProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_a(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propA_const_float(self.as_raw_mut_Detail_PaniniPortraitProjector(), val) };
		ret
	}

	// cv::detail::PaniniPortraitProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:422
	// ("cv::detail::PaniniPortraitProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_b(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propB_const_float(self.as_raw_mut_Detail_PaniniPortraitProjector(), val) };
		ret
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:424
	// ("cv::detail::PaniniPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:425
	// ("cv::detail::PaniniPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PaniniPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:420
pub struct Detail_PaniniPortraitProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PaniniPortraitProjector }

impl Drop for Detail_PaniniPortraitProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PaniniPortraitProjector_delete(self.as_raw_mut_Detail_PaniniPortraitProjector()) };
	}
}

unsafe impl Send for Detail_PaniniPortraitProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_PaniniPortraitProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_PaniniPortraitProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniPortraitProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_PaniniPortraitProjectorTraitConst for Detail_PaniniPortraitProjector {
	#[inline] fn as_raw_Detail_PaniniPortraitProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PaniniPortraitProjectorTrait for Detail_PaniniPortraitProjector {
	#[inline] fn as_raw_mut_Detail_PaniniPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniPortraitProjector, crate::stitching::Detail_PaniniPortraitProjectorTraitConst, as_raw_Detail_PaniniPortraitProjector, crate::stitching::Detail_PaniniPortraitProjectorTrait, as_raw_mut_Detail_PaniniPortraitProjector }

impl Detail_PaniniPortraitProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_PaniniPortraitProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_PaniniPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PaniniPortraitProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_PaniniPortraitProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PaniniPortraitProjector")
			.field("a", &crate::stitching::Detail_PaniniPortraitProjectorTraitConst::a(self))
			.field("b", &crate::stitching::Detail_PaniniPortraitProjectorTraitConst::b(self))
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_PaniniPortraitProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_PaniniPortraitWarper]
// PaniniPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:429
pub trait Detail_PaniniPortraitWarperTraitConst {
	fn as_raw_Detail_PaniniPortraitWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PaniniPortraitWarper]
pub trait Detail_PaniniPortraitWarperTrait: crate::stitching::Detail_PaniniPortraitWarperTraitConst {
	fn as_raw_mut_Detail_PaniniPortraitWarper(&mut self) -> *mut c_void;

}

// PaniniPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:429
pub struct Detail_PaniniPortraitWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PaniniPortraitWarper }

impl Drop for Detail_PaniniPortraitWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PaniniPortraitWarper_delete(self.as_raw_mut_Detail_PaniniPortraitWarper()) };
	}
}

unsafe impl Send for Detail_PaniniPortraitWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_PaniniPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_PaniniPortraitWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniPortraitWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_PaniniPortraitWarperTraitConst for Detail_PaniniPortraitWarper {
	#[inline] fn as_raw_Detail_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PaniniPortraitWarperTrait for Detail_PaniniPortraitWarper {
	#[inline] fn as_raw_mut_Detail_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniPortraitWarper, crate::stitching::Detail_PaniniPortraitWarperTraitConst, as_raw_Detail_PaniniPortraitWarper, crate::stitching::Detail_PaniniPortraitWarperTrait, as_raw_mut_Detail_PaniniPortraitWarper }

impl Detail_PaniniPortraitWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// PaniniPortraitWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:432
	// ("cv::detail::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	#[inline]
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_PaniniPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(scale, a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PaniniPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::detail::PaniniPortraitWarper::PaniniPortraitWarper(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:432
	// ("cv::detail::PaniniPortraitWarper::PaniniPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new_def(scale: f32) -> Result<crate::stitching::Detail_PaniniPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PaniniPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_PaniniPortraitWarper, crate::stitching::Detail_RotationWarper, cv_detail_PaniniPortraitWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_PaniniPortraitWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PaniniPortraitWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_PaniniProjector]
// PaniniProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:399
pub trait Detail_PaniniProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_PaniniProjector(&self) -> *const c_void;

	// cv::detail::PaniniProjector::a() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:401
	// ("cv::detail::PaniniProjector::a", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn a(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_PaniniProjector_propA_const(self.as_raw_Detail_PaniniProjector()) };
		ret
	}

	// cv::detail::PaniniProjector::b() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:401
	// ("cv::detail::PaniniProjector::b", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn b(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_PaniniProjector_propB_const(self.as_raw_Detail_PaniniProjector()) };
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_PaniniProjector]
pub trait Detail_PaniniProjectorTrait: crate::stitching::Detail_PaniniProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_PaniniProjector(&mut self) -> *mut c_void;

	// cv::detail::PaniniProjector::setA(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:401
	// ("cv::detail::PaniniProjector::setA", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_a(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_PaniniProjector_propA_const_float(self.as_raw_mut_Detail_PaniniProjector(), val) };
		ret
	}

	// cv::detail::PaniniProjector::setB(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:401
	// ("cv::detail::PaniniProjector::setB", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_b(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_PaniniProjector_propB_const_float(self.as_raw_mut_Detail_PaniniProjector(), val) };
		ret
	}

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:403
	// ("cv::detail::PaniniProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:404
	// ("cv::detail::PaniniProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PaniniProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:399
pub struct Detail_PaniniProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PaniniProjector }

impl Drop for Detail_PaniniProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PaniniProjector_delete(self.as_raw_mut_Detail_PaniniProjector()) };
	}
}

unsafe impl Send for Detail_PaniniProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_PaniniProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_PaniniProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_PaniniProjectorTraitConst for Detail_PaniniProjector {
	#[inline] fn as_raw_Detail_PaniniProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PaniniProjectorTrait for Detail_PaniniProjector {
	#[inline] fn as_raw_mut_Detail_PaniniProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniProjector, crate::stitching::Detail_PaniniProjectorTraitConst, as_raw_Detail_PaniniProjector, crate::stitching::Detail_PaniniProjectorTrait, as_raw_mut_Detail_PaniniProjector }

impl Detail_PaniniProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_PaniniProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_PaniniProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PaniniProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_PaniniProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PaniniProjector")
			.field("a", &crate::stitching::Detail_PaniniProjectorTraitConst::a(self))
			.field("b", &crate::stitching::Detail_PaniniProjectorTraitConst::b(self))
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_PaniniProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_PaniniWarper]
// PaniniWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:408
pub trait Detail_PaniniWarperTraitConst {
	fn as_raw_Detail_PaniniWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PaniniWarper]
pub trait Detail_PaniniWarperTrait: crate::stitching::Detail_PaniniWarperTraitConst {
	fn as_raw_mut_Detail_PaniniWarper(&mut self) -> *mut c_void;

}

// PaniniWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:408
pub struct Detail_PaniniWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PaniniWarper }

impl Drop for Detail_PaniniWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PaniniWarper_delete(self.as_raw_mut_Detail_PaniniWarper()) };
	}
}

unsafe impl Send for Detail_PaniniWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_PaniniWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_PaniniWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_PaniniWarperTraitConst for Detail_PaniniWarper {
	#[inline] fn as_raw_Detail_PaniniWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PaniniWarperTrait for Detail_PaniniWarper {
	#[inline] fn as_raw_mut_Detail_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PaniniWarper, crate::stitching::Detail_PaniniWarperTraitConst, as_raw_Detail_PaniniWarper, crate::stitching::Detail_PaniniWarperTrait, as_raw_mut_Detail_PaniniWarper }

impl Detail_PaniniWarper {
	/// ## C++ default parameters
	/// * a: 1
	/// * b: 1
	// PaniniWarper(float, float, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:411
	// ("cv::detail::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["scale", "A", "B"], ["float", "float", "float"]), _)]),
	#[inline]
	pub fn new(scale: f32, a: f32, b: f32) -> Result<crate::stitching::Detail_PaniniWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniWarper_PaniniWarper_float_float_float(scale, a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PaniniWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * a: 1
	/// * b: 1
	// cv::detail::PaniniWarper::PaniniWarper(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:411
	// ("cv::detail::PaniniWarper::PaniniWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new_def(scale: f32) -> Result<crate::stitching::Detail_PaniniWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PaniniWarper_PaniniWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PaniniWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_PaniniWarper, crate::stitching::Detail_RotationWarper, cv_detail_PaniniWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_PaniniWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PaniniWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_PlanePortraitProjector]
// PlanePortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:649
pub trait Detail_PlanePortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_PlanePortraitProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PlanePortraitProjector]
pub trait Detail_PlanePortraitProjectorTrait: crate::stitching::Detail_PlanePortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_PlanePortraitProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:651
	// ("cv::detail::PlanePortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlanePortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlanePortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:652
	// ("cv::detail::PlanePortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlanePortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlanePortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PlanePortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:649
pub struct Detail_PlanePortraitProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PlanePortraitProjector }

impl Drop for Detail_PlanePortraitProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PlanePortraitProjector_delete(self.as_raw_mut_Detail_PlanePortraitProjector()) };
	}
}

unsafe impl Send for Detail_PlanePortraitProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_PlanePortraitProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_PlanePortraitProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlanePortraitProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_PlanePortraitProjectorTraitConst for Detail_PlanePortraitProjector {
	#[inline] fn as_raw_Detail_PlanePortraitProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PlanePortraitProjectorTrait for Detail_PlanePortraitProjector {
	#[inline] fn as_raw_mut_Detail_PlanePortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlanePortraitProjector, crate::stitching::Detail_PlanePortraitProjectorTraitConst, as_raw_Detail_PlanePortraitProjector, crate::stitching::Detail_PlanePortraitProjectorTrait, as_raw_mut_Detail_PlanePortraitProjector }

impl Detail_PlanePortraitProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_PlanePortraitProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_PlanePortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PlanePortraitProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_PlanePortraitProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PlanePortraitProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_PlanePortraitProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_PlanePortraitWarper]
// PlanePortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:656
pub trait Detail_PlanePortraitWarperTraitConst {
	fn as_raw_Detail_PlanePortraitWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PlanePortraitWarper]
pub trait Detail_PlanePortraitWarperTrait: crate::stitching::Detail_PlanePortraitWarperTraitConst {
	fn as_raw_mut_Detail_PlanePortraitWarper(&mut self) -> *mut c_void;

}

// PlanePortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:656
pub struct Detail_PlanePortraitWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PlanePortraitWarper }

impl Drop for Detail_PlanePortraitWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PlanePortraitWarper_delete(self.as_raw_mut_Detail_PlanePortraitWarper()) };
	}
}

unsafe impl Send for Detail_PlanePortraitWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_PlanePortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_PlanePortraitWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlanePortraitWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_PlanePortraitWarperTraitConst for Detail_PlanePortraitWarper {
	#[inline] fn as_raw_Detail_PlanePortraitWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PlanePortraitWarperTrait for Detail_PlanePortraitWarper {
	#[inline] fn as_raw_mut_Detail_PlanePortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlanePortraitWarper, crate::stitching::Detail_PlanePortraitWarperTraitConst, as_raw_Detail_PlanePortraitWarper, crate::stitching::Detail_PlanePortraitWarperTrait, as_raw_mut_Detail_PlanePortraitWarper }

impl Detail_PlanePortraitWarper {
	// PlanePortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:659
	// ("cv::detail::PlanePortraitWarper::PlanePortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_PlanePortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PlanePortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_PlanePortraitWarper, crate::stitching::Detail_RotationWarper, cv_detail_PlanePortraitWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_PlanePortraitWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PlanePortraitWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_PlaneProjector]
// PlaneProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:172
pub trait Detail_PlaneProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_PlaneProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PlaneProjector]
pub trait Detail_PlaneProjectorTrait: crate::stitching::Detail_PlaneProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
	fn as_raw_mut_Detail_PlaneProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:174
	// ("cv::detail::PlaneProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlaneProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:175
	// ("cv::detail::PlaneProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlaneProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PlaneProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:172
pub struct Detail_PlaneProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PlaneProjector }

impl Drop for Detail_PlaneProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PlaneProjector_delete(self.as_raw_mut_Detail_PlaneProjector()) };
	}
}

unsafe impl Send for Detail_PlaneProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_PlaneProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_PlaneProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlaneProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_PlaneProjectorTraitConst for Detail_PlaneProjector {
	#[inline] fn as_raw_Detail_PlaneProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PlaneProjectorTrait for Detail_PlaneProjector {
	#[inline] fn as_raw_mut_Detail_PlaneProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlaneProjector, crate::stitching::Detail_PlaneProjectorTraitConst, as_raw_Detail_PlaneProjector, crate::stitching::Detail_PlaneProjectorTrait, as_raw_mut_Detail_PlaneProjector }

impl Detail_PlaneProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_PlaneProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_PlaneProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PlaneProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_PlaneProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PlaneProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_PlaneProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_PlaneWarper]
// PlaneWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:180
pub trait Detail_PlaneWarperTraitConst {
	fn as_raw_Detail_PlaneWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PlaneWarper]
pub trait Detail_PlaneWarperTrait: crate::stitching::Detail_PlaneWarperTraitConst {
	fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void;

	// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:189
	// ("cv::detail::PlaneWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_point(&mut self, pt: core::Point2f, k: &impl ToInputArray, r: &impl ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warpPoint(const Point2f &, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:190
	// ("cv::detail::PlaneWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R", "T"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_point_1(&mut self, pt: core::Point2f, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// buildMaps(Size, InputArray, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:192
	// ("cv::detail::PlaneWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:193
	// ("cv::detail::PlaneWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps_1(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:195
	// ("cv::detail::PlaneWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:197
	// ("cv::detail::PlaneWarper::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp_1(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:200
	// ("cv::detail::PlaneWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_roi(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warpRoi(Size, InputArray, InputArray, InputArray)(SimpleClass, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:201
	// ("cv::detail::PlaneWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R", "T"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_roi_1(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Warper that maps an image onto the z = 1 plane.
// PlaneWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:180
pub struct Detail_PlaneWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PlaneWarper }

impl Drop for Detail_PlaneWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PlaneWarper_delete(self.as_raw_mut_Detail_PlaneWarper()) };
	}
}

unsafe impl Send for Detail_PlaneWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_PlaneWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_PlaneWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlaneWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_PlaneWarperTraitConst for Detail_PlaneWarper {
	#[inline] fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PlaneWarperTrait for Detail_PlaneWarper {
	#[inline] fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlaneWarper, crate::stitching::Detail_PlaneWarperTraitConst, as_raw_Detail_PlaneWarper, crate::stitching::Detail_PlaneWarperTrait, as_raw_mut_Detail_PlaneWarper }

impl Detail_PlaneWarper {
	/// Construct an instance of the plane warper class.
	///
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	///
	/// ## C++ default parameters
	/// * scale: 1.f
	// PlaneWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:187
	// ("cv::detail::PlaneWarper::PlaneWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_PlaneWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_PlaneWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PlaneWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Construct an instance of the plane warper class.
	///
	/// ## Parameters
	/// * scale: Projected image scale multiplier
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * scale: 1.f
	// cv::detail::PlaneWarper::PlaneWarper() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:187
	// ("cv::detail::PlaneWarper::PlaneWarper", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_PlaneWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarper_PlaneWarper(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PlaneWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Detail_PlaneWarper, crate::stitching::Detail_AffineWarper, cv_detail_PlaneWarper_to_Detail_AffineWarper }

boxed_cast_descendant! { Detail_PlaneWarper, crate::stitching::Detail_PlaneWarperGpu, cv_detail_PlaneWarper_to_Detail_PlaneWarperGpu }

boxed_cast_base! { Detail_PlaneWarper, crate::stitching::Detail_RotationWarper, cv_detail_PlaneWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_PlaneWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PlaneWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_PlaneWarperGpu]
// PlaneWarperGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:470
pub trait Detail_PlaneWarperGpuTraitConst: crate::stitching::Detail_PlaneWarperTraitConst {
	fn as_raw_Detail_PlaneWarperGpu(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_PlaneWarperGpu]
pub trait Detail_PlaneWarperGpuTrait: crate::stitching::Detail_PlaneWarperGpuTraitConst + crate::stitching::Detail_PlaneWarperTrait {
	fn as_raw_mut_Detail_PlaneWarperGpu(&mut self) -> *mut c_void;

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:480
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// buildMaps(Size, InputArray, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:488
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps_1(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:496
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:505
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp_1(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:517
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn build_maps_2(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// buildMaps(Size, InputArray, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:519
	// ("cv::detail::PlaneWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "T", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn build_maps_3(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:521
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn warp_2(&mut self, src: &impl core::GpuMatTraitConst, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::GpuMatTrait) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:524
	// ("cv::detail::PlaneWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "T", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn warp_3(&mut self, src: &impl core::GpuMatTraitConst, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::GpuMatTrait) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PlaneWarperGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:470
pub struct Detail_PlaneWarperGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_PlaneWarperGpu }

impl Drop for Detail_PlaneWarperGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_PlaneWarperGpu_delete(self.as_raw_mut_Detail_PlaneWarperGpu()) };
	}
}

unsafe impl Send for Detail_PlaneWarperGpu {}

impl crate::stitching::Detail_PlaneWarperTraitConst for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PlaneWarperTrait for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlaneWarperGpu, crate::stitching::Detail_PlaneWarperTraitConst, as_raw_Detail_PlaneWarper, crate::stitching::Detail_PlaneWarperTrait, as_raw_mut_Detail_PlaneWarper }

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlaneWarperGpu, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_PlaneWarperGpuTraitConst for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_Detail_PlaneWarperGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PlaneWarperGpuTrait for Detail_PlaneWarperGpu {
	#[inline] fn as_raw_mut_Detail_PlaneWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_PlaneWarperGpu, crate::stitching::Detail_PlaneWarperGpuTraitConst, as_raw_Detail_PlaneWarperGpu, crate::stitching::Detail_PlaneWarperGpuTrait, as_raw_mut_Detail_PlaneWarperGpu }

impl Detail_PlaneWarperGpu {
	/// ## C++ default parameters
	/// * scale: 1.f
	// PlaneWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:473
	// ("cv::detail::PlaneWarperGpu::PlaneWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_PlaneWarperGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PlaneWarperGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * scale: 1.f
	// cv::detail::PlaneWarperGpu::PlaneWarperGpu() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:473
	// ("cv::detail::PlaneWarperGpu::PlaneWarperGpu", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_PlaneWarperGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_PlaneWarperGpu_PlaneWarperGpu(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_PlaneWarperGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_PlaneWarperGpu, crate::stitching::Detail_PlaneWarper, cv_detail_PlaneWarperGpu_to_Detail_PlaneWarper }

boxed_cast_base! { Detail_PlaneWarperGpu, crate::stitching::Detail_RotationWarper, cv_detail_PlaneWarperGpu_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_PlaneWarperGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_PlaneWarperGpu")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_ProjectorBase]
// ProjectorBase /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:124
pub trait Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_ProjectorBase(&self) -> *const c_void;

	// cv::detail::ProjectorBase::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:130
	// ("cv::detail::ProjectorBase::scale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propScale_const(self.as_raw_Detail_ProjectorBase()) };
		ret
	}

	// cv::detail::ProjectorBase::k() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:131
	// ("cv::detail::ProjectorBase::k", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn k(&self) -> &[f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propK_const(self.as_raw_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_ref() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::rinv() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:132
	// ("cv::detail::ProjectorBase::rinv", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rinv(&self) -> &[f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propRinv_const(self.as_raw_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_ref() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::r_kinv() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:133
	// ("cv::detail::ProjectorBase::r_kinv", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn r_kinv(&self) -> &[f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propR_kinv_const(self.as_raw_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_ref() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::k_rinv() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:134
	// ("cv::detail::ProjectorBase::k_rinv", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn k_rinv(&self) -> &[f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propK_rinv_const(self.as_raw_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_ref() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::t() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:135
	// ("cv::detail::ProjectorBase::t", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn t(&self) -> &[f32; 3] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propT_const(self.as_raw_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_ref() }.expect("Function returned null pointer");
		ret
	}

}

/// Mutable methods for [crate::stitching::Detail_ProjectorBase]
pub trait Detail_ProjectorBaseTrait: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void;

	// cv::detail::ProjectorBase::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:130
	// ("cv::detail::ProjectorBase::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propScale_const_float(self.as_raw_mut_Detail_ProjectorBase(), val) };
		ret
	}

	// cv::detail::ProjectorBase::kMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:131
	// ("cv::detail::ProjectorBase::kMut", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn k_mut(&mut self) -> &mut [f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propK(self.as_raw_mut_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::rinvMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:132
	// ("cv::detail::ProjectorBase::rinvMut", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn rinv_mut(&mut self) -> &mut [f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propRinv(self.as_raw_mut_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::r_kinvMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:133
	// ("cv::detail::ProjectorBase::r_kinvMut", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn r_kinv_mut(&mut self) -> &mut [f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propR_kinv(self.as_raw_mut_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::k_rinvMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:134
	// ("cv::detail::ProjectorBase::k_rinvMut", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn k_rinv_mut(&mut self) -> &mut [f32; 9] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propK_rinv(self.as_raw_mut_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}

	// cv::detail::ProjectorBase::tMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:135
	// ("cv::detail::ProjectorBase::tMut", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn t_mut(&mut self) -> &mut [f32; 3] {
		let ret = unsafe { sys::cv_detail_ProjectorBase_propT(self.as_raw_mut_Detail_ProjectorBase()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}

	/// ## C++ default parameters
	/// * k: Mat::eye(3,3,CV_32F)
	/// * r: Mat::eye(3,3,CV_32F)
	/// * t: Mat::zeros(3,1,CV_32F)
	// setCameraParams(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:126
	// ("cv::detail::ProjectorBase::setCameraParams", vec![(pred!(mut, ["K", "R", "T"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_camera_params(&mut self, k: &impl ToInputArray, r: &impl ToInputArray, t: &impl ToInputArray) -> Result<()> {
		input_array_arg!(k);
		input_array_arg!(r);
		input_array_arg!(t);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_ProjectorBase_setCameraParams_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_ProjectorBase(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Detail_ProjectorBaseTrait::set_camera_params] function uses the following default values for its arguments:
	/// * k: Mat::eye(3,3,CV_32F)
	/// * r: Mat::eye(3,3,CV_32F)
	/// * t: Mat::zeros(3,1,CV_32F)
	// cv::detail::ProjectorBase::setCameraParams() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:126
	// ("cv::detail::ProjectorBase::setCameraParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn set_camera_params_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_ProjectorBase_setCameraParams(self.as_raw_mut_Detail_ProjectorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for warping logic implementation.
// ProjectorBase /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:124
pub struct Detail_ProjectorBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_ProjectorBase }

impl Drop for Detail_ProjectorBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_ProjectorBase_delete(self.as_raw_mut_Detail_ProjectorBase()) };
	}
}

unsafe impl Send for Detail_ProjectorBase {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_ProjectorBase {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_ProjectorBase {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl Detail_ProjectorBase {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_ProjectorBase_defaultNew_const()) }
	}

}

impl std::fmt::Debug for Detail_ProjectorBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_ProjectorBase")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_ProjectorBase {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_RotationWarper]
// RotationWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:59
pub trait Detail_RotationWarperTraitConst {
	fn as_raw_Detail_RotationWarper(&self) -> *const c_void;

	// getScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:118
	// ("cv::detail::RotationWarper::getScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_scale(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_RotationWarper_getScale_const(self.as_raw_Detail_RotationWarper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::stitching::Detail_RotationWarper]
pub trait Detail_RotationWarperTrait: crate::stitching::Detail_RotationWarperTraitConst {
	fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void;

	/// Projects the image point.
	///
	/// ## Parameters
	/// * pt: Source point
	/// * K: Camera intrinsic parameters
	/// * R: Camera rotation matrix
	/// ## Returns
	/// Projected point
	// warpPoint(const Point2f &, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:71
	// ("cv::detail::RotationWarper::warpPoint", vec![(pred!(mut, ["pt", "K", "R"], ["const cv::Point2f*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_point(&mut self, pt: core::Point2f, k: &impl ToInputArray, r: &impl ToInputArray) -> Result<core::Point2f> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_RotationWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:82
	// ("cv::detail::RotationWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:94
	// ("cv::detail::RotationWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_RotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	// warpBackward(InputArray, InputArray, InputArray, int, int, Size, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:107
	// ("cv::detail::RotationWarper::warpBackward", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst_size", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::Size", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp_backward(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst_size: core::Size, dst: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, &dst_size, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Parameters
	/// * src_size: Source image bounding box
	/// * K: Camera intrinsic parameters
	/// * R: Camera rotation matrix
	/// ## Returns
	/// Projected image minimum bounding box
	// warpRoi(Size, InputArray, InputArray)(SimpleClass, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:116
	// ("cv::detail::RotationWarper::warpRoi", vec![(pred!(mut, ["src_size", "K", "R"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn warp_roi(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_RotationWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:119
	// ("cv::detail::RotationWarper::setScale", vec![(pred!(mut, ["unnamed"], ["float"]), _)]),
	#[inline]
	fn set_scale(&mut self, unnamed: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_RotationWarper_setScale_float(self.as_raw_mut_Detail_RotationWarper(), unnamed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Rotation-only model image warper interface.
// RotationWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:59
pub struct Detail_RotationWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_RotationWarper }

impl Drop for Detail_RotationWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_RotationWarper_delete(self.as_raw_mut_Detail_RotationWarper()) };
	}
}

unsafe impl Send for Detail_RotationWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_RotationWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_RotationWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_RotationWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl Detail_RotationWarper {
}

impl std::fmt::Debug for Detail_RotationWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_RotationWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_SeamFinder]
// SeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:58
pub trait Detail_SeamFinderTraitConst {
	fn as_raw_Detail_SeamFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SeamFinder]
pub trait Detail_SeamFinderTrait: crate::stitching::Detail_SeamFinderTraitConst {
	fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void;

	/// Estimates seams.
	///
	/// ## Parameters
	/// * src: Source images
	/// * corners: Source image top-left corners
	/// * masks: Source image masks to update
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:68
	// ("cv::detail::SeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_SeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for a seam estimator.
// SeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:58
pub struct Detail_SeamFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SeamFinder }

impl Drop for Detail_SeamFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SeamFinder_delete(self.as_raw_mut_Detail_SeamFinder()) };
	}
}

unsafe impl Send for Detail_SeamFinder {}

impl crate::stitching::Detail_SeamFinderTraitConst for Detail_SeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for Detail_SeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SeamFinder, crate::stitching::Detail_SeamFinderTraitConst, as_raw_Detail_SeamFinder, crate::stitching::Detail_SeamFinderTrait, as_raw_mut_Detail_SeamFinder }

impl Detail_SeamFinder {
}

boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_DpSeamFinder, cv_detail_SeamFinder_to_Detail_DpSeamFinder }

boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_GraphCutSeamFinder, cv_detail_SeamFinder_to_Detail_GraphCutSeamFinder }

boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_NoSeamFinder, cv_detail_SeamFinder_to_Detail_NoSeamFinder }

boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_PairwiseSeamFinder, cv_detail_SeamFinder_to_Detail_PairwiseSeamFinder }

boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_VoronoiSeamFinder, cv_detail_SeamFinder_to_Detail_VoronoiSeamFinder }

impl std::fmt::Debug for Detail_SeamFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SeamFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_SiftFeaturesFinder]
// SiftFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:145
pub trait Detail_SiftFeaturesFinderTraitConst: crate::stitching::Detail_FeaturesFinderTraitConst {
	fn as_raw_Detail_SiftFeaturesFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SiftFeaturesFinder]
pub trait Detail_SiftFeaturesFinderTrait: crate::stitching::Detail_FeaturesFinderTrait + crate::stitching::Detail_SiftFeaturesFinderTraitConst {
	fn as_raw_mut_Detail_SiftFeaturesFinder(&mut self) -> *mut c_void;

}

/// SIFT features finder.
/// ## See also
/// detail::FeaturesFinder, SIFT
// SiftFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:145
pub struct Detail_SiftFeaturesFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SiftFeaturesFinder }

impl Drop for Detail_SiftFeaturesFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SiftFeaturesFinder_delete(self.as_raw_mut_Detail_SiftFeaturesFinder()) };
	}
}

unsafe impl Send for Detail_SiftFeaturesFinder {}

impl crate::stitching::Detail_FeaturesFinderTraitConst for Detail_SiftFeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for Detail_SiftFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SiftFeaturesFinder, crate::stitching::Detail_FeaturesFinderTraitConst, as_raw_Detail_FeaturesFinder, crate::stitching::Detail_FeaturesFinderTrait, as_raw_mut_Detail_FeaturesFinder }

impl crate::stitching::Detail_SiftFeaturesFinderTraitConst for Detail_SiftFeaturesFinder {
	#[inline] fn as_raw_Detail_SiftFeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SiftFeaturesFinderTrait for Detail_SiftFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_SiftFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SiftFeaturesFinder, crate::stitching::Detail_SiftFeaturesFinderTraitConst, as_raw_Detail_SiftFeaturesFinder, crate::stitching::Detail_SiftFeaturesFinderTrait, as_raw_mut_Detail_SiftFeaturesFinder }

impl Detail_SiftFeaturesFinder {
	// SiftFeaturesFinder()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:148
	// ("cv::detail::SiftFeaturesFinder::SiftFeaturesFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::stitching::Detail_SiftFeaturesFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SiftFeaturesFinder_SiftFeaturesFinder(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SiftFeaturesFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_SiftFeaturesFinder, crate::stitching::Detail_FeaturesFinder, cv_detail_SiftFeaturesFinder_to_Detail_FeaturesFinder }

impl std::fmt::Debug for Detail_SiftFeaturesFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SiftFeaturesFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_SphericalPortraitProjector]
// SphericalPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:612
pub trait Detail_SphericalPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_SphericalPortraitProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SphericalPortraitProjector]
pub trait Detail_SphericalPortraitProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_SphericalPortraitProjectorTraitConst {
	fn as_raw_mut_Detail_SphericalPortraitProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:614
	// ("cv::detail::SphericalPortraitProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:615
	// ("cv::detail::SphericalPortraitProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// SphericalPortraitProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:612
pub struct Detail_SphericalPortraitProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SphericalPortraitProjector }

impl Drop for Detail_SphericalPortraitProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SphericalPortraitProjector_delete(self.as_raw_mut_Detail_SphericalPortraitProjector()) };
	}
}

unsafe impl Send for Detail_SphericalPortraitProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_SphericalPortraitProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_SphericalPortraitProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalPortraitProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_SphericalPortraitProjectorTraitConst for Detail_SphericalPortraitProjector {
	#[inline] fn as_raw_Detail_SphericalPortraitProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SphericalPortraitProjectorTrait for Detail_SphericalPortraitProjector {
	#[inline] fn as_raw_mut_Detail_SphericalPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalPortraitProjector, crate::stitching::Detail_SphericalPortraitProjectorTraitConst, as_raw_Detail_SphericalPortraitProjector, crate::stitching::Detail_SphericalPortraitProjectorTrait, as_raw_mut_Detail_SphericalPortraitProjector }

impl Detail_SphericalPortraitProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_SphericalPortraitProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_SphericalPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_SphericalPortraitProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_SphericalPortraitProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SphericalPortraitProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_SphericalPortraitProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_SphericalPortraitWarper]
// SphericalPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:621
pub trait Detail_SphericalPortraitWarperTraitConst {
	fn as_raw_Detail_SphericalPortraitWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SphericalPortraitWarper]
pub trait Detail_SphericalPortraitWarperTrait: crate::stitching::Detail_SphericalPortraitWarperTraitConst {
	fn as_raw_mut_Detail_SphericalPortraitWarper(&mut self) -> *mut c_void;

}

// SphericalPortraitWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:621
pub struct Detail_SphericalPortraitWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SphericalPortraitWarper }

impl Drop for Detail_SphericalPortraitWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SphericalPortraitWarper_delete(self.as_raw_mut_Detail_SphericalPortraitWarper()) };
	}
}

unsafe impl Send for Detail_SphericalPortraitWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_SphericalPortraitWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_SphericalPortraitWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalPortraitWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_SphericalPortraitWarperTraitConst for Detail_SphericalPortraitWarper {
	#[inline] fn as_raw_Detail_SphericalPortraitWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SphericalPortraitWarperTrait for Detail_SphericalPortraitWarper {
	#[inline] fn as_raw_mut_Detail_SphericalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalPortraitWarper, crate::stitching::Detail_SphericalPortraitWarperTraitConst, as_raw_Detail_SphericalPortraitWarper, crate::stitching::Detail_SphericalPortraitWarperTrait, as_raw_mut_Detail_SphericalPortraitWarper }

impl Detail_SphericalPortraitWarper {
	// SphericalPortraitWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:624
	// ("cv::detail::SphericalPortraitWarper::SphericalPortraitWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_SphericalPortraitWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SphericalPortraitWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_SphericalPortraitWarper, crate::stitching::Detail_RotationWarper, cv_detail_SphericalPortraitWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_SphericalPortraitWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SphericalPortraitWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_SphericalProjector]
// SphericalProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:271
pub trait Detail_SphericalProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_SphericalProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SphericalProjector]
pub trait Detail_SphericalProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_SphericalProjectorTraitConst {
	fn as_raw_mut_Detail_SphericalProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:273
	// ("cv::detail::SphericalProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:274
	// ("cv::detail::SphericalProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// SphericalProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:271
pub struct Detail_SphericalProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SphericalProjector }

impl Drop for Detail_SphericalProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SphericalProjector_delete(self.as_raw_mut_Detail_SphericalProjector()) };
	}
}

unsafe impl Send for Detail_SphericalProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_SphericalProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_SphericalProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_SphericalProjectorTraitConst for Detail_SphericalProjector {
	#[inline] fn as_raw_Detail_SphericalProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SphericalProjectorTrait for Detail_SphericalProjector {
	#[inline] fn as_raw_mut_Detail_SphericalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalProjector, crate::stitching::Detail_SphericalProjectorTraitConst, as_raw_Detail_SphericalProjector, crate::stitching::Detail_SphericalProjectorTrait, as_raw_mut_Detail_SphericalProjector }

impl Detail_SphericalProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_SphericalProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_SphericalProjector, crate::stitching::Detail_ProjectorBase, cv_detail_SphericalProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_SphericalProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SphericalProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_SphericalProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_SphericalWarper]
// SphericalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:284
pub trait Detail_SphericalWarperTraitConst {
	fn as_raw_Detail_SphericalWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SphericalWarper]
pub trait Detail_SphericalWarperTrait: crate::stitching::Detail_SphericalWarperTraitConst {
	fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void;

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:294
	// ("cv::detail::SphericalWarper::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarper(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:295
	// ("cv::detail::SphericalWarper::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Warper that maps an image onto the unit sphere located at the origin.
///
/// Projects image onto unit sphere with origin at (0, 0, 0) and radius scale, measured in pixels.
/// A 360 panorama would therefore have a resulting width of 2 * scale * PI pixels.
/// Poles are located at (0, -1, 0) and (0, 1, 0) points.
// SphericalWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:284
pub struct Detail_SphericalWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SphericalWarper }

impl Drop for Detail_SphericalWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SphericalWarper_delete(self.as_raw_mut_Detail_SphericalWarper()) };
	}
}

unsafe impl Send for Detail_SphericalWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_SphericalWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_SphericalWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_SphericalWarperTraitConst for Detail_SphericalWarper {
	#[inline] fn as_raw_Detail_SphericalWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SphericalWarperTrait for Detail_SphericalWarper {
	#[inline] fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalWarper, crate::stitching::Detail_SphericalWarperTraitConst, as_raw_Detail_SphericalWarper, crate::stitching::Detail_SphericalWarperTrait, as_raw_mut_Detail_SphericalWarper }

impl Detail_SphericalWarper {
	/// Construct an instance of the spherical warper class.
	///
	/// ## Parameters
	/// * scale: Radius of the projected sphere, in pixels. An image spanning the
	///              whole sphere will have a width of 2 * scale * PI pixels.
	// SphericalWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:292
	// ("cv::detail::SphericalWarper::SphericalWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_SphericalWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarper_SphericalWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SphericalWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Detail_SphericalWarper, crate::stitching::Detail_SphericalWarperGpu, cv_detail_SphericalWarper_to_Detail_SphericalWarperGpu }

boxed_cast_base! { Detail_SphericalWarper, crate::stitching::Detail_RotationWarper, cv_detail_SphericalWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_SphericalWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SphericalWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_SphericalWarperGpu]
// SphericalWarperGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:532
pub trait Detail_SphericalWarperGpuTraitConst: crate::stitching::Detail_SphericalWarperTraitConst {
	fn as_raw_Detail_SphericalWarperGpu(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SphericalWarperGpu]
pub trait Detail_SphericalWarperGpuTrait: crate::stitching::Detail_SphericalWarperGpuTraitConst + crate::stitching::Detail_SphericalWarperTrait {
	fn as_raw_mut_Detail_SphericalWarperGpu(&mut self) -> *mut c_void;

	// buildMaps(Size, InputArray, InputArray, OutputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:542
	// ("cv::detail::SphericalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn build_maps(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl ToOutputArray, ymap: &mut impl ToOutputArray) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(xmap);
		output_array_arg!(ymap);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(InputArray, InputArray, InputArray, int, int, OutputArray)(InputArray, InputArray, InputArray, Primitive, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:550
	// ("cv::detail::SphericalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn warp(&mut self, src: &impl ToInputArray, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl ToOutputArray) -> Result<core::Point> {
		input_array_arg!(src);
		input_array_arg!(k);
		input_array_arg!(r);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarperGpu(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// buildMaps(Size, InputArray, InputArray, cuda::GpuMat &, cuda::GpuMat &)(SimpleClass, InputArray, InputArray, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:562
	// ("cv::detail::SphericalWarperGpu::buildMaps", vec![(pred!(mut, ["src_size", "K", "R", "xmap", "ymap"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn build_maps_1(&mut self, src_size: core::Size, k: &impl ToInputArray, r: &impl ToInputArray, xmap: &mut impl core::GpuMatTrait, ymap: &mut impl core::GpuMatTrait) -> Result<core::Rect> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_SphericalWarperGpu(), &src_size, k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// warp(const cuda::GpuMat &, InputArray, InputArray, int, int, cuda::GpuMat &)(TraitClass, InputArray, InputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:564
	// ("cv::detail::SphericalWarperGpu::warp", vec![(pred!(mut, ["src", "K", "R", "interp_mode", "border_mode", "dst"], ["const cv::cuda::GpuMat*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn warp_1(&mut self, src: &impl core::GpuMatTraitConst, k: &impl ToInputArray, r: &impl ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::GpuMatTrait) -> Result<core::Point> {
		input_array_arg!(k);
		input_array_arg!(r);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_SphericalWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// SphericalWarperGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:532
pub struct Detail_SphericalWarperGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SphericalWarperGpu }

impl Drop for Detail_SphericalWarperGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SphericalWarperGpu_delete(self.as_raw_mut_Detail_SphericalWarperGpu()) };
	}
}

unsafe impl Send for Detail_SphericalWarperGpu {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalWarperGpu, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_SphericalWarperTraitConst for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_Detail_SphericalWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SphericalWarperTrait for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalWarperGpu, crate::stitching::Detail_SphericalWarperTraitConst, as_raw_Detail_SphericalWarper, crate::stitching::Detail_SphericalWarperTrait, as_raw_mut_Detail_SphericalWarper }

impl crate::stitching::Detail_SphericalWarperGpuTraitConst for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_Detail_SphericalWarperGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SphericalWarperGpuTrait for Detail_SphericalWarperGpu {
	#[inline] fn as_raw_mut_Detail_SphericalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SphericalWarperGpu, crate::stitching::Detail_SphericalWarperGpuTraitConst, as_raw_Detail_SphericalWarperGpu, crate::stitching::Detail_SphericalWarperGpuTrait, as_raw_mut_Detail_SphericalWarperGpu }

impl Detail_SphericalWarperGpu {
	// SphericalWarperGpu(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:535
	// ("cv::detail::SphericalWarperGpu::SphericalWarperGpu", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_SphericalWarperGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SphericalWarperGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_SphericalWarperGpu, crate::stitching::Detail_RotationWarper, cv_detail_SphericalWarperGpu_to_Detail_RotationWarper }

boxed_cast_base! { Detail_SphericalWarperGpu, crate::stitching::Detail_SphericalWarper, cv_detail_SphericalWarperGpu_to_Detail_SphericalWarper }

impl std::fmt::Debug for Detail_SphericalWarperGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SphericalWarperGpu")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_StereographicProjector]
// StereographicProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:343
pub trait Detail_StereographicProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_StereographicProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_StereographicProjector]
pub trait Detail_StereographicProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_StereographicProjectorTraitConst {
	fn as_raw_mut_Detail_StereographicProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:345
	// ("cv::detail::StereographicProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_StereographicProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:346
	// ("cv::detail::StereographicProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_StereographicProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// StereographicProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:343
pub struct Detail_StereographicProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_StereographicProjector }

impl Drop for Detail_StereographicProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_StereographicProjector_delete(self.as_raw_mut_Detail_StereographicProjector()) };
	}
}

unsafe impl Send for Detail_StereographicProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_StereographicProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_StereographicProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_StereographicProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_StereographicProjectorTraitConst for Detail_StereographicProjector {
	#[inline] fn as_raw_Detail_StereographicProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_StereographicProjectorTrait for Detail_StereographicProjector {
	#[inline] fn as_raw_mut_Detail_StereographicProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_StereographicProjector, crate::stitching::Detail_StereographicProjectorTraitConst, as_raw_Detail_StereographicProjector, crate::stitching::Detail_StereographicProjectorTrait, as_raw_mut_Detail_StereographicProjector }

impl Detail_StereographicProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_StereographicProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_StereographicProjector, crate::stitching::Detail_ProjectorBase, cv_detail_StereographicProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_StereographicProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_StereographicProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_StereographicProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_StereographicWarper]
// StereographicWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:350
pub trait Detail_StereographicWarperTraitConst {
	fn as_raw_Detail_StereographicWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_StereographicWarper]
pub trait Detail_StereographicWarperTrait: crate::stitching::Detail_StereographicWarperTraitConst {
	fn as_raw_mut_Detail_StereographicWarper(&mut self) -> *mut c_void;

}

// StereographicWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:350
pub struct Detail_StereographicWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_StereographicWarper }

impl Drop for Detail_StereographicWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_StereographicWarper_delete(self.as_raw_mut_Detail_StereographicWarper()) };
	}
}

unsafe impl Send for Detail_StereographicWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_StereographicWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_StereographicWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_StereographicWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_StereographicWarperTraitConst for Detail_StereographicWarper {
	#[inline] fn as_raw_Detail_StereographicWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_StereographicWarperTrait for Detail_StereographicWarper {
	#[inline] fn as_raw_mut_Detail_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_StereographicWarper, crate::stitching::Detail_StereographicWarperTraitConst, as_raw_Detail_StereographicWarper, crate::stitching::Detail_StereographicWarperTrait, as_raw_mut_Detail_StereographicWarper }

impl Detail_StereographicWarper {
	// StereographicWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:353
	// ("cv::detail::StereographicWarper::StereographicWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_StereographicWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_StereographicWarper_StereographicWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_StereographicWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_StereographicWarper, crate::stitching::Detail_RotationWarper, cv_detail_StereographicWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_StereographicWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_StereographicWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_SurfFeaturesFinder]
// SurfFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:126
pub trait Detail_SurfFeaturesFinderTraitConst: crate::stitching::Detail_FeaturesFinderTraitConst {
	fn as_raw_Detail_SurfFeaturesFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SurfFeaturesFinder]
pub trait Detail_SurfFeaturesFinderTrait: crate::stitching::Detail_FeaturesFinderTrait + crate::stitching::Detail_SurfFeaturesFinderTraitConst {
	fn as_raw_mut_Detail_SurfFeaturesFinder(&mut self) -> *mut c_void;

}

/// SURF features finder.
/// ## See also
/// detail::FeaturesFinder, SURF
// SurfFeaturesFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:126
pub struct Detail_SurfFeaturesFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SurfFeaturesFinder }

impl Drop for Detail_SurfFeaturesFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SurfFeaturesFinder_delete(self.as_raw_mut_Detail_SurfFeaturesFinder()) };
	}
}

unsafe impl Send for Detail_SurfFeaturesFinder {}

impl crate::stitching::Detail_FeaturesFinderTraitConst for Detail_SurfFeaturesFinder {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for Detail_SurfFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SurfFeaturesFinder, crate::stitching::Detail_FeaturesFinderTraitConst, as_raw_Detail_FeaturesFinder, crate::stitching::Detail_FeaturesFinderTrait, as_raw_mut_Detail_FeaturesFinder }

impl crate::stitching::Detail_SurfFeaturesFinderTraitConst for Detail_SurfFeaturesFinder {
	#[inline] fn as_raw_Detail_SurfFeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SurfFeaturesFinderTrait for Detail_SurfFeaturesFinder {
	#[inline] fn as_raw_mut_Detail_SurfFeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SurfFeaturesFinder, crate::stitching::Detail_SurfFeaturesFinderTraitConst, as_raw_Detail_SurfFeaturesFinder, crate::stitching::Detail_SurfFeaturesFinderTrait, as_raw_mut_Detail_SurfFeaturesFinder }

impl Detail_SurfFeaturesFinder {
	/// ## C++ default parameters
	/// * hess_thresh: 300.
	/// * num_octaves: 3
	/// * num_layers: 4
	/// * num_octaves_descr: /*4*/3
	/// * num_layers_descr: /*2*/4
	// SurfFeaturesFinder(double, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:129
	// ("cv::detail::SurfFeaturesFinder::SurfFeaturesFinder", vec![(pred!(mut, ["hess_thresh", "num_octaves", "num_layers", "num_octaves_descr", "num_layers_descr"], ["double", "int", "int", "int", "int"]), _)]),
	#[inline]
	pub fn new(hess_thresh: f64, num_octaves: i32, num_layers: i32, num_octaves_descr: i32, num_layers_descr: i32) -> Result<crate::stitching::Detail_SurfFeaturesFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SurfFeaturesFinder_SurfFeaturesFinder_double_int_int_int_int(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SurfFeaturesFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * hess_thresh: 300.
	/// * num_octaves: 3
	/// * num_layers: 4
	/// * num_octaves_descr: /*4*/3
	/// * num_layers_descr: /*2*/4
	// cv::detail::SurfFeaturesFinder::SurfFeaturesFinder() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:129
	// ("cv::detail::SurfFeaturesFinder::SurfFeaturesFinder", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_SurfFeaturesFinder> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SurfFeaturesFinder_SurfFeaturesFinder(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SurfFeaturesFinder::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_SurfFeaturesFinder, crate::stitching::Detail_FeaturesFinder, cv_detail_SurfFeaturesFinder_to_Detail_FeaturesFinder }

impl std::fmt::Debug for Detail_SurfFeaturesFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SurfFeaturesFinder")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_SurfFeaturesFinderGpu]
// SurfFeaturesFinderGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:193
pub trait Detail_SurfFeaturesFinderGpuTraitConst: crate::stitching::Detail_FeaturesFinderTraitConst {
	fn as_raw_Detail_SurfFeaturesFinderGpu(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_SurfFeaturesFinderGpu]
pub trait Detail_SurfFeaturesFinderGpuTrait: crate::stitching::Detail_FeaturesFinderTrait + crate::stitching::Detail_SurfFeaturesFinderGpuTraitConst {
	fn as_raw_mut_Detail_SurfFeaturesFinderGpu(&mut self) -> *mut c_void;

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:199
	// ("cv::detail::SurfFeaturesFinderGpu::collectGarbage", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SurfFeaturesFinderGpu_collectGarbage(self.as_raw_mut_Detail_SurfFeaturesFinderGpu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// SurfFeaturesFinderGpu /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:193
pub struct Detail_SurfFeaturesFinderGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_SurfFeaturesFinderGpu }

impl Drop for Detail_SurfFeaturesFinderGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_SurfFeaturesFinderGpu_delete(self.as_raw_mut_Detail_SurfFeaturesFinderGpu()) };
	}
}

unsafe impl Send for Detail_SurfFeaturesFinderGpu {}

impl crate::stitching::Detail_FeaturesFinderTraitConst for Detail_SurfFeaturesFinderGpu {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for Detail_SurfFeaturesFinderGpu {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SurfFeaturesFinderGpu, crate::stitching::Detail_FeaturesFinderTraitConst, as_raw_Detail_FeaturesFinder, crate::stitching::Detail_FeaturesFinderTrait, as_raw_mut_Detail_FeaturesFinder }

impl crate::stitching::Detail_SurfFeaturesFinderGpuTraitConst for Detail_SurfFeaturesFinderGpu {
	#[inline] fn as_raw_Detail_SurfFeaturesFinderGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SurfFeaturesFinderGpuTrait for Detail_SurfFeaturesFinderGpu {
	#[inline] fn as_raw_mut_Detail_SurfFeaturesFinderGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_SurfFeaturesFinderGpu, crate::stitching::Detail_SurfFeaturesFinderGpuTraitConst, as_raw_Detail_SurfFeaturesFinderGpu, crate::stitching::Detail_SurfFeaturesFinderGpuTrait, as_raw_mut_Detail_SurfFeaturesFinderGpu }

impl Detail_SurfFeaturesFinderGpu {
	/// ## C++ default parameters
	/// * hess_thresh: 300.
	/// * num_octaves: 3
	/// * num_layers: 4
	/// * num_octaves_descr: 4
	/// * num_layers_descr: 2
	// SurfFeaturesFinderGpu(double, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:196
	// ("cv::detail::SurfFeaturesFinderGpu::SurfFeaturesFinderGpu", vec![(pred!(mut, ["hess_thresh", "num_octaves", "num_layers", "num_octaves_descr", "num_layers_descr"], ["double", "int", "int", "int", "int"]), _)]),
	#[inline]
	pub fn new(hess_thresh: f64, num_octaves: i32, num_layers: i32, num_octaves_descr: i32, num_layers_descr: i32) -> Result<crate::stitching::Detail_SurfFeaturesFinderGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SurfFeaturesFinderGpu_SurfFeaturesFinderGpu_double_int_int_int_int(hess_thresh, num_octaves, num_layers, num_octaves_descr, num_layers_descr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SurfFeaturesFinderGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * hess_thresh: 300.
	/// * num_octaves: 3
	/// * num_layers: 4
	/// * num_octaves_descr: 4
	/// * num_layers_descr: 2
	// cv::detail::SurfFeaturesFinderGpu::SurfFeaturesFinderGpu() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/matchers.hpp:196
	// ("cv::detail::SurfFeaturesFinderGpu::SurfFeaturesFinderGpu", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::stitching::Detail_SurfFeaturesFinderGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_SurfFeaturesFinderGpu_SurfFeaturesFinderGpu(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_SurfFeaturesFinderGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_SurfFeaturesFinderGpu, crate::stitching::Detail_FeaturesFinder, cv_detail_SurfFeaturesFinderGpu_to_Detail_FeaturesFinder }

impl std::fmt::Debug for Detail_SurfFeaturesFinderGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_SurfFeaturesFinderGpu")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_TransverseMercatorProjector]
// TransverseMercatorProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:456
pub trait Detail_TransverseMercatorProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
	fn as_raw_Detail_TransverseMercatorProjector(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_TransverseMercatorProjector]
pub trait Detail_TransverseMercatorProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_TransverseMercatorProjectorTraitConst {
	fn as_raw_mut_Detail_TransverseMercatorProjector(&mut self) -> *mut c_void;

	// mapForward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:458
	// ("cv::detail::TransverseMercatorProjector::mapForward", vec![(pred!(mut, ["x", "y", "u", "v"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_TransverseMercatorProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// mapBackward(float, float, float &, float &)(Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:459
	// ("cv::detail::TransverseMercatorProjector::mapBackward", vec![(pred!(mut, ["u", "v", "x", "y"], ["float", "float", "float*", "float*"]), _)]),
	#[inline]
	fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_TransverseMercatorProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// TransverseMercatorProjector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:456
pub struct Detail_TransverseMercatorProjector {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_TransverseMercatorProjector }

impl Drop for Detail_TransverseMercatorProjector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_TransverseMercatorProjector_delete(self.as_raw_mut_Detail_TransverseMercatorProjector()) };
	}
}

unsafe impl Send for Detail_TransverseMercatorProjector {}

impl crate::stitching::Detail_ProjectorBaseTraitConst for Detail_TransverseMercatorProjector {
	#[inline] fn as_raw_Detail_ProjectorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_ProjectorBaseTrait for Detail_TransverseMercatorProjector {
	#[inline] fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_TransverseMercatorProjector, crate::stitching::Detail_ProjectorBaseTraitConst, as_raw_Detail_ProjectorBase, crate::stitching::Detail_ProjectorBaseTrait, as_raw_mut_Detail_ProjectorBase }

impl crate::stitching::Detail_TransverseMercatorProjectorTraitConst for Detail_TransverseMercatorProjector {
	#[inline] fn as_raw_Detail_TransverseMercatorProjector(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_TransverseMercatorProjectorTrait for Detail_TransverseMercatorProjector {
	#[inline] fn as_raw_mut_Detail_TransverseMercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_TransverseMercatorProjector, crate::stitching::Detail_TransverseMercatorProjectorTraitConst, as_raw_Detail_TransverseMercatorProjector, crate::stitching::Detail_TransverseMercatorProjectorTrait, as_raw_mut_Detail_TransverseMercatorProjector }

impl Detail_TransverseMercatorProjector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_TransverseMercatorProjector_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_TransverseMercatorProjector, crate::stitching::Detail_ProjectorBase, cv_detail_TransverseMercatorProjector_to_Detail_ProjectorBase }

impl std::fmt::Debug for Detail_TransverseMercatorProjector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_TransverseMercatorProjector")
			.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
			.field("k", &crate::stitching::Detail_ProjectorBaseTraitConst::k(self))
			.field("rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::rinv(self))
			.field("r_kinv", &crate::stitching::Detail_ProjectorBaseTraitConst::r_kinv(self))
			.field("k_rinv", &crate::stitching::Detail_ProjectorBaseTraitConst::k_rinv(self))
			.field("t", &crate::stitching::Detail_ProjectorBaseTraitConst::t(self))
			.finish()
	}
}

impl Default for Detail_TransverseMercatorProjector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::stitching::Detail_TransverseMercatorWarper]
// TransverseMercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:463
pub trait Detail_TransverseMercatorWarperTraitConst {
	fn as_raw_Detail_TransverseMercatorWarper(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_TransverseMercatorWarper]
pub trait Detail_TransverseMercatorWarperTrait: crate::stitching::Detail_TransverseMercatorWarperTraitConst {
	fn as_raw_mut_Detail_TransverseMercatorWarper(&mut self) -> *mut c_void;

}

// TransverseMercatorWarper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:463
pub struct Detail_TransverseMercatorWarper {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_TransverseMercatorWarper }

impl Drop for Detail_TransverseMercatorWarper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_TransverseMercatorWarper_delete(self.as_raw_mut_Detail_TransverseMercatorWarper()) };
	}
}

unsafe impl Send for Detail_TransverseMercatorWarper {}

impl crate::stitching::Detail_RotationWarperTraitConst for Detail_TransverseMercatorWarper {
	#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_RotationWarperTrait for Detail_TransverseMercatorWarper {
	#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_TransverseMercatorWarper, crate::stitching::Detail_RotationWarperTraitConst, as_raw_Detail_RotationWarper, crate::stitching::Detail_RotationWarperTrait, as_raw_mut_Detail_RotationWarper }

impl crate::stitching::Detail_TransverseMercatorWarperTraitConst for Detail_TransverseMercatorWarper {
	#[inline] fn as_raw_Detail_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_TransverseMercatorWarperTrait for Detail_TransverseMercatorWarper {
	#[inline] fn as_raw_mut_Detail_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_TransverseMercatorWarper, crate::stitching::Detail_TransverseMercatorWarperTraitConst, as_raw_Detail_TransverseMercatorWarper, crate::stitching::Detail_TransverseMercatorWarperTrait, as_raw_mut_Detail_TransverseMercatorWarper }

impl Detail_TransverseMercatorWarper {
	// TransverseMercatorWarper(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/warpers.hpp:466
	// ("cv::detail::TransverseMercatorWarper::TransverseMercatorWarper", vec![(pred!(mut, ["scale"], ["float"]), _)]),
	#[inline]
	pub fn new(scale: f32) -> Result<crate::stitching::Detail_TransverseMercatorWarper> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::stitching::Detail_TransverseMercatorWarper::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Detail_TransverseMercatorWarper, crate::stitching::Detail_RotationWarper, cv_detail_TransverseMercatorWarper_to_Detail_RotationWarper }

impl std::fmt::Debug for Detail_TransverseMercatorWarper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_TransverseMercatorWarper")
			.finish()
	}
}

/// Constant methods for [crate::stitching::Detail_VoronoiSeamFinder]
// VoronoiSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:106
pub trait Detail_VoronoiSeamFinderTraitConst: crate::stitching::Detail_PairwiseSeamFinderTraitConst {
	fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void;

}

/// Mutable methods for [crate::stitching::Detail_VoronoiSeamFinder]
pub trait Detail_VoronoiSeamFinderTrait: crate::stitching::Detail_PairwiseSeamFinderTrait + crate::stitching::Detail_VoronoiSeamFinderTraitConst {
	fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void;

	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:109
	// ("cv::detail::VoronoiSeamFinder::find", vec![(pred!(mut, ["src", "corners", "masks"], ["const std::vector<cv::UMat>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_VoronoiSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_VoronoiSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// find(const std::vector<Size> &, const std::vector<Point> &, std::vector<UMat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:111
	// ("cv::detail::VoronoiSeamFinder::find", vec![(pred!(mut, ["size", "corners", "masks"], ["const std::vector<cv::Size>*", "const std::vector<cv::Point>*", "std::vector<cv::UMat>*"]), _)]),
	#[inline]
	fn find_1(&mut self, size: &core::Vector<core::Size>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_VoronoiSeamFinder_find_const_vectorLSizeGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_VoronoiSeamFinder(), size.as_raw_VectorOfSize(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Voronoi diagram-based seam estimator.
// VoronoiSeamFinder /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/stitching/detail/seam_finders.hpp:106
pub struct Detail_VoronoiSeamFinder {
	ptr: *mut c_void,
}

opencv_type_boxed! { Detail_VoronoiSeamFinder }

impl Drop for Detail_VoronoiSeamFinder {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_detail_VoronoiSeamFinder_delete(self.as_raw_mut_Detail_VoronoiSeamFinder()) };
	}
}

unsafe impl Send for Detail_VoronoiSeamFinder {}

impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_PairwiseSeamFinderTrait for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_VoronoiSeamFinder, crate::stitching::Detail_PairwiseSeamFinderTraitConst, as_raw_Detail_PairwiseSeamFinder, crate::stitching::Detail_PairwiseSeamFinderTrait, as_raw_mut_Detail_PairwiseSeamFinder }

impl crate::stitching::Detail_SeamFinderTraitConst for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_SeamFinderTrait for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_VoronoiSeamFinder, crate::stitching::Detail_SeamFinderTraitConst, as_raw_Detail_SeamFinder, crate::stitching::Detail_SeamFinderTrait, as_raw_mut_Detail_SeamFinder }

impl crate::stitching::Detail_VoronoiSeamFinderTraitConst for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::stitching::Detail_VoronoiSeamFinderTrait for Detail_VoronoiSeamFinder {
	#[inline] fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Detail_VoronoiSeamFinder, crate::stitching::Detail_VoronoiSeamFinderTraitConst, as_raw_Detail_VoronoiSeamFinder, crate::stitching::Detail_VoronoiSeamFinderTrait, as_raw_mut_Detail_VoronoiSeamFinder }

impl Detail_VoronoiSeamFinder {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_detail_VoronoiSeamFinder_defaultNew_const()) }
	}

}

boxed_cast_base! { Detail_VoronoiSeamFinder, crate::stitching::Detail_PairwiseSeamFinder, cv_detail_VoronoiSeamFinder_to_Detail_PairwiseSeamFinder }

boxed_cast_base! { Detail_VoronoiSeamFinder, crate::stitching::Detail_SeamFinder, cv_detail_VoronoiSeamFinder_to_Detail_SeamFinder }

impl std::fmt::Debug for Detail_VoronoiSeamFinder {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Detail_VoronoiSeamFinder")
			.finish()
	}
}

impl Default for Detail_VoronoiSeamFinder {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}
