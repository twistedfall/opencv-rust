pub mod stitching {
	//! # Images stitching
	//! 
	//! This figure illustrates the stitching module pipeline implemented in the Stitcher class. Using that
	//! class it's possible to configure/remove some steps, i.e. adjust the stitching pipeline according to
	//! the particular needs. All building blocks from the pipeline are available in the detail namespace,
	//! one can combine and use them separately.
	//! 
	//! The implemented stitching pipeline is very similar to the one proposed in [BL07](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_BL07) .
	//! 
	//! ![stitching pipeline](https://docs.opencv.org/4.8.1/StitchingPipeline.jpg)
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
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::Detail_RotationWarperTraitConst, super::Detail_RotationWarperTrait, super::Detail_ProjectorBaseTraitConst, super::Detail_ProjectorBaseTrait, super::Detail_PlaneProjectorTraitConst, super::Detail_PlaneProjectorTrait, super::Detail_PlaneWarperTraitConst, super::Detail_PlaneWarperTrait, super::Detail_AffineWarperTraitConst, super::Detail_AffineWarperTrait, super::Detail_SphericalProjectorTraitConst, super::Detail_SphericalProjectorTrait, super::Detail_SphericalWarperTraitConst, super::Detail_SphericalWarperTrait, super::Detail_CylindricalProjectorTraitConst, super::Detail_CylindricalProjectorTrait, super::Detail_CylindricalWarperTraitConst, super::Detail_CylindricalWarperTrait, super::Detail_FisheyeProjectorTraitConst, super::Detail_FisheyeProjectorTrait, super::Detail_FisheyeWarperTraitConst, super::Detail_FisheyeWarperTrait, super::Detail_StereographicProjectorTraitConst, super::Detail_StereographicProjectorTrait, super::Detail_StereographicWarperTraitConst, super::Detail_StereographicWarperTrait, super::Detail_CompressedRectilinearProjectorTraitConst, super::Detail_CompressedRectilinearProjectorTrait, super::Detail_CompressedRectilinearWarperTraitConst, super::Detail_CompressedRectilinearWarperTrait, super::Detail_CompressedRectilinearPortraitProjectorTraitConst, super::Detail_CompressedRectilinearPortraitProjectorTrait, super::Detail_CompressedRectilinearPortraitWarperTraitConst, super::Detail_CompressedRectilinearPortraitWarperTrait, super::Detail_PaniniProjectorTraitConst, super::Detail_PaniniProjectorTrait, super::Detail_PaniniWarperTraitConst, super::Detail_PaniniWarperTrait, super::Detail_PaniniPortraitProjectorTraitConst, super::Detail_PaniniPortraitProjectorTrait, super::Detail_PaniniPortraitWarperTraitConst, super::Detail_PaniniPortraitWarperTrait, super::Detail_MercatorProjectorTraitConst, super::Detail_MercatorProjectorTrait, super::Detail_MercatorWarperTraitConst, super::Detail_MercatorWarperTrait, super::Detail_TransverseMercatorProjectorTraitConst, super::Detail_TransverseMercatorProjectorTrait, super::Detail_TransverseMercatorWarperTraitConst, super::Detail_TransverseMercatorWarperTrait, super::Detail_PlaneWarperGpuTraitConst, super::Detail_PlaneWarperGpuTrait, super::Detail_SphericalWarperGpuTraitConst, super::Detail_SphericalWarperGpuTrait, super::Detail_CylindricalWarperGpuTraitConst, super::Detail_CylindricalWarperGpuTrait, super::Detail_SphericalPortraitProjectorTraitConst, super::Detail_SphericalPortraitProjectorTrait, super::Detail_SphericalPortraitWarperTraitConst, super::Detail_SphericalPortraitWarperTrait, super::Detail_CylindricalPortraitProjectorTraitConst, super::Detail_CylindricalPortraitProjectorTrait, super::Detail_CylindricalPortraitWarperTraitConst, super::Detail_CylindricalPortraitWarperTrait, super::Detail_PlanePortraitProjectorTraitConst, super::Detail_PlanePortraitProjectorTrait, super::Detail_PlanePortraitWarperTraitConst, super::Detail_PlanePortraitWarperTrait, super::PyRotationWarperTraitConst, super::PyRotationWarperTrait, super::WarperCreatorTraitConst, super::WarperCreatorTrait, super::PlaneWarperTraitConst, super::PlaneWarperTrait, super::AffineWarperTraitConst, super::AffineWarperTrait, super::CylindricalWarperTraitConst, super::CylindricalWarperTrait, super::SphericalWarperTraitConst, super::SphericalWarperTrait, super::FisheyeWarperTraitConst, super::FisheyeWarperTrait, super::StereographicWarperTraitConst, super::StereographicWarperTrait, super::CompressedRectilinearWarperTraitConst, super::CompressedRectilinearWarperTrait, super::CompressedRectilinearPortraitWarperTraitConst, super::CompressedRectilinearPortraitWarperTrait, super::PaniniWarperTraitConst, super::PaniniWarperTrait, super::PaniniPortraitWarperTraitConst, super::PaniniPortraitWarperTrait, super::MercatorWarperTraitConst, super::MercatorWarperTrait, super::TransverseMercatorWarperTraitConst, super::TransverseMercatorWarperTrait, super::PlaneWarperGpuTraitConst, super::PlaneWarperGpuTrait, super::CylindricalWarperGpuTraitConst, super::CylindricalWarperGpuTrait, super::SphericalWarperGpuTraitConst, super::SphericalWarperGpuTrait, super::Detail_ImageFeaturesTraitConst, super::Detail_ImageFeaturesTrait, super::Detail_MatchesInfoTraitConst, super::Detail_MatchesInfoTrait, super::Detail_FeaturesMatcherTraitConst, super::Detail_FeaturesMatcherTrait, super::Detail_BestOf2NearestMatcherTraitConst, super::Detail_BestOf2NearestMatcherTrait, super::Detail_BestOf2NearestRangeMatcherTraitConst, super::Detail_BestOf2NearestRangeMatcherTrait, super::Detail_AffineBestOf2NearestMatcherTraitConst, super::Detail_AffineBestOf2NearestMatcherTrait, super::Detail_DisjointSetsTraitConst, super::Detail_DisjointSetsTrait, super::Detail_GraphEdgeTraitConst, super::Detail_GraphEdgeTrait, super::Detail_GraphTraitConst, super::Detail_GraphTrait, super::Detail_CameraParamsTraitConst, super::Detail_CameraParamsTrait, super::Detail_EstimatorTraitConst, super::Detail_EstimatorTrait, super::Detail_HomographyBasedEstimatorTraitConst, super::Detail_HomographyBasedEstimatorTrait, super::Detail_AffineBasedEstimatorTraitConst, super::Detail_AffineBasedEstimatorTrait, super::Detail_BundleAdjusterBaseTraitConst, super::Detail_BundleAdjusterBaseTrait, super::Detail_NoBundleAdjusterTraitConst, super::Detail_NoBundleAdjusterTrait, super::Detail_BundleAdjusterReprojTraitConst, super::Detail_BundleAdjusterReprojTrait, super::Detail_BundleAdjusterRayTraitConst, super::Detail_BundleAdjusterRayTrait, super::Detail_BundleAdjusterAffineTraitConst, super::Detail_BundleAdjusterAffineTrait, super::Detail_BundleAdjusterAffinePartialTraitConst, super::Detail_BundleAdjusterAffinePartialTrait, super::Detail_ExposureCompensatorTraitConst, super::Detail_ExposureCompensatorTrait, super::Detail_NoExposureCompensatorTraitConst, super::Detail_NoExposureCompensatorTrait, super::Detail_GainCompensatorTraitConst, super::Detail_GainCompensatorTrait, super::Detail_ChannelsCompensatorTraitConst, super::Detail_ChannelsCompensatorTrait, super::Detail_BlocksCompensatorTraitConst, super::Detail_BlocksCompensatorTrait, super::Detail_BlocksGainCompensatorTraitConst, super::Detail_BlocksGainCompensatorTrait, super::Detail_BlocksChannelsCompensatorTraitConst, super::Detail_BlocksChannelsCompensatorTrait, super::Detail_SeamFinderTraitConst, super::Detail_SeamFinderTrait, super::Detail_NoSeamFinderTraitConst, super::Detail_NoSeamFinderTrait, super::Detail_PairwiseSeamFinderTraitConst, super::Detail_PairwiseSeamFinderTrait, super::Detail_VoronoiSeamFinderTraitConst, super::Detail_VoronoiSeamFinderTrait, super::Detail_DpSeamFinderTraitConst, super::Detail_DpSeamFinderTrait, super::Detail_GraphCutSeamFinderBaseTraitConst, super::Detail_GraphCutSeamFinderBaseTrait, super::Detail_GraphCutSeamFinderTraitConst, super::Detail_GraphCutSeamFinderTrait, super::Detail_GraphCutSeamFinderGpuTraitConst, super::Detail_GraphCutSeamFinderGpuTrait, super::Detail_BlenderTraitConst, super::Detail_BlenderTrait, super::Detail_FeatherBlenderTraitConst, super::Detail_FeatherBlenderTrait, super::Detail_MultiBandBlenderTraitConst, super::Detail_MultiBandBlenderTrait, super::StitcherTraitConst, super::StitcherTrait };
	}
	
	pub const Detail_Blender_FEATHER: i32 = 1;
	pub const Detail_Blender_MULTI_BAND: i32 = 2;
	pub const Detail_Blender_NO: i32 = 0;
	pub const Detail_DpSeamFinder_COLOR: i32 = 0;
	pub const Detail_DpSeamFinder_COLOR_GRAD: i32 = 1;
	pub const Detail_ExposureCompensator_CHANNELS: i32 = 3;
	pub const Detail_ExposureCompensator_CHANNELS_BLOCKS: i32 = 4;
	pub const Detail_ExposureCompensator_GAIN: i32 = 1;
	pub const Detail_ExposureCompensator_GAIN_BLOCKS: i32 = 2;
	pub const Detail_ExposureCompensator_NO: i32 = 0;
	pub const Detail_GraphCutSeamFinderBase_COST_COLOR: i32 = 0;
	pub const Detail_GraphCutSeamFinderBase_COST_COLOR_GRAD: i32 = 1;
	pub const Detail_SeamFinder_DP_SEAM: i32 = 2;
	pub const Detail_SeamFinder_NO: i32 = 0;
	pub const Detail_SeamFinder_VORONOI_SEAM: i32 = 1;
	pub const Detail_WAVE_CORRECT_AUTO: i32 = 2;
	pub const Detail_WAVE_CORRECT_HORIZ: i32 = 0;
	pub const Detail_WAVE_CORRECT_VERT: i32 = 1;
	pub const Stitcher_ERR_CAMERA_PARAMS_ADJUST_FAIL: i32 = 3;
	pub const Stitcher_ERR_HOMOGRAPHY_EST_FAIL: i32 = 2;
	pub const Stitcher_ERR_NEED_MORE_IMGS: i32 = 1;
	pub const Stitcher_OK: i32 = 0;
	/// Mode for creating photo panoramas. Expects images under perspective
	/// transformation and projects resulting pano to sphere.
	/// ## See also
	/// detail::BestOf2NearestMatcher SphericalWarper
	pub const Stitcher_PANORAMA: i32 = 0;
	/// Mode for composing scans. Expects images under affine transformation does
	/// not compensate exposure by default.
	/// ## See also
	/// detail::AffineBestOf2NearestMatcher AffineWarper
	pub const Stitcher_SCANS: i32 = 1;
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Detail_DpSeamFinder_CostFunction {
		COLOR = 0,
		COLOR_GRAD = 1,
	}
	
	opencv_type_enum! { crate::stitching::Detail_DpSeamFinder_CostFunction }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Detail_GraphCutSeamFinderBase_CostType {
		COST_COLOR = 0,
		COST_COLOR_GRAD = 1,
	}
	
	opencv_type_enum! { crate::stitching::Detail_GraphCutSeamFinderBase_CostType }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Detail_WaveCorrectKind {
		WAVE_CORRECT_HORIZ = 0,
		WAVE_CORRECT_VERT = 1,
		WAVE_CORRECT_AUTO = 2,
	}
	
	opencv_type_enum! { crate::stitching::Detail_WaveCorrectKind }
	
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
	
	opencv_type_enum! { crate::stitching::Stitcher_Mode }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Stitcher_Status {
		OK = 0,
		ERR_NEED_MORE_IMGS = 1,
		ERR_HOMOGRAPHY_EST_FAIL = 2,
		ERR_CAMERA_PARAMS_ADJUST_FAIL = 3,
	}
	
	opencv_type_enum! { crate::stitching::Stitcher_Status }
	
	/// Tries to detect the wave correction kind depending
	/// on whether a panorama spans horizontally or vertically
	/// 
	/// ## Parameters
	/// * rmats: Camera rotation matrices.
	/// ## Returns
	/// The correction kind to use for this panorama
	#[inline]
	pub fn auto_detect_wave_correct_kind(rmats: &core::Vector<core::Mat>) -> Result<crate::stitching::Detail_WaveCorrectKind> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_autoDetectWaveCorrectKind_const_vectorLMatGR(rmats.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// 
	/// ## Parameters
	/// * featuresFinder: 
	/// * image: 
	/// * features: 
	/// * mask: 
	/// 
	/// ## Note
	/// This alternative version of [compute_image_features2] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn compute_image_features2_def(features_finder: &core::Ptr<crate::features2d::Feature2D>, image: &impl core::ToInputArray, features: &mut crate::stitching::Detail_ImageFeatures) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_ImageFeaturesR(features_finder.as_raw_PtrOfFeature2D(), image.as_raw__InputArray(), features.as_raw_mut_Detail_ImageFeatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// 
	/// ## Parameters
	/// * featuresFinder: 
	/// * image: 
	/// * features: 
	/// * mask: 
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn compute_image_features2(features_finder: &core::Ptr<crate::features2d::Feature2D>, image: &impl core::ToInputArray, features: &mut crate::stitching::Detail_ImageFeatures, mask: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_ImageFeaturesR_const__InputArrayR(features_finder.as_raw_PtrOfFeature2D(), image.as_raw__InputArray(), features.as_raw_mut_Detail_ImageFeatures(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// 
	/// ## Parameters
	/// * featuresFinder: 
	/// * images: 
	/// * features: 
	/// * masks: 
	/// 
	/// ## Note
	/// This alternative version of [compute_image_features] function uses the following default values for its arguments:
	/// * masks: noArray()
	#[inline]
	pub fn compute_image_features_def(features_finder: &core::Ptr<crate::features2d::Feature2D>, images: &impl core::ToInputArray, features: &mut core::Vector<crate::stitching::Detail_ImageFeatures>) -> Result<()> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_vectorLImageFeaturesGR(features_finder.as_raw_PtrOfFeature2D(), images.as_raw__InputArray(), features.as_raw_mut_VectorOfDetail_ImageFeatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// 
	/// 
	/// ## Parameters
	/// * featuresFinder: 
	/// * images: 
	/// * features: 
	/// * masks: 
	/// 
	/// ## C++ default parameters
	/// * masks: noArray()
	#[inline]
	pub fn compute_image_features(features_finder: &core::Ptr<crate::features2d::Feature2D>, images: &impl core::ToInputArray, features: &mut core::Vector<crate::stitching::Detail_ImageFeatures>, masks: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(images);
		input_array_arg!(masks);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_computeImageFeatures_const_PtrLFeature2DGR_const__InputArrayR_vectorLImageFeaturesGR_const__InputArrayR(features_finder.as_raw_PtrOfFeature2D(), images.as_raw__InputArray(), features.as_raw_mut_VectorOfDetail_ImageFeatures(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn create_laplace_pyr_gpu(img: &impl core::ToInputArray, num_levels: i32, pyr: &mut core::Vector<core::UMat>) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vectorLUMatGR(img.as_raw__InputArray(), num_levels, pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn create_laplace_pyr(img: &impl core::ToInputArray, num_levels: i32, pyr: &mut core::Vector<core::UMat>) -> Result<()> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_createLaplacePyr_const__InputArrayR_int_vectorLUMatGR(img.as_raw__InputArray(), num_levels, pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn create_weight_map(mask: &impl core::ToInputArray, sharpness: f32, weight: &mut impl core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(mask);
		input_output_array_arg!(weight);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(mask.as_raw__InputArray(), sharpness, weight.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn find_max_spanning_tree(num_images: i32, pairwise_matches: &core::Vector<crate::stitching::Detail_MatchesInfo>, span_tree: &mut crate::stitching::Detail_Graph, centers: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_findMaxSpanningTree_int_const_vectorLMatchesInfoGR_GraphR_vectorLintGR(num_images, pairwise_matches.as_raw_VectorOfDetail_MatchesInfo(), span_tree.as_raw_mut_Detail_Graph(), centers.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	pub fn matches_graph_as_string(paths: &mut core::Vector<String>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>, conf_threshold: f32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_matchesGraphAsString_vectorLStringGR_vectorLMatchesInfoGR_float(paths.as_raw_mut_VectorOfString(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), conf_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ///////////////////////////////////////////////////////////////////////////
	#[inline]
	pub fn normalize_using_weight_map(weight: &impl core::ToInputArray, src: &mut impl core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(weight);
		input_output_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(weight.as_raw__InputArray(), src.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ///////////////////////////////////////////////////////////////////////////
	#[inline]
	pub fn overlap_roi(tl1: core::Point, tl2: core::Point, sz1: core::Size, sz2: core::Size, roi: &mut core::Rect) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_overlapRoi_Point_Point_Size_Size_RectR(tl1.opencv_as_extern(), tl2.opencv_as_extern(), sz1.opencv_as_extern(), sz2.opencv_as_extern(), roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn restore_image_from_laplace_pyr_gpu(pyr: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_restoreImageFromLaplacePyrGpu_vectorLUMatGR(pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn restore_image_from_laplace_pyr(pyr: &mut core::Vector<core::UMat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_restoreImageFromLaplacePyr_vectorLUMatGR(pyr.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn result_roi_intersection(corners: &core::Vector<core::Point>, sizes: &core::Vector<core::Size>) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_resultRoiIntersection_const_vectorLPointGR_const_vectorLSizeGR(corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn result_roi_1(corners: &core::Vector<core::Point>, sizes: &core::Vector<core::Size>) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_resultRoi_const_vectorLPointGR_const_vectorLSizeGR(corners.as_raw_VectorOfPoint(), sizes.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn result_roi(corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_resultRoi_const_vectorLPointGR_const_vectorLUMatGR(corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn result_tl(corners: &core::Vector<core::Point>) -> Result<core::Point> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_resultTl_const_vectorLPointGR(corners.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn select_random_subset(count: i32, size: i32, subset: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_selectRandomSubset_int_int_vectorLintGR(count, size, subset.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	pub fn wave_correct(rmats: &mut core::Vector<core::Mat>, kind: crate::stitching::Detail_WaveCorrectKind) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_detail_waveCorrect_vectorLMatGR_WaveCorrectKind(rmats.as_raw_mut_VectorOfMat(), kind, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::stitching::AffineWarper]
	pub trait AffineWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_AffineWarper(&self) -> *const c_void;
	
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
	pub struct AffineWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::AffineWarperTraitConst for AffineWarper {
		#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::AffineWarperTrait for AffineWarper {
		#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AffineWarper {
	}
	
	boxed_cast_base! { AffineWarper, crate::stitching::WarperCreator, cv_AffineWarper_to_WarperCreator }
	
	impl std::fmt::Debug for AffineWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AffineWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::CompressedRectilinearPortraitWarper]
	pub trait CompressedRectilinearPortraitWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void;
	
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
	
	pub struct CompressedRectilinearPortraitWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTraitConst for CompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTrait for CompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CompressedRectilinearPortraitWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait CompressedRectilinearWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void;
	
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
	
	pub struct CompressedRectilinearWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::CompressedRectilinearWarperTraitConst for CompressedRectilinearWarper {
		#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearWarperTrait for CompressedRectilinearWarper {
		#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CompressedRectilinearWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait CylindricalWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_CylindricalWarper(&self) -> *const c_void;
	
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
	pub struct CylindricalWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::CylindricalWarperTraitConst for CylindricalWarper {
		#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::CylindricalWarperTrait for CylindricalWarper {
		#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CylindricalWarper {
	}
	
	boxed_cast_base! { CylindricalWarper, crate::stitching::WarperCreator, cv_CylindricalWarper_to_WarperCreator }
	
	impl std::fmt::Debug for CylindricalWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CylindricalWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::CylindricalWarperGpu]
	pub trait CylindricalWarperGpuTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_CylindricalWarperGpu(&self) -> *const c_void;
	
		#[inline]
		fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_CylindricalWarperGpu_create_const_float(self.as_raw_CylindricalWarperGpu(), scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::stitching::CylindricalWarperGpu]
	pub trait CylindricalWarperGpuTrait: crate::stitching::CylindricalWarperGpuTraitConst + crate::stitching::WarperCreatorTrait {
		fn as_raw_mut_CylindricalWarperGpu(&mut self) -> *mut c_void;
	
	}
	
	pub struct CylindricalWarperGpu {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CylindricalWarperGpu }
	
	impl Drop for CylindricalWarperGpu {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_CylindricalWarperGpu_delete(self.as_raw_mut_CylindricalWarperGpu()) };
		}
	}
	
	unsafe impl Send for CylindricalWarperGpu {}
	
	impl crate::stitching::WarperCreatorTraitConst for CylindricalWarperGpu {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for CylindricalWarperGpu {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::CylindricalWarperGpuTraitConst for CylindricalWarperGpu {
		#[inline] fn as_raw_CylindricalWarperGpu(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::CylindricalWarperGpuTrait for CylindricalWarperGpu {
		#[inline] fn as_raw_mut_CylindricalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CylindricalWarperGpu {
	}
	
	boxed_cast_base! { CylindricalWarperGpu, crate::stitching::WarperCreator, cv_CylindricalWarperGpu_to_WarperCreator }
	
	impl std::fmt::Debug for CylindricalWarperGpu {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CylindricalWarperGpu")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::FisheyeWarper]
	pub trait FisheyeWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_FisheyeWarper(&self) -> *const c_void;
	
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
	
	pub struct FisheyeWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::FisheyeWarperTraitConst for FisheyeWarper {
		#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::FisheyeWarperTrait for FisheyeWarper {
		#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FisheyeWarper {
	}
	
	boxed_cast_base! { FisheyeWarper, crate::stitching::WarperCreator, cv_FisheyeWarper_to_WarperCreator }
	
	impl std::fmt::Debug for FisheyeWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FisheyeWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::MercatorWarper]
	pub trait MercatorWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_MercatorWarper(&self) -> *const c_void;
	
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
	
	pub struct MercatorWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::MercatorWarperTraitConst for MercatorWarper {
		#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::MercatorWarperTrait for MercatorWarper {
		#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MercatorWarper {
	}
	
	boxed_cast_base! { MercatorWarper, crate::stitching::WarperCreator, cv_MercatorWarper_to_WarperCreator }
	
	impl std::fmt::Debug for MercatorWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("MercatorWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::PaniniPortraitWarper]
	pub trait PaniniPortraitWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_PaniniPortraitWarper(&self) -> *const c_void;
	
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
	
	pub struct PaniniPortraitWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::PaniniPortraitWarperTraitConst for PaniniPortraitWarper {
		#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::PaniniPortraitWarperTrait for PaniniPortraitWarper {
		#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PaniniPortraitWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait PaniniWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_PaniniWarper(&self) -> *const c_void;
	
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
	
	pub struct PaniniWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::PaniniWarperTraitConst for PaniniWarper {
		#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::PaniniWarperTrait for PaniniWarper {
		#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PaniniWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait PlaneWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_PlaneWarper(&self) -> *const c_void;
	
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
	pub struct PlaneWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::PlaneWarperTraitConst for PlaneWarper {
		#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::PlaneWarperTrait for PlaneWarper {
		#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PlaneWarper {
	}
	
	boxed_cast_base! { PlaneWarper, crate::stitching::WarperCreator, cv_PlaneWarper_to_WarperCreator }
	
	impl std::fmt::Debug for PlaneWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PlaneWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::PlaneWarperGpu]
	pub trait PlaneWarperGpuTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_PlaneWarperGpu(&self) -> *const c_void;
	
		#[inline]
		fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PlaneWarperGpu_create_const_float(self.as_raw_PlaneWarperGpu(), scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::stitching::PlaneWarperGpu]
	pub trait PlaneWarperGpuTrait: crate::stitching::PlaneWarperGpuTraitConst + crate::stitching::WarperCreatorTrait {
		fn as_raw_mut_PlaneWarperGpu(&mut self) -> *mut c_void;
	
	}
	
	pub struct PlaneWarperGpu {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { PlaneWarperGpu }
	
	impl Drop for PlaneWarperGpu {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_PlaneWarperGpu_delete(self.as_raw_mut_PlaneWarperGpu()) };
		}
	}
	
	unsafe impl Send for PlaneWarperGpu {}
	
	impl crate::stitching::WarperCreatorTraitConst for PlaneWarperGpu {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for PlaneWarperGpu {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::PlaneWarperGpuTraitConst for PlaneWarperGpu {
		#[inline] fn as_raw_PlaneWarperGpu(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::PlaneWarperGpuTrait for PlaneWarperGpu {
		#[inline] fn as_raw_mut_PlaneWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PlaneWarperGpu {
	}
	
	boxed_cast_base! { PlaneWarperGpu, crate::stitching::WarperCreator, cv_PlaneWarperGpu_to_WarperCreator }
	
	impl std::fmt::Debug for PlaneWarperGpu {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PlaneWarperGpu")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::PyRotationWarper]
	pub trait PyRotationWarperTraitConst {
		fn as_raw_PyRotationWarper(&self) -> *const c_void;
	
		#[inline]
		fn get_scale(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_getScale_const(self.as_raw_PyRotationWarper(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::stitching::PyRotationWarper]
	pub trait PyRotationWarperTrait: crate::stitching::PyRotationWarperTraitConst {
		fn as_raw_mut_PyRotationWarper(&mut self) -> *mut c_void;
	
		/// Projects the image point.
		/// 
		/// ## Parameters
		/// * pt: Source point
		/// * K: Camera intrinsic parameters
		/// * R: Camera rotation matrix
		/// ## Returns
		/// Projected point
		#[inline]
		fn warp_point(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_PyRotationWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_point_backward(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_PyRotationWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_PyRotationWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
			input_array_arg!(src);
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(self.as_raw_mut_PyRotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn warp_backward(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst_size: core::Size, dst: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(self.as_raw_mut_PyRotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst_size.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn warp_roi(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_PyRotationWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_scale(&mut self, unnamed: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_setScale_float(self.as_raw_mut_PyRotationWarper(), unnamed, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct PyRotationWarper {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { PyRotationWarper }
	
	impl Drop for PyRotationWarper {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_PyRotationWarper_delete(self.as_raw_mut_PyRotationWarper()) };
		}
	}
	
	unsafe impl Send for PyRotationWarper {}
	
	impl crate::stitching::PyRotationWarperTraitConst for PyRotationWarper {
		#[inline] fn as_raw_PyRotationWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::PyRotationWarperTrait for PyRotationWarper {
		#[inline] fn as_raw_mut_PyRotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PyRotationWarper {
		#[inline]
		pub fn new(typ: &str, scale: f32) -> Result<crate::stitching::PyRotationWarper> {
			extern_container_arg!(mut typ);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_PyRotationWarper_String_float(typ.opencv_as_extern_mut(), scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::PyRotationWarper::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::stitching::PyRotationWarper> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_PyRotationWarper_PyRotationWarper(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::PyRotationWarper::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for PyRotationWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PyRotationWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::SphericalWarper]
	pub trait SphericalWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_SphericalWarper(&self) -> *const c_void;
	
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
	pub struct SphericalWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::SphericalWarperTraitConst for SphericalWarper {
		#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::SphericalWarperTrait for SphericalWarper {
		#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SphericalWarper {
	}
	
	boxed_cast_base! { SphericalWarper, crate::stitching::WarperCreator, cv_SphericalWarper_to_WarperCreator }
	
	impl std::fmt::Debug for SphericalWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SphericalWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::SphericalWarperGpu]
	pub trait SphericalWarperGpuTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_SphericalWarperGpu(&self) -> *const c_void;
	
		#[inline]
		fn create(&self, scale: f32) -> Result<core::Ptr<crate::stitching::Detail_RotationWarper>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_SphericalWarperGpu_create_const_float(self.as_raw_SphericalWarperGpu(), scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_RotationWarper>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::stitching::SphericalWarperGpu]
	pub trait SphericalWarperGpuTrait: crate::stitching::SphericalWarperGpuTraitConst + crate::stitching::WarperCreatorTrait {
		fn as_raw_mut_SphericalWarperGpu(&mut self) -> *mut c_void;
	
	}
	
	pub struct SphericalWarperGpu {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SphericalWarperGpu }
	
	impl Drop for SphericalWarperGpu {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_SphericalWarperGpu_delete(self.as_raw_mut_SphericalWarperGpu()) };
		}
	}
	
	unsafe impl Send for SphericalWarperGpu {}
	
	impl crate::stitching::WarperCreatorTraitConst for SphericalWarperGpu {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for SphericalWarperGpu {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::SphericalWarperGpuTraitConst for SphericalWarperGpu {
		#[inline] fn as_raw_SphericalWarperGpu(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::SphericalWarperGpuTrait for SphericalWarperGpu {
		#[inline] fn as_raw_mut_SphericalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SphericalWarperGpu {
	}
	
	boxed_cast_base! { SphericalWarperGpu, crate::stitching::WarperCreator, cv_SphericalWarperGpu_to_WarperCreator }
	
	impl std::fmt::Debug for SphericalWarperGpu {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SphericalWarperGpu")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::StereographicWarper]
	pub trait StereographicWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_StereographicWarper(&self) -> *const c_void;
	
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
	
	pub struct StereographicWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::StereographicWarperTraitConst for StereographicWarper {
		#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::StereographicWarperTrait for StereographicWarper {
		#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl StereographicWarper {
	}
	
	boxed_cast_base! { StereographicWarper, crate::stitching::WarperCreator, cv_StereographicWarper_to_WarperCreator }
	
	impl std::fmt::Debug for StereographicWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("StereographicWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Stitcher]
	pub trait StitcherTraitConst {
		fn as_raw_Stitcher(&self) -> *const c_void;
	
		#[inline]
		fn registration_resol(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_registrationResol_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn seam_estimation_resol(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_seamEstimationResol_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn compositing_resol(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_compositingResol_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn pano_confidence_thresh(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_panoConfidenceThresh_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn wave_correction(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_waveCorrection_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn interpolation_flags(&self) -> Result<crate::imgproc::InterpolationFlags> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_interpolationFlags_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn wave_correct_kind(&self) -> Result<crate::stitching::Detail_WaveCorrectKind> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_waveCorrectKind_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn features_finder(&self) -> Result<core::Ptr<crate::features2d::Feature2D>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_featuresFinder_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn features_matcher(&self) -> Result<core::Ptr<crate::stitching::Detail_FeaturesMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_featuresMatcher_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_FeaturesMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn matching_mask(&self) -> Result<core::UMat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_matchingMask_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::UMat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn bundle_adjuster(&self) -> Result<core::Ptr<crate::stitching::Detail_BundleAdjusterBase>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_bundleAdjuster_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_BundleAdjusterBase>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn estimator(&self) -> Result<core::Ptr<crate::stitching::Detail_Estimator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_estimator_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_Estimator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn warper(&self) -> Result<core::Ptr<crate::stitching::WarperCreator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_warper_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::WarperCreator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn exposure_compensator(&self) -> Result<core::Ptr<crate::stitching::Detail_ExposureCompensator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_exposureCompensator_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn seam_finder(&self) -> Result<core::Ptr<crate::stitching::Detail_SeamFinder>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_seamFinder_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_SeamFinder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn blender(&self) -> Result<core::Ptr<crate::stitching::Detail_Blender>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_blender_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn component(&self) -> Result<core::Vector<i32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_component_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn cameras(&self) -> Result<core::Vector<crate::stitching::Detail_CameraParams>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_cameras_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::stitching::Detail_CameraParams>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn work_scale(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_workScale_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Return the mask of the panorama.
		/// 
		/// The mask is a 8U UMat with the values: 0xFF (white) for pixels filled by the input images,
		/// 0 (black) for unused pixels. It can be used as the mask for inpaint.
		/// 
		/// ## Returns
		/// The mask.
		#[inline]
		fn result_mask(&self) -> Result<core::UMat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_resultMask_const(self.as_raw_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::UMat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Stitcher]
	pub trait StitcherTrait: crate::stitching::StitcherTraitConst {
		fn as_raw_mut_Stitcher(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_registration_resol(&mut self, resol_mpx: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setRegistrationResol_double(self.as_raw_mut_Stitcher(), resol_mpx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_seam_estimation_resol(&mut self, resol_mpx: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setSeamEstimationResol_double(self.as_raw_mut_Stitcher(), resol_mpx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_compositing_resol(&mut self, resol_mpx: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setCompositingResol_double(self.as_raw_mut_Stitcher(), resol_mpx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_pano_confidence_thresh(&mut self, conf_thresh: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setPanoConfidenceThresh_double(self.as_raw_mut_Stitcher(), conf_thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_wave_correction(&mut self, flag: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setWaveCorrection_bool(self.as_raw_mut_Stitcher(), flag, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_interpolation_flags(&mut self, interp_flags: crate::imgproc::InterpolationFlags) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setInterpolationFlags_InterpolationFlags(self.as_raw_mut_Stitcher(), interp_flags, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_wave_correct_kind(&mut self, kind: crate::stitching::Detail_WaveCorrectKind) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(self.as_raw_mut_Stitcher(), kind, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn features_finder_1(&mut self) -> Result<core::Ptr<crate::features2d::Feature2D>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_featuresFinder(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_features_finder(&mut self, mut features_finder: core::Ptr<crate::features2d::Feature2D>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setFeaturesFinder_PtrLFeature2DG(self.as_raw_mut_Stitcher(), features_finder.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn features_matcher_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_FeaturesMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_featuresMatcher(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_FeaturesMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_features_matcher(&mut self, mut features_matcher: core::Ptr<crate::stitching::Detail_FeaturesMatcher>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setFeaturesMatcher_PtrLFeaturesMatcherG(self.as_raw_mut_Stitcher(), features_matcher.as_raw_mut_PtrOfDetail_FeaturesMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_matching_mask(&mut self, mask: &core::UMat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setMatchingMask_const_UMatR(self.as_raw_mut_Stitcher(), mask.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn bundle_adjuster_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_BundleAdjusterBase>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_bundleAdjuster(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_BundleAdjusterBase>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_bundle_adjuster(&mut self, mut bundle_adjuster: core::Ptr<crate::stitching::Detail_BundleAdjusterBase>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setBundleAdjuster_PtrLBundleAdjusterBaseG(self.as_raw_mut_Stitcher(), bundle_adjuster.as_raw_mut_PtrOfDetail_BundleAdjusterBase(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn estimator_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_Estimator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_estimator(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_Estimator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_estimator(&mut self, mut estimator: core::Ptr<crate::stitching::Detail_Estimator>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setEstimator_PtrLEstimatorG(self.as_raw_mut_Stitcher(), estimator.as_raw_mut_PtrOfDetail_Estimator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warper_1(&mut self) -> Result<core::Ptr<crate::stitching::WarperCreator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_warper(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::WarperCreator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_warper(&mut self, mut creator: core::Ptr<crate::stitching::WarperCreator>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setWarper_PtrLWarperCreatorG(self.as_raw_mut_Stitcher(), creator.as_raw_mut_PtrOfWarperCreator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn exposure_compensator_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_ExposureCompensator>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_exposureCompensator(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_ExposureCompensator>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_exposure_compensator(&mut self, mut exposure_comp: core::Ptr<crate::stitching::Detail_ExposureCompensator>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setExposureCompensator_PtrLExposureCompensatorG(self.as_raw_mut_Stitcher(), exposure_comp.as_raw_mut_PtrOfDetail_ExposureCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn seam_finder_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_SeamFinder>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_seamFinder(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_SeamFinder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_seam_finder(&mut self, mut seam_finder: core::Ptr<crate::stitching::Detail_SeamFinder>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setSeamFinder_PtrLSeamFinderG(self.as_raw_mut_Stitcher(), seam_finder.as_raw_mut_PtrOfDetail_SeamFinder(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn blender_1(&mut self) -> Result<core::Ptr<crate::stitching::Detail_Blender>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_blender(self.as_raw_mut_Stitcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_Blender>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
		/// * masks: Masks for each input image specifying where to look for keypoints (optional).
		/// ## Returns
		/// Status code.
		/// 
		/// ## C++ default parameters
		/// * masks: noArray()
		#[inline]
		fn estimate_transform(&mut self, images: &impl core::ToInputArray, masks: &impl core::ToInputArray) -> Result<crate::stitching::Stitcher_Status> {
			input_array_arg!(images);
			input_array_arg!(masks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_estimateTransform_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), masks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
		/// * masks: Masks for each input image specifying where to look for keypoints (optional).
		/// ## Returns
		/// Status code.
		/// 
		/// ## Note
		/// This alternative version of [estimate_transform] function uses the following default values for its arguments:
		/// * masks: noArray()
		#[inline]
		fn estimate_transform_def(&mut self, images: &impl core::ToInputArray) -> Result<crate::stitching::Stitcher_Status> {
			input_array_arg!(images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_estimateTransform_const__InputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// These function restors camera rotation and camera intrinsics of each camera
		/// that can be got with [Stitcher::cameras] call
		/// 
		/// ## Parameters
		/// * images: Input images.
		/// * cameras: Estimated rotation of cameras for each of the input images.
		/// * component: Indices (0-based) of images constituting the final panorama (optional).
		/// ## Returns
		/// Status code.
		#[inline]
		fn set_transform(&mut self, images: &impl core::ToInputArray, cameras: &core::Vector<crate::stitching::Detail_CameraParams>, component: &core::Vector<i32>) -> Result<crate::stitching::Stitcher_Status> {
			input_array_arg!(images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setTransform_const__InputArrayR_const_vectorLCameraParamsGR_const_vectorLintGR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), cameras.as_raw_VectorOfDetail_CameraParams(), component.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// These function restors camera rotation and camera intrinsics of each camera
		/// that can be got with [Stitcher::cameras] call
		/// 
		/// ## Parameters
		/// * images: Input images.
		/// * cameras: Estimated rotation of cameras for each of the input images.
		/// * component: Indices (0-based) of images constituting the final panorama (optional).
		/// ## Returns
		/// Status code.
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn set_transform_1(&mut self, images: &impl core::ToInputArray, cameras: &core::Vector<crate::stitching::Detail_CameraParams>) -> Result<crate::stitching::Stitcher_Status> {
			input_array_arg!(images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_setTransform_const__InputArrayR_const_vectorLCameraParamsGR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), cameras.as_raw_VectorOfDetail_CameraParams(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn compose_panorama(&mut self, pano: &mut impl core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
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
		#[inline]
		fn compose_panorama_images(&mut self, images: &impl core::ToInputArray, pano: &mut impl core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
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
		/// * masks: Masks for each input image specifying where to look for keypoints (optional).
		/// * pano: Final pano.
		/// ## Returns
		/// Status code.
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn stitch(&mut self, images: &impl core::ToInputArray, pano: &mut impl core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
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
		/// * masks: Masks for each input image specifying where to look for keypoints (optional).
		/// * pano: Final pano.
		/// ## Returns
		/// Status code.
		#[inline]
		fn stitch_mask(&mut self, images: &impl core::ToInputArray, masks: &impl core::ToInputArray, pano: &mut impl core::ToOutputArray) -> Result<crate::stitching::Stitcher_Status> {
			input_array_arg!(images);
			input_array_arg!(masks);
			output_array_arg!(pano);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_stitch_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Stitcher(), images.as_raw__InputArray(), masks.as_raw__InputArray(), pano.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
	/// *   A basic example on image stitching can be found at
	///    opencv_source_code/samples/cpp/stitching.cpp
	/// *   A basic example on image stitching in Python can be found at
	///    opencv_source_code/samples/python/stitching.py
	/// *   A detailed example on image stitching can be found at
	///    opencv_source_code/samples/cpp/stitching_detailed.cpp
	pub struct Stitcher {
		ptr: *mut c_void
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
	
	impl Stitcher {
		pub const ORIG_RESOL: f64 = -1.;
		/// Creates a Stitcher configured in one of the stitching modes.
		/// 
		/// ## Parameters
		/// * mode: Scenario for stitcher operation. This is usually determined by source of images
		/// to stitch and their transformation. Default parameters will be chosen for operation in given
		/// scenario.
		/// ## Returns
		/// Stitcher class instance.
		/// 
		/// ## C++ default parameters
		/// * mode: Stitcher::PANORAMA
		#[inline]
		pub fn create(mode: crate::stitching::Stitcher_Mode) -> Result<core::Ptr<crate::stitching::Stitcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Stitcher_create_Mode(mode, ocvrs_return.as_mut_ptr()) };
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
		/// ## Returns
		/// Stitcher class instance.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * mode: Stitcher::PANORAMA
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
	
	/// Constant methods for [crate::stitching::TransverseMercatorWarper]
	pub trait TransverseMercatorWarperTraitConst: crate::stitching::WarperCreatorTraitConst {
		fn as_raw_TransverseMercatorWarper(&self) -> *const c_void;
	
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
	
	pub struct TransverseMercatorWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::TransverseMercatorWarperTraitConst for TransverseMercatorWarper {
		#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::TransverseMercatorWarperTrait for TransverseMercatorWarper {
		#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TransverseMercatorWarper {
	}
	
	boxed_cast_base! { TransverseMercatorWarper, crate::stitching::WarperCreator, cv_TransverseMercatorWarper_to_WarperCreator }
	
	impl std::fmt::Debug for TransverseMercatorWarper {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TransverseMercatorWarper")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::WarperCreator]
	pub trait WarperCreatorTraitConst {
		fn as_raw_WarperCreator(&self) -> *const c_void;
	
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
	pub struct WarperCreator {
		ptr: *mut c_void
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
	
	impl WarperCreator {
	}
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::AffineWarper, cv_WarperCreator_to_AffineWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::CompressedRectilinearPortraitWarper, cv_WarperCreator_to_CompressedRectilinearPortraitWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::CompressedRectilinearWarper, cv_WarperCreator_to_CompressedRectilinearWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::CylindricalWarper, cv_WarperCreator_to_CylindricalWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::CylindricalWarperGpu, cv_WarperCreator_to_CylindricalWarperGpu }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::FisheyeWarper, cv_WarperCreator_to_FisheyeWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::MercatorWarper, cv_WarperCreator_to_MercatorWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::PaniniPortraitWarper, cv_WarperCreator_to_PaniniPortraitWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::PaniniWarper, cv_WarperCreator_to_PaniniWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::PlaneWarper, cv_WarperCreator_to_PlaneWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::PlaneWarperGpu, cv_WarperCreator_to_PlaneWarperGpu }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::SphericalWarper, cv_WarperCreator_to_SphericalWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::SphericalWarperGpu, cv_WarperCreator_to_SphericalWarperGpu }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::StereographicWarper, cv_WarperCreator_to_StereographicWarper }
	
	boxed_cast_descendant! { WarperCreator, crate::stitching::TransverseMercatorWarper, cv_WarperCreator_to_TransverseMercatorWarper }
	
	impl std::fmt::Debug for WarperCreator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("WarperCreator")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_AffineBasedEstimator]
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
	pub struct Detail_AffineBasedEstimator {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_AffineBasedEstimatorTraitConst for Detail_AffineBasedEstimator {
		#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_AffineBasedEstimatorTrait for Detail_AffineBasedEstimator {
		#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_AffineBasedEstimator {
		#[inline]
		pub fn default() -> Result<crate::stitching::Detail_AffineBasedEstimator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_AffineBasedEstimator_AffineBasedEstimator(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_AffineBasedEstimator::opencv_from_extern(ret) };
			Ok(ret)
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
	
	/// Constant methods for [crate::stitching::Detail_AffineBestOf2NearestMatcher]
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
	pub struct Detail_AffineBestOf2NearestMatcher {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_FeaturesMatcherTraitConst for Detail_AffineBestOf2NearestMatcher {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTrait for Detail_AffineBestOf2NearestMatcher {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_AffineBestOf2NearestMatcherTraitConst for Detail_AffineBestOf2NearestMatcher {
		#[inline] fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_AffineBestOf2NearestMatcherTrait for Detail_AffineBestOf2NearestMatcher {
		#[inline] fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
		#[inline]
		fn warp_point(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, h: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(h);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_AffineWarper(), &pt, k.as_raw__InputArray(), h.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Projects the image point backward.
		/// 
		/// ## Parameters
		/// * pt: Projected point
		/// * K: Camera intrinsic parameters
		/// * H: Camera extrinsic parameters
		/// ## Returns
		/// Backward-projected point
		#[inline]
		fn warp_point_backward(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, h: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(h);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_AffineWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_AffineWarper(), &pt, k.as_raw__InputArray(), h.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, h: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(h);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_AffineWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), h.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, h: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		#[inline]
		fn warp_roi(&mut self, src_size: core::Size, k: &impl core::ToInputArray, h: &impl core::ToInputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(h);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_AffineWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), h.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
	
	impl crate::stitching::Detail_RotationWarperTraitConst for Detail_AffineWarper {
		#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_RotationWarperTrait for Detail_AffineWarper {
		#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_AffineWarperTraitConst for Detail_AffineWarper {
		#[inline] fn as_raw_Detail_AffineWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_AffineWarperTrait for Detail_AffineWarper {
		#[inline] fn as_raw_mut_Detail_AffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_AffineWarper {
		/// Construct an instance of the affine warper class.
		/// 
		/// ## Parameters
		/// * scale: Projected image scale multiplier
		/// 
		/// ## C++ default parameters
		/// * scale: 1.f
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
	pub trait Detail_BestOf2NearestMatcherTraitConst: crate::stitching::Detail_FeaturesMatcherTraitConst {
		fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_BestOf2NearestMatcher]
	pub trait Detail_BestOf2NearestMatcherTrait: crate::stitching::Detail_BestOf2NearestMatcherTraitConst + crate::stitching::Detail_FeaturesMatcherTrait {
		fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void;
	
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
	pub struct Detail_BestOf2NearestMatcher {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for Detail_BestOf2NearestMatcher {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for Detail_BestOf2NearestMatcher {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
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
		/// * matches_confindece_thresh: Matching confidence threshold to take the match into account.
		/// The threshold was determined experimentally and set to 3 by default.
		/// 
		/// ## C++ default parameters
		/// * try_use_gpu: false
		/// * match_conf: 0.3f
		/// * num_matches_thresh1: 6
		/// * num_matches_thresh2: 6
		/// * matches_confindece_thresh: 3.
		#[inline]
		pub fn new(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, matches_confindece_thresh: f64) -> Result<crate::stitching::Detail_BestOf2NearestMatcher> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int_double(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2, matches_confindece_thresh, ocvrs_return.as_mut_ptr()) };
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
		/// * matches_confindece_thresh: Matching confidence threshold to take the match into account.
		/// The threshold was determined experimentally and set to 3 by default.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * try_use_gpu: false
		/// * match_conf: 0.3f
		/// * num_matches_thresh1: 6
		/// * num_matches_thresh2: 6
		/// * matches_confindece_thresh: 3.
		#[inline]
		pub fn new_def() -> Result<crate::stitching::Detail_BestOf2NearestMatcher> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_BestOf2NearestMatcher::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * try_use_gpu: false
		/// * match_conf: 0.3f
		/// * num_matches_thresh1: 6
		/// * num_matches_thresh2: 6
		/// * matches_confindece_thresh: 3.
		#[inline]
		pub fn create(try_use_gpu: bool, match_conf: f32, num_matches_thresh1: i32, num_matches_thresh2: i32, matches_confindece_thresh: f64) -> Result<core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BestOf2NearestMatcher_create_bool_float_int_int_double(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2, matches_confindece_thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_BestOf2NearestMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * try_use_gpu: false
		/// * match_conf: 0.3f
		/// * num_matches_thresh1: 6
		/// * num_matches_thresh2: 6
		/// * matches_confindece_thresh: 3.
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BestOf2NearestMatcher_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_BestOf2NearestMatcher>::opencv_from_extern(ret) };
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
	pub trait Detail_BestOf2NearestRangeMatcherTraitConst: crate::stitching::Detail_BestOf2NearestMatcherTraitConst {
		fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_BestOf2NearestRangeMatcher]
	pub trait Detail_BestOf2NearestRangeMatcherTrait: crate::stitching::Detail_BestOf2NearestMatcherTrait + crate::stitching::Detail_BestOf2NearestRangeMatcherTraitConst {
		fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_BestOf2NearestRangeMatcher {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_FeaturesMatcherTraitConst for Detail_BestOf2NearestRangeMatcher {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTrait for Detail_BestOf2NearestRangeMatcher {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestRangeMatcherTraitConst for Detail_BestOf2NearestRangeMatcher {
		#[inline] fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestRangeMatcherTrait for Detail_BestOf2NearestRangeMatcher {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BestOf2NearestRangeMatcher {
		/// ## C++ default parameters
		/// * range_width: 5
		/// * try_use_gpu: false
		/// * match_conf: 0.3f
		/// * num_matches_thresh1: 6
		/// * num_matches_thresh2: 6
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
		#[inline]
		fn prepare_1(&mut self, dst_roi: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_Blender_prepare_Rect(self.as_raw_mut_Detail_Blender(), dst_roi.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn feed(&mut self, img: &impl core::ToInputArray, mask: &impl core::ToInputArray, tl: core::Point) -> Result<()> {
			input_array_arg!(img);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_Blender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), tl.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Blends and returns the final pano.
		/// 
		/// ## Parameters
		/// * dst: Final pano
		/// * dst_mask: Final pano mask
		#[inline]
		fn blend(&mut self, dst: &mut impl core::ToInputOutputArray, dst_mask: &mut impl core::ToInputOutputArray) -> Result<()> {
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
	pub struct Detail_Blender {
		ptr: *mut c_void
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
	
	impl Detail_Blender {
		/// ## C++ default parameters
		/// * try_gpu: false
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
		/// This alternative version of [create_default] function uses the following default values for its arguments:
		/// * try_gpu: false
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
	
	/// Constant methods for [crate::stitching::Detail_BlocksChannelsCompensator]
	pub trait Detail_BlocksChannelsCompensatorTraitConst: crate::stitching::Detail_BlocksCompensatorTraitConst {
		fn as_raw_Detail_BlocksChannelsCompensator(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_BlocksChannelsCompensator]
	pub trait Detail_BlocksChannelsCompensatorTrait: crate::stitching::Detail_BlocksChannelsCompensatorTraitConst + crate::stitching::Detail_BlocksCompensatorTrait {
		fn as_raw_mut_Detail_BlocksChannelsCompensator(&mut self) -> *mut c_void;
	
		#[inline]
		fn feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksChannelsCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_BlocksChannelsCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Exposure compensator which tries to remove exposure related artifacts by adjusting image block
	/// on each channel.
	pub struct Detail_BlocksChannelsCompensator {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_BlocksChannelsCompensator }
	
	impl Drop for Detail_BlocksChannelsCompensator {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_BlocksChannelsCompensator_delete(self.as_raw_mut_Detail_BlocksChannelsCompensator()) };
		}
	}
	
	unsafe impl Send for Detail_BlocksChannelsCompensator {}
	
	impl crate::stitching::Detail_BlocksCompensatorTraitConst for Detail_BlocksChannelsCompensator {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTrait for Detail_BlocksChannelsCompensator {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_BlocksChannelsCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_BlocksChannelsCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksChannelsCompensatorTraitConst for Detail_BlocksChannelsCompensator {
		#[inline] fn as_raw_Detail_BlocksChannelsCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksChannelsCompensatorTrait for Detail_BlocksChannelsCompensator {
		#[inline] fn as_raw_mut_Detail_BlocksChannelsCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BlocksChannelsCompensator {
		/// ## C++ default parameters
		/// * bl_width: 32
		/// * bl_height: 32
		/// * nr_feeds: 1
		#[inline]
		pub fn new(bl_width: i32, bl_height: i32, nr_feeds: i32) -> Result<crate::stitching::Detail_BlocksChannelsCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator_int_int_int(bl_width, bl_height, nr_feeds, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_BlocksChannelsCompensator::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * bl_width: 32
		/// * bl_height: 32
		/// * nr_feeds: 1
		#[inline]
		pub fn new_def() -> Result<crate::stitching::Detail_BlocksChannelsCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_BlocksChannelsCompensator::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Detail_BlocksChannelsCompensator, crate::stitching::Detail_BlocksCompensator, cv_detail_BlocksChannelsCompensator_to_Detail_BlocksCompensator }
	
	boxed_cast_base! { Detail_BlocksChannelsCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_BlocksChannelsCompensator_to_Detail_ExposureCompensator }
	
	impl std::fmt::Debug for Detail_BlocksChannelsCompensator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_BlocksChannelsCompensator")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_BlocksCompensator]
	pub trait Detail_BlocksCompensatorTraitConst: crate::stitching::Detail_ExposureCompensatorTraitConst {
		fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void;
	
		#[inline]
		fn get_similarity_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_getSimilarityThreshold_const(self.as_raw_Detail_BlocksCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_block_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_getBlockSize_const(self.as_raw_Detail_BlocksCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nr_gains_filtering_iterations(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_getNrGainsFilteringIterations_const(self.as_raw_Detail_BlocksCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_BlocksCompensator]
	pub trait Detail_BlocksCompensatorTrait: crate::stitching::Detail_BlocksCompensatorTraitConst + crate::stitching::Detail_ExposureCompensatorTrait {
		fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void;
	
		#[inline]
		fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl core::ToInputOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
			input_output_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_BlocksCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_getMatGains_vectorLMatGR(self.as_raw_mut_Detail_BlocksCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_setMatGains_vectorLMatGR(self.as_raw_mut_Detail_BlocksCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_nr_feeds(&mut self, nr_feeds: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_setNrFeeds_int(self.as_raw_mut_Detail_BlocksCompensator(), nr_feeds, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nr_feeds(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_getNrFeeds(self.as_raw_mut_Detail_BlocksCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_similarity_threshold(&mut self, similarity_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_setSimilarityThreshold_double(self.as_raw_mut_Detail_BlocksCompensator(), similarity_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_block_size(&mut self, width: i32, height: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_setBlockSize_int_int(self.as_raw_mut_Detail_BlocksCompensator(), width, height, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_block_size_1(&mut self, size: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_setBlockSize_Size(self.as_raw_mut_Detail_BlocksCompensator(), size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_nr_gains_filtering_iterations(&mut self, nr_iterations: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksCompensator_setNrGainsFilteringIterations_int(self.as_raw_mut_Detail_BlocksCompensator(), nr_iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Exposure compensator which tries to remove exposure related artifacts by adjusting image blocks.
	pub struct Detail_BlocksCompensator {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_BlocksCompensator }
	
	impl Drop for Detail_BlocksCompensator {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_BlocksCompensator_delete(self.as_raw_mut_Detail_BlocksCompensator()) };
		}
	}
	
	unsafe impl Send for Detail_BlocksCompensator {}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_BlocksCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_BlocksCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTraitConst for Detail_BlocksCompensator {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTrait for Detail_BlocksCompensator {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BlocksCompensator {
	}
	
	boxed_cast_descendant! { Detail_BlocksCompensator, crate::stitching::Detail_BlocksChannelsCompensator, cv_detail_BlocksCompensator_to_Detail_BlocksChannelsCompensator }
	
	boxed_cast_descendant! { Detail_BlocksCompensator, crate::stitching::Detail_BlocksGainCompensator, cv_detail_BlocksCompensator_to_Detail_BlocksGainCompensator }
	
	boxed_cast_base! { Detail_BlocksCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_BlocksCompensator_to_Detail_ExposureCompensator }
	
	impl std::fmt::Debug for Detail_BlocksCompensator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_BlocksCompensator")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_BlocksGainCompensator]
	pub trait Detail_BlocksGainCompensatorTraitConst: crate::stitching::Detail_BlocksCompensatorTraitConst {
		fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_BlocksGainCompensator]
	pub trait Detail_BlocksGainCompensatorTrait: crate::stitching::Detail_BlocksCompensatorTrait + crate::stitching::Detail_BlocksGainCompensatorTraitConst {
		fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void;
	
		#[inline]
		fn feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksGainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_BlocksGainCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl core::ToInputOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
			input_output_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_BlocksGainCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksGainCompensator_getMatGains_vectorLMatGR(self.as_raw_mut_Detail_BlocksGainCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksGainCompensator_setMatGains_vectorLMatGR(self.as_raw_mut_Detail_BlocksGainCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Exposure compensator which tries to remove exposure related artifacts by adjusting image block
	/// intensities, see [UES01](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_UES01) for details.
	pub struct Detail_BlocksGainCompensator {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_BlocksGainCompensator }
	
	impl Drop for Detail_BlocksGainCompensator {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_BlocksGainCompensator_delete(self.as_raw_mut_Detail_BlocksGainCompensator()) };
		}
	}
	
	unsafe impl Send for Detail_BlocksGainCompensator {}
	
	impl crate::stitching::Detail_BlocksCompensatorTraitConst for Detail_BlocksGainCompensator {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTrait for Detail_BlocksGainCompensator {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_BlocksGainCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_BlocksGainCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksGainCompensatorTraitConst for Detail_BlocksGainCompensator {
		#[inline] fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksGainCompensatorTrait for Detail_BlocksGainCompensator {
		#[inline] fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BlocksGainCompensator {
		/// ## C++ default parameters
		/// * bl_width: 32
		/// * bl_height: 32
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
		#[inline]
		pub fn new_def() -> Result<crate::stitching::Detail_BlocksGainCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksGainCompensator_BlocksGainCompensator(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_BlocksGainCompensator::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn new_1(bl_width: i32, bl_height: i32, nr_feeds: i32) -> Result<crate::stitching::Detail_BlocksGainCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int_int(bl_width, bl_height, nr_feeds, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_BlocksGainCompensator::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Detail_BlocksGainCompensator, crate::stitching::Detail_BlocksCompensator, cv_detail_BlocksGainCompensator_to_Detail_BlocksCompensator }
	
	boxed_cast_base! { Detail_BlocksGainCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_BlocksGainCompensator_to_Detail_ExposureCompensator }
	
	impl std::fmt::Debug for Detail_BlocksGainCompensator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_BlocksGainCompensator")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_BundleAdjusterAffine]
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
	pub struct Detail_BundleAdjusterAffine {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterAffine {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterAffine {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTraitConst for Detail_BundleAdjusterAffine {
		#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTrait for Detail_BundleAdjusterAffine {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BundleAdjusterAffine {
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
	pub struct Detail_BundleAdjusterAffinePartial {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst for Detail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for Detail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BundleAdjusterAffinePartial {
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
	pub trait Detail_BundleAdjusterBaseTraitConst: crate::stitching::Detail_EstimatorTraitConst {
		fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void;
	
		#[inline]
		fn refinement_mask(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BundleAdjusterBase_refinementMask_const(self.as_raw_Detail_BundleAdjusterBase(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_refinement_mask(&mut self, mask: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(self.as_raw_mut_Detail_BundleAdjusterBase(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_conf_thresh(&mut self, conf_thresh: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BundleAdjusterBase_setConfThresh_double(self.as_raw_mut_Detail_BundleAdjusterBase(), conf_thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn term_criteria(&mut self) -> Result<core::TermCriteria> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_BundleAdjusterBase_termCriteria(self.as_raw_mut_Detail_BundleAdjusterBase(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct Detail_BundleAdjusterBase {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for Detail_BundleAdjusterBase {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTrait for Detail_BundleAdjusterBase {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
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
	pub struct Detail_BundleAdjusterRay {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterRay {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterRay {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTraitConst for Detail_BundleAdjusterRay {
		#[inline] fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTrait for Detail_BundleAdjusterRay {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BundleAdjusterRay {
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
	pub struct Detail_BundleAdjusterReproj {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_EstimatorTraitConst for Detail_BundleAdjusterReproj {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for Detail_BundleAdjusterReproj {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTraitConst for Detail_BundleAdjusterReproj {
		#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTrait for Detail_BundleAdjusterReproj {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_BundleAdjusterReproj {
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
	pub trait Detail_CameraParamsTraitConst {
		fn as_raw_Detail_CameraParams(&self) -> *const c_void;
	
		#[inline]
		fn focal(&self) -> f64 {
			let ret = unsafe { sys::cv_detail_CameraParams_propFocal_const(self.as_raw_Detail_CameraParams()) };
			ret
		}
		
		#[inline]
		fn aspect(&self) -> f64 {
			let ret = unsafe { sys::cv_detail_CameraParams_propAspect_const(self.as_raw_Detail_CameraParams()) };
			ret
		}
		
		#[inline]
		fn ppx(&self) -> f64 {
			let ret = unsafe { sys::cv_detail_CameraParams_propPpx_const(self.as_raw_Detail_CameraParams()) };
			ret
		}
		
		#[inline]
		fn ppy(&self) -> f64 {
			let ret = unsafe { sys::cv_detail_CameraParams_propPpy_const(self.as_raw_Detail_CameraParams()) };
			ret
		}
		
		#[inline]
		fn r(&self) -> core::Mat {
			let ret = unsafe { sys::cv_detail_CameraParams_propR_const(self.as_raw_Detail_CameraParams()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn t(&self) -> core::Mat {
			let ret = unsafe { sys::cv_detail_CameraParams_propT_const(self.as_raw_Detail_CameraParams()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
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
	
		#[inline]
		fn set_focal(&mut self, val: f64) {
			let ret = unsafe { sys::cv_detail_CameraParams_propFocal_double(self.as_raw_mut_Detail_CameraParams(), val) };
			ret
		}
		
		#[inline]
		fn set_aspect(&mut self, val: f64) {
			let ret = unsafe { sys::cv_detail_CameraParams_propAspect_double(self.as_raw_mut_Detail_CameraParams(), val) };
			ret
		}
		
		#[inline]
		fn set_ppx(&mut self, val: f64) {
			let ret = unsafe { sys::cv_detail_CameraParams_propPpx_double(self.as_raw_mut_Detail_CameraParams(), val) };
			ret
		}
		
		#[inline]
		fn set_ppy(&mut self, val: f64) {
			let ret = unsafe { sys::cv_detail_CameraParams_propPpy_double(self.as_raw_mut_Detail_CameraParams(), val) };
			ret
		}
		
		#[inline]
		fn set_r(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_detail_CameraParams_propR_Mat(self.as_raw_mut_Detail_CameraParams(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_t(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_detail_CameraParams_propT_Mat(self.as_raw_mut_Detail_CameraParams(), val.as_raw_mut_Mat()) };
			ret
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
	
	impl Detail_CameraParams {
		#[inline]
		pub fn default() -> Result<crate::stitching::Detail_CameraParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CameraParams_CameraParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_CameraParams::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn copy(other: &crate::stitching::Detail_CameraParams) -> Result<crate::stitching::Detail_CameraParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CameraParams_CameraParams_const_CameraParamsR(other.as_raw_Detail_CameraParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_CameraParams::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for Detail_CameraParams {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_detail_CameraParams_implicitClone_const(self.as_raw_Detail_CameraParams())) }
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
	
	/// Constant methods for [crate::stitching::Detail_ChannelsCompensator]
	pub trait Detail_ChannelsCompensatorTraitConst: crate::stitching::Detail_ExposureCompensatorTraitConst {
		fn as_raw_Detail_ChannelsCompensator(&self) -> *const c_void;
	
		#[inline]
		fn get_similarity_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_getSimilarityThreshold_const(self.as_raw_Detail_ChannelsCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn gains(&self) -> Result<core::Vector<core::Scalar>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_gains_const(self.as_raw_Detail_ChannelsCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Scalar>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_ChannelsCompensator]
	pub trait Detail_ChannelsCompensatorTrait: crate::stitching::Detail_ChannelsCompensatorTraitConst + crate::stitching::Detail_ExposureCompensatorTrait {
		fn as_raw_mut_Detail_ChannelsCompensator(&mut self) -> *mut c_void;
	
		#[inline]
		fn feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_ChannelsCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl core::ToInputOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
			input_output_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_ChannelsCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_getMatGains_vectorLMatGR(self.as_raw_mut_Detail_ChannelsCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_setMatGains_vectorLMatGR(self.as_raw_mut_Detail_ChannelsCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_nr_feeds(&mut self, nr_feeds: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_setNrFeeds_int(self.as_raw_mut_Detail_ChannelsCompensator(), nr_feeds, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nr_feeds(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_getNrFeeds(self.as_raw_mut_Detail_ChannelsCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_similarity_threshold(&mut self, similarity_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_setSimilarityThreshold_double(self.as_raw_mut_Detail_ChannelsCompensator(), similarity_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Exposure compensator which tries to remove exposure related artifacts by adjusting image
	/// intensities on each channel independently.
	pub struct Detail_ChannelsCompensator {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_ChannelsCompensator }
	
	impl Drop for Detail_ChannelsCompensator {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_ChannelsCompensator_delete(self.as_raw_mut_Detail_ChannelsCompensator()) };
		}
	}
	
	unsafe impl Send for Detail_ChannelsCompensator {}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for Detail_ChannelsCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for Detail_ChannelsCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTraitConst for Detail_ChannelsCompensator {
		#[inline] fn as_raw_Detail_ChannelsCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTrait for Detail_ChannelsCompensator {
		#[inline] fn as_raw_mut_Detail_ChannelsCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_ChannelsCompensator {
		/// ## C++ default parameters
		/// * nr_feeds: 1
		#[inline]
		pub fn new(nr_feeds: i32) -> Result<crate::stitching::Detail_ChannelsCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_ChannelsCompensator_int(nr_feeds, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_ChannelsCompensator::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * nr_feeds: 1
		#[inline]
		pub fn new_def() -> Result<crate::stitching::Detail_ChannelsCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ChannelsCompensator_ChannelsCompensator(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_ChannelsCompensator::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Detail_ChannelsCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_ChannelsCompensator_to_Detail_ExposureCompensator }
	
	impl std::fmt::Debug for Detail_ChannelsCompensator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_ChannelsCompensator")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_CompressedRectilinearPortraitProjector]
	pub trait Detail_CompressedRectilinearPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_CompressedRectilinearPortraitProjector(&self) -> *const c_void;
	
		#[inline]
		fn a(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propA_const(self.as_raw_Detail_CompressedRectilinearPortraitProjector()) };
			ret
		}
		
		#[inline]
		fn b(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propB_const(self.as_raw_Detail_CompressedRectilinearPortraitProjector()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearPortraitProjector]
	pub trait Detail_CompressedRectilinearPortraitProjectorTrait: crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_CompressedRectilinearPortraitProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_a(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propA_float(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), val) };
			ret
		}
		
		#[inline]
		fn set_b(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_propB_float(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), val) };
			ret
		}
		
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_CompressedRectilinearPortraitProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst for Detail_CompressedRectilinearPortraitProjector {
		#[inline] fn as_raw_Detail_CompressedRectilinearPortraitProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CompressedRectilinearPortraitProjectorTrait for Detail_CompressedRectilinearPortraitProjector {
		#[inline] fn as_raw_mut_Detail_CompressedRectilinearPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CompressedRectilinearPortraitProjector {
	}
	
	boxed_cast_base! { Detail_CompressedRectilinearPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CompressedRectilinearPortraitProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_CompressedRectilinearPortraitProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_CompressedRectilinearPortraitProjector")
				.field("a", &crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst::a(self))
				.field("b", &crate::stitching::Detail_CompressedRectilinearPortraitProjectorTraitConst::b(self))
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_CompressedRectilinearPortraitWarper]
	pub trait Detail_CompressedRectilinearPortraitWarperTraitConst {
		fn as_raw_Detail_CompressedRectilinearPortraitWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearPortraitWarper]
	pub trait Detail_CompressedRectilinearPortraitWarperTrait: crate::stitching::Detail_CompressedRectilinearPortraitWarperTraitConst {
		fn as_raw_mut_Detail_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_CompressedRectilinearPortraitWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CompressedRectilinearPortraitWarperTraitConst for Detail_CompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_Detail_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CompressedRectilinearPortraitWarperTrait for Detail_CompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_mut_Detail_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CompressedRectilinearPortraitWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait Detail_CompressedRectilinearProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_CompressedRectilinearProjector(&self) -> *const c_void;
	
		#[inline]
		fn a(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propA_const(self.as_raw_Detail_CompressedRectilinearProjector()) };
			ret
		}
		
		#[inline]
		fn b(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propB_const(self.as_raw_Detail_CompressedRectilinearProjector()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearProjector]
	pub trait Detail_CompressedRectilinearProjectorTrait: crate::stitching::Detail_CompressedRectilinearProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_CompressedRectilinearProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_a(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propA_float(self.as_raw_mut_Detail_CompressedRectilinearProjector(), val) };
			ret
		}
		
		#[inline]
		fn set_b(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_CompressedRectilinearProjector_propB_float(self.as_raw_mut_Detail_CompressedRectilinearProjector(), val) };
			ret
		}
		
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CompressedRectilinearProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_CompressedRectilinearProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CompressedRectilinearProjectorTraitConst for Detail_CompressedRectilinearProjector {
		#[inline] fn as_raw_Detail_CompressedRectilinearProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CompressedRectilinearProjectorTrait for Detail_CompressedRectilinearProjector {
		#[inline] fn as_raw_mut_Detail_CompressedRectilinearProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CompressedRectilinearProjector {
	}
	
	boxed_cast_base! { Detail_CompressedRectilinearProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CompressedRectilinearProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_CompressedRectilinearProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_CompressedRectilinearProjector")
				.field("a", &crate::stitching::Detail_CompressedRectilinearProjectorTraitConst::a(self))
				.field("b", &crate::stitching::Detail_CompressedRectilinearProjectorTraitConst::b(self))
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_CompressedRectilinearWarper]
	pub trait Detail_CompressedRectilinearWarperTraitConst {
		fn as_raw_Detail_CompressedRectilinearWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_CompressedRectilinearWarper]
	pub trait Detail_CompressedRectilinearWarperTrait: crate::stitching::Detail_CompressedRectilinearWarperTraitConst {
		fn as_raw_mut_Detail_CompressedRectilinearWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_CompressedRectilinearWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CompressedRectilinearWarperTraitConst for Detail_CompressedRectilinearWarper {
		#[inline] fn as_raw_Detail_CompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CompressedRectilinearWarperTrait for Detail_CompressedRectilinearWarper {
		#[inline] fn as_raw_mut_Detail_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CompressedRectilinearWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait Detail_CylindricalPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_CylindricalPortraitProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_CylindricalPortraitProjector]
	pub trait Detail_CylindricalPortraitProjectorTrait: crate::stitching::Detail_CylindricalPortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_CylindricalPortraitProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_CylindricalPortraitProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CylindricalPortraitProjectorTraitConst for Detail_CylindricalPortraitProjector {
		#[inline] fn as_raw_Detail_CylindricalPortraitProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CylindricalPortraitProjectorTrait for Detail_CylindricalPortraitProjector {
		#[inline] fn as_raw_mut_Detail_CylindricalPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CylindricalPortraitProjector {
	}
	
	boxed_cast_base! { Detail_CylindricalPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CylindricalPortraitProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_CylindricalPortraitProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_CylindricalPortraitProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_CylindricalPortraitWarper]
	pub trait Detail_CylindricalPortraitWarperTraitConst {
		fn as_raw_Detail_CylindricalPortraitWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_CylindricalPortraitWarper]
	pub trait Detail_CylindricalPortraitWarperTrait: crate::stitching::Detail_CylindricalPortraitWarperTraitConst {
		fn as_raw_mut_Detail_CylindricalPortraitWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_CylindricalPortraitWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CylindricalPortraitWarperTraitConst for Detail_CylindricalPortraitWarper {
		#[inline] fn as_raw_Detail_CylindricalPortraitWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CylindricalPortraitWarperTrait for Detail_CylindricalPortraitWarper {
		#[inline] fn as_raw_mut_Detail_CylindricalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CylindricalPortraitWarper {
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
	pub trait Detail_CylindricalProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_CylindricalProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_CylindricalProjector]
	pub trait Detail_CylindricalProjectorTrait: crate::stitching::Detail_CylindricalProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_CylindricalProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_CylindricalProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_CylindricalProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CylindricalProjectorTraitConst for Detail_CylindricalProjector {
		#[inline] fn as_raw_Detail_CylindricalProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CylindricalProjectorTrait for Detail_CylindricalProjector {
		#[inline] fn as_raw_mut_Detail_CylindricalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CylindricalProjector {
	}
	
	boxed_cast_base! { Detail_CylindricalProjector, crate::stitching::Detail_ProjectorBase, cv_detail_CylindricalProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_CylindricalProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_CylindricalProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_CylindricalWarper]
	pub trait Detail_CylindricalWarperTraitConst {
		fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_CylindricalWarper]
	pub trait Detail_CylindricalWarperTrait: crate::stitching::Detail_CylindricalWarperTraitConst {
		fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void;
	
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
	pub struct Detail_CylindricalWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_CylindricalWarperTraitConst for Detail_CylindricalWarper {
		#[inline] fn as_raw_Detail_CylindricalWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CylindricalWarperTrait for Detail_CylindricalWarper {
		#[inline] fn as_raw_mut_Detail_CylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CylindricalWarper {
		/// Construct an instance of the cylindrical warper class.
		/// 
		/// ## Parameters
		/// * scale: Projected image scale multiplier
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
	pub trait Detail_CylindricalWarperGpuTraitConst: crate::stitching::Detail_CylindricalWarperTraitConst {
		fn as_raw_Detail_CylindricalWarperGpu(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_CylindricalWarperGpu]
	pub trait Detail_CylindricalWarperGpuTrait: crate::stitching::Detail_CylindricalWarperGpuTraitConst + crate::stitching::Detail_CylindricalWarperTrait {
		fn as_raw_mut_Detail_CylindricalWarperGpu(&mut self) -> *mut c_void;
	
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		
		#[inline]
		fn build_maps_1(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_1(&mut self, src: &core::GpuMat, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_CylindricalWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_CylindricalWarperGpu {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_RotationWarperTraitConst for Detail_CylindricalWarperGpu {
		#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_RotationWarperTrait for Detail_CylindricalWarperGpu {
		#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_CylindricalWarperGpuTraitConst for Detail_CylindricalWarperGpu {
		#[inline] fn as_raw_Detail_CylindricalWarperGpu(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_CylindricalWarperGpuTrait for Detail_CylindricalWarperGpu {
		#[inline] fn as_raw_mut_Detail_CylindricalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_CylindricalWarperGpu {
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
	pub trait Detail_DisjointSetsTraitConst {
		fn as_raw_Detail_DisjointSets(&self) -> *const c_void;
	
		#[inline]
		fn parent(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_detail_DisjointSets_propParent_const(self.as_raw_Detail_DisjointSets()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
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
	
		#[inline]
		fn set_parent(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_detail_DisjointSets_propParent_vectorLintG(self.as_raw_mut_Detail_DisjointSets(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		#[inline]
		fn set_size(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_detail_DisjointSets_propSize_vectorLintG(self.as_raw_mut_Detail_DisjointSets(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		#[inline]
		fn create_one_elem_sets(&mut self, elem_count: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DisjointSets_createOneElemSets_int(self.as_raw_mut_Detail_DisjointSets(), elem_count, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn find_set_by_elem(&mut self, elem: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DisjointSets_findSetByElem_int(self.as_raw_mut_Detail_DisjointSets(), elem, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn merge_sets(&mut self, set1: i32, set2: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DisjointSets_mergeSets_int_int(self.as_raw_mut_Detail_DisjointSets(), set1, set2, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_DisjointSets {
		ptr: *mut c_void
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
	
	impl Detail_DisjointSets {
		/// ## C++ default parameters
		/// * elem_count: 0
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
	pub trait Detail_DpSeamFinderTraitConst: crate::stitching::Detail_SeamFinderTraitConst {
		fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void;
	
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
	
		#[inline]
		fn set_cost_function(&mut self, val: crate::stitching::Detail_DpSeamFinder_CostFunction) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DpSeamFinder_setCostFunction_CostFunction(self.as_raw_mut_Detail_DpSeamFinder(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_cost_function_1(&mut self, val: &str) -> Result<()> {
			extern_container_arg!(mut val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DpSeamFinder_setCostFunction_String(self.as_raw_mut_Detail_DpSeamFinder(), val.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DpSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_DpSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_DpSeamFinder {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_DpSeamFinderTraitConst for Detail_DpSeamFinder {
		#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_DpSeamFinderTrait for Detail_DpSeamFinder {
		#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_DpSeamFinder {
		/// ## C++ default parameters
		/// * cost_func: COLOR
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
		#[inline]
		pub fn new_def() -> Result<crate::stitching::Detail_DpSeamFinder> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DpSeamFinder_DpSeamFinder(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_DpSeamFinder::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn new_1(cost_func: &str) -> Result<crate::stitching::Detail_DpSeamFinder> {
			extern_container_arg!(mut cost_func);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_DpSeamFinder_DpSeamFinder_String(cost_func.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
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
	pub struct Detail_Estimator {
		ptr: *mut c_void
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
		#[inline]
		fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl core::ToInputOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
			input_output_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_ExposureCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mat_gains(&mut self, unnamed: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ExposureCompensator_getMatGains_vectorLMatGR(self.as_raw_mut_Detail_ExposureCompensator(), unnamed.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mat_gains(&mut self, unnamed: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ExposureCompensator_setMatGains_vectorLMatGR(self.as_raw_mut_Detail_ExposureCompensator(), unnamed.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_update_gain(&mut self, b: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ExposureCompensator_setUpdateGain_bool(self.as_raw_mut_Detail_ExposureCompensator(), b, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_update_gain(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ExposureCompensator_getUpdateGain(self.as_raw_mut_Detail_ExposureCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base class for all exposure compensators.
	pub struct Detail_ExposureCompensator {
		ptr: *mut c_void
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
	
	impl Detail_ExposureCompensator {
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
	
	boxed_cast_descendant! { Detail_ExposureCompensator, crate::stitching::Detail_BlocksChannelsCompensator, cv_detail_ExposureCompensator_to_Detail_BlocksChannelsCompensator }
	
	boxed_cast_descendant! { Detail_ExposureCompensator, crate::stitching::Detail_BlocksCompensator, cv_detail_ExposureCompensator_to_Detail_BlocksCompensator }
	
	boxed_cast_descendant! { Detail_ExposureCompensator, crate::stitching::Detail_BlocksGainCompensator, cv_detail_ExposureCompensator_to_Detail_BlocksGainCompensator }
	
	boxed_cast_descendant! { Detail_ExposureCompensator, crate::stitching::Detail_ChannelsCompensator, cv_detail_ExposureCompensator_to_Detail_ChannelsCompensator }
	
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
	pub trait Detail_FeatherBlenderTraitConst: crate::stitching::Detail_BlenderTraitConst {
		fn as_raw_Detail_FeatherBlender(&self) -> *const c_void;
	
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
	
		#[inline]
		fn set_sharpness(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_FeatherBlender_setSharpness_float(self.as_raw_mut_Detail_FeatherBlender(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn prepare(&mut self, dst_roi: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_FeatherBlender_prepare_Rect(self.as_raw_mut_Detail_FeatherBlender(), dst_roi.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn feed(&mut self, img: &impl core::ToInputArray, mask: &impl core::ToInputArray, tl: core::Point) -> Result<()> {
			input_array_arg!(img);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_FeatherBlender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), tl.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn blend(&mut self, dst: &mut impl core::ToInputOutputArray, dst_mask: &mut impl core::ToInputOutputArray) -> Result<()> {
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
	pub struct Detail_FeatherBlender {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_FeatherBlenderTraitConst for Detail_FeatherBlender {
		#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_FeatherBlenderTrait for Detail_FeatherBlender {
		#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_FeatherBlender {
		/// ## C++ default parameters
		/// * sharpness: 0.02f
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
	
	/// Constant methods for [crate::stitching::Detail_FeaturesMatcher]
	pub trait Detail_FeaturesMatcherTraitConst {
		fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void;
	
		/// ## Returns
		/// True, if it's possible to use the same matcher instance in parallel, false otherwise
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
		#[inline]
		fn apply(&mut self, features1: &crate::stitching::Detail_ImageFeatures, features2: &crate::stitching::Detail_ImageFeatures, matches_info: &mut crate::stitching::Detail_MatchesInfo) -> Result<()> {
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
		#[inline]
		fn apply2(&mut self, features: &core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>, mask: &core::UMat) -> Result<()> {
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
		/// This alternative version of [apply2] function uses the following default values for its arguments:
		/// * mask: cv::UMat()
		#[inline]
		fn apply2_def(&mut self, features: &core::Vector<crate::stitching::Detail_ImageFeatures>, pairwise_matches: &mut core::Vector<crate::stitching::Detail_MatchesInfo>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_FeaturesMatcher_operator___const_vectorLImageFeaturesGR_vectorLMatchesInfoGR(self.as_raw_mut_Detail_FeaturesMatcher(), features.as_raw_VectorOfDetail_ImageFeatures(), pairwise_matches.as_raw_mut_VectorOfDetail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Frees unused memory allocated before if there is any.
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
	pub struct Detail_FeaturesMatcher {
		ptr: *mut c_void
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
	pub trait Detail_FisheyeProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_FisheyeProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_FisheyeProjector]
	pub trait Detail_FisheyeProjectorTrait: crate::stitching::Detail_FisheyeProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_FisheyeProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_FisheyeProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_FisheyeProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_FisheyeProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_FisheyeProjectorTraitConst for Detail_FisheyeProjector {
		#[inline] fn as_raw_Detail_FisheyeProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_FisheyeProjectorTrait for Detail_FisheyeProjector {
		#[inline] fn as_raw_mut_Detail_FisheyeProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_FisheyeProjector {
	}
	
	boxed_cast_base! { Detail_FisheyeProjector, crate::stitching::Detail_ProjectorBase, cv_detail_FisheyeProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_FisheyeProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_FisheyeProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_FisheyeWarper]
	pub trait Detail_FisheyeWarperTraitConst {
		fn as_raw_Detail_FisheyeWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_FisheyeWarper]
	pub trait Detail_FisheyeWarperTrait: crate::stitching::Detail_FisheyeWarperTraitConst {
		fn as_raw_mut_Detail_FisheyeWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_FisheyeWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_FisheyeWarperTraitConst for Detail_FisheyeWarper {
		#[inline] fn as_raw_Detail_FisheyeWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_FisheyeWarperTrait for Detail_FisheyeWarper {
		#[inline] fn as_raw_mut_Detail_FisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_FisheyeWarper {
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
	pub trait Detail_GainCompensatorTraitConst: crate::stitching::Detail_ExposureCompensatorTraitConst {
		fn as_raw_Detail_GainCompensator(&self) -> *const c_void;
	
		#[inline]
		fn get_similarity_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_getSimilarityThreshold_const(self.as_raw_Detail_GainCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_GainCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn single_feed(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>, masks: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_singleFeed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_GainCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), masks.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn apply(&mut self, index: i32, corner: core::Point, image: &mut impl core::ToInputOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
			input_output_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_GainCompensator(), index, corner.opencv_as_extern(), image.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_getMatGains_vectorLMatGR(self.as_raw_mut_Detail_GainCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_setMatGains_vectorLMatGR(self.as_raw_mut_Detail_GainCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_nr_feeds(&mut self, nr_feeds: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_setNrFeeds_int(self.as_raw_mut_Detail_GainCompensator(), nr_feeds, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nr_feeds(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_getNrFeeds(self.as_raw_mut_Detail_GainCompensator(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_similarity_threshold(&mut self, similarity_threshold: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_setSimilarityThreshold_double(self.as_raw_mut_Detail_GainCompensator(), similarity_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn prepare_similarity_mask(&mut self, corners: &core::Vector<core::Point>, images: &core::Vector<core::UMat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_prepareSimilarityMask_const_vectorLPointGR_const_vectorLUMatGR(self.as_raw_mut_Detail_GainCompensator(), corners.as_raw_VectorOfPoint(), images.as_raw_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Exposure compensator which tries to remove exposure related artifacts by adjusting image
	/// intensities, see [BL07](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_BL07) and [WJ10](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_WJ10) for details.
	pub struct Detail_GainCompensator {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_GainCompensatorTraitConst for Detail_GainCompensator {
		#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_GainCompensatorTrait for Detail_GainCompensator {
		#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_GainCompensator {
		#[inline]
		pub fn default() -> Result<crate::stitching::Detail_GainCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_GainCompensator(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_GainCompensator::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn new(nr_feeds: i32) -> Result<crate::stitching::Detail_GainCompensator> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GainCompensator_GainCompensator_int(nr_feeds, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_GainCompensator::opencv_from_extern(ret) };
			Ok(ret)
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
	
	/// Constant methods for [crate::stitching::Detail_Graph]
	pub trait Detail_GraphTraitConst {
		fn as_raw_Detail_Graph(&self) -> *const c_void;
	
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
	
		#[inline]
		fn create(&mut self, num_vertices: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_Graph_create_int(self.as_raw_mut_Detail_Graph(), num_vertices, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn add_edge(&mut self, from: i32, to: i32, weight: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_Graph_addEdge_int_int_float(self.as_raw_mut_Detail_Graph(), from, to, weight, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_Graph {
		ptr: *mut c_void
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
	
	impl Detail_Graph {
		/// ## C++ default parameters
		/// * num_vertices: 0
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
	pub trait Detail_GraphCutSeamFinderTraitConst: crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst + crate::stitching::Detail_SeamFinderTraitConst {
		fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_GraphCutSeamFinder]
	pub trait Detail_GraphCutSeamFinderTrait: crate::stitching::Detail_GraphCutSeamFinderBaseTrait + crate::stitching::Detail_GraphCutSeamFinderTraitConst + crate::stitching::Detail_SeamFinderTrait {
		fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void;
	
		#[inline]
		fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_GraphCutSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Minimum graph cut-based seam estimator. See details in [V03](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_V03) .
	pub struct Detail_GraphCutSeamFinder {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_SeamFinderTraitConst for Detail_GraphCutSeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for Detail_GraphCutSeamFinder {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTraitConst for Detail_GraphCutSeamFinder {
		#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTrait for Detail_GraphCutSeamFinder {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_GraphCutSeamFinder {
		/// ## C++ default parameters
		/// * cost_type: COST_COLOR_GRAD
		/// * terminal_cost: 10000.f
		/// * bad_region_penalty: 1000.f
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
		#[inline]
		pub fn new_def() -> Result<crate::stitching::Detail_GraphCutSeamFinder> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinder_GraphCutSeamFinder(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_GraphCutSeamFinder::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * terminal_cost: 10000.f
		/// * bad_region_penalty: 1000.f
		#[inline]
		pub fn new_1(cost_type: &str, terminal_cost: f32, bad_region_penalty: f32) -> Result<crate::stitching::Detail_GraphCutSeamFinder> {
			extern_container_arg!(mut cost_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String_float_float(cost_type.opencv_as_extern_mut(), terminal_cost, bad_region_penalty, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_GraphCutSeamFinder::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * terminal_cost: 10000.f
		/// * bad_region_penalty: 1000.f
		#[inline]
		pub fn new_def_1(cost_type: &str) -> Result<crate::stitching::Detail_GraphCutSeamFinder> {
			extern_container_arg!(mut cost_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String(cost_type.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
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
	pub trait Detail_GraphCutSeamFinderBaseTraitConst {
		fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_GraphCutSeamFinderBase]
	pub trait Detail_GraphCutSeamFinderBaseTrait: crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst {
		fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void;
	
	}
	
	/// Base class for all minimum graph-cut-based seam estimators.
	pub struct Detail_GraphCutSeamFinderBase {
		ptr: *mut c_void
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
	
	impl Detail_GraphCutSeamFinderBase {
	}
	
	impl std::fmt::Debug for Detail_GraphCutSeamFinderBase {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_GraphCutSeamFinderBase")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_GraphCutSeamFinderGpu]
	pub trait Detail_GraphCutSeamFinderGpuTraitConst: crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst + crate::stitching::Detail_PairwiseSeamFinderTraitConst {
		fn as_raw_Detail_GraphCutSeamFinderGpu(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_GraphCutSeamFinderGpu]
	pub trait Detail_GraphCutSeamFinderGpuTrait: crate::stitching::Detail_GraphCutSeamFinderBaseTrait + crate::stitching::Detail_GraphCutSeamFinderGpuTraitConst + crate::stitching::Detail_PairwiseSeamFinderTrait {
		fn as_raw_mut_Detail_GraphCutSeamFinderGpu(&mut self) -> *mut c_void;
	
		#[inline]
		fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinderGpu_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_GraphCutSeamFinderGpu(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn find_in_pair(&mut self, first: size_t, second: size_t, roi: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinderGpu_findInPair_size_t_size_t_Rect(self.as_raw_mut_Detail_GraphCutSeamFinderGpu(), first, second, roi.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_GraphCutSeamFinderGpu {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_GraphCutSeamFinderGpu }
	
	impl Drop for Detail_GraphCutSeamFinderGpu {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_GraphCutSeamFinderGpu_delete(self.as_raw_mut_Detail_GraphCutSeamFinderGpu()) };
		}
	}
	
	unsafe impl Send for Detail_GraphCutSeamFinderGpu {}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTrait for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderTraitConst for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderGpuTraitConst for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_Detail_GraphCutSeamFinderGpu(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderGpuTrait for Detail_GraphCutSeamFinderGpu {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_GraphCutSeamFinderGpu {
		/// ## C++ default parameters
		/// * cost_type: COST_COLOR_GRAD
		/// * terminal_cost: 10000.f
		/// * bad_region_penalty: 1000.f
		#[inline]
		pub fn new(cost_type: i32, terminal_cost: f32, bad_region_penalty: f32) -> Result<crate::stitching::Detail_GraphCutSeamFinderGpu> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinderGpu_GraphCutSeamFinderGpu_int_float_float(cost_type, terminal_cost, bad_region_penalty, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_GraphCutSeamFinderGpu::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * cost_type: COST_COLOR_GRAD
		/// * terminal_cost: 10000.f
		/// * bad_region_penalty: 1000.f
		#[inline]
		pub fn new_def() -> Result<crate::stitching::Detail_GraphCutSeamFinderGpu> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphCutSeamFinderGpu_GraphCutSeamFinderGpu(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_GraphCutSeamFinderGpu::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Detail_GraphCutSeamFinderGpu, crate::stitching::Detail_GraphCutSeamFinderBase, cv_detail_GraphCutSeamFinderGpu_to_Detail_GraphCutSeamFinderBase }
	
	boxed_cast_base! { Detail_GraphCutSeamFinderGpu, crate::stitching::Detail_PairwiseSeamFinder, cv_detail_GraphCutSeamFinderGpu_to_Detail_PairwiseSeamFinder }
	
	boxed_cast_base! { Detail_GraphCutSeamFinderGpu, crate::stitching::Detail_SeamFinder, cv_detail_GraphCutSeamFinderGpu_to_Detail_SeamFinder }
	
	impl std::fmt::Debug for Detail_GraphCutSeamFinderGpu {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_GraphCutSeamFinderGpu")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_GraphEdge]
	pub trait Detail_GraphEdgeTraitConst {
		fn as_raw_Detail_GraphEdge(&self) -> *const c_void;
	
		#[inline]
		fn from(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_GraphEdge_propFrom_const(self.as_raw_Detail_GraphEdge()) };
			ret
		}
		
		#[inline]
		fn to(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_GraphEdge_propTo_const(self.as_raw_Detail_GraphEdge()) };
			ret
		}
		
		#[inline]
		fn weight(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_GraphEdge_propWeight_const(self.as_raw_Detail_GraphEdge()) };
			ret
		}
		
		#[inline]
		fn less_than(&self, other: &crate::stitching::Detail_GraphEdge) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_GraphEdge_operatorL_const_const_GraphEdgeR(self.as_raw_Detail_GraphEdge(), other.as_raw_Detail_GraphEdge(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn greater_than(&self, other: &crate::stitching::Detail_GraphEdge) -> Result<bool> {
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
	
		#[inline]
		fn set_from(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_GraphEdge_propFrom_int(self.as_raw_mut_Detail_GraphEdge(), val) };
			ret
		}
		
		#[inline]
		fn set_to(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_GraphEdge_propTo_int(self.as_raw_mut_Detail_GraphEdge(), val) };
			ret
		}
		
		#[inline]
		fn set_weight(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_GraphEdge_propWeight_float(self.as_raw_mut_Detail_GraphEdge(), val) };
			ret
		}
		
	}
	
	pub struct Detail_GraphEdge {
		ptr: *mut c_void
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
	
	impl Detail_GraphEdge {
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
	pub trait Detail_HomographyBasedEstimatorTraitConst: crate::stitching::Detail_EstimatorTraitConst {
		fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_HomographyBasedEstimator]
	pub trait Detail_HomographyBasedEstimatorTrait: crate::stitching::Detail_EstimatorTrait + crate::stitching::Detail_HomographyBasedEstimatorTraitConst {
		fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void;
	
	}
	
	/// Homography based rotation estimator.
	pub struct Detail_HomographyBasedEstimator {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTraitConst for Detail_HomographyBasedEstimator {
		#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTrait for Detail_HomographyBasedEstimator {
		#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_HomographyBasedEstimator {
		/// ## C++ default parameters
		/// * is_focals_estimated: false
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
	pub trait Detail_ImageFeaturesTraitConst {
		fn as_raw_Detail_ImageFeatures(&self) -> *const c_void;
	
		#[inline]
		fn img_idx(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_ImageFeatures_propImg_idx_const(self.as_raw_Detail_ImageFeatures()) };
			ret
		}
		
		#[inline]
		fn img_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ImageFeatures_propImg_size_const(self.as_raw_Detail_ImageFeatures(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn keypoints(&self) -> core::Vector<core::KeyPoint> {
			let ret = unsafe { sys::cv_detail_ImageFeatures_propKeypoints_const(self.as_raw_Detail_ImageFeatures()) };
			let ret = unsafe { core::Vector::<core::KeyPoint>::opencv_from_extern(ret) };
			ret
		}
		
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
	
		#[inline]
		fn set_img_idx(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_ImageFeatures_propImg_idx_int(self.as_raw_mut_Detail_ImageFeatures(), val) };
			ret
		}
		
		#[inline]
		fn set_img_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_detail_ImageFeatures_propImg_size_Size(self.as_raw_mut_Detail_ImageFeatures(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_keypoints(&mut self, mut val: core::Vector<core::KeyPoint>) {
			let ret = unsafe { sys::cv_detail_ImageFeatures_propKeypoints_vectorLKeyPointG(self.as_raw_mut_Detail_ImageFeatures(), val.as_raw_mut_VectorOfKeyPoint()) };
			ret
		}
		
		#[inline]
		fn set_descriptors(&mut self, mut val: core::UMat) {
			let ret = unsafe { sys::cv_detail_ImageFeatures_propDescriptors_UMat(self.as_raw_mut_Detail_ImageFeatures(), val.as_raw_mut_UMat()) };
			ret
		}
		
		#[inline]
		fn get_keypoints(&mut self) -> Result<core::Vector<core::KeyPoint>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ImageFeatures_getKeypoints(self.as_raw_mut_Detail_ImageFeatures(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::KeyPoint>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Structure containing image keypoints and descriptors.
	pub struct Detail_ImageFeatures {
		ptr: *mut c_void
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
	
	impl Detail_ImageFeatures {
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
	pub trait Detail_MatchesInfoTraitConst {
		fn as_raw_Detail_MatchesInfo(&self) -> *const c_void;
	
		#[inline]
		fn src_img_idx(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propSrc_img_idx_const(self.as_raw_Detail_MatchesInfo()) };
			ret
		}
		
		/// Images indices (optional)
		#[inline]
		fn dst_img_idx(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propDst_img_idx_const(self.as_raw_Detail_MatchesInfo()) };
			ret
		}
		
		#[inline]
		fn matches(&self) -> core::Vector<core::DMatch> {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propMatches_const(self.as_raw_Detail_MatchesInfo()) };
			let ret = unsafe { core::Vector::<core::DMatch>::opencv_from_extern(ret) };
			ret
		}
		
		/// Geometrically consistent matches mask
		#[inline]
		fn inliers_mask(&self) -> core::Vector<u8> {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propInliers_mask_const(self.as_raw_Detail_MatchesInfo()) };
			let ret = unsafe { core::Vector::<u8>::opencv_from_extern(ret) };
			ret
		}
		
		/// Number of geometrically consistent matches
		#[inline]
		fn num_inliers(&self) -> i32 {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propNum_inliers_const(self.as_raw_Detail_MatchesInfo()) };
			ret
		}
		
		/// Estimated transformation
		#[inline]
		fn h(&self) -> core::Mat {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propH_const(self.as_raw_Detail_MatchesInfo()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		/// Confidence two images are from the same panorama
		#[inline]
		fn confidence(&self) -> f64 {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propConfidence_const(self.as_raw_Detail_MatchesInfo()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_MatchesInfo]
	pub trait Detail_MatchesInfoTrait: crate::stitching::Detail_MatchesInfoTraitConst {
		fn as_raw_mut_Detail_MatchesInfo(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_src_img_idx(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propSrc_img_idx_int(self.as_raw_mut_Detail_MatchesInfo(), val) };
			ret
		}
		
		/// Images indices (optional)
		#[inline]
		fn set_dst_img_idx(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propDst_img_idx_int(self.as_raw_mut_Detail_MatchesInfo(), val) };
			ret
		}
		
		#[inline]
		fn set_matches(&mut self, mut val: core::Vector<core::DMatch>) {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propMatches_vectorLDMatchG(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_mut_VectorOfDMatch()) };
			ret
		}
		
		/// Geometrically consistent matches mask
		#[inline]
		fn set_inliers_mask(&mut self, mut val: core::Vector<u8>) {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propInliers_mask_vectorLunsigned_charG(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_mut_VectorOfu8()) };
			ret
		}
		
		/// Number of geometrically consistent matches
		#[inline]
		fn set_num_inliers(&mut self, val: i32) {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propNum_inliers_int(self.as_raw_mut_Detail_MatchesInfo(), val) };
			ret
		}
		
		/// Estimated transformation
		#[inline]
		fn set_h(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propH_Mat(self.as_raw_mut_Detail_MatchesInfo(), val.as_raw_mut_Mat()) };
			ret
		}
		
		/// Confidence two images are from the same panorama
		#[inline]
		fn set_confidence(&mut self, val: f64) {
			let ret = unsafe { sys::cv_detail_MatchesInfo_propConfidence_double(self.as_raw_mut_Detail_MatchesInfo(), val) };
			ret
		}
		
		#[inline]
		fn get_matches(&mut self) -> Result<core::Vector<core::DMatch>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MatchesInfo_getMatches(self.as_raw_mut_Detail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::DMatch>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_inliers(&mut self) -> Result<core::Vector<u8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MatchesInfo_getInliers(self.as_raw_mut_Detail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<u8>::opencv_from_extern(ret) };
			Ok(ret)
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
	
	impl Detail_MatchesInfo {
		#[inline]
		pub fn default() -> Result<crate::stitching::Detail_MatchesInfo> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MatchesInfo_MatchesInfo(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_MatchesInfo::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn copy(other: &crate::stitching::Detail_MatchesInfo) -> Result<crate::stitching::Detail_MatchesInfo> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(other.as_raw_Detail_MatchesInfo(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::stitching::Detail_MatchesInfo::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for Detail_MatchesInfo {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_detail_MatchesInfo_implicitClone_const(self.as_raw_Detail_MatchesInfo())) }
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
	pub trait Detail_MercatorProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_MercatorProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_MercatorProjector]
	pub trait Detail_MercatorProjectorTrait: crate::stitching::Detail_MercatorProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_MercatorProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_MercatorProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_MercatorProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_MercatorProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_MercatorProjectorTraitConst for Detail_MercatorProjector {
		#[inline] fn as_raw_Detail_MercatorProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_MercatorProjectorTrait for Detail_MercatorProjector {
		#[inline] fn as_raw_mut_Detail_MercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_MercatorProjector {
	}
	
	boxed_cast_base! { Detail_MercatorProjector, crate::stitching::Detail_ProjectorBase, cv_detail_MercatorProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_MercatorProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_MercatorProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_MercatorWarper]
	pub trait Detail_MercatorWarperTraitConst {
		fn as_raw_Detail_MercatorWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_MercatorWarper]
	pub trait Detail_MercatorWarperTrait: crate::stitching::Detail_MercatorWarperTraitConst {
		fn as_raw_mut_Detail_MercatorWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_MercatorWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_MercatorWarperTraitConst for Detail_MercatorWarper {
		#[inline] fn as_raw_Detail_MercatorWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_MercatorWarperTrait for Detail_MercatorWarper {
		#[inline] fn as_raw_mut_Detail_MercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_MercatorWarper {
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
	pub trait Detail_MultiBandBlenderTraitConst: crate::stitching::Detail_BlenderTraitConst {
		fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void;
	
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
	
		#[inline]
		fn set_num_bands(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MultiBandBlender_setNumBands_int(self.as_raw_mut_Detail_MultiBandBlender(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn prepare(&mut self, dst_roi: core::Rect) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MultiBandBlender_prepare_Rect(self.as_raw_mut_Detail_MultiBandBlender(), dst_roi.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn feed(&mut self, img: &impl core::ToInputArray, mask: &impl core::ToInputArray, tl: core::Point) -> Result<()> {
			input_array_arg!(img);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_Detail_MultiBandBlender(), img.as_raw__InputArray(), mask.as_raw__InputArray(), tl.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn blend(&mut self, dst: &mut impl core::ToInputOutputArray, dst_mask: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_output_array_arg!(dst);
			input_output_array_arg!(dst_mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Detail_MultiBandBlender(), dst.as_raw__InputOutputArray(), dst_mask.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Blender which uses multi-band blending algorithm (see [BA83](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_BA83)).
	pub struct Detail_MultiBandBlender {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_MultiBandBlenderTraitConst for Detail_MultiBandBlender {
		#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_MultiBandBlenderTrait for Detail_MultiBandBlender {
		#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_MultiBandBlender {
		/// ## C++ default parameters
		/// * try_gpu: false
		/// * num_bands: 5
		/// * weight_type: CV_32F
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
	pub trait Detail_NoBundleAdjusterTraitConst: crate::stitching::Detail_BundleAdjusterBaseTraitConst {
		fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_NoBundleAdjuster]
	pub trait Detail_NoBundleAdjusterTrait: crate::stitching::Detail_BundleAdjusterBaseTrait + crate::stitching::Detail_NoBundleAdjusterTraitConst {
		fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void;
	
	}
	
	/// Stub bundle adjuster that does nothing.
	pub struct Detail_NoBundleAdjuster {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_EstimatorTraitConst for Detail_NoBundleAdjuster {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for Detail_NoBundleAdjuster {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTraitConst for Detail_NoBundleAdjuster {
		#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTrait for Detail_NoBundleAdjuster {
		#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_NoBundleAdjuster {
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
	pub trait Detail_NoExposureCompensatorTraitConst: crate::stitching::Detail_ExposureCompensatorTraitConst {
		fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_NoExposureCompensator]
	pub trait Detail_NoExposureCompensatorTrait: crate::stitching::Detail_ExposureCompensatorTrait + crate::stitching::Detail_NoExposureCompensatorTraitConst {
		fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void;
	
		#[inline]
		fn feed(&mut self, unnamed: &core::Vector<core::Point>, unnamed_1: &core::Vector<core::UMat>, unnamed_2: &core::Vector<core::Tuple<(core::UMat, u8)>>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_NoExposureCompensator_feed_const_vectorLPointGR_const_vectorLUMatGR_const_vectorLpairLcv_UMat__unsigned_charGGR(self.as_raw_mut_Detail_NoExposureCompensator(), unnamed.as_raw_VectorOfPoint(), unnamed_1.as_raw_VectorOfUMat(), unnamed_2.as_raw_VectorOfTupleOfUMat_u8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn apply(&mut self, unnamed: i32, unnamed_1: core::Point, unnamed_2: &mut impl core::ToInputOutputArray, unnamed_3: &impl core::ToInputArray) -> Result<()> {
			input_output_array_arg!(unnamed_2);
			input_array_arg!(unnamed_3);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_Detail_NoExposureCompensator(), unnamed, unnamed_1.opencv_as_extern(), unnamed_2.as_raw__InputOutputArray(), unnamed_3.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_NoExposureCompensator_getMatGains_vectorLMatGR(self.as_raw_mut_Detail_NoExposureCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_mat_gains(&mut self, umv: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_NoExposureCompensator_setMatGains_vectorLMatGR(self.as_raw_mut_Detail_NoExposureCompensator(), umv.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Stub exposure compensator which does nothing.
	pub struct Detail_NoExposureCompensator {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_NoExposureCompensatorTraitConst for Detail_NoExposureCompensator {
		#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_NoExposureCompensatorTrait for Detail_NoExposureCompensator {
		#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_NoExposureCompensator {
	}
	
	boxed_cast_base! { Detail_NoExposureCompensator, crate::stitching::Detail_ExposureCompensator, cv_detail_NoExposureCompensator_to_Detail_ExposureCompensator }
	
	impl std::fmt::Debug for Detail_NoExposureCompensator {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_NoExposureCompensator")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_NoSeamFinder]
	pub trait Detail_NoSeamFinderTraitConst: crate::stitching::Detail_SeamFinderTraitConst {
		fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_NoSeamFinder]
	pub trait Detail_NoSeamFinderTrait: crate::stitching::Detail_NoSeamFinderTraitConst + crate::stitching::Detail_SeamFinderTrait {
		fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void;
	
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
	pub struct Detail_NoSeamFinder {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_NoSeamFinderTraitConst for Detail_NoSeamFinder {
		#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_NoSeamFinderTrait for Detail_NoSeamFinder {
		#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_NoSeamFinder {
	}
	
	boxed_cast_base! { Detail_NoSeamFinder, crate::stitching::Detail_SeamFinder, cv_detail_NoSeamFinder_to_Detail_SeamFinder }
	
	impl std::fmt::Debug for Detail_NoSeamFinder {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_NoSeamFinder")
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_PairwiseSeamFinder]
	pub trait Detail_PairwiseSeamFinderTraitConst: crate::stitching::Detail_SeamFinderTraitConst {
		fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PairwiseSeamFinder]
	pub trait Detail_PairwiseSeamFinderTrait: crate::stitching::Detail_PairwiseSeamFinderTraitConst + crate::stitching::Detail_SeamFinderTrait {
		fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void;
	
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
	pub struct Detail_PairwiseSeamFinder {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for Detail_PairwiseSeamFinder {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTrait for Detail_PairwiseSeamFinder {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PairwiseSeamFinder {
	}
	
	boxed_cast_descendant! { Detail_PairwiseSeamFinder, crate::stitching::Detail_GraphCutSeamFinderGpu, cv_detail_PairwiseSeamFinder_to_Detail_GraphCutSeamFinderGpu }
	
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
	pub trait Detail_PaniniPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_PaniniPortraitProjector(&self) -> *const c_void;
	
		#[inline]
		fn a(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propA_const(self.as_raw_Detail_PaniniPortraitProjector()) };
			ret
		}
		
		#[inline]
		fn b(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propB_const(self.as_raw_Detail_PaniniPortraitProjector()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_PaniniPortraitProjector]
	pub trait Detail_PaniniPortraitProjectorTrait: crate::stitching::Detail_PaniniPortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_PaniniPortraitProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_a(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propA_float(self.as_raw_mut_Detail_PaniniPortraitProjector(), val) };
			ret
		}
		
		#[inline]
		fn set_b(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_PaniniPortraitProjector_propB_float(self.as_raw_mut_Detail_PaniniPortraitProjector(), val) };
			ret
		}
		
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_PaniniPortraitProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PaniniPortraitProjectorTraitConst for Detail_PaniniPortraitProjector {
		#[inline] fn as_raw_Detail_PaniniPortraitProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PaniniPortraitProjectorTrait for Detail_PaniniPortraitProjector {
		#[inline] fn as_raw_mut_Detail_PaniniPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PaniniPortraitProjector {
	}
	
	boxed_cast_base! { Detail_PaniniPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PaniniPortraitProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_PaniniPortraitProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_PaniniPortraitProjector")
				.field("a", &crate::stitching::Detail_PaniniPortraitProjectorTraitConst::a(self))
				.field("b", &crate::stitching::Detail_PaniniPortraitProjectorTraitConst::b(self))
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_PaniniPortraitWarper]
	pub trait Detail_PaniniPortraitWarperTraitConst {
		fn as_raw_Detail_PaniniPortraitWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PaniniPortraitWarper]
	pub trait Detail_PaniniPortraitWarperTrait: crate::stitching::Detail_PaniniPortraitWarperTraitConst {
		fn as_raw_mut_Detail_PaniniPortraitWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_PaniniPortraitWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PaniniPortraitWarperTraitConst for Detail_PaniniPortraitWarper {
		#[inline] fn as_raw_Detail_PaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PaniniPortraitWarperTrait for Detail_PaniniPortraitWarper {
		#[inline] fn as_raw_mut_Detail_PaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PaniniPortraitWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait Detail_PaniniProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_PaniniProjector(&self) -> *const c_void;
	
		#[inline]
		fn a(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_PaniniProjector_propA_const(self.as_raw_Detail_PaniniProjector()) };
			ret
		}
		
		#[inline]
		fn b(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_PaniniProjector_propB_const(self.as_raw_Detail_PaniniProjector()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_PaniniProjector]
	pub trait Detail_PaniniProjectorTrait: crate::stitching::Detail_PaniniProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_PaniniProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_a(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_PaniniProjector_propA_float(self.as_raw_mut_Detail_PaniniProjector(), val) };
			ret
		}
		
		#[inline]
		fn set_b(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_PaniniProjector_propB_float(self.as_raw_mut_Detail_PaniniProjector(), val) };
			ret
		}
		
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PaniniProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_PaniniProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PaniniProjectorTraitConst for Detail_PaniniProjector {
		#[inline] fn as_raw_Detail_PaniniProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PaniniProjectorTrait for Detail_PaniniProjector {
		#[inline] fn as_raw_mut_Detail_PaniniProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PaniniProjector {
	}
	
	boxed_cast_base! { Detail_PaniniProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PaniniProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_PaniniProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_PaniniProjector")
				.field("a", &crate::stitching::Detail_PaniniProjectorTraitConst::a(self))
				.field("b", &crate::stitching::Detail_PaniniProjectorTraitConst::b(self))
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_PaniniWarper]
	pub trait Detail_PaniniWarperTraitConst {
		fn as_raw_Detail_PaniniWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PaniniWarper]
	pub trait Detail_PaniniWarperTrait: crate::stitching::Detail_PaniniWarperTraitConst {
		fn as_raw_mut_Detail_PaniniWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_PaniniWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PaniniWarperTraitConst for Detail_PaniniWarper {
		#[inline] fn as_raw_Detail_PaniniWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PaniniWarperTrait for Detail_PaniniWarper {
		#[inline] fn as_raw_mut_Detail_PaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PaniniWarper {
		/// ## C++ default parameters
		/// * a: 1
		/// * b: 1
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
	pub trait Detail_PlanePortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_PlanePortraitProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PlanePortraitProjector]
	pub trait Detail_PlanePortraitProjectorTrait: crate::stitching::Detail_PlanePortraitProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_PlanePortraitProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlanePortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlanePortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlanePortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlanePortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_PlanePortraitProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PlanePortraitProjectorTraitConst for Detail_PlanePortraitProjector {
		#[inline] fn as_raw_Detail_PlanePortraitProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PlanePortraitProjectorTrait for Detail_PlanePortraitProjector {
		#[inline] fn as_raw_mut_Detail_PlanePortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PlanePortraitProjector {
	}
	
	boxed_cast_base! { Detail_PlanePortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PlanePortraitProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_PlanePortraitProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_PlanePortraitProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_PlanePortraitWarper]
	pub trait Detail_PlanePortraitWarperTraitConst {
		fn as_raw_Detail_PlanePortraitWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PlanePortraitWarper]
	pub trait Detail_PlanePortraitWarperTrait: crate::stitching::Detail_PlanePortraitWarperTraitConst {
		fn as_raw_mut_Detail_PlanePortraitWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_PlanePortraitWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PlanePortraitWarperTraitConst for Detail_PlanePortraitWarper {
		#[inline] fn as_raw_Detail_PlanePortraitWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PlanePortraitWarperTrait for Detail_PlanePortraitWarper {
		#[inline] fn as_raw_mut_Detail_PlanePortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PlanePortraitWarper {
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
	pub trait Detail_PlaneProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_PlaneProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PlaneProjector]
	pub trait Detail_PlaneProjectorTrait: crate::stitching::Detail_PlaneProjectorTraitConst + crate::stitching::Detail_ProjectorBaseTrait {
		fn as_raw_mut_Detail_PlaneProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlaneProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_PlaneProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_PlaneProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PlaneProjectorTraitConst for Detail_PlaneProjector {
		#[inline] fn as_raw_Detail_PlaneProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PlaneProjectorTrait for Detail_PlaneProjector {
		#[inline] fn as_raw_mut_Detail_PlaneProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PlaneProjector {
	}
	
	boxed_cast_base! { Detail_PlaneProjector, crate::stitching::Detail_ProjectorBase, cv_detail_PlaneProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_PlaneProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_PlaneProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_PlaneWarper]
	pub trait Detail_PlaneWarperTraitConst {
		fn as_raw_Detail_PlaneWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PlaneWarper]
	pub trait Detail_PlaneWarperTrait: crate::stitching::Detail_PlaneWarperTraitConst {
		fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void;
	
		#[inline]
		fn warp_point(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_point_1(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			input_array_arg!(t);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_point_backward(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_point_backward_1(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			input_array_arg!(t);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			input_array_arg!(t);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn build_maps_1(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		
		#[inline]
		fn warp_1(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		
		#[inline]
		fn warp_roi(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_roi_1(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			input_array_arg!(t);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_PlaneWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Warper that maps an image onto the z = 1 plane.
	pub struct Detail_PlaneWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_PlaneWarperTraitConst for Detail_PlaneWarper {
		#[inline] fn as_raw_Detail_PlaneWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PlaneWarperTrait for Detail_PlaneWarper {
		#[inline] fn as_raw_mut_Detail_PlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PlaneWarper {
		/// Construct an instance of the plane warper class.
		/// 
		/// ## Parameters
		/// * scale: Projected image scale multiplier
		/// 
		/// ## C++ default parameters
		/// * scale: 1.f
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
	pub trait Detail_PlaneWarperGpuTraitConst: crate::stitching::Detail_PlaneWarperTraitConst {
		fn as_raw_Detail_PlaneWarperGpu(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_PlaneWarperGpu]
	pub trait Detail_PlaneWarperGpuTrait: crate::stitching::Detail_PlaneWarperGpuTraitConst + crate::stitching::Detail_PlaneWarperTrait {
		fn as_raw_mut_Detail_PlaneWarperGpu(&mut self) -> *mut c_void;
	
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn build_maps_1(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			input_array_arg!(t);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		
		#[inline]
		fn warp_1(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		
		#[inline]
		fn build_maps_2(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn build_maps_3(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			input_array_arg!(t);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_2(&mut self, src: &core::GpuMat, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_PlaneWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_3(&mut self, src: &core::GpuMat, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
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
	
	pub struct Detail_PlaneWarperGpu {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_RotationWarperTraitConst for Detail_PlaneWarperGpu {
		#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_RotationWarperTrait for Detail_PlaneWarperGpu {
		#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PlaneWarperGpuTraitConst for Detail_PlaneWarperGpu {
		#[inline] fn as_raw_Detail_PlaneWarperGpu(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_PlaneWarperGpuTrait for Detail_PlaneWarperGpu {
		#[inline] fn as_raw_mut_Detail_PlaneWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_PlaneWarperGpu {
		/// ## C++ default parameters
		/// * scale: 1.f
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
	pub trait Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_ProjectorBase(&self) -> *const c_void;
	
		#[inline]
		fn scale(&self) -> f32 {
			let ret = unsafe { sys::cv_detail_ProjectorBase_propScale_const(self.as_raw_Detail_ProjectorBase()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::stitching::Detail_ProjectorBase]
	pub trait Detail_ProjectorBaseTrait: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_mut_Detail_ProjectorBase(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_scale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_detail_ProjectorBase_propScale_float(self.as_raw_mut_Detail_ProjectorBase(), val) };
			ret
		}
		
		#[inline]
		fn k(&mut self) -> &mut [f32; 9] {
			let ret = unsafe { sys::cv_detail_ProjectorBase_propK(self.as_raw_mut_Detail_ProjectorBase()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		#[inline]
		fn rinv(&mut self) -> &mut [f32; 9] {
			let ret = unsafe { sys::cv_detail_ProjectorBase_propRinv(self.as_raw_mut_Detail_ProjectorBase()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		#[inline]
		fn r_kinv(&mut self) -> &mut [f32; 9] {
			let ret = unsafe { sys::cv_detail_ProjectorBase_propR_kinv(self.as_raw_mut_Detail_ProjectorBase()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		#[inline]
		fn k_rinv(&mut self) -> &mut [f32; 9] {
			let ret = unsafe { sys::cv_detail_ProjectorBase_propK_rinv(self.as_raw_mut_Detail_ProjectorBase()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		#[inline]
		fn t(&mut self) -> &mut [f32; 3] {
			let ret = unsafe { sys::cv_detail_ProjectorBase_propT(self.as_raw_mut_Detail_ProjectorBase()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		/// ## C++ default parameters
		/// * k: Mat::eye(3,3,CV_32F)
		/// * r: Mat::eye(3,3,CV_32F)
		/// * t: Mat::zeros(3,1,CV_32F)
		#[inline]
		fn set_camera_params(&mut self, k: &impl core::ToInputArray, r: &impl core::ToInputArray, t: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [set_camera_params] function uses the following default values for its arguments:
		/// * k: Mat::eye(3,3,CV_32F)
		/// * r: Mat::eye(3,3,CV_32F)
		/// * t: Mat::zeros(3,1,CV_32F)
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
	pub struct Detail_ProjectorBase {
		ptr: *mut c_void
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
	
	impl Detail_ProjectorBase {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_detail_ProjectorBase_defaultNew_const()) }
		}
		
	}
	
	impl Clone for Detail_ProjectorBase {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_detail_ProjectorBase_implicitClone_const(self.as_raw_Detail_ProjectorBase())) }
		}
	}
	
	impl std::fmt::Debug for Detail_ProjectorBase {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_ProjectorBase")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
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
	pub trait Detail_RotationWarperTraitConst {
		fn as_raw_Detail_RotationWarper(&self) -> *const c_void;
	
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
		#[inline]
		fn warp_point(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_RotationWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_point_backward(&mut self, pt: core::Point2f, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Point2f> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_RotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_RotationWarper(), &pt, k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		#[inline]
		fn warp_backward(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst_size: core::Size, dst: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(self.as_raw_mut_Detail_RotationWarper(), src.as_raw__InputArray(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst_size.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn warp_roi(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Detail_RotationWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct Detail_RotationWarper {
		ptr: *mut c_void
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
	pub struct Detail_SeamFinder {
		ptr: *mut c_void
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
	
	impl Detail_SeamFinder {
		#[inline]
		pub fn create_default(typ: i32) -> Result<core::Ptr<crate::stitching::Detail_SeamFinder>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SeamFinder_createDefault_int(typ, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::stitching::Detail_SeamFinder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_DpSeamFinder, cv_detail_SeamFinder_to_Detail_DpSeamFinder }
	
	boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_GraphCutSeamFinder, cv_detail_SeamFinder_to_Detail_GraphCutSeamFinder }
	
	boxed_cast_descendant! { Detail_SeamFinder, crate::stitching::Detail_GraphCutSeamFinderGpu, cv_detail_SeamFinder_to_Detail_GraphCutSeamFinderGpu }
	
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
	
	/// Constant methods for [crate::stitching::Detail_SphericalPortraitProjector]
	pub trait Detail_SphericalPortraitProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_SphericalPortraitProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_SphericalPortraitProjector]
	pub trait Detail_SphericalPortraitProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_SphericalPortraitProjectorTraitConst {
		fn as_raw_mut_Detail_SphericalPortraitProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalPortraitProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalPortraitProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalPortraitProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_SphericalPortraitProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_SphericalPortraitProjectorTraitConst for Detail_SphericalPortraitProjector {
		#[inline] fn as_raw_Detail_SphericalPortraitProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SphericalPortraitProjectorTrait for Detail_SphericalPortraitProjector {
		#[inline] fn as_raw_mut_Detail_SphericalPortraitProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_SphericalPortraitProjector {
	}
	
	boxed_cast_base! { Detail_SphericalPortraitProjector, crate::stitching::Detail_ProjectorBase, cv_detail_SphericalPortraitProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_SphericalPortraitProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_SphericalPortraitProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_SphericalPortraitWarper]
	pub trait Detail_SphericalPortraitWarperTraitConst {
		fn as_raw_Detail_SphericalPortraitWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_SphericalPortraitWarper]
	pub trait Detail_SphericalPortraitWarperTrait: crate::stitching::Detail_SphericalPortraitWarperTraitConst {
		fn as_raw_mut_Detail_SphericalPortraitWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_SphericalPortraitWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_SphericalPortraitWarperTraitConst for Detail_SphericalPortraitWarper {
		#[inline] fn as_raw_Detail_SphericalPortraitWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SphericalPortraitWarperTrait for Detail_SphericalPortraitWarper {
		#[inline] fn as_raw_mut_Detail_SphericalPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_SphericalPortraitWarper {
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
	pub trait Detail_SphericalProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_SphericalProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_SphericalProjector]
	pub trait Detail_SphericalProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_SphericalProjectorTraitConst {
		fn as_raw_mut_Detail_SphericalProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_SphericalProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_SphericalProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_SphericalProjectorTraitConst for Detail_SphericalProjector {
		#[inline] fn as_raw_Detail_SphericalProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SphericalProjectorTrait for Detail_SphericalProjector {
		#[inline] fn as_raw_mut_Detail_SphericalProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_SphericalProjector {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_detail_SphericalProjector_defaultNew_const()) }
		}
		
	}
	
	impl Clone for Detail_SphericalProjector {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_detail_SphericalProjector_implicitClone_const(self.as_raw_Detail_SphericalProjector())) }
		}
	}
	
	boxed_cast_base! { Detail_SphericalProjector, crate::stitching::Detail_ProjectorBase, cv_detail_SphericalProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_SphericalProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_SphericalProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
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
	pub trait Detail_SphericalWarperTraitConst {
		fn as_raw_Detail_SphericalWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_SphericalWarper]
	pub trait Detail_SphericalWarperTrait: crate::stitching::Detail_SphericalWarperTraitConst {
		fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void;
	
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarper(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
	pub struct Detail_SphericalWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_SphericalWarperTraitConst for Detail_SphericalWarper {
		#[inline] fn as_raw_Detail_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SphericalWarperTrait for Detail_SphericalWarper {
		#[inline] fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_SphericalWarper {
		/// Construct an instance of the spherical warper class.
		/// 
		/// ## Parameters
		/// * scale: Radius of the projected sphere, in pixels. An image spanning the
		///              whole sphere will have a width of 2 * scale * PI pixels.
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
	pub trait Detail_SphericalWarperGpuTraitConst: crate::stitching::Detail_SphericalWarperTraitConst {
		fn as_raw_Detail_SphericalWarperGpu(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_SphericalWarperGpu]
	pub trait Detail_SphericalWarperGpuTrait: crate::stitching::Detail_SphericalWarperGpuTraitConst + crate::stitching::Detail_SphericalWarperTrait {
		fn as_raw_mut_Detail_SphericalWarperGpu(&mut self) -> *mut c_void;
	
		#[inline]
		fn build_maps(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut impl core::ToOutputArray, ymap: &mut impl core::ToOutputArray) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			output_array_arg!(xmap);
			output_array_arg!(ymap);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Detail_SphericalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw__OutputArray(), ymap.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp(&mut self, src: &impl core::ToInputArray, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut impl core::ToOutputArray) -> Result<core::Point> {
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
		
		#[inline]
		fn build_maps_1(&mut self, src_size: core::Size, k: &impl core::ToInputArray, r: &impl core::ToInputArray, xmap: &mut core::GpuMat, ymap: &mut core::GpuMat) -> Result<core::Rect> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(self.as_raw_mut_Detail_SphericalWarperGpu(), src_size.opencv_as_extern(), k.as_raw__InputArray(), r.as_raw__InputArray(), xmap.as_raw_mut_GpuMat(), ymap.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn warp_1(&mut self, src: &core::GpuMat, k: &impl core::ToInputArray, r: &impl core::ToInputArray, interp_mode: i32, border_mode: i32, dst: &mut core::GpuMat) -> Result<core::Point> {
			input_array_arg!(k);
			input_array_arg!(r);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(self.as_raw_mut_Detail_SphericalWarperGpu(), src.as_raw_GpuMat(), k.as_raw__InputArray(), r.as_raw__InputArray(), interp_mode, border_mode, dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_SphericalWarperGpu {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_SphericalWarperTraitConst for Detail_SphericalWarperGpu {
		#[inline] fn as_raw_Detail_SphericalWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SphericalWarperTrait for Detail_SphericalWarperGpu {
		#[inline] fn as_raw_mut_Detail_SphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SphericalWarperGpuTraitConst for Detail_SphericalWarperGpu {
		#[inline] fn as_raw_Detail_SphericalWarperGpu(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SphericalWarperGpuTrait for Detail_SphericalWarperGpu {
		#[inline] fn as_raw_mut_Detail_SphericalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_SphericalWarperGpu {
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
	pub trait Detail_StereographicProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_StereographicProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_StereographicProjector]
	pub trait Detail_StereographicProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_StereographicProjectorTraitConst {
		fn as_raw_mut_Detail_StereographicProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_StereographicProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_StereographicProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_StereographicProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_StereographicProjectorTraitConst for Detail_StereographicProjector {
		#[inline] fn as_raw_Detail_StereographicProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_StereographicProjectorTrait for Detail_StereographicProjector {
		#[inline] fn as_raw_mut_Detail_StereographicProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_StereographicProjector {
	}
	
	boxed_cast_base! { Detail_StereographicProjector, crate::stitching::Detail_ProjectorBase, cv_detail_StereographicProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_StereographicProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_StereographicProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_StereographicWarper]
	pub trait Detail_StereographicWarperTraitConst {
		fn as_raw_Detail_StereographicWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_StereographicWarper]
	pub trait Detail_StereographicWarperTrait: crate::stitching::Detail_StereographicWarperTraitConst {
		fn as_raw_mut_Detail_StereographicWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_StereographicWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_StereographicWarperTraitConst for Detail_StereographicWarper {
		#[inline] fn as_raw_Detail_StereographicWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_StereographicWarperTrait for Detail_StereographicWarper {
		#[inline] fn as_raw_mut_Detail_StereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_StereographicWarper {
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
	
	/// Constant methods for [crate::stitching::Detail_TransverseMercatorProjector]
	pub trait Detail_TransverseMercatorProjectorTraitConst: crate::stitching::Detail_ProjectorBaseTraitConst {
		fn as_raw_Detail_TransverseMercatorProjector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_TransverseMercatorProjector]
	pub trait Detail_TransverseMercatorProjectorTrait: crate::stitching::Detail_ProjectorBaseTrait + crate::stitching::Detail_TransverseMercatorProjectorTraitConst {
		fn as_raw_mut_Detail_TransverseMercatorProjector(&mut self) -> *mut c_void;
	
		#[inline]
		fn map_forward(&mut self, x: f32, y: f32, u: &mut f32, v: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(self.as_raw_mut_Detail_TransverseMercatorProjector(), x, y, u, v, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn map_backward(&mut self, u: f32, v: f32, x: &mut f32, y: &mut f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(self.as_raw_mut_Detail_TransverseMercatorProjector(), u, v, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Detail_TransverseMercatorProjector {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_TransverseMercatorProjectorTraitConst for Detail_TransverseMercatorProjector {
		#[inline] fn as_raw_Detail_TransverseMercatorProjector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_TransverseMercatorProjectorTrait for Detail_TransverseMercatorProjector {
		#[inline] fn as_raw_mut_Detail_TransverseMercatorProjector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_TransverseMercatorProjector {
	}
	
	boxed_cast_base! { Detail_TransverseMercatorProjector, crate::stitching::Detail_ProjectorBase, cv_detail_TransverseMercatorProjector_to_Detail_ProjectorBase }
	
	impl std::fmt::Debug for Detail_TransverseMercatorProjector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_TransverseMercatorProjector")
				.field("scale", &crate::stitching::Detail_ProjectorBaseTraitConst::scale(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::stitching::Detail_TransverseMercatorWarper]
	pub trait Detail_TransverseMercatorWarperTraitConst {
		fn as_raw_Detail_TransverseMercatorWarper(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_TransverseMercatorWarper]
	pub trait Detail_TransverseMercatorWarperTrait: crate::stitching::Detail_TransverseMercatorWarperTraitConst {
		fn as_raw_mut_Detail_TransverseMercatorWarper(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_TransverseMercatorWarper {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_TransverseMercatorWarperTraitConst for Detail_TransverseMercatorWarper {
		#[inline] fn as_raw_Detail_TransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_TransverseMercatorWarperTrait for Detail_TransverseMercatorWarper {
		#[inline] fn as_raw_mut_Detail_TransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_TransverseMercatorWarper {
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
	pub trait Detail_VoronoiSeamFinderTraitConst: crate::stitching::Detail_PairwiseSeamFinderTraitConst {
		fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::stitching::Detail_VoronoiSeamFinder]
	pub trait Detail_VoronoiSeamFinderTrait: crate::stitching::Detail_PairwiseSeamFinderTrait + crate::stitching::Detail_VoronoiSeamFinderTraitConst {
		fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void;
	
		#[inline]
		fn find(&mut self, src: &core::Vector<core::UMat>, corners: &core::Vector<core::Point>, masks: &mut core::Vector<core::UMat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_VoronoiSeamFinder_find_const_vectorLUMatGR_const_vectorLPointGR_vectorLUMatGR(self.as_raw_mut_Detail_VoronoiSeamFinder(), src.as_raw_VectorOfUMat(), corners.as_raw_VectorOfPoint(), masks.as_raw_mut_VectorOfUMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct Detail_VoronoiSeamFinder {
		ptr: *mut c_void
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
	
	impl crate::stitching::Detail_SeamFinderTraitConst for Detail_VoronoiSeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for Detail_VoronoiSeamFinder {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_VoronoiSeamFinderTraitConst for Detail_VoronoiSeamFinder {
		#[inline] fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::stitching::Detail_VoronoiSeamFinderTrait for Detail_VoronoiSeamFinder {
		#[inline] fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_VoronoiSeamFinder {
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
}
