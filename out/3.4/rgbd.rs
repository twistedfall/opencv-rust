//! # RGB-Depth Processing
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{DepthCleanerTrait, DepthCleanerTraitConst, ICPOdometryTrait, ICPOdometryTraitConst, LineMod_ColorGradientTrait, LineMod_ColorGradientTraitConst, LineMod_DepthNormalTrait, LineMod_DepthNormalTraitConst, LineMod_DetectorTrait, LineMod_DetectorTraitConst, LineMod_MatchTrait, LineMod_MatchTraitConst, LineMod_ModalityTrait, LineMod_ModalityTraitConst, LineMod_QuantizedPyramidTrait, LineMod_QuantizedPyramidTraitConst, LineMod_TemplateTrait, LineMod_TemplateTraitConst, OdometryFrameTrait, OdometryFrameTraitConst, OdometryTrait, OdometryTraitConst, RgbdFrameTrait, RgbdFrameTraitConst, RgbdICPOdometryTrait, RgbdICPOdometryTraitConst, RgbdNormalsTrait, RgbdNormalsTraitConst, RgbdOdometryTrait, RgbdOdometryTraitConst, RgbdPlaneTrait, RgbdPlaneTraitConst};
}

// DEPTH_CLEANER_NIL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:229
pub const DepthCleaner_DEPTH_CLEANER_NIL: i32 = 0;
// CACHE_ALL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:527
pub const OdometryFrame_CACHE_ALL: i32 = 3;
// CACHE_DST /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:527
pub const OdometryFrame_CACHE_DST: i32 = 2;
// CACHE_SRC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:527
pub const OdometryFrame_CACHE_SRC: i32 = 1;
// RIGID_BODY_MOTION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:564
pub const Odometry_RIGID_BODY_MOTION: i32 = 4;
// ROTATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:564
pub const Odometry_ROTATION: i32 = 1;
// TRANSLATION /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:564
pub const Odometry_TRANSLATION: i32 = 2;
// RGBD_NORMALS_METHOD_FALS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:112
pub const RgbdNormals_RGBD_NORMALS_METHOD_FALS: i32 = 0;
// RGBD_NORMALS_METHOD_LINEMOD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:113
pub const RgbdNormals_RGBD_NORMALS_METHOD_LINEMOD: i32 = 1;
// RGBD_NORMALS_METHOD_SRI /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:114
pub const RgbdNormals_RGBD_NORMALS_METHOD_SRI: i32 = 2;
// RGBD_PLANE_METHOD_DEFAULT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:367
pub const RgbdPlane_RGBD_PLANE_METHOD_DEFAULT: i32 = 0;
/// NIL method is from
/// ``Modeling Kinect Sensor Noise for Improved 3d Reconstruction and Tracking``
/// by C. Nguyen, S. Izadi, D. Lovel
// DEPTH_CLEANER_METHOD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:227
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

// RGBD_NORMALS_METHOD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:110
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

// RGBD_PLANE_METHOD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:365
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

/// \brief Debug function to colormap a quantized image for viewing.
// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:282
// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
#[inline]
pub fn colormap(quantized: &impl core::MatTraitConst, dst: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_linemod_colormap_const_MatR_MatR(quantized.as_raw_Mat(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// \brief Factory function for detector using LINE algorithm with color gradients.
///
/// Default parameter settings suitable for VGA images.
// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:448
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
// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:456
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
// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:333
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
// cv::rgbd::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:346
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
// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:346
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

// isValidDepth(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:67
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
// isValidDepth(const float &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:61
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const float*"]), _)]),
#[inline]
pub fn is_valid_depth(depth: &f32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_floatR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:86
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const int*"]), _)]),
#[inline]
pub fn is_valid_depth_4(depth: &i32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_intR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const short &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:73
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const short*"]), _)]),
#[inline]
pub fn is_valid_depth_2(depth: &i16) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_shortR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const unsigned int &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:92
// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned int*"]), _)]),
#[inline]
pub fn is_valid_depth_5(depth: &u32) -> Result<bool> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_intR(depth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// isValidDepth(const unsigned short &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:79
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
// cv::rgbd::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:321
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
// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:321
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
/// by 1000 to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
/// Otherwise, the image is simply converted to floats
/// ## Parameters
/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), it is assumed in meters)
/// * depth: the desired output depth (floats or double)
/// * out: The rescaled float depth image
// rescaleDepth(InputArray, int, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:358
// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn rescale_depth(in_: &impl ToInputArray, depth: i32, out: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(in_);
	output_array_arg!(out);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(in_.as_raw__InputArray(), depth, out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
// cv::rgbd::warpFrame(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1076
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
// warpFrame(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, OutputArray, OutputArray)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1076
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

/// Constant methods for [crate::rgbd::LineMod_ColorGradient]
// ColorGradient /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:203
pub trait LineMod_ColorGradientTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
	fn as_raw_LineMod_ColorGradient(&self) -> *const c_void;

	// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:228
	// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weak_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:229
	// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:230
	// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn strong_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:223
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:226
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

	// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:228
	// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_weak_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:229
	// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const_size_t(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:230
	// ("cv::linemod::ColorGradient::setStrong_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_strong_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:225
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
// ColorGradient /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:203
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
	// ColorGradient()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:209
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
	// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:219
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

	// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:221
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
// DepthNormal /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:240
pub trait LineMod_DepthNormalTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
	fn as_raw_LineMod_DepthNormal(&self) -> *const c_void;

	// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:269
	// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn distance_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:270
	// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn difference_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:271
	// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:272
	// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn extract_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:264
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:267
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

	// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:269
	// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_distance_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:270
	// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_difference_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:271
	// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const_size_t(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:272
	// ("cv::linemod::DepthNormal::setExtract_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_extract_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:266
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
// DepthNormal /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:240
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
	// DepthNormal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:246
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
	// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:258
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

	// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:261
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
// Detector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:326
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
	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:358
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
	// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:358
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
	// getModalities()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:387
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
	// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:392
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
	// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:397
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
	// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:405
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

	// numTemplates()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:407
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_templates(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numTemplates_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:408
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

	// numClasses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:409
	// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_classes(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numClasses_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// classIds()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:411
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:414
	// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_write_const_FileStorageR(self.as_raw_LineMod_Detector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:417
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
	// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:421
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
	// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:421
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
	// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:373
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
	// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:373
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
	// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:379
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:413
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
	// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:416
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
	// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:416
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
	// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:419
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
	// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:419
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
// Detector /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:326
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
	// Detector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:332
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
	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:341
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
// Feature /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:63
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
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:73
	// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	pub fn write(self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_write_const_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Feature()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:69
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::LineMod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:70
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn new(x: i32, y: i32, label: i32) -> Result<crate::rgbd::LineMod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature_int_int_int(x, y, label, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:72
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
// Match /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:287
pub trait LineMod_MatchTraitConst {
	fn as_raw_LineMod_Match(&self) -> *const c_void;

	// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:310
	// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn x(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propX_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:311
	// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn y(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propY_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:312
	// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn similarity(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:313
	// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn class_id(&self) -> String {
		let ret = unsafe { sys::cv_linemod_Match_propClass_id_const(self.as_raw_LineMod_Match()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:314
	// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn template_id(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const(self.as_raw_LineMod_Match()) };
		ret
	}

	/// Sort matches with high similarity to the front
	// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:296
	// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	#[inline]
	fn less_than(&self, rhs: &impl crate::rgbd::LineMod_MatchTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_operatorL_const_const_MatchR(self.as_raw_LineMod_Match(), rhs.as_raw_LineMod_Match(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:305
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

	// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:310
	// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_x(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propX_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:311
	// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_y(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propY_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:312
	// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_similarity(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const_float(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:313
	// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_class_id(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_linemod_Match_propClass_id_const_String(self.as_raw_mut_LineMod_Match(), val.opencv_as_extern()) };
		ret
	}

	// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:314
	// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_template_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

}

/// \brief Represents a successful template match.
// Match /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:287
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
	// Match()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:289
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

	// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:293
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
// Modality /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:156
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
	// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:169
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
	// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:169
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

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:175
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:178
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:177
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
// Modality /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:156
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
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:187
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
	// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:192
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
// QuantizedPyramid /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:92
pub trait LineMod_QuantizedPyramidTraitConst {
	fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void;

	/// \brief Compute quantized image at current pyramid level for online detection.
	///
	/// \param[out] dst The destination 8-bit image. For each pixel at most one bit is set,
	///                representing its classification.
	// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:104
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
	// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:111
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
	// pyrDown()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:118
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
// QuantizedPyramid /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:92
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
// Template /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:78
pub trait LineMod_TemplateTraitConst {
	fn as_raw_LineMod_Template(&self) -> *const c_void;

	// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:80
	// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn width(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propWidth_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:81
	// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn height(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propHeight_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:82
	// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_level(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:83
	// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn features(&self) -> core::Vector<crate::rgbd::LineMod_Feature> {
		let ret = unsafe { sys::cv_linemod_Template_propFeatures_const(self.as_raw_LineMod_Template()) };
		let ret = unsafe { core::Vector::<crate::rgbd::LineMod_Feature>::opencv_from_extern(ret) };
		ret
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:86
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

	// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:80
	// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_width(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propWidth_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:81
	// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_height(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propHeight_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:82
	// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:83
	// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
	#[inline]
	fn set_features(&mut self, val: core::Vector<crate::rgbd::LineMod_Feature>) {
		let ret = unsafe { sys::cv_linemod_Template_propFeatures_const_vectorLFeatureG(self.as_raw_mut_LineMod_Template(), val.as_raw_VectorOfLineMod_Feature()) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:85
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

// Template /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:78
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
// DepthCleaner /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:220
pub trait DepthCleanerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_DepthCleaner(&self) -> *const c_void;

	/// Given a set of 3d points in a depth image, compute the normals at each point.
	/// ## Parameters
	/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
	/// * depth: a rows x cols matrix of the cleaned up depth
	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:257
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
	// initialize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:263
	// ("cv::rgbd::DepthCleaner::initialize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn initialize(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_initialize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:265
	// ("cv::rgbd::DepthCleaner::getWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getWindowSize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:273
	// ("cv::rgbd::DepthCleaner::getDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_getDepth_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:281
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

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:269
	// ("cv::rgbd::DepthCleaner::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setWindowSize_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:277
	// ("cv::rgbd::DepthCleaner::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_depth(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_DepthCleaner_setDepth_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:285
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
// DepthCleaner /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:220
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
	// DepthCleaner()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:232
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
	// DepthCleaner(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:246
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
	// cv::rgbd::DepthCleaner::DepthCleaner(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:246
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
	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:250
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
	// cv::rgbd::DepthCleaner::create(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:250
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

/// Constant methods for [crate::rgbd::ICPOdometry]
// ICPOdometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:792
pub trait ICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
	fn as_raw_ICPOdometry(&self) -> *const c_void;

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:814
	// ("cv::rgbd::ICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_ICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:816
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

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:824
	// ("cv::rgbd::ICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMinDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:832
	// ("cv::rgbd::ICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:840
	// ("cv::rgbd::ICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepthDiff_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:848
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

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:856
	// ("cv::rgbd::ICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxPointsPart_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:864
	// ("cv::rgbd::ICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getTransformType_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:872
	// ("cv::rgbd::ICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxTranslation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:880
	// ("cv::rgbd::ICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_getMaxRotation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:888
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

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:820
	// ("cv::rgbd::ICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:828
	// ("cv::rgbd::ICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMinDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:836
	// ("cv::rgbd::ICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:844
	// ("cv::rgbd::ICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:852
	// ("cv::rgbd::ICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_iteration_counts(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:860
	// ("cv::rgbd::ICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxPointsPart_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:868
	// ("cv::rgbd::ICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setTransformType_int(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:876
	// ("cv::rgbd::ICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_ICPOdometry_setMaxTranslation_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:884
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
// ICPOdometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:792
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
	// ICPOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:795
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
	// ICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:806
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
	// cv::rgbd::ICPOdometry::ICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:806
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
	// create(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:810
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
	// cv::rgbd::ICPOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:810
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
// Odometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:557
pub trait OdometryTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Odometry(&self) -> *const c_void;

	/// Method to compute a transformation from the source frame to the destination one.
	/// Some odometry algorithms do not used some data of frames (eg. ICP does not use images).
	/// In such case corresponding arguments can be set as empty Mat.
	/// The method returns true if all internal computions were possible (e.g. there were enough correspondences,
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
	// compute(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:617
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
	/// The method returns true if all internal computions were possible (e.g. there were enough correspondences,
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
	// cv::rgbd::Odometry::compute(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:617
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
	// compute(Ptr<OdometryFrame> &, Ptr<OdometryFrame> &, OutputArray, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:624
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
	// cv::rgbd::Odometry::compute(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:624
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
	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:632
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
	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:637
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
	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:641
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
	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:639
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
	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:643
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
// Odometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:557
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
	// DEFAULT_MIN_DEPTH()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:568
	// ("cv::rgbd::Odometry::DEFAULT_MIN_DEPTH", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_min_depth() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_DEPTH()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:573
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_depth() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_DEPTH_DIFF()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:578
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_depth_diff() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_POINTS_PART()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:583
	// ("cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_points_part() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_TRANSLATION()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:588
	// ("cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_translation() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// DEFAULT_MAX_ROTATION()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:593
	// ("cv::rgbd::Odometry::DEFAULT_MAX_ROTATION", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default_max_rotation() -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:634
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
// OdometryFrame /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:516
pub trait OdometryFrameTraitConst: crate::rgbd::RgbdFrameTraitConst {
	fn as_raw_OdometryFrame(&self) -> *const c_void;

	// cv::rgbd::OdometryFrame::pyramidImage() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:541
	// ("cv::rgbd::OdometryFrame::pyramidImage", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_image(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidImage_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidDepth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:542
	// ("cv::rgbd::OdometryFrame::pyramidDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_depth(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidDepth_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:543
	// ("cv::rgbd::OdometryFrame::pyramidMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidCloud() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:545
	// ("cv::rgbd::OdometryFrame::pyramidCloud", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_cloud(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidCloud_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:547
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_d_i_dx(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dx_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dy() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:548
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dy", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_d_i_dy(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dy_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidTexturedMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:549
	// ("cv::rgbd::OdometryFrame::pyramidTexturedMask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_textured_mask(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidTexturedMask_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidNormals() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:551
	// ("cv::rgbd::OdometryFrame::pyramidNormals", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_normals(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormals_const(self.as_raw_OdometryFrame()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::OdometryFrame::pyramidNormalsMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:552
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

	// cv::rgbd::OdometryFrame::setPyramidImage(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:541
	// ("cv::rgbd::OdometryFrame::setPyramidImage", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_image(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidImage_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidDepth(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:542
	// ("cv::rgbd::OdometryFrame::setPyramidDepth", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_depth(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidDepth_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:543
	// ("cv::rgbd::OdometryFrame::setPyramidMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_mask(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidMask_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidCloud(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:545
	// ("cv::rgbd::OdometryFrame::setPyramidCloud", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_cloud(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidCloud_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:547
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dx", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_d_i_dx(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dx_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dy(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:548
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dy", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_d_i_dy(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dy_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidTexturedMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:549
	// ("cv::rgbd::OdometryFrame::setPyramidTexturedMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_textured_mask(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidTexturedMask_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidNormals(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:551
	// ("cv::rgbd::OdometryFrame::setPyramidNormals", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_normals(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormals_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// cv::rgbd::OdometryFrame::setPyramidNormalsMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:552
	// ("cv::rgbd::OdometryFrame::setPyramidNormalsMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_pyramid_normals_mask(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormalsMask_const_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_VectorOfMat()) };
		ret
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:536
	// ("cv::rgbd::OdometryFrame::release", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_OdometryFrame_release(self.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// releasePyramids()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:539
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
// OdometryFrame /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:516
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
	// OdometryFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:530
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
	// OdometryFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:531
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
	// cv::rgbd::OdometryFrame::OdometryFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:531
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
	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:533
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
	// cv::rgbd::OdometryFrame::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:533
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
// RgbdFrame /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:494
pub trait RgbdFrameTraitConst {
	fn as_raw_RgbdFrame(&self) -> *const c_void;

	// cv::rgbd::RgbdFrame::ID() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:505
	// ("cv::rgbd::RgbdFrame::ID", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn id(&self) -> i32 {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propID_const(self.as_raw_RgbdFrame()) };
		ret
	}

	// cv::rgbd::RgbdFrame::image() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:506
	// ("cv::rgbd::RgbdFrame::image", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn image(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propImage_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::RgbdFrame::depth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:507
	// ("cv::rgbd::RgbdFrame::depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propDepth_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::RgbdFrame::mask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:508
	// ("cv::rgbd::RgbdFrame::mask", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn mask(&self) -> core::Mat {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propMask_const(self.as_raw_RgbdFrame()) };
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		ret
	}

	// cv::rgbd::RgbdFrame::normals() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:509
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

	// cv::rgbd::RgbdFrame::setID(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:505
	// ("cv::rgbd::RgbdFrame::setID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propID_const_int(self.as_raw_mut_RgbdFrame(), val) };
		ret
	}

	// cv::rgbd::RgbdFrame::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:506
	// ("cv::rgbd::RgbdFrame::setImage", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_image(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propImage_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// cv::rgbd::RgbdFrame::setDepth(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:507
	// ("cv::rgbd::RgbdFrame::setDepth", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_depth(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propDepth_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// cv::rgbd::RgbdFrame::setMask(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:508
	// ("cv::rgbd::RgbdFrame::setMask", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_mask(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propMask_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// cv::rgbd::RgbdFrame::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:509
	// ("cv::rgbd::RgbdFrame::setNormals", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_normals(&mut self, val: core::Mat) {
		let ret = unsafe { sys::cv_rgbd_RgbdFrame_propNormals_const_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_Mat()) };
		ret
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:503
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
// RgbdFrame /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:494
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
	// RgbdFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:496
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
	// RgbdFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:497
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
	// cv::rgbd::RgbdFrame::RgbdFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:497
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
	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:500
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
	// cv::rgbd::RgbdFrame::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:500
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
// RgbdICPOdometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:920
pub trait RgbdICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
	fn as_raw_RgbdICPOdometry(&self) -> *const c_void;

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:948
	// ("cv::rgbd::RgbdICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_RgbdICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:950
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

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:958
	// ("cv::rgbd::RgbdICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMinDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:966
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:974
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:982
	// ("cv::rgbd::RgbdICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:990
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

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:998
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

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1006
	// ("cv::rgbd::RgbdICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getTransformType_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1014
	// ("cv::rgbd::RgbdICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1022
	// ("cv::rgbd::RgbdICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_rotation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxRotation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1030
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

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:954
	// ("cv::rgbd::RgbdICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:962
	// ("cv::rgbd::RgbdICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:970
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:978
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:986
	// ("cv::rgbd::RgbdICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:994
	// ("cv::rgbd::RgbdICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_iteration_counts(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1002
	// ("cv::rgbd::RgbdICPOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_min_gradient_magnitudes(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1010
	// ("cv::rgbd::RgbdICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setTransformType_int(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1018
	// ("cv::rgbd::RgbdICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1026
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
// RgbdICPOdometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:920
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
	// RgbdICPOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:923
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
	// RgbdICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:936
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
	// cv::rgbd::RgbdICPOdometry::RgbdICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:936
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
	// create(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:942
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
	// cv::rgbd::RgbdICPOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:942
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
// RgbdNormals /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:107
pub trait RgbdNormalsTraitConst: core::AlgorithmTraitConst {
	fn as_raw_RgbdNormals(&self) -> *const c_void;

	/// Given a set of 3d points in a depth image, compute the normals at each point.
	/// ## Parameters
	/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
	/// * normals: a rows x cols x 3 matrix
	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:150
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
	// initialize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:156
	// ("cv::rgbd::RgbdNormals::initialize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn initialize(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_initialize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getRows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:158
	// ("cv::rgbd::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_rows(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getRows_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCols()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:166
	// ("cv::rgbd::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_cols(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getCols_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:174
	// ("cv::rgbd::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_window_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getWindowSize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:182
	// ("cv::rgbd::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_getDepth_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:190
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

	// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:198
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

	// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:162
	// ("cv::rgbd::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_rows(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setRows_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:170
	// ("cv::rgbd::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_cols(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setCols_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:178
	// ("cv::rgbd::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setWindowSize_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:186
	// ("cv::rgbd::RgbdNormals::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_depth(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setDepth_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setK(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:194
	// ("cv::rgbd::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_k(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdNormals_setK_const_MatR(self.as_raw_mut_RgbdNormals(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:202
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
// RgbdNormals /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:107
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
	// RgbdNormals()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:117
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
	// RgbdNormals(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:137
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
	// cv::rgbd::RgbdNormals::RgbdNormals(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:137
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
	// create(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:142
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
	// cv::rgbd::RgbdNormals::create(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:142
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
// RgbdOdometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:657
pub trait RgbdOdometryTraitConst: crate::rgbd::OdometryTraitConst {
	fn as_raw_RgbdOdometry(&self) -> *const c_void;

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:683
	// ("cv::rgbd::RgbdOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	#[inline]
	fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_RgbdOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:685
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

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:693
	// ("cv::rgbd::RgbdOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMinDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:701
	// ("cv::rgbd::RgbdOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:709
	// ("cv::rgbd::RgbdOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_depth_diff(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:717
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

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:725
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

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:733
	// ("cv::rgbd::RgbdOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_points_part(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxPointsPart_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:741
	// ("cv::rgbd::RgbdOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_transform_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getTransformType_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:749
	// ("cv::rgbd::RgbdOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_max_translation(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxTranslation_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:757
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

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:689
	// ("cv::rgbd::RgbdOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_camera_matrix(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:697
	// ("cv::rgbd::RgbdOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:705
	// ("cv::rgbd::RgbdOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:713
	// ("cv::rgbd::RgbdOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:721
	// ("cv::rgbd::RgbdOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_iteration_counts(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:729
	// ("cv::rgbd::RgbdOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_min_gradient_magnitudes(&mut self, val: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:737
	// ("cv::rgbd::RgbdOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:745
	// ("cv::rgbd::RgbdOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setTransformType_int(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:753
	// ("cv::rgbd::RgbdOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:761
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
// RgbdOdometry /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:657
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
	// RgbdOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:660
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
	// RgbdOdometry(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:673
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
	// cv::rgbd::RgbdOdometry::RgbdOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:673
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
	// create(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:678
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
	// cv::rgbd::RgbdOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:678
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
// RgbdPlane /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:362
pub trait RgbdPlaneTraitConst: core::AlgorithmTraitConst {
	fn as_raw_RgbdPlane(&self) -> *const c_void;

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:422
	// ("cv::rgbd::RgbdPlane::getBlockSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_block_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getBlockSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:430
	// ("cv::rgbd::RgbdPlane::getMinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_min_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getMinSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:438
	// ("cv::rgbd::RgbdPlane::getMethod", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_method(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getMethod_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:446
	// ("cv::rgbd::RgbdPlane::getThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getThreshold_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSensorErrorA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:454
	// ("cv::rgbd::RgbdPlane::getSensorErrorA", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sensor_error_a(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorA_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSensorErrorB()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:462
	// ("cv::rgbd::RgbdPlane::getSensorErrorB", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sensor_error_b(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorB_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getSensorErrorC()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:470
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
	// operator()(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:410
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
	// operator()(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:420
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

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:426
	// ("cv::rgbd::RgbdPlane::setBlockSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_block_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setBlockSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:434
	// ("cv::rgbd::RgbdPlane::setMinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_min_size(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setMinSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:442
	// ("cv::rgbd::RgbdPlane::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_method(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setMethod_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:450
	// ("cv::rgbd::RgbdPlane::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setThreshold_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSensorErrorA(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:458
	// ("cv::rgbd::RgbdPlane::setSensorErrorA", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_sensor_error_a(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorA_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSensorErrorB(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:466
	// ("cv::rgbd::RgbdPlane::setSensorErrorB", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_sensor_error_b(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorB_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setSensorErrorC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:474
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
// RgbdPlane /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:362
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
	// RgbdPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:370
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
	// cv::rgbd::RgbdPlane::RgbdPlane() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:370
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
	// RgbdPlane(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:391
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
	// cv::rgbd::RgbdPlane::RgbdPlane(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:391
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
	// create(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:397
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
	// cv::rgbd::RgbdPlane::create(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:397
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
