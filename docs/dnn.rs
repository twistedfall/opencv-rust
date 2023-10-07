pub mod dnn {
	//! # Deep Neural Network module
	//!   This module contains:
	//!       - API for new layers creation, layers are building bricks of neural networks;
	//!       - set of built-in most-useful Layers;
	//!       - API to construct and modify comprehensive neural networks from layers;
	//!       - functionality for loading serialized networks models from different frameworks.
	//! 
	//!   Functionality of this module is designed only for forward pass computations (i.e. network testing).
	//!   A network training is in principle not supported.
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::DictValueTraitConst, super::DictValueTrait, super::DictTraitConst, super::DictTrait, super::LayerParamsTraitConst, super::LayerParamsTrait, super::BackendNodeTraitConst, super::BackendNodeTrait, super::BackendWrapperTraitConst, super::BackendWrapperTrait, super::LayerTraitConst, super::LayerTrait, super::NetTraitConst, super::NetTrait, super::ModelTraitConst, super::ModelTrait, super::ClassificationModelTraitConst, super::ClassificationModelTrait, super::KeypointsModelTraitConst, super::KeypointsModelTrait, super::SegmentationModelTraitConst, super::SegmentationModelTrait, super::DetectionModelTraitConst, super::DetectionModelTrait, super::TextRecognitionModelTraitConst, super::TextRecognitionModelTrait, super::TextDetectionModelTraitConst, super::TextDetectionModelTrait, super::TextDetectionModel_EASTTraitConst, super::TextDetectionModel_EASTTrait, super::TextDetectionModel_DBTraitConst, super::TextDetectionModel_DBTrait, super::LayerFactoryTraitConst, super::LayerFactoryTrait, super::BlankLayerTraitConst, super::BlankLayerTrait, super::ConstLayerTraitConst, super::ConstLayerTrait, super::LSTMLayerTraitConst, super::LSTMLayerTrait, super::GRULayerTraitConst, super::GRULayerTrait, super::RNNLayerTraitConst, super::RNNLayerTrait, super::BaseConvolutionLayerTraitConst, super::BaseConvolutionLayerTrait, super::ConvolutionLayerTraitConst, super::ConvolutionLayerTrait, super::ConvolutionLayerInt8TraitConst, super::ConvolutionLayerInt8Trait, super::DeconvolutionLayerTraitConst, super::DeconvolutionLayerTrait, super::LRNLayerTraitConst, super::LRNLayerTrait, super::ArgLayerTraitConst, super::ArgLayerTrait, super::GatherLayerTraitConst, super::GatherLayerTrait, super::PoolingLayerTraitConst, super::PoolingLayerTrait, super::PoolingLayerInt8TraitConst, super::PoolingLayerInt8Trait, super::ReduceLayerTraitConst, super::ReduceLayerTrait, super::SoftmaxLayerTraitConst, super::SoftmaxLayerTrait, super::SoftmaxLayerInt8TraitConst, super::SoftmaxLayerInt8Trait, super::InnerProductLayerTraitConst, super::InnerProductLayerTrait, super::InnerProductLayerInt8TraitConst, super::InnerProductLayerInt8Trait, super::MVNLayerTraitConst, super::MVNLayerTrait, super::ReshapeLayerTraitConst, super::ReshapeLayerTrait, super::FlattenLayerTraitConst, super::FlattenLayerTrait, super::QuantizeLayerTraitConst, super::QuantizeLayerTrait, super::DequantizeLayerTraitConst, super::DequantizeLayerTrait, super::RequantizeLayerTraitConst, super::RequantizeLayerTrait, super::ConcatLayerTraitConst, super::ConcatLayerTrait, super::SplitLayerTraitConst, super::SplitLayerTrait, super::SliceLayerTraitConst, super::SliceLayerTrait, super::PermuteLayerTraitConst, super::PermuteLayerTrait, super::ShuffleChannelLayerTraitConst, super::ShuffleChannelLayerTrait, super::PaddingLayerTraitConst, super::PaddingLayerTrait, super::ActivationLayerTraitConst, super::ActivationLayerTrait, super::ReLULayerTraitConst, super::ReLULayerTrait, super::ReLU6LayerTraitConst, super::ReLU6LayerTrait, super::ChannelsPReLULayerTraitConst, super::ChannelsPReLULayerTrait, super::ELULayerTraitConst, super::ELULayerTrait, super::TanHLayerTraitConst, super::TanHLayerTrait, super::SwishLayerTraitConst, super::SwishLayerTrait, super::MishLayerTraitConst, super::MishLayerTrait, super::SigmoidLayerTraitConst, super::SigmoidLayerTrait, super::BNLLLayerTraitConst, super::BNLLLayerTrait, super::AbsLayerTraitConst, super::AbsLayerTrait, super::PowerLayerTraitConst, super::PowerLayerTrait, super::ExpLayerTraitConst, super::ExpLayerTrait, super::CeilLayerTraitConst, super::CeilLayerTrait, super::FloorLayerTraitConst, super::FloorLayerTrait, super::LogLayerTraitConst, super::LogLayerTrait, super::RoundLayerTraitConst, super::RoundLayerTrait, super::SqrtLayerTraitConst, super::SqrtLayerTrait, super::NotLayerTraitConst, super::NotLayerTrait, super::AcosLayerTraitConst, super::AcosLayerTrait, super::AcoshLayerTraitConst, super::AcoshLayerTrait, super::AsinLayerTraitConst, super::AsinLayerTrait, super::AsinhLayerTraitConst, super::AsinhLayerTrait, super::AtanLayerTraitConst, super::AtanLayerTrait, super::AtanhLayerTraitConst, super::AtanhLayerTrait, super::CosLayerTraitConst, super::CosLayerTrait, super::CoshLayerTraitConst, super::CoshLayerTrait, super::ErfLayerTraitConst, super::ErfLayerTrait, super::HardSwishLayerTraitConst, super::HardSwishLayerTrait, super::SinLayerTraitConst, super::SinLayerTrait, super::SinhLayerTraitConst, super::SinhLayerTrait, super::SoftplusLayerTraitConst, super::SoftplusLayerTrait, super::SoftsignLayerTraitConst, super::SoftsignLayerTrait, super::TanLayerTraitConst, super::TanLayerTrait, super::CeluLayerTraitConst, super::CeluLayerTrait, super::HardSigmoidLayerTraitConst, super::HardSigmoidLayerTrait, super::SeluLayerTraitConst, super::SeluLayerTrait, super::GeluLayerTraitConst, super::GeluLayerTrait, super::GeluApproximationLayerTraitConst, super::GeluApproximationLayerTrait, super::ThresholdedReluLayerTraitConst, super::ThresholdedReluLayerTrait, super::ActivationLayerInt8TraitConst, super::ActivationLayerInt8Trait, super::SignLayerTraitConst, super::SignLayerTrait, super::ShrinkLayerTraitConst, super::ShrinkLayerTrait, super::ReciprocalLayerTraitConst, super::ReciprocalLayerTrait, super::CropLayerTraitConst, super::CropLayerTrait, super::EltwiseLayerTraitConst, super::EltwiseLayerTrait, super::EltwiseLayerInt8TraitConst, super::EltwiseLayerInt8Trait, super::NaryEltwiseLayerTraitConst, super::NaryEltwiseLayerTrait, super::BatchNormLayerTraitConst, super::BatchNormLayerTrait, super::BatchNormLayerInt8TraitConst, super::BatchNormLayerInt8Trait, super::MaxUnpoolLayerTraitConst, super::MaxUnpoolLayerTrait, super::ScaleLayerTraitConst, super::ScaleLayerTrait, super::ScaleLayerInt8TraitConst, super::ScaleLayerInt8Trait, super::ShiftLayerTraitConst, super::ShiftLayerTrait, super::ShiftLayerInt8TraitConst, super::ShiftLayerInt8Trait, super::CompareLayerTraitConst, super::CompareLayerTrait, super::DataAugmentationLayerTraitConst, super::DataAugmentationLayerTrait, super::CorrelationLayerTraitConst, super::CorrelationLayerTrait, super::AccumLayerTraitConst, super::AccumLayerTrait, super::FlowWarpLayerTraitConst, super::FlowWarpLayerTrait, super::PriorBoxLayerTraitConst, super::PriorBoxLayerTrait, super::ReorgLayerTraitConst, super::ReorgLayerTrait, super::RegionLayerTraitConst, super::RegionLayerTrait, super::DetectionOutputLayerTraitConst, super::DetectionOutputLayerTrait, super::NormalizeBBoxLayerTraitConst, super::NormalizeBBoxLayerTrait, super::ResizeLayerTraitConst, super::ResizeLayerTrait, super::InterpLayerTraitConst, super::InterpLayerTrait, super::ProposalLayerTraitConst, super::ProposalLayerTrait, super::CropAndResizeLayerTraitConst, super::CropAndResizeLayerTrait, super::CumSumLayerTraitConst, super::CumSumLayerTrait, super::ScatterLayerTraitConst, super::ScatterLayerTrait, super::ScatterNDLayerTraitConst, super::ScatterNDLayerTrait, super::TileLayerTraitConst, super::TileLayerTrait, super::LayerNormLayerTraitConst, super::LayerNormLayerTrait, super::_RangeTraitConst, super::_RangeTrait };
	}
	
	pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NGRAPH: &str = "NGRAPH";
	pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_API: &str = "NN_BUILDER";
	pub const CV_DNN_INFERENCE_ENGINE_CPU_TYPE_ARM_COMPUTE: &str = "ARM_COMPUTE";
	pub const CV_DNN_INFERENCE_ENGINE_CPU_TYPE_X86: &str = "X86";
	pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_2: &str = "Myriad2";
	pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_X: &str = "MyriadX";
	pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_UNSPECIFIED: &str = "";
	pub const DNN_BACKEND_CANN: i32 = 8;
	pub const DNN_BACKEND_CUDA: i32 = 5;
	/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
	/// OpenCV is built with Intel OpenVINO or
	/// DNN_BACKEND_OPENCV otherwise.
	pub const DNN_BACKEND_DEFAULT: i32 = 0;
	/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
	/// OpenCV is built with Intel OpenVINO or
	/// DNN_BACKEND_OPENCV otherwise.
	pub const DNN_BACKEND_HALIDE: i32 = 1;
	/// Intel OpenVINO computational backend
	/// 
	/// Note: Tutorial how to build OpenCV with OpenVINO: [tutorial_dnn_openvino]
	pub const DNN_BACKEND_INFERENCE_ENGINE: i32 = 2;
	pub const DNN_BACKEND_OPENCV: i32 = 3;
	pub const DNN_BACKEND_TIMVX: i32 = 7;
	pub const DNN_BACKEND_VKCOM: i32 = 4;
	pub const DNN_BACKEND_WEBNN: i32 = 6;
	/// OpenCV data layout for 5D data.
	pub const DNN_LAYOUT_NCDHW: i32 = 3;
	/// OpenCV data layout for 4D data.
	pub const DNN_LAYOUT_NCHW: i32 = 2;
	/// OpenCV data layout for 2D data.
	pub const DNN_LAYOUT_ND: i32 = 1;
	/// Tensorflow-like data layout for 5D data.
	pub const DNN_LAYOUT_NDHWC: i32 = 5;
	/// Tensorflow-like data layout for 4D data.
	pub const DNN_LAYOUT_NHWC: i32 = 4;
	/// Tensorflow-like data layout, it should only be used at tf or tflite model parsing.
	pub const DNN_LAYOUT_PLANAR: i32 = 6;
	pub const DNN_LAYOUT_UNKNOWN: i32 = 0;
	pub const DNN_PMODE_CROP_CENTER: i32 = 1;
	pub const DNN_PMODE_LETTERBOX: i32 = 2;
	pub const DNN_PMODE_NULL: i32 = 0;
	pub const DNN_TARGET_CPU: i32 = 0;
	pub const DNN_TARGET_CPU_FP16: i32 = 10;
	pub const DNN_TARGET_CUDA: i32 = 6;
	pub const DNN_TARGET_CUDA_FP16: i32 = 7;
	/// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
	pub const DNN_TARGET_FPGA: i32 = 5;
	pub const DNN_TARGET_HDDL: i32 = 8;
	pub const DNN_TARGET_MYRIAD: i32 = 3;
	pub const DNN_TARGET_NPU: i32 = 9;
	pub const DNN_TARGET_OPENCL: i32 = 1;
	pub const DNN_TARGET_OPENCL_FP16: i32 = 2;
	pub const DNN_TARGET_VULKAN: i32 = 4;
	pub const OPENCV_DNN_API_VERSION: i32 = 20230620;
	pub const SoftNMSMethod_SOFTNMS_GAUSSIAN: i32 = 2;
	pub const SoftNMSMethod_SOFTNMS_LINEAR: i32 = 1;
	/// Enum of computation backends supported by layers.
	/// ## See also
	/// Net::setPreferableBackend
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Backend {
		/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
		/// OpenCV is built with Intel OpenVINO or
		/// DNN_BACKEND_OPENCV otherwise.
		DNN_BACKEND_DEFAULT = 0,
		/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
		/// OpenCV is built with Intel OpenVINO or
		/// DNN_BACKEND_OPENCV otherwise.
		DNN_BACKEND_HALIDE = 1,
		/// Intel OpenVINO computational backend
		/// 
		/// Note: Tutorial how to build OpenCV with OpenVINO: [tutorial_dnn_openvino]
		DNN_BACKEND_INFERENCE_ENGINE = 2,
		DNN_BACKEND_OPENCV = 3,
		DNN_BACKEND_VKCOM = 4,
		DNN_BACKEND_CUDA = 5,
		DNN_BACKEND_WEBNN = 6,
		DNN_BACKEND_TIMVX = 7,
		DNN_BACKEND_CANN = 8,
	}
	
	opencv_type_enum! { crate::dnn::Backend }
	
	/// Enum of data layout for model inference.
	/// ## See also
	/// Image2BlobParams
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum DataLayout {
		DNN_LAYOUT_UNKNOWN = 0,
		/// OpenCV data layout for 2D data.
		DNN_LAYOUT_ND = 1,
		/// OpenCV data layout for 4D data.
		DNN_LAYOUT_NCHW = 2,
		/// OpenCV data layout for 5D data.
		DNN_LAYOUT_NCDHW = 3,
		/// Tensorflow-like data layout for 4D data.
		DNN_LAYOUT_NHWC = 4,
		/// Tensorflow-like data layout for 5D data.
		DNN_LAYOUT_NDHWC = 5,
		/// Tensorflow-like data layout, it should only be used at tf or tflite model parsing.
		DNN_LAYOUT_PLANAR = 6,
	}
	
	opencv_type_enum! { crate::dnn::DataLayout }
	
	/// Enum of image processing mode.
	/// To facilitate the specialization pre-processing requirements of the dnn model.
	/// For example, the `letter box` often used in the Yolo series of models.
	/// ## See also
	/// Image2BlobParams
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ImagePaddingMode {
		DNN_PMODE_NULL = 0,
		DNN_PMODE_CROP_CENTER = 1,
		DNN_PMODE_LETTERBOX = 2,
	}
	
	opencv_type_enum! { crate::dnn::ImagePaddingMode }
	
	/// Enum of Soft NMS methods.
	/// ## See also
	/// softNMSBoxes
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum SoftNMSMethod {
		SOFTNMS_LINEAR = 1,
		SOFTNMS_GAUSSIAN = 2,
	}
	
	opencv_type_enum! { crate::dnn::SoftNMSMethod }
	
	/// Enum of target devices for computations.
	/// ## See also
	/// Net::setPreferableTarget
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Target {
		DNN_TARGET_CPU = 0,
		DNN_TARGET_OPENCL = 1,
		DNN_TARGET_OPENCL_FP16 = 2,
		DNN_TARGET_MYRIAD = 3,
		DNN_TARGET_VULKAN = 4,
		/// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
		DNN_TARGET_FPGA = 5,
		DNN_TARGET_CUDA = 6,
		DNN_TARGET_CUDA_FP16 = 7,
		DNN_TARGET_HDDL = 8,
		DNN_TARGET_NPU = 9,
		DNN_TARGET_CPU_FP16 = 10,
	}
	
	opencv_type_enum! { crate::dnn::Target }
	
	/// Each Layer class must provide this function to the factory
	pub type LayerFactory_Constructor = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;
	pub type MatShape = core::Vector<i32>;
	/// Container for strings and integers.
	/// 
	/// 
	/// **Deprecated**: Use getLayerId() with int result.
	#[deprecated = "Use getLayerId() with int result."]
	pub type Net_LayerId = crate::dnn::DictValue;
	/// ## Note
	/// This alternative version of [nms_boxes_batched_1] function uses the following default values for its arguments:
	/// * eta: 1.f
	/// * top_k: 0
	#[inline]
	pub fn nms_boxes_batched_1_def(bboxes: &core::Vector<core::Rect2d>, scores: &core::Vector<f32>, class_ids: &core::Vector<i32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NMSBoxesBatched_const_vectorLRect2dGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOff32(), class_ids.as_raw_VectorOfi32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * eta: 1.f
	/// * top_k: 0
	#[inline]
	pub fn nms_boxes_batched_1(bboxes: &core::Vector<core::Rect2d>, scores: &core::Vector<f32>, class_ids: &core::Vector<i32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NMSBoxesBatched_const_vectorLRect2dGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOff32(), class_ids.as_raw_VectorOfi32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs batched non maximum suppression on given boxes and corresponding scores across different classes.
	/// 
	/// ## Parameters
	/// * bboxes: a set of bounding boxes to apply NMS.
	/// * scores: a set of corresponding confidences.
	/// * class_ids: a set of corresponding class ids. Ids are integer and usually start from 0.
	/// * score_threshold: a threshold used to filter boxes by score.
	/// * nms_threshold: a threshold used in non maximum suppression.
	/// * indices: the kept indices of bboxes after NMS.
	/// * eta: a coefficient in adaptive threshold formula: ![inline formula](https://latex.codecogs.com/png.latex?nms%5C%5Fthreshold%5F%7Bi%2B1%7D%3Deta%5Ccdot%20nms%5C%5Fthreshold%5Fi).
	/// * top_k: if `>0`, keep at most @p top_k picked indices.
	/// 
	/// ## Note
	/// This alternative version of [nms_boxes_batched] function uses the following default values for its arguments:
	/// * eta: 1.f
	/// * top_k: 0
	#[inline]
	pub fn nms_boxes_batched_def(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, class_ids: &core::Vector<i32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NMSBoxesBatched_const_vectorLRectGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), class_ids.as_raw_VectorOfi32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs batched non maximum suppression on given boxes and corresponding scores across different classes.
	/// 
	/// ## Parameters
	/// * bboxes: a set of bounding boxes to apply NMS.
	/// * scores: a set of corresponding confidences.
	/// * class_ids: a set of corresponding class ids. Ids are integer and usually start from 0.
	/// * score_threshold: a threshold used to filter boxes by score.
	/// * nms_threshold: a threshold used in non maximum suppression.
	/// * indices: the kept indices of bboxes after NMS.
	/// * eta: a coefficient in adaptive threshold formula: ![inline formula](https://latex.codecogs.com/png.latex?nms%5C%5Fthreshold%5F%7Bi%2B1%7D%3Deta%5Ccdot%20nms%5C%5Fthreshold%5Fi).
	/// * top_k: if `>0`, keep at most @p top_k picked indices.
	/// 
	/// ## C++ default parameters
	/// * eta: 1.f
	/// * top_k: 0
	#[inline]
	pub fn nms_boxes_batched(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, class_ids: &core::Vector<i32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NMSBoxesBatched_const_vectorLRectGR_const_vectorLfloatGR_const_vectorLintGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), class_ids.as_raw_VectorOfi32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [nms_boxes_f64] function uses the following default values for its arguments:
	/// * eta: 1.f
	/// * top_k: 0
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
	#[inline]
	pub fn nms_boxes_rotated(bboxes: &core::Vector<core::RotatedRect>, scores: &core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, eta: f32, top_k: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes.as_raw_VectorOfRotatedRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), eta, top_k, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from image with given params.
	/// 
	/// @details This function is an extension of [blobFromImage] to meet more image preprocess needs.
	/// Given input image and preprocessing parameters, and function outputs the blob.
	/// 
	/// ## Parameters
	/// * image: input image (all with 1-, 3- or 4-channels).
	/// * param: struct of Image2BlobParams, contains all parameters needed by processing of image to blob.
	/// ## Returns
	/// 4-dimensional Mat.
	/// 
	/// ## Note
	/// This alternative version of [blob_from_image_with_params] function uses the following default values for its arguments:
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_image_with_params_def(image: &impl core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImageWithParams_const__InputArrayR(image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from image with given params.
	/// 
	/// @details This function is an extension of [blobFromImage] to meet more image preprocess needs.
	/// Given input image and preprocessing parameters, and function outputs the blob.
	/// 
	/// ## Parameters
	/// * image: input image (all with 1-, 3- or 4-channels).
	/// * param: struct of Image2BlobParams, contains all parameters needed by processing of image to blob.
	/// ## Returns
	/// 4-dimensional Mat.
	/// 
	/// ## C++ default parameters
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_image_with_params(image: &impl core::ToInputArray, param: crate::dnn::Image2BlobParams) -> Result<core::Mat> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImageWithParams_const__InputArrayR_const_Image2BlobParamsR(image.as_raw__InputArray(), &param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [blob_from_image_with_params_1] function uses the following default values for its arguments:
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_image_with_params_1_def(image: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(blob);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImageWithParams_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), blob.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from image with given params.
	/// 
	/// @details This function is an extension of [blobFromImage] to meet more image preprocess needs.
	/// Given input image and preprocessing parameters, and function outputs the blob.
	/// 
	/// ## Parameters
	/// * image: input image (all with 1-, 3- or 4-channels).
	/// * param: struct of Image2BlobParams, contains all parameters needed by processing of image to blob.
	/// ## Returns
	/// 4-dimensional Mat.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_image_with_params_1(image: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray, param: crate::dnn::Image2BlobParams) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(blob);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImageWithParams_const__InputArrayR_const__OutputArrayR_const_Image2BlobParamsR(image.as_raw__InputArray(), blob.as_raw__OutputArray(), &param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from image. Optionally resizes and crops @p image from center,
	///  subtract @p mean values, scales values by @p scalefactor, swap Blue and Red channels.
	/// ## Parameters
	/// * image: input image (with 1-, 3- or 4-channels).
	/// * scalefactor: multiplier for @p images values.
	/// * size: spatial size for output image
	/// * mean: scalar with mean values which are subtracted from channels. Values are intended
	///  to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
	/// * swapRB: flag which indicates that swap first and last channels
	///  in 3-channel image is necessary.
	/// * crop: flag which indicates whether image will be cropped after resize or not
	/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
	///  @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
	///  dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
	///  If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
	/// ## Returns
	/// 4-dimensional Mat with NCHW dimensions order.
	/// 
	/// 
	/// Note:
	/// The order and usage of `scalefactor` and `mean` are (input - mean) * scalefactor.
	/// 
	/// ## Note
	/// This alternative version of [blob_from_image] function uses the following default values for its arguments:
	/// * scalefactor: 1.0
	/// * size: Size()
	/// * mean: Scalar()
	/// * swap_rb: false
	/// * crop: false
	/// * ddepth: CV_32F
	#[inline]
	pub fn blob_from_image_def(image: &impl core::ToInputArray) -> Result<core::Mat> {
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
	#[inline]
	pub fn blob_from_image_to_def(image: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn blob_from_image_to(image: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(blob);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from image. Optionally resizes and crops @p image from center,
	///  subtract @p mean values, scales values by @p scalefactor, swap Blue and Red channels.
	/// ## Parameters
	/// * image: input image (with 1-, 3- or 4-channels).
	/// * scalefactor: multiplier for @p images values.
	/// * size: spatial size for output image
	/// * mean: scalar with mean values which are subtracted from channels. Values are intended
	///  to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
	/// * swapRB: flag which indicates that swap first and last channels
	///  in 3-channel image is necessary.
	/// * crop: flag which indicates whether image will be cropped after resize or not
	/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
	///  @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
	///  dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
	///  If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
	/// ## Returns
	/// 4-dimensional Mat with NCHW dimensions order.
	/// 
	/// 
	/// Note:
	/// The order and usage of `scalefactor` and `mean` are (input - mean) * scalefactor.
	/// 
	/// ## C++ default parameters
	/// * scalefactor: 1.0
	/// * size: Size()
	/// * mean: Scalar()
	/// * swap_rb: false
	/// * crop: false
	/// * ddepth: CV_32F
	#[inline]
	pub fn blob_from_image(image: &impl core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image.as_raw__InputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from series of images with given params.
	/// 
	/// @details This function is an extension of [blobFromImages] to meet more image preprocess needs.
	/// Given input image and preprocessing parameters, and function outputs the blob.
	/// 
	/// ## Parameters
	/// * images: input image (all with 1-, 3- or 4-channels).
	/// * param: struct of Image2BlobParams, contains all parameters needed by processing of image to blob.
	/// ## Returns
	/// 4-dimensional Mat.
	/// 
	/// ## Note
	/// This alternative version of [blob_from_images_with_params] function uses the following default values for its arguments:
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_images_with_params_def(images: &impl core::ToInputArray) -> Result<core::Mat> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImagesWithParams_const__InputArrayR(images.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from series of images with given params.
	/// 
	/// @details This function is an extension of [blobFromImages] to meet more image preprocess needs.
	/// Given input image and preprocessing parameters, and function outputs the blob.
	/// 
	/// ## Parameters
	/// * images: input image (all with 1-, 3- or 4-channels).
	/// * param: struct of Image2BlobParams, contains all parameters needed by processing of image to blob.
	/// ## Returns
	/// 4-dimensional Mat.
	/// 
	/// ## C++ default parameters
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_images_with_params(images: &impl core::ToInputArray, param: crate::dnn::Image2BlobParams) -> Result<core::Mat> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImagesWithParams_const__InputArrayR_const_Image2BlobParamsR(images.as_raw__InputArray(), &param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [blob_from_images_with_params_1] function uses the following default values for its arguments:
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_images_with_params_1_def(images: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(blob);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImagesWithParams_const__InputArrayR_const__OutputArrayR(images.as_raw__InputArray(), blob.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from series of images with given params.
	/// 
	/// @details This function is an extension of [blobFromImages] to meet more image preprocess needs.
	/// Given input image and preprocessing parameters, and function outputs the blob.
	/// 
	/// ## Parameters
	/// * images: input image (all with 1-, 3- or 4-channels).
	/// * param: struct of Image2BlobParams, contains all parameters needed by processing of image to blob.
	/// ## Returns
	/// 4-dimensional Mat.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * param: Image2BlobParams()
	#[inline]
	pub fn blob_from_images_with_params_1(images: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray, param: crate::dnn::Image2BlobParams) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(blob);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImagesWithParams_const__InputArrayR_const__OutputArrayR_const_Image2BlobParamsR(images.as_raw__InputArray(), blob.as_raw__OutputArray(), &param, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from series of images. Optionally resizes and
	///  crops @p images from center, subtract @p mean values, scales values by @p scalefactor,
	///  swap Blue and Red channels.
	/// ## Parameters
	/// * images: input images (all with 1-, 3- or 4-channels).
	/// * size: spatial size for output image
	/// * mean: scalar with mean values which are subtracted from channels. Values are intended
	///  to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
	/// * scalefactor: multiplier for @p images values.
	/// * swapRB: flag which indicates that swap first and last channels
	///  in 3-channel image is necessary.
	/// * crop: flag which indicates whether image will be cropped after resize or not
	/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
	///  @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
	///  dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
	///  If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
	/// ## Returns
	/// 4-dimensional Mat with NCHW dimensions order.
	/// 
	/// 
	/// Note:
	/// The order and usage of `scalefactor` and `mean` are (input - mean) * scalefactor.
	/// 
	/// ## Note
	/// This alternative version of [blob_from_images] function uses the following default values for its arguments:
	/// * scalefactor: 1.0
	/// * size: Size()
	/// * mean: Scalar()
	/// * swap_rb: false
	/// * crop: false
	/// * ddepth: CV_32F
	#[inline]
	pub fn blob_from_images_def(images: &impl core::ToInputArray) -> Result<core::Mat> {
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
	#[inline]
	pub fn blob_from_images_to_def(images: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray) -> Result<()> {
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
	#[inline]
	pub fn blob_from_images_to(images: &impl core::ToInputArray, blob: &mut impl core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(blob);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(images.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, size.opencv_as_extern(), &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates 4-dimensional blob from series of images. Optionally resizes and
	///  crops @p images from center, subtract @p mean values, scales values by @p scalefactor,
	///  swap Blue and Red channels.
	/// ## Parameters
	/// * images: input images (all with 1-, 3- or 4-channels).
	/// * size: spatial size for output image
	/// * mean: scalar with mean values which are subtracted from channels. Values are intended
	///  to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
	/// * scalefactor: multiplier for @p images values.
	/// * swapRB: flag which indicates that swap first and last channels
	///  in 3-channel image is necessary.
	/// * crop: flag which indicates whether image will be cropped after resize or not
	/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
	///  @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
	///  dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
	///  If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
	/// ## Returns
	/// 4-dimensional Mat with NCHW dimensions order.
	/// 
	/// 
	/// Note:
	/// The order and usage of `scalefactor` and `mean` are (input - mean) * scalefactor.
	/// 
	/// ## C++ default parameters
	/// * scalefactor: 1.0
	/// * size: Size()
	/// * mean: Scalar()
	/// * swap_rb: false
	/// * crop: false
	/// * ddepth: CV_32F
	#[inline]
	pub fn blob_from_images(images: &impl core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
		input_array_arg!(images);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(images.as_raw__InputArray(), scalefactor, size.opencv_as_extern(), &mean, swap_rb, crop, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn concat(a: &crate::dnn::MatShape, b: &crate::dnn::MatShape) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_concat_const_MatShapeR_const_MatShapeR(a.as_raw_VectorOfi32(), b.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Enables detailed logging of the DNN model loading with CV DNN API.
	/// ## Parameters
	/// * isDiagnosticsMode: Indicates whether diagnostic mode should be set.
	/// 
	/// Diagnostic mode provides detailed logging of the model loading stage to explore
	/// potential problems (ex.: not implemented layer type).
	/// 
	/// 
	/// Note: In diagnostic mode series of assertions will be skipped, it can lead to the
	/// expected application crash.
	#[inline]
	pub fn enable_model_diagnostics(is_diagnostics_mode: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_enableModelDiagnostics_bool(is_diagnostics_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_available_backends() -> Result<core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_getAvailableBackends(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
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
	#[inline]
	pub fn get_inference_engine_vpu_type() -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_getInferenceEngineVPUType(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn get_plane(m: &core::Mat, n: i32, cn: i32) -> Result<core::Mat> {
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
	#[inline]
	pub fn images_from_blob(blob_: &core::Mat, images_: &mut impl core::ToOutputArray) -> Result<()> {
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
	/// 
	/// ## Note
	/// This alternative version of [read_net_from_darknet] function uses the following default values for its arguments:
	/// * darknet_model: String()
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
	/// 
	/// ## C++ default parameters
	/// * darknet_model: String()
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
	#[inline]
	pub fn read_net_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size, ocvrs_return.as_mut_ptr()) };
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
	#[inline]
	pub fn read_net_from_onnx_buffer(buffer: &core::Vector<u8>) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_readNetFromONNX_const_vectorLunsigned_charGR(buffer.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Reads a network model stored in <a href="https://www.tensorflow.org/lite">TFLite</a> framework's format.
	/// ## Parameters
	/// * model: path to the .tflite file with binary flatbuffers description of the network architecture
	/// ## Returns
	/// Net object.
	#[inline]
	pub fn read_net_from_tf_lite(model: &str) -> Result<crate::dnn::Net> {
		extern_container_arg!(model);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_readNetFromTFLite_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Reads a network model stored in <a href="https://www.tensorflow.org/lite">TFLite</a> framework's format.
	/// @details This is an overloaded member function, provided for convenience.
	/// It differs from the above function only in what argument(s) it accepts.
	/// ## Parameters
	/// * bufferModel: buffer containing the content of the tflite file
	/// * lenModel: length of bufferModel
	#[inline]
	pub fn read_net_from_tf_lite_2(buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
		extern_container_arg!(buffer_model);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_readNetFromTFLite_const_charX_size_t(buffer_model.opencv_as_extern(), len_model, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Reads a network model stored in <a href="https://www.tensorflow.org/lite">TFLite</a> framework's format.
	/// ## Parameters
	/// * bufferModel: buffer containing the content of the tflite file
	/// ## Returns
	/// Net object.
	#[inline]
	pub fn read_net_from_tf_lite_1(buffer_model: &core::Vector<u8>) -> Result<crate::dnn::Net> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_readNetFromTFLite_const_vectorLunsigned_charGR(buffer_model.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
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
	
	/// Release a HDDL plugin.
	#[inline]
	pub fn release_hddl_plugin() -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_releaseHDDLPlugin(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Release a Myriad device (binded by OpenCV).
	/// 
	/// Single Myriad device cannot be shared across multiple processes which uses
	/// Inference Engine's Myriad plugin.
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
	
	#[inline]
	pub fn shape_1(mat: &core::Mat) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_shape_const_MatR(mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn shape_2(sz: &core::MatSize) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_shape_const_MatSizeR(sz.as_raw_MatSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn shape_3(mat: &core::UMat) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_shape_const_UMatR(mat.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn shape(dims: &i32, n: i32) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_shape_const_intX_const_int(dims, n, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [shape_4] function uses the following default values for its arguments:
	/// * a1: -1
	/// * a2: -1
	/// * a3: -1
	#[inline]
	pub fn shape_4_def(a0: i32) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_shape_int(a0, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * a1: -1
	/// * a2: -1
	/// * a3: -1
	#[inline]
	pub fn shape_4(a0: i32, a1: i32, a2: i32, a3: i32) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_shape_int_int_int_int(a0, a1, a2, a3, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
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
	
	#[inline]
	pub fn slice(m: &core::Mat, r0: &crate::dnn::_Range) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn slice_1(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn slice_2(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range, r2: &crate::dnn::_Range) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn slice_3(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range, r2: &crate::dnn::_Range, r3: &crate::dnn::_Range) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range(), r3.as_raw__Range(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs soft non maximum suppression given boxes and corresponding scores.
	/// Reference: <https://arxiv.org/abs/1704.04503>
	/// ## Parameters
	/// * bboxes: a set of bounding boxes to apply Soft NMS.
	/// * scores: a set of corresponding confidences.
	/// * updated_scores: a set of corresponding updated confidences.
	/// * score_threshold: a threshold used to filter boxes by score.
	/// * nms_threshold: a threshold used in non maximum suppression.
	/// * indices: the kept indices of bboxes after NMS.
	/// * top_k: keep at most @p top_k picked indices.
	/// * sigma: parameter of Gaussian weighting.
	/// * method: Gaussian or linear.
	/// ## See also
	/// SoftNMSMethod
	/// 
	/// ## Note
	/// This alternative version of [soft_nms_boxes] function uses the following default values for its arguments:
	/// * top_k: 0
	/// * sigma: 0.5
	/// * method: SoftNMSMethod::SOFTNMS_GAUSSIAN
	#[inline]
	pub fn soft_nms_boxes_def(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, updated_scores: &mut core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_softNMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_vectorLfloatGR_const_float_const_float_vectorLintGR(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), updated_scores.as_raw_mut_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs soft non maximum suppression given boxes and corresponding scores.
	/// Reference: <https://arxiv.org/abs/1704.04503>
	/// ## Parameters
	/// * bboxes: a set of bounding boxes to apply Soft NMS.
	/// * scores: a set of corresponding confidences.
	/// * updated_scores: a set of corresponding updated confidences.
	/// * score_threshold: a threshold used to filter boxes by score.
	/// * nms_threshold: a threshold used in non maximum suppression.
	/// * indices: the kept indices of bboxes after NMS.
	/// * top_k: keep at most @p top_k picked indices.
	/// * sigma: parameter of Gaussian weighting.
	/// * method: Gaussian or linear.
	/// ## See also
	/// SoftNMSMethod
	/// 
	/// ## C++ default parameters
	/// * top_k: 0
	/// * sigma: 0.5
	/// * method: SoftNMSMethod::SOFTNMS_GAUSSIAN
	#[inline]
	pub fn soft_nms_boxes(bboxes: &core::Vector<core::Rect>, scores: &core::Vector<f32>, updated_scores: &mut core::Vector<f32>, score_threshold: f32, nms_threshold: f32, indices: &mut core::Vector<i32>, top_k: size_t, sigma: f32, method: crate::dnn::SoftNMSMethod) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_softNMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_vectorLfloatGR_const_float_const_float_vectorLintGR_size_t_const_float_SoftNMSMethod(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), updated_scores.as_raw_mut_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_mut_VectorOfi32(), top_k, sigma, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [total_1] function uses the following default values for its arguments:
	/// * start: -1
	/// * end: -1
	#[inline]
	pub fn total_1_def(mat: &core::Mat) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dnn_total_const_MatR(mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * start: -1
	/// * end: -1
	#[inline]
	pub fn total_1(mat: &core::Mat, start: i32, end: i32) -> Result<i32> {
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
	pub trait AbsLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_AbsLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AbsLayer]
	pub trait AbsLayerTrait: crate::dnn::AbsLayerTraitConst + crate::dnn::ActivationLayerTrait {
		fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AbsLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for AbsLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AbsLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for AbsLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AbsLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AbsLayerTraitConst for AbsLayer {
		#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AbsLayerTrait for AbsLayer {
		#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AbsLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AbsLayer>> {
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
	pub trait AccumLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_AccumLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AccumLayer]
	pub trait AccumLayerTrait: crate::dnn::AccumLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AccumLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for AccumLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AccumLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AccumLayerTraitConst for AccumLayer {
		#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AccumLayerTrait for AccumLayer {
		#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AccumLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AccumLayer>> {
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
	
	/// Constant methods for [crate::dnn::AcosLayer]
	pub trait AcosLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_AcosLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AcosLayer]
	pub trait AcosLayerTrait: crate::dnn::AcosLayerTraitConst + crate::dnn::ActivationLayerTrait {
		fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AcosLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AcosLayer }
	
	impl Drop for AcosLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_AcosLayer_delete(self.as_raw_mut_AcosLayer()) };
		}
	}
	
	unsafe impl Send for AcosLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for AcosLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for AcosLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for AcosLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AcosLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for AcosLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AcosLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcosLayerTraitConst for AcosLayer {
		#[inline] fn as_raw_AcosLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AcosLayerTrait for AcosLayer {
		#[inline] fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AcosLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AcosLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_AcosLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::AcosLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { AcosLayer, crate::dnn::ActivationLayer, cv_dnn_AcosLayer_to_ActivationLayer }
	
	boxed_cast_base! { AcosLayer, core::Algorithm, cv_dnn_AcosLayer_to_Algorithm }
	
	boxed_cast_base! { AcosLayer, crate::dnn::Layer, cv_dnn_AcosLayer_to_Layer }
	
	impl std::fmt::Debug for AcosLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AcosLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::AcoshLayer]
	pub trait AcoshLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_AcoshLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AcoshLayer]
	pub trait AcoshLayerTrait: crate::dnn::AcoshLayerTraitConst + crate::dnn::ActivationLayerTrait {
		fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AcoshLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AcoshLayer }
	
	impl Drop for AcoshLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_AcoshLayer_delete(self.as_raw_mut_AcoshLayer()) };
		}
	}
	
	unsafe impl Send for AcoshLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for AcoshLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for AcoshLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for AcoshLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AcoshLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for AcoshLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AcoshLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcoshLayerTraitConst for AcoshLayer {
		#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AcoshLayerTrait for AcoshLayer {
		#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AcoshLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AcoshLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_AcoshLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::AcoshLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { AcoshLayer, crate::dnn::ActivationLayer, cv_dnn_AcoshLayer_to_ActivationLayer }
	
	boxed_cast_base! { AcoshLayer, core::Algorithm, cv_dnn_AcoshLayer_to_Algorithm }
	
	boxed_cast_base! { AcoshLayer, crate::dnn::Layer, cv_dnn_AcoshLayer_to_Layer }
	
	impl std::fmt::Debug for AcoshLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AcoshLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ActivationLayer]
	pub trait ActivationLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ActivationLayer(&self) -> *const c_void;
	
		#[inline]
		fn forward_slice(&self, src: &f32, dst: &mut f32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(self.as_raw_ActivationLayer(), src, dst, len, out_plane_size, cn0, cn1, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn forward_slice_1(&self, src: &i32, lut: &i32, dst: &mut i32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ActivationLayer_forwardSlice_const_const_intX_const_intX_intX_int_size_t_int_int(self.as_raw_ActivationLayer(), src, lut, dst, len, out_plane_size, cn0, cn1, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ActivationLayer]
	pub trait ActivationLayerTrait: crate::dnn::ActivationLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ActivationLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ActivationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ActivationLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for ActivationLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for ActivationLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ActivationLayer {
	}
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::AbsLayer, cv_dnn_ActivationLayer_to_AbsLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::AcosLayer, cv_dnn_ActivationLayer_to_AcosLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::AcoshLayer, cv_dnn_ActivationLayer_to_AcoshLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ActivationLayerInt8, cv_dnn_ActivationLayer_to_ActivationLayerInt8 }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::AsinLayer, cv_dnn_ActivationLayer_to_AsinLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::AsinhLayer, cv_dnn_ActivationLayer_to_AsinhLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::AtanLayer, cv_dnn_ActivationLayer_to_AtanLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::AtanhLayer, cv_dnn_ActivationLayer_to_AtanhLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::BNLLLayer, cv_dnn_ActivationLayer_to_BNLLLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::BatchNormLayer, cv_dnn_ActivationLayer_to_BatchNormLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::BatchNormLayerInt8, cv_dnn_ActivationLayer_to_BatchNormLayerInt8 }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::CeilLayer, cv_dnn_ActivationLayer_to_CeilLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::CeluLayer, cv_dnn_ActivationLayer_to_CeluLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ChannelsPReLULayer, cv_dnn_ActivationLayer_to_ChannelsPReLULayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::CosLayer, cv_dnn_ActivationLayer_to_CosLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::CoshLayer, cv_dnn_ActivationLayer_to_CoshLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ELULayer, cv_dnn_ActivationLayer_to_ELULayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ErfLayer, cv_dnn_ActivationLayer_to_ErfLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ExpLayer, cv_dnn_ActivationLayer_to_ExpLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::FloorLayer, cv_dnn_ActivationLayer_to_FloorLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::GeluApproximationLayer, cv_dnn_ActivationLayer_to_GeluApproximationLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::GeluLayer, cv_dnn_ActivationLayer_to_GeluLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::HardSigmoidLayer, cv_dnn_ActivationLayer_to_HardSigmoidLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::HardSwishLayer, cv_dnn_ActivationLayer_to_HardSwishLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::LogLayer, cv_dnn_ActivationLayer_to_LogLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::MishLayer, cv_dnn_ActivationLayer_to_MishLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::NotLayer, cv_dnn_ActivationLayer_to_NotLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::PowerLayer, cv_dnn_ActivationLayer_to_PowerLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ReLU6Layer, cv_dnn_ActivationLayer_to_ReLU6Layer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ReLULayer, cv_dnn_ActivationLayer_to_ReLULayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ReciprocalLayer, cv_dnn_ActivationLayer_to_ReciprocalLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::RoundLayer, cv_dnn_ActivationLayer_to_RoundLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SeluLayer, cv_dnn_ActivationLayer_to_SeluLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ShrinkLayer, cv_dnn_ActivationLayer_to_ShrinkLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SigmoidLayer, cv_dnn_ActivationLayer_to_SigmoidLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SignLayer, cv_dnn_ActivationLayer_to_SignLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SinLayer, cv_dnn_ActivationLayer_to_SinLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SinhLayer, cv_dnn_ActivationLayer_to_SinhLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SoftplusLayer, cv_dnn_ActivationLayer_to_SoftplusLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SoftsignLayer, cv_dnn_ActivationLayer_to_SoftsignLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SqrtLayer, cv_dnn_ActivationLayer_to_SqrtLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::SwishLayer, cv_dnn_ActivationLayer_to_SwishLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::TanHLayer, cv_dnn_ActivationLayer_to_TanHLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::TanLayer, cv_dnn_ActivationLayer_to_TanLayer }
	
	boxed_cast_descendant! { ActivationLayer, crate::dnn::ThresholdedReluLayer, cv_dnn_ActivationLayer_to_ThresholdedReluLayer }
	
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
	
	/// Constant methods for [crate::dnn::ActivationLayerInt8]
	pub trait ActivationLayerInt8TraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ActivationLayerInt8(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ActivationLayerInt8]
	pub trait ActivationLayerInt8Trait: crate::dnn::ActivationLayerInt8TraitConst + crate::dnn::ActivationLayerTrait {
		fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void;
	
	}
	
	pub struct ActivationLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ActivationLayerInt8 }
	
	impl Drop for ActivationLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ActivationLayerInt8_delete(self.as_raw_mut_ActivationLayerInt8()) };
		}
	}
	
	unsafe impl Send for ActivationLayerInt8 {}
	
	impl crate::dnn::ActivationLayerTraitConst for ActivationLayerInt8 {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for ActivationLayerInt8 {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for ActivationLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ActivationLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ActivationLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ActivationLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerInt8TraitConst for ActivationLayerInt8 {
		#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerInt8Trait for ActivationLayerInt8 {
		#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ActivationLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ActivationLayerInt8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ActivationLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ActivationLayerInt8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ActivationLayerInt8, crate::dnn::ActivationLayer, cv_dnn_ActivationLayerInt8_to_ActivationLayer }
	
	boxed_cast_base! { ActivationLayerInt8, core::Algorithm, cv_dnn_ActivationLayerInt8_to_Algorithm }
	
	boxed_cast_base! { ActivationLayerInt8, crate::dnn::Layer, cv_dnn_ActivationLayerInt8_to_Layer }
	
	impl std::fmt::Debug for ActivationLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ActivationLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ArgLayer]
	pub trait ArgLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ArgLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ArgLayer]
	pub trait ArgLayerTrait: crate::dnn::ArgLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void;
	
	}
	
	/// ArgMax/ArgMin layer
	/// 
	/// Note: returns indices as floats, which means the supported range is [-2^24; 2^24]
	pub struct ArgLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ArgLayer }
	
	impl Drop for ArgLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ArgLayer_delete(self.as_raw_mut_ArgLayer()) };
		}
	}
	
	unsafe impl Send for ArgLayer {}
	
	impl core::AlgorithmTraitConst for ArgLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ArgLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ArgLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ArgLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ArgLayerTraitConst for ArgLayer {
		#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ArgLayerTrait for ArgLayer {
		#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ArgLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ArgLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ArgLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ArgLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ArgLayer, core::Algorithm, cv_dnn_ArgLayer_to_Algorithm }
	
	boxed_cast_base! { ArgLayer, crate::dnn::Layer, cv_dnn_ArgLayer_to_Layer }
	
	impl std::fmt::Debug for ArgLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ArgLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::AsinLayer]
	pub trait AsinLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_AsinLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AsinLayer]
	pub trait AsinLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AsinLayerTraitConst {
		fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AsinLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AsinLayer }
	
	impl Drop for AsinLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_AsinLayer_delete(self.as_raw_mut_AsinLayer()) };
		}
	}
	
	unsafe impl Send for AsinLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for AsinLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for AsinLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for AsinLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AsinLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for AsinLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AsinLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinLayerTraitConst for AsinLayer {
		#[inline] fn as_raw_AsinLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AsinLayerTrait for AsinLayer {
		#[inline] fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AsinLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AsinLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_AsinLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::AsinLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { AsinLayer, crate::dnn::ActivationLayer, cv_dnn_AsinLayer_to_ActivationLayer }
	
	boxed_cast_base! { AsinLayer, core::Algorithm, cv_dnn_AsinLayer_to_Algorithm }
	
	boxed_cast_base! { AsinLayer, crate::dnn::Layer, cv_dnn_AsinLayer_to_Layer }
	
	impl std::fmt::Debug for AsinLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AsinLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::AsinhLayer]
	pub trait AsinhLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_AsinhLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AsinhLayer]
	pub trait AsinhLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AsinhLayerTraitConst {
		fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AsinhLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AsinhLayer }
	
	impl Drop for AsinhLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_AsinhLayer_delete(self.as_raw_mut_AsinhLayer()) };
		}
	}
	
	unsafe impl Send for AsinhLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for AsinhLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for AsinhLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for AsinhLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AsinhLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for AsinhLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AsinhLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinhLayerTraitConst for AsinhLayer {
		#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AsinhLayerTrait for AsinhLayer {
		#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AsinhLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AsinhLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_AsinhLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::AsinhLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { AsinhLayer, crate::dnn::ActivationLayer, cv_dnn_AsinhLayer_to_ActivationLayer }
	
	boxed_cast_base! { AsinhLayer, core::Algorithm, cv_dnn_AsinhLayer_to_Algorithm }
	
	boxed_cast_base! { AsinhLayer, crate::dnn::Layer, cv_dnn_AsinhLayer_to_Layer }
	
	impl std::fmt::Debug for AsinhLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AsinhLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::AtanLayer]
	pub trait AtanLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_AtanLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AtanLayer]
	pub trait AtanLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AtanLayerTraitConst {
		fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AtanLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AtanLayer }
	
	impl Drop for AtanLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_AtanLayer_delete(self.as_raw_mut_AtanLayer()) };
		}
	}
	
	unsafe impl Send for AtanLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for AtanLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for AtanLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for AtanLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AtanLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for AtanLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AtanLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanLayerTraitConst for AtanLayer {
		#[inline] fn as_raw_AtanLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AtanLayerTrait for AtanLayer {
		#[inline] fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AtanLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AtanLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_AtanLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::AtanLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { AtanLayer, crate::dnn::ActivationLayer, cv_dnn_AtanLayer_to_ActivationLayer }
	
	boxed_cast_base! { AtanLayer, core::Algorithm, cv_dnn_AtanLayer_to_Algorithm }
	
	boxed_cast_base! { AtanLayer, crate::dnn::Layer, cv_dnn_AtanLayer_to_Layer }
	
	impl std::fmt::Debug for AtanLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AtanLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::AtanhLayer]
	pub trait AtanhLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_AtanhLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::AtanhLayer]
	pub trait AtanhLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::AtanhLayerTraitConst {
		fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct AtanhLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AtanhLayer }
	
	impl Drop for AtanhLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_AtanhLayer_delete(self.as_raw_mut_AtanhLayer()) };
		}
	}
	
	unsafe impl Send for AtanhLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for AtanhLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for AtanhLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for AtanhLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AtanhLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for AtanhLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for AtanhLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanhLayerTraitConst for AtanhLayer {
		#[inline] fn as_raw_AtanhLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::AtanhLayerTrait for AtanhLayer {
		#[inline] fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AtanhLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::AtanhLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_AtanhLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::AtanhLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { AtanhLayer, crate::dnn::ActivationLayer, cv_dnn_AtanhLayer_to_ActivationLayer }
	
	boxed_cast_base! { AtanhLayer, core::Algorithm, cv_dnn_AtanhLayer_to_Algorithm }
	
	boxed_cast_base! { AtanhLayer, crate::dnn::Layer, cv_dnn_AtanhLayer_to_Layer }
	
	impl std::fmt::Debug for AtanhLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AtanhLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::BNLLLayer]
	pub trait BNLLLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_BNLLLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::BNLLLayer]
	pub trait BNLLLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::BNLLLayerTraitConst {
		fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct BNLLLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for BNLLLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for BNLLLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for BNLLLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for BNLLLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BNLLLayerTraitConst for BNLLLayer {
		#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BNLLLayerTrait for BNLLLayer {
		#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BNLLLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BNLLLayer>> {
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
	pub trait BackendNodeTraitConst {
		fn as_raw_BackendNode(&self) -> *const c_void;
	
		/// Backend identifier.
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
		#[inline]
		fn set_backend_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_BackendNode_propBackendId_int(self.as_raw_mut_BackendNode(), val) };
			ret
		}
		
	}
	
	/// Derivatives of this class encapsulates functions of certain backends.
	pub struct BackendNode {
		ptr: *mut c_void
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
	pub trait BackendWrapperTraitConst {
		fn as_raw_BackendWrapper(&self) -> *const c_void;
	
		/// Backend identifier.
		#[inline]
		fn backend_id(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_BackendWrapper_propBackendId_const(self.as_raw_BackendWrapper()) };
			ret
		}
		
		/// Target identifier.
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
		#[inline]
		fn set_backend_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_BackendWrapper_propBackendId_int(self.as_raw_mut_BackendWrapper(), val) };
			ret
		}
		
		/// Target identifier.
		#[inline]
		fn set_target_id(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_BackendWrapper_propTargetId_int(self.as_raw_mut_BackendWrapper(), val) };
			ret
		}
		
		/// Transfer data to CPU host memory.
		#[inline]
		fn copy_to_host(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_BackendWrapper_copyToHost(self.as_raw_mut_BackendWrapper(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Indicate that an actual data is on CPU.
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
	pub struct BackendWrapper {
		ptr: *mut c_void
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
	pub trait BaseConvolutionLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_BaseConvolutionLayer(&self) -> *const c_void;
	
		#[inline]
		fn kernel(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn stride(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_BaseConvolutionLayer_propStride_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn pad(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_BaseConvolutionLayer_propPad_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn dilation(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilation_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn adjust_pad(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjustPad_const(self.as_raw_BaseConvolutionLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn adjust_pads(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjust_pads_const(self.as_raw_BaseConvolutionLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn kernel_size(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_size_const(self.as_raw_BaseConvolutionLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn strides(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propStrides_const(self.as_raw_BaseConvolutionLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn dilations(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilations_const(self.as_raw_BaseConvolutionLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pads_begin(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_begin_const(self.as_raw_BaseConvolutionLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pads_end(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_end_const(self.as_raw_BaseConvolutionLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pad_mode(&self) -> String {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPadMode_const(self.as_raw_BaseConvolutionLayer()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn num_output(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propNumOutput_const(self.as_raw_BaseConvolutionLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::BaseConvolutionLayer]
	pub trait BaseConvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_kernel(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_stride(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propStride_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_pad(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPad_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_dilation(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilation_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_adjust_pad(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjustPad_Size(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_adjust_pads(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propAdjust_pads_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_kernel_size(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propKernel_size_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_strides(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propStrides_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_dilations(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propDilations_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_pads_begin(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_begin_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_pads_end(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPads_end_vectorLsize_tG(self.as_raw_mut_BaseConvolutionLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_pad_mode(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propPadMode_String(self.as_raw_mut_BaseConvolutionLayer(), val.opencv_as_extern_mut()) };
			ret
		}
		
		#[inline]
		fn set_num_output(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_BaseConvolutionLayer_propNumOutput_int(self.as_raw_mut_BaseConvolutionLayer(), val) };
			ret
		}
		
	}
	
	pub struct BaseConvolutionLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for BaseConvolutionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for BaseConvolutionLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTraitConst for BaseConvolutionLayer {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for BaseConvolutionLayer {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BaseConvolutionLayer {
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
	
	/// Constant methods for [crate::dnn::BatchNormLayer]
	pub trait BatchNormLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_BatchNormLayer(&self) -> *const c_void;
	
		#[inline]
		fn has_weights(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasWeights_const(self.as_raw_BatchNormLayer()) };
			ret
		}
		
		#[inline]
		fn has_bias(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasBias_const(self.as_raw_BatchNormLayer()) };
			ret
		}
		
		#[inline]
		fn epsilon(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_BatchNormLayer_propEpsilon_const(self.as_raw_BatchNormLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::BatchNormLayer]
	pub trait BatchNormLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::BatchNormLayerTraitConst {
		fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_has_weights(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasWeights_bool(self.as_raw_mut_BatchNormLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_has_bias(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_BatchNormLayer_propHasBias_bool(self.as_raw_mut_BatchNormLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_epsilon(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_BatchNormLayer_propEpsilon_float(self.as_raw_mut_BatchNormLayer(), val) };
			ret
		}
		
	}
	
	pub struct BatchNormLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for BatchNormLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for BatchNormLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for BatchNormLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for BatchNormLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for BatchNormLayer {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for BatchNormLayer {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BatchNormLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BatchNormLayer>> {
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
	
	/// Constant methods for [crate::dnn::BatchNormLayerInt8]
	pub trait BatchNormLayerInt8TraitConst: crate::dnn::BatchNormLayerTraitConst {
		fn as_raw_BatchNormLayerInt8(&self) -> *const c_void;
	
		#[inline]
		fn input_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propInput_sc_const(self.as_raw_BatchNormLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propOutput_sc_const(self.as_raw_BatchNormLayerInt8()) };
			ret
		}
		
		#[inline]
		fn input_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propInput_zp_const(self.as_raw_BatchNormLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propOutput_zp_const(self.as_raw_BatchNormLayerInt8()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::BatchNormLayerInt8]
	pub trait BatchNormLayerInt8Trait: crate::dnn::BatchNormLayerInt8TraitConst + crate::dnn::BatchNormLayerTrait {
		fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_input_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propInput_sc_float(self.as_raw_mut_BatchNormLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propOutput_sc_float(self.as_raw_mut_BatchNormLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_input_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propInput_zp_int(self.as_raw_mut_BatchNormLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_BatchNormLayerInt8_propOutput_zp_int(self.as_raw_mut_BatchNormLayerInt8(), val) };
			ret
		}
		
	}
	
	pub struct BatchNormLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BatchNormLayerInt8 }
	
	impl Drop for BatchNormLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_BatchNormLayerInt8_delete(self.as_raw_mut_BatchNormLayerInt8()) };
		}
	}
	
	unsafe impl Send for BatchNormLayerInt8 {}
	
	impl crate::dnn::ActivationLayerTraitConst for BatchNormLayerInt8 {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for BatchNormLayerInt8 {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for BatchNormLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for BatchNormLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for BatchNormLayerInt8 {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for BatchNormLayerInt8 {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for BatchNormLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for BatchNormLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerInt8TraitConst for BatchNormLayerInt8 {
		#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerInt8Trait for BatchNormLayerInt8 {
		#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BatchNormLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BatchNormLayerInt8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_BatchNormLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::BatchNormLayerInt8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { BatchNormLayerInt8, crate::dnn::ActivationLayer, cv_dnn_BatchNormLayerInt8_to_ActivationLayer }
	
	boxed_cast_base! { BatchNormLayerInt8, core::Algorithm, cv_dnn_BatchNormLayerInt8_to_Algorithm }
	
	boxed_cast_base! { BatchNormLayerInt8, crate::dnn::BatchNormLayer, cv_dnn_BatchNormLayerInt8_to_BatchNormLayer }
	
	boxed_cast_base! { BatchNormLayerInt8, crate::dnn::Layer, cv_dnn_BatchNormLayerInt8_to_Layer }
	
	impl std::fmt::Debug for BatchNormLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BatchNormLayerInt8")
				.field("input_sc", &crate::dnn::BatchNormLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::BatchNormLayerInt8TraitConst::output_sc(self))
				.field("input_zp", &crate::dnn::BatchNormLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::BatchNormLayerInt8TraitConst::output_zp(self))
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
	pub struct BlankLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for BlankLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for BlankLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BlankLayerTraitConst for BlankLayer {
		#[inline] fn as_raw_BlankLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BlankLayerTrait for BlankLayer {
		#[inline] fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BlankLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::CeilLayer]
	pub trait CeilLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_CeilLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::CeilLayer]
	pub trait CeilLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CeilLayerTraitConst {
		fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct CeilLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CeilLayer }
	
	impl Drop for CeilLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_CeilLayer_delete(self.as_raw_mut_CeilLayer()) };
		}
	}
	
	unsafe impl Send for CeilLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for CeilLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for CeilLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for CeilLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CeilLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for CeilLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CeilLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeilLayerTraitConst for CeilLayer {
		#[inline] fn as_raw_CeilLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CeilLayerTrait for CeilLayer {
		#[inline] fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CeilLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CeilLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_CeilLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::CeilLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CeilLayer, crate::dnn::ActivationLayer, cv_dnn_CeilLayer_to_ActivationLayer }
	
	boxed_cast_base! { CeilLayer, core::Algorithm, cv_dnn_CeilLayer_to_Algorithm }
	
	boxed_cast_base! { CeilLayer, crate::dnn::Layer, cv_dnn_CeilLayer_to_Layer }
	
	impl std::fmt::Debug for CeilLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CeilLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::CeluLayer]
	pub trait CeluLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_CeluLayer(&self) -> *const c_void;
	
		#[inline]
		fn alpha(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_CeluLayer_propAlpha_const(self.as_raw_CeluLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::CeluLayer]
	pub trait CeluLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CeluLayerTraitConst {
		fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_alpha(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_CeluLayer_propAlpha_float(self.as_raw_mut_CeluLayer(), val) };
			ret
		}
		
	}
	
	pub struct CeluLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CeluLayer }
	
	impl Drop for CeluLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_CeluLayer_delete(self.as_raw_mut_CeluLayer()) };
		}
	}
	
	unsafe impl Send for CeluLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for CeluLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for CeluLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for CeluLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CeluLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for CeluLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CeluLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeluLayerTraitConst for CeluLayer {
		#[inline] fn as_raw_CeluLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CeluLayerTrait for CeluLayer {
		#[inline] fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CeluLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CeluLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_CeluLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::CeluLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CeluLayer, crate::dnn::ActivationLayer, cv_dnn_CeluLayer_to_ActivationLayer }
	
	boxed_cast_base! { CeluLayer, core::Algorithm, cv_dnn_CeluLayer_to_Algorithm }
	
	boxed_cast_base! { CeluLayer, crate::dnn::Layer, cv_dnn_CeluLayer_to_Layer }
	
	impl std::fmt::Debug for CeluLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CeluLayer")
				.field("alpha", &crate::dnn::CeluLayerTraitConst::alpha(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ChannelsPReLULayer]
	pub trait ChannelsPReLULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ChannelsPReLULayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ChannelsPReLULayer]
	pub trait ChannelsPReLULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ChannelsPReLULayerTraitConst {
		fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ChannelsPReLULayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for ChannelsPReLULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ChannelsPReLULayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ChannelsPReLULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ChannelsPReLULayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ChannelsPReLULayerTraitConst for ChannelsPReLULayer {
		#[inline] fn as_raw_ChannelsPReLULayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ChannelsPReLULayerTrait for ChannelsPReLULayer {
		#[inline] fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ChannelsPReLULayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::ClassificationModel]
	pub trait ClassificationModelTraitConst: crate::dnn::ModelTraitConst {
		fn as_raw_ClassificationModel(&self) -> *const c_void;
	
		/// Get enable/disable softmax post processing option.
		/// 
		/// This option defaults to false, softmax post processing is not applied within the classify() function.
		#[inline]
		fn get_enable_softmax_post_processing(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_getEnableSoftmaxPostProcessing_const(self.as_raw_ClassificationModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ClassificationModel]
	pub trait ClassificationModelTrait: crate::dnn::ClassificationModelTraitConst + crate::dnn::ModelTrait {
		fn as_raw_mut_ClassificationModel(&mut self) -> *mut c_void;
	
		/// Set enable/disable softmax post processing option.
		/// 
		/// If this option is true, softmax is applied after forward inference within the classify() function
		/// to convert the confidences range to [0.0-1.0].
		/// This function allows you to toggle this behavior.
		/// Please turn true when not contain softmax layer in model.
		/// ## Parameters
		/// * enable: Set enable softmax post processing within the classify() function.
		#[inline]
		fn set_enable_softmax_post_processing(&mut self, enable: bool) -> Result<crate::dnn::ClassificationModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_setEnableSoftmaxPostProcessing_bool(self.as_raw_mut_ClassificationModel(), enable, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Given the @p input frame, create input blob, run net and return top-1 prediction.
		/// ## Parameters
		/// * frame: The input image.
		#[inline]
		fn classify(&mut self, frame: &impl core::ToInputArray) -> Result<core::Tuple<(i32, f32)>> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_classify_const__InputArrayR(self.as_raw_mut_ClassificationModel(), frame.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Tuple::<(i32, f32)>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Given the @p input frame, create input blob, run net and return top-1 prediction.
		/// ## Parameters
		/// * frame: The input image.
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn classify_1(&mut self, frame: &impl core::ToInputArray, class_id: &mut i32, conf: &mut f32) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_classify_const__InputArrayR_intR_floatR(self.as_raw_mut_ClassificationModel(), frame.as_raw__InputArray(), class_id, conf, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// This class represents high-level API for classification models.
	/// 
	/// ClassificationModel allows to set params for preprocessing input image.
	/// ClassificationModel creates net from file with trained weights and config,
	/// sets preprocessing input, runs forward pass and return top-1 prediction.
	pub struct ClassificationModel {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ClassificationModel }
	
	impl Drop for ClassificationModel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ClassificationModel_delete(self.as_raw_mut_ClassificationModel()) };
		}
	}
	
	unsafe impl Send for ClassificationModel {}
	
	impl crate::dnn::ModelTraitConst for ClassificationModel {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for ClassificationModel {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ClassificationModelTraitConst for ClassificationModel {
		#[inline] fn as_raw_ClassificationModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ClassificationModelTrait for ClassificationModel {
		#[inline] fn as_raw_mut_ClassificationModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ClassificationModel {
		#[inline]
		pub fn default() -> Result<crate::dnn::ClassificationModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_ClassificationModel(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create classification model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn new(model: &str, config: &str) -> Result<crate::dnn::ClassificationModel> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_ClassificationModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create classification model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn new_def(model: &str) -> Result<crate::dnn::ClassificationModel> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_ClassificationModel_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create model from deep learning network.
		/// ## Parameters
		/// * network: Net object.
		#[inline]
		pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::ClassificationModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ClassificationModel_ClassificationModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::ClassificationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for ClassificationModel {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_ClassificationModel_implicitClone_const(self.as_raw_ClassificationModel())) }
		}
	}
	
	boxed_cast_base! { ClassificationModel, crate::dnn::Model, cv_dnn_ClassificationModel_to_Model }
	
	impl std::fmt::Debug for ClassificationModel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ClassificationModel")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::CompareLayer]
	pub trait CompareLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_CompareLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::CompareLayer]
	pub trait CompareLayerTrait: crate::dnn::CompareLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_CompareLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct CompareLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CompareLayer }
	
	impl Drop for CompareLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_CompareLayer_delete(self.as_raw_mut_CompareLayer()) };
		}
	}
	
	unsafe impl Send for CompareLayer {}
	
	impl core::AlgorithmTraitConst for CompareLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CompareLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for CompareLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CompareLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CompareLayerTraitConst for CompareLayer {
		#[inline] fn as_raw_CompareLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CompareLayerTrait for CompareLayer {
		#[inline] fn as_raw_mut_CompareLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CompareLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_CompareLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CompareLayer, core::Algorithm, cv_dnn_CompareLayer_to_Algorithm }
	
	boxed_cast_base! { CompareLayer, crate::dnn::Layer, cv_dnn_CompareLayer_to_Layer }
	
	impl std::fmt::Debug for CompareLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CompareLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ConcatLayer]
	pub trait ConcatLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ConcatLayer(&self) -> *const c_void;
	
		#[inline]
		fn axis(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_ConcatLayer_propAxis_const(self.as_raw_ConcatLayer()) };
			ret
		}
		
		/// Add zero padding in case of concatenation of blobs with different
		/// spatial sizes.
		/// 
		/// Details: <https://github.com/torch/nn/blob/master/doc/containers.md#depthconcat>
		#[inline]
		fn padding(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_ConcatLayer_propPadding_const(self.as_raw_ConcatLayer()) };
			ret
		}
		
		#[inline]
		fn padding_value(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_ConcatLayer_propPaddingValue_const(self.as_raw_ConcatLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ConcatLayer]
	pub trait ConcatLayerTrait: crate::dnn::ConcatLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_axis(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_ConcatLayer_propAxis_int(self.as_raw_mut_ConcatLayer(), val) };
			ret
		}
		
		/// Add zero padding in case of concatenation of blobs with different
		/// spatial sizes.
		/// 
		/// Details: <https://github.com/torch/nn/blob/master/doc/containers.md#depthconcat>
		#[inline]
		fn set_padding(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_ConcatLayer_propPadding_bool(self.as_raw_mut_ConcatLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_padding_value(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_ConcatLayer_propPaddingValue_int(self.as_raw_mut_ConcatLayer(), val) };
			ret
		}
		
	}
	
	pub struct ConcatLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ConcatLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ConcatLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConcatLayerTraitConst for ConcatLayer {
		#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ConcatLayerTrait for ConcatLayer {
		#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ConcatLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ConcatLayer>> {
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
				.field("padding_value", &crate::dnn::ConcatLayerTraitConst::padding_value(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ConstLayer]
	pub trait ConstLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ConstLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ConstLayer]
	pub trait ConstLayerTrait: crate::dnn::ConstLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void;
	
	}
	
	/// Constant layer produces the same data blob at an every forward pass.
	pub struct ConstLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ConstLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ConstLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConstLayerTraitConst for ConstLayer {
		#[inline] fn as_raw_ConstLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ConstLayerTrait for ConstLayer {
		#[inline] fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ConstLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::ConvolutionLayer]
	pub trait ConvolutionLayerTraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
		fn as_raw_ConvolutionLayer(&self) -> *const c_void;
	
		#[inline]
		fn fused_activation(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayer_propFusedActivation_const(self.as_raw_ConvolutionLayer()) };
			ret
		}
		
		#[inline]
		fn fused_add(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayer_propFusedAdd_const(self.as_raw_ConvolutionLayer()) };
			ret
		}
		
		#[inline]
		fn use_winograd(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayer_propUseWinograd_const(self.as_raw_ConvolutionLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ConvolutionLayer]
	pub trait ConvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::ConvolutionLayerTraitConst {
		fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * val: false
		#[inline]
		fn set_fused_activation(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayer_propFusedActivation_bool(self.as_raw_mut_ConvolutionLayer(), val) };
			ret
		}
		
		/// ## C++ default parameters
		/// * val: false
		#[inline]
		fn set_fused_add(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayer_propFusedAdd_bool(self.as_raw_mut_ConvolutionLayer(), val) };
			ret
		}
		
		/// ## C++ default parameters
		/// * val: false
		#[inline]
		fn set_use_winograd(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayer_propUseWinograd_bool(self.as_raw_mut_ConvolutionLayer(), val) };
			ret
		}
		
	}
	
	pub struct ConvolutionLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::BaseConvolutionLayerTraitConst for ConvolutionLayer {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayer {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ConvolutionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ConvolutionLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConvolutionLayerTraitConst for ConvolutionLayer {
		#[inline] fn as_raw_ConvolutionLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ConvolutionLayerTrait for ConvolutionLayer {
		#[inline] fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ConvolutionLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
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
				.field("fused_activation", &crate::dnn::ConvolutionLayerTraitConst::fused_activation(self))
				.field("fused_add", &crate::dnn::ConvolutionLayerTraitConst::fused_add(self))
				.field("use_winograd", &crate::dnn::ConvolutionLayerTraitConst::use_winograd(self))
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
	
	/// Constant methods for [crate::dnn::ConvolutionLayerInt8]
	pub trait ConvolutionLayerInt8TraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
		fn as_raw_ConvolutionLayerInt8(&self) -> *const c_void;
	
		#[inline]
		fn input_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propInput_zp_const(self.as_raw_ConvolutionLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propOutput_zp_const(self.as_raw_ConvolutionLayerInt8()) };
			ret
		}
		
		#[inline]
		fn input_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propInput_sc_const(self.as_raw_ConvolutionLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propOutput_sc_const(self.as_raw_ConvolutionLayerInt8()) };
			ret
		}
		
		#[inline]
		fn per_channel(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propPer_channel_const(self.as_raw_ConvolutionLayerInt8()) };
			ret
		}
		
		#[inline]
		fn use_winograd(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propUseWinograd_const(self.as_raw_ConvolutionLayerInt8()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ConvolutionLayerInt8]
	pub trait ConvolutionLayerInt8Trait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::ConvolutionLayerInt8TraitConst {
		fn as_raw_mut_ConvolutionLayerInt8(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_input_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propInput_zp_int(self.as_raw_mut_ConvolutionLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propOutput_zp_int(self.as_raw_mut_ConvolutionLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_input_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propInput_sc_float(self.as_raw_mut_ConvolutionLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propOutput_sc_float(self.as_raw_mut_ConvolutionLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_per_channel(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propPer_channel_bool(self.as_raw_mut_ConvolutionLayerInt8(), val) };
			ret
		}
		
		/// ## C++ default parameters
		/// * val: true
		#[inline]
		fn set_use_winograd(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_ConvolutionLayerInt8_propUseWinograd_bool(self.as_raw_mut_ConvolutionLayerInt8(), val) };
			ret
		}
		
	}
	
	pub struct ConvolutionLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ConvolutionLayerInt8 }
	
	impl Drop for ConvolutionLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ConvolutionLayerInt8_delete(self.as_raw_mut_ConvolutionLayerInt8()) };
		}
	}
	
	unsafe impl Send for ConvolutionLayerInt8 {}
	
	impl core::AlgorithmTraitConst for ConvolutionLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ConvolutionLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTraitConst for ConvolutionLayerInt8 {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayerInt8 {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ConvolutionLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ConvolutionLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConvolutionLayerInt8TraitConst for ConvolutionLayerInt8 {
		#[inline] fn as_raw_ConvolutionLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ConvolutionLayerInt8Trait for ConvolutionLayerInt8 {
		#[inline] fn as_raw_mut_ConvolutionLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ConvolutionLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ConvolutionLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::BaseConvolutionLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ConvolutionLayerInt8, core::Algorithm, cv_dnn_ConvolutionLayerInt8_to_Algorithm }
	
	boxed_cast_base! { ConvolutionLayerInt8, crate::dnn::BaseConvolutionLayer, cv_dnn_ConvolutionLayerInt8_to_BaseConvolutionLayer }
	
	boxed_cast_base! { ConvolutionLayerInt8, crate::dnn::Layer, cv_dnn_ConvolutionLayerInt8_to_Layer }
	
	impl std::fmt::Debug for ConvolutionLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ConvolutionLayerInt8")
				.field("input_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::output_zp(self))
				.field("input_sc", &crate::dnn::ConvolutionLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::ConvolutionLayerInt8TraitConst::output_sc(self))
				.field("per_channel", &crate::dnn::ConvolutionLayerInt8TraitConst::per_channel(self))
				.field("use_winograd", &crate::dnn::ConvolutionLayerInt8TraitConst::use_winograd(self))
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
	
	/// Constant methods for [crate::dnn::CorrelationLayer]
	pub trait CorrelationLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_CorrelationLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::CorrelationLayer]
	pub trait CorrelationLayerTrait: crate::dnn::CorrelationLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct CorrelationLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for CorrelationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CorrelationLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CorrelationLayerTraitConst for CorrelationLayer {
		#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CorrelationLayerTrait for CorrelationLayer {
		#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CorrelationLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CorrelationLayer>> {
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
	
	/// Constant methods for [crate::dnn::CosLayer]
	pub trait CosLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_CosLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::CosLayer]
	pub trait CosLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CosLayerTraitConst {
		fn as_raw_mut_CosLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct CosLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CosLayer }
	
	impl Drop for CosLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_CosLayer_delete(self.as_raw_mut_CosLayer()) };
		}
	}
	
	unsafe impl Send for CosLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for CosLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for CosLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for CosLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CosLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for CosLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CosLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CosLayerTraitConst for CosLayer {
		#[inline] fn as_raw_CosLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CosLayerTrait for CosLayer {
		#[inline] fn as_raw_mut_CosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CosLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CosLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_CosLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::CosLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CosLayer, crate::dnn::ActivationLayer, cv_dnn_CosLayer_to_ActivationLayer }
	
	boxed_cast_base! { CosLayer, core::Algorithm, cv_dnn_CosLayer_to_Algorithm }
	
	boxed_cast_base! { CosLayer, crate::dnn::Layer, cv_dnn_CosLayer_to_Layer }
	
	impl std::fmt::Debug for CosLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CosLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::CoshLayer]
	pub trait CoshLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_CoshLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::CoshLayer]
	pub trait CoshLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::CoshLayerTraitConst {
		fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct CoshLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CoshLayer }
	
	impl Drop for CoshLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_CoshLayer_delete(self.as_raw_mut_CoshLayer()) };
		}
	}
	
	unsafe impl Send for CoshLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for CoshLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for CoshLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for CoshLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CoshLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for CoshLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CoshLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CoshLayerTraitConst for CoshLayer {
		#[inline] fn as_raw_CoshLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CoshLayerTrait for CoshLayer {
		#[inline] fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CoshLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CoshLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_CoshLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::CoshLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CoshLayer, crate::dnn::ActivationLayer, cv_dnn_CoshLayer_to_ActivationLayer }
	
	boxed_cast_base! { CoshLayer, core::Algorithm, cv_dnn_CoshLayer_to_Algorithm }
	
	boxed_cast_base! { CoshLayer, crate::dnn::Layer, cv_dnn_CoshLayer_to_Layer }
	
	impl std::fmt::Debug for CoshLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CoshLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::CropAndResizeLayer]
	pub trait CropAndResizeLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_CropAndResizeLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::CropAndResizeLayer]
	pub trait CropAndResizeLayerTrait: crate::dnn::CropAndResizeLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct CropAndResizeLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for CropAndResizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CropAndResizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CropAndResizeLayerTraitConst for CropAndResizeLayer {
		#[inline] fn as_raw_CropAndResizeLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CropAndResizeLayerTrait for CropAndResizeLayer {
		#[inline] fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CropAndResizeLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::CropLayer]
	pub trait CropLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_CropLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::CropLayer]
	pub trait CropLayerTrait: crate::dnn::CropLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_CropLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct CropLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for CropLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CropLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CropLayerTraitConst for CropLayer {
		#[inline] fn as_raw_CropLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CropLayerTrait for CropLayer {
		#[inline] fn as_raw_mut_CropLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CropLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::CumSumLayer]
	pub trait CumSumLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_CumSumLayer(&self) -> *const c_void;
	
		#[inline]
		fn exclusive(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_CumSumLayer_propExclusive_const(self.as_raw_CumSumLayer()) };
			ret
		}
		
		#[inline]
		fn reverse(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_CumSumLayer_propReverse_const(self.as_raw_CumSumLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::CumSumLayer]
	pub trait CumSumLayerTrait: crate::dnn::CumSumLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_exclusive(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_CumSumLayer_propExclusive_int(self.as_raw_mut_CumSumLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_reverse(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_CumSumLayer_propReverse_int(self.as_raw_mut_CumSumLayer(), val) };
			ret
		}
		
	}
	
	pub struct CumSumLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CumSumLayer }
	
	impl Drop for CumSumLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_CumSumLayer_delete(self.as_raw_mut_CumSumLayer()) };
		}
	}
	
	unsafe impl Send for CumSumLayer {}
	
	impl core::AlgorithmTraitConst for CumSumLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CumSumLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for CumSumLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for CumSumLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CumSumLayerTraitConst for CumSumLayer {
		#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::CumSumLayerTrait for CumSumLayer {
		#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CumSumLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::CumSumLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_CumSumLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::CumSumLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CumSumLayer, core::Algorithm, cv_dnn_CumSumLayer_to_Algorithm }
	
	boxed_cast_base! { CumSumLayer, crate::dnn::Layer, cv_dnn_CumSumLayer_to_Layer }
	
	impl std::fmt::Debug for CumSumLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CumSumLayer")
				.field("exclusive", &crate::dnn::CumSumLayerTraitConst::exclusive(self))
				.field("reverse", &crate::dnn::CumSumLayerTraitConst::reverse(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::DataAugmentationLayer]
	pub trait DataAugmentationLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_DataAugmentationLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::DataAugmentationLayer]
	pub trait DataAugmentationLayerTrait: crate::dnn::DataAugmentationLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct DataAugmentationLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for DataAugmentationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for DataAugmentationLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DataAugmentationLayerTraitConst for DataAugmentationLayer {
		#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::DataAugmentationLayerTrait for DataAugmentationLayer {
		#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DataAugmentationLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::DataAugmentationLayer>> {
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
	
	/// Constant methods for [crate::dnn::DeconvolutionLayer]
	pub trait DeconvolutionLayerTraitConst: crate::dnn::BaseConvolutionLayerTraitConst {
		fn as_raw_DeconvolutionLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::DeconvolutionLayer]
	pub trait DeconvolutionLayerTrait: crate::dnn::BaseConvolutionLayerTrait + crate::dnn::DeconvolutionLayerTraitConst {
		fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct DeconvolutionLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::BaseConvolutionLayerTraitConst for DeconvolutionLayer {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for DeconvolutionLayer {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for DeconvolutionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for DeconvolutionLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DeconvolutionLayerTraitConst for DeconvolutionLayer {
		#[inline] fn as_raw_DeconvolutionLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::DeconvolutionLayerTrait for DeconvolutionLayer {
		#[inline] fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DeconvolutionLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::BaseConvolutionLayer>> {
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
	
	/// Constant methods for [crate::dnn::DequantizeLayer]
	pub trait DequantizeLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_DequantizeLayer(&self) -> *const c_void;
	
		#[inline]
		fn scales(&self) -> core::Vector<f32> {
			let ret = unsafe { sys::cv_dnn_DequantizeLayer_propScales_const(self.as_raw_DequantizeLayer()) };
			let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn zeropoints(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_dnn_DequantizeLayer_propZeropoints_const(self.as_raw_DequantizeLayer()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::DequantizeLayer]
	pub trait DequantizeLayerTrait: crate::dnn::DequantizeLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_scales(&mut self, mut val: core::Vector<f32>) {
			let ret = unsafe { sys::cv_dnn_DequantizeLayer_propScales_vectorLfloatG(self.as_raw_mut_DequantizeLayer(), val.as_raw_mut_VectorOff32()) };
			ret
		}
		
		#[inline]
		fn set_zeropoints(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_dnn_DequantizeLayer_propZeropoints_vectorLintG(self.as_raw_mut_DequantizeLayer(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
	}
	
	pub struct DequantizeLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DequantizeLayer }
	
	impl Drop for DequantizeLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_DequantizeLayer_delete(self.as_raw_mut_DequantizeLayer()) };
		}
	}
	
	unsafe impl Send for DequantizeLayer {}
	
	impl core::AlgorithmTraitConst for DequantizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for DequantizeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for DequantizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for DequantizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DequantizeLayerTraitConst for DequantizeLayer {
		#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::DequantizeLayerTrait for DequantizeLayer {
		#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DequantizeLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::DequantizeLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DequantizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::DequantizeLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { DequantizeLayer, core::Algorithm, cv_dnn_DequantizeLayer_to_Algorithm }
	
	boxed_cast_base! { DequantizeLayer, crate::dnn::Layer, cv_dnn_DequantizeLayer_to_Layer }
	
	impl std::fmt::Debug for DequantizeLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DequantizeLayer")
				.field("scales", &crate::dnn::DequantizeLayerTraitConst::scales(self))
				.field("zeropoints", &crate::dnn::DequantizeLayerTraitConst::zeropoints(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::DetectionModel]
	pub trait DetectionModelTraitConst: crate::dnn::ModelTraitConst {
		fn as_raw_DetectionModel(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::DetectionModel]
	pub trait DetectionModelTrait: crate::dnn::DetectionModelTraitConst + crate::dnn::ModelTrait {
		fn as_raw_mut_DetectionModel(&mut self) -> *mut c_void;
	
		/// nmsAcrossClasses defaults to false,
		/// such that when non max suppression is used during the detect() function, it will do so per-class.
		/// This function allows you to toggle this behaviour.
		/// ## Parameters
		/// * value: The new value for nmsAcrossClasses
		#[inline]
		fn set_nms_across_classes(&mut self, value: bool) -> Result<crate::dnn::DetectionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_setNmsAcrossClasses_bool(self.as_raw_mut_DetectionModel(), value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Getter for nmsAcrossClasses. This variable defaults to false,
		/// such that when non max suppression is used during the detect() function, it will do so only per-class
		#[inline]
		fn get_nms_across_classes(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_getNmsAcrossClasses(self.as_raw_mut_DetectionModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Given the @p input frame, create input blob, run net and return result detections.
		/// ## Parameters
		/// * frame: The input image.
		/// * classIds:[out] Class indexes in result detection.
		/// * confidences:[out] A set of corresponding confidences.
		/// * boxes:[out] A set of bounding boxes.
		/// * confThreshold: A threshold used to filter boxes by confidences.
		/// * nmsThreshold: A threshold used in non maximum suppression.
		/// 
		/// ## C++ default parameters
		/// * conf_threshold: 0.5f
		/// * nms_threshold: 0.0f
		#[inline]
		fn detect(&mut self, frame: &impl core::ToInputArray, class_ids: &mut core::Vector<i32>, confidences: &mut core::Vector<f32>, boxes: &mut core::Vector<core::Rect>, conf_threshold: f32, nms_threshold: f32) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_detect_const__InputArrayR_vectorLintGR_vectorLfloatGR_vectorLRectGR_float_float(self.as_raw_mut_DetectionModel(), frame.as_raw__InputArray(), class_ids.as_raw_mut_VectorOfi32(), confidences.as_raw_mut_VectorOff32(), boxes.as_raw_mut_VectorOfRect(), conf_threshold, nms_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Given the @p input frame, create input blob, run net and return result detections.
		/// ## Parameters
		/// * frame: The input image.
		/// * classIds:[out] Class indexes in result detection.
		/// * confidences:[out] A set of corresponding confidences.
		/// * boxes:[out] A set of bounding boxes.
		/// * confThreshold: A threshold used to filter boxes by confidences.
		/// * nmsThreshold: A threshold used in non maximum suppression.
		/// 
		/// ## Note
		/// This alternative version of [detect] function uses the following default values for its arguments:
		/// * conf_threshold: 0.5f
		/// * nms_threshold: 0.0f
		#[inline]
		fn detect_def(&mut self, frame: &impl core::ToInputArray, class_ids: &mut core::Vector<i32>, confidences: &mut core::Vector<f32>, boxes: &mut core::Vector<core::Rect>) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_detect_const__InputArrayR_vectorLintGR_vectorLfloatGR_vectorLRectGR(self.as_raw_mut_DetectionModel(), frame.as_raw__InputArray(), class_ids.as_raw_mut_VectorOfi32(), confidences.as_raw_mut_VectorOff32(), boxes.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// This class represents high-level API for object detection networks.
	/// 
	/// DetectionModel allows to set params for preprocessing input image.
	/// DetectionModel creates net from file with trained weights and config,
	/// sets preprocessing input, runs forward pass and return result detections.
	/// For DetectionModel SSD, Faster R-CNN, YOLO topologies are supported.
	pub struct DetectionModel {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DetectionModel }
	
	impl Drop for DetectionModel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_DetectionModel_delete(self.as_raw_mut_DetectionModel()) };
		}
	}
	
	unsafe impl Send for DetectionModel {}
	
	impl crate::dnn::ModelTraitConst for DetectionModel {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for DetectionModel {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DetectionModelTraitConst for DetectionModel {
		#[inline] fn as_raw_DetectionModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::DetectionModelTrait for DetectionModel {
		#[inline] fn as_raw_mut_DetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectionModel {
		/// Create detection model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn new(model: &str, config: &str) -> Result<crate::dnn::DetectionModel> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_DetectionModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create detection model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn new_def(model: &str) -> Result<crate::dnn::DetectionModel> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_DetectionModel_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create model from deep learning network.
		/// ## Parameters
		/// * network: Net object.
		#[inline]
		pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::DetectionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_DetectionModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::dnn::DetectionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DetectionModel_DetectionModel(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DetectionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for DetectionModel {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_DetectionModel_implicitClone_const(self.as_raw_DetectionModel())) }
		}
	}
	
	boxed_cast_base! { DetectionModel, crate::dnn::Model, cv_dnn_DetectionModel_to_Model }
	
	impl std::fmt::Debug for DetectionModel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DetectionModel")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::DetectionOutputLayer]
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
	pub struct DetectionOutputLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for DetectionOutputLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for DetectionOutputLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DetectionOutputLayerTraitConst for DetectionOutputLayer {
		#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::DetectionOutputLayerTrait for DetectionOutputLayer {
		#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DetectionOutputLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::DetectionOutputLayer>> {
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
	
	/// Constant methods for [crate::dnn::Dict]
	pub trait DictTraitConst {
		fn as_raw_Dict(&self) -> *const c_void;
	
		/// Checks a presence of the @p key in the dictionary.
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
		#[inline]
		fn set(&mut self, key: &str, value: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
			extern_container_arg!(key);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Dict_set_const_cv_dnn_DictValue_const_StringR_const_DictValueR(self.as_raw_mut_Dict(), key.opencv_as_extern(), value.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
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
	pub struct Dict {
		ptr: *mut c_void
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
	
	impl Dict {
	}
	
	impl std::fmt::Debug for Dict {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Dict")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::DictValue]
	pub trait DictValueTraitConst {
		fn as_raw_DictValue(&self) -> *const c_void;
	
		/// ## C++ default parameters
		/// * idx: -1
		#[inline]
		fn get_str(&self, idx: i32) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_get_cv_String_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
		#[inline]
		fn get_f64(&self, idx: i32) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_get_double_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn get_i32(&self, idx: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_get_int_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn get_i64(&self, idx: i32) -> Result<i64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_get_int64_t_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_i64_def(&self) -> Result<i64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_get_int64_t_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_size_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_int(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_isInt_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_string(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_isString_const(self.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn get_int_value(&self, idx: i32) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_getIntValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [get_int_value] function uses the following default values for its arguments:
		/// * idx: -1
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
		#[inline]
		fn get_real_value(&self, idx: i32) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_getRealValue_const_int(self.as_raw_DictValue(), idx, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [get_real_value] function uses the following default values for its arguments:
		/// * idx: -1
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
		/// This alternative version of [get_string_value] function uses the following default values for its arguments:
		/// * idx: -1
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
	
	}
	
	/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
	/// @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
	pub struct DictValue {
		ptr: *mut c_void
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
	
	impl DictValue {
		#[inline]
		pub fn copy(r: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_DictValue_const_DictValueR(r.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
		#[inline]
		pub fn from_i64_def() -> Result<crate::dnn::DictValue> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_DictValue(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_DictValue_int(i, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_DictValue_unsigned_int(p, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_DictValue_DictValue_double(p, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::DictValue::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
	pub trait ELULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ELULayer(&self) -> *const c_void;
	
		#[inline]
		fn alpha(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ELULayer_propAlpha_const(self.as_raw_ELULayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ELULayer]
	pub trait ELULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ELULayerTraitConst {
		fn as_raw_mut_ELULayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_alpha(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ELULayer_propAlpha_float(self.as_raw_mut_ELULayer(), val) };
			ret
		}
		
	}
	
	pub struct ELULayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for ELULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ELULayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ELULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ELULayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ELULayerTraitConst for ELULayer {
		#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ELULayerTrait for ELULayer {
		#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ELULayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ELULayer>> {
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
	/// - "operation" as string. Values are "sum" (default), "prod", "max", "div", "min"
	/// - "coeff" as float array. Specify weights of inputs for SUM operation
	/// - "output_channels_mode" as string. Values are "same" (default, all input must have the same layout), "input_0", "input_0_truncate", "max_input_channels"
	pub struct EltwiseLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for EltwiseLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for EltwiseLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerTraitConst for EltwiseLayer {
		#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerTrait for EltwiseLayer {
		#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl EltwiseLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::EltwiseLayer>> {
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
	
	/// Constant methods for [crate::dnn::EltwiseLayerInt8]
	pub trait EltwiseLayerInt8TraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_EltwiseLayerInt8(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::EltwiseLayerInt8]
	pub trait EltwiseLayerInt8Trait: crate::dnn::EltwiseLayerInt8TraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void;
	
	}
	
	pub struct EltwiseLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { EltwiseLayerInt8 }
	
	impl Drop for EltwiseLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_EltwiseLayerInt8_delete(self.as_raw_mut_EltwiseLayerInt8()) };
		}
	}
	
	unsafe impl Send for EltwiseLayerInt8 {}
	
	impl core::AlgorithmTraitConst for EltwiseLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for EltwiseLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for EltwiseLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for EltwiseLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerInt8TraitConst for EltwiseLayerInt8 {
		#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerInt8Trait for EltwiseLayerInt8 {
		#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl EltwiseLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::EltwiseLayerInt8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_EltwiseLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::EltwiseLayerInt8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { EltwiseLayerInt8, core::Algorithm, cv_dnn_EltwiseLayerInt8_to_Algorithm }
	
	boxed_cast_base! { EltwiseLayerInt8, crate::dnn::Layer, cv_dnn_EltwiseLayerInt8_to_Layer }
	
	impl std::fmt::Debug for EltwiseLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("EltwiseLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ErfLayer]
	pub trait ErfLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ErfLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ErfLayer]
	pub trait ErfLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ErfLayerTraitConst {
		fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ErfLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ErfLayer }
	
	impl Drop for ErfLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ErfLayer_delete(self.as_raw_mut_ErfLayer()) };
		}
	}
	
	unsafe impl Send for ErfLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for ErfLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for ErfLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for ErfLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ErfLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ErfLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ErfLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ErfLayerTraitConst for ErfLayer {
		#[inline] fn as_raw_ErfLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ErfLayerTrait for ErfLayer {
		#[inline] fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ErfLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ErfLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ErfLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ErfLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ErfLayer, crate::dnn::ActivationLayer, cv_dnn_ErfLayer_to_ActivationLayer }
	
	boxed_cast_base! { ErfLayer, core::Algorithm, cv_dnn_ErfLayer_to_Algorithm }
	
	boxed_cast_base! { ErfLayer, crate::dnn::Layer, cv_dnn_ErfLayer_to_Layer }
	
	impl std::fmt::Debug for ErfLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ErfLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ExpLayer]
	pub trait ExpLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ExpLayer(&self) -> *const c_void;
	
		#[inline]
		fn base(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ExpLayer_propBase_const(self.as_raw_ExpLayer()) };
			ret
		}
		
		#[inline]
		fn scale(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ExpLayer_propScale_const(self.as_raw_ExpLayer()) };
			ret
		}
		
		#[inline]
		fn shift(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ExpLayer_propShift_const(self.as_raw_ExpLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ExpLayer]
	pub trait ExpLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ExpLayerTraitConst {
		fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_base(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ExpLayer_propBase_float(self.as_raw_mut_ExpLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_scale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ExpLayer_propScale_float(self.as_raw_mut_ExpLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_shift(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ExpLayer_propShift_float(self.as_raw_mut_ExpLayer(), val) };
			ret
		}
		
	}
	
	pub struct ExpLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for ExpLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ExpLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ExpLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ExpLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ExpLayerTraitConst for ExpLayer {
		#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ExpLayerTrait for ExpLayer {
		#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ExpLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ExpLayer>> {
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
	pub trait FlattenLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_FlattenLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::FlattenLayer]
	pub trait FlattenLayerTrait: crate::dnn::FlattenLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct FlattenLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for FlattenLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for FlattenLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlattenLayerTraitConst for FlattenLayer {
		#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::FlattenLayerTrait for FlattenLayer {
		#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FlattenLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::FlattenLayer>> {
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
	
	/// Constant methods for [crate::dnn::FloorLayer]
	pub trait FloorLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_FloorLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::FloorLayer]
	pub trait FloorLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::FloorLayerTraitConst {
		fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct FloorLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FloorLayer }
	
	impl Drop for FloorLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_FloorLayer_delete(self.as_raw_mut_FloorLayer()) };
		}
	}
	
	unsafe impl Send for FloorLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for FloorLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for FloorLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for FloorLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FloorLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for FloorLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for FloorLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::FloorLayerTraitConst for FloorLayer {
		#[inline] fn as_raw_FloorLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::FloorLayerTrait for FloorLayer {
		#[inline] fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FloorLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::FloorLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_FloorLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::FloorLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { FloorLayer, crate::dnn::ActivationLayer, cv_dnn_FloorLayer_to_ActivationLayer }
	
	boxed_cast_base! { FloorLayer, core::Algorithm, cv_dnn_FloorLayer_to_Algorithm }
	
	boxed_cast_base! { FloorLayer, crate::dnn::Layer, cv_dnn_FloorLayer_to_Layer }
	
	impl std::fmt::Debug for FloorLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FloorLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::FlowWarpLayer]
	pub trait FlowWarpLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_FlowWarpLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::FlowWarpLayer]
	pub trait FlowWarpLayerTrait: crate::dnn::FlowWarpLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct FlowWarpLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for FlowWarpLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for FlowWarpLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlowWarpLayerTraitConst for FlowWarpLayer {
		#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::FlowWarpLayerTrait for FlowWarpLayer {
		#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FlowWarpLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::FlowWarpLayer>> {
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
	
	/// Constant methods for [crate::dnn::GRULayer]
	pub trait GRULayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_GRULayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::GRULayer]
	pub trait GRULayerTrait: crate::dnn::GRULayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_GRULayer(&mut self) -> *mut c_void;
	
	}
	
	/// GRU recurrent one-layer
	/// 
	/// Accepts input sequence and computes the final hidden state for each element in the batch.
	/// 
	/// - input[0] containing the features of the input sequence.
	/// input[0] should have shape [`T`, `N`, `data_dims`] where `T` is sequence length, `N` is batch size, `data_dims` is input size
	/// - output would have shape [`T`, `N`, `D` * `hidden_size`] where `D = 2` if layer is bidirectional otherwise `D = 1`
	/// 
	/// Depends on the following attributes:
	/// - hidden_size - Number of neurons in the hidden layer
	/// - direction - RNN could be bidirectional or forward
	/// 
	/// The final hidden state @f$ h_t @f$ computes by the following formulas:
	/// 
	///   @f{eqnarray*}{
	///   r_t = \sigma(W_{ir} x_t + b_{ir} + W_{hr} h_{(t-1)} + b_{hr}) \\
	///   z_t = \sigma(W_{iz} x_t + b_{iz} + W_{hz} h_{(t-1)} + b_{hz}) \\
	///   n_t = \tanh(W_{in} x_t + b_{in} + r_t \odot (W_{hn} h_{(t-1)}+ b_{hn})) \\
	///   h_t = (1 - z_t) \odot n_t + z_t \odot h_{(t-1)} \\
	///   @f}
	/// Where @f$x_t@f$ is current input, @f$h_{(t-1)}@f$ is previous or initial hidden state.
	/// 
	/// @f$W_{x?}@f$, @f$W_{h?}@f$ and @f$b_{?}@f$ are learned weights represented as matrices:
	/// @f$W_{x?} \in R^{N_h \times N_x}@f$, @f$W_{h?} \in R^{N_h \times N_h}@f$, @f$b_? \in R^{N_h}@f$.
	/// 
	/// @f$\odot@f$ is per-element multiply operation.
	pub struct GRULayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GRULayer }
	
	impl Drop for GRULayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_GRULayer_delete(self.as_raw_mut_GRULayer()) };
		}
	}
	
	unsafe impl Send for GRULayer {}
	
	impl core::AlgorithmTraitConst for GRULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for GRULayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for GRULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for GRULayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::GRULayerTraitConst for GRULayer {
		#[inline] fn as_raw_GRULayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::GRULayerTrait for GRULayer {
		#[inline] fn as_raw_mut_GRULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GRULayer {
		/// Creates instance of GRU layer
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::GRULayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_GRULayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::GRULayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { GRULayer, core::Algorithm, cv_dnn_GRULayer_to_Algorithm }
	
	boxed_cast_base! { GRULayer, crate::dnn::Layer, cv_dnn_GRULayer_to_Layer }
	
	impl std::fmt::Debug for GRULayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GRULayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::GatherLayer]
	pub trait GatherLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_GatherLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::GatherLayer]
	pub trait GatherLayerTrait: crate::dnn::GatherLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_GatherLayer(&mut self) -> *mut c_void;
	
	}
	
	/// Gather layer
	pub struct GatherLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GatherLayer }
	
	impl Drop for GatherLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_GatherLayer_delete(self.as_raw_mut_GatherLayer()) };
		}
	}
	
	unsafe impl Send for GatherLayer {}
	
	impl core::AlgorithmTraitConst for GatherLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for GatherLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for GatherLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for GatherLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::GatherLayerTraitConst for GatherLayer {
		#[inline] fn as_raw_GatherLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::GatherLayerTrait for GatherLayer {
		#[inline] fn as_raw_mut_GatherLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GatherLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::GatherLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_GatherLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::GatherLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { GatherLayer, core::Algorithm, cv_dnn_GatherLayer_to_Algorithm }
	
	boxed_cast_base! { GatherLayer, crate::dnn::Layer, cv_dnn_GatherLayer_to_Layer }
	
	impl std::fmt::Debug for GatherLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GatherLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::GeluApproximationLayer]
	pub trait GeluApproximationLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_GeluApproximationLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::GeluApproximationLayer]
	pub trait GeluApproximationLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::GeluApproximationLayerTraitConst {
		fn as_raw_mut_GeluApproximationLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct GeluApproximationLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GeluApproximationLayer }
	
	impl Drop for GeluApproximationLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_GeluApproximationLayer_delete(self.as_raw_mut_GeluApproximationLayer()) };
		}
	}
	
	unsafe impl Send for GeluApproximationLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for GeluApproximationLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for GeluApproximationLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for GeluApproximationLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for GeluApproximationLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for GeluApproximationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for GeluApproximationLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::GeluApproximationLayerTraitConst for GeluApproximationLayer {
		#[inline] fn as_raw_GeluApproximationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::GeluApproximationLayerTrait for GeluApproximationLayer {
		#[inline] fn as_raw_mut_GeluApproximationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GeluApproximationLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::GeluApproximationLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_GeluApproximationLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::GeluApproximationLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { GeluApproximationLayer, crate::dnn::ActivationLayer, cv_dnn_GeluApproximationLayer_to_ActivationLayer }
	
	boxed_cast_base! { GeluApproximationLayer, core::Algorithm, cv_dnn_GeluApproximationLayer_to_Algorithm }
	
	boxed_cast_base! { GeluApproximationLayer, crate::dnn::Layer, cv_dnn_GeluApproximationLayer_to_Layer }
	
	impl std::fmt::Debug for GeluApproximationLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GeluApproximationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::GeluLayer]
	pub trait GeluLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_GeluLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::GeluLayer]
	pub trait GeluLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::GeluLayerTraitConst {
		fn as_raw_mut_GeluLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct GeluLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GeluLayer }
	
	impl Drop for GeluLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_GeluLayer_delete(self.as_raw_mut_GeluLayer()) };
		}
	}
	
	unsafe impl Send for GeluLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for GeluLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for GeluLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for GeluLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for GeluLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for GeluLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for GeluLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::GeluLayerTraitConst for GeluLayer {
		#[inline] fn as_raw_GeluLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::GeluLayerTrait for GeluLayer {
		#[inline] fn as_raw_mut_GeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GeluLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::GeluLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_GeluLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::GeluLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { GeluLayer, crate::dnn::ActivationLayer, cv_dnn_GeluLayer_to_ActivationLayer }
	
	boxed_cast_base! { GeluLayer, core::Algorithm, cv_dnn_GeluLayer_to_Algorithm }
	
	boxed_cast_base! { GeluLayer, crate::dnn::Layer, cv_dnn_GeluLayer_to_Layer }
	
	impl std::fmt::Debug for GeluLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GeluLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::HardSigmoidLayer]
	pub trait HardSigmoidLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_HardSigmoidLayer(&self) -> *const c_void;
	
		#[inline]
		fn alpha(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_propAlpha_const(self.as_raw_HardSigmoidLayer()) };
			ret
		}
		
		#[inline]
		fn beta(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_propBeta_const(self.as_raw_HardSigmoidLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::HardSigmoidLayer]
	pub trait HardSigmoidLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::HardSigmoidLayerTraitConst {
		fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_alpha(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_propAlpha_float(self.as_raw_mut_HardSigmoidLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_beta(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_HardSigmoidLayer_propBeta_float(self.as_raw_mut_HardSigmoidLayer(), val) };
			ret
		}
		
	}
	
	pub struct HardSigmoidLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { HardSigmoidLayer }
	
	impl Drop for HardSigmoidLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_HardSigmoidLayer_delete(self.as_raw_mut_HardSigmoidLayer()) };
		}
	}
	
	unsafe impl Send for HardSigmoidLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for HardSigmoidLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for HardSigmoidLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for HardSigmoidLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for HardSigmoidLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for HardSigmoidLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for HardSigmoidLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSigmoidLayerTraitConst for HardSigmoidLayer {
		#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::HardSigmoidLayerTrait for HardSigmoidLayer {
		#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl HardSigmoidLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::HardSigmoidLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_HardSigmoidLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::HardSigmoidLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { HardSigmoidLayer, crate::dnn::ActivationLayer, cv_dnn_HardSigmoidLayer_to_ActivationLayer }
	
	boxed_cast_base! { HardSigmoidLayer, core::Algorithm, cv_dnn_HardSigmoidLayer_to_Algorithm }
	
	boxed_cast_base! { HardSigmoidLayer, crate::dnn::Layer, cv_dnn_HardSigmoidLayer_to_Layer }
	
	impl std::fmt::Debug for HardSigmoidLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("HardSigmoidLayer")
				.field("alpha", &crate::dnn::HardSigmoidLayerTraitConst::alpha(self))
				.field("beta", &crate::dnn::HardSigmoidLayerTraitConst::beta(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::HardSwishLayer]
	pub trait HardSwishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_HardSwishLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::HardSwishLayer]
	pub trait HardSwishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::HardSwishLayerTraitConst {
		fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct HardSwishLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { HardSwishLayer }
	
	impl Drop for HardSwishLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_HardSwishLayer_delete(self.as_raw_mut_HardSwishLayer()) };
		}
	}
	
	unsafe impl Send for HardSwishLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for HardSwishLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for HardSwishLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for HardSwishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for HardSwishLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for HardSwishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for HardSwishLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSwishLayerTraitConst for HardSwishLayer {
		#[inline] fn as_raw_HardSwishLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::HardSwishLayerTrait for HardSwishLayer {
		#[inline] fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl HardSwishLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::HardSwishLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_HardSwishLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::HardSwishLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { HardSwishLayer, crate::dnn::ActivationLayer, cv_dnn_HardSwishLayer_to_ActivationLayer }
	
	boxed_cast_base! { HardSwishLayer, core::Algorithm, cv_dnn_HardSwishLayer_to_Algorithm }
	
	boxed_cast_base! { HardSwishLayer, crate::dnn::Layer, cv_dnn_HardSwishLayer_to_Layer }
	
	impl std::fmt::Debug for HardSwishLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("HardSwishLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Processing params of image to blob.
	/// 
	/// It includes all possible image processing operations and corresponding parameters.
	/// ## See also
	/// blobFromImageWithParams
	/// 
	/// 
	/// Note:
	/// The order and usage of `scalefactor` and `mean` are (input - mean) * scalefactor.
	/// The order and usage of `scalefactor`, `size`, `mean`, `swapRB`, and `ddepth` are consistent
	/// with the function of [blobFromImage].
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct Image2BlobParams {
		/// scalefactor multiplier for input image values.
		pub scalefactor: core::Scalar,
		/// Spatial size for output image.
		pub size: core::Size,
		/// Scalar with mean values which are subtracted from channels.
		pub mean: core::Scalar,
		/// Flag which indicates that swap first and last channels
		pub swap_rb: bool,
		/// Depth of output blob. Choose CV_32F or CV_8U.
		pub ddepth: i32,
		/// Order of output dimensions. Choose DNN_LAYOUT_NCHW or DNN_LAYOUT_NHWC.
		pub datalayout: crate::dnn::DataLayout,
		/// Image padding mode. see also: ImagePaddingMode.
		pub paddingmode: crate::dnn::ImagePaddingMode,
	}
	
	opencv_type_simple! { crate::dnn::Image2BlobParams }
	
	impl Image2BlobParams {
		#[inline]
		pub fn default() -> Result<crate::dnn::Image2BlobParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Image2BlobParams_Image2BlobParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * size: Size()
		/// * mean: Scalar()
		/// * swap_rb: false
		/// * ddepth: CV_32F
		/// * datalayout: DNN_LAYOUT_NCHW
		/// * mode: DNN_PMODE_NULL
		#[inline]
		pub fn new(scalefactor: core::Scalar, size: core::Size, mean: core::Scalar, swap_rb: bool, ddepth: i32, datalayout: crate::dnn::DataLayout, mode: crate::dnn::ImagePaddingMode) -> Result<crate::dnn::Image2BlobParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Image2BlobParams_Image2BlobParams_const_ScalarR_const_SizeR_const_ScalarR_bool_int_DataLayout_ImagePaddingMode(&scalefactor, &size, &mean, swap_rb, ddepth, datalayout, mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * size: Size()
		/// * mean: Scalar()
		/// * swap_rb: false
		/// * ddepth: CV_32F
		/// * datalayout: DNN_LAYOUT_NCHW
		/// * mode: DNN_PMODE_NULL
		#[inline]
		pub fn new_def(scalefactor: core::Scalar) -> Result<crate::dnn::Image2BlobParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Image2BlobParams_Image2BlobParams_const_ScalarR(&scalefactor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::dnn::InnerProductLayer]
	pub trait InnerProductLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_InnerProductLayer(&self) -> *const c_void;
	
		#[inline]
		fn axis(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_InnerProductLayer_propAxis_const(self.as_raw_InnerProductLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::InnerProductLayer]
	pub trait InnerProductLayerTrait: crate::dnn::InnerProductLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_axis(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_InnerProductLayer_propAxis_int(self.as_raw_mut_InnerProductLayer(), val) };
			ret
		}
		
	}
	
	/// `InnerProduct`, `MatMul` and `Gemm` operations are all implemented by Fully Connected Layer.
	/// Parameter `is_matmul` is used to distinguish `MatMul` and `Gemm` from `InnerProduct`.
	pub struct InnerProductLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for InnerProductLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for InnerProductLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for InnerProductLayer {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for InnerProductLayer {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl InnerProductLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::InnerProductLayer>> {
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
	
	/// Constant methods for [crate::dnn::InnerProductLayerInt8]
	pub trait InnerProductLayerInt8TraitConst: crate::dnn::InnerProductLayerTraitConst {
		fn as_raw_InnerProductLayerInt8(&self) -> *const c_void;
	
		#[inline]
		fn input_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propInput_zp_const(self.as_raw_InnerProductLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propOutput_zp_const(self.as_raw_InnerProductLayerInt8()) };
			ret
		}
		
		#[inline]
		fn input_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propInput_sc_const(self.as_raw_InnerProductLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propOutput_sc_const(self.as_raw_InnerProductLayerInt8()) };
			ret
		}
		
		#[inline]
		fn per_channel(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propPer_channel_const(self.as_raw_InnerProductLayerInt8()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::InnerProductLayerInt8]
	pub trait InnerProductLayerInt8Trait: crate::dnn::InnerProductLayerInt8TraitConst + crate::dnn::InnerProductLayerTrait {
		fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_input_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propInput_zp_int(self.as_raw_mut_InnerProductLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propOutput_zp_int(self.as_raw_mut_InnerProductLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_input_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propInput_sc_float(self.as_raw_mut_InnerProductLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propOutput_sc_float(self.as_raw_mut_InnerProductLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_per_channel(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_InnerProductLayerInt8_propPer_channel_bool(self.as_raw_mut_InnerProductLayerInt8(), val) };
			ret
		}
		
	}
	
	pub struct InnerProductLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { InnerProductLayerInt8 }
	
	impl Drop for InnerProductLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_InnerProductLayerInt8_delete(self.as_raw_mut_InnerProductLayerInt8()) };
		}
	}
	
	unsafe impl Send for InnerProductLayerInt8 {}
	
	impl core::AlgorithmTraitConst for InnerProductLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for InnerProductLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for InnerProductLayerInt8 {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for InnerProductLayerInt8 {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for InnerProductLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for InnerProductLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerInt8TraitConst for InnerProductLayerInt8 {
		#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerInt8Trait for InnerProductLayerInt8 {
		#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl InnerProductLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::InnerProductLayerInt8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_InnerProductLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::InnerProductLayerInt8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { InnerProductLayerInt8, core::Algorithm, cv_dnn_InnerProductLayerInt8_to_Algorithm }
	
	boxed_cast_base! { InnerProductLayerInt8, crate::dnn::InnerProductLayer, cv_dnn_InnerProductLayerInt8_to_InnerProductLayer }
	
	boxed_cast_base! { InnerProductLayerInt8, crate::dnn::Layer, cv_dnn_InnerProductLayerInt8_to_Layer }
	
	impl std::fmt::Debug for InnerProductLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("InnerProductLayerInt8")
				.field("input_zp", &crate::dnn::InnerProductLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::InnerProductLayerInt8TraitConst::output_zp(self))
				.field("input_sc", &crate::dnn::InnerProductLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::InnerProductLayerInt8TraitConst::output_sc(self))
				.field("per_channel", &crate::dnn::InnerProductLayerInt8TraitConst::per_channel(self))
				.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::InterpLayer]
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
	pub struct InterpLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for InterpLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for InterpLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::InterpLayerTraitConst for InterpLayer {
		#[inline] fn as_raw_InterpLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::InterpLayerTrait for InterpLayer {
		#[inline] fn as_raw_mut_InterpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl InterpLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::KeypointsModel]
	pub trait KeypointsModelTraitConst: crate::dnn::ModelTraitConst {
		fn as_raw_KeypointsModel(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::KeypointsModel]
	pub trait KeypointsModelTrait: crate::dnn::KeypointsModelTraitConst + crate::dnn::ModelTrait {
		fn as_raw_mut_KeypointsModel(&mut self) -> *mut c_void;
	
		/// Given the @p input frame, create input blob, run net
		/// ## Parameters
		/// * frame: The input image.
		/// * thresh: minimum confidence threshold to select a keypoint
		/// ## Returns
		/// a vector holding the x and y coordinates of each detected keypoint
		/// 
		/// ## C++ default parameters
		/// * thresh: 0.5
		#[inline]
		fn estimate(&mut self, frame: &impl core::ToInputArray, thresh: f32) -> Result<core::Vector<core::Point2f>> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_KeypointsModel_estimate_const__InputArrayR_float(self.as_raw_mut_KeypointsModel(), frame.as_raw__InputArray(), thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Given the @p input frame, create input blob, run net
		/// ## Parameters
		/// * frame: The input image.
		/// * thresh: minimum confidence threshold to select a keypoint
		/// ## Returns
		/// a vector holding the x and y coordinates of each detected keypoint
		/// 
		/// ## Note
		/// This alternative version of [estimate] function uses the following default values for its arguments:
		/// * thresh: 0.5
		#[inline]
		fn estimate_def(&mut self, frame: &impl core::ToInputArray) -> Result<core::Vector<core::Point2f>> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_KeypointsModel_estimate_const__InputArrayR(self.as_raw_mut_KeypointsModel(), frame.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// This class represents high-level API for keypoints models
	/// 
	/// KeypointsModel allows to set params for preprocessing input image.
	/// KeypointsModel creates net from file with trained weights and config,
	/// sets preprocessing input, runs forward pass and returns the x and y coordinates of each detected keypoint
	pub struct KeypointsModel {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { KeypointsModel }
	
	impl Drop for KeypointsModel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_KeypointsModel_delete(self.as_raw_mut_KeypointsModel()) };
		}
	}
	
	unsafe impl Send for KeypointsModel {}
	
	impl crate::dnn::ModelTraitConst for KeypointsModel {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for KeypointsModel {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::KeypointsModelTraitConst for KeypointsModel {
		#[inline] fn as_raw_KeypointsModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::KeypointsModelTrait for KeypointsModel {
		#[inline] fn as_raw_mut_KeypointsModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl KeypointsModel {
		/// Create keypoints model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn new(model: &str, config: &str) -> Result<crate::dnn::KeypointsModel> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_KeypointsModel_KeypointsModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::KeypointsModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create keypoints model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn new_def(model: &str) -> Result<crate::dnn::KeypointsModel> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_KeypointsModel_KeypointsModel_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::KeypointsModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create model from deep learning network.
		/// ## Parameters
		/// * network: Net object.
		#[inline]
		pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::KeypointsModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_KeypointsModel_KeypointsModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::KeypointsModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for KeypointsModel {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_KeypointsModel_implicitClone_const(self.as_raw_KeypointsModel())) }
		}
	}
	
	boxed_cast_base! { KeypointsModel, crate::dnn::Model, cv_dnn_KeypointsModel_to_Model }
	
	impl std::fmt::Debug for KeypointsModel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("KeypointsModel")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::LRNLayer]
	pub trait LRNLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_LRNLayer(&self) -> *const c_void;
	
		#[inline]
		fn typ(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propType_const(self.as_raw_LRNLayer()) };
			ret
		}
		
		#[inline]
		fn size(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propSize_const(self.as_raw_LRNLayer()) };
			ret
		}
		
		#[inline]
		fn alpha(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propAlpha_const(self.as_raw_LRNLayer()) };
			ret
		}
		
		#[inline]
		fn beta(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propBeta_const(self.as_raw_LRNLayer()) };
			ret
		}
		
		#[inline]
		fn bias(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propBias_const(self.as_raw_LRNLayer()) };
			ret
		}
		
		#[inline]
		fn norm_by_size(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propNormBySize_const(self.as_raw_LRNLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::LRNLayer]
	pub trait LRNLayerTrait: crate::dnn::LRNLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_type(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propType_int(self.as_raw_mut_LRNLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_size(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propSize_int(self.as_raw_mut_LRNLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_alpha(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propAlpha_float(self.as_raw_mut_LRNLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_beta(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propBeta_float(self.as_raw_mut_LRNLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_bias(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propBias_float(self.as_raw_mut_LRNLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_norm_by_size(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_LRNLayer_propNormBySize_bool(self.as_raw_mut_LRNLayer(), val) };
			ret
		}
		
	}
	
	pub struct LRNLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for LRNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for LRNLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LRNLayerTraitConst for LRNLayer {
		#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LRNLayerTrait for LRNLayer {
		#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LRNLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::LRNLayer>> {
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
	
	/// Constant methods for [crate::dnn::LSTMLayer]
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
		#[inline]
		fn set_weights(&mut self, wh: &core::Mat, wx: &core::Mat, b: &core::Mat) -> Result<()> {
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
		/// This alternative version of [set_out_shape] function uses the following default values for its arguments:
		/// * out_tail_shape: MatShape()
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
		/// This alternative version of [set_use_timstamps_dim] function uses the following default values for its arguments:
		/// * use_: true
		#[deprecated = "Use flag `produce_cell_output` in LayerParams."]
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
		/// This alternative version of [set_produce_cell_output] function uses the following default values for its arguments:
		/// * produce: false
		#[deprecated = "Use flag `use_timestamp_dim` in LayerParams."]
		#[inline]
		fn set_produce_cell_output_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput(self.as_raw_mut_LSTMLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
			extern_container_arg!(mut input_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_mut_LSTMLayer(), input_name.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
	pub struct LSTMLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for LSTMLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for LSTMLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LSTMLayerTraitConst for LSTMLayer {
		#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LSTMLayerTrait for LSTMLayer {
		#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LSTMLayer {
		/// Creates instance of LSTM layer
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::LSTMLayer>> {
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
	pub trait LayerTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Layer(&self) -> *const c_void;
	
		/// List of learned parameters must be stored here to allow read them by using Net::getParam().
		#[inline]
		fn blobs(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_dnn_Layer_propBlobs_const(self.as_raw_Layer()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		/// Name of the layer instance, can be used for logging or other internal purposes.
		#[inline]
		fn name(&self) -> String {
			let ret = unsafe { sys::cv_dnn_Layer_propName_const(self.as_raw_Layer()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// Type name which was used for creating layer by layer factory.
		#[inline]
		fn typ(&self) -> String {
			let ret = unsafe { sys::cv_dnn_Layer_propType_const(self.as_raw_Layer()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// prefer target for layer forwarding
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
		#[inline]
		fn get_scale_shift(&self, scale: &mut core::Mat, shift: &mut core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_getScaleShift_const_MatR_MatR(self.as_raw_Layer(), scale.as_raw_mut_Mat(), shift.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns scale and zeropoint of layers
		/// ## Parameters
		/// * scale:[out] Output scale
		/// * zeropoint:[out] Output zeropoint
		/// 
		/// By default, @p scale is 1 and @p zeropoint is 0.
		#[inline]
		fn get_scale_zeropoint(&self, scale: &mut f32, zeropoint: &mut i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_getScaleZeropoint_const_floatR_intR(self.as_raw_Layer(), scale, zeropoint, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_memory_shapes(&self, inputs: &core::Vector<crate::dnn::MatShape>, required_outputs: i32, outputs: &mut core::Vector<crate::dnn::MatShape>, internals: &mut core::Vector<crate::dnn::MatShape>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_getMemoryShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(self.as_raw_Layer(), inputs.as_raw_VectorOfMatShape(), required_outputs, outputs.as_raw_mut_VectorOfMatShape(), internals.as_raw_mut_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn set_blobs(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_dnn_Layer_propBlobs_vectorLMatG(self.as_raw_mut_Layer(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		/// Name of the layer instance, can be used for logging or other internal purposes.
		#[inline]
		fn set_name(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_dnn_Layer_propName_String(self.as_raw_mut_Layer(), val.opencv_as_extern_mut()) };
			ret
		}
		
		/// Type name which was used for creating layer by layer factory.
		#[inline]
		fn set_type(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_dnn_Layer_propType_String(self.as_raw_mut_Layer(), val.opencv_as_extern_mut()) };
			ret
		}
		
		/// prefer target for layer forwarding
		#[inline]
		fn set_preferable_target(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_Layer_propPreferableTarget_int(self.as_raw_mut_Layer(), val) };
			ret
		}
		
		/// Computes and sets internal parameters according to inputs, outputs and blobs.
		/// ## Parameters
		/// * inputs: vector of already allocated input blobs
		/// * outputs:[out] vector of already allocated output blobs
		/// 
		/// If this method is called after network has allocated all memory for input and output blobs
		/// and before inferencing.
		#[inline]
		fn finalize(&mut self, inputs: &impl core::ToInputArray, outputs: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn forward(&mut self, inputs: &impl core::ToInputArray, outputs: &mut impl core::ToOutputArray, internals: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(inputs);
			output_array_arg!(outputs);
			output_array_arg!(internals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Tries to quantize the given layer and compute the quantization parameters required for fixed point implementation.
		/// ## Parameters
		/// * scales: input and output scales.
		/// * zeropoints: input and output zeropoints.
		/// * params:[out] Quantized parameters required for fixed point implementation of that layer.
		/// ## Returns
		/// True if layer can be quantized.
		#[inline]
		fn try_quantize(&mut self, scales: &core::Vector<core::Vector<f32>>, zeropoints: &core::Vector<core::Vector<i32>>, params: &mut crate::dnn::LayerParams) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_tryQuantize_const_vectorLvectorLfloatGGR_const_vectorLvectorLintGGR_LayerParamsR(self.as_raw_mut_Layer(), scales.as_raw_VectorOfVectorOff32(), zeropoints.as_raw_VectorOfVectorOfi32(), params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Given the @p input blobs, computes the output @p blobs.
		/// ## Parameters
		/// * inputs: the input blobs.
		/// * outputs:[out] allocated output blobs, which will store results of the computation.
		/// * internals:[out] allocated internal blobs
		#[inline]
		fn forward_fallback(&mut self, inputs: &impl core::ToInputArray, outputs: &mut impl core::ToOutputArray, internals: &mut impl core::ToOutputArray) -> Result<()> {
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
		///  Computes and sets internal parameters according to inputs, outputs and blobs.
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
		#[inline]
		fn finalize_mat_to(&mut self, inputs: &core::Vector<core::Mat>, outputs: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_finalize_const_vectorLMatGR_vectorLMatGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// 
		///  Computes and sets internal parameters according to inputs, outputs and blobs.
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
		#[inline]
		fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
			extern_container_arg!(mut input_name);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_mut_Layer(), input_name.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns index of output blob in output array.
		/// ## See also
		/// inputNameToIndex()
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
		#[inline]
		fn init_halide(&mut self, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_initHalide_const_vectorLPtrLBackendWrapperGGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn init_ngraph(&mut self, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, nodes: &core::Vector<core::Ptr<crate::dnn::BackendNode>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_initNgraph_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn init_vk_com(&mut self, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, outputs: &mut core::Vector<core::Ptr<crate::dnn::BackendWrapper>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_initVkCom_const_vectorLPtrLBackendWrapperGGR_vectorLPtrLBackendWrapperGGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), outputs.as_raw_mut_VectorOfPtrOfBackendWrapper(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn init_webnn(&mut self, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, nodes: &core::Vector<core::Ptr<crate::dnn::BackendNode>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_initWebnn_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns a CUDA backend node
		/// 
		/// ## Parameters
		/// * context: void pointer to CSLContext object
		/// * inputs: layer inputs
		/// * outputs: layer outputs
		#[inline]
		unsafe fn init_cuda(&mut self, context: *mut c_void, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, outputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
			return_send!(via ocvrs_return);
			{ sys::cv_dnn_Layer_initCUDA_voidX_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendWrapperGGR(self.as_raw_mut_Layer(), context, inputs.as_raw_VectorOfPtrOfBackendWrapper(), outputs.as_raw_VectorOfPtrOfBackendWrapper(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns a TimVX backend node
		/// 
		/// ## Parameters
		/// * timVxInfo: void pointer to CSLContext object
		/// * inputsWrapper: layer inputs
		/// * outputsWrapper: layer outputs
		/// * isLast: if the node is the last one of the TimVX Graph.
		#[inline]
		unsafe fn init_tim_vx(&mut self, tim_vx_info: *mut c_void, inputs_wrapper: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, outputs_wrapper: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, is_last: bool) -> Result<core::Ptr<crate::dnn::BackendNode>> {
			return_send!(via ocvrs_return);
			{ sys::cv_dnn_Layer_initTimVX_voidX_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendWrapperGGR_bool(self.as_raw_mut_Layer(), tim_vx_info, inputs_wrapper.as_raw_VectorOfPtrOfBackendWrapper(), outputs_wrapper.as_raw_VectorOfPtrOfBackendWrapper(), is_last, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = { core::Ptr::<crate::dnn::BackendNode>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns a CANN backend node
		/// 
		/// ## Parameters
		/// * inputs: input tensors of CANN operator
		/// * outputs: output tensors of CANN operator
		/// * nodes: nodes of input tensors
		#[inline]
		fn init_cann(&mut self, inputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, outputs: &core::Vector<core::Ptr<crate::dnn::BackendWrapper>>, nodes: &core::Vector<core::Ptr<crate::dnn::BackendNode>>) -> Result<core::Ptr<crate::dnn::BackendNode>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_initCann_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), outputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn try_fuse(&mut self, top: &mut core::Ptr<crate::dnn::Layer>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_tryFuse_PtrLLayerGR(self.as_raw_mut_Layer(), top.as_raw_mut_PtrOfLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// "Detaches" all the layers, attached to particular layer.
		#[inline]
		fn unset_attached(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_unsetAttached(self.as_raw_mut_Layer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn update_memory_shapes(&mut self, inputs: &core::Vector<crate::dnn::MatShape>) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_updateMemoryShapes_const_vectorLMatShapeGR(self.as_raw_mut_Layer(), inputs.as_raw_VectorOfMatShape(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_params_from(&mut self, params: &crate::dnn::LayerParams) -> Result<()> {
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
	pub struct Layer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for Layer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for Layer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Layer {
		#[inline]
		pub fn default() -> Result<crate::dnn::Layer> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Layer_Layer(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Layer::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn new(params: &crate::dnn::LayerParams) -> Result<crate::dnn::Layer> {
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
	
	boxed_cast_descendant! { Layer, crate::dnn::AcosLayer, cv_dnn_Layer_to_AcosLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::AcoshLayer, cv_dnn_Layer_to_AcoshLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ActivationLayer, cv_dnn_Layer_to_ActivationLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ActivationLayerInt8, cv_dnn_Layer_to_ActivationLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::ArgLayer, cv_dnn_Layer_to_ArgLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::AsinLayer, cv_dnn_Layer_to_AsinLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::AsinhLayer, cv_dnn_Layer_to_AsinhLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::AtanLayer, cv_dnn_Layer_to_AtanLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::AtanhLayer, cv_dnn_Layer_to_AtanhLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::BNLLLayer, cv_dnn_Layer_to_BNLLLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::BaseConvolutionLayer, cv_dnn_Layer_to_BaseConvolutionLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::BatchNormLayer, cv_dnn_Layer_to_BatchNormLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::BatchNormLayerInt8, cv_dnn_Layer_to_BatchNormLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::BlankLayer, cv_dnn_Layer_to_BlankLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CeilLayer, cv_dnn_Layer_to_CeilLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CeluLayer, cv_dnn_Layer_to_CeluLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ChannelsPReLULayer, cv_dnn_Layer_to_ChannelsPReLULayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CompareLayer, cv_dnn_Layer_to_CompareLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ConcatLayer, cv_dnn_Layer_to_ConcatLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ConstLayer, cv_dnn_Layer_to_ConstLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ConvolutionLayer, cv_dnn_Layer_to_ConvolutionLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ConvolutionLayerInt8, cv_dnn_Layer_to_ConvolutionLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::CorrelationLayer, cv_dnn_Layer_to_CorrelationLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CosLayer, cv_dnn_Layer_to_CosLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CoshLayer, cv_dnn_Layer_to_CoshLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CropAndResizeLayer, cv_dnn_Layer_to_CropAndResizeLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CropLayer, cv_dnn_Layer_to_CropLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::CumSumLayer, cv_dnn_Layer_to_CumSumLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::DataAugmentationLayer, cv_dnn_Layer_to_DataAugmentationLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::DeconvolutionLayer, cv_dnn_Layer_to_DeconvolutionLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::DequantizeLayer, cv_dnn_Layer_to_DequantizeLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::DetectionOutputLayer, cv_dnn_Layer_to_DetectionOutputLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ELULayer, cv_dnn_Layer_to_ELULayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::EltwiseLayer, cv_dnn_Layer_to_EltwiseLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::EltwiseLayerInt8, cv_dnn_Layer_to_EltwiseLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::ErfLayer, cv_dnn_Layer_to_ErfLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ExpLayer, cv_dnn_Layer_to_ExpLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::FlattenLayer, cv_dnn_Layer_to_FlattenLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::FloorLayer, cv_dnn_Layer_to_FloorLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::FlowWarpLayer, cv_dnn_Layer_to_FlowWarpLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::GRULayer, cv_dnn_Layer_to_GRULayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::GatherLayer, cv_dnn_Layer_to_GatherLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::GeluApproximationLayer, cv_dnn_Layer_to_GeluApproximationLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::GeluLayer, cv_dnn_Layer_to_GeluLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::HardSigmoidLayer, cv_dnn_Layer_to_HardSigmoidLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::HardSwishLayer, cv_dnn_Layer_to_HardSwishLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::InnerProductLayer, cv_dnn_Layer_to_InnerProductLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::InnerProductLayerInt8, cv_dnn_Layer_to_InnerProductLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::InterpLayer, cv_dnn_Layer_to_InterpLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::LRNLayer, cv_dnn_Layer_to_LRNLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::LSTMLayer, cv_dnn_Layer_to_LSTMLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::LayerNormLayer, cv_dnn_Layer_to_LayerNormLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::LogLayer, cv_dnn_Layer_to_LogLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::MVNLayer, cv_dnn_Layer_to_MVNLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::MaxUnpoolLayer, cv_dnn_Layer_to_MaxUnpoolLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::MishLayer, cv_dnn_Layer_to_MishLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::NaryEltwiseLayer, cv_dnn_Layer_to_NaryEltwiseLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::NormalizeBBoxLayer, cv_dnn_Layer_to_NormalizeBBoxLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::NotLayer, cv_dnn_Layer_to_NotLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::PaddingLayer, cv_dnn_Layer_to_PaddingLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::PermuteLayer, cv_dnn_Layer_to_PermuteLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::PoolingLayer, cv_dnn_Layer_to_PoolingLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::PoolingLayerInt8, cv_dnn_Layer_to_PoolingLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::PowerLayer, cv_dnn_Layer_to_PowerLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::PriorBoxLayer, cv_dnn_Layer_to_PriorBoxLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ProposalLayer, cv_dnn_Layer_to_ProposalLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::QuantizeLayer, cv_dnn_Layer_to_QuantizeLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::RNNLayer, cv_dnn_Layer_to_RNNLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ReLU6Layer, cv_dnn_Layer_to_ReLU6Layer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ReLULayer, cv_dnn_Layer_to_ReLULayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ReciprocalLayer, cv_dnn_Layer_to_ReciprocalLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ReduceLayer, cv_dnn_Layer_to_ReduceLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::RegionLayer, cv_dnn_Layer_to_RegionLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ReorgLayer, cv_dnn_Layer_to_ReorgLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::RequantizeLayer, cv_dnn_Layer_to_RequantizeLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ReshapeLayer, cv_dnn_Layer_to_ReshapeLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ResizeLayer, cv_dnn_Layer_to_ResizeLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::RoundLayer, cv_dnn_Layer_to_RoundLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ScaleLayer, cv_dnn_Layer_to_ScaleLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ScaleLayerInt8, cv_dnn_Layer_to_ScaleLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::ScatterLayer, cv_dnn_Layer_to_ScatterLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ScatterNDLayer, cv_dnn_Layer_to_ScatterNDLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SeluLayer, cv_dnn_Layer_to_SeluLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ShiftLayer, cv_dnn_Layer_to_ShiftLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ShiftLayerInt8, cv_dnn_Layer_to_ShiftLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::ShrinkLayer, cv_dnn_Layer_to_ShrinkLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ShuffleChannelLayer, cv_dnn_Layer_to_ShuffleChannelLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SigmoidLayer, cv_dnn_Layer_to_SigmoidLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SignLayer, cv_dnn_Layer_to_SignLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SinLayer, cv_dnn_Layer_to_SinLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SinhLayer, cv_dnn_Layer_to_SinhLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SliceLayer, cv_dnn_Layer_to_SliceLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SoftmaxLayer, cv_dnn_Layer_to_SoftmaxLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SoftmaxLayerInt8, cv_dnn_Layer_to_SoftmaxLayerInt8 }
	
	boxed_cast_descendant! { Layer, crate::dnn::SoftplusLayer, cv_dnn_Layer_to_SoftplusLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SoftsignLayer, cv_dnn_Layer_to_SoftsignLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SplitLayer, cv_dnn_Layer_to_SplitLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SqrtLayer, cv_dnn_Layer_to_SqrtLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::SwishLayer, cv_dnn_Layer_to_SwishLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::TanHLayer, cv_dnn_Layer_to_TanHLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::TanLayer, cv_dnn_Layer_to_TanLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::ThresholdedReluLayer, cv_dnn_Layer_to_ThresholdedReluLayer }
	
	boxed_cast_descendant! { Layer, crate::dnn::TileLayer, cv_dnn_Layer_to_TileLayer }
	
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
	pub trait LayerFactoryTraitConst {
		fn as_raw_LayerFactory(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::LayerFactory]
	pub trait LayerFactoryTrait: crate::dnn::LayerFactoryTraitConst {
		fn as_raw_mut_LayerFactory(&mut self) -> *mut c_void;
	
	}
	
	/// %Layer factory allows to create instances of registered layers.
	pub struct LayerFactory {
		ptr: *mut c_void
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
	
	impl LayerFactory {
		/// Registers the layer class with typename @p type and specified @p constructor. Thread-safe.
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
		#[inline]
		pub fn unregister_layer(typ: &str) -> Result<()> {
			extern_container_arg!(typ);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_const_StringR(typ.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Check if layer is registered.
		#[inline]
		pub fn is_layer_registered(typ: &str) -> Result<bool> {
			extern_container_arg!(typ);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_LayerFactory_isLayerRegistered_const_stringR(typ.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		pub fn create_layer_instance(typ: &str, params: &mut crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::LayerNormLayer]
	pub trait LayerNormLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_LayerNormLayer(&self) -> *const c_void;
	
		#[inline]
		fn has_bias(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_LayerNormLayer_propHasBias_const(self.as_raw_LayerNormLayer()) };
			ret
		}
		
		#[inline]
		fn axis(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_LayerNormLayer_propAxis_const(self.as_raw_LayerNormLayer()) };
			ret
		}
		
		#[inline]
		fn epsilon(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_LayerNormLayer_propEpsilon_const(self.as_raw_LayerNormLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::LayerNormLayer]
	pub trait LayerNormLayerTrait: crate::dnn::LayerNormLayerTraitConst + crate::dnn::LayerTrait {
		fn as_raw_mut_LayerNormLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_has_bias(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_LayerNormLayer_propHasBias_bool(self.as_raw_mut_LayerNormLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_axis(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_LayerNormLayer_propAxis_int(self.as_raw_mut_LayerNormLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_epsilon(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_LayerNormLayer_propEpsilon_float(self.as_raw_mut_LayerNormLayer(), val) };
			ret
		}
		
	}
	
	pub struct LayerNormLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { LayerNormLayer }
	
	impl Drop for LayerNormLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_LayerNormLayer_delete(self.as_raw_mut_LayerNormLayer()) };
		}
	}
	
	unsafe impl Send for LayerNormLayer {}
	
	impl core::AlgorithmTraitConst for LayerNormLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for LayerNormLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for LayerNormLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for LayerNormLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerNormLayerTraitConst for LayerNormLayer {
		#[inline] fn as_raw_LayerNormLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerNormLayerTrait for LayerNormLayer {
		#[inline] fn as_raw_mut_LayerNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LayerNormLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::LayerNormLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_LayerNormLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::LayerNormLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { LayerNormLayer, core::Algorithm, cv_dnn_LayerNormLayer_to_Algorithm }
	
	boxed_cast_base! { LayerNormLayer, crate::dnn::Layer, cv_dnn_LayerNormLayer_to_Layer }
	
	impl std::fmt::Debug for LayerNormLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LayerNormLayer")
				.field("has_bias", &crate::dnn::LayerNormLayerTraitConst::has_bias(self))
				.field("axis", &crate::dnn::LayerNormLayerTraitConst::axis(self))
				.field("epsilon", &crate::dnn::LayerNormLayerTraitConst::epsilon(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::LayerParams]
	pub trait LayerParamsTraitConst: crate::dnn::DictTraitConst {
		fn as_raw_LayerParams(&self) -> *const c_void;
	
		/// List of learned parameters stored as blobs.
		#[inline]
		fn blobs(&self) -> core::Vector<core::Mat> {
			let ret = unsafe { sys::cv_dnn_LayerParams_propBlobs_const(self.as_raw_LayerParams()) };
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			ret
		}
		
		/// Name of the layer instance (optional, can be used internal purposes).
		#[inline]
		fn name(&self) -> String {
			let ret = unsafe { sys::cv_dnn_LayerParams_propName_const(self.as_raw_LayerParams()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// Type name which was used for creating layer by layer factory (optional).
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
		#[inline]
		fn set_blobs(&mut self, mut val: core::Vector<core::Mat>) {
			let ret = unsafe { sys::cv_dnn_LayerParams_propBlobs_vectorLMatG(self.as_raw_mut_LayerParams(), val.as_raw_mut_VectorOfMat()) };
			ret
		}
		
		/// Name of the layer instance (optional, can be used internal purposes).
		#[inline]
		fn set_name(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_dnn_LayerParams_propName_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern_mut()) };
			ret
		}
		
		/// Type name which was used for creating layer by layer factory (optional).
		#[inline]
		fn set_type(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_dnn_LayerParams_propType_String(self.as_raw_mut_LayerParams(), val.opencv_as_extern_mut()) };
			ret
		}
		
	}
	
	/// This class provides all data needed to initialize layer.
	/// 
	/// It includes dictionary with scalar params (which can be read by using Dict interface),
	/// blob params [blobs] and optional meta information: [name] and [type] of layer instance.
	pub struct LayerParams {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerParamsTraitConst for LayerParams {
		#[inline] fn as_raw_LayerParams(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerParamsTrait for LayerParams {
		#[inline] fn as_raw_mut_LayerParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LayerParams {
		#[inline]
		pub fn default() -> Result<crate::dnn::LayerParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_LayerParams_new_const(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::LayerParams::opencv_from_extern(ret) };
			Ok(ret)
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
	
	/// Constant methods for [crate::dnn::LogLayer]
	pub trait LogLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_LogLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::LogLayer]
	pub trait LogLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::LogLayerTraitConst {
		fn as_raw_mut_LogLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct LogLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { LogLayer }
	
	impl Drop for LogLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_LogLayer_delete(self.as_raw_mut_LogLayer()) };
		}
	}
	
	unsafe impl Send for LogLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for LogLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for LogLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for LogLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for LogLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for LogLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for LogLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LogLayerTraitConst for LogLayer {
		#[inline] fn as_raw_LogLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LogLayerTrait for LogLayer {
		#[inline] fn as_raw_mut_LogLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LogLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::LogLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_LogLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::LogLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { LogLayer, crate::dnn::ActivationLayer, cv_dnn_LogLayer_to_ActivationLayer }
	
	boxed_cast_base! { LogLayer, core::Algorithm, cv_dnn_LogLayer_to_Algorithm }
	
	boxed_cast_base! { LogLayer, crate::dnn::Layer, cv_dnn_LogLayer_to_Layer }
	
	impl std::fmt::Debug for LogLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LogLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::MVNLayer]
	pub trait MVNLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_MVNLayer(&self) -> *const c_void;
	
		#[inline]
		fn eps(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_MVNLayer_propEps_const(self.as_raw_MVNLayer()) };
			ret
		}
		
		#[inline]
		fn norm_variance(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_MVNLayer_propNormVariance_const(self.as_raw_MVNLayer()) };
			ret
		}
		
		#[inline]
		fn across_channels(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_MVNLayer_propAcrossChannels_const(self.as_raw_MVNLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::MVNLayer]
	pub trait MVNLayerTrait: crate::dnn::LayerTrait + crate::dnn::MVNLayerTraitConst {
		fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_eps(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_MVNLayer_propEps_float(self.as_raw_mut_MVNLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_norm_variance(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_MVNLayer_propNormVariance_bool(self.as_raw_mut_MVNLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_across_channels(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_MVNLayer_propAcrossChannels_bool(self.as_raw_mut_MVNLayer(), val) };
			ret
		}
		
	}
	
	pub struct MVNLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for MVNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for MVNLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::MVNLayerTraitConst for MVNLayer {
		#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::MVNLayerTrait for MVNLayer {
		#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MVNLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::MVNLayer>> {
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
	
	/// Constant methods for [crate::dnn::MaxUnpoolLayer]
	pub trait MaxUnpoolLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_MaxUnpoolLayer(&self) -> *const c_void;
	
		#[inline]
		fn pool_kernel(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolKernel_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn pool_pad(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolPad_const(self.as_raw_MaxUnpoolLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
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
	
		#[inline]
		fn set_pool_kernel(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolKernel_Size(self.as_raw_mut_MaxUnpoolLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_pool_pad(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolPad_Size(self.as_raw_mut_MaxUnpoolLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_pool_stride(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_MaxUnpoolLayer_propPoolStride_Size(self.as_raw_mut_MaxUnpoolLayer(), val.opencv_as_extern()) };
			ret
		}
		
	}
	
	pub struct MaxUnpoolLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for MaxUnpoolLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for MaxUnpoolLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTraitConst for MaxUnpoolLayer {
		#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTrait for MaxUnpoolLayer {
		#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MaxUnpoolLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::MaxUnpoolLayer>> {
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
	
	/// Constant methods for [crate::dnn::MishLayer]
	pub trait MishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_MishLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::MishLayer]
	pub trait MishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::MishLayerTraitConst {
		fn as_raw_mut_MishLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct MishLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for MishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for MishLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for MishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for MishLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::MishLayerTraitConst for MishLayer {
		#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::MishLayerTrait for MishLayer {
		#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MishLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::MishLayer>> {
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
	
	/// Constant methods for [crate::dnn::Model]
	pub trait ModelTraitConst {
		fn as_raw_Model(&self) -> *const c_void;
	
		/// Given the @p input frame, create input blob, run net and return the output @p blobs.
		/// ## Parameters
		/// * frame: The input image.
		/// * outs:[out] Allocated output blobs, which will store results of the computation.
		#[inline]
		fn predict(&self, frame: &impl core::ToInputArray, outs: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(frame);
			output_array_arg!(outs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_predict_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Model(), frame.as_raw__InputArray(), outs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_network_(&self) -> Result<crate::dnn::Net> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_getNetwork__const(self.as_raw_Model(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::dnn::Model]
	pub trait ModelTrait: crate::dnn::ModelTraitConst {
		fn as_raw_mut_Model(&mut self) -> *mut c_void;
	
		/// Set input size for frame.
		/// ## Parameters
		/// * size: New input size.
		/// 
		/// Note: If shape of the new blob less than 0, then frame size not change.
		#[inline]
		fn set_input_size(&mut self, size: core::Size) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputSize_const_SizeR(self.as_raw_mut_Model(), &size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set input size for frame.
		/// ## Parameters
		/// * size: New input size.
		/// 
		/// Note: If shape of the new blob less than 0, then frame size not change.
		/// 
		/// ## Overloaded parameters
		/// 
		/// * width: New input width.
		/// * height: New input height.
		#[inline]
		fn set_input_size_1(&mut self, width: i32, height: i32) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputSize_int_int(self.as_raw_mut_Model(), width, height, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set mean value for frame.
		/// ## Parameters
		/// * mean: Scalar with mean values which are subtracted from channels.
		#[inline]
		fn set_input_mean(&mut self, mean: core::Scalar) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputMean_const_ScalarR(self.as_raw_mut_Model(), &mean, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set scalefactor value for frame.
		/// ## Parameters
		/// * scale: Multiplier for frame values.
		#[inline]
		fn set_input_scale(&mut self, scale: core::Scalar) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputScale_const_ScalarR(self.as_raw_mut_Model(), &scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set flag crop for frame.
		/// ## Parameters
		/// * crop: Flag which indicates whether image will be cropped after resize or not.
		#[inline]
		fn set_input_crop(&mut self, crop: bool) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputCrop_bool(self.as_raw_mut_Model(), crop, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set flag swapRB for frame.
		/// ## Parameters
		/// * swapRB: Flag which indicates that swap first and last channels.
		#[inline]
		fn set_input_swap_rb(&mut self, swap_rb: bool) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputSwapRB_bool(self.as_raw_mut_Model(), swap_rb, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set preprocessing parameters for frame.
		/// ## Parameters
		/// * size: New input size.
		/// * mean: Scalar with mean values which are subtracted from channels.
		/// * scale: Multiplier for frame values.
		/// * swapRB: Flag which indicates that swap first and last channels.
		/// * crop: Flag which indicates whether image will be cropped after resize or not.
		/// blob(n, c, y, x) = scale * resize( frame(y, x, c) ) - mean(c) )
		/// 
		/// ## C++ default parameters
		/// * scale: 1.0
		/// * size: Size()
		/// * mean: Scalar()
		/// * swap_rb: false
		/// * crop: false
		#[inline]
		fn set_input_params(&mut self, scale: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputParams_double_const_SizeR_const_ScalarR_bool_bool(self.as_raw_mut_Model(), scale, &size, &mean, swap_rb, crop, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Set preprocessing parameters for frame.
		/// ## Parameters
		/// * size: New input size.
		/// * mean: Scalar with mean values which are subtracted from channels.
		/// * scale: Multiplier for frame values.
		/// * swapRB: Flag which indicates that swap first and last channels.
		/// * crop: Flag which indicates whether image will be cropped after resize or not.
		/// blob(n, c, y, x) = scale * resize( frame(y, x, c) ) - mean(c) )
		/// 
		/// ## Note
		/// This alternative version of [set_input_params] function uses the following default values for its arguments:
		/// * scale: 1.0
		/// * size: Size()
		/// * mean: Scalar()
		/// * swap_rb: false
		/// * crop: false
		#[inline]
		fn set_input_params_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setInputParams(self.as_raw_mut_Model(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// Net::setPreferableBackend
		#[inline]
		fn set_preferable_backend(&mut self, backend_id: crate::dnn::Backend) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setPreferableBackend_Backend(self.as_raw_mut_Model(), backend_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## See also
		/// Net::setPreferableTarget
		#[inline]
		fn set_preferable_target(&mut self, target_id: crate::dnn::Target) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_setPreferableTarget_Target(self.as_raw_mut_Model(), target_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_network__1(&mut self) -> Result<crate::dnn::Net> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_getNetwork_(self.as_raw_mut_Model(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// This class is presented high-level API for neural networks.
	/// 
	/// Model allows to set params for preprocessing input image.
	/// Model creates net from file with trained weights and config,
	/// sets preprocessing input and runs forward pass.
	pub struct Model {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Model }
	
	impl Drop for Model {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_Model_delete(self.as_raw_mut_Model()) };
		}
	}
	
	unsafe impl Send for Model {}
	
	impl crate::dnn::ModelTraitConst for Model {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for Model {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Model {
		#[inline]
		pub fn default() -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_Model(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn copy(unnamed: &crate::dnn::Model) -> crate::dnn::Model {
			let ret = unsafe { sys::cv_dnn_Model_Model_const_ModelR(unnamed.as_raw_Model()) };
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		pub fn copy_mut(mut unnamed: crate::dnn::Model) -> crate::dnn::Model {
			let ret = unsafe { sys::cv_dnn_Model_Model_ModelRR(unnamed.as_raw_mut_Model()) };
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			ret
		}
		
		/// Create model from deep learning network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn new(model: &str, config: &str) -> Result<crate::dnn::Model> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_Model_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create model from deep learning network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn new_def(model: &str) -> Result<crate::dnn::Model> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_Model_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create model from deep learning network.
		/// ## Parameters
		/// * network: Net object.
		#[inline]
		pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::Model> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Model_Model_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Model::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for Model {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_Model_implicitClone_const(self.as_raw_Model())) }
		}
	}
	
	impl std::fmt::Debug for Model {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Model")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::NaryEltwiseLayer]
	pub trait NaryEltwiseLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_NaryEltwiseLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::NaryEltwiseLayer]
	pub trait NaryEltwiseLayerTrait: crate::dnn::LayerTrait + crate::dnn::NaryEltwiseLayerTraitConst {
		fn as_raw_mut_NaryEltwiseLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct NaryEltwiseLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { NaryEltwiseLayer }
	
	impl Drop for NaryEltwiseLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_NaryEltwiseLayer_delete(self.as_raw_mut_NaryEltwiseLayer()) };
		}
	}
	
	unsafe impl Send for NaryEltwiseLayer {}
	
	impl core::AlgorithmTraitConst for NaryEltwiseLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for NaryEltwiseLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for NaryEltwiseLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for NaryEltwiseLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::NaryEltwiseLayerTraitConst for NaryEltwiseLayer {
		#[inline] fn as_raw_NaryEltwiseLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::NaryEltwiseLayerTrait for NaryEltwiseLayer {
		#[inline] fn as_raw_mut_NaryEltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl NaryEltwiseLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::NaryEltwiseLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_NaryEltwiseLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::NaryEltwiseLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { NaryEltwiseLayer, core::Algorithm, cv_dnn_NaryEltwiseLayer_to_Algorithm }
	
	boxed_cast_base! { NaryEltwiseLayer, crate::dnn::Layer, cv_dnn_NaryEltwiseLayer_to_Layer }
	
	impl std::fmt::Debug for NaryEltwiseLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("NaryEltwiseLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::Net]
	pub trait NetTraitConst {
		fn as_raw_Net(&self) -> *const c_void;
	
		/// Returns true if there are no layers in the network.
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
		#[inline]
		fn get_layer_id(&self, layer: &str) -> Result<i32> {
			extern_container_arg!(layer);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_getLayerId_const_const_StringR(self.as_raw_Net(), layer.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		#[inline]
		fn get_layer_2(&self, layer_id: &crate::dnn::Net_LayerId) -> Result<core::Ptr<crate::dnn::Layer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_getLayer_const_const_LayerIdR(self.as_raw_Net(), layer_id.as_raw_DictValue(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns pointers to input layers of specific layer.
		#[inline]
		fn get_layer_inputs(&self, layer_id: i32) -> Result<core::Vector<core::Ptr<crate::dnn::Layer>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_getLayerInputs_const_int(self.as_raw_Net(), layer_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Ptr<crate::dnn::Layer>>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns input scale and zeropoint for a quantized Net.
		/// ## Parameters
		/// * scales: output parameter for returning input scales.
		/// * zeropoints: output parameter for returning input zeropoints.
		#[inline]
		fn get_input_details(&self, scales: &mut core::Vector<f32>, zeropoints: &mut core::Vector<i32>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_getInputDetails_const_vectorLfloatGR_vectorLintGR(self.as_raw_Net(), scales.as_raw_mut_VectorOff32(), zeropoints.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns output scale and zeropoint for a quantized Net.
		/// ## Parameters
		/// * scales: output parameter for returning output scales.
		/// * zeropoints: output parameter for returning output zeropoints.
		#[inline]
		fn get_output_details(&self, scales: &mut core::Vector<f32>, zeropoints: &mut core::Vector<i32>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_getOutputDetails_const_vectorLfloatGR_vectorLintGR(self.as_raw_Net(), scales.as_raw_mut_VectorOff32(), zeropoints.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
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
		/// This alternative version of [get_param] function uses the following default values for its arguments:
		/// * num_param: 0
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
		/// This alternative version of [get_param] function uses the following default values for its arguments:
		/// * num_param: 0
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
		/// * dtype: datatype of output blobs.
		/// * params: parameters which will be used to initialize the creating layer.
		/// ## Returns
		/// unique identifier of created layer, or -1 if a failure will happen.
		#[inline]
		fn add_layer_type(&mut self, name: &str, typ: &str, dtype: &i32, params: &mut crate::dnn::LayerParams) -> Result<i32> {
			extern_container_arg!(name);
			extern_container_arg!(typ);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_addLayer_const_StringR_const_StringR_const_intR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), dtype, params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Adds new layer to the net.
		/// ## Parameters
		/// * name: unique name of the adding layer.
		/// * type: typename of the adding layer (type must be registered in LayerRegister).
		/// * dtype: datatype of output blobs.
		/// * params: parameters which will be used to initialize the creating layer.
		/// ## Returns
		/// unique identifier of created layer, or -1 if a failure will happen.
		/// 
		/// ## Overloaded parameters
		///  Datatype of output blobs set to default CV_32F
		#[inline]
		fn add_layer(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
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
		#[inline]
		fn add_layer_to_prev_type(&mut self, name: &str, typ: &str, dtype: &i32, params: &mut crate::dnn::LayerParams) -> Result<i32> {
			extern_container_arg!(name);
			extern_container_arg!(typ);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_const_intR_LayerParamsR(self.as_raw_mut_Net(), name.opencv_as_extern(), typ.opencv_as_extern(), dtype, params.as_raw_mut_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Adds new layer and connects its first input to the first output of previously added layer.
		/// ## See also
		/// addLayer()
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn add_layer_to_prev(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
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
		#[inline]
		fn connect_first_second(&mut self, out_pin: &str, inp_pin: &str) -> Result<()> {
			extern_container_arg!(mut out_pin);
			extern_container_arg!(mut inp_pin);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_mut_Net(), out_pin.opencv_as_extern_mut(), inp_pin.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
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
		#[inline]
		fn set_inputs_names(&mut self, input_blob_names: &core::Vector<String>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_setInputsNames_const_vectorLStringGR(self.as_raw_mut_Net(), input_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Specify shape of network input.
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
		/// This alternative version of [forward_single] function uses the following default values for its arguments:
		/// * output_name: String()
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
		/// This alternative version of [forward_async] function uses the following default values for its arguments:
		/// * output_name: String()
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
		#[inline]
		fn forward_layer(&mut self, output_blobs: &mut impl core::ToOutputArray, output_name: &str) -> Result<()> {
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
		/// This alternative version of [forward_layer] function uses the following default values for its arguments:
		/// * output_name: String()
		#[inline]
		fn forward_layer_def(&mut self, output_blobs: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn forward(&mut self, output_blobs: &mut impl core::ToOutputArray, out_blob_names: &core::Vector<String>) -> Result<()> {
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
		#[inline]
		fn forward_and_retrieve(&mut self, output_blobs: &mut core::Vector<core::Vector<core::Mat>>, out_blob_names: &core::Vector<String>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_forward_vectorLvectorLMatGGR_const_vectorLStringGR(self.as_raw_mut_Net(), output_blobs.as_raw_mut_VectorOfVectorOfMat(), out_blob_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns a quantized Net from a floating-point Net.
		/// ## Parameters
		/// * calibData: Calibration data to compute the quantization parameters.
		/// * inputsDtype: Datatype of quantized net's inputs. Can be CV_32F or CV_8S.
		/// * outputsDtype: Datatype of quantized net's outputs. Can be CV_32F or CV_8S.
		/// * perChannel: Quantization granularity of quantized Net. The default is true, that means quantize model
		/// in per-channel way (channel-wise). Set it false to quantize model in per-tensor way (or tensor-wise).
		/// 
		/// ## C++ default parameters
		/// * per_channel: true
		#[inline]
		fn quantize(&mut self, calib_data: &impl core::ToInputArray, inputs_dtype: i32, outputs_dtype: i32, per_channel: bool) -> Result<crate::dnn::Net> {
			input_array_arg!(calib_data);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_quantize_const__InputArrayR_int_int_bool(self.as_raw_mut_Net(), calib_data.as_raw__InputArray(), inputs_dtype, outputs_dtype, per_channel, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns a quantized Net from a floating-point Net.
		/// ## Parameters
		/// * calibData: Calibration data to compute the quantization parameters.
		/// * inputsDtype: Datatype of quantized net's inputs. Can be CV_32F or CV_8S.
		/// * outputsDtype: Datatype of quantized net's outputs. Can be CV_32F or CV_8S.
		/// * perChannel: Quantization granularity of quantized Net. The default is true, that means quantize model
		/// in per-channel way (channel-wise). Set it false to quantize model in per-tensor way (or tensor-wise).
		/// 
		/// ## Note
		/// This alternative version of [quantize] function uses the following default values for its arguments:
		/// * per_channel: true
		#[inline]
		fn quantize_def(&mut self, calib_data: &impl core::ToInputArray, inputs_dtype: i32, outputs_dtype: i32) -> Result<crate::dnn::Net> {
			input_array_arg!(calib_data);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_quantize_const__InputArrayR_int_int(self.as_raw_mut_Net(), calib_data.as_raw__InputArray(), inputs_dtype, outputs_dtype, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::Net::opencv_from_extern(ret) };
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
		/// |                        | DNN_BACKEND_OPENCV | DNN_BACKEND_INFERENCE_ENGINE | DNN_BACKEND_HALIDE |  DNN_BACKEND_CUDA |
		/// |------------------------|--------------------|------------------------------|--------------------|-------------------|
		/// | DNN_TARGET_CPU         |                  + |                            + |                  + |                   |
		/// | DNN_TARGET_OPENCL      |                  + |                            + |                  + |                   |
		/// | DNN_TARGET_OPENCL_FP16 |                  + |                            + |                    |                   |
		/// | DNN_TARGET_MYRIAD      |                    |                            + |                    |                   |
		/// | DNN_TARGET_FPGA        |                    |                            + |                    |                   |
		/// | DNN_TARGET_CUDA        |                    |                              |                    |                 + |
		/// | DNN_TARGET_CUDA_FP16   |                    |                              |                    |                 + |
		/// | DNN_TARGET_HDDL        |                    |                            + |                    |                   |
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
		#[inline]
		fn set_input(&mut self, blob: &impl core::ToInputArray, name: &str, scalefactor: f64, mean: core::Scalar) -> Result<()> {
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
		/// This alternative version of [set_input] function uses the following default values for its arguments:
		/// * name: ""
		/// * scalefactor: 1.0
		/// * mean: Scalar()
		#[inline]
		fn set_input_def(&mut self, blob: &impl core::ToInputArray) -> Result<()> {
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
		#[inline]
		fn set_param(&mut self, layer: i32, num_param: i32, blob: &core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_setParam_int_int_const_MatR(self.as_raw_mut_Net(), layer, num_param, blob.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_param_1(&mut self, layer_name: &str, num_param: i32, blob: &core::Mat) -> Result<()> {
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
		#[inline]
		fn enable_fusion(&mut self, fusion: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_enableFusion_bool(self.as_raw_mut_Net(), fusion, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Enables or disables the Winograd compute branch. The Winograd compute branch can speed up
		/// 3x3 Convolution at a small loss of accuracy.
		/// ## Parameters
		/// * useWinograd: true to enable the Winograd compute branch. The default is true.
		#[inline]
		fn enable_winograd(&mut self, use_winograd: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_enableWinograd_bool(self.as_raw_mut_Net(), use_winograd, ocvrs_return.as_mut_ptr()) };
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
	pub struct Net {
		ptr: *mut c_void
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
	
	impl Net {
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
		#[inline]
		pub fn read_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size, ocvrs_return.as_mut_ptr()) };
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
	pub trait NormalizeBBoxLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void;
	
		#[inline]
		fn pnorm(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propPnorm_const(self.as_raw_NormalizeBBoxLayer()) };
			ret
		}
		
		#[inline]
		fn epsilon(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propEpsilon_const(self.as_raw_NormalizeBBoxLayer()) };
			ret
		}
		
		#[inline]
		fn across_spatial(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const(self.as_raw_NormalizeBBoxLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::NormalizeBBoxLayer]
	pub trait NormalizeBBoxLayerTrait: crate::dnn::LayerTrait + crate::dnn::NormalizeBBoxLayerTraitConst {
		fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_pnorm(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propPnorm_float(self.as_raw_mut_NormalizeBBoxLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_epsilon(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propEpsilon_float(self.as_raw_mut_NormalizeBBoxLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_across_spatial(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_bool(self.as_raw_mut_NormalizeBBoxLayer(), val) };
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
	pub struct NormalizeBBoxLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for NormalizeBBoxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for NormalizeBBoxLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTraitConst for NormalizeBBoxLayer {
		#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTrait for NormalizeBBoxLayer {
		#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl NormalizeBBoxLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::NormalizeBBoxLayer>> {
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
	
	/// Constant methods for [crate::dnn::NotLayer]
	pub trait NotLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_NotLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::NotLayer]
	pub trait NotLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::NotLayerTraitConst {
		fn as_raw_mut_NotLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct NotLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { NotLayer }
	
	impl Drop for NotLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_NotLayer_delete(self.as_raw_mut_NotLayer()) };
		}
	}
	
	unsafe impl Send for NotLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for NotLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for NotLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for NotLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for NotLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for NotLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for NotLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::NotLayerTraitConst for NotLayer {
		#[inline] fn as_raw_NotLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::NotLayerTrait for NotLayer {
		#[inline] fn as_raw_mut_NotLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl NotLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::NotLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_NotLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::NotLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { NotLayer, crate::dnn::ActivationLayer, cv_dnn_NotLayer_to_ActivationLayer }
	
	boxed_cast_base! { NotLayer, core::Algorithm, cv_dnn_NotLayer_to_Algorithm }
	
	boxed_cast_base! { NotLayer, crate::dnn::Layer, cv_dnn_NotLayer_to_Layer }
	
	impl std::fmt::Debug for NotLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("NotLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::PaddingLayer]
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
	pub struct PaddingLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for PaddingLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PaddingLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PaddingLayerTraitConst for PaddingLayer {
		#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::PaddingLayerTrait for PaddingLayer {
		#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PaddingLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PaddingLayer>> {
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
	
	/// Constant methods for [crate::dnn::PermuteLayer]
	pub trait PermuteLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_PermuteLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::PermuteLayer]
	pub trait PermuteLayerTrait: crate::dnn::LayerTrait + crate::dnn::PermuteLayerTraitConst {
		fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct PermuteLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for PermuteLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PermuteLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PermuteLayerTraitConst for PermuteLayer {
		#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::PermuteLayerTrait for PermuteLayer {
		#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PermuteLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PermuteLayer>> {
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
	
	/// Constant methods for [crate::dnn::PoolingLayer]
	pub trait PoolingLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_PoolingLayer(&self) -> *const c_void;
	
		#[inline]
		fn typ(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propType_const(self.as_raw_PoolingLayer()) };
			ret
		}
		
		#[inline]
		fn kernel_size(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propKernel_size_const(self.as_raw_PoolingLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn strides(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propStrides_const(self.as_raw_PoolingLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pads_begin(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_begin_const(self.as_raw_PoolingLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn pads_end(&self) -> core::Vector<size_t> {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_end_const(self.as_raw_PoolingLayer()) };
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			ret
		}
		
		/// Flag is true if at least one of the axes is global pooled.
		#[inline]
		fn global_pooling(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propGlobalPooling_const(self.as_raw_PoolingLayer()) };
			ret
		}
		
		#[inline]
		fn is_global_pooling(&self) -> core::Vector<bool> {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propIsGlobalPooling_const(self.as_raw_PoolingLayer()) };
			let ret = unsafe { core::Vector::<bool>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn compute_max_idx(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propComputeMaxIdx_const(self.as_raw_PoolingLayer()) };
			ret
		}
		
		#[inline]
		fn pad_mode(&self) -> String {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPadMode_const(self.as_raw_PoolingLayer()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn ceil_mode(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propCeilMode_const(self.as_raw_PoolingLayer()) };
			ret
		}
		
		#[inline]
		fn ave_pool_padded_area(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propAvePoolPaddedArea_const(self.as_raw_PoolingLayer()) };
			ret
		}
		
		#[inline]
		fn pooled_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_PoolingLayer_propPooledSize_const(self.as_raw_PoolingLayer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn spatial_scale(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propSpatialScale_const(self.as_raw_PoolingLayer()) };
			ret
		}
		
		#[inline]
		fn ps_roi_out_channels(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPsRoiOutChannels_const(self.as_raw_PoolingLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::PoolingLayer]
	pub trait PoolingLayerTrait: crate::dnn::LayerTrait + crate::dnn::PoolingLayerTraitConst {
		fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_type(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propType_int(self.as_raw_mut_PoolingLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_kernel_size(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propKernel_size_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_strides(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propStrides_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_pads_begin(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_begin_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		#[inline]
		fn set_pads_end(&mut self, mut val: core::Vector<size_t>) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPads_end_vectorLsize_tG(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfsize_t()) };
			ret
		}
		
		/// Flag is true if at least one of the axes is global pooled.
		#[inline]
		fn set_global_pooling(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propGlobalPooling_bool(self.as_raw_mut_PoolingLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_is_global_pooling(&mut self, mut val: core::Vector<bool>) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propIsGlobalPooling_vectorLboolG(self.as_raw_mut_PoolingLayer(), val.as_raw_mut_VectorOfbool()) };
			ret
		}
		
		#[inline]
		fn set_compute_max_idx(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propComputeMaxIdx_bool(self.as_raw_mut_PoolingLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_pad_mode(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPadMode_String(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern_mut()) };
			ret
		}
		
		#[inline]
		fn set_ceil_mode(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propCeilMode_bool(self.as_raw_mut_PoolingLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_ave_pool_padded_area(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propAvePoolPaddedArea_bool(self.as_raw_mut_PoolingLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_pooled_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPooledSize_Size(self.as_raw_mut_PoolingLayer(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_spatial_scale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propSpatialScale_float(self.as_raw_mut_PoolingLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_ps_roi_out_channels(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_PoolingLayer_propPsRoiOutChannels_int(self.as_raw_mut_PoolingLayer(), val) };
			ret
		}
		
	}
	
	pub struct PoolingLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for PoolingLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PoolingLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for PoolingLayer {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for PoolingLayer {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PoolingLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PoolingLayer>> {
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
	
	/// Constant methods for [crate::dnn::PoolingLayerInt8]
	pub trait PoolingLayerInt8TraitConst: crate::dnn::PoolingLayerTraitConst {
		fn as_raw_PoolingLayerInt8(&self) -> *const c_void;
	
		#[inline]
		fn input_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propInput_zp_const(self.as_raw_PoolingLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propOutput_zp_const(self.as_raw_PoolingLayerInt8()) };
			ret
		}
		
		#[inline]
		fn input_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propInput_sc_const(self.as_raw_PoolingLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propOutput_sc_const(self.as_raw_PoolingLayerInt8()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::PoolingLayerInt8]
	pub trait PoolingLayerInt8Trait: crate::dnn::PoolingLayerInt8TraitConst + crate::dnn::PoolingLayerTrait {
		fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_input_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propInput_zp_int(self.as_raw_mut_PoolingLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propOutput_zp_int(self.as_raw_mut_PoolingLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_input_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propInput_sc_float(self.as_raw_mut_PoolingLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_PoolingLayerInt8_propOutput_sc_float(self.as_raw_mut_PoolingLayerInt8(), val) };
			ret
		}
		
	}
	
	pub struct PoolingLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { PoolingLayerInt8 }
	
	impl Drop for PoolingLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_PoolingLayerInt8_delete(self.as_raw_mut_PoolingLayerInt8()) };
		}
	}
	
	unsafe impl Send for PoolingLayerInt8 {}
	
	impl core::AlgorithmTraitConst for PoolingLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for PoolingLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PoolingLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PoolingLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for PoolingLayerInt8 {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for PoolingLayerInt8 {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerInt8TraitConst for PoolingLayerInt8 {
		#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::PoolingLayerInt8Trait for PoolingLayerInt8 {
		#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PoolingLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PoolingLayerInt8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_PoolingLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::PoolingLayerInt8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { PoolingLayerInt8, core::Algorithm, cv_dnn_PoolingLayerInt8_to_Algorithm }
	
	boxed_cast_base! { PoolingLayerInt8, crate::dnn::Layer, cv_dnn_PoolingLayerInt8_to_Layer }
	
	boxed_cast_base! { PoolingLayerInt8, crate::dnn::PoolingLayer, cv_dnn_PoolingLayerInt8_to_PoolingLayer }
	
	impl std::fmt::Debug for PoolingLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PoolingLayerInt8")
				.field("input_zp", &crate::dnn::PoolingLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::PoolingLayerInt8TraitConst::output_zp(self))
				.field("input_sc", &crate::dnn::PoolingLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::PoolingLayerInt8TraitConst::output_sc(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
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
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::PowerLayer]
	pub trait PowerLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_PowerLayer(&self) -> *const c_void;
	
		#[inline]
		fn power(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_PowerLayer_propPower_const(self.as_raw_PowerLayer()) };
			ret
		}
		
		#[inline]
		fn scale(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_PowerLayer_propScale_const(self.as_raw_PowerLayer()) };
			ret
		}
		
		#[inline]
		fn shift(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_PowerLayer_propShift_const(self.as_raw_PowerLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::PowerLayer]
	pub trait PowerLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::PowerLayerTraitConst {
		fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_power(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_PowerLayer_propPower_float(self.as_raw_mut_PowerLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_scale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_PowerLayer_propScale_float(self.as_raw_mut_PowerLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_shift(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_PowerLayer_propShift_float(self.as_raw_mut_PowerLayer(), val) };
			ret
		}
		
	}
	
	pub struct PowerLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for PowerLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for PowerLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PowerLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PowerLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PowerLayerTraitConst for PowerLayer {
		#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::PowerLayerTrait for PowerLayer {
		#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PowerLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PowerLayer>> {
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
	pub trait PriorBoxLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_PriorBoxLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::PriorBoxLayer]
	pub trait PriorBoxLayerTrait: crate::dnn::LayerTrait + crate::dnn::PriorBoxLayerTraitConst {
		fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct PriorBoxLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for PriorBoxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PriorBoxLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PriorBoxLayerTraitConst for PriorBoxLayer {
		#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::PriorBoxLayerTrait for PriorBoxLayer {
		#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PriorBoxLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::PriorBoxLayer>> {
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
	
	/// Constant methods for [crate::dnn::ProposalLayer]
	pub trait ProposalLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ProposalLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ProposalLayer]
	pub trait ProposalLayerTrait: crate::dnn::LayerTrait + crate::dnn::ProposalLayerTraitConst {
		fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ProposalLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ProposalLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ProposalLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ProposalLayerTraitConst for ProposalLayer {
		#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ProposalLayerTrait for ProposalLayer {
		#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ProposalLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ProposalLayer>> {
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
	
	/// Constant methods for [crate::dnn::QuantizeLayer]
	pub trait QuantizeLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_QuantizeLayer(&self) -> *const c_void;
	
		#[inline]
		fn scales(&self) -> core::Vector<f32> {
			let ret = unsafe { sys::cv_dnn_QuantizeLayer_propScales_const(self.as_raw_QuantizeLayer()) };
			let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn zeropoints(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_dnn_QuantizeLayer_propZeropoints_const(self.as_raw_QuantizeLayer()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::QuantizeLayer]
	pub trait QuantizeLayerTrait: crate::dnn::LayerTrait + crate::dnn::QuantizeLayerTraitConst {
		fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_scales(&mut self, mut val: core::Vector<f32>) {
			let ret = unsafe { sys::cv_dnn_QuantizeLayer_propScales_vectorLfloatG(self.as_raw_mut_QuantizeLayer(), val.as_raw_mut_VectorOff32()) };
			ret
		}
		
		#[inline]
		fn set_zeropoints(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_dnn_QuantizeLayer_propZeropoints_vectorLintG(self.as_raw_mut_QuantizeLayer(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
	}
	
	pub struct QuantizeLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QuantizeLayer }
	
	impl Drop for QuantizeLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_QuantizeLayer_delete(self.as_raw_mut_QuantizeLayer()) };
		}
	}
	
	unsafe impl Send for QuantizeLayer {}
	
	impl core::AlgorithmTraitConst for QuantizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for QuantizeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for QuantizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for QuantizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::QuantizeLayerTraitConst for QuantizeLayer {
		#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::QuantizeLayerTrait for QuantizeLayer {
		#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl QuantizeLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::QuantizeLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_QuantizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::QuantizeLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { QuantizeLayer, core::Algorithm, cv_dnn_QuantizeLayer_to_Algorithm }
	
	boxed_cast_base! { QuantizeLayer, crate::dnn::Layer, cv_dnn_QuantizeLayer_to_Layer }
	
	impl std::fmt::Debug for QuantizeLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QuantizeLayer")
				.field("scales", &crate::dnn::QuantizeLayerTraitConst::scales(self))
				.field("zeropoints", &crate::dnn::QuantizeLayerTraitConst::zeropoints(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::RNNLayer]
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
		#[inline]
		fn set_weights(&mut self, wxh: &core::Mat, bh: &core::Mat, whh: &core::Mat, who: &core::Mat, bo: &core::Mat) -> Result<()> {
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
		/// This alternative version of [set_produce_hidden_output] function uses the following default values for its arguments:
		/// * produce: false
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
	pub struct RNNLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for RNNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for RNNLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RNNLayerTraitConst for RNNLayer {
		#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::RNNLayerTrait for RNNLayer {
		#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RNNLayer {
		/// Creates instance of RNNLayer
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::RNNLayer>> {
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
	pub trait ReLU6LayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ReLU6Layer(&self) -> *const c_void;
	
		#[inline]
		fn min_value(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMinValue_const(self.as_raw_ReLU6Layer()) };
			ret
		}
		
		#[inline]
		fn max_value(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMaxValue_const(self.as_raw_ReLU6Layer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ReLU6Layer]
	pub trait ReLU6LayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReLU6LayerTraitConst {
		fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_min_value(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMinValue_float(self.as_raw_mut_ReLU6Layer(), val) };
			ret
		}
		
		#[inline]
		fn set_max_value(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ReLU6Layer_propMaxValue_float(self.as_raw_mut_ReLU6Layer(), val) };
			ret
		}
		
	}
	
	pub struct ReLU6Layer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for ReLU6Layer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ReLU6Layer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ReLU6Layer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ReLU6Layer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLU6LayerTraitConst for ReLU6Layer {
		#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ReLU6LayerTrait for ReLU6Layer {
		#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ReLU6Layer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReLU6Layer>> {
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
	pub trait ReLULayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ReLULayer(&self) -> *const c_void;
	
		#[inline]
		fn negative_slope(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ReLULayer_propNegativeSlope_const(self.as_raw_ReLULayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ReLULayer]
	pub trait ReLULayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReLULayerTraitConst {
		fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_negative_slope(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ReLULayer_propNegativeSlope_float(self.as_raw_mut_ReLULayer(), val) };
			ret
		}
		
	}
	
	pub struct ReLULayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for ReLULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ReLULayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ReLULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ReLULayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLULayerTraitConst for ReLULayer {
		#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ReLULayerTrait for ReLULayer {
		#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ReLULayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReLULayer>> {
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
	
	/// Constant methods for [crate::dnn::ReciprocalLayer]
	pub trait ReciprocalLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ReciprocalLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ReciprocalLayer]
	pub trait ReciprocalLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ReciprocalLayerTraitConst {
		fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ReciprocalLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ReciprocalLayer }
	
	impl Drop for ReciprocalLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ReciprocalLayer_delete(self.as_raw_mut_ReciprocalLayer()) };
		}
	}
	
	unsafe impl Send for ReciprocalLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for ReciprocalLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for ReciprocalLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for ReciprocalLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ReciprocalLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ReciprocalLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ReciprocalLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReciprocalLayerTraitConst for ReciprocalLayer {
		#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ReciprocalLayerTrait for ReciprocalLayer {
		#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ReciprocalLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReciprocalLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ReciprocalLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ReciprocalLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ReciprocalLayer, crate::dnn::ActivationLayer, cv_dnn_ReciprocalLayer_to_ActivationLayer }
	
	boxed_cast_base! { ReciprocalLayer, core::Algorithm, cv_dnn_ReciprocalLayer_to_Algorithm }
	
	boxed_cast_base! { ReciprocalLayer, crate::dnn::Layer, cv_dnn_ReciprocalLayer_to_Layer }
	
	impl std::fmt::Debug for ReciprocalLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ReciprocalLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ReduceLayer]
	pub trait ReduceLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ReduceLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ReduceLayer]
	pub trait ReduceLayerTrait: crate::dnn::LayerTrait + crate::dnn::ReduceLayerTraitConst {
		fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ReduceLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ReduceLayer }
	
	impl Drop for ReduceLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ReduceLayer_delete(self.as_raw_mut_ReduceLayer()) };
		}
	}
	
	unsafe impl Send for ReduceLayer {}
	
	impl core::AlgorithmTraitConst for ReduceLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ReduceLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ReduceLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ReduceLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerTraitConst for ReduceLayer {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ReduceLayerTrait for ReduceLayer {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ReduceLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReduceLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ReduceLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ReduceLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ReduceLayer, core::Algorithm, cv_dnn_ReduceLayer_to_Algorithm }
	
	boxed_cast_base! { ReduceLayer, crate::dnn::Layer, cv_dnn_ReduceLayer_to_Layer }
	
	impl std::fmt::Debug for ReduceLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ReduceLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::RegionLayer]
	pub trait RegionLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_RegionLayer(&self) -> *const c_void;
	
		#[inline]
		fn nms_threshold(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_RegionLayer_propNmsThreshold_const(self.as_raw_RegionLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::RegionLayer]
	pub trait RegionLayerTrait: crate::dnn::LayerTrait + crate::dnn::RegionLayerTraitConst {
		fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_nms_threshold(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_RegionLayer_propNmsThreshold_float(self.as_raw_mut_RegionLayer(), val) };
			ret
		}
		
	}
	
	pub struct RegionLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for RegionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for RegionLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RegionLayerTraitConst for RegionLayer {
		#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::RegionLayerTrait for RegionLayer {
		#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RegionLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::RegionLayer>> {
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
				.field("nms_threshold", &crate::dnn::RegionLayerTraitConst::nms_threshold(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ReorgLayer]
	pub trait ReorgLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ReorgLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ReorgLayer]
	pub trait ReorgLayerTrait: crate::dnn::LayerTrait + crate::dnn::ReorgLayerTraitConst {
		fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ReorgLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ReorgLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ReorgLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReorgLayerTraitConst for ReorgLayer {
		#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ReorgLayerTrait for ReorgLayer {
		#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ReorgLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReorgLayer>> {
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
	
	/// Constant methods for [crate::dnn::RequantizeLayer]
	pub trait RequantizeLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_RequantizeLayer(&self) -> *const c_void;
	
		#[inline]
		fn scale(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_RequantizeLayer_propScale_const(self.as_raw_RequantizeLayer()) };
			ret
		}
		
		#[inline]
		fn shift(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_RequantizeLayer_propShift_const(self.as_raw_RequantizeLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::RequantizeLayer]
	pub trait RequantizeLayerTrait: crate::dnn::LayerTrait + crate::dnn::RequantizeLayerTraitConst {
		fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_scale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_RequantizeLayer_propScale_float(self.as_raw_mut_RequantizeLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_shift(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_RequantizeLayer_propShift_float(self.as_raw_mut_RequantizeLayer(), val) };
			ret
		}
		
	}
	
	pub struct RequantizeLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RequantizeLayer }
	
	impl Drop for RequantizeLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_RequantizeLayer_delete(self.as_raw_mut_RequantizeLayer()) };
		}
	}
	
	unsafe impl Send for RequantizeLayer {}
	
	impl core::AlgorithmTraitConst for RequantizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for RequantizeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for RequantizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for RequantizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RequantizeLayerTraitConst for RequantizeLayer {
		#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::RequantizeLayerTrait for RequantizeLayer {
		#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RequantizeLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::RequantizeLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_RequantizeLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::RequantizeLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { RequantizeLayer, core::Algorithm, cv_dnn_RequantizeLayer_to_Algorithm }
	
	boxed_cast_base! { RequantizeLayer, crate::dnn::Layer, cv_dnn_RequantizeLayer_to_Layer }
	
	impl std::fmt::Debug for RequantizeLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RequantizeLayer")
				.field("scale", &crate::dnn::RequantizeLayerTraitConst::scale(self))
				.field("shift", &crate::dnn::RequantizeLayerTraitConst::shift(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ReshapeLayer]
	pub trait ReshapeLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ReshapeLayer(&self) -> *const c_void;
	
		#[inline]
		fn new_shape_desc(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_dnn_ReshapeLayer_propNewShapeDesc_const(self.as_raw_ReshapeLayer()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
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
	
		#[inline]
		fn set_new_shape_desc(&mut self, mut val: crate::dnn::MatShape) {
			let ret = unsafe { sys::cv_dnn_ReshapeLayer_propNewShapeDesc_MatShape(self.as_raw_mut_ReshapeLayer(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		#[inline]
		fn set_new_shape_range(&mut self, mut val: core::Range) {
			let ret = unsafe { sys::cv_dnn_ReshapeLayer_propNewShapeRange_Range(self.as_raw_mut_ReshapeLayer(), val.as_raw_mut_Range()) };
			ret
		}
		
	}
	
	pub struct ReshapeLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ReshapeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ReshapeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReshapeLayerTraitConst for ReshapeLayer {
		#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ReshapeLayerTrait for ReshapeLayer {
		#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ReshapeLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ReshapeLayer>> {
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
	
	/// Constant methods for [crate::dnn::ResizeLayer]
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
	pub struct ResizeLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ResizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ResizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ResizeLayerTraitConst for ResizeLayer {
		#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ResizeLayerTrait for ResizeLayer {
		#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ResizeLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ResizeLayer>> {
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
	
	/// Constant methods for [crate::dnn::RoundLayer]
	pub trait RoundLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_RoundLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::RoundLayer]
	pub trait RoundLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::RoundLayerTraitConst {
		fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct RoundLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RoundLayer }
	
	impl Drop for RoundLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_RoundLayer_delete(self.as_raw_mut_RoundLayer()) };
		}
	}
	
	unsafe impl Send for RoundLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for RoundLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for RoundLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for RoundLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for RoundLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for RoundLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for RoundLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RoundLayerTraitConst for RoundLayer {
		#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::RoundLayerTrait for RoundLayer {
		#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RoundLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::RoundLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_RoundLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::RoundLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { RoundLayer, crate::dnn::ActivationLayer, cv_dnn_RoundLayer_to_ActivationLayer }
	
	boxed_cast_base! { RoundLayer, core::Algorithm, cv_dnn_RoundLayer_to_Algorithm }
	
	boxed_cast_base! { RoundLayer, crate::dnn::Layer, cv_dnn_RoundLayer_to_Layer }
	
	impl std::fmt::Debug for RoundLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RoundLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ScaleLayer]
	pub trait ScaleLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ScaleLayer(&self) -> *const c_void;
	
		#[inline]
		fn has_bias(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_ScaleLayer_propHasBias_const(self.as_raw_ScaleLayer()) };
			ret
		}
		
		#[inline]
		fn axis(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_ScaleLayer_propAxis_const(self.as_raw_ScaleLayer()) };
			ret
		}
		
		#[inline]
		fn mode(&self) -> String {
			let ret = unsafe { sys::cv_dnn_ScaleLayer_propMode_const(self.as_raw_ScaleLayer()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ScaleLayer]
	pub trait ScaleLayerTrait: crate::dnn::LayerTrait + crate::dnn::ScaleLayerTraitConst {
		fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_has_bias(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_ScaleLayer_propHasBias_bool(self.as_raw_mut_ScaleLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_axis(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_ScaleLayer_propAxis_int(self.as_raw_mut_ScaleLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_mode(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_dnn_ScaleLayer_propMode_String(self.as_raw_mut_ScaleLayer(), val.opencv_as_extern_mut()) };
			ret
		}
		
	}
	
	pub struct ScaleLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ScaleLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ScaleLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for ScaleLayer {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for ScaleLayer {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ScaleLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ScaleLayer>> {
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
				.field("mode", &crate::dnn::ScaleLayerTraitConst::mode(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ScaleLayerInt8]
	pub trait ScaleLayerInt8TraitConst: crate::dnn::ScaleLayerTraitConst {
		fn as_raw_ScaleLayerInt8(&self) -> *const c_void;
	
		#[inline]
		fn output_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_propOutput_sc_const(self.as_raw_ScaleLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_propOutput_zp_const(self.as_raw_ScaleLayerInt8()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ScaleLayerInt8]
	pub trait ScaleLayerInt8Trait: crate::dnn::ScaleLayerInt8TraitConst + crate::dnn::ScaleLayerTrait {
		fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_output_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_propOutput_sc_float(self.as_raw_mut_ScaleLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_ScaleLayerInt8_propOutput_zp_int(self.as_raw_mut_ScaleLayerInt8(), val) };
			ret
		}
		
	}
	
	pub struct ScaleLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ScaleLayerInt8 }
	
	impl Drop for ScaleLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ScaleLayerInt8_delete(self.as_raw_mut_ScaleLayerInt8()) };
		}
	}
	
	unsafe impl Send for ScaleLayerInt8 {}
	
	impl core::AlgorithmTraitConst for ScaleLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ScaleLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ScaleLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ScaleLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for ScaleLayerInt8 {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for ScaleLayerInt8 {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerInt8TraitConst for ScaleLayerInt8 {
		#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ScaleLayerInt8Trait for ScaleLayerInt8 {
		#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ScaleLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ScaleLayerInt8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ScaleLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ScaleLayerInt8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ScaleLayerInt8, core::Algorithm, cv_dnn_ScaleLayerInt8_to_Algorithm }
	
	boxed_cast_base! { ScaleLayerInt8, crate::dnn::Layer, cv_dnn_ScaleLayerInt8_to_Layer }
	
	boxed_cast_base! { ScaleLayerInt8, crate::dnn::ScaleLayer, cv_dnn_ScaleLayerInt8_to_ScaleLayer }
	
	impl std::fmt::Debug for ScaleLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ScaleLayerInt8")
				.field("output_sc", &crate::dnn::ScaleLayerInt8TraitConst::output_sc(self))
				.field("output_zp", &crate::dnn::ScaleLayerInt8TraitConst::output_zp(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
				.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
				.field("mode", &crate::dnn::ScaleLayerTraitConst::mode(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ScatterLayer]
	pub trait ScatterLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ScatterLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ScatterLayer]
	pub trait ScatterLayerTrait: crate::dnn::LayerTrait + crate::dnn::ScatterLayerTraitConst {
		fn as_raw_mut_ScatterLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ScatterLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ScatterLayer }
	
	impl Drop for ScatterLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ScatterLayer_delete(self.as_raw_mut_ScatterLayer()) };
		}
	}
	
	unsafe impl Send for ScatterLayer {}
	
	impl core::AlgorithmTraitConst for ScatterLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ScatterLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ScatterLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ScatterLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScatterLayerTraitConst for ScatterLayer {
		#[inline] fn as_raw_ScatterLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ScatterLayerTrait for ScatterLayer {
		#[inline] fn as_raw_mut_ScatterLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ScatterLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ScatterLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ScatterLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ScatterLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ScatterLayer, core::Algorithm, cv_dnn_ScatterLayer_to_Algorithm }
	
	boxed_cast_base! { ScatterLayer, crate::dnn::Layer, cv_dnn_ScatterLayer_to_Layer }
	
	impl std::fmt::Debug for ScatterLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ScatterLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ScatterNDLayer]
	pub trait ScatterNDLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ScatterNDLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ScatterNDLayer]
	pub trait ScatterNDLayerTrait: crate::dnn::LayerTrait + crate::dnn::ScatterNDLayerTraitConst {
		fn as_raw_mut_ScatterNDLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ScatterNDLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ScatterNDLayer }
	
	impl Drop for ScatterNDLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ScatterNDLayer_delete(self.as_raw_mut_ScatterNDLayer()) };
		}
	}
	
	unsafe impl Send for ScatterNDLayer {}
	
	impl core::AlgorithmTraitConst for ScatterNDLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ScatterNDLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ScatterNDLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ScatterNDLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScatterNDLayerTraitConst for ScatterNDLayer {
		#[inline] fn as_raw_ScatterNDLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ScatterNDLayerTrait for ScatterNDLayer {
		#[inline] fn as_raw_mut_ScatterNDLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ScatterNDLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ScatterNDLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ScatterNDLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ScatterNDLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ScatterNDLayer, core::Algorithm, cv_dnn_ScatterNDLayer_to_Algorithm }
	
	boxed_cast_base! { ScatterNDLayer, crate::dnn::Layer, cv_dnn_ScatterNDLayer_to_Layer }
	
	impl std::fmt::Debug for ScatterNDLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ScatterNDLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SegmentationModel]
	pub trait SegmentationModelTraitConst: crate::dnn::ModelTraitConst {
		fn as_raw_SegmentationModel(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SegmentationModel]
	pub trait SegmentationModelTrait: crate::dnn::ModelTrait + crate::dnn::SegmentationModelTraitConst {
		fn as_raw_mut_SegmentationModel(&mut self) -> *mut c_void;
	
		/// Given the @p input frame, create input blob, run net
		/// ## Parameters
		/// * frame: The input image.
		/// * mask:[out] Allocated class prediction for each pixel
		#[inline]
		fn segment(&mut self, frame: &impl core::ToInputArray, mask: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(frame);
			output_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SegmentationModel_segment_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_SegmentationModel(), frame.as_raw__InputArray(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// This class represents high-level API for segmentation  models
	/// 
	/// SegmentationModel allows to set params for preprocessing input image.
	/// SegmentationModel creates net from file with trained weights and config,
	/// sets preprocessing input, runs forward pass and returns the class prediction for each pixel.
	pub struct SegmentationModel {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SegmentationModel }
	
	impl Drop for SegmentationModel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SegmentationModel_delete(self.as_raw_mut_SegmentationModel()) };
		}
	}
	
	unsafe impl Send for SegmentationModel {}
	
	impl crate::dnn::ModelTraitConst for SegmentationModel {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for SegmentationModel {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SegmentationModelTraitConst for SegmentationModel {
		#[inline] fn as_raw_SegmentationModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SegmentationModelTrait for SegmentationModel {
		#[inline] fn as_raw_mut_SegmentationModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SegmentationModel {
		/// Create segmentation model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn new(model: &str, config: &str) -> Result<crate::dnn::SegmentationModel> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SegmentationModel_SegmentationModel_const_StringR_const_StringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::SegmentationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create segmentation model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn new_def(model: &str) -> Result<crate::dnn::SegmentationModel> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SegmentationModel_SegmentationModel_const_StringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::SegmentationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create model from deep learning network.
		/// ## Parameters
		/// * network: Net object.
		#[inline]
		pub fn new_1(network: &crate::dnn::Net) -> Result<crate::dnn::SegmentationModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SegmentationModel_SegmentationModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::SegmentationModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for SegmentationModel {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_SegmentationModel_implicitClone_const(self.as_raw_SegmentationModel())) }
		}
	}
	
	boxed_cast_base! { SegmentationModel, crate::dnn::Model, cv_dnn_SegmentationModel_to_Model }
	
	impl std::fmt::Debug for SegmentationModel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SegmentationModel")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SeluLayer]
	pub trait SeluLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SeluLayer(&self) -> *const c_void;
	
		#[inline]
		fn alpha(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_SeluLayer_propAlpha_const(self.as_raw_SeluLayer()) };
			ret
		}
		
		#[inline]
		fn gamma(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_SeluLayer_propGamma_const(self.as_raw_SeluLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::SeluLayer]
	pub trait SeluLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SeluLayerTraitConst {
		fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_alpha(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_SeluLayer_propAlpha_float(self.as_raw_mut_SeluLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_gamma(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_SeluLayer_propGamma_float(self.as_raw_mut_SeluLayer(), val) };
			ret
		}
		
	}
	
	pub struct SeluLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SeluLayer }
	
	impl Drop for SeluLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SeluLayer_delete(self.as_raw_mut_SeluLayer()) };
		}
	}
	
	unsafe impl Send for SeluLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for SeluLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for SeluLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for SeluLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SeluLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SeluLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SeluLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SeluLayerTraitConst for SeluLayer {
		#[inline] fn as_raw_SeluLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SeluLayerTrait for SeluLayer {
		#[inline] fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SeluLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SeluLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SeluLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SeluLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SeluLayer, crate::dnn::ActivationLayer, cv_dnn_SeluLayer_to_ActivationLayer }
	
	boxed_cast_base! { SeluLayer, core::Algorithm, cv_dnn_SeluLayer_to_Algorithm }
	
	boxed_cast_base! { SeluLayer, crate::dnn::Layer, cv_dnn_SeluLayer_to_Layer }
	
	impl std::fmt::Debug for SeluLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SeluLayer")
				.field("alpha", &crate::dnn::SeluLayerTraitConst::alpha(self))
				.field("gamma", &crate::dnn::SeluLayerTraitConst::gamma(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ShiftLayer]
	pub trait ShiftLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ShiftLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ShiftLayer]
	pub trait ShiftLayerTrait: crate::dnn::LayerTrait + crate::dnn::ShiftLayerTraitConst {
		fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct ShiftLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ShiftLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ShiftLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ShiftLayerTraitConst for ShiftLayer {
		#[inline] fn as_raw_ShiftLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ShiftLayerTrait for ShiftLayer {
		#[inline] fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ShiftLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::ShiftLayerInt8]
	pub trait ShiftLayerInt8TraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ShiftLayerInt8(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::ShiftLayerInt8]
	pub trait ShiftLayerInt8Trait: crate::dnn::LayerTrait + crate::dnn::ShiftLayerInt8TraitConst {
		fn as_raw_mut_ShiftLayerInt8(&mut self) -> *mut c_void;
	
	}
	
	pub struct ShiftLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ShiftLayerInt8 }
	
	impl Drop for ShiftLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ShiftLayerInt8_delete(self.as_raw_mut_ShiftLayerInt8()) };
		}
	}
	
	unsafe impl Send for ShiftLayerInt8 {}
	
	impl core::AlgorithmTraitConst for ShiftLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ShiftLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ShiftLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ShiftLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ShiftLayerInt8TraitConst for ShiftLayerInt8 {
		#[inline] fn as_raw_ShiftLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ShiftLayerInt8Trait for ShiftLayerInt8 {
		#[inline] fn as_raw_mut_ShiftLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ShiftLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ShiftLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::Layer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ShiftLayerInt8, core::Algorithm, cv_dnn_ShiftLayerInt8_to_Algorithm }
	
	boxed_cast_base! { ShiftLayerInt8, crate::dnn::Layer, cv_dnn_ShiftLayerInt8_to_Layer }
	
	impl std::fmt::Debug for ShiftLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ShiftLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ShrinkLayer]
	pub trait ShrinkLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ShrinkLayer(&self) -> *const c_void;
	
		#[inline]
		fn bias(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ShrinkLayer_propBias_const(self.as_raw_ShrinkLayer()) };
			ret
		}
		
		#[inline]
		fn lambd(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ShrinkLayer_propLambd_const(self.as_raw_ShrinkLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ShrinkLayer]
	pub trait ShrinkLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ShrinkLayerTraitConst {
		fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_bias(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ShrinkLayer_propBias_float(self.as_raw_mut_ShrinkLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_lambd(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ShrinkLayer_propLambd_float(self.as_raw_mut_ShrinkLayer(), val) };
			ret
		}
		
	}
	
	pub struct ShrinkLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ShrinkLayer }
	
	impl Drop for ShrinkLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ShrinkLayer_delete(self.as_raw_mut_ShrinkLayer()) };
		}
	}
	
	unsafe impl Send for ShrinkLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for ShrinkLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for ShrinkLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for ShrinkLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ShrinkLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ShrinkLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ShrinkLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ShrinkLayerTraitConst for ShrinkLayer {
		#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ShrinkLayerTrait for ShrinkLayer {
		#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ShrinkLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ShrinkLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ShrinkLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ShrinkLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ShrinkLayer, crate::dnn::ActivationLayer, cv_dnn_ShrinkLayer_to_ActivationLayer }
	
	boxed_cast_base! { ShrinkLayer, core::Algorithm, cv_dnn_ShrinkLayer_to_Algorithm }
	
	boxed_cast_base! { ShrinkLayer, crate::dnn::Layer, cv_dnn_ShrinkLayer_to_Layer }
	
	impl std::fmt::Debug for ShrinkLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ShrinkLayer")
				.field("bias", &crate::dnn::ShrinkLayerTraitConst::bias(self))
				.field("lambd", &crate::dnn::ShrinkLayerTraitConst::lambd(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ShuffleChannelLayer]
	pub trait ShuffleChannelLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_ShuffleChannelLayer(&self) -> *const c_void;
	
		#[inline]
		fn group(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_ShuffleChannelLayer_propGroup_const(self.as_raw_ShuffleChannelLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ShuffleChannelLayer]
	pub trait ShuffleChannelLayerTrait: crate::dnn::LayerTrait + crate::dnn::ShuffleChannelLayerTraitConst {
		fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_group(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_ShuffleChannelLayer_propGroup_int(self.as_raw_mut_ShuffleChannelLayer(), val) };
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
	pub struct ShuffleChannelLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for ShuffleChannelLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ShuffleChannelLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ShuffleChannelLayerTraitConst for ShuffleChannelLayer {
		#[inline] fn as_raw_ShuffleChannelLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ShuffleChannelLayerTrait for ShuffleChannelLayer {
		#[inline] fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ShuffleChannelLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::Layer>> {
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
	
	/// Constant methods for [crate::dnn::SigmoidLayer]
	pub trait SigmoidLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SigmoidLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SigmoidLayer]
	pub trait SigmoidLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SigmoidLayerTraitConst {
		fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SigmoidLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for SigmoidLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SigmoidLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SigmoidLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SigmoidLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SigmoidLayerTraitConst for SigmoidLayer {
		#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SigmoidLayerTrait for SigmoidLayer {
		#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SigmoidLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SigmoidLayer>> {
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
	
	/// Constant methods for [crate::dnn::SignLayer]
	pub trait SignLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SignLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SignLayer]
	pub trait SignLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SignLayerTraitConst {
		fn as_raw_mut_SignLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SignLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SignLayer }
	
	impl Drop for SignLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SignLayer_delete(self.as_raw_mut_SignLayer()) };
		}
	}
	
	unsafe impl Send for SignLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for SignLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for SignLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for SignLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SignLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SignLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SignLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SignLayerTraitConst for SignLayer {
		#[inline] fn as_raw_SignLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SignLayerTrait for SignLayer {
		#[inline] fn as_raw_mut_SignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SignLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SignLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SignLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SignLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SignLayer, crate::dnn::ActivationLayer, cv_dnn_SignLayer_to_ActivationLayer }
	
	boxed_cast_base! { SignLayer, core::Algorithm, cv_dnn_SignLayer_to_Algorithm }
	
	boxed_cast_base! { SignLayer, crate::dnn::Layer, cv_dnn_SignLayer_to_Layer }
	
	impl std::fmt::Debug for SignLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SignLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SinLayer]
	pub trait SinLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SinLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SinLayer]
	pub trait SinLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SinLayerTraitConst {
		fn as_raw_mut_SinLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SinLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SinLayer }
	
	impl Drop for SinLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SinLayer_delete(self.as_raw_mut_SinLayer()) };
		}
	}
	
	unsafe impl Send for SinLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for SinLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for SinLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for SinLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SinLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SinLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SinLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinLayerTraitConst for SinLayer {
		#[inline] fn as_raw_SinLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SinLayerTrait for SinLayer {
		#[inline] fn as_raw_mut_SinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SinLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SinLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SinLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SinLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SinLayer, crate::dnn::ActivationLayer, cv_dnn_SinLayer_to_ActivationLayer }
	
	boxed_cast_base! { SinLayer, core::Algorithm, cv_dnn_SinLayer_to_Algorithm }
	
	boxed_cast_base! { SinLayer, crate::dnn::Layer, cv_dnn_SinLayer_to_Layer }
	
	impl std::fmt::Debug for SinLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SinLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SinhLayer]
	pub trait SinhLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SinhLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SinhLayer]
	pub trait SinhLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SinhLayerTraitConst {
		fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SinhLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SinhLayer }
	
	impl Drop for SinhLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SinhLayer_delete(self.as_raw_mut_SinhLayer()) };
		}
	}
	
	unsafe impl Send for SinhLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for SinhLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for SinhLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for SinhLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SinhLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SinhLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SinhLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinhLayerTraitConst for SinhLayer {
		#[inline] fn as_raw_SinhLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SinhLayerTrait for SinhLayer {
		#[inline] fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SinhLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SinhLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SinhLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SinhLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SinhLayer, crate::dnn::ActivationLayer, cv_dnn_SinhLayer_to_ActivationLayer }
	
	boxed_cast_base! { SinhLayer, core::Algorithm, cv_dnn_SinhLayer_to_Algorithm }
	
	boxed_cast_base! { SinhLayer, crate::dnn::Layer, cv_dnn_SinhLayer_to_Layer }
	
	impl std::fmt::Debug for SinhLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SinhLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SliceLayer]
	pub trait SliceLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_SliceLayer(&self) -> *const c_void;
	
		/// Vector of slice ranges.
		/// 
		/// The first dimension equals number of output blobs.
		/// Inner vector has slice ranges for the first number of input dimensions.
		#[inline]
		fn slice_ranges(&self) -> core::Vector<core::Vector<core::Range>> {
			let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceRanges_const(self.as_raw_SliceLayer()) };
			let ret = unsafe { core::Vector::<core::Vector<core::Range>>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn slice_steps(&self) -> core::Vector<core::Vector<i32>> {
			let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceSteps_const(self.as_raw_SliceLayer()) };
			let ret = unsafe { core::Vector::<core::Vector<i32>>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn axis(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_SliceLayer_propAxis_const(self.as_raw_SliceLayer()) };
			ret
		}
		
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
		#[inline]
		fn set_slice_ranges(&mut self, mut val: core::Vector<core::Vector<core::Range>>) {
			let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceRanges_vectorLvectorLRangeGG(self.as_raw_mut_SliceLayer(), val.as_raw_mut_VectorOfVectorOfRange()) };
			ret
		}
		
		#[inline]
		fn set_slice_steps(&mut self, mut val: core::Vector<core::Vector<i32>>) {
			let ret = unsafe { sys::cv_dnn_SliceLayer_propSliceSteps_vectorLvectorLintGG(self.as_raw_mut_SliceLayer(), val.as_raw_mut_VectorOfVectorOfi32()) };
			ret
		}
		
		#[inline]
		fn set_axis(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_SliceLayer_propAxis_int(self.as_raw_mut_SliceLayer(), val) };
			ret
		}
		
		#[inline]
		fn set_num_split(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_SliceLayer_propNum_split_int(self.as_raw_mut_SliceLayer(), val) };
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
	pub struct SliceLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for SliceLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SliceLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SliceLayerTraitConst for SliceLayer {
		#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SliceLayerTrait for SliceLayer {
		#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SliceLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SliceLayer>> {
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
	
	/// Constant methods for [crate::dnn::SoftmaxLayer]
	pub trait SoftmaxLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_SoftmaxLayer(&self) -> *const c_void;
	
		#[inline]
		fn log_soft_max(&self) -> bool {
			let ret = unsafe { sys::cv_dnn_SoftmaxLayer_propLogSoftMax_const(self.as_raw_SoftmaxLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::SoftmaxLayer]
	pub trait SoftmaxLayerTrait: crate::dnn::LayerTrait + crate::dnn::SoftmaxLayerTraitConst {
		fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_log_soft_max(&mut self, val: bool) {
			let ret = unsafe { sys::cv_dnn_SoftmaxLayer_propLogSoftMax_bool(self.as_raw_mut_SoftmaxLayer(), val) };
			ret
		}
		
	}
	
	pub struct SoftmaxLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for SoftmaxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SoftmaxLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for SoftmaxLayer {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for SoftmaxLayer {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SoftmaxLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftmaxLayer>> {
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
	
	/// Constant methods for [crate::dnn::SoftmaxLayerInt8]
	pub trait SoftmaxLayerInt8TraitConst: crate::dnn::SoftmaxLayerTraitConst {
		fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void;
	
		#[inline]
		fn output_sc(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_propOutput_sc_const(self.as_raw_SoftmaxLayerInt8()) };
			ret
		}
		
		#[inline]
		fn output_zp(&self) -> i32 {
			let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_propOutput_zp_const(self.as_raw_SoftmaxLayerInt8()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::SoftmaxLayerInt8]
	pub trait SoftmaxLayerInt8Trait: crate::dnn::SoftmaxLayerInt8TraitConst + crate::dnn::SoftmaxLayerTrait {
		fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_output_sc(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_propOutput_sc_float(self.as_raw_mut_SoftmaxLayerInt8(), val) };
			ret
		}
		
		#[inline]
		fn set_output_zp(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_SoftmaxLayerInt8_propOutput_zp_int(self.as_raw_mut_SoftmaxLayerInt8(), val) };
			ret
		}
		
	}
	
	pub struct SoftmaxLayerInt8 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SoftmaxLayerInt8 }
	
	impl Drop for SoftmaxLayerInt8 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SoftmaxLayerInt8_delete(self.as_raw_mut_SoftmaxLayerInt8()) };
		}
	}
	
	unsafe impl Send for SoftmaxLayerInt8 {}
	
	impl core::AlgorithmTraitConst for SoftmaxLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SoftmaxLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for SoftmaxLayerInt8 {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for SoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8TraitConst for SoftmaxLayerInt8 {
		#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8Trait for SoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SoftmaxLayerInt8 {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftmaxLayerInt8>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SoftmaxLayerInt8_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SoftmaxLayerInt8>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SoftmaxLayerInt8, core::Algorithm, cv_dnn_SoftmaxLayerInt8_to_Algorithm }
	
	boxed_cast_base! { SoftmaxLayerInt8, crate::dnn::Layer, cv_dnn_SoftmaxLayerInt8_to_Layer }
	
	boxed_cast_base! { SoftmaxLayerInt8, crate::dnn::SoftmaxLayer, cv_dnn_SoftmaxLayerInt8_to_SoftmaxLayer }
	
	impl std::fmt::Debug for SoftmaxLayerInt8 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SoftmaxLayerInt8")
				.field("output_sc", &crate::dnn::SoftmaxLayerInt8TraitConst::output_sc(self))
				.field("output_zp", &crate::dnn::SoftmaxLayerInt8TraitConst::output_zp(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SoftplusLayer]
	pub trait SoftplusLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SoftplusLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SoftplusLayer]
	pub trait SoftplusLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SoftplusLayerTraitConst {
		fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SoftplusLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SoftplusLayer }
	
	impl Drop for SoftplusLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SoftplusLayer_delete(self.as_raw_mut_SoftplusLayer()) };
		}
	}
	
	unsafe impl Send for SoftplusLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for SoftplusLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for SoftplusLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for SoftplusLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SoftplusLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SoftplusLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SoftplusLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftplusLayerTraitConst for SoftplusLayer {
		#[inline] fn as_raw_SoftplusLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SoftplusLayerTrait for SoftplusLayer {
		#[inline] fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SoftplusLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftplusLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SoftplusLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SoftplusLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SoftplusLayer, crate::dnn::ActivationLayer, cv_dnn_SoftplusLayer_to_ActivationLayer }
	
	boxed_cast_base! { SoftplusLayer, core::Algorithm, cv_dnn_SoftplusLayer_to_Algorithm }
	
	boxed_cast_base! { SoftplusLayer, crate::dnn::Layer, cv_dnn_SoftplusLayer_to_Layer }
	
	impl std::fmt::Debug for SoftplusLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SoftplusLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SoftsignLayer]
	pub trait SoftsignLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SoftsignLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SoftsignLayer]
	pub trait SoftsignLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SoftsignLayerTraitConst {
		fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SoftsignLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SoftsignLayer }
	
	impl Drop for SoftsignLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SoftsignLayer_delete(self.as_raw_mut_SoftsignLayer()) };
		}
	}
	
	unsafe impl Send for SoftsignLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for SoftsignLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for SoftsignLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for SoftsignLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SoftsignLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SoftsignLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SoftsignLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftsignLayerTraitConst for SoftsignLayer {
		#[inline] fn as_raw_SoftsignLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SoftsignLayerTrait for SoftsignLayer {
		#[inline] fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SoftsignLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SoftsignLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SoftsignLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SoftsignLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SoftsignLayer, crate::dnn::ActivationLayer, cv_dnn_SoftsignLayer_to_ActivationLayer }
	
	boxed_cast_base! { SoftsignLayer, core::Algorithm, cv_dnn_SoftsignLayer_to_Algorithm }
	
	boxed_cast_base! { SoftsignLayer, crate::dnn::Layer, cv_dnn_SoftsignLayer_to_Layer }
	
	impl std::fmt::Debug for SoftsignLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SoftsignLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SplitLayer]
	pub trait SplitLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_SplitLayer(&self) -> *const c_void;
	
		/// Number of copies that will be produced (is ignored when negative).
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
		#[inline]
		fn set_outputs_count(&mut self, val: i32) {
			let ret = unsafe { sys::cv_dnn_SplitLayer_propOutputsCount_int(self.as_raw_mut_SplitLayer(), val) };
			ret
		}
		
	}
	
	pub struct SplitLayer {
		ptr: *mut c_void
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
	
	impl crate::dnn::LayerTraitConst for SplitLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SplitLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SplitLayerTraitConst for SplitLayer {
		#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SplitLayerTrait for SplitLayer {
		#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SplitLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SplitLayer>> {
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
	
	/// Constant methods for [crate::dnn::SqrtLayer]
	pub trait SqrtLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SqrtLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SqrtLayer]
	pub trait SqrtLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SqrtLayerTraitConst {
		fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SqrtLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { SqrtLayer }
	
	impl Drop for SqrtLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_SqrtLayer_delete(self.as_raw_mut_SqrtLayer()) };
		}
	}
	
	unsafe impl Send for SqrtLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for SqrtLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for SqrtLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for SqrtLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SqrtLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SqrtLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SqrtLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SqrtLayerTraitConst for SqrtLayer {
		#[inline] fn as_raw_SqrtLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SqrtLayerTrait for SqrtLayer {
		#[inline] fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SqrtLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SqrtLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_SqrtLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::SqrtLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { SqrtLayer, crate::dnn::ActivationLayer, cv_dnn_SqrtLayer_to_ActivationLayer }
	
	boxed_cast_base! { SqrtLayer, core::Algorithm, cv_dnn_SqrtLayer_to_Algorithm }
	
	boxed_cast_base! { SqrtLayer, crate::dnn::Layer, cv_dnn_SqrtLayer_to_Layer }
	
	impl std::fmt::Debug for SqrtLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("SqrtLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::SwishLayer]
	pub trait SwishLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_SwishLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::SwishLayer]
	pub trait SwishLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::SwishLayerTraitConst {
		fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct SwishLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for SwishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for SwishLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for SwishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for SwishLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SwishLayerTraitConst for SwishLayer {
		#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::SwishLayerTrait for SwishLayer {
		#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl SwishLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::SwishLayer>> {
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
	pub trait TanHLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_TanHLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::TanHLayer]
	pub trait TanHLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::TanHLayerTraitConst {
		fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct TanHLayer {
		ptr: *mut c_void
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
	
	impl core::AlgorithmTraitConst for TanHLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for TanHLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for TanHLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for TanHLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanHLayerTraitConst for TanHLayer {
		#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TanHLayerTrait for TanHLayer {
		#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TanHLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::TanHLayer>> {
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
	
	/// Constant methods for [crate::dnn::TanLayer]
	pub trait TanLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_TanLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::TanLayer]
	pub trait TanLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::TanLayerTraitConst {
		fn as_raw_mut_TanLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct TanLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TanLayer }
	
	impl Drop for TanLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_TanLayer_delete(self.as_raw_mut_TanLayer()) };
		}
	}
	
	unsafe impl Send for TanLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for TanLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for TanLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for TanLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for TanLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for TanLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for TanLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanLayerTraitConst for TanLayer {
		#[inline] fn as_raw_TanLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TanLayerTrait for TanLayer {
		#[inline] fn as_raw_mut_TanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TanLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::TanLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TanLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::TanLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { TanLayer, crate::dnn::ActivationLayer, cv_dnn_TanLayer_to_ActivationLayer }
	
	boxed_cast_base! { TanLayer, core::Algorithm, cv_dnn_TanLayer_to_Algorithm }
	
	boxed_cast_base! { TanLayer, crate::dnn::Layer, cv_dnn_TanLayer_to_Layer }
	
	impl std::fmt::Debug for TanLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TanLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::TextDetectionModel]
	pub trait TextDetectionModelTraitConst: crate::dnn::ModelTraitConst {
		fn as_raw_TextDetectionModel(&self) -> *const c_void;
	
		/// Performs detection
		/// 
		/// Given the input @p frame, prepare network input, run network inference, post-process network output and return result detections.
		/// 
		/// Each result is quadrangle's 4 points in this order:
		/// - bottom-left
		/// - top-left
		/// - top-right
		/// - bottom-right
		/// 
		/// Use cv::getPerspectiveTransform function to retrieve image region without perspective transformations.
		/// 
		/// 
		/// Note: If DL model doesn't support that kind of output then result may be derived from detectTextRectangles() output.
		/// 
		/// ## Parameters
		/// * frame: The input image
		/// * detections:[out] array with detections' quadrangles (4 points per result)
		/// * confidences:[out] array with detection confidences
		#[inline]
		fn detect_with_confidences(&self, frame: &impl core::ToInputArray, detections: &mut core::Vector<core::Vector<core::Point>>, confidences: &mut core::Vector<f32>) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vectorLvectorLPointGGR_vectorLfloatGR(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfVectorOfPoint(), confidences.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs detection
		/// 
		/// Given the input @p frame, prepare network input, run network inference, post-process network output and return result detections.
		/// 
		/// Each result is quadrangle's 4 points in this order:
		/// - bottom-left
		/// - top-left
		/// - top-right
		/// - bottom-right
		/// 
		/// Use cv::getPerspectiveTransform function to retrieve image region without perspective transformations.
		/// 
		/// 
		/// Note: If DL model doesn't support that kind of output then result may be derived from detectTextRectangles() output.
		/// 
		/// ## Parameters
		/// * frame: The input image
		/// * detections:[out] array with detections' quadrangles (4 points per result)
		/// * confidences:[out] array with detection confidences
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn detect(&self, frame: &impl core::ToInputArray, detections: &mut core::Vector<core::Vector<core::Point>>) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_detect_const_const__InputArrayR_vectorLvectorLPointGGR(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs detection
		/// 
		/// Given the input @p frame, prepare network input, run network inference, post-process network output and return result detections.
		/// 
		/// Each result is rotated rectangle.
		/// 
		/// 
		/// Note: Result may be inaccurate in case of strong perspective transformations.
		/// 
		/// ## Parameters
		/// * frame: the input image
		/// * detections:[out] array with detections' RotationRect results
		/// * confidences:[out] array with detection confidences
		#[inline]
		fn detect_text_rectangles(&self, frame: &impl core::ToInputArray, detections: &mut core::Vector<core::RotatedRect>, confidences: &mut core::Vector<f32>) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vectorLRotatedRectGR_vectorLfloatGR(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfRotatedRect(), confidences.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Performs detection
		/// 
		/// Given the input @p frame, prepare network input, run network inference, post-process network output and return result detections.
		/// 
		/// Each result is rotated rectangle.
		/// 
		/// 
		/// Note: Result may be inaccurate in case of strong perspective transformations.
		/// 
		/// ## Parameters
		/// * frame: the input image
		/// * detections:[out] array with detections' RotationRect results
		/// * confidences:[out] array with detection confidences
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn detect_text_rectangles_1(&self, frame: &impl core::ToInputArray, detections: &mut core::Vector<core::RotatedRect>) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_detectTextRectangles_const_const__InputArrayR_vectorLRotatedRectGR(self.as_raw_TextDetectionModel(), frame.as_raw__InputArray(), detections.as_raw_mut_VectorOfRotatedRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::dnn::TextDetectionModel]
	pub trait TextDetectionModelTrait: crate::dnn::ModelTrait + crate::dnn::TextDetectionModelTraitConst {
		fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void;
	
	}
	
	/// Base class for text detection networks
	pub struct TextDetectionModel {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TextDetectionModel }
	
	impl Drop for TextDetectionModel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_TextDetectionModel_delete(self.as_raw_mut_TextDetectionModel()) };
		}
	}
	
	unsafe impl Send for TextDetectionModel {}
	
	impl crate::dnn::ModelTraitConst for TextDetectionModel {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for TextDetectionModel {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TextDetectionModelTraitConst for TextDetectionModel {
		#[inline] fn as_raw_TextDetectionModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TextDetectionModelTrait for TextDetectionModel {
		#[inline] fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TextDetectionModel {
	}
	
	boxed_cast_base! { TextDetectionModel, crate::dnn::Model, cv_dnn_TextDetectionModel_to_Model }
	
	impl std::fmt::Debug for TextDetectionModel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TextDetectionModel")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::TextDetectionModel_DB]
	pub trait TextDetectionModel_DBTraitConst: crate::dnn::TextDetectionModelTraitConst {
		fn as_raw_TextDetectionModel_DB(&self) -> *const c_void;
	
		#[inline]
		fn get_binary_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_getBinaryThreshold_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_polygon_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_getPolygonThreshold_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_unclip_ratio(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_getUnclipRatio_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_candidates(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_getMaxCandidates_const(self.as_raw_TextDetectionModel_DB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::dnn::TextDetectionModel_DB]
	pub trait TextDetectionModel_DBTrait: crate::dnn::TextDetectionModelTrait + crate::dnn::TextDetectionModel_DBTraitConst {
		fn as_raw_mut_TextDetectionModel_DB(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_binary_threshold(&mut self, binary_threshold: f32) -> Result<crate::dnn::TextDetectionModel_DB> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_setBinaryThreshold_float(self.as_raw_mut_TextDetectionModel_DB(), binary_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_polygon_threshold(&mut self, polygon_threshold: f32) -> Result<crate::dnn::TextDetectionModel_DB> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_setPolygonThreshold_float(self.as_raw_mut_TextDetectionModel_DB(), polygon_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_unclip_ratio(&mut self, unclip_ratio: f64) -> Result<crate::dnn::TextDetectionModel_DB> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_setUnclipRatio_double(self.as_raw_mut_TextDetectionModel_DB(), unclip_ratio, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_max_candidates(&mut self, max_candidates: i32) -> Result<crate::dnn::TextDetectionModel_DB> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_setMaxCandidates_int(self.as_raw_mut_TextDetectionModel_DB(), max_candidates, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// This class represents high-level API for text detection DL networks compatible with DB model.
	/// 
	/// Related publications: [liao2020real](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_liao2020real)
	/// Paper: <https://arxiv.org/abs/1911.08947>
	/// For more information about the hyper-parameters setting, please refer to <https://github.com/MhLiao/DB>
	/// 
	/// Configurable parameters:
	/// - (float) binaryThreshold - The threshold of the binary map. It is usually set to 0.3.
	/// - (float) polygonThreshold - The threshold of text polygons. It is usually set to 0.5, 0.6, and 0.7. Default is 0.5f
	/// - (double) unclipRatio - The unclip ratio of the detected text region, which determines the output size. It is usually set to 2.0.
	/// - (int) maxCandidates - The max number of the output results.
	pub struct TextDetectionModel_DB {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TextDetectionModel_DB }
	
	impl Drop for TextDetectionModel_DB {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_TextDetectionModel_DB_delete(self.as_raw_mut_TextDetectionModel_DB()) };
		}
	}
	
	unsafe impl Send for TextDetectionModel_DB {}
	
	impl crate::dnn::ModelTraitConst for TextDetectionModel_DB {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for TextDetectionModel_DB {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TextDetectionModelTraitConst for TextDetectionModel_DB {
		#[inline] fn as_raw_TextDetectionModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TextDetectionModelTrait for TextDetectionModel_DB {
		#[inline] fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TextDetectionModel_DBTraitConst for TextDetectionModel_DB {
		#[inline] fn as_raw_TextDetectionModel_DB(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TextDetectionModel_DBTrait for TextDetectionModel_DB {
		#[inline] fn as_raw_mut_TextDetectionModel_DB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TextDetectionModel_DB {
		#[inline]
		pub fn default() -> Result<crate::dnn::TextDetectionModel_DB> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text detection algorithm from deep learning network.
		/// ## Parameters
		/// * network: Net object.
		#[inline]
		pub fn new(network: &crate::dnn::Net) -> Result<crate::dnn::TextDetectionModel_DB> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text detection model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn new_1(model: &str, config: &str) -> Result<crate::dnn::TextDetectionModel_DB> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_stringR_const_stringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text detection model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn new_def(model: &str) -> Result<crate::dnn::TextDetectionModel_DB> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_DB_TextDetectionModel_DB_const_stringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_DB::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for TextDetectionModel_DB {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_TextDetectionModel_DB_implicitClone_const(self.as_raw_TextDetectionModel_DB())) }
		}
	}
	
	boxed_cast_base! { TextDetectionModel_DB, crate::dnn::Model, cv_dnn_TextDetectionModel_DB_to_Model }
	
	boxed_cast_base! { TextDetectionModel_DB, crate::dnn::TextDetectionModel, cv_dnn_TextDetectionModel_DB_to_TextDetectionModel }
	
	impl std::fmt::Debug for TextDetectionModel_DB {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TextDetectionModel_DB")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::TextDetectionModel_EAST]
	pub trait TextDetectionModel_EASTTraitConst: crate::dnn::TextDetectionModelTraitConst {
		fn as_raw_TextDetectionModel_EAST(&self) -> *const c_void;
	
		/// Get the detection confidence threshold
		#[inline]
		fn get_confidence_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_getConfidenceThreshold_const(self.as_raw_TextDetectionModel_EAST(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Get the detection confidence threshold
		#[inline]
		fn get_nms_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_getNMSThreshold_const(self.as_raw_TextDetectionModel_EAST(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::dnn::TextDetectionModel_EAST]
	pub trait TextDetectionModel_EASTTrait: crate::dnn::TextDetectionModelTrait + crate::dnn::TextDetectionModel_EASTTraitConst {
		fn as_raw_mut_TextDetectionModel_EAST(&mut self) -> *mut c_void;
	
		/// Set the detection confidence threshold
		/// ## Parameters
		/// * confThreshold: A threshold used to filter boxes by confidences
		#[inline]
		fn set_confidence_threshold(&mut self, conf_threshold: f32) -> Result<crate::dnn::TextDetectionModel_EAST> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_setConfidenceThreshold_float(self.as_raw_mut_TextDetectionModel_EAST(), conf_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set the detection NMS filter threshold
		/// ## Parameters
		/// * nmsThreshold: A threshold used in non maximum suppression
		#[inline]
		fn set_nms_threshold(&mut self, nms_threshold: f32) -> Result<crate::dnn::TextDetectionModel_EAST> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_setNMSThreshold_float(self.as_raw_mut_TextDetectionModel_EAST(), nms_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// This class represents high-level API for text detection DL networks compatible with EAST model.
	/// 
	/// Configurable parameters:
	/// - (float) confThreshold - used to filter boxes by confidences, default: 0.5f
	/// - (float) nmsThreshold - used in non maximum suppression, default: 0.0f
	pub struct TextDetectionModel_EAST {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TextDetectionModel_EAST }
	
	impl Drop for TextDetectionModel_EAST {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_delete(self.as_raw_mut_TextDetectionModel_EAST()) };
		}
	}
	
	unsafe impl Send for TextDetectionModel_EAST {}
	
	impl crate::dnn::ModelTraitConst for TextDetectionModel_EAST {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for TextDetectionModel_EAST {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TextDetectionModelTraitConst for TextDetectionModel_EAST {
		#[inline] fn as_raw_TextDetectionModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TextDetectionModelTrait for TextDetectionModel_EAST {
		#[inline] fn as_raw_mut_TextDetectionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TextDetectionModel_EASTTraitConst for TextDetectionModel_EAST {
		#[inline] fn as_raw_TextDetectionModel_EAST(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TextDetectionModel_EASTTrait for TextDetectionModel_EAST {
		#[inline] fn as_raw_mut_TextDetectionModel_EAST(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TextDetectionModel_EAST {
		#[inline]
		pub fn default() -> Result<crate::dnn::TextDetectionModel_EAST> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text detection algorithm from deep learning network
		/// ## Parameters
		/// * network: Net object
		#[inline]
		pub fn new(network: &crate::dnn::Net) -> Result<crate::dnn::TextDetectionModel_EAST> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text detection model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn from_file(model: &str, config: &str) -> Result<crate::dnn::TextDetectionModel_EAST> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_stringR_const_stringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text detection model from network represented in one of the supported formats.
		/// An order of @p model and @p config arguments does not matter.
		/// ## Parameters
		/// * model: Binary file contains trained weights.
		/// * config: Text file contains network configuration.
		/// 
		/// ## Note
		/// This alternative version of [from_file] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn from_file_def(model: &str) -> Result<crate::dnn::TextDetectionModel_EAST> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextDetectionModel_EAST_TextDetectionModel_EAST_const_stringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextDetectionModel_EAST::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for TextDetectionModel_EAST {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_TextDetectionModel_EAST_implicitClone_const(self.as_raw_TextDetectionModel_EAST())) }
		}
	}
	
	boxed_cast_base! { TextDetectionModel_EAST, crate::dnn::Model, cv_dnn_TextDetectionModel_EAST_to_Model }
	
	boxed_cast_base! { TextDetectionModel_EAST, crate::dnn::TextDetectionModel, cv_dnn_TextDetectionModel_EAST_to_TextDetectionModel }
	
	impl std::fmt::Debug for TextDetectionModel_EAST {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TextDetectionModel_EAST")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::TextRecognitionModel]
	pub trait TextRecognitionModelTraitConst: crate::dnn::ModelTraitConst {
		fn as_raw_TextRecognitionModel(&self) -> *const c_void;
	
		/// Get the decoding method
		/// ## Returns
		/// the decoding method
		#[inline]
		fn get_decode_type(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_getDecodeType_const(self.as_raw_TextRecognitionModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Get the vocabulary for recognition.
		/// ## Returns
		/// vocabulary the associated vocabulary
		#[inline]
		fn get_vocabulary(&self) -> Result<core::Vector<String>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_getVocabulary_const(self.as_raw_TextRecognitionModel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Given the @p input frame, create input blob, run net and return recognition result
		/// ## Parameters
		/// * frame: The input image
		/// ## Returns
		/// The text recognition result
		#[inline]
		fn recognize(&self, frame: &impl core::ToInputArray) -> Result<String> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR(self.as_raw_TextRecognitionModel(), frame.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Given the @p input frame, create input blob, run net and return recognition result
		/// ## Parameters
		/// * frame: The input image
		/// * roiRects: List of text detection regions of interest (cv::Rect, CV_32SC4). ROIs is be cropped as the network inputs
		/// * results:[out] A set of text recognition results.
		#[inline]
		fn recognize_1(&self, frame: &impl core::ToInputArray, roi_rects: &impl core::ToInputArray, results: &mut core::Vector<String>) -> Result<()> {
			input_array_arg!(frame);
			input_array_arg!(roi_rects);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_recognize_const_const__InputArrayR_const__InputArrayR_vectorLstringGR(self.as_raw_TextRecognitionModel(), frame.as_raw__InputArray(), roi_rects.as_raw__InputArray(), results.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::dnn::TextRecognitionModel]
	pub trait TextRecognitionModelTrait: crate::dnn::ModelTrait + crate::dnn::TextRecognitionModelTraitConst {
		fn as_raw_mut_TextRecognitionModel(&mut self) -> *mut c_void;
	
		/// Set the decoding method of translating the network output into string
		/// ## Parameters
		/// * decodeType: The decoding method of translating the network output into string, currently supported type:
		///    - `"CTC-greedy"` greedy decoding for the output of CTC-based methods
		///    - `"CTC-prefix-beam-search"` Prefix beam search decoding for the output of CTC-based methods
		#[inline]
		fn set_decode_type(&mut self, decode_type: &str) -> Result<crate::dnn::TextRecognitionModel> {
			extern_container_arg!(decode_type);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_setDecodeType_const_stringR(self.as_raw_mut_TextRecognitionModel(), decode_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set the decoding method options for `"CTC-prefix-beam-search"` decode usage
		/// ## Parameters
		/// * beamSize: Beam size for search
		/// * vocPruneSize: Parameter to optimize big vocabulary search,
		/// only take top @p vocPruneSize tokens in each search step, @p vocPruneSize <= 0 stands for disable this prune.
		/// 
		/// ## C++ default parameters
		/// * voc_prune_size: 0
		#[inline]
		fn set_decode_opts_ctc_prefix_beam_search(&mut self, beam_size: i32, voc_prune_size: i32) -> Result<crate::dnn::TextRecognitionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_setDecodeOptsCTCPrefixBeamSearch_int_int(self.as_raw_mut_TextRecognitionModel(), beam_size, voc_prune_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set the decoding method options for `"CTC-prefix-beam-search"` decode usage
		/// ## Parameters
		/// * beamSize: Beam size for search
		/// * vocPruneSize: Parameter to optimize big vocabulary search,
		/// only take top @p vocPruneSize tokens in each search step, @p vocPruneSize <= 0 stands for disable this prune.
		/// 
		/// ## Note
		/// This alternative version of [set_decode_opts_ctc_prefix_beam_search] function uses the following default values for its arguments:
		/// * voc_prune_size: 0
		#[inline]
		fn set_decode_opts_ctc_prefix_beam_search_def(&mut self, beam_size: i32) -> Result<crate::dnn::TextRecognitionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_setDecodeOptsCTCPrefixBeamSearch_int(self.as_raw_mut_TextRecognitionModel(), beam_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Set the vocabulary for recognition.
		/// ## Parameters
		/// * vocabulary: the associated vocabulary of the network.
		#[inline]
		fn set_vocabulary(&mut self, vocabulary: &core::Vector<String>) -> Result<crate::dnn::TextRecognitionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_setVocabulary_const_vectorLstringGR(self.as_raw_mut_TextRecognitionModel(), vocabulary.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// This class represents high-level API for text recognition networks.
	/// 
	/// TextRecognitionModel allows to set params for preprocessing input image.
	/// TextRecognitionModel creates net from file with trained weights and config,
	/// sets preprocessing input, runs forward pass and return recognition result.
	/// For TextRecognitionModel, CRNN-CTC is supported.
	pub struct TextRecognitionModel {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TextRecognitionModel }
	
	impl Drop for TextRecognitionModel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_TextRecognitionModel_delete(self.as_raw_mut_TextRecognitionModel()) };
		}
	}
	
	unsafe impl Send for TextRecognitionModel {}
	
	impl crate::dnn::ModelTraitConst for TextRecognitionModel {
		#[inline] fn as_raw_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ModelTrait for TextRecognitionModel {
		#[inline] fn as_raw_mut_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TextRecognitionModelTraitConst for TextRecognitionModel {
		#[inline] fn as_raw_TextRecognitionModel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TextRecognitionModelTrait for TextRecognitionModel {
		#[inline] fn as_raw_mut_TextRecognitionModel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TextRecognitionModel {
		#[inline]
		pub fn default() -> Result<crate::dnn::TextRecognitionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_TextRecognitionModel(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create Text Recognition model from deep learning network
		/// Call setDecodeType() and setVocabulary() after constructor to initialize the decoding method
		/// ## Parameters
		/// * network: Net object
		#[inline]
		pub fn new(network: &crate::dnn::Net) -> Result<crate::dnn::TextRecognitionModel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_TextRecognitionModel_const_NetR(network.as_raw_Net(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text recognition model from network represented in one of the supported formats
		/// Call setDecodeType() and setVocabulary() after constructor to initialize the decoding method
		/// ## Parameters
		/// * model: Binary file contains trained weights
		/// * config: Text file contains network configuration
		/// 
		/// ## C++ default parameters
		/// * config: ""
		#[inline]
		pub fn from_file(model: &str, config: &str) -> Result<crate::dnn::TextRecognitionModel> {
			extern_container_arg!(model);
			extern_container_arg!(config);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_TextRecognitionModel_const_stringR_const_stringR(model.opencv_as_extern(), config.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create text recognition model from network represented in one of the supported formats
		/// Call setDecodeType() and setVocabulary() after constructor to initialize the decoding method
		/// ## Parameters
		/// * model: Binary file contains trained weights
		/// * config: Text file contains network configuration
		/// 
		/// ## Note
		/// This alternative version of [from_file] function uses the following default values for its arguments:
		/// * config: ""
		#[inline]
		pub fn from_file_def(model: &str) -> Result<crate::dnn::TextRecognitionModel> {
			extern_container_arg!(model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TextRecognitionModel_TextRecognitionModel_const_stringR(model.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::TextRecognitionModel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for TextRecognitionModel {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_dnn_TextRecognitionModel_implicitClone_const(self.as_raw_TextRecognitionModel())) }
		}
	}
	
	boxed_cast_base! { TextRecognitionModel, crate::dnn::Model, cv_dnn_TextRecognitionModel_to_Model }
	
	impl std::fmt::Debug for TextRecognitionModel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TextRecognitionModel")
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::ThresholdedReluLayer]
	pub trait ThresholdedReluLayerTraitConst: crate::dnn::ActivationLayerTraitConst {
		fn as_raw_ThresholdedReluLayer(&self) -> *const c_void;
	
		#[inline]
		fn alpha(&self) -> f32 {
			let ret = unsafe { sys::cv_dnn_ThresholdedReluLayer_propAlpha_const(self.as_raw_ThresholdedReluLayer()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::dnn::ThresholdedReluLayer]
	pub trait ThresholdedReluLayerTrait: crate::dnn::ActivationLayerTrait + crate::dnn::ThresholdedReluLayerTraitConst {
		fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_alpha(&mut self, val: f32) {
			let ret = unsafe { sys::cv_dnn_ThresholdedReluLayer_propAlpha_float(self.as_raw_mut_ThresholdedReluLayer(), val) };
			ret
		}
		
	}
	
	pub struct ThresholdedReluLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ThresholdedReluLayer }
	
	impl Drop for ThresholdedReluLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_ThresholdedReluLayer_delete(self.as_raw_mut_ThresholdedReluLayer()) };
		}
	}
	
	unsafe impl Send for ThresholdedReluLayer {}
	
	impl crate::dnn::ActivationLayerTraitConst for ThresholdedReluLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for ThresholdedReluLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for ThresholdedReluLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ThresholdedReluLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for ThresholdedReluLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for ThresholdedReluLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTraitConst for ThresholdedReluLayer {
		#[inline] fn as_raw_ThresholdedReluLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTrait for ThresholdedReluLayer {
		#[inline] fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ThresholdedReluLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::ThresholdedReluLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_ThresholdedReluLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::ThresholdedReluLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ThresholdedReluLayer, crate::dnn::ActivationLayer, cv_dnn_ThresholdedReluLayer_to_ActivationLayer }
	
	boxed_cast_base! { ThresholdedReluLayer, core::Algorithm, cv_dnn_ThresholdedReluLayer_to_Algorithm }
	
	boxed_cast_base! { ThresholdedReluLayer, crate::dnn::Layer, cv_dnn_ThresholdedReluLayer_to_Layer }
	
	impl std::fmt::Debug for ThresholdedReluLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ThresholdedReluLayer")
				.field("alpha", &crate::dnn::ThresholdedReluLayerTraitConst::alpha(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::TileLayer]
	pub trait TileLayerTraitConst: crate::dnn::LayerTraitConst {
		fn as_raw_TileLayer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::TileLayer]
	pub trait TileLayerTrait: crate::dnn::LayerTrait + crate::dnn::TileLayerTraitConst {
		fn as_raw_mut_TileLayer(&mut self) -> *mut c_void;
	
	}
	
	pub struct TileLayer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TileLayer }
	
	impl Drop for TileLayer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_dnn_TileLayer_delete(self.as_raw_mut_TileLayer()) };
		}
	}
	
	unsafe impl Send for TileLayer {}
	
	impl core::AlgorithmTraitConst for TileLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for TileLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for TileLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::LayerTrait for TileLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TileLayerTraitConst for TileLayer {
		#[inline] fn as_raw_TileLayer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::TileLayerTrait for TileLayer {
		#[inline] fn as_raw_mut_TileLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TileLayer {
		#[inline]
		pub fn create(params: &crate::dnn::LayerParams) -> Result<core::Ptr<crate::dnn::TileLayer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn_TileLayer_create_const_LayerParamsR(params.as_raw_LayerParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::dnn::TileLayer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { TileLayer, core::Algorithm, cv_dnn_TileLayer_to_Algorithm }
	
	boxed_cast_base! { TileLayer, crate::dnn::Layer, cv_dnn_TileLayer_to_Layer }
	
	impl std::fmt::Debug for TileLayer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TileLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::dnn::_Range]
	pub trait _RangeTraitConst: core::RangeTraitConst {
		fn as_raw__Range(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::dnn::_Range]
	pub trait _RangeTrait: core::RangeTrait + crate::dnn::_RangeTraitConst {
		fn as_raw_mut__Range(&mut self) -> *mut c_void;
	
	}
	
	pub struct _Range {
		ptr: *mut c_void
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
	
	impl crate::dnn::_RangeTraitConst for _Range {
		#[inline] fn as_raw__Range(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::dnn::_RangeTrait for _Range {
		#[inline] fn as_raw_mut__Range(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl _Range {
		#[inline]
		pub fn from_base(r: &core::Range) -> Result<crate::dnn::_Range> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_dnn__Range__Range_const_RangeR(r.as_raw_Range(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::dnn::_Range::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * size_: 1
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
pub use crate::manual::dnn::*;
}
