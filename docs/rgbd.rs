pub mod rgbd {
	//! # RGB-Depth Processing
	//! 
	//! [kinfu_icp]
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::LineMod_TemplateTraitConst, super::LineMod_TemplateTrait, super::LineMod_QuantizedPyramidTraitConst, super::LineMod_QuantizedPyramidTrait, super::LineMod_ModalityTraitConst, super::LineMod_ModalityTrait, super::LineMod_ColorGradientTraitConst, super::LineMod_ColorGradientTrait, super::LineMod_DepthNormalTraitConst, super::LineMod_DepthNormalTrait, super::LineMod_MatchTraitConst, super::LineMod_MatchTrait, super::LineMod_DetectorTraitConst, super::LineMod_DetectorTrait, super::RgbdNormalsTraitConst, super::RgbdNormalsTrait, super::DepthCleanerTraitConst, super::DepthCleanerTrait, super::RgbdPlaneTraitConst, super::RgbdPlaneTrait, super::RgbdFrameTraitConst, super::RgbdFrameTrait, super::OdometryFrameTraitConst, super::OdometryFrameTrait, super::OdometryTraitConst, super::OdometryTrait, super::RgbdOdometryTraitConst, super::RgbdOdometryTrait, super::ICPOdometryTraitConst, super::ICPOdometryTrait, super::RgbdICPOdometryTraitConst, super::RgbdICPOdometryTrait, super::FastICPOdometryTraitConst, super::FastICPOdometryTrait, super::Kinfu_VolumeTraitConst, super::Kinfu_VolumeTrait, super::Kinfu_VolumeParamsTraitConst, super::Kinfu_VolumeParamsTrait, super::Kinfu_ParamsTraitConst, super::Kinfu_ParamsTrait, super::Kinfu_KinFuTraitConst, super::Kinfu_KinFuTrait, super::Dynafu_DynaFuTraitConst, super::Dynafu_DynaFuTrait, super::ParamsTraitConst, super::ParamsTrait, super::LargeKinfuTraitConst, super::LargeKinfuTrait, super::Kinfu_Detail_PoseGraphTraitConst, super::Kinfu_Detail_PoseGraphTrait, super::ColoredKinfu_ParamsTraitConst, super::ColoredKinfu_ParamsTrait, super::ColoredKinfu_ColoredKinFuTraitConst, super::ColoredKinfu_ColoredKinFuTrait };
	}
	
	pub const DepthCleaner_DEPTH_CLEANER_NIL: i32 = 0;
	pub const Kinfu_VolumeType_COLOREDTSDF: i32 = 2;
	pub const Kinfu_VolumeType_HASHTSDF: i32 = 1;
	pub const Kinfu_VolumeType_TSDF: i32 = 0;
	pub const OdometryFrame_CACHE_ALL: i32 = 3;
	pub const OdometryFrame_CACHE_DST: i32 = 2;
	pub const OdometryFrame_CACHE_SRC: i32 = 1;
	pub const Odometry_RIGID_BODY_MOTION: i32 = 4;
	pub const Odometry_ROTATION: i32 = 1;
	pub const Odometry_TRANSLATION: i32 = 2;
	pub const RgbdNormals_RGBD_NORMALS_METHOD_FALS: i32 = 0;
	pub const RgbdNormals_RGBD_NORMALS_METHOD_LINEMOD: i32 = 1;
	pub const RgbdNormals_RGBD_NORMALS_METHOD_SRI: i32 = 2;
	pub const RgbdPlane_RGBD_PLANE_METHOD_DEFAULT: i32 = 0;
	/// NIL method is from
	/// ``Modeling Kinect Sensor Noise for Improved 3d Reconstruction and Tracking``
	/// by C. Nguyen, S. Izadi, D. Lovel
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum DepthCleaner_DEPTH_CLEANER_METHOD {
		DEPTH_CLEANER_NIL = 0,
	}
	
	opencv_type_enum! { crate::rgbd::DepthCleaner_DEPTH_CLEANER_METHOD }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Kinfu_VolumeType {
		TSDF = 0,
		HASHTSDF = 1,
		COLOREDTSDF = 2,
	}
	
	opencv_type_enum! { crate::rgbd::Kinfu_VolumeType }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum RgbdNormals_RGBD_NORMALS_METHOD {
		RGBD_NORMALS_METHOD_FALS = 0,
		RGBD_NORMALS_METHOD_LINEMOD = 1,
		RGBD_NORMALS_METHOD_SRI = 2,
	}
	
	opencv_type_enum! { crate::rgbd::RgbdNormals_RGBD_NORMALS_METHOD }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum RgbdPlane_RGBD_PLANE_METHOD {
		RGBD_PLANE_METHOD_DEFAULT = 0,
	}
	
	opencv_type_enum! { crate::rgbd::RgbdPlane_RGBD_PLANE_METHOD }
	
	/// Backwards compatibility for old versions
	pub type Dynafu_Params = crate::rgbd::Kinfu_Params;
	#[inline]
	pub fn make_volume(_volume_type: crate::rgbd::Kinfu_VolumeType, _voxel_size: f32, _pose: core::Matx44f, _raycast_step_factor: f32, _trunc_dist: f32, _max_weight: i32, _truncate_threshold: f32, _resolution: core::Vec3i) -> Result<core::Ptr<crate::rgbd::Kinfu_Volume>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_makeVolume_VolumeType_float_Matx44f_float_float_int_float_Vec3i(_volume_type, _voxel_size, _pose.opencv_as_extern(), _raycast_step_factor, _trunc_dist, _max_weight, _truncate_threshold, _resolution.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Volume>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// \brief Debug function to colormap a quantized image for viewing.
	#[inline]
	pub fn colormap(quantized: &core::Mat, dst: &mut core::Mat) -> Result<()> {
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
	#[inline]
	pub fn draw_features_def(img: &mut impl core::ToInputOutputArray, templates: &core::Vector<crate::rgbd::LineMod_Template>, tl: core::Point2i) -> Result<()> {
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
	#[inline]
	pub fn draw_features(img: &mut impl core::ToInputOutputArray, templates: &core::Vector<crate::rgbd::LineMod_Template>, tl: core::Point2i, size: i32) -> Result<()> {
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
	#[inline]
	pub fn depth_to3d_sparse(depth: &impl core::ToInputArray, in_k: &impl core::ToInputArray, in_points: &impl core::ToInputArray, points3d: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn depth_to3d_def(depth: &impl core::ToInputArray, k: &impl core::ToInputArray, points3d: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn depth_to3d(depth: &impl core::ToInputArray, k: &impl core::ToInputArray, points3d: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn is_valid_depth(depth: &f32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_isValidDepth_const_floatR(depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn is_valid_depth_4(depth: &i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_isValidDepth_const_intR(depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn is_valid_depth_2(depth: &i16) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_isValidDepth_const_shortR(depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn is_valid_depth_5(depth: &u32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_intR(depth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
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
	#[inline]
	pub fn register_depth_def(unregistered_camera_matrix: &impl core::ToInputArray, registered_camera_matrix: &impl core::ToInputArray, registered_dist_coeffs: &impl core::ToInputArray, rt: &impl core::ToInputArray, unregistered_depth: &impl core::ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn register_depth(unregistered_camera_matrix: &impl core::ToInputArray, registered_camera_matrix: &impl core::ToInputArray, registered_dist_coeffs: &impl core::ToInputArray, rt: &impl core::ToInputArray, unregistered_depth: &impl core::ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl core::ToOutputArray, depth_dilation: bool) -> Result<()> {
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
	#[inline]
	pub fn rescale_depth_def(in_: &impl core::ToInputArray, depth: i32, out: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn rescale_depth(in_: &impl core::ToInputArray, depth: i32, out: &mut impl core::ToOutputArray, depth_factor: f64) -> Result<()> {
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
	#[inline]
	pub fn warp_frame_def(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, rt: &core::Mat, camera_matrix: &core::Mat, dist_coeff: &core::Mat, warped_image: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn warp_frame(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, rt: &core::Mat, camera_matrix: &core::Mat, dist_coeff: &core::Mat, warped_image: &mut impl core::ToOutputArray, warped_depth: &mut impl core::ToOutputArray, warped_mask: &mut impl core::ToOutputArray) -> Result<()> {
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
	pub trait ColoredKinfu_ColoredKinFuTraitConst {
		fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void;
	
		/// Get current parameters
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
		#[inline]
		fn render(&self, image: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn render_1(&self, image: &mut impl core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
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
		#[inline]
		fn get_cloud(&self, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray, colors: &mut impl core::ToOutputArray) -> Result<()> {
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
		/// This alternative version of [get_cloud] function uses the following default values for its arguments:
		/// * colors: noArray()
		#[inline]
		fn get_cloud_def(&self, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_points(&self, points: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_normals(&self, points: &impl core::ToInputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Get current pose in voxel space
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
		#[inline]
		fn update(&mut self, depth: &impl core::ToInputArray, rgb: &impl core::ToInputArray) -> Result<bool> {
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
	/// [kinectfusion](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_kinectfusion) paper.
	/// 
	/// It takes a sequence of depth images taken from depth sensor
	/// (or any depth images source such as stereo camera matching algorithm or even raymarching renderer).
	/// The output can be obtained as a vector of points and their normals
	/// or can be Phong-rendered from given camera pose.
	/// 
	/// An internal representation of a model is a voxel cuboid that keeps TSDF values
	/// which are a sort of distances to the surface (for details read the [kinectfusion](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_kinectfusion) article about TSDF).
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
	pub struct ColoredKinfu_ColoredKinFu {
		ptr: *mut c_void
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
	
	impl ColoredKinfu_ColoredKinFu {
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
	pub trait ColoredKinfu_ParamsTraitConst {
		fn as_raw_ColoredKinfu_Params(&self) -> *const c_void;
	
		/// frame size in pixels
		#[inline]
		fn frame_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propFrameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// rgb frame size in pixels
		#[inline]
		fn rgb_frame_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn volume_type(&self) -> crate::rgbd::Kinfu_VolumeType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propVolumeType_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// camera intrinsics
		#[inline]
		fn intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propIntr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// rgb camera intrinsics
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
		#[inline]
		fn depth_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propDepthFactor_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn bilateral_sigma_depth(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn bilateral_sigma_spatial(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn bilateral_kernel_size(&self) -> i32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// Number of pyramid levels for ICP
		#[inline]
		fn pyramid_levels(&self) -> i32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propPyramidLevels_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// Resolution of voxel space
		/// 
		/// Number of voxels in each dimension.
		#[inline]
		fn volume_dims(&self) -> core::Vec3i {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propVolumeDims_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Size of voxel in meters
		#[inline]
		fn voxel_size(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVoxelSize_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// Minimal camera movement in meters
		/// 
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn tsdf_min_camera_movement(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// initial volume pose in meters
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
		#[inline]
		fn tsdf_trunc_dist(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_trunc_dist_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// max number of frames per voxel
		/// 
		/// Each voxel keeps running average of distances no longer than this value.
		#[inline]
		fn tsdf_max_weight(&self) -> i32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_max_weight_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// A length of one raycast step
		/// 
		/// How much voxel sizes we skip each raycast step
		#[inline]
		fn raycast_step_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propRaycast_step_factor_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// light pose for rendering in meters
		#[inline]
		fn light_pose(&self) -> core::Vec3f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propLightPose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// distance theshold for ICP in meters
		#[inline]
		fn icp_dist_thresh(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpDistThresh_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// angle threshold for ICP in radians
		#[inline]
		fn icp_angle_thresh(&self) -> f32 {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpAngleThresh_const(self.as_raw_ColoredKinfu_Params()) };
			ret
		}
		
		/// number of ICP iterations for each pyramid level
		#[inline]
		fn icp_iterations(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpIterations_const(self.as_raw_ColoredKinfu_Params()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// 
		/// All depth values beyond this threshold will be set to zero
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
		#[inline]
		fn set_frame_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propFrameSize_Size(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// rgb frame size in pixels
		#[inline]
		fn set_rgb_frame_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_Size(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_volume_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeType_VolumeType(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// camera intrinsics
		#[inline]
		fn set_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIntr_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// rgb camera intrinsics
		#[inline]
		fn set_rgb_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propRgb_intr_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// pre-scale per 1 meter for input values
		/// 
		/// Typical values are:
		///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
		///      * 1000 per 1 meter for Kinect 2 device
		///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
		#[inline]
		fn set_depth_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propDepthFactor_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_depth(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_depth_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_spatial(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_spatial_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_kernel_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_kernel_size_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// Number of pyramid levels for ICP
		#[inline]
		fn set_pyramid_levels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propPyramidLevels_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// Resolution of voxel space
		/// 
		/// Number of voxels in each dimension.
		#[inline]
		fn set_volume_dims(&mut self, val: core::Vec3i) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeDims_Vec3i(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// Size of voxel in meters
		#[inline]
		fn set_voxel_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVoxelSize_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// Minimal camera movement in meters
		/// 
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn set_tsdf_min_camera_movement(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_min_camera_movement_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// initial volume pose in meters
		#[inline]
		fn set_volume_pose(&mut self, val: core::Affine3f) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumePose_Affine3f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// distance to truncate in meters
		/// 
		/// Distances to surface that exceed this value will be truncated to 1.0.
		#[inline]
		fn set_tsdf_trunc_dist(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_trunc_dist_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// max number of frames per voxel
		/// 
		/// Each voxel keeps running average of distances no longer than this value.
		#[inline]
		fn set_tsdf_max_weight(&mut self, val: i32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_max_weight_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// A length of one raycast step
		/// 
		/// How much voxel sizes we skip each raycast step
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propRaycast_step_factor_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// light pose for rendering in meters
		#[inline]
		fn set_light_pose(&mut self, val: core::Vec3f) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propLightPose_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// distance theshold for ICP in meters
		#[inline]
		fn set_icp_dist_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpDistThresh_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// angle threshold for ICP in radians
		#[inline]
		fn set_icp_angle_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpAngleThresh_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// number of ICP iterations for each pyramid level
		#[inline]
		fn set_icp_iterations(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpIterations_vectorLintG(self.as_raw_mut_ColoredKinfu_Params(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// 
		/// All depth values beyond this threshold will be set to zero
		#[inline]
		fn set_truncate_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTruncateThreshold_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}
		
		/// Set Initial Volume Pose
		/// Sets the initial pose of the TSDF volume.
		/// ## Parameters
		/// * R: rotation matrix
		/// * t: translation vector
		#[inline]
		fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), r.opencv_as_extern(), t.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Set Initial Volume Pose
		/// Sets the initial pose of the TSDF volume.
		/// ## Parameters
		/// * homogen_tf: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
		#[inline]
		fn set_initial_volume_pose_1(&mut self, homogen_tf: core::Matx44f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_ColoredKinfu_Params(), homogen_tf.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct ColoredKinfu_Params {
		ptr: *mut c_void
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
	
	impl ColoredKinfu_Params {
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
		#[inline]
		pub fn new(volume_initial_pose_rot: core::Matx33f, volume_initial_pose_transl: core::Vec3f) -> Result<crate::rgbd::ColoredKinfu_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot.opencv_as_extern(), volume_initial_pose_transl.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructor for Params
		/// Sets the initial pose of the TSDF volume.
		/// ## Parameters
		/// * volumeInitialPose: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
		#[inline]
		pub fn new_1(volume_initial_pose: core::Matx44f) -> Result<crate::rgbd::ColoredKinfu_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_Params_Matx44f(volume_initial_pose.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::ColoredKinfu_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Default parameters
		/// A set of parameters which provides better model quality, can be very slow.
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
	pub trait Dynafu_DynaFuTraitConst {
		fn as_raw_Dynafu_DynaFu(&self) -> *const c_void;
	
		/// Get current parameters
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
		#[inline]
		fn render(&self, image: &mut impl core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
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
		/// This alternative version of [render] function uses the following default values for its arguments:
		/// * camera_pose: Matx44f::eye()
		#[inline]
		fn render_def(&self, image: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_cloud(&self, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_points(&self, points: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_normals(&self, points: &impl core::ToInputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Get current pose in voxel space
		#[inline]
		fn get_pose(&self) -> Result<core::Affine3f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getPose_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nodes_pos(&self) -> Result<core::Vector<core::Point3f>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getNodesPos_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn march_cubes(&self, vertices: &mut impl core::ToOutputArray, edges: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn update(&mut self, depth: &impl core::ToInputArray) -> Result<bool> {
			input_array_arg!(depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_update_const__InputArrayR(self.as_raw_mut_Dynafu_DynaFu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * warp: true
		#[inline]
		fn render_surface(&mut self, depth_image: &mut impl core::ToOutputArray, vert_image: &mut impl core::ToOutputArray, norm_image: &mut impl core::ToOutputArray, warp: bool) -> Result<()> {
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
		/// This alternative version of [render_surface] function uses the following default values for its arguments:
		/// * warp: true
		#[inline]
		fn render_surface_def(&mut self, depth_image: &mut impl core::ToOutputArray, vert_image: &mut impl core::ToOutputArray, norm_image: &mut impl core::ToOutputArray) -> Result<()> {
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
	
	pub struct Dynafu_DynaFu {
		ptr: *mut c_void
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
	
	impl Dynafu_DynaFu {
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
		#[inline]
		pub fn scale(self, pyr: i32) -> Result<crate::rgbd::Kinfu_Intr> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_scale_const_int(self.opencv_as_extern(), pyr, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn make_reprojector(self) -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_makeReprojector_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn make_projector(self) -> Result<crate::rgbd::Kinfu_Intr_Projector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_makeProjector_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn get_mat(self) -> Result<core::Matx33f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_getMat_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::rgbd::Kinfu_Intr> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_Intr(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn new(_fx: f32, _fy: f32, _cx: f32, _cy: f32) -> Result<crate::rgbd::Kinfu_Intr> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_Intr_float_float_float_float(_fx, _fy, _cx, _cy, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn new_1(m: core::Matx33f) -> Result<crate::rgbd::Kinfu_Intr> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_Intr_Matx33f(m.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Projects camera space vector onto screen
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
		#[inline]
		pub fn new(intr: crate::rgbd::Kinfu_Intr) -> Result<crate::rgbd::Kinfu_Intr_Projector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_Projector_Projector_Intr(intr.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Camera intrinsics
	/// Reprojects screen point to camera space given z coord.
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
		#[inline]
		pub fn default() -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_Reprojector_Reprojector(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn new(intr: crate::rgbd::Kinfu_Intr) -> Result<crate::rgbd::Kinfu_Intr_Reprojector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Intr_Reprojector_Reprojector_Intr(intr.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::rgbd::Kinfu_KinFu]
	pub trait Kinfu_KinFuTraitConst {
		fn as_raw_Kinfu_KinFu(&self) -> *const c_void;
	
		/// Get current parameters
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
		#[inline]
		fn render(&self, image: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn render_1(&self, image: &mut impl core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
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
		#[inline]
		fn get_cloud(&self, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_points(&self, points: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_normals(&self, points: &impl core::ToInputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Get current pose in voxel space
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
		#[inline]
		fn update(&mut self, depth: &impl core::ToInputArray) -> Result<bool> {
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
	/// [kinectfusion](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_kinectfusion) paper.
	/// 
	/// It takes a sequence of depth images taken from depth sensor
	/// (or any depth images source such as stereo camera matching algorithm or even raymarching renderer).
	/// The output can be obtained as a vector of points and their normals
	/// or can be Phong-rendered from given camera pose.
	/// 
	/// An internal representation of a model is a voxel cuboid that keeps TSDF values
	/// which are a sort of distances to the surface (for details read the [kinectfusion](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_kinectfusion) article about TSDF).
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
	pub struct Kinfu_KinFu {
		ptr: *mut c_void
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
	
	impl Kinfu_KinFu {
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
	pub trait Kinfu_ParamsTraitConst {
		fn as_raw_Kinfu_Params(&self) -> *const c_void;
	
		/// frame size in pixels
		#[inline]
		fn frame_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propFrameSize_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// rgb frame size in pixels
		#[inline]
		fn volume_type(&self) -> crate::rgbd::Kinfu_VolumeType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propVolumeType_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// camera intrinsics
		#[inline]
		fn intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propIntr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// rgb camera intrinsics
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
		#[inline]
		fn depth_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propDepthFactor_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn bilateral_sigma_depth(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn bilateral_sigma_spatial(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn bilateral_kernel_size(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// Number of pyramid levels for ICP
		#[inline]
		fn pyramid_levels(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_Params_propPyramidLevels_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// Resolution of voxel space
		/// 
		/// Number of voxels in each dimension.
		#[inline]
		fn volume_dims(&self) -> core::Vec3i {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propVolumeDims_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Size of voxel in meters
		#[inline]
		fn voxel_size(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propVoxelSize_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// Minimal camera movement in meters
		/// 
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn tsdf_min_camera_movement(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// initial volume pose in meters
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
		#[inline]
		fn tsdf_trunc_dist(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_trunc_dist_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// max number of frames per voxel
		/// 
		/// Each voxel keeps running average of distances no longer than this value.
		#[inline]
		fn tsdf_max_weight(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_max_weight_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// A length of one raycast step
		/// 
		/// How much voxel sizes we skip each raycast step
		#[inline]
		fn raycast_step_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propRaycast_step_factor_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// light pose for rendering in meters
		#[inline]
		fn light_pose(&self) -> core::Vec3f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propLightPose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// distance theshold for ICP in meters
		#[inline]
		fn icp_dist_thresh(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpDistThresh_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// angle threshold for ICP in radians
		#[inline]
		fn icp_angle_thresh(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpAngleThresh_const(self.as_raw_Kinfu_Params()) };
			ret
		}
		
		/// number of ICP iterations for each pyramid level
		#[inline]
		fn icp_iterations(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpIterations_const(self.as_raw_Kinfu_Params()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// 
		/// All depth values beyond this threshold will be set to zero
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
		#[inline]
		fn set_frame_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_kinfu_Params_propFrameSize_Size(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// rgb frame size in pixels
		#[inline]
		fn set_volume_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
			let ret = unsafe { sys::cv_kinfu_Params_propVolumeType_VolumeType(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// camera intrinsics
		#[inline]
		fn set_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_kinfu_Params_propIntr_Matx33f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// rgb camera intrinsics
		#[inline]
		fn set_rgb_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_kinfu_Params_propRgb_intr_Matx33f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// pre-scale per 1 meter for input values
		/// 
		/// Typical values are:
		///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
		///      * 1000 per 1 meter for Kinect 2 device
		///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
		#[inline]
		fn set_depth_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propDepthFactor_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_depth(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_depth_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_spatial(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_spatial_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_kernel_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_kernel_size_int(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// Number of pyramid levels for ICP
		#[inline]
		fn set_pyramid_levels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_Params_propPyramidLevels_int(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// Resolution of voxel space
		/// 
		/// Number of voxels in each dimension.
		#[inline]
		fn set_volume_dims(&mut self, val: core::Vec3i) {
			let ret = unsafe { sys::cv_kinfu_Params_propVolumeDims_Vec3i(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// Size of voxel in meters
		#[inline]
		fn set_voxel_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propVoxelSize_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// Minimal camera movement in meters
		/// 
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn set_tsdf_min_camera_movement(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_min_camera_movement_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// initial volume pose in meters
		#[inline]
		fn set_volume_pose(&mut self, val: core::Affine3f) {
			let ret = unsafe { sys::cv_kinfu_Params_propVolumePose_Affine3f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// distance to truncate in meters
		/// 
		/// Distances to surface that exceed this value will be truncated to 1.0.
		#[inline]
		fn set_tsdf_trunc_dist(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_trunc_dist_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// max number of frames per voxel
		/// 
		/// Each voxel keeps running average of distances no longer than this value.
		#[inline]
		fn set_tsdf_max_weight(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_max_weight_int(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// A length of one raycast step
		/// 
		/// How much voxel sizes we skip each raycast step
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propRaycast_step_factor_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// light pose for rendering in meters
		#[inline]
		fn set_light_pose(&mut self, val: core::Vec3f) {
			let ret = unsafe { sys::cv_kinfu_Params_propLightPose_Vec3f(self.as_raw_mut_Kinfu_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// distance theshold for ICP in meters
		#[inline]
		fn set_icp_dist_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpDistThresh_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// angle threshold for ICP in radians
		#[inline]
		fn set_icp_angle_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpAngleThresh_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// number of ICP iterations for each pyramid level
		#[inline]
		fn set_icp_iterations(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpIterations_vectorLintG(self.as_raw_mut_Kinfu_Params(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// 
		/// All depth values beyond this threshold will be set to zero
		#[inline]
		fn set_truncate_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propTruncateThreshold_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}
		
		/// Set Initial Volume Pose
		/// Sets the initial pose of the TSDF volume.
		/// ## Parameters
		/// * R: rotation matrix
		/// * t: translation vector
		#[inline]
		fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_Kinfu_Params(), r.opencv_as_extern(), t.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Set Initial Volume Pose
		/// Sets the initial pose of the TSDF volume.
		/// ## Parameters
		/// * homogen_tf: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
		#[inline]
		fn set_initial_volume_pose_1(&mut self, homogen_tf: core::Matx44f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_Kinfu_Params(), homogen_tf.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Kinfu_Params {
		ptr: *mut c_void
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
	
	impl Kinfu_Params {
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
		#[inline]
		pub fn new(volume_initial_pose_rot: core::Matx33f, volume_initial_pose_transl: core::Vec3f) -> Result<crate::rgbd::Kinfu_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_Params_Matx33f_Vec3f(volume_initial_pose_rot.opencv_as_extern(), volume_initial_pose_transl.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructor for Params
		/// Sets the initial pose of the TSDF volume.
		/// ## Parameters
		/// * volumeInitialPose: 4 by 4 Homogeneous Transform matrix to set the intial pose of TSDF volume
		#[inline]
		pub fn new_1(volume_initial_pose: core::Matx44f) -> Result<crate::rgbd::Kinfu_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_Params_Matx44f(volume_initial_pose.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::Kinfu_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Default parameters
		/// A set of parameters which provides better model quality, can be very slow.
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
	pub trait Kinfu_VolumeTraitConst {
		fn as_raw_Kinfu_Volume(&self) -> *const c_void;
	
		#[inline]
		fn voxel_size(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Volume_propVoxelSize_const(self.as_raw_Kinfu_Volume()) };
			ret
		}
		
		#[inline]
		fn voxel_size_inv(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Volume_propVoxelSizeInv_const(self.as_raw_Kinfu_Volume()) };
			ret
		}
		
		#[inline]
		fn pose(&self) -> core::Affine3f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_propPose_const(self.as_raw_Kinfu_Volume(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn raycast_step_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_Volume_propRaycastStepFactor_const(self.as_raw_Kinfu_Volume()) };
			ret
		}
		
		#[inline]
		fn raycast(&self, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_size: core::Size, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), &camera_pose, &intrinsics, &frame_size, points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn raycast_1(&self, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_size: core::Size, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray, colors: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			output_array_arg!(colors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), &camera_pose, &intrinsics, &frame_size, points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn fetch_normals(&self, points: &impl core::ToInputArray, _normals: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(_normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), points.as_raw__InputArray(), _normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn fetch_points_normals(&self, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_Volume(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn fetch_points_normals_colors(&self, unnamed: &mut impl core::ToOutputArray, unnamed_1: &mut impl core::ToOutputArray, unnamed_2: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn integrate(&mut self, _depth: &impl core::ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, frame_id: i32) -> Result<()> {
			input_array_arg!(_depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_int(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, frame_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [integrate] function uses the following default values for its arguments:
		/// * frame_id: 0
		#[inline]
		fn integrate_def(&mut self, _depth: &impl core::ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr) -> Result<()> {
			input_array_arg!(_depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * frame_id: 0
		#[inline]
		fn integrate_1(&mut self, _depth: &impl core::ToInputArray, _rgb: &impl core::ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, rgb_intrinsics: crate::rgbd::Kinfu_Intr, frame_id: i32) -> Result<()> {
			input_array_arg!(_depth);
			input_array_arg!(_rgb);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR_const_int(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), _rgb.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, &rgb_intrinsics, frame_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [integrate] function uses the following default values for its arguments:
		/// * frame_id: 0
		#[inline]
		fn integrate_def_1(&mut self, _depth: &impl core::ToInputArray, _rgb: &impl core::ToInputArray, depth_factor: f32, camera_pose: core::Matx44f, intrinsics: crate::rgbd::Kinfu_Intr, rgb_intrinsics: crate::rgbd::Kinfu_Intr) -> Result<()> {
			input_array_arg!(_depth);
			input_array_arg!(_rgb);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR(self.as_raw_mut_Kinfu_Volume(), _depth.as_raw__InputArray(), _rgb.as_raw__InputArray(), depth_factor, &camera_pose, &intrinsics, &rgb_intrinsics, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn reset(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Volume_reset(self.as_raw_mut_Kinfu_Volume(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Kinfu_Volume {
		ptr: *mut c_void
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
	pub trait Kinfu_VolumeParamsTraitConst {
		fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void;
	
		/// Type of Volume
		/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
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
		#[inline]
		fn unit_resolution(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propUnitResolution_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}
		
		/// Initial pose of the volume in meters
		#[inline]
		fn pose(&self) -> core::Affine3f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_VolumeParams_propPose_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Length of voxels in meters
		#[inline]
		fn voxel_size(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propVoxelSize_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}
		
		/// TSDF truncation distance
		/// Distances greater than value from surface will be truncated to 1.0
		#[inline]
		fn tsdf_trunc_dist(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propTsdfTruncDist_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}
		
		/// Max number of frames to integrate per voxel
		/// Represents the max number of frames over which a running average
		/// of the TSDF is calculated for a voxel
		#[inline]
		fn max_weight(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propMaxWeight_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// Truncates the depth greater than threshold to 0
		#[inline]
		fn depth_trunc_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propDepthTruncThreshold_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}
		
		/// Length of single raycast step
		/// Describes the percentage of voxel length that is skipped per march
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
		#[inline]
		fn set_type(&mut self, val: crate::rgbd::Kinfu_VolumeType) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propType_VolumeType(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}
		
		/// Resolution of voxel space
		/// Number of voxels in each dimension.
		/// Applicable only for TSDF Volume.
		/// HashTSDF volume only supports equal resolution in all three dimensions
		#[inline]
		fn set_resolution(&mut self, val: core::Vec3i) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolution_Vec3i(self.as_raw_mut_Kinfu_VolumeParams(), val.opencv_as_extern()) };
			ret
		}
		
		/// Resolution of volumeUnit in voxel space
		/// Number of voxels in each dimension for volumeUnit
		/// Applicable only for hashTSDF.
		/// 
		/// ## C++ default parameters
		/// * val: {0}
		#[inline]
		fn set_unit_resolution(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propUnitResolution_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}
		
		/// Initial pose of the volume in meters
		#[inline]
		fn set_pose(&mut self, val: core::Affine3f) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propPose_Affine3f(self.as_raw_mut_Kinfu_VolumeParams(), val.opencv_as_extern()) };
			ret
		}
		
		/// Length of voxels in meters
		#[inline]
		fn set_voxel_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propVoxelSize_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}
		
		/// TSDF truncation distance
		/// Distances greater than value from surface will be truncated to 1.0
		#[inline]
		fn set_tsdf_trunc_dist(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propTsdfTruncDist_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}
		
		/// Max number of frames to integrate per voxel
		/// Represents the max number of frames over which a running average
		/// of the TSDF is calculated for a voxel
		#[inline]
		fn set_max_weight(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propMaxWeight_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// Truncates the depth greater than threshold to 0
		#[inline]
		fn set_depth_trunc_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propDepthTruncThreshold_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}
		
		/// Length of single raycast step
		/// Describes the percentage of voxel length that is skipped per march
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propRaycastStepFactor_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}
		
	}
	
	pub struct Kinfu_VolumeParams {
		ptr: *mut c_void
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
	
	impl Kinfu_VolumeParams {
		/// Default set of parameters that provide higher quality reconstruction
		/// at the cost of slow performance.
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
	
	/// Constant methods for [crate::rgbd::Kinfu_Detail_PoseGraph]
	pub trait Kinfu_Detail_PoseGraphTraitConst {
		fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void;
	
		#[inline]
		fn is_node_exist(&self, node_id: size_t) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_isNodeExist_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_node_fixed(&self, node_id: size_t) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_isNodeFixed_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_node_pose(&self, node_id: size_t) -> Result<core::Affine3d> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_getNodePose_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), node_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_nodes_ids(&self) -> Result<core::Vector<size_t>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_getNodesIds_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_num_nodes(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_getNumNodes_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_edge_start(&self, i: size_t) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_getEdgeStart_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), i, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_edge_end(&self, i: size_t) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_getEdgeEnd_const_size_t(self.as_raw_Kinfu_Detail_PoseGraph(), i, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_num_edges(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_getNumEdges_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_valid(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_isValid_const(self.as_raw_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn add_node(&mut self, _node_id: size_t, _pose: core::Affine3d, fixed: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_addNode_size_t_const_Affine3dR_bool(self.as_raw_mut_Kinfu_Detail_PoseGraph(), _node_id, &_pose, fixed, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn add_edge(&mut self, _source_node_id: size_t, _target_node_id: size_t, _transformation: core::Affine3f, _information: core::Matx66f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR_const_Matx66fR(self.as_raw_mut_Kinfu_Detail_PoseGraph(), _source_node_id, _target_node_id, &_transformation, &_information, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [add_edge] function uses the following default values for its arguments:
		/// * _information: Matx66f::eye()
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
		#[inline]
		fn optimize(&mut self, tc: core::TermCriteria) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_optimize_const_TermCriteriaR(self.as_raw_mut_Kinfu_Detail_PoseGraph(), &tc, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [optimize] function uses the following default values for its arguments:
		/// * tc: cv::TermCriteria(TermCriteria::COUNT+TermCriteria::EPS,100,1e-6)
		#[inline]
		fn optimize_def(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_detail_PoseGraph_optimize(self.as_raw_mut_Kinfu_Detail_PoseGraph(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Kinfu_Detail_PoseGraph {
		ptr: *mut c_void
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
	
	impl Kinfu_Detail_PoseGraph {
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
	pub trait LargeKinfuTraitConst {
		fn as_raw_LargeKinfu(&self) -> *const c_void;
	
		#[inline]
		fn get_params(&self) -> Result<crate::rgbd::Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getParams_const(self.as_raw_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn render(&self, image: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn render_1(&self, image: &mut impl core::ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_cloud(&self, points: &mut impl core::ToOutputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_points(&self, points: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_normals(&self, points: &impl core::ToInputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn reset(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_reset(self.as_raw_mut_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn update(&mut self, depth: &impl core::ToInputArray) -> Result<bool> {
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
	/// which represent the distance to the closest surface (for details read the [kinectfusion](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_kinectfusion) article
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
	pub struct LargeKinfu {
		ptr: *mut c_void
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
	
	impl LargeKinfu {
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
	pub trait ParamsTraitConst {
		fn as_raw_Params(&self) -> *const c_void;
	
		/// frame size in pixels
		#[inline]
		fn frame_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_propFrameSize_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// camera intrinsics
		#[inline]
		fn intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_propIntr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// rgb camera intrinsics
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
		#[inline]
		fn depth_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propDepthFactor_const(self.as_raw_Params()) };
			ret
		}
		
		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn bilateral_sigma_depth(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_Params()) };
			ret
		}
		
		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn bilateral_sigma_spatial(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_Params()) };
			ret
		}
		
		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn bilateral_kernel_size(&self) -> i32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_Params()) };
			ret
		}
		
		/// Number of pyramid levels for ICP
		#[inline]
		fn pyramid_levels(&self) -> i32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propPyramidLevels_const(self.as_raw_Params()) };
			ret
		}
		
		/// Minimal camera movement in meters
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn tsdf_min_camera_movement(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_Params()) };
			ret
		}
		
		/// light pose for rendering in meters
		#[inline]
		fn light_pose(&self) -> core::Vec3f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_propLightPose_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// distance theshold for ICP in meters
		#[inline]
		fn icp_dist_thresh(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpDistThresh_const(self.as_raw_Params()) };
			ret
		}
		
		/// angle threshold for ICP in radians
		#[inline]
		fn icp_angle_thresh(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpAngleThresh_const(self.as_raw_Params()) };
			ret
		}
		
		/// number of ICP iterations for each pyramid level
		#[inline]
		fn icp_iterations(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpIterations_const(self.as_raw_Params()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// All depth values beyond this threshold will be set to zero
		#[inline]
		fn truncate_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_const(self.as_raw_Params()) };
			ret
		}
		
		/// Volume parameters
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
		#[inline]
		fn set_frame_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propFrameSize_Size(self.as_raw_mut_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// camera intrinsics
		#[inline]
		fn set_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIntr_Matx33f(self.as_raw_mut_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// rgb camera intrinsics
		#[inline]
		fn set_rgb_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propRgb_intr_Matx33f(self.as_raw_mut_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// pre-scale per 1 meter for input values
		/// Typical values are:
		///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
		///      * 1000 per 1 meter for Kinect 2 device
		///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
		#[inline]
		fn set_depth_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propDepthFactor_float(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_depth(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_depth_float(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_spatial(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_spatial_float(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_kernel_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_kernel_size_int(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// Number of pyramid levels for ICP
		#[inline]
		fn set_pyramid_levels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propPyramidLevels_int(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// Minimal camera movement in meters
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn set_tsdf_min_camera_movement(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propTsdf_min_camera_movement_float(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// light pose for rendering in meters
		#[inline]
		fn set_light_pose(&mut self, val: core::Vec3f) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propLightPose_Vec3f(self.as_raw_mut_Params(), val.opencv_as_extern()) };
			ret
		}
		
		/// distance theshold for ICP in meters
		#[inline]
		fn set_icp_dist_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpDistThresh_float(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// angle threshold for ICP in radians
		#[inline]
		fn set_icp_angle_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpAngleThresh_float(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// number of ICP iterations for each pyramid level
		#[inline]
		fn set_icp_iterations(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpIterations_vectorLintG(self.as_raw_mut_Params(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		/// Threshold for depth truncation in meters
		/// All depth values beyond this threshold will be set to zero
		#[inline]
		fn set_truncate_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_float(self.as_raw_mut_Params(), val) };
			ret
		}
		
		/// Volume parameters
		#[inline]
		fn set_volume_params(&mut self, mut val: crate::rgbd::Kinfu_VolumeParams) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propVolumeParams_VolumeParams(self.as_raw_mut_Params(), val.as_raw_mut_Kinfu_VolumeParams()) };
			ret
		}
		
	}
	
	pub struct Params {
		ptr: *mut c_void
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
	
	impl Params {
		/// Default parameters
		/// A set of parameters which provides better model quality, can be very slow.
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
	
	/// Constant methods for [crate::rgbd::LineMod_ColorGradient]
	pub trait LineMod_ColorGradientTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
		fn as_raw_LineMod_ColorGradient(&self) -> *const c_void;
	
		#[inline]
		fn weak_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const(self.as_raw_LineMod_ColorGradient()) };
			ret
		}
		
		#[inline]
		fn num_features(&self) -> size_t {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const(self.as_raw_LineMod_ColorGradient()) };
			ret
		}
		
		#[inline]
		fn strong_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const(self.as_raw_LineMod_ColorGradient()) };
			ret
		}
		
		#[inline]
		fn name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_ColorGradient_name_const(self.as_raw_LineMod_ColorGradient(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
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
	
		#[inline]
		fn set_weak_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
			ret
		}
		
		#[inline]
		fn set_num_features(&mut self, val: size_t) {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_size_t(self.as_raw_mut_LineMod_ColorGradient(), val) };
			ret
		}
		
		#[inline]
		fn set_strong_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
			ret
		}
		
		#[inline]
		fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_ColorGradient_read_const_FileNodeR(self.as_raw_mut_LineMod_ColorGradient(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// \brief Modality that computes quantized gradient orientations from a color image.
	pub struct LineMod_ColorGradient {
		ptr: *mut c_void
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
	
	impl crate::rgbd::LineMod_ColorGradientTraitConst for LineMod_ColorGradient {
		#[inline] fn as_raw_LineMod_ColorGradient(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::LineMod_ColorGradientTrait for LineMod_ColorGradient {
		#[inline] fn as_raw_mut_LineMod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LineMod_ColorGradient {
		/// \brief Default constructor. Uses reasonable default parameter values.
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
		#[inline]
		pub fn new(weak_threshold: f32, num_features: size_t, strong_threshold: f32) -> Result<crate::rgbd::LineMod_ColorGradient> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold, num_features, strong_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_ColorGradient::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	pub trait LineMod_DepthNormalTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
		fn as_raw_LineMod_DepthNormal(&self) -> *const c_void;
	
		#[inline]
		fn distance_threshold(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const(self.as_raw_LineMod_DepthNormal()) };
			ret
		}
		
		#[inline]
		fn difference_threshold(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const(self.as_raw_LineMod_DepthNormal()) };
			ret
		}
		
		#[inline]
		fn num_features(&self) -> size_t {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const(self.as_raw_LineMod_DepthNormal()) };
			ret
		}
		
		#[inline]
		fn extract_threshold(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const(self.as_raw_LineMod_DepthNormal()) };
			ret
		}
		
		#[inline]
		fn name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_DepthNormal_name_const(self.as_raw_LineMod_DepthNormal(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
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
	
		#[inline]
		fn set_distance_threshold(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}
		
		#[inline]
		fn set_difference_threshold(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}
		
		#[inline]
		fn set_num_features(&mut self, val: size_t) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_size_t(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}
		
		#[inline]
		fn set_extract_threshold(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}
		
		#[inline]
		fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_DepthNormal_read_const_FileNodeR(self.as_raw_mut_LineMod_DepthNormal(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// \brief Modality that computes quantized surface normals from a dense depth map.
	pub struct LineMod_DepthNormal {
		ptr: *mut c_void
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
	
	impl crate::rgbd::LineMod_DepthNormalTraitConst for LineMod_DepthNormal {
		#[inline] fn as_raw_LineMod_DepthNormal(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::LineMod_DepthNormalTrait for LineMod_DepthNormal {
		#[inline] fn as_raw_mut_LineMod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LineMod_DepthNormal {
		/// \brief Default constructor. Uses reasonable default parameter values.
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
		#[inline]
		pub fn new(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32) -> Result<crate::rgbd::LineMod_DepthNormal> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold, difference_threshold, num_features, extract_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_DepthNormal::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
		#[inline]
		fn match_(&self, sources: &core::Vector<core::Mat>, threshold: f32, matches: &mut core::Vector<crate::rgbd::LineMod_Match>, class_ids: &core::Vector<String>, quantized_images: &mut impl core::ToOutputArray, masks: &core::Vector<core::Mat>) -> Result<()> {
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
		/// This alternative version of [match_] function uses the following default values for its arguments:
		/// * class_ids: std::vector<String>()
		/// * quantized_images: noArray()
		/// * masks: std::vector<Mat>()
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
		#[inline]
		fn get_t(&self, pyramid_level: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_getT_const_int(self.as_raw_LineMod_Detector(), pyramid_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// \brief Get number of pyramid levels used by this detector.
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
		
		#[inline]
		fn num_templates(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_numTemplates_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn num_templates_1(&self, class_id: &str) -> Result<i32> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_numTemplates_const_const_StringR(self.as_raw_LineMod_Detector(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn num_classes(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_numClasses_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn class_ids(&self) -> Result<core::Vector<String>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_classIds_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_write_const_FileStorageR(self.as_raw_LineMod_Detector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn write_class(&self, class_id: &str, fs: &mut core::FileStorage) -> Result<()> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(self.as_raw_LineMod_Detector(), class_id.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * format: "templates_%s.yml.gz"
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
		/// This alternative version of [write_classes] function uses the following default values for its arguments:
		/// * format: "templates_%s.yml.gz"
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
		#[inline]
		fn add_template(&mut self, sources: &core::Vector<core::Mat>, class_id: &str, object_mask: &core::Mat, bounding_box: &mut core::Rect) -> Result<i32> {
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
		/// This alternative version of [add_template] function uses the following default values for its arguments:
		/// * bounding_box: NULL
		#[inline]
		fn add_template_def(&mut self, sources: &core::Vector<core::Mat>, class_id: &str, object_mask: &core::Mat) -> Result<i32> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(self.as_raw_mut_LineMod_Detector(), sources.as_raw_VectorOfMat(), class_id.opencv_as_extern(), object_mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// \brief Add a new object template computed by external means.
		#[inline]
		fn add_synthetic_template(&mut self, templates: &core::Vector<crate::rgbd::LineMod_Template>, class_id: &str) -> Result<i32> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(self.as_raw_mut_LineMod_Detector(), templates.as_raw_VectorOfLineMod_Template(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_read_const_FileNodeR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * class_id_override: ""
		#[inline]
		fn read_class(&mut self, fn_: &core::FileNode, class_id_override: &str) -> Result<String> {
			extern_container_arg!(class_id_override);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), class_id_override.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [read_class] function uses the following default values for its arguments:
		/// * class_id_override: ""
		#[inline]
		fn read_class_def(&mut self, fn_: &core::FileNode) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * format: "templates_%s.yml.gz"
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
		/// This alternative version of [read_classes] function uses the following default values for its arguments:
		/// * format: "templates_%s.yml.gz"
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
	pub struct LineMod_Detector {
		ptr: *mut c_void
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
	
	impl LineMod_Detector {
		/// \brief Empty constructor, initialize with read().
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
		#[inline]
		pub fn write(self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_write_const_FileStorageR(self.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::rgbd::LineMod_Feature> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_Feature(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn new(x: i32, y: i32, label: i32) -> Result<crate::rgbd::LineMod_Feature> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_Feature_int_int_int(x, y, label, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn read(self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_read_const_FileNodeR(self.opencv_as_extern(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::rgbd::LineMod_Match]
	pub trait LineMod_MatchTraitConst {
		fn as_raw_LineMod_Match(&self) -> *const c_void;
	
		#[inline]
		fn x(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_Match_propX_const(self.as_raw_LineMod_Match()) };
			ret
		}
		
		#[inline]
		fn y(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_Match_propY_const(self.as_raw_LineMod_Match()) };
			ret
		}
		
		#[inline]
		fn similarity(&self) -> f32 {
			let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const(self.as_raw_LineMod_Match()) };
			ret
		}
		
		#[inline]
		fn class_id(&self) -> String {
			let ret = unsafe { sys::cv_linemod_Match_propClass_id_const(self.as_raw_LineMod_Match()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn template_id(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const(self.as_raw_LineMod_Match()) };
			ret
		}
		
		/// Sort matches with high similarity to the front
		#[inline]
		fn less_than(&self, rhs: &crate::rgbd::LineMod_Match) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Match_operatorL_const_const_MatchR(self.as_raw_LineMod_Match(), rhs.as_raw_LineMod_Match(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn equals(&self, rhs: &crate::rgbd::LineMod_Match) -> Result<bool> {
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
	
		#[inline]
		fn set_x(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Match_propX_int(self.as_raw_mut_LineMod_Match(), val) };
			ret
		}
		
		#[inline]
		fn set_y(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Match_propY_int(self.as_raw_mut_LineMod_Match(), val) };
			ret
		}
		
		#[inline]
		fn set_similarity(&mut self, val: f32) {
			let ret = unsafe { sys::cv_linemod_Match_propSimilarity_float(self.as_raw_mut_LineMod_Match(), val) };
			ret
		}
		
		#[inline]
		fn set_class_id(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_linemod_Match_propClass_id_String(self.as_raw_mut_LineMod_Match(), val.opencv_as_extern_mut()) };
			ret
		}
		
		#[inline]
		fn set_template_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_int(self.as_raw_mut_LineMod_Match(), val) };
			ret
		}
		
	}
	
	/// \brief Represents a successful template match.
	pub struct LineMod_Match {
		ptr: *mut c_void
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
	
	impl LineMod_Match {
		#[inline]
		pub fn default() -> Result<crate::rgbd::LineMod_Match> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Match_Match(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_Match::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
		#[inline]
		fn process(&self, src: &core::Mat, mask: &core::Mat) -> Result<core::Ptr<crate::rgbd::LineMod_QuantizedPyramid>> {
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
		/// This alternative version of [process] function uses the following default values for its arguments:
		/// * mask: Mat()
		#[inline]
		fn process_def(&self, src: &core::Mat) -> Result<core::Ptr<crate::rgbd::LineMod_QuantizedPyramid>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_process_const_const_MatR(self.as_raw_LineMod_Modality(), src.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_QuantizedPyramid>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_name_const(self.as_raw_LineMod_Modality(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
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
	
		#[inline]
		fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
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
	pub struct LineMod_Modality {
		ptr: *mut c_void
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
	
	impl LineMod_Modality {
		/// \brief Create modality by name.
		/// 
		/// The following modality types are supported:
		/// - "ColorGradient"
		/// - "DepthNormal"
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
		#[inline]
		pub fn create_1(fn_: &core::FileNode) -> Result<core::Ptr<crate::rgbd::LineMod_Modality>> {
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
	pub trait LineMod_QuantizedPyramidTraitConst {
		fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void;
	
		/// \brief Compute quantized image at current pyramid level for online detection.
		/// 
		/// \param[out] dst The destination 8-bit image. For each pixel at most one bit is set,
		///                representing its classification.
		#[inline]
		fn quantize(&self, dst: &mut core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_QuantizedPyramid_quantize_const_MatR(self.as_raw_LineMod_QuantizedPyramid(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// \brief Extract most discriminant features at current pyramid level to form a new template.
		/// 
		/// \param[out] templ The new template.
		#[inline]
		fn extract_template(&self, templ: &mut crate::rgbd::LineMod_Template) -> Result<bool> {
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
	pub struct LineMod_QuantizedPyramid {
		ptr: *mut c_void
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
	pub trait LineMod_TemplateTraitConst {
		fn as_raw_LineMod_Template(&self) -> *const c_void;
	
		#[inline]
		fn width(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_Template_propWidth_const(self.as_raw_LineMod_Template()) };
			ret
		}
		
		#[inline]
		fn height(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_Template_propHeight_const(self.as_raw_LineMod_Template()) };
			ret
		}
		
		#[inline]
		fn pyramid_level(&self) -> i32 {
			let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const(self.as_raw_LineMod_Template()) };
			ret
		}
		
		#[inline]
		fn features(&self) -> core::Vector<crate::rgbd::LineMod_Feature> {
			let ret = unsafe { sys::cv_linemod_Template_propFeatures_const(self.as_raw_LineMod_Template()) };
			let ret = unsafe { core::Vector::<crate::rgbd::LineMod_Feature>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
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
	
		#[inline]
		fn set_width(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Template_propWidth_int(self.as_raw_mut_LineMod_Template(), val) };
			ret
		}
		
		#[inline]
		fn set_height(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Template_propHeight_int(self.as_raw_mut_LineMod_Template(), val) };
			ret
		}
		
		#[inline]
		fn set_pyramid_level(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_int(self.as_raw_mut_LineMod_Template(), val) };
			ret
		}
		
		#[inline]
		fn set_features(&mut self, mut val: core::Vector<crate::rgbd::LineMod_Feature>) {
			let ret = unsafe { sys::cv_linemod_Template_propFeatures_vectorLFeatureG(self.as_raw_mut_LineMod_Template(), val.as_raw_mut_VectorOfLineMod_Feature()) };
			ret
		}
		
		#[inline]
		fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Template_read_const_FileNodeR(self.as_raw_mut_LineMod_Template(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct LineMod_Template {
		ptr: *mut c_void
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
	
	impl LineMod_Template {
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
	pub trait DepthCleanerTraitConst: core::AlgorithmTraitConst {
		fn as_raw_DepthCleaner(&self) -> *const c_void;
	
		/// Given a set of 3d points in a depth image, compute the normals at each point.
		/// ## Parameters
		/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
		/// * depth: a rows x cols matrix of the cleaned up depth
		#[inline]
		fn apply(&self, points: &impl core::ToInputArray, depth: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn initialize(&self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_DepthCleaner_initialize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_window_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_DepthCleaner_getWindowSize_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_depth(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_DepthCleaner_getDepth_const(self.as_raw_DepthCleaner(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_window_size(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_DepthCleaner_setWindowSize_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_depth(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_DepthCleaner_setDepth_int(self.as_raw_mut_DepthCleaner(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct DepthCleaner {
		ptr: *mut c_void
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
	
	impl crate::rgbd::DepthCleanerTraitConst for DepthCleaner {
		#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::DepthCleanerTrait for DepthCleaner {
		#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DepthCleaner {
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * window_size: 5
		/// * method: DepthCleaner::DEPTH_CLEANER_NIL
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
	pub trait FastICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
		fn as_raw_FastICPOdometry(&self) -> *const c_void;
	
		#[inline]
		fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_FastICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_camera_matrix(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_getCameraMatrix_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_max_dist_diff(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_getMaxDistDiff_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_angle_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_getAngleThreshold_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_sigma_depth(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_getSigmaDepth_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_sigma_spatial(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_getSigmaSpatial_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_kernel_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_getKernelSize_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_iteration_counts(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_getIterationCounts_const(self.as_raw_FastICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_FastICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_dist_diff(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_setMaxDistDiff_float(self.as_raw_mut_FastICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_angle_threshold(&mut self, f: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_setAngleThreshold_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_sigma_depth(&mut self, f: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_setSigmaDepth_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_sigma_spatial(&mut self, f: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_setSigmaSpatial_float(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_kernel_size(&mut self, f: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_setKernelSize_int(self.as_raw_mut_FastICPOdometry(), f, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_FastICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct FastICPOdometry {
		ptr: *mut c_void
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
	
	impl crate::rgbd::OdometryTraitConst for FastICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for FastICPOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::FastICPOdometryTraitConst for FastICPOdometry {
		#[inline] fn as_raw_FastICPOdometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::FastICPOdometryTrait for FastICPOdometry {
		#[inline] fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FastICPOdometry {
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
		#[inline]
		pub fn new(camera_matrix: &core::Mat, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: &core::Vector<i32>) -> Result<crate::rgbd::FastICPOdometry> {
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
		#[inline]
		pub fn new_def(camera_matrix: &core::Mat) -> Result<crate::rgbd::FastICPOdometry> {
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
		#[inline]
		pub fn create(camera_matrix: &core::Mat, max_dist_diff: f32, angle_threshold: f32, sigma_depth: f32, sigma_spatial: f32, kernel_size: i32, iter_counts: &core::Vector<i32>) -> Result<core::Ptr<crate::rgbd::FastICPOdometry>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_FastICPOdometry_create_const_MatR_float_float_float_float_int_const_vectorLintGR(camera_matrix.as_raw_Mat(), max_dist_diff, angle_threshold, sigma_depth, sigma_spatial, kernel_size, iter_counts.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::FastICPOdometry>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * max_dist_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
		/// * angle_threshold: (float)(30.*CV_PI/180.)
		/// * sigma_depth: 0.04f
		/// * sigma_spatial: 4.5f
		/// * kernel_size: 7
		/// * iter_counts: std::vector<int>()
		#[inline]
		pub fn create_def(camera_matrix: &core::Mat) -> Result<core::Ptr<crate::rgbd::FastICPOdometry>> {
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
	pub trait ICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
		fn as_raw_ICPOdometry(&self) -> *const c_void;
	
		#[inline]
		fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_ICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_camera_matrix(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getCameraMatrix_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_min_depth(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getMinDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_depth(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepth_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_depth_diff(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getMaxDepthDiff_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_iteration_counts(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getIterationCounts_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_max_points_part(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getMaxPointsPart_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_transform_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getTransformType_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_translation(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getMaxTranslation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_rotation(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_getMaxRotation_const(self.as_raw_ICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_depth(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setMinDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_depth(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepth_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_ICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_points_part(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setMaxPointsPart_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_transform_type(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setTransformType_int(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_translation(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_setMaxTranslation_double(self.as_raw_mut_ICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct ICPOdometry {
		ptr: *mut c_void
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
	
	impl crate::rgbd::OdometryTraitConst for ICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for ICPOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ICPOdometryTraitConst for ICPOdometry {
		#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::ICPOdometryTrait for ICPOdometry {
		#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ICPOdometry {
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
		#[inline]
		pub fn new(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, transform_type: i32) -> Result<crate::rgbd::ICPOdometry> {
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
		#[inline]
		pub fn new_def(camera_matrix: &core::Mat) -> Result<crate::rgbd::ICPOdometry> {
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
		#[inline]
		pub fn create(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, transform_type: i32) -> Result<core::Ptr<crate::rgbd::ICPOdometry>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), transform_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::ICPOdometry>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * camera_matrix: Mat()
		/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
		/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
		/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
		/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
		/// * iter_counts: std::vector<int>()
		/// * transform_type: Odometry::RIGID_BODY_MOTION
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
		#[inline]
		fn compute(&self, src_image: &core::Mat, src_depth: &core::Mat, src_mask: &core::Mat, dst_image: &core::Mat, dst_depth: &core::Mat, dst_mask: &core::Mat, rt: &mut impl core::ToOutputArray, init_rt: &core::Mat) -> Result<bool> {
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
		/// This alternative version of [compute] function uses the following default values for its arguments:
		/// * init_rt: Mat()
		#[inline]
		fn compute_def(&self, src_image: &core::Mat, src_depth: &core::Mat, src_mask: &core::Mat, dst_image: &core::Mat, dst_depth: &core::Mat, dst_mask: &core::Mat, rt: &mut impl core::ToOutputArray) -> Result<bool> {
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
		#[inline]
		fn compute2(&self, src_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, dst_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, rt: &mut impl core::ToOutputArray, init_rt: &core::Mat) -> Result<bool> {
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
		/// This alternative version of [compute2] function uses the following default values for its arguments:
		/// * init_rt: Mat()
		#[inline]
		fn compute2_def(&self, src_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, dst_frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, rt: &mut impl core::ToOutputArray) -> Result<bool> {
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
		#[inline]
		fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_Odometry_setCameraMatrix_const_MatR(self.as_raw_mut_Odometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setTransformType getTransformType
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
	pub struct Odometry {
		ptr: *mut c_void
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
	
	impl crate::rgbd::OdometryTraitConst for Odometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for Odometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Odometry {
		#[inline]
		pub fn default_min_depth() -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default_max_depth() -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default_max_depth_diff() -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default_max_points_part() -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default_max_translation() -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default_max_rotation() -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub trait OdometryFrameTraitConst: crate::rgbd::RgbdFrameTraitConst {
		fn as_raw_OdometryFrame(&self) -> *const c_void;
	
		#[inline]
		fn pyramid_image(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidImage_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pyramid_depth(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidDepth_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pyramid_mask(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidMask_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pyramid_cloud(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidCloud_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pyramid_d_i_dx(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dx_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pyramid_d_i_dy(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dy_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pyramid_textured_mask(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidTexturedMask_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pyramid_normals(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormals_const(self.as_raw_OdometryFrame()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
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
	
		#[inline]
		fn set_pyramid_image(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidImage_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_depth(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidDepth_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_mask(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidMask_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_cloud(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidCloud_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_d_i_dx(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dx_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_d_i_dy(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramid_dI_dy_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_textured_mask(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidTexturedMask_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_normals(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormals_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn set_pyramid_normals_mask(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_rgbd_OdometryFrame_propPyramidNormalsMask_vectorLMatG(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		#[inline]
		fn release(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_OdometryFrame_release(self.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct OdometryFrame {
		ptr: *mut c_void
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
	
	impl crate::rgbd::OdometryFrameTraitConst for OdometryFrame {
		#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::OdometryFrameTrait for OdometryFrame {
		#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl OdometryFrame {
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
		#[inline]
		pub fn new(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<crate::rgbd::OdometryFrame> {
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
		#[inline]
		pub fn new_def(image: &core::Mat, depth: &core::Mat) -> Result<crate::rgbd::OdometryFrame> {
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
		#[inline]
		pub fn create(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<core::Ptr<crate::rgbd::OdometryFrame>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::OdometryFrame>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * image: Mat()
		/// * depth: Mat()
		/// * mask: Mat()
		/// * normals: Mat()
		/// * id: -1
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
	pub trait RgbdFrameTraitConst {
		fn as_raw_RgbdFrame(&self) -> *const c_void;
	
		#[inline]
		fn id(&self) -> i32 {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propID_const(self.as_raw_RgbdFrame()) };
			ret
		}
		
		#[inline]
		fn image(&self) -> core::Mat {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propImage_const(self.as_raw_RgbdFrame()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn depth(&self) -> core::Mat {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propDepth_const(self.as_raw_RgbdFrame()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn mask(&self) -> core::Mat {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propMask_const(self.as_raw_RgbdFrame()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
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
	
		#[inline]
		fn set_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propID_int(self.as_raw_mut_RgbdFrame(), val) };
			ret
		}
		
		#[inline]
		fn set_image(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propImage_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_depth(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propDepth_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_mask(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propMask_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_normals(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_rgbd_RgbdFrame_propNormals_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) };
			ret
		}
		
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
	pub struct RgbdFrame {
		ptr: *mut c_void
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
	
	impl RgbdFrame {
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
		#[inline]
		pub fn new(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<crate::rgbd::RgbdFrame> {
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
		#[inline]
		pub fn new_def(image: &core::Mat, depth: &core::Mat) -> Result<crate::rgbd::RgbdFrame> {
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
		#[inline]
		pub fn create(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<core::Ptr<crate::rgbd::RgbdFrame>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::RgbdFrame>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * image: Mat()
		/// * depth: Mat()
		/// * mask: Mat()
		/// * normals: Mat()
		/// * id: -1
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
	pub trait RgbdICPOdometryTraitConst: crate::rgbd::OdometryTraitConst {
		fn as_raw_RgbdICPOdometry(&self) -> *const c_void;
	
		#[inline]
		fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_RgbdICPOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_camera_matrix(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_min_depth(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getMinDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_depth(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepth_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_depth_diff(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_points_part(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_iteration_counts(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getIterationCounts_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_min_gradient_magnitudes(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_transform_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getTransformType_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_translation(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_rotation(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_getMaxRotation_const(self.as_raw_RgbdICPOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_depth(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_depth(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepth_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_points_part(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_gradient_magnitudes(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdICPOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_transform_type(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setTransformType_int(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_translation(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdICPOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct RgbdICPOdometry {
		ptr: *mut c_void
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
	
	impl crate::rgbd::OdometryTraitConst for RgbdICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for RgbdICPOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdICPOdometryTraitConst for RgbdICPOdometry {
		#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::RgbdICPOdometryTrait for RgbdICPOdometry {
		#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RgbdICPOdometry {
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
		#[inline]
		pub fn new(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, transform_type: i32) -> Result<crate::rgbd::RgbdICPOdometry> {
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
		#[inline]
		pub fn new_def(camera_matrix: &core::Mat) -> Result<crate::rgbd::RgbdICPOdometry> {
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
		#[inline]
		pub fn create(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, max_points_part: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, transform_type: i32) -> Result<core::Ptr<crate::rgbd::RgbdICPOdometry>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, max_points_part, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), transform_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::RgbdICPOdometry>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * camera_matrix: Mat()
		/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
		/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
		/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
		/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
		/// * iter_counts: std::vector<int>()
		/// * min_gradient_magnitudes: std::vector<float>()
		/// * transform_type: Odometry::RIGID_BODY_MOTION
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
	pub trait RgbdNormalsTraitConst: core::AlgorithmTraitConst {
		fn as_raw_RgbdNormals(&self) -> *const c_void;
	
		/// Given a set of 3d points in a depth image, compute the normals at each point.
		/// ## Parameters
		/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
		/// * normals: a rows x cols x 3 matrix
		#[inline]
		fn apply(&self, points: &impl core::ToInputArray, normals: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn initialize(&self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_initialize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_rows(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_getRows_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_cols(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_getCols_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_window_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_getWindowSize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_depth(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_getDepth_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_k(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_getK_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_rows(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_setRows_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_cols(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_setCols_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_window_size(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_setWindowSize_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_depth(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_setDepth_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_k(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_setK_const_MatR(self.as_raw_mut_RgbdNormals(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct RgbdNormals {
		ptr: *mut c_void
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
	
	impl crate::rgbd::RgbdNormalsTraitConst for RgbdNormals {
		#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::RgbdNormalsTrait for RgbdNormals {
		#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RgbdNormals {
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
		#[inline]
		pub fn new(rows: i32, cols: i32, depth: i32, k: &impl core::ToInputArray, window_size: i32, method: i32) -> Result<crate::rgbd::RgbdNormals> {
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
		#[inline]
		pub fn new_def(rows: i32, cols: i32, depth: i32, k: &impl core::ToInputArray) -> Result<crate::rgbd::RgbdNormals> {
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
		#[inline]
		pub fn create(rows: i32, cols: i32, depth: i32, k: &impl core::ToInputArray, window_size: i32, method: i32) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
			input_array_arg!(k);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(rows, cols, depth, k.as_raw__InputArray(), window_size, method, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::RgbdNormals>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * window_size: 5
		/// * method: RgbdNormals::RGBD_NORMALS_METHOD_FALS
		#[inline]
		pub fn create_def(rows: i32, cols: i32, depth: i32, k: &impl core::ToInputArray) -> Result<core::Ptr<crate::rgbd::RgbdNormals>> {
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
	pub trait RgbdOdometryTraitConst: crate::rgbd::OdometryTraitConst {
		fn as_raw_RgbdOdometry(&self) -> *const c_void;
	
		#[inline]
		fn prepare_frame_cache(&self, frame: &mut core::Ptr<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(self.as_raw_RgbdOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_camera_matrix(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getCameraMatrix_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_min_depth(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getMinDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_depth(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepth_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_depth_diff(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_iteration_counts(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getIterationCounts_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_min_gradient_magnitudes(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_max_points_part(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getMaxPointsPart_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_transform_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getTransformType_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_translation(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_getMaxTranslation_const(self.as_raw_RgbdOdometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	
		#[inline]
		fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_depth(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setMinDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_depth(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepth_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_gradient_magnitudes(&mut self, val: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_points_part(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_transform_type(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setTransformType_int(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_translation(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdOdometry(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct RgbdOdometry {
		ptr: *mut c_void
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
	
	impl crate::rgbd::OdometryTraitConst for RgbdOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for RgbdOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdOdometryTraitConst for RgbdOdometry {
		#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::RgbdOdometryTrait for RgbdOdometry {
		#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RgbdOdometry {
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
		#[inline]
		pub fn new(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, max_points_part: f32, transform_type: i32) -> Result<crate::rgbd::RgbdOdometry> {
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
		#[inline]
		pub fn new_def(camera_matrix: &core::Mat) -> Result<crate::rgbd::RgbdOdometry> {
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
		#[inline]
		pub fn create(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: &core::Vector<i32>, min_gradient_magnitudes: &core::Vector<f32>, max_points_part: f32, transform_type: i32) -> Result<core::Ptr<crate::rgbd::RgbdOdometry>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), max_points_part, transform_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::RgbdOdometry>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * camera_matrix: Mat()
		/// * min_depth: Odometry::DEFAULT_MIN_DEPTH()
		/// * max_depth: Odometry::DEFAULT_MAX_DEPTH()
		/// * max_depth_diff: Odometry::DEFAULT_MAX_DEPTH_DIFF()
		/// * iter_counts: std::vector<int>()
		/// * min_gradient_magnitudes: std::vector<float>()
		/// * max_points_part: Odometry::DEFAULT_MAX_POINTS_PART()
		/// * transform_type: Odometry::RIGID_BODY_MOTION
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
	pub trait RgbdPlaneTraitConst: core::AlgorithmTraitConst {
		fn as_raw_RgbdPlane(&self) -> *const c_void;
	
		#[inline]
		fn get_block_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_getBlockSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_getMinSize_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_method(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_getMethod_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_getThreshold_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_sensor_error_a(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorA_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_sensor_error_b(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorB_const(self.as_raw_RgbdPlane(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn apply(&mut self, points3d: &impl core::ToInputArray, normals: &impl core::ToInputArray, mask: &mut impl core::ToOutputArray, plane_coefficients: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn apply_1(&mut self, points3d: &impl core::ToInputArray, mask: &mut impl core::ToOutputArray, plane_coefficients: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(points3d);
			output_array_arg!(mask);
			output_array_arg!(plane_coefficients);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_RgbdPlane(), points3d.as_raw__InputArray(), mask.as_raw__OutputArray(), plane_coefficients.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_block_size(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_setBlockSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_size(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_setMinSize_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_method(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_setMethod_int(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_threshold(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_setThreshold_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_sensor_error_a(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorA_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_sensor_error_b(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorB_double(self.as_raw_mut_RgbdPlane(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct RgbdPlane {
		ptr: *mut c_void
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
	
	impl crate::rgbd::RgbdPlaneTraitConst for RgbdPlane {
		#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rgbd::RgbdPlaneTrait for RgbdPlane {
		#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RgbdPlane {
		/// ## C++ default parameters
		/// * method: RgbdPlane::RGBD_PLANE_METHOD_DEFAULT
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
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * sensor_error_a: 0
		/// * sensor_error_b: 0
		/// * sensor_error_c: 0
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
}
