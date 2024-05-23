//! # Video Stabilization
//!
//! The video stabilization module contains a set of functions and classes that can be used to solve the
//! problem of video stabilization. There are a few methods implemented, most of them are described in
//! the papers [OF06](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_OF06) and [G11](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_G11) . However, there are some extensions and deviations from the original
//! paper methods.
//!
//! ### References
//!
//! 1. "Full-Frame Video Stabilization with Motion Inpainting"
//!    Yasuyuki Matsushita, Eyal Ofek, Weina Ge, Xiaoou Tang, Senior Member, and Heung-Yeung Shum
//! 2. "Auto-Directed Video Stabilization with Robust L1 Optimal Camera Paths"
//!    Matthias Grundmann, Vivek Kwatra, Irfan Essa
//!    # Global Motion Estimation
//!
//!    The video stabilization module contains a set of functions and classes for global motion estimation
//!    between point clouds or between images. In the last case features are extracted and matched
//!    internally. For the sake of convenience the motion estimation functions are wrapped into classes.
//!    Both the functions and the classes are available.
//!
//!    # Fast Marching Method
//!
//!    The Fast Marching Method [Telea04](https://docs.opencv.org/4.11.0/d0/de3/citelist.html#CITEREF_Telea04) is used in of the video stabilization routines to do motion and
//!    color inpainting. The method is implemented is a flexible way and it's made public for other users.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{ColorAverageInpainterTrait, ColorAverageInpainterTraitConst, ColorInpainterTrait, ColorInpainterTraitConst, ConsistentMosaicInpainterTrait, ConsistentMosaicInpainterTraitConst, DeblurerBaseTrait, DeblurerBaseTraitConst, DensePyrLkOptFlowEstimatorGpuTrait, DensePyrLkOptFlowEstimatorGpuTraitConst, FastMarchingMethodTrait, FastMarchingMethodTraitConst, FromFileMotionReaderTrait, FromFileMotionReaderTraitConst, GaussianMotionFilterTrait, GaussianMotionFilterTraitConst, IDenseOptFlowEstimatorTrait, IDenseOptFlowEstimatorTraitConst, IFrameSourceTrait, IFrameSourceTraitConst, ILogTrait, ILogTraitConst, IMotionStabilizerTrait, IMotionStabilizerTraitConst, IOutlierRejectorTrait, IOutlierRejectorTraitConst, ISparseOptFlowEstimatorTrait, ISparseOptFlowEstimatorTraitConst, ImageMotionEstimatorBaseTrait, ImageMotionEstimatorBaseTraitConst, InpainterBaseTrait, InpainterBaseTraitConst, InpaintingPipelineTrait, InpaintingPipelineTraitConst, KeypointBasedMotionEstimatorGpuTrait, KeypointBasedMotionEstimatorGpuTraitConst, KeypointBasedMotionEstimatorTrait, KeypointBasedMotionEstimatorTraitConst, LogToStdoutTrait, LogToStdoutTraitConst, LpMotionStabilizerTrait, LpMotionStabilizerTraitConst, MaskFrameSourceTrait, MaskFrameSourceTraitConst, MoreAccurateMotionWobbleSuppressorBaseTrait, MoreAccurateMotionWobbleSuppressorBaseTraitConst, MoreAccurateMotionWobbleSuppressorGpuTrait, MoreAccurateMotionWobbleSuppressorGpuTraitConst, MoreAccurateMotionWobbleSuppressorTrait, MoreAccurateMotionWobbleSuppressorTraitConst, MotionEstimatorBaseTrait, MotionEstimatorBaseTraitConst, MotionEstimatorL1Trait, MotionEstimatorL1TraitConst, MotionEstimatorRansacL2Trait, MotionEstimatorRansacL2TraitConst, MotionFilterBaseTrait, MotionFilterBaseTraitConst, MotionInpainterTrait, MotionInpainterTraitConst, MotionStabilizationPipelineTrait, MotionStabilizationPipelineTraitConst, NullDeblurerTrait, NullDeblurerTraitConst, NullFrameSourceTrait, NullFrameSourceTraitConst, NullInpainterTrait, NullInpainterTraitConst, NullLogTrait, NullLogTraitConst, NullOutlierRejectorTrait, NullOutlierRejectorTraitConst, NullWobbleSuppressorTrait, NullWobbleSuppressorTraitConst, OnePassStabilizerTrait, OnePassStabilizerTraitConst, PyrLkOptFlowEstimatorBaseTrait, PyrLkOptFlowEstimatorBaseTraitConst, RansacParamsTrait, RansacParamsTraitConst, SparsePyrLkOptFlowEstimatorGpuTrait, SparsePyrLkOptFlowEstimatorGpuTraitConst, SparsePyrLkOptFlowEstimatorTrait, SparsePyrLkOptFlowEstimatorTraitConst, StabilizerBaseTrait, StabilizerBaseTraitConst, ToFileMotionWriterTrait, ToFileMotionWriterTraitConst, TranslationBasedLocalOutlierRejectorTrait, TranslationBasedLocalOutlierRejectorTraitConst, TwoPassStabilizerTrait, TwoPassStabilizerTraitConst, VideoFileSourceTrait, VideoFileSourceTraitConst, WeightingDeblurerTrait, WeightingDeblurerTraitConst, WobbleSuppressorBaseTrait, WobbleSuppressorBaseTraitConst};
}

// MM_AFFINE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:66
pub const MM_AFFINE: i32 = 5;
// MM_HOMOGRAPHY /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:67
pub const MM_HOMOGRAPHY: i32 = 6;
// MM_RIGID /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:64
pub const MM_RIGID: i32 = 3;
// MM_ROTATION /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:63
pub const MM_ROTATION: i32 = 2;
// MM_SIMILARITY /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:65
pub const MM_SIMILARITY: i32 = 4;
// MM_TRANSLATION /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:61
pub const MM_TRANSLATION: i32 = 0;
// MM_TRANSLATION_AND_SCALE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:62
pub const MM_TRANSLATION_AND_SCALE: i32 = 1;
// MM_UNKNOWN /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:68
pub const MM_UNKNOWN: i32 = 7;
/// Describes motion model between two point clouds.
// MotionModel /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:59
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MotionModel {
	MM_TRANSLATION = 0,
	MM_TRANSLATION_AND_SCALE = 1,
	MM_ROTATION = 2,
	MM_RIGID = 3,
	MM_SIMILARITY = 4,
	MM_AFFINE = 5,
	MM_HOMOGRAPHY = 6,
	MM_UNKNOWN = 7,
}

impl TryFrom<i32> for MotionModel {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::MM_TRANSLATION),
			1 => Ok(Self::MM_TRANSLATION_AND_SCALE),
			2 => Ok(Self::MM_ROTATION),
			3 => Ok(Self::MM_RIGID),
			4 => Ok(Self::MM_SIMILARITY),
			5 => Ok(Self::MM_AFFINE),
			6 => Ok(Self::MM_HOMOGRAPHY),
			7 => Ok(Self::MM_UNKNOWN),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::videostab::MotionModel"))),
		}
	}
}

opencv_type_enum! { crate::videostab::MotionModel }

// calcBlurriness(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:57
// ("cv::videostab::calcBlurriness", vec![(pred!(mut, ["frame"], ["const cv::Mat*"]), _)]),
#[inline]
pub fn calc_blurriness(frame: &impl core::MatTraitConst) -> Result<f32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_calcBlurriness_const_MatR(frame.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// calcFlowMask(const Mat &, const Mat &, const Mat &, float, const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:199
// ("cv::videostab::calcFlowMask", vec![(pred!(mut, ["flowX", "flowY", "errors", "maxError", "mask0", "mask1", "flowMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "float", "const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
#[inline]
pub fn calc_flow_mask(flow_x: &impl core::MatTraitConst, flow_y: &impl core::MatTraitConst, errors: &impl core::MatTraitConst, max_error: f32, mask0: &impl core::MatTraitConst, mask1: &impl core::MatTraitConst, flow_mask: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_calcFlowMask_const_MatR_const_MatR_const_MatR_float_const_MatR_const_MatR_MatR(flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), errors.as_raw_Mat(), max_error, mask0.as_raw_Mat(), mask1.as_raw_Mat(), flow_mask.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// completeFrameAccordingToFlow(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, float, Mat &, Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:203
// ("cv::videostab::completeFrameAccordingToFlow", vec![(pred!(mut, ["flowMask", "flowX", "flowY", "frame1", "mask1", "distThresh", "frame0", "mask0"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "float", "cv::Mat*", "cv::Mat*"]), _)]),
#[inline]
pub fn complete_frame_according_to_flow(flow_mask: &impl core::MatTraitConst, flow_x: &impl core::MatTraitConst, flow_y: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst, mask1: &impl core::MatTraitConst, dist_thresh: f32, frame0: &mut impl core::MatTrait, mask0: &mut impl core::MatTrait) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_completeFrameAccordingToFlow_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_float_MatR_MatR(flow_mask.as_raw_Mat(), flow_x.as_raw_Mat(), flow_y.as_raw_Mat(), frame1.as_raw_Mat(), mask1.as_raw_Mat(), dist_thresh, frame0.as_raw_mut_Mat(), mask0.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// ensureInclusionConstraint(const Mat &, Size, float)(TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:165
// ("cv::videostab::ensureInclusionConstraint", vec![(pred!(mut, ["M", "size", "trimRatio"], ["const cv::Mat*", "cv::Size", "float"]), _)]),
#[inline]
pub fn ensure_inclusion_constraint(m: &impl core::MatTraitConst, size: core::Size, trim_ratio: f32) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_ensureInclusionConstraint_const_MatR_Size_float(m.as_raw_Mat(), &size, trim_ratio, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Estimates best global motion between two 2D point clouds in the least-squares sense.
///
///
/// Note: Works in-place and changes input point arrays.
///
/// ## Parameters
/// * points0: Source set of 2D points (32F).
/// * points1: Destination set of 2D points (32F).
/// * model: Motion model (up to MM_AFFINE).
/// * rmse: Final root-mean-square error.
/// ## Returns
/// 3x3 2D transformation matrix (32F).
///
/// ## Note
/// This alternative version of [estimate_global_motion_least_squares] function uses the following default values for its arguments:
/// * model: MM_AFFINE
/// * rmse: 0
// cv::videostab::estimateGlobalMotionLeastSquares(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:77
// ("cv::videostab::estimateGlobalMotionLeastSquares", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
#[inline]
pub fn estimate_global_motion_least_squares_def(points0: &mut impl ToInputOutputArray, points1: &mut impl ToInputOutputArray) -> Result<core::Mat> {
	input_output_array_arg!(points0);
	input_output_array_arg!(points1);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR(points0.as_raw__InputOutputArray(), points1.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Estimates best global motion between two 2D point clouds in the least-squares sense.
///
///
/// Note: Works in-place and changes input point arrays.
///
/// ## Parameters
/// * points0: Source set of 2D points (32F).
/// * points1: Destination set of 2D points (32F).
/// * model: Motion model (up to MM_AFFINE).
/// * rmse: Final root-mean-square error.
/// ## Returns
/// 3x3 2D transformation matrix (32F).
///
/// ## C++ default parameters
/// * model: MM_AFFINE
/// * rmse: 0
// estimateGlobalMotionLeastSquares(InputOutputArray, InputOutputArray, int, float *)(InputOutputArray, InputOutputArray, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:77
// ("cv::videostab::estimateGlobalMotionLeastSquares", vec![(pred!(mut, ["points0", "points1", "model", "rmse"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "int", "float*"]), _)]),
#[inline]
pub fn estimate_global_motion_least_squares(points0: &mut impl ToInputOutputArray, points1: &mut impl ToInputOutputArray, model: i32, rmse: &mut f32) -> Result<core::Mat> {
	input_output_array_arg!(points0);
	input_output_array_arg!(points1);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR_int_floatX(points0.as_raw__InputOutputArray(), points1.as_raw__InputOutputArray(), model, rmse, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Estimates best global motion between two 2D point clouds robustly (using RANSAC method).
///
/// ## Parameters
/// * points0: Source set of 2D points (32F).
/// * points1: Destination set of 2D points (32F).
/// * model: Motion model. See cv::videostab::MotionModel.
/// * params: RANSAC method parameters. See videostab::RansacParams.
/// * rmse: Final root-mean-square error.
/// * ninliers: Final number of inliers.
///
/// ## Note
/// This alternative version of [estimate_global_motion_ransac] function uses the following default values for its arguments:
/// * model: MM_AFFINE
/// * params: RansacParams::default2dMotion(MM_AFFINE)
/// * rmse: 0
/// * ninliers: 0
// cv::videostab::estimateGlobalMotionRansac(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:90
// ("cv::videostab::estimateGlobalMotionRansac", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn estimate_global_motion_ransac_def(points0: &impl ToInputArray, points1: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(points0);
	input_array_arg!(points1);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR(points0.as_raw__InputArray(), points1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Estimates best global motion between two 2D point clouds robustly (using RANSAC method).
///
/// ## Parameters
/// * points0: Source set of 2D points (32F).
/// * points1: Destination set of 2D points (32F).
/// * model: Motion model. See cv::videostab::MotionModel.
/// * params: RANSAC method parameters. See videostab::RansacParams.
/// * rmse: Final root-mean-square error.
/// * ninliers: Final number of inliers.
///
/// ## C++ default parameters
/// * model: MM_AFFINE
/// * params: RansacParams::default2dMotion(MM_AFFINE)
/// * rmse: 0
/// * ninliers: 0
// estimateGlobalMotionRansac(InputArray, InputArray, int, const RansacParams &, float *, int *)(InputArray, InputArray, Primitive, TraitClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:90
// ("cv::videostab::estimateGlobalMotionRansac", vec![(pred!(mut, ["points0", "points1", "model", "params", "rmse", "ninliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::videostab::RansacParams*", "float*", "int*"]), _)]),
#[inline]
pub fn estimate_global_motion_ransac(points0: &impl ToInputArray, points1: &impl ToInputArray, model: i32, params: &impl crate::videostab::RansacParamsTraitConst, rmse: &mut f32, ninliers: &mut i32) -> Result<core::Mat> {
	input_array_arg!(points0);
	input_array_arg!(points1);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR_int_const_RansacParamsR_floatX_intX(points0.as_raw__InputArray(), points1.as_raw__InputArray(), model, params.as_raw_RansacParams(), rmse, ninliers, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// estimateOptimalTrimRatio(const Mat &, Size)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:167
// ("cv::videostab::estimateOptimalTrimRatio", vec![(pred!(mut, ["M", "size"], ["const cv::Mat*", "cv::Size"]), _)]),
#[inline]
pub fn estimate_optimal_trim_ratio(m: &impl core::MatTraitConst, size: core::Size) -> Result<f32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_estimateOptimalTrimRatio_const_MatR_Size(m.as_raw_Mat(), &size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes motion between two frames assuming that all the intermediate motions are known.
///
/// ## Parameters
/// * from: Source frame index.
/// * to: Destination frame index.
/// * motions: Pair-wise motions. motions[i] denotes motion from the frame i to the frame i+1
/// ## Returns
/// Motion from the Source frame to the Destination frame.
// getMotion(int, int, const std::vector<Mat> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:304
// ("cv::videostab::getMotion", vec![(pred!(mut, ["from", "to", "motions"], ["int", "int", "const std::vector<cv::Mat>*"]), _)]),
#[inline]
pub fn get_motion(from: i32, to: i32, motions: &core::Vector<core::Mat>) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_videostab_getMotion_int_int_const_vectorLMatGR(from, to, motions.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Constant methods for [crate::videostab::ColorAverageInpainter]
// ColorAverageInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:174
pub trait ColorAverageInpainterTraitConst: crate::videostab::InpainterBaseTraitConst {
	fn as_raw_ColorAverageInpainter(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::ColorAverageInpainter]
pub trait ColorAverageInpainterTrait: crate::videostab::ColorAverageInpainterTraitConst + crate::videostab::InpainterBaseTrait {
	fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void;

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:177
	// ("cv::videostab::ColorAverageInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn inpaint(&mut self, idx: i32, frame: &mut impl core::MatTrait, mask: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ColorAverageInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_ColorAverageInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// ColorAverageInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:174
pub struct ColorAverageInpainter {
	ptr: *mut c_void,
}

opencv_type_boxed! { ColorAverageInpainter }

impl Drop for ColorAverageInpainter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_ColorAverageInpainter_delete(self.as_raw_mut_ColorAverageInpainter()) };
	}
}

unsafe impl Send for ColorAverageInpainter {}

impl crate::videostab::InpainterBaseTraitConst for ColorAverageInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpainterBaseTrait for ColorAverageInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ColorAverageInpainter, crate::videostab::InpainterBaseTraitConst, as_raw_InpainterBase, crate::videostab::InpainterBaseTrait, as_raw_mut_InpainterBase }

impl crate::videostab::ColorAverageInpainterTraitConst for ColorAverageInpainter {
	#[inline] fn as_raw_ColorAverageInpainter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ColorAverageInpainterTrait for ColorAverageInpainter {
	#[inline] fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ColorAverageInpainter, crate::videostab::ColorAverageInpainterTraitConst, as_raw_ColorAverageInpainter, crate::videostab::ColorAverageInpainterTrait, as_raw_mut_ColorAverageInpainter }

impl ColorAverageInpainter {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_ColorAverageInpainter_defaultNew_const()) }
	}

}

boxed_cast_base! { ColorAverageInpainter, crate::videostab::InpainterBase, cv_videostab_ColorAverageInpainter_to_InpainterBase }

impl std::fmt::Debug for ColorAverageInpainter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ColorAverageInpainter")
			.finish()
	}
}

impl Default for ColorAverageInpainter {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::ColorInpainter]
// ColorInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:183
pub trait ColorInpainterTraitConst: crate::videostab::InpainterBaseTraitConst {
	fn as_raw_ColorInpainter(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::ColorInpainter]
pub trait ColorInpainterTrait: crate::videostab::ColorInpainterTraitConst + crate::videostab::InpainterBaseTrait {
	fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void;

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:188
	// ("cv::videostab::ColorInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn inpaint(&mut self, idx: i32, frame: &mut impl core::MatTrait, mask: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ColorInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_ColorInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// ColorInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:183
pub struct ColorInpainter {
	ptr: *mut c_void,
}

opencv_type_boxed! { ColorInpainter }

impl Drop for ColorInpainter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_ColorInpainter_delete(self.as_raw_mut_ColorInpainter()) };
	}
}

unsafe impl Send for ColorInpainter {}

impl crate::videostab::InpainterBaseTraitConst for ColorInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpainterBaseTrait for ColorInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ColorInpainter, crate::videostab::InpainterBaseTraitConst, as_raw_InpainterBase, crate::videostab::InpainterBaseTrait, as_raw_mut_InpainterBase }

impl crate::videostab::ColorInpainterTraitConst for ColorInpainter {
	#[inline] fn as_raw_ColorInpainter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ColorInpainterTrait for ColorInpainter {
	#[inline] fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ColorInpainter, crate::videostab::ColorInpainterTraitConst, as_raw_ColorInpainter, crate::videostab::ColorInpainterTrait, as_raw_mut_ColorInpainter }

impl ColorInpainter {
	/// ## C++ default parameters
	/// * method: INPAINT_TELEA
	/// * radius: 2.
	// ColorInpainter(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:186
	// ("cv::videostab::ColorInpainter::ColorInpainter", vec![(pred!(mut, ["method", "radius"], ["int", "double"]), _)]),
	#[inline]
	pub fn new(method: i32, radius: f64) -> Result<crate::videostab::ColorInpainter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ColorInpainter_ColorInpainter_int_double(method, radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::ColorInpainter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * method: INPAINT_TELEA
	/// * radius: 2.
	// cv::videostab::ColorInpainter::ColorInpainter() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:186
	// ("cv::videostab::ColorInpainter::ColorInpainter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::videostab::ColorInpainter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ColorInpainter_ColorInpainter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::ColorInpainter::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ColorInpainter, crate::videostab::InpainterBase, cv_videostab_ColorInpainter_to_InpainterBase }

impl std::fmt::Debug for ColorInpainter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ColorInpainter")
			.finish()
	}
}

/// Constant methods for [crate::videostab::ConsistentMosaicInpainter]
// ConsistentMosaicInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:127
pub trait ConsistentMosaicInpainterTraitConst: crate::videostab::InpainterBaseTraitConst {
	fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void;

	// stdevThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:133
	// ("cv::videostab::ConsistentMosaicInpainter::stdevThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn stdev_thresh(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(self.as_raw_ConsistentMosaicInpainter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::ConsistentMosaicInpainter]
pub trait ConsistentMosaicInpainterTrait: crate::videostab::ConsistentMosaicInpainterTraitConst + crate::videostab::InpainterBaseTrait {
	fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void;

	// setStdevThresh(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:132
	// ("cv::videostab::ConsistentMosaicInpainter::setStdevThresh", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_stdev_thresh(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(self.as_raw_mut_ConsistentMosaicInpainter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:135
	// ("cv::videostab::ConsistentMosaicInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn inpaint(&mut self, idx: i32, frame: &mut impl core::MatTrait, mask: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_ConsistentMosaicInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// ConsistentMosaicInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:127
pub struct ConsistentMosaicInpainter {
	ptr: *mut c_void,
}

opencv_type_boxed! { ConsistentMosaicInpainter }

impl Drop for ConsistentMosaicInpainter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_delete(self.as_raw_mut_ConsistentMosaicInpainter()) };
	}
}

unsafe impl Send for ConsistentMosaicInpainter {}

impl crate::videostab::InpainterBaseTraitConst for ConsistentMosaicInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpainterBaseTrait for ConsistentMosaicInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConsistentMosaicInpainter, crate::videostab::InpainterBaseTraitConst, as_raw_InpainterBase, crate::videostab::InpainterBaseTrait, as_raw_mut_InpainterBase }

impl crate::videostab::ConsistentMosaicInpainterTraitConst for ConsistentMosaicInpainter {
	#[inline] fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ConsistentMosaicInpainterTrait for ConsistentMosaicInpainter {
	#[inline] fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConsistentMosaicInpainter, crate::videostab::ConsistentMosaicInpainterTraitConst, as_raw_ConsistentMosaicInpainter, crate::videostab::ConsistentMosaicInpainterTrait, as_raw_mut_ConsistentMosaicInpainter }

impl ConsistentMosaicInpainter {
	// ConsistentMosaicInpainter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:130
	// ("cv::videostab::ConsistentMosaicInpainter::ConsistentMosaicInpainter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::ConsistentMosaicInpainter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::ConsistentMosaicInpainter::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ConsistentMosaicInpainter, crate::videostab::InpainterBase, cv_videostab_ConsistentMosaicInpainter_to_InpainterBase }

impl std::fmt::Debug for ConsistentMosaicInpainter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ConsistentMosaicInpainter")
			.finish()
	}
}

/// Constant methods for [crate::videostab::DeblurerBase]
// DeblurerBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:59
pub trait DeblurerBaseTraitConst {
	fn as_raw_DeblurerBase(&self) -> *const c_void;

	// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:67
	// ("cv::videostab::DeblurerBase::radius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_radius_const(self.as_raw_DeblurerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// frames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:75
	// ("cv::videostab::DeblurerBase::frames", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frames(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_frames_const(self.as_raw_DeblurerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// motions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:78
	// ("cv::videostab::DeblurerBase::motions", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motions(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_motions_const(self.as_raw_DeblurerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// blurrinessRates()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:81
	// ("cv::videostab::DeblurerBase::blurrinessRates", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn blurriness_rates(&self) -> Result<core::Vector<f32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_blurrinessRates_const(self.as_raw_DeblurerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::DeblurerBase]
pub trait DeblurerBaseTrait: crate::videostab::DeblurerBaseTraitConst {
	fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void;

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:66
	// ("cv::videostab::DeblurerBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_radius(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_setRadius_int(self.as_raw_mut_DeblurerBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// deblur(int, Mat &, const Range &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:69
	// ("cv::videostab::DeblurerBase::deblur", vec![(pred!(mut, ["idx", "frame", "range"], ["int", "cv::Mat*", "const cv::Range*"]), _)]),
	#[inline]
	fn deblur(&mut self, idx: i32, frame: &mut impl core::MatTrait, range: &impl core::RangeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_deblur_int_MatR_const_RangeR(self.as_raw_mut_DeblurerBase(), idx, frame.as_raw_mut_Mat(), range.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:74
	// ("cv::videostab::DeblurerBase::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_frames(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_setFrames_const_vectorLMatGR(self.as_raw_mut_DeblurerBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:77
	// ("cv::videostab::DeblurerBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_motions(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_setMotions_const_vectorLMatGR(self.as_raw_mut_DeblurerBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBlurrinessRates(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:80
	// ("cv::videostab::DeblurerBase::setBlurrinessRates", vec![(pred!(mut, ["val"], ["const std::vector<float>*"]), _)]),
	#[inline]
	fn set_blurriness_rates(&mut self, val: &core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DeblurerBase_setBlurrinessRates_const_vectorLfloatGR(self.as_raw_mut_DeblurerBase(), val.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DeblurerBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:59
pub struct DeblurerBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { DeblurerBase }

impl Drop for DeblurerBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_DeblurerBase_delete(self.as_raw_mut_DeblurerBase()) };
	}
}

unsafe impl Send for DeblurerBase {}

impl crate::videostab::DeblurerBaseTraitConst for DeblurerBase {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::DeblurerBaseTrait for DeblurerBase {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DeblurerBase, crate::videostab::DeblurerBaseTraitConst, as_raw_DeblurerBase, crate::videostab::DeblurerBaseTrait, as_raw_mut_DeblurerBase }

impl DeblurerBase {
}

boxed_cast_descendant! { DeblurerBase, crate::videostab::NullDeblurer, cv_videostab_DeblurerBase_to_NullDeblurer }

boxed_cast_descendant! { DeblurerBase, crate::videostab::WeightingDeblurer, cv_videostab_DeblurerBase_to_WeightingDeblurer }

impl std::fmt::Debug for DeblurerBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DeblurerBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::DensePyrLkOptFlowEstimatorGpu]
// DensePyrLkOptFlowEstimatorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:128
pub trait DensePyrLkOptFlowEstimatorGpuTraitConst: crate::videostab::IDenseOptFlowEstimatorTraitConst + crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst {
	fn as_raw_DensePyrLkOptFlowEstimatorGpu(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::DensePyrLkOptFlowEstimatorGpu]
pub trait DensePyrLkOptFlowEstimatorGpuTrait: crate::videostab::DensePyrLkOptFlowEstimatorGpuTraitConst + crate::videostab::IDenseOptFlowEstimatorTrait + crate::videostab::PyrLkOptFlowEstimatorBaseTrait {
	fn as_raw_mut_DensePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void;

	// run(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:134
	// ("cv::videostab::DensePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "flowX", "flowY", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, flow_x: &mut impl ToInputOutputArray, flow_y: &mut impl ToInputOutputArray, errors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_output_array_arg!(flow_x);
		input_output_array_arg!(flow_y);
		output_array_arg!(errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DensePyrLkOptFlowEstimatorGpu_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(self.as_raw_mut_DensePyrLkOptFlowEstimatorGpu(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), flow_x.as_raw__InputOutputArray(), flow_y.as_raw__InputOutputArray(), errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// DensePyrLkOptFlowEstimatorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:128
pub struct DensePyrLkOptFlowEstimatorGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { DensePyrLkOptFlowEstimatorGpu }

impl Drop for DensePyrLkOptFlowEstimatorGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_DensePyrLkOptFlowEstimatorGpu_delete(self.as_raw_mut_DensePyrLkOptFlowEstimatorGpu()) };
	}
}

unsafe impl Send for DensePyrLkOptFlowEstimatorGpu {}

impl crate::videostab::IDenseOptFlowEstimatorTraitConst for DensePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IDenseOptFlowEstimatorTrait for DensePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DensePyrLkOptFlowEstimatorGpu, crate::videostab::IDenseOptFlowEstimatorTraitConst, as_raw_IDenseOptFlowEstimator, crate::videostab::IDenseOptFlowEstimatorTrait, as_raw_mut_IDenseOptFlowEstimator }

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for DensePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for DensePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DensePyrLkOptFlowEstimatorGpu, crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst, as_raw_PyrLkOptFlowEstimatorBase, crate::videostab::PyrLkOptFlowEstimatorBaseTrait, as_raw_mut_PyrLkOptFlowEstimatorBase }

impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTraitConst for DensePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_DensePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTrait for DensePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_mut_DensePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DensePyrLkOptFlowEstimatorGpu, crate::videostab::DensePyrLkOptFlowEstimatorGpuTraitConst, as_raw_DensePyrLkOptFlowEstimatorGpu, crate::videostab::DensePyrLkOptFlowEstimatorGpuTrait, as_raw_mut_DensePyrLkOptFlowEstimatorGpu }

impl DensePyrLkOptFlowEstimatorGpu {
	// DensePyrLkOptFlowEstimatorGpu()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:132
	// ("cv::videostab::DensePyrLkOptFlowEstimatorGpu::DensePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_DensePyrLkOptFlowEstimatorGpu_DensePyrLkOptFlowEstimatorGpu(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::DensePyrLkOptFlowEstimatorGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DensePyrLkOptFlowEstimatorGpu, crate::videostab::IDenseOptFlowEstimator, cv_videostab_DensePyrLkOptFlowEstimatorGpu_to_IDenseOptFlowEstimator }

boxed_cast_base! { DensePyrLkOptFlowEstimatorGpu, crate::videostab::PyrLkOptFlowEstimatorBase, cv_videostab_DensePyrLkOptFlowEstimatorGpu_to_PyrLkOptFlowEstimatorBase }

impl std::fmt::Debug for DensePyrLkOptFlowEstimatorGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DensePyrLkOptFlowEstimatorGpu")
			.finish()
	}
}

/// Constant methods for [crate::videostab::FastMarchingMethod]
// FastMarchingMethod /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/fast_marching.hpp:63
pub trait FastMarchingMethodTraitConst {
	fn as_raw_FastMarchingMethod(&self) -> *const c_void;

	/// ## Returns
	/// Distance map that's created during working of the method.
	// distanceMap()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/fast_marching.hpp:81
	// ("cv::videostab::FastMarchingMethod::distanceMap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn distance_map(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_FastMarchingMethod_distanceMap_const(self.as_raw_FastMarchingMethod(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::FastMarchingMethod]
pub trait FastMarchingMethodTrait: crate::videostab::FastMarchingMethodTraitConst {
	fn as_raw_mut_FastMarchingMethod(&mut self) -> *mut c_void;

}

/// Describes the Fast Marching Method implementation.
///
/// See <http://iwi.eldoc.ub.rug.nl/FILES/root/2004/JGraphToolsTelea/2004JGraphToolsTelea.pdf>
// FastMarchingMethod /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/fast_marching.hpp:63
pub struct FastMarchingMethod {
	ptr: *mut c_void,
}

opencv_type_boxed! { FastMarchingMethod }

impl Drop for FastMarchingMethod {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_FastMarchingMethod_delete(self.as_raw_mut_FastMarchingMethod()) };
	}
}

unsafe impl Send for FastMarchingMethod {}

impl crate::videostab::FastMarchingMethodTraitConst for FastMarchingMethod {
	#[inline] fn as_raw_FastMarchingMethod(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::FastMarchingMethodTrait for FastMarchingMethod {
	#[inline] fn as_raw_mut_FastMarchingMethod(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FastMarchingMethod, crate::videostab::FastMarchingMethodTraitConst, as_raw_FastMarchingMethod, crate::videostab::FastMarchingMethodTrait, as_raw_mut_FastMarchingMethod }

impl FastMarchingMethod {
	// FastMarchingMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/fast_marching.hpp:66
	// ("cv::videostab::FastMarchingMethod::FastMarchingMethod", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::FastMarchingMethod> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_FastMarchingMethod_FastMarchingMethod(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::FastMarchingMethod::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for FastMarchingMethod {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FastMarchingMethod")
			.finish()
	}
}

/// Constant methods for [crate::videostab::FromFileMotionReader]
// FromFileMotionReader /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:198
pub trait FromFileMotionReaderTraitConst: crate::videostab::ImageMotionEstimatorBaseTraitConst {
	fn as_raw_FromFileMotionReader(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::FromFileMotionReader]
pub trait FromFileMotionReaderTrait: crate::videostab::FromFileMotionReaderTraitConst + crate::videostab::ImageMotionEstimatorBaseTrait {
	fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:203
	// ("cv::videostab::FromFileMotionReader::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst, ok: &mut bool) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_FromFileMotionReader(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [FromFileMotionReaderTrait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::FromFileMotionReader::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:203
	// ("cv::videostab::FromFileMotionReader::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR(self.as_raw_mut_FromFileMotionReader(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// FromFileMotionReader /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:198
pub struct FromFileMotionReader {
	ptr: *mut c_void,
}

opencv_type_boxed! { FromFileMotionReader }

impl Drop for FromFileMotionReader {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_FromFileMotionReader_delete(self.as_raw_mut_FromFileMotionReader()) };
	}
}

unsafe impl Send for FromFileMotionReader {}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for FromFileMotionReader {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for FromFileMotionReader {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FromFileMotionReader, crate::videostab::ImageMotionEstimatorBaseTraitConst, as_raw_ImageMotionEstimatorBase, crate::videostab::ImageMotionEstimatorBaseTrait, as_raw_mut_ImageMotionEstimatorBase }

impl crate::videostab::FromFileMotionReaderTraitConst for FromFileMotionReader {
	#[inline] fn as_raw_FromFileMotionReader(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::FromFileMotionReaderTrait for FromFileMotionReader {
	#[inline] fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FromFileMotionReader, crate::videostab::FromFileMotionReaderTraitConst, as_raw_FromFileMotionReader, crate::videostab::FromFileMotionReaderTrait, as_raw_mut_FromFileMotionReader }

impl FromFileMotionReader {
	// FromFileMotionReader(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:201
	// ("cv::videostab::FromFileMotionReader::FromFileMotionReader", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn new(path: &str) -> Result<crate::videostab::FromFileMotionReader> {
		extern_container_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::FromFileMotionReader::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FromFileMotionReader, crate::videostab::ImageMotionEstimatorBase, cv_videostab_FromFileMotionReader_to_ImageMotionEstimatorBase }

impl std::fmt::Debug for FromFileMotionReader {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FromFileMotionReader")
			.finish()
	}
}

/// Constant methods for [crate::videostab::GaussianMotionFilter]
// GaussianMotionFilter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:97
pub trait GaussianMotionFilterTraitConst: crate::videostab::MotionFilterBaseTraitConst {
	fn as_raw_GaussianMotionFilter(&self) -> *const c_void;

	// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:103
	// ("cv::videostab::GaussianMotionFilter::radius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_GaussianMotionFilter_radius_const(self.as_raw_GaussianMotionFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// stdev()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:104
	// ("cv::videostab::GaussianMotionFilter::stdev", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn stdev(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_GaussianMotionFilter_stdev_const(self.as_raw_GaussianMotionFilter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::GaussianMotionFilter]
pub trait GaussianMotionFilterTrait: crate::videostab::GaussianMotionFilterTraitConst + crate::videostab::MotionFilterBaseTrait {
	fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * stdev: -1.f
	// setParams(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:102
	// ("cv::videostab::GaussianMotionFilter::setParams", vec![(pred!(mut, ["radius", "stdev"], ["int", "float"]), _)]),
	#[inline]
	fn set_params(&mut self, radius: i32, stdev: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_GaussianMotionFilter_setParams_int_float(self.as_raw_mut_GaussianMotionFilter(), radius, stdev, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [GaussianMotionFilterTrait::set_params] function uses the following default values for its arguments:
	/// * stdev: -1.f
	// cv::videostab::GaussianMotionFilter::setParams(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:102
	// ("cv::videostab::GaussianMotionFilter::setParams", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	#[inline]
	fn set_params_def(&mut self, radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_GaussianMotionFilter_setParams_int(self.as_raw_mut_GaussianMotionFilter(), radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// stabilize(int, const std::vector<Mat> &, const Range &)(Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:106
	// ("cv::videostab::GaussianMotionFilter::stabilize", vec![(pred!(mut, ["idx", "motions", "range"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
	#[inline]
	fn stabilize(&mut self, idx: i32, motions: &core::Vector<core::Mat>, range: &impl core::RangeTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_GaussianMotionFilter_stabilize_int_const_vectorLMatGR_const_RangeR(self.as_raw_mut_GaussianMotionFilter(), idx, motions.as_raw_VectorOfMat(), range.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// GaussianMotionFilter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:97
pub struct GaussianMotionFilter {
	ptr: *mut c_void,
}

opencv_type_boxed! { GaussianMotionFilter }

impl Drop for GaussianMotionFilter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_GaussianMotionFilter_delete(self.as_raw_mut_GaussianMotionFilter()) };
	}
}

unsafe impl Send for GaussianMotionFilter {}

impl crate::videostab::IMotionStabilizerTraitConst for GaussianMotionFilter {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for GaussianMotionFilter {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GaussianMotionFilter, crate::videostab::IMotionStabilizerTraitConst, as_raw_IMotionStabilizer, crate::videostab::IMotionStabilizerTrait, as_raw_mut_IMotionStabilizer }

impl crate::videostab::MotionFilterBaseTraitConst for GaussianMotionFilter {
	#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionFilterBaseTrait for GaussianMotionFilter {
	#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GaussianMotionFilter, crate::videostab::MotionFilterBaseTraitConst, as_raw_MotionFilterBase, crate::videostab::MotionFilterBaseTrait, as_raw_mut_MotionFilterBase }

impl crate::videostab::GaussianMotionFilterTraitConst for GaussianMotionFilter {
	#[inline] fn as_raw_GaussianMotionFilter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::GaussianMotionFilterTrait for GaussianMotionFilter {
	#[inline] fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { GaussianMotionFilter, crate::videostab::GaussianMotionFilterTraitConst, as_raw_GaussianMotionFilter, crate::videostab::GaussianMotionFilterTrait, as_raw_mut_GaussianMotionFilter }

impl GaussianMotionFilter {
	/// ## C++ default parameters
	/// * radius: 15
	/// * stdev: -1.f
	// GaussianMotionFilter(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:100
	// ("cv::videostab::GaussianMotionFilter::GaussianMotionFilter", vec![(pred!(mut, ["radius", "stdev"], ["int", "float"]), _)]),
	#[inline]
	pub fn new(radius: i32, stdev: f32) -> Result<crate::videostab::GaussianMotionFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(radius, stdev, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::GaussianMotionFilter::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * radius: 15
	/// * stdev: -1.f
	// cv::videostab::GaussianMotionFilter::GaussianMotionFilter() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:100
	// ("cv::videostab::GaussianMotionFilter::GaussianMotionFilter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::videostab::GaussianMotionFilter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_GaussianMotionFilter_GaussianMotionFilter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::GaussianMotionFilter::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { GaussianMotionFilter, crate::videostab::IMotionStabilizer, cv_videostab_GaussianMotionFilter_to_IMotionStabilizer }

boxed_cast_base! { GaussianMotionFilter, crate::videostab::MotionFilterBase, cv_videostab_GaussianMotionFilter_to_MotionFilterBase }

impl std::fmt::Debug for GaussianMotionFilter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GaussianMotionFilter")
			.finish()
	}
}

/// Constant methods for [crate::videostab::IDenseOptFlowEstimator]
// IDenseOptFlowEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:70
pub trait IDenseOptFlowEstimatorTraitConst {
	fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::IDenseOptFlowEstimator]
pub trait IDenseOptFlowEstimatorTrait: crate::videostab::IDenseOptFlowEstimatorTraitConst {
	fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void;

	// run(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:74
	// ("cv::videostab::IDenseOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "flowX", "flowY", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, flow_x: &mut impl ToInputOutputArray, flow_y: &mut impl ToInputOutputArray, errors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_output_array_arg!(flow_x);
		input_output_array_arg!(flow_y);
		output_array_arg!(errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(self.as_raw_mut_IDenseOptFlowEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), flow_x.as_raw__InputOutputArray(), flow_y.as_raw__InputOutputArray(), errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// IDenseOptFlowEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:70
pub struct IDenseOptFlowEstimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { IDenseOptFlowEstimator }

impl Drop for IDenseOptFlowEstimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_IDenseOptFlowEstimator_delete(self.as_raw_mut_IDenseOptFlowEstimator()) };
	}
}

unsafe impl Send for IDenseOptFlowEstimator {}

impl crate::videostab::IDenseOptFlowEstimatorTraitConst for IDenseOptFlowEstimator {
	#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IDenseOptFlowEstimatorTrait for IDenseOptFlowEstimator {
	#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { IDenseOptFlowEstimator, crate::videostab::IDenseOptFlowEstimatorTraitConst, as_raw_IDenseOptFlowEstimator, crate::videostab::IDenseOptFlowEstimatorTrait, as_raw_mut_IDenseOptFlowEstimator }

impl IDenseOptFlowEstimator {
}

boxed_cast_descendant! { IDenseOptFlowEstimator, crate::videostab::DensePyrLkOptFlowEstimatorGpu, cv_videostab_IDenseOptFlowEstimator_to_DensePyrLkOptFlowEstimatorGpu }

impl std::fmt::Debug for IDenseOptFlowEstimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("IDenseOptFlowEstimator")
			.finish()
	}
}

/// Constant methods for [crate::videostab::IFrameSource]
// IFrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:57
pub trait IFrameSourceTraitConst {
	fn as_raw_IFrameSource(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::IFrameSource]
pub trait IFrameSourceTrait: crate::videostab::IFrameSourceTraitConst {
	fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void;

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:61
	// ("cv::videostab::IFrameSource::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_IFrameSource_reset(self.as_raw_mut_IFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:62
	// ("cv::videostab::IFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn next_frame(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_IFrameSource_nextFrame(self.as_raw_mut_IFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// IFrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:57
pub struct IFrameSource {
	ptr: *mut c_void,
}

opencv_type_boxed! { IFrameSource }

impl Drop for IFrameSource {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_IFrameSource_delete(self.as_raw_mut_IFrameSource()) };
	}
}

unsafe impl Send for IFrameSource {}

impl crate::videostab::IFrameSourceTraitConst for IFrameSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IFrameSourceTrait for IFrameSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { IFrameSource, crate::videostab::IFrameSourceTraitConst, as_raw_IFrameSource, crate::videostab::IFrameSourceTrait, as_raw_mut_IFrameSource }

impl IFrameSource {
}

boxed_cast_descendant! { IFrameSource, crate::videostab::MaskFrameSource, cv_videostab_IFrameSource_to_MaskFrameSource }

boxed_cast_descendant! { IFrameSource, crate::videostab::NullFrameSource, cv_videostab_IFrameSource_to_NullFrameSource }

boxed_cast_descendant! { IFrameSource, crate::videostab::OnePassStabilizer, cv_videostab_IFrameSource_to_OnePassStabilizer }

boxed_cast_descendant! { IFrameSource, crate::videostab::TwoPassStabilizer, cv_videostab_IFrameSource_to_TwoPassStabilizer }

boxed_cast_descendant! { IFrameSource, crate::videostab::VideoFileSource, cv_videostab_IFrameSource_to_VideoFileSource }

impl std::fmt::Debug for IFrameSource {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("IFrameSource")
			.finish()
	}
}

/// Constant methods for [crate::videostab::ILog]
// ILog /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:56
pub trait ILogTraitConst {
	fn as_raw_ILog(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::ILog]
pub trait ILogTrait: crate::videostab::ILogTraitConst {
	fn as_raw_mut_ILog(&mut self) -> *mut c_void;

	// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:60
	// ("cv::videostab::ILog::print", vec![(pred!(mut, ["format"], ["const char*"]), _)]),
	#[inline]
	fn print(&mut self, format: &str) -> Result<()> {
		extern_container_arg!(format);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ILog_print_const_charX(self.as_raw_mut_ILog(), format.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// ILog /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:56
pub struct ILog {
	ptr: *mut c_void,
}

opencv_type_boxed! { ILog }

impl Drop for ILog {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_ILog_delete(self.as_raw_mut_ILog()) };
	}
}

unsafe impl Send for ILog {}

impl crate::videostab::ILogTraitConst for ILog {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ILogTrait for ILog {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ILog, crate::videostab::ILogTraitConst, as_raw_ILog, crate::videostab::ILogTrait, as_raw_mut_ILog }

impl ILog {
}

boxed_cast_descendant! { ILog, crate::videostab::LogToStdout, cv_videostab_ILog_to_LogToStdout }

boxed_cast_descendant! { ILog, crate::videostab::NullLog, cv_videostab_ILog_to_NullLog }

impl std::fmt::Debug for ILog {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ILog")
			.finish()
	}
}

/// Constant methods for [crate::videostab::IMotionStabilizer]
// IMotionStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:59
pub trait IMotionStabilizerTraitConst {
	fn as_raw_IMotionStabilizer(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::IMotionStabilizer]
pub trait IMotionStabilizerTrait: crate::videostab::IMotionStabilizerTraitConst {
	fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void;

	/// assumes that [0, size-1) is in or equals to [range.first, range.second)
	// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:65
	// ("cv::videostab::IMotionStabilizer::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
	#[inline]
	fn stabilize(&mut self, size: i32, motions: &core::Vector<core::Mat>, range: &impl core::RangeTraitConst, stabilization_motions: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_IMotionStabilizer_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(self.as_raw_mut_IMotionStabilizer(), size, motions.as_raw_VectorOfMat(), range.as_raw_Range(), stabilization_motions.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// IMotionStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:59
pub struct IMotionStabilizer {
	ptr: *mut c_void,
}

opencv_type_boxed! { IMotionStabilizer }

impl Drop for IMotionStabilizer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_IMotionStabilizer_delete(self.as_raw_mut_IMotionStabilizer()) };
	}
}

unsafe impl Send for IMotionStabilizer {}

impl crate::videostab::IMotionStabilizerTraitConst for IMotionStabilizer {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for IMotionStabilizer {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { IMotionStabilizer, crate::videostab::IMotionStabilizerTraitConst, as_raw_IMotionStabilizer, crate::videostab::IMotionStabilizerTrait, as_raw_mut_IMotionStabilizer }

impl IMotionStabilizer {
}

boxed_cast_descendant! { IMotionStabilizer, crate::videostab::GaussianMotionFilter, cv_videostab_IMotionStabilizer_to_GaussianMotionFilter }

boxed_cast_descendant! { IMotionStabilizer, crate::videostab::LpMotionStabilizer, cv_videostab_IMotionStabilizer_to_LpMotionStabilizer }

boxed_cast_descendant! { IMotionStabilizer, crate::videostab::MotionFilterBase, cv_videostab_IMotionStabilizer_to_MotionFilterBase }

boxed_cast_descendant! { IMotionStabilizer, crate::videostab::MotionStabilizationPipeline, cv_videostab_IMotionStabilizer_to_MotionStabilizationPipeline }

impl std::fmt::Debug for IMotionStabilizer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("IMotionStabilizer")
			.finish()
	}
}

/// Constant methods for [crate::videostab::IOutlierRejector]
// IOutlierRejector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:58
pub trait IOutlierRejectorTraitConst {
	fn as_raw_IOutlierRejector(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::IOutlierRejector]
pub trait IOutlierRejectorTrait: crate::videostab::IOutlierRejectorTraitConst {
	fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void;

	// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:63
	// ("cv::videostab::IOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn process(&mut self, frame_size: core::Size, points0: &impl ToInputArray, points1: &impl ToInputArray, mask: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_IOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_IOutlierRejector(), &frame_size, points0.as_raw__InputArray(), points1.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// IOutlierRejector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:58
pub struct IOutlierRejector {
	ptr: *mut c_void,
}

opencv_type_boxed! { IOutlierRejector }

impl Drop for IOutlierRejector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_IOutlierRejector_delete(self.as_raw_mut_IOutlierRejector()) };
	}
}

unsafe impl Send for IOutlierRejector {}

impl crate::videostab::IOutlierRejectorTraitConst for IOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IOutlierRejectorTrait for IOutlierRejector {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { IOutlierRejector, crate::videostab::IOutlierRejectorTraitConst, as_raw_IOutlierRejector, crate::videostab::IOutlierRejectorTrait, as_raw_mut_IOutlierRejector }

impl IOutlierRejector {
}

boxed_cast_descendant! { IOutlierRejector, crate::videostab::NullOutlierRejector, cv_videostab_IOutlierRejector_to_NullOutlierRejector }

boxed_cast_descendant! { IOutlierRejector, crate::videostab::TranslationBasedLocalOutlierRejector, cv_videostab_IOutlierRejector_to_TranslationBasedLocalOutlierRejector }

impl std::fmt::Debug for IOutlierRejector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("IOutlierRejector")
			.finish()
	}
}

/// Constant methods for [crate::videostab::ISparseOptFlowEstimator]
// ISparseOptFlowEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:61
pub trait ISparseOptFlowEstimatorTraitConst {
	fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::ISparseOptFlowEstimator]
pub trait ISparseOptFlowEstimatorTrait: crate::videostab::ISparseOptFlowEstimatorTraitConst {
	fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void;

	// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:65
	// ("cv::videostab::ISparseOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, points0: &impl ToInputArray, points1: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, errors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_array_arg!(points0);
		input_output_array_arg!(points1);
		output_array_arg!(status);
		output_array_arg!(errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_ISparseOptFlowEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), points0.as_raw__InputArray(), points1.as_raw__InputOutputArray(), status.as_raw__OutputArray(), errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// ISparseOptFlowEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:61
pub struct ISparseOptFlowEstimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { ISparseOptFlowEstimator }

impl Drop for ISparseOptFlowEstimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_ISparseOptFlowEstimator_delete(self.as_raw_mut_ISparseOptFlowEstimator()) };
	}
}

unsafe impl Send for ISparseOptFlowEstimator {}

impl crate::videostab::ISparseOptFlowEstimatorTraitConst for ISparseOptFlowEstimator {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimatorTrait for ISparseOptFlowEstimator {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ISparseOptFlowEstimator, crate::videostab::ISparseOptFlowEstimatorTraitConst, as_raw_ISparseOptFlowEstimator, crate::videostab::ISparseOptFlowEstimatorTrait, as_raw_mut_ISparseOptFlowEstimator }

impl ISparseOptFlowEstimator {
}

boxed_cast_descendant! { ISparseOptFlowEstimator, crate::videostab::SparsePyrLkOptFlowEstimator, cv_videostab_ISparseOptFlowEstimator_to_SparsePyrLkOptFlowEstimator }

boxed_cast_descendant! { ISparseOptFlowEstimator, crate::videostab::SparsePyrLkOptFlowEstimatorGpu, cv_videostab_ISparseOptFlowEstimator_to_SparsePyrLkOptFlowEstimatorGpu }

impl std::fmt::Debug for ISparseOptFlowEstimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ISparseOptFlowEstimator")
			.finish()
	}
}

/// Constant methods for [crate::videostab::ImageMotionEstimatorBase]
// ImageMotionEstimatorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:175
pub trait ImageMotionEstimatorBaseTraitConst {
	fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void;

	// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:181
	// ("cv::videostab::ImageMotionEstimatorBase::motionModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_motionModel_const(self.as_raw_ImageMotionEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::ImageMotionEstimatorBase]
pub trait ImageMotionEstimatorBaseTrait: crate::videostab::ImageMotionEstimatorBaseTraitConst {
	fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void;

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:180
	// ("cv::videostab::ImageMotionEstimatorBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(self.as_raw_mut_ImageMotionEstimatorBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrameMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:183
	// ("cv::videostab::ImageMotionEstimatorBase::setFrameMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_frame_mask(&mut self, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_setFrameMask_const__InputArrayR(self.as_raw_mut_ImageMotionEstimatorBase(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:189
	// ("cv::videostab::ImageMotionEstimatorBase::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst, ok: &mut bool) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_ImageMotionEstimatorBase(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [ImageMotionEstimatorBaseTrait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::ImageMotionEstimatorBase::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:189
	// ("cv::videostab::ImageMotionEstimatorBase::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR(self.as_raw_mut_ImageMotionEstimatorBase(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Base class for global 2D motion estimation methods which take frames as input.
// ImageMotionEstimatorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:175
pub struct ImageMotionEstimatorBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { ImageMotionEstimatorBase }

impl Drop for ImageMotionEstimatorBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_ImageMotionEstimatorBase_delete(self.as_raw_mut_ImageMotionEstimatorBase()) };
	}
}

unsafe impl Send for ImageMotionEstimatorBase {}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for ImageMotionEstimatorBase {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for ImageMotionEstimatorBase {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ImageMotionEstimatorBase, crate::videostab::ImageMotionEstimatorBaseTraitConst, as_raw_ImageMotionEstimatorBase, crate::videostab::ImageMotionEstimatorBaseTrait, as_raw_mut_ImageMotionEstimatorBase }

impl ImageMotionEstimatorBase {
}

boxed_cast_descendant! { ImageMotionEstimatorBase, crate::videostab::FromFileMotionReader, cv_videostab_ImageMotionEstimatorBase_to_FromFileMotionReader }

boxed_cast_descendant! { ImageMotionEstimatorBase, crate::videostab::KeypointBasedMotionEstimator, cv_videostab_ImageMotionEstimatorBase_to_KeypointBasedMotionEstimator }

boxed_cast_descendant! { ImageMotionEstimatorBase, crate::videostab::KeypointBasedMotionEstimatorGpu, cv_videostab_ImageMotionEstimatorBase_to_KeypointBasedMotionEstimatorGpu }

boxed_cast_descendant! { ImageMotionEstimatorBase, crate::videostab::ToFileMotionWriter, cv_videostab_ImageMotionEstimatorBase_to_ToFileMotionWriter }

impl std::fmt::Debug for ImageMotionEstimatorBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ImageMotionEstimatorBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::InpainterBase]
// InpainterBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:61
pub trait InpainterBaseTraitConst {
	fn as_raw_InpainterBase(&self) -> *const c_void;

	// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:71
	// ("cv::videostab::InpainterBase::radius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_radius_const(self.as_raw_InpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:74
	// ("cv::videostab::InpainterBase::motionModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_motionModel_const(self.as_raw_InpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// frames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:82
	// ("cv::videostab::InpainterBase::frames", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frames(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_frames_const(self.as_raw_InpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// motions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:85
	// ("cv::videostab::InpainterBase::motions", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motions(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_motions_const(self.as_raw_InpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// stabilizedFrames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:88
	// ("cv::videostab::InpainterBase::stabilizedFrames", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn stabilized_frames(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_stabilizedFrames_const(self.as_raw_InpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// stabilizationMotions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:91
	// ("cv::videostab::InpainterBase::stabilizationMotions", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn stabilization_motions(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_stabilizationMotions_const(self.as_raw_InpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::InpainterBase]
pub trait InpainterBaseTrait: crate::videostab::InpainterBaseTraitConst {
	fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void;

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:70
	// ("cv::videostab::InpainterBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_radius(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_setRadius_int(self.as_raw_mut_InpainterBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:73
	// ("cv::videostab::InpainterBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_setMotionModel_MotionModel(self.as_raw_mut_InpainterBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:76
	// ("cv::videostab::InpainterBase::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn inpaint(&mut self, idx: i32, frame: &mut impl core::MatTrait, mask: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_inpaint_int_MatR_MatR(self.as_raw_mut_InpainterBase(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:81
	// ("cv::videostab::InpainterBase::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_frames(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_setFrames_const_vectorLMatGR(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:84
	// ("cv::videostab::InpainterBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_motions(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_setMotions_const_vectorLMatGR(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setStabilizedFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:87
	// ("cv::videostab::InpainterBase::setStabilizedFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_stabilized_frames(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_setStabilizedFrames_const_vectorLMatGR(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:90
	// ("cv::videostab::InpainterBase::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_stabilization_motions(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpainterBase_setStabilizationMotions_const_vectorLMatGR(self.as_raw_mut_InpainterBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// InpainterBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:61
pub struct InpainterBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { InpainterBase }

impl Drop for InpainterBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_InpainterBase_delete(self.as_raw_mut_InpainterBase()) };
	}
}

unsafe impl Send for InpainterBase {}

impl crate::videostab::InpainterBaseTraitConst for InpainterBase {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpainterBaseTrait for InpainterBase {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InpainterBase, crate::videostab::InpainterBaseTraitConst, as_raw_InpainterBase, crate::videostab::InpainterBaseTrait, as_raw_mut_InpainterBase }

impl InpainterBase {
}

boxed_cast_descendant! { InpainterBase, crate::videostab::ColorAverageInpainter, cv_videostab_InpainterBase_to_ColorAverageInpainter }

boxed_cast_descendant! { InpainterBase, crate::videostab::ColorInpainter, cv_videostab_InpainterBase_to_ColorInpainter }

boxed_cast_descendant! { InpainterBase, crate::videostab::ConsistentMosaicInpainter, cv_videostab_InpainterBase_to_ConsistentMosaicInpainter }

boxed_cast_descendant! { InpainterBase, crate::videostab::InpaintingPipeline, cv_videostab_InpainterBase_to_InpaintingPipeline }

boxed_cast_descendant! { InpainterBase, crate::videostab::MotionInpainter, cv_videostab_InpainterBase_to_MotionInpainter }

boxed_cast_descendant! { InpainterBase, crate::videostab::NullInpainter, cv_videostab_InpainterBase_to_NullInpainter }

impl std::fmt::Debug for InpainterBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("InpainterBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::InpaintingPipeline]
// InpaintingPipeline /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:108
pub trait InpaintingPipelineTraitConst: crate::videostab::InpainterBaseTraitConst {
	fn as_raw_InpaintingPipeline(&self) -> *const c_void;

	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:112
	// ("cv::videostab::InpaintingPipeline::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_empty_const(self.as_raw_InpaintingPipeline(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::InpaintingPipeline]
pub trait InpaintingPipelineTrait: crate::videostab::InpainterBaseTrait + crate::videostab::InpaintingPipelineTraitConst {
	fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void;

	// pushBack(Ptr<InpainterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:111
	// ("cv::videostab::InpaintingPipeline::pushBack", vec![(pred!(mut, ["inpainter"], ["cv::Ptr<cv::videostab::InpainterBase>"]), _)]),
	#[inline]
	fn push_back(&mut self, mut inpainter: core::Ptr<crate::videostab::InpainterBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_pushBack_PtrLInpainterBaseG(self.as_raw_mut_InpaintingPipeline(), inpainter.as_raw_mut_PtrOfInpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:114
	// ("cv::videostab::InpaintingPipeline::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_radius(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_setRadius_int(self.as_raw_mut_InpaintingPipeline(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:115
	// ("cv::videostab::InpaintingPipeline::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(self.as_raw_mut_InpaintingPipeline(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:116
	// ("cv::videostab::InpaintingPipeline::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_frames(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_setFrames_const_vectorLMatGR(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:117
	// ("cv::videostab::InpaintingPipeline::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_motions(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_setMotions_const_vectorLMatGR(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setStabilizedFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:118
	// ("cv::videostab::InpaintingPipeline::setStabilizedFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_stabilized_frames(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vectorLMatGR(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:119
	// ("cv::videostab::InpaintingPipeline::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_stabilization_motions(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vectorLMatGR(self.as_raw_mut_InpaintingPipeline(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:121
	// ("cv::videostab::InpaintingPipeline::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn inpaint(&mut self, idx: i32, frame: &mut impl core::MatTrait, mask: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_InpaintingPipeline_inpaint_int_MatR_MatR(self.as_raw_mut_InpaintingPipeline(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// InpaintingPipeline /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:108
pub struct InpaintingPipeline {
	ptr: *mut c_void,
}

opencv_type_boxed! { InpaintingPipeline }

impl Drop for InpaintingPipeline {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_InpaintingPipeline_delete(self.as_raw_mut_InpaintingPipeline()) };
	}
}

unsafe impl Send for InpaintingPipeline {}

impl crate::videostab::InpainterBaseTraitConst for InpaintingPipeline {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpainterBaseTrait for InpaintingPipeline {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InpaintingPipeline, crate::videostab::InpainterBaseTraitConst, as_raw_InpainterBase, crate::videostab::InpainterBaseTrait, as_raw_mut_InpainterBase }

impl crate::videostab::InpaintingPipelineTraitConst for InpaintingPipeline {
	#[inline] fn as_raw_InpaintingPipeline(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpaintingPipelineTrait for InpaintingPipeline {
	#[inline] fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InpaintingPipeline, crate::videostab::InpaintingPipelineTraitConst, as_raw_InpaintingPipeline, crate::videostab::InpaintingPipelineTrait, as_raw_mut_InpaintingPipeline }

impl InpaintingPipeline {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_InpaintingPipeline_defaultNew_const()) }
	}

}

boxed_cast_base! { InpaintingPipeline, crate::videostab::InpainterBase, cv_videostab_InpaintingPipeline_to_InpainterBase }

impl std::fmt::Debug for InpaintingPipeline {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("InpaintingPipeline")
			.finish()
	}
}

impl Default for InpaintingPipeline {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::KeypointBasedMotionEstimator]
// KeypointBasedMotionEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:229
pub trait KeypointBasedMotionEstimatorTraitConst: crate::videostab::ImageMotionEstimatorBaseTraitConst {
	fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void;

	// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:235
	// ("cv::videostab::KeypointBasedMotionEstimator::motionModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_motionModel_const(self.as_raw_KeypointBasedMotionEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// detector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:238
	// ("cv::videostab::KeypointBasedMotionEstimator::detector", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn detector(&self) -> Result<core::Ptr<crate::features2d::Feature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_detector_const(self.as_raw_KeypointBasedMotionEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::features2d::Feature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// opticalFlowEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:241
	// ("cv::videostab::KeypointBasedMotionEstimator::opticalFlowEstimator", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn optical_flow_estimator(&self) -> Result<core::Ptr<crate::videostab::ISparseOptFlowEstimator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(self.as_raw_KeypointBasedMotionEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::ISparseOptFlowEstimator>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// outlierRejector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:244
	// ("cv::videostab::KeypointBasedMotionEstimator::outlierRejector", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn outlier_rejector(&self) -> Result<core::Ptr<crate::videostab::IOutlierRejector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(self.as_raw_KeypointBasedMotionEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::IOutlierRejector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::KeypointBasedMotionEstimator]
pub trait KeypointBasedMotionEstimatorTrait: crate::videostab::ImageMotionEstimatorBaseTrait + crate::videostab::KeypointBasedMotionEstimatorTraitConst {
	fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void;

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:234
	// ("cv::videostab::KeypointBasedMotionEstimator::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(self.as_raw_mut_KeypointBasedMotionEstimator(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDetector(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:237
	// ("cv::videostab::KeypointBasedMotionEstimator::setDetector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	#[inline]
	fn set_detector(&mut self, mut val: core::Ptr<crate::features2d::Feature2D>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setDetector_PtrLFeature2DG(self.as_raw_mut_KeypointBasedMotionEstimator(), val.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setOpticalFlowEstimator(Ptr<ISparseOptFlowEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:240
	// ("cv::videostab::KeypointBasedMotionEstimator::setOpticalFlowEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ISparseOptFlowEstimator>"]), _)]),
	#[inline]
	fn set_optical_flow_estimator(&mut self, mut val: core::Ptr<crate::videostab::ISparseOptFlowEstimator>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_PtrLISparseOptFlowEstimatorG(self.as_raw_mut_KeypointBasedMotionEstimator(), val.as_raw_mut_PtrOfISparseOptFlowEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setOutlierRejector(Ptr<IOutlierRejector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:243
	// ("cv::videostab::KeypointBasedMotionEstimator::setOutlierRejector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IOutlierRejector>"]), _)]),
	#[inline]
	fn set_outlier_rejector(&mut self, mut val: core::Ptr<crate::videostab::IOutlierRejector>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_PtrLIOutlierRejectorG(self.as_raw_mut_KeypointBasedMotionEstimator(), val.as_raw_mut_PtrOfIOutlierRejector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrameMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:246
	// ("cv::videostab::KeypointBasedMotionEstimator::setFrameMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_frame_mask(&mut self, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_setFrameMask_const__InputArrayR(self.as_raw_mut_KeypointBasedMotionEstimator(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:248
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	#[inline]
	fn estimate_mat(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst, ok: &mut bool) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_KeypointBasedMotionEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [KeypointBasedMotionEstimatorTrait::estimate_mat] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::KeypointBasedMotionEstimator::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:248
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn estimate_mat_def(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR(self.as_raw_mut_KeypointBasedMotionEstimator(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:249
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, ok: &mut bool) -> Result<core::Mat> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR_boolX(self.as_raw_mut_KeypointBasedMotionEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [KeypointBasedMotionEstimatorTrait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::KeypointBasedMotionEstimator::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:249
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR(self.as_raw_mut_KeypointBasedMotionEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Describes a global 2D motion estimation method which uses keypoints detection and optical flow for
/// matching.
// KeypointBasedMotionEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:229
pub struct KeypointBasedMotionEstimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { KeypointBasedMotionEstimator }

impl Drop for KeypointBasedMotionEstimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_delete(self.as_raw_mut_KeypointBasedMotionEstimator()) };
	}
}

unsafe impl Send for KeypointBasedMotionEstimator {}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for KeypointBasedMotionEstimator {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for KeypointBasedMotionEstimator {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KeypointBasedMotionEstimator, crate::videostab::ImageMotionEstimatorBaseTraitConst, as_raw_ImageMotionEstimatorBase, crate::videostab::ImageMotionEstimatorBaseTrait, as_raw_mut_ImageMotionEstimatorBase }

impl crate::videostab::KeypointBasedMotionEstimatorTraitConst for KeypointBasedMotionEstimator {
	#[inline] fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::KeypointBasedMotionEstimatorTrait for KeypointBasedMotionEstimator {
	#[inline] fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KeypointBasedMotionEstimator, crate::videostab::KeypointBasedMotionEstimatorTraitConst, as_raw_KeypointBasedMotionEstimator, crate::videostab::KeypointBasedMotionEstimatorTrait, as_raw_mut_KeypointBasedMotionEstimator }

impl KeypointBasedMotionEstimator {
	// KeypointBasedMotionEstimator(Ptr<MotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:232
	// ("cv::videostab::KeypointBasedMotionEstimator::KeypointBasedMotionEstimator", vec![(pred!(mut, ["estimator"], ["cv::Ptr<cv::videostab::MotionEstimatorBase>"]), _)]),
	#[inline]
	pub fn new(mut estimator: core::Ptr<crate::videostab::MotionEstimatorBase>) -> Result<crate::videostab::KeypointBasedMotionEstimator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_PtrLMotionEstimatorBaseG(estimator.as_raw_mut_PtrOfMotionEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::KeypointBasedMotionEstimator::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { KeypointBasedMotionEstimator, crate::videostab::ImageMotionEstimatorBase, cv_videostab_KeypointBasedMotionEstimator_to_ImageMotionEstimatorBase }

impl std::fmt::Debug for KeypointBasedMotionEstimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KeypointBasedMotionEstimator")
			.finish()
	}
}

/// Constant methods for [crate::videostab::KeypointBasedMotionEstimatorGpu]
// KeypointBasedMotionEstimatorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:266
pub trait KeypointBasedMotionEstimatorGpuTraitConst: crate::videostab::ImageMotionEstimatorBaseTraitConst {
	fn as_raw_KeypointBasedMotionEstimatorGpu(&self) -> *const c_void;

	// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:272
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::motionModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_motionModel_const(self.as_raw_KeypointBasedMotionEstimatorGpu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// outlierRejector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:275
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::outlierRejector", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn outlier_rejector(&self) -> Result<core::Ptr<crate::videostab::IOutlierRejector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_outlierRejector_const(self.as_raw_KeypointBasedMotionEstimatorGpu(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::IOutlierRejector>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::KeypointBasedMotionEstimatorGpu]
pub trait KeypointBasedMotionEstimatorGpuTrait: crate::videostab::ImageMotionEstimatorBaseTrait + crate::videostab::KeypointBasedMotionEstimatorGpuTraitConst {
	fn as_raw_mut_KeypointBasedMotionEstimatorGpu(&mut self) -> *mut c_void;

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:271
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_setMotionModel_MotionModel(self.as_raw_mut_KeypointBasedMotionEstimatorGpu(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setOutlierRejector(Ptr<IOutlierRejector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:274
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::setOutlierRejector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IOutlierRejector>"]), _)]),
	#[inline]
	fn set_outlier_rejector(&mut self, mut val: core::Ptr<crate::videostab::IOutlierRejector>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_setOutlierRejector_PtrLIOutlierRejectorG(self.as_raw_mut_KeypointBasedMotionEstimatorGpu(), val.as_raw_mut_PtrOfIOutlierRejector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:277
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst, ok: &mut bool) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_KeypointBasedMotionEstimatorGpu(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [KeypointBasedMotionEstimatorGpuTrait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::KeypointBasedMotionEstimatorGpu::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:277
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_MatR_const_MatR(self.as_raw_mut_KeypointBasedMotionEstimatorGpu(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(const cuda::GpuMat &, const cuda::GpuMat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:278
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "bool*"]), _)]),
	#[inline]
	fn estimate_1(&mut self, frame0: &impl core::GpuMatTraitConst, frame1: &impl core::GpuMatTraitConst, ok: &mut bool) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_GpuMatR_const_GpuMatR_boolX(self.as_raw_mut_KeypointBasedMotionEstimatorGpu(), frame0.as_raw_GpuMat(), frame1.as_raw_GpuMat(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [KeypointBasedMotionEstimatorGpuTrait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::KeypointBasedMotionEstimatorGpu::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:278
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn estimate_def_1(&mut self, frame0: &impl core::GpuMatTraitConst, frame1: &impl core::GpuMatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_estimate_const_GpuMatR_const_GpuMatR(self.as_raw_mut_KeypointBasedMotionEstimatorGpu(), frame0.as_raw_GpuMat(), frame1.as_raw_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// KeypointBasedMotionEstimatorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:266
pub struct KeypointBasedMotionEstimatorGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { KeypointBasedMotionEstimatorGpu }

impl Drop for KeypointBasedMotionEstimatorGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_delete(self.as_raw_mut_KeypointBasedMotionEstimatorGpu()) };
	}
}

unsafe impl Send for KeypointBasedMotionEstimatorGpu {}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for KeypointBasedMotionEstimatorGpu {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for KeypointBasedMotionEstimatorGpu {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KeypointBasedMotionEstimatorGpu, crate::videostab::ImageMotionEstimatorBaseTraitConst, as_raw_ImageMotionEstimatorBase, crate::videostab::ImageMotionEstimatorBaseTrait, as_raw_mut_ImageMotionEstimatorBase }

impl crate::videostab::KeypointBasedMotionEstimatorGpuTraitConst for KeypointBasedMotionEstimatorGpu {
	#[inline] fn as_raw_KeypointBasedMotionEstimatorGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::KeypointBasedMotionEstimatorGpuTrait for KeypointBasedMotionEstimatorGpu {
	#[inline] fn as_raw_mut_KeypointBasedMotionEstimatorGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { KeypointBasedMotionEstimatorGpu, crate::videostab::KeypointBasedMotionEstimatorGpuTraitConst, as_raw_KeypointBasedMotionEstimatorGpu, crate::videostab::KeypointBasedMotionEstimatorGpuTrait, as_raw_mut_KeypointBasedMotionEstimatorGpu }

impl KeypointBasedMotionEstimatorGpu {
	// KeypointBasedMotionEstimatorGpu(Ptr<MotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:269
	// ("cv::videostab::KeypointBasedMotionEstimatorGpu::KeypointBasedMotionEstimatorGpu", vec![(pred!(mut, ["estimator"], ["cv::Ptr<cv::videostab::MotionEstimatorBase>"]), _)]),
	#[inline]
	pub fn new(mut estimator: core::Ptr<crate::videostab::MotionEstimatorBase>) -> Result<crate::videostab::KeypointBasedMotionEstimatorGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_KeypointBasedMotionEstimatorGpu_KeypointBasedMotionEstimatorGpu_PtrLMotionEstimatorBaseG(estimator.as_raw_mut_PtrOfMotionEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::KeypointBasedMotionEstimatorGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { KeypointBasedMotionEstimatorGpu, crate::videostab::ImageMotionEstimatorBase, cv_videostab_KeypointBasedMotionEstimatorGpu_to_ImageMotionEstimatorBase }

impl std::fmt::Debug for KeypointBasedMotionEstimatorGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("KeypointBasedMotionEstimatorGpu")
			.finish()
	}
}

/// Constant methods for [crate::videostab::LogToStdout]
// LogToStdout /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:69
pub trait LogToStdoutTraitConst: crate::videostab::ILogTraitConst {
	fn as_raw_LogToStdout(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::LogToStdout]
pub trait LogToStdoutTrait: crate::videostab::ILogTrait + crate::videostab::LogToStdoutTraitConst {
	fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void;

	// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:72
	// ("cv::videostab::LogToStdout::print", vec![(pred!(mut, ["format"], ["const char*"]), _)]),
	#[inline]
	fn print(&mut self, format: &str) -> Result<()> {
		extern_container_arg!(format);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LogToStdout_print_const_charX(self.as_raw_mut_LogToStdout(), format.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// LogToStdout /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:69
pub struct LogToStdout {
	ptr: *mut c_void,
}

opencv_type_boxed! { LogToStdout }

impl Drop for LogToStdout {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_LogToStdout_delete(self.as_raw_mut_LogToStdout()) };
	}
}

unsafe impl Send for LogToStdout {}

impl crate::videostab::ILogTraitConst for LogToStdout {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ILogTrait for LogToStdout {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LogToStdout, crate::videostab::ILogTraitConst, as_raw_ILog, crate::videostab::ILogTrait, as_raw_mut_ILog }

impl crate::videostab::LogToStdoutTraitConst for LogToStdout {
	#[inline] fn as_raw_LogToStdout(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::LogToStdoutTrait for LogToStdout {
	#[inline] fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LogToStdout, crate::videostab::LogToStdoutTraitConst, as_raw_LogToStdout, crate::videostab::LogToStdoutTrait, as_raw_mut_LogToStdout }

impl LogToStdout {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_LogToStdout_defaultNew_const()) }
	}

}

boxed_cast_base! { LogToStdout, crate::videostab::ILog, cv_videostab_LogToStdout_to_ILog }

impl std::fmt::Debug for LogToStdout {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LogToStdout")
			.finish()
	}
}

impl Default for LogToStdout {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::LpMotionStabilizer]
// LpMotionStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:117
pub trait LpMotionStabilizerTraitConst: crate::videostab::IMotionStabilizerTraitConst {
	fn as_raw_LpMotionStabilizer(&self) -> *const c_void;

	// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:123
	// ("cv::videostab::LpMotionStabilizer::motionModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_motionModel_const(self.as_raw_LpMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// frameSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:126
	// ("cv::videostab::LpMotionStabilizer::frameSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_frameSize_const(self.as_raw_LpMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// trimRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:129
	// ("cv::videostab::LpMotionStabilizer::trimRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn trim_ratio(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_trimRatio_const(self.as_raw_LpMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// weight1()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:132
	// ("cv::videostab::LpMotionStabilizer::weight1", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weight1(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight1_const(self.as_raw_LpMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// weight2()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:135
	// ("cv::videostab::LpMotionStabilizer::weight2", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weight2(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight2_const(self.as_raw_LpMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// weight3()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:138
	// ("cv::videostab::LpMotionStabilizer::weight3", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weight3(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight3_const(self.as_raw_LpMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// weight4()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:141
	// ("cv::videostab::LpMotionStabilizer::weight4", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weight4(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_weight4_const(self.as_raw_LpMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::LpMotionStabilizer]
pub trait LpMotionStabilizerTrait: crate::videostab::IMotionStabilizerTrait + crate::videostab::LpMotionStabilizerTraitConst {
	fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void;

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:122
	// ("cv::videostab::LpMotionStabilizer::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(self.as_raw_mut_LpMotionStabilizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrameSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:125
	// ("cv::videostab::LpMotionStabilizer::setFrameSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	#[inline]
	fn set_frame_size(&mut self, val: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_setFrameSize_Size(self.as_raw_mut_LpMotionStabilizer(), &val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTrimRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:128
	// ("cv::videostab::LpMotionStabilizer::setTrimRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_setTrimRatio_float(self.as_raw_mut_LpMotionStabilizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWeight1(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:131
	// ("cv::videostab::LpMotionStabilizer::setWeight1", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_weight1(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight1_float(self.as_raw_mut_LpMotionStabilizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWeight2(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:134
	// ("cv::videostab::LpMotionStabilizer::setWeight2", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_weight2(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight2_float(self.as_raw_mut_LpMotionStabilizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWeight3(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:137
	// ("cv::videostab::LpMotionStabilizer::setWeight3", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_weight3(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight3_float(self.as_raw_mut_LpMotionStabilizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWeight4(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:140
	// ("cv::videostab::LpMotionStabilizer::setWeight4", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_weight4(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_setWeight4_float(self.as_raw_mut_LpMotionStabilizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:143
	// ("cv::videostab::LpMotionStabilizer::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
	#[inline]
	fn stabilize(&mut self, size: i32, motions: &core::Vector<core::Mat>, range: &impl core::RangeTraitConst, stabilization_motions: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(self.as_raw_mut_LpMotionStabilizer(), size, motions.as_raw_VectorOfMat(), range.as_raw_Range(), stabilization_motions.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// LpMotionStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:117
pub struct LpMotionStabilizer {
	ptr: *mut c_void,
}

opencv_type_boxed! { LpMotionStabilizer }

impl Drop for LpMotionStabilizer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_LpMotionStabilizer_delete(self.as_raw_mut_LpMotionStabilizer()) };
	}
}

unsafe impl Send for LpMotionStabilizer {}

impl crate::videostab::IMotionStabilizerTraitConst for LpMotionStabilizer {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for LpMotionStabilizer {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LpMotionStabilizer, crate::videostab::IMotionStabilizerTraitConst, as_raw_IMotionStabilizer, crate::videostab::IMotionStabilizerTrait, as_raw_mut_IMotionStabilizer }

impl crate::videostab::LpMotionStabilizerTraitConst for LpMotionStabilizer {
	#[inline] fn as_raw_LpMotionStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::LpMotionStabilizerTrait for LpMotionStabilizer {
	#[inline] fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LpMotionStabilizer, crate::videostab::LpMotionStabilizerTraitConst, as_raw_LpMotionStabilizer, crate::videostab::LpMotionStabilizerTrait, as_raw_mut_LpMotionStabilizer }

impl LpMotionStabilizer {
	/// ## C++ default parameters
	/// * model: MM_SIMILARITY
	// LpMotionStabilizer(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:120
	// ("cv::videostab::LpMotionStabilizer::LpMotionStabilizer", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	pub fn new(model: crate::videostab::MotionModel) -> Result<crate::videostab::LpMotionStabilizer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(model, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::LpMotionStabilizer::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * model: MM_SIMILARITY
	// cv::videostab::LpMotionStabilizer::LpMotionStabilizer() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:120
	// ("cv::videostab::LpMotionStabilizer::LpMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::videostab::LpMotionStabilizer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_LpMotionStabilizer_LpMotionStabilizer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::LpMotionStabilizer::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LpMotionStabilizer, crate::videostab::IMotionStabilizer, cv_videostab_LpMotionStabilizer_to_IMotionStabilizer }

impl std::fmt::Debug for LpMotionStabilizer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LpMotionStabilizer")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MaskFrameSource]
// MaskFrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:89
pub trait MaskFrameSourceTraitConst: crate::videostab::IFrameSourceTraitConst {
	fn as_raw_MaskFrameSource(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::MaskFrameSource]
pub trait MaskFrameSourceTrait: crate::videostab::IFrameSourceTrait + crate::videostab::MaskFrameSourceTraitConst {
	fn as_raw_mut_MaskFrameSource(&mut self) -> *mut c_void;

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:94
	// ("cv::videostab::MaskFrameSource::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MaskFrameSource_reset(self.as_raw_mut_MaskFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:95
	// ("cv::videostab::MaskFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn next_frame(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MaskFrameSource_nextFrame(self.as_raw_mut_MaskFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// MaskFrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:89
pub struct MaskFrameSource {
	ptr: *mut c_void,
}

opencv_type_boxed! { MaskFrameSource }

impl Drop for MaskFrameSource {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MaskFrameSource_delete(self.as_raw_mut_MaskFrameSource()) };
	}
}

unsafe impl Send for MaskFrameSource {}

impl crate::videostab::IFrameSourceTraitConst for MaskFrameSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IFrameSourceTrait for MaskFrameSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MaskFrameSource, crate::videostab::IFrameSourceTraitConst, as_raw_IFrameSource, crate::videostab::IFrameSourceTrait, as_raw_mut_IFrameSource }

impl crate::videostab::MaskFrameSourceTraitConst for MaskFrameSource {
	#[inline] fn as_raw_MaskFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MaskFrameSourceTrait for MaskFrameSource {
	#[inline] fn as_raw_mut_MaskFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MaskFrameSource, crate::videostab::MaskFrameSourceTraitConst, as_raw_MaskFrameSource, crate::videostab::MaskFrameSourceTrait, as_raw_mut_MaskFrameSource }

impl MaskFrameSource {
	// MaskFrameSource(const Ptr<IFrameSource> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:92
	// ("cv::videostab::MaskFrameSource::MaskFrameSource", vec![(pred!(mut, ["source"], ["const cv::Ptr<cv::videostab::IFrameSource>*"]), _)]),
	#[inline]
	pub fn from_base(source: &core::Ptr<crate::videostab::IFrameSource>) -> Result<crate::videostab::MaskFrameSource> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MaskFrameSource_MaskFrameSource_const_PtrLIFrameSourceGR(source.as_raw_PtrOfIFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::MaskFrameSource::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MaskFrameSource, crate::videostab::IFrameSource, cv_videostab_MaskFrameSource_to_IFrameSource }

impl std::fmt::Debug for MaskFrameSource {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MaskFrameSource")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MoreAccurateMotionWobbleSuppressor]
// MoreAccurateMotionWobbleSuppressor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:113
pub trait MoreAccurateMotionWobbleSuppressorTraitConst: crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst {
	fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::MoreAccurateMotionWobbleSuppressor]
pub trait MoreAccurateMotionWobbleSuppressorTrait: crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait + crate::videostab::MoreAccurateMotionWobbleSuppressorTraitConst {
	fn as_raw_mut_MoreAccurateMotionWobbleSuppressor(&mut self) -> *mut c_void;

	// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:116
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn suppress(&mut self, idx: i32, frame: &impl core::MatTraitConst, result: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatR_MatR(self.as_raw_mut_MoreAccurateMotionWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MoreAccurateMotionWobbleSuppressor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:113
pub struct MoreAccurateMotionWobbleSuppressor {
	ptr: *mut c_void,
}

opencv_type_boxed! { MoreAccurateMotionWobbleSuppressor }

impl Drop for MoreAccurateMotionWobbleSuppressor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressor_delete(self.as_raw_mut_MoreAccurateMotionWobbleSuppressor()) };
	}
}

unsafe impl Send for MoreAccurateMotionWobbleSuppressor {}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressor, crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst, as_raw_MoreAccurateMotionWobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait, as_raw_mut_MoreAccurateMotionWobbleSuppressorBase }

impl crate::videostab::WobbleSuppressorBaseTraitConst for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressor, crate::videostab::WobbleSuppressorBaseTraitConst, as_raw_WobbleSuppressorBase, crate::videostab::WobbleSuppressorBaseTrait, as_raw_mut_WobbleSuppressorBase }

impl crate::videostab::MoreAccurateMotionWobbleSuppressorTraitConst for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorTrait for MoreAccurateMotionWobbleSuppressor {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressor, crate::videostab::MoreAccurateMotionWobbleSuppressorTraitConst, as_raw_MoreAccurateMotionWobbleSuppressor, crate::videostab::MoreAccurateMotionWobbleSuppressorTrait, as_raw_mut_MoreAccurateMotionWobbleSuppressor }

impl MoreAccurateMotionWobbleSuppressor {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_MoreAccurateMotionWobbleSuppressor_defaultNew_const()) }
	}

}

boxed_cast_base! { MoreAccurateMotionWobbleSuppressor, crate::videostab::MoreAccurateMotionWobbleSuppressorBase, cv_videostab_MoreAccurateMotionWobbleSuppressor_to_MoreAccurateMotionWobbleSuppressorBase }

boxed_cast_base! { MoreAccurateMotionWobbleSuppressor, crate::videostab::WobbleSuppressorBase, cv_videostab_MoreAccurateMotionWobbleSuppressor_to_WobbleSuppressorBase }

impl std::fmt::Debug for MoreAccurateMotionWobbleSuppressor {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MoreAccurateMotionWobbleSuppressor")
			.finish()
	}
}

impl Default for MoreAccurateMotionWobbleSuppressor {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::MoreAccurateMotionWobbleSuppressorBase]
// MoreAccurateMotionWobbleSuppressorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:101
pub trait MoreAccurateMotionWobbleSuppressorBaseTraitConst: crate::videostab::WobbleSuppressorBaseTraitConst {
	fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void;

	// period()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:105
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::period", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn period(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(self.as_raw_MoreAccurateMotionWobbleSuppressorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::MoreAccurateMotionWobbleSuppressorBase]
pub trait MoreAccurateMotionWobbleSuppressorBaseTrait: crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst + crate::videostab::WobbleSuppressorBaseTrait {
	fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void;

	// setPeriod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:104
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::setPeriod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_period(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(self.as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MoreAccurateMotionWobbleSuppressorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:101
pub struct MoreAccurateMotionWobbleSuppressorBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { MoreAccurateMotionWobbleSuppressorBase }

impl Drop for MoreAccurateMotionWobbleSuppressorBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorBase_delete(self.as_raw_mut_MoreAccurateMotionWobbleSuppressorBase()) };
	}
}

unsafe impl Send for MoreAccurateMotionWobbleSuppressorBase {}

impl crate::videostab::WobbleSuppressorBaseTraitConst for MoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for MoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressorBase, crate::videostab::WobbleSuppressorBaseTraitConst, as_raw_WobbleSuppressorBase, crate::videostab::WobbleSuppressorBaseTrait, as_raw_mut_WobbleSuppressorBase }

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for MoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for MoreAccurateMotionWobbleSuppressorBase {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst, as_raw_MoreAccurateMotionWobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait, as_raw_mut_MoreAccurateMotionWobbleSuppressorBase }

impl MoreAccurateMotionWobbleSuppressorBase {
}

boxed_cast_descendant! { MoreAccurateMotionWobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressor, cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressor }

boxed_cast_descendant! { MoreAccurateMotionWobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressorGpu, cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressorGpu }

boxed_cast_base! { MoreAccurateMotionWobbleSuppressorBase, crate::videostab::WobbleSuppressorBase, cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_WobbleSuppressorBase }

impl std::fmt::Debug for MoreAccurateMotionWobbleSuppressorBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MoreAccurateMotionWobbleSuppressorBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MoreAccurateMotionWobbleSuppressorGpu]
// MoreAccurateMotionWobbleSuppressorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:123
pub trait MoreAccurateMotionWobbleSuppressorGpuTraitConst: crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst {
	fn as_raw_MoreAccurateMotionWobbleSuppressorGpu(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::MoreAccurateMotionWobbleSuppressorGpu]
pub trait MoreAccurateMotionWobbleSuppressorGpuTrait: crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait + crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTraitConst {
	fn as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu(&mut self) -> *mut c_void;

	// suppress(int, const cuda::GpuMat &, cuda::GpuMat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:126
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn suppress(&mut self, idx: i32, frame: &impl core::GpuMatTraitConst, result: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_suppress_int_const_GpuMatR_GpuMatR(self.as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu(), idx, frame.as_raw_GpuMat(), result.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:127
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorGpu::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn suppress_1(&mut self, idx: i32, frame: &impl core::MatTraitConst, result: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_suppress_int_const_MatR_MatR(self.as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu(), idx, frame.as_raw_Mat(), result.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MoreAccurateMotionWobbleSuppressorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:123
pub struct MoreAccurateMotionWobbleSuppressorGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { MoreAccurateMotionWobbleSuppressorGpu }

impl Drop for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_delete(self.as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu()) };
	}
}

unsafe impl Send for MoreAccurateMotionWobbleSuppressorGpu {}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressorGpu, crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst, as_raw_MoreAccurateMotionWobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait, as_raw_mut_MoreAccurateMotionWobbleSuppressorBase }

impl crate::videostab::WobbleSuppressorBaseTraitConst for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressorGpu, crate::videostab::WobbleSuppressorBaseTraitConst, as_raw_WobbleSuppressorBase, crate::videostab::WobbleSuppressorBaseTrait, as_raw_mut_WobbleSuppressorBase }

impl crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTraitConst for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTrait for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MoreAccurateMotionWobbleSuppressorGpu, crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTraitConst, as_raw_MoreAccurateMotionWobbleSuppressorGpu, crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTrait, as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu }

impl MoreAccurateMotionWobbleSuppressorGpu {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_defaultNew_const()) }
	}

}

boxed_cast_base! { MoreAccurateMotionWobbleSuppressorGpu, crate::videostab::MoreAccurateMotionWobbleSuppressorBase, cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_to_MoreAccurateMotionWobbleSuppressorBase }

boxed_cast_base! { MoreAccurateMotionWobbleSuppressorGpu, crate::videostab::WobbleSuppressorBase, cv_videostab_MoreAccurateMotionWobbleSuppressorGpu_to_WobbleSuppressorBase }

impl std::fmt::Debug for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MoreAccurateMotionWobbleSuppressorGpu")
			.finish()
	}
}

impl Default for MoreAccurateMotionWobbleSuppressorGpu {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::MotionEstimatorBase]
// MotionEstimatorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:97
pub trait MotionEstimatorBaseTraitConst {
	fn as_raw_MotionEstimatorBase(&self) -> *const c_void;

	/// ## Returns
	/// Motion model. See cv::videostab::MotionModel.
	// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:111
	// ("cv::videostab::MotionEstimatorBase::motionModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorBase_motionModel_const(self.as_raw_MotionEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::MotionEstimatorBase]
pub trait MotionEstimatorBaseTrait: crate::videostab::MotionEstimatorBaseTraitConst {
	fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void;

	/// Sets motion model.
	///
	/// ## Parameters
	/// * val: Motion model. See cv::videostab::MotionModel.
	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:106
	// ("cv::videostab::MotionEstimatorBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(self.as_raw_mut_MotionEstimatorBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimates global motion between two 2D point clouds.
	///
	/// ## Parameters
	/// * points0: Source set of 2D points (32F).
	/// * points1: Destination set of 2D points (32F).
	/// * ok: Indicates whether motion was estimated successfully.
	/// ## Returns
	/// 3x3 2D transformation matrix (32F).
	///
	/// ## C++ default parameters
	/// * ok: 0
	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:120
	// ("cv::videostab::MotionEstimatorBase::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, points0: &impl ToInputArray, points1: &impl ToInputArray, ok: &mut bool) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR_boolX(self.as_raw_mut_MotionEstimatorBase(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Estimates global motion between two 2D point clouds.
	///
	/// ## Parameters
	/// * points0: Source set of 2D points (32F).
	/// * points1: Destination set of 2D points (32F).
	/// * ok: Indicates whether motion was estimated successfully.
	/// ## Returns
	/// 3x3 2D transformation matrix (32F).
	///
	/// ## Note
	/// This alternative version of [MotionEstimatorBaseTrait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::MotionEstimatorBase::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:120
	// ("cv::videostab::MotionEstimatorBase::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, points0: &impl ToInputArray, points1: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR(self.as_raw_mut_MotionEstimatorBase(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Base class for all global motion estimation methods.
// MotionEstimatorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:97
pub struct MotionEstimatorBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionEstimatorBase }

impl Drop for MotionEstimatorBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MotionEstimatorBase_delete(self.as_raw_mut_MotionEstimatorBase()) };
	}
}

unsafe impl Send for MotionEstimatorBase {}

impl crate::videostab::MotionEstimatorBaseTraitConst for MotionEstimatorBase {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionEstimatorBaseTrait for MotionEstimatorBase {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionEstimatorBase, crate::videostab::MotionEstimatorBaseTraitConst, as_raw_MotionEstimatorBase, crate::videostab::MotionEstimatorBaseTrait, as_raw_mut_MotionEstimatorBase }

impl MotionEstimatorBase {
}

boxed_cast_descendant! { MotionEstimatorBase, crate::videostab::MotionEstimatorL1, cv_videostab_MotionEstimatorBase_to_MotionEstimatorL1 }

boxed_cast_descendant! { MotionEstimatorBase, crate::videostab::MotionEstimatorRansacL2, cv_videostab_MotionEstimatorBase_to_MotionEstimatorRansacL2 }

impl std::fmt::Debug for MotionEstimatorBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionEstimatorBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MotionEstimatorL1]
// MotionEstimatorL1 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:153
pub trait MotionEstimatorL1TraitConst: crate::videostab::MotionEstimatorBaseTraitConst {
	fn as_raw_MotionEstimatorL1(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::MotionEstimatorL1]
pub trait MotionEstimatorL1Trait: crate::videostab::MotionEstimatorBaseTrait + crate::videostab::MotionEstimatorL1TraitConst {
	fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:158
	// ("cv::videostab::MotionEstimatorL1::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, points0: &impl ToInputArray, points1: &impl ToInputArray, ok: &mut bool) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR_boolX(self.as_raw_mut_MotionEstimatorL1(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [MotionEstimatorL1Trait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::MotionEstimatorL1::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:158
	// ("cv::videostab::MotionEstimatorL1::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, points0: &impl ToInputArray, points1: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR(self.as_raw_mut_MotionEstimatorL1(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Describes a global 2D motion estimation method which minimizes L1 error.
///
///
/// Note: To be able to use this method you must build OpenCV with CLP library support. :
// MotionEstimatorL1 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:153
pub struct MotionEstimatorL1 {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionEstimatorL1 }

impl Drop for MotionEstimatorL1 {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MotionEstimatorL1_delete(self.as_raw_mut_MotionEstimatorL1()) };
	}
}

unsafe impl Send for MotionEstimatorL1 {}

impl crate::videostab::MotionEstimatorBaseTraitConst for MotionEstimatorL1 {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionEstimatorBaseTrait for MotionEstimatorL1 {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionEstimatorL1, crate::videostab::MotionEstimatorBaseTraitConst, as_raw_MotionEstimatorBase, crate::videostab::MotionEstimatorBaseTrait, as_raw_mut_MotionEstimatorBase }

impl crate::videostab::MotionEstimatorL1TraitConst for MotionEstimatorL1 {
	#[inline] fn as_raw_MotionEstimatorL1(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionEstimatorL1Trait for MotionEstimatorL1 {
	#[inline] fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionEstimatorL1, crate::videostab::MotionEstimatorL1TraitConst, as_raw_MotionEstimatorL1, crate::videostab::MotionEstimatorL1Trait, as_raw_mut_MotionEstimatorL1 }

impl MotionEstimatorL1 {
	/// ## C++ default parameters
	/// * model: MM_AFFINE
	// MotionEstimatorL1(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:156
	// ("cv::videostab::MotionEstimatorL1::MotionEstimatorL1", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	pub fn new(model: crate::videostab::MotionModel) -> Result<crate::videostab::MotionEstimatorL1> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(model, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::MotionEstimatorL1::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * model: MM_AFFINE
	// cv::videostab::MotionEstimatorL1::MotionEstimatorL1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:156
	// ("cv::videostab::MotionEstimatorL1::MotionEstimatorL1", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::videostab::MotionEstimatorL1> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorL1_MotionEstimatorL1(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::MotionEstimatorL1::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MotionEstimatorL1, crate::videostab::MotionEstimatorBase, cv_videostab_MotionEstimatorL1_to_MotionEstimatorBase }

impl std::fmt::Debug for MotionEstimatorL1 {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionEstimatorL1")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MotionEstimatorRansacL2]
// MotionEstimatorRansacL2 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:131
pub trait MotionEstimatorRansacL2TraitConst: crate::videostab::MotionEstimatorBaseTraitConst {
	fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void;

	// ransacParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:137
	// ("cv::videostab::MotionEstimatorRansacL2::ransacParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ransac_params(&self) -> Result<crate::videostab::RansacParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_ransacParams_const(self.as_raw_MotionEstimatorRansacL2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::RansacParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	// minInlierRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:140
	// ("cv::videostab::MotionEstimatorRansacL2::minInlierRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_inlier_ratio(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(self.as_raw_MotionEstimatorRansacL2(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::MotionEstimatorRansacL2]
pub trait MotionEstimatorRansacL2Trait: crate::videostab::MotionEstimatorBaseTrait + crate::videostab::MotionEstimatorRansacL2TraitConst {
	fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void;

	// setRansacParams(const RansacParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:136
	// ("cv::videostab::MotionEstimatorRansacL2::setRansacParams", vec![(pred!(mut, ["val"], ["const cv::videostab::RansacParams*"]), _)]),
	#[inline]
	fn set_ransac_params(&mut self, val: &impl crate::videostab::RansacParamsTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsR(self.as_raw_mut_MotionEstimatorRansacL2(), val.as_raw_RansacParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinInlierRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:139
	// ("cv::videostab::MotionEstimatorRansacL2::setMinInlierRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_min_inlier_ratio(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(self.as_raw_mut_MotionEstimatorRansacL2(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:142
	// ("cv::videostab::MotionEstimatorRansacL2::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, points0: &impl ToInputArray, points1: &impl ToInputArray, ok: &mut bool) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR_boolX(self.as_raw_mut_MotionEstimatorRansacL2(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [MotionEstimatorRansacL2Trait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::MotionEstimatorRansacL2::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:142
	// ("cv::videostab::MotionEstimatorRansacL2::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, points0: &impl ToInputArray, points1: &impl ToInputArray) -> Result<core::Mat> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR(self.as_raw_mut_MotionEstimatorRansacL2(), points0.as_raw__InputArray(), points1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Describes a robust RANSAC-based global 2D motion estimation method which minimizes L2 error.
// MotionEstimatorRansacL2 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:131
pub struct MotionEstimatorRansacL2 {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionEstimatorRansacL2 }

impl Drop for MotionEstimatorRansacL2 {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_delete(self.as_raw_mut_MotionEstimatorRansacL2()) };
	}
}

unsafe impl Send for MotionEstimatorRansacL2 {}

impl crate::videostab::MotionEstimatorBaseTraitConst for MotionEstimatorRansacL2 {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionEstimatorBaseTrait for MotionEstimatorRansacL2 {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionEstimatorRansacL2, crate::videostab::MotionEstimatorBaseTraitConst, as_raw_MotionEstimatorBase, crate::videostab::MotionEstimatorBaseTrait, as_raw_mut_MotionEstimatorBase }

impl crate::videostab::MotionEstimatorRansacL2TraitConst for MotionEstimatorRansacL2 {
	#[inline] fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionEstimatorRansacL2Trait for MotionEstimatorRansacL2 {
	#[inline] fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionEstimatorRansacL2, crate::videostab::MotionEstimatorRansacL2TraitConst, as_raw_MotionEstimatorRansacL2, crate::videostab::MotionEstimatorRansacL2Trait, as_raw_mut_MotionEstimatorRansacL2 }

impl MotionEstimatorRansacL2 {
	/// ## C++ default parameters
	/// * model: MM_AFFINE
	// MotionEstimatorRansacL2(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:134
	// ("cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	pub fn new(model: crate::videostab::MotionModel) -> Result<crate::videostab::MotionEstimatorRansacL2> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(model, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::MotionEstimatorRansacL2::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * model: MM_AFFINE
	// cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:134
	// ("cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::videostab::MotionEstimatorRansacL2> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::MotionEstimatorRansacL2::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MotionEstimatorRansacL2, crate::videostab::MotionEstimatorBase, cv_videostab_MotionEstimatorRansacL2_to_MotionEstimatorBase }

impl std::fmt::Debug for MotionEstimatorRansacL2 {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionEstimatorRansacL2")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MotionFilterBase]
// MotionFilterBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:84
pub trait MotionFilterBaseTraitConst: crate::videostab::IMotionStabilizerTraitConst {
	fn as_raw_MotionFilterBase(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::MotionFilterBase]
pub trait MotionFilterBaseTrait: crate::videostab::IMotionStabilizerTrait + crate::videostab::MotionFilterBaseTraitConst {
	fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void;

	// stabilize(int, const std::vector<Mat> &, const Range &)(Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:89
	// ("cv::videostab::MotionFilterBase::stabilize", vec![(pred!(mut, ["idx", "motions", "range"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
	#[inline]
	fn stabilize(&mut self, idx: i32, motions: &core::Vector<core::Mat>, range: &impl core::RangeTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionFilterBase_stabilize_int_const_vectorLMatGR_const_RangeR(self.as_raw_mut_MotionFilterBase(), idx, motions.as_raw_VectorOfMat(), range.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:92
	// ("cv::videostab::MotionFilterBase::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
	#[inline]
	fn stabilize_1(&mut self, size: i32, motions: &core::Vector<core::Mat>, range: &impl core::RangeTraitConst, stabilization_motions: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionFilterBase_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(self.as_raw_mut_MotionFilterBase(), size, motions.as_raw_VectorOfMat(), range.as_raw_Range(), stabilization_motions.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MotionFilterBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:84
pub struct MotionFilterBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionFilterBase }

impl Drop for MotionFilterBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MotionFilterBase_delete(self.as_raw_mut_MotionFilterBase()) };
	}
}

unsafe impl Send for MotionFilterBase {}

impl crate::videostab::IMotionStabilizerTraitConst for MotionFilterBase {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for MotionFilterBase {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionFilterBase, crate::videostab::IMotionStabilizerTraitConst, as_raw_IMotionStabilizer, crate::videostab::IMotionStabilizerTrait, as_raw_mut_IMotionStabilizer }

impl crate::videostab::MotionFilterBaseTraitConst for MotionFilterBase {
	#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionFilterBaseTrait for MotionFilterBase {
	#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionFilterBase, crate::videostab::MotionFilterBaseTraitConst, as_raw_MotionFilterBase, crate::videostab::MotionFilterBaseTrait, as_raw_mut_MotionFilterBase }

impl MotionFilterBase {
}

boxed_cast_descendant! { MotionFilterBase, crate::videostab::GaussianMotionFilter, cv_videostab_MotionFilterBase_to_GaussianMotionFilter }

boxed_cast_base! { MotionFilterBase, crate::videostab::IMotionStabilizer, cv_videostab_MotionFilterBase_to_IMotionStabilizer }

impl std::fmt::Debug for MotionFilterBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionFilterBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MotionInpainter]
// MotionInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:141
pub trait MotionInpainterTraitConst: crate::videostab::InpainterBaseTraitConst {
	fn as_raw_MotionInpainter(&self) -> *const c_void;

	// optFlowEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:147
	// ("cv::videostab::MotionInpainter::optFlowEstimator", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn opt_flow_estimator(&self) -> Result<core::Ptr<crate::videostab::IDenseOptFlowEstimator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_optFlowEstimator_const(self.as_raw_MotionInpainter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::IDenseOptFlowEstimator>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// flowErrorThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:150
	// ("cv::videostab::MotionInpainter::flowErrorThreshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn flow_error_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_flowErrorThreshold_const(self.as_raw_MotionInpainter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// distThresh()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:153
	// ("cv::videostab::MotionInpainter::distThresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn dist_thresh(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_distThresh_const(self.as_raw_MotionInpainter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// borderMode()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:156
	// ("cv::videostab::MotionInpainter::borderMode", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn border_mode(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_borderMode_const(self.as_raw_MotionInpainter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::MotionInpainter]
pub trait MotionInpainterTrait: crate::videostab::InpainterBaseTrait + crate::videostab::MotionInpainterTraitConst {
	fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void;

	// setOptFlowEstimator(Ptr<IDenseOptFlowEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:146
	// ("cv::videostab::MotionInpainter::setOptFlowEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IDenseOptFlowEstimator>"]), _)]),
	#[inline]
	fn set_opt_flow_estimator(&mut self, mut val: core::Ptr<crate::videostab::IDenseOptFlowEstimator>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_setOptFlowEstimator_PtrLIDenseOptFlowEstimatorG(self.as_raw_mut_MotionInpainter(), val.as_raw_mut_PtrOfIDenseOptFlowEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFlowErrorThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:149
	// ("cv::videostab::MotionInpainter::setFlowErrorThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_flow_error_threshold(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_setFlowErrorThreshold_float(self.as_raw_mut_MotionInpainter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDistThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:152
	// ("cv::videostab::MotionInpainter::setDistThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_dist_threshold(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_setDistThreshold_float(self.as_raw_mut_MotionInpainter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBorderMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:155
	// ("cv::videostab::MotionInpainter::setBorderMode", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_border_mode(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_setBorderMode_int(self.as_raw_mut_MotionInpainter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:158
	// ("cv::videostab::MotionInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn inpaint(&mut self, idx: i32, frame: &mut impl core::MatTrait, mask: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_MotionInpainter(), idx, frame.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MotionInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:141
pub struct MotionInpainter {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionInpainter }

impl Drop for MotionInpainter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MotionInpainter_delete(self.as_raw_mut_MotionInpainter()) };
	}
}

unsafe impl Send for MotionInpainter {}

impl crate::videostab::InpainterBaseTraitConst for MotionInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpainterBaseTrait for MotionInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionInpainter, crate::videostab::InpainterBaseTraitConst, as_raw_InpainterBase, crate::videostab::InpainterBaseTrait, as_raw_mut_InpainterBase }

impl crate::videostab::MotionInpainterTraitConst for MotionInpainter {
	#[inline] fn as_raw_MotionInpainter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionInpainterTrait for MotionInpainter {
	#[inline] fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionInpainter, crate::videostab::MotionInpainterTraitConst, as_raw_MotionInpainter, crate::videostab::MotionInpainterTrait, as_raw_mut_MotionInpainter }

impl MotionInpainter {
	// MotionInpainter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:144
	// ("cv::videostab::MotionInpainter::MotionInpainter", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::MotionInpainter> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionInpainter_MotionInpainter(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::MotionInpainter::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MotionInpainter, crate::videostab::InpainterBase, cv_videostab_MotionInpainter_to_InpainterBase }

impl std::fmt::Debug for MotionInpainter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionInpainter")
			.finish()
	}
}

/// Constant methods for [crate::videostab::MotionStabilizationPipeline]
// MotionStabilizationPipeline /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:70
pub trait MotionStabilizationPipelineTraitConst: crate::videostab::IMotionStabilizerTraitConst {
	fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void;

	// empty()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:74
	// ("cv::videostab::MotionStabilizationPipeline::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionStabilizationPipeline_empty_const(self.as_raw_MotionStabilizationPipeline(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::MotionStabilizationPipeline]
pub trait MotionStabilizationPipelineTrait: crate::videostab::IMotionStabilizerTrait + crate::videostab::MotionStabilizationPipelineTraitConst {
	fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void;

	// pushBack(Ptr<IMotionStabilizer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:73
	// ("cv::videostab::MotionStabilizationPipeline::pushBack", vec![(pred!(mut, ["stabilizer"], ["cv::Ptr<cv::videostab::IMotionStabilizer>"]), _)]),
	#[inline]
	fn push_back(&mut self, mut stabilizer: core::Ptr<crate::videostab::IMotionStabilizer>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionStabilizationPipeline_pushBack_PtrLIMotionStabilizerG(self.as_raw_mut_MotionStabilizationPipeline(), stabilizer.as_raw_mut_PtrOfIMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// stabilize(int, const std::vector<Mat> &, const Range &, Mat *)(Primitive, CppPassByVoidPtr, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:76
	// ("cv::videostab::MotionStabilizationPipeline::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "const cv::Range*", "cv::Mat*"]), _)]),
	#[inline]
	fn stabilize(&mut self, size: i32, motions: &core::Vector<core::Mat>, range: &impl core::RangeTraitConst, stabilization_motions: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_MotionStabilizationPipeline_stabilize_int_const_vectorLMatGR_const_RangeR_MatX(self.as_raw_mut_MotionStabilizationPipeline(), size, motions.as_raw_VectorOfMat(), range.as_raw_Range(), stabilization_motions.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// MotionStabilizationPipeline /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_stabilizing.hpp:70
pub struct MotionStabilizationPipeline {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionStabilizationPipeline }

impl Drop for MotionStabilizationPipeline {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_MotionStabilizationPipeline_delete(self.as_raw_mut_MotionStabilizationPipeline()) };
	}
}

unsafe impl Send for MotionStabilizationPipeline {}

impl crate::videostab::IMotionStabilizerTraitConst for MotionStabilizationPipeline {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for MotionStabilizationPipeline {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionStabilizationPipeline, crate::videostab::IMotionStabilizerTraitConst, as_raw_IMotionStabilizer, crate::videostab::IMotionStabilizerTrait, as_raw_mut_IMotionStabilizer }

impl crate::videostab::MotionStabilizationPipelineTraitConst for MotionStabilizationPipeline {
	#[inline] fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::MotionStabilizationPipelineTrait for MotionStabilizationPipeline {
	#[inline] fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionStabilizationPipeline, crate::videostab::MotionStabilizationPipelineTraitConst, as_raw_MotionStabilizationPipeline, crate::videostab::MotionStabilizationPipelineTrait, as_raw_mut_MotionStabilizationPipeline }

impl MotionStabilizationPipeline {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_MotionStabilizationPipeline_defaultNew_const()) }
	}

}

boxed_cast_base! { MotionStabilizationPipeline, crate::videostab::IMotionStabilizer, cv_videostab_MotionStabilizationPipeline_to_IMotionStabilizer }

impl std::fmt::Debug for MotionStabilizationPipeline {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionStabilizationPipeline")
			.finish()
	}
}

impl Default for MotionStabilizationPipeline {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::NullDeblurer]
// NullDeblurer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:90
pub trait NullDeblurerTraitConst: crate::videostab::DeblurerBaseTraitConst {
	fn as_raw_NullDeblurer(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::NullDeblurer]
pub trait NullDeblurerTrait: crate::videostab::DeblurerBaseTrait + crate::videostab::NullDeblurerTraitConst {
	fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void;

	// deblur(int, Mat &, const Range &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:93
	// ("cv::videostab::NullDeblurer::deblur", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["int", "cv::Mat*", "const cv::Range*"]), _)]),
	#[inline]
	fn deblur(&mut self, unnamed: i32, unnamed_1: &mut impl core::MatTrait, unnamed_2: &impl core::RangeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_NullDeblurer_deblur_int_MatR_const_RangeR(self.as_raw_mut_NullDeblurer(), unnamed, unnamed_1.as_raw_mut_Mat(), unnamed_2.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// NullDeblurer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:90
pub struct NullDeblurer {
	ptr: *mut c_void,
}

opencv_type_boxed! { NullDeblurer }

impl Drop for NullDeblurer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_NullDeblurer_delete(self.as_raw_mut_NullDeblurer()) };
	}
}

unsafe impl Send for NullDeblurer {}

impl crate::videostab::DeblurerBaseTraitConst for NullDeblurer {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::DeblurerBaseTrait for NullDeblurer {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullDeblurer, crate::videostab::DeblurerBaseTraitConst, as_raw_DeblurerBase, crate::videostab::DeblurerBaseTrait, as_raw_mut_DeblurerBase }

impl crate::videostab::NullDeblurerTraitConst for NullDeblurer {
	#[inline] fn as_raw_NullDeblurer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::NullDeblurerTrait for NullDeblurer {
	#[inline] fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullDeblurer, crate::videostab::NullDeblurerTraitConst, as_raw_NullDeblurer, crate::videostab::NullDeblurerTrait, as_raw_mut_NullDeblurer }

impl NullDeblurer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_NullDeblurer_defaultNew_const()) }
	}

}

boxed_cast_base! { NullDeblurer, crate::videostab::DeblurerBase, cv_videostab_NullDeblurer_to_DeblurerBase }

impl std::fmt::Debug for NullDeblurer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("NullDeblurer")
			.finish()
	}
}

impl Default for NullDeblurer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::NullFrameSource]
// NullFrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:65
pub trait NullFrameSourceTraitConst: crate::videostab::IFrameSourceTraitConst {
	fn as_raw_NullFrameSource(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::NullFrameSource]
pub trait NullFrameSourceTrait: crate::videostab::IFrameSourceTrait + crate::videostab::NullFrameSourceTraitConst {
	fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void;

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:68
	// ("cv::videostab::NullFrameSource::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_NullFrameSource_reset(self.as_raw_mut_NullFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:69
	// ("cv::videostab::NullFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn next_frame(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_NullFrameSource_nextFrame(self.as_raw_mut_NullFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// NullFrameSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:65
pub struct NullFrameSource {
	ptr: *mut c_void,
}

opencv_type_boxed! { NullFrameSource }

impl Drop for NullFrameSource {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_NullFrameSource_delete(self.as_raw_mut_NullFrameSource()) };
	}
}

unsafe impl Send for NullFrameSource {}

impl crate::videostab::IFrameSourceTraitConst for NullFrameSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IFrameSourceTrait for NullFrameSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullFrameSource, crate::videostab::IFrameSourceTraitConst, as_raw_IFrameSource, crate::videostab::IFrameSourceTrait, as_raw_mut_IFrameSource }

impl crate::videostab::NullFrameSourceTraitConst for NullFrameSource {
	#[inline] fn as_raw_NullFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::NullFrameSourceTrait for NullFrameSource {
	#[inline] fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullFrameSource, crate::videostab::NullFrameSourceTraitConst, as_raw_NullFrameSource, crate::videostab::NullFrameSourceTrait, as_raw_mut_NullFrameSource }

impl NullFrameSource {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_NullFrameSource_defaultNew_const()) }
	}

}

boxed_cast_base! { NullFrameSource, crate::videostab::IFrameSource, cv_videostab_NullFrameSource_to_IFrameSource }

impl std::fmt::Debug for NullFrameSource {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("NullFrameSource")
			.finish()
	}
}

impl Default for NullFrameSource {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::NullInpainter]
// NullInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:102
pub trait NullInpainterTraitConst: crate::videostab::InpainterBaseTraitConst {
	fn as_raw_NullInpainter(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::NullInpainter]
pub trait NullInpainterTrait: crate::videostab::InpainterBaseTrait + crate::videostab::NullInpainterTraitConst {
	fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void;

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:105
	// ("cv::videostab::NullInpainter::inpaint", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn inpaint(&mut self, unnamed: i32, unnamed_1: &mut impl core::MatTrait, unnamed_2: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_NullInpainter_inpaint_int_MatR_MatR(self.as_raw_mut_NullInpainter(), unnamed, unnamed_1.as_raw_mut_Mat(), unnamed_2.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// NullInpainter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/inpainting.hpp:102
pub struct NullInpainter {
	ptr: *mut c_void,
}

opencv_type_boxed! { NullInpainter }

impl Drop for NullInpainter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_NullInpainter_delete(self.as_raw_mut_NullInpainter()) };
	}
}

unsafe impl Send for NullInpainter {}

impl crate::videostab::InpainterBaseTraitConst for NullInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::InpainterBaseTrait for NullInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullInpainter, crate::videostab::InpainterBaseTraitConst, as_raw_InpainterBase, crate::videostab::InpainterBaseTrait, as_raw_mut_InpainterBase }

impl crate::videostab::NullInpainterTraitConst for NullInpainter {
	#[inline] fn as_raw_NullInpainter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::NullInpainterTrait for NullInpainter {
	#[inline] fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullInpainter, crate::videostab::NullInpainterTraitConst, as_raw_NullInpainter, crate::videostab::NullInpainterTrait, as_raw_mut_NullInpainter }

impl NullInpainter {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_NullInpainter_defaultNew_const()) }
	}

}

boxed_cast_base! { NullInpainter, crate::videostab::InpainterBase, cv_videostab_NullInpainter_to_InpainterBase }

impl std::fmt::Debug for NullInpainter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("NullInpainter")
			.finish()
	}
}

impl Default for NullInpainter {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::NullLog]
// NullLog /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:63
pub trait NullLogTraitConst: crate::videostab::ILogTraitConst {
	fn as_raw_NullLog(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::NullLog]
pub trait NullLogTrait: crate::videostab::ILogTrait + crate::videostab::NullLogTraitConst {
	fn as_raw_mut_NullLog(&mut self) -> *mut c_void;

	// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:66
	// ("cv::videostab::NullLog::print", vec![(pred!(mut, ["unnamed"], ["const char*"]), _)]),
	#[inline]
	fn print(&mut self, unnamed: &str) -> Result<()> {
		extern_container_arg!(unnamed);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_NullLog_print_const_charX(self.as_raw_mut_NullLog(), unnamed.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// NullLog /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/log.hpp:63
pub struct NullLog {
	ptr: *mut c_void,
}

opencv_type_boxed! { NullLog }

impl Drop for NullLog {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_NullLog_delete(self.as_raw_mut_NullLog()) };
	}
}

unsafe impl Send for NullLog {}

impl crate::videostab::ILogTraitConst for NullLog {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ILogTrait for NullLog {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullLog, crate::videostab::ILogTraitConst, as_raw_ILog, crate::videostab::ILogTrait, as_raw_mut_ILog }

impl crate::videostab::NullLogTraitConst for NullLog {
	#[inline] fn as_raw_NullLog(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::NullLogTrait for NullLog {
	#[inline] fn as_raw_mut_NullLog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullLog, crate::videostab::NullLogTraitConst, as_raw_NullLog, crate::videostab::NullLogTrait, as_raw_mut_NullLog }

impl NullLog {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_NullLog_defaultNew_const()) }
	}

}

boxed_cast_base! { NullLog, crate::videostab::ILog, cv_videostab_NullLog_to_ILog }

impl std::fmt::Debug for NullLog {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("NullLog")
			.finish()
	}
}

impl Default for NullLog {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::NullOutlierRejector]
// NullOutlierRejector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:67
pub trait NullOutlierRejectorTraitConst: crate::videostab::IOutlierRejectorTraitConst {
	fn as_raw_NullOutlierRejector(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::NullOutlierRejector]
pub trait NullOutlierRejectorTrait: crate::videostab::IOutlierRejectorTrait + crate::videostab::NullOutlierRejectorTraitConst {
	fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void;

	// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:70
	// ("cv::videostab::NullOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn process(&mut self, frame_size: core::Size, points0: &impl ToInputArray, points1: &impl ToInputArray, mask: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_NullOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_NullOutlierRejector(), &frame_size, points0.as_raw__InputArray(), points1.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// NullOutlierRejector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:67
pub struct NullOutlierRejector {
	ptr: *mut c_void,
}

opencv_type_boxed! { NullOutlierRejector }

impl Drop for NullOutlierRejector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_NullOutlierRejector_delete(self.as_raw_mut_NullOutlierRejector()) };
	}
}

unsafe impl Send for NullOutlierRejector {}

impl crate::videostab::IOutlierRejectorTraitConst for NullOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IOutlierRejectorTrait for NullOutlierRejector {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullOutlierRejector, crate::videostab::IOutlierRejectorTraitConst, as_raw_IOutlierRejector, crate::videostab::IOutlierRejectorTrait, as_raw_mut_IOutlierRejector }

impl crate::videostab::NullOutlierRejectorTraitConst for NullOutlierRejector {
	#[inline] fn as_raw_NullOutlierRejector(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::NullOutlierRejectorTrait for NullOutlierRejector {
	#[inline] fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullOutlierRejector, crate::videostab::NullOutlierRejectorTraitConst, as_raw_NullOutlierRejector, crate::videostab::NullOutlierRejectorTrait, as_raw_mut_NullOutlierRejector }

impl NullOutlierRejector {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_NullOutlierRejector_defaultNew_const()) }
	}

}

boxed_cast_base! { NullOutlierRejector, crate::videostab::IOutlierRejector, cv_videostab_NullOutlierRejector_to_IOutlierRejector }

impl std::fmt::Debug for NullOutlierRejector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("NullOutlierRejector")
			.finish()
	}
}

impl Default for NullOutlierRejector {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::NullWobbleSuppressor]
// NullWobbleSuppressor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:95
pub trait NullWobbleSuppressorTraitConst: crate::videostab::WobbleSuppressorBaseTraitConst {
	fn as_raw_NullWobbleSuppressor(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::NullWobbleSuppressor]
pub trait NullWobbleSuppressorTrait: crate::videostab::NullWobbleSuppressorTraitConst + crate::videostab::WobbleSuppressorBaseTrait {
	fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void;

	// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:98
	// ("cv::videostab::NullWobbleSuppressor::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn suppress(&mut self, idx: i32, frame: &impl core::MatTraitConst, result: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_NullWobbleSuppressor_suppress_int_const_MatR_MatR(self.as_raw_mut_NullWobbleSuppressor(), idx, frame.as_raw_Mat(), result.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// NullWobbleSuppressor /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:95
pub struct NullWobbleSuppressor {
	ptr: *mut c_void,
}

opencv_type_boxed! { NullWobbleSuppressor }

impl Drop for NullWobbleSuppressor {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_NullWobbleSuppressor_delete(self.as_raw_mut_NullWobbleSuppressor()) };
	}
}

unsafe impl Send for NullWobbleSuppressor {}

impl crate::videostab::WobbleSuppressorBaseTraitConst for NullWobbleSuppressor {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for NullWobbleSuppressor {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullWobbleSuppressor, crate::videostab::WobbleSuppressorBaseTraitConst, as_raw_WobbleSuppressorBase, crate::videostab::WobbleSuppressorBaseTrait, as_raw_mut_WobbleSuppressorBase }

impl crate::videostab::NullWobbleSuppressorTraitConst for NullWobbleSuppressor {
	#[inline] fn as_raw_NullWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::NullWobbleSuppressorTrait for NullWobbleSuppressor {
	#[inline] fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NullWobbleSuppressor, crate::videostab::NullWobbleSuppressorTraitConst, as_raw_NullWobbleSuppressor, crate::videostab::NullWobbleSuppressorTrait, as_raw_mut_NullWobbleSuppressor }

impl NullWobbleSuppressor {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_NullWobbleSuppressor_defaultNew_const()) }
	}

}

boxed_cast_base! { NullWobbleSuppressor, crate::videostab::WobbleSuppressorBase, cv_videostab_NullWobbleSuppressor_to_WobbleSuppressorBase }

impl std::fmt::Debug for NullWobbleSuppressor {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("NullWobbleSuppressor")
			.finish()
	}
}

impl Default for NullWobbleSuppressor {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::OnePassStabilizer]
// OnePassStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:143
pub trait OnePassStabilizerTraitConst: crate::videostab::IFrameSourceTraitConst + crate::videostab::StabilizerBaseTraitConst {
	fn as_raw_OnePassStabilizer(&self) -> *const c_void;

	// motionFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:149
	// ("cv::videostab::OnePassStabilizer::motionFilter", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_filter(&self) -> Result<core::Ptr<crate::videostab::MotionFilterBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_OnePassStabilizer_motionFilter_const(self.as_raw_OnePassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::MotionFilterBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::OnePassStabilizer]
pub trait OnePassStabilizerTrait: crate::videostab::IFrameSourceTrait + crate::videostab::OnePassStabilizerTraitConst + crate::videostab::StabilizerBaseTrait {
	fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void;

	// setMotionFilter(Ptr<MotionFilterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:148
	// ("cv::videostab::OnePassStabilizer::setMotionFilter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::MotionFilterBase>"]), _)]),
	#[inline]
	fn set_motion_filter(&mut self, mut val: core::Ptr<crate::videostab::MotionFilterBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_OnePassStabilizer_setMotionFilter_PtrLMotionFilterBaseG(self.as_raw_mut_OnePassStabilizer(), val.as_raw_mut_PtrOfMotionFilterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:151
	// ("cv::videostab::OnePassStabilizer::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_OnePassStabilizer_reset(self.as_raw_mut_OnePassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:152
	// ("cv::videostab::OnePassStabilizer::nextFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn next_frame(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_OnePassStabilizer_nextFrame(self.as_raw_mut_OnePassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// OnePassStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:143
pub struct OnePassStabilizer {
	ptr: *mut c_void,
}

opencv_type_boxed! { OnePassStabilizer }

impl Drop for OnePassStabilizer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_OnePassStabilizer_delete(self.as_raw_mut_OnePassStabilizer()) };
	}
}

unsafe impl Send for OnePassStabilizer {}

impl crate::videostab::IFrameSourceTraitConst for OnePassStabilizer {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IFrameSourceTrait for OnePassStabilizer {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OnePassStabilizer, crate::videostab::IFrameSourceTraitConst, as_raw_IFrameSource, crate::videostab::IFrameSourceTrait, as_raw_mut_IFrameSource }

impl crate::videostab::StabilizerBaseTraitConst for OnePassStabilizer {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::StabilizerBaseTrait for OnePassStabilizer {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OnePassStabilizer, crate::videostab::StabilizerBaseTraitConst, as_raw_StabilizerBase, crate::videostab::StabilizerBaseTrait, as_raw_mut_StabilizerBase }

impl crate::videostab::OnePassStabilizerTraitConst for OnePassStabilizer {
	#[inline] fn as_raw_OnePassStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::OnePassStabilizerTrait for OnePassStabilizer {
	#[inline] fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { OnePassStabilizer, crate::videostab::OnePassStabilizerTraitConst, as_raw_OnePassStabilizer, crate::videostab::OnePassStabilizerTrait, as_raw_mut_OnePassStabilizer }

impl OnePassStabilizer {
	// OnePassStabilizer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:146
	// ("cv::videostab::OnePassStabilizer::OnePassStabilizer", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::OnePassStabilizer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_OnePassStabilizer_OnePassStabilizer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::OnePassStabilizer::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { OnePassStabilizer, crate::videostab::IFrameSource, cv_videostab_OnePassStabilizer_to_IFrameSource }

boxed_cast_base! { OnePassStabilizer, crate::videostab::StabilizerBase, cv_videostab_OnePassStabilizer_to_StabilizerBase }

impl std::fmt::Debug for OnePassStabilizer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("OnePassStabilizer")
			.finish()
	}
}

/// Constant methods for [crate::videostab::PyrLkOptFlowEstimatorBase]
// PyrLkOptFlowEstimatorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:79
pub trait PyrLkOptFlowEstimatorBaseTraitConst {
	fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void;

	// winSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:85
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::winSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn win_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(self.as_raw_PyrLkOptFlowEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// maxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:88
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::maxLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_level(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(self.as_raw_PyrLkOptFlowEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::PyrLkOptFlowEstimatorBase]
pub trait PyrLkOptFlowEstimatorBaseTrait: crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst {
	fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void;

	// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:84
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::setWinSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	#[inline]
	fn set_win_size(&mut self, val: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(self.as_raw_mut_PyrLkOptFlowEstimatorBase(), &val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:87
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_max_level(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(self.as_raw_mut_PyrLkOptFlowEstimatorBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// PyrLkOptFlowEstimatorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:79
pub struct PyrLkOptFlowEstimatorBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { PyrLkOptFlowEstimatorBase }

impl Drop for PyrLkOptFlowEstimatorBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_delete(self.as_raw_mut_PyrLkOptFlowEstimatorBase()) };
	}
}

unsafe impl Send for PyrLkOptFlowEstimatorBase {}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for PyrLkOptFlowEstimatorBase {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for PyrLkOptFlowEstimatorBase {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PyrLkOptFlowEstimatorBase, crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst, as_raw_PyrLkOptFlowEstimatorBase, crate::videostab::PyrLkOptFlowEstimatorBaseTrait, as_raw_mut_PyrLkOptFlowEstimatorBase }

impl PyrLkOptFlowEstimatorBase {
	// PyrLkOptFlowEstimatorBase()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:82
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::PyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::PyrLkOptFlowEstimatorBase> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::PyrLkOptFlowEstimatorBase::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { PyrLkOptFlowEstimatorBase, crate::videostab::DensePyrLkOptFlowEstimatorGpu, cv_videostab_PyrLkOptFlowEstimatorBase_to_DensePyrLkOptFlowEstimatorGpu }

boxed_cast_descendant! { PyrLkOptFlowEstimatorBase, crate::videostab::SparsePyrLkOptFlowEstimator, cv_videostab_PyrLkOptFlowEstimatorBase_to_SparsePyrLkOptFlowEstimator }

boxed_cast_descendant! { PyrLkOptFlowEstimatorBase, crate::videostab::SparsePyrLkOptFlowEstimatorGpu, cv_videostab_PyrLkOptFlowEstimatorBase_to_SparsePyrLkOptFlowEstimatorGpu }

impl std::fmt::Debug for PyrLkOptFlowEstimatorBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PyrLkOptFlowEstimatorBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::RansacParams]
// RansacParams /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:73
pub trait RansacParamsTraitConst {
	fn as_raw_RansacParams(&self) -> *const c_void;

	/// subset size
	// cv::videostab::RansacParams::size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:75
	// ("cv::videostab::RansacParams::size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn size(&self) -> i32 {
		let ret = unsafe { sys::cv_videostab_RansacParams_propSize_const(self.as_raw_RansacParams()) };
		ret
	}

	/// max error to classify as inlier
	// cv::videostab::RansacParams::thresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:76
	// ("cv::videostab::RansacParams::thresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_videostab_RansacParams_propThresh_const(self.as_raw_RansacParams()) };
		ret
	}

	/// max outliers ratio
	// cv::videostab::RansacParams::eps() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:77
	// ("cv::videostab::RansacParams::eps", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn eps(&self) -> f32 {
		let ret = unsafe { sys::cv_videostab_RansacParams_propEps_const(self.as_raw_RansacParams()) };
		ret
	}

	/// probability of success
	// cv::videostab::RansacParams::prob() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:78
	// ("cv::videostab::RansacParams::prob", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn prob(&self) -> f32 {
		let ret = unsafe { sys::cv_videostab_RansacParams_propProb_const(self.as_raw_RansacParams()) };
		ret
	}

	/// ## Returns
	/// Number of iterations that'll be performed by RANSAC method.
	// niters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:92
	// ("cv::videostab::RansacParams::niters", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn niters(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_RansacParams_niters_const(self.as_raw_RansacParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::RansacParams]
pub trait RansacParamsTrait: crate::videostab::RansacParamsTraitConst {
	fn as_raw_mut_RansacParams(&mut self) -> *mut c_void;

	/// subset size
	// cv::videostab::RansacParams::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:75
	// ("cv::videostab::RansacParams::setSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_videostab_RansacParams_propSize_const_int(self.as_raw_mut_RansacParams(), val) };
		ret
	}

	/// max error to classify as inlier
	// cv::videostab::RansacParams::setThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:76
	// ("cv::videostab::RansacParams::setThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_videostab_RansacParams_propThresh_const_float(self.as_raw_mut_RansacParams(), val) };
		ret
	}

	/// max outliers ratio
	// cv::videostab::RansacParams::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:77
	// ("cv::videostab::RansacParams::setEps", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_eps(&mut self, val: f32) {
		let ret = unsafe { sys::cv_videostab_RansacParams_propEps_const_float(self.as_raw_mut_RansacParams(), val) };
		ret
	}

	/// probability of success
	// cv::videostab::RansacParams::setProb(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:78
	// ("cv::videostab::RansacParams::setProb", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_prob(&mut self, val: f32) {
		let ret = unsafe { sys::cv_videostab_RansacParams_propProb_const_float(self.as_raw_mut_RansacParams(), val) };
		ret
	}

}

/// Describes RANSAC method parameters.
// RansacParams /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:73
pub struct RansacParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { RansacParams }

impl Drop for RansacParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_RansacParams_delete(self.as_raw_mut_RansacParams()) };
	}
}

unsafe impl Send for RansacParams {}

impl crate::videostab::RansacParamsTraitConst for RansacParams {
	#[inline] fn as_raw_RansacParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::RansacParamsTrait for RansacParams {
	#[inline] fn as_raw_mut_RansacParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RansacParams, crate::videostab::RansacParamsTraitConst, as_raw_RansacParams, crate::videostab::RansacParamsTrait, as_raw_mut_RansacParams }

impl RansacParams {
	// RansacParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:80
	// ("cv::videostab::RansacParams::RansacParams", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::RansacParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_RansacParams_RansacParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::RansacParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * size: Subset size.
	/// * thresh: Maximum re-projection error value to classify as inlier.
	/// * eps: Maximum ratio of incorrect correspondences.
	/// * prob: Required success probability.
	// RansacParams(int, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:87
	// ("cv::videostab::RansacParams::RansacParams", vec![(pred!(mut, ["size", "thresh", "eps", "prob"], ["int", "float", "float", "float"]), _)]),
	#[inline]
	pub fn new(size: i32, thresh: f32, eps: f32, prob: f32) -> Result<crate::videostab::RansacParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_RansacParams_RansacParams_int_float_float_float(size, thresh, eps, prob, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::RansacParams::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Parameters
	/// * model: Motion model. See cv::videostab::MotionModel.
	/// ## Returns
	/// Default RANSAC method parameters for the given motion model.
	// default2dMotion(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/motion_core.hpp:102
	// ("cv::videostab::RansacParams::default2dMotion", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	pub fn default2d_motion(model: crate::videostab::MotionModel) -> Result<crate::videostab::RansacParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_RansacParams_default2dMotion_MotionModel(model, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::RansacParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for RansacParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RansacParams")
			.field("size", &crate::videostab::RansacParamsTraitConst::size(self))
			.field("thresh", &crate::videostab::RansacParamsTraitConst::thresh(self))
			.field("eps", &crate::videostab::RansacParamsTraitConst::eps(self))
			.field("prob", &crate::videostab::RansacParamsTraitConst::prob(self))
			.finish()
	}
}

/// Constant methods for [crate::videostab::SparsePyrLkOptFlowEstimator]
// SparsePyrLkOptFlowEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:96
pub trait SparsePyrLkOptFlowEstimatorTraitConst: crate::videostab::ISparseOptFlowEstimatorTraitConst + crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst {
	fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::SparsePyrLkOptFlowEstimator]
pub trait SparsePyrLkOptFlowEstimatorTrait: crate::videostab::ISparseOptFlowEstimatorTrait + crate::videostab::PyrLkOptFlowEstimatorBaseTrait + crate::videostab::SparsePyrLkOptFlowEstimatorTraitConst {
	fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void;

	// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:100
	// ("cv::videostab::SparsePyrLkOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, points0: &impl ToInputArray, points1: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, errors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_array_arg!(points0);
		input_output_array_arg!(points1);
		output_array_arg!(status);
		output_array_arg!(errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SparsePyrLkOptFlowEstimator(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), points0.as_raw__InputArray(), points1.as_raw__InputOutputArray(), status.as_raw__OutputArray(), errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// SparsePyrLkOptFlowEstimator /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:96
pub struct SparsePyrLkOptFlowEstimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { SparsePyrLkOptFlowEstimator }

impl Drop for SparsePyrLkOptFlowEstimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimator_delete(self.as_raw_mut_SparsePyrLkOptFlowEstimator()) };
	}
}

unsafe impl Send for SparsePyrLkOptFlowEstimator {}

impl crate::videostab::ISparseOptFlowEstimatorTraitConst for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimatorTrait for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLkOptFlowEstimator, crate::videostab::ISparseOptFlowEstimatorTraitConst, as_raw_ISparseOptFlowEstimator, crate::videostab::ISparseOptFlowEstimatorTrait, as_raw_mut_ISparseOptFlowEstimator }

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLkOptFlowEstimator, crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst, as_raw_PyrLkOptFlowEstimatorBase, crate::videostab::PyrLkOptFlowEstimatorBaseTrait, as_raw_mut_PyrLkOptFlowEstimatorBase }

impl crate::videostab::SparsePyrLkOptFlowEstimatorTraitConst for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorTrait for SparsePyrLkOptFlowEstimator {
	#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLkOptFlowEstimator, crate::videostab::SparsePyrLkOptFlowEstimatorTraitConst, as_raw_SparsePyrLkOptFlowEstimator, crate::videostab::SparsePyrLkOptFlowEstimatorTrait, as_raw_mut_SparsePyrLkOptFlowEstimator }

impl SparsePyrLkOptFlowEstimator {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_videostab_SparsePyrLkOptFlowEstimator_defaultNew_const()) }
	}

}

boxed_cast_base! { SparsePyrLkOptFlowEstimator, crate::videostab::ISparseOptFlowEstimator, cv_videostab_SparsePyrLkOptFlowEstimator_to_ISparseOptFlowEstimator }

boxed_cast_base! { SparsePyrLkOptFlowEstimator, crate::videostab::PyrLkOptFlowEstimatorBase, cv_videostab_SparsePyrLkOptFlowEstimator_to_PyrLkOptFlowEstimatorBase }

impl std::fmt::Debug for SparsePyrLkOptFlowEstimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SparsePyrLkOptFlowEstimator")
			.finish()
	}
}

impl Default for SparsePyrLkOptFlowEstimator {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::videostab::SparsePyrLkOptFlowEstimatorGpu]
// SparsePyrLkOptFlowEstimatorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:107
pub trait SparsePyrLkOptFlowEstimatorGpuTraitConst: crate::videostab::ISparseOptFlowEstimatorTraitConst + crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst {
	fn as_raw_SparsePyrLkOptFlowEstimatorGpu(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::SparsePyrLkOptFlowEstimatorGpu]
pub trait SparsePyrLkOptFlowEstimatorGpuTrait: crate::videostab::ISparseOptFlowEstimatorTrait + crate::videostab::PyrLkOptFlowEstimatorBaseTrait + crate::videostab::SparsePyrLkOptFlowEstimatorGpuTraitConst {
	fn as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void;

	// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:113
	// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn run(&mut self, frame0: &impl ToInputArray, frame1: &impl ToInputArray, points0: &impl ToInputArray, points1: &mut impl ToInputOutputArray, status: &mut impl ToOutputArray, errors: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(frame0);
		input_array_arg!(frame1);
		input_array_arg!(points0);
		input_output_array_arg!(points1);
		output_array_arg!(status);
		output_array_arg!(errors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimatorGpu_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(), frame0.as_raw__InputArray(), frame1.as_raw__InputArray(), points0.as_raw__InputArray(), points1.as_raw__InputOutputArray(), status.as_raw__OutputArray(), errors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// run(const cuda::GpuMat &, const cuda::GpuMat &, const cuda::GpuMat &, cuda::GpuMat &, cuda::GpuMat &, cuda::GpuMat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:117
	// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn run_1(&mut self, frame0: &impl core::GpuMatTraitConst, frame1: &impl core::GpuMatTraitConst, points0: &impl core::GpuMatTraitConst, points1: &mut impl core::GpuMatTrait, status: &mut impl core::GpuMatTrait, errors: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimatorGpu_run_const_GpuMatR_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_GpuMatR(self.as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(), frame0.as_raw_GpuMat(), frame1.as_raw_GpuMat(), points0.as_raw_GpuMat(), points1.as_raw_mut_GpuMat(), status.as_raw_mut_GpuMat(), errors.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// run(const cuda::GpuMat &, const cuda::GpuMat &, const cuda::GpuMat &, cuda::GpuMat &, cuda::GpuMat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:120
	// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	#[inline]
	fn run_2(&mut self, frame0: &impl core::GpuMatTraitConst, frame1: &impl core::GpuMatTraitConst, points0: &impl core::GpuMatTraitConst, points1: &mut impl core::GpuMatTrait, status: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimatorGpu_run_const_GpuMatR_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(self.as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(), frame0.as_raw_GpuMat(), frame1.as_raw_GpuMat(), points0.as_raw_GpuMat(), points1.as_raw_mut_GpuMat(), status.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// SparsePyrLkOptFlowEstimatorGpu /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:107
pub struct SparsePyrLkOptFlowEstimatorGpu {
	ptr: *mut c_void,
}

opencv_type_boxed! { SparsePyrLkOptFlowEstimatorGpu }

impl Drop for SparsePyrLkOptFlowEstimatorGpu {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimatorGpu_delete(self.as_raw_mut_SparsePyrLkOptFlowEstimatorGpu()) };
	}
}

unsafe impl Send for SparsePyrLkOptFlowEstimatorGpu {}

impl crate::videostab::ISparseOptFlowEstimatorTraitConst for SparsePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ISparseOptFlowEstimatorTrait for SparsePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLkOptFlowEstimatorGpu, crate::videostab::ISparseOptFlowEstimatorTraitConst, as_raw_ISparseOptFlowEstimator, crate::videostab::ISparseOptFlowEstimatorTrait, as_raw_mut_ISparseOptFlowEstimator }

impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for SparsePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for SparsePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLkOptFlowEstimatorGpu, crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst, as_raw_PyrLkOptFlowEstimatorBase, crate::videostab::PyrLkOptFlowEstimatorBaseTrait, as_raw_mut_PyrLkOptFlowEstimatorBase }

impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTraitConst for SparsePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_SparsePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTrait for SparsePyrLkOptFlowEstimatorGpu {
	#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SparsePyrLkOptFlowEstimatorGpu, crate::videostab::SparsePyrLkOptFlowEstimatorGpuTraitConst, as_raw_SparsePyrLkOptFlowEstimatorGpu, crate::videostab::SparsePyrLkOptFlowEstimatorGpuTrait, as_raw_mut_SparsePyrLkOptFlowEstimatorGpu }

impl SparsePyrLkOptFlowEstimatorGpu {
	// SparsePyrLkOptFlowEstimatorGpu()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/optical_flow.hpp:111
	// ("cv::videostab::SparsePyrLkOptFlowEstimatorGpu::SparsePyrLkOptFlowEstimatorGpu", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_SparsePyrLkOptFlowEstimatorGpu_SparsePyrLkOptFlowEstimatorGpu(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::SparsePyrLkOptFlowEstimatorGpu::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SparsePyrLkOptFlowEstimatorGpu, crate::videostab::ISparseOptFlowEstimator, cv_videostab_SparsePyrLkOptFlowEstimatorGpu_to_ISparseOptFlowEstimator }

boxed_cast_base! { SparsePyrLkOptFlowEstimatorGpu, crate::videostab::PyrLkOptFlowEstimatorBase, cv_videostab_SparsePyrLkOptFlowEstimatorGpu_to_PyrLkOptFlowEstimatorBase }

impl std::fmt::Debug for SparsePyrLkOptFlowEstimatorGpu {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SparsePyrLkOptFlowEstimatorGpu")
			.finish()
	}
}

/// Constant methods for [crate::videostab::StabilizerBase]
// StabilizerBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:66
pub trait StabilizerBaseTraitConst {
	fn as_raw_StabilizerBase(&self) -> *const c_void;

	// log()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:72
	// ("cv::videostab::StabilizerBase::log", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn log(&self) -> Result<core::Ptr<crate::videostab::ILog>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_log_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::ILog>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// radius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:75
	// ("cv::videostab::StabilizerBase::radius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_radius_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// frameSource()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:78
	// ("cv::videostab::StabilizerBase::frameSource", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_source(&self) -> Result<core::Ptr<crate::videostab::IFrameSource>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_frameSource_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::IFrameSource>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// maskSource()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:81
	// ("cv::videostab::StabilizerBase::maskSource", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn mask_source(&self) -> Result<core::Ptr<crate::videostab::IFrameSource>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_maskSource_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::IFrameSource>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// motionEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:84
	// ("cv::videostab::StabilizerBase::motionEstimator", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_estimator(&self) -> Result<core::Ptr<crate::videostab::ImageMotionEstimatorBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_motionEstimator_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::ImageMotionEstimatorBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// deblurrer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:87
	// ("cv::videostab::StabilizerBase::deblurrer", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn deblurrer(&self) -> Result<core::Ptr<crate::videostab::DeblurerBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_deblurrer_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::DeblurerBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// trimRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:90
	// ("cv::videostab::StabilizerBase::trimRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn trim_ratio(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_trimRatio_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// doCorrectionForInclusion()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:93
	// ("cv::videostab::StabilizerBase::doCorrectionForInclusion", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn do_correction_for_inclusion(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_doCorrectionForInclusion_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// borderMode()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:96
	// ("cv::videostab::StabilizerBase::borderMode", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn border_mode(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_borderMode_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// inpainter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:99
	// ("cv::videostab::StabilizerBase::inpainter", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn inpainter(&self) -> Result<core::Ptr<crate::videostab::InpainterBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_inpainter_const(self.as_raw_StabilizerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::InpainterBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::StabilizerBase]
pub trait StabilizerBaseTrait: crate::videostab::StabilizerBaseTraitConst {
	fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void;

	// setLog(Ptr<ILog>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:71
	// ("cv::videostab::StabilizerBase::setLog", vec![(pred!(mut, ["ilog"], ["cv::Ptr<cv::videostab::ILog>"]), _)]),
	#[inline]
	fn set_log(&mut self, mut ilog: core::Ptr<crate::videostab::ILog>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setLog_PtrLILogG(self.as_raw_mut_StabilizerBase(), ilog.as_raw_mut_PtrOfILog(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:74
	// ("cv::videostab::StabilizerBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_radius(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setRadius_int(self.as_raw_mut_StabilizerBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrameSource(Ptr<IFrameSource>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:77
	// ("cv::videostab::StabilizerBase::setFrameSource", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IFrameSource>"]), _)]),
	#[inline]
	fn set_frame_source(&mut self, mut val: core::Ptr<crate::videostab::IFrameSource>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setFrameSource_PtrLIFrameSourceG(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfIFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaskSource(const Ptr<IFrameSource> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:80
	// ("cv::videostab::StabilizerBase::setMaskSource", vec![(pred!(mut, ["val"], ["const cv::Ptr<cv::videostab::IFrameSource>*"]), _)]),
	#[inline]
	fn set_mask_source(&mut self, val: &core::Ptr<crate::videostab::IFrameSource>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setMaskSource_const_PtrLIFrameSourceGR(self.as_raw_mut_StabilizerBase(), val.as_raw_PtrOfIFrameSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotionEstimator(Ptr<ImageMotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:83
	// ("cv::videostab::StabilizerBase::setMotionEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
	#[inline]
	fn set_motion_estimator(&mut self, mut val: core::Ptr<crate::videostab::ImageMotionEstimatorBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setMotionEstimator_PtrLImageMotionEstimatorBaseG(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfImageMotionEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setDeblurer(Ptr<DeblurerBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:86
	// ("cv::videostab::StabilizerBase::setDeblurer", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::DeblurerBase>"]), _)]),
	#[inline]
	fn set_deblurer(&mut self, mut val: core::Ptr<crate::videostab::DeblurerBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setDeblurer_PtrLDeblurerBaseG(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfDeblurerBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setTrimRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:89
	// ("cv::videostab::StabilizerBase::setTrimRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_trim_ratio(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setTrimRatio_float(self.as_raw_mut_StabilizerBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setCorrectionForInclusion(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:92
	// ("cv::videostab::StabilizerBase::setCorrectionForInclusion", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_correction_for_inclusion(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(self.as_raw_mut_StabilizerBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBorderMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:95
	// ("cv::videostab::StabilizerBase::setBorderMode", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_border_mode(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setBorderMode_int(self.as_raw_mut_StabilizerBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setInpainter(Ptr<InpainterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:98
	// ("cv::videostab::StabilizerBase::setInpainter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::InpainterBase>"]), _)]),
	#[inline]
	fn set_inpainter(&mut self, mut val: core::Ptr<crate::videostab::InpainterBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_StabilizerBase_setInpainter_PtrLInpainterBaseG(self.as_raw_mut_StabilizerBase(), val.as_raw_mut_PtrOfInpainterBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// StabilizerBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:66
pub struct StabilizerBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { StabilizerBase }

impl Drop for StabilizerBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_StabilizerBase_delete(self.as_raw_mut_StabilizerBase()) };
	}
}

unsafe impl Send for StabilizerBase {}

impl crate::videostab::StabilizerBaseTraitConst for StabilizerBase {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::StabilizerBaseTrait for StabilizerBase {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StabilizerBase, crate::videostab::StabilizerBaseTraitConst, as_raw_StabilizerBase, crate::videostab::StabilizerBaseTrait, as_raw_mut_StabilizerBase }

impl StabilizerBase {
}

boxed_cast_descendant! { StabilizerBase, crate::videostab::OnePassStabilizer, cv_videostab_StabilizerBase_to_OnePassStabilizer }

boxed_cast_descendant! { StabilizerBase, crate::videostab::TwoPassStabilizer, cv_videostab_StabilizerBase_to_TwoPassStabilizer }

impl std::fmt::Debug for StabilizerBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StabilizerBase")
			.finish()
	}
}

/// Constant methods for [crate::videostab::ToFileMotionWriter]
// ToFileMotionWriter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:209
pub trait ToFileMotionWriterTraitConst: crate::videostab::ImageMotionEstimatorBaseTraitConst {
	fn as_raw_ToFileMotionWriter(&self) -> *const c_void;

	// motionModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:215
	// ("cv::videostab::ToFileMotionWriter::motionModel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_model(&self) -> Result<crate::videostab::MotionModel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ToFileMotionWriter_motionModel_const(self.as_raw_ToFileMotionWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::ToFileMotionWriter]
pub trait ToFileMotionWriterTrait: crate::videostab::ImageMotionEstimatorBaseTrait + crate::videostab::ToFileMotionWriterTraitConst {
	fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void;

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:214
	// ("cv::videostab::ToFileMotionWriter::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	#[inline]
	fn set_motion_model(&mut self, val: crate::videostab::MotionModel) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(self.as_raw_mut_ToFileMotionWriter(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrameMask(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:217
	// ("cv::videostab::ToFileMotionWriter::setFrameMask", vec![(pred!(mut, ["mask"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_frame_mask(&mut self, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ToFileMotionWriter_setFrameMask_const__InputArrayR(self.as_raw_mut_ToFileMotionWriter(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * ok: 0
	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:219
	// ("cv::videostab::ToFileMotionWriter::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	#[inline]
	fn estimate(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst, ok: &mut bool) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR_boolX(self.as_raw_mut_ToFileMotionWriter(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ok, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [ToFileMotionWriterTrait::estimate] function uses the following default values for its arguments:
	/// * ok: 0
	// cv::videostab::ToFileMotionWriter::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:219
	// ("cv::videostab::ToFileMotionWriter::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn estimate_def(&mut self, frame0: &impl core::MatTraitConst, frame1: &impl core::MatTraitConst) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR(self.as_raw_mut_ToFileMotionWriter(), frame0.as_raw_Mat(), frame1.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// ToFileMotionWriter /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:209
pub struct ToFileMotionWriter {
	ptr: *mut c_void,
}

opencv_type_boxed! { ToFileMotionWriter }

impl Drop for ToFileMotionWriter {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_ToFileMotionWriter_delete(self.as_raw_mut_ToFileMotionWriter()) };
	}
}

unsafe impl Send for ToFileMotionWriter {}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for ToFileMotionWriter {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for ToFileMotionWriter {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ToFileMotionWriter, crate::videostab::ImageMotionEstimatorBaseTraitConst, as_raw_ImageMotionEstimatorBase, crate::videostab::ImageMotionEstimatorBaseTrait, as_raw_mut_ImageMotionEstimatorBase }

impl crate::videostab::ToFileMotionWriterTraitConst for ToFileMotionWriter {
	#[inline] fn as_raw_ToFileMotionWriter(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::ToFileMotionWriterTrait for ToFileMotionWriter {
	#[inline] fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ToFileMotionWriter, crate::videostab::ToFileMotionWriterTraitConst, as_raw_ToFileMotionWriter, crate::videostab::ToFileMotionWriterTrait, as_raw_mut_ToFileMotionWriter }

impl ToFileMotionWriter {
	// ToFileMotionWriter(const String &, Ptr<ImageMotionEstimatorBase>)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/global_motion.hpp:212
	// ("cv::videostab::ToFileMotionWriter::ToFileMotionWriter", vec![(pred!(mut, ["path", "estimator"], ["const cv::String*", "cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
	#[inline]
	pub fn new(path: &str, mut estimator: core::Ptr<crate::videostab::ImageMotionEstimatorBase>) -> Result<crate::videostab::ToFileMotionWriter> {
		extern_container_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringR_PtrLImageMotionEstimatorBaseG(path.opencv_as_extern(), estimator.as_raw_mut_PtrOfImageMotionEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::ToFileMotionWriter::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ToFileMotionWriter, crate::videostab::ImageMotionEstimatorBase, cv_videostab_ToFileMotionWriter_to_ImageMotionEstimatorBase }

impl std::fmt::Debug for ToFileMotionWriter {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ToFileMotionWriter")
			.finish()
	}
}

/// Constant methods for [crate::videostab::TranslationBasedLocalOutlierRejector]
// TranslationBasedLocalOutlierRejector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:74
pub trait TranslationBasedLocalOutlierRejectorTraitConst: crate::videostab::IOutlierRejectorTraitConst {
	fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void;

	// cellSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:80
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::cellSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cell_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(self.as_raw_TranslationBasedLocalOutlierRejector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// ransacParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:83
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::ransacParams", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ransac_params(&self) -> Result<crate::videostab::RansacParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(self.as_raw_TranslationBasedLocalOutlierRejector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::RansacParams::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::TranslationBasedLocalOutlierRejector]
pub trait TranslationBasedLocalOutlierRejectorTrait: crate::videostab::IOutlierRejectorTrait + crate::videostab::TranslationBasedLocalOutlierRejectorTraitConst {
	fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void;

	// setCellSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:79
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::setCellSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	#[inline]
	fn set_cell_size(&mut self, val: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(self.as_raw_mut_TranslationBasedLocalOutlierRejector(), &val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setRansacParams(RansacParams)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:82
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::setRansacParams", vec![(pred!(mut, ["val"], ["cv::videostab::RansacParams"]), _)]),
	#[inline]
	fn set_ransac_params(&mut self, mut val: impl crate::videostab::RansacParamsTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(self.as_raw_mut_TranslationBasedLocalOutlierRejector(), val.as_raw_mut_RansacParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:85
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn process(&mut self, frame_size: core::Size, points0: &impl ToInputArray, points1: &impl ToInputArray, mask: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points0);
		input_array_arg!(points1);
		output_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_TranslationBasedLocalOutlierRejector(), &frame_size, points0.as_raw__InputArray(), points1.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// TranslationBasedLocalOutlierRejector /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:74
pub struct TranslationBasedLocalOutlierRejector {
	ptr: *mut c_void,
}

opencv_type_boxed! { TranslationBasedLocalOutlierRejector }

impl Drop for TranslationBasedLocalOutlierRejector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_delete(self.as_raw_mut_TranslationBasedLocalOutlierRejector()) };
	}
}

unsafe impl Send for TranslationBasedLocalOutlierRejector {}

impl crate::videostab::IOutlierRejectorTraitConst for TranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IOutlierRejectorTrait for TranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TranslationBasedLocalOutlierRejector, crate::videostab::IOutlierRejectorTraitConst, as_raw_IOutlierRejector, crate::videostab::IOutlierRejectorTrait, as_raw_mut_IOutlierRejector }

impl crate::videostab::TranslationBasedLocalOutlierRejectorTraitConst for TranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::TranslationBasedLocalOutlierRejectorTrait for TranslationBasedLocalOutlierRejector {
	#[inline] fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TranslationBasedLocalOutlierRejector, crate::videostab::TranslationBasedLocalOutlierRejectorTraitConst, as_raw_TranslationBasedLocalOutlierRejector, crate::videostab::TranslationBasedLocalOutlierRejectorTrait, as_raw_mut_TranslationBasedLocalOutlierRejector }

impl TranslationBasedLocalOutlierRejector {
	// TranslationBasedLocalOutlierRejector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/outlier_rejection.hpp:77
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::TranslationBasedLocalOutlierRejector", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::TranslationBasedLocalOutlierRejector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::TranslationBasedLocalOutlierRejector::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TranslationBasedLocalOutlierRejector, crate::videostab::IOutlierRejector, cv_videostab_TranslationBasedLocalOutlierRejector_to_IOutlierRejector }

impl std::fmt::Debug for TranslationBasedLocalOutlierRejector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TranslationBasedLocalOutlierRejector")
			.finish()
	}
}

/// Constant methods for [crate::videostab::TwoPassStabilizer]
// TwoPassStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:163
pub trait TwoPassStabilizerTraitConst: crate::videostab::IFrameSourceTraitConst + crate::videostab::StabilizerBaseTraitConst {
	fn as_raw_TwoPassStabilizer(&self) -> *const c_void;

	// motionStabilizer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:169
	// ("cv::videostab::TwoPassStabilizer::motionStabilizer", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_stabilizer(&self) -> Result<core::Ptr<crate::videostab::IMotionStabilizer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_motionStabilizer_const(self.as_raw_TwoPassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::IMotionStabilizer>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// wobbleSuppressor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:172
	// ("cv::videostab::TwoPassStabilizer::wobbleSuppressor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn wobble_suppressor(&self) -> Result<core::Ptr<crate::videostab::WobbleSuppressorBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(self.as_raw_TwoPassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::WobbleSuppressorBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// mustEstimateTrimaRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:175
	// ("cv::videostab::TwoPassStabilizer::mustEstimateTrimaRatio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn must_estimate_trima_ratio(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(self.as_raw_TwoPassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::TwoPassStabilizer]
pub trait TwoPassStabilizerTrait: crate::videostab::IFrameSourceTrait + crate::videostab::StabilizerBaseTrait + crate::videostab::TwoPassStabilizerTraitConst {
	fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void;

	// setMotionStabilizer(Ptr<IMotionStabilizer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:168
	// ("cv::videostab::TwoPassStabilizer::setMotionStabilizer", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IMotionStabilizer>"]), _)]),
	#[inline]
	fn set_motion_stabilizer(&mut self, mut val: core::Ptr<crate::videostab::IMotionStabilizer>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_setMotionStabilizer_PtrLIMotionStabilizerG(self.as_raw_mut_TwoPassStabilizer(), val.as_raw_mut_PtrOfIMotionStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setWobbleSuppressor(Ptr<WobbleSuppressorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:171
	// ("cv::videostab::TwoPassStabilizer::setWobbleSuppressor", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::WobbleSuppressorBase>"]), _)]),
	#[inline]
	fn set_wobble_suppressor(&mut self, mut val: core::Ptr<crate::videostab::WobbleSuppressorBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_setWobbleSuppressor_PtrLWobbleSuppressorBaseG(self.as_raw_mut_TwoPassStabilizer(), val.as_raw_mut_PtrOfWobbleSuppressorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setEstimateTrimRatio(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:174
	// ("cv::videostab::TwoPassStabilizer::setEstimateTrimRatio", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	#[inline]
	fn set_estimate_trim_ratio(&mut self, val: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(self.as_raw_mut_TwoPassStabilizer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:177
	// ("cv::videostab::TwoPassStabilizer::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_reset(self.as_raw_mut_TwoPassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:178
	// ("cv::videostab::TwoPassStabilizer::nextFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn next_frame(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_nextFrame(self.as_raw_mut_TwoPassStabilizer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

// TwoPassStabilizer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:163
pub struct TwoPassStabilizer {
	ptr: *mut c_void,
}

opencv_type_boxed! { TwoPassStabilizer }

impl Drop for TwoPassStabilizer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_TwoPassStabilizer_delete(self.as_raw_mut_TwoPassStabilizer()) };
	}
}

unsafe impl Send for TwoPassStabilizer {}

impl crate::videostab::IFrameSourceTraitConst for TwoPassStabilizer {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IFrameSourceTrait for TwoPassStabilizer {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TwoPassStabilizer, crate::videostab::IFrameSourceTraitConst, as_raw_IFrameSource, crate::videostab::IFrameSourceTrait, as_raw_mut_IFrameSource }

impl crate::videostab::StabilizerBaseTraitConst for TwoPassStabilizer {
	#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::StabilizerBaseTrait for TwoPassStabilizer {
	#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TwoPassStabilizer, crate::videostab::StabilizerBaseTraitConst, as_raw_StabilizerBase, crate::videostab::StabilizerBaseTrait, as_raw_mut_StabilizerBase }

impl crate::videostab::TwoPassStabilizerTraitConst for TwoPassStabilizer {
	#[inline] fn as_raw_TwoPassStabilizer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::TwoPassStabilizerTrait for TwoPassStabilizer {
	#[inline] fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TwoPassStabilizer, crate::videostab::TwoPassStabilizerTraitConst, as_raw_TwoPassStabilizer, crate::videostab::TwoPassStabilizerTrait, as_raw_mut_TwoPassStabilizer }

impl TwoPassStabilizer {
	// TwoPassStabilizer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/stabilizer.hpp:166
	// ("cv::videostab::TwoPassStabilizer::TwoPassStabilizer", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::TwoPassStabilizer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_TwoPassStabilizer_TwoPassStabilizer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::TwoPassStabilizer::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TwoPassStabilizer, crate::videostab::IFrameSource, cv_videostab_TwoPassStabilizer_to_IFrameSource }

boxed_cast_base! { TwoPassStabilizer, crate::videostab::StabilizerBase, cv_videostab_TwoPassStabilizer_to_StabilizerBase }

impl std::fmt::Debug for TwoPassStabilizer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TwoPassStabilizer")
			.finish()
	}
}

/// Constant methods for [crate::videostab::VideoFileSource]
// VideoFileSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:72
pub trait VideoFileSourceTraitConst: crate::videostab::IFrameSourceTraitConst {
	fn as_raw_VideoFileSource(&self) -> *const c_void;

}

/// Mutable methods for [crate::videostab::VideoFileSource]
pub trait VideoFileSourceTrait: crate::videostab::IFrameSourceTrait + crate::videostab::VideoFileSourceTraitConst {
	fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void;

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:77
	// ("cv::videostab::VideoFileSource::reset", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn reset(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_reset(self.as_raw_mut_VideoFileSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:78
	// ("cv::videostab::VideoFileSource::nextFrame", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn next_frame(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_nextFrame(self.as_raw_mut_VideoFileSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	// width()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:80
	// ("cv::videostab::VideoFileSource::width", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn width(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_width(self.as_raw_mut_VideoFileSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// height()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:81
	// ("cv::videostab::VideoFileSource::height", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn height(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_height(self.as_raw_mut_VideoFileSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// count()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:82
	// ("cv::videostab::VideoFileSource::count", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn count(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_count(self.as_raw_mut_VideoFileSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// fps()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:83
	// ("cv::videostab::VideoFileSource::fps", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn fps(&mut self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_fps(self.as_raw_mut_VideoFileSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// VideoFileSource /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:72
pub struct VideoFileSource {
	ptr: *mut c_void,
}

opencv_type_boxed! { VideoFileSource }

impl Drop for VideoFileSource {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_VideoFileSource_delete(self.as_raw_mut_VideoFileSource()) };
	}
}

unsafe impl Send for VideoFileSource {}

impl crate::videostab::IFrameSourceTraitConst for VideoFileSource {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::IFrameSourceTrait for VideoFileSource {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VideoFileSource, crate::videostab::IFrameSourceTraitConst, as_raw_IFrameSource, crate::videostab::IFrameSourceTrait, as_raw_mut_IFrameSource }

impl crate::videostab::VideoFileSourceTraitConst for VideoFileSource {
	#[inline] fn as_raw_VideoFileSource(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::VideoFileSourceTrait for VideoFileSource {
	#[inline] fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { VideoFileSource, crate::videostab::VideoFileSourceTraitConst, as_raw_VideoFileSource, crate::videostab::VideoFileSourceTrait, as_raw_mut_VideoFileSource }

impl VideoFileSource {
	/// ## C++ default parameters
	/// * volatile_frame: false
	// VideoFileSource(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:75
	// ("cv::videostab::VideoFileSource::VideoFileSource", vec![(pred!(mut, ["path", "volatileFrame"], ["const cv::String*", "bool"]), _)]),
	#[inline]
	pub fn new(path: &str, volatile_frame: bool) -> Result<crate::videostab::VideoFileSource> {
		extern_container_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_VideoFileSource_const_StringR_bool(path.opencv_as_extern(), volatile_frame, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::VideoFileSource::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * volatile_frame: false
	// cv::videostab::VideoFileSource::VideoFileSource(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/frame_source.hpp:75
	// ("cv::videostab::VideoFileSource::VideoFileSource", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn new_def(path: &str) -> Result<crate::videostab::VideoFileSource> {
		extern_container_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_VideoFileSource_VideoFileSource_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::VideoFileSource::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { VideoFileSource, crate::videostab::IFrameSource, cv_videostab_VideoFileSource_to_IFrameSource }

impl std::fmt::Debug for VideoFileSource {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("VideoFileSource")
			.finish()
	}
}

/// Constant methods for [crate::videostab::WeightingDeblurer]
// WeightingDeblurer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:96
pub trait WeightingDeblurerTraitConst: crate::videostab::DeblurerBaseTraitConst {
	fn as_raw_WeightingDeblurer(&self) -> *const c_void;

	// sensitivity()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:102
	// ("cv::videostab::WeightingDeblurer::sensitivity", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sensitivity(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WeightingDeblurer_sensitivity_const(self.as_raw_WeightingDeblurer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::WeightingDeblurer]
pub trait WeightingDeblurerTrait: crate::videostab::DeblurerBaseTrait + crate::videostab::WeightingDeblurerTraitConst {
	fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void;

	// setSensitivity(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:101
	// ("cv::videostab::WeightingDeblurer::setSensitivity", vec![(pred!(mut, ["val"], ["float"]), _)]),
	#[inline]
	fn set_sensitivity(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WeightingDeblurer_setSensitivity_float(self.as_raw_mut_WeightingDeblurer(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// deblur(int, Mat &, const Range &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:104
	// ("cv::videostab::WeightingDeblurer::deblur", vec![(pred!(mut, ["idx", "frame", "range"], ["int", "cv::Mat*", "const cv::Range*"]), _)]),
	#[inline]
	fn deblur(&mut self, idx: i32, frame: &mut impl core::MatTrait, range: &impl core::RangeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WeightingDeblurer_deblur_int_MatR_const_RangeR(self.as_raw_mut_WeightingDeblurer(), idx, frame.as_raw_mut_Mat(), range.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// WeightingDeblurer /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:96
pub struct WeightingDeblurer {
	ptr: *mut c_void,
}

opencv_type_boxed! { WeightingDeblurer }

impl Drop for WeightingDeblurer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_WeightingDeblurer_delete(self.as_raw_mut_WeightingDeblurer()) };
	}
}

unsafe impl Send for WeightingDeblurer {}

impl crate::videostab::DeblurerBaseTraitConst for WeightingDeblurer {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::DeblurerBaseTrait for WeightingDeblurer {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { WeightingDeblurer, crate::videostab::DeblurerBaseTraitConst, as_raw_DeblurerBase, crate::videostab::DeblurerBaseTrait, as_raw_mut_DeblurerBase }

impl crate::videostab::WeightingDeblurerTraitConst for WeightingDeblurer {
	#[inline] fn as_raw_WeightingDeblurer(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::WeightingDeblurerTrait for WeightingDeblurer {
	#[inline] fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { WeightingDeblurer, crate::videostab::WeightingDeblurerTraitConst, as_raw_WeightingDeblurer, crate::videostab::WeightingDeblurerTrait, as_raw_mut_WeightingDeblurer }

impl WeightingDeblurer {
	// WeightingDeblurer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/deblurring.hpp:99
	// ("cv::videostab::WeightingDeblurer::WeightingDeblurer", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::videostab::WeightingDeblurer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WeightingDeblurer_WeightingDeblurer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::videostab::WeightingDeblurer::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { WeightingDeblurer, crate::videostab::DeblurerBase, cv_videostab_WeightingDeblurer_to_DeblurerBase }

impl std::fmt::Debug for WeightingDeblurer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("WeightingDeblurer")
			.finish()
	}
}

/// Constant methods for [crate::videostab::WobbleSuppressorBase]
// WobbleSuppressorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:60
pub trait WobbleSuppressorBaseTraitConst {
	fn as_raw_WobbleSuppressorBase(&self) -> *const c_void;

	// motionEstimator()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:68
	// ("cv::videostab::WobbleSuppressorBase::motionEstimator", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motion_estimator(&self) -> Result<core::Ptr<crate::videostab::ImageMotionEstimatorBase>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_motionEstimator_const(self.as_raw_WobbleSuppressorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::videostab::ImageMotionEstimatorBase>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// frameCount()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:76
	// ("cv::videostab::WobbleSuppressorBase::frameCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn frame_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_frameCount_const(self.as_raw_WobbleSuppressorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// motions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:79
	// ("cv::videostab::WobbleSuppressorBase::motions", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motions(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_motions_const(self.as_raw_WobbleSuppressorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// motions2()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:82
	// ("cv::videostab::WobbleSuppressorBase::motions2", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn motions2(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_motions2_const(self.as_raw_WobbleSuppressorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// stabilizationMotions()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:85
	// ("cv::videostab::WobbleSuppressorBase::stabilizationMotions", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn stabilization_motions(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(self.as_raw_WobbleSuppressorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::videostab::WobbleSuppressorBase]
pub trait WobbleSuppressorBaseTrait: crate::videostab::WobbleSuppressorBaseTraitConst {
	fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void;

	// setMotionEstimator(Ptr<ImageMotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:67
	// ("cv::videostab::WobbleSuppressorBase::setMotionEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
	#[inline]
	fn set_motion_estimator(&mut self, mut val: core::Ptr<crate::videostab::ImageMotionEstimatorBase>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotionEstimator_PtrLImageMotionEstimatorBaseG(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_mut_PtrOfImageMotionEstimatorBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:70
	// ("cv::videostab::WobbleSuppressorBase::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn suppress(&mut self, idx: i32, frame: &impl core::MatTraitConst, result: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_suppress_int_const_MatR_MatR(self.as_raw_mut_WobbleSuppressorBase(), idx, frame.as_raw_Mat(), result.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setFrameCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:75
	// ("cv::videostab::WobbleSuppressorBase::setFrameCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_frame_count(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setFrameCount_int(self.as_raw_mut_WobbleSuppressorBase(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:78
	// ("cv::videostab::WobbleSuppressorBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_motions(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotions_const_vectorLMatGR(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMotions2(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:81
	// ("cv::videostab::WobbleSuppressorBase::setMotions2", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_motions2(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setMotions2_const_vectorLMatGR(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:84
	// ("cv::videostab::WobbleSuppressorBase::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn set_stabilization_motions(&mut self, val: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vectorLMatGR(self.as_raw_mut_WobbleSuppressorBase(), val.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// WobbleSuppressorBase /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videostab/wobble_suppression.hpp:60
pub struct WobbleSuppressorBase {
	ptr: *mut c_void,
}

opencv_type_boxed! { WobbleSuppressorBase }

impl Drop for WobbleSuppressorBase {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_videostab_WobbleSuppressorBase_delete(self.as_raw_mut_WobbleSuppressorBase()) };
	}
}

unsafe impl Send for WobbleSuppressorBase {}

impl crate::videostab::WobbleSuppressorBaseTraitConst for WobbleSuppressorBase {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for WobbleSuppressorBase {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { WobbleSuppressorBase, crate::videostab::WobbleSuppressorBaseTraitConst, as_raw_WobbleSuppressorBase, crate::videostab::WobbleSuppressorBaseTrait, as_raw_mut_WobbleSuppressorBase }

impl WobbleSuppressorBase {
}

boxed_cast_descendant! { WobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressor, cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressor }

boxed_cast_descendant! { WobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressorBase, cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressorBase }

boxed_cast_descendant! { WobbleSuppressorBase, crate::videostab::MoreAccurateMotionWobbleSuppressorGpu, cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressorGpu }

boxed_cast_descendant! { WobbleSuppressorBase, crate::videostab::NullWobbleSuppressor, cv_videostab_WobbleSuppressorBase_to_NullWobbleSuppressor }

impl std::fmt::Debug for WobbleSuppressorBase {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("WobbleSuppressorBase")
			.finish()
	}
}
