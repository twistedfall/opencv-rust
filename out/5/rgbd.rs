//! # RGB-Depth Processing
//!
//! [kinfu_icp]
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{ColoredKinfu_ColoredKinFuTrait, ColoredKinfu_ColoredKinFuTraitConst, ColoredKinfu_ParamsTrait, ColoredKinfu_ParamsTraitConst, Dynafu_DynaFuTrait, Dynafu_DynaFuTraitConst, Kinfu_KinFuTrait, Kinfu_KinFuTraitConst, Kinfu_ParamsTrait, Kinfu_ParamsTraitConst, Kinfu_VolumeParamsTrait, Kinfu_VolumeParamsTraitConst, LargeKinfuTrait, LargeKinfuTraitConst, LineMod_ColorGradientTrait, LineMod_ColorGradientTraitConst, LineMod_DepthNormalTrait, LineMod_DepthNormalTraitConst, LineMod_DetectorTrait, LineMod_DetectorTraitConst, LineMod_MatchTrait, LineMod_MatchTraitConst, LineMod_ModalityTrait, LineMod_ModalityTraitConst, LineMod_QuantizedPyramidTrait, LineMod_QuantizedPyramidTraitConst, LineMod_TemplateTrait, LineMod_TemplateTraitConst, ParamsTrait, ParamsTraitConst, VolumeParamsTrait, VolumeParamsTraitConst};
}

/// Backwards compatibility for old versions
// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:44
pub type Dynafu_Params = crate::rgbd::Kinfu_Params;
/// \brief Debug function to colormap a quantized image for viewing.
// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:245
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
// cv::linemod::drawFeatures(InputOutputArray, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:254
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
// drawFeatures(InputOutputArray, const std::vector<Template> &, const Point2i &, int)(InputOutputArray, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:254
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
// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:420
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
// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:428
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

/// Constant methods for [crate::rgbd::ColoredKinfu_ColoredKinFu]
// ColoredKinFu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:197
pub trait ColoredKinfu_ColoredKinFuTraitConst {
	fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void;

	/// Get current parameters
	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:204
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
	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:214
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
	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:226
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

	/// Gets points and normals of current 3d mesh
	///
	/// The order of normals corresponds to order of points.
	/// The order of points is undefined.
	///
	/// ## Parameters
	/// * points: vector of points which are 4-float vectors
	/// * normals: vector of normals which are 4-float vectors
	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:236
	// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_cloud(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
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
	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:244
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
	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:250
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
	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:259
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
	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:256
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
	// update(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:267
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
// ColoredKinFu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:197
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
	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:200
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
// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:19
pub trait ColoredKinfu_ParamsTraitConst {
	fn as_raw_ColoredKinfu_Params(&self) -> *const c_void;

	/// frame size in pixels
	// cv::colored_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:85
	// ("cv::colored_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propFrameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb frame size in pixels
	// cv::colored_kinfu::Params::rgb_frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::rgb_frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rgb_frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::colored_kinfu::Params::volumeKind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:90
	// ("cv::colored_kinfu::Params::volumeKind", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_kind(&self) -> crate::mod_3d::VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propVolumeKind_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// camera intrinsics
	// cv::colored_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:93
	// ("cv::colored_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propIntr_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb camera intrinsics
	// cv::colored_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:96
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
	// cv::colored_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:105
	// ("cv::colored_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propDepthFactor_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::colored_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::colored_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::colored_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:112
	// ("cv::colored_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::colored_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:115
	// ("cv::colored_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propPyramidLevels_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::colored_kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_dims(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propVolumeDims_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Size of voxel in meters
	// cv::colored_kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:123
	// ("cv::colored_kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVoxelSize_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::colored_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:129
	// ("cv::colored_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// initial volume pose in meters
	// cv::colored_kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:132
	// ("cv::colored_kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_pose(&self) -> core::Matx44f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propVolumePose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::colored_kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:138
	// ("cv::colored_kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_trunc_dist_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::colored_kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:144
	// ("cv::colored_kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_max_weight_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::colored_kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:150
	// ("cv::colored_kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propRaycast_step_factor_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// light pose for rendering in meters
	// cv::colored_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:157
	// ("cv::colored_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_colored_kinfu_Params_propLightPose_const(self.as_raw_ColoredKinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance theshold for ICP in meters
	// cv::colored_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpDistThresh_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::colored_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpAngleThresh_const(self.as_raw_ColoredKinfu_Params()) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::colored_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:164
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
	// cv::colored_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:170
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
	// cv::colored_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:85
	// ("cv::colored_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// rgb frame size in pixels
	// cv::colored_kinfu::Params::setRgb_frameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::setRgb_frameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_rgb_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propRgb_frameSize_const_Size(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	// cv::colored_kinfu::Params::setVolumeKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:90
	// ("cv::colored_kinfu::Params::setVolumeKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	#[inline]
	fn set_volume_kind(&mut self, val: crate::mod_3d::VolumeType) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeKind_const_VolumeType(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// camera intrinsics
	// cv::colored_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:93
	// ("cv::colored_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// rgb camera intrinsics
	// cv::colored_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:96
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
	// cv::colored_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:105
	// ("cv::colored_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::colored_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::colored_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::colored_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:112
	// ("cv::colored_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::colored_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:115
	// ("cv::colored_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::colored_kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	#[inline]
	fn set_volume_dims(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumeDims_const_Vec3i(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// Size of voxel in meters
	// cv::colored_kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:123
	// ("cv::colored_kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVoxelSize_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::colored_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:129
	// ("cv::colored_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// initial volume pose in meters
	// cv::colored_kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:132
	// ("cv::colored_kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	#[inline]
	fn set_volume_pose(&mut self, val: core::Matx44f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propVolumePose_const_Matx44f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::colored_kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:138
	// ("cv::colored_kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_trunc_dist_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::colored_kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:144
	// ("cv::colored_kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_tsdf_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propTsdf_max_weight_const_int(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::colored_kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:150
	// ("cv::colored_kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propRaycast_step_factor_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// light pose for rendering in meters
	// cv::colored_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:157
	// ("cv::colored_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_ColoredKinfu_Params(), &val) };
		ret
	}

	/// distance theshold for ICP in meters
	// cv::colored_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::colored_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_ColoredKinfu_Params(), val) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::colored_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:164
	// ("cv::colored_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_colored_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_ColoredKinfu_Params(), val.as_raw_VectorOfi32()) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::colored_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:170
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
	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:53
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
	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:60
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

// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:19
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
	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:21
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
	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:32
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
	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:42
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
	// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:66
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
	// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:72
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
	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:77
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
	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:82
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

/// Constant methods for [crate::rgbd::Dynafu_DynaFu]
// DynaFu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:46
pub trait Dynafu_DynaFuTraitConst {
	fn as_raw_Dynafu_DynaFu(&self) -> *const c_void;

	/// Get current parameters
	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:53
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
	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:65
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
	// cv::dynafu::DynaFu::render(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:65
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
	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:75
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
	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:83
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
	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:89
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
	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:98
	// ("cv::dynafu::DynaFu::getPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dynafu_DynaFu_getPose_const(self.as_raw_Dynafu_DynaFu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNodesPos()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:110
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

	// marchCubes(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:112
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
	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:95
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
	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:108
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
	// renderSurface(OutputArray, OutputArray, OutputArray, bool)(OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:114
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
	// cv::dynafu::DynaFu::renderSurface(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:114
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

// DynaFu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:46
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
	// create(const Ptr<kinfu::Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:49
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

/// Constant methods for [crate::rgbd::Kinfu_KinFu]
// KinFu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:249
pub trait Kinfu_KinFuTraitConst {
	fn as_raw_Kinfu_KinFu(&self) -> *const c_void;

	/// Get current parameters
	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:256
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
	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:266
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
	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:278
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
	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:288
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
	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:296
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
	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:302
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
	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:311
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
	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:308
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
	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:321
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
// KinFu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:249
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
	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:252
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
// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:74
pub trait Kinfu_ParamsTraitConst {
	fn as_raw_Kinfu_Params(&self) -> *const c_void;

	/// frame size in pixels
	// cv::kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:140
	// ("cv::kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propFrameSize_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Volume kind
	// cv::kinfu::Params::volumeKind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:143
	// ("cv::kinfu::Params::volumeKind", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_kind(&self) -> crate::mod_3d::VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propVolumeKind_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// camera intrinsics
	// cv::kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:146
	// ("cv::kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propIntr_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb camera intrinsics
	// cv::kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:149
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
	// cv::kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propDepthFactor_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:160
	// ("cv::kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:162
	// ("cv::kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:164
	// ("cv::kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:167
	// ("cv::kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_propPyramidLevels_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:173
	// ("cv::kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_dims(&self) -> core::Vec3i {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propVolumeDims_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Size of voxel in meters
	// cv::kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:175
	// ("cv::kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propVoxelSize_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:181
	// ("cv::kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// initial volume pose in meters
	// cv::kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:184
	// ("cv::kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volume_pose(&self) -> core::Matx44f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propVolumePose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:190
	// ("cv::kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_trunc_dist_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:196
	// ("cv::kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_max_weight_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:202
	// ("cv::kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn raycast_step_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propRaycast_step_factor_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// light pose for rendering in meters
	// cv::kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:209
	// ("cv::kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_Params_propLightPose_const(self.as_raw_Kinfu_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance theshold for ICP in meters
	// cv::kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:212
	// ("cv::kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpDistThresh_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:214
	// ("cv::kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpAngleThresh_const(self.as_raw_Kinfu_Params()) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:216
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
	// cv::kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:222
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
	// cv::kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:140
	// ("cv::kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// Volume kind
	// cv::kinfu::Params::setVolumeKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:143
	// ("cv::kinfu::Params::setVolumeKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	#[inline]
	fn set_volume_kind(&mut self, val: crate::mod_3d::VolumeType) {
		let ret = unsafe { sys::cv_kinfu_Params_propVolumeKind_const_VolumeType(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// camera intrinsics
	// cv::kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:146
	// ("cv::kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// rgb camera intrinsics
	// cv::kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:149
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
	// cv::kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:160
	// ("cv::kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:162
	// ("cv::kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:164
	// ("cv::kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:167
	// ("cv::kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	// cv::kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:173
	// ("cv::kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	#[inline]
	fn set_volume_dims(&mut self, val: core::Vec3i) {
		let ret = unsafe { sys::cv_kinfu_Params_propVolumeDims_const_Vec3i(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// Size of voxel in meters
	// cv::kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:175
	// ("cv::kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_voxel_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propVoxelSize_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:181
	// ("cv::kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// initial volume pose in meters
	// cv::kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:184
	// ("cv::kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	#[inline]
	fn set_volume_pose(&mut self, val: core::Matx44f) {
		let ret = unsafe { sys::cv_kinfu_Params_propVolumePose_const_Matx44f(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// distance to truncate in meters
	///
	/// Distances to surface that exceed this value will be truncated to 1.0.
	// cv::kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:190
	// ("cv::kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_trunc_dist(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_trunc_dist_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// max number of frames per voxel
	///
	/// Each voxel keeps running average of distances no longer than this value.
	// cv::kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:196
	// ("cv::kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_tsdf_max_weight(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_Params_propTsdf_max_weight_const_int(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// A length of one raycast step
	///
	/// How much voxel sizes we skip each raycast step
	// cv::kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:202
	// ("cv::kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propRaycast_step_factor_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// light pose for rendering in meters
	// cv::kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:209
	// ("cv::kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_Kinfu_Params(), &val) };
		ret
	}

	/// distance theshold for ICP in meters
	// cv::kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:212
	// ("cv::kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:214
	// ("cv::kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_Kinfu_Params(), val) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:216
	// ("cv::kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_Kinfu_Params(), val.as_raw_VectorOfi32()) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:222
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
	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:108
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
	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:115
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

// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:74
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
	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:76
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
	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:87
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
	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:97
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
	// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:121
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
	// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:127
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
	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:132
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
	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:137
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

/// Constant methods for [crate::rgbd::Kinfu_VolumeParams]
// VolumeParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:19
pub trait Kinfu_VolumeParamsTraitConst {
	fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void;

	/// Kind of Volume
	/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
	// cv::kinfu::VolumeParams::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:24
	// ("cv::kinfu::VolumeParams::kind", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kind(&self) -> crate::mod_3d::VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_propKind_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Resolution of voxel space
	/// Number of voxels in each dimension.
	/// Applicable only for TSDF Volume.
	/// HashTSDF volume only supports equal resolution in all three dimensions
	// cv::kinfu::VolumeParams::resolutionX() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:31
	// ("cv::kinfu::VolumeParams::resolutionX", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resolution_x(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionX_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	// cv::kinfu::VolumeParams::resolutionY() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:32
	// ("cv::kinfu::VolumeParams::resolutionY", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resolution_y(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionY_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	// cv::kinfu::VolumeParams::resolutionZ() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:33
	// ("cv::kinfu::VolumeParams::resolutionZ", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resolution_z(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionZ_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Resolution of volumeUnit in voxel space
	/// Number of voxels in each dimension for volumeUnit
	/// Applicable only for hashTSDF.
	// cv::kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:39
	// ("cv::kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn unit_resolution(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propUnitResolution_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Size of volume in meters
	// cv::kinfu::VolumeParams::volumSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:42
	// ("cv::kinfu::VolumeParams::volumSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volum_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propVolumSize_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Initial pose of the volume in meters, should be 4x4 float or double matrix
	// cv::kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:45
	// ("cv::kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pose(&self) -> core::Matx44f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_kinfu_VolumeParams_propPose_const(self.as_raw_Kinfu_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Length of voxels in meters
	// cv::kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:48
	// ("cv::kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propVoxelSize_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// TSDF truncation distance
	/// Distances greater than value from surface will be truncated to 1.0
	// cv::kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:53
	// ("cv::kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propTsdfTruncDist_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Max number of frames to integrate per voxel
	/// Represents the max number of frames over which a running average
	/// of the TSDF is calculated for a voxel
	// cv::kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:59
	// ("cv::kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propMaxWeight_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Threshold for depth truncation in meters
	/// Truncates the depth greater than threshold to 0
	// cv::kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:64
	// ("cv::kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_trunc_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propDepthTruncThreshold_const(self.as_raw_Kinfu_VolumeParams()) };
		ret
	}

	/// Length of single raycast step
	/// Describes the percentage of voxel length that is skipped per march
	// cv::kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:69
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

	/// Kind of Volume
	/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
	///
	/// ## C++ default parameters
	/// * val: VolumeType::TSDF
	// cv::kinfu::VolumeParams::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:24
	// ("cv::kinfu::VolumeParams::setKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	#[inline]
	fn set_kind(&mut self, val: crate::mod_3d::VolumeType) {
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
	// cv::kinfu::VolumeParams::setResolutionX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:31
	// ("cv::kinfu::VolumeParams::setResolutionX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_resolution_x(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionX_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// ## C++ default parameters
	/// * val: 128
	// cv::kinfu::VolumeParams::setResolutionY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:32
	// ("cv::kinfu::VolumeParams::setResolutionY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_resolution_y(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propResolutionY_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// ## C++ default parameters
	/// * val: 128
	// cv::kinfu::VolumeParams::setResolutionZ(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:33
	// ("cv::kinfu::VolumeParams::setResolutionZ", vec![(pred!(mut, ["val"], ["const int"]), _)]),
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
	// cv::kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:39
	// ("cv::kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_unit_resolution(&mut self, val: i32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propUnitResolution_const_int(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// Size of volume in meters
	///
	/// ## C++ default parameters
	/// * val: 3.f
	// cv::kinfu::VolumeParams::setVolumSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:42
	// ("cv::kinfu::VolumeParams::setVolumSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_volum_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propVolumSize_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

	/// Initial pose of the volume in meters, should be 4x4 float or double matrix
	///
	/// ## C++ default parameters
	/// * val: Affine3f().translate(Vec3f(-volumSize/2.f,-volumSize/2.f,0.5f)).matrix
	// cv::kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:45
	// ("cv::kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	#[inline]
	fn set_pose(&mut self, val: core::Matx44f) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propPose_const_Matx44f(self.as_raw_mut_Kinfu_VolumeParams(), &val) };
		ret
	}

	/// Length of voxels in meters
	///
	/// ## C++ default parameters
	/// * val: volumSize/512.f
	// cv::kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:48
	// ("cv::kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
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
	// cv::kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:53
	// ("cv::kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
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
	// cv::kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:59
	// ("cv::kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
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
	// cv::kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:64
	// ("cv::kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
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
	// cv::kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:69
	// ("cv::kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_kinfu_VolumeParams_propRaycastStepFactor_const_float(self.as_raw_mut_Kinfu_VolumeParams(), val) };
		ret
	}

}

// VolumeParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:19
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

impl Default for Kinfu_VolumeParams {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::rgbd::LargeKinfu]
// LargeKinfu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:192
pub trait LargeKinfuTraitConst {
	fn as_raw_LargeKinfu(&self) -> *const c_void;

	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:198
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

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:200
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

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:201
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

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:203
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

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:205
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

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:207
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

	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:211
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

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:209
	// ("cv::large_kinfu::LargeKinfu::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_LargeKinfu_reset(self.as_raw_mut_LargeKinfu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:213
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
// LargeKinfu /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:192
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
	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:195
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
// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:86
pub trait ParamsTraitConst {
	fn as_raw_Params(&self) -> *const c_void;

	/// frame size in pixels
	// cv::large_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:108
	// ("cv::large_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propFrameSize_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// camera intrinsics
	// cv::large_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:111
	// ("cv::large_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propIntr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// rgb camera intrinsics
	// cv::large_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:114
	// ("cv::large_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rgb_intr(&self) -> core::Matx33f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propRgb_intr_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// pre-scale per 1 meter for input values
	///
	/// Typical values are:
	///      * 5000 per 1 meter for the 16-bit PNG files of TUM database
	///      * 1000 per 1 meter for Kinect 2 device
	///      * 1 per 1 meter for the 32-bit float images in the ROS bag files
	// cv::large_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:123
	// ("cv::large_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propDepthFactor_const(self.as_raw_Params()) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::large_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:126
	// ("cv::large_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_depth(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_depth_const(self.as_raw_Params()) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::large_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:128
	// ("cv::large_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_sigma_spatial(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_spatial_const(self.as_raw_Params()) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::large_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:130
	// ("cv::large_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bilateral_kernel_size(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_kernel_size_const(self.as_raw_Params()) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::large_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:133
	// ("cv::large_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_levels(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propPyramidLevels_const(self.as_raw_Params()) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::large_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:139
	// ("cv::large_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_min_camera_movement(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTsdf_min_camera_movement_const(self.as_raw_Params()) };
		ret
	}

	/// light pose for rendering in meters
	// cv::large_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:142
	// ("cv::large_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn light_pose(&self) -> core::Vec3f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_Params_propLightPose_const(self.as_raw_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// distance theshold for ICP in meters
	// cv::large_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:145
	// ("cv::large_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_dist_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpDistThresh_const(self.as_raw_Params()) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::large_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:147
	// ("cv::large_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_angle_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpAngleThresh_const(self.as_raw_Params()) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::large_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:149
	// ("cv::large_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn icp_iterations(&self) -> core::Vector<i32> {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpIterations_const(self.as_raw_Params()) };
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::large_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:155
	// ("cv::large_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn truncate_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_const(self.as_raw_Params()) };
		ret
	}

	/// Volume parameters
	// cv::large_kinfu::Params::volumeParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:159
	// ("cv::large_kinfu::Params::volumeParams", vec![(pred!(const, [], []), _)]),
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
	// cv::large_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:108
	// ("cv::large_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_frame_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propFrameSize_const_Size(self.as_raw_mut_Params(), &val) };
		ret
	}

	/// camera intrinsics
	// cv::large_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:111
	// ("cv::large_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	#[inline]
	fn set_intr(&mut self, val: core::Matx33f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIntr_const_Matx33f(self.as_raw_mut_Params(), &val) };
		ret
	}

	/// rgb camera intrinsics
	// cv::large_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:114
	// ("cv::large_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
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
	// cv::large_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:123
	// ("cv::large_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_depth_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propDepthFactor_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Depth sigma in meters for bilateral smooth
	// cv::large_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:126
	// ("cv::large_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_depth(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_depth_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Spatial sigma in pixels for bilateral smooth
	// cv::large_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:128
	// ("cv::large_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bilateral_sigma_spatial(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_sigma_spatial_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Kernel size in pixels for bilateral smooth
	// cv::large_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:130
	// ("cv::large_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_bilateral_kernel_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propBilateral_kernel_size_const_int(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Number of pyramid levels for ICP
	// cv::large_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:133
	// ("cv::large_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_levels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propPyramidLevels_const_int(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Minimal camera movement in meters
	///
	/// Integrate new depth frame only if camera movement exceeds this value.
	// cv::large_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:139
	// ("cv::large_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_tsdf_min_camera_movement(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTsdf_min_camera_movement_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// light pose for rendering in meters
	// cv::large_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:142
	// ("cv::large_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	#[inline]
	fn set_light_pose(&mut self, val: core::Vec3f) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propLightPose_const_Vec3f(self.as_raw_mut_Params(), &val) };
		ret
	}

	/// distance theshold for ICP in meters
	// cv::large_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:145
	// ("cv::large_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_dist_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpDistThresh_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// angle threshold for ICP in radians
	// cv::large_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:147
	// ("cv::large_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_icp_angle_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpAngleThresh_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// number of ICP iterations for each pyramid level
	// cv::large_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:149
	// ("cv::large_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	#[inline]
	fn set_icp_iterations(&mut self, val: core::Vector<i32>) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propIcpIterations_const_vectorLintG(self.as_raw_mut_Params(), val.as_raw_VectorOfi32()) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// All depth values beyond this threshold will be set to zero
	// cv::large_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:155
	// ("cv::large_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_truncate_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propTruncateThreshold_const_float(self.as_raw_mut_Params(), val) };
		ret
	}

	/// Volume parameters
	// cv::large_kinfu::Params::setVolumeParams(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:159
	// ("cv::large_kinfu::Params::setVolumeParams", vec![(pred!(mut, ["val"], ["const cv::large_kinfu::VolumeParams"]), _)]),
	#[inline]
	fn set_volume_params(&mut self, val: crate::rgbd::VolumeParams) {
		let ret = unsafe { sys::cv_large_kinfu_Params_propVolumeParams_const_VolumeParams(self.as_raw_mut_Params(), val.as_raw_VolumeParams()) };
		ret
	}

}

// Params /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:86
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
	///
	/// A set of parameters which provides better model quality, can be very slow.
	// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:92
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
	///
	/// A set of parameters which provides better speed, can fail to match frames
	/// in case of rapid sensor motion.
	// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:99
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
	///
	/// A set of parameters suitable for use with HashTSDFVolume
	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:105
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

/// Constant methods for [crate::rgbd::VolumeParams]
// VolumeParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:20
pub trait VolumeParamsTraitConst {
	fn as_raw_VolumeParams(&self) -> *const c_void;

	/// Kind of Volume
	///
	/// Values can be TSDF (single volume) or HASHTSDF (hashtable of volume units)
	// cv::large_kinfu::VolumeParams::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:26
	// ("cv::large_kinfu::VolumeParams::kind", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kind(&self) -> crate::mod_3d::VolumeType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_VolumeParams_propKind_const(self.as_raw_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Resolution of voxel space
	///
	/// Number of voxels in each dimension.
	/// Applicable only for TSDF Volume.
	/// HashTSDF volume only supports equal resolution in all three dimensions
	// cv::large_kinfu::VolumeParams::resolutionX() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:34
	// ("cv::large_kinfu::VolumeParams::resolutionX", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resolution_x(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionX_const(self.as_raw_VolumeParams()) };
		ret
	}

	// cv::large_kinfu::VolumeParams::resolutionY() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:35
	// ("cv::large_kinfu::VolumeParams::resolutionY", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resolution_y(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionY_const(self.as_raw_VolumeParams()) };
		ret
	}

	// cv::large_kinfu::VolumeParams::resolutionZ() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:36
	// ("cv::large_kinfu::VolumeParams::resolutionZ", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resolution_z(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionZ_const(self.as_raw_VolumeParams()) };
		ret
	}

	/// Resolution of volumeUnit in voxel space
	///
	/// Number of voxels in each dimension for volumeUnit
	/// Applicable only for hashTSDF.
	// cv::large_kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:43
	// ("cv::large_kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn unit_resolution(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propUnitResolution_const(self.as_raw_VolumeParams()) };
		ret
	}

	/// Size of volume in meters
	// cv::large_kinfu::VolumeParams::volumSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:46
	// ("cv::large_kinfu::VolumeParams::volumSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn volum_size(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propVolumSize_const(self.as_raw_VolumeParams()) };
		ret
	}

	/// Initial pose of the volume in meters, should be 4x4 float or double matrix
	// cv::large_kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:49
	// ("cv::large_kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pose(&self) -> core::Matx44f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_VolumeParams_propPose_const(self.as_raw_VolumeParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// Length of voxels in meters
	// cv::large_kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:52
	// ("cv::large_kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn voxel_size(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propVoxelSize_const(self.as_raw_VolumeParams()) };
		ret
	}

	/// TSDF truncation distance
	///
	/// Distances greater than value from surface will be truncated to 1.0
	// cv::large_kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn tsdf_trunc_dist(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propTsdfTruncDist_const(self.as_raw_VolumeParams()) };
		ret
	}

	/// Max number of frames to integrate per voxel
	///
	/// Represents the max number of frames over which a running average
	/// of the TSDF is calculated for a voxel
	// cv::large_kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:65
	// ("cv::large_kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_weight(&self) -> i32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propMaxWeight_const(self.as_raw_VolumeParams()) };
		ret
	}

	/// Threshold for depth truncation in meters
	///
	/// Truncates the depth greater than threshold to 0
	// cv::large_kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn depth_trunc_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propDepthTruncThreshold_const(self.as_raw_VolumeParams()) };
		ret
	}

	/// Length of single raycast step
	///
	/// Describes the percentage of voxel length that is skipped per march
	// cv::large_kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:77
	// ("cv::large_kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
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
	// cv::large_kinfu::VolumeParams::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:26
	// ("cv::large_kinfu::VolumeParams::setKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	#[inline]
	fn set_kind(&mut self, val: crate::mod_3d::VolumeType) {
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
	// cv::large_kinfu::VolumeParams::setResolutionX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:34
	// ("cv::large_kinfu::VolumeParams::setResolutionX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_resolution_x(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionX_const_int(self.as_raw_mut_VolumeParams(), val) };
		ret
	}

	/// ## C++ default parameters
	/// * val: 128
	// cv::large_kinfu::VolumeParams::setResolutionY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:35
	// ("cv::large_kinfu::VolumeParams::setResolutionY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_resolution_y(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propResolutionY_const_int(self.as_raw_mut_VolumeParams(), val) };
		ret
	}

	/// ## C++ default parameters
	/// * val: 128
	// cv::large_kinfu::VolumeParams::setResolutionZ(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:36
	// ("cv::large_kinfu::VolumeParams::setResolutionZ", vec![(pred!(mut, ["val"], ["const int"]), _)]),
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
	// cv::large_kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:43
	// ("cv::large_kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_unit_resolution(&mut self, val: i32) {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propUnitResolution_const_int(self.as_raw_mut_VolumeParams(), val) };
		ret
	}

	/// Size of volume in meters
	///
	/// ## C++ default parameters
	/// * val: 3.f
	// cv::large_kinfu::VolumeParams::setVolumSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:46
	// ("cv::large_kinfu::VolumeParams::setVolumSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_volum_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propVolumSize_const_float(self.as_raw_mut_VolumeParams(), val) };
		ret
	}

	/// Initial pose of the volume in meters, should be 4x4 float or double matrix
	///
	/// ## C++ default parameters
	/// * val: Affine3f().translate(Vec3f(-volumSize/2.f,-volumSize/2.f,0.5f)).matrix
	// cv::large_kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:49
	// ("cv::large_kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	#[inline]
	fn set_pose(&mut self, val: core::Matx44f) {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propPose_const_Matx44f(self.as_raw_mut_VolumeParams(), &val) };
		ret
	}

	/// Length of voxels in meters
	///
	/// ## C++ default parameters
	/// * val: volumSize/512.f
	// cv::large_kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:52
	// ("cv::large_kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
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
	// cv::large_kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
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
	// cv::large_kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:65
	// ("cv::large_kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
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
	// cv::large_kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
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
	// cv::large_kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:77
	// ("cv::large_kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_raycast_step_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_large_kinfu_VolumeParams_propRaycastStepFactor_const_float(self.as_raw_mut_VolumeParams(), val) };
		ret
	}

}

// VolumeParams /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:20
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

impl crate::rgbd::VolumeParamsTraitConst for VolumeParams {
	#[inline] fn as_raw_VolumeParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::rgbd::VolumeParamsTrait for VolumeParams {
	#[inline] fn as_raw_mut_VolumeParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VolumeParams, crate::rgbd::VolumeParamsTraitConst, as_raw_VolumeParams, crate::rgbd::VolumeParamsTrait, as_raw_mut_VolumeParams }

impl VolumeParams {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_large_kinfu_VolumeParams_defaultNew_const()) }
	}

	/// Default set of parameters that provide higher quality reconstruction at the cost of slow performance.
	// defaultParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:80
	// ("cv::large_kinfu::VolumeParams::defaultParams", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
	#[inline]
	pub fn default_params(volume_type: crate::mod_3d::VolumeType) -> Result<core::Ptr<crate::rgbd::VolumeParams>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_VolumeParams_defaultParams_VolumeType(volume_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::VolumeParams>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Coarse set of parameters that provides relatively higher performance at the cost of reconstrution quality.
	// coarseParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:83
	// ("cv::large_kinfu::VolumeParams::coarseParams", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
	#[inline]
	pub fn coarse_params(volume_type: crate::mod_3d::VolumeType) -> Result<core::Ptr<crate::rgbd::VolumeParams>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_large_kinfu_VolumeParams_coarseParams_VolumeType(volume_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rgbd::VolumeParams>::opencv_from_extern(ret) };
		Ok(ret)
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

impl Default for VolumeParams {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::rgbd::LineMod_ColorGradient]
// ColorGradient /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:166
pub trait LineMod_ColorGradientTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
	fn as_raw_LineMod_ColorGradient(&self) -> *const c_void;

	// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weak_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:193
	// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn strong_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const(self.as_raw_LineMod_ColorGradient()) };
		ret
	}

	// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:186
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:189
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

	// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_weak_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propWeak_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propNum_features_const_size_t(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:193
	// ("cv::linemod::ColorGradient::setStrong_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_strong_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_ColorGradient_propStrong_threshold_const_float(self.as_raw_mut_LineMod_ColorGradient(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:188
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
// ColorGradient /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:166
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
	// ColorGradient()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:172
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
	// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:182
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

	// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:184
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
// DepthNormal /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:203
pub trait LineMod_DepthNormalTraitConst: crate::rgbd::LineMod_ModalityTraitConst {
	fn as_raw_LineMod_DepthNormal(&self) -> *const c_void;

	// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn distance_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn difference_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> size_t {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:235
	// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn extract_threshold(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const(self.as_raw_LineMod_DepthNormal()) };
		ret
	}

	// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:227
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:230
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

	// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_distance_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDistance_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_difference_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propDifference_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: size_t) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propNum_features_const_size_t(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:235
	// ("cv::linemod::DepthNormal::setExtract_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_extract_threshold(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_DepthNormal_propExtract_threshold_const_int(self.as_raw_mut_LineMod_DepthNormal(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:229
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
// DepthNormal /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:203
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
	// DepthNormal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:209
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
	// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:221
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

	// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:224
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
// Detector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:298
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
	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:330
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
	// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:330
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
	// getModalities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:359
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
	// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:364
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
	// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:369
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
	// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:377
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

	// numTemplates()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:379
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_templates(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numTemplates_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:380
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

	// numClasses()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:381
	// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_classes(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_numClasses_const(self.as_raw_LineMod_Detector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// classIds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:383
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:386
	// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Detector_write_const_FileStorageR(self.as_raw_LineMod_Detector(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:389
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
	// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:393
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
	// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:393
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
	// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:345
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
	// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:345
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
	// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:351
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:385
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
	// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:388
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
	// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:388
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
	// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:391
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
	// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:391
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
// Detector /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:298
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
	// Detector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:304
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
	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:313
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
// Feature /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:26
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
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:36
	// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	pub fn write(self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_write_const_FileStorageR(&self, fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Feature()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:32
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::rgbd::LineMod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:33
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
	#[inline]
	pub fn new(x: i32, y: i32, label: i32) -> Result<crate::rgbd::LineMod_Feature> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Feature_Feature_int_int_int(x, y, label, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:35
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
// Match /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:259
pub trait LineMod_MatchTraitConst {
	fn as_raw_LineMod_Match(&self) -> *const c_void;

	// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn x(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propX_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn y(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propY_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn similarity(&self) -> f32 {
		let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const(self.as_raw_LineMod_Match()) };
		ret
	}

	// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn class_id(&self) -> String {
		let ret = unsafe { sys::cv_linemod_Match_propClass_id_const(self.as_raw_LineMod_Match()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn template_id(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const(self.as_raw_LineMod_Match()) };
		ret
	}

	/// Sort matches with high similarity to the front
	// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:268
	// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	#[inline]
	fn less_than(&self, rhs: &impl crate::rgbd::LineMod_MatchTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_linemod_Match_operatorL_const_const_MatchR(self.as_raw_LineMod_Match(), rhs.as_raw_LineMod_Match(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:277
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

	// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_x(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propX_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_y(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propY_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_similarity(&mut self, val: f32) {
		let ret = unsafe { sys::cv_linemod_Match_propSimilarity_const_float(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

	// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_class_id(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_linemod_Match_propClass_id_const_String(self.as_raw_mut_LineMod_Match(), val.opencv_as_extern()) };
		ret
	}

	// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_template_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Match_propTemplate_id_const_int(self.as_raw_mut_LineMod_Match(), val) };
		ret
	}

}

/// \brief Represents a successful template match.
// Match /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:259
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
	// Match()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:261
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

	// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:265
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
// Modality /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:119
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
	// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:132
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
	// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:132
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

	// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:138
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

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:141
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:140
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
// Modality /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:119
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
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:150
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
	// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:155
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
// QuantizedPyramid /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:55
pub trait LineMod_QuantizedPyramidTraitConst {
	fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void;

	/// \brief Compute quantized image at current pyramid level for online detection.
	///
	/// \param[out] dst The destination 8-bit image. For each pixel at most one bit is set,
	///                representing its classification.
	// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:67
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
	// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:74
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
	// pyrDown()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:81
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
// QuantizedPyramid /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:55
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
// Template /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:41
pub trait LineMod_TemplateTraitConst {
	fn as_raw_LineMod_Template(&self) -> *const c_void;

	// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn width(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propWidth_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn height(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propHeight_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pyramid_level(&self) -> i32 {
		let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const(self.as_raw_LineMod_Template()) };
		ret
	}

	// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn features(&self) -> core::Vector<crate::rgbd::LineMod_Feature> {
		let ret = unsafe { sys::cv_linemod_Template_propFeatures_const(self.as_raw_LineMod_Template()) };
		let ret = unsafe { core::Vector::<crate::rgbd::LineMod_Feature>::opencv_from_extern(ret) };
		ret
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:49
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

	// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_width(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propWidth_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_height(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propHeight_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_pyramid_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_linemod_Template_propPyramid_level_const_int(self.as_raw_mut_LineMod_Template(), val) };
		ret
	}

	// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
	#[inline]
	fn set_features(&mut self, val: core::Vector<crate::rgbd::LineMod_Feature>) {
		let ret = unsafe { sys::cv_linemod_Template_propFeatures_const_vectorLFeatureG(self.as_raw_mut_LineMod_Template(), val.as_raw_VectorOfLineMod_Feature()) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:48
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

// Template /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:41
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
