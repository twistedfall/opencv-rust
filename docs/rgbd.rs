pub mod rgbd {
	//! # RGB-Depth Processing
	//!
	//! [kinfu_icp]
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{ColoredKinfu_ColoredKinFuTrait, ColoredKinfu_ColoredKinFuTraitConst, ColoredKinfu_ParamsTrait, ColoredKinfu_ParamsTraitConst, Dynafu_DynaFuTrait, Dynafu_DynaFuTraitConst, Kinfu_KinFuTrait, Kinfu_KinFuTraitConst, Kinfu_ParamsTrait, Kinfu_ParamsTraitConst, Kinfu_VolumeParamsTrait, Kinfu_VolumeParamsTraitConst, LargeKinfuTrait, LargeKinfuTraitConst, LineMod_ColorGradientTrait, LineMod_ColorGradientTraitConst, LineMod_DepthNormalTrait, LineMod_DepthNormalTraitConst, LineMod_DetectorTrait, LineMod_DetectorTraitConst, LineMod_MatchTrait, LineMod_MatchTraitConst, LineMod_ModalityTrait, LineMod_ModalityTraitConst, LineMod_QuantizedPyramidTrait, LineMod_QuantizedPyramidTraitConst, LineMod_TemplateTrait, LineMod_TemplateTraitConst, ParamsTrait, ParamsTraitConst, VolumeParamsTrait, VolumeParamsTraitConst};
	}

	/// Backwards compatibility for old versions
	pub type Dynafu_Params = crate::rgbd::Kinfu_Params;
	/// \brief Debug function to colormap a quantized image for viewing.
	#[inline]
	pub fn colormap(quantized: &impl core::MatTraitConst, dst: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_colormap_const_MatR_MatR(quantized.as_raw_Mat(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	pub fn draw_features_def(img: &mut impl ToInputOutputArray, templates: &core::Vector<crate::rgbd::LineMod_Template>, tl: core::Point2i) -> Result<()> {
		input_output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR(img.as_raw__InputOutputArray(), templates.as_raw_VectorOfLineMod_Template(), &tl, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
	pub fn draw_features(img: &mut impl ToInputOutputArray, templates: &core::Vector<crate::rgbd::LineMod_Template>, tl: core::Point2i, size: i32) -> Result<()> {
		input_output_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR_int(img.as_raw__InputOutputArray(), templates.as_raw_VectorOfLineMod_Template(), &tl, size, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
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
		return_receive!(ocvrs_return => ret);
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
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_Detector>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// KinectFusion implementation
	///
	/// This class implements a 3d reconstruction algorithm described in
	/// [kinectfusion](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_kinectfusion) paper.
	///
	/// It takes a sequence of depth images taken from depth sensor
	/// (or any depth images source such as stereo camera matching algorithm or even raymarching renderer).
	/// The output can be obtained as a vector of points and their normals
	/// or can be Phong-rendered from given camera pose.
	///
	/// An internal representation of a model is a voxel cuboid that keeps TSDF values
	/// which are a sort of distances to the surface (for details read the [kinectfusion](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_kinectfusion) article about TSDF).
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

	impl ColoredKinfu_ColoredKinFu {
		#[inline]
		pub fn create(_params: &core::Ptr<crate::rgbd::ColoredKinfu_Params>) -> Result<core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_create_const_PtrLParamsGR(_params.as_raw_PtrOfColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_ColoredKinFu>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::rgbd::ColoredKinfu_ColoredKinFu]
	pub trait ColoredKinfu_ColoredKinFuTraitConst {
		fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void;

		/// Get current parameters
		#[inline]
		fn get_params(&self) -> Result<crate::rgbd::ColoredKinfu_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_getParams_const(self.as_raw_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn render(&self, image: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn render_1(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_ColoredKinfu_ColoredKinFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Calculates normals for given points
		/// ## Parameters
		/// * points: input vector of points which are 4-float vectors
		/// * normals: output vector of corresponding normals which are 4-float vectors
		#[inline]
		fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_ColoredKinfu_ColoredKinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get current pose in voxel space
		#[inline]
		fn get_pose(&self) -> Result<core::Affine3f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_getPose_const(self.as_raw_ColoredKinfu_ColoredKinFu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
		fn update(&mut self, depth: &impl ToInputArray, rgb: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(depth);
			input_array_arg!(rgb);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ColoredKinfu_ColoredKinFu(), depth.as_raw__InputArray(), rgb.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
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

	impl crate::rgbd::ColoredKinfu_ColoredKinFuTraitConst for ColoredKinfu_ColoredKinFu {
		#[inline] fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::ColoredKinfu_ColoredKinFuTrait for ColoredKinfu_ColoredKinFu {
		#[inline] fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ColoredKinfu_ColoredKinFu, crate::rgbd::ColoredKinfu_ColoredKinFuTraitConst, as_raw_ColoredKinfu_ColoredKinFu, crate::rgbd::ColoredKinfu_ColoredKinFuTrait, as_raw_mut_ColoredKinfu_ColoredKinFu }

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

	impl ColoredKinfu_Params {
		#[inline]
		pub fn default() -> Result<crate::rgbd::ColoredKinfu_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			unsafe { sys::cv_colored_kinfu_Params_Params_Matx33f_Vec3f(&volume_initial_pose_rot, &volume_initial_pose_transl, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			unsafe { sys::cv_colored_kinfu_Params_Params_Matx44f(&volume_initial_pose, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::ColoredKinfu_Params>::opencv_from_extern(ret) };
			Ok(ret)
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
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// rgb frame size in pixels
		#[inline]
		fn rgb_frame_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		#[inline]
		fn volume_kind(&self) -> crate::ptcloud::VolumeType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propVolumeKind_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// camera intrinsics
		#[inline]
		fn intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propIntr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// rgb camera intrinsics
		#[inline]
		fn rgb_intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propRgb_intr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
		fn volume_pose(&self) -> core::Matx44f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_propVolumePose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			let ret = unsafe { sys::cv_colored_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_ColoredKinfu_Params(), &val) };
			ret
		}

		/// rgb frame size in pixels
		#[inline]
		fn set_rgb_frame_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_const_Size(self.as_raw_mut_ColoredKinfu_Params(), &val) };
			ret
		}

		#[inline]
		fn set_volume_kind(&mut self, val: crate::ptcloud::VolumeType) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeKind_const_VolumeType(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// camera intrinsics
		#[inline]
		fn set_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
			ret
		}

		/// rgb camera intrinsics
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
		#[inline]
		fn set_depth_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_depth(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_spatial(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_kernel_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// Number of pyramid levels for ICP
		#[inline]
		fn set_pyramid_levels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// Resolution of voxel space
		///
		/// Number of voxels in each dimension.
		#[inline]
		fn set_volume_dims(&mut self, val: core::Vec3i) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeDims_const_Vec3i(self.as_raw_mut_ColoredKinfu_Params(), &val) };
			ret
		}

		/// Size of voxel in meters
		#[inline]
		fn set_voxel_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVoxelSize_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// Minimal camera movement in meters
		///
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn set_tsdf_min_camera_movement(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// initial volume pose in meters
		#[inline]
		fn set_volume_pose(&mut self, val: core::Matx44f) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumePose_const_Matx44f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
			ret
		}

		/// distance to truncate in meters
		///
		/// Distances to surface that exceed this value will be truncated to 1.0.
		#[inline]
		fn set_tsdf_trunc_dist(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_trunc_dist_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// max number of frames per voxel
		///
		/// Each voxel keeps running average of distances no longer than this value.
		#[inline]
		fn set_tsdf_max_weight(&mut self, val: i32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_max_weight_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// A length of one raycast step
		///
		/// How much voxel sizes we skip each raycast step
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propRaycast_step_factor_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// light pose for rendering in meters
		#[inline]
		fn set_light_pose(&mut self, val: core::Vec3f) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
			ret
		}

		/// distance theshold for ICP in meters
		#[inline]
		fn set_icp_dist_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// angle threshold for ICP in radians
		#[inline]
		fn set_icp_angle_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
			ret
		}

		/// number of ICP iterations for each pyramid level
		#[inline]
		fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_ColoredKinfu_Params(), val.as_raw_VectorOfi32()) };
			ret
		}

		/// Threshold for depth truncation in meters
		///
		/// All depth values beyond this threshold will be set to zero
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
		#[inline]
		fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), &r, &t, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			unsafe { sys::cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_ColoredKinfu_Params(), &homogen_tf, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for ColoredKinfu_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ColoredKinfu_Params")
				.field("frame_size", &crate::rgbd::ColoredKinfu_ParamsTraitConst::frame_size(self))
				.field("rgb_frame_size", &crate::rgbd::ColoredKinfu_ParamsTraitConst::rgb_frame_size(self))
				.field("volume_kind", &crate::rgbd::ColoredKinfu_ParamsTraitConst::volume_kind(self))
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

	impl crate::rgbd::ColoredKinfu_ParamsTraitConst for ColoredKinfu_Params {
		#[inline] fn as_raw_ColoredKinfu_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::ColoredKinfu_ParamsTrait for ColoredKinfu_Params {
		#[inline] fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { ColoredKinfu_Params, crate::rgbd::ColoredKinfu_ParamsTraitConst, as_raw_ColoredKinfu_Params, crate::rgbd::ColoredKinfu_ParamsTrait, as_raw_mut_ColoredKinfu_Params }

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

	impl Dynafu_DynaFu {
		#[inline]
		pub fn create(_params: &core::Ptr<crate::rgbd::Kinfu_Params>) -> Result<core::Ptr<crate::rgbd::Dynafu_DynaFu>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_create_const_PtrLParamsGR(_params.as_raw_PtrOfKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::Dynafu_DynaFu>::opencv_from_extern(ret) };
			Ok(ret)
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
			return_receive!(ocvrs_return => ret);
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
		fn render(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_Dynafu_DynaFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn render_def(&self, image: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_render_const_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Calculates normals for given points
		/// ## Parameters
		/// * points: input vector of points which are 4-float vectors
		/// * normals: output vector of corresponding normals which are 4-float vectors
		#[inline]
		fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get current pose in voxel space
		#[inline]
		fn get_pose(&self) -> Result<core::Affine3f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getPose_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_nodes_pos(&self) -> Result<core::Vector<core::Point3f>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_getNodesPos_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point3f>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn march_cubes(&self, vertices: &mut impl ToOutputArray, edges: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(vertices);
			output_array_arg!(edges);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Dynafu_DynaFu(), vertices.as_raw__OutputArray(), edges.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
		fn update(&mut self, depth: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_update_const__InputArrayR(self.as_raw_mut_Dynafu_DynaFu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## C++ default parameters
		/// * warp: true
		#[inline]
		fn render_surface(&mut self, depth_image: &mut impl ToOutputArray, vert_image: &mut impl ToOutputArray, norm_image: &mut impl ToOutputArray, warp: bool) -> Result<()> {
			output_array_arg!(depth_image);
			output_array_arg!(vert_image);
			output_array_arg!(norm_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(self.as_raw_mut_Dynafu_DynaFu(), depth_image.as_raw__OutputArray(), vert_image.as_raw__OutputArray(), norm_image.as_raw__OutputArray(), warp, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [Dynafu_DynaFuTrait::render_surface] function uses the following default values for its arguments:
		/// * warp: true
		#[inline]
		fn render_surface_def(&mut self, depth_image: &mut impl ToOutputArray, vert_image: &mut impl ToOutputArray, norm_image: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(depth_image);
			output_array_arg!(vert_image);
			output_array_arg!(norm_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Dynafu_DynaFu(), depth_image.as_raw__OutputArray(), vert_image.as_raw__OutputArray(), norm_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
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

	impl crate::rgbd::Dynafu_DynaFuTraitConst for Dynafu_DynaFu {
		#[inline] fn as_raw_Dynafu_DynaFu(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::Dynafu_DynaFuTrait for Dynafu_DynaFu {
		#[inline] fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Dynafu_DynaFu, crate::rgbd::Dynafu_DynaFuTraitConst, as_raw_Dynafu_DynaFu, crate::rgbd::Dynafu_DynaFuTrait, as_raw_mut_Dynafu_DynaFu }

	/// KinectFusion implementation
	///
	/// This class implements a 3d reconstruction algorithm described in
	/// [kinectfusion](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_kinectfusion) paper.
	///
	/// It takes a sequence of depth images taken from depth sensor
	/// (or any depth images source such as stereo camera matching algorithm or even raymarching renderer).
	/// The output can be obtained as a vector of points and their normals
	/// or can be Phong-rendered from given camera pose.
	///
	/// An internal representation of a model is a voxel cuboid that keeps TSDF values
	/// which are a sort of distances to the surface (for details read the [kinectfusion](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_kinectfusion) article about TSDF).
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

	impl Kinfu_KinFu {
		#[inline]
		pub fn create(_params: &core::Ptr<crate::rgbd::Kinfu_Params>) -> Result<core::Ptr<crate::rgbd::Kinfu_KinFu>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_create_const_PtrLParamsGR(_params.as_raw_PtrOfKinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_KinFu>::opencv_from_extern(ret) };
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
			return_receive!(ocvrs_return => ret);
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
		fn render(&self, image: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_render_const_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn render_1(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_Kinfu_KinFu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Calculates normals for given points
		/// ## Parameters
		/// * points: input vector of points which are 4-float vectors
		/// * normals: output vector of corresponding normals which are 4-float vectors
		#[inline]
		fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Kinfu_KinFu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get current pose in voxel space
		#[inline]
		fn get_pose(&self) -> Result<core::Affine3f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_getPose_const(self.as_raw_Kinfu_KinFu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
		fn update(&mut self, depth: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_KinFu_update_const__InputArrayR(self.as_raw_mut_Kinfu_KinFu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
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

	impl crate::rgbd::Kinfu_KinFuTraitConst for Kinfu_KinFu {
		#[inline] fn as_raw_Kinfu_KinFu(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::Kinfu_KinFuTrait for Kinfu_KinFu {
		#[inline] fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Kinfu_KinFu, crate::rgbd::Kinfu_KinFuTraitConst, as_raw_Kinfu_KinFu, crate::rgbd::Kinfu_KinFuTrait, as_raw_mut_Kinfu_KinFu }

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

	impl Kinfu_Params {
		#[inline]
		pub fn default() -> Result<crate::rgbd::Kinfu_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			unsafe { sys::cv_kinfu_Params_Params_Matx33f_Vec3f(&volume_initial_pose_rot, &volume_initial_pose_transl, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			unsafe { sys::cv_kinfu_Params_Params_Matx44f(&volume_initial_pose, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::Kinfu_Params>::opencv_from_extern(ret) };
			Ok(ret)
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
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// Volume kind
		#[inline]
		fn volume_kind(&self) -> crate::ptcloud::VolumeType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propVolumeKind_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// camera intrinsics
		#[inline]
		fn intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propIntr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// rgb camera intrinsics
		#[inline]
		fn rgb_intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propRgb_intr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
		fn volume_pose(&self) -> core::Matx44f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_propVolumePose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
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
			let ret = unsafe { sys::cv_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_Kinfu_Params(), &val) };
			ret
		}

		/// Volume kind
		#[inline]
		fn set_volume_kind(&mut self, val: crate::ptcloud::VolumeType) {
			let ret = unsafe { sys::cv_kinfu_Params_propVolumeKind_const_VolumeType(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// camera intrinsics
		#[inline]
		fn set_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_Kinfu_Params(), &val) };
			ret
		}

		/// rgb camera intrinsics
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
		#[inline]
		fn set_depth_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_depth(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_spatial(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_kernel_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// Number of pyramid levels for ICP
		#[inline]
		fn set_pyramid_levels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// Resolution of voxel space
		///
		/// Number of voxels in each dimension.
		#[inline]
		fn set_volume_dims(&mut self, val: core::Vec3i) {
			let ret = unsafe { sys::cv_kinfu_Params_propVolumeDims_const_Vec3i(self.as_raw_mut_Kinfu_Params(), &val) };
			ret
		}

		/// Size of voxel in meters
		#[inline]
		fn set_voxel_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propVoxelSize_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// Minimal camera movement in meters
		///
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn set_tsdf_min_camera_movement(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// initial volume pose in meters
		#[inline]
		fn set_volume_pose(&mut self, val: core::Matx44f) {
			let ret = unsafe { sys::cv_kinfu_Params_propVolumePose_const_Matx44f(self.as_raw_mut_Kinfu_Params(), &val) };
			ret
		}

		/// distance to truncate in meters
		///
		/// Distances to surface that exceed this value will be truncated to 1.0.
		#[inline]
		fn set_tsdf_trunc_dist(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_trunc_dist_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// max number of frames per voxel
		///
		/// Each voxel keeps running average of distances no longer than this value.
		#[inline]
		fn set_tsdf_max_weight(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_Params_propTsdf_max_weight_const_int(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// A length of one raycast step
		///
		/// How much voxel sizes we skip each raycast step
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propRaycast_step_factor_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// light pose for rendering in meters
		#[inline]
		fn set_light_pose(&mut self, val: core::Vec3f) {
			let ret = unsafe { sys::cv_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_Kinfu_Params(), &val) };
			ret
		}

		/// distance theshold for ICP in meters
		#[inline]
		fn set_icp_dist_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// angle threshold for ICP in radians
		#[inline]
		fn set_icp_angle_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_Kinfu_Params(), val) };
			ret
		}

		/// number of ICP iterations for each pyramid level
		#[inline]
		fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_Kinfu_Params(), val.as_raw_VectorOfi32()) };
			ret
		}

		/// Threshold for depth truncation in meters
		///
		/// All depth values beyond this threshold will be set to zero
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
		#[inline]
		fn set_initial_volume_pose(&mut self, r: core::Matx33f, t: core::Vec3f) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(self.as_raw_mut_Kinfu_Params(), &r, &t, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			unsafe { sys::cv_kinfu_Params_setInitialVolumePose_Matx44f(self.as_raw_mut_Kinfu_Params(), &homogen_tf, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Kinfu_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Kinfu_Params")
				.field("frame_size", &crate::rgbd::Kinfu_ParamsTraitConst::frame_size(self))
				.field("volume_kind", &crate::rgbd::Kinfu_ParamsTraitConst::volume_kind(self))
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

	impl crate::rgbd::Kinfu_ParamsTraitConst for Kinfu_Params {
		#[inline] fn as_raw_Kinfu_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::Kinfu_ParamsTrait for Kinfu_Params {
		#[inline] fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Kinfu_Params, crate::rgbd::Kinfu_ParamsTraitConst, as_raw_Kinfu_Params, crate::rgbd::Kinfu_ParamsTrait, as_raw_mut_Kinfu_Params }

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

	impl Kinfu_VolumeParams {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::rgbd::Kinfu_VolumeParams {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_defaultNew_const() };
			let ret = unsafe { crate::rgbd::Kinfu_VolumeParams::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::rgbd::Kinfu_VolumeParams]
	pub trait Kinfu_VolumeParamsTraitConst {
		fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void;

		/// Kind of Volume
		/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
		#[inline]
		fn kind(&self) -> crate::ptcloud::VolumeType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_VolumeParams_propKind_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// Resolution of voxel space
		/// Number of voxels in each dimension.
		/// Applicable only for TSDF Volume.
		/// HashTSDF volume only supports equal resolution in all three dimensions
		#[inline]
		fn resolution_x(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionX_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}

		#[inline]
		fn resolution_y(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionY_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}

		#[inline]
		fn resolution_z(&self) -> i32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionZ_const(self.as_raw_Kinfu_VolumeParams()) };
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

		/// Size of volume in meters
		#[inline]
		fn volum_size(&self) -> f32 {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propVolumSize_const(self.as_raw_Kinfu_VolumeParams()) };
			ret
		}

		/// Initial pose of the volume in meters, should be 4x4 float or double matrix
		#[inline]
		fn pose(&self) -> core::Matx44f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_kinfu_VolumeParams_propPose_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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

		/// Kind of Volume
		/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
		///
		/// ## C++ default parameters
		/// * val: VolumeType::TSDF
		#[inline]
		fn set_kind(&mut self, val: crate::ptcloud::VolumeType) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propKind_const_VolumeType(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// Resolution of voxel space
		/// Number of voxels in each dimension.
		/// Applicable only for TSDF Volume.
		/// HashTSDF volume only supports equal resolution in all three dimensions
		///
		/// ## C++ default parameters
		/// * val: 128
		#[inline]
		fn set_resolution_x(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionX_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// ## C++ default parameters
		/// * val: 128
		#[inline]
		fn set_resolution_y(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionY_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// ## C++ default parameters
		/// * val: 128
		#[inline]
		fn set_resolution_z(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionZ_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// Resolution of volumeUnit in voxel space
		/// Number of voxels in each dimension for volumeUnit
		/// Applicable only for hashTSDF.
		///
		/// ## C++ default parameters
		/// * val: 0
		#[inline]
		fn set_unit_resolution(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propUnitResolution_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// Size of volume in meters
		///
		/// ## C++ default parameters
		/// * val: 3.f
		#[inline]
		fn set_volum_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propVolumSize_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// Initial pose of the volume in meters, should be 4x4 float or double matrix
		///
		/// ## C++ default parameters
		/// * val: Affine3f().translate(Vec3f(-volumSize/2.f,-volumSize/2.f,0.5f)).matrix
		#[inline]
		fn set_pose(&mut self, val: core::Matx44f) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propPose_const_Matx44f(self.as_raw_mut_Kinfu_VolumeParams(), &val) };
			ret
		}

		/// Length of voxels in meters
		///
		/// ## C++ default parameters
		/// * val: volumSize/512.f
		#[inline]
		fn set_voxel_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propVoxelSize_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// TSDF truncation distance
		/// Distances greater than value from surface will be truncated to 1.0
		///
		/// ## C++ default parameters
		/// * val: 7.f*voxelSize
		#[inline]
		fn set_tsdf_trunc_dist(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propTsdfTruncDist_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// Max number of frames to integrate per voxel
		/// Represents the max number of frames over which a running average
		/// of the TSDF is calculated for a voxel
		///
		/// ## C++ default parameters
		/// * val: 64
		#[inline]
		fn set_max_weight(&mut self, val: i32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propMaxWeight_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// Threshold for depth truncation in meters
		/// Truncates the depth greater than threshold to 0
		///
		/// ## C++ default parameters
		/// * val: 0.f
		#[inline]
		fn set_depth_trunc_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propDepthTruncThreshold_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

		/// Length of single raycast step
		/// Describes the percentage of voxel length that is skipped per march
		///
		/// ## C++ default parameters
		/// * val: 0.25f
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_kinfu_VolumeParams_propRaycastStepFactor_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
			ret
		}

	}

	impl Default for Kinfu_VolumeParams {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for Kinfu_VolumeParams {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Kinfu_VolumeParams")
				.field("kind", &crate::rgbd::Kinfu_VolumeParamsTraitConst::kind(self))
				.field("resolution_x", &crate::rgbd::Kinfu_VolumeParamsTraitConst::resolution_x(self))
				.field("resolution_y", &crate::rgbd::Kinfu_VolumeParamsTraitConst::resolution_y(self))
				.field("resolution_z", &crate::rgbd::Kinfu_VolumeParamsTraitConst::resolution_z(self))
				.field("unit_resolution", &crate::rgbd::Kinfu_VolumeParamsTraitConst::unit_resolution(self))
				.field("volum_size", &crate::rgbd::Kinfu_VolumeParamsTraitConst::volum_size(self))
				.field("pose", &crate::rgbd::Kinfu_VolumeParamsTraitConst::pose(self))
				.field("voxel_size", &crate::rgbd::Kinfu_VolumeParamsTraitConst::voxel_size(self))
				.field("tsdf_trunc_dist", &crate::rgbd::Kinfu_VolumeParamsTraitConst::tsdf_trunc_dist(self))
				.field("max_weight", &crate::rgbd::Kinfu_VolumeParamsTraitConst::max_weight(self))
				.field("depth_trunc_threshold", &crate::rgbd::Kinfu_VolumeParamsTraitConst::depth_trunc_threshold(self))
				.field("raycast_step_factor", &crate::rgbd::Kinfu_VolumeParamsTraitConst::raycast_step_factor(self))
				.finish()
		}
	}

	impl crate::rgbd::Kinfu_VolumeParamsTraitConst for Kinfu_VolumeParams {
		#[inline] fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::Kinfu_VolumeParamsTrait for Kinfu_VolumeParams {
		#[inline] fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Kinfu_VolumeParams, crate::rgbd::Kinfu_VolumeParamsTraitConst, as_raw_Kinfu_VolumeParams, crate::rgbd::Kinfu_VolumeParamsTrait, as_raw_mut_Kinfu_VolumeParams }

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
	/// which represent the distance to the closest surface (for details read the [kinectfusion](https://docs.opencv.org/5.0.0/d0/de3/citelist.html#CITEREF_kinectfusion) article
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

	impl LargeKinfu {
		#[inline]
		pub fn create(_params: &core::Ptr<crate::rgbd::Params>) -> Result<core::Ptr<crate::rgbd::LargeKinfu>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_create_const_PtrLParamsGR(_params.as_raw_PtrOfParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::LargeKinfu>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::rgbd::LargeKinfu]
	pub trait LargeKinfuTraitConst {
		fn as_raw_LargeKinfu(&self) -> *const c_void;

		#[inline]
		fn get_params(&self) -> Result<crate::rgbd::Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getParams_const(self.as_raw_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::Params::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn render(&self, image: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn render_1(&self, image: &mut impl ToOutputArray, camera_pose: core::Matx44f) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(self.as_raw_LargeKinfu(), image.as_raw__OutputArray(), &camera_pose, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_points(&self, points: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_LargeKinfu(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_pose(&self) -> Result<core::Affine3f> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_getPose_const(self.as_raw_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn update(&mut self, depth: &impl ToInputArray) -> Result<bool> {
			input_array_arg!(depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_LargeKinfu_update_const__InputArrayR(self.as_raw_mut_LargeKinfu(), depth.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
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

	impl crate::rgbd::LargeKinfuTraitConst for LargeKinfu {
		#[inline] fn as_raw_LargeKinfu(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::LargeKinfuTrait for LargeKinfu {
		#[inline] fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LargeKinfu, crate::rgbd::LargeKinfuTraitConst, as_raw_LargeKinfu, crate::rgbd::LargeKinfuTrait, as_raw_mut_LargeKinfu }

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

	impl Params {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::rgbd::Params {
			let ret = unsafe { sys::cv_large_kinfu_Params_defaultNew_const() };
			let ret = unsafe { crate::rgbd::Params::opencv_from_extern(ret) };
			ret
		}

		/// Default parameters
		///
		/// A set of parameters which provides better model quality, can be very slow.
		#[inline]
		pub fn default_params() -> Result<core::Ptr<crate::rgbd::Params>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_defaultParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Coarse parameters
		///
		/// A set of parameters which provides better speed, can fail to match frames
		/// in case of rapid sensor motion.
		#[inline]
		pub fn coarse_params() -> Result<core::Ptr<crate::rgbd::Params>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_coarseParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// HashTSDF parameters
		///
		/// A set of parameters suitable for use with HashTSDFVolume
		#[inline]
		pub fn hash_tsdf_params(is_coarse: bool) -> Result<core::Ptr<crate::rgbd::Params>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_hashTSDFParams_bool(is_coarse, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::Params>::opencv_from_extern(ret) };
			Ok(ret)
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
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// camera intrinsics
		#[inline]
		fn intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_propIntr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// rgb camera intrinsics
		#[inline]
		fn rgb_intr(&self) -> core::Matx33f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_Params_propRgb_intr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		///
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
			return_receive!(ocvrs_return => ret);
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
		///
		/// All depth values beyond this threshold will be set to zero
		#[inline]
		fn truncate_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_const(self.as_raw_Params()) };
			ret
		}

		/// Volume parameters
		#[inline]
		fn volume_params(&self) -> crate::rgbd::VolumeParams {
			let ret = unsafe { sys::cv_large_kinfu_Params_propVolumeParams_const(self.as_raw_Params()) };
			let ret = unsafe { crate::rgbd::VolumeParams::opencv_from_extern(ret) };
			ret
		}

	}

	/// Mutable methods for [crate::rgbd::Params]
	pub trait ParamsTrait: crate::rgbd::ParamsTraitConst {
		fn as_raw_mut_Params(&mut self) -> *mut c_void;

		/// frame size in pixels
		#[inline]
		fn set_frame_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_Params(), &val) };
			ret
		}

		/// camera intrinsics
		#[inline]
		fn set_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_Params(), &val) };
			ret
		}

		/// rgb camera intrinsics
		#[inline]
		fn set_rgb_intr(&mut self, val: core::Matx33f) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propRgb_intr_const_Matx33f(self.as_raw_mut_Params(), &val) };
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
			let ret = unsafe { sys::cv_large_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_Params(), val) };
			ret
		}

		/// Depth sigma in meters for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_depth(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_Params(), val) };
			ret
		}

		/// Spatial sigma in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_sigma_spatial(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_Params(), val) };
			ret
		}

		/// Kernel size in pixels for bilateral smooth
		#[inline]
		fn set_bilateral_kernel_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_Params(), val) };
			ret
		}

		/// Number of pyramid levels for ICP
		#[inline]
		fn set_pyramid_levels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_Params(), val) };
			ret
		}

		/// Minimal camera movement in meters
		///
		/// Integrate new depth frame only if camera movement exceeds this value.
		#[inline]
		fn set_tsdf_min_camera_movement(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_Params(), val) };
			ret
		}

		/// light pose for rendering in meters
		#[inline]
		fn set_light_pose(&mut self, val: core::Vec3f) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_Params(), &val) };
			ret
		}

		/// distance theshold for ICP in meters
		#[inline]
		fn set_icp_dist_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_Params(), val) };
			ret
		}

		/// angle threshold for ICP in radians
		#[inline]
		fn set_icp_angle_thresh(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_Params(), val) };
			ret
		}

		/// number of ICP iterations for each pyramid level
		#[inline]
		fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_Params(), val.as_raw_VectorOfi32()) };
			ret
		}

		/// Threshold for depth truncation in meters
		///
		/// All depth values beyond this threshold will be set to zero
		#[inline]
		fn set_truncate_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_const_float(self.as_raw_mut_Params(), val) };
			ret
		}

		/// Volume parameters
		#[inline]
		fn set_volume_params(&mut self, val: crate::rgbd::VolumeParams) {
			let ret = unsafe { sys::cv_large_kinfu_Params_propVolumeParams_const_VolumeParams(self.as_raw_mut_Params(), val.as_raw_VolumeParams()) };
			ret
		}

	}

	impl Default for Params {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
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

	impl crate::rgbd::ParamsTraitConst for Params {
		#[inline] fn as_raw_Params(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::ParamsTrait for Params {
		#[inline] fn as_raw_mut_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Params, crate::rgbd::ParamsTraitConst, as_raw_Params, crate::rgbd::ParamsTrait, as_raw_mut_Params }

	pub struct VolumeParams {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { VolumeParams }

	impl Drop for VolumeParams {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_large_kinfu_VolumeParams_delete(self.as_raw_mut_VolumeParams()) };
		}
	}

	unsafe impl Send for VolumeParams {}

	impl VolumeParams {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::rgbd::VolumeParams {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_defaultNew_const() };
			let ret = unsafe { crate::rgbd::VolumeParams::opencv_from_extern(ret) };
			ret
		}

		/// Default set of parameters that provide higher quality reconstruction at the cost of slow performance.
		#[inline]
		pub fn default_params(volume_type: crate::ptcloud::VolumeType) -> Result<core::Ptr<crate::rgbd::VolumeParams>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_VolumeParams_defaultParams_VolumeType(volume_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::VolumeParams>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Coarse set of parameters that provides relatively higher performance at the cost of reconstrution quality.
		#[inline]
		pub fn coarse_params(volume_type: crate::ptcloud::VolumeType) -> Result<core::Ptr<crate::rgbd::VolumeParams>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_VolumeParams_coarseParams_VolumeType(volume_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::VolumeParams>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::rgbd::VolumeParams]
	pub trait VolumeParamsTraitConst {
		fn as_raw_VolumeParams(&self) -> *const c_void;

		/// Kind of Volume
		///
		/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
		#[inline]
		fn kind(&self) -> crate::ptcloud::VolumeType {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_VolumeParams_propKind_const(self.as_raw_VolumeParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// Resolution of voxel space
		///
		/// Number of voxels in each dimension.
		/// Applicable only for TSDF Volume.
		/// HashTSDF volume only supports equal resolution in all three dimensions
		#[inline]
		fn resolution_x(&self) -> i32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionX_const(self.as_raw_VolumeParams()) };
			ret
		}

		#[inline]
		fn resolution_y(&self) -> i32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionY_const(self.as_raw_VolumeParams()) };
			ret
		}

		#[inline]
		fn resolution_z(&self) -> i32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionZ_const(self.as_raw_VolumeParams()) };
			ret
		}

		/// Resolution of volumeUnit in voxel space
		///
		/// Number of voxels in each dimension for volumeUnit
		/// Applicable only for hashTSDF.
		#[inline]
		fn unit_resolution(&self) -> i32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propUnitResolution_const(self.as_raw_VolumeParams()) };
			ret
		}

		/// Size of volume in meters
		#[inline]
		fn volum_size(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propVolumSize_const(self.as_raw_VolumeParams()) };
			ret
		}

		/// Initial pose of the volume in meters, should be 4x4 float or double matrix
		#[inline]
		fn pose(&self) -> core::Matx44f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_large_kinfu_VolumeParams_propPose_const(self.as_raw_VolumeParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			ret
		}

		/// Length of voxels in meters
		#[inline]
		fn voxel_size(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propVoxelSize_const(self.as_raw_VolumeParams()) };
			ret
		}

		/// TSDF truncation distance
		///
		/// Distances greater than value from surface will be truncated to 1.0
		#[inline]
		fn tsdf_trunc_dist(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propTsdfTruncDist_const(self.as_raw_VolumeParams()) };
			ret
		}

		/// Max number of frames to integrate per voxel
		///
		/// Represents the max number of frames over which a running average
		/// of the TSDF is calculated for a voxel
		#[inline]
		fn max_weight(&self) -> i32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propMaxWeight_const(self.as_raw_VolumeParams()) };
			ret
		}

		/// Threshold for depth truncation in meters
		///
		/// Truncates the depth greater than threshold to 0
		#[inline]
		fn depth_trunc_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propDepthTruncThreshold_const(self.as_raw_VolumeParams()) };
			ret
		}

		/// Length of single raycast step
		///
		/// Describes the percentage of voxel length that is skipped per march
		#[inline]
		fn raycast_step_factor(&self) -> f32 {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propRaycastStepFactor_const(self.as_raw_VolumeParams()) };
			ret
		}

	}

	/// Mutable methods for [crate::rgbd::VolumeParams]
	pub trait VolumeParamsTrait: crate::rgbd::VolumeParamsTraitConst {
		fn as_raw_mut_VolumeParams(&mut self) -> *mut c_void;

		/// Kind of Volume
		///
		/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
		///
		/// ## C++ default parameters
		/// * val: VolumeType::TSDF
		#[inline]
		fn set_kind(&mut self, val: crate::ptcloud::VolumeType) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propKind_const_VolumeType(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// Resolution of voxel space
		///
		/// Number of voxels in each dimension.
		/// Applicable only for TSDF Volume.
		/// HashTSDF volume only supports equal resolution in all three dimensions
		///
		/// ## C++ default parameters
		/// * val: 128
		#[inline]
		fn set_resolution_x(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionX_const_int(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// ## C++ default parameters
		/// * val: 128
		#[inline]
		fn set_resolution_y(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionY_const_int(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// ## C++ default parameters
		/// * val: 128
		#[inline]
		fn set_resolution_z(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionZ_const_int(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// Resolution of volumeUnit in voxel space
		///
		/// Number of voxels in each dimension for volumeUnit
		/// Applicable only for hashTSDF.
		///
		/// ## C++ default parameters
		/// * val: 0
		#[inline]
		fn set_unit_resolution(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propUnitResolution_const_int(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// Size of volume in meters
		///
		/// ## C++ default parameters
		/// * val: 3.f
		#[inline]
		fn set_volum_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propVolumSize_const_float(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// Initial pose of the volume in meters, should be 4x4 float or double matrix
		///
		/// ## C++ default parameters
		/// * val: Affine3f().translate(Vec3f(-volumSize/2.f,-volumSize/2.f,0.5f)).matrix
		#[inline]
		fn set_pose(&mut self, val: core::Matx44f) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propPose_const_Matx44f(self.as_raw_mut_VolumeParams(), &val) };
			ret
		}

		/// Length of voxels in meters
		///
		/// ## C++ default parameters
		/// * val: volumSize/512.f
		#[inline]
		fn set_voxel_size(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propVoxelSize_const_float(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// TSDF truncation distance
		///
		/// Distances greater than value from surface will be truncated to 1.0
		///
		/// ## C++ default parameters
		/// * val: 7.f*voxelSize
		#[inline]
		fn set_tsdf_trunc_dist(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propTsdfTruncDist_const_float(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// Max number of frames to integrate per voxel
		///
		/// Represents the max number of frames over which a running average
		/// of the TSDF is calculated for a voxel
		///
		/// ## C++ default parameters
		/// * val: 64
		#[inline]
		fn set_max_weight(&mut self, val: i32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propMaxWeight_const_int(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// Threshold for depth truncation in meters
		///
		/// Truncates the depth greater than threshold to 0
		///
		/// ## C++ default parameters
		/// * val: 0.f
		#[inline]
		fn set_depth_trunc_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propDepthTruncThreshold_const_float(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

		/// Length of single raycast step
		///
		/// Describes the percentage of voxel length that is skipped per march
		///
		/// ## C++ default parameters
		/// * val: 0.25f
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) {
			let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propRaycastStepFactor_const_float(self.as_raw_mut_VolumeParams(), val) };
			ret
		}

	}

	impl Default for VolumeParams {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for VolumeParams {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("VolumeParams")
				.field("kind", &crate::rgbd::VolumeParamsTraitConst::kind(self))
				.field("resolution_x", &crate::rgbd::VolumeParamsTraitConst::resolution_x(self))
				.field("resolution_y", &crate::rgbd::VolumeParamsTraitConst::resolution_y(self))
				.field("resolution_z", &crate::rgbd::VolumeParamsTraitConst::resolution_z(self))
				.field("unit_resolution", &crate::rgbd::VolumeParamsTraitConst::unit_resolution(self))
				.field("volum_size", &crate::rgbd::VolumeParamsTraitConst::volum_size(self))
				.field("pose", &crate::rgbd::VolumeParamsTraitConst::pose(self))
				.field("voxel_size", &crate::rgbd::VolumeParamsTraitConst::voxel_size(self))
				.field("tsdf_trunc_dist", &crate::rgbd::VolumeParamsTraitConst::tsdf_trunc_dist(self))
				.field("max_weight", &crate::rgbd::VolumeParamsTraitConst::max_weight(self))
				.field("depth_trunc_threshold", &crate::rgbd::VolumeParamsTraitConst::depth_trunc_threshold(self))
				.field("raycast_step_factor", &crate::rgbd::VolumeParamsTraitConst::raycast_step_factor(self))
				.finish()
		}
	}

	impl crate::rgbd::VolumeParamsTraitConst for VolumeParams {
		#[inline] fn as_raw_VolumeParams(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::VolumeParamsTrait for VolumeParams {
		#[inline] fn as_raw_mut_VolumeParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { VolumeParams, crate::rgbd::VolumeParamsTraitConst, as_raw_VolumeParams, crate::rgbd::VolumeParamsTrait, as_raw_mut_VolumeParams }

	/// \brief Modality that computes quantized gradient orientations from a color image.
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

	impl LineMod_ColorGradient {
		/// \brief Default constructor. Uses reasonable default parameter values.
		#[inline]
		pub fn default() -> Result<crate::rgbd::LineMod_ColorGradient> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_ColorGradient_ColorGradient(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_ColorGradient::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create(weak_threshold: f32, num_features: size_t, strong_threshold: f32) -> Result<core::Ptr<crate::rgbd::LineMod_ColorGradient>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_ColorGradient_create_float_size_t_float(weak_threshold, num_features, strong_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_ColorGradient>::opencv_from_extern(ret) };
			Ok(ret)
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_ColorGradient_write_const_FileStorageR(self.as_raw_LineMod_ColorGradient(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::rgbd::LineMod_ColorGradient]
	pub trait LineMod_ColorGradientTrait: crate::rgbd::LineMod_ColorGradientTraitConst + crate::rgbd::LineMod_ModalityTrait {
		fn as_raw_mut_LineMod_ColorGradient(&mut self) -> *mut c_void;

		#[inline]
		fn set_weak_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
			ret
		}

		#[inline]
		fn set_num_features(&mut self, val: size_t) {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const_size_t(self.as_raw_mut_LineMod_ColorGradient(), val) };
			ret
		}

		#[inline]
		fn set_strong_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
			ret
		}

		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_ColorGradient_read_const_FileNodeR(self.as_raw_mut_LineMod_ColorGradient(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

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

	boxed_cast_base! { LineMod_ColorGradient, crate::rgbd::LineMod_Modality, cv_linemod_ColorGradient_to_LineMod_Modality }

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

	/// \brief Modality that computes quantized surface normals from a dense depth map.
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

	impl LineMod_DepthNormal {
		/// \brief Default constructor. Uses reasonable default parameter values.
		#[inline]
		pub fn default() -> Result<crate::rgbd::LineMod_DepthNormal> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_DepthNormal_DepthNormal(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_DepthNormal::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn create(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32) -> Result<core::Ptr<crate::rgbd::LineMod_DepthNormal>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_DepthNormal_create_int_int_size_t_int(distance_threshold, difference_threshold, num_features, extract_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_DepthNormal>::opencv_from_extern(ret) };
			Ok(ret)
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_DepthNormal_write_const_FileStorageR(self.as_raw_LineMod_DepthNormal(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::rgbd::LineMod_DepthNormal]
	pub trait LineMod_DepthNormalTrait: crate::rgbd::LineMod_DepthNormalTraitConst + crate::rgbd::LineMod_ModalityTrait {
		fn as_raw_mut_LineMod_DepthNormal(&mut self) -> *mut c_void;

		#[inline]
		fn set_distance_threshold(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}

		#[inline]
		fn set_difference_threshold(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}

		#[inline]
		fn set_num_features(&mut self, val: size_t) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const_size_t(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}

		#[inline]
		fn set_extract_threshold(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
			ret
		}

		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_DepthNormal_read_const_FileNodeR(self.as_raw_mut_LineMod_DepthNormal(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

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

	boxed_cast_base! { LineMod_DepthNormal, crate::rgbd::LineMod_Modality, cv_linemod_DepthNormal_to_LineMod_Modality }

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

	/// \brief Object detector using the LINE template matching algorithm with any set of
	/// modalities.
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

	impl LineMod_Detector {
		/// \brief Empty constructor, initialize with read().
		#[inline]
		pub fn default() -> Result<crate::rgbd::LineMod_Detector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_Detector(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_Detector::opencv_from_extern(ret) };
			Ok(ret)
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
		fn match_(&self, sources: &core::Vector<core::Mat>, threshold: f32, matches: &mut core::Vector<crate::rgbd::LineMod_Match>, class_ids: &core::Vector<String>, quantized_images: &mut impl ToOutputArray, masks: &core::Vector<core::Mat>) -> Result<()> {
			output_array_arg!(quantized_images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(self.as_raw_LineMod_Detector(), sources.as_raw_VectorOfMat(), threshold, matches.as_raw_mut_VectorOfLineMod_Match(), class_ids.as_raw_VectorOfString(), quantized_images.as_raw__OutputArray(), masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn match__def(&self, sources: &core::Vector<core::Mat>, threshold: f32, matches: &mut core::Vector<crate::rgbd::LineMod_Match>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(self.as_raw_LineMod_Detector(), sources.as_raw_VectorOfMat(), threshold, matches.as_raw_mut_VectorOfLineMod_Match(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Ptr<crate::rgbd::LineMod_Modality>>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// \brief Get sampling step T at pyramid_level.
		#[inline]
		fn get_t(&self, pyramid_level: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_getT_const_int(self.as_raw_LineMod_Detector(), pyramid_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// \brief Get number of pyramid levels used by this detector.
		#[inline]
		fn pyramid_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_pyramidLevels_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::rgbd::LineMod_Template>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn num_templates(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_numTemplates_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn num_templates_1(&self, class_id: &str) -> Result<i32> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_numTemplates_const_const_StringR(self.as_raw_LineMod_Detector(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn num_classes(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_numClasses_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn class_ids(&self) -> Result<core::Vector<String>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_classIds_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_write_const_FileStorageR(self.as_raw_LineMod_Detector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn write_class(&self, class_id: &str, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(self.as_raw_LineMod_Detector(), class_id.opencv_as_extern(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [LineMod_DetectorTraitConst::write_classes] function uses the following default values for its arguments:
		/// * format: "templates_%s.yml.gz"
		#[inline]
		fn write_classes_def(&self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_writeClasses_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		fn add_template(&mut self, sources: &core::Vector<core::Mat>, class_id: &str, object_mask: &impl core::MatTraitConst, bounding_box: &mut core::Rect) -> Result<i32> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(self.as_raw_mut_LineMod_Detector(), sources.as_raw_VectorOfMat(), class_id.opencv_as_extern(), object_mask.as_raw_Mat(), bounding_box, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn add_template_def(&mut self, sources: &core::Vector<core::Mat>, class_id: &str, object_mask: &impl core::MatTraitConst) -> Result<i32> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(self.as_raw_mut_LineMod_Detector(), sources.as_raw_VectorOfMat(), class_id.opencv_as_extern(), object_mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// \brief Add a new object template computed by external means.
		#[inline]
		fn add_synthetic_template(&mut self, templates: &core::Vector<crate::rgbd::LineMod_Template>, class_id: &str) -> Result<i32> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(self.as_raw_mut_LineMod_Detector(), templates.as_raw_VectorOfLineMod_Template(), class_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_read_const_FileNodeR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## C++ default parameters
		/// * class_id_override: ""
		#[inline]
		fn read_class(&mut self, fn_: &impl core::FileNodeTraitConst, class_id_override: &str) -> Result<String> {
			extern_container_arg!(class_id_override);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), class_id_override.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [LineMod_DetectorTrait::read_class] function uses the following default values for its arguments:
		/// * class_id_override: ""
		#[inline]
		fn read_class_def(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeR(self.as_raw_mut_LineMod_Detector(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [LineMod_DetectorTrait::read_classes] function uses the following default values for its arguments:
		/// * format: "templates_%s.yml.gz"
		#[inline]
		fn read_classes_def(&mut self, class_ids: &core::Vector<String>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Detector_readClasses_const_vectorLStringGR(self.as_raw_mut_LineMod_Detector(), class_ids.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
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

	impl crate::rgbd::LineMod_DetectorTraitConst for LineMod_Detector {
		#[inline] fn as_raw_LineMod_Detector(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::LineMod_DetectorTrait for LineMod_Detector {
		#[inline] fn as_raw_mut_LineMod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LineMod_Detector, crate::rgbd::LineMod_DetectorTraitConst, as_raw_LineMod_Detector, crate::rgbd::LineMod_DetectorTrait, as_raw_mut_LineMod_Detector }

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
		pub fn write(self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_write_const_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn default() -> Result<crate::rgbd::LineMod_Feature> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_Feature(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn new(x: i32, y: i32, label: i32) -> Result<crate::rgbd::LineMod_Feature> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_Feature_int_int_int(x, y, label, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn read(self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Feature_read_const_FileNodeR(&self, fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// \brief Represents a successful template match.
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

	impl LineMod_Match {
		#[inline]
		pub fn default() -> Result<crate::rgbd::LineMod_Match> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Match_Match(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_Match::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn new(x: i32, y: i32, similarity: f32, class_id: &str, template_id: i32) -> Result<crate::rgbd::LineMod_Match> {
			extern_container_arg!(class_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Match_Match_int_int_float_const_StringR_int(x, y, similarity, class_id.opencv_as_extern(), template_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::rgbd::LineMod_Match::opencv_from_extern(ret) };
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
		fn less_than(&self, rhs: &impl crate::rgbd::LineMod_MatchTraitConst) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Match_operatorL_const_const_MatchR(self.as_raw_LineMod_Match(), rhs.as_raw_LineMod_Match(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn equals(&self, rhs: &impl crate::rgbd::LineMod_MatchTraitConst) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Match_operatorEQ_const_const_MatchR(self.as_raw_LineMod_Match(), rhs.as_raw_LineMod_Match(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::rgbd::LineMod_Match]
	pub trait LineMod_MatchTrait: crate::rgbd::LineMod_MatchTraitConst {
		fn as_raw_mut_LineMod_Match(&mut self) -> *mut c_void;

		#[inline]
		fn set_x(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Match_propX_const_int(self.as_raw_mut_LineMod_Match(), val) };
			ret
		}

		#[inline]
		fn set_y(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Match_propY_const_int(self.as_raw_mut_LineMod_Match(), val) };
			ret
		}

		#[inline]
		fn set_similarity(&mut self, val: f32) {
			let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const_float(self.as_raw_mut_LineMod_Match(), val) };
			ret
		}

		#[inline]
		fn set_class_id(&mut self, val: &str) {
			extern_container_arg!(nofail val);
			let ret = unsafe { sys::cv_linemod_Match_propClass_id_const_String(self.as_raw_mut_LineMod_Match(), val.opencv_as_extern()) };
			ret
		}

		#[inline]
		fn set_template_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const_int(self.as_raw_mut_LineMod_Match(), val) };
			ret
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

	impl crate::rgbd::LineMod_MatchTraitConst for LineMod_Match {
		#[inline] fn as_raw_LineMod_Match(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::LineMod_MatchTrait for LineMod_Match {
		#[inline] fn as_raw_mut_LineMod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LineMod_Match, crate::rgbd::LineMod_MatchTraitConst, as_raw_LineMod_Match, crate::rgbd::LineMod_MatchTrait, as_raw_mut_LineMod_Match }

	/// \brief Interface for modalities that plug into the LINE template matching representation.
	///
	/// \todo Max response, to allow optimization of summing (255/MAX) features as uint8
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_Modality>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// \brief Load a modality from file.
		#[inline]
		pub fn create_1(fn_: &impl core::FileNodeTraitConst) -> Result<core::Ptr<crate::rgbd::LineMod_Modality>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_create_const_FileNodeR(fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_Modality>::opencv_from_extern(ret) };
			Ok(ret)
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
		fn process(&self, src: &impl core::MatTraitConst, mask: &impl core::MatTraitConst) -> Result<core::Ptr<crate::rgbd::LineMod_QuantizedPyramid>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_process_const_const_MatR_const_MatR(self.as_raw_LineMod_Modality(), src.as_raw_Mat(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
		#[inline]
		fn process_def(&self, src: &impl core::MatTraitConst) -> Result<core::Ptr<crate::rgbd::LineMod_QuantizedPyramid>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_process_const_const_MatR(self.as_raw_LineMod_Modality(), src.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rgbd::LineMod_QuantizedPyramid>::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn name(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_name_const(self.as_raw_LineMod_Modality(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_write_const_FileStorageR(self.as_raw_LineMod_Modality(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::rgbd::LineMod_Modality]
	pub trait LineMod_ModalityTrait: crate::rgbd::LineMod_ModalityTraitConst {
		fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void;

		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Modality_read_const_FileNodeR(self.as_raw_mut_LineMod_Modality(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for LineMod_Modality {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LineMod_Modality")
				.finish()
		}
	}

	boxed_cast_descendant! { LineMod_Modality, crate::rgbd::LineMod_ColorGradient, cv_linemod_Modality_to_LineMod_ColorGradient }

	boxed_cast_descendant! { LineMod_Modality, crate::rgbd::LineMod_DepthNormal, cv_linemod_Modality_to_LineMod_DepthNormal }

	impl crate::rgbd::LineMod_ModalityTraitConst for LineMod_Modality {
		#[inline] fn as_raw_LineMod_Modality(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::LineMod_ModalityTrait for LineMod_Modality {
		#[inline] fn as_raw_mut_LineMod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LineMod_Modality, crate::rgbd::LineMod_ModalityTraitConst, as_raw_LineMod_Modality, crate::rgbd::LineMod_ModalityTrait, as_raw_mut_LineMod_Modality }

	/// \brief Represents a modality operating over an image pyramid.
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

	/// Constant methods for [crate::rgbd::LineMod_QuantizedPyramid]
	pub trait LineMod_QuantizedPyramidTraitConst {
		fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void;

		/// \brief Compute quantized image at current pyramid level for online detection.
		///
		/// \param[out] dst The destination 8-bit image. For each pixel at most one bit is set,
		///                representing its classification.
		#[inline]
		fn quantize(&self, dst: &mut impl core::MatTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_QuantizedPyramid_quantize_const_MatR(self.as_raw_LineMod_QuantizedPyramid(), dst.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// \brief Extract most discriminant features at current pyramid level to form a new template.
		///
		/// \param[out] templ The new template.
		#[inline]
		fn extract_template(&self, templ: &mut impl crate::rgbd::LineMod_TemplateTrait) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(self.as_raw_LineMod_QuantizedPyramid(), templ.as_raw_mut_LineMod_Template(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
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
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for LineMod_QuantizedPyramid {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LineMod_QuantizedPyramid")
				.finish()
		}
	}

	impl crate::rgbd::LineMod_QuantizedPyramidTraitConst for LineMod_QuantizedPyramid {
		#[inline] fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::LineMod_QuantizedPyramidTrait for LineMod_QuantizedPyramid {
		#[inline] fn as_raw_mut_LineMod_QuantizedPyramid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LineMod_QuantizedPyramid, crate::rgbd::LineMod_QuantizedPyramidTraitConst, as_raw_LineMod_QuantizedPyramid, crate::rgbd::LineMod_QuantizedPyramidTrait, as_raw_mut_LineMod_QuantizedPyramid }

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

	impl LineMod_Template {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::rgbd::LineMod_Template {
			let ret = unsafe { sys::cv_linemod_Template_defaultNew_const() };
			let ret = unsafe { crate::rgbd::LineMod_Template::opencv_from_extern(ret) };
			ret
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
		fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Template_write_const_FileStorageR(self.as_raw_LineMod_Template(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::rgbd::LineMod_Template]
	pub trait LineMod_TemplateTrait: crate::rgbd::LineMod_TemplateTraitConst {
		fn as_raw_mut_LineMod_Template(&mut self) -> *mut c_void;

		#[inline]
		fn set_width(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Template_propWidth_const_int(self.as_raw_mut_LineMod_Template(), val) };
			ret
		}

		#[inline]
		fn set_height(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Template_propHeight_const_int(self.as_raw_mut_LineMod_Template(), val) };
			ret
		}

		#[inline]
		fn set_pyramid_level(&mut self, val: i32) {
			let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const_int(self.as_raw_mut_LineMod_Template(), val) };
			ret
		}

		#[inline]
		fn set_features(&mut self, val: core::Vector<crate::rgbd::LineMod_Feature>) {
			let ret = unsafe { sys::cv_linemod_Template_propFeatures_const_vectorLFeatureG(self.as_raw_mut_LineMod_Template(), val.as_raw_VectorOfLineMod_Feature()) };
			ret
		}

		#[inline]
		fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_linemod_Template_read_const_FileNodeR(self.as_raw_mut_LineMod_Template(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for LineMod_Template {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
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

	impl crate::rgbd::LineMod_TemplateTraitConst for LineMod_Template {
		#[inline] fn as_raw_LineMod_Template(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::rgbd::LineMod_TemplateTrait for LineMod_Template {
		#[inline] fn as_raw_mut_LineMod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { LineMod_Template, crate::rgbd::LineMod_TemplateTraitConst, as_raw_LineMod_Template, crate::rgbd::LineMod_TemplateTrait, as_raw_mut_LineMod_Template }

}
