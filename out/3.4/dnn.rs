//! # Deep Neural Network module
//!   This module contains:
//!       - API for new layers creation, layers are building bricks of neural networks;
//!       - set of built-in most-useful Layers;
//!       - API to construct and modify comprehensive neural networks from layers;
//!       - functionality for loading serialized networks models from different frameworks.
//!
//!   Functionality of this module is designed only for forward pass computations (i.e. network testing).
//!   A network training is in principle not supported.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{AbsLayerTrait, AbsLayerTraitConst, AccumLayerTrait, AccumLayerTraitConst, ActivationLayerTrait, ActivationLayerTraitConst, BNLLLayerTrait, BNLLLayerTraitConst, BackendNodeTrait, BackendNodeTraitConst, BackendWrapperTrait, BackendWrapperTraitConst, BaseConvolutionLayerTrait, BaseConvolutionLayerTraitConst, BatchNormLayerTrait, BatchNormLayerTraitConst, BlankLayerTrait, BlankLayerTraitConst, ChannelsPReLULayerTrait, ChannelsPReLULayerTraitConst, ConcatLayerTrait, ConcatLayerTraitConst, ConstLayerTrait, ConstLayerTraitConst, ConvolutionLayerTrait, ConvolutionLayerTraitConst, CorrelationLayerTrait, CorrelationLayerTraitConst, CropAndResizeLayerTrait, CropAndResizeLayerTraitConst, CropLayerTrait, CropLayerTraitConst, DataAugmentationLayerTrait, DataAugmentationLayerTraitConst, DeconvolutionLayerTrait, DeconvolutionLayerTraitConst, DetectionOutputLayerTrait, DetectionOutputLayerTraitConst, DictTrait, DictTraitConst, DictValueTrait, DictValueTraitConst, ELULayerTrait, ELULayerTraitConst, EltwiseLayerTrait, EltwiseLayerTraitConst, ExpLayerTrait, ExpLayerTraitConst, FlattenLayerTrait, FlattenLayerTraitConst, FlowWarpLayerTrait, FlowWarpLayerTraitConst, InnerProductLayerTrait, InnerProductLayerTraitConst, InterpLayerTrait, InterpLayerTraitConst, LRNLayerTrait, LRNLayerTraitConst, LSTMLayerTrait, LSTMLayerTraitConst, LayerFactoryTrait, LayerFactoryTraitConst, LayerParamsTrait, LayerParamsTraitConst, LayerTrait, LayerTraitConst, MVNLayerTrait, MVNLayerTraitConst, MaxUnpoolLayerTrait, MaxUnpoolLayerTraitConst, MishLayerTrait, MishLayerTraitConst, NetTrait, NetTraitConst, NormalizeBBoxLayerTrait, NormalizeBBoxLayerTraitConst, PaddingLayerTrait, PaddingLayerTraitConst, PermuteLayerTrait, PermuteLayerTraitConst, PoolingLayerTrait, PoolingLayerTraitConst, PowerLayerTrait, PowerLayerTraitConst, PriorBoxLayerTrait, PriorBoxLayerTraitConst, ProposalLayerTrait, ProposalLayerTraitConst, RNNLayerTrait, RNNLayerTraitConst, ReLU6LayerTrait, ReLU6LayerTraitConst, ReLULayerTrait, ReLULayerTraitConst, RegionLayerTrait, RegionLayerTraitConst, ReorgLayerTrait, ReorgLayerTraitConst, ReshapeLayerTrait, ReshapeLayerTraitConst, ResizeLayerTrait, ResizeLayerTraitConst, ScaleLayerTrait, ScaleLayerTraitConst, ShiftLayerTrait, ShiftLayerTraitConst, ShuffleChannelLayerTrait, ShuffleChannelLayerTraitConst, SigmoidLayerTrait, SigmoidLayerTraitConst, SliceLayerTrait, SliceLayerTraitConst, SoftmaxLayerTrait, SoftmaxLayerTraitConst, SplitLayerTrait, SplitLayerTraitConst, SwishLayerTrait, SwishLayerTraitConst, TanHLayerTrait, TanHLayerTraitConst, _RangeTrait, _RangeTraitConst};
}

// CV_DNN_BACKEND_INFERENCE_ENGINE_NGRAPH /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:21
pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NGRAPH: &str = "NGRAPH";
// CV_DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_API /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:19
pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_API: &str = "NN_BUILDER";
// CV_DNN_INFERENCE_ENGINE_CPU_TYPE_ARM_COMPUTE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:58
pub const CV_DNN_INFERENCE_ENGINE_CPU_TYPE_ARM_COMPUTE: &str = "ARM_COMPUTE";
// CV_DNN_INFERENCE_ENGINE_CPU_TYPE_X86 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:59
pub const CV_DNN_INFERENCE_ENGINE_CPU_TYPE_X86: &str = "X86";
// CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_2 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:55
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_2: &str = "Myriad2";
// CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_X /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:57
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_X: &str = "MyriadX";
// CV_DNN_INFERENCE_ENGINE_VPU_TYPE_UNSPECIFIED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:53
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_UNSPECIFIED: &str = "";
/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
/// OpenCV is built with Intel's Inference Engine library or
/// DNN_BACKEND_OPENCV otherwise.
// DNN_BACKEND_DEFAULT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:77
pub const DNN_BACKEND_DEFAULT: i32 = 0;
/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
/// OpenCV is built with Intel's Inference Engine library or
/// DNN_BACKEND_OPENCV otherwise.
// DNN_BACKEND_HALIDE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:78
pub const DNN_BACKEND_HALIDE: i32 = 1;
/// Intel's Inference Engine computational backend
/// ## See also
/// setInferenceEngineBackendType
// DNN_BACKEND_INFERENCE_ENGINE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:79
pub const DNN_BACKEND_INFERENCE_ENGINE: i32 = 2;
// DNN_BACKEND_OPENCV /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:81
pub const DNN_BACKEND_OPENCV: i32 = 3;
// DNN_TARGET_CPU /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:97
pub const DNN_TARGET_CPU: i32 = 0;
/// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
// DNN_TARGET_FPGA /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:101
pub const DNN_TARGET_FPGA: i32 = 4;
// DNN_TARGET_MYRIAD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:100
pub const DNN_TARGET_MYRIAD: i32 = 3;
// DNN_TARGET_OPENCL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:98
pub const DNN_TARGET_OPENCL: i32 = 1;
// DNN_TARGET_OPENCL_FP16 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:99
pub const DNN_TARGET_OPENCL_FP16: i32 = 2;
/// Enum of computation backends supported by layers.
/// ## See also
/// Net::setPreferableBackend
// Backend /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:72
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Backend {
	/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
	/// OpenCV is built with Intel's Inference Engine library or
	/// DNN_BACKEND_OPENCV otherwise.
	DNN_BACKEND_DEFAULT = 0,
	/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
	/// OpenCV is built with Intel's Inference Engine library or
	/// DNN_BACKEND_OPENCV otherwise.
	DNN_BACKEND_HALIDE = 1,
	/// Intel's Inference Engine computational backend
	/// ## See also
	/// setInferenceEngineBackendType
	DNN_BACKEND_INFERENCE_ENGINE = 2,
	DNN_BACKEND_OPENCV = 3,
}

impl TryFrom<i32> for Backend {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DNN_BACKEND_DEFAULT),
			1 => Ok(Self::DNN_BACKEND_HALIDE),
			2 => Ok(Self::DNN_BACKEND_INFERENCE_ENGINE),
			3 => Ok(Self::DNN_BACKEND_OPENCV),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::dnn::Backend"))),
		}
	}
}

opencv_type_enum! { crate::dnn::Backend }

/// Enum of target devices for computations.
/// ## See also
/// Net::setPreferableTarget
// Target /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:95
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Target {
	DNN_TARGET_CPU = 0,
	DNN_TARGET_OPENCL = 1,
	DNN_TARGET_OPENCL_FP16 = 2,
	DNN_TARGET_MYRIAD = 3,
	/// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
	DNN_TARGET_FPGA = 4,
}

impl TryFrom<i32> for Target {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::DNN_TARGET_CPU),
			1 => Ok(Self::DNN_TARGET_OPENCL),
			2 => Ok(Self::DNN_TARGET_OPENCL_FP16),
			3 => Ok(Self::DNN_TARGET_MYRIAD),
			4 => Ok(Self::DNN_TARGET_FPGA),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::dnn::Target"))),
		}
	}
}

opencv_type_enum! { crate::dnn::Target }

/// Each Layer class must provide this function to the factory
// Constructor /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:61
pub type LayerFactory_Constructor = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;
// MatShape /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:66
pub type MatShape = core::Vector<i32>;
/// Container for strings and integers.
///
///
/// **Deprecated**: Use getLayerId() with int result.
#[deprecated = "Use getLayerId() with int result."]
// LayerId /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:451
pub type Net_LayerId = crate::dnn::DictValue;
/// ## Note
/// This alternative version of [nms_boxes_f64] function uses the following default values for its arguments:
/// * eta: 1.f
/// * top_k: 0
// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1078
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
#[inline]
pub fn nms_boxes_f64_def(bboxes: &core::Vector<core::Rect2d>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
// NMSBoxes(const std::vector<Rect2d> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1078
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
#[inline]
pub fn nms_boxes_f64(bboxes: &core::Vector<core::Rect2d>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Performs non maximum suppression given boxes and corresponding scores.
///
/// ## Parameters
/// * bboxes: a set of bounding boxes to apply NMS.
/// * scores: a set of corresponding confidences.
/// * score_threshold: a threshold used to filter boxes by score.
/// * nms_threshold: a threshold used in non maximum suppression.
/// * indices: the kept indices of bboxes after NMS.
/// * eta: a coefficient in adaptive threshold formula: ![inline formula](https://latex.codecogs.com/png.latex?nms%5C%5Fthreshold%5F%7Bi%2B1%7D%3Deta%5Ccdot%20nms%5C%5Fthreshold%5Fi).
/// * top_k: if `>0`, keep at most @p top_k picked indices.
///
/// ## Note
/// This alternative version of [nms_boxes] function uses the following default values for its arguments:
/// * eta: 1.f
/// * top_k: 0
// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1073
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
#[inline]
pub fn nms_boxes_def(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Performs non maximum suppression given boxes and corresponding scores.
///
/// ## Parameters
/// * bboxes: a set of bounding boxes to apply NMS.
/// * scores: a set of corresponding confidences.
/// * score_threshold: a threshold used to filter boxes by score.
/// * nms_threshold: a threshold used in non maximum suppression.
/// * indices: the kept indices of bboxes after NMS.
/// * eta: a coefficient in adaptive threshold formula: ![inline formula](https://latex.codecogs.com/png.latex?nms%5C%5Fthreshold%5F%7Bi%2B1%7D%3Deta%5Ccdot%20nms%5C%5Fthreshold%5Fi).
/// * top_k: if `>0`, keep at most @p top_k picked indices.
///
/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
// NMSBoxes(const std::vector<Rect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1073
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
#[inline]
pub fn nms_boxes(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [nms_boxes_rotated] function uses the following default values for its arguments:
/// * eta: 1.f
/// * top_k: 0
// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1083
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
#[inline]
pub fn nms_boxes_rotated_def(bboxes: &core::Vector<core::RotatedRect>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(bboxes.as_raw_VectorOfRotatedRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
// NMSBoxes(const std::vector<RotatedRect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1083
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
#[inline]
pub fn nms_boxes_rotated(bboxes: &core::Vector<core::RotatedRect>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes.as_raw_VectorOfRotatedRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates 4-dimensional blob from image. Optionally resizes and crops @p image from center,
/// subtract @p mean values, scales values by @p scalefactor, swap Blue and Red channels.
/// ## Parameters
/// * image: input image (with 1-, 3- or 4-channels).
/// * size: spatial size for output image
/// * mean: scalar with mean values which are subtracted from channels. Values are intended
/// to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
/// * scalefactor: multiplier for @p image values.
/// * swapRB: flag which indicates that swap first and last channels
/// in 3-channel image is necessary.
/// * crop: flag which indicates whether image will be cropped after resize or not
/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
/// @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
/// dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
/// If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
/// ## Returns
/// 4-dimensional Mat with NCHW dimensions order.
///
/// ## Note
/// This alternative version of [blob_from_image] function uses the following default values for its arguments:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// cv::dnn::blobFromImage(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:986
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
#[inline]
pub fn blob_from_image_def(image: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR(image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates 4-dimensional blob from image.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
///
/// ## Note
/// This alternative version of [blob_from_image_to] function uses the following default values for its arguments:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// cv::dnn::blobFromImage(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:994
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn blob_from_image_to_def(image: &impl ToInputArray, blob: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(image);
	output_array_arg!(blob);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), blob.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates 4-dimensional blob from image.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
///
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// blobFromImage(InputArray, OutputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:994
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
#[inline]
pub fn blob_from_image_to(image: &impl ToInputArray, blob: &mut impl ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(image);
	output_array_arg!(blob);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates 4-dimensional blob from image. Optionally resizes and crops @p image from center,
/// subtract @p mean values, scales values by @p scalefactor, swap Blue and Red channels.
/// ## Parameters
/// * image: input image (with 1-, 3- or 4-channels).
/// * size: spatial size for output image
/// * mean: scalar with mean values which are subtracted from channels. Values are intended
/// to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
/// * scalefactor: multiplier for @p image values.
/// * swapRB: flag which indicates that swap first and last channels
/// in 3-channel image is necessary.
/// * crop: flag which indicates whether image will be cropped after resize or not
/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
/// @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
/// dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
/// If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
/// ## Returns
/// 4-dimensional Mat with NCHW dimensions order.
///
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// blobFromImage(InputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:986
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
#[inline]
pub fn blob_from_image(image: &impl ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image.as_raw__InputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates 4-dimensional blob from series of images. Optionally resizes and
/// crops @p images from center, subtract @p mean values, scales values by @p scalefactor,
/// swap Blue and Red channels.
/// ## Parameters
/// * images: input images (all with 1-, 3- or 4-channels).
/// * size: spatial size for output image
/// * mean: scalar with mean values which are subtracted from channels. Values are intended
/// to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
/// * scalefactor: multiplier for @p images values.
/// * swapRB: flag which indicates that swap first and last channels
/// in 3-channel image is necessary.
/// * crop: flag which indicates whether image will be cropped after resize or not
/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
/// @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
/// dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
/// If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
/// ## Returns
/// 4-dimensional Mat with NCHW dimensions order.
///
/// ## Note
/// This alternative version of [blob_from_images] function uses the following default values for its arguments:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// cv::dnn::blobFromImages(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1016
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
#[inline]
pub fn blob_from_images_def(images: &impl ToInputArray) -> Result<core::Mat> {
	input_array_arg!(images);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR(images.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates 4-dimensional blob from series of images.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
///
/// ## Note
/// This alternative version of [blob_from_images_to] function uses the following default values for its arguments:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// cv::dnn::blobFromImages(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1024
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn blob_from_images_to_def(images: &impl ToInputArray, blob: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(images);
	output_array_arg!(blob);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR(images.as_raw__InputArray(), blob.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates 4-dimensional blob from series of images.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
///
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// blobFromImages(InputArrayOfArrays, OutputArray, double, Size, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1024
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
#[inline]
pub fn blob_from_images_to(images: &impl ToInputArray, blob: &mut impl ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(images);
	output_array_arg!(blob);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(images.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates 4-dimensional blob from series of images. Optionally resizes and
/// crops @p images from center, subtract @p mean values, scales values by @p scalefactor,
/// swap Blue and Red channels.
/// ## Parameters
/// * images: input images (all with 1-, 3- or 4-channels).
/// * size: spatial size for output image
/// * mean: scalar with mean values which are subtracted from channels. Values are intended
/// to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
/// * scalefactor: multiplier for @p images values.
/// * swapRB: flag which indicates that swap first and last channels
/// in 3-channel image is necessary.
/// * crop: flag which indicates whether image will be cropped after resize or not
/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
/// @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
/// dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
/// If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
/// ## Returns
/// 4-dimensional Mat with NCHW dimensions order.
///
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
// blobFromImages(InputArrayOfArrays, double, Size, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1016
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
#[inline]
pub fn blob_from_images(images: &impl ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
	input_array_arg!(images);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(images.as_raw__InputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// concat(const MatShape &, const MatShape &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:206
// ("cv::dnn::concat", vec![(pred!(mut, ["a", "b"], ["const cv::dnn::MatShape*", "const cv::dnn::MatShape*"]), _)]),
#[inline]
pub fn concat(a: &crate::dnn::MatShape, b: &crate::dnn::MatShape) -> Result<crate::dnn::MatShape> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_concat_const_MatShapeR_const_MatShapeR(a.as_raw_VectorOfi32(), b.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
	Ok(ret)
}

// getAvailableBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:104
// ("cv::dnn::getAvailableBackends", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_available_backends() -> Result<core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getAvailableBackends(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>::opencv_from_extern(ret) };
	Ok(ret)
}

// getAvailableTargets(dnn::Backend)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:105
// ("cv::dnn::getAvailableTargets", vec![(pred!(mut, ["be"], ["cv::dnn::Backend"]), _)]),
#[inline]
pub fn get_available_targets(be: crate::dnn::Backend) -> Result<core::Vector<crate::dnn::Target>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getAvailableTargets_Backend(be, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Vector::<crate::dnn::Target>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns Inference Engine internal backend API.
///
/// See values of `CV_DNN_BACKEND_INFERENCE_ENGINE_*` macros.
///
/// `OPENCV_DNN_BACKEND_INFERENCE_ENGINE_TYPE` runtime parameter (environment variable) is ignored since 4.6.0.
///
/// @deprecated
// getInferenceEngineBackendType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:31
// ("cv::dnn::getInferenceEngineBackendType", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_inference_engine_backend_type() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getInferenceEngineBackendType(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns Inference Engine CPU type.
///
/// Specify OpenVINO plugin: CPU or ARM.
// getInferenceEngineCPUType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:72
// ("cv::dnn::getInferenceEngineCPUType", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_inference_engine_cpu_type() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getInferenceEngineCPUType(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// Returns Inference Engine VPU type.
///
/// See values of `CV_DNN_INFERENCE_ENGINE_VPU_TYPE_*` macros.
// getInferenceEngineVPUType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:66
// ("cv::dnn::getInferenceEngineVPUType", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn get_inference_engine_vpu_type() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getInferenceEngineVPUType(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// getPlane(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:108
// ("cv::dnn::getPlane", vec![(pred!(mut, ["m", "n", "cn"], ["const cv::Mat*", "int", "int"]), _)]),
#[inline]
pub fn get_plane(m: &impl core::MatTraitConst, n: i32, cn: i32) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_getPlane_const_MatR_int_int(m.as_raw_Mat(), n, cn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Parse a 4D blob and output the images it contains as 2D arrays through a simpler data structure
/// (std::vector<cv::Mat>).
/// ## Parameters
/// * blob_: 4 dimensional array (images, channels, height, width) in floating point precision (CV_32F) from
/// which you would like to extract the images.
/// * images_:[out] array of 2D Mat containing the images extracted from the blob in floating point precision
/// (CV_32F). They are non normalized neither mean added. The number of returned images equals the first dimension
/// of the blob (batch size). Every image has a number of channels equals to the second dimension of the blob (depth).
// imagesFromBlob(const cv::Mat &, OutputArrayOfArrays)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1037
// ("cv::dnn::imagesFromBlob", vec![(pred!(mut, ["blob_", "images_"], ["const cv::Mat*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn images_from_blob(blob_: &impl core::MatTraitConst, images_: &mut impl ToOutputArray) -> Result<()> {
	output_array_arg!(images_);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_imagesFromBlob_const_MatR_const__OutputArrayR(blob_.as_raw_Mat(), images_.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Reads a network model stored in <a href="http://caffe.berkeleyvision.org">Caffe</a> framework's format.
/// ## Parameters
/// * prototxt: path to the .prototxt file with text description of the network architecture.
/// * caffeModel: path to the .caffemodel file with learned network.
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_caffe] function uses the following default values for its arguments:
/// * caffe_model: String()
// cv::dnn::readNetFromCaffe(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:787
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_caffe_def(prototxt: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(prototxt);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_StringR(prototxt.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="http://caffe.berkeleyvision.org">Caffe</a> framework's format.
/// ## Parameters
/// * prototxt: path to the .prototxt file with text description of the network architecture.
/// * caffeModel: path to the .caffemodel file with learned network.
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * caffe_model: String()
// readNetFromCaffe(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:787
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt", "caffeModel"], ["const cv::String*", "const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_caffe(prototxt: &str, caffe_model: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(prototxt);
	extern_container_arg!(caffe_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_StringR_const_StringR(prototxt.opencv_as_extern(), caffe_model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in Caffe model in memory.
/// @details This is an overloaded member function, provided for convenience.
/// It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * bufferProto: buffer containing the content of the .prototxt file
/// * lenProto: length of bufferProto
/// * bufferModel: buffer containing the content of the .caffemodel file
/// * lenModel: length of bufferModel
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_caffe_str] function uses the following default values for its arguments:
/// * buffer_model: NULL
/// * len_model: 0
// cv::dnn::readNetFromCaffe(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:806
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto"], ["const char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_caffe_str_def(buffer_proto: &str, len_proto: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_proto);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_charX_size_t(buffer_proto.opencv_as_extern(), len_proto, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in Caffe model in memory.
/// @details This is an overloaded member function, provided for convenience.
/// It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * bufferProto: buffer containing the content of the .prototxt file
/// * lenProto: length of bufferProto
/// * bufferModel: buffer containing the content of the .caffemodel file
/// * lenModel: length of bufferModel
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * buffer_model: NULL
/// * len_model: 0
// readNetFromCaffe(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:806
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_caffe_str(buffer_proto: &str, len_proto: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_proto);
	extern_container_arg!(buffer_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(buffer_proto.opencv_as_extern(), len_proto, buffer_model.opencv_as_extern(), len_model, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in Caffe model in memory.
/// ## Parameters
/// * bufferProto: buffer containing the content of the .prototxt file
/// * bufferModel: buffer containing the content of the .caffemodel file
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_caffe_buffer] function uses the following default values for its arguments:
/// * buffer_model: std::vector<uchar>()
// cv::dnn::readNetFromCaffe(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:794
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto"], ["const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_caffe_buffer_def(buffer_proto: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR(buffer_proto.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in Caffe model in memory.
/// ## Parameters
/// * bufferProto: buffer containing the content of the .prototxt file
/// * bufferModel: buffer containing the content of the .caffemodel file
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * buffer_model: std::vector<uchar>()
// readNetFromCaffe(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:794
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_caffe_buffer(buffer_proto: &core::Vector<u8>, buffer_model: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_proto.as_raw_VectorOfu8(), buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * cfgFile: path to the .cfg file with text description of the network architecture.
/// * darknetModel: path to the .weights file with learned network.
/// ## Returns
/// Network object that ready to do forward, throw an exception in failure cases.
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_darknet] function uses the following default values for its arguments:
/// * darknet_model: String()
// cv::dnn::readNetFromDarknet(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:762
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_darknet_def(cfg_file: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(cfg_file);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_StringR(cfg_file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * cfgFile: path to the .cfg file with text description of the network architecture.
/// * darknetModel: path to the .weights file with learned network.
/// ## Returns
/// Network object that ready to do forward, throw an exception in failure cases.
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * darknet_model: String()
// readNetFromDarknet(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:762
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile", "darknetModel"], ["const cv::String*", "const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_darknet(cfg_file: &str, darknet_model: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(cfg_file);
	extern_container_arg!(darknet_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_StringR_const_StringR(cfg_file.opencv_as_extern(), darknet_model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * bufferCfg: A buffer contains a content of .cfg file with text description of the network architecture.
/// * lenCfg: Number of bytes to read from bufferCfg
/// * bufferModel: A buffer contains a content of .weights file with learned network.
/// * lenModel: Number of bytes to read from bufferModel
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_darknet_str] function uses the following default values for its arguments:
/// * buffer_model: NULL
/// * len_model: 0
// cv::dnn::readNetFromDarknet(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:779
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg"], ["const char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_darknet_str_def(buffer_cfg: &str, len_cfg: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_cfg);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_charX_size_t(buffer_cfg.opencv_as_extern(), len_cfg, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * bufferCfg: A buffer contains a content of .cfg file with text description of the network architecture.
/// * lenCfg: Number of bytes to read from bufferCfg
/// * bufferModel: A buffer contains a content of .weights file with learned network.
/// * lenModel: Number of bytes to read from bufferModel
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * buffer_model: NULL
/// * len_model: 0
// readNetFromDarknet(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:779
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_darknet_str(buffer_cfg: &str, len_cfg: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_cfg);
	extern_container_arg!(buffer_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(buffer_cfg.opencv_as_extern(), len_cfg, buffer_model.opencv_as_extern(), len_model, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * bufferCfg: A buffer contains a content of .cfg file with text description of the network architecture.
/// * bufferModel: A buffer contains a content of .weights file with learned network.
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_darknet_buffer] function uses the following default values for its arguments:
/// * buffer_model: std::vector<uchar>()
// cv::dnn::readNetFromDarknet(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:769
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg"], ["const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_darknet_buffer_def(buffer_cfg: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR(buffer_cfg.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * bufferCfg: A buffer contains a content of .cfg file with text description of the network architecture.
/// * bufferModel: A buffer contains a content of .weights file with learned network.
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * buffer_model: std::vector<uchar>()
// readNetFromDarknet(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:769
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_darknet_buffer(buffer_cfg: &core::Vector<u8>, buffer_model: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_cfg.as_raw_VectorOfu8(), buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * xml: XML configuration file with network's topology.
/// * bin: Binary file with trained weights.
/// ## Returns
/// Net object.
/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
/// backend.
// readNetFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:916
// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(xml);
	extern_container_arg!(bin);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_StringR_const_StringR(xml.opencv_as_extern(), bin.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * bufferModelConfigPtr: Pointer to buffer which contains XML configuration with network's topology.
/// * bufferModelConfigSize: Binary size of XML configuration data.
/// * bufferWeightsPtr: Pointer to buffer which contains binary data with trained weights.
/// * bufferWeightsSize: Binary size of trained weights data.
/// ## Returns
/// Net object.
/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
/// backend.
// readNetFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:938
// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_model_optimizer_2(buffer_model_config_ptr: &[u8], buffer_weights_ptr: &[u8]) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr.as_ptr(), buffer_model_config_ptr.len(), buffer_weights_ptr.as_ptr(), buffer_weights_ptr.len(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * bufferModelConfig: Buffer contains XML configuration with network's topology.
/// * bufferWeights: Buffer contains binary data with trained weights.
/// ## Returns
/// Net object.
/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
/// backend.
// readNetFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:926
// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_model_optimizer_1(buffer_model_config: &core::Vector<u8>, buffer_weights: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_model_config.as_raw_VectorOfu8(), buffer_weights.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model <a href="https://onnx.ai/">ONNX</a>.
/// ## Parameters
/// * onnxFile: path to the .onnx file with text description of the network architecture.
/// ## Returns
/// Network object that ready to do forward, throw an exception in failure cases.
// readNetFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:945
// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["onnxFile"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_onnx(onnx_file: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(onnx_file);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromONNX_const_StringR(onnx_file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model from <a href="https://onnx.ai/">ONNX</a>
///        in-memory buffer.
/// ## Parameters
/// * buffer: memory address of the first byte of the buffer.
/// * sizeBuffer: size of the buffer.
/// ## Returns
/// Network object that ready to do forward, throw an exception
///       in failure cases.
// readNetFromONNX(const char *, size_t)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:954
// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer", "sizeBuffer"], ["const char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_onnx_str(buffer: &str, size_buffer: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromONNX_const_charX_size_t(buffer.opencv_as_extern(), size_buffer, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model from <a href="https://onnx.ai/">ONNX</a>
///        in-memory buffer.
/// ## Parameters
/// * buffer: in-memory buffer that stores the ONNX model bytes.
/// ## Returns
/// Network object that ready to do forward, throw an exception
///       in failure cases.
// readNetFromONNX(const std::vector<uchar> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:962
// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer"], ["const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_onnx_buffer(buffer: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromONNX_const_vectorLunsigned_charGR(buffer.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * model: path to the .pb file with binary protobuf description of the network architecture
/// * config: path to the .pbtxt file that contains text graph definition in protobuf format.
///               Resulting Net object is built by text graph using weights from a binary one that
///               let us make it more flexible.
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_tensorflow] function uses the following default values for its arguments:
/// * config: String()
// cv::dnn::readNetFromTensorflow(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:816
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_tensorflow_def(model: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * model: path to the .pb file with binary protobuf description of the network architecture
/// * config: path to the .pbtxt file that contains text graph definition in protobuf format.
///               Resulting Net object is built by text graph using weights from a binary one that
///               let us make it more flexible.
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * config: String()
// readNetFromTensorflow(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:816
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_tensorflow(model: &str, config: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	extern_container_arg!(config);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// @details This is an overloaded member function, provided for convenience.
/// It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * bufferModel: buffer containing the content of the pb file
/// * lenModel: length of bufferModel
/// * bufferConfig: buffer containing the content of the pbtxt file
/// * lenConfig: length of bufferConfig
///
/// ## Note
/// This alternative version of [read_net_from_tensorflow_str] function uses the following default values for its arguments:
/// * buffer_config: NULL
/// * len_config: 0
// cv::dnn::readNetFromTensorflow(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:834
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel"], ["const char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_tensorflow_str_def(buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_charX_size_t(buffer_model.opencv_as_extern(), len_model, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// @details This is an overloaded member function, provided for convenience.
/// It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * bufferModel: buffer containing the content of the pb file
/// * lenModel: length of bufferModel
/// * bufferConfig: buffer containing the content of the pbtxt file
/// * lenConfig: length of bufferConfig
///
/// ## C++ default parameters
/// * buffer_config: NULL
/// * len_config: 0
// readNetFromTensorflow(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:834
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel", "bufferConfig", "lenConfig"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
#[inline]
pub fn read_net_from_tensorflow_str(buffer_model: &str, len_model: size_t, buffer_config: &str, len_config: size_t) -> Result<crate::dnn::Net> {
	extern_container_arg!(buffer_model);
	extern_container_arg!(buffer_config);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(buffer_model.opencv_as_extern(), len_model, buffer_config.opencv_as_extern(), len_config, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * bufferModel: buffer containing the content of the pb file
/// * bufferConfig: buffer containing the content of the pbtxt file
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_from_tensorflow_buffer] function uses the following default values for its arguments:
/// * buffer_config: std::vector<uchar>()
// cv::dnn::readNetFromTensorflow(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:823
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel"], ["const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_tensorflow_buffer_def(buffer_model: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR(buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * bufferModel: buffer containing the content of the pb file
/// * bufferConfig: buffer containing the content of the pbtxt file
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * buffer_config: std::vector<uchar>()
// readNetFromTensorflow(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:823
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "bufferConfig"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_from_tensorflow_buffer(buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="http://torch.ch">Torch7</a> framework's format.
/// ## Parameters
/// * model: path to the file, dumped from Torch by using torch.save() function.
/// * isBinary: specifies whether the network was serialized in ascii mode or binary.
/// * evaluate: specifies testing phase of network. If true, it's similar to evaluate() method in Torch.
/// ## Returns
/// Net object.
///
///  
/// Note: Ascii mode of Torch serializer is more preferable, because binary mode extensively use `long` type of C language,
///  which has various bit-length on different systems.
///
/// The loading file must contain serialized <a href="https://github.com/torch/nn/blob/master/doc/module.md">nn.Module</a> object
/// with importing network. Try to eliminate a custom objects from serialazing data to avoid importing errors.
///
/// List of supported layers (i.e. object instances derived from Torch nn.Module class):
/// - nn.Sequential
/// - nn.Parallel
/// - nn.Concat
/// - nn.Linear
/// - nn.SpatialConvolution
/// - nn.SpatialMaxPooling, nn.SpatialAveragePooling
/// - nn.ReLU, nn.TanH, nn.Sigmoid
/// - nn.Reshape
/// - nn.SoftMax, nn.LogSoftMax
///
/// Also some equivalents of these classes from cunn, cudnn, and fbcunn may be successfully imported.
///
/// ## Note
/// This alternative version of [read_net_from_torch] function uses the following default values for its arguments:
/// * is_binary: true
/// * evaluate: true
// cv::dnn::readNetFromTorch(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:863
// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_net_from_torch_def(model: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTorch_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Reads a network model stored in <a href="http://torch.ch">Torch7</a> framework's format.
/// ## Parameters
/// * model: path to the file, dumped from Torch by using torch.save() function.
/// * isBinary: specifies whether the network was serialized in ascii mode or binary.
/// * evaluate: specifies testing phase of network. If true, it's similar to evaluate() method in Torch.
/// ## Returns
/// Net object.
///
///  
/// Note: Ascii mode of Torch serializer is more preferable, because binary mode extensively use `long` type of C language,
///  which has various bit-length on different systems.
///
/// The loading file must contain serialized <a href="https://github.com/torch/nn/blob/master/doc/module.md">nn.Module</a> object
/// with importing network. Try to eliminate a custom objects from serialazing data to avoid importing errors.
///
/// List of supported layers (i.e. object instances derived from Torch nn.Module class):
/// - nn.Sequential
/// - nn.Parallel
/// - nn.Concat
/// - nn.Linear
/// - nn.SpatialConvolution
/// - nn.SpatialMaxPooling, nn.SpatialAveragePooling
/// - nn.ReLU, nn.TanH, nn.Sigmoid
/// - nn.Reshape
/// - nn.SoftMax, nn.LogSoftMax
///
/// Also some equivalents of these classes from cunn, cudnn, and fbcunn may be successfully imported.
///
/// ## C++ default parameters
/// * is_binary: true
/// * evaluate: true
// readNetFromTorch(const String &, bool, bool)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:863
// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model", "isBinary", "evaluate"], ["const cv::String*", "bool", "bool"]), _)]),
#[inline]
pub fn read_net_from_torch(model: &str, is_binary: bool, evaluate: bool) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNetFromTorch_const_StringR_bool_bool(model.opencv_as_extern(), is_binary, evaluate, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Read deep learning network represented in one of the supported formats.
/// ## Parameters
/// * model: Binary file contains trained weights. The following file
///                  extensions are expected for models from different frameworks:
///                  * `*.caffemodel` (Caffe, <http://caffe.berkeleyvision.org/>)
///                  * `*.pb` (TensorFlow, <https://www.tensorflow.org/>)
///                  * `*.t7` | `*.net` (Torch, <http://torch.ch/>)
///                  * `*.weights` (Darknet, <https://pjreddie.com/darknet/>)
///                  * `*.bin` (DLDT, <https://software.intel.com/openvino-toolkit>)
///                  * `*.onnx` (ONNX, <https://onnx.ai/>)
/// * config: Text file contains network configuration. It could be a
///                   file with the following extensions:
///                  * `*.prototxt` (Caffe, <http://caffe.berkeleyvision.org/>)
///                  * `*.pbtxt` (TensorFlow, <https://www.tensorflow.org/>)
///                  * `*.cfg` (Darknet, <https://pjreddie.com/darknet/>)
///                  * `*.xml` (DLDT, <https://software.intel.com/openvino-toolkit>)
/// * framework: Explicit framework name tag to determine a format.
/// ## Returns
/// Net object.
///
/// This function automatically detects an origin framework of trained model
/// and calls an appropriate function such [readNetFromCaffe], [readNetFromTensorflow],
/// [readNetFromTorch] or [readNetFromDarknet]. An order of @p model and @p config
/// arguments does not matter.
///
/// ## Note
/// This alternative version of [read_net] function uses the following default values for its arguments:
/// * config: ""
/// * framework: ""
// cv::dnn::readNet(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:889
// ("cv::dnn::readNet", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_net_def(model: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNet_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Read deep learning network represented in one of the supported formats.
/// ## Parameters
/// * model: Binary file contains trained weights. The following file
///                  extensions are expected for models from different frameworks:
///                  * `*.caffemodel` (Caffe, <http://caffe.berkeleyvision.org/>)
///                  * `*.pb` (TensorFlow, <https://www.tensorflow.org/>)
///                  * `*.t7` | `*.net` (Torch, <http://torch.ch/>)
///                  * `*.weights` (Darknet, <https://pjreddie.com/darknet/>)
///                  * `*.bin` (DLDT, <https://software.intel.com/openvino-toolkit>)
///                  * `*.onnx` (ONNX, <https://onnx.ai/>)
/// * config: Text file contains network configuration. It could be a
///                   file with the following extensions:
///                  * `*.prototxt` (Caffe, <http://caffe.berkeleyvision.org/>)
///                  * `*.pbtxt` (TensorFlow, <https://www.tensorflow.org/>)
///                  * `*.cfg` (Darknet, <https://pjreddie.com/darknet/>)
///                  * `*.xml` (DLDT, <https://software.intel.com/openvino-toolkit>)
/// * framework: Explicit framework name tag to determine a format.
/// ## Returns
/// Net object.
///
/// This function automatically detects an origin framework of trained model
/// and calls an appropriate function such [readNetFromCaffe], [readNetFromTensorflow],
/// [readNetFromTorch] or [readNetFromDarknet]. An order of @p model and @p config
/// arguments does not matter.
///
/// ## C++ default parameters
/// * config: ""
/// * framework: ""
// readNet(const String &, const String &, const String &)(InString, InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:889
// ("cv::dnn::readNet", vec![(pred!(mut, ["model", "config", "framework"], ["const cv::String*", "const cv::String*", "const cv::String*"]), _)]),
#[inline]
pub fn read_net(model: &str, config: &str, framework: &str) -> Result<crate::dnn::Net> {
	extern_container_arg!(model);
	extern_container_arg!(config);
	extern_container_arg!(framework);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNet_const_StringR_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), framework.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Read deep learning network represented in one of the supported formats.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * framework: Name of origin framework.
/// * bufferModel: A buffer with a content of binary file with weights
/// * bufferConfig: A buffer with a content of text file contains network configuration.
/// ## Returns
/// Net object.
///
/// ## Note
/// This alternative version of [read_net_1] function uses the following default values for its arguments:
/// * buffer_config: std::vector<uchar>()
// cv::dnn::readNet(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:900
// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel"], ["const cv::String*", "const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_1_def(framework: &str, buffer_model: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	extern_container_arg!(framework);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR(framework.opencv_as_extern(), buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Read deep learning network represented in one of the supported formats.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * framework: Name of origin framework.
/// * bufferModel: A buffer with a content of binary file with weights
/// * bufferConfig: A buffer with a content of text file contains network configuration.
/// ## Returns
/// Net object.
///
/// ## C++ default parameters
/// * buffer_config: std::vector<uchar>()
// readNet(const String &, const std::vector<uchar> &, const std::vector<uchar> &)(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:900
// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
#[inline]
pub fn read_net_1(framework: &str, buffer_model: &core::Vector<u8>, buffer_config: &core::Vector<u8>) -> Result<crate::dnn::Net> {
	extern_container_arg!(framework);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(framework.opencv_as_extern(), buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates blob from .pb file.
/// ## Parameters
/// * path: to the .pb file with input tensor.
/// ## Returns
/// Mat.
// readTensorFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:968
// ("cv::dnn::readTensorFromONNX", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_tensor_from_onnx(path: &str) -> Result<core::Mat> {
	extern_container_arg!(path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readTensorFromONNX_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
/// @warning This function has the same limitations as readNetFromTorch().
///
/// ## Note
/// This alternative version of [read_torch_blob] function uses the following default values for its arguments:
/// * is_binary: true
// cv::dnn::readTorchBlob(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:906
// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
#[inline]
pub fn read_torch_blob_def(filename: &str) -> Result<core::Mat> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readTorchBlob_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
/// @warning This function has the same limitations as readNetFromTorch().
///
/// ## C++ default parameters
/// * is_binary: true
// readTorchBlob(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:906
// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename", "isBinary"], ["const cv::String*", "bool"]), _)]),
#[inline]
pub fn read_torch_blob(filename: &str, is_binary: bool) -> Result<core::Mat> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_readTorchBlob_const_StringR_bool(filename.opencv_as_extern(), is_binary, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// Release a Myriad device (binded by OpenCV).
///
/// Single Myriad device cannot be shared across multiple processes which uses
/// Inference Engine's Myriad plugin.
// resetMyriadDevice()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:49
// ("cv::dnn::resetMyriadDevice", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn reset_myriad_device() -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_resetMyriadDevice(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Specify Inference Engine internal backend API.
///
/// See values of `CV_DNN_BACKEND_INFERENCE_ENGINE_*` macros.
///
/// ## Returns
/// previous value of internal backend API
///
/// @deprecated
// setInferenceEngineBackendType(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:41
// ("cv::dnn::setInferenceEngineBackendType", vec![(pred!(mut, ["newBackendType"], ["const cv::String*"]), _)]),
#[inline]
pub fn set_inference_engine_backend_type(new_backend_type: &str) -> Result<String> {
	extern_container_arg!(new_backend_type);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_setInferenceEngineBackendType_const_StringR(new_backend_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

// shape(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:126
// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
#[inline]
pub fn shape_1(mat: &impl core::MatTraitConst) -> Result<crate::dnn::MatShape> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_MatR(mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
	Ok(ret)
}

// shape(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:131
// ("cv::dnn::shape", vec![(pred!(mut, ["sz"], ["const cv::MatSize*"]), _)]),
#[inline]
pub fn shape_2(sz: &impl core::MatSizeTraitConst) -> Result<crate::dnn::MatShape> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_MatSizeR(sz.as_raw_MatSize(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
	Ok(ret)
}

// shape(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:136
// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::UMat*"]), _)]),
#[inline]
pub fn shape_3(mat: &impl core::UMatTraitConst) -> Result<crate::dnn::MatShape> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_UMatR(mat.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
	Ok(ret)
}

// shape(const int *, const int)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:119
// ("cv::dnn::shape", vec![(pred!(mut, ["dims", "n"], ["const int*", "const int"]), _)]),
#[inline]
pub fn shape(dims: &i32, n: i32) -> Result<crate::dnn::MatShape> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_const_intX_const_int(dims, n, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## Note
/// This alternative version of [shape_4] function uses the following default values for its arguments:
/// * a1: -1
/// * a2: -1
/// * a3: -1
// cv::dnn::shape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:153
// ("cv::dnn::shape", vec![(pred!(mut, ["a0"], ["int"]), _)]),
#[inline]
pub fn shape_4_def(a0: i32) -> Result<crate::dnn::MatShape> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_int(a0, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * a1: -1
/// * a2: -1
/// * a3: -1
// shape(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:153
// ("cv::dnn::shape", vec![(pred!(mut, ["a0", "a1", "a2", "a3"], ["int", "int", "int", "int"]), _)]),
#[inline]
pub fn shape_4(a0: i32, a1: i32, a2: i32, a3: i32) -> Result<crate::dnn::MatShape> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shape_int_int_int_int(a0, a1, a2, a3, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
	Ok(ret)
}

/// Convert all weights of Caffe network to half precision floating point.
/// ## Parameters
/// * src: Path to origin model from Caffe framework contains single
///            precision floating point weights (usually has `.caffemodel` extension).
/// * dst: Path to destination model with updated weights.
/// * layersTypes: Set of layers types which parameters will be converted.
///                    By default, converts only Convolutional and Fully-Connected layers'
///                    weights.
///
///
/// Note: Shrinked model has no origin float32 weights so it can't be used
///       in origin Caffe framework anymore. However the structure of data
///       is taken from NVidia's Caffe fork: <https://github.com/NVIDIA/caffe>.
///       So the resulting model may be used there.
///
/// ## Note
/// This alternative version of [shrink_caffe_model] function uses the following default values for its arguments:
/// * layers_types: std::vector<String>()
// cv::dnn::shrinkCaffeModel(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1052
// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst"], ["const cv::String*", "const cv::String*"]), _)]),
#[inline]
pub fn shrink_caffe_model_def(src: &str, dst: &str) -> Result<()> {
	extern_container_arg!(src);
	extern_container_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shrinkCaffeModel_const_StringR_const_StringR(src.opencv_as_extern(), dst.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Convert all weights of Caffe network to half precision floating point.
/// ## Parameters
/// * src: Path to origin model from Caffe framework contains single
///            precision floating point weights (usually has `.caffemodel` extension).
/// * dst: Path to destination model with updated weights.
/// * layersTypes: Set of layers types which parameters will be converted.
///                    By default, converts only Convolutional and Fully-Connected layers'
///                    weights.
///
///
/// Note: Shrinked model has no origin float32 weights so it can't be used
///       in origin Caffe framework anymore. However the structure of data
///       is taken from NVidia's Caffe fork: <https://github.com/NVIDIA/caffe>.
///       So the resulting model may be used there.
///
/// ## C++ default parameters
/// * layers_types: std::vector<String>()
// shrinkCaffeModel(const String &, const String &, const std::vector<String> &)(InString, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1052
// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst", "layersTypes"], ["const cv::String*", "const cv::String*", "const std::vector<cv::String>*"]), _)]),
#[inline]
pub fn shrink_caffe_model(src: &str, dst: &str, layers_types: &core::Vector<String>) -> Result<()> {
	extern_container_arg!(src);
	extern_container_arg!(dst);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_shrinkCaffeModel_const_StringR_const_StringR_const_vectorLStringGR(src.opencv_as_extern(), dst.opencv_as_extern(), layers_types.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// slice(const Mat &, const _Range &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:63
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0"], ["const cv::Mat*", "const cv::dnn::_Range*"]), _)]),
#[inline]
pub fn slice(m: &impl core::MatTraitConst, r0: &impl crate::dnn::_RangeTraitConst) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// slice(const Mat &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:72
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
#[inline]
pub fn slice_1(m: &impl core::MatTraitConst, r0: &impl crate::dnn::_RangeTraitConst, r1: &impl crate::dnn::_RangeTraitConst) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// slice(const Mat &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:83
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
#[inline]
pub fn slice_2(m: &impl core::MatTraitConst, r0: &impl crate::dnn::_RangeTraitConst, r1: &impl crate::dnn::_RangeTraitConst, r2: &impl crate::dnn::_RangeTraitConst) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

// slice(const Mat &, const _Range &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:95
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2", "r3"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
#[inline]
pub fn slice_3(m: &impl core::MatTraitConst, r0: &impl crate::dnn::_RangeTraitConst, r1: &impl crate::dnn::_RangeTraitConst, r2: &impl crate::dnn::_RangeTraitConst, r3: &impl crate::dnn::_RangeTraitConst) -> Result<core::Mat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range(), r3.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Mat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## Note
/// This alternative version of [total_1] function uses the following default values for its arguments:
/// * start: -1
/// * end: -1
// cv::dnn::total(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:184
// ("cv::dnn::total", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
#[inline]
pub fn total_1_def(mat: &impl core::MatTraitConst) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_total_const_MatR(mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * start: -1
/// * end: -1
// total(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:184
// ("cv::dnn::total", vec![(pred!(mut, ["mat", "start", "end"], ["const cv::Mat*", "int", "int"]), _)]),
#[inline]
pub fn total_1(mat: &impl core::MatTraitConst, start: i32, end: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_total_const_MatR_int_int(mat.as_raw_Mat(), start, end, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## Note
/// This alternative version of [total] function uses the following default values for its arguments:
/// * start: -1
/// * end: -1
// cv::dnn::total(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:161
// ("cv::dnn::total", vec![(pred!(mut, ["shape"], ["const cv::dnn::MatShape*"]), _)]),
#[inline]
pub fn total_def(shape: &crate::dnn::MatShape) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_total_const_MatShapeR(shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * start: -1
/// * end: -1
// total(const MatShape &, int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:161
// ("cv::dnn::total", vec![(pred!(mut, ["shape", "start", "end"], ["const cv::dnn::MatShape*", "int", "int"]), _)]),
#[inline]
pub fn total(shape: &crate::dnn::MatShape, start: i32, end: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_total_const_MatShapeR_int_int(shape.as_raw_VectorOfi32(), start, end, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Create a text representation for a binary network stored in protocol buffer format.
/// ## Parameters
/// * model: A path to binary network.
/// * output: A path to output text file to be created.
///
///
/// Note: To reduce output file size, trained weights are not included.
// writeTextGraph(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1061
// ("cv::dnn::writeTextGraph", vec![(pred!(mut, ["model", "output"], ["const cv::String*", "const cv::String*"]), _)]),
#[inline]
pub fn write_text_graph(model: &str, output: &str) -> Result<()> {
	extern_container_arg!(model);
	extern_container_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_dnn_writeTextGraph_const_StringR_const_StringR(model.opencv_as_extern(), output.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::dnn::AbsLayer]
// AbsLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:491
pub trait AbsLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_AbsLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::AbsLayer]
pub trait AbsLayerTrait: crate::dnn::AbsLayerTraitConst + crate::dnn::ActivationLayerTrait {
	fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void;

}

// AbsLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:491
pub struct AbsLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { AbsLayer }

impl Drop for AbsLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_AbsLayer_delete(self.as_raw_mut_AbsLayer()) };
	}
}

unsafe impl Send for AbsLayer {}

impl crate::dnn::ActivationLayerTraitConst for AbsLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for AbsLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AbsLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for AbsLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AbsLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AbsLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for AbsLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AbsLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AbsLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::AbsLayerTraitConst for AbsLayer {
	#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AbsLayerTrait for AbsLayer {
	#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AbsLayer, crate::dnn::AbsLayerTraitConst, as_raw_AbsLayer, crate::dnn::AbsLayerTrait, as_raw_mut_AbsLayer }

impl AbsLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:494
	// ("cv::dnn::AbsLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::AbsLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AbsLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AbsLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AbsLayer, crate::dnn::ActivationLayer, cv_dnn_AbsLayer_to_ActivationLayer }

boxed_cast_base! { AbsLayer, core::Algorithm, cv_dnn_AbsLayer_to_Algorithm }

boxed_cast_base! { AbsLayer, crate::dnn::Layer, cv_dnn_AbsLayer_to_Layer }

impl std::fmt::Debug for AbsLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AbsLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::AccumLayer]
// AccumLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:580
pub trait AccumLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_AccumLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::AccumLayer]
pub trait AccumLayerTrait: crate::dnn::AccumLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void;

}

// AccumLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:580
pub struct AccumLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { AccumLayer }

impl Drop for AccumLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_AccumLayer_delete(self.as_raw_mut_AccumLayer()) };
	}
}

unsafe impl Send for AccumLayer {}

impl core::AlgorithmTraitConst for AccumLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for AccumLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AccumLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for AccumLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for AccumLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AccumLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::AccumLayerTraitConst for AccumLayer {
	#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::AccumLayerTrait for AccumLayer {
	#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { AccumLayer, crate::dnn::AccumLayerTraitConst, as_raw_AccumLayer, crate::dnn::AccumLayerTrait, as_raw_mut_AccumLayer }

impl AccumLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_AccumLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:583
	// ("cv::dnn::AccumLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::AccumLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_AccumLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::AccumLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { AccumLayer, core::Algorithm, cv_dnn_AccumLayer_to_Algorithm }

boxed_cast_base! { AccumLayer, crate::dnn::Layer, cv_dnn_AccumLayer_to_Layer }

impl std::fmt::Debug for AccumLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AccumLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for AccumLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ActivationLayer]
// ActivationLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:424
pub trait ActivationLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ActivationLayer(&self) -> *const c_void;

	// forwardSlice(const float *, float *, int, size_t, int, int)(VariableArray, VariableArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:427
	// ("cv::dnn::ActivationLayer::forwardSlice", vec![(pred!(const, ["src", "dst", "len", "outPlaneSize", "cn0", "cn1"], ["const float*", "float*", "int", "size_t", "int", "int"]), _)]),
	#[inline]
	fn forward_slice(&self, src: &[f32], dst: &mut [f32], out_plane_size: size_t, cn0: i32, cn1: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(self.as_raw_ActivationLayer(), src.as_ptr(), dst.as_mut_ptr(), src.len().min(dst.len()).try_into()?, out_plane_size, cn0, cn1, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::dnn::ActivationLayer]
pub trait ActivationLayerTrait: crate::dnn::ActivationLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void;

}

// ActivationLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:424
pub struct ActivationLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ActivationLayer }

impl Drop for ActivationLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ActivationLayer_delete(self.as_raw_mut_ActivationLayer()) };
	}
}

unsafe impl Send for ActivationLayer {}

impl core::AlgorithmTraitConst for ActivationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ActivationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ActivationLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ActivationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ActivationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ActivationLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ActivationLayerTraitConst for ActivationLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ActivationLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ActivationLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl ActivationLayer {
}

boxed_cast_descendant! { ActivationLayer, crate::dnn::AbsLayer, cv_dnn_ActivationLayer_to_AbsLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::BNLLLayer, cv_dnn_ActivationLayer_to_BNLLLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::BatchNormLayer, cv_dnn_ActivationLayer_to_BatchNormLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ChannelsPReLULayer, cv_dnn_ActivationLayer_to_ChannelsPReLULayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ELULayer, cv_dnn_ActivationLayer_to_ELULayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ExpLayer, cv_dnn_ActivationLayer_to_ExpLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::MishLayer, cv_dnn_ActivationLayer_to_MishLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::PowerLayer, cv_dnn_ActivationLayer_to_PowerLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ReLU6Layer, cv_dnn_ActivationLayer_to_ReLU6Layer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::ReLULayer, cv_dnn_ActivationLayer_to_ReLULayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SigmoidLayer, cv_dnn_ActivationLayer_to_SigmoidLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::SwishLayer, cv_dnn_ActivationLayer_to_SwishLayer }

boxed_cast_descendant! { ActivationLayer, crate::dnn::TanHLayer, cv_dnn_ActivationLayer_to_TanHLayer }

boxed_cast_base! { ActivationLayer, core::Algorithm, cv_dnn_ActivationLayer_to_Algorithm }

boxed_cast_base! { ActivationLayer, crate::dnn::Layer, cv_dnn_ActivationLayer_to_Layer }

impl std::fmt::Debug for ActivationLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ActivationLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::BNLLLayer]
// BNLLLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:485
pub trait BNLLLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_BNLLLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::BNLLLayer]
pub trait BNLLLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::BNLLLayerTraitConst {
	fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void;

}

// BNLLLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:485
pub struct BNLLLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BNLLLayer }

impl Drop for BNLLLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_BNLLLayer_delete(self.as_raw_mut_BNLLLayer()) };
	}
}

unsafe impl Send for BNLLLayer {}

impl crate::dnn::ActivationLayerTraitConst for BNLLLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for BNLLLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BNLLLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for BNLLLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BNLLLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BNLLLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for BNLLLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BNLLLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BNLLLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::BNLLLayerTraitConst for BNLLLayer {
	#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BNLLLayerTrait for BNLLLayer {
	#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BNLLLayer, crate::dnn::BNLLLayerTraitConst, as_raw_BNLLLayer, crate::dnn::BNLLLayerTrait, as_raw_mut_BNLLLayer }

impl BNLLLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:488
	// ("cv::dnn::BNLLLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::BNLLLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BNLLLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BNLLLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BNLLLayer, crate::dnn::ActivationLayer, cv_dnn_BNLLLayer_to_ActivationLayer }

boxed_cast_base! { BNLLLayer, core::Algorithm, cv_dnn_BNLLLayer_to_Algorithm }

boxed_cast_base! { BNLLLayer, crate::dnn::Layer, cv_dnn_BNLLLayer_to_Layer }

impl std::fmt::Debug for BNLLLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BNLLLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::BackendNode]
// BackendNode /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:125
pub trait BackendNodeTraitConst {
	fn as_raw_BackendNode(&self) -> *const c_void;

	/// Backend identifier.
	// cv::dnn::BackendNode::backendId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:132
	// ("cv::dnn::BackendNode::backendId", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn backend_id(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BackendNode_propBackendId_const(self.as_raw_BackendNode()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::BackendNode]
pub trait BackendNodeTrait: crate::dnn::BackendNodeTraitConst {
	fn as_raw_mut_BackendNode(&mut self) -> *mut c_void;

	/// Backend identifier.
	// cv::dnn::BackendNode::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:132
	// ("cv::dnn::BackendNode::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_backend_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BackendNode_propBackendId_const_int(self.as_raw_mut_BackendNode(), val) };
		ret
	}

}

/// Derivatives of this class encapsulates functions of certain backends.
// BackendNode /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:125
pub struct BackendNode {
	ptr: *mut c_void,
}

opencv_type_boxed! { BackendNode }

impl Drop for BackendNode {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_BackendNode_delete(self.as_raw_mut_BackendNode()) };
	}
}

unsafe impl Send for BackendNode {}

impl crate::dnn::BackendNodeTraitConst for BackendNode {
	#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BackendNodeTrait for BackendNode {
	#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackendNode, crate::dnn::BackendNodeTraitConst, as_raw_BackendNode, crate::dnn::BackendNodeTrait, as_raw_mut_BackendNode }

impl BackendNode {
}

impl std::fmt::Debug for BackendNode {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BackendNode")
			.field("backend_id", &crate::dnn::BackendNodeTraitConst::backend_id(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::BackendWrapper]
// BackendWrapper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:138
pub trait BackendWrapperTraitConst {
	fn as_raw_BackendWrapper(&self) -> *const c_void;

	/// Backend identifier.
	// cv::dnn::BackendWrapper::backendId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:176
	// ("cv::dnn::BackendWrapper::backendId", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn backend_id(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_propBackendId_const(self.as_raw_BackendWrapper()) };
		ret
	}

	/// Target identifier.
	// cv::dnn::BackendWrapper::targetId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:177
	// ("cv::dnn::BackendWrapper::targetId", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn target_id(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_propTargetId_const(self.as_raw_BackendWrapper()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::BackendWrapper]
pub trait BackendWrapperTrait: crate::dnn::BackendWrapperTraitConst {
	fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void;

	/// Backend identifier.
	// cv::dnn::BackendWrapper::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:176
	// ("cv::dnn::BackendWrapper::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_backend_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_propBackendId_const_int(self.as_raw_mut_BackendWrapper(), val) };
		ret
	}

	/// Target identifier.
	// cv::dnn::BackendWrapper::setTargetId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:177
	// ("cv::dnn::BackendWrapper::setTargetId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_target_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BackendWrapper_propTargetId_const_int(self.as_raw_mut_BackendWrapper(), val) };
		ret
	}

	/// Transfer data to CPU host memory.
	// copyToHost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:169
	// ("cv::dnn::BackendWrapper::copyToHost", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn copy_to_host(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BackendWrapper_copyToHost(self.as_raw_mut_BackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Indicate that an actual data is on CPU.
	// setHostDirty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:174
	// ("cv::dnn::BackendWrapper::setHostDirty", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn set_host_dirty(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BackendWrapper_setHostDirty(self.as_raw_mut_BackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Derivatives of this class wraps cv::Mat for different backends and targets.
// BackendWrapper /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:138
pub struct BackendWrapper {
	ptr: *mut c_void,
}

opencv_type_boxed! { BackendWrapper }

impl Drop for BackendWrapper {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_BackendWrapper_delete(self.as_raw_mut_BackendWrapper()) };
	}
}

unsafe impl Send for BackendWrapper {}

impl crate::dnn::BackendWrapperTraitConst for BackendWrapper {
	#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BackendWrapperTrait for BackendWrapper {
	#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BackendWrapper, crate::dnn::BackendWrapperTraitConst, as_raw_BackendWrapper, crate::dnn::BackendWrapperTrait, as_raw_mut_BackendWrapper }

impl BackendWrapper {
}

impl std::fmt::Debug for BackendWrapper {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BackendWrapper")
			.field("backend_id", &crate::dnn::BackendWrapperTraitConst::backend_id(self))
			.field("target_id", &crate::dnn::BackendWrapperTraitConst::target_id(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::BaseConvolutionLayer]
// BaseConvolutionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:210
pub trait BaseConvolutionLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_BaseConvolutionLayer(&self) -> *const c_void;

	// cv::dnn::BaseConvolutionLayer::kernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::kernel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kernel(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::BaseConvolutionLayer::stride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::stride", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn stride(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_propStride_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::BaseConvolutionLayer::pad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::pad", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pad(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_propPad_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::BaseConvolutionLayer::dilation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::dilation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn dilation(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilation_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::BaseConvolutionLayer::adjustPad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::adjustPad", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adjust_pad(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjustPad_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::BaseConvolutionLayer::adjust_pads() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:214
	// ("cv::dnn::BaseConvolutionLayer::adjust_pads", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn adjust_pads(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjust_pads_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kernel_size(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_size_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::strides() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::strides", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn strides(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propStrides_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::dilations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::dilations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn dilations(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilations_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::pads_begin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pads_begin(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_begin_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::pads_end() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::pads_end", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pads_end(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_end_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::padMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:217
	// ("cv::dnn::BaseConvolutionLayer::padMode", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pad_mode(&self) -> String {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPadMode_const(self.as_raw_BaseConvolutionLayer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::numOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:218
	// ("cv::dnn::BaseConvolutionLayer::numOutput", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_output(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propNumOutput_const(self.as_raw_BaseConvolutionLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::BaseConvolutionLayer]
pub trait BaseConvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void;

	// cv::dnn::BaseConvolutionLayer::setKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_kernel(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_const_Size(self.as_raw_mut_BaseConvolutionLayer(), &val) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_stride(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propStride_const_Size(self.as_raw_mut_BaseConvolutionLayer(), &val) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_pad(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPad_const_Size(self.as_raw_mut_BaseConvolutionLayer(), &val) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setDilation(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setDilation", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_dilation(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilation_const_Size(self.as_raw_mut_BaseConvolutionLayer(), &val) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setAdjustPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
	// ("cv::dnn::BaseConvolutionLayer::setAdjustPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_adjust_pad(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjustPad_const_Size(self.as_raw_mut_BaseConvolutionLayer(), &val) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setAdjust_pads(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:214
	// ("cv::dnn::BaseConvolutionLayer::setAdjust_pads", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_adjust_pads(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjust_pads_const_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_kernel_size(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_size_const_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_strides(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propStrides_const_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setDilations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
	// ("cv::dnn::BaseConvolutionLayer::setDilations", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_dilations(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilations_const_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_pads_begin(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_begin_const_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
	// ("cv::dnn::BaseConvolutionLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_pads_end(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_end_const_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:217
	// ("cv::dnn::BaseConvolutionLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_pad_mode(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPadMode_const_String(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
		ret
	}

	// cv::dnn::BaseConvolutionLayer::setNumOutput(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:218
	// ("cv::dnn::BaseConvolutionLayer::setNumOutput", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_output(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propNumOutput_const_int(self.as_raw_mut_BaseConvolutionLayer(), val) };
		ret
	}

}

// BaseConvolutionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:210
pub struct BaseConvolutionLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BaseConvolutionLayer }

impl Drop for BaseConvolutionLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_delete(self.as_raw_mut_BaseConvolutionLayer()) };
	}
}

unsafe impl Send for BaseConvolutionLayer {}

impl core::AlgorithmTraitConst for BaseConvolutionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BaseConvolutionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BaseConvolutionLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for BaseConvolutionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BaseConvolutionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BaseConvolutionLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::BaseConvolutionLayerTraitConst for BaseConvolutionLayer {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for BaseConvolutionLayer {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BaseConvolutionLayer, crate::dnn::BaseConvolutionLayerTraitConst, as_raw_BaseConvolutionLayer, crate::dnn::BaseConvolutionLayerTrait, as_raw_mut_BaseConvolutionLayer }

impl BaseConvolutionLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_BaseConvolutionLayer_defaultNew_const()) }
	}

}

boxed_cast_base! { BaseConvolutionLayer, core::Algorithm, cv_dnn_BaseConvolutionLayer_to_Algorithm }

boxed_cast_base! { BaseConvolutionLayer, crate::dnn::Layer, cv_dnn_BaseConvolutionLayer_to_Layer }

impl std::fmt::Debug for BaseConvolutionLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BaseConvolutionLayer")
			.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
			.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
			.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
			.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
			.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
			.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
			.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
			.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
			.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
			.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
			.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
			.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
			.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for BaseConvolutionLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::BatchNormLayer]
// BatchNormLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:534
pub trait BatchNormLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_BatchNormLayer(&self) -> *const c_void;

	// cv::dnn::BatchNormLayer::hasWeights() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::hasWeights", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn has_weights(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasWeights_const(self.as_raw_BatchNormLayer()) };
		ret
	}

	// cv::dnn::BatchNormLayer::hasBias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::hasBias", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn has_bias(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasBias_const(self.as_raw_BatchNormLayer()) };
		ret
	}

	// cv::dnn::BatchNormLayer::epsilon() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:538
	// ("cv::dnn::BatchNormLayer::epsilon", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn epsilon(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_propEpsilon_const(self.as_raw_BatchNormLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::BatchNormLayer]
pub trait BatchNormLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::BatchNormLayerTraitConst {
	fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void;

	// cv::dnn::BatchNormLayer::setHasWeights(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::setHasWeights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_has_weights(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasWeights_const_bool(self.as_raw_mut_BatchNormLayer(), val) };
		ret
	}

	// cv::dnn::BatchNormLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
	// ("cv::dnn::BatchNormLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_has_bias(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasBias_const_bool(self.as_raw_mut_BatchNormLayer(), val) };
		ret
	}

	// cv::dnn::BatchNormLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:538
	// ("cv::dnn::BatchNormLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_epsilon(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_BatchNormLayer_propEpsilon_const_float(self.as_raw_mut_BatchNormLayer(), val) };
		ret
	}

}

// BatchNormLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:534
pub struct BatchNormLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BatchNormLayer }

impl Drop for BatchNormLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_BatchNormLayer_delete(self.as_raw_mut_BatchNormLayer()) };
	}
}

unsafe impl Send for BatchNormLayer {}

impl crate::dnn::ActivationLayerTraitConst for BatchNormLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BatchNormLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for BatchNormLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BatchNormLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for BatchNormLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BatchNormLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::BatchNormLayerTraitConst for BatchNormLayer {
	#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BatchNormLayerTrait for BatchNormLayer {
	#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BatchNormLayer, crate::dnn::BatchNormLayerTraitConst, as_raw_BatchNormLayer, crate::dnn::BatchNormLayerTrait, as_raw_mut_BatchNormLayer }

impl BatchNormLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:540
	// ("cv::dnn::BatchNormLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::BatchNormLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BatchNormLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BatchNormLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BatchNormLayer, crate::dnn::ActivationLayer, cv_dnn_BatchNormLayer_to_ActivationLayer }

boxed_cast_base! { BatchNormLayer, core::Algorithm, cv_dnn_BatchNormLayer_to_Algorithm }

boxed_cast_base! { BatchNormLayer, crate::dnn::Layer, cv_dnn_BatchNormLayer_to_Layer }

impl std::fmt::Debug for BatchNormLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BatchNormLayer")
			.field("has_weights", &crate::dnn::BatchNormLayerTraitConst::has_weights(self))
			.field("has_bias", &crate::dnn::BatchNormLayerTraitConst::has_bias(self))
			.field("epsilon", &crate::dnn::BatchNormLayerTraitConst::epsilon(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::BlankLayer]
// BlankLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:74
pub trait BlankLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_BlankLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::BlankLayer]
pub trait BlankLayerTrait: crate::dnn::BlankLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void;

}

/// # Partial List of Implemented Layers
/// This subsection of dnn module contains information about built-in layers and their descriptions.
///
/// Classes listed here, in fact, provides C++ API for creating instances of built-in layers.
/// In addition to this way of layers instantiation, there is a more common factory API (see [dnnLayerFactory]), it allows to create layers dynamically (by name) and register new ones.
/// You can use both API, but factory API is less convenient for native C++ programming and basically designed for use inside importers (see [readNetFromCaffe](), [readNetFromTorch](), [readNetFromTensorflow]()).
///
/// Built-in layers partially reproduce functionality of corresponding Caffe and Torch7 layers.
/// In particular, the following layers and Caffe importer were tested to reproduce <a href="http://caffe.berkeleyvision.org/tutorial/layers.html">Caffe</a> functionality:
/// - Convolution
/// - Deconvolution
/// - Pooling
/// - InnerProduct
/// - TanH, ReLU, Sigmoid, BNLL, Power, AbsVal
/// - Softmax
/// - Reshape, Flatten, Slice, Split
/// - LRN
/// - MVN
/// - Dropout (since it does nothing on forward pass -))
// BlankLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:74
pub struct BlankLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { BlankLayer }

impl Drop for BlankLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_BlankLayer_delete(self.as_raw_mut_BlankLayer()) };
	}
}

unsafe impl Send for BlankLayer {}

impl core::AlgorithmTraitConst for BlankLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BlankLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BlankLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for BlankLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for BlankLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BlankLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::BlankLayerTraitConst for BlankLayer {
	#[inline] fn as_raw_BlankLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BlankLayerTrait for BlankLayer {
	#[inline] fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { BlankLayer, crate::dnn::BlankLayerTraitConst, as_raw_BlankLayer, crate::dnn::BlankLayerTrait, as_raw_mut_BlankLayer }

impl BlankLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_BlankLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:77
	// ("cv::dnn::BlankLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_BlankLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { BlankLayer, core::Algorithm, cv_dnn_BlankLayer_to_Algorithm }

boxed_cast_base! { BlankLayer, crate::dnn::Layer, cv_dnn_BlankLayer_to_Layer }

impl std::fmt::Debug for BlankLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BlankLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for BlankLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ChannelsPReLULayer]
// ChannelsPReLULayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:447
pub trait ChannelsPReLULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ChannelsPReLULayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::ChannelsPReLULayer]
pub trait ChannelsPReLULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ChannelsPReLULayerTraitConst {
	fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void;

}

// ChannelsPReLULayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:447
pub struct ChannelsPReLULayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ChannelsPReLULayer }

impl Drop for ChannelsPReLULayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ChannelsPReLULayer_delete(self.as_raw_mut_ChannelsPReLULayer()) };
	}
}

unsafe impl Send for ChannelsPReLULayer {}

impl crate::dnn::ActivationLayerTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ChannelsPReLULayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ChannelsPReLULayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ChannelsPReLULayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ChannelsPReLULayerTraitConst for ChannelsPReLULayer {
	#[inline] fn as_raw_ChannelsPReLULayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ChannelsPReLULayerTrait for ChannelsPReLULayer {
	#[inline] fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ChannelsPReLULayer, crate::dnn::ChannelsPReLULayerTraitConst, as_raw_ChannelsPReLULayer, crate::dnn::ChannelsPReLULayerTrait, as_raw_mut_ChannelsPReLULayer }

impl ChannelsPReLULayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:450
	// ("cv::dnn::ChannelsPReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ChannelsPReLULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ChannelsPReLULayer, crate::dnn::ActivationLayer, cv_dnn_ChannelsPReLULayer_to_ActivationLayer }

boxed_cast_base! { ChannelsPReLULayer, core::Algorithm, cv_dnn_ChannelsPReLULayer_to_Algorithm }

boxed_cast_base! { ChannelsPReLULayer, crate::dnn::Layer, cv_dnn_ChannelsPReLULayer_to_Layer }

impl std::fmt::Debug for ChannelsPReLULayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ChannelsPReLULayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::ConcatLayer]
// ConcatLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:310
pub trait ConcatLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ConcatLayer(&self) -> *const c_void;

	// cv::dnn::ConcatLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:313
	// ("cv::dnn::ConcatLayer::axis", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_propAxis_const(self.as_raw_ConcatLayer()) };
		ret
	}

	/// Add zero padding in case of concatenation of blobs with different
	/// spatial sizes.
	///
	/// Details: <https://github.com/torch/nn/blob/master/doc/containers.md#depthconcat>
	// cv::dnn::ConcatLayer::padding() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:320
	// ("cv::dnn::ConcatLayer::padding", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn padding(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_propPadding_const(self.as_raw_ConcatLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ConcatLayer]
pub trait ConcatLayerTrait: crate::dnn::ConcatLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void;

	// cv::dnn::ConcatLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:313
	// ("cv::dnn::ConcatLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_propAxis_const_int(self.as_raw_mut_ConcatLayer(), val) };
		ret
	}

	/// Add zero padding in case of concatenation of blobs with different
	/// spatial sizes.
	///
	/// Details: <https://github.com/torch/nn/blob/master/doc/containers.md#depthconcat>
	// cv::dnn::ConcatLayer::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:320
	// ("cv::dnn::ConcatLayer::setPadding", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_padding(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_ConcatLayer_propPadding_const_bool(self.as_raw_mut_ConcatLayer(), val) };
		ret
	}

}

// ConcatLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:310
pub struct ConcatLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ConcatLayer }

impl Drop for ConcatLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ConcatLayer_delete(self.as_raw_mut_ConcatLayer()) };
	}
}

unsafe impl Send for ConcatLayer {}

impl core::AlgorithmTraitConst for ConcatLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ConcatLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConcatLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ConcatLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ConcatLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConcatLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ConcatLayerTraitConst for ConcatLayer {
	#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ConcatLayerTrait for ConcatLayer {
	#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConcatLayer, crate::dnn::ConcatLayerTraitConst, as_raw_ConcatLayer, crate::dnn::ConcatLayerTrait, as_raw_mut_ConcatLayer }

impl ConcatLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ConcatLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:322
	// ("cv::dnn::ConcatLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ConcatLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ConcatLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ConcatLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ConcatLayer, core::Algorithm, cv_dnn_ConcatLayer_to_Algorithm }

boxed_cast_base! { ConcatLayer, crate::dnn::Layer, cv_dnn_ConcatLayer_to_Layer }

impl std::fmt::Debug for ConcatLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ConcatLayer")
			.field("axis", &crate::dnn::ConcatLayerTraitConst::axis(self))
			.field("padding", &crate::dnn::ConcatLayerTraitConst::padding(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ConcatLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ConstLayer]
// ConstLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:83
pub trait ConstLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ConstLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::ConstLayer]
pub trait ConstLayerTrait: crate::dnn::ConstLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void;

}

/// Constant layer produces the same data blob at an every forward pass.
// ConstLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:83
pub struct ConstLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ConstLayer }

impl Drop for ConstLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ConstLayer_delete(self.as_raw_mut_ConstLayer()) };
	}
}

unsafe impl Send for ConstLayer {}

impl core::AlgorithmTraitConst for ConstLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ConstLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConstLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ConstLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ConstLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConstLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ConstLayerTraitConst for ConstLayer {
	#[inline] fn as_raw_ConstLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ConstLayerTrait for ConstLayer {
	#[inline] fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConstLayer, crate::dnn::ConstLayerTraitConst, as_raw_ConstLayer, crate::dnn::ConstLayerTrait, as_raw_mut_ConstLayer }

impl ConstLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ConstLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:86
	// ("cv::dnn::ConstLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ConstLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ConstLayer, core::Algorithm, cv_dnn_ConstLayer_to_Algorithm }

boxed_cast_base! { ConstLayer, crate::dnn::Layer, cv_dnn_ConstLayer_to_Layer }

impl std::fmt::Debug for ConstLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ConstLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ConstLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ConvolutionLayer]
// ConvolutionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:221
pub trait ConvolutionLayerTraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
	fn as_raw_ConvolutionLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::ConvolutionLayer]
pub trait ConvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::ConvolutionLayerTraitConst {
	fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void;

}

// ConvolutionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:221
pub struct ConvolutionLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ConvolutionLayer }

impl Drop for ConvolutionLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ConvolutionLayer_delete(self.as_raw_mut_ConvolutionLayer()) };
	}
}

unsafe impl Send for ConvolutionLayer {}

impl core::AlgorithmTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConvolutionLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::BaseConvolutionLayerTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConvolutionLayer, crate::dnn::BaseConvolutionLayerTraitConst, as_raw_BaseConvolutionLayer, crate::dnn::BaseConvolutionLayerTrait, as_raw_mut_BaseConvolutionLayer }

impl crate::dnn::LayerTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConvolutionLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ConvolutionLayerTraitConst for ConvolutionLayer {
	#[inline] fn as_raw_ConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ConvolutionLayerTrait for ConvolutionLayer {
	#[inline] fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ConvolutionLayer, crate::dnn::ConvolutionLayerTraitConst, as_raw_ConvolutionLayer, crate::dnn::ConvolutionLayerTrait, as_raw_mut_ConvolutionLayer }

impl ConvolutionLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ConvolutionLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:224
	// ("cv::dnn::ConvolutionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ConvolutionLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BaseConvolutionLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ConvolutionLayer, core::Algorithm, cv_dnn_ConvolutionLayer_to_Algorithm }

boxed_cast_base! { ConvolutionLayer, crate::dnn::BaseConvolutionLayer, cv_dnn_ConvolutionLayer_to_BaseConvolutionLayer }

boxed_cast_base! { ConvolutionLayer, crate::dnn::Layer, cv_dnn_ConvolutionLayer_to_Layer }

impl std::fmt::Debug for ConvolutionLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ConvolutionLayer")
			.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
			.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
			.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
			.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
			.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
			.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
			.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
			.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
			.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
			.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
			.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
			.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
			.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ConvolutionLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::CorrelationLayer]
// CorrelationLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:574
pub trait CorrelationLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CorrelationLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::CorrelationLayer]
pub trait CorrelationLayerTrait: crate::dnn::CorrelationLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void;

}

// CorrelationLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:574
pub struct CorrelationLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { CorrelationLayer }

impl Drop for CorrelationLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_CorrelationLayer_delete(self.as_raw_mut_CorrelationLayer()) };
	}
}

unsafe impl Send for CorrelationLayer {}

impl core::AlgorithmTraitConst for CorrelationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CorrelationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CorrelationLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for CorrelationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CorrelationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CorrelationLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::CorrelationLayerTraitConst for CorrelationLayer {
	#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CorrelationLayerTrait for CorrelationLayer {
	#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CorrelationLayer, crate::dnn::CorrelationLayerTraitConst, as_raw_CorrelationLayer, crate::dnn::CorrelationLayerTrait, as_raw_mut_CorrelationLayer }

impl CorrelationLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_CorrelationLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:577
	// ("cv::dnn::CorrelationLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::CorrelationLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CorrelationLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::CorrelationLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CorrelationLayer, core::Algorithm, cv_dnn_CorrelationLayer_to_Algorithm }

boxed_cast_base! { CorrelationLayer, crate::dnn::Layer, cv_dnn_CorrelationLayer_to_Layer }

impl std::fmt::Debug for CorrelationLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CorrelationLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for CorrelationLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::CropAndResizeLayer]
// CropAndResizeLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:686
pub trait CropAndResizeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CropAndResizeLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::CropAndResizeLayer]
pub trait CropAndResizeLayerTrait: crate::dnn::CropAndResizeLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void;

}

// CropAndResizeLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:686
pub struct CropAndResizeLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { CropAndResizeLayer }

impl Drop for CropAndResizeLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_CropAndResizeLayer_delete(self.as_raw_mut_CropAndResizeLayer()) };
	}
}

unsafe impl Send for CropAndResizeLayer {}

impl core::AlgorithmTraitConst for CropAndResizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CropAndResizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CropAndResizeLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for CropAndResizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CropAndResizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CropAndResizeLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::CropAndResizeLayerTraitConst for CropAndResizeLayer {
	#[inline] fn as_raw_CropAndResizeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CropAndResizeLayerTrait for CropAndResizeLayer {
	#[inline] fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CropAndResizeLayer, crate::dnn::CropAndResizeLayerTraitConst, as_raw_CropAndResizeLayer, crate::dnn::CropAndResizeLayerTrait, as_raw_mut_CropAndResizeLayer }

impl CropAndResizeLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_CropAndResizeLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:689
	// ("cv::dnn::CropAndResizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CropAndResizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CropAndResizeLayer, core::Algorithm, cv_dnn_CropAndResizeLayer_to_Algorithm }

boxed_cast_base! { CropAndResizeLayer, crate::dnn::Layer, cv_dnn_CropAndResizeLayer_to_Layer }

impl std::fmt::Debug for CropAndResizeLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CropAndResizeLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for CropAndResizeLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::CropLayer]
// CropLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:515
pub trait CropLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_CropLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::CropLayer]
pub trait CropLayerTrait: crate::dnn::CropLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_CropLayer(&mut self) -> *mut c_void;

}

// CropLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:515
pub struct CropLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { CropLayer }

impl Drop for CropLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_CropLayer_delete(self.as_raw_mut_CropLayer()) };
	}
}

unsafe impl Send for CropLayer {}

impl core::AlgorithmTraitConst for CropLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CropLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CropLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for CropLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for CropLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CropLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::CropLayerTraitConst for CropLayer {
	#[inline] fn as_raw_CropLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::CropLayerTrait for CropLayer {
	#[inline] fn as_raw_mut_CropLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CropLayer, crate::dnn::CropLayerTraitConst, as_raw_CropLayer, crate::dnn::CropLayerTrait, as_raw_mut_CropLayer }

impl CropLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_CropLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:518
	// ("cv::dnn::CropLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_CropLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { CropLayer, core::Algorithm, cv_dnn_CropLayer_to_Algorithm }

boxed_cast_base! { CropLayer, crate::dnn::Layer, cv_dnn_CropLayer_to_Layer }

impl std::fmt::Debug for CropLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CropLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for CropLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::DataAugmentationLayer]
// DataAugmentationLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:568
pub trait DataAugmentationLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_DataAugmentationLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::DataAugmentationLayer]
pub trait DataAugmentationLayerTrait: crate::dnn::DataAugmentationLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void;

}

// DataAugmentationLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:568
pub struct DataAugmentationLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { DataAugmentationLayer }

impl Drop for DataAugmentationLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_DataAugmentationLayer_delete(self.as_raw_mut_DataAugmentationLayer()) };
	}
}

unsafe impl Send for DataAugmentationLayer {}

impl core::AlgorithmTraitConst for DataAugmentationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DataAugmentationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DataAugmentationLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for DataAugmentationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for DataAugmentationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DataAugmentationLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::DataAugmentationLayerTraitConst for DataAugmentationLayer {
	#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DataAugmentationLayerTrait for DataAugmentationLayer {
	#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DataAugmentationLayer, crate::dnn::DataAugmentationLayerTraitConst, as_raw_DataAugmentationLayer, crate::dnn::DataAugmentationLayerTrait, as_raw_mut_DataAugmentationLayer }

impl DataAugmentationLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_DataAugmentationLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:571
	// ("cv::dnn::DataAugmentationLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::DataAugmentationLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DataAugmentationLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::DataAugmentationLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DataAugmentationLayer, core::Algorithm, cv_dnn_DataAugmentationLayer_to_Algorithm }

boxed_cast_base! { DataAugmentationLayer, crate::dnn::Layer, cv_dnn_DataAugmentationLayer_to_Layer }

impl std::fmt::Debug for DataAugmentationLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DataAugmentationLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for DataAugmentationLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::DeconvolutionLayer]
// DeconvolutionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:227
pub trait DeconvolutionLayerTraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
	fn as_raw_DeconvolutionLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::DeconvolutionLayer]
pub trait DeconvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::DeconvolutionLayerTraitConst {
	fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void;

}

// DeconvolutionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:227
pub struct DeconvolutionLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { DeconvolutionLayer }

impl Drop for DeconvolutionLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_DeconvolutionLayer_delete(self.as_raw_mut_DeconvolutionLayer()) };
	}
}

unsafe impl Send for DeconvolutionLayer {}

impl core::AlgorithmTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DeconvolutionLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::BaseConvolutionLayerTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DeconvolutionLayer, crate::dnn::BaseConvolutionLayerTraitConst, as_raw_BaseConvolutionLayer, crate::dnn::BaseConvolutionLayerTrait, as_raw_mut_BaseConvolutionLayer }

impl crate::dnn::LayerTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DeconvolutionLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::DeconvolutionLayerTraitConst for DeconvolutionLayer {
	#[inline] fn as_raw_DeconvolutionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DeconvolutionLayerTrait for DeconvolutionLayer {
	#[inline] fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DeconvolutionLayer, crate::dnn::DeconvolutionLayerTraitConst, as_raw_DeconvolutionLayer, crate::dnn::DeconvolutionLayerTrait, as_raw_mut_DeconvolutionLayer }

impl DeconvolutionLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_DeconvolutionLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:230
	// ("cv::dnn::DeconvolutionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DeconvolutionLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BaseConvolutionLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DeconvolutionLayer, core::Algorithm, cv_dnn_DeconvolutionLayer_to_Algorithm }

boxed_cast_base! { DeconvolutionLayer, crate::dnn::BaseConvolutionLayer, cv_dnn_DeconvolutionLayer_to_BaseConvolutionLayer }

boxed_cast_base! { DeconvolutionLayer, crate::dnn::Layer, cv_dnn_DeconvolutionLayer_to_Layer }

impl std::fmt::Debug for DeconvolutionLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DeconvolutionLayer")
			.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
			.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
			.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
			.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
			.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
			.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
			.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
			.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
			.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
			.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
			.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
			.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
			.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for DeconvolutionLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::DetectionOutputLayer]
// DetectionOutputLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:618
pub trait DetectionOutputLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_DetectionOutputLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::DetectionOutputLayer]
pub trait DetectionOutputLayerTrait: crate::dnn::DetectionOutputLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void;

}

/// Detection output layer.
///
/// The layer size is: @f$ (1 \times 1 \times N \times 7) @f$
///    where N is [keep_top_k] parameter multiplied by batch size. Each row is:
///    [image_id, label, confidence, xmin, ymin, xmax, ymax]
///    where image_id is the index of image input in the batch.
// DetectionOutputLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:618
pub struct DetectionOutputLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { DetectionOutputLayer }

impl Drop for DetectionOutputLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_DetectionOutputLayer_delete(self.as_raw_mut_DetectionOutputLayer()) };
	}
}

unsafe impl Send for DetectionOutputLayer {}

impl core::AlgorithmTraitConst for DetectionOutputLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for DetectionOutputLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DetectionOutputLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for DetectionOutputLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for DetectionOutputLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DetectionOutputLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::DetectionOutputLayerTraitConst for DetectionOutputLayer {
	#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DetectionOutputLayerTrait for DetectionOutputLayer {
	#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DetectionOutputLayer, crate::dnn::DetectionOutputLayerTraitConst, as_raw_DetectionOutputLayer, crate::dnn::DetectionOutputLayerTrait, as_raw_mut_DetectionOutputLayer }

impl DetectionOutputLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_DetectionOutputLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:621
	// ("cv::dnn::DetectionOutputLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::DetectionOutputLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DetectionOutputLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::DetectionOutputLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { DetectionOutputLayer, core::Algorithm, cv_dnn_DetectionOutputLayer_to_Algorithm }

boxed_cast_base! { DetectionOutputLayer, crate::dnn::Layer, cv_dnn_DetectionOutputLayer_to_Layer }

impl std::fmt::Debug for DetectionOutputLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("DetectionOutputLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for DetectionOutputLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::Dict]
// Dict /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:114
pub trait DictTraitConst {
	fn as_raw_Dict(&self) -> *const c_void;

	/// Checks a presence of the @p key in the dictionary.
	// has(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:122
	// ("cv::dnn::Dict::has", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	fn has(&self, key: &str) -> Result<bool> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_has_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
	///
	/// ## Overloaded parameters
	// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:128
	// ("cv::dnn::Dict::ptr", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	unsafe fn ptr(&self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		{ sys::cv_dnn_Dict_ptr_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// If the @p key in the dictionary then returns its value, else an error will be generated.
	// get(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:131
	// ("cv::dnn::Dict::get", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	fn get(&self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_get_const_const_StringR(self.as_raw_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::dnn::Dict]
pub trait DictTrait: crate::dnn::DictTraitConst {
	fn as_raw_mut_Dict(&mut self) -> *mut c_void;

	/// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
	// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:125
	// ("cv::dnn::Dict::ptr", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	unsafe fn ptr_mut(&mut self, key: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		{ sys::cv_dnn_Dict_ptr_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	// cv::dnn::Dict::set(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn set_str(&mut self, key: &str, value: &str) -> Result<String> {
		extern_container_arg!(key);
		extern_container_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_const_cv_String_const_StringR_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	// cv::dnn::Dict::set(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::dnn::DictValue*"]), _)]),
	#[inline]
	fn set(&mut self, key: &str, value: &impl crate::dnn::DictValueTraitConst) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_const_cv_dnn_DictValue_const_StringR_const_DictValueR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const double*"]), _)]),
	#[inline]
	fn set_f64(&mut self, key: &str, value: &f64) -> Result<f64> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_const_double_const_StringR_const_doubleR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
	// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const int64_t*"]), _)]),
	#[inline]
	fn set_i64(&mut self, key: &str, value: &i64) -> Result<i64> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_set_const_int64_t_const_StringR_const_int64_tR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Erase @p key from the dictionary.
	// erase(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:146
	// ("cv::dnn::Dict::erase", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
	#[inline]
	fn erase(&mut self, key: &str) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Dict_erase_const_StringR(self.as_raw_mut_Dict(), key.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// This class implements name-value dictionary, values are instances of DictValue.
// Dict /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:114
pub struct Dict {
	ptr: *mut c_void,
}

opencv_type_boxed! { Dict }

impl Drop for Dict {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_Dict_delete(self.as_raw_mut_Dict()) };
	}
}

unsafe impl Send for Dict {}

impl crate::dnn::DictTraitConst for Dict {
	#[inline] fn as_raw_Dict(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DictTrait for Dict {
	#[inline] fn as_raw_mut_Dict(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Dict, crate::dnn::DictTraitConst, as_raw_Dict, crate::dnn::DictTrait, as_raw_mut_Dict }

impl Dict {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_Dict_defaultNew_const()) }
	}

}

impl std::fmt::Debug for Dict {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Dict")
			.finish()
	}
}

impl Default for Dict {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::DictValue]
// DictValue /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:60
pub trait DictValueTraitConst {
	fn as_raw_DictValue(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * idx: -1
	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_str(&self, idx: i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_cv_String_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DictValueTraitConst::get_str] function uses the following default values for its arguments:
	/// * idx: -1
	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_str_def(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_cv_String_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * idx: -1
	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_f64(&self, idx: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_double_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DictValueTraitConst::get_f64] function uses the following default values for its arguments:
	/// * idx: -1
	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_f64_def(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_double_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * idx: -1
	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_i32(&self, idx: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_int_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DictValueTraitConst::get_i32] function uses the following default values for its arguments:
	/// * idx: -1
	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_i32_def(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_int_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * idx: -1
	// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_i64(&self, idx: i32) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_int64_t_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DictValueTraitConst::get_i64] function uses the following default values for its arguments:
	/// * idx: -1
	// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
	// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_i64_def(&self) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_get_int64_t_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:81
	// ("cv::dnn::DictValue::size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_size_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isInt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:83
	// ("cv::dnn::DictValue::isInt", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_int(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_isInt_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isString()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:84
	// ("cv::dnn::DictValue::isString", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_string(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_isString_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// isReal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:85
	// ("cv::dnn::DictValue::isReal", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_real(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_isReal_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * idx: -1
	// getIntValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:87
	// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_int_value(&self, idx: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getIntValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DictValueTraitConst::get_int_value] function uses the following default values for its arguments:
	/// * idx: -1
	// cv::dnn::DictValue::getIntValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:87
	// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_int_value_def(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getIntValue_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * idx: -1
	// getRealValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:88
	// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_real_value(&self, idx: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getRealValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DictValueTraitConst::get_real_value] function uses the following default values for its arguments:
	/// * idx: -1
	// cv::dnn::DictValue::getRealValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:88
	// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_real_value_def(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getRealValue_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * idx: -1
	// getStringValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:89
	// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_string_value(&self, idx: i32) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getStringValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [DictValueTraitConst::get_string_value] function uses the following default values for its arguments:
	/// * idx: -1
	// cv::dnn::DictValue::getStringValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:89
	// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_string_value_def(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_getStringValue_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::dnn::DictValue]
pub trait DictValueTrait: crate::dnn::DictValueTraitConst {
	fn as_raw_mut_DictValue(&mut self) -> *mut c_void;

	// operator=(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:91
	// ("cv::dnn::DictValue::operator=", vec![(pred!(mut, ["r"], ["const cv::dnn::DictValue*"]), _)]),
	#[inline]
	fn set(&mut self, r: &impl crate::dnn::DictValueTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_operatorST_const_DictValueR(self.as_raw_mut_DictValue(), r.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
/// @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
// DictValue /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:60
pub struct DictValue {
	ptr: *mut c_void,
}

opencv_type_boxed! { DictValue }

impl Drop for DictValue {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_DictValue_delete(self.as_raw_mut_DictValue()) };
	}
}

unsafe impl Send for DictValue {}

impl crate::dnn::DictValueTraitConst for DictValue {
	#[inline] fn as_raw_DictValue(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DictValueTrait for DictValue {
	#[inline] fn as_raw_mut_DictValue(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { DictValue, crate::dnn::DictValueTraitConst, as_raw_DictValue, crate::dnn::DictValueTrait, as_raw_mut_DictValue }

impl DictValue {
	// DictValue(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:62
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["r"], ["const cv::dnn::DictValue*"]), _)]),
	#[inline]
	pub fn copy(r: &impl crate::dnn::DictValueTraitConst) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_const_DictValueR(r.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	// DictValue(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:63
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["bool"]), _)]),
	#[inline]
	pub fn from_bool(i: bool) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_bool(i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * i: 0
	// DictValue(int64)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:64
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int64_t"]), _)]),
	#[inline]
	pub fn from_i64(i: i64) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_int64_t(i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [from_i64] function uses the following default values for its arguments:
	/// * i: 0
	// cv::dnn::DictValue::DictValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:64
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn from_i64_def() -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	// DictValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:65
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int"]), _)]),
	#[inline]
	pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_int(i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	// DictValue(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:66
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["unsigned int"]), _)]),
	#[inline]
	pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_unsigned_int(p, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	// DictValue(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:67
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["double"]), _)]),
	#[inline]
	pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_double(p, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

	// DictValue(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:69
	// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["s"], ["const char*"]), _)]),
	#[inline]
	pub fn from_str(s: &str) -> Result<crate::dnn::DictValue> {
		extern_container_arg!(s);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_DictValue_DictValue_const_charX(s.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Constant methods for [crate::dnn::ELULayer]
// ELULayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:453
pub trait ELULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ELULayer(&self) -> *const c_void;

	// cv::dnn::ELULayer::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:456
	// ("cv::dnn::ELULayer::alpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ELULayer_propAlpha_const(self.as_raw_ELULayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ELULayer]
pub trait ELULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ELULayerTraitConst {
	fn as_raw_mut_ELULayer(&mut self) -> *mut c_void;

	// cv::dnn::ELULayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:456
	// ("cv::dnn::ELULayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ELULayer_propAlpha_const_float(self.as_raw_mut_ELULayer(), val) };
		ret
	}

}

// ELULayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:453
pub struct ELULayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ELULayer }

impl Drop for ELULayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ELULayer_delete(self.as_raw_mut_ELULayer()) };
	}
}

unsafe impl Send for ELULayer {}

impl crate::dnn::ActivationLayerTraitConst for ELULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ELULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ELULayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for ELULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ELULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ELULayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ELULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ELULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ELULayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ELULayerTraitConst for ELULayer {
	#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ELULayerTrait for ELULayer {
	#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ELULayer, crate::dnn::ELULayerTraitConst, as_raw_ELULayer, crate::dnn::ELULayerTrait, as_raw_mut_ELULayer }

impl ELULayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:458
	// ("cv::dnn::ELULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ELULayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ELULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ELULayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ELULayer, crate::dnn::ActivationLayer, cv_dnn_ELULayer_to_ActivationLayer }

boxed_cast_base! { ELULayer, core::Algorithm, cv_dnn_ELULayer_to_Algorithm }

boxed_cast_base! { ELULayer, crate::dnn::Layer, cv_dnn_ELULayer_to_Layer }

impl std::fmt::Debug for ELULayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ELULayer")
			.field("alpha", &crate::dnn::ELULayerTraitConst::alpha(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::EltwiseLayer]
// EltwiseLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:528
pub trait EltwiseLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_EltwiseLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::EltwiseLayer]
pub trait EltwiseLayerTrait: crate::dnn::EltwiseLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void;

}

/// Element wise operation on inputs
///
/// Extra optional parameters:
/// - "operation" as string. Values are "sum" (default), "prod", "max", "div"
/// - "coeff" as float array. Specify weights of inputs for SUM operation
/// - "output_channels_mode" as string. Values are "same" (default, all input must have the same layout), "input_0", "input_0_truncate", "max_input_channels"
// EltwiseLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:528
pub struct EltwiseLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { EltwiseLayer }

impl Drop for EltwiseLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_EltwiseLayer_delete(self.as_raw_mut_EltwiseLayer()) };
	}
}

unsafe impl Send for EltwiseLayer {}

impl core::AlgorithmTraitConst for EltwiseLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for EltwiseLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { EltwiseLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for EltwiseLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for EltwiseLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { EltwiseLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::EltwiseLayerTraitConst for EltwiseLayer {
	#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::EltwiseLayerTrait for EltwiseLayer {
	#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { EltwiseLayer, crate::dnn::EltwiseLayerTraitConst, as_raw_EltwiseLayer, crate::dnn::EltwiseLayerTrait, as_raw_mut_EltwiseLayer }

impl EltwiseLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_EltwiseLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:531
	// ("cv::dnn::EltwiseLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::EltwiseLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_EltwiseLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::EltwiseLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { EltwiseLayer, core::Algorithm, cv_dnn_EltwiseLayer_to_Algorithm }

boxed_cast_base! { EltwiseLayer, crate::dnn::Layer, cv_dnn_EltwiseLayer_to_Layer }

impl std::fmt::Debug for EltwiseLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("EltwiseLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for EltwiseLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ExpLayer]
// ExpLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:505
pub trait ExpLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ExpLayer(&self) -> *const c_void;

	// cv::dnn::ExpLayer::base() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::base", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn base(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ExpLayer_propBase_const(self.as_raw_ExpLayer()) };
		ret
	}

	// cv::dnn::ExpLayer::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::scale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ExpLayer_propScale_const(self.as_raw_ExpLayer()) };
		ret
	}

	// cv::dnn::ExpLayer::shift() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::shift", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn shift(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ExpLayer_propShift_const(self.as_raw_ExpLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ExpLayer]
pub trait ExpLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ExpLayerTraitConst {
	fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void;

	// cv::dnn::ExpLayer::setBase(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::setBase", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_base(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ExpLayer_propBase_const_float(self.as_raw_mut_ExpLayer(), val) };
		ret
	}

	// cv::dnn::ExpLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ExpLayer_propScale_const_float(self.as_raw_mut_ExpLayer(), val) };
		ret
	}

	// cv::dnn::ExpLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
	// ("cv::dnn::ExpLayer::setShift", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_shift(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ExpLayer_propShift_const_float(self.as_raw_mut_ExpLayer(), val) };
		ret
	}

}

// ExpLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:505
pub struct ExpLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ExpLayer }

impl Drop for ExpLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ExpLayer_delete(self.as_raw_mut_ExpLayer()) };
	}
}

unsafe impl Send for ExpLayer {}

impl crate::dnn::ActivationLayerTraitConst for ExpLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ExpLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ExpLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for ExpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ExpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ExpLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ExpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ExpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ExpLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ExpLayerTraitConst for ExpLayer {
	#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ExpLayerTrait for ExpLayer {
	#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ExpLayer, crate::dnn::ExpLayerTraitConst, as_raw_ExpLayer, crate::dnn::ExpLayerTrait, as_raw_mut_ExpLayer }

impl ExpLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:510
	// ("cv::dnn::ExpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ExpLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ExpLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ExpLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ExpLayer, crate::dnn::ActivationLayer, cv_dnn_ExpLayer_to_ActivationLayer }

boxed_cast_base! { ExpLayer, core::Algorithm, cv_dnn_ExpLayer_to_Algorithm }

boxed_cast_base! { ExpLayer, crate::dnn::Layer, cv_dnn_ExpLayer_to_Layer }

impl std::fmt::Debug for ExpLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ExpLayer")
			.field("base", &crate::dnn::ExpLayerTraitConst::base(self))
			.field("scale", &crate::dnn::ExpLayerTraitConst::scale(self))
			.field("shift", &crate::dnn::ExpLayerTraitConst::shift(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::FlattenLayer]
// FlattenLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:304
pub trait FlattenLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_FlattenLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::FlattenLayer]
pub trait FlattenLayerTrait: crate::dnn::FlattenLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void;

}

// FlattenLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:304
pub struct FlattenLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { FlattenLayer }

impl Drop for FlattenLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_FlattenLayer_delete(self.as_raw_mut_FlattenLayer()) };
	}
}

unsafe impl Send for FlattenLayer {}

impl core::AlgorithmTraitConst for FlattenLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FlattenLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlattenLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for FlattenLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for FlattenLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlattenLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::FlattenLayerTraitConst for FlattenLayer {
	#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::FlattenLayerTrait for FlattenLayer {
	#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlattenLayer, crate::dnn::FlattenLayerTraitConst, as_raw_FlattenLayer, crate::dnn::FlattenLayerTrait, as_raw_mut_FlattenLayer }

impl FlattenLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_FlattenLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:307
	// ("cv::dnn::FlattenLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::FlattenLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_FlattenLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::FlattenLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FlattenLayer, core::Algorithm, cv_dnn_FlattenLayer_to_Algorithm }

boxed_cast_base! { FlattenLayer, crate::dnn::Layer, cv_dnn_FlattenLayer_to_Layer }

impl std::fmt::Debug for FlattenLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FlattenLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for FlattenLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::FlowWarpLayer]
// FlowWarpLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:586
pub trait FlowWarpLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_FlowWarpLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::FlowWarpLayer]
pub trait FlowWarpLayerTrait: crate::dnn::FlowWarpLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void;

}

// FlowWarpLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:586
pub struct FlowWarpLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { FlowWarpLayer }

impl Drop for FlowWarpLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_FlowWarpLayer_delete(self.as_raw_mut_FlowWarpLayer()) };
	}
}

unsafe impl Send for FlowWarpLayer {}

impl core::AlgorithmTraitConst for FlowWarpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FlowWarpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlowWarpLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for FlowWarpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for FlowWarpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlowWarpLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::FlowWarpLayerTraitConst for FlowWarpLayer {
	#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::FlowWarpLayerTrait for FlowWarpLayer {
	#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { FlowWarpLayer, crate::dnn::FlowWarpLayerTraitConst, as_raw_FlowWarpLayer, crate::dnn::FlowWarpLayerTrait, as_raw_mut_FlowWarpLayer }

impl FlowWarpLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_FlowWarpLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:589
	// ("cv::dnn::FlowWarpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::FlowWarpLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_FlowWarpLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::FlowWarpLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { FlowWarpLayer, core::Algorithm, cv_dnn_FlowWarpLayer_to_Algorithm }

boxed_cast_base! { FlowWarpLayer, crate::dnn::Layer, cv_dnn_FlowWarpLayer_to_Layer }

impl std::fmt::Debug for FlowWarpLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("FlowWarpLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for FlowWarpLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::InnerProductLayer]
// InnerProductLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:277
pub trait InnerProductLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_InnerProductLayer(&self) -> *const c_void;

	// cv::dnn::InnerProductLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::InnerProductLayer::axis", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_InnerProductLayer_propAxis_const(self.as_raw_InnerProductLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::InnerProductLayer]
pub trait InnerProductLayerTrait: crate::dnn::InnerProductLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void;

	// cv::dnn::InnerProductLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:280
	// ("cv::dnn::InnerProductLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_InnerProductLayer_propAxis_const_int(self.as_raw_mut_InnerProductLayer(), val) };
		ret
	}

}

// InnerProductLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:277
pub struct InnerProductLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { InnerProductLayer }

impl Drop for InnerProductLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_InnerProductLayer_delete(self.as_raw_mut_InnerProductLayer()) };
	}
}

unsafe impl Send for InnerProductLayer {}

impl core::AlgorithmTraitConst for InnerProductLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for InnerProductLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InnerProductLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for InnerProductLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for InnerProductLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InnerProductLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::InnerProductLayerTraitConst for InnerProductLayer {
	#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::InnerProductLayerTrait for InnerProductLayer {
	#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InnerProductLayer, crate::dnn::InnerProductLayerTraitConst, as_raw_InnerProductLayer, crate::dnn::InnerProductLayerTrait, as_raw_mut_InnerProductLayer }

impl InnerProductLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_InnerProductLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:281
	// ("cv::dnn::InnerProductLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::InnerProductLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_InnerProductLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::InnerProductLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { InnerProductLayer, core::Algorithm, cv_dnn_InnerProductLayer_to_Algorithm }

boxed_cast_base! { InnerProductLayer, crate::dnn::Layer, cv_dnn_InnerProductLayer_to_Layer }

impl std::fmt::Debug for InnerProductLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("InnerProductLayer")
			.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for InnerProductLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::InterpLayer]
// InterpLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:674
pub trait InterpLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_InterpLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::InterpLayer]
pub trait InterpLayerTrait: crate::dnn::InterpLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_InterpLayer(&mut self) -> *mut c_void;

}

/// Bilinear resize layer from <https://github.com/cdmh/deeplab-public-ver2>
///
/// It differs from [ResizeLayer] in output shape and resize scales computations.
// InterpLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:674
pub struct InterpLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { InterpLayer }

impl Drop for InterpLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_InterpLayer_delete(self.as_raw_mut_InterpLayer()) };
	}
}

unsafe impl Send for InterpLayer {}

impl core::AlgorithmTraitConst for InterpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for InterpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InterpLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for InterpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for InterpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InterpLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::InterpLayerTraitConst for InterpLayer {
	#[inline] fn as_raw_InterpLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::InterpLayerTrait for InterpLayer {
	#[inline] fn as_raw_mut_InterpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { InterpLayer, crate::dnn::InterpLayerTraitConst, as_raw_InterpLayer, crate::dnn::InterpLayerTrait, as_raw_mut_InterpLayer }

impl InterpLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_InterpLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:677
	// ("cv::dnn::InterpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_InterpLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { InterpLayer, core::Algorithm, cv_dnn_InterpLayer_to_Algorithm }

boxed_cast_base! { InterpLayer, crate::dnn::Layer, cv_dnn_InterpLayer_to_Layer }

impl std::fmt::Debug for InterpLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("InterpLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for InterpLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::LRNLayer]
// LRNLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:233
pub trait LRNLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_LRNLayer(&self) -> *const c_void;

	// cv::dnn::LRNLayer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:236
	// ("cv::dnn::LRNLayer::type", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn typ(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propType_const(self.as_raw_LRNLayer()) };
		ret
	}

	// cv::dnn::LRNLayer::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:238
	// ("cv::dnn::LRNLayer::size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn size(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propSize_const(self.as_raw_LRNLayer()) };
		ret
	}

	// cv::dnn::LRNLayer::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::alpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propAlpha_const(self.as_raw_LRNLayer()) };
		ret
	}

	// cv::dnn::LRNLayer::beta() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::beta", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn beta(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propBeta_const(self.as_raw_LRNLayer()) };
		ret
	}

	// cv::dnn::LRNLayer::bias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::bias", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bias(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propBias_const(self.as_raw_LRNLayer()) };
		ret
	}

	// cv::dnn::LRNLayer::normBySize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:240
	// ("cv::dnn::LRNLayer::normBySize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn norm_by_size(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propNormBySize_const(self.as_raw_LRNLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::LRNLayer]
pub trait LRNLayerTrait: crate::dnn::LRNLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void;

	// cv::dnn::LRNLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:236
	// ("cv::dnn::LRNLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propType_const_int(self.as_raw_mut_LRNLayer(), val) };
		ret
	}

	// cv::dnn::LRNLayer::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:238
	// ("cv::dnn::LRNLayer::setSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propSize_const_int(self.as_raw_mut_LRNLayer(), val) };
		ret
	}

	// cv::dnn::LRNLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propAlpha_const_float(self.as_raw_mut_LRNLayer(), val) };
		ret
	}

	// cv::dnn::LRNLayer::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::setBeta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_beta(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propBeta_const_float(self.as_raw_mut_LRNLayer(), val) };
		ret
	}

	// cv::dnn::LRNLayer::setBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
	// ("cv::dnn::LRNLayer::setBias", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_bias(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propBias_const_float(self.as_raw_mut_LRNLayer(), val) };
		ret
	}

	// cv::dnn::LRNLayer::setNormBySize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:240
	// ("cv::dnn::LRNLayer::setNormBySize", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_norm_by_size(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_LRNLayer_propNormBySize_const_bool(self.as_raw_mut_LRNLayer(), val) };
		ret
	}

}

// LRNLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:233
pub struct LRNLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { LRNLayer }

impl Drop for LRNLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_LRNLayer_delete(self.as_raw_mut_LRNLayer()) };
	}
}

unsafe impl Send for LRNLayer {}

impl core::AlgorithmTraitConst for LRNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LRNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LRNLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for LRNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for LRNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LRNLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::LRNLayerTraitConst for LRNLayer {
	#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LRNLayerTrait for LRNLayer {
	#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LRNLayer, crate::dnn::LRNLayerTraitConst, as_raw_LRNLayer, crate::dnn::LRNLayerTrait, as_raw_mut_LRNLayer }

impl LRNLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_LRNLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:242
	// ("cv::dnn::LRNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::LRNLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LRNLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::LRNLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LRNLayer, core::Algorithm, cv_dnn_LRNLayer_to_Algorithm }

boxed_cast_base! { LRNLayer, crate::dnn::Layer, cv_dnn_LRNLayer_to_Layer }

impl std::fmt::Debug for LRNLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LRNLayer")
			.field("typ", &crate::dnn::LRNLayerTraitConst::typ(self))
			.field("size", &crate::dnn::LRNLayerTraitConst::size(self))
			.field("alpha", &crate::dnn::LRNLayerTraitConst::alpha(self))
			.field("beta", &crate::dnn::LRNLayerTraitConst::beta(self))
			.field("bias", &crate::dnn::LRNLayerTraitConst::bias(self))
			.field("norm_by_size", &crate::dnn::LRNLayerTraitConst::norm_by_size(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for LRNLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::LSTMLayer]
// LSTMLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:90
pub trait LSTMLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_LSTMLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::LSTMLayer]
pub trait LSTMLayerTrait: crate::dnn::LSTMLayerTraitConst + crate::dnn::LayerTrait {
	fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void;

	///
	/// **Deprecated**: Use LayerParams::blobs instead.
	/// Set trained weights for LSTM layer.
	///
	/// LSTM behavior on each step is defined by current input, previous output, previous cell state and learned weights.
	///
	/// Let @f$x_t@f$ be current input, @f$h_t@f$ be current output, @f$c_t@f$ be current state.
	/// Than current output and current cell state is computed as follows:
	/// @f{eqnarray*}{
	/// h_t &= o_t \odot tanh(c_t),               \\
	/// c_t &= f_t \odot c_{t-1} + i_t \odot g_t, \\
	/// @f}
	/// where @f$\odot@f$ is per-element multiply operation and @f$i_t, f_t, o_t, g_t@f$ is internal gates that are computed using learned weights.
	///
	/// Gates are computed as follows:
	/// @f{eqnarray*}{
	/// i_t &= sigmoid&(W_{xi} x_t + W_{hi} h_{t-1} + b_i), \\
	/// f_t &= sigmoid&(W_{xf} x_t + W_{hf} h_{t-1} + b_f), \\
	/// o_t &= sigmoid&(W_{xo} x_t + W_{ho} h_{t-1} + b_o), \\
	/// g_t &= tanh   &(W_{xg} x_t + W_{hg} h_{t-1} + b_g), \\
	/// @f}
	/// where @f$W_{x?}@f$, @f$W_{h?}@f$ and @f$b_{?}@f$ are learned weights represented as matrices:
	/// @f$W_{x?} \in R^{N_h \times N_x}@f$, @f$W_{h?} \in R^{N_h \times N_h}@f$, @f$b_? \in R^{N_h}@f$.
	///
	/// For simplicity and performance purposes we use @f$ W_x = [W_{xi}; W_{xf}; W_{xo}, W_{xg}] @f$
	/// (i.e. @f$W_x@f$ is vertical concatenation of @f$ W_{x?} @f$), @f$ W_x \in R^{4N_h \times N_x} @f$.
	/// The same for @f$ W_h = [W_{hi}; W_{hf}; W_{ho}, W_{hg}], W_h \in R^{4N_h \times N_h} @f$
	/// and for @f$ b = [b_i; b_f, b_o, b_g]@f$, @f$b \in R^{4N_h} @f$.
	///
	/// ## Parameters
	/// * Wh: is matrix defining how previous output is transformed to internal gates (i.e. according to above mentioned notation is @f$ W_h @f$)
	/// * Wx: is matrix defining how current input is transformed to internal gates (i.e. according to above mentioned notation is @f$ W_x @f$)
	/// * b: is bias vector (i.e. according to above mentioned notation is @f$ b @f$)
	#[deprecated = "Use LayerParams::blobs instead."]
	// setWeights(const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:128
	// ("cv::dnn::LSTMLayer::setWeights", vec![(pred!(mut, ["Wh", "Wx", "b"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn set_weights(&mut self, wh: &impl core::MatTraitConst, wx: &impl core::MatTraitConst, b: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setWeights_const_MatR_const_MatR_const_MatR(self.as_raw_mut_LSTMLayer(), wh.as_raw_Mat(), wx.as_raw_Mat(), b.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Specifies shape of output blob which will be [[`T`], `N`] + @p outTailShape.
	/// @details If this parameter is empty or unset then @p outTailShape = [`Wh`.size(0)] will be used,
	/// where `Wh` is parameter from setWeights().
	///
	/// ## C++ default parameters
	/// * out_tail_shape: MatShape()
	// setOutShape(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:134
	// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, ["outTailShape"], ["const cv::dnn::MatShape*"]), _)]),
	#[inline]
	fn set_out_shape(&mut self, out_tail_shape: &crate::dnn::MatShape) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setOutShape_const_MatShapeR(self.as_raw_mut_LSTMLayer(), out_tail_shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Specifies shape of output blob which will be [[`T`], `N`] + @p outTailShape.
	/// @details If this parameter is empty or unset then @p outTailShape = [`Wh`.size(0)] will be used,
	/// where `Wh` is parameter from setWeights().
	///
	/// ## Note
	/// This alternative version of [LSTMLayerTrait::set_out_shape] function uses the following default values for its arguments:
	/// * out_tail_shape: MatShape()
	// cv::dnn::LSTMLayer::setOutShape() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:134
	// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn set_out_shape_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setOutShape(self.as_raw_mut_LSTMLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	///
	/// **Deprecated**: Use flag `produce_cell_output` in LayerParams.
	/// Specifies either interpret first dimension of input blob as timestamp dimension either as sample.
	///
	/// If flag is set to true then shape of input blob will be interpreted as [`T`, `N`, `[data dims]`] where `T` specifies number of timestamps, `N` is number of independent streams.
	/// In this case each forward() call will iterate through `T` timestamps and update layer's state `T` times.
	///
	/// If flag is set to false then shape of input blob will be interpreted as [`N`, `[data dims]`].
	/// In this case each forward() call will make one iteration and produce one timestamp with shape [`N`, `[out dims]`].
	///
	/// ## C++ default parameters
	/// * use_: true
	#[deprecated = "Use flag `produce_cell_output` in LayerParams."]
	// setUseTimstampsDim(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:145
	// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, ["use"], ["bool"]), _)]),
	#[inline]
	fn set_use_timstamps_dim(&mut self, use_: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim_bool(self.as_raw_mut_LSTMLayer(), use_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	///
	/// **Deprecated**: Use flag `produce_cell_output` in LayerParams.
	/// Specifies either interpret first dimension of input blob as timestamp dimension either as sample.
	///
	/// If flag is set to true then shape of input blob will be interpreted as [`T`, `N`, `[data dims]`] where `T` specifies number of timestamps, `N` is number of independent streams.
	/// In this case each forward() call will iterate through `T` timestamps and update layer's state `T` times.
	///
	/// If flag is set to false then shape of input blob will be interpreted as [`N`, `[data dims]`].
	/// In this case each forward() call will make one iteration and produce one timestamp with shape [`N`, `[out dims]`].
	///
	/// ## Note
	/// This alternative version of [LSTMLayerTrait::set_use_timstamps_dim] function uses the following default values for its arguments:
	/// * use_: true
	#[deprecated = "Use flag `produce_cell_output` in LayerParams."]
	// cv::dnn::LSTMLayer::setUseTimstampsDim() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:145
	// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn set_use_timstamps_dim_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim(self.as_raw_mut_LSTMLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	///
	/// **Deprecated**: Use flag `use_timestamp_dim` in LayerParams.
	/// If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	///
	/// ## C++ default parameters
	/// * produce: false
	#[deprecated = "Use flag `use_timestamp_dim` in LayerParams."]
	// setProduceCellOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:151
	// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
	#[inline]
	fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput_bool(self.as_raw_mut_LSTMLayer(), produce, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	///
	/// **Deprecated**: Use flag `use_timestamp_dim` in LayerParams.
	/// If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	///
	/// ## Note
	/// This alternative version of [LSTMLayerTrait::set_produce_cell_output] function uses the following default values for its arguments:
	/// * produce: false
	#[deprecated = "Use flag `use_timestamp_dim` in LayerParams."]
	// cv::dnn::LSTMLayer::setProduceCellOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:151
	// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn set_produce_cell_output_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput(self.as_raw_mut_LSTMLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:164
	// ("cv::dnn::LSTMLayer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
	#[inline]
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		extern_container_arg!(input_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_mut_LSTMLayer(), input_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:165
	// ("cv::dnn::LSTMLayer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	#[inline]
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_outputNameToIndex_const_StringR(self.as_raw_mut_LSTMLayer(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// LSTM recurrent layer
// LSTMLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:90
pub struct LSTMLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { LSTMLayer }

impl Drop for LSTMLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_LSTMLayer_delete(self.as_raw_mut_LSTMLayer()) };
	}
}

unsafe impl Send for LSTMLayer {}

impl core::AlgorithmTraitConst for LSTMLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LSTMLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LSTMLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for LSTMLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for LSTMLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LSTMLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::LSTMLayerTraitConst for LSTMLayer {
	#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LSTMLayerTrait for LSTMLayer {
	#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LSTMLayer, crate::dnn::LSTMLayerTraitConst, as_raw_LSTMLayer, crate::dnn::LSTMLayerTrait, as_raw_mut_LSTMLayer }

impl LSTMLayer {
	/// Creates instance of LSTM layer
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:94
	// ("cv::dnn::LSTMLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::LSTMLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LSTMLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::LSTMLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { LSTMLayer, core::Algorithm, cv_dnn_LSTMLayer_to_Algorithm }

boxed_cast_base! { LSTMLayer, crate::dnn::Layer, cv_dnn_LSTMLayer_to_Layer }

impl std::fmt::Debug for LSTMLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LSTMLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::Layer]
// Layer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:187
pub trait LayerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Layer(&self) -> *const c_void;

	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	// cv::dnn::Layer::blobs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:192
	// ("cv::dnn::Layer::blobs", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn blobs(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_dnn_Layer_propBlobs_const(self.as_raw_Layer()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	/// Name of the layer instance, can be used for logging or other internal purposes.
	// cv::dnn::Layer::name() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:361
	// ("cv::dnn::Layer::name", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn name(&self) -> String {
		let ret = unsafe { sys::cv_dnn_Layer_propName_const(self.as_raw_Layer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	/// Type name which was used for creating layer by layer factory.
	// cv::dnn::Layer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:362
	// ("cv::dnn::Layer::type", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn typ(&self) -> String {
		let ret = unsafe { sys::cv_dnn_Layer_propType_const(self.as_raw_Layer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	/// prefer target for layer forwarding
	// cv::dnn::Layer::preferableTarget() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:363
	// ("cv::dnn::Layer::preferableTarget", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn preferable_target(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_Layer_propPreferableTarget_const(self.as_raw_Layer()) };
		ret
	}

	/// Automatic Halide scheduling based on layer hyper-parameters.
	/// ## Parameters
	/// * node: Backend node with Halide functions.
	/// * inputs: Blobs that will be used in forward invocations.
	/// * outputs: Blobs that will be used in forward invocations.
	/// * targetId: Target identifier
	/// ## See also
	/// BackendNode, Target
	///
	/// Layer don't use own Halide::Func members because we can have applied
	/// layers fusing. In this way the fused function should be scheduled.
	// applyHalideScheduler(Ptr<BackendNode> &, const std::vector<Mat *> &, const std::vector<Mat> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:300
	// ("cv::dnn::Layer::applyHalideScheduler", vec![(pred!(const, ["node", "inputs", "outputs", "targetId"], ["cv::Ptr<cv::dnn::BackendNode>*", "const std::vector<cv::Mat*>*", "const std::vector<cv::Mat>*", "int"]), _)]),
	#[inline]
	fn apply_halide_scheduler(&self, node: &mut core::Ptr<crate::dnn::BackendNode>, inputs: &core::Vector<core::Mat>, outputs: &core::Vector<core::Mat>, target_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_applyHalideScheduler_const_PtrLBackendNodeGR_const_vectorLMatXGR_const_vectorLMatGR_int(self.as_raw_Layer(), node.as_raw_mut_PtrOfBackendNode(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), target_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns parameters of layers with channel-wise multiplication and addition.
	/// ## Parameters
	/// * scale:[out] Channel-wise multipliers. Total number of values should
	///                   be equal to number of channels.
	/// * shift:[out] Channel-wise offsets. Total number of values should
	///                   be equal to number of channels.
	///
	/// Some layers can fuse their transformations with further layers.
	/// In example, convolution + batch normalization. This way base layer
	/// use weights from layer after it. Fused layer is skipped.
	/// By default, @p scale and @p shift are empty that means layer has no
	/// element-wise multiplications or additions.
	// getScaleShift(Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:344
	// ("cv::dnn::Layer::getScaleShift", vec![(pred!(const, ["scale", "shift"], ["cv::Mat*", "cv::Mat*"]), _)]),
	#[inline]
	fn get_scale_shift(&self, scale: &mut impl core::MatTrait, shift: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_getScaleShift_const_MatR_MatR(self.as_raw_Layer(), scale.as_raw_mut_Mat(), shift.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getMemoryShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:351
	// ("cv::dnn::Layer::getMemoryShapes", vec![(pred!(const, ["inputs", "requiredOutputs", "outputs", "internals"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	#[inline]
	fn get_memory_shapes(&self, inputs: &core::Vector<crate::dnn::MatShape>, required_outputs: i32, outputs: &mut core::Vector<crate::dnn::MatShape>, internals: &mut core::Vector<crate::dnn::MatShape>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_getMemoryShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(self.as_raw_Layer(), inputs.as_raw_VectorOfMatShape(), required_outputs, outputs.as_raw_mut_VectorOfMatShape(), internals.as_raw_mut_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFLOPS(const std::vector<MatShape> &, const std::vector<MatShape> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:356
	// ("cv::dnn::Layer::getFLOPS", vec![(pred!(const, ["inputs", "outputs"], ["const std::vector<cv::dnn::MatShape>*", "const std::vector<cv::dnn::MatShape>*"]), _)]),
	#[inline]
	fn get_flops(&self, inputs: &core::Vector<crate::dnn::MatShape>, outputs: &core::Vector<crate::dnn::MatShape>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_getFLOPS_const_const_vectorLMatShapeGR_const_vectorLMatShapeGR(self.as_raw_Layer(), inputs.as_raw_VectorOfMatShape(), outputs.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::dnn::Layer]
pub trait LayerTrait: core::AlgorithmTrait + crate::dnn::LayerTraitConst {
	fn as_raw_mut_Layer(&mut self) -> *mut c_void;

	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	// cv::dnn::Layer::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:192
	// ("cv::dnn::Layer::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_blobs(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_dnn_Layer_propBlobs_const_vectorLMatG(self.as_raw_mut_Layer(), val.as_raw_VectorOfMat()) };
		ret
	}

	/// Name of the layer instance, can be used for logging or other internal purposes.
	// cv::dnn::Layer::setName(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:361
	// ("cv::dnn::Layer::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_name(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_dnn_Layer_propName_const_String(self.as_raw_mut_Layer(), val.opencv_as_extern()) };
		ret
	}

	/// Type name which was used for creating layer by layer factory.
	// cv::dnn::Layer::setType(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:362
	// ("cv::dnn::Layer::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_type(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_dnn_Layer_propType_const_String(self.as_raw_mut_Layer(), val.opencv_as_extern()) };
		ret
	}

	/// prefer target for layer forwarding
	// cv::dnn::Layer::setPreferableTarget(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:363
	// ("cv::dnn::Layer::setPreferableTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_preferable_target(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_Layer_propPreferableTarget_const_int(self.as_raw_mut_Layer(), val) };
		ret
	}

	/// Computes and sets internal parameters according to inputs, outputs and blobs.
	/// ## Parameters
	/// * inputs: vector of already allocated input blobs
	/// * outputs:[out] vector of already allocated output blobs
	///
	/// If this method is called after network has allocated all memory for input and output blobs
	/// and before inferencing.
	// finalize(InputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:212
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn finalize(&mut self, inputs: &impl ToInputArray, outputs: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Given the @p input blobs, computes the output @p blobs.
	///
	/// **Deprecated**: Use Layer::forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays) instead
	/// ## Parameters
	/// * input: the input blobs.
	/// * output:[out] allocated output blobs, which will store results of the computation.
	/// * internals:[out] allocated internal blobs
	#[deprecated = "Use Layer::forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays) instead"]
	// forward(std::vector<Mat *> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:221
	// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["input", "output", "internals"], ["std::vector<cv::Mat*>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn forward_mat(&mut self, input: &mut core::Vector<core::Mat>, output: &mut core::Vector<core::Mat>, internals: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_forward_vectorLMatXGR_vectorLMatGR_vectorLMatGR(self.as_raw_mut_Layer(), input.as_raw_mut_VectorOfMat(), output.as_raw_mut_VectorOfMat(), internals.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * inputs: the input blobs.
	/// * outputs:[out] allocated output blobs, which will store results of the computation.
	/// * internals:[out] allocated internal blobs
	// forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:228
	// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn forward(&mut self, inputs: &impl ToInputArray, outputs: &mut impl ToOutputArray, internals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		output_array_arg!(internals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * inputs: the input blobs.
	/// * outputs:[out] allocated output blobs, which will store results of the computation.
	/// * internals:[out] allocated internal blobs
	// forward_fallback(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:235
	// ("cv::dnn::Layer::forward_fallback", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn forward_fallback(&mut self, inputs: &impl ToInputArray, outputs: &mut impl ToOutputArray, internals: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		output_array_arg!(internals);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_forward_fallback_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	///
	/// Computes and sets internal parameters according to inputs, outputs and blobs.
	/// ## Parameters
	/// * inputs: vector of already allocated input blobs
	/// * outputs:[out] vector of already allocated output blobs
	///
	/// If this method is called after network has allocated all memory for input and output blobs
	/// and before inferencing.
	///
	/// ## Overloaded parameters
	///
	///
	/// **Deprecated**: Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead
	#[deprecated = "Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead"]
	// finalize(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:242
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn finalize_mat_to(&mut self, inputs: &core::Vector<core::Mat>, outputs: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_finalize_const_vectorLMatGR_vectorLMatGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	///
	/// Computes and sets internal parameters according to inputs, outputs and blobs.
	/// ## Parameters
	/// * inputs: vector of already allocated input blobs
	/// * outputs:[out] vector of already allocated output blobs
	///
	/// If this method is called after network has allocated all memory for input and output blobs
	/// and before inferencing.
	///
	/// ## Overloaded parameters
	///
	///
	/// **Deprecated**: Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead
	#[deprecated = "Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead"]
	// finalize(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:248
	// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn finalize_mat(&mut self, inputs: &core::Vector<core::Mat>) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_finalize_const_vectorLMatGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Allocates layer and computes output.
	///
	/// **Deprecated**: This method will be removed in the future release.
	#[deprecated = "This method will be removed in the future release."]
	// run(const std::vector<Mat> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:253
	// ("cv::dnn::Layer::run", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn run(&mut self, inputs: &core::Vector<core::Mat>, outputs: &mut core::Vector<core::Mat>, internals: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_run_const_vectorLMatGR_vectorLMatGR_vectorLMatGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_mut_VectorOfMat(), internals.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns index of input blob into the input array.
	/// ## Parameters
	/// * inputName: label of input blob
	///
	/// Each layer input and output can be labeled to easily identify them using "%<layer_name%>[.output_name]" notation.
	/// This method maps label of input blob to its index into input vector.
	// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:262
	// ("cv::dnn::Layer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
	#[inline]
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		extern_container_arg!(input_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_mut_Layer(), input_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns index of output blob in output array.
	/// ## See also
	/// inputNameToIndex()
	// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:266
	// ("cv::dnn::Layer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	#[inline]
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_outputNameToIndex_const_StringR(self.as_raw_mut_Layer(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Ask layer if it support specific backend for doing computations.
	/// ## Parameters
	/// * backendId: computation backend identifier.
	/// ## See also
	/// Backend
	// supportBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:273
	// ("cv::dnn::Layer::supportBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
	#[inline]
	fn support_backend(&mut self, backend_id: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_supportBackend_int(self.as_raw_mut_Layer(), backend_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns Halide backend node.
	/// ## Parameters
	/// * inputs: Input Halide buffers.
	/// ## See also
	/// BackendNode, BackendWrapper
	///
	/// Input buffers should be exactly the same that will be used in forward invocations.
	/// Despite we can use Halide::ImageParam based on input shape only,
	/// it helps prevent some memory management issues (if something wrong,
	/// Halide tests will be failed).
	// initHalide(const std::vector<Ptr<BackendWrapper>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:285
	// ("cv::dnn::Layer::initHalide", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*"]), _)]),
	#[inline]
	fn init_halide(&mut self, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_initHalide_const_vectorLPtrLBackendWrapperGGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// initNgraph(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:287
	// ("cv::dnn::Layer::initNgraph", vec![(pred!(mut, ["inputs", "nodes"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendNode>>*"]), _)]),
	#[inline]
	fn init_ngraph(&mut self, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, nodes: &core::Vector<core::Ptr<crate::dnn::BackendNode>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_initNgraph_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Implement layers fusing.
	/// ## Parameters
	/// * node: Backend node of bottom layer.
	/// ## See also
	/// BackendNode
	///
	/// Actual for graph-based backends. If layer attached successfully,
	/// returns non-empty cv::Ptr to node of the same backend.
	/// Fuse only over the last function.
	// tryAttach(const Ptr<BackendNode> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:314
	// ("cv::dnn::Layer::tryAttach", vec![(pred!(mut, ["node"], ["const cv::Ptr<cv::dnn::BackendNode>*"]), _)]),
	#[inline]
	fn try_attach(&mut self, node: &core::Ptr<crate::dnn::BackendNode>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_tryAttach_const_PtrLBackendNodeGR(self.as_raw_mut_Layer(), node.as_raw_PtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Tries to attach to the layer the subsequent activation layer, i.e. do the layer fusion in a partial case.
	/// ## Parameters
	/// * layer: The subsequent activation layer.
	///
	/// Returns true if the activation layer has been attached successfully.
	// setActivation(const Ptr<ActivationLayer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:322
	// ("cv::dnn::Layer::setActivation", vec![(pred!(mut, ["layer"], ["const cv::Ptr<cv::dnn::ActivationLayer>*"]), _)]),
	#[inline]
	fn set_activation(&mut self, layer: &core::Ptr<crate::dnn::ActivationLayer>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_setActivation_const_PtrLActivationLayerGR(self.as_raw_mut_Layer(), layer.as_raw_PtrOfActivationLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Try to fuse current layer with a next one
	/// ## Parameters
	/// * top: Next layer to be fused.
	/// ## Returns
	/// True if fusion was performed.
	// tryFuse(Ptr<Layer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:329
	// ("cv::dnn::Layer::tryFuse", vec![(pred!(mut, ["top"], ["cv::Ptr<cv::dnn::Layer>*"]), _)]),
	#[inline]
	fn try_fuse(&mut self, top: &mut core::Ptr<crate::dnn::Layer>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_tryFuse_PtrLLayerGR(self.as_raw_mut_Layer(), top.as_raw_mut_PtrOfLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// "Deattaches" all the layers, attached to particular layer.
	// unsetAttached()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:349
	// ("cv::dnn::Layer::unsetAttached", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn unset_attached(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_unsetAttached(self.as_raw_mut_Layer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// updateMemoryShapes(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:359
	// ("cv::dnn::Layer::updateMemoryShapes", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
	#[inline]
	fn update_memory_shapes(&mut self, inputs: &core::Vector<crate::dnn::MatShape>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_updateMemoryShapes_const_vectorLMatShapeGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setParamsFrom(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:367
	// ("cv::dnn::Layer::setParamsFrom", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	fn set_params_from(&mut self, params: &impl crate::dnn::LayerParamsTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_setParamsFrom_const_LayerParamsR(self.as_raw_mut_Layer(), params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// This interface class allows to build new Layers - are building blocks of networks.
///
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of [dnnLayerFactory] "LayerFactory" macros.
// Layer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:187
pub struct Layer {
	ptr: *mut c_void,
}

opencv_type_boxed! { Layer }

impl Drop for Layer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_Layer_delete(self.as_raw_mut_Layer()) };
	}
}

unsafe impl Send for Layer {}

impl core::AlgorithmTraitConst for Layer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Layer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Layer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for Layer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for Layer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Layer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl Layer {
	// Layer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:365
	// ("cv::dnn::Layer::Layer", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::dnn::Layer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_Layer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Layer::opencv_from_extern(ret) };
		Ok(ret)
	}

	// Layer(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:366
	// ("cv::dnn::Layer::Layer", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn new(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<crate::dnn::Layer> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Layer_Layer_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Layer::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { Layer, crate::dnn::AbsLayer, cv_dnn_Layer_to_AbsLayer }

boxed_cast_descendant! { Layer, crate::dnn::AccumLayer, cv_dnn_Layer_to_AccumLayer }

boxed_cast_descendant! { Layer, crate::dnn::ActivationLayer, cv_dnn_Layer_to_ActivationLayer }

boxed_cast_descendant! { Layer, crate::dnn::BNLLLayer, cv_dnn_Layer_to_BNLLLayer }

boxed_cast_descendant! { Layer, crate::dnn::BaseConvolutionLayer, cv_dnn_Layer_to_BaseConvolutionLayer }

boxed_cast_descendant! { Layer, crate::dnn::BatchNormLayer, cv_dnn_Layer_to_BatchNormLayer }

boxed_cast_descendant! { Layer, crate::dnn::BlankLayer, cv_dnn_Layer_to_BlankLayer }

boxed_cast_descendant! { Layer, crate::dnn::ChannelsPReLULayer, cv_dnn_Layer_to_ChannelsPReLULayer }

boxed_cast_descendant! { Layer, crate::dnn::ConcatLayer, cv_dnn_Layer_to_ConcatLayer }

boxed_cast_descendant! { Layer, crate::dnn::ConstLayer, cv_dnn_Layer_to_ConstLayer }

boxed_cast_descendant! { Layer, crate::dnn::ConvolutionLayer, cv_dnn_Layer_to_ConvolutionLayer }

boxed_cast_descendant! { Layer, crate::dnn::CorrelationLayer, cv_dnn_Layer_to_CorrelationLayer }

boxed_cast_descendant! { Layer, crate::dnn::CropAndResizeLayer, cv_dnn_Layer_to_CropAndResizeLayer }

boxed_cast_descendant! { Layer, crate::dnn::CropLayer, cv_dnn_Layer_to_CropLayer }

boxed_cast_descendant! { Layer, crate::dnn::DataAugmentationLayer, cv_dnn_Layer_to_DataAugmentationLayer }

boxed_cast_descendant! { Layer, crate::dnn::DeconvolutionLayer, cv_dnn_Layer_to_DeconvolutionLayer }

boxed_cast_descendant! { Layer, crate::dnn::DetectionOutputLayer, cv_dnn_Layer_to_DetectionOutputLayer }

boxed_cast_descendant! { Layer, crate::dnn::ELULayer, cv_dnn_Layer_to_ELULayer }

boxed_cast_descendant! { Layer, crate::dnn::EltwiseLayer, cv_dnn_Layer_to_EltwiseLayer }

boxed_cast_descendant! { Layer, crate::dnn::ExpLayer, cv_dnn_Layer_to_ExpLayer }

boxed_cast_descendant! { Layer, crate::dnn::FlattenLayer, cv_dnn_Layer_to_FlattenLayer }

boxed_cast_descendant! { Layer, crate::dnn::FlowWarpLayer, cv_dnn_Layer_to_FlowWarpLayer }

boxed_cast_descendant! { Layer, crate::dnn::InnerProductLayer, cv_dnn_Layer_to_InnerProductLayer }

boxed_cast_descendant! { Layer, crate::dnn::InterpLayer, cv_dnn_Layer_to_InterpLayer }

boxed_cast_descendant! { Layer, crate::dnn::LRNLayer, cv_dnn_Layer_to_LRNLayer }

boxed_cast_descendant! { Layer, crate::dnn::LSTMLayer, cv_dnn_Layer_to_LSTMLayer }

boxed_cast_descendant! { Layer, crate::dnn::MVNLayer, cv_dnn_Layer_to_MVNLayer }

boxed_cast_descendant! { Layer, crate::dnn::MaxUnpoolLayer, cv_dnn_Layer_to_MaxUnpoolLayer }

boxed_cast_descendant! { Layer, crate::dnn::MishLayer, cv_dnn_Layer_to_MishLayer }

boxed_cast_descendant! { Layer, crate::dnn::NormalizeBBoxLayer, cv_dnn_Layer_to_NormalizeBBoxLayer }

boxed_cast_descendant! { Layer, crate::dnn::PaddingLayer, cv_dnn_Layer_to_PaddingLayer }

boxed_cast_descendant! { Layer, crate::dnn::PermuteLayer, cv_dnn_Layer_to_PermuteLayer }

boxed_cast_descendant! { Layer, crate::dnn::PoolingLayer, cv_dnn_Layer_to_PoolingLayer }

boxed_cast_descendant! { Layer, crate::dnn::PowerLayer, cv_dnn_Layer_to_PowerLayer }

boxed_cast_descendant! { Layer, crate::dnn::PriorBoxLayer, cv_dnn_Layer_to_PriorBoxLayer }

boxed_cast_descendant! { Layer, crate::dnn::ProposalLayer, cv_dnn_Layer_to_ProposalLayer }

boxed_cast_descendant! { Layer, crate::dnn::RNNLayer, cv_dnn_Layer_to_RNNLayer }

boxed_cast_descendant! { Layer, crate::dnn::ReLU6Layer, cv_dnn_Layer_to_ReLU6Layer }

boxed_cast_descendant! { Layer, crate::dnn::ReLULayer, cv_dnn_Layer_to_ReLULayer }

boxed_cast_descendant! { Layer, crate::dnn::RegionLayer, cv_dnn_Layer_to_RegionLayer }

boxed_cast_descendant! { Layer, crate::dnn::ReorgLayer, cv_dnn_Layer_to_ReorgLayer }

boxed_cast_descendant! { Layer, crate::dnn::ReshapeLayer, cv_dnn_Layer_to_ReshapeLayer }

boxed_cast_descendant! { Layer, crate::dnn::ResizeLayer, cv_dnn_Layer_to_ResizeLayer }

boxed_cast_descendant! { Layer, crate::dnn::ScaleLayer, cv_dnn_Layer_to_ScaleLayer }

boxed_cast_descendant! { Layer, crate::dnn::ShiftLayer, cv_dnn_Layer_to_ShiftLayer }

boxed_cast_descendant! { Layer, crate::dnn::ShuffleChannelLayer, cv_dnn_Layer_to_ShuffleChannelLayer }

boxed_cast_descendant! { Layer, crate::dnn::SigmoidLayer, cv_dnn_Layer_to_SigmoidLayer }

boxed_cast_descendant! { Layer, crate::dnn::SliceLayer, cv_dnn_Layer_to_SliceLayer }

boxed_cast_descendant! { Layer, crate::dnn::SoftmaxLayer, cv_dnn_Layer_to_SoftmaxLayer }

boxed_cast_descendant! { Layer, crate::dnn::SplitLayer, cv_dnn_Layer_to_SplitLayer }

boxed_cast_descendant! { Layer, crate::dnn::SwishLayer, cv_dnn_Layer_to_SwishLayer }

boxed_cast_descendant! { Layer, crate::dnn::TanHLayer, cv_dnn_Layer_to_TanHLayer }

boxed_cast_base! { Layer, core::Algorithm, cv_dnn_Layer_to_Algorithm }

impl std::fmt::Debug for Layer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Layer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::LayerFactory]
// LayerFactory /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:56
pub trait LayerFactoryTraitConst {
	fn as_raw_LayerFactory(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::LayerFactory]
pub trait LayerFactoryTrait: crate::dnn::LayerFactoryTraitConst {
	fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void;

}

/// %Layer factory allows to create instances of registered layers.
// LayerFactory /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:56
pub struct LayerFactory {
	ptr: *mut c_void,
}

opencv_type_boxed! { LayerFactory }

impl Drop for LayerFactory {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_LayerFactory_delete(self.as_raw_mut_LayerFactory()) };
	}
}

unsafe impl Send for LayerFactory {}

impl crate::dnn::LayerFactoryTraitConst for LayerFactory {
	#[inline] fn as_raw_LayerFactory(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerFactoryTrait for LayerFactory {
	#[inline] fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LayerFactory, crate::dnn::LayerFactoryTraitConst, as_raw_LayerFactory, crate::dnn::LayerFactoryTrait, as_raw_mut_LayerFactory }

impl LayerFactory {
	/// Registers the layer class with typename @p type and specified @p constructor. Thread-safe.
	// registerLayer(const String &, Constructor)(InString, Function) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:64
	// ("cv::dnn::LayerFactory::registerLayer", vec![(pred!(mut, ["type", "constructor"], ["const cv::String*", "cv::dnn::LayerFactory::Constructor"]), _)]),
	#[inline]
	pub fn register_layer(typ: &str, constructor: crate::dnn::LayerFactory_Constructor) -> Result<()> {
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LayerFactory_registerLayer_const_StringR_Constructor(typ.opencv_as_extern(), constructor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Unregisters registered layer with specified type name. Thread-safe.
	// unregisterLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:67
	// ("cv::dnn::LayerFactory::unregisterLayer", vec![(pred!(mut, ["type"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn unregister_layer(typ: &str) -> Result<()> {
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_const_StringR(typ.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Creates instance of registered layer.
	/// ## Parameters
	/// * type: type name of creating layer.
	/// * params: parameters which will be used for layer initialization.
	///
	/// Note: Thread-safe.
	// createLayerInstance(const String &, LayerParams &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:74
	// ("cv::dnn::LayerFactory::createLayerInstance", vec![(pred!(mut, ["type", "params"], ["const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create_layer_instance(typ: &str, params: &mut impl crate::dnn::LayerParamsTrait) -> Result<core::Ptr<crate::dnn::Layer>> {
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(typ.opencv_as_extern(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for LayerFactory {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LayerFactory")
			.finish()
	}
}

/// Constant methods for [crate::dnn::LayerParams]
// LayerParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:112
pub trait LayerParamsTraitConst: crate::dnn::DictTraitConst {
	fn as_raw_LayerParams(&self) -> *const c_void;

	/// List of learned parameters stored as blobs.
	// cv::dnn::LayerParams::blobs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:116
	// ("cv::dnn::LayerParams::blobs", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn blobs(&self) -> core::Vector<core::Mat> {
		let ret = unsafe { sys::cv_dnn_LayerParams_propBlobs_const(self.as_raw_LayerParams()) };
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		ret
	}

	/// Name of the layer instance (optional, can be used internal purposes).
	// cv::dnn::LayerParams::name() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:118
	// ("cv::dnn::LayerParams::name", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn name(&self) -> String {
		let ret = unsafe { sys::cv_dnn_LayerParams_propName_const(self.as_raw_LayerParams()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	/// Type name which was used for creating layer by layer factory (optional).
	// cv::dnn::LayerParams::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:119
	// ("cv::dnn::LayerParams::type", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn typ(&self) -> String {
		let ret = unsafe { sys::cv_dnn_LayerParams_propType_const(self.as_raw_LayerParams()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::dnn::LayerParams]
pub trait LayerParamsTrait: crate::dnn::DictTrait + crate::dnn::LayerParamsTraitConst {
	fn as_raw_mut_LayerParams(&mut self) -> *mut c_void;

	/// List of learned parameters stored as blobs.
	// cv::dnn::LayerParams::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:116
	// ("cv::dnn::LayerParams::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	#[inline]
	fn set_blobs(&mut self, val: core::Vector<core::Mat>) {
		let ret = unsafe { sys::cv_dnn_LayerParams_propBlobs_const_vectorLMatG(self.as_raw_mut_LayerParams(), val.as_raw_VectorOfMat()) };
		ret
	}

	/// Name of the layer instance (optional, can be used internal purposes).
	// cv::dnn::LayerParams::setName(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:118
	// ("cv::dnn::LayerParams::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_name(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_dnn_LayerParams_propName_const_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern()) };
		ret
	}

	/// Type name which was used for creating layer by layer factory (optional).
	// cv::dnn::LayerParams::setType(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:119
	// ("cv::dnn::LayerParams::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_type(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_dnn_LayerParams_propType_const_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern()) };
		ret
	}

}

/// This class provides all data needed to initialize layer.
///
/// It includes dictionary with scalar params (which can be read by using Dict interface),
/// blob params [blobs] and optional meta information: [name] and [type] of layer instance.
// LayerParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:112
pub struct LayerParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { LayerParams }

impl Drop for LayerParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_LayerParams_delete(self.as_raw_mut_LayerParams()) };
	}
}

unsafe impl Send for LayerParams {}

impl crate::dnn::DictTraitConst for LayerParams {
	#[inline] fn as_raw_Dict(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::DictTrait for LayerParams {
	#[inline] fn as_raw_mut_Dict(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LayerParams, crate::dnn::DictTraitConst, as_raw_Dict, crate::dnn::DictTrait, as_raw_mut_Dict }

impl crate::dnn::LayerParamsTraitConst for LayerParams {
	#[inline] fn as_raw_LayerParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerParamsTrait for LayerParams {
	#[inline] fn as_raw_mut_LayerParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { LayerParams, crate::dnn::LayerParamsTraitConst, as_raw_LayerParams, crate::dnn::LayerParamsTrait, as_raw_mut_LayerParams }

impl LayerParams {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_LayerParams_defaultNew_const()) }
	}

}

boxed_cast_base! { LayerParams, crate::dnn::Dict, cv_dnn_LayerParams_to_Dict }

impl std::fmt::Debug for LayerParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LayerParams")
			.field("blobs", &crate::dnn::LayerParamsTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerParamsTraitConst::name(self))
			.field("typ", &crate::dnn::LayerParamsTraitConst::typ(self))
			.finish()
	}
}

impl Default for LayerParams {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::MVNLayer]
// MVNLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:284
pub trait MVNLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_MVNLayer(&self) -> *const c_void;

	// cv::dnn::MVNLayer::eps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:287
	// ("cv::dnn::MVNLayer::eps", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn eps(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_MVNLayer_propEps_const(self.as_raw_MVNLayer()) };
		ret
	}

	// cv::dnn::MVNLayer::normVariance() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
	// ("cv::dnn::MVNLayer::normVariance", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn norm_variance(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_MVNLayer_propNormVariance_const(self.as_raw_MVNLayer()) };
		ret
	}

	// cv::dnn::MVNLayer::acrossChannels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
	// ("cv::dnn::MVNLayer::acrossChannels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn across_channels(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_MVNLayer_propAcrossChannels_const(self.as_raw_MVNLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::MVNLayer]
pub trait MVNLayerTrait: crate::dnn::LayerTrait + crate::dnn::MVNLayerTraitConst {
	fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void;

	// cv::dnn::MVNLayer::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:287
	// ("cv::dnn::MVNLayer::setEps", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_eps(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_MVNLayer_propEps_const_float(self.as_raw_mut_MVNLayer(), val) };
		ret
	}

	// cv::dnn::MVNLayer::setNormVariance(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
	// ("cv::dnn::MVNLayer::setNormVariance", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_norm_variance(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_MVNLayer_propNormVariance_const_bool(self.as_raw_mut_MVNLayer(), val) };
		ret
	}

	// cv::dnn::MVNLayer::setAcrossChannels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
	// ("cv::dnn::MVNLayer::setAcrossChannels", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_across_channels(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_MVNLayer_propAcrossChannels_const_bool(self.as_raw_mut_MVNLayer(), val) };
		ret
	}

}

// MVNLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:284
pub struct MVNLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { MVNLayer }

impl Drop for MVNLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_MVNLayer_delete(self.as_raw_mut_MVNLayer()) };
	}
}

unsafe impl Send for MVNLayer {}

impl core::AlgorithmTraitConst for MVNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MVNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MVNLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for MVNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for MVNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MVNLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::MVNLayerTraitConst for MVNLayer {
	#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::MVNLayerTrait for MVNLayer {
	#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MVNLayer, crate::dnn::MVNLayerTraitConst, as_raw_MVNLayer, crate::dnn::MVNLayerTrait, as_raw_mut_MVNLayer }

impl MVNLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_MVNLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:290
	// ("cv::dnn::MVNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::MVNLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MVNLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::MVNLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MVNLayer, core::Algorithm, cv_dnn_MVNLayer_to_Algorithm }

boxed_cast_base! { MVNLayer, crate::dnn::Layer, cv_dnn_MVNLayer_to_Layer }

impl std::fmt::Debug for MVNLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MVNLayer")
			.field("eps", &crate::dnn::MVNLayerTraitConst::eps(self))
			.field("norm_variance", &crate::dnn::MVNLayerTraitConst::norm_variance(self))
			.field("across_channels", &crate::dnn::MVNLayerTraitConst::across_channels(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for MVNLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::MaxUnpoolLayer]
// MaxUnpoolLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:543
pub trait MaxUnpoolLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_MaxUnpoolLayer(&self) -> *const c_void;

	// cv::dnn::MaxUnpoolLayer::poolKernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:546
	// ("cv::dnn::MaxUnpoolLayer::poolKernel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pool_kernel(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolKernel_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::MaxUnpoolLayer::poolPad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:547
	// ("cv::dnn::MaxUnpoolLayer::poolPad", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pool_pad(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolPad_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::MaxUnpoolLayer::poolStride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:548
	// ("cv::dnn::MaxUnpoolLayer::poolStride", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pool_stride(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolStride_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

}

/// Mutable methods for [crate::dnn::MaxUnpoolLayer]
pub trait MaxUnpoolLayerTrait: crate::dnn::LayerTrait + crate::dnn::MaxUnpoolLayerTraitConst {
	fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void;

	// cv::dnn::MaxUnpoolLayer::setPoolKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:546
	// ("cv::dnn::MaxUnpoolLayer::setPoolKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_pool_kernel(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolKernel_const_Size(self.as_raw_mut_MaxUnpoolLayer(), &val) };
		ret
	}

	// cv::dnn::MaxUnpoolLayer::setPoolPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:547
	// ("cv::dnn::MaxUnpoolLayer::setPoolPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_pool_pad(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolPad_const_Size(self.as_raw_mut_MaxUnpoolLayer(), &val) };
		ret
	}

	// cv::dnn::MaxUnpoolLayer::setPoolStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:548
	// ("cv::dnn::MaxUnpoolLayer::setPoolStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_pool_stride(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolStride_const_Size(self.as_raw_mut_MaxUnpoolLayer(), &val) };
		ret
	}

}

// MaxUnpoolLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:543
pub struct MaxUnpoolLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { MaxUnpoolLayer }

impl Drop for MaxUnpoolLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_delete(self.as_raw_mut_MaxUnpoolLayer()) };
	}
}

unsafe impl Send for MaxUnpoolLayer {}

impl core::AlgorithmTraitConst for MaxUnpoolLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MaxUnpoolLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MaxUnpoolLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for MaxUnpoolLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for MaxUnpoolLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MaxUnpoolLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::MaxUnpoolLayerTraitConst for MaxUnpoolLayer {
	#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::MaxUnpoolLayerTrait for MaxUnpoolLayer {
	#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MaxUnpoolLayer, crate::dnn::MaxUnpoolLayerTraitConst, as_raw_MaxUnpoolLayer, crate::dnn::MaxUnpoolLayerTrait, as_raw_mut_MaxUnpoolLayer }

impl MaxUnpoolLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_MaxUnpoolLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:550
	// ("cv::dnn::MaxUnpoolLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::MaxUnpoolLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MaxUnpoolLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::MaxUnpoolLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MaxUnpoolLayer, core::Algorithm, cv_dnn_MaxUnpoolLayer_to_Algorithm }

boxed_cast_base! { MaxUnpoolLayer, crate::dnn::Layer, cv_dnn_MaxUnpoolLayer_to_Layer }

impl std::fmt::Debug for MaxUnpoolLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MaxUnpoolLayer")
			.field("pool_kernel", &crate::dnn::MaxUnpoolLayerTraitConst::pool_kernel(self))
			.field("pool_pad", &crate::dnn::MaxUnpoolLayerTraitConst::pool_pad(self))
			.field("pool_stride", &crate::dnn::MaxUnpoolLayerTraitConst::pool_stride(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for MaxUnpoolLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::MishLayer]
// MishLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:473
pub trait MishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_MishLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::MishLayer]
pub trait MishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::MishLayerTraitConst {
	fn as_raw_mut_MishLayer(&mut self) -> *mut c_void;

}

// MishLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:473
pub struct MishLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { MishLayer }

impl Drop for MishLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_MishLayer_delete(self.as_raw_mut_MishLayer()) };
	}
}

unsafe impl Send for MishLayer {}

impl crate::dnn::ActivationLayerTraitConst for MishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for MishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MishLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for MishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MishLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for MishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for MishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MishLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::MishLayerTraitConst for MishLayer {
	#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::MishLayerTrait for MishLayer {
	#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MishLayer, crate::dnn::MishLayerTraitConst, as_raw_MishLayer, crate::dnn::MishLayerTrait, as_raw_mut_MishLayer }

impl MishLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:476
	// ("cv::dnn::MishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::MishLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_MishLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::MishLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MishLayer, crate::dnn::ActivationLayer, cv_dnn_MishLayer_to_ActivationLayer }

boxed_cast_base! { MishLayer, core::Algorithm, cv_dnn_MishLayer_to_Algorithm }

boxed_cast_base! { MishLayer, crate::dnn::Layer, cv_dnn_MishLayer_to_Layer }

impl std::fmt::Debug for MishLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MishLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::Net]
// Net /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:381
pub trait NetTraitConst {
	fn as_raw_Net(&self) -> *const c_void;

	/// Returns true if there are no layers in the network.
	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:416
	// ("cv::dnn::Net::empty", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_empty_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts string name of the layer to the integer identifier.
	/// ## Returns
	/// id of the layer, or -1 if the layer wasn't found.
	// getLayerId(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:443
	// ("cv::dnn::Net::getLayerId", vec![(pred!(const, ["layer"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_layer_id(&self, layer: &str) -> Result<i32> {
		extern_container_arg!(layer);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerId_const_const_StringR(self.as_raw_Net(), layer.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getLayerNames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:445
	// ("cv::dnn::Net::getLayerNames", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_layer_names(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerNames_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns pointer to layer with specified id or name which the network use.
	// getLayer(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:454
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["int"]), _)]),
	#[inline]
	fn get_layer(&self, layer_id: i32) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayer_const_int(self.as_raw_Net(), layer_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns pointer to layer with specified id or name which the network use.
	///
	/// ## Overloaded parameters
	///
	///
	/// **Deprecated**: Use int getLayerId(const String &layer)
	#[deprecated = "Use int getLayerId(const String &layer)"]
	// getLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:458
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_layer_1(&self, layer_name: &str) -> Result<core::Ptr<crate::dnn::Layer>> {
		extern_container_arg!(layer_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayer_const_const_StringR(self.as_raw_Net(), layer_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns pointer to layer with specified id or name which the network use.
	///
	/// ## Overloaded parameters
	///
	///
	/// **Deprecated**: to be removed
	#[deprecated = "to be removed"]
	// getLayer(const LayerId &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:462
	// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["const cv::dnn::Net::LayerId*"]), _)]),
	#[inline]
	fn get_layer_2(&self, layer_id: &impl crate::dnn::DictValueTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayer_const_const_LayerIdR(self.as_raw_Net(), layer_id.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns pointers to input layers of specific layer.
	// getLayerInputs(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:465
	// ("cv::dnn::Net::getLayerInputs", vec![(pred!(const, ["layerId"], ["int"]), _)]),
	#[inline]
	fn get_layer_inputs(&self, layer_id: i32) -> Result<core::Vector<core::Ptr<crate::dnn::Layer>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerInputs_const_int(self.as_raw_Net(), layer_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Ptr<crate::dnn::Layer>>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns parameter blob of the layer.
	/// ## Parameters
	/// * layer: name or id of the layer.
	/// * numParam: index of the layer parameter in the Layer::blobs array.
	/// ## See also
	/// Layer::blobs
	///
	/// ## C++ default parameters
	/// * num_param: 0
	// getParam(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:619
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer", "numParam"], ["int", "int"]), _)]),
	#[inline]
	fn get_param(&self, layer: i32, num_param: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getParam_const_int_int(self.as_raw_Net(), layer, num_param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns parameter blob of the layer.
	/// ## Parameters
	/// * layer: name or id of the layer.
	/// * numParam: index of the layer parameter in the Layer::blobs array.
	/// ## See also
	/// Layer::blobs
	///
	/// ## Note
	/// This alternative version of [NetTraitConst::get_param] function uses the following default values for its arguments:
	/// * num_param: 0
	// cv::dnn::Net::getParam(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:619
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer"], ["int"]), _)]),
	#[inline]
	fn get_param_def(&self, layer: i32) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getParam_const_int(self.as_raw_Net(), layer, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * num_param: 0
	// getParam(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:620
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName", "numParam"], ["const cv::String*", "int"]), _)]),
	#[inline]
	fn get_param_1(&self, layer_name: &str, num_param: i32) -> Result<core::Mat> {
		extern_container_arg!(layer_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getParam_const_const_StringR_int(self.as_raw_Net(), layer_name.opencv_as_extern(), num_param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [NetTraitConst::get_param] function uses the following default values for its arguments:
	/// * num_param: 0
	// cv::dnn::Net::getParam(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:620
	// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_param_def_1(&self, layer_name: &str) -> Result<core::Mat> {
		extern_container_arg!(layer_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getParam_const_const_StringR(self.as_raw_Net(), layer_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns indexes of layers with unconnected outputs.
	///
	/// FIXIT: Rework API to registerOutput() approach, deprecate this call
	// getUnconnectedOutLayers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:626
	// ("cv::dnn::Net::getUnconnectedOutLayers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_unconnected_out_layers(&self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getUnconnectedOutLayers_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns names of layers with unconnected outputs.
	///
	/// FIXIT: Rework API to registerOutput() approach, deprecate this call
	// getUnconnectedOutLayersNames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:632
	// ("cv::dnn::Net::getUnconnectedOutLayersNames", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_unconnected_out_layers_names(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getUnconnectedOutLayersNames_const(self.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Returns input and output shapes for all layers in loaded model;
	///  preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShapes: shapes for all input blobs in net input layer.
	/// * layersIds: output parameter for layer IDs.
	/// * inLayersShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayersShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	// getLayersShapes(const std::vector<MatShape> &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:643
	// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShapes", "layersIds", "inLayersShapes", "outLayersShapes"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
	#[inline]
	fn get_layers_shapes(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, layers_ids: &mut core::Vector<i32>, in_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>, out_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayersShapes_const_const_vectorLMatShapeGR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layers_ids.as_raw_mut_VectorOfi32(), in_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), out_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns input and output shapes for all layers in loaded model;
	///  preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShapes: shapes for all input blobs in net input layer.
	/// * layersIds: output parameter for layer IDs.
	/// * inLayersShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayersShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	///
	/// ## Overloaded parameters
	// getLayersShapes(const MatShape &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:649
	// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShape", "layersIds", "inLayersShapes", "outLayersShapes"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
	#[inline]
	fn get_layers_shapes_1(&self, net_input_shape: &crate::dnn::MatShape, layers_ids: &mut core::Vector<i32>, in_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>, out_layers_shapes: &mut core::Vector<core::Vector<crate::dnn::MatShape>>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayersShapes_const_const_MatShapeR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layers_ids.as_raw_mut_VectorOfi32(), in_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), out_layers_shapes.as_raw_mut_VectorOfVectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns input and output shapes for layer with specified
	/// id in loaded model; preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShape: shape input blob in net input layer.
	/// * layerId: id for layer.
	/// * inLayerShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayerShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	// getLayerShapes(const MatShape &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:663
	// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShape", "layerId", "inLayerShapes", "outLayerShapes"], ["const cv::dnn::MatShape*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	#[inline]
	fn get_layer_shapes(&self, net_input_shape: &crate::dnn::MatShape, layer_id: i32, in_layer_shapes: &mut core::Vector<crate::dnn::MatShape>, out_layer_shapes: &mut core::Vector<crate::dnn::MatShape>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerShapes_const_const_MatShapeR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layer_id, in_layer_shapes.as_raw_mut_VectorOfMatShape(), out_layer_shapes.as_raw_mut_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns input and output shapes for layer with specified
	/// id in loaded model; preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShape: shape input blob in net input layer.
	/// * layerId: id for layer.
	/// * inLayerShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayerShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	///
	/// ## Overloaded parameters
	// getLayerShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:669
	// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShapes", "layerId", "inLayerShapes", "outLayerShapes"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
	#[inline]
	fn get_layer_shapes_1(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, layer_id: i32, in_layer_shapes: &mut core::Vector<crate::dnn::MatShape>, out_layer_shapes: &mut core::Vector<crate::dnn::MatShape>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layer_id, in_layer_shapes.as_raw_mut_VectorOfMatShape(), out_layer_shapes.as_raw_mut_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	// getFLOPS(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:678
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShapes"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
	#[inline]
	fn get_flops(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_vectorLMatShapeGR(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	///
	/// ## Overloaded parameters
	// getFLOPS(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:680
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShape"], ["const cv::dnn::MatShape*"]), _)]),
	#[inline]
	fn get_flops_1(&self, net_input_shape: &crate::dnn::MatShape) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_MatShapeR(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	///
	/// ## Overloaded parameters
	// getFLOPS(const int, const std::vector<MatShape> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:682
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShapes"], ["const int", "const std::vector<cv::dnn::MatShape>*"]), _)]),
	#[inline]
	fn get_flops_2(&self, layer_id: i32, net_input_shapes: &core::Vector<crate::dnn::MatShape>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_int_const_vectorLMatShapeGR(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	///
	/// ## Overloaded parameters
	// getFLOPS(const int, const MatShape &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:685
	// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShape"], ["const int", "const cv::dnn::MatShape*"]), _)]),
	#[inline]
	fn get_flops_3(&self, layer_id: i32, net_input_shape: &crate::dnn::MatShape) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_int_const_MatShapeR(self.as_raw_Net(), layer_id, net_input_shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns list of types for layer used in model.
	/// ## Parameters
	/// * layersTypes: output parameter for returning types.
	// getLayerTypes(std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:691
	// ("cv::dnn::Net::getLayerTypes", vec![(pred!(const, ["layersTypes"], ["std::vector<cv::String>*"]), _)]),
	#[inline]
	fn get_layer_types(&self, layers_types: &mut core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayerTypes_const_vectorLStringGR(self.as_raw_Net(), layers_types.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns count of layers of specified type.
	/// ## Parameters
	/// * layerType: type.
	/// ## Returns
	/// count of layers
	// getLayersCount(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:697
	// ("cv::dnn::Net::getLayersCount", vec![(pred!(const, ["layerType"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_layers_count(&self, layer_type: &str) -> Result<i32> {
		extern_container_arg!(layer_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getLayersCount_const_const_StringR(self.as_raw_Net(), layer_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for model.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	// getMemoryConsumption(const std::vector<MatShape> &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:705
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
	#[inline]
	fn get_memory_consumption(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_size_tR_size_tR(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for model.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	///
	/// ## Overloaded parameters
	// getMemoryConsumption(const MatShape &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:708
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "weights", "blobs"], ["const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
	#[inline]
	fn get_memory_consumption_1(&self, net_input_shape: &crate::dnn::MatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_size_tR_size_tR(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for model.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	///
	/// ## Overloaded parameters
	// getMemoryConsumption(const int, const std::vector<MatShape> &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:711
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShapes", "weights", "blobs"], ["const int", "const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
	#[inline]
	fn get_memory_consumption_for_layer(&self, layer_id: i32, net_input_shapes: &core::Vector<crate::dnn::MatShape>, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_int_const_vectorLMatShapeGR_size_tR_size_tR(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfMatShape(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for model.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	///
	/// ## Overloaded parameters
	// getMemoryConsumption(const int, const MatShape &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:715
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShape", "weights", "blobs"], ["const int", "const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
	#[inline]
	fn get_memory_consumption_2(&self, layer_id: i32, net_input_shape: &crate::dnn::MatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_int_const_MatShapeR_size_tR_size_tR(self.as_raw_Net(), layer_id, net_input_shape.as_raw_VectorOfi32(), weights, blobs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for each layer.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * layerIds: output vector to save layer IDs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	// getMemoryConsumption(const std::vector<MatShape> &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:726
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "layerIds", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
	#[inline]
	fn get_memory_consumption_for_layers(&self, net_input_shapes: &core::Vector<crate::dnn::MatShape>, layer_ids: &mut core::Vector<i32>, weights: &mut core::Vector<size_t>, blobs: &mut core::Vector<size_t>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layer_ids.as_raw_mut_VectorOfi32(), weights.as_raw_mut_VectorOfsize_t(), blobs.as_raw_mut_VectorOfsize_t(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for each layer.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * layerIds: output vector to save layer IDs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	///
	/// ## Overloaded parameters
	// getMemoryConsumption(const MatShape &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:731
	// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "layerIds", "weights", "blobs"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
	#[inline]
	fn get_memory_consumption_3(&self, net_input_shape: &crate::dnn::MatShape, layer_ids: &mut core::Vector<i32>, weights: &mut core::Vector<size_t>, blobs: &mut core::Vector<size_t>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layer_ids.as_raw_mut_VectorOfi32(), weights.as_raw_mut_VectorOfsize_t(), blobs.as_raw_mut_VectorOfsize_t(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::dnn::Net]
pub trait NetTrait: crate::dnn::NetTraitConst {
	fn as_raw_mut_Net(&mut self) -> *mut c_void;

	/// Dump net to String
	/// ## Returns
	/// String with structure, hyperparameters, backend, target and fusion
	/// Call method after setInput(). To see correct backend, target and fusion run after forward().
	// dump()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:422
	// ("cv::dnn::Net::dump", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn dump(&mut self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_dump(self.as_raw_mut_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Dump net structure, hyperparameters, backend, target and fusion to dot file
	/// ## Parameters
	/// * path: path to output file with .dot extension
	/// ## See also
	/// dump()
	// dumpToFile(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:427
	// ("cv::dnn::Net::dumpToFile", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	#[inline]
	fn dump_to_file(&mut self, path: &str) -> Result<()> {
		extern_container_arg!(path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_dumpToFile_const_StringR(self.as_raw_mut_Net(), path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Adds new layer to the net.
	/// ## Parameters
	/// * name: unique name of the adding layer.
	/// * type: typename of the adding layer (type must be registered in LayerRegister).
	/// * params: parameters which will be used to initialize the creating layer.
	/// ## Returns
	/// unique identifier of created layer, or -1 if a failure will happen.
	// addLayer(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:434
	// ("cv::dnn::Net::addLayer", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	#[inline]
	fn add_layer(&mut self, name: &str, typ: &str, params: &mut impl crate::dnn::LayerParamsTrait) -> Result<i32> {
		extern_container_arg!(name);
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Adds new layer and connects its first input to the first output of previously added layer.
	/// ## See also
	/// addLayer()
	// addLayerToPrev(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:438
	// ("cv::dnn::Net::addLayerToPrev", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
	#[inline]
	fn add_layer_to_prev(&mut self, name: &str, typ: &str, params: &mut impl crate::dnn::LayerParamsTrait) -> Result<i32> {
		extern_container_arg!(name);
		extern_container_arg!(typ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Connects output of the first layer to input of the second layer.
	/// ## Parameters
	/// * outPin: descriptor of the first layer output.
	/// * inpPin: descriptor of the second layer input.
	///
	/// Descriptors have the following template <DFN>&lt;layer_name&gt;[.input_number]</DFN>:
	/// - the first part of the template <DFN>layer_name</DFN> is string name of the added layer.
	///   If this part is empty then the network input pseudo layer will be used;
	/// - the second optional part of the template <DFN>input_number</DFN>
	///   is either number of the layer input, either label one.
	///   If this part is omitted then the first layer input will be used.
	/// ## See also
	/// setNetInputs(), Layer::inputNameToIndex(), Layer::outputNameToIndex()
	// connect(String, String)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:480
	// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outPin", "inpPin"], ["cv::String", "cv::String"]), _)]),
	#[inline]
	fn connect_first_second(&mut self, out_pin: &str, inp_pin: &str) -> Result<()> {
		extern_container_arg!(out_pin);
		extern_container_arg!(inp_pin);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_mut_Net(), out_pin.opencv_as_extern(), inp_pin.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Connects #@p outNum output of the first layer to #@p inNum input of the second layer.
	/// ## Parameters
	/// * outLayerId: identifier of the first layer
	/// * outNum: number of the first layer output
	/// * inpLayerId: identifier of the second layer
	/// * inpNum: number of the second layer input
	// connect(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:488
	// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outLayerId", "outNum", "inpLayerId", "inpNum"], ["int", "int", "int", "int"]), _)]),
	#[inline]
	fn connect(&mut self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_connect_int_int_int_int(self.as_raw_mut_Net(), out_layer_id, out_num, inp_layer_id, inp_num, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Registers network output with name
	///
	/// Function may create additional 'Identity' layer.
	///
	/// ## Parameters
	/// * outputName: identifier of the output
	/// * layerId: identifier of the second layer
	/// * outputPort: number of the second layer input
	///
	/// ## Returns
	/// index of bound layer (the same as layerId or newly created)
	// registerOutput(const std::string &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:500
	// ("cv::dnn::Net::registerOutput", vec![(pred!(mut, ["outputName", "layerId", "outputPort"], ["const std::string*", "int", "int"]), _)]),
	#[inline]
	fn register_output(&mut self, output_name: &str, layer_id: i32, output_port: i32) -> Result<i32> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_registerOutput_const_stringR_int_int(self.as_raw_mut_Net(), output_name.opencv_as_extern(), layer_id, output_port, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets outputs names of the network input pseudo layer.
	///
	/// Each net always has special own the network input pseudo layer with id=0.
	/// This layer stores the user blobs only and don't make any computations.
	/// In fact, this layer provides the only way to pass user data into the network.
	/// As any other layer, this layer can label its outputs and this function provides an easy way to do this.
	// setInputsNames(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:509
	// ("cv::dnn::Net::setInputsNames", vec![(pred!(mut, ["inputBlobNames"], ["const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn set_inputs_names(&mut self, input_blob_names: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setInputsNames_const_vectorLStringGR(self.as_raw_mut_Net(), input_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Specify shape of network input.
	// setInputShape(const String &, const MatShape &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:513
	// ("cv::dnn::Net::setInputShape", vec![(pred!(mut, ["inputName", "shape"], ["const cv::String*", "const cv::dnn::MatShape*"]), _)]),
	#[inline]
	fn set_input_shape(&mut self, input_name: &str, shape: &crate::dnn::MatShape) -> Result<()> {
		extern_container_arg!(input_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setInputShape_const_StringR_const_MatShapeR(self.as_raw_mut_Net(), input_name.opencv_as_extern(), shape.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputName: name for layer which output is needed to get
	/// ## Returns
	/// blob for first output of specified layer.
	/// @details By default runs forward pass for the whole network.
	///
	/// ## C++ default parameters
	/// * output_name: String()
	// forward(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:520
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	#[inline]
	fn forward_single(&mut self, output_name: &str) -> Result<core::Mat> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_const_StringR(self.as_raw_mut_Net(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputName: name for layer which output is needed to get
	/// ## Returns
	/// blob for first output of specified layer.
	/// @details By default runs forward pass for the whole network.
	///
	/// ## Note
	/// This alternative version of [NetTrait::forward_single] function uses the following default values for its arguments:
	/// * output_name: String()
	// cv::dnn::Net::forward() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:520
	// ("cv::dnn::Net::forward", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn forward_single_def(&mut self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward(self.as_raw_mut_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputName: name for layer which output is needed to get
	/// @details By default runs forward pass for the whole network.
	///
	/// This is an asynchronous version of forward(const String&).
	/// dnn::DNN_BACKEND_INFERENCE_ENGINE backend is required.
	///
	/// ## C++ default parameters
	/// * output_name: String()
	// forwardAsync(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:529
	// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
	#[inline]
	fn forward_async(&mut self, output_name: &str) -> Result<core::AsyncArray> {
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forwardAsync_const_StringR(self.as_raw_mut_Net(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputName: name for layer which output is needed to get
	/// @details By default runs forward pass for the whole network.
	///
	/// This is an asynchronous version of forward(const String&).
	/// dnn::DNN_BACKEND_INFERENCE_ENGINE backend is required.
	///
	/// ## Note
	/// This alternative version of [NetTrait::forward_async] function uses the following default values for its arguments:
	/// * output_name: String()
	// cv::dnn::Net::forwardAsync() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:529
	// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn forward_async_def(&mut self) -> Result<core::AsyncArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forwardAsync(self.as_raw_mut_Net(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::AsyncArray::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputBlobs: contains all output blobs for specified layer.
	/// * outputName: name for layer which output is needed to get
	/// @details If @p outputName is empty, runs forward pass for the whole network.
	///
	/// ## C++ default parameters
	/// * output_name: String()
	// forward(OutputArrayOfArrays, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:536
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outputName"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
	#[inline]
	fn forward_layer(&mut self, output_blobs: &mut impl ToOutputArray, output_name: &str) -> Result<()> {
		output_array_arg!(output_blobs);
		extern_container_arg!(output_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_const__OutputArrayR_const_StringR(self.as_raw_mut_Net(), output_blobs.as_raw__OutputArray(), output_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputBlobs: contains all output blobs for specified layer.
	/// * outputName: name for layer which output is needed to get
	/// @details If @p outputName is empty, runs forward pass for the whole network.
	///
	/// ## Note
	/// This alternative version of [NetTrait::forward_layer] function uses the following default values for its arguments:
	/// * output_name: String()
	// cv::dnn::Net::forward(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:536
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn forward_layer_def(&mut self, output_blobs: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(output_blobs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_const__OutputArrayR(self.as_raw_mut_Net(), output_blobs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
	/// ## Parameters
	/// * outputBlobs: contains blobs for first outputs of specified layers.
	/// * outBlobNames: names for layers which outputs are needed to get
	// forward(OutputArrayOfArrays, const std::vector<String> &)(OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:542
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["const cv::_OutputArray*", "const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn forward(&mut self, output_blobs: &mut impl ToOutputArray, out_blob_names: &core::Vector<String>) -> Result<()> {
		output_array_arg!(output_blobs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_const__OutputArrayR_const_vectorLStringGR(self.as_raw_mut_Net(), output_blobs.as_raw__OutputArray(), out_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
	/// ## Parameters
	/// * outputBlobs: contains all output blobs for each layer specified in @p outBlobNames.
	/// * outBlobNames: names for layers which outputs are needed to get
	// forward(std::vector<std::vector<Mat>> &, const std::vector<String> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:549
	// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn forward_and_retrieve(&mut self, output_blobs: &mut core::Vector<core::Vector<core::Mat>>, out_blob_names: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_forward_vectorLvectorLMatGGR_const_vectorLStringGR(self.as_raw_mut_Net(), output_blobs.as_raw_mut_VectorOfVectorOfMat(), out_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Compile Halide layers.
	/// ## Parameters
	/// * scheduler: Path to YAML file with scheduling directives.
	/// ## See also
	/// setPreferableBackend
	///
	/// Schedule layers that support Halide backend. Then compile them for
	/// specific target. For layers that not represented in scheduling file
	/// or if no manual scheduling used at all, automatic scheduling will be applied.
	// setHalideScheduler(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:561
	// ("cv::dnn::Net::setHalideScheduler", vec![(pred!(mut, ["scheduler"], ["const cv::String*"]), _)]),
	#[inline]
	fn set_halide_scheduler(&mut self, scheduler: &str) -> Result<()> {
		extern_container_arg!(scheduler);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setHalideScheduler_const_StringR(self.as_raw_mut_Net(), scheduler.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Ask network to use specific computation backend where it supported.
	/// ## Parameters
	/// * backendId: backend identifier.
	/// ## See also
	/// Backend
	///
	/// If OpenCV is compiled with Intel's Inference Engine library, DNN_BACKEND_DEFAULT
	/// means DNN_BACKEND_INFERENCE_ENGINE. Otherwise it equals to DNN_BACKEND_OPENCV.
	// setPreferableBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:571
	// ("cv::dnn::Net::setPreferableBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
	#[inline]
	fn set_preferable_backend(&mut self, backend_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setPreferableBackend_int(self.as_raw_mut_Net(), backend_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Ask network to make computations on specific target device.
	/// ## Parameters
	/// * targetId: target identifier.
	/// ## See also
	/// Target
	///
	/// List of supported combinations backend / target:
	/// |                        | DNN_BACKEND_OPENCV | DNN_BACKEND_INFERENCE_ENGINE | DNN_BACKEND_HALIDE |
	/// |------------------------|--------------------|------------------------------|--------------------|
	/// | DNN_TARGET_CPU         |                  + |                            + |                  + |
	/// | DNN_TARGET_OPENCL      |                  + |                            + |                  + |
	/// | DNN_TARGET_OPENCL_FP16 |                  + |                            + |                    |
	/// | DNN_TARGET_MYRIAD      |                    |                            + |                    |
	/// | DNN_TARGET_FPGA        |                    |                            + |                    |
	// setPreferableTarget(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:587
	// ("cv::dnn::Net::setPreferableTarget", vec![(pred!(mut, ["targetId"], ["int"]), _)]),
	#[inline]
	fn set_preferable_target(&mut self, target_id: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setPreferableTarget_int(self.as_raw_mut_Net(), target_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the new input value for the network
	/// ## Parameters
	/// * blob: A new blob. Should have CV_32F or CV_8U depth.
	/// * name: A name of input layer.
	/// * scalefactor: An optional normalization scale.
	/// * mean: An optional mean subtraction values.
	/// ## See also
	/// connect(String, String) to know format of the descriptor.
	///
	///  If scale or mean values are specified, a final input blob is computed
	///  as:
	/// ![block formula](https://latex.codecogs.com/png.latex?input%28n%2Cc%2Ch%2Cw%29%20%3D%20scalefactor%20%5Ctimes%20%28blob%28n%2Cc%2Ch%2Cw%29%20%2D%20mean%5Fc%29)
	///
	/// ## C++ default parameters
	/// * name: ""
	/// * scalefactor: 1.0
	/// * mean: Scalar()
	// setInput(InputArray, const String &, double, const Scalar &)(InputArray, InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:600
	// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob", "name", "scalefactor", "mean"], ["const cv::_InputArray*", "const cv::String*", "double", "const cv::Scalar*"]), _)]),
	#[inline]
	fn set_input(&mut self, blob: &impl ToInputArray, name: &str, scalefactor: f64, mean: core::Scalar) -> Result<()> {
		input_array_arg!(blob);
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setInput_const__InputArrayR_const_StringR_double_const_ScalarR(self.as_raw_mut_Net(), blob.as_raw__InputArray(), name.opencv_as_extern(), scalefactor, &mean, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the new input value for the network
	/// ## Parameters
	/// * blob: A new blob. Should have CV_32F or CV_8U depth.
	/// * name: A name of input layer.
	/// * scalefactor: An optional normalization scale.
	/// * mean: An optional mean subtraction values.
	/// ## See also
	/// connect(String, String) to know format of the descriptor.
	///
	///  If scale or mean values are specified, a final input blob is computed
	///  as:
	/// ![block formula](https://latex.codecogs.com/png.latex?input%28n%2Cc%2Ch%2Cw%29%20%3D%20scalefactor%20%5Ctimes%20%28blob%28n%2Cc%2Ch%2Cw%29%20%2D%20mean%5Fc%29)
	///
	/// ## Note
	/// This alternative version of [NetTrait::set_input] function uses the following default values for its arguments:
	/// * name: ""
	/// * scalefactor: 1.0
	/// * mean: Scalar()
	// cv::dnn::Net::setInput(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:600
	// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_input_def(&mut self, blob: &impl ToInputArray) -> Result<()> {
		input_array_arg!(blob);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setInput_const__InputArrayR(self.as_raw_mut_Net(), blob.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the new value for the learned param of the layer.
	/// ## Parameters
	/// * layer: name or id of the layer.
	/// * numParam: index of the layer parameter in the Layer::blobs array.
	/// * blob: the new value.
	/// ## See also
	/// Layer::blobs
	///
	/// Note: If shape of the new blob differs from the previous shape,
	/// then the following forward pass may fail.
	// setParam(int, int, const Mat &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:611
	// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layer", "numParam", "blob"], ["int", "int", "const cv::Mat*"]), _)]),
	#[inline]
	fn set_param(&mut self, layer: i32, num_param: i32, blob: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setParam_int_int_const_MatR(self.as_raw_mut_Net(), layer, num_param, blob.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setParam(const String &, int, const Mat &)(InString, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:612
	// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layerName", "numParam", "blob"], ["const cv::String*", "int", "const cv::Mat*"]), _)]),
	#[inline]
	fn set_param_1(&mut self, layer_name: &str, num_param: i32, blob: &impl core::MatTraitConst) -> Result<()> {
		extern_container_arg!(layer_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_setParam_const_StringR_int_const_MatR(self.as_raw_mut_Net(), layer_name.opencv_as_extern(), num_param, blob.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Enables or disables layer fusion in the network.
	/// ## Parameters
	/// * fusion: true to enable the fusion, false to disable. The fusion is enabled by default.
	// enableFusion(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:739
	// ("cv::dnn::Net::enableFusion", vec![(pred!(mut, ["fusion"], ["bool"]), _)]),
	#[inline]
	fn enable_fusion(&mut self, fusion: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_enableFusion_bool(self.as_raw_mut_Net(), fusion, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Returns overall time for inference and timings (in ticks) for layers.
	///
	/// Indexes in returned vector correspond to layers ids. Some layers can be fused with others,
	/// in this case zero ticks count will be return for that skipped layers. Supported by DNN_BACKEND_OPENCV on DNN_TARGET_CPU only.
	///
	/// ## Parameters
	/// * timings:[out] vector for tick timings for all layers.
	/// ## Returns
	/// overall ticks for model inference.
	// getPerfProfile(std::vector<double> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:749
	// ("cv::dnn::Net::getPerfProfile", vec![(pred!(mut, ["timings"], ["std::vector<double>*"]), _)]),
	#[inline]
	fn get_perf_profile(&mut self, timings: &mut core::Vector<f64>) -> Result<i64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_getPerfProfile_vectorLdoubleGR(self.as_raw_mut_Net(), timings.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// This class allows to create and manipulate comprehensive artificial neural networks.
///
/// Neural network is presented as directed acyclic graph (DAG), where vertices are Layer instances,
/// and edges specify relationships between layers inputs and outputs.
///
/// Each network layer has unique integer id and unique string name inside its network.
/// LayerId can store either layer name or layer id.
///
/// This class supports reference counting of its instances, i. e. copies point to the same instance.
// Net /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:381
pub struct Net {
	ptr: *mut c_void,
}

opencv_type_boxed! { Net }

impl Drop for Net {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_Net_delete(self.as_raw_mut_Net()) };
	}
}

unsafe impl Send for Net {}

impl crate::dnn::NetTraitConst for Net {
	#[inline] fn as_raw_Net(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::NetTrait for Net {
	#[inline] fn as_raw_mut_Net(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Net, crate::dnn::NetTraitConst, as_raw_Net, crate::dnn::NetTrait, as_raw_mut_Net }

impl Net {
	// Net()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:385
	// ("cv::dnn::Net::Net", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_Net(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create a network from Intel's Model Optimizer intermediate representation (IR).
	/// ## Parameters
	/// * xml: XML configuration file with network's topology.
	/// * bin: Binary file with trained weights.
	/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
	/// backend.
	// readFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:394
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	pub fn read_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
		extern_container_arg!(xml);
		extern_container_arg!(bin);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_StringR_const_StringR(xml.opencv_as_extern(), bin.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create a network from Intel's Model Optimizer in-memory buffers with intermediate representation (IR).
	/// ## Parameters
	/// * bufferModelConfig: buffer with model's configuration.
	/// * bufferWeights: buffer with model's trained weights.
	/// ## Returns
	/// Net object.
	// readFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:402
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
	#[inline]
	pub fn read_from_model_optimizer_1(buffer_model_config: &core::Vector<u8>, buffer_weights: &core::Vector<u8>) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_model_config.as_raw_VectorOfu8(), buffer_weights.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Create a network from Intel's Model Optimizer in-memory buffers with intermediate representation (IR).
	/// ## Parameters
	/// * bufferModelConfigPtr: buffer pointer of model's configuration.
	/// * bufferModelConfigSize: buffer size of model's configuration.
	/// * bufferWeightsPtr: buffer pointer of model's trained weights.
	/// * bufferWeightsSize: buffer size of model's trained weights.
	/// ## Returns
	/// Net object.
	// readFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:412
	// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
	#[inline]
	pub fn read_from_model_optimizer_2(buffer_model_config_ptr: &[u8], buffer_weights_ptr: &[u8]) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr.as_ptr(), buffer_model_config_ptr.len(), buffer_weights_ptr.as_ptr(), buffer_weights_ptr.len(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl Clone for Net {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_Net_implicitClone_const(self.as_raw_Net())) }
	}
}

impl std::fmt::Debug for Net {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Net")
			.finish()
	}
}

/// Constant methods for [crate::dnn::NormalizeBBoxLayer]
// NormalizeBBoxLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:649
pub trait NormalizeBBoxLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void;

	// cv::dnn::NormalizeBBoxLayer::pnorm() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::pnorm", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pnorm(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propPnorm_const(self.as_raw_NormalizeBBoxLayer()) };
		ret
	}

	// cv::dnn::NormalizeBBoxLayer::epsilon() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::epsilon", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn epsilon(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propEpsilon_const(self.as_raw_NormalizeBBoxLayer()) };
		ret
	}

	// cv::dnn::NormalizeBBoxLayer::acrossSpatial() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:653
	// ("cv::dnn::NormalizeBBoxLayer::acrossSpatial", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn across_spatial(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const(self.as_raw_NormalizeBBoxLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::NormalizeBBoxLayer]
pub trait NormalizeBBoxLayerTrait: crate::dnn::LayerTrait + crate::dnn::NormalizeBBoxLayerTraitConst {
	fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void;

	// cv::dnn::NormalizeBBoxLayer::setPnorm(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::setPnorm", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_pnorm(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propPnorm_const_float(self.as_raw_mut_NormalizeBBoxLayer(), val) };
		ret
	}

	// cv::dnn::NormalizeBBoxLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
	// ("cv::dnn::NormalizeBBoxLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_epsilon(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propEpsilon_const_float(self.as_raw_mut_NormalizeBBoxLayer(), val) };
		ret
	}

	// cv::dnn::NormalizeBBoxLayer::setAcrossSpatial(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:653
	// ("cv::dnn::NormalizeBBoxLayer::setAcrossSpatial", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_across_spatial(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const_bool(self.as_raw_mut_NormalizeBBoxLayer(), val) };
		ret
	}

}

/// ![inline formula](https://latex.codecogs.com/png.latex?%20L%5Fp%20) - normalization layer.
/// ## Parameters
/// * p: Normalization factor. The most common `p = 1` for ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F1%20) -
///          normalization or `p = 2` for ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F2%20) - normalization or a custom one.
/// * eps: Parameter ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cepsilon%20) to prevent a division by zero.
/// * across_spatial: If true, normalize an input across all non-batch dimensions.
///                       Otherwise normalize an every channel separately.
///
/// Across spatial:
/// @f[
/// norm = \sqrt[p]{\epsilon + \sum_{x, y, c} |src(x, y, c)|^p } \\
/// dst(x, y, c) = \frac{ src(x, y, c) }{norm}
/// @f]
///
/// Channel wise normalization:
/// @f[
/// norm(c) = \sqrt[p]{\epsilon + \sum_{x, y} |src(x, y, c)|^p } \\
/// dst(x, y, c) = \frac{ src(x, y, c) }{norm(c)}
/// @f]
///
/// Where `x, y` - spatial coordinates, `c` - channel.
///
/// An every sample in the batch is normalized separately. Optionally,
/// output is scaled by the trained parameters.
// NormalizeBBoxLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:649
pub struct NormalizeBBoxLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { NormalizeBBoxLayer }

impl Drop for NormalizeBBoxLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_delete(self.as_raw_mut_NormalizeBBoxLayer()) };
	}
}

unsafe impl Send for NormalizeBBoxLayer {}

impl core::AlgorithmTraitConst for NormalizeBBoxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for NormalizeBBoxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NormalizeBBoxLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for NormalizeBBoxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for NormalizeBBoxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NormalizeBBoxLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::NormalizeBBoxLayerTraitConst for NormalizeBBoxLayer {
	#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::NormalizeBBoxLayerTrait for NormalizeBBoxLayer {
	#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { NormalizeBBoxLayer, crate::dnn::NormalizeBBoxLayerTraitConst, as_raw_NormalizeBBoxLayer, crate::dnn::NormalizeBBoxLayerTrait, as_raw_mut_NormalizeBBoxLayer }

impl NormalizeBBoxLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_NormalizeBBoxLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:655
	// ("cv::dnn::NormalizeBBoxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::NormalizeBBoxLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::NormalizeBBoxLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { NormalizeBBoxLayer, core::Algorithm, cv_dnn_NormalizeBBoxLayer_to_Algorithm }

boxed_cast_base! { NormalizeBBoxLayer, crate::dnn::Layer, cv_dnn_NormalizeBBoxLayer_to_Layer }

impl std::fmt::Debug for NormalizeBBoxLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("NormalizeBBoxLayer")
			.field("pnorm", &crate::dnn::NormalizeBBoxLayerTraitConst::pnorm(self))
			.field("epsilon", &crate::dnn::NormalizeBBoxLayerTraitConst::epsilon(self))
			.field("across_spatial", &crate::dnn::NormalizeBBoxLayerTraitConst::across_spatial(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for NormalizeBBoxLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::PaddingLayer]
// PaddingLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:417
pub trait PaddingLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PaddingLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::PaddingLayer]
pub trait PaddingLayerTrait: crate::dnn::LayerTrait + crate::dnn::PaddingLayerTraitConst {
	fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void;

}

/// Adds extra values for specific axes.
/// ## Parameters
/// * paddings: Vector of paddings in format
///                ```C++
///                [ pad_before, pad_after,  // [0]th dimension
///                   pad_before, pad_after,  // [1]st dimension
///                   ...
///                   pad_before, pad_after ] // [n]th dimension
///                ```
///
///                that represents number of padded values at every dimension
///                starting from the first one. The rest of dimensions won't
///                be padded.
/// * value: Value to be padded. Defaults to zero.
/// * type: Padding type: 'constant', 'reflect'
/// * input_dims: Torch's parameter. If @p input_dims is not equal to the
///                   actual input dimensionality then the `[0]th` dimension
///                   is considered as a batch dimension and @p paddings are shifted
///                   to a one dimension. Defaults to `-1` that means padding
///                   corresponding to @p paddings.
// PaddingLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:417
pub struct PaddingLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { PaddingLayer }

impl Drop for PaddingLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_PaddingLayer_delete(self.as_raw_mut_PaddingLayer()) };
	}
}

unsafe impl Send for PaddingLayer {}

impl core::AlgorithmTraitConst for PaddingLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PaddingLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PaddingLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for PaddingLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PaddingLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PaddingLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::PaddingLayerTraitConst for PaddingLayer {
	#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PaddingLayerTrait for PaddingLayer {
	#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PaddingLayer, crate::dnn::PaddingLayerTraitConst, as_raw_PaddingLayer, crate::dnn::PaddingLayerTrait, as_raw_mut_PaddingLayer }

impl PaddingLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_PaddingLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:420
	// ("cv::dnn::PaddingLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::PaddingLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PaddingLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PaddingLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PaddingLayer, core::Algorithm, cv_dnn_PaddingLayer_to_Algorithm }

boxed_cast_base! { PaddingLayer, crate::dnn::Layer, cv_dnn_PaddingLayer_to_Layer }

impl std::fmt::Debug for PaddingLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PaddingLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for PaddingLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::PermuteLayer]
// PermuteLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:374
pub trait PermuteLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PermuteLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::PermuteLayer]
pub trait PermuteLayerTrait: crate::dnn::LayerTrait + crate::dnn::PermuteLayerTraitConst {
	fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void;

}

// PermuteLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:374
pub struct PermuteLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { PermuteLayer }

impl Drop for PermuteLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_PermuteLayer_delete(self.as_raw_mut_PermuteLayer()) };
	}
}

unsafe impl Send for PermuteLayer {}

impl core::AlgorithmTraitConst for PermuteLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PermuteLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PermuteLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for PermuteLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PermuteLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PermuteLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::PermuteLayerTraitConst for PermuteLayer {
	#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PermuteLayerTrait for PermuteLayer {
	#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PermuteLayer, crate::dnn::PermuteLayerTraitConst, as_raw_PermuteLayer, crate::dnn::PermuteLayerTrait, as_raw_mut_PermuteLayer }

impl PermuteLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_PermuteLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:377
	// ("cv::dnn::PermuteLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::PermuteLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PermuteLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PermuteLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PermuteLayer, core::Algorithm, cv_dnn_PermuteLayer_to_Algorithm }

boxed_cast_base! { PermuteLayer, crate::dnn::Layer, cv_dnn_PermuteLayer_to_Layer }

impl std::fmt::Debug for PermuteLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PermuteLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for PermuteLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::PoolingLayer]
// PoolingLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:245
pub trait PoolingLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PoolingLayer(&self) -> *const c_void;

	// cv::dnn::PoolingLayer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:248
	// ("cv::dnn::PoolingLayer::type", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn typ(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propType_const(self.as_raw_PoolingLayer()) };
		ret
	}

	// cv::dnn::PoolingLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::kernel_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kernel_size(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propKernel_size_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::PoolingLayer::strides() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::strides", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn strides(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propStrides_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::PoolingLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::pads_begin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pads_begin(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_begin_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::PoolingLayer::pads_end() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::pads_end", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pads_end(&self) -> core::Vector<size_t> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_end_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		ret
	}

	/// Flag is true if at least one of the axes is global pooled.
	// cv::dnn::PoolingLayer::globalPooling() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:251
	// ("cv::dnn::PoolingLayer::globalPooling", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn global_pooling(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propGlobalPooling_const(self.as_raw_PoolingLayer()) };
		ret
	}

	// cv::dnn::PoolingLayer::isGlobalPooling() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:252
	// ("cv::dnn::PoolingLayer::isGlobalPooling", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_global_pooling(&self) -> core::Vector<bool> {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propIsGlobalPooling_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { core::Vector::<bool>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::PoolingLayer::computeMaxIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:253
	// ("cv::dnn::PoolingLayer::computeMaxIdx", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn compute_max_idx(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propComputeMaxIdx_const(self.as_raw_PoolingLayer()) };
		ret
	}

	// cv::dnn::PoolingLayer::padMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:254
	// ("cv::dnn::PoolingLayer::padMode", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pad_mode(&self) -> String {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPadMode_const(self.as_raw_PoolingLayer()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::PoolingLayer::ceilMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:255
	// ("cv::dnn::PoolingLayer::ceilMode", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ceil_mode(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propCeilMode_const(self.as_raw_PoolingLayer()) };
		ret
	}

	// cv::dnn::PoolingLayer::avePoolPaddedArea() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:259
	// ("cv::dnn::PoolingLayer::avePoolPaddedArea", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ave_pool_padded_area(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propAvePoolPaddedArea_const(self.as_raw_PoolingLayer()) };
		ret
	}

	// cv::dnn::PoolingLayer::pooledSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:261
	// ("cv::dnn::PoolingLayer::pooledSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pooled_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PoolingLayer_propPooledSize_const(self.as_raw_PoolingLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	// cv::dnn::PoolingLayer::spatialScale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:262
	// ("cv::dnn::PoolingLayer::spatialScale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn spatial_scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propSpatialScale_const(self.as_raw_PoolingLayer()) };
		ret
	}

	// cv::dnn::PoolingLayer::psRoiOutChannels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:264
	// ("cv::dnn::PoolingLayer::psRoiOutChannels", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn ps_roi_out_channels(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPsRoiOutChannels_const(self.as_raw_PoolingLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::PoolingLayer]
pub trait PoolingLayerTrait: crate::dnn::LayerTrait + crate::dnn::PoolingLayerTraitConst {
	fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void;

	// cv::dnn::PoolingLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:248
	// ("cv::dnn::PoolingLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propType_const_int(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}

	// cv::dnn::PoolingLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_kernel_size(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propKernel_size_const_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::PoolingLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
	// ("cv::dnn::PoolingLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_strides(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propStrides_const_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::PoolingLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_pads_begin(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_begin_const_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	// cv::dnn::PoolingLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
	// ("cv::dnn::PoolingLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
	#[inline]
	fn set_pads_end(&mut self, val: core::Vector<size_t>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_end_const_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_VectorOfsize_t()) };
		ret
	}

	/// Flag is true if at least one of the axes is global pooled.
	// cv::dnn::PoolingLayer::setGlobalPooling(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:251
	// ("cv::dnn::PoolingLayer::setGlobalPooling", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_global_pooling(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propGlobalPooling_const_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}

	// cv::dnn::PoolingLayer::setIsGlobalPooling(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:252
	// ("cv::dnn::PoolingLayer::setIsGlobalPooling", vec![(pred!(mut, ["val"], ["const std::vector<bool>"]), _)]),
	#[inline]
	fn set_is_global_pooling(&mut self, val: core::Vector<bool>) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propIsGlobalPooling_const_vectorLboolG(self.as_raw_mut_PoolingLayer(), val.as_raw_VectorOfbool()) };
		ret
	}

	// cv::dnn::PoolingLayer::setComputeMaxIdx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:253
	// ("cv::dnn::PoolingLayer::setComputeMaxIdx", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_compute_max_idx(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propComputeMaxIdx_const_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}

	// cv::dnn::PoolingLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:254
	// ("cv::dnn::PoolingLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_pad_mode(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPadMode_const_String(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern()) };
		ret
	}

	// cv::dnn::PoolingLayer::setCeilMode(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:255
	// ("cv::dnn::PoolingLayer::setCeilMode", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_ceil_mode(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propCeilMode_const_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}

	// cv::dnn::PoolingLayer::setAvePoolPaddedArea(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:259
	// ("cv::dnn::PoolingLayer::setAvePoolPaddedArea", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_ave_pool_padded_area(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propAvePoolPaddedArea_const_bool(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}

	// cv::dnn::PoolingLayer::setPooledSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:261
	// ("cv::dnn::PoolingLayer::setPooledSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_pooled_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPooledSize_const_Size(self.as_raw_mut_PoolingLayer(), &val) };
		ret
	}

	// cv::dnn::PoolingLayer::setSpatialScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:262
	// ("cv::dnn::PoolingLayer::setSpatialScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_spatial_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propSpatialScale_const_float(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}

	// cv::dnn::PoolingLayer::setPsRoiOutChannels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:264
	// ("cv::dnn::PoolingLayer::setPsRoiOutChannels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_ps_roi_out_channels(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_PoolingLayer_propPsRoiOutChannels_const_int(self.as_raw_mut_PoolingLayer(), val) };
		ret
	}

}

// PoolingLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:245
pub struct PoolingLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { PoolingLayer }

impl Drop for PoolingLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_PoolingLayer_delete(self.as_raw_mut_PoolingLayer()) };
	}
}

unsafe impl Send for PoolingLayer {}

impl core::AlgorithmTraitConst for PoolingLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PoolingLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PoolingLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for PoolingLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PoolingLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PoolingLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::PoolingLayerTraitConst for PoolingLayer {
	#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PoolingLayerTrait for PoolingLayer {
	#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PoolingLayer, crate::dnn::PoolingLayerTraitConst, as_raw_PoolingLayer, crate::dnn::PoolingLayerTrait, as_raw_mut_PoolingLayer }

impl PoolingLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_PoolingLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:266
	// ("cv::dnn::PoolingLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::PoolingLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PoolingLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PoolingLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PoolingLayer, core::Algorithm, cv_dnn_PoolingLayer_to_Algorithm }

boxed_cast_base! { PoolingLayer, crate::dnn::Layer, cv_dnn_PoolingLayer_to_Layer }

impl std::fmt::Debug for PoolingLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PoolingLayer")
			.field("typ", &crate::dnn::PoolingLayerTraitConst::typ(self))
			.field("kernel_size", &crate::dnn::PoolingLayerTraitConst::kernel_size(self))
			.field("strides", &crate::dnn::PoolingLayerTraitConst::strides(self))
			.field("pads_begin", &crate::dnn::PoolingLayerTraitConst::pads_begin(self))
			.field("pads_end", &crate::dnn::PoolingLayerTraitConst::pads_end(self))
			.field("global_pooling", &crate::dnn::PoolingLayerTraitConst::global_pooling(self))
			.field("is_global_pooling", &crate::dnn::PoolingLayerTraitConst::is_global_pooling(self))
			.field("compute_max_idx", &crate::dnn::PoolingLayerTraitConst::compute_max_idx(self))
			.field("pad_mode", &crate::dnn::PoolingLayerTraitConst::pad_mode(self))
			.field("ceil_mode", &crate::dnn::PoolingLayerTraitConst::ceil_mode(self))
			.field("ave_pool_padded_area", &crate::dnn::PoolingLayerTraitConst::ave_pool_padded_area(self))
			.field("pooled_size", &crate::dnn::PoolingLayerTraitConst::pooled_size(self))
			.field("spatial_scale", &crate::dnn::PoolingLayerTraitConst::spatial_scale(self))
			.field("ps_roi_out_channels", &crate::dnn::PoolingLayerTraitConst::ps_roi_out_channels(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for PoolingLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::PowerLayer]
// PowerLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:497
pub trait PowerLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_PowerLayer(&self) -> *const c_void;

	// cv::dnn::PowerLayer::power() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::power", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn power(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PowerLayer_propPower_const(self.as_raw_PowerLayer()) };
		ret
	}

	// cv::dnn::PowerLayer::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::scale", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PowerLayer_propScale_const(self.as_raw_PowerLayer()) };
		ret
	}

	// cv::dnn::PowerLayer::shift() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::shift", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn shift(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_PowerLayer_propShift_const(self.as_raw_PowerLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::PowerLayer]
pub trait PowerLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::PowerLayerTraitConst {
	fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void;

	// cv::dnn::PowerLayer::setPower(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::setPower", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_power(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PowerLayer_propPower_const_float(self.as_raw_mut_PowerLayer(), val) };
		ret
	}

	// cv::dnn::PowerLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PowerLayer_propScale_const_float(self.as_raw_mut_PowerLayer(), val) };
		ret
	}

	// cv::dnn::PowerLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
	// ("cv::dnn::PowerLayer::setShift", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_shift(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_PowerLayer_propShift_const_float(self.as_raw_mut_PowerLayer(), val) };
		ret
	}

}

// PowerLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:497
pub struct PowerLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { PowerLayer }

impl Drop for PowerLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_PowerLayer_delete(self.as_raw_mut_PowerLayer()) };
	}
}

unsafe impl Send for PowerLayer {}

impl crate::dnn::ActivationLayerTraitConst for PowerLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PowerLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PowerLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for PowerLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PowerLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PowerLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for PowerLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PowerLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PowerLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::PowerLayerTraitConst for PowerLayer {
	#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PowerLayerTrait for PowerLayer {
	#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PowerLayer, crate::dnn::PowerLayerTraitConst, as_raw_PowerLayer, crate::dnn::PowerLayerTrait, as_raw_mut_PowerLayer }

impl PowerLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:502
	// ("cv::dnn::PowerLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::PowerLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PowerLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PowerLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PowerLayer, crate::dnn::ActivationLayer, cv_dnn_PowerLayer_to_ActivationLayer }

boxed_cast_base! { PowerLayer, core::Algorithm, cv_dnn_PowerLayer_to_Algorithm }

boxed_cast_base! { PowerLayer, crate::dnn::Layer, cv_dnn_PowerLayer_to_Layer }

impl std::fmt::Debug for PowerLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PowerLayer")
			.field("power", &crate::dnn::PowerLayerTraitConst::power(self))
			.field("scale", &crate::dnn::PowerLayerTraitConst::scale(self))
			.field("shift", &crate::dnn::PowerLayerTraitConst::shift(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::PriorBoxLayer]
// PriorBoxLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:592
pub trait PriorBoxLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_PriorBoxLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::PriorBoxLayer]
pub trait PriorBoxLayerTrait: crate::dnn::LayerTrait + crate::dnn::PriorBoxLayerTraitConst {
	fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void;

}

// PriorBoxLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:592
pub struct PriorBoxLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { PriorBoxLayer }

impl Drop for PriorBoxLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_PriorBoxLayer_delete(self.as_raw_mut_PriorBoxLayer()) };
	}
}

unsafe impl Send for PriorBoxLayer {}

impl core::AlgorithmTraitConst for PriorBoxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for PriorBoxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PriorBoxLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for PriorBoxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for PriorBoxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PriorBoxLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::PriorBoxLayerTraitConst for PriorBoxLayer {
	#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::PriorBoxLayerTrait for PriorBoxLayer {
	#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { PriorBoxLayer, crate::dnn::PriorBoxLayerTraitConst, as_raw_PriorBoxLayer, crate::dnn::PriorBoxLayerTrait, as_raw_mut_PriorBoxLayer }

impl PriorBoxLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_PriorBoxLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:595
	// ("cv::dnn::PriorBoxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::PriorBoxLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_PriorBoxLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::PriorBoxLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { PriorBoxLayer, core::Algorithm, cv_dnn_PriorBoxLayer_to_Algorithm }

boxed_cast_base! { PriorBoxLayer, crate::dnn::Layer, cv_dnn_PriorBoxLayer_to_Layer }

impl std::fmt::Debug for PriorBoxLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PriorBoxLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for PriorBoxLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ProposalLayer]
// ProposalLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:680
pub trait ProposalLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ProposalLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::ProposalLayer]
pub trait ProposalLayerTrait: crate::dnn::LayerTrait + crate::dnn::ProposalLayerTraitConst {
	fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void;

}

// ProposalLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:680
pub struct ProposalLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ProposalLayer }

impl Drop for ProposalLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ProposalLayer_delete(self.as_raw_mut_ProposalLayer()) };
	}
}

unsafe impl Send for ProposalLayer {}

impl core::AlgorithmTraitConst for ProposalLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ProposalLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ProposalLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ProposalLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ProposalLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ProposalLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ProposalLayerTraitConst for ProposalLayer {
	#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ProposalLayerTrait for ProposalLayer {
	#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ProposalLayer, crate::dnn::ProposalLayerTraitConst, as_raw_ProposalLayer, crate::dnn::ProposalLayerTrait, as_raw_mut_ProposalLayer }

impl ProposalLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ProposalLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:683
	// ("cv::dnn::ProposalLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ProposalLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ProposalLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ProposalLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ProposalLayer, core::Algorithm, cv_dnn_ProposalLayer_to_Algorithm }

boxed_cast_base! { ProposalLayer, crate::dnn::Layer, cv_dnn_ProposalLayer_to_Layer }

impl std::fmt::Debug for ProposalLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ProposalLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ProposalLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::RNNLayer]
// RNNLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:181
pub trait RNNLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_RNNLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::RNNLayer]
pub trait RNNLayerTrait: crate::dnn::LayerTrait + crate::dnn::RNNLayerTraitConst {
	fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void;

	/// Setups learned weights.
	///
	/// Recurrent-layer behavior on each step is defined by current input @f$ x_t @f$, previous state @f$ h_t @f$ and learned weights as follows:
	/// @f{eqnarray*}{
	/// h_t &= tanh&(W_{hh} h_{t-1} + W_{xh} x_t + b_h),  \\
	/// o_t &= tanh&(W_{ho} h_t + b_o),
	/// @f}
	///
	/// ## Parameters
	/// * Wxh: is @f$ W_{xh} @f$ matrix
	/// * bh: is @f$ b_{h}  @f$ vector
	/// * Whh: is @f$ W_{hh} @f$ matrix
	/// * Who: is @f$ W_{xo} @f$ matrix
	/// * bo: is @f$ b_{o}  @f$ vector
	// setWeights(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:201
	// ("cv::dnn::RNNLayer::setWeights", vec![(pred!(mut, ["Wxh", "bh", "Whh", "Who", "bo"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn set_weights(&mut self, wxh: &impl core::MatTraitConst, bh: &impl core::MatTraitConst, whh: &impl core::MatTraitConst, who: &impl core::MatTraitConst, bo: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RNNLayer_setWeights_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR(self.as_raw_mut_RNNLayer(), wxh.as_raw_Mat(), bh.as_raw_Mat(), whh.as_raw_Mat(), who.as_raw_Mat(), bo.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	///
	/// ## C++ default parameters
	/// * produce: false
	// setProduceHiddenOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:206
	// ("cv::dnn::RNNLayer::setProduceHiddenOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
	#[inline]
	fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput_bool(self.as_raw_mut_RNNLayer(), produce, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	///
	/// ## Note
	/// This alternative version of [RNNLayerTrait::set_produce_hidden_output] function uses the following default values for its arguments:
	/// * produce: false
	// cv::dnn::RNNLayer::setProduceHiddenOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:206
	// ("cv::dnn::RNNLayer::setProduceHiddenOutput", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn set_produce_hidden_output_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput(self.as_raw_mut_RNNLayer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Classical recurrent layer
///
/// Accepts two inputs @f$x_t@f$ and @f$h_{t-1}@f$ and compute two outputs @f$o_t@f$ and @f$h_t@f$.
///
/// - input: should contain packed input @f$x_t@f$.
/// - output: should contain output @f$o_t@f$ (and @f$h_t@f$ if setProduceHiddenOutput() is set to true).
///
/// input[0] should have shape [`T`, `N`, `data_dims`] where `T` and `N` is number of timestamps and number of independent samples of @f$x_t@f$ respectively.
///
/// output[0] will have shape [`T`, `N`, @f$N_o@f$], where @f$N_o@f$ is number of rows in @f$ W_{xo} @f$ matrix.
///
/// If setProduceHiddenOutput() is set to true then @p output[1] will contain a Mat with shape [`T`, `N`, @f$N_h@f$], where @f$N_h@f$ is number of rows in @f$ W_{hh} @f$ matrix.
// RNNLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:181
pub struct RNNLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { RNNLayer }

impl Drop for RNNLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_RNNLayer_delete(self.as_raw_mut_RNNLayer()) };
	}
}

unsafe impl Send for RNNLayer {}

impl core::AlgorithmTraitConst for RNNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RNNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RNNLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for RNNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for RNNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RNNLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::RNNLayerTraitConst for RNNLayer {
	#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::RNNLayerTrait for RNNLayer {
	#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RNNLayer, crate::dnn::RNNLayerTraitConst, as_raw_RNNLayer, crate::dnn::RNNLayerTrait, as_raw_mut_RNNLayer }

impl RNNLayer {
	/// Creates instance of RNNLayer
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:185
	// ("cv::dnn::RNNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::RNNLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RNNLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::RNNLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { RNNLayer, core::Algorithm, cv_dnn_RNNLayer_to_Algorithm }

boxed_cast_base! { RNNLayer, crate::dnn::Layer, cv_dnn_RNNLayer_to_Layer }

impl std::fmt::Debug for RNNLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RNNLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::ReLU6Layer]
// ReLU6Layer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:439
pub trait ReLU6LayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ReLU6Layer(&self) -> *const c_void;

	// cv::dnn::ReLU6Layer::minValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::ReLU6Layer::minValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn min_value(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMinValue_const(self.as_raw_ReLU6Layer()) };
		ret
	}

	// cv::dnn::ReLU6Layer::maxValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::ReLU6Layer::maxValue", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_value(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMaxValue_const(self.as_raw_ReLU6Layer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ReLU6Layer]
pub trait ReLU6LayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReLU6LayerTraitConst {
	fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void;

	// cv::dnn::ReLU6Layer::setMinValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::ReLU6Layer::setMinValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_min_value(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMinValue_const_float(self.as_raw_mut_ReLU6Layer(), val) };
		ret
	}

	// cv::dnn::ReLU6Layer::setMaxValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
	// ("cv::dnn::ReLU6Layer::setMaxValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_max_value(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMaxValue_const_float(self.as_raw_mut_ReLU6Layer(), val) };
		ret
	}

}

// ReLU6Layer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:439
pub struct ReLU6Layer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ReLU6Layer }

impl Drop for ReLU6Layer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ReLU6Layer_delete(self.as_raw_mut_ReLU6Layer()) };
	}
}

unsafe impl Send for ReLU6Layer {}

impl crate::dnn::ActivationLayerTraitConst for ReLU6Layer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLU6Layer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for ReLU6Layer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLU6Layer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ReLU6Layer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLU6Layer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ReLU6LayerTraitConst for ReLU6Layer {
	#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReLU6LayerTrait for ReLU6Layer {
	#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLU6Layer, crate::dnn::ReLU6LayerTraitConst, as_raw_ReLU6Layer, crate::dnn::ReLU6LayerTrait, as_raw_mut_ReLU6Layer }

impl ReLU6Layer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:444
	// ("cv::dnn::ReLU6Layer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ReLU6Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReLU6Layer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReLU6Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ReLU6Layer, crate::dnn::ActivationLayer, cv_dnn_ReLU6Layer_to_ActivationLayer }

boxed_cast_base! { ReLU6Layer, core::Algorithm, cv_dnn_ReLU6Layer_to_Algorithm }

boxed_cast_base! { ReLU6Layer, crate::dnn::Layer, cv_dnn_ReLU6Layer_to_Layer }

impl std::fmt::Debug for ReLU6Layer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ReLU6Layer")
			.field("min_value", &crate::dnn::ReLU6LayerTraitConst::min_value(self))
			.field("max_value", &crate::dnn::ReLU6LayerTraitConst::max_value(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::ReLULayer]
// ReLULayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:431
pub trait ReLULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_ReLULayer(&self) -> *const c_void;

	// cv::dnn::ReLULayer::negativeSlope() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:434
	// ("cv::dnn::ReLULayer::negativeSlope", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn negative_slope(&self) -> f32 {
		let ret = unsafe { sys::cv_dnn_ReLULayer_propNegativeSlope_const(self.as_raw_ReLULayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ReLULayer]
pub trait ReLULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReLULayerTraitConst {
	fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void;

	// cv::dnn::ReLULayer::setNegativeSlope(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:434
	// ("cv::dnn::ReLULayer::setNegativeSlope", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_negative_slope(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dnn_ReLULayer_propNegativeSlope_const_float(self.as_raw_mut_ReLULayer(), val) };
		ret
	}

}

// ReLULayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:431
pub struct ReLULayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ReLULayer }

impl Drop for ReLULayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ReLULayer_delete(self.as_raw_mut_ReLULayer()) };
	}
}

unsafe impl Send for ReLULayer {}

impl crate::dnn::ActivationLayerTraitConst for ReLULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for ReLULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLULayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for ReLULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReLULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLULayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ReLULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReLULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLULayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ReLULayerTraitConst for ReLULayer {
	#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReLULayerTrait for ReLULayer {
	#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReLULayer, crate::dnn::ReLULayerTraitConst, as_raw_ReLULayer, crate::dnn::ReLULayerTrait, as_raw_mut_ReLULayer }

impl ReLULayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:436
	// ("cv::dnn::ReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ReLULayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReLULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReLULayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ReLULayer, crate::dnn::ActivationLayer, cv_dnn_ReLULayer_to_ActivationLayer }

boxed_cast_base! { ReLULayer, core::Algorithm, cv_dnn_ReLULayer_to_Algorithm }

boxed_cast_base! { ReLULayer, crate::dnn::Layer, cv_dnn_ReLULayer_to_Layer }

impl std::fmt::Debug for ReLULayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ReLULayer")
			.field("negative_slope", &crate::dnn::ReLULayerTraitConst::negative_slope(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::RegionLayer]
// RegionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:604
pub trait RegionLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_RegionLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::RegionLayer]
pub trait RegionLayerTrait: crate::dnn::LayerTrait + crate::dnn::RegionLayerTraitConst {
	fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void;

}

// RegionLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:604
pub struct RegionLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { RegionLayer }

impl Drop for RegionLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_RegionLayer_delete(self.as_raw_mut_RegionLayer()) };
	}
}

unsafe impl Send for RegionLayer {}

impl core::AlgorithmTraitConst for RegionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for RegionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RegionLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for RegionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for RegionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RegionLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::RegionLayerTraitConst for RegionLayer {
	#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::RegionLayerTrait for RegionLayer {
	#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { RegionLayer, crate::dnn::RegionLayerTraitConst, as_raw_RegionLayer, crate::dnn::RegionLayerTrait, as_raw_mut_RegionLayer }

impl RegionLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_RegionLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:607
	// ("cv::dnn::RegionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::RegionLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_RegionLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::RegionLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { RegionLayer, core::Algorithm, cv_dnn_RegionLayer_to_Algorithm }

boxed_cast_base! { RegionLayer, crate::dnn::Layer, cv_dnn_RegionLayer_to_Layer }

impl std::fmt::Debug for RegionLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("RegionLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for RegionLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ReorgLayer]
// ReorgLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:598
pub trait ReorgLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ReorgLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::ReorgLayer]
pub trait ReorgLayerTrait: crate::dnn::LayerTrait + crate::dnn::ReorgLayerTraitConst {
	fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void;

}

// ReorgLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:598
pub struct ReorgLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ReorgLayer }

impl Drop for ReorgLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ReorgLayer_delete(self.as_raw_mut_ReorgLayer()) };
	}
}

unsafe impl Send for ReorgLayer {}

impl core::AlgorithmTraitConst for ReorgLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReorgLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReorgLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ReorgLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReorgLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReorgLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ReorgLayerTraitConst for ReorgLayer {
	#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReorgLayerTrait for ReorgLayer {
	#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReorgLayer, crate::dnn::ReorgLayerTraitConst, as_raw_ReorgLayer, crate::dnn::ReorgLayerTrait, as_raw_mut_ReorgLayer }

impl ReorgLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ReorgLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:601
	// ("cv::dnn::ReorgLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ReorgLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReorgLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReorgLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ReorgLayer, core::Algorithm, cv_dnn_ReorgLayer_to_Algorithm }

boxed_cast_base! { ReorgLayer, crate::dnn::Layer, cv_dnn_ReorgLayer_to_Layer }

impl std::fmt::Debug for ReorgLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ReorgLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ReorgLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ReshapeLayer]
// ReshapeLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:295
pub trait ReshapeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ReshapeLayer(&self) -> *const c_void;

	// cv::dnn::ReshapeLayer::newShapeDesc() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:298
	// ("cv::dnn::ReshapeLayer::newShapeDesc", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn new_shape_desc(&self) -> crate::dnn::MatShape {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_propNewShapeDesc_const(self.as_raw_ReshapeLayer()) };
		let ret = unsafe { crate::dnn::MatShape::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::ReshapeLayer::newShapeRange() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:299
	// ("cv::dnn::ReshapeLayer::newShapeRange", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn new_shape_range(&self) -> core::Range {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_propNewShapeRange_const(self.as_raw_ReshapeLayer()) };
		let ret = unsafe { core::Range::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ReshapeLayer]
pub trait ReshapeLayerTrait: crate::dnn::LayerTrait + crate::dnn::ReshapeLayerTraitConst {
	fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void;

	// cv::dnn::ReshapeLayer::setNewShapeDesc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:298
	// ("cv::dnn::ReshapeLayer::setNewShapeDesc", vec![(pred!(mut, ["val"], ["const cv::dnn::MatShape"]), _)]),
	#[inline]
	fn set_new_shape_desc(&mut self, val: crate::dnn::MatShape) {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_propNewShapeDesc_const_MatShape(self.as_raw_mut_ReshapeLayer(), val.as_raw_VectorOfi32()) };
		ret
	}

	// cv::dnn::ReshapeLayer::setNewShapeRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:299
	// ("cv::dnn::ReshapeLayer::setNewShapeRange", vec![(pred!(mut, ["val"], ["const cv::Range"]), _)]),
	#[inline]
	fn set_new_shape_range(&mut self, val: core::Range) {
		let ret = unsafe { sys::cv_dnn_ReshapeLayer_propNewShapeRange_const_Range(self.as_raw_mut_ReshapeLayer(), val.as_raw_Range()) };
		ret
	}

}

// ReshapeLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:295
pub struct ReshapeLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ReshapeLayer }

impl Drop for ReshapeLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ReshapeLayer_delete(self.as_raw_mut_ReshapeLayer()) };
	}
}

unsafe impl Send for ReshapeLayer {}

impl core::AlgorithmTraitConst for ReshapeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ReshapeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReshapeLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ReshapeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ReshapeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReshapeLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ReshapeLayerTraitConst for ReshapeLayer {
	#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ReshapeLayerTrait for ReshapeLayer {
	#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ReshapeLayer, crate::dnn::ReshapeLayerTraitConst, as_raw_ReshapeLayer, crate::dnn::ReshapeLayerTrait, as_raw_mut_ReshapeLayer }

impl ReshapeLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ReshapeLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:301
	// ("cv::dnn::ReshapeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ReshapeLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ReshapeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ReshapeLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ReshapeLayer, core::Algorithm, cv_dnn_ReshapeLayer_to_Algorithm }

boxed_cast_base! { ReshapeLayer, crate::dnn::Layer, cv_dnn_ReshapeLayer_to_Layer }

impl std::fmt::Debug for ReshapeLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ReshapeLayer")
			.field("new_shape_desc", &crate::dnn::ReshapeLayerTraitConst::new_shape_desc(self))
			.field("new_shape_range", &crate::dnn::ReshapeLayerTraitConst::new_shape_range(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ReshapeLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ResizeLayer]
// ResizeLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:663
pub trait ResizeLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ResizeLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::ResizeLayer]
pub trait ResizeLayerTrait: crate::dnn::LayerTrait + crate::dnn::ResizeLayerTraitConst {
	fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void;

}

/// Resize input 4-dimensional blob by nearest neighbor or bilinear strategy.
///
/// Layer is used to support TensorFlow's resize_nearest_neighbor and resize_bilinear ops.
// ResizeLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:663
pub struct ResizeLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ResizeLayer }

impl Drop for ResizeLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ResizeLayer_delete(self.as_raw_mut_ResizeLayer()) };
	}
}

unsafe impl Send for ResizeLayer {}

impl core::AlgorithmTraitConst for ResizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ResizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ResizeLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ResizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ResizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ResizeLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ResizeLayerTraitConst for ResizeLayer {
	#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ResizeLayerTrait for ResizeLayer {
	#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ResizeLayer, crate::dnn::ResizeLayerTraitConst, as_raw_ResizeLayer, crate::dnn::ResizeLayerTrait, as_raw_mut_ResizeLayer }

impl ResizeLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ResizeLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:666
	// ("cv::dnn::ResizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ResizeLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ResizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ResizeLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ResizeLayer, core::Algorithm, cv_dnn_ResizeLayer_to_Algorithm }

boxed_cast_base! { ResizeLayer, crate::dnn::Layer, cv_dnn_ResizeLayer_to_Layer }

impl std::fmt::Debug for ResizeLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ResizeLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ResizeLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ScaleLayer]
// ScaleLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:553
pub trait ScaleLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ScaleLayer(&self) -> *const c_void;

	// cv::dnn::ScaleLayer::hasBias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:556
	// ("cv::dnn::ScaleLayer::hasBias", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn has_bias(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_propHasBias_const(self.as_raw_ScaleLayer()) };
		ret
	}

	// cv::dnn::ScaleLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:557
	// ("cv::dnn::ScaleLayer::axis", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_propAxis_const(self.as_raw_ScaleLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ScaleLayer]
pub trait ScaleLayerTrait: crate::dnn::LayerTrait + crate::dnn::ScaleLayerTraitConst {
	fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void;

	// cv::dnn::ScaleLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:556
	// ("cv::dnn::ScaleLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_has_bias(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_propHasBias_const_bool(self.as_raw_mut_ScaleLayer(), val) };
		ret
	}

	// cv::dnn::ScaleLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:557
	// ("cv::dnn::ScaleLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ScaleLayer_propAxis_const_int(self.as_raw_mut_ScaleLayer(), val) };
		ret
	}

}

// ScaleLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:553
pub struct ScaleLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ScaleLayer }

impl Drop for ScaleLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ScaleLayer_delete(self.as_raw_mut_ScaleLayer()) };
	}
}

unsafe impl Send for ScaleLayer {}

impl core::AlgorithmTraitConst for ScaleLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ScaleLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ScaleLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ScaleLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ScaleLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ScaleLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ScaleLayerTraitConst for ScaleLayer {
	#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ScaleLayerTrait for ScaleLayer {
	#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ScaleLayer, crate::dnn::ScaleLayerTraitConst, as_raw_ScaleLayer, crate::dnn::ScaleLayerTrait, as_raw_mut_ScaleLayer }

impl ScaleLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ScaleLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:559
	// ("cv::dnn::ScaleLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::ScaleLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ScaleLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::ScaleLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ScaleLayer, core::Algorithm, cv_dnn_ScaleLayer_to_Algorithm }

boxed_cast_base! { ScaleLayer, crate::dnn::Layer, cv_dnn_ScaleLayer_to_Layer }

impl std::fmt::Debug for ScaleLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ScaleLayer")
			.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
			.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ScaleLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ShiftLayer]
// ShiftLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:562
pub trait ShiftLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ShiftLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::ShiftLayer]
pub trait ShiftLayerTrait: crate::dnn::LayerTrait + crate::dnn::ShiftLayerTraitConst {
	fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void;

}

// ShiftLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:562
pub struct ShiftLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ShiftLayer }

impl Drop for ShiftLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ShiftLayer_delete(self.as_raw_mut_ShiftLayer()) };
	}
}

unsafe impl Send for ShiftLayer {}

impl core::AlgorithmTraitConst for ShiftLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ShiftLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ShiftLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ShiftLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ShiftLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ShiftLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ShiftLayerTraitConst for ShiftLayer {
	#[inline] fn as_raw_ShiftLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ShiftLayerTrait for ShiftLayer {
	#[inline] fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ShiftLayer, crate::dnn::ShiftLayerTraitConst, as_raw_ShiftLayer, crate::dnn::ShiftLayerTrait, as_raw_mut_ShiftLayer }

impl ShiftLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ShiftLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:565
	// ("cv::dnn::ShiftLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ShiftLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ShiftLayer, core::Algorithm, cv_dnn_ShiftLayer_to_Algorithm }

boxed_cast_base! { ShiftLayer, crate::dnn::Layer, cv_dnn_ShiftLayer_to_Layer }

impl std::fmt::Debug for ShiftLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ShiftLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ShiftLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::ShuffleChannelLayer]
// ShuffleChannelLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:389
pub trait ShuffleChannelLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_ShuffleChannelLayer(&self) -> *const c_void;

	// cv::dnn::ShuffleChannelLayer::group() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:394
	// ("cv::dnn::ShuffleChannelLayer::group", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn group(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_ShuffleChannelLayer_propGroup_const(self.as_raw_ShuffleChannelLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::ShuffleChannelLayer]
pub trait ShuffleChannelLayerTrait: crate::dnn::LayerTrait + crate::dnn::ShuffleChannelLayerTraitConst {
	fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void;

	// cv::dnn::ShuffleChannelLayer::setGroup(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:394
	// ("cv::dnn::ShuffleChannelLayer::setGroup", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_group(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_ShuffleChannelLayer_propGroup_const_int(self.as_raw_mut_ShuffleChannelLayer(), val) };
		ret
	}

}

/// Permute channels of 4-dimensional input blob.
/// ## Parameters
/// * group: Number of groups to split input channels and pick in turns
///              into output blob.
///
/// ![block formula](https://latex.codecogs.com/png.latex?%20groupSize%20%3D%20%5Cfrac%7Bnumber%5C%20of%5C%20channels%7D%7Bgroup%7D%20)
/// ![block formula](https://latex.codecogs.com/png.latex?%20output%28n%2C%20c%2C%20h%2C%20w%29%20%3D%20input%28n%2C%20groupSize%20%5Ctimes%20%28c%20%5C%25%20group%29%20%2B%20%5Clfloor%20%5Cfrac%7Bc%7D%7Bgroup%7D%20%5Crfloor%2C%20h%2C%20w%29%20)
/// Read more at <https://arxiv.org/pdf/1707.01083.pdf>
// ShuffleChannelLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:389
pub struct ShuffleChannelLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { ShuffleChannelLayer }

impl Drop for ShuffleChannelLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_ShuffleChannelLayer_delete(self.as_raw_mut_ShuffleChannelLayer()) };
	}
}

unsafe impl Send for ShuffleChannelLayer {}

impl core::AlgorithmTraitConst for ShuffleChannelLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ShuffleChannelLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ShuffleChannelLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for ShuffleChannelLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for ShuffleChannelLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ShuffleChannelLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::ShuffleChannelLayerTraitConst for ShuffleChannelLayer {
	#[inline] fn as_raw_ShuffleChannelLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ShuffleChannelLayerTrait for ShuffleChannelLayer {
	#[inline] fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ShuffleChannelLayer, crate::dnn::ShuffleChannelLayerTraitConst, as_raw_ShuffleChannelLayer, crate::dnn::ShuffleChannelLayerTrait, as_raw_mut_ShuffleChannelLayer }

impl ShuffleChannelLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_ShuffleChannelLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:392
	// ("cv::dnn::ShuffleChannelLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::Layer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_ShuffleChannelLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ShuffleChannelLayer, core::Algorithm, cv_dnn_ShuffleChannelLayer_to_Algorithm }

boxed_cast_base! { ShuffleChannelLayer, crate::dnn::Layer, cv_dnn_ShuffleChannelLayer_to_Layer }

impl std::fmt::Debug for ShuffleChannelLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ShuffleChannelLayer")
			.field("group", &crate::dnn::ShuffleChannelLayerTraitConst::group(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for ShuffleChannelLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::SigmoidLayer]
// SigmoidLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:479
pub trait SigmoidLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SigmoidLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::SigmoidLayer]
pub trait SigmoidLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SigmoidLayerTraitConst {
	fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void;

}

// SigmoidLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:479
pub struct SigmoidLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { SigmoidLayer }

impl Drop for SigmoidLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_SigmoidLayer_delete(self.as_raw_mut_SigmoidLayer()) };
	}
}

unsafe impl Send for SigmoidLayer {}

impl crate::dnn::ActivationLayerTraitConst for SigmoidLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SigmoidLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for SigmoidLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SigmoidLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for SigmoidLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SigmoidLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::SigmoidLayerTraitConst for SigmoidLayer {
	#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SigmoidLayerTrait for SigmoidLayer {
	#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SigmoidLayer, crate::dnn::SigmoidLayerTraitConst, as_raw_SigmoidLayer, crate::dnn::SigmoidLayerTrait, as_raw_mut_SigmoidLayer }

impl SigmoidLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:482
	// ("cv::dnn::SigmoidLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::SigmoidLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SigmoidLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SigmoidLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SigmoidLayer, crate::dnn::ActivationLayer, cv_dnn_SigmoidLayer_to_ActivationLayer }

boxed_cast_base! { SigmoidLayer, core::Algorithm, cv_dnn_SigmoidLayer_to_Algorithm }

boxed_cast_base! { SigmoidLayer, crate::dnn::Layer, cv_dnn_SigmoidLayer_to_Layer }

impl std::fmt::Debug for SigmoidLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SigmoidLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::SliceLayer]
// SliceLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:357
pub trait SliceLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_SliceLayer(&self) -> *const c_void;

	/// Vector of slice ranges.
	///
	/// The first dimension equals number of output blobs.
	/// Inner vector has slice ranges for the first number of input dimensions.
	// cv::dnn::SliceLayer::sliceRanges() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::SliceLayer::sliceRanges", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn slice_ranges(&self) -> core::Vector<core::Vector<core::Range>> {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceRanges_const(self.as_raw_SliceLayer()) };
		let ret = unsafe { core::Vector::<core::Vector<core::Range>>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::SliceLayer::sliceSteps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::SliceLayer::sliceSteps", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn slice_steps(&self) -> core::Vector<core::Vector<i32>> {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceSteps_const(self.as_raw_SliceLayer()) };
		let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
		ret
	}

	// cv::dnn::SliceLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:368
	// ("cv::dnn::SliceLayer::axis", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn axis(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propAxis_const(self.as_raw_SliceLayer()) };
		ret
	}

	// cv::dnn::SliceLayer::num_split() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:369
	// ("cv::dnn::SliceLayer::num_split", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_split(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propNum_split_const(self.as_raw_SliceLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::SliceLayer]
pub trait SliceLayerTrait: crate::dnn::LayerTrait + crate::dnn::SliceLayerTraitConst {
	fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void;

	/// Vector of slice ranges.
	///
	/// The first dimension equals number of output blobs.
	/// Inner vector has slice ranges for the first number of input dimensions.
	// cv::dnn::SliceLayer::setSliceRanges(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:366
	// ("cv::dnn::SliceLayer::setSliceRanges", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Range>>"]), _)]),
	#[inline]
	fn set_slice_ranges(&mut self, val: core::Vector<core::Vector<core::Range>>) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceRanges_const_vectorLvectorLRangeGG(self.as_raw_mut_SliceLayer(), val.as_raw_VectorOfVectorOfRange()) };
		ret
	}

	// cv::dnn::SliceLayer::setSliceSteps(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:367
	// ("cv::dnn::SliceLayer::setSliceSteps", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
	#[inline]
	fn set_slice_steps(&mut self, val: core::Vector<core::Vector<i32>>) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceSteps_const_vectorLvectorLintGG(self.as_raw_mut_SliceLayer(), val.as_raw_VectorOfVectorOfi32()) };
		ret
	}

	// cv::dnn::SliceLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:368
	// ("cv::dnn::SliceLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_axis(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propAxis_const_int(self.as_raw_mut_SliceLayer(), val) };
		ret
	}

	// cv::dnn::SliceLayer::setNum_split(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:369
	// ("cv::dnn::SliceLayer::setNum_split", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_split(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_SliceLayer_propNum_split_const_int(self.as_raw_mut_SliceLayer(), val) };
		ret
	}

}

/// Slice layer has several modes:
/// 1. Caffe mode
/// ## Parameters
/// * axis: Axis of split operation
/// * slice_point: Array of split points
///
/// Number of output blobs equals to number of split points plus one. The
/// first blob is a slice on input from 0 to @p slice_point[0] - 1 by @p axis,
/// the second output blob is a slice of input from @p slice_point[0] to
/// @p slice_point[1] - 1 by @p axis and the last output blob is a slice of
/// input from @p slice_point[-1] up to the end of @p axis size.
///
/// 2. TensorFlow mode
/// * begin: Vector of start indices
/// * size: Vector of sizes
///
/// More convenient numpy-like slice. One and only output blob
/// is a slice `input[begin[0]:begin[0]+size[0], begin[1]:begin[1]+size[1], ...]`
///
/// 3. Torch mode
/// * axis: Axis of split operation
///
/// Split input blob on the equal parts by @p axis.
// SliceLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:357
pub struct SliceLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { SliceLayer }

impl Drop for SliceLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_SliceLayer_delete(self.as_raw_mut_SliceLayer()) };
	}
}

unsafe impl Send for SliceLayer {}

impl core::AlgorithmTraitConst for SliceLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SliceLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SliceLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for SliceLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SliceLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SliceLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::SliceLayerTraitConst for SliceLayer {
	#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SliceLayerTrait for SliceLayer {
	#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SliceLayer, crate::dnn::SliceLayerTraitConst, as_raw_SliceLayer, crate::dnn::SliceLayerTrait, as_raw_mut_SliceLayer }

impl SliceLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_SliceLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:371
	// ("cv::dnn::SliceLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::SliceLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SliceLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SliceLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SliceLayer, core::Algorithm, cv_dnn_SliceLayer_to_Algorithm }

boxed_cast_base! { SliceLayer, crate::dnn::Layer, cv_dnn_SliceLayer_to_Layer }

impl std::fmt::Debug for SliceLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SliceLayer")
			.field("slice_ranges", &crate::dnn::SliceLayerTraitConst::slice_ranges(self))
			.field("slice_steps", &crate::dnn::SliceLayerTraitConst::slice_steps(self))
			.field("axis", &crate::dnn::SliceLayerTraitConst::axis(self))
			.field("num_split", &crate::dnn::SliceLayerTraitConst::num_split(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for SliceLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::SoftmaxLayer]
// SoftmaxLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:269
pub trait SoftmaxLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_SoftmaxLayer(&self) -> *const c_void;

	// cv::dnn::SoftmaxLayer::logSoftMax() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:272
	// ("cv::dnn::SoftmaxLayer::logSoftMax", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn log_soft_max(&self) -> bool {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayer_propLogSoftMax_const(self.as_raw_SoftmaxLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::SoftmaxLayer]
pub trait SoftmaxLayerTrait: crate::dnn::LayerTrait + crate::dnn::SoftmaxLayerTraitConst {
	fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void;

	// cv::dnn::SoftmaxLayer::setLogSoftMax(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:272
	// ("cv::dnn::SoftmaxLayer::setLogSoftMax", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_log_soft_max(&mut self, val: bool) {
		let ret = unsafe { sys::cv_dnn_SoftmaxLayer_propLogSoftMax_const_bool(self.as_raw_mut_SoftmaxLayer(), val) };
		ret
	}

}

// SoftmaxLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:269
pub struct SoftmaxLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { SoftmaxLayer }

impl Drop for SoftmaxLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_SoftmaxLayer_delete(self.as_raw_mut_SoftmaxLayer()) };
	}
}

unsafe impl Send for SoftmaxLayer {}

impl core::AlgorithmTraitConst for SoftmaxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SoftmaxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SoftmaxLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for SoftmaxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SoftmaxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SoftmaxLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::SoftmaxLayerTraitConst for SoftmaxLayer {
	#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SoftmaxLayerTrait for SoftmaxLayer {
	#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SoftmaxLayer, crate::dnn::SoftmaxLayerTraitConst, as_raw_SoftmaxLayer, crate::dnn::SoftmaxLayerTrait, as_raw_mut_SoftmaxLayer }

impl SoftmaxLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_SoftmaxLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:274
	// ("cv::dnn::SoftmaxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::SoftmaxLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SoftmaxLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SoftmaxLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SoftmaxLayer, core::Algorithm, cv_dnn_SoftmaxLayer_to_Algorithm }

boxed_cast_base! { SoftmaxLayer, crate::dnn::Layer, cv_dnn_SoftmaxLayer_to_Layer }

impl std::fmt::Debug for SoftmaxLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SoftmaxLayer")
			.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for SoftmaxLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::SplitLayer]
// SplitLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:325
pub trait SplitLayerTraitConst: crate::dnn::LayerTraitConst {
	fn as_raw_SplitLayer(&self) -> *const c_void;

	/// Number of copies that will be produced (is ignored when negative).
	// cv::dnn::SplitLayer::outputsCount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:328
	// ("cv::dnn::SplitLayer::outputsCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn outputs_count(&self) -> i32 {
		let ret = unsafe { sys::cv_dnn_SplitLayer_propOutputsCount_const(self.as_raw_SplitLayer()) };
		ret
	}

}

/// Mutable methods for [crate::dnn::SplitLayer]
pub trait SplitLayerTrait: crate::dnn::LayerTrait + crate::dnn::SplitLayerTraitConst {
	fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void;

	/// Number of copies that will be produced (is ignored when negative).
	// cv::dnn::SplitLayer::setOutputsCount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:328
	// ("cv::dnn::SplitLayer::setOutputsCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_outputs_count(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dnn_SplitLayer_propOutputsCount_const_int(self.as_raw_mut_SplitLayer(), val) };
		ret
	}

}

// SplitLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:325
pub struct SplitLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { SplitLayer }

impl Drop for SplitLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_SplitLayer_delete(self.as_raw_mut_SplitLayer()) };
	}
}

unsafe impl Send for SplitLayer {}

impl core::AlgorithmTraitConst for SplitLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SplitLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SplitLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for SplitLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SplitLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SplitLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::SplitLayerTraitConst for SplitLayer {
	#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SplitLayerTrait for SplitLayer {
	#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SplitLayer, crate::dnn::SplitLayerTraitConst, as_raw_SplitLayer, crate::dnn::SplitLayerTrait, as_raw_mut_SplitLayer }

impl SplitLayer {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_dnn_SplitLayer_defaultNew_const()) }
	}

	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:330
	// ("cv::dnn::SplitLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::SplitLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SplitLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SplitLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SplitLayer, core::Algorithm, cv_dnn_SplitLayer_to_Algorithm }

boxed_cast_base! { SplitLayer, crate::dnn::Layer, cv_dnn_SplitLayer_to_Layer }

impl std::fmt::Debug for SplitLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SplitLayer")
			.field("outputs_count", &crate::dnn::SplitLayerTraitConst::outputs_count(self))
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

impl Default for SplitLayer {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::dnn::SwishLayer]
// SwishLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:467
pub trait SwishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_SwishLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::SwishLayer]
pub trait SwishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SwishLayerTraitConst {
	fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void;

}

// SwishLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:467
pub struct SwishLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { SwishLayer }

impl Drop for SwishLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_SwishLayer_delete(self.as_raw_mut_SwishLayer()) };
	}
}

unsafe impl Send for SwishLayer {}

impl crate::dnn::ActivationLayerTraitConst for SwishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for SwishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SwishLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for SwishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SwishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SwishLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for SwishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for SwishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SwishLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::SwishLayerTraitConst for SwishLayer {
	#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::SwishLayerTrait for SwishLayer {
	#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { SwishLayer, crate::dnn::SwishLayerTraitConst, as_raw_SwishLayer, crate::dnn::SwishLayerTrait, as_raw_mut_SwishLayer }

impl SwishLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:470
	// ("cv::dnn::SwishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::SwishLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_SwishLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::SwishLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { SwishLayer, crate::dnn::ActivationLayer, cv_dnn_SwishLayer_to_ActivationLayer }

boxed_cast_base! { SwishLayer, core::Algorithm, cv_dnn_SwishLayer_to_Algorithm }

boxed_cast_base! { SwishLayer, crate::dnn::Layer, cv_dnn_SwishLayer_to_Layer }

impl std::fmt::Debug for SwishLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SwishLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::TanHLayer]
// TanHLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:461
pub trait TanHLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
	fn as_raw_TanHLayer(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::TanHLayer]
pub trait TanHLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::TanHLayerTraitConst {
	fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void;

}

// TanHLayer /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:461
pub struct TanHLayer {
	ptr: *mut c_void,
}

opencv_type_boxed! { TanHLayer }

impl Drop for TanHLayer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn_TanHLayer_delete(self.as_raw_mut_TanHLayer()) };
	}
}

unsafe impl Send for TanHLayer {}

impl crate::dnn::ActivationLayerTraitConst for TanHLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::ActivationLayerTrait for TanHLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TanHLayer, crate::dnn::ActivationLayerTraitConst, as_raw_ActivationLayer, crate::dnn::ActivationLayerTrait, as_raw_mut_ActivationLayer }

impl core::AlgorithmTraitConst for TanHLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TanHLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TanHLayer, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::dnn::LayerTraitConst for TanHLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::LayerTrait for TanHLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TanHLayer, crate::dnn::LayerTraitConst, as_raw_Layer, crate::dnn::LayerTrait, as_raw_mut_Layer }

impl crate::dnn::TanHLayerTraitConst for TanHLayer {
	#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::TanHLayerTrait for TanHLayer {
	#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TanHLayer, crate::dnn::TanHLayerTraitConst, as_raw_TanHLayer, crate::dnn::TanHLayerTrait, as_raw_mut_TanHLayer }

impl TanHLayer {
	// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:464
	// ("cv::dnn::TanHLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
	#[inline]
	pub fn create(params: &impl crate::dnn::LayerParamsTraitConst) -> Result<core::Ptr<crate::dnn::TanHLayer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_TanHLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::dnn::TanHLayer>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TanHLayer, crate::dnn::ActivationLayer, cv_dnn_TanHLayer_to_ActivationLayer }

boxed_cast_base! { TanHLayer, core::Algorithm, cv_dnn_TanHLayer_to_Algorithm }

boxed_cast_base! { TanHLayer, crate::dnn::Layer, cv_dnn_TanHLayer_to_Layer }

impl std::fmt::Debug for TanHLayer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TanHLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

/// Constant methods for [crate::dnn::_Range]
// _Range /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:57
pub trait _RangeTraitConst: core::RangeTraitConst {
	fn as_raw__Range(&self) -> *const c_void;

}

/// Mutable methods for [crate::dnn::_Range]
pub trait _RangeTrait: core::RangeTrait + crate::dnn::_RangeTraitConst {
	fn as_raw_mut__Range(&mut self) -> *mut c_void;

}

// _Range /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:57
pub struct _Range {
	ptr: *mut c_void,
}

opencv_type_boxed! { _Range }

impl Drop for _Range {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_dnn__Range_delete(self.as_raw_mut__Range()) };
	}
}

unsafe impl Send for _Range {}

impl core::RangeTraitConst for _Range {
	#[inline] fn as_raw_Range(&self) -> *const c_void { self.as_raw() }
}

impl core::RangeTrait for _Range {
	#[inline] fn as_raw_mut_Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { _Range, core::RangeTraitConst, as_raw_Range, core::RangeTrait, as_raw_mut_Range }

impl crate::dnn::_RangeTraitConst for _Range {
	#[inline] fn as_raw__Range(&self) -> *const c_void { self.as_raw() }
}

impl crate::dnn::_RangeTrait for _Range {
	#[inline] fn as_raw_mut__Range(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { _Range, crate::dnn::_RangeTraitConst, as_raw__Range, crate::dnn::_RangeTrait, as_raw_mut__Range }

impl _Range {
	// _Range(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:59
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
	#[inline]
	pub fn from_base(r: &impl core::RangeTraitConst) -> Result<crate::dnn::_Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn__Range__Range_const_RangeR(r.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::_Range::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * size_: 1
	// _Range(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:60
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["start_", "size_"], ["int", "int"]), _)]),
	#[inline]
	pub fn new(start_: i32, size_: i32) -> Result<crate::dnn::_Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn__Range__Range_int_int(start_, size_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::_Range::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * size_: 1
	// cv::dnn::_Range::_Range(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:60
	// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["start_"], ["int"]), _)]),
	#[inline]
	pub fn new_def(start_: i32) -> Result<crate::dnn::_Range> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn__Range__Range_int(start_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::_Range::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { _Range, core::Range, cv_dnn__Range_to_Range }

impl std::fmt::Debug for _Range {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("_Range")
			.field("start", &core::RangeTraitConst::start(self))
			.field("end", &core::RangeTraitConst::end(self))
			.finish()
	}
}
