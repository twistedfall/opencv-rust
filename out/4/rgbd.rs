//! # RGB-Depth Processing
//!
//! [kinfu_icp]
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{ColoredKinfu_ColoredKinFuTrait, ColoredKinfu_ColoredKinFuTraitConst, ColoredKinfu_ParamsTrait, ColoredKinfu_ParamsTraitConst, DepthCleanerTrait, DepthCleanerTraitConst, Dynafu_DynaFuTrait, Dynafu_DynaFuTraitConst, FastICPOdometryTrait, FastICPOdometryTraitConst, ICPOdometryTrait, ICPOdometryTraitConst, Kinfu_Detail_PoseGraphTrait, Kinfu_Detail_PoseGraphTraitConst, Kinfu_KinFuTrait, Kinfu_KinFuTraitConst, Kinfu_ParamsTrait, Kinfu_ParamsTraitConst, Kinfu_VolumeParamsTrait, Kinfu_VolumeParamsTraitConst, Kinfu_VolumeTrait, Kinfu_VolumeTraitConst, LargeKinfuTrait, LargeKinfuTraitConst, LineMod_ColorGradientTrait, LineMod_ColorGradientTraitConst, LineMod_DepthNormalTrait, LineMod_DepthNormalTraitConst, LineMod_DetectorTrait, LineMod_DetectorTraitConst, LineMod_MatchTrait, LineMod_MatchTraitConst, LineMod_ModalityTrait, LineMod_ModalityTraitConst, LineMod_QuantizedPyramidTrait, LineMod_QuantizedPyramidTraitConst, LineMod_TemplateTrait, LineMod_TemplateTraitConst, OdometryFrameTrait, OdometryFrameTraitConst, OdometryTrait, OdometryTraitConst, ParamsTrait, ParamsTraitConst, RgbdFrameTrait, RgbdFrameTraitConst, RgbdICPOdometryTrait, RgbdICPOdometryTraitConst, RgbdNormalsTrait, RgbdNormalsTraitConst, RgbdOdometryTrait, RgbdOdometryTraitConst, RgbdPlaneTrait, RgbdPlaneTraitConst};
}

// DEPTH_CLEANER_NIL /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:195
pub const DepthCleaner_DEPTH_CLEANER_NIL: i32 = 0;
// COLOREDTSDF /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:59
pub const Kinfu_VolumeType_COLOREDTSDF: i32 = 2;
// HASHTSDF /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:58
pub const Kinfu_VolumeType_HASHTSDF: i32 = 1;
// TSDF /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:57
pub const Kinfu_VolumeType_TSDF: i32 = 0;
// CACHE_ALL /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:494
pub const OdometryFrame_CACHE_ALL: i32 = 3;
// CACHE_DST /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:494
pub const OdometryFrame_CACHE_DST: i32 = 2;
// CACHE_SRC /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:494
pub const OdometryFrame_CACHE_SRC: i32 = 1;
// RIGID_BODY_MOTION /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:531
pub const Odometry_RIGID_BODY_MOTION: i32 = 4;
// ROTATION /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:531
pub const Odometry_ROTATION: i32 = 1;
// TRANSLATION /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:531
pub const Odometry_TRANSLATION: i32 = 2;
// RGBD_NORMALS_METHOD_FALS /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:78
pub const RgbdNormals_RGBD_NORMALS_METHOD_FALS: i32 = 0;
// RGBD_NORMALS_METHOD_LINEMOD /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:79
pub const RgbdNormals_RGBD_NORMALS_METHOD_LINEMOD: i32 = 1;
// RGBD_NORMALS_METHOD_SRI /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:80
pub const RgbdNormals_RGBD_NORMALS_METHOD_SRI: i32 = 2;
// RGBD_PLANE_METHOD_DEFAULT /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:334
pub const RgbdPlane_RGBD_PLANE_METHOD_DEFAULT: i32 = 0;
/// NIL method is from
/// ``Modeling Kinect Sensor Noise for Improved 3d Reconstruction and Tracking``
/// by C. Nguyen, S. Izadi, D. Lovel
// DEPTH_CLEANER_METHOD /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:193
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepthCleaner_DEPTH_CLEANER_METHOD {
	DEPTH_CLEANER_NIL = 0,
}

impl TryFrom<i32> for DepthCleaner_DEPTH_CLEANER_METHOD {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DEPTH_CLEANER_NIL),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::rgbd::DepthCleaner_DEPTH_CLEANER_METHOD"))),
		}
	}
}

opencv_type_enum! { crate::rgbd::DepthCleaner_DEPTH_CLEANER_METHOD }

// VolumeType /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:55
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kinfu_VolumeType {
	TSDF = 0,
	HASHTSDF = 1,
	COLOREDTSDF = 2,
}

impl TryFrom<i32> for Kinfu_VolumeType {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::TSDF),
			1 => Ok(Self::HASHTSDF),
			2 => Ok(Self::COLOREDTSDF),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::rgbd::Kinfu_VolumeType"))),
		}
	}
}

opencv_type_enum! { crate::rgbd::Kinfu_VolumeType }

// RGBD_NORMALS_METHOD /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:76
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RgbdNormals_RGBD_NORMALS_METHOD {
	RGBD_NORMALS_METHOD_FALS = 0,
	RGBD_NORMALS_METHOD_LINEMOD = 1,
	RGBD_NORMALS_METHOD_SRI = 2,
}

impl TryFrom<i32> for RgbdNormals_RGBD_NORMALS_METHOD {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RGBD_NORMALS_METHOD_FALS),
			1 => Ok(Self::RGBD_NORMALS_METHOD_LINEMOD),
			2 => Ok(Self::RGBD_NORMALS_METHOD_SRI),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::rgbd::RgbdNormals_RGBD_NORMALS_METHOD"))),
		}
	}
}

opencv_type_enum! { crate::rgbd::RgbdNormals_RGBD_NORMALS_METHOD }

// RGBD_PLANE_METHOD /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:332
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RgbdPlane_RGBD_PLANE_METHOD {
	RGBD_PLANE_METHOD_DEFAULT = 0,
}

impl TryFrom<i32> for RgbdPlane_RGBD_PLANE_METHOD {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RGBD_PLANE_METHOD_DEFAULT),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::rgbd::RgbdPlane_RGBD_PLANE_METHOD"))),
		}
	}
}

opencv_type_enum! { crate::rgbd::RgbdPlane_RGBD_PLANE_METHOD }

/// Backwards compatibility for old versions
// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:44
pub type Dynafu_Params = crate::rgbd::Kinfu_Params;
// makeVolume(VolumeType, float, Matx44f, float, float, int, float, Vec3i)(Enum, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:122
// ("cv::kinfu::makeVolume", vec![(pred!(mut, ["_volumeType", "_voxelSize", "_pose", "_raycastStepFactor", "_truncDist", "_maxWeight", "_truncateThreshold", "_resolution"], ["cv::kinfu::VolumeType", "float", "cv::Matx44f", "float", "float", "int", "float", "cv::Vec3i"]), _)]),
#[inline]
pub fn make_volume(_volume_type: crate::rgbd::Kinfu_VolumeType, _voxel_size: f32, _pose: core::Matx44f, _raycast_step_factor: f32, _trunc_dist: f32, _max_weight: i32, _truncate_threshold: f32, _resolution: core::Vec3i) -> Result<core::Ptr<crate::rgbd::Kinfu_Volume>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_kinfu_makeVolume_VolumeType_float_Matx44f_float_float_int_float_Vec3i(_volume_type, _voxel_size, &_pose, _raycast_step_factor, _trunc_dist, _max_weight, _truncate_threshold, &_resolution, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Volume>::opencv_from_extern(ret) };
	Ok(ret)
}

/// \brief Debug function to colormap a quantized image for viewing.
// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:245
// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
#[inline]
pub fn colormap(quantized: &impl core::MatTraitConst, dst: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_colormap_const_MatR_MatR(quantized.as_raw_Mat(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// \brief Debug function to draw linemod features
/// ## Parameters
/// * img: 
/// * templates: see [Detector::addTemplate]
/// * tl: template bbox top-left offset see [Detector::addTemplate]
/// * size: marker size see [cv::drawMarker]
///
/// ## Note
/// This alternative version of [draw_features] function uses the following default values for its arguments:
/// * size: 10
// cv::linemod::drawFeatures(InputOutputArray, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:254
// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*"]), _)]),
#[inline]
pub fn draw_features_def(img: &mut impl ToInputOutputArray, templates: &core::Vector<crate::rgbd::LineMod_Template>, tl: core::Point2i) -> Result<()> {
	input_output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR(img.as_raw__InputOutputArray(), templates.as_raw_VectorOfLineMod_Template(), &tl, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// \brief Debug function to draw linemod features
/// ## Parameters
/// * img: 
/// * templates: see [Detector::addTemplate]
/// * tl: template bbox top-left offset see [Detector::addTemplate]
/// * size: marker size see [cv::drawMarker]
///
/// ## C++ default parameters
/// * size: 10
// drawFeatures(InputOutputArray, const std::vector<Template> &, const Point2i &, int)(InputOutputArray, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:254
// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl", "size"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*", "int"]), _)]),
#[inline]
pub fn draw_features(img: &mut impl ToInputOutputArray, templates: &core::Vector<crate::rgbd::LineMod_Template>, tl: core::Point2i, size: i32) -> Result<()> {
	input_output_array_arg!(img);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR_int(img.as_raw__InputOutputArray(), templates.as_raw_VectorOfLineMod_Template(), &tl, size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// \brief Factory function for detector using LINE algorithm with color gradients.
///
/// Default parameter settings suitable for VGA images.
// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:420
// ("cv::linemod::getDefaultLINE", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_default_line() -> Result<core::Ptr<crate::rgbd::LineMod_Detector>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_getDefaultLINE(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_Detector>::opencv_from_extern(ret) };
	Ok(ret)
}

/// \brief Factory function for detector using LINE-MOD algorithm with color gradients
/// and depth normals.
///
/// Default parameter settings suitable for VGA images.
// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:428
// ("cv::linemod::getDefaultLINEMOD", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_default_linemod() -> Result<core::Ptr<crate::rgbd::LineMod_Detector>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_getDefaultLINEMOD(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_Detector>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## Parameters
/// * depth: the depth image
/// * in_K: 
/// * in_points: the list of xy coordinates
/// * points3d: the resulting 3d points
// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:299
// ("cv::rgbd::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn depth_to3d_sparse(depth: &impl ToInputArray, in_k: &impl ToInputArray, in_points: &impl ToInputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(in_k);
	input_array_arg!(in_points);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), in_k.as_raw__InputArray(), in_points.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts a depth image to an organized set of 3d points.
/// The coordinate system is x pointing left, y down and z away from the camera
/// ## Parameters
/// * depth: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), otherwise, if given as CV_32F or CV_64F, it is assumed in meters)
/// * K: The calibration matrix
/// * points3d: the resulting 3d points. They are of depth the same as `depth` if it is CV_32F or CV_64F, and the
///        depth of `K` if `depth` is of depth CV_U
/// * mask: the mask of the points to consider (can be empty)
///
/// ## Note
/// This alternative version of [depth_to3d] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::rgbd::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:312
// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn depth_to3d_def(depth: &impl ToInputArray, k: &impl ToInputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(k);
	output_array_arg!(points3d);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Converts a depth image to an organized set of 3d points.
/// The coordinate system is x pointing left, y down and z away from the camera
/// ## Parameters
/// * depth: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), otherwise, if given as CV_32F or CV_64F, it is assumed in meters)
/// * K: The calibration matrix
/// * points3d: the resulting 3d points. They are of depth the same as `depth` if it is CV_32F or CV_64F, and the
///        depth of `K` if `depth` is of depth CV_U
/// * mask: the mask of the points to consider (can be empty)
///
/// ## C++ default parameters
/// * mask: noArray()
// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:312
// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn depth_to3d(depth: &impl ToInputArray, k: &impl ToInputArray, points3d: &mut impl ToOutputArray, mask: &impl ToInputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(k);
	output_array_arg!(points3d);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:33
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const double*"]), _)]),
#[inline]
pub fn is_valid_depth_1(depth: &f64) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_doubleR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Checks if the value is a valid depth. For CV_16U or CV_16S, the convention is to be invalid if it is
/// a limit. For a float/double, we just check if it is a NaN
/// ## Parameters
/// * depth: the depth to check for validity
// isValidDepth(const float &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:27
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const float*"]), _)]),
#[inline]
pub fn is_valid_depth(depth: &f32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_floatR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:52
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const int*"]), _)]),
#[inline]
pub fn is_valid_depth_4(depth: &i32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_intR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const short &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:39
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const short*"]), _)]),
#[inline]
pub fn is_valid_depth_2(depth: &i16) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_shortR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const unsigned int &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:58
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned int*"]), _)]),
#[inline]
pub fn is_valid_depth_5(depth: &u32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_intR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const unsigned short &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:45
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned short*"]), _)]),
#[inline]
pub fn is_valid_depth_3(depth: &u16) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_shortR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Registers depth data to an external camera
/// Registration is performed by creating a depth cloud, transforming the cloud by
/// the rigid body transformation between the cameras, and then projecting the
/// transformed points into the RGB camera.
///
/// uv_rgb = K_rgb * [R | t] * z * inv(K_ir) * uv_ir
///
/// Currently does not check for negative depth values.
///
/// ## Parameters
/// * unregisteredCameraMatrix: the camera matrix of the depth camera
/// * registeredCameraMatrix: the camera matrix of the external camera
/// * registeredDistCoeffs: the distortion coefficients of the external camera
/// * Rt: the rigid body transform between the cameras. Transforms points from depth camera frame to external camera frame.
/// * unregisteredDepth: the input depth data
/// * outputImagePlaneSize: the image plane dimensions of the external camera (width, height)
/// * registeredDepth: the result of transforming the depth into the external camera
/// * depthDilation: whether or not the depth is dilated to avoid holes and occlusion errors (optional)
///
/// ## Note
/// This alternative version of [register_depth] function uses the following default values for its arguments:
/// * depth_dilation: false
// cv::rgbd::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:287
// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn register_depth_def(unregistered_camera_matrix: &impl ToInputArray, registered_camera_matrix: &impl ToInputArray, registered_dist_coeffs: &impl ToInputArray, rt: &impl ToInputArray, unregistered_depth: &impl ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(unregistered_camera_matrix);
	input_array_arg!(registered_camera_matrix);
	input_array_arg!(registered_dist_coeffs);
	input_array_arg!(rt);
	input_array_arg!(unregistered_depth);
	output_array_arg!(registered_depth);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Registers depth data to an external camera
/// Registration is performed by creating a depth cloud, transforming the cloud by
/// the rigid body transformation between the cameras, and then projecting the
/// transformed points into the RGB camera.
///
/// uv_rgb = K_rgb * [R | t] * z * inv(K_ir) * uv_ir
///
/// Currently does not check for negative depth values.
///
/// ## Parameters
/// * unregisteredCameraMatrix: the camera matrix of the depth camera
/// * registeredCameraMatrix: the camera matrix of the external camera
/// * registeredDistCoeffs: the distortion coefficients of the external camera
/// * Rt: the rigid body transform between the cameras. Transforms points from depth camera frame to external camera frame.
/// * unregisteredDepth: the input depth data
/// * outputImagePlaneSize: the image plane dimensions of the external camera (width, height)
/// * registeredDepth: the result of transforming the depth into the external camera
/// * depthDilation: whether or not the depth is dilated to avoid holes and occlusion errors (optional)
///
/// ## C++ default parameters
/// * depth_dilation: false
// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:287
// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
#[inline]
pub fn register_depth(unregistered_camera_matrix: &impl ToInputArray, registered_camera_matrix: &impl ToInputArray, registered_dist_coeffs: &impl ToInputArray, rt: &impl ToInputArray, unregistered_depth: &impl ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl ToOutputArray, depth_dilation: bool) -> Result<()> {
	input_array_arg!(unregistered_camera_matrix);
	input_array_arg!(registered_camera_matrix);
	input_array_arg!(registered_dist_coeffs);
	input_array_arg!(rt);
	input_array_arg!(unregistered_depth);
	output_array_arg!(registered_depth);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), depth_dilation, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// If the input image is of type CV_16UC1 (like the Kinect one), the image is converted to floats, divided
/// by depth_factor to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
/// Otherwise, the image is simply converted to floats
/// ## Parameters
/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), it is assumed in meters)
/// * depth: the desired output depth (floats or double)
/// * out: The rescaled float depth image
/// * depth_factor: (optional) factor by which depth is converted to distance (by default = 1000.0 for Kinect sensor)
///
/// ## Note
/// This alternative version of [rescale_depth] function uses the following default values for its arguments:
/// * depth_factor: 1000.0
// cv::rgbd::rescaleDepth(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:325
// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn rescale_depth_def(in_: &impl ToInputArray, depth: i32, out: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(in_);
	output_array_arg!(out);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(in_.as_raw__InputArray(), depth, out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// If the input image is of type CV_16UC1 (like the Kinect one), the image is converted to floats, divided
/// by depth_factor to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
/// Otherwise, the image is simply converted to floats
/// ## Parameters
/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), it is assumed in meters)
/// * depth: the desired output depth (floats or double)
/// * out: The rescaled float depth image
/// * depth_factor: (optional) factor by which depth is converted to distance (by default = 1000.0 for Kinect sensor)
///
/// ## C++ default parameters
/// * depth_factor: 1000.0
// rescaleDepth(InputArray, int, OutputArray, double)(InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:325
// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out", "depth_factor"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "double"]), _)]),
#[inline]
pub fn rescale_depth(in_: &impl ToInputArray, depth: i32, out: &mut impl ToOutputArray, depth_factor: f64) -> Result<()> {
	input_array_arg!(in_);
	output_array_arg!(out);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(in_.as_raw__InputArray(), depth, out.as_raw__OutputArray(), depth_factor, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Warp the image: compute 3d points from the depth, transform them using given transformation,
/// then project color point cloud to an image plane.
/// This function can be used to visualize results of the Odometry algorithm.
/// ## Parameters
/// * image: The image (of CV_8UC1 or CV_8UC3 type)
/// * depth: The depth (of type used in depthTo3d fuction)
/// * mask: The mask of used pixels (of CV_8UC1), it can be empty
/// * Rt: The transformation that will be applied to the 3d points computed from the depth
/// * cameraMatrix: Camera matrix
/// * distCoeff: Distortion coefficients
/// * warpedImage: The warped image.
/// * warpedDepth: The warped depth.
/// * warpedMask: The warped mask.
///
/// ## Note
/// This alternative version of [warp_frame] function uses the following default values for its arguments:
/// * warped_depth: noArray()
/// * warped_mask: noArray()
// cv::rgbd::warpFrame(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1179
// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn warp_frame_def(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst, mask: &impl core::MatTraitConst, rt: &impl core::MatTraitConst, camera_matrix: &impl core::MatTraitConst, dist_coeff: &impl core::MatTraitConst, warped_image: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(warped_image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), rt.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeff.as_raw_Mat(), warped_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Warp the image: compute 3d points from the depth, transform them using given transformation,
/// then project color point cloud to an image plane.
/// This function can be used to visualize results of the Odometry algorithm.
/// ## Parameters
/// * image: The image (of CV_8UC1 or CV_8UC3 type)
/// * depth: The depth (of type used in depthTo3d fuction)
/// * mask: The mask of used pixels (of CV_8UC1), it can be empty
/// * Rt: The transformation that will be applied to the 3d points computed from the depth
/// * cameraMatrix: Camera matrix
/// * distCoeff: Distortion coefficients
/// * warpedImage: The warped image.
/// * warpedDepth: The warped depth.
/// * warpedMask: The warped mask.
///
/// ## C++ default parameters
/// * warped_depth: noArray()
/// * warped_mask: noArray()
// warpFrame(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, OutputArray, OutputArray)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1179
// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage", "warpedDepth", "warpedMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn warp_frame(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst, mask: &impl core::MatTraitConst, rt: &impl core::MatTraitConst, camera_matrix: &impl core::MatTraitConst, dist_coeff: &impl core::MatTraitConst, warped_image: &mut impl ToOutputArray, warped_depth: &mut impl ToOutputArray, warped_mask: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(warped_image);
	output_array_arg!(warped_depth);
	output_array_arg!(warped_mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), rt.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeff.as_raw_Mat(), warped_image.as_raw__OutputArray(), warped_depth.as_raw__OutputArray(), warped_mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::rgbd::ColoredKinfu_ColoredKinFu]
// ColoredKinFu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:195
pub trait ColoredKinfu_ColoredKinFuTraitConst {
	fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void;

	/// Get current parameters
	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:202
	// ("cv::colored_kinfu::ColoredKinFu::getParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getParams_const(self.as_raw_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Renders a volume into an image
	///
	/// Renders a 0-surface of TSDF using Phong shading into a CV_8UC4 Mat.
	/// Light pose is fixed in KinFu params.
	///
	/// ## Parameters
	/// * image: resulting image
	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:212
	// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn render(&self, image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders a volume into an image
	///
	/// Renders a 0-surface of TSDF using Phong shading into a CV_8UC4 Mat.
	/// Light pose is fixed in KinFu params.
	///
	/// ## Parameters
	/// * image: resulting image
	/// * cameraPose: pose of camera to render from. If empty then render from current pose
	///   which is a last frame camera pose.
	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:224
	// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	#[inline]
	fn render_1(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_ColoredKinfu_ColoredKinFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets points, normals and colors of current 3d mesh
	///
	/// The order of normals corresponds to order of points.
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	/// * normals: vector of normals which are 4-float vectors
	/// * colors: vector of colors which are 4-float vectors
	///
	/// ## C++ default parameters
	/// * colors: noArray()
	// getCloud(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:235
	// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals", "colors"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		output_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets points, normals and colors of current 3d mesh
	///
	/// The order of normals corresponds to order of points.
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	/// * normals: vector of normals which are 4-float vectors
	/// * colors: vector of colors which are 4-float vectors
	///
	/// ## Note
	/// This alternative version of [ColoredKinfu_ColoredKinFuTraitConst::get_cloud] function uses the following default values for its arguments:
	/// * colors: noArray()
	// cv::colored_kinfu::ColoredKinFu::getCloud(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:235
	// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cloud_def(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets points of current 3d mesh
	///
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:243
	// ("cv::colored_kinfu::ColoredKinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates normals for given points
	/// ## Parameters
	/// * points: input vector of points which are 4-float vectors
	/// * normals: output vector of corresponding normals which are 4-float vectors
	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:249
	// ("cv::colored_kinfu::ColoredKinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get current pose in voxel space
	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:258
	// ("cv::colored_kinfu::ColoredKinFu::getPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_getPose_const(self.as_raw_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::ColoredKinfu_ColoredKinFu]
pub trait ColoredKinfu_ColoredKinFuTrait: crate::rgbd::ColoredKinfu_ColoredKinFuTraitConst {
	fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void;

	/// Resets the algorithm
	///
	/// Clears current model and resets a pose.
	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:255
	// ("cv::colored_kinfu::ColoredKinFu::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_reset(self.as_raw_mut_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Process next depth frame
	/// ## Parameters
	/// * depth: input Mat of depth frame
	/// * rgb: input Mat of rgb (colored) frame
	///
	/// ## Returns
	/// true if succeeded to align new frame with current scene, false if opposite
	// update(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:266
	// ("cv::colored_kinfu::ColoredKinFu::update", vec![(pred!(mut, ["depth", "rgb"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn update(&mut self, depth: &impl ToInputArray, rgb: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		input_array_arg!(rgb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ColoredKinfu_ColoredKinFu(), depth.as_raw__InputArray(), rgb.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// KinectFusion implementation
///
/// This class implements a 3d reconstruction algorithm described in
/// [kinectfusion](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_kinectfusion) paper.
///
/// It takes a sequence of depth images taken from depth sensor
/// (or any depth images source such as stereo camera matching algorithm or even raymarching renderer).
/// The output can be obtained as a vector of points and their normals
/// or can be Phong-rendered from given camera pose.
///
/// An internal representation of a model is a voxel cuboid that keeps TSDF values
/// which are a sort of distances to the surface (for details read the [kinectfusion](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_kinectfusion) article about TSDF).
/// There is no interface to that representation yet.
///
/// KinFu uses OpenCL acceleration automatically if available.
/// To enable or disable it explicitly use cv::setUseOptimized() or cv::ocl::setUseOpenCL().
///
/// This implementation is based on [kinfu-remake](https://github.com/Nerei/kinfu_remake).
///
/// Note that the KinectFusion algorithm was patented and its use may be restricted by
/// the list of patents mentioned in README.md file in this module directory.
///
/// That's why you need to set the OPENCV_ENABLE_NONFREE option in CMake to use KinectFusion.
// ColoredKinFu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:195
pub struct ColoredKinfu_ColoredKinFu {
	ptr: *mut c_void,
}

opencv_type_boxed! { ColoredKinfu_ColoredKinFu }

impl Drop for ColoredKinfu_ColoredKinFu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_delete(self.as_raw_mut_ColoredKinfu_ColoredKinFu()) };
	}
}

unsafe impl Send for ColoredKinfu_ColoredKinFu {}

impl crate::rgbd::ColoredKinfu_ColoredKinFuTraitConst for ColoredKinfu_ColoredKinFu {
	#[inline] fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::ColoredKinfu_ColoredKinFuTrait for ColoredKinfu_ColoredKinFu {
	#[inline] fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ColoredKinfu_ColoredKinFu, crate::rgbd::ColoredKinfu_ColoredKinFuTraitConst, as_raw_ColoredKinfu_ColoredKinFu, crate::rgbd::ColoredKinfu_ColoredKinFuTrait, as_raw_mut_ColoredKinfu_ColoredKinFu }

impl ColoredKinfu_ColoredKinFu {
	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:198
	// ("cv::colored_kinfu::ColoredKinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::colored_kinfu::Params>*"]), _)]),
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::ColoredKinfu_Params>) -> Result<core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_ColoredKinFu_create_const_PtrLParamsGR(_params.as_raw_PtrOfColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_ColoredKinFu>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for ColoredKinfu_ColoredKinFu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ColoredKinfu_ColoredKinFu")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::ColoredKinfu_Params]
// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:19
pub trait ColoredKinfu_ParamsTraitConst {
	fn as_raw_ColoredKinfu_Params(&self) -> *const c_void;

	/// frame size in pixels
	// cv::colored_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:83
	// ("cv::colored_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propFrameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb frame size in pixels
	// cv::colored_kinfu::Params::rgb_frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:86
	// ("cv::colored_kinfu::Params::rgb_frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rgb_frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::colored_kinfu::Params::volumeType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::volumeType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_type(&self) -> crate::rgbd::Kinfu_VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propVolumeType_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// camera intrinsics
	// cv::colored_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:91
	// ("cv::colored_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propIntr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb camera intrinsics
	// cv::colored_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:94
	// ("cv::colored_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rgb_intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propRgb_intr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// pre-scale per 1 meter for input values
	///
	/// Typical values are:
	///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
	///      * 1000 per 1 meter for Kinect 2 device
	///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
	// cv::colored_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:103
	// ("cv::colored_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propDepthFactor_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::colored_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:106
	// ("cv::colored_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::colored_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::colored_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::colored_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:113
	// ("cv::colored_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propPyramidLevels_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::colored_kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:119
	// ("cv::colored_kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_dims(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propVolumeDims_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Size of voxel in meters
	// cv::colored_kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVoxelSize_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::colored_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:127
	// ("cv::colored_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// initial volume pose in meters
	// cv::colored_kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:130
	// ("cv::colored_kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propVolumePose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::colored_kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:136
	// ("cv::colored_kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_trunc_dist_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::colored_kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:142
	// ("cv::colored_kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_max_weight_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::colored_kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:148
	// ("cv::colored_kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propRaycast_step_factor_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// light pose for rendering in meters
	// cv::colored_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:155
	// ("cv::colored_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propLightPose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance theshold for ICP in meters
	// cv::colored_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:158
	// ("cv::colored_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpDistThresh_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::colored_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpAngleThresh_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::colored_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_iterations(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpIterations_const(self.as_raw_ColoredKinfu_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::colored_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:168
	// ("cv::colored_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn truncate_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTruncateThreshold_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

}

/// Mutable methods for [crate::rgbd::ColoredKinfu_Params]
pub trait ColoredKinfu_ParamsTrait: crate::rgbd::ColoredKinfu_ParamsTraitConst {
	fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void;

	/// frame size in pixels
	// cv::colored_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:83
	// ("cv::colored_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// rgb frame size in pixels
	// cv::colored_kinfu::Params::setRgb_frameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:86
	// ("cv::colored_kinfu::Params::setRgb_frameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_rgb_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_const_Size(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	// cv::colored_kinfu::Params::setVolumeType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::setVolumeType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
	#[inline]
	fn set_volume_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeType_const_VolumeType(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// camera intrinsics
	// cv::colored_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:91
	// ("cv::colored_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// rgb camera intrinsics
	// cv::colored_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:94
	// ("cv::colored_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_rgb_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propRgb_intr_const_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// pre-scale per 1 meter for input values
	///
	/// Typical values are:
	///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
	///      * 1000 per 1 meter for Kinect 2 device
	///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
	// cv::colored_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:103
	// ("cv::colored_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::colored_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:106
	// ("cv::colored_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::colored_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::colored_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::colored_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:113
	// ("cv::colored_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::colored_kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:119
	// ("cv::colored_kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	#[inline]
	fn set_volume_dims(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeDims_const_Vec3i(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// Size of voxel in meters
	// cv::colored_kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVoxelSize_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::colored_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:127
	// ("cv::colored_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// initial volume pose in meters
	// cv::colored_kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:130
	// ("cv::colored_kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
	#[inline]
	fn set_volume_pose(&mut self, val: core::Affine3f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumePose_const_Affine3f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::colored_kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:136
	// ("cv::colored_kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_trunc_dist_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::colored_kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:142
	// ("cv::colored_kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_tsdf_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_max_weight_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::colored_kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:148
	// ("cv::colored_kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propRaycast_step_factor_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// light pose for rendering in meters
	// cv::colored_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:155
	// ("cv::colored_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// distance theshold for ICP in meters
	// cv::colored_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:158
	// ("cv::colored_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::colored_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::colored_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_ColoredKinfu_Params(), val.as_raw_VectorOfi32()) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::colored_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:168
	// ("cv::colored_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_truncate_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTruncateThreshold_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Set Initial Volume Pose
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * R: rotation matrix
	/// * t: translation vector
	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:51
	// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	#[inline]
	fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), &r, &t, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set Initial Volume Pose
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * homogen_tf: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:58
	// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
	#[inline]
	fn set_initial_volume_pose_1(&mut self, homogen_tf: core::Matx44f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_ColoredKinfu_Params(), &homogen_tf, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:19
pub struct ColoredKinfu_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { ColoredKinfu_Params }

impl Drop for ColoredKinfu_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_colored_kinfu_Params_delete(self.as_raw_mut_ColoredKinfu_Params()) };
	}
}

unsafe impl Send for ColoredKinfu_Params {}

impl crate::rgbd::ColoredKinfu_ParamsTraitConst for ColoredKinfu_Params {
	#[inline] fn as_raw_ColoredKinfu_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::ColoredKinfu_ParamsTrait for ColoredKinfu_Params {
	#[inline] fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ColoredKinfu_Params, crate::rgbd::ColoredKinfu_ParamsTraitConst, as_raw_ColoredKinfu_Params, crate::rgbd::ColoredKinfu_ParamsTrait, as_raw_mut_ColoredKinfu_Params }

impl ColoredKinfu_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:22
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor for Params
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * volumeInitialPoseRot: rotation matrix
	/// * volumeInitialPoseTransl: translation vector
	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:30
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	#[inline]
	pub fn new(volume_initial_pose_rot: core::Matx33f, volume_initial_pose_transl: core::Vec3f) -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_Params_Matx33f_Vec3f(&volume_initial_pose_rot, &volume_initial_pose_transl, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor for Params
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * volumeInitialPose: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:40
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
	#[inline]
	pub fn new_1(volume_initial_pose: core::Matx44f) -> Result<crate::rgbd::ColoredKinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_Params_Matx44f(&volume_initial_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default parameters
	/// A set of parameters which provides better model quality, can be very slow.
	// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:64
	// ("cv::colored_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_params() -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_defaultParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Coarse parameters
	/// A set of parameters which provides better speed, can fail to match frames
	/// in case of rapid sensor motion.
	// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:70
	// ("cv::colored_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn coarse_params() -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_coarseParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// HashTSDF parameters
	/// A set of parameters suitable for use with HashTSDFVolume
	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:75
	// ("cv::colored_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	#[inline]
	pub fn hash_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_hashTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ColoredTSDF parameters
	/// A set of parameters suitable for use with HashTSDFVolume
	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:80
	// ("cv::colored_kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	#[inline]
	pub fn colored_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::ColoredKinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_coloredTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for ColoredKinfu_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ColoredKinfu_Params")
			.field("frame_size", &crate::rgbd::ColoredKinfu_ParamsTraitConst::frame_size(self))
			.field("rgb_frame_size", &crate::rgbd::ColoredKinfu_ParamsTraitConst::rgb_frame_size(self))
			.field("volume_type", &crate::rgbd::ColoredKinfu_ParamsTraitConst::volume_type(self))
			.field("intr", &crate::rgbd::ColoredKinfu_ParamsTraitConst::intr(self))
			.field("rgb_intr", &crate::rgbd::ColoredKinfu_ParamsTraitConst::rgb_intr(self))
			.field("depth_factor", &crate::rgbd::ColoredKinfu_ParamsTraitConst::depth_factor(self))
			.field("bilateral_sigma_depth", &crate::rgbd::ColoredKinfu_ParamsTraitConst::bilateral_sigma_depth(self))
			.field("bilateral_sigma_spatial", &crate::rgbd::ColoredKinfu_ParamsTraitConst::bilateral_sigma_spatial(self))
			.field("bilateral_kernel_size", &crate::rgbd::ColoredKinfu_ParamsTraitConst::bilateral_kernel_size(self))
			.field("pyramid_levels", &crate::rgbd::ColoredKinfu_ParamsTraitConst::pyramid_levels(self))
			.field("volume_dims", &crate::rgbd::ColoredKinfu_ParamsTraitConst::volume_dims(self))
			.field("voxel_size", &crate::rgbd::ColoredKinfu_ParamsTraitConst::voxel_size(self))
			.field("tsdf_min_camera_movement", &crate::rgbd::ColoredKinfu_ParamsTraitConst::tsdf_min_camera_movement(self))
			.field("volume_pose", &crate::rgbd::ColoredKinfu_ParamsTraitConst::volume_pose(self))
			.field("tsdf_trunc_dist", &crate::rgbd::ColoredKinfu_ParamsTraitConst::tsdf_trunc_dist(self))
			.field("tsdf_max_weight", &crate::rgbd::ColoredKinfu_ParamsTraitConst::tsdf_max_weight(self))
			.field("raycast_step_factor", &crate::rgbd::ColoredKinfu_ParamsTraitConst::raycast_step_factor(self))
			.field("light_pose", &crate::rgbd::ColoredKinfu_ParamsTraitConst::light_pose(self))
			.field("icp_dist_thresh", &crate::rgbd::ColoredKinfu_ParamsTraitConst::icp_dist_thresh(self))
			.field("icp_angle_thresh", &crate::rgbd::ColoredKinfu_ParamsTraitConst::icp_angle_thresh(self))
			.field("icp_iterations", &crate::rgbd::ColoredKinfu_ParamsTraitConst::icp_iterations(self))
			.field("truncate_threshold", &crate::rgbd::ColoredKinfu_ParamsTraitConst::truncate_threshold(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::Dynafu_DynaFu]
// DynaFu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:46
pub trait Dynafu_DynaFuTraitConst {
	fn as_raw_Dynafu_DynaFu(&self) -> *const c_void;

	/// Get current parameters
	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:53
	// ("cv::dynafu::DynaFu::getParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getParams_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Renders a volume into an image
	///
	/// Renders a 0-surface of TSDF using Phong shading into a CV_8UC4 Mat.
	/// Light pose is fixed in DynaFu params.
	///
	/// ## Parameters
	/// * image: resulting image
	/// * cameraPose: pose of camera to render from. If empty then render from current pose
	///   which is a last frame camera pose.
	///
	/// ## C++ default parameters
	/// * camera_pose: Matx44f::eye()
	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:65
	// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	#[inline]
	fn render(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_Dynafu_DynaFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders a volume into an image
	///
	/// Renders a 0-surface of TSDF using Phong shading into a CV_8UC4 Mat.
	/// Light pose is fixed in DynaFu params.
	///
	/// ## Parameters
	/// * image: resulting image
	/// * cameraPose: pose of camera to render from. If empty then render from current pose
	///   which is a last frame camera pose.
	///
	/// ## Note
	/// This alternative version of [Dynafu_DynaFuTraitConst::render] function uses the following default values for its arguments:
	/// * camera_pose: Matx44f::eye()
	// cv::dynafu::DynaFu::render(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:65
	// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn render_def(&self, image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_render_const_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets points and normals of current 3d mesh
	///
	/// The order of normals corresponds to order of points.
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	/// * normals: vector of normals which are 4-float vectors
	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:75
	// ("cv::dynafu::DynaFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets points of current 3d mesh
	///
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:83
	// ("cv::dynafu::DynaFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates normals for given points
	/// ## Parameters
	/// * points: input vector of points which are 4-float vectors
	/// * normals: output vector of corresponding normals which are 4-float vectors
	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:89
	// ("cv::dynafu::DynaFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get current pose in voxel space
	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:98
	// ("cv::dynafu::DynaFu::getPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getPose_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNodesPos()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:110
	// ("cv::dynafu::DynaFu::getNodesPos", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nodes_pos(&self) -> Result<core::Vector<core::Point3f>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getNodesPos_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// marchCubes(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:112
	// ("cv::dynafu::DynaFu::marchCubes", vec![(pred!(const, ["vertices", "edges"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn march_cubes(&self, vertices: &mut impl ToOutputArray, edges: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(vertices);
		output_array_arg!(edges);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), vertices.as_raw__OutputArray(), edges.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::Dynafu_DynaFu]
pub trait Dynafu_DynaFuTrait: crate::rgbd::Dynafu_DynaFuTraitConst {
	fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void;

	/// Resets the algorithm
	///
	/// Clears current model and resets a pose.
	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:95
	// ("cv::dynafu::DynaFu::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_reset(self.as_raw_mut_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Process next depth frame
	///
	///   Integrates depth into voxel space with respect to its ICP-calculated pose.
	///   Input image is converted to CV_32F internally if has another type.
	///
	/// ## Parameters
	/// * depth: one-channel image which size and depth scale is described in algorithm's parameters
	/// ## Returns
	/// true if succeeded to align new frame with current scene, false if opposite
	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:108
	// ("cv::dynafu::DynaFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn update(&mut self, depth: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_update_const__InputArrayR(self.as_raw_mut_Dynafu_DynaFu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * warp: true
	// renderSurface(OutputArray, OutputArray, OutputArray, bool)(OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:114
	// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage", "warp"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	#[inline]
	fn render_surface(&mut self, depth_image: &mut impl ToOutputArray, vert_image: &mut impl ToOutputArray, norm_image: &mut impl ToOutputArray, warp: bool) -> Result<()> {
		output_array_arg!(depth_image);
		output_array_arg!(vert_image);
		output_array_arg!(norm_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(self.as_raw_mut_Dynafu_DynaFu(), depth_image.as_raw__OutputArray(), vert_image.as_raw__OutputArray(), norm_image.as_raw__OutputArray(), warp, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Dynafu_DynaFuTrait::render_surface] function uses the following default values for its arguments:
	/// * warp: true
	// cv::dynafu::DynaFu::renderSurface(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:114
	// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn render_surface_def(&mut self, depth_image: &mut impl ToOutputArray, vert_image: &mut impl ToOutputArray, norm_image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(depth_image);
		output_array_arg!(vert_image);
		output_array_arg!(norm_image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Dynafu_DynaFu(), depth_image.as_raw__OutputArray(), vert_image.as_raw__OutputArray(), norm_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DynaFu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:46
pub struct Dynafu_DynaFu {
	ptr: *mut c_void,
}

opencv_type_boxed! { Dynafu_DynaFu }

impl Drop for Dynafu_DynaFu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dynafu_DynaFu_delete(self.as_raw_mut_Dynafu_DynaFu()) };
	}
}

unsafe impl Send for Dynafu_DynaFu {}

impl crate::rgbd::Dynafu_DynaFuTraitConst for Dynafu_DynaFu {
	#[inline] fn as_raw_Dynafu_DynaFu(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Dynafu_DynaFuTrait for Dynafu_DynaFu {
	#[inline] fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Dynafu_DynaFu, crate::rgbd::Dynafu_DynaFuTraitConst, as_raw_Dynafu_DynaFu, crate::rgbd::Dynafu_DynaFuTrait, as_raw_mut_Dynafu_DynaFu }

impl Dynafu_DynaFu {
	// create(const Ptr<kinfu::Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:49
	// ("cv::dynafu::DynaFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::Kinfu_Params>) -> Result<core::Ptr<crate::rgbd::Dynafu_DynaFu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_create_const_PtrLParamsGR(_params.as_raw_PtrOfKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Dynafu_DynaFu>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Dynafu_DynaFu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Dynafu_DynaFu")
			.finish()
	}
}

// Intr /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:15
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Kinfu_Intr {
	pub fx: f32,
	pub fy: f32,
	pub cx: f32,
	pub cy: f32,
}

opencv_type_simple! { crate::rgbd::Kinfu_Intr }

impl Kinfu_Intr {
	// scale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:65
	// ("cv::kinfu::Intr::scale", vec![(pred!(const, ["pyr"], ["int"]), _)]),
	#[inline]
	pub fn scale(self, pyr: i32) -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_scale_const_int(&self, pyr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// makeReprojector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:70
	// ("cv::kinfu::Intr::makeReprojector", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn make_reprojector(self) -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_makeReprojector_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// makeProjector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:71
	// ("cv::kinfu::Intr::makeProjector", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn make_projector(self) -> Result<crate::rgbd::Kinfu_Intr_Projector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_makeProjector_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMat()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:73
	// ("cv::kinfu::Intr::getMat", vec![(pred!(const, [], []), _)]),
	#[inline]
	pub fn get_mat(self) -> Result<core::Matx33f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_getMat_const(&self, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Intr()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:61
	// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Intr(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Intr(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:62
	// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, ["_fx", "_fy", "_cx", "_cy"], ["float", "float", "float", "float"]), _)]),
	#[inline]
	pub fn new(_fx: f32, _fy: f32, _cx: f32, _cy: f32) -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Intr_float_float_float_float(_fx, _fy, _cx, _cy, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Intr(cv::Matx33f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:63
	// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, ["m"], ["cv::Matx33f"]), _)]),
	#[inline]
	pub fn new_1(m: core::Matx33f) -> Result<crate::rgbd::Kinfu_Intr> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Intr_Matx33f(&m, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Projects camera space vector onto screen
// Projector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:39
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Kinfu_Intr_Projector {
	pub fx: f32,
	pub fy: f32,
	pub cx: f32,
	pub cy: f32,
}

opencv_type_simple! { crate::rgbd::Kinfu_Intr_Projector }

impl Kinfu_Intr_Projector {
	// Projector(Intr)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:41
	// ("cv::kinfu::Intr::Projector::Projector", vec![(pred!(mut, ["intr"], ["cv::kinfu::Intr"]), _)]),
	#[inline]
	pub fn new(intr: crate::rgbd::Kinfu_Intr) -> Result<crate::rgbd::Kinfu_Intr_Projector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Projector_Projector_Intr(&intr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Camera intrinsics
/// Reprojects screen point to camera space given z coord.
// Reprojector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:19
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Kinfu_Intr_Reprojector {
	pub fxinv: f32,
	pub fyinv: f32,
	pub cx: f32,
	pub cy: f32,
}

opencv_type_simple! { crate::rgbd::Kinfu_Intr_Reprojector }

impl Kinfu_Intr_Reprojector {
	// Reprojector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:21
	// ("cv::kinfu::Intr::Reprojector::Reprojector", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Reprojector_Reprojector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Reprojector(Intr)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:22
	// ("cv::kinfu::Intr::Reprojector::Reprojector", vec![(pred!(mut, ["intr"], ["cv::kinfu::Intr"]), _)]),
	#[inline]
	pub fn new(intr: crate::rgbd::Kinfu_Intr) -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Intr_Reprojector_Reprojector_Intr(&intr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Constant methods for [crate::rgbd::Kinfu_KinFu]
// KinFu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:192
pub trait Kinfu_KinFuTraitConst {
	fn as_raw_Kinfu_KinFu(&self) -> *const c_void;

	/// Get current parameters
	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:199
	// ("cv::kinfu::KinFu::getParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getParams_const(self.as_raw_Kinfu_KinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Renders a volume into an image
	///
	/// Renders a 0-surface of TSDF using Phong shading into a CV_8UC4 Mat.
	/// Light pose is fixed in KinFu params.
	///
	/// ## Parameters
	/// * image: resulting image
	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:209
	// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn render(&self, image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_render_const_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders a volume into an image
	///
	/// Renders a 0-surface of TSDF using Phong shading into a CV_8UC4 Mat.
	/// Light pose is fixed in KinFu params.
	///
	/// ## Parameters
	/// * image: resulting image
	/// * cameraPose: pose of camera to render from. If empty then render from current pose
	///   which is a last frame camera pose.
	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:221
	// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	#[inline]
	fn render_1(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_Kinfu_KinFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets points and normals of current 3d mesh
	///
	/// The order of normals corresponds to order of points.
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	/// * normals: vector of normals which are 4-float vectors
	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:231
	// ("cv::kinfu::KinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Gets points of current 3d mesh
	///
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:239
	// ("cv::kinfu::KinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates normals for given points
	/// ## Parameters
	/// * points: input vector of points which are 4-float vectors
	/// * normals: output vector of corresponding normals which are 4-float vectors
	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:245
	// ("cv::kinfu::KinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get current pose in voxel space
	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:254
	// ("cv::kinfu::KinFu::getPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_getPose_const(self.as_raw_Kinfu_KinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::Kinfu_KinFu]
pub trait Kinfu_KinFuTrait: crate::rgbd::Kinfu_KinFuTraitConst {
	fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void;

	/// Resets the algorithm
	///
	/// Clears current model and resets a pose.
	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:251
	// ("cv::kinfu::KinFu::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_reset(self.as_raw_mut_Kinfu_KinFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Process next depth frame
	///
	///   Integrates depth into voxel space with respect to its ICP-calculated pose.
	///   Input image is converted to CV_32F internally if has another type.
	///
	/// ## Parameters
	/// * depth: one-channel image which size and depth scale is described in algorithm's parameters
	/// ## Returns
	/// true if succeeded to align new frame with current scene, false if opposite
	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:264
	// ("cv::kinfu::KinFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn update(&mut self, depth: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_update_const__InputArrayR(self.as_raw_mut_Kinfu_KinFu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// KinectFusion implementation
///
/// This class implements a 3d reconstruction algorithm described in
/// [kinectfusion](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_kinectfusion) paper.
///
/// It takes a sequence of depth images taken from depth sensor
/// (or any depth images source such as stereo camera matching algorithm or even raymarching renderer).
/// The output can be obtained as a vector of points and their normals
/// or can be Phong-rendered from given camera pose.
///
/// An internal representation of a model is a voxel cuboid that keeps TSDF values
/// which are a sort of distances to the surface (for details read the [kinectfusion](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_kinectfusion) article about TSDF).
/// There is no interface to that representation yet.
///
/// KinFu uses OpenCL acceleration automatically if available.
/// To enable or disable it explicitly use cv::setUseOptimized() or cv::ocl::setUseOpenCL().
///
/// This implementation is based on [kinfu-remake](https://github.com/Nerei/kinfu_remake).
///
/// Note that the KinectFusion algorithm was patented and its use may be restricted by
/// the list of patents mentioned in README.md file in this module directory.
///
/// That's why you need to set the OPENCV_ENABLE_NONFREE option in CMake to use KinectFusion.
// KinFu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:192
pub struct Kinfu_KinFu {
	ptr: *mut c_void,
}

opencv_type_boxed! { Kinfu_KinFu }

impl Drop for Kinfu_KinFu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_kinfu_KinFu_delete(self.as_raw_mut_Kinfu_KinFu()) };
	}
}

unsafe impl Send for Kinfu_KinFu {}

impl crate::rgbd::Kinfu_KinFuTraitConst for Kinfu_KinFu {
	#[inline] fn as_raw_Kinfu_KinFu(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Kinfu_KinFuTrait for Kinfu_KinFu {
	#[inline] fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Kinfu_KinFu, crate::rgbd::Kinfu_KinFuTraitConst, as_raw_Kinfu_KinFu, crate::rgbd::Kinfu_KinFuTrait, as_raw_mut_Kinfu_KinFu }

impl Kinfu_KinFu {
	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:195
	// ("cv::kinfu::KinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::Kinfu_Params>) -> Result<core::Ptr<crate::rgbd::Kinfu_KinFu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_KinFu_create_const_PtrLParamsGR(_params.as_raw_PtrOfKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_KinFu>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Kinfu_KinFu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Kinfu_KinFu")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::Kinfu_Params]
// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:19
pub trait Kinfu_ParamsTraitConst {
	fn as_raw_Kinfu_Params(&self) -> *const c_void;

	/// frame size in pixels
	// cv::kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:83
	// ("cv::kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propFrameSize_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb frame size in pixels
	// cv::kinfu::Params::volumeType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:86
	// ("cv::kinfu::Params::volumeType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_type(&self) -> crate::rgbd::Kinfu_VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propVolumeType_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// camera intrinsics
	// cv::kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:89
	// ("cv::kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propIntr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb camera intrinsics
	// cv::kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:92
	// ("cv::kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rgb_intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propRgb_intr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// pre-scale per 1 meter for input values
	///
	/// Typical values are:
	///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
	///      * 1000 per 1 meter for Kinect 2 device
	///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
	// cv::kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:100
	// ("cv::kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propDepthFactor_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:103
	// ("cv::kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:105
	// ("cv::kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:107
	// ("cv::kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:110
	// ("cv::kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_propPyramidLevels_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:116
	// ("cv::kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_dims(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propVolumeDims_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Size of voxel in meters
	// cv::kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:118
	// ("cv::kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propVoxelSize_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:124
	// ("cv::kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// initial volume pose in meters
	// cv::kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:127
	// ("cv::kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propVolumePose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:133
	// ("cv::kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_trunc_dist_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:139
	// ("cv::kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_max_weight_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:145
	// ("cv::kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propRaycast_step_factor_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// light pose for rendering in meters
	// cv::kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:152
	// ("cv::kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propLightPose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance theshold for ICP in meters
	// cv::kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:155
	// ("cv::kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpDistThresh_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpAngleThresh_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:159
	// ("cv::kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_iterations(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpIterations_const(self.as_raw_Kinfu_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:165
	// ("cv::kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn truncate_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propTruncateThreshold_const(self.as_raw_Kinfu_Params()) };
		ret
	}

}

/// Mutable methods for [crate::rgbd::Kinfu_Params]
pub trait Kinfu_ParamsTrait: crate::rgbd::Kinfu_ParamsTraitConst {
	fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void;

	/// frame size in pixels
	// cv::kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:83
	// ("cv::kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// rgb frame size in pixels
	// cv::kinfu::Params::setVolumeType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:86
	// ("cv::kinfu::Params::setVolumeType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
	#[inline]
	fn set_volume_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
		let ret = unsafe { sys::cv_kinfu_Params_propVolumeType_const_VolumeType(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// camera intrinsics
	// cv::kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:89
	// ("cv::kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// rgb camera intrinsics
	// cv::kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:92
	// ("cv::kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_rgb_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_kinfu_Params_propRgb_intr_const_Matx33f(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// pre-scale per 1 meter for input values
	///
	/// Typical values are:
	///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
	///      * 1000 per 1 meter for Kinect 2 device
	///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
	// cv::kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:100
	// ("cv::kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:103
	// ("cv::kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:105
	// ("cv::kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:107
	// ("cv::kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:110
	// ("cv::kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:116
	// ("cv::kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	#[inline]
	fn set_volume_dims(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_kinfu_Params_propVolumeDims_const_Vec3i(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// Size of voxel in meters
	// cv::kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:118
	// ("cv::kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propVoxelSize_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:124
	// ("cv::kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// initial volume pose in meters
	// cv::kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:127
	// ("cv::kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
	#[inline]
	fn set_volume_pose(&mut self, val: core::Affine3f) {
		let ret = unsafe { sys::cv_kinfu_Params_propVolumePose_const_Affine3f(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:133
	// ("cv::kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_trunc_dist_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:139
	// ("cv::kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_tsdf_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_max_weight_const_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:145
	// ("cv::kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propRaycast_step_factor_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// light pose for rendering in meters
	// cv::kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:152
	// ("cv::kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// distance theshold for ICP in meters
	// cv::kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:155
	// ("cv::kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:159
	// ("cv::kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_Kinfu_Params(), val.as_raw_VectorOfi32()) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:165
	// ("cv::kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_truncate_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propTruncateThreshold_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Set Initial Volume Pose
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * R: rotation matrix
	/// * t: translation vector
	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:51
	// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	#[inline]
	fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_Kinfu_Params(), &r, &t, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set Initial Volume Pose
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * homogen_tf: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:58
	// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
	#[inline]
	fn set_initial_volume_pose_1(&mut self, homogen_tf: core::Matx44f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_Kinfu_Params(), &homogen_tf, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:19
pub struct Kinfu_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { Kinfu_Params }

impl Drop for Kinfu_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_kinfu_Params_delete(self.as_raw_mut_Kinfu_Params()) };
	}
}

unsafe impl Send for Kinfu_Params {}

impl crate::rgbd::Kinfu_ParamsTraitConst for Kinfu_Params {
	#[inline] fn as_raw_Kinfu_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Kinfu_ParamsTrait for Kinfu_Params {
	#[inline] fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Kinfu_Params, crate::rgbd::Kinfu_ParamsTraitConst, as_raw_Kinfu_Params, crate::rgbd::Kinfu_ParamsTrait, as_raw_mut_Kinfu_Params }

impl Kinfu_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:22
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor for Params
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * volumeInitialPoseRot: rotation matrix
	/// * volumeInitialPoseTransl: translation vector
	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:30
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	#[inline]
	pub fn new(volume_initial_pose_rot: core::Matx33f, volume_initial_pose_transl: core::Vec3f) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_Params_Matx33f_Vec3f(&volume_initial_pose_rot, &volume_initial_pose_transl, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor for Params
	/// Sets the initial pose of the TSDF volume.
	/// ## Parameters
	/// * volumeInitialPose: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:40
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
	#[inline]
	pub fn new_1(volume_initial_pose: core::Matx44f) -> Result<crate::rgbd::Kinfu_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_Params_Matx44f(&volume_initial_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Default parameters
	/// A set of parameters which provides better model quality, can be very slow.
	// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:64
	// ("cv::kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_params() -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_defaultParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Coarse parameters
	/// A set of parameters which provides better speed, can fail to match frames
	/// in case of rapid sensor motion.
	// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:70
	// ("cv::kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn coarse_params() -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_coarseParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// HashTSDF parameters
	/// A set of parameters suitable for use with HashTSDFVolume
	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:75
	// ("cv::kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	#[inline]
	pub fn hash_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_hashTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ColoredTSDF parameters
	/// A set of parameters suitable for use with ColoredTSDFVolume
	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:80
	// ("cv::kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	#[inline]
	pub fn colored_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::Kinfu_Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_coloredTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Kinfu_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Kinfu_Params")
			.field("frame_size", &crate::rgbd::Kinfu_ParamsTraitConst::frame_size(self))
			.field("volume_type", &crate::rgbd::Kinfu_ParamsTraitConst::volume_type(self))
			.field("intr", &crate::rgbd::Kinfu_ParamsTraitConst::intr(self))
			.field("rgb_intr", &crate::rgbd::Kinfu_ParamsTraitConst::rgb_intr(self))
			.field("depth_factor", &crate::rgbd::Kinfu_ParamsTraitConst::depth_factor(self))
			.field("bilateral_sigma_depth", &crate::rgbd::Kinfu_ParamsTraitConst::bilateral_sigma_depth(self))
			.field("bilateral_sigma_spatial", &crate::rgbd::Kinfu_ParamsTraitConst::bilateral_sigma_spatial(self))
			.field("bilateral_kernel_size", &crate::rgbd::Kinfu_ParamsTraitConst::bilateral_kernel_size(self))
			.field("pyramid_levels", &crate::rgbd::Kinfu_ParamsTraitConst::pyramid_levels(self))
			.field("volume_dims", &crate::rgbd::Kinfu_ParamsTraitConst::volume_dims(self))
			.field("voxel_size", &crate::rgbd::Kinfu_ParamsTraitConst::voxel_size(self))
			.field("tsdf_min_camera_movement", &crate::rgbd::Kinfu_ParamsTraitConst::tsdf_min_camera_movement(self))
			.field("volume_pose", &crate::rgbd::Kinfu_ParamsTraitConst::volume_pose(self))
			.field("tsdf_trunc_dist", &crate::rgbd::Kinfu_ParamsTraitConst::tsdf_trunc_dist(self))
			.field("tsdf_max_weight", &crate::rgbd::Kinfu_ParamsTraitConst::tsdf_max_weight(self))
			.field("raycast_step_factor", &crate::rgbd::Kinfu_ParamsTraitConst::raycast_step_factor(self))
			.field("light_pose", &crate::rgbd::Kinfu_ParamsTraitConst::light_pose(self))
			.field("icp_dist_thresh", &crate::rgbd::Kinfu_ParamsTraitConst::icp_dist_thresh(self))
			.field("icp_angle_thresh", &crate::rgbd::Kinfu_ParamsTraitConst::icp_angle_thresh(self))
			.field("icp_iterations", &crate::rgbd::Kinfu_ParamsTraitConst::icp_iterations(self))
			.field("truncate_threshold", &crate::rgbd::Kinfu_ParamsTraitConst::truncate_threshold(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::Kinfu_Volume]
// Volume /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:18
pub trait Kinfu_VolumeTraitConst {
	fn as_raw_Kinfu_Volume(&self) -> *const c_void;

	// cv::kinfu::Volume::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:49
	// ("cv::kinfu::Volume::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Volume_propVoxelSize_const(self.as_raw_Kinfu_Volume()) };
		ret
	}

	// cv::kinfu::Volume::voxelSizeInv() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:50
	// ("cv::kinfu::Volume::voxelSizeInv", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size_inv(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Volume_propVoxelSizeInv_const(self.as_raw_Kinfu_Volume()) };
		ret
	}

	// cv::kinfu::Volume::pose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:51
	// ("cv::kinfu::Volume::pose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_propPose_const(self.as_raw_Kinfu_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::kinfu::Volume::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:52
	// ("cv::kinfu::Volume::raycastStepFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Volume_propRaycastStepFactor_const(self.as_raw_Kinfu_Volume()) };
		ret
	}

	// raycast(const Matx44f &, const kinfu::Intr &, const Size &, OutputArray, OutputArray)(SimpleClass, SimpleClass, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:36
	// ("cv::kinfu::Volume::raycast", vec![(pred!(const, ["cameraPose", "intrinsics", "frameSize", "points", "normals"], ["const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::Size*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn raycast(&self, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_size: core::Size, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), &camera_pose, &intrinsics, &frame_size, points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// raycast(const Matx44f &, const kinfu::Intr &, const Size &, OutputArray, OutputArray, OutputArray)(SimpleClass, SimpleClass, SimpleClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:38
	// ("cv::kinfu::Volume::raycast", vec![(pred!(const, ["cameraPose", "intrinsics", "frameSize", "points", "normals", "colors"], ["const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::Size*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn raycast_1(&self, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_size: core::Size, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		output_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), &camera_pose, &intrinsics, &frame_size, points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// fetchNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:40
	// ("cv::kinfu::Volume::fetchNormals", vec![(pred!(const, ["points", "_normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn fetch_normals(&self, points: &impl ToInputArray, _normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(_normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), points.as_raw__InputArray(), _normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// fetchPointsNormals(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:41
	// ("cv::kinfu::Volume::fetchPointsNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn fetch_points_normals(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// fetchPointsNormalsColors(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:42
	// ("cv::kinfu::Volume::fetchPointsNormalsColors", vec![(pred!(const, ["unnamed", "unnamed", "unnamed"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn fetch_points_normals_colors(&self, unnamed: &mut impl ToOutputArray, unnamed_1: &mut impl ToOutputArray, unnamed_2: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(unnamed);
		output_array_arg!(unnamed_1);
		output_array_arg!(unnamed_2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), unnamed.as_raw__OutputArray(), unnamed_1.as_raw__OutputArray(), unnamed_2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::Kinfu_Volume]
pub trait Kinfu_VolumeTrait: crate::rgbd::Kinfu_VolumeTraitConst {
	fn as_raw_mut_Kinfu_Volume(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * frame_id: 0
	// integrate(InputArray, float, const Matx44f &, const kinfu::Intr &, const int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:31
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "depthFactor", "cameraPose", "intrinsics", "frameId"], ["const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const int"]), _)]),
	#[inline]
	fn integrate(&mut self, _depth: &impl ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_id: i32) -> Result<()> {
		input_array_arg!(_depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_int(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, frame_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Kinfu_VolumeTrait::integrate] function uses the following default values for its arguments:
	/// * frame_id: 0
	// cv::kinfu::Volume::integrate(InputArray, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:31
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "depthFactor", "cameraPose", "intrinsics"], ["const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*"]), _)]),
	#[inline]
	fn integrate_def(&mut self, _depth: &impl ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr) -> Result<()> {
		input_array_arg!(_depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * frame_id: 0
	// integrate(InputArray, InputArray, float, const Matx44f &, const kinfu::Intr &, const Intr &, const int)(InputArray, InputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:33
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "_rgb", "depthFactor", "cameraPose", "intrinsics", "rgb_intrinsics", "frameId"], ["const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::kinfu::Intr*", "const int"]), _)]),
	#[inline]
	fn integrate_1(&mut self, _depth: &impl ToInputArray, _rgb: &impl ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, rgb_intrinsics: crate::rgbd::Kinfu_Intr, frame_id: i32) -> Result<()> {
		input_array_arg!(_depth);
		input_array_arg!(_rgb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR_const_int(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), _rgb.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, &rgb_intrinsics, frame_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Kinfu_VolumeTrait::integrate] function uses the following default values for its arguments:
	/// * frame_id: 0
	// cv::kinfu::Volume::integrate(InputArray, InputArray, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:33
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "_rgb", "depthFactor", "cameraPose", "intrinsics", "rgb_intrinsics"], ["const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::kinfu::Intr*"]), _)]),
	#[inline]
	fn integrate_def_1(&mut self, _depth: &impl ToInputArray, _rgb: &impl ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, rgb_intrinsics: crate::rgbd::Kinfu_Intr) -> Result<()> {
		input_array_arg!(_depth);
		input_array_arg!(_rgb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), _rgb.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, &rgb_intrinsics, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:46
	// ("cv::kinfu::Volume::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Volume_reset(self.as_raw_mut_Kinfu_Volume(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Volume /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:18
pub struct Kinfu_Volume {
	ptr: *mut c_void,
}

opencv_type_boxed! { Kinfu_Volume }

impl Drop for Kinfu_Volume {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_kinfu_Volume_delete(self.as_raw_mut_Kinfu_Volume()) };
	}
}

unsafe impl Send for Kinfu_Volume {}

impl crate::rgbd::Kinfu_VolumeTraitConst for Kinfu_Volume {
	#[inline] fn as_raw_Kinfu_Volume(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Kinfu_VolumeTrait for Kinfu_Volume {
	#[inline] fn as_raw_mut_Kinfu_Volume(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Kinfu_Volume, crate::rgbd::Kinfu_VolumeTraitConst, as_raw_Kinfu_Volume, crate::rgbd::Kinfu_VolumeTrait, as_raw_mut_Kinfu_Volume }

impl Kinfu_Volume {
}

impl std::fmt::Debug for Kinfu_Volume {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Kinfu_Volume")
			.field("voxel_size", &crate::rgbd::Kinfu_VolumeTraitConst::voxel_size(self))
			.field("voxel_size_inv", &crate::rgbd::Kinfu_VolumeTraitConst::voxel_size_inv(self))
			.field("pose", &crate::rgbd::Kinfu_VolumeTraitConst::pose(self))
			.field("raycast_step_factor", &crate::rgbd::Kinfu_VolumeTraitConst::raycast_step_factor(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::Kinfu_VolumeParams]
// VolumeParams /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:62
pub trait Kinfu_VolumeParamsTraitConst {
	fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void;

	/// Type of Volume
	/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
	// cv::kinfu::VolumeParams::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:67
	// ("cv::kinfu::VolumeParams::type", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn typ(&self) -> crate::rgbd::Kinfu_VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_propType_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Resolution of voxel space
	/// Number of voxels in each dimension.
	/// Applicable only for TSDF Volume.
	/// HashTSDF volume only supports equal resolution in all three dimensions
	// cv::kinfu::VolumeParams::resolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:74
	// ("cv::kinfu::VolumeParams::resolution", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resolution(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_propResolution_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Resolution of volumeUnit in voxel space
	/// Number of voxels in each dimension for volumeUnit
	/// Applicable only for hashTSDF.
	// cv::kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:80
	// ("cv::kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn unit_resolution(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propUnitResolution_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Initial pose of the volume in meters
	// cv::kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:83
	// ("cv::kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pose(&self) -> core::Affine3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_propPose_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Length of voxels in meters
	// cv::kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:86
	// ("cv::kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propVoxelSize_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// TSDF truncation distance
	/// Distances greater than value from surface will be truncated to 1.0
	// cv::kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:91
	// ("cv::kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propTsdfTruncDist_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Max number of frames to integrate per voxel
	/// Represents the max number of frames over which a running average
	/// of the TSDF is calculated for a voxel
	// cv::kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:97
	// ("cv::kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propMaxWeight_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Threshold for depth truncation in meters
	/// Truncates the depth greater than threshold to 0
	// cv::kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:102
	// ("cv::kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_trunc_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propDepthTruncThreshold_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Length of single raycast step
	/// Describes the percentage of voxel length that is skipped per march
	// cv::kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:107
	// ("cv::kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propRaycastStepFactor_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

}

/// Mutable methods for [crate::rgbd::Kinfu_VolumeParams]
pub trait Kinfu_VolumeParamsTrait: crate::rgbd::Kinfu_VolumeParamsTraitConst {
	fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void;

	/// Type of Volume
	/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
	// cv::kinfu::VolumeParams::setType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:67
	// ("cv::kinfu::VolumeParams::setType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
	#[inline]
	fn set_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propType_const_VolumeType(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// Resolution of voxel space
	/// Number of voxels in each dimension.
	/// Applicable only for TSDF Volume.
	/// HashTSDF volume only supports equal resolution in all three dimensions
	// cv::kinfu::VolumeParams::setResolution(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:74
	// ("cv::kinfu::VolumeParams::setResolution", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	#[inline]
	fn set_resolution(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolution_const_Vec3i(self.as_raw_mut_Kinfu_VolumeParams(), &val) };
		ret
	}

	/// Resolution of volumeUnit in voxel space
	/// Number of voxels in each dimension for volumeUnit
	/// Applicable only for hashTSDF.
	///
	/// ## C++ default parameters
	/// * val: {0}
	// cv::kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:80
	// ("cv::kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_unit_resolution(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propUnitResolution_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// Initial pose of the volume in meters
	// cv::kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:83
	// ("cv::kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
	#[inline]
	fn set_pose(&mut self, val: core::Affine3f) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propPose_const_Affine3f(self.as_raw_mut_Kinfu_VolumeParams(), &val) };
		ret
	}

	/// Length of voxels in meters
	// cv::kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:86
	// ("cv::kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propVoxelSize_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// TSDF truncation distance
	/// Distances greater than value from surface will be truncated to 1.0
	// cv::kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:91
	// ("cv::kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propTsdfTruncDist_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// Max number of frames to integrate per voxel
	/// Represents the max number of frames over which a running average
	/// of the TSDF is calculated for a voxel
	// cv::kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:97
	// ("cv::kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propMaxWeight_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// Threshold for depth truncation in meters
	/// Truncates the depth greater than threshold to 0
	// cv::kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:102
	// ("cv::kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_depth_trunc_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propDepthTruncThreshold_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// Length of single raycast step
	/// Describes the percentage of voxel length that is skipped per march
	// cv::kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:107
	// ("cv::kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propRaycastStepFactor_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

}

// VolumeParams /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:62
pub struct Kinfu_VolumeParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { Kinfu_VolumeParams }

impl Drop for Kinfu_VolumeParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_kinfu_VolumeParams_delete(self.as_raw_mut_Kinfu_VolumeParams()) };
	}
}

unsafe impl Send for Kinfu_VolumeParams {}

impl crate::rgbd::Kinfu_VolumeParamsTraitConst for Kinfu_VolumeParams {
	#[inline] fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Kinfu_VolumeParamsTrait for Kinfu_VolumeParams {
	#[inline] fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Kinfu_VolumeParams, crate::rgbd::Kinfu_VolumeParamsTraitConst, as_raw_Kinfu_VolumeParams, crate::rgbd::Kinfu_VolumeParamsTrait, as_raw_mut_Kinfu_VolumeParams }

impl Kinfu_VolumeParams {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_kinfu_VolumeParams_defaultNew_const()) }
	}

	/// Default set of parameters that provide higher quality reconstruction
	/// at the cost of slow performance.
	// defaultParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:112
	// ("cv::kinfu::VolumeParams::defaultParams", vec![(pred!(mut, ["_volumeType"], ["cv::kinfu::VolumeType"]), _)]),
	#[inline]
	pub fn default_params(_volume_type: crate::rgbd::Kinfu_VolumeType) -> Result<core::Ptr<crate::rgbd::Kinfu_VolumeParams>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_defaultParams_VolumeType(_volume_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_VolumeParams>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Coarse set of parameters that provides relatively higher performance
	/// at the cost of reconstrution quality.
	// coarseParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:117
	// ("cv::kinfu::VolumeParams::coarseParams", vec![(pred!(mut, ["_volumeType"], ["cv::kinfu::VolumeType"]), _)]),
	#[inline]
	pub fn coarse_params(_volume_type: crate::rgbd::Kinfu_VolumeType) -> Result<core::Ptr<crate::rgbd::Kinfu_VolumeParams>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_coarseParams_VolumeType(_volume_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_VolumeParams>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Kinfu_VolumeParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Kinfu_VolumeParams")
			.field("typ", &crate::rgbd::Kinfu_VolumeParamsTraitConst::typ(self))
			.field("resolution", &crate::rgbd::Kinfu_VolumeParamsTraitConst::resolution(self))
			.field("unit_resolution", &crate::rgbd::Kinfu_VolumeParamsTraitConst::unit_resolution(self))
			.field("pose", &crate::rgbd::Kinfu_VolumeParamsTraitConst::pose(self))
			.field("voxel_size", &crate::rgbd::Kinfu_VolumeParamsTraitConst::voxel_size(self))
			.field("tsdf_trunc_dist", &crate::rgbd::Kinfu_VolumeParamsTraitConst::tsdf_trunc_dist(self))
			.field("max_weight", &crate::rgbd::Kinfu_VolumeParamsTraitConst::max_weight(self))
			.field("depth_trunc_threshold", &crate::rgbd::Kinfu_VolumeParamsTraitConst::depth_trunc_threshold(self))
			.field("raycast_step_factor", &crate::rgbd::Kinfu_VolumeParamsTraitConst::raycast_step_factor(self))
			.finish()
	}
}

impl Default for Kinfu_VolumeParams {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::rgbd::Kinfu_Detail_PoseGraph]
// PoseGraph /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:26
pub trait Kinfu_Detail_PoseGraphTraitConst {
	fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void;

	// isNodeExist(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:34
	// ("cv::kinfu::detail::PoseGraph::isNodeExist", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
	#[inline]
	fn is_node_exist(&self, node_id: size_t) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_isNodeExist_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isNodeFixed(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:36
	// ("cv::kinfu::detail::PoseGraph::isNodeFixed", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
	#[inline]
	fn is_node_fixed(&self, node_id: size_t) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_isNodeFixed_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNodePose(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:37
	// ("cv::kinfu::detail::PoseGraph::getNodePose", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
	#[inline]
	fn get_node_pose(&self, node_id: size_t) -> Result<core::Affine3d> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNodePose_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNodesIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:38
	// ("cv::kinfu::detail::PoseGraph::getNodesIds", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nodes_ids(&self) -> Result<core::Vector<size_t>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNodesIds_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getNumNodes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:39
	// ("cv::kinfu::detail::PoseGraph::getNumNodes", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_nodes(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNumNodes_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeStart(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:44
	// ("cv::kinfu::detail::PoseGraph::getEdgeStart", vec![(pred!(const, ["i"], ["size_t"]), _)]),
	#[inline]
	fn get_edge_start(&self, i: size_t) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getEdgeStart_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getEdgeEnd(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:45
	// ("cv::kinfu::detail::PoseGraph::getEdgeEnd", vec![(pred!(const, ["i"], ["size_t"]), _)]),
	#[inline]
	fn get_edge_end(&self, i: size_t) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getEdgeEnd_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNumEdges()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:46
	// ("cv::kinfu::detail::PoseGraph::getNumEdges", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_num_edges(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_getNumEdges_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isValid()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:49
	// ("cv::kinfu::detail::PoseGraph::isValid", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_valid(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_isValid_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// calcEnergy()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:56
	// ("cv::kinfu::detail::PoseGraph::calcEnergy", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn calc_energy(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_calcEnergy_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::Kinfu_Detail_PoseGraph]
pub trait Kinfu_Detail_PoseGraphTrait: crate::rgbd::Kinfu_Detail_PoseGraphTraitConst {
	fn as_raw_mut_Kinfu_Detail_PoseGraph(&mut self) -> *mut c_void;

	// addNode(size_t, const Affine3d &, bool)(Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:33
	// ("cv::kinfu::detail::PoseGraph::addNode", vec![(pred!(mut, ["_nodeId", "_pose", "fixed"], ["size_t", "const cv::Affine3d*", "bool"]), _)]),
	#[inline]
	fn add_node(&mut self, _node_id: size_t, _pose: core::Affine3d, fixed: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_addNode_size_t_const_Affine3dR_bool(self.as_raw_mut_Kinfu_Detail_PoseGraph(), _node_id, &_pose, fixed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNodeFixed(size_t, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:35
	// ("cv::kinfu::detail::PoseGraph::setNodeFixed", vec![(pred!(mut, ["nodeId", "fixed"], ["size_t", "bool"]), _)]),
	#[inline]
	fn set_node_fixed(&mut self, node_id: size_t, fixed: bool) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_setNodeFixed_size_t_bool(self.as_raw_mut_Kinfu_Detail_PoseGraph(), node_id, fixed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * _information: Matx66f::eye()
	// addEdge(size_t, size_t, const Affine3f &, const Matx66f &)(Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:42
	// ("cv::kinfu::detail::PoseGraph::addEdge", vec![(pred!(mut, ["_sourceNodeId", "_targetNodeId", "_transformation", "_information"], ["size_t", "size_t", "const cv::Affine3f*", "const cv::Matx66f*"]), _)]),
	#[inline]
	fn add_edge(&mut self, _source_node_id: size_t, _target_node_id: size_t, _transformation: core::Affine3f, _information: core::Matx66f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR_const_Matx66fR(self.as_raw_mut_Kinfu_Detail_PoseGraph(), _source_node_id, _target_node_id, &_transformation, &_information, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Kinfu_Detail_PoseGraphTrait::add_edge] function uses the following default values for its arguments:
	/// * _information: Matx66f::eye()
	// cv::kinfu::detail::PoseGraph::addEdge(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:42
	// ("cv::kinfu::detail::PoseGraph::addEdge", vec![(pred!(mut, ["_sourceNodeId", "_targetNodeId", "_transformation"], ["size_t", "size_t", "const cv::Affine3f*"]), _)]),
	#[inline]
	fn add_edge_def(&mut self, _source_node_id: size_t, _target_node_id: size_t, _transformation: core::Affine3f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR(self.as_raw_mut_Kinfu_Detail_PoseGraph(), _source_node_id, _target_node_id, &_transformation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * tc: cv::TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	// optimize(const cv::TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:53
	// ("cv::kinfu::detail::PoseGraph::optimize", vec![(pred!(mut, ["tc"], ["const cv::TermCriteria*"]), _)]),
	#[inline]
	fn optimize(&mut self, tc: core::TermCriteria) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_optimize_const_TermCriteriaR(self.as_raw_mut_Kinfu_Detail_PoseGraph(), &tc, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [Kinfu_Detail_PoseGraphTrait::optimize] function uses the following default values for its arguments:
	/// * tc: cv::TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
	// cv::kinfu::detail::PoseGraph::optimize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:53
	// ("cv::kinfu::detail::PoseGraph::optimize", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn optimize_def(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_optimize(self.as_raw_mut_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PoseGraph /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:26
pub struct Kinfu_Detail_PoseGraph {
	ptr: *mut c_void,
}

opencv_type_boxed! { Kinfu_Detail_PoseGraph }

impl Drop for Kinfu_Detail_PoseGraph {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_kinfu_detail_PoseGraph_delete(self.as_raw_mut_Kinfu_Detail_PoseGraph()) };
	}
}

unsafe impl Send for Kinfu_Detail_PoseGraph {}

impl crate::rgbd::Kinfu_Detail_PoseGraphTraitConst for Kinfu_Detail_PoseGraph {
	#[inline] fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::Kinfu_Detail_PoseGraphTrait for Kinfu_Detail_PoseGraph {
	#[inline] fn as_raw_mut_Kinfu_Detail_PoseGraph(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Kinfu_Detail_PoseGraph, crate::rgbd::Kinfu_Detail_PoseGraphTraitConst, as_raw_Kinfu_Detail_PoseGraph, crate::rgbd::Kinfu_Detail_PoseGraphTrait, as_raw_mut_Kinfu_Detail_PoseGraph }

impl Kinfu_Detail_PoseGraph {
	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:29
	// ("cv::kinfu::detail::PoseGraph::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_detail_PoseGraph_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Detail_PoseGraph>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Kinfu_Detail_PoseGraph {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Kinfu_Detail_PoseGraph")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::LargeKinfu]
// LargeKinfu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:120
pub trait LargeKinfuTraitConst {
	fn as_raw_LargeKinfu(&self) -> *const c_void;

	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:126
	// ("cv::large_kinfu::LargeKinfu::getParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_params(&self) -> Result<crate::rgbd::Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getParams_const(self.as_raw_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::Params::opencv_from_extern(ret) };
		Ok(ret)
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:128
	// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn render(&self, image: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:129
	// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	#[inline]
	fn render_1(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
		output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:131
	// ("cv::large_kinfu::LargeKinfu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:133
	// ("cv::large_kinfu::LargeKinfu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:135
	// ("cv::large_kinfu::LargeKinfu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:139
	// ("cv::large_kinfu::LargeKinfu::getPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_getPose_const(self.as_raw_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LargeKinfu]
pub trait LargeKinfuTrait: crate::rgbd::LargeKinfuTraitConst {
	fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void;

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:137
	// ("cv::large_kinfu::LargeKinfu::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_reset(self.as_raw_mut_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:141
	// ("cv::large_kinfu::LargeKinfu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn update(&mut self, depth: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_update_const__InputArrayR(self.as_raw_mut_LargeKinfu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Large Scale Dense Depth Fusion implementation
///
/// This class implements a 3d reconstruction algorithm for larger environments using
/// Spatially hashed TSDF volume "Submaps".
/// It also runs a periodic posegraph optimization to minimize drift in tracking over long sequences.
/// Currently the algorithm does not implement a relocalization or loop closure module.
/// Potentially a Bag of words implementation or RGBD relocalization as described in
/// Glocker et al. ISMAR 2013 will be implemented
///
/// It takes a sequence of depth images taken from depth sensor
/// (or any depth images source such as stereo camera matching algorithm or even raymarching
/// renderer). The output can be obtained as a vector of points and their normals or can be
/// Phong-rendered from given camera pose.
///
/// An internal representation of a model is a spatially hashed voxel cube that stores TSDF values
/// which represent the distance to the closest surface (for details read the [kinectfusion](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_kinectfusion) article
/// about TSDF). There is no interface to that representation yet.
///
/// For posegraph optimization, a Submap abstraction over the Volume class is created.
/// New submaps are added to the model when there is low visibility overlap between current viewing frustrum
/// and the existing volume/model. Multiple submaps are simultaneously tracked and a posegraph is created and
/// optimized periodically.
///
/// LargeKinfu does not use any OpenCL acceleration yet.
/// To enable or disable it explicitly use cv::setUseOptimized() or cv::ocl::setUseOpenCL().
///
/// This implementation is inspired from Kintinuous, InfiniTAM and other SOTA algorithms
///
/// You need to set the OPENCV_ENABLE_NONFREE option in CMake to use KinectFusion.
// LargeKinfu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:120
pub struct LargeKinfu {
	ptr: *mut c_void,
}

opencv_type_boxed! { LargeKinfu }

impl Drop for LargeKinfu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_large_kinfu_LargeKinfu_delete(self.as_raw_mut_LargeKinfu()) };
	}
}

unsafe impl Send for LargeKinfu {}

impl crate::rgbd::LargeKinfuTraitConst for LargeKinfu {
	#[inline] fn as_raw_LargeKinfu(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LargeKinfuTrait for LargeKinfu {
	#[inline] fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LargeKinfu, crate::rgbd::LargeKinfuTraitConst, as_raw_LargeKinfu, crate::rgbd::LargeKinfuTrait, as_raw_mut_LargeKinfu }

impl LargeKinfu {
	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:123
	// ("cv::large_kinfu::LargeKinfu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::large_kinfu::Params>*"]), _)]),
	#[inline]
	pub fn create(_params: &core::Ptr<crate::rgbd::Params>) -> Result<core::Ptr<crate::rgbd::LargeKinfu>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_create_const_PtrLParamsGR(_params.as_raw_PtrOfParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LargeKinfu>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for LargeKinfu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LargeKinfu")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::Params]
// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:20
pub trait ParamsTraitConst {
	fn as_raw_Params(&self) -> *const c_void;

	/// frame size in pixels
	// cv::large_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:39
	// ("cv::large_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propFrameSize_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// camera intrinsics
	// cv::large_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:42
	// ("cv::large_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propIntr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb camera intrinsics
	// cv::large_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:45
	// ("cv::large_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rgb_intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propRgb_intr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// pre-scale per 1 meter for input values
	/// Typical values are:
	///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
	///      * 1000 per 1 meter for Kinect 2 device
	///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
	// cv::large_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:53
	// ("cv::large_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propDepthFactor_const(self.as_raw_Params()) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::large_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:56
	// ("cv::large_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_Params()) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::large_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_Params()) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::large_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:60
	// ("cv::large_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_Params()) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::large_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:63
	// ("cv::large_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propPyramidLevels_const(self.as_raw_Params()) };
		ret
	}

	/// Minimal camera movement in meters
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::large_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:68
	// ("cv::large_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_Params()) };
		ret
	}

	/// light pose for rendering in meters
	// cv::large_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propLightPose_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance theshold for ICP in meters
	// cv::large_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:74
	// ("cv::large_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpDistThresh_const(self.as_raw_Params()) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::large_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:76
	// ("cv::large_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpAngleThresh_const(self.as_raw_Params()) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::large_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:78
	// ("cv::large_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_iterations(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpIterations_const(self.as_raw_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}

	/// Threshold for depth truncation in meters
	/// All depth values beyond this threshold will be set to zero
	// cv::large_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:83
	// ("cv::large_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn truncate_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_const(self.as_raw_Params()) };
		ret
	}

	/// Volume parameters
	// cv::large_kinfu::Params::volumeParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:87
	// ("cv::large_kinfu::Params::volumeParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_params(&self) -> crate::rgbd::Kinfu_VolumeParams {
		let ret = unsafe { sys::cv_large_kinfu_Params_propVolumeParams_const(self.as_raw_Params()) };
		let ret = unsafe { crate::rgbd::Kinfu_VolumeParams::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::rgbd::Params]
pub trait ParamsTrait: crate::rgbd::ParamsTraitConst {
	fn as_raw_mut_Params(&mut self) -> *mut c_void;

	/// frame size in pixels
	// cv::large_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:39
	// ("cv::large_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_Params(), &val) };
		ret
	}

	/// camera intrinsics
	// cv::large_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:42
	// ("cv::large_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_Params(), &val) };
		ret
	}

	/// rgb camera intrinsics
	// cv::large_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:45
	// ("cv::large_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_rgb_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propRgb_intr_const_Matx33f(self.as_raw_mut_Params(), &val) };
		ret
	}

	/// pre-scale per 1 meter for input values
	/// Typical values are:
	///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
	///      * 1000 per 1 meter for Kinect 2 device
	///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
	// cv::large_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:53
	// ("cv::large_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::large_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:56
	// ("cv::large_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::large_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::large_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:60
	// ("cv::large_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::large_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:63
	// ("cv::large_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Minimal camera movement in meters
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::large_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:68
	// ("cv::large_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// light pose for rendering in meters
	// cv::large_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_Params(), &val) };
		ret
	}

	/// distance theshold for ICP in meters
	// cv::large_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:74
	// ("cv::large_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::large_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:76
	// ("cv::large_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::large_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:78
	// ("cv::large_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_Params(), val.as_raw_VectorOfi32()) };
		ret
	}

	/// Threshold for depth truncation in meters
	/// All depth values beyond this threshold will be set to zero
	// cv::large_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:83
	// ("cv::large_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_truncate_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Volume parameters
	// cv::large_kinfu::Params::setVolumeParams(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:87
	// ("cv::large_kinfu::Params::setVolumeParams", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeParams"]), _)]),
	#[inline]
	fn set_volume_params(&mut self, val: crate::rgbd::Kinfu_VolumeParams) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propVolumeParams_const_VolumeParams(self.as_raw_mut_Params(), val.as_raw_Kinfu_VolumeParams()) };
		ret
	}

}

// Params /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:20
pub struct Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { Params }

impl Drop for Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_large_kinfu_Params_delete(self.as_raw_mut_Params()) };
	}
}

unsafe impl Send for Params {}

impl crate::rgbd::ParamsTraitConst for Params {
	#[inline] fn as_raw_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::ParamsTrait for Params {
	#[inline] fn as_raw_mut_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Params, crate::rgbd::ParamsTraitConst, as_raw_Params, crate::rgbd::ParamsTrait, as_raw_mut_Params }

impl Params {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_large_kinfu_Params_defaultNew_const()) }
	}

	/// Default parameters
	/// A set of parameters which provides better model quality, can be very slow.
	// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:25
	// ("cv::large_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_params() -> Result<core::Ptr<crate::rgbd::Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_defaultParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Coarse parameters
	/// A set of parameters which provides better speed, can fail to match frames
	/// in case of rapid sensor motion.
	// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:31
	// ("cv::large_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn coarse_params() -> Result<core::Ptr<crate::rgbd::Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_coarseParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// HashTSDF parameters
	/// A set of parameters suitable for use with HashTSDFVolume
	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:36
	// ("cv::large_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	#[inline]
	pub fn hash_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::Params>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_hashTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Params")
			.field("frame_size", &crate::rgbd::ParamsTraitConst::frame_size(self))
			.field("intr", &crate::rgbd::ParamsTraitConst::intr(self))
			.field("rgb_intr", &crate::rgbd::ParamsTraitConst::rgb_intr(self))
			.field("depth_factor", &crate::rgbd::ParamsTraitConst::depth_factor(self))
			.field("bilateral_sigma_depth", &crate::rgbd::ParamsTraitConst::bilateral_sigma_depth(self))
			.field("bilateral_sigma_spatial", &crate::rgbd::ParamsTraitConst::bilateral_sigma_spatial(self))
			.field("bilateral_kernel_size", &crate::rgbd::ParamsTraitConst::bilateral_kernel_size(self))
			.field("pyramid_levels", &crate::rgbd::ParamsTraitConst::pyramid_levels(self))
			.field("tsdf_min_camera_movement", &crate::rgbd::ParamsTraitConst::tsdf_min_camera_movement(self))
			.field("light_pose", &crate::rgbd::ParamsTraitConst::light_pose(self))
			.field("icp_dist_thresh", &crate::rgbd::ParamsTraitConst::icp_dist_thresh(self))
			.field("icp_angle_thresh", &crate::rgbd::ParamsTraitConst::icp_angle_thresh(self))
			.field("icp_iterations", &crate::rgbd::ParamsTraitConst::icp_iterations(self))
			.field("truncate_threshold", &crate::rgbd::ParamsTraitConst::truncate_threshold(self))
			.field("volume_params", &crate::rgbd::ParamsTraitConst::volume_params(self))
			.finish()
	}
}

impl Default for Params {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::rgbd::LineMod_ColorGradient]
// ColorGradient /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:166
pub trait LineMod_ColorGradientTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
	fn as_raw_LineMod_ColorGradient(&self) -> *const c_void;

	// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weak_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:193
	// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn strong_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:186
	// ("cv::linemod::ColorGradient::name", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_name_const(self.as_raw_LineMod_ColorGradient(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:189
	// ("cv::linemod::ColorGradient::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_write_const_FileStorageR(self.as_raw_LineMod_ColorGradient(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LineMod_ColorGradient]
pub trait LineMod_ColorGradientTrait: crate::rgbd::LineMod_ColorGradientTraitConst + crate::rgbd::LineMod_ModalityTrait {
	fn as_raw_mut_LineMod_ColorGradient(&mut self) -> *mut c_void;

	// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_weak_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const_size_t(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:193
	// ("cv::linemod::ColorGradient::setStrong_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_strong_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:188
	// ("cv::linemod::ColorGradient::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_read_const_FileNodeR(self.as_raw_mut_LineMod_ColorGradient(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief Modality that computes quantized gradient orientations from a color image.
// ColorGradient /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:166
pub struct LineMod_ColorGradient {
	ptr: *mut c_void,
}

opencv_type_boxed! { LineMod_ColorGradient }

impl Drop for LineMod_ColorGradient {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_linemod_ColorGradient_delete(self.as_raw_mut_LineMod_ColorGradient()) };
	}
}

unsafe impl Send for LineMod_ColorGradient {}

impl crate::rgbd::LineMod_ModalityTraitConst for LineMod_ColorGradient {
	#[inline] fn as_raw_LineMod_Modality(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_ModalityTrait for LineMod_ColorGradient {
	#[inline] fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_ColorGradient, crate::rgbd::LineMod_ModalityTraitConst, as_raw_LineMod_Modality, crate::rgbd::LineMod_ModalityTrait, as_raw_mut_LineMod_Modality }

impl crate::rgbd::LineMod_ColorGradientTraitConst for LineMod_ColorGradient {
	#[inline] fn as_raw_LineMod_ColorGradient(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_ColorGradientTrait for LineMod_ColorGradient {
	#[inline] fn as_raw_mut_LineMod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_ColorGradient, crate::rgbd::LineMod_ColorGradientTraitConst, as_raw_LineMod_ColorGradient, crate::rgbd::LineMod_ColorGradientTrait, as_raw_mut_LineMod_ColorGradient }

impl LineMod_ColorGradient {
	/// \brief Default constructor. Uses reasonable default parameter values.
	// ColorGradient()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:172
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::LineMod_ColorGradient> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_ColorGradient(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_ColorGradient::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Constructor.
	///
	/// \param weak_threshold   When quantizing, discard gradients with magnitude less than this.
	/// \param num_features     How many features a template must contain.
	/// \param strong_threshold Consider as candidate features only gradients whose norms are
	///                         larger than this.
	// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:182
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	#[inline]
	pub fn new(weak_threshold: f32, num_features: size_t, strong_threshold: f32) -> Result<crate::rgbd::LineMod_ColorGradient> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold, num_features, strong_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_ColorGradient::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:184
	// ("cv::linemod::ColorGradient::create", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	#[inline]
	pub fn create(weak_threshold: f32, num_features: size_t, strong_threshold: f32) -> Result<core::Ptr<crate::rgbd::LineMod_ColorGradient>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_ColorGradient_create_float_size_t_float(weak_threshold, num_features, strong_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_ColorGradient>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LineMod_ColorGradient, crate::rgbd::LineMod_Modality, cv_linemod_ColorGradient_to_LineMod_Modality }

impl std::fmt::Debug for LineMod_ColorGradient {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineMod_ColorGradient")
			.field("weak_threshold", &crate::rgbd::LineMod_ColorGradientTraitConst::weak_threshold(self))
			.field("num_features", &crate::rgbd::LineMod_ColorGradientTraitConst::num_features(self))
			.field("strong_threshold", &crate::rgbd::LineMod_ColorGradientTraitConst::strong_threshold(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::LineMod_DepthNormal]
// DepthNormal /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:203
pub trait LineMod_DepthNormalTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
	fn as_raw_LineMod_DepthNormal(&self) -> *const c_void;

	// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn distance_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn difference_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:235
	// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn extract_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:227
	// ("cv::linemod::DepthNormal::name", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_name_const(self.as_raw_LineMod_DepthNormal(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:230
	// ("cv::linemod::DepthNormal::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_write_const_FileStorageR(self.as_raw_LineMod_DepthNormal(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LineMod_DepthNormal]
pub trait LineMod_DepthNormalTrait: crate::rgbd::LineMod_DepthNormalTraitConst + crate::rgbd::LineMod_ModalityTrait {
	fn as_raw_mut_LineMod_DepthNormal(&mut self) -> *mut c_void;

	// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_distance_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_difference_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const_size_t(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:235
	// ("cv::linemod::DepthNormal::setExtract_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_extract_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:229
	// ("cv::linemod::DepthNormal::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_read_const_FileNodeR(self.as_raw_mut_LineMod_DepthNormal(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief Modality that computes quantized surface normals from a dense depth map.
// DepthNormal /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:203
pub struct LineMod_DepthNormal {
	ptr: *mut c_void,
}

opencv_type_boxed! { LineMod_DepthNormal }

impl Drop for LineMod_DepthNormal {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_linemod_DepthNormal_delete(self.as_raw_mut_LineMod_DepthNormal()) };
	}
}

unsafe impl Send for LineMod_DepthNormal {}

impl crate::rgbd::LineMod_ModalityTraitConst for LineMod_DepthNormal {
	#[inline] fn as_raw_LineMod_Modality(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_ModalityTrait for LineMod_DepthNormal {
	#[inline] fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_DepthNormal, crate::rgbd::LineMod_ModalityTraitConst, as_raw_LineMod_Modality, crate::rgbd::LineMod_ModalityTrait, as_raw_mut_LineMod_Modality }

impl crate::rgbd::LineMod_DepthNormalTraitConst for LineMod_DepthNormal {
	#[inline] fn as_raw_LineMod_DepthNormal(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_DepthNormalTrait for LineMod_DepthNormal {
	#[inline] fn as_raw_mut_LineMod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_DepthNormal, crate::rgbd::LineMod_DepthNormalTraitConst, as_raw_LineMod_DepthNormal, crate::rgbd::LineMod_DepthNormalTrait, as_raw_mut_LineMod_DepthNormal }

impl LineMod_DepthNormal {
	/// \brief Default constructor. Uses reasonable default parameter values.
	// DepthNormal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:209
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::LineMod_DepthNormal> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_DepthNormal(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_DepthNormal::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Constructor.
	///
	/// \param distance_threshold   Ignore pixels beyond this distance.
	/// \param difference_threshold When computing normals, ignore contributions of pixels whose
	///                             depth difference with the central pixel is above this threshold.
	/// \param num_features         How many features a template must contain.
	/// \param extract_threshold    Consider as candidate feature only if there are no differing
	///                             orientations within a distance of extract_threshold.
	// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:221
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	#[inline]
	pub fn new(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32) -> Result<crate::rgbd::LineMod_DepthNormal> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold, difference_threshold, num_features, extract_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_DepthNormal::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:224
	// ("cv::linemod::DepthNormal::create", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	#[inline]
	pub fn create(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32) -> Result<core::Ptr<crate::rgbd::LineMod_DepthNormal>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_DepthNormal_create_int_int_size_t_int(distance_threshold, difference_threshold, num_features, extract_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_DepthNormal>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LineMod_DepthNormal, crate::rgbd::LineMod_Modality, cv_linemod_DepthNormal_to_LineMod_Modality }

impl std::fmt::Debug for LineMod_DepthNormal {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineMod_DepthNormal")
			.field("distance_threshold", &crate::rgbd::LineMod_DepthNormalTraitConst::distance_threshold(self))
			.field("difference_threshold", &crate::rgbd::LineMod_DepthNormalTraitConst::difference_threshold(self))
			.field("num_features", &crate::rgbd::LineMod_DepthNormalTraitConst::num_features(self))
			.field("extract_threshold", &crate::rgbd::LineMod_DepthNormalTraitConst::extract_threshold(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::LineMod_Detector]
// Detector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:298
pub trait LineMod_DetectorTraitConst {
	fn as_raw_LineMod_Detector(&self) -> *const c_void;

	/// \brief Detect objects by template matching.
	///
	/// Matches globally at the lowest pyramid level, then refines locally stepping up the pyramid.
	///
	/// \param      sources   Source images, one for each modality.
	/// \param      threshold Similarity threshold, a percentage between 0 and 100.
	/// \param[out] matches   Template matches, sorted by similarity score.
	/// \param      class_ids If non-empty, only search for the desired object classes.
	/// \param[out] quantized_images Optionally return vector<Mat> of quantized images.
	/// \param      masks     The masks for consideration during matching. The masks should be CV_8UC1
	///                       where 255 represents a valid pixel.  If non-empty, the vector must be
	///                       the same size as sources.  Each element must be
	///                       empty or the same size as its corresponding source.
	///
	/// ## C++ default parameters
	/// * class_ids: std::vector<String>()
	/// * quantized_images: noArray()
	/// * masks: std::vector<Mat>()
	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:330
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches", "class_ids", "quantized_images", "masks"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*", "const std::vector<cv::String>*", "const cv::_OutputArray*", "const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn match_(&self, sources: &core::Vector<core::Mat>, threshold: f32, matches: &mut core::Vector<crate::rgbd::LineMod_Match>, class_ids: &core::Vector<String>, quantized_images: &mut impl ToOutputArray, masks: &core::Vector<core::Mat>) -> Result<()> {
		output_array_arg!(quantized_images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(self.as_raw_LineMod_Detector(), sources.as_raw_VectorOfMat(), threshold, matches.as_raw_mut_VectorOfLineMod_Match(), class_ids.as_raw_VectorOfString(), quantized_images.as_raw__OutputArray(), masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Detect objects by template matching.
	///
	/// Matches globally at the lowest pyramid level, then refines locally stepping up the pyramid.
	///
	/// \param      sources   Source images, one for each modality.
	/// \param      threshold Similarity threshold, a percentage between 0 and 100.
	/// \param[out] matches   Template matches, sorted by similarity score.
	/// \param      class_ids If non-empty, only search for the desired object classes.
	/// \param[out] quantized_images Optionally return vector<Mat> of quantized images.
	/// \param      masks     The masks for consideration during matching. The masks should be CV_8UC1
	///                       where 255 represents a valid pixel.  If non-empty, the vector must be
	///                       the same size as sources.  Each element must be
	///                       empty or the same size as its corresponding source.
	///
	/// ## Note
	/// This alternative version of [LineMod_DetectorTraitConst::match_] function uses the following default values for its arguments:
	/// * class_ids: std::vector<String>()
	/// * quantized_images: noArray()
	/// * masks: std::vector<Mat>()
	// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:330
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*"]), _)]),
	#[inline]
	fn match__def(&self, sources: &core::Vector<core::Mat>, threshold: f32, matches: &mut core::Vector<crate::rgbd::LineMod_Match>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(self.as_raw_LineMod_Detector(), sources.as_raw_VectorOfMat(), threshold, matches.as_raw_mut_VectorOfLineMod_Match(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Get the modalities used by this detector.
	///
	/// You are not permitted to add/remove modalities, but you may dynamic_cast them to
	/// tweak parameters.
	// getModalities()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:359
	// ("cv::linemod::Detector::getModalities", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_modalities(&self) -> Result<core::Vector<core::Ptr<crate::rgbd::LineMod_Modality>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_getModalities_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Ptr<crate::rgbd::LineMod_Modality>>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Get sampling step T at pyramid_level.
	// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:364
	// ("cv::linemod::Detector::getT", vec![(pred!(const, ["pyramid_level"], ["int"]), _)]),
	#[inline]
	fn get_t(&self, pyramid_level: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_getT_const_int(self.as_raw_LineMod_Detector(), pyramid_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Get number of pyramid levels used by this detector.
	// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:369
	// ("cv::linemod::Detector::pyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_pyramidLevels_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Get the template pyramid identified by template_id.
	///
	/// For example, with 2 modalities (Gradient, Normal) and two pyramid levels
	/// (L0, L1), the order is (GradientL0, NormalL0, GradientL1, NormalL1).
	// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:377
	// ("cv::linemod::Detector::getTemplates", vec![(pred!(const, ["class_id", "template_id"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn get_templates(&self, class_id: &str, template_id: i32) -> Result<core::Vector<crate::rgbd::LineMod_Template>> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_getTemplates_const_const_StringR_int(self.as_raw_LineMod_Detector(), class_id.opencv_as_extern(), template_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::rgbd::LineMod_Template>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// numTemplates()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:379
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_templates(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numTemplates_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:380
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, ["class_id"], ["const cv::String*"]), _)]),
	#[inline]
	fn num_templates_1(&self, class_id: &str) -> Result<i32> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numTemplates_const_const_StringR(self.as_raw_LineMod_Detector(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// numClasses()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:381
	// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_classes(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numClasses_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// classIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:383
	// ("cv::linemod::Detector::classIds", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn class_ids(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_classIds_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:386
	// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_write_const_FileStorageR(self.as_raw_LineMod_Detector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:389
	// ("cv::linemod::Detector::writeClass", vec![(pred!(const, ["class_id", "fs"], ["const cv::String*", "cv::FileStorage*"]), _)]),
	#[inline]
	fn write_class(&self, class_id: &str, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(self.as_raw_LineMod_Detector(), class_id.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * format: "templates_%s.yml.gz"
	// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:393
	// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, ["format"], ["const cv::String*"]), _)]),
	#[inline]
	fn write_classes(&self, format: &str) -> Result<()> {
		extern_container_arg!(format);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_writeClasses_const_const_StringR(self.as_raw_LineMod_Detector(), format.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [LineMod_DetectorTraitConst::write_classes] function uses the following default values for its arguments:
	/// * format: "templates_%s.yml.gz"
	// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:393
	// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn write_classes_def(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_writeClasses_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LineMod_Detector]
pub trait LineMod_DetectorTrait: crate::rgbd::LineMod_DetectorTraitConst {
	fn as_raw_mut_LineMod_Detector(&mut self) -> *mut c_void;

	/// \brief Add new object template.
	///
	/// \param      sources      Source images, one for each modality.
	/// \param      class_id     Object class ID.
	/// \param      object_mask  Mask separating object from background.
	/// \param[out] bounding_box Optionally return bounding box of the extracted features.
	///
	/// \return Template ID, or -1 if failed to extract a valid template.
	///
	/// ## C++ default parameters
	/// * bounding_box: NULL
	// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:345
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask", "bounding_box"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*", "cv::Rect*"]), _)]),
	#[inline]
	fn add_template(&mut self, sources: &core::Vector<core::Mat>, class_id: &str, object_mask: &impl core::MatTraitConst, bounding_box: &mut core::Rect) -> Result<i32> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(self.as_raw_mut_LineMod_Detector(), sources.as_raw_VectorOfMat(), class_id.opencv_as_extern(), object_mask.as_raw_Mat(), bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Add new object template.
	///
	/// \param      sources      Source images, one for each modality.
	/// \param      class_id     Object class ID.
	/// \param      object_mask  Mask separating object from background.
	/// \param[out] bounding_box Optionally return bounding box of the extracted features.
	///
	/// \return Template ID, or -1 if failed to extract a valid template.
	///
	/// ## Note
	/// This alternative version of [LineMod_DetectorTrait::add_template] function uses the following default values for its arguments:
	/// * bounding_box: NULL
	// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:345
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*"]), _)]),
	#[inline]
	fn add_template_def(&mut self, sources: &core::Vector<core::Mat>, class_id: &str, object_mask: &impl core::MatTraitConst) -> Result<i32> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(self.as_raw_mut_LineMod_Detector(), sources.as_raw_VectorOfMat(), class_id.opencv_as_extern(), object_mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Add a new object template computed by external means.
	// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:351
	// ("cv::linemod::Detector::addSyntheticTemplate", vec![(pred!(mut, ["templates", "class_id"], ["const std::vector<cv::linemod::Template>*", "const cv::String*"]), _)]),
	#[inline]
	fn add_synthetic_template(&mut self, templates: &core::Vector<crate::rgbd::LineMod_Template>, class_id: &str) -> Result<i32> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(self.as_raw_mut_LineMod_Detector(), templates.as_raw_VectorOfLineMod_Template(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:385
	// ("cv::linemod::Detector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_read_const_FileNodeR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * class_id_override: ""
	// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:388
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn", "class_id_override"], ["const cv::FileNode*", "const cv::String*"]), _)]),
	#[inline]
	fn read_class(&mut self, fn_: &impl core::FileNodeTraitConst, class_id_override: &str) -> Result<String> {
		extern_container_arg!(class_id_override);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), class_id_override.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [LineMod_DetectorTrait::read_class] function uses the following default values for its arguments:
	/// * class_id_override: ""
	// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:388
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read_class_def(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * format: "templates_%s.yml.gz"
	// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:391
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids", "format"], ["const std::vector<cv::String>*", "const cv::String*"]), _)]),
	#[inline]
	fn read_classes(&mut self, class_ids: &core::Vector<String>, format: &str) -> Result<()> {
		extern_container_arg!(format);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_readClasses_const_vectorLStringGR_const_StringR(self.as_raw_mut_LineMod_Detector(), class_ids.as_raw_VectorOfString(), format.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [LineMod_DetectorTrait::read_classes] function uses the following default values for its arguments:
	/// * format: "templates_%s.yml.gz"
	// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:391
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids"], ["const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn read_classes_def(&mut self, class_ids: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_readClasses_const_vectorLStringGR(self.as_raw_mut_LineMod_Detector(), class_ids.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief Object detector using the LINE template matching algorithm with any set of
/// modalities.
// Detector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:298
pub struct LineMod_Detector {
	ptr: *mut c_void,
}

opencv_type_boxed! { LineMod_Detector }

impl Drop for LineMod_Detector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_linemod_Detector_delete(self.as_raw_mut_LineMod_Detector()) };
	}
}

unsafe impl Send for LineMod_Detector {}

impl crate::rgbd::LineMod_DetectorTraitConst for LineMod_Detector {
	#[inline] fn as_raw_LineMod_Detector(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_DetectorTrait for LineMod_Detector {
	#[inline] fn as_raw_mut_LineMod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_Detector, crate::rgbd::LineMod_DetectorTraitConst, as_raw_LineMod_Detector, crate::rgbd::LineMod_DetectorTrait, as_raw_mut_LineMod_Detector }

impl LineMod_Detector {
	/// \brief Empty constructor, initialize with read().
	// Detector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:304
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::LineMod_Detector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_Detector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_Detector::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Constructor.
	///
	/// \param modalities       Modalities to use (color gradients, depth normals, ...).
	/// \param T_pyramid        Value of the sampling step T at each pyramid level. The
	///                         number of pyramid levels is T_pyramid.size().
	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:313
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, ["modalities", "T_pyramid"], ["const std::vector<cv::Ptr<cv::linemod::Modality>>*", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn new(modalities: &core::Vector<core::Ptr<crate::rgbd::LineMod_Modality>>, t_pyramid: &core::Vector<i32>) -> Result<crate::rgbd::LineMod_Detector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_Detector_const_vectorLPtrLModalityGGR_const_vectorLintGR(modalities.as_raw_VectorOfPtrOfLineMod_Modality(), t_pyramid.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_Detector::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for LineMod_Detector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineMod_Detector")
			.finish()
	}
}

/// \brief Discriminant feature described by its location and label.
// Feature /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:26
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LineMod_Feature {
	/// x offset
	pub x: i32,
	/// y offset
	pub y: i32,
	/// Quantization
	pub label: i32,
}

opencv_type_simple! { crate::rgbd::LineMod_Feature }

impl LineMod_Feature {
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:36
	// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	pub fn write(self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_write_const_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Feature()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:32
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::LineMod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:33
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn new(x: i32, y: i32, label: i32) -> Result<crate::rgbd::LineMod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature_int_int_int(x, y, label, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:35
	// ("cv::linemod::Feature::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	pub fn read(self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_read_const_FileNodeR(&self, fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Constant methods for [crate::rgbd::LineMod_Match]
// Match /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:259
pub trait LineMod_MatchTraitConst {
	fn as_raw_LineMod_Match(&self) -> *const c_void;

	// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn x(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propX_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn y(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propY_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn similarity(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn class_id(&self) -> String {
		let ret = unsafe { sys::cv_linemod_Match_propClass_id_const(self.as_raw_LineMod_Match()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn template_id(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const(self.as_raw_LineMod_Match()) };
		ret
	}

	/// Sort matches with high similarity to the front
	// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:268
	// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	#[inline]
	fn less_than(&self, rhs: &impl crate::rgbd::LineMod_MatchTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_operatorL_const_const_MatchR(self.as_raw_LineMod_Match(), rhs.as_raw_LineMod_Match(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:277
	// ("cv::linemod::Match::operator==", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	#[inline]
	fn equals(&self, rhs: &impl crate::rgbd::LineMod_MatchTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_operatorEQ_const_const_MatchR(self.as_raw_LineMod_Match(), rhs.as_raw_LineMod_Match(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LineMod_Match]
pub trait LineMod_MatchTrait: crate::rgbd::LineMod_MatchTraitConst {
	fn as_raw_mut_LineMod_Match(&mut self) -> *mut c_void;

	// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_x(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propX_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_y(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propY_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_similarity(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const_float(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_class_id(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_linemod_Match_propClass_id_const_String(self.as_raw_mut_LineMod_Match(), val.opencv_as_extern()) };
		ret
	}

	// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_template_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

}

/// \brief Represents a successful template match.
// Match /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:259
pub struct LineMod_Match {
	ptr: *mut c_void,
}

opencv_type_boxed! { LineMod_Match }

impl Drop for LineMod_Match {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_linemod_Match_delete(self.as_raw_mut_LineMod_Match()) };
	}
}

unsafe impl Send for LineMod_Match {}

impl crate::rgbd::LineMod_MatchTraitConst for LineMod_Match {
	#[inline] fn as_raw_LineMod_Match(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_MatchTrait for LineMod_Match {
	#[inline] fn as_raw_mut_LineMod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_Match, crate::rgbd::LineMod_MatchTraitConst, as_raw_LineMod_Match, crate::rgbd::LineMod_MatchTrait, as_raw_mut_LineMod_Match }

impl LineMod_Match {
	// Match()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:261
	// ("cv::linemod::Match::Match", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::LineMod_Match> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_Match(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_Match::opencv_from_extern(ret) };
		Ok(ret)
	}

	// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:265
	// ("cv::linemod::Match::Match", vec![(pred!(mut, ["x", "y", "similarity", "class_id", "template_id"], ["int", "int", "float", "const cv::String*", "int"]), _)]),
	#[inline]
	pub fn new(x: i32, y: i32, similarity: f32, class_id: &str, template_id: i32) -> Result<crate::rgbd::LineMod_Match> {
		extern_container_arg!(class_id);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_Match_int_int_float_const_StringR_int(x, y, similarity, class_id.opencv_as_extern(), template_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::LineMod_Match::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for LineMod_Match {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_linemod_Match_implicitClone_const(self.as_raw_LineMod_Match())) }
	}
}

impl std::fmt::Debug for LineMod_Match {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineMod_Match")
			.field("x", &crate::rgbd::LineMod_MatchTraitConst::x(self))
			.field("y", &crate::rgbd::LineMod_MatchTraitConst::y(self))
			.field("similarity", &crate::rgbd::LineMod_MatchTraitConst::similarity(self))
			.field("class_id", &crate::rgbd::LineMod_MatchTraitConst::class_id(self))
			.field("template_id", &crate::rgbd::LineMod_MatchTraitConst::template_id(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::LineMod_Modality]
// Modality /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:119
pub trait LineMod_ModalityTraitConst {
	fn as_raw_LineMod_Modality(&self) -> *const c_void;

	/// \brief Form a quantized image pyramid from a source image.
	///
	/// \param[in] src  The source image. Type depends on the modality.
	/// \param[in] mask Optional mask. If not empty, unmasked pixels are set to zero
	///                in quantized image and cannot be extracted as features.
	///
	/// ## C++ default parameters
	/// * mask: Mat()
	// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:132
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src", "mask"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn process(&self, src: &impl core::MatTraitConst, mask: &impl core::MatTraitConst) -> Result<core::Ptr<crate::rgbd::LineMod_QuantizedPyramid>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_process_const_const_MatR_const_MatR(self.as_raw_LineMod_Modality(), src.as_raw_Mat(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_QuantizedPyramid>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Form a quantized image pyramid from a source image.
	///
	/// \param[in] src  The source image. Type depends on the modality.
	/// \param[in] mask Optional mask. If not empty, unmasked pixels are set to zero
	///                in quantized image and cannot be extracted as features.
	///
	/// ## Note
	/// This alternative version of [LineMod_ModalityTraitConst::process] function uses the following default values for its arguments:
	/// * mask: Mat()
	// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:132
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn process_def(&self, src: &impl core::MatTraitConst) -> Result<core::Ptr<crate::rgbd::LineMod_QuantizedPyramid>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_process_const_const_MatR(self.as_raw_LineMod_Modality(), src.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_QuantizedPyramid>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:138
	// ("cv::linemod::Modality::name", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_name_const(self.as_raw_LineMod_Modality(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:141
	// ("cv::linemod::Modality::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_write_const_FileStorageR(self.as_raw_LineMod_Modality(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LineMod_Modality]
pub trait LineMod_ModalityTrait: crate::rgbd::LineMod_ModalityTraitConst {
	fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void;

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:140
	// ("cv::linemod::Modality::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_read_const_FileNodeR(self.as_raw_mut_LineMod_Modality(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief Interface for modalities that plug into the LINE template matching representation.
///
/// \todo Max response, to allow optimization of summing (255/MAX) features as uint8
// Modality /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:119
pub struct LineMod_Modality {
	ptr: *mut c_void,
}

opencv_type_boxed! { LineMod_Modality }

impl Drop for LineMod_Modality {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_linemod_Modality_delete(self.as_raw_mut_LineMod_Modality()) };
	}
}

unsafe impl Send for LineMod_Modality {}

impl crate::rgbd::LineMod_ModalityTraitConst for LineMod_Modality {
	#[inline] fn as_raw_LineMod_Modality(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_ModalityTrait for LineMod_Modality {
	#[inline] fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_Modality, crate::rgbd::LineMod_ModalityTraitConst, as_raw_LineMod_Modality, crate::rgbd::LineMod_ModalityTrait, as_raw_mut_LineMod_Modality }

impl LineMod_Modality {
	/// \brief Create modality by name.
	///
	/// The following modality types are supported:
	/// - "ColorGradient"
	/// - "DepthNormal"
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:150
	// ("cv::linemod::Modality::create", vec![(pred!(mut, ["modality_type"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn create(modality_type: &str) -> Result<core::Ptr<crate::rgbd::LineMod_Modality>> {
		extern_container_arg!(modality_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_create_const_StringR(modality_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_Modality>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Load a modality from file.
	// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:155
	// ("cv::linemod::Modality::create", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	pub fn create_1(fn_: &impl core::FileNodeTraitConst) -> Result<core::Ptr<crate::rgbd::LineMod_Modality>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Modality_create_const_FileNodeR(fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_Modality>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { LineMod_Modality, crate::rgbd::LineMod_ColorGradient, cv_linemod_Modality_to_LineMod_ColorGradient }

boxed_cast_descendant! { LineMod_Modality, crate::rgbd::LineMod_DepthNormal, cv_linemod_Modality_to_LineMod_DepthNormal }

impl std::fmt::Debug for LineMod_Modality {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineMod_Modality")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::LineMod_QuantizedPyramid]
// QuantizedPyramid /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:55
pub trait LineMod_QuantizedPyramidTraitConst {
	fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void;

	/// \brief Compute quantized image at current pyramid level for online detection.
	///
	/// \param[out] dst The destination 8-bit image. For each pixel at most one bit is set,
	///                representing its classification.
	// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:67
	// ("cv::linemod::QuantizedPyramid::quantize", vec![(pred!(const, ["dst"], ["cv::Mat*"]), _)]),
	#[inline]
	fn quantize(&self, dst: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_QuantizedPyramid_quantize_const_MatR(self.as_raw_LineMod_QuantizedPyramid(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Extract most discriminant features at current pyramid level to form a new template.
	///
	/// \param[out] templ The new template.
	// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:74
	// ("cv::linemod::QuantizedPyramid::extractTemplate", vec![(pred!(const, ["templ"], ["cv::linemod::Template*"]), _)]),
	#[inline]
	fn extract_template(&self, templ: &mut impl crate::rgbd::LineMod_TemplateTrait) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(self.as_raw_LineMod_QuantizedPyramid(), templ.as_raw_mut_LineMod_Template(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LineMod_QuantizedPyramid]
pub trait LineMod_QuantizedPyramidTrait: crate::rgbd::LineMod_QuantizedPyramidTraitConst {
	fn as_raw_mut_LineMod_QuantizedPyramid(&mut self) -> *mut c_void;

	/// \brief Go to the next pyramid level.
	///
	/// \todo Allow pyramid scale factor other than 2
	// pyrDown()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:81
	// ("cv::linemod::QuantizedPyramid::pyrDown", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn pyr_down(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_QuantizedPyramid_pyrDown(self.as_raw_mut_LineMod_QuantizedPyramid(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief Represents a modality operating over an image pyramid.
// QuantizedPyramid /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:55
pub struct LineMod_QuantizedPyramid {
	ptr: *mut c_void,
}

opencv_type_boxed! { LineMod_QuantizedPyramid }

impl Drop for LineMod_QuantizedPyramid {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_linemod_QuantizedPyramid_delete(self.as_raw_mut_LineMod_QuantizedPyramid()) };
	}
}

unsafe impl Send for LineMod_QuantizedPyramid {}

impl crate::rgbd::LineMod_QuantizedPyramidTraitConst for LineMod_QuantizedPyramid {
	#[inline] fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_QuantizedPyramidTrait for LineMod_QuantizedPyramid {
	#[inline] fn as_raw_mut_LineMod_QuantizedPyramid(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_QuantizedPyramid, crate::rgbd::LineMod_QuantizedPyramidTraitConst, as_raw_LineMod_QuantizedPyramid, crate::rgbd::LineMod_QuantizedPyramidTrait, as_raw_mut_LineMod_QuantizedPyramid }

impl LineMod_QuantizedPyramid {
}

impl std::fmt::Debug for LineMod_QuantizedPyramid {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineMod_QuantizedPyramid")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::LineMod_Template]
// Template /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:41
pub trait LineMod_TemplateTraitConst {
	fn as_raw_LineMod_Template(&self) -> *const c_void;

	// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn width(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propWidth_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn height(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propHeight_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_level(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn features(&self) -> core::Vector<crate::rgbd::LineMod_Feature> {
		let ret = unsafe { sys::cv_linemod_Template_propFeatures_const(self.as_raw_LineMod_Template()) };
		let ret = unsafe { core::Vector::<crate::rgbd::LineMod_Feature>::opencv_from_extern(ret) };
		ret
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:49
	// ("cv::linemod::Template::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Template_write_const_FileStorageR(self.as_raw_LineMod_Template(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::LineMod_Template]
pub trait LineMod_TemplateTrait: crate::rgbd::LineMod_TemplateTraitConst {
	fn as_raw_mut_LineMod_Template(&mut self) -> *mut c_void;

	// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_width(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propWidth_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_height(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propHeight_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
	#[inline]
	fn set_features(&mut self, val: core::Vector<crate::rgbd::LineMod_Feature>) {
		let ret = unsafe { sys::cv_linemod_Template_propFeatures_const_vectorLFeatureG(self.as_raw_mut_LineMod_Template(), val.as_raw_VectorOfLineMod_Feature()) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:48
	// ("cv::linemod::Template::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Template_read_const_FileNodeR(self.as_raw_mut_LineMod_Template(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Template /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:41
pub struct LineMod_Template {
	ptr: *mut c_void,
}

opencv_type_boxed! { LineMod_Template }

impl Drop for LineMod_Template {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_linemod_Template_delete(self.as_raw_mut_LineMod_Template()) };
	}
}

unsafe impl Send for LineMod_Template {}

impl crate::rgbd::LineMod_TemplateTraitConst for LineMod_Template {
	#[inline] fn as_raw_LineMod_Template(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::LineMod_TemplateTrait for LineMod_Template {
	#[inline] fn as_raw_mut_LineMod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LineMod_Template, crate::rgbd::LineMod_TemplateTraitConst, as_raw_LineMod_Template, crate::rgbd::LineMod_TemplateTrait, as_raw_mut_LineMod_Template }

impl LineMod_Template {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_linemod_Template_defaultNew_const()) }
	}

}

impl Clone for LineMod_Template {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_linemod_Template_implicitClone_const(self.as_raw_LineMod_Template())) }
	}
}

impl std::fmt::Debug for LineMod_Template {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineMod_Template")
			.field("width", &crate::rgbd::LineMod_TemplateTraitConst::width(self))
			.field("height", &crate::rgbd::LineMod_TemplateTraitConst::height(self))
			.field("pyramid_level", &crate::rgbd::LineMod_TemplateTraitConst::pyramid_level(self))
			.field("features", &crate::rgbd::LineMod_TemplateTraitConst::features(self))
			.finish()
	}
}

impl Default for LineMod_Template {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::rgbd::DepthCleaner]
// DepthCleaner /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:186
pub trait DepthCleanerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_DepthCleaner(&self) -> *const c_void;

	/// Given a set of 3d points in a depth image, compute the normals at each point.
	/// ## Parameters
	/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
	/// * depth: a rows x cols matrix of the cleaned up depth
	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:223
	// ("cv::rgbd::DepthCleaner::operator()", vec![(pred!(const, ["points", "depth"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply(&self, points: &impl ToInputArray, depth: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_operator___const_const__InputArrayR_const__OutputArrayR(self.as_raw_DepthCleaner(), points.as_raw__InputArray(), depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Initializes some data that is cached for later computation
	/// If that function is not called, it will be called the first time normals are computed
	// initialize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:229
	// ("cv::rgbd::DepthCleaner::initialize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn initialize(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_initialize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:231
	// ("cv::rgbd::DepthCleaner::getWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getWindowSize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:239
	// ("cv::rgbd::DepthCleaner::getDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getDepth_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:247
	// ("cv::rgbd::DepthCleaner::getMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_method(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getMethod_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::DepthCleaner]
pub trait DepthCleanerTrait: core::AlgorithmTrait + crate::rgbd::DepthCleanerTraitConst {
	fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void;

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:235
	// ("cv::rgbd::DepthCleaner::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setWindowSize_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:243
	// ("cv::rgbd::DepthCleaner::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_depth(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setDepth_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:251
	// ("cv::rgbd::DepthCleaner::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_method(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setMethod_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Object that can clean a noisy depth image
// DepthCleaner /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:186
pub struct DepthCleaner {
	ptr: *mut c_void,
}

opencv_type_boxed! { DepthCleaner }

impl Drop for DepthCleaner {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_DepthCleaner_delete(self.as_raw_mut_DepthCleaner()) };
	}
}

unsafe impl Send for DepthCleaner {}

impl core::AlgorithmTraitConst for DepthCleaner {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DepthCleaner {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DepthCleaner, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::DepthCleanerTraitConst for DepthCleaner {
	#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::DepthCleanerTrait for DepthCleaner {
	#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DepthCleaner, crate::rgbd::DepthCleanerTraitConst, as_raw_DepthCleaner, crate::rgbd::DepthCleanerTrait, as_raw_mut_DepthCleaner }

impl DepthCleaner {
	// DepthCleaner()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:198
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::DepthCleaner> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_DepthCleaner(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::DepthCleaner::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	///
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: DepthCleaner::DEPTH_CLEANER_NIL
	// DepthCleaner(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:212
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn new(depth: i32, window_size: i32, method: i32) -> Result<crate::rgbd::DepthCleaner> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(depth, window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::DepthCleaner::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * window_size: 5
	/// * method: DepthCleaner::DEPTH_CLEANER_NIL
	// cv::rgbd::DepthCleaner::DepthCleaner(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:212
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth"], ["int"]), _)]),
	#[inline]
	pub fn new_def(depth: i32) -> Result<crate::rgbd::DepthCleaner> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_DepthCleaner_int(depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::DepthCleaner::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: DepthCleaner::DEPTH_CLEANER_NIL
	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:216
	// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn create(depth: i32, window_size: i32, method: i32) -> Result<core::Ptr<crate::rgbd::DepthCleaner>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_create_int_int_int(depth, window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::DepthCleaner>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DepthCleaner::create] function uses the following default values for its arguments:
	/// * window_size: 5
	/// * method: DepthCleaner::DEPTH_CLEANER_NIL
	// cv::rgbd::DepthCleaner::create(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:216
	// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth"], ["int"]), _)]),
	#[inline]
	pub fn create_def(depth: i32) -> Result<core::Ptr<crate::rgbd::DepthCleaner>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_create_int(depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::DepthCleaner>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DepthCleaner, core::Algorithm, cv_rgbd_DepthCleaner_to_Algorithm }

impl std::fmt::Debug for DepthCleaner {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DepthCleaner")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::FastICPOdometry]
// FastICPOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1039
pub trait FastICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
	fn as_raw_FastICPOdometry(&self) -> *const c_void;

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1070
	// ("cv::rgbd::FastICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_FastICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1072
	// ("cv::rgbd::FastICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getCameraMatrix_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMaxDistDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1080
	// ("cv::rgbd::FastICPOdometry::getMaxDistDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_dist_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getMaxDistDiff_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getAngleThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1088
	// ("cv::rgbd::FastICPOdometry::getAngleThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_angle_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getAngleThreshold_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSigmaDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1096
	// ("cv::rgbd::FastICPOdometry::getSigmaDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sigma_depth(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getSigmaDepth_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSigmaSpatial()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1104
	// ("cv::rgbd::FastICPOdometry::getSigmaSpatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sigma_spatial(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getSigmaSpatial_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getKernelSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1112
	// ("cv::rgbd::FastICPOdometry::getKernelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_kernel_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getKernelSize_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1120
	// ("cv::rgbd::FastICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getIterationCounts_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1128
	// ("cv::rgbd::FastICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_getTransformType_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::FastICPOdometry]
pub trait FastICPOdometryTrait: crate::rgbd::FastICPOdometryTraitConst + crate::rgbd::OdometryTrait {
	fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void;

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1076
	// ("cv::rgbd::FastICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_FastICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDistDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1084
	// ("cv::rgbd::FastICPOdometry::setMaxDistDiff", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_max_dist_diff(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setMaxDistDiff_float(self.as_raw_mut_FastICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setAngleThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1092
	// ("cv::rgbd::FastICPOdometry::setAngleThreshold", vec![(pred!(mut, ["f"], ["float"]), _)]),
	#[inline]
	fn set_angle_threshold(&mut self, f: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setAngleThreshold_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSigmaDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1100
	// ("cv::rgbd::FastICPOdometry::setSigmaDepth", vec![(pred!(mut, ["f"], ["float"]), _)]),
	#[inline]
	fn set_sigma_depth(&mut self, f: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setSigmaDepth_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSigmaSpatial(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1108
	// ("cv::rgbd::FastICPOdometry::setSigmaSpatial", vec![(pred!(mut, ["f"], ["float"]), _)]),
	#[inline]
	fn set_sigma_spatial(&mut self, f: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setSigmaSpatial_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1116
	// ("cv::rgbd::FastICPOdometry::setKernelSize", vec![(pred!(mut, ["f"], ["int"]), _)]),
	#[inline]
	fn set_kernel_size(&mut self, f: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setKernelSize_int(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1124
	// ("cv::rgbd::FastICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_iteration_counts(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_FastICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1132
	// ("cv::rgbd::FastICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_setTransformType_int(self.as_raw_mut_FastICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// A faster version of ICPOdometry which is used in KinectFusion implementation
/// Partial list of differences:
/// - Works in parallel
/// - Written in universal intrinsics
/// - Filters points by angle
/// - Interpolates points and normals
/// - Doesn't use masks or min/max depth filtering
/// - Doesn't use random subsets of points
/// - Supports only Rt transform type
/// - Supports only 4-float vectors as input type
// FastICPOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1039
pub struct FastICPOdometry {
	ptr: *mut c_void,
}

opencv_type_boxed! { FastICPOdometry }

impl Drop for FastICPOdometry {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_FastICPOdometry_delete(self.as_raw_mut_FastICPOdometry()) };
	}
}

unsafe impl Send for FastICPOdometry {}

impl core::AlgorithmTraitConst for FastICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FastICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastICPOdometry, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::OdometryTraitConst for FastICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::OdometryTrait for FastICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastICPOdometry, crate::rgbd::OdometryTraitConst, as_raw_Odometry, crate::rgbd::OdometryTrait, as_raw_mut_Odometry }

impl crate::rgbd::FastICPOdometryTraitConst for FastICPOdometry {
	#[inline] fn as_raw_FastICPOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::FastICPOdometryTrait for FastICPOdometry {
	#[inline] fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastICPOdometry, crate::rgbd::FastICPOdometryTraitConst, as_raw_FastICPOdometry, crate::rgbd::FastICPOdometryTrait, as_raw_mut_FastICPOdometry }

impl FastICPOdometry {
	// FastICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1042
	// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::FastICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_FastICPOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::FastICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * maxDistDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff
	/// * angleThreshold: Correspondence will be filtered out
	///                    if an angle between their normals is bigger than threshold
	/// * sigmaDepth: Depth sigma in meters for bilateral smooth
	/// * sigmaSpatial: Spatial sigma in pixels for bilateral smooth
	/// * kernelSize: Kernel size in pixels for bilateral smooth
	/// * iterCounts: Count of iterations on each pyramid level
	///
	/// ## C++ default parameters
	/// * max_dist_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * angle_threshold: (float)(30.*CV_PI/180.)
	/// * sigma_depth: 0.04f
	/// * sigma_spatial: 4.5f
	/// * kernel_size: 7
	/// * iter_counts: std::vector<int>()
	// FastICPOdometry(const Mat &, float, float, float, float, int, const std::vector<int> &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1054
	// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, ["cameraMatrix", "maxDistDiff", "angleThreshold", "sigmaDepth", "sigmaSpatial", "kernelSize", "iterCounts"], ["const cv::Mat*", "float", "float", "float", "float", "int", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn new(camera_matrix: &impl core::MatTraitConst, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: &core::Vector<i32>) -> Result<crate::rgbd::FastICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR_float_float_float_float_int_const_vectorLintGR(camera_matrix.as_raw_Mat(), max_dist_diff, angle_threshold, sigma_depth, sigma_spatial, kernel_size, iter_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::FastICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * maxDistDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff
	/// * angleThreshold: Correspondence will be filtered out
	///                    if an angle between their normals is bigger than threshold
	/// * sigmaDepth: Depth sigma in meters for bilateral smooth
	/// * sigmaSpatial: Spatial sigma in pixels for bilateral smooth
	/// * kernelSize: Kernel size in pixels for bilateral smooth
	/// * iterCounts: Count of iterations on each pyramid level
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * max_dist_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * angle_threshold: (float)(30.*CV_PI/180.)
	/// * sigma_depth: 0.04f
	/// * sigma_spatial: 4.5f
	/// * kernel_size: 7
	/// * iter_counts: std::vector<int>()
	// cv::rgbd::FastICPOdometry::FastICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1054
	// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	#[inline]
	pub fn new_def(camera_matrix: &impl core::MatTraitConst) -> Result<crate::rgbd::FastICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR(camera_matrix.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::FastICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * max_dist_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * angle_threshold: (float)(30.*CV_PI/180.)
	/// * sigma_depth: 0.04f
	/// * sigma_spatial: 4.5f
	/// * kernel_size: 7
	/// * iter_counts: std::vector<int>()
	// create(const Mat &, float, float, float, float, int, const std::vector<int> &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1062
	// ("cv::rgbd::FastICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "maxDistDiff", "angleThreshold", "sigmaDepth", "sigmaSpatial", "kernelSize", "iterCounts"], ["const cv::Mat*", "float", "float", "float", "float", "int", "const std::vector<int>*"]), _)]),
	#[inline]
	pub fn create(camera_matrix: &impl core::MatTraitConst, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: &core::Vector<i32>) -> Result<core::Ptr<crate::rgbd::FastICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_create_const_MatR_float_float_float_float_int_const_vectorLintGR(camera_matrix.as_raw_Mat(), max_dist_diff, angle_threshold, sigma_depth, sigma_spatial, kernel_size, iter_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::FastICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [FastICPOdometry::create] function uses the following default values for its arguments:
	/// * max_dist_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * angle_threshold: (float)(30.*CV_PI/180.)
	/// * sigma_depth: 0.04f
	/// * sigma_spatial: 4.5f
	/// * kernel_size: 7
	/// * iter_counts: std::vector<int>()
	// cv::rgbd::FastICPOdometry::create(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1062
	// ("cv::rgbd::FastICPOdometry::create", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	#[inline]
	pub fn create_def(camera_matrix: &impl core::MatTraitConst) -> Result<core::Ptr<crate::rgbd::FastICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_FastICPOdometry_create_const_MatR(camera_matrix.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::FastICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FastICPOdometry, core::Algorithm, cv_rgbd_FastICPOdometry_to_Algorithm }

boxed_cast_base! { FastICPOdometry, crate::rgbd::Odometry, cv_rgbd_FastICPOdometry_to_Odometry }

impl std::fmt::Debug for FastICPOdometry {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FastICPOdometry")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::ICPOdometry]
// ICPOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:759
pub trait ICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
	fn as_raw_ICPOdometry(&self) -> *const c_void;

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:781
	// ("cv::rgbd::ICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_ICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:783
	// ("cv::rgbd::ICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getCameraMatrix_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:791
	// ("cv::rgbd::ICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMinDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:799
	// ("cv::rgbd::ICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:807
	// ("cv::rgbd::ICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepthDiff_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:815
	// ("cv::rgbd::ICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getIterationCounts_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:823
	// ("cv::rgbd::ICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxPointsPart_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:831
	// ("cv::rgbd::ICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getTransformType_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:839
	// ("cv::rgbd::ICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxTranslation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:847
	// ("cv::rgbd::ICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxRotation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:855
	// ("cv::rgbd::ICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_normals_computer(&self) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getNormalsComputer_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::ICPOdometry]
pub trait ICPOdometryTrait: crate::rgbd::ICPOdometryTraitConst + crate::rgbd::OdometryTrait {
	fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void;

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:787
	// ("cv::rgbd::ICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:795
	// ("cv::rgbd::ICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMinDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:803
	// ("cv::rgbd::ICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:811
	// ("cv::rgbd::ICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:819
	// ("cv::rgbd::ICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_iteration_counts(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:827
	// ("cv::rgbd::ICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxPointsPart_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:835
	// ("cv::rgbd::ICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setTransformType_int(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:843
	// ("cv::rgbd::ICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxTranslation_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:851
	// ("cv::rgbd::ICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_rotation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxRotation_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Odometry based on the paper "KinectFusion: Real-Time Dense Surface Mapping and Tracking",
/// Richard A. Newcombe, Andrew Fitzgibbon, at al, SIGGRAPH, 2011.
// ICPOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:759
pub struct ICPOdometry {
	ptr: *mut c_void,
}

opencv_type_boxed! { ICPOdometry }

impl Drop for ICPOdometry {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_ICPOdometry_delete(self.as_raw_mut_ICPOdometry()) };
	}
}

unsafe impl Send for ICPOdometry {}

impl core::AlgorithmTraitConst for ICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ICPOdometry, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::OdometryTraitConst for ICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::OdometryTrait for ICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ICPOdometry, crate::rgbd::OdometryTraitConst, as_raw_Odometry, crate::rgbd::OdometryTrait, as_raw_mut_Odometry }

impl crate::rgbd::ICPOdometryTraitConst for ICPOdometry {
	#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::ICPOdometryTrait for ICPOdometry {
	#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ICPOdometry, crate::rgbd::ICPOdometryTraitConst, as_raw_ICPOdometry, crate::rgbd::ICPOdometryTrait, as_raw_mut_ICPOdometry }

impl ICPOdometry {
	// ICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:762
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::ICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_ICPOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * minDepth: Pixels with depth less than minDepth will not be used
	/// * maxDepth: Pixels with depth larger than maxDepth will not be used
	/// * maxDepthDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff
	/// * maxPointsPart: The method uses a random pixels subset of size frameWidth x frameHeight x pointsPart
	/// * iterCounts: Count of iterations on each pyramid level.
	/// * transformType: Class of trasformation
	///
	/// ## C++ default parameters
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// ICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:773
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
	#[inline]
	pub fn new(camera_matrix: &impl core::MatTraitConst, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, transform_type: i32) -> Result<crate::rgbd::ICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * minDepth: Pixels with depth less than minDepth will not be used
	/// * maxDepth: Pixels with depth larger than maxDepth will not be used
	/// * maxDepthDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff
	/// * maxPointsPart: The method uses a random pixels subset of size frameWidth x frameHeight x pointsPart
	/// * iterCounts: Count of iterations on each pyramid level.
	/// * transformType: Class of trasformation
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// cv::rgbd::ICPOdometry::ICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:773
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	#[inline]
	pub fn new_def(camera_matrix: &impl core::MatTraitConst) -> Result<crate::rgbd::ICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_ICPOdometry_const_MatR(camera_matrix.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::ICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// create(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:777
	// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
	#[inline]
	pub fn create(camera_matrix: &impl core::MatTraitConst, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, transform_type: i32) -> Result<core::Ptr<crate::rgbd::ICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [ICPOdometry::create] function uses the following default values for its arguments:
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// cv::rgbd::ICPOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:777
	// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::rgbd::ICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::ICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ICPOdometry, core::Algorithm, cv_rgbd_ICPOdometry_to_Algorithm }

boxed_cast_base! { ICPOdometry, crate::rgbd::Odometry, cv_rgbd_ICPOdometry_to_Odometry }

impl std::fmt::Debug for ICPOdometry {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ICPOdometry")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::Odometry]
// Odometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:524
pub trait OdometryTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Odometry(&self) -> *const c_void;

	/// Method to compute a transformation from the source frame to the destination one.
	/// Some odometry algorithms do not used some data of frames (eg. ICP does not use images).
	/// In such case corresponding arguments can be set as empty Mat.
	/// The method returns true if all internal computations were possible (e.g. there were enough correspondences,
	/// system of equations has a solution, etc) and resulting transformation satisfies some test if it's provided
	/// by the Odometry inheritor implementation (e.g. thresholds for maximum translation and rotation).
	/// ## Parameters
	/// * srcImage: Image data of the source frame (CV_8UC1)
	/// * srcDepth: Depth data of the source frame (CV_32FC1, in meters)
	/// * srcMask: Mask that sets which pixels have to be used from the source frame (CV_8UC1)
	/// * dstImage: Image data of the destination frame (CV_8UC1)
	/// * dstDepth: Depth data of the destination frame (CV_32FC1, in meters)
	/// * dstMask: Mask that sets which pixels have to be used from the destination frame (CV_8UC1)
	/// * Rt: Resulting transformation from the source frame to the destination one (rigid body motion):
	///   dst_p = Rt * src_p, where dst_p is a homogeneous point in the destination frame and src_p is
	///   homogeneous point in the source frame,
	///   Rt is 4x4 matrix of CV_64FC1 type.
	/// * initRt: Initial transformation from the source frame to the destination one (optional)
	///
	/// ## C++ default parameters
	/// * init_rt: Mat()
	// compute(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:584
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt", "initRt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
	#[inline]
	fn compute(&self, src_image: &impl core::MatTraitConst, src_depth: &impl core::MatTraitConst, src_mask: &impl core::MatTraitConst, dst_image: &impl core::MatTraitConst, dst_depth: &impl core::MatTraitConst, dst_mask: &impl core::MatTraitConst, rt: &mut impl ToOutputArray, init_rt: &impl core::MatTraitConst) -> Result<bool> {
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(self.as_raw_Odometry(), src_image.as_raw_Mat(), src_depth.as_raw_Mat(), src_mask.as_raw_Mat(), dst_image.as_raw_Mat(), dst_depth.as_raw_Mat(), dst_mask.as_raw_Mat(), rt.as_raw__OutputArray(), init_rt.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Method to compute a transformation from the source frame to the destination one.
	/// Some odometry algorithms do not used some data of frames (eg. ICP does not use images).
	/// In such case corresponding arguments can be set as empty Mat.
	/// The method returns true if all internal computations were possible (e.g. there were enough correspondences,
	/// system of equations has a solution, etc) and resulting transformation satisfies some test if it's provided
	/// by the Odometry inheritor implementation (e.g. thresholds for maximum translation and rotation).
	/// ## Parameters
	/// * srcImage: Image data of the source frame (CV_8UC1)
	/// * srcDepth: Depth data of the source frame (CV_32FC1, in meters)
	/// * srcMask: Mask that sets which pixels have to be used from the source frame (CV_8UC1)
	/// * dstImage: Image data of the destination frame (CV_8UC1)
	/// * dstDepth: Depth data of the destination frame (CV_32FC1, in meters)
	/// * dstMask: Mask that sets which pixels have to be used from the destination frame (CV_8UC1)
	/// * Rt: Resulting transformation from the source frame to the destination one (rigid body motion):
	///   dst_p = Rt * src_p, where dst_p is a homogeneous point in the destination frame and src_p is
	///   homogeneous point in the source frame,
	///   Rt is 4x4 matrix of CV_64FC1 type.
	/// * initRt: Initial transformation from the source frame to the destination one (optional)
	///
	/// ## Note
	/// This alternative version of [OdometryTraitConst::compute] function uses the following default values for its arguments:
	/// * init_rt: Mat()
	// cv::rgbd::Odometry::compute(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:584
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_def(&self, src_image: &impl core::MatTraitConst, src_depth: &impl core::MatTraitConst, src_mask: &impl core::MatTraitConst, dst_image: &impl core::MatTraitConst, dst_depth: &impl core::MatTraitConst, dst_mask: &impl core::MatTraitConst, rt: &mut impl ToOutputArray) -> Result<bool> {
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(self.as_raw_Odometry(), src_image.as_raw_Mat(), src_depth.as_raw_Mat(), src_mask.as_raw_Mat(), dst_image.as_raw_Mat(), dst_depth.as_raw_Mat(), dst_mask.as_raw_Mat(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// One more method to compute a transformation from the source frame to the destination one.
	/// It is designed to save on computing the frame data (image pyramids, normals, etc.).
	///
	/// ## C++ default parameters
	/// * init_rt: Mat()
	// compute(Ptr<OdometryFrame> &, Ptr<OdometryFrame> &, OutputArray, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:591
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt", "initRt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
	#[inline]
	fn compute2(&self, src_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, dst_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, rt: &mut impl ToOutputArray, init_rt: &impl core::MatTraitConst) -> Result<bool> {
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR_const_MatR(self.as_raw_Odometry(), src_frame.as_raw_mut_PtrOfOdometryFrame(), dst_frame.as_raw_mut_PtrOfOdometryFrame(), rt.as_raw__OutputArray(), init_rt.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// One more method to compute a transformation from the source frame to the destination one.
	/// It is designed to save on computing the frame data (image pyramids, normals, etc.).
	///
	/// ## Note
	/// This alternative version of [OdometryTraitConst::compute2] function uses the following default values for its arguments:
	/// * init_rt: Mat()
	// cv::rgbd::Odometry::compute(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:591
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute2_def(&self, src_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, dst_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, rt: &mut impl ToOutputArray) -> Result<bool> {
		output_array_arg!(rt);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR(self.as_raw_Odometry(), src_frame.as_raw_mut_PtrOfOdometryFrame(), dst_frame.as_raw_mut_PtrOfOdometryFrame(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Prepare a cache for the frame. The function checks the precomputed/passed data (throws the error if this data
	/// does not satisfy) and computes all remaining cache data needed for the frame. Returned size is a resolution
	/// of the prepared frame.
	/// ## Parameters
	/// * frame: The odometry which will process the frame.
	/// * cacheType: The cache type: CACHE_SRC, CACHE_DST or CACHE_ALL.
	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:599
	// ("cv::rgbd::Odometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_Odometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setCameraMatrix
	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:604
	// ("cv::rgbd::Odometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_getCameraMatrix_const(self.as_raw_Odometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## See also
	/// setTransformType
	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:608
	// ("cv::rgbd::Odometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_getTransformType_const(self.as_raw_Odometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::Odometry]
pub trait OdometryTrait: core::AlgorithmTrait + crate::rgbd::OdometryTraitConst {
	fn as_raw_mut_Odometry(&mut self) -> *mut c_void;

	/// ## See also
	/// setCameraMatrix getCameraMatrix
	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:606
	// ("cv::rgbd::Odometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_setCameraMatrix_const_MatR(self.as_raw_mut_Odometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## See also
	/// setTransformType getTransformType
	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:610
	// ("cv::rgbd::Odometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_setTransformType_int(self.as_raw_mut_Odometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base class for computation of odometry.
// Odometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:524
pub struct Odometry {
	ptr: *mut c_void,
}

opencv_type_boxed! { Odometry }

impl Drop for Odometry {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_Odometry_delete(self.as_raw_mut_Odometry()) };
	}
}

unsafe impl Send for Odometry {}

impl core::AlgorithmTraitConst for Odometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Odometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Odometry, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::OdometryTraitConst for Odometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::OdometryTrait for Odometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Odometry, crate::rgbd::OdometryTraitConst, as_raw_Odometry, crate::rgbd::OdometryTrait, as_raw_mut_Odometry }

impl Odometry {
	// DEFAULT_MIN_DEPTH()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:535
	// ("cv::rgbd::Odometry::DEFAULT_MIN_DEPTH", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_min_depth() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_DEPTH()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:540
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_depth() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_DEPTH_DIFF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:545
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_depth_diff() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_POINTS_PART()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:550
	// ("cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_points_part() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_TRANSLATION()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:555
	// ("cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_translation() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_ROTATION()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:560
	// ("cv::rgbd::Odometry::DEFAULT_MAX_ROTATION", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_rotation() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:601
	// ("cv::rgbd::Odometry::create", vec![(pred!(mut, ["odometryType"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn create(odometry_type: &str) -> Result<core::Ptr<crate::rgbd::Odometry>> {
		extern_container_arg!(odometry_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_create_const_StringR(odometry_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Odometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Odometry, crate::rgbd::FastICPOdometry, cv_rgbd_Odometry_to_FastICPOdometry }

boxed_cast_descendant! { Odometry, crate::rgbd::ICPOdometry, cv_rgbd_Odometry_to_ICPOdometry }

boxed_cast_descendant! { Odometry, crate::rgbd::RgbdICPOdometry, cv_rgbd_Odometry_to_RgbdICPOdometry }

boxed_cast_descendant! { Odometry, crate::rgbd::RgbdOdometry, cv_rgbd_Odometry_to_RgbdOdometry }

boxed_cast_base! { Odometry, core::Algorithm, cv_rgbd_Odometry_to_Algorithm }

impl std::fmt::Debug for Odometry {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Odometry")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::OdometryFrame]
// OdometryFrame /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:483
pub trait OdometryFrameTraitConst: crate::rgbd::RgbdFrameTraitConst {
	fn as_raw_OdometryFrame(&self) -> *const c_void;

	// cv::rgbd::OdometryFrame::pyramidImage() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:508
	// ("cv::rgbd::OdometryFrame::pyramidImage", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_image(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidImage_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidDepth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:509
	// ("cv::rgbd::OdometryFrame::pyramidDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_depth(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidDepth_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:510
	// ("cv::rgbd::OdometryFrame::pyramidMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidCloud() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:512
	// ("cv::rgbd::OdometryFrame::pyramidCloud", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_cloud(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidCloud_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:514
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_d_i_dx(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dx_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dy() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:515
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dy", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_d_i_dy(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dy_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidTexturedMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:516
	// ("cv::rgbd::OdometryFrame::pyramidTexturedMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_textured_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidTexturedMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidNormals() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:518
	// ("cv::rgbd::OdometryFrame::pyramidNormals", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_normals(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormals_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidNormalsMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:519
	// ("cv::rgbd::OdometryFrame::pyramidNormalsMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_normals_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormalsMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::rgbd::OdometryFrame]
pub trait OdometryFrameTrait: crate::rgbd::OdometryFrameTraitConst + crate::rgbd::RgbdFrameTrait {
	fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void;

	// cv::rgbd::OdometryFrame::setPyramidImage(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:508
	// ("cv::rgbd::OdometryFrame::setPyramidImage", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_image(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidImage_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidDepth(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:509
	// ("cv::rgbd::OdometryFrame::setPyramidDepth", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_depth(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidDepth_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:510
	// ("cv::rgbd::OdometryFrame::setPyramidMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_mask(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidMask_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidCloud(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:512
	// ("cv::rgbd::OdometryFrame::setPyramidCloud", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_cloud(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidCloud_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:514
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dx", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_d_i_dx(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dx_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dy(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:515
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dy", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_d_i_dy(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dy_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidTexturedMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:516
	// ("cv::rgbd::OdometryFrame::setPyramidTexturedMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_textured_mask(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidTexturedMask_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidNormals(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:518
	// ("cv::rgbd::OdometryFrame::setPyramidNormals", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_normals(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormals_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidNormalsMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:519
	// ("cv::rgbd::OdometryFrame::setPyramidNormalsMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_normals_mask(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormalsMask_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:503
	// ("cv::rgbd::OdometryFrame::release", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_release(self.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// releasePyramids()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:506
	// ("cv::rgbd::OdometryFrame::releasePyramids", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release_pyramids(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_releasePyramids(self.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Object that contains a frame data that is possibly needed for the Odometry.
/// It's used for the efficiency (to pass precomputed/cached data of the frame that participates
/// in the Odometry processing several times).
// OdometryFrame /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:483
pub struct OdometryFrame {
	ptr: *mut c_void,
}

opencv_type_boxed! { OdometryFrame }

impl Drop for OdometryFrame {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_OdometryFrame_delete(self.as_raw_mut_OdometryFrame()) };
	}
}

unsafe impl Send for OdometryFrame {}

impl crate::rgbd::RgbdFrameTraitConst for OdometryFrame {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for OdometryFrame {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OdometryFrame, crate::rgbd::RgbdFrameTraitConst, as_raw_RgbdFrame, crate::rgbd::RgbdFrameTrait, as_raw_mut_RgbdFrame }

impl crate::rgbd::OdometryFrameTraitConst for OdometryFrame {
	#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::OdometryFrameTrait for OdometryFrame {
	#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OdometryFrame, crate::rgbd::OdometryFrameTraitConst, as_raw_OdometryFrame, crate::rgbd::OdometryFrameTrait, as_raw_mut_OdometryFrame }

impl OdometryFrame {
	// OdometryFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:497
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::OdometryFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_OdometryFrame(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::OdometryFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// OdometryFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:498
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	#[inline]
	pub fn new(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst, mask: &impl core::MatTraitConst, normals: &impl core::MatTraitConst, id: i32) -> Result<crate::rgbd::OdometryFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::OdometryFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// cv::rgbd::OdometryFrame::OdometryFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:498
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	pub fn new_def(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst) -> Result<crate::rgbd::OdometryFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR(image.as_raw_Mat(), depth.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::OdometryFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * image: Mat()
	/// * depth: Mat()
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:500
	// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	#[inline]
	pub fn create(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst, mask: &impl core::MatTraitConst, normals: &impl core::MatTraitConst, id: i32) -> Result<core::Ptr<crate::rgbd::OdometryFrame>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::OdometryFrame>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [OdometryFrame::create] function uses the following default values for its arguments:
	/// * image: Mat()
	/// * depth: Mat()
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// cv::rgbd::OdometryFrame::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:500
	// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::rgbd::OdometryFrame>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::OdometryFrame>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { OdometryFrame, crate::rgbd::RgbdFrame, cv_rgbd_OdometryFrame_to_RgbdFrame }

impl std::fmt::Debug for OdometryFrame {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("OdometryFrame")
			.field("pyramid_image", &crate::rgbd::OdometryFrameTraitConst::pyramid_image(self))
			.field("pyramid_depth", &crate::rgbd::OdometryFrameTraitConst::pyramid_depth(self))
			.field("pyramid_mask", &crate::rgbd::OdometryFrameTraitConst::pyramid_mask(self))
			.field("pyramid_cloud", &crate::rgbd::OdometryFrameTraitConst::pyramid_cloud(self))
			.field("pyramid_d_i_dx", &crate::rgbd::OdometryFrameTraitConst::pyramid_d_i_dx(self))
			.field("pyramid_d_i_dy", &crate::rgbd::OdometryFrameTraitConst::pyramid_d_i_dy(self))
			.field("pyramid_textured_mask", &crate::rgbd::OdometryFrameTraitConst::pyramid_textured_mask(self))
			.field("pyramid_normals", &crate::rgbd::OdometryFrameTraitConst::pyramid_normals(self))
			.field("pyramid_normals_mask", &crate::rgbd::OdometryFrameTraitConst::pyramid_normals_mask(self))
			.field("id", &crate::rgbd::RgbdFrameTraitConst::id(self))
			.field("image", &crate::rgbd::RgbdFrameTraitConst::image(self))
			.field("depth", &crate::rgbd::RgbdFrameTraitConst::depth(self))
			.field("mask", &crate::rgbd::RgbdFrameTraitConst::mask(self))
			.field("normals", &crate::rgbd::RgbdFrameTraitConst::normals(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::RgbdFrame]
// RgbdFrame /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:461
pub trait RgbdFrameTraitConst {
	fn as_raw_RgbdFrame(&self) -> *const c_void;

	// cv::rgbd::RgbdFrame::ID() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:472
	// ("cv::rgbd::RgbdFrame::ID", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn id(&self) -> i32 {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propID_const(self.as_raw_RgbdFrame()) };
		ret
	}

	// cv::rgbd::RgbdFrame::image() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:473
	// ("cv::rgbd::RgbdFrame::image", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn image(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propImage_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::RgbdFrame::depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:474
	// ("cv::rgbd::RgbdFrame::depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propDepth_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::RgbdFrame::mask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:475
	// ("cv::rgbd::RgbdFrame::mask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn mask(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propMask_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::RgbdFrame::normals() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:476
	// ("cv::rgbd::RgbdFrame::normals", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn normals(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propNormals_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::rgbd::RgbdFrame]
pub trait RgbdFrameTrait: crate::rgbd::RgbdFrameTraitConst {
	fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void;

	// cv::rgbd::RgbdFrame::setID(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:472
	// ("cv::rgbd::RgbdFrame::setID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propID_const_int(self.as_raw_mut_RgbdFrame(), val) };
		ret
	}

	// cv::rgbd::RgbdFrame::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:473
	// ("cv::rgbd::RgbdFrame::setImage", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_image(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propImage_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// cv::rgbd::RgbdFrame::setDepth(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:474
	// ("cv::rgbd::RgbdFrame::setDepth", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_depth(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propDepth_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// cv::rgbd::RgbdFrame::setMask(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:475
	// ("cv::rgbd::RgbdFrame::setMask", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_mask(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propMask_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// cv::rgbd::RgbdFrame::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:476
	// ("cv::rgbd::RgbdFrame::setNormals", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_normals(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propNormals_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:470
	// ("cv::rgbd::RgbdFrame::release", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_release(self.as_raw_mut_RgbdFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Object that contains a frame data.
// RgbdFrame /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:461
pub struct RgbdFrame {
	ptr: *mut c_void,
}

opencv_type_boxed! { RgbdFrame }

impl Drop for RgbdFrame {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_RgbdFrame_delete(self.as_raw_mut_RgbdFrame()) };
	}
}

unsafe impl Send for RgbdFrame {}

impl crate::rgbd::RgbdFrameTraitConst for RgbdFrame {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for RgbdFrame {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdFrame, crate::rgbd::RgbdFrameTraitConst, as_raw_RgbdFrame, crate::rgbd::RgbdFrameTrait, as_raw_mut_RgbdFrame }

impl RgbdFrame {
	// RgbdFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:463
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_RgbdFrame(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// RgbdFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:464
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	#[inline]
	pub fn new(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst, mask: &impl core::MatTraitConst, normals: &impl core::MatTraitConst, id: i32) -> Result<crate::rgbd::RgbdFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// cv::rgbd::RgbdFrame::RgbdFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:464
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	pub fn new_def(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst) -> Result<crate::rgbd::RgbdFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR(image.as_raw_Mat(), depth.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdFrame::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * image: Mat()
	/// * depth: Mat()
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:467
	// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	#[inline]
	pub fn create(image: &impl core::MatTraitConst, depth: &impl core::MatTraitConst, mask: &impl core::MatTraitConst, normals: &impl core::MatTraitConst, id: i32) -> Result<core::Ptr<crate::rgbd::RgbdFrame>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdFrame>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [RgbdFrame::create] function uses the following default values for its arguments:
	/// * image: Mat()
	/// * depth: Mat()
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	// cv::rgbd::RgbdFrame::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:467
	// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::rgbd::RgbdFrame>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdFrame_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdFrame>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { RgbdFrame, crate::rgbd::OdometryFrame, cv_rgbd_RgbdFrame_to_OdometryFrame }

impl std::fmt::Debug for RgbdFrame {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RgbdFrame")
			.field("id", &crate::rgbd::RgbdFrameTraitConst::id(self))
			.field("image", &crate::rgbd::RgbdFrameTraitConst::image(self))
			.field("depth", &crate::rgbd::RgbdFrameTraitConst::depth(self))
			.field("mask", &crate::rgbd::RgbdFrameTraitConst::mask(self))
			.field("normals", &crate::rgbd::RgbdFrameTraitConst::normals(self))
			.finish()
	}
}

/// Constant methods for [crate::rgbd::RgbdICPOdometry]
// RgbdICPOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:887
pub trait RgbdICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
	fn as_raw_RgbdICPOdometry(&self) -> *const c_void;

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:915
	// ("cv::rgbd::RgbdICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_RgbdICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:917
	// ("cv::rgbd::RgbdICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:925
	// ("cv::rgbd::RgbdICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMinDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:933
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:941
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:949
	// ("cv::rgbd::RgbdICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:957
	// ("cv::rgbd::RgbdICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getIterationCounts_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:965
	// ("cv::rgbd::RgbdICPOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_gradient_magnitudes(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:973
	// ("cv::rgbd::RgbdICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getTransformType_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:981
	// ("cv::rgbd::RgbdICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:989
	// ("cv::rgbd::RgbdICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxRotation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:997
	// ("cv::rgbd::RgbdICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_normals_computer(&self) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::RgbdICPOdometry]
pub trait RgbdICPOdometryTrait: crate::rgbd::OdometryTrait + crate::rgbd::RgbdICPOdometryTraitConst {
	fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void;

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:921
	// ("cv::rgbd::RgbdICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:929
	// ("cv::rgbd::RgbdICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:937
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:945
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:953
	// ("cv::rgbd::RgbdICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:961
	// ("cv::rgbd::RgbdICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_iteration_counts(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:969
	// ("cv::rgbd::RgbdICPOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_min_gradient_magnitudes(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:977
	// ("cv::rgbd::RgbdICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setTransformType_int(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:985
	// ("cv::rgbd::RgbdICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:993
	// ("cv::rgbd::RgbdICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_rotation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxRotation_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Odometry that merges RgbdOdometry and ICPOdometry by minimize sum of their energy functions.
// RgbdICPOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:887
pub struct RgbdICPOdometry {
	ptr: *mut c_void,
}

opencv_type_boxed! { RgbdICPOdometry }

impl Drop for RgbdICPOdometry {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_RgbdICPOdometry_delete(self.as_raw_mut_RgbdICPOdometry()) };
	}
}

unsafe impl Send for RgbdICPOdometry {}

impl core::AlgorithmTraitConst for RgbdICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdICPOdometry, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::OdometryTraitConst for RgbdICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::OdometryTrait for RgbdICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdICPOdometry, crate::rgbd::OdometryTraitConst, as_raw_Odometry, crate::rgbd::OdometryTrait, as_raw_mut_Odometry }

impl crate::rgbd::RgbdICPOdometryTraitConst for RgbdICPOdometry {
	#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdICPOdometryTrait for RgbdICPOdometry {
	#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdICPOdometry, crate::rgbd::RgbdICPOdometryTraitConst, as_raw_RgbdICPOdometry, crate::rgbd::RgbdICPOdometryTrait, as_raw_mut_RgbdICPOdometry }

impl RgbdICPOdometry {
	// RgbdICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:890
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * minDepth: Pixels with depth less than minDepth will not be used
	/// * maxDepth: Pixels with depth larger than maxDepth will not be used
	/// * maxDepthDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff
	/// * maxPointsPart: The method uses a random pixels subset of size frameWidth x frameHeight x pointsPart
	/// * iterCounts: Count of iterations on each pyramid level.
	/// * minGradientMagnitudes: For each pyramid level the pixels will be filtered out
	///                              if they have gradient magnitude less than minGradientMagnitudes[level].
	/// * transformType: Class of trasformation
	///
	/// ## C++ default parameters
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// RgbdICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:903
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
	#[inline]
	pub fn new(camera_matrix: &impl core::MatTraitConst, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, transform_type: i32) -> Result<crate::rgbd::RgbdICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * minDepth: Pixels with depth less than minDepth will not be used
	/// * maxDepth: Pixels with depth larger than maxDepth will not be used
	/// * maxDepthDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff
	/// * maxPointsPart: The method uses a random pixels subset of size frameWidth x frameHeight x pointsPart
	/// * iterCounts: Count of iterations on each pyramid level.
	/// * minGradientMagnitudes: For each pyramid level the pixels will be filtered out
	///                              if they have gradient magnitude less than minGradientMagnitudes[level].
	/// * transformType: Class of trasformation
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// cv::rgbd::RgbdICPOdometry::RgbdICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:903
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	#[inline]
	pub fn new_def(camera_matrix: &impl core::MatTraitConst) -> Result<crate::rgbd::RgbdICPOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR(camera_matrix.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdICPOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// create(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:909
	// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
	#[inline]
	pub fn create(camera_matrix: &impl core::MatTraitConst, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, transform_type: i32) -> Result<core::Ptr<crate::rgbd::RgbdICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [RgbdICPOdometry::create] function uses the following default values for its arguments:
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// cv::rgbd::RgbdICPOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:909
	// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::rgbd::RgbdICPOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdICPOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { RgbdICPOdometry, core::Algorithm, cv_rgbd_RgbdICPOdometry_to_Algorithm }

boxed_cast_base! { RgbdICPOdometry, crate::rgbd::Odometry, cv_rgbd_RgbdICPOdometry_to_Odometry }

impl std::fmt::Debug for RgbdICPOdometry {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RgbdICPOdometry")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::RgbdNormals]
// RgbdNormals /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:73
pub trait RgbdNormalsTraitConst: core::AlgorithmTraitConst {
	fn as_raw_RgbdNormals(&self) -> *const c_void;

	/// Given a set of 3d points in a depth image, compute the normals at each point.
	/// ## Parameters
	/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
	/// * normals: a rows x cols x 3 matrix
	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:116
	// ("cv::rgbd::RgbdNormals::operator()", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points);
		output_array_arg!(normals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_operator___const_const__InputArrayR_const__OutputArrayR(self.as_raw_RgbdNormals(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Initializes some data that is cached for later computation
	/// If that function is not called, it will be called the first time normals are computed
	// initialize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:122
	// ("cv::rgbd::RgbdNormals::initialize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn initialize(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_initialize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getRows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:124
	// ("cv::rgbd::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_rows(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getRows_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCols()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:132
	// ("cv::rgbd::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_cols(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getCols_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:140
	// ("cv::rgbd::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getWindowSize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:148
	// ("cv::rgbd::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getDepth_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:156
	// ("cv::rgbd::RgbdNormals::getK", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_k(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getK_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:164
	// ("cv::rgbd::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_method(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getMethod_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::RgbdNormals]
pub trait RgbdNormalsTrait: core::AlgorithmTrait + crate::rgbd::RgbdNormalsTraitConst {
	fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void;

	// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:128
	// ("cv::rgbd::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_rows(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setRows_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:136
	// ("cv::rgbd::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_cols(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setCols_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:144
	// ("cv::rgbd::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setWindowSize_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:152
	// ("cv::rgbd::RgbdNormals::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_depth(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setDepth_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setK(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:160
	// ("cv::rgbd::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_k(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setK_const_MatR(self.as_raw_mut_RgbdNormals(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:168
	// ("cv::rgbd::RgbdNormals::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_method(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setMethod_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Object that can compute the normals in an image.
/// It is an object as it can cache data for speed efficiency
/// The implemented methods are either:
/// - FALS (the fastest) and SRI from
/// ``Fast and Accurate Computation of Surface Normals from Range Images``
/// by H. Badino, D. Huber, Y. Park and T. Kanade
/// - the normals with bilateral filtering on a depth image from
/// ``Gradient Response Maps for Real-Time Detection of Texture-Less Objects``
/// by S. Hinterstoisser, C. Cagniart, S. Ilic, P. Sturm, N. Navab, P. Fua, and V. Lepetit
// RgbdNormals /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:73
pub struct RgbdNormals {
	ptr: *mut c_void,
}

opencv_type_boxed! { RgbdNormals }

impl Drop for RgbdNormals {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_RgbdNormals_delete(self.as_raw_mut_RgbdNormals()) };
	}
}

unsafe impl Send for RgbdNormals {}

impl core::AlgorithmTraitConst for RgbdNormals {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdNormals {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdNormals, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::RgbdNormalsTraitConst for RgbdNormals {
	#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdNormalsTrait for RgbdNormals {
	#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdNormals, crate::rgbd::RgbdNormalsTraitConst, as_raw_RgbdNormals, crate::rgbd::RgbdNormalsTrait, as_raw_mut_RgbdNormals }

impl RgbdNormals {
	// RgbdNormals()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:83
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdNormals> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_RgbdNormals(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdNormals::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * rows: the number of rows of the depth image normals will be computed on
	/// * cols: the number of cols of the depth image normals will be computed on
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * K: the calibration matrix to use
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	///
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: RgbdNormals::RGBD_NORMALS_METHOD_FALS
	// RgbdNormals(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:103
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
	#[inline]
	pub fn new(rows: i32, cols: i32, depth: i32, k: &impl ToInputArray, window_size: i32, method: i32) -> Result<crate::rgbd::RgbdNormals> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(rows, cols, depth, k.as_raw__InputArray(), window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdNormals::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * rows: the number of rows of the depth image normals will be computed on
	/// * cols: the number of cols of the depth image normals will be computed on
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * K: the calibration matrix to use
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * window_size: 5
	/// * method: RgbdNormals::RGBD_NORMALS_METHOD_FALS
	// cv::rgbd::RgbdNormals::RgbdNormals(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:103
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn new_def(rows: i32, cols: i32, depth: i32, k: &impl ToInputArray) -> Result<crate::rgbd::RgbdNormals> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR(rows, cols, depth, k.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdNormals::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: RgbdNormals::RGBD_NORMALS_METHOD_FALS
	// create(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:108
	// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
	#[inline]
	pub fn create(rows: i32, cols: i32, depth: i32, k: &impl ToInputArray, window_size: i32, method: i32) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(rows, cols, depth, k.as_raw__InputArray(), window_size, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [RgbdNormals::create] function uses the following default values for its arguments:
	/// * window_size: 5
	/// * method: RgbdNormals::RGBD_NORMALS_METHOD_FALS
	// cv::rgbd::RgbdNormals::create(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:108
	// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create_def(rows: i32, cols: i32, depth: i32, k: &impl ToInputArray) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR(rows, cols, depth, k.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { RgbdNormals, core::Algorithm, cv_rgbd_RgbdNormals_to_Algorithm }

impl std::fmt::Debug for RgbdNormals {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RgbdNormals")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::RgbdOdometry]
// RgbdOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:624
pub trait RgbdOdometryTraitConst: crate::rgbd::OdometryTraitConst {
	fn as_raw_RgbdOdometry(&self) -> *const c_void;

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:650
	// ("cv::rgbd::RgbdOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_RgbdOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:652
	// ("cv::rgbd::RgbdOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getCameraMatrix_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:660
	// ("cv::rgbd::RgbdOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMinDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:668
	// ("cv::rgbd::RgbdOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:676
	// ("cv::rgbd::RgbdOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:684
	// ("cv::rgbd::RgbdOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getIterationCounts_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:692
	// ("cv::rgbd::RgbdOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_gradient_magnitudes(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:700
	// ("cv::rgbd::RgbdOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxPointsPart_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:708
	// ("cv::rgbd::RgbdOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getTransformType_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:716
	// ("cv::rgbd::RgbdOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxTranslation_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:724
	// ("cv::rgbd::RgbdOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxRotation_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::RgbdOdometry]
pub trait RgbdOdometryTrait: crate::rgbd::OdometryTrait + crate::rgbd::RgbdOdometryTraitConst {
	fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void;

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:656
	// ("cv::rgbd::RgbdOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:664
	// ("cv::rgbd::RgbdOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:672
	// ("cv::rgbd::RgbdOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:680
	// ("cv::rgbd::RgbdOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:688
	// ("cv::rgbd::RgbdOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_iteration_counts(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:696
	// ("cv::rgbd::RgbdOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_min_gradient_magnitudes(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:704
	// ("cv::rgbd::RgbdOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:712
	// ("cv::rgbd::RgbdOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setTransformType_int(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:720
	// ("cv::rgbd::RgbdOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:728
	// ("cv::rgbd::RgbdOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_rotation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxRotation_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Odometry based on the paper "Real-Time Visual Odometry from Dense RGB-D Images",
/// F. Steinbucker, J. Strum, D. Cremers, ICCV, 2011.
// RgbdOdometry /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:624
pub struct RgbdOdometry {
	ptr: *mut c_void,
}

opencv_type_boxed! { RgbdOdometry }

impl Drop for RgbdOdometry {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_RgbdOdometry_delete(self.as_raw_mut_RgbdOdometry()) };
	}
}

unsafe impl Send for RgbdOdometry {}

impl core::AlgorithmTraitConst for RgbdOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdOdometry, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::OdometryTraitConst for RgbdOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::OdometryTrait for RgbdOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdOdometry, crate::rgbd::OdometryTraitConst, as_raw_Odometry, crate::rgbd::OdometryTrait, as_raw_mut_Odometry }

impl crate::rgbd::RgbdOdometryTraitConst for RgbdOdometry {
	#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdOdometryTrait for RgbdOdometry {
	#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdOdometry, crate::rgbd::RgbdOdometryTraitConst, as_raw_RgbdOdometry, crate::rgbd::RgbdOdometryTrait, as_raw_mut_RgbdOdometry }

impl RgbdOdometry {
	// RgbdOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:627
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::RgbdOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_RgbdOdometry(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * minDepth: Pixels with depth less than minDepth will not be used (in meters)
	/// * maxDepth: Pixels with depth larger than maxDepth will not be used (in meters)
	/// * maxDepthDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff (in meters)
	/// * iterCounts: Count of iterations on each pyramid level.
	/// * minGradientMagnitudes: For each pyramid level the pixels will be filtered out
	///                              if they have gradient magnitude less than minGradientMagnitudes[level].
	/// * maxPointsPart: The method uses a random pixels subset of size frameWidth x frameHeight x pointsPart
	/// * transformType: Class of transformation
	///
	/// ## C++ default parameters
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// RgbdOdometry(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:640
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
	#[inline]
	pub fn new(camera_matrix: &impl core::MatTraitConst, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, max_points_part: f32, transform_type: i32) -> Result<crate::rgbd::RgbdOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), max_points_part, transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * minDepth: Pixels with depth less than minDepth will not be used (in meters)
	/// * maxDepth: Pixels with depth larger than maxDepth will not be used (in meters)
	/// * maxDepthDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff (in meters)
	/// * iterCounts: Count of iterations on each pyramid level.
	/// * minGradientMagnitudes: For each pyramid level the pixels will be filtered out
	///                              if they have gradient magnitude less than minGradientMagnitudes[level].
	/// * maxPointsPart: The method uses a random pixels subset of size frameWidth x frameHeight x pointsPart
	/// * transformType: Class of transformation
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// cv::rgbd::RgbdOdometry::RgbdOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:640
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	#[inline]
	pub fn new_def(camera_matrix: &impl core::MatTraitConst) -> Result<crate::rgbd::RgbdOdometry> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR(camera_matrix.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdOdometry::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// create(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:645
	// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
	#[inline]
	pub fn create(camera_matrix: &impl core::MatTraitConst, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, max_points_part: f32, transform_type: i32) -> Result<core::Ptr<crate::rgbd::RgbdOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), max_points_part, transform_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [RgbdOdometry::create] function uses the following default values for its arguments:
	/// * camera_matrix: Mat()
	/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
	/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
	/// * transform_type: Odometry::RIGID_BODY_MOTION
	// cv::rgbd::RgbdOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:645
	// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::rgbd::RgbdOdometry>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdOdometry>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { RgbdOdometry, core::Algorithm, cv_rgbd_RgbdOdometry_to_Algorithm }

boxed_cast_base! { RgbdOdometry, crate::rgbd::Odometry, cv_rgbd_RgbdOdometry_to_Odometry }

impl std::fmt::Debug for RgbdOdometry {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RgbdOdometry")
			.finish()
	}
}

/// Constant methods for [crate::rgbd::RgbdPlane]
// RgbdPlane /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:329
pub trait RgbdPlaneTraitConst: core::AlgorithmTraitConst {
	fn as_raw_RgbdPlane(&self) -> *const c_void;

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:389
	// ("cv::rgbd::RgbdPlane::getBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getBlockSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:397
	// ("cv::rgbd::RgbdPlane::getMinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getMinSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:405
	// ("cv::rgbd::RgbdPlane::getMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_method(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getMethod_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:413
	// ("cv::rgbd::RgbdPlane::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getThreshold_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSensorErrorA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:421
	// ("cv::rgbd::RgbdPlane::getSensorErrorA", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sensor_error_a(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorA_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSensorErrorB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:429
	// ("cv::rgbd::RgbdPlane::getSensorErrorB", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sensor_error_b(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorB_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSensorErrorC()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:437
	// ("cv::rgbd::RgbdPlane::getSensorErrorC", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sensor_error_c(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorC_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::rgbd::RgbdPlane]
pub trait RgbdPlaneTrait: core::AlgorithmTrait + crate::rgbd::RgbdPlaneTraitConst {
	fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void;

	/// Find The planes in a depth image
	/// ## Parameters
	/// * points3d: the 3d points organized like the depth image: rows x cols with 3 channels
	/// * normals: the normals for every point in the depth image
	/// * mask: An image where each pixel is labeled with the plane it belongs to
	///        and 255 if it does not belong to any plane
	/// * plane_coefficients: the coefficients of the corresponding planes (a,b,c,d) such that ax+by+cz+d=0, norm(a,b,c)=1
	///        and c < 0 (so that the normal points towards the camera)
	// operator()(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:377
	// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply(&mut self, points3d: &impl ToInputArray, normals: &impl ToInputArray, mask: &mut impl ToOutputArray, plane_coefficients: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points3d);
		input_array_arg!(normals);
		output_array_arg!(mask);
		output_array_arg!(plane_coefficients);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_RgbdPlane(), points3d.as_raw__InputArray(), normals.as_raw__InputArray(), mask.as_raw__OutputArray(), plane_coefficients.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Find The planes in a depth image but without doing a normal check, which is faster but less accurate
	/// ## Parameters
	/// * points3d: the 3d points organized like the depth image: rows x cols with 3 channels
	/// * mask: An image where each pixel is labeled with the plane it belongs to
	///        and 255 if it does not belong to any plane
	/// * plane_coefficients: the coefficients of the corresponding planes (a,b,c,d) such that ax+by+cz+d=0
	// operator()(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:387
	// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn apply_1(&mut self, points3d: &impl ToInputArray, mask: &mut impl ToOutputArray, plane_coefficients: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points3d);
		output_array_arg!(mask);
		output_array_arg!(plane_coefficients);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_RgbdPlane(), points3d.as_raw__InputArray(), mask.as_raw__OutputArray(), plane_coefficients.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:393
	// ("cv::rgbd::RgbdPlane::setBlockSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_block_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setBlockSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:401
	// ("cv::rgbd::RgbdPlane::setMinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_min_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setMinSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:409
	// ("cv::rgbd::RgbdPlane::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_method(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setMethod_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:417
	// ("cv::rgbd::RgbdPlane::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setThreshold_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSensorErrorA(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:425
	// ("cv::rgbd::RgbdPlane::setSensorErrorA", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_sensor_error_a(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorA_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSensorErrorB(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:433
	// ("cv::rgbd::RgbdPlane::setSensorErrorB", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_sensor_error_b(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorB_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSensorErrorC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:441
	// ("cv::rgbd::RgbdPlane::setSensorErrorC", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_sensor_error_c(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorC_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Object that can compute planes in an image
// RgbdPlane /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:329
pub struct RgbdPlane {
	ptr: *mut c_void,
}

opencv_type_boxed! { RgbdPlane }

impl Drop for RgbdPlane {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rgbd_RgbdPlane_delete(self.as_raw_mut_RgbdPlane()) };
	}
}

unsafe impl Send for RgbdPlane {}

impl core::AlgorithmTraitConst for RgbdPlane {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RgbdPlane {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdPlane, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::rgbd::RgbdPlaneTraitConst for RgbdPlane {
	#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::RgbdPlaneTrait for RgbdPlane {
	#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RgbdPlane, crate::rgbd::RgbdPlaneTraitConst, as_raw_RgbdPlane, crate::rgbd::RgbdPlaneTrait, as_raw_mut_RgbdPlane }

impl RgbdPlane {
	/// ## C++ default parameters
	/// * method: RgbdPlane::RGBD_PLANE_METHOD_DEFAULT
	// RgbdPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:337
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method"], ["int"]), _)]),
	#[inline]
	pub fn new(method: i32) -> Result<crate::rgbd::RgbdPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_RgbdPlane_int(method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdPlane::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * method: RgbdPlane::RGBD_PLANE_METHOD_DEFAULT
	// cv::rgbd::RgbdPlane::RgbdPlane() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:337
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::rgbd::RgbdPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_RgbdPlane(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdPlane::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * block_size: The size of the blocks to look at for a stable MSE
	/// * min_size: The minimum size of a cluster to be considered a plane
	/// * threshold: The maximum distance of a point from a plane to belong to it (in meters)
	/// * sensor_error_a: coefficient of the sensor error. 0 by default, 0.0075 for a Kinect
	/// * sensor_error_b: coefficient of the sensor error. 0 by default
	/// * sensor_error_c: coefficient of the sensor error. 0 by default
	/// * method: The method to use to compute the planes.
	///
	/// ## C++ default parameters
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	// RgbdPlane(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:358
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
	#[inline]
	pub fn new_1(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64) -> Result<crate::rgbd::RgbdPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdPlane::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * block_size: The size of the blocks to look at for a stable MSE
	/// * min_size: The minimum size of a cluster to be considered a plane
	/// * threshold: The maximum distance of a point from a plane to belong to it (in meters)
	/// * sensor_error_a: coefficient of the sensor error. 0 by default, 0.0075 for a Kinect
	/// * sensor_error_b: coefficient of the sensor error. 0 by default
	/// * sensor_error_c: coefficient of the sensor error. 0 by default
	/// * method: The method to use to compute the planes.
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	// cv::rgbd::RgbdPlane::RgbdPlane(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:358
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
	#[inline]
	pub fn new_def_1(method: i32, block_size: i32, min_size: i32, threshold: f64) -> Result<crate::rgbd::RgbdPlane> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double(method, block_size, min_size, threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::rgbd::RgbdPlane::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	// create(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:364
	// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
	#[inline]
	pub fn create(method: i32, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64) -> Result<core::Ptr<crate::rgbd::RgbdPlane>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdPlane>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [RgbdPlane::create] function uses the following default values for its arguments:
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	// cv::rgbd::RgbdPlane::create(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:364
	// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
	#[inline]
	pub fn create_def(method: i32, block_size: i32, min_size: i32, threshold: f64) -> Result<core::Ptr<crate::rgbd::RgbdPlane>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_create_int_int_int_double(method, block_size, min_size, threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::RgbdPlane>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { RgbdPlane, core::Algorithm, cv_rgbd_RgbdPlane_to_Algorithm }

impl std::fmt::Debug for RgbdPlane {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RgbdPlane")
			.finish()
	}
}
