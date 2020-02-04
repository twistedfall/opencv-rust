//! # Deep Neural Network module
//! This module contains:
//! - API for new layers creation, layers are building bricks of neural networks;
//! - set of built-in most-useful Layers;
//! - API to construct and modify comprehensive neural networks from layers;
//! - functionality for loading serialized networks models from different frameworks.
//!
//! Functionality of this module is designed only for forward pass computations (i.e. network testing).
//! A network training is in principle not supported.
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NGRAPH: &'static str = "NGRAPH";
pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_API: &'static str = "NN_BUILDER";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_2: &'static str = "Myriad2";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_X: &'static str = "MyriadX";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_UNSPECIFIED: &'static str = "";
pub const DNN_BACKEND_DEFAULT: i32 = 0;
pub const DNN_BACKEND_HALIDE: i32 = 0+1;
/// Intel's Inference Engine computational backend
pub const DNN_BACKEND_INFERENCE_ENGINE: i32 = 0+2;
pub const DNN_BACKEND_INFERENCE_ENGINE_NGRAPH: i32 = 1000000;
pub const DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_2019: i32 = 1000000+1;
pub const DNN_BACKEND_OPENCV: i32 = 0+3;
pub const DNN_TARGET_CPU: i32 = 0;
/// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
pub const DNN_TARGET_FPGA: i32 = 4;
pub const DNN_TARGET_MYRIAD: i32 = 3;
pub const DNN_TARGET_OPENCL: i32 = 1;
pub const DNN_TARGET_OPENCL_FP16: i32 = 2;

/// Enum of computation backends supported by layers.
/// @see Net::setPreferableBackend
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Backend {
    DNN_BACKEND_DEFAULT = DNN_BACKEND_DEFAULT as isize,
    DNN_BACKEND_HALIDE = DNN_BACKEND_HALIDE as isize,
    /// Intel's Inference Engine computational backend
    DNN_BACKEND_INFERENCE_ENGINE = DNN_BACKEND_INFERENCE_ENGINE as isize,
    DNN_BACKEND_OPENCV = DNN_BACKEND_OPENCV as isize,
    DNN_BACKEND_INFERENCE_ENGINE_NGRAPH = DNN_BACKEND_INFERENCE_ENGINE_NGRAPH as isize,
    DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_2019 = DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_2019 as isize,
}

/// Enum of target devices for computations.
/// @see Net::setPreferableTarget
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Target {
    DNN_TARGET_CPU = DNN_TARGET_CPU as isize,
    DNN_TARGET_OPENCL = DNN_TARGET_OPENCL as isize,
    DNN_TARGET_OPENCL_FP16 = DNN_TARGET_OPENCL_FP16 as isize,
    DNN_TARGET_MYRIAD = DNN_TARGET_MYRIAD as isize,
    /// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
    DNN_TARGET_FPGA = DNN_TARGET_FPGA as isize,
}

///
/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes_f64(bboxes: &types::VectorOfRect2d, scores: &types::VectorOffloat, score_threshold: f32, nms_threshold: f32, indices: &mut types::VectorOfint, eta: f32, top_k: i32) -> Result<()> {
    unsafe { sys::cv_dnn_NMSBoxes_VectorOfRect2d_VectorOffloat_float_float_VectorOfint_float_int(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOffloat(), score_threshold, nms_threshold, indices.as_raw_VectorOfint(), eta, top_k) }.into_result()
}

/// Performs non maximum suppression given boxes and corresponding scores.
///
/// ## Parameters
/// * bboxes: a set of bounding boxes to apply NMS.
/// * scores: a set of corresponding confidences.
/// * score_threshold: a threshold used to filter boxes by score.
/// * nms_threshold: a threshold used in non maximum suppression.
/// * indices: the kept indices of bboxes after NMS.
/// * eta: a coefficient in adaptive threshold formula: ![inline formula](https://latex.codecogs.com/png.latex?nms%5C_threshold_%7Bi%2B1%7D%3Deta%5Ccdot%20nms%5C_threshold_i).
/// * top_k: if `>0`, keep at most @p top_k picked indices.
///
/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes(bboxes: &types::VectorOfRect, scores: &types::VectorOffloat, score_threshold: f32, nms_threshold: f32, indices: &mut types::VectorOfint, eta: f32, top_k: i32) -> Result<()> {
    unsafe { sys::cv_dnn_NMSBoxes_VectorOfRect_VectorOffloat_float_float_VectorOfint_float_int(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOffloat(), score_threshold, nms_threshold, indices.as_raw_VectorOfint(), eta, top_k) }.into_result()
}

///
/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes_rotated(bboxes: &types::VectorOfRotatedRect, scores: &types::VectorOffloat, score_threshold: f32, nms_threshold: f32, indices: &mut types::VectorOfint, eta: f32, top_k: i32) -> Result<()> {
    unsafe { sys::cv_dnn_NMSBoxes_VectorOfRotatedRect_VectorOffloat_float_float_VectorOfint_float_int(bboxes.as_raw_VectorOfRotatedRect(), scores.as_raw_VectorOffloat(), score_threshold, nms_threshold, indices.as_raw_VectorOfint(), eta, top_k) }.into_result()
}

/// Creates 4-dimensional blob from image.
///  @details This is an overloaded member function, provided for convenience.
///           It differs from the above function only in what argument(s) it accepts.
///
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_image_to(image: &dyn core::ToInputArray, blob: &mut dyn core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
    input_array_arg!(image);
    output_array_arg!(blob);
    unsafe { sys::cv_dnn_blobFromImage__InputArray__OutputArray_double_Size_Scalar_bool_bool_int(image.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, size, mean, swap_rb, crop, ddepth) }.into_result()
}

/// Creates 4-dimensional blob from image. Optionally resizes and crops @p image from center,
///  subtract @p mean values, scales values by @p scalefactor, swap Blue and Red channels.
/// ## Parameters
/// * image: input image (with 1-, 3- or 4-channels).
/// * size: spatial size for output image
/// * mean: scalar with mean values which are subtracted from channels. Values are intended
///  to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
/// * scalefactor: multiplier for @p image values.
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
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_image(image: &dyn core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
    input_array_arg!(image);
    unsafe { sys::cv_dnn_blobFromImage__InputArray_double_Size_Scalar_bool_bool_int(image.as_raw__InputArray(), scalefactor, size, mean, swap_rb, crop, ddepth) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Creates 4-dimensional blob from series of images.
///  @details This is an overloaded member function, provided for convenience.
///           It differs from the above function only in what argument(s) it accepts.
///
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_images_to(images: &dyn core::ToInputArray, blob: &mut dyn core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
    input_array_arg!(images);
    output_array_arg!(blob);
    unsafe { sys::cv_dnn_blobFromImages__InputArray__OutputArray_double_Size_Scalar_bool_bool_int(images.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, size, mean, swap_rb, crop, ddepth) }.into_result()
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
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_images(images: &dyn core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
    input_array_arg!(images);
    unsafe { sys::cv_dnn_blobFromImages__InputArray_double_Size_Scalar_bool_bool_int(images.as_raw__InputArray(), scalefactor, size, mean, swap_rb, crop, ddepth) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn clamp_range(r: &core::Range, axis_size: i32) -> Result<core::Range> {
    unsafe { sys::cv_dnn_clamp_Range_int(r.as_raw_Range(), axis_size) }.into_result().map(|ptr| core::Range { ptr })
}

pub fn clamp(ax: i32, dims: i32) -> Result<i32> {
    unsafe { sys::cv_dnn_clamp_int_int(ax, dims) }.into_result()
}

pub fn get_available_targets(be: crate::dnn::Backend) -> Result<types::VectorOfTarget> {
    unsafe { sys::cv_dnn_getAvailableTargets_Backend(be) }.into_result().map(|ptr| types::VectorOfTarget { ptr })
}

/// Returns Inference Engine internal backend API.
///
/// See values of `CV_DNN_BACKEND_INFERENCE_ENGINE_*` macros.
///
/// Default value is controlled through `OPENCV_DNN_BACKEND_INFERENCE_ENGINE_TYPE` runtime parameter (environment variable).
pub fn get_inference_engine_backend_type() -> Result<String> {
    unsafe { sys::cv_dnn_getInferenceEngineBackendType() }.into_result().map(crate::templ::receive_string_mut)
}

/// Returns Inference Engine VPU type.
///
/// See values of `CV_DNN_INFERENCE_ENGINE_VPU_TYPE_*` macros.
pub fn get_inference_engine_vpu_type() -> Result<String> {
    unsafe { sys::cv_dnn_getInferenceEngineVPUType() }.into_result().map(crate::templ::receive_string_mut)
}

pub fn get_plane(m: &core::Mat, n: i32, cn: i32) -> Result<core::Mat> {
    unsafe { sys::cv_dnn_getPlane_Mat_int_int(m.as_raw_Mat(), n, cn) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Parse a 4D blob and output the images it contains as 2D arrays through a simpler data structure
///  (std::vector<cv::Mat>).
/// ## Parameters
/// * blob_: 4 dimensional array (images, channels, height, width) in floating point precision (CV_32F) from
///  which you would like to extract the images.
/// * images_: [out] array of 2D Mat containing the images extracted from the blob in floating point precision
///  (CV_32F). They are non normalized neither mean added. The number of returned images equals the first dimension
///  of the blob (batch size). Every image has a number of channels equals to the second dimension of the blob (depth).
pub fn images_from_blob(blob_: &core::Mat, images_: &mut dyn core::ToOutputArray) -> Result<()> {
    output_array_arg!(images_);
    unsafe { sys::cv_dnn_imagesFromBlob_Mat__OutputArray(blob_.as_raw_Mat(), images_.as_raw__OutputArray()) }.into_result()
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
pub fn read_net_from_caffe(prototxt: &str, caffe_model: &str) -> Result<crate::dnn::Net> {
    string_arg!(prototxt);
    string_arg!(caffe_model);
    unsafe { sys::cv_dnn_readNetFromCaffe_String_String(prototxt.as_ptr(), caffe_model.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_caffe_buffer(buffer_proto: &types::VectorOfuchar, buffer_model: &types::VectorOfuchar) -> Result<crate::dnn::Net> {
    unsafe { sys::cv_dnn_readNetFromCaffe_VectorOfuchar_VectorOfuchar(buffer_proto.as_raw_VectorOfuchar(), buffer_model.as_raw_VectorOfuchar()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_caffe_str(buffer_proto: &str, len_proto: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
    string_arg!(buffer_proto);
    string_arg!(buffer_model);
    unsafe { sys::cv_dnn_readNetFromCaffe_const_char_X_size_t_const_char_X_size_t(buffer_proto.as_ptr(), len_proto, buffer_model.as_ptr(), len_model) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_darknet(cfg_file: &str, darknet_model: &str) -> Result<crate::dnn::Net> {
    string_arg!(cfg_file);
    string_arg!(darknet_model);
    unsafe { sys::cv_dnn_readNetFromDarknet_String_String(cfg_file.as_ptr(), darknet_model.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_darknet_buffer(buffer_cfg: &types::VectorOfuchar, buffer_model: &types::VectorOfuchar) -> Result<crate::dnn::Net> {
    unsafe { sys::cv_dnn_readNetFromDarknet_VectorOfuchar_VectorOfuchar(buffer_cfg.as_raw_VectorOfuchar(), buffer_model.as_raw_VectorOfuchar()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_darknet_str(buffer_cfg: &str, len_cfg: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
    string_arg!(buffer_cfg);
    string_arg!(buffer_model);
    unsafe { sys::cv_dnn_readNetFromDarknet_const_char_X_size_t_const_char_X_size_t(buffer_cfg.as_ptr(), len_cfg, buffer_model.as_ptr(), len_model) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * xml: XML configuration file with network's topology.
/// * bin: Binary file with trained weights.
/// ## Returns
/// Net object.
///  Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
///  backend.
pub fn read_net_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
    string_arg!(xml);
    string_arg!(bin);
    unsafe { sys::cv_dnn_readNetFromModelOptimizer_String_String(xml.as_ptr(), bin.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * bufferModelConfig: Buffer contains XML configuration with network's topology.
/// * bufferWeights: Buffer contains binary data with trained weights.
/// ## Returns
/// Net object.
///  Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
///  backend.
pub fn read_net_from_model_optimizer_1(buffer_model_config: &types::VectorOfuchar, buffer_weights: &types::VectorOfuchar) -> Result<crate::dnn::Net> {
    unsafe { sys::cv_dnn_readNetFromModelOptimizer_VectorOfuchar_VectorOfuchar(buffer_model_config.as_raw_VectorOfuchar(), buffer_weights.as_raw_VectorOfuchar()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * bufferModelConfigPtr: Pointer to buffer which contains XML configuration with network's topology.
/// * bufferModelConfigSize: Binary size of XML configuration data.
/// * bufferWeightsPtr: Pointer to buffer which contains binary data with trained weights.
/// * bufferWeightsSize: Binary size of trained weights data.
/// ## Returns
/// Net object.
///  Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
///  backend.
pub fn read_net_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
    unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_uchar_X_size_t_const_uchar_X_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model <a href="https://onnx.ai/">ONNX</a>.
/// ## Parameters
/// * onnxFile: path to the .onnx file with text description of the network architecture.
/// ## Returns
/// Network object that ready to do forward, throw an exception in failure cases.
pub fn read_net_from_onnx(onnx_file: &str) -> Result<crate::dnn::Net> {
    string_arg!(onnx_file);
    unsafe { sys::cv_dnn_readNetFromONNX_String(onnx_file.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model from <a href="https://onnx.ai/">ONNX</a>
///   in-memory buffer.
/// ## Parameters
/// * buffer: in-memory buffer that stores the ONNX model bytes.
/// ## Returns
/// Network object that ready to do forward, throw an exception
///        in failure cases.
pub fn read_net_from_onnx_buffer(buffer: &types::VectorOfuchar) -> Result<crate::dnn::Net> {
    unsafe { sys::cv_dnn_readNetFromONNX_VectorOfuchar(buffer.as_raw_VectorOfuchar()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model from <a href="https://onnx.ai/">ONNX</a>
///   in-memory buffer.
/// ## Parameters
/// * buffer: memory address of the first byte of the buffer.
/// * sizeBuffer: size of the buffer.
/// ## Returns
/// Network object that ready to do forward, throw an exception
///        in failure cases.
pub fn read_net_from_onnx_str(buffer: &str, size_buffer: size_t) -> Result<crate::dnn::Net> {
    string_arg!(buffer);
    unsafe { sys::cv_dnn_readNetFromONNX_const_char_X_size_t(buffer.as_ptr(), size_buffer) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_tensorflow(model: &str, config: &str) -> Result<crate::dnn::Net> {
    string_arg!(model);
    string_arg!(config);
    unsafe { sys::cv_dnn_readNetFromTensorflow_String_String(model.as_ptr(), config.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_tensorflow_buffer(buffer_model: &types::VectorOfuchar, buffer_config: &types::VectorOfuchar) -> Result<crate::dnn::Net> {
    unsafe { sys::cv_dnn_readNetFromTensorflow_VectorOfuchar_VectorOfuchar(buffer_model.as_raw_VectorOfuchar(), buffer_config.as_raw_VectorOfuchar()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_tensorflow_str(buffer_model: &str, len_model: size_t, buffer_config: &str, len_config: size_t) -> Result<crate::dnn::Net> {
    string_arg!(buffer_model);
    string_arg!(buffer_config);
    unsafe { sys::cv_dnn_readNetFromTensorflow_const_char_X_size_t_const_char_X_size_t(buffer_model.as_ptr(), len_model, buffer_config.as_ptr(), len_config) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_from_torch(model: &str, is_binary: bool, evaluate: bool) -> Result<crate::dnn::Net> {
    string_arg!(model);
    unsafe { sys::cv_dnn_readNetFromTorch_String_bool_bool(model.as_ptr(), is_binary, evaluate) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Read deep learning network represented in one of the supported formats.
/// ## Parameters
/// * model: Binary file contains trained weights. The following file
///                  extensions are expected for models from different frameworks:
///                  * `*.caffemodel` (Caffe, http://caffe.berkeleyvision.org/)
///                  * `*.pb` (TensorFlow, https://www.tensorflow.org/)
///                  * `*.t7` | `*.net` (Torch, http://torch.ch/)
///                  * `*.weights` (Darknet, https://pjreddie.com/darknet/)
///                  * `*.bin` (DLDT, https://software.intel.com/openvino-toolkit)
///                  * `*.onnx` (ONNX, https://onnx.ai/)
/// * config: Text file contains network configuration. It could be a
///                   file with the following extensions:
///                  * `*.prototxt` (Caffe, http://caffe.berkeleyvision.org/)
///                  * `*.pbtxt` (TensorFlow, https://www.tensorflow.org/)
///                  * `*.cfg` (Darknet, https://pjreddie.com/darknet/)
///                  * `*.xml` (DLDT, https://software.intel.com/openvino-toolkit)
/// * framework: Explicit framework name tag to determine a format.
/// ## Returns
/// Net object.
///
/// This function automatically detects an origin framework of trained model
/// and calls an appropriate function such @ref readNetFromCaffe, @ref readNetFromTensorflow,
/// @ref readNetFromTorch or @ref readNetFromDarknet. An order of @p model and @p config
/// arguments does not matter.
///
/// ## C++ default parameters
/// * config: ""
/// * framework: ""
pub fn read_net(model: &str, config: &str, framework: &str) -> Result<crate::dnn::Net> {
    string_arg!(model);
    string_arg!(config);
    string_arg!(framework);
    unsafe { sys::cv_dnn_readNet_String_String_String(model.as_ptr(), config.as_ptr(), framework.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
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
pub fn read_net_1(framework: &str, buffer_model: &types::VectorOfuchar, buffer_config: &types::VectorOfuchar) -> Result<crate::dnn::Net> {
    string_arg!(framework);
    unsafe { sys::cv_dnn_readNet_String_VectorOfuchar_VectorOfuchar(framework.as_ptr(), buffer_model.as_raw_VectorOfuchar(), buffer_config.as_raw_VectorOfuchar()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Creates blob from .pb file.
/// ## Parameters
/// * path: to the .pb file with input tensor.
/// ## Returns
/// Mat.
pub fn read_tensor_from_onnx(path: &str) -> Result<core::Mat> {
    string_arg!(path);
    unsafe { sys::cv_dnn_readTensorFromONNX_String(path.as_ptr()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
///  @warning This function has the same limitations as readNetFromTorch().
///
/// ## C++ default parameters
/// * is_binary: true
pub fn read_torch_blob(filename: &str, is_binary: bool) -> Result<core::Mat> {
    string_arg!(filename);
    unsafe { sys::cv_dnn_readTorchBlob_String_bool(filename.as_ptr(), is_binary) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Release a Myriad device (binded by OpenCV).
///
/// Single Myriad device cannot be shared across multiple processes which uses
/// Inference Engine's Myriad plugin.
pub fn reset_myriad_device() -> Result<()> {
    unsafe { sys::cv_dnn_resetMyriadDevice() }.into_result()
}

/// Specify Inference Engine internal backend API.
///
/// See values of `CV_DNN_BACKEND_INFERENCE_ENGINE_*` macros.
///
/// ## Returns
/// previous value of internal backend API
pub fn set_inference_engine_backend_type(new_backend_type: &str) -> Result<String> {
    string_arg!(new_backend_type);
    unsafe { sys::cv_dnn_setInferenceEngineBackendType_String(new_backend_type.as_ptr()) }.into_result().map(crate::templ::receive_string_mut)
}

pub fn shape(mat: &core::Mat) -> Result<types::VectorOfint> {
    unsafe { sys::cv_dnn_shape_Mat(mat.as_raw_Mat()) }.into_result().map(|ptr| types::VectorOfint { ptr })
}

pub fn shape_umat(mat: &core::UMat) -> Result<types::VectorOfint> {
    unsafe { sys::cv_dnn_shape_UMat(mat.as_raw_UMat()) }.into_result().map(|ptr| types::VectorOfint { ptr })
}

pub fn shape_nd(dims: &i32, n: i32) -> Result<types::VectorOfint> {
    unsafe { sys::cv_dnn_shape_const_int_X_int(dims, n) }.into_result().map(|ptr| types::VectorOfint { ptr })
}

///
/// ## C++ default parameters
/// * a1: -1
/// * a2: -1
/// * a3: -1
pub fn shape_3d(a0: i32, a1: i32, a2: i32, a3: i32) -> Result<types::VectorOfint> {
    unsafe { sys::cv_dnn_shape_int_int_int_int(a0, a1, a2, a3) }.into_result().map(|ptr| types::VectorOfint { ptr })
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
///       is taken from NVidia's Caffe fork: https://github.com/NVIDIA/caffe.
///       So the resulting model may be used there.
///
/// ## C++ default parameters
/// * layers_types: std::vector<String>()
pub fn shrink_caffe_model(src: &str, dst: &str, layers_types: &types::VectorOfString) -> Result<()> {
    string_arg!(src);
    string_arg!(dst);
    unsafe { sys::cv_dnn_shrinkCaffeModel_String_String_VectorOfString(src.as_ptr(), dst.as_ptr(), layers_types.as_raw_VectorOfString()) }.into_result()
}

pub fn slice_1d(m: &core::Mat, r0: &core::Range) -> Result<core::Mat> {
    unsafe { sys::cv_dnn_slice_Mat_Range(m.as_raw_Mat(), r0.as_raw_Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn slice_2d(m: &core::Mat, r0: &core::Range, r1: &core::Range) -> Result<core::Mat> {
    unsafe { sys::cv_dnn_slice_Mat_Range_Range(m.as_raw_Mat(), r0.as_raw_Range(), r1.as_raw_Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn slice_3d(m: &core::Mat, r0: &core::Range, r1: &core::Range, r2: &core::Range) -> Result<core::Mat> {
    unsafe { sys::cv_dnn_slice_Mat_Range_Range_Range(m.as_raw_Mat(), r0.as_raw_Range(), r1.as_raw_Range(), r2.as_raw_Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn slice_4d(m: &core::Mat, r0: &core::Range, r1: &core::Range, r2: &core::Range, r3: &core::Range) -> Result<core::Mat> {
    unsafe { sys::cv_dnn_slice_Mat_Range_Range_Range_Range(m.as_raw_Mat(), r0.as_raw_Range(), r1.as_raw_Range(), r2.as_raw_Range(), r3.as_raw_Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Create a text representation for a binary network stored in protocol buffer format.
/// ## Parameters
/// * model: A path to binary network.
/// * output: A path to output text file to be created.
///
///
/// Note: To reduce output file size, trained weights are not included.
pub fn write_text_graph(model: &str, output: &str) -> Result<()> {
    string_arg!(model);
    string_arg!(output);
    unsafe { sys::cv_dnn_writeTextGraph_String_String(model.as_ptr(), output.as_ptr()) }.into_result()
}

// boxed class cv::dnn::AbsLayer
pub struct AbsLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for AbsLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_AbsLayer_delete(self.ptr) };
    }
}

impl AbsLayer {
    #[inline(always)] pub fn as_raw_AbsLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for AbsLayer {}

impl core::AlgorithmTrait for AbsLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for AbsLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for AbsLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl AbsLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfAbsLayer> {
        unsafe { sys::cv_dnn_AbsLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfAbsLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::ActivationLayer
pub trait ActivationLayer: crate::dnn::LayerTrait {
    fn as_raw_ActivationLayer(&self) -> *mut c_void;
    fn forward_slice(&self, src: &f32, dst: &mut f32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32) -> Result<()> {
        unsafe { sys::cv_dnn_ActivationLayer_forwardSlice_const_const_float_X_float_X_int_size_t_int_int(self.as_raw_ActivationLayer(), src, dst, len, out_plane_size, cn0, cn1) }.into_result()
    }
    
}

// boxed class cv::dnn::BNLLLayer
pub struct BNLLLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BNLLLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_BNLLLayer_delete(self.ptr) };
    }
}

impl BNLLLayer {
    #[inline(always)] pub fn as_raw_BNLLLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BNLLLayer {}

impl core::AlgorithmTrait for BNLLLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for BNLLLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for BNLLLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl BNLLLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBNLLLayer> {
        unsafe { sys::cv_dnn_BNLLLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBNLLLayer { ptr })
    }
    
}

// boxed class cv::dnn::BackendNode
/// Derivatives of this class encapsulates functions of certain backends.
pub struct BackendNode {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BackendNode {
    fn drop(&mut self) {
        unsafe { sys::cv_BackendNode_delete(self.ptr) };
    }
}

impl BackendNode {
    #[inline(always)] pub fn as_raw_BackendNode(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BackendNode {}

// Generating impl for trait crate::dnn::BackendWrapper
/// Derivatives of this class wraps cv::Mat for different backends and targets.
pub trait BackendWrapper {
    fn as_raw_BackendWrapper(&self) -> *mut c_void;
    /// Transfer data to CPU host memory.
    fn copy_to_host(&mut self) -> Result<()> {
        unsafe { sys::cv_dnn_BackendWrapper_copyToHost(self.as_raw_BackendWrapper()) }.into_result()
    }
    
    /// Indicate that an actual data is on CPU.
    fn set_host_dirty(&mut self) -> Result<()> {
        unsafe { sys::cv_dnn_BackendWrapper_setHostDirty(self.as_raw_BackendWrapper()) }.into_result()
    }
    
}

// Generating impl for trait crate::dnn::BaseConvolutionLayer
pub trait BaseConvolutionLayerTrait: crate::dnn::LayerTrait {
    fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void;
}

// boxed class cv::dnn::BaseConvolutionLayer
pub struct BaseConvolutionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BaseConvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_BaseConvolutionLayer_delete(self.ptr) };
    }
}

impl BaseConvolutionLayer {
    #[inline(always)] pub fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BaseConvolutionLayer {}

impl core::AlgorithmTrait for BaseConvolutionLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::BaseConvolutionLayerTrait for BaseConvolutionLayer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for BaseConvolutionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::dnn::BatchNormLayer
pub struct BatchNormLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BatchNormLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_BatchNormLayer_delete(self.ptr) };
    }
}

impl BatchNormLayer {
    #[inline(always)] pub fn as_raw_BatchNormLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BatchNormLayer {}

impl core::AlgorithmTrait for BatchNormLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for BatchNormLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for BatchNormLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl BatchNormLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBatchNormLayer> {
        unsafe { sys::cv_dnn_BatchNormLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBatchNormLayer { ptr })
    }
    
}

// boxed class cv::dnn::BlankLayer
/// # Partial List of Implemented Layers
/// This subsection of dnn module contains information about built-in layers and their descriptions.
///
/// Classes listed here, in fact, provides C++ API for creating instances of built-in layers.
/// In addition to this way of layers instantiation, there is a more common factory API (see @ref dnnLayerFactory), it allows to create layers dynamically (by name) and register new ones.
/// You can use both API, but factory API is less convenient for native C++ programming and basically designed for use inside importers (see @ref readNetFromCaffe(), @ref readNetFromTorch(), @ref readNetFromTensorflow()).
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
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BlankLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_BlankLayer_delete(self.ptr) };
    }
}

impl BlankLayer {
    #[inline(always)] pub fn as_raw_BlankLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BlankLayer {}

impl core::AlgorithmTrait for BlankLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for BlankLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl BlankLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_BlankLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::ChannelsPReLULayer
pub struct ChannelsPReLULayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ChannelsPReLULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ChannelsPReLULayer_delete(self.ptr) };
    }
}

impl ChannelsPReLULayer {
    #[inline(always)] pub fn as_raw_ChannelsPReLULayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ChannelsPReLULayer {}

impl core::AlgorithmTrait for ChannelsPReLULayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for ChannelsPReLULayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ChannelsPReLULayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ChannelsPReLULayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_ChannelsPReLULayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::ConcatLayer
pub struct ConcatLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ConcatLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ConcatLayer_delete(self.ptr) };
    }
}

impl ConcatLayer {
    #[inline(always)] pub fn as_raw_ConcatLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ConcatLayer {}

impl core::AlgorithmTrait for ConcatLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ConcatLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConcatLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfConcatLayer> {
        unsafe { sys::cv_dnn_ConcatLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfConcatLayer { ptr })
    }
    
}

// boxed class cv::dnn::ConstLayer
/// Constant layer produces the same data blob at an every forward pass.
pub struct ConstLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ConstLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ConstLayer_delete(self.ptr) };
    }
}

impl ConstLayer {
    #[inline(always)] pub fn as_raw_ConstLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ConstLayer {}

impl core::AlgorithmTrait for ConstLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ConstLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConstLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_ConstLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::ConvolutionLayer
pub struct ConvolutionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ConvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ConvolutionLayer_delete(self.ptr) };
    }
}

impl ConvolutionLayer {
    #[inline(always)] pub fn as_raw_ConvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ConvolutionLayer {}

impl core::AlgorithmTrait for ConvolutionLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ConvolutionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConvolutionLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBaseConvolutionLayer> {
        unsafe { sys::cv_dnn_ConvolutionLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
    }
    
}

// boxed class cv::dnn::CropAndResizeLayer
pub struct CropAndResizeLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CropAndResizeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_CropAndResizeLayer_delete(self.ptr) };
    }
}

impl CropAndResizeLayer {
    #[inline(always)] pub fn as_raw_CropAndResizeLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CropAndResizeLayer {}

impl core::AlgorithmTrait for CropAndResizeLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for CropAndResizeLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl CropAndResizeLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_CropAndResizeLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::CropLayer
pub struct CropLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CropLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_CropLayer_delete(self.ptr) };
    }
}

impl CropLayer {
    #[inline(always)] pub fn as_raw_CropLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CropLayer {}

impl core::AlgorithmTrait for CropLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for CropLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl CropLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_CropLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::DeconvolutionLayer
pub struct DeconvolutionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DeconvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_DeconvolutionLayer_delete(self.ptr) };
    }
}

impl DeconvolutionLayer {
    #[inline(always)] pub fn as_raw_DeconvolutionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DeconvolutionLayer {}

impl core::AlgorithmTrait for DeconvolutionLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::BaseConvolutionLayerTrait for DeconvolutionLayer {
    #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for DeconvolutionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl DeconvolutionLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBaseConvolutionLayer> {
        unsafe { sys::cv_dnn_DeconvolutionLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
    }
    
}

// boxed class cv::dnn::DetectionOutputLayer
pub struct DetectionOutputLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DetectionOutputLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_DetectionOutputLayer_delete(self.ptr) };
    }
}

impl DetectionOutputLayer {
    #[inline(always)] pub fn as_raw_DetectionOutputLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DetectionOutputLayer {}

impl core::AlgorithmTrait for DetectionOutputLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for DetectionOutputLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl DetectionOutputLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfDetectionOutputLayer> {
        unsafe { sys::cv_dnn_DetectionOutputLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfDetectionOutputLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::Dict
/// This class implements name-value dictionary, values are instances of DictValue.
pub trait DictTrait {
    fn as_raw_Dict(&self) -> *mut c_void;
    /// Checks a presence of the @p key in the dictionary.
    fn has(&self, key: &str) -> Result<bool> {
        string_arg!(key);
        unsafe { sys::cv_dnn_Dict_has_const_String(self.as_raw_Dict(), key.as_ptr()) }.into_result()
    }
    
    /// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
    unsafe fn ptr_mut(&mut self, key: &str) -> Result<crate::dnn::DictValue> {
        string_arg!(key);
        { sys::cv_dnn_Dict_ptr_String(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    unsafe fn ptr(&self, key: &str) -> Result<crate::dnn::DictValue> {
        string_arg!(key);
        { sys::cv_dnn_Dict_ptr_const_String(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// If the @p key in the dictionary then returns its value, else an error will be generated.
    fn get(&self, key: &str) -> Result<crate::dnn::DictValue> {
        string_arg!(key);
        unsafe { sys::cv_dnn_Dict_get_const_String(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
    fn set(&mut self, key: &str, value: &mut crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
        string_arg!(key);
        unsafe { sys::cv_dnn_Dict_set_String_DictValue(self.as_raw_Dict(), key.as_ptr(), value.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Erase @p key from the dictionary.
    fn erase(&mut self, key: &str) -> Result<()> {
        string_arg!(key);
        unsafe { sys::cv_dnn_Dict_erase_String(self.as_raw_Dict(), key.as_ptr()) }.into_result()
    }
    
}

// boxed class cv::dnn::Dict
/// This class implements name-value dictionary, values are instances of DictValue.
pub struct Dict {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Dict {
    fn drop(&mut self) {
        unsafe { sys::cv_Dict_delete(self.ptr) };
    }
}

impl Dict {
    #[inline(always)] pub fn as_raw_Dict(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Dict {}

impl crate::dnn::DictTrait for Dict {
    #[inline(always)] fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::dnn::DictValue
/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
///  @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub struct DictValue {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DictValue {
    fn drop(&mut self) {
        unsafe { sys::cv_DictValue_delete(self.ptr) };
    }
}

impl DictValue {
    #[inline(always)] pub fn as_raw_DictValue(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DictValue {}

impl DictValue {
    pub fn copy(r: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_DictValue(r.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs integer scalar
    pub fn from_bool(i: bool) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_bool(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs integer scalar
    ///
    /// ## C++ default parameters
    /// * i: 0
    pub fn from_i64(i: i64) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_int64(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs integer scalar
    pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_int(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs integer scalar
    pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_unsigned(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    /// Constructs floating point scalar
    pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
        unsafe { sys::cv_dnn_DictValue_DictValue_double(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    pub fn from_str(s: &str) -> Result<crate::dnn::DictValue> {
        string_arg!(s);
        unsafe { sys::cv_dnn_DictValue_DictValue_const_char_X(s.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
    }
    
    pub fn size(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_DictValue_size_const(self.as_raw_DictValue()) }.into_result()
    }
    
    pub fn is_int(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_DictValue_isInt_const(self.as_raw_DictValue()) }.into_result()
    }
    
    pub fn is_string(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_DictValue_isString_const(self.as_raw_DictValue()) }.into_result()
    }
    
    pub fn is_real(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_DictValue_isReal_const(self.as_raw_DictValue()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * idx: -1
    pub fn get_int_value(&self, idx: i32) -> Result<i32> {
        unsafe { sys::cv_dnn_DictValue_getIntValue_const_int(self.as_raw_DictValue(), idx) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * idx: -1
    pub fn get_real_value(&self, idx: i32) -> Result<f64> {
        unsafe { sys::cv_dnn_DictValue_getRealValue_const_int(self.as_raw_DictValue(), idx) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * idx: -1
    pub fn get_string_value(&self, idx: i32) -> Result<String> {
        unsafe { sys::cv_dnn_DictValue_getStringValue_const_int(self.as_raw_DictValue(), idx) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

// boxed class cv::dnn::ELULayer
pub struct ELULayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ELULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ELULayer_delete(self.ptr) };
    }
}

impl ELULayer {
    #[inline(always)] pub fn as_raw_ELULayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ELULayer {}

impl core::AlgorithmTrait for ELULayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for ELULayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ELULayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ELULayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfELULayer> {
        unsafe { sys::cv_dnn_ELULayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfELULayer { ptr })
    }
    
}

// boxed class cv::dnn::EltwiseLayer
/// Element wise operation on inputs
///
/// Extra optional parameters:
/// - "operation" as string. Values are "sum" (default), "prod", "max", "div"
/// - "coeff" as float array. Specify weights of inputs for SUM operation
/// - "output_channels_mode" as string. Values are "same" (default, all input must have the same layout), "input_0", "input_0_truncate", "max_input_channels"
pub struct EltwiseLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for EltwiseLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_EltwiseLayer_delete(self.ptr) };
    }
}

impl EltwiseLayer {
    #[inline(always)] pub fn as_raw_EltwiseLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for EltwiseLayer {}

impl core::AlgorithmTrait for EltwiseLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for EltwiseLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl EltwiseLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfEltwiseLayer> {
        unsafe { sys::cv_dnn_EltwiseLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfEltwiseLayer { ptr })
    }
    
}

// boxed class cv::dnn::FlattenLayer
pub struct FlattenLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FlattenLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_FlattenLayer_delete(self.ptr) };
    }
}

impl FlattenLayer {
    #[inline(always)] pub fn as_raw_FlattenLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FlattenLayer {}

impl core::AlgorithmTrait for FlattenLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for FlattenLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl FlattenLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfFlattenLayer> {
        unsafe { sys::cv_dnn_FlattenLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfFlattenLayer { ptr })
    }
    
}

// boxed class cv::dnn::InnerProductLayer
pub struct InnerProductLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for InnerProductLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_InnerProductLayer_delete(self.ptr) };
    }
}

impl InnerProductLayer {
    #[inline(always)] pub fn as_raw_InnerProductLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for InnerProductLayer {}

impl core::AlgorithmTrait for InnerProductLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for InnerProductLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl InnerProductLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfInnerProductLayer> {
        unsafe { sys::cv_dnn_InnerProductLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfInnerProductLayer { ptr })
    }
    
}

// boxed class cv::dnn::InterpLayer
/// Bilinear resize layer from https://github.com/cdmh/deeplab-public-ver2
///
/// It differs from @ref ResizeLayer in output shape and resize scales computations.
pub struct InterpLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for InterpLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_InterpLayer_delete(self.ptr) };
    }
}

impl InterpLayer {
    #[inline(always)] pub fn as_raw_InterpLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for InterpLayer {}

impl core::AlgorithmTrait for InterpLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for InterpLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl InterpLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_InterpLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::LRNLayer
pub struct LRNLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LRNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_LRNLayer_delete(self.ptr) };
    }
}

impl LRNLayer {
    #[inline(always)] pub fn as_raw_LRNLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LRNLayer {}

impl core::AlgorithmTrait for LRNLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for LRNLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl LRNLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLRNLayer> {
        unsafe { sys::cv_dnn_LRNLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLRNLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::LSTMLayer
/// LSTM recurrent layer
pub trait LSTMLayer: crate::dnn::LayerTrait {
    fn as_raw_LSTMLayer(&self) -> *mut c_void;
    /// **Deprecated**: Use LayerParams::blobs instead.
    ///
    ///  Set trained weights for LSTM layer.
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
    fn set_weights(&mut self, wh: &core::Mat, wx: &core::Mat, b: &core::Mat) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setWeights_Mat_Mat_Mat(self.as_raw_LSTMLayer(), wh.as_raw_Mat(), wx.as_raw_Mat(), b.as_raw_Mat()) }.into_result()
    }
    
    /// **Deprecated**: Use flag `produce_cell_output` in LayerParams.
    ///
    ///  Specifies either interpret first dimension of input blob as timestamp dimenion either as sample.
    ///
    /// If flag is set to true then shape of input blob will be interpreted as [`T`, `N`, `[data dims]`] where `T` specifies number of timestamps, `N` is number of independent streams.
    /// In this case each forward() call will iterate through `T` timestamps and update layer's state `T` times.
    ///
    /// If flag is set to false then shape of input blob will be interpreted as [`N`, `[data dims]`].
    /// In this case each forward() call will make one iteration and produce one timestamp with shape [`N`, `[out dims]`].
    ///
    /// ## C++ default parameters
    /// * _use: true
    #[deprecated = "Use flag `produce_cell_output` in LayerParams."]
    fn set_use_timstamps_dim(&mut self, _use: bool) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim_bool(self.as_raw_LSTMLayer(), _use) }.into_result()
    }
    
    /// **Deprecated**: Use flag `use_timestamp_dim` in LayerParams.
    ///
    ///  If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
    /// @details Shape of the second output is the same as first output.
    ///
    /// ## C++ default parameters
    /// * produce: false
    #[deprecated = "Use flag `use_timestamp_dim` in LayerParams."]
    fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
        unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput_bool(self.as_raw_LSTMLayer(), produce) }.into_result()
    }
    
    fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
        string_arg!(mut input_name);
        unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_LSTMLayer(), input_name.as_ptr() as _) }.into_result()
    }
    
    fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
        string_arg!(output_name);
        unsafe { sys::cv_dnn_LSTMLayer_outputNameToIndex_String(self.as_raw_LSTMLayer(), output_name.as_ptr()) }.into_result()
    }
    
}

impl dyn LSTMLayer + '_ {
    /// Creates instance of LSTM layer
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLSTMLayer> {
        unsafe { sys::cv_dnn_LSTMLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLSTMLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::Layer
/// This interface class allows to build new Layers - are building blocks of networks.
///
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub trait LayerTrait: core::AlgorithmTrait {
    fn as_raw_Layer(&self) -> *mut c_void;
    /// List of learned parameters must be stored here to allow read them by using Net::getParam().
    fn blobs(&mut self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_dnn_Layer_blobs(self.as_raw_Layer()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    /// List of learned parameters must be stored here to allow read them by using Net::getParam().
    fn set_blobs(&mut self, val: types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_set_blobs_VectorOfMat(self.as_raw_Layer(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Name of the layer instance, can be used for logging or other internal purposes.
    fn name(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_Layer_name(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Name of the layer instance, can be used for logging or other internal purposes.
    fn set_name(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_Layer_set_name_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result()
    }
    
    /// Type name which was used for creating layer by layer factory.
    fn _type(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_Layer_type(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Type name which was used for creating layer by layer factory.
    fn set_type(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_Layer_set_type_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result()
    }
    
    /// prefer target for layer forwarding
    fn preferable_target(&self) -> Result<i32> {
        unsafe { sys::cv_dnn_Layer_preferableTarget_const(self.as_raw_Layer()) }.into_result()
    }
    
    /// prefer target for layer forwarding
    fn set_preferable_target(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_set_preferableTarget_int(self.as_raw_Layer(), val) }.into_result()
    }
    
    /// Computes and sets internal parameters according to inputs, outputs and blobs.
    /// ## Parameters
    /// * inputs: vector of already allocated input blobs
    /// * outputs: [out] vector of already allocated output blobs
    ///
    /// If this method is called after network has allocated all memory for input and output blobs
    /// and before inferencing.
    fn finalize(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(inputs);
        output_array_arg!(outputs);
        unsafe { sys::cv_dnn_Layer_finalize__InputArray__OutputArray(self.as_raw_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray()) }.into_result()
    }
    
    /// Given the @p input blobs, computes the output @p blobs.
    /// ## Parameters
    /// * inputs: the input blobs.
    /// * outputs: [out] allocated output blobs, which will store results of the computation.
    /// * internals: [out] allocated internal blobs
    fn forward(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray, internals: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(inputs);
        output_array_arg!(outputs);
        output_array_arg!(internals);
        unsafe { sys::cv_dnn_Layer_forward__InputArray__OutputArray__OutputArray(self.as_raw_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray()) }.into_result()
    }
    
    /// Given the @p input blobs, computes the output @p blobs.
    /// ## Parameters
    /// * inputs: the input blobs.
    /// * outputs: [out] allocated output blobs, which will store results of the computation.
    /// * internals: [out] allocated internal blobs
    fn forward_fallback(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray, internals: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(inputs);
        output_array_arg!(outputs);
        output_array_arg!(internals);
        unsafe { sys::cv_dnn_Layer_forward_fallback__InputArray__OutputArray__OutputArray(self.as_raw_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray()) }.into_result()
    }
    
    /// **Deprecated**: Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead
    #[deprecated = "Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead"]
    fn finalize_mat(&mut self, inputs: &types::VectorOfMat) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_dnn_Layer_finalize_VectorOfMat(self.as_raw_Layer(), inputs.as_raw_VectorOfMat()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    /// Allocates layer and computes output.
    /// **Deprecated**: This method will be removed in the future release.
    #[deprecated = "This method will be removed in the future release."]
    fn run(&mut self, inputs: &types::VectorOfMat, outputs: &mut types::VectorOfMat, internals: &mut types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_run_VectorOfMat_VectorOfMat_VectorOfMat(self.as_raw_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), internals.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Returns index of input blob into the input array.
    /// ## Parameters
    /// * inputName: label of input blob
    ///
    /// Each layer input and output can be labeled to easily identify them using "%<layer_name%>[.output_name]" notation.
    /// This method maps label of input blob to its index into input vector.
    fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
        string_arg!(mut input_name);
        unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_Layer(), input_name.as_ptr() as _) }.into_result()
    }
    
    /// Returns index of output blob in output array.
    ///  @see inputNameToIndex()
    fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
        string_arg!(output_name);
        unsafe { sys::cv_dnn_Layer_outputNameToIndex_String(self.as_raw_Layer(), output_name.as_ptr()) }.into_result()
    }
    
    /// Ask layer if it support specific backend for doing computations.
    /// ## Parameters
    /// * backendId: computation backend identifier.
    /// @see Backend
    fn support_backend(&mut self, backend_id: i32) -> Result<bool> {
        unsafe { sys::cv_dnn_Layer_supportBackend_int(self.as_raw_Layer(), backend_id) }.into_result()
    }
    
    /// Returns Halide backend node.
    /// ## Parameters
    /// * inputs: Input Halide buffers.
    /// @see BackendNode, BackendWrapper
    ///
    /// Input buffers should be exactly the same that will be used in forward invocations.
    /// Despite we can use Halide::ImageParam based on input shape only,
    /// it helps prevent some memory management issues (if something wrong,
    /// Halide tests will be failed).
    fn init_halide(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper) -> Result<types::PtrOfBackendNode> {
        unsafe { sys::cv_dnn_Layer_initHalide_VectorOfPtrOfBackendWrapper(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
    }
    
    fn init_inf_engine(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper) -> Result<types::PtrOfBackendNode> {
        unsafe { sys::cv_dnn_Layer_initInfEngine_VectorOfPtrOfBackendWrapper(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
    }
    
    fn init_ngraph(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper, nodes: &types::VectorOfPtrOfBackendNode) -> Result<types::PtrOfBackendNode> {
        unsafe { sys::cv_dnn_Layer_initNgraph_VectorOfPtrOfBackendWrapper_VectorOfPtrOfBackendNode(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
    }
    
    /// Implement layers fusing.
    /// ## Parameters
    /// * node: Backend node of bottom layer.
    /// @see BackendNode
    ///
    /// Actual for graph-based backends. If layer attached successfully,
    /// returns non-empty cv::Ptr to node of the same backend.
    /// Fuse only over the last function.
    fn try_attach(&mut self, node: &types::PtrOfBackendNode) -> Result<types::PtrOfBackendNode> {
        unsafe { sys::cv_dnn_Layer_tryAttach_PtrOfBackendNode(self.as_raw_Layer(), node.as_raw_PtrOfBackendNode()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
    }
    
    /// Tries to attach to the layer the subsequent activation layer, i.e. do the layer fusion in a partial case.
    /// ## Parameters
    /// * layer: The subsequent activation layer.
    ///
    /// Returns true if the activation layer has been attached successfully.
    fn set_activation(&mut self, layer: &types::PtrOfActivationLayer) -> Result<bool> {
        unsafe { sys::cv_dnn_Layer_setActivation_PtrOfActivationLayer(self.as_raw_Layer(), layer.as_raw_PtrOfActivationLayer()) }.into_result()
    }
    
    /// Try to fuse current layer with a next one
    /// ## Parameters
    /// * top: Next layer to be fused.
    /// ## Returns
    /// True if fusion was performed.
    fn try_fuse(&mut self, top: &mut types::PtrOfLayer) -> Result<bool> {
        unsafe { sys::cv_dnn_Layer_tryFuse_PtrOfLayer(self.as_raw_Layer(), top.as_raw_PtrOfLayer()) }.into_result()
    }
    
    /// Returns parameters of layers with channel-wise multiplication and addition.
    /// ## Parameters
    /// * scale: [out] Channel-wise multipliers. Total number of values should
    ///                   be equal to number of channels.
    /// * shift: [out] Channel-wise offsets. Total number of values should
    ///                   be equal to number of channels.
    ///
    /// Some layers can fuse their transformations with further layers.
    /// In example, convolution + batch normalization. This way base layer
    /// use weights from layer after it. Fused layer is skipped.
    /// By default, @p scale and @p shift are empty that means layer has no
    /// element-wise multiplications or additions.
    fn get_scale_shift(&self, scale: &mut core::Mat, shift: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_getScaleShift_const_Mat_Mat(self.as_raw_Layer(), scale.as_raw_Mat(), shift.as_raw_Mat()) }.into_result()
    }
    
    /// "Deattaches" all the layers, attached to particular layer.
    fn unset_attached(&mut self) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_unsetAttached(self.as_raw_Layer()) }.into_result()
    }
    
    fn get_memory_shapes(&self, inputs: &types::VectorOfVectorOfint, required_outputs: i32, outputs: &mut types::VectorOfVectorOfint, internals: &mut types::VectorOfVectorOfint) -> Result<bool> {
        unsafe { sys::cv_dnn_Layer_getMemoryShapes_const_VectorOfVectorOfint_int_VectorOfVectorOfint_VectorOfVectorOfint(self.as_raw_Layer(), inputs.as_raw_VectorOfVectorOfint(), required_outputs, outputs.as_raw_VectorOfVectorOfint(), internals.as_raw_VectorOfVectorOfint()) }.into_result()
    }
    
    fn get_flops(&self, inputs: &types::VectorOfVectorOfint, outputs: &types::VectorOfVectorOfint) -> Result<i64> {
        unsafe { sys::cv_dnn_Layer_getFLOPS_const_VectorOfVectorOfint_VectorOfVectorOfint(self.as_raw_Layer(), inputs.as_raw_VectorOfVectorOfint(), outputs.as_raw_VectorOfVectorOfint()) }.into_result()
    }
    
    /// Initializes only #name, #type and #blobs fields.
    fn set_params_from(&mut self, params: &crate::dnn::LayerParams) -> Result<()> {
        unsafe { sys::cv_dnn_Layer_setParamsFrom_LayerParams(self.as_raw_Layer(), params.as_raw_LayerParams()) }.into_result()
    }
    
}

// boxed class cv::dnn::Layer
/// This interface class allows to build new Layers - are building blocks of networks.
///
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub struct Layer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Layer {
    fn drop(&mut self) {
        unsafe { sys::cv_Layer_delete(self.ptr) };
    }
}

impl Layer {
    #[inline(always)] pub fn as_raw_Layer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Layer {}

impl core::AlgorithmTrait for Layer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for Layer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl Layer {
    pub fn default() -> Result<crate::dnn::Layer> {
        unsafe { sys::cv_dnn_Layer_Layer() }.into_result().map(|ptr| crate::dnn::Layer { ptr })
    }
    
    /// Initializes only #name, #type and #blobs fields.
    pub fn new(params: &crate::dnn::LayerParams) -> Result<crate::dnn::Layer> {
        unsafe { sys::cv_dnn_Layer_Layer_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| crate::dnn::Layer { ptr })
    }
    
}

// boxed class cv::dnn::LayerFactory
/// %Layer factory allows to create instances of registered layers.
pub struct LayerFactory {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LayerFactory {
    fn drop(&mut self) {
        unsafe { sys::cv_LayerFactory_delete(self.ptr) };
    }
}

impl LayerFactory {
    #[inline(always)] pub fn as_raw_LayerFactory(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LayerFactory {}

impl LayerFactory {
    /// Unregisters registered layer with specified type name. Thread-safe.
    pub fn unregister_layer(_type: &str) -> Result<()> {
        string_arg!(_type);
        unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_String(_type.as_ptr()) }.into_result()
    }
    
    /// Creates instance of registered layer.
    /// ## Parameters
    /// * type: type name of creating layer.
    /// * params: parameters which will be used for layer initialization.
    ///
    /// Note: Thread-safe.
    pub fn create_layer_instance(_type: &str, params: &mut crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        string_arg!(_type);
        unsafe { sys::cv_dnn_LayerFactory_createLayerInstance_String_LayerParams(_type.as_ptr(), params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::LayerParams
/// This class provides all data needed to initialize layer.
///
/// It includes dictionary with scalar params (which can be read by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub struct LayerParams {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for LayerParams {
    fn drop(&mut self) {
        unsafe { sys::cv_LayerParams_delete(self.ptr) };
    }
}

impl LayerParams {
    #[inline(always)] pub fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for LayerParams {}

impl crate::dnn::DictTrait for LayerParams {
    #[inline(always)] fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

impl LayerParams {
    /// List of learned parameters stored as blobs.
    pub fn blobs(&mut self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_dnn_LayerParams_blobs(self.as_raw_LayerParams()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    /// List of learned parameters stored as blobs.
    pub fn set_blobs(&mut self, val: types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_dnn_LayerParams_set_blobs_VectorOfMat(self.as_raw_LayerParams(), val.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Name of the layer instance (optional, can be used internal purposes).
    pub fn name(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_LayerParams_name(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Name of the layer instance (optional, can be used internal purposes).
    pub fn set_name(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_LayerParams_set_name_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result()
    }
    
    /// Type name which was used for creating layer by layer factory (optional).
    pub fn _type(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_LayerParams_type(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Type name which was used for creating layer by layer factory (optional).
    pub fn set_type(&mut self, val: &str) -> Result<()> {
        string_arg!(mut val);
        unsafe { sys::cv_dnn_LayerParams_set_type_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result()
    }
    
    pub fn default() -> Result<crate::dnn::LayerParams> {
        unsafe { sys::cv_dnn_LayerParams_LayerParams() }.into_result().map(|ptr| crate::dnn::LayerParams { ptr })
    }
    
}

// boxed class cv::dnn::MVNLayer
pub struct MVNLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MVNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_MVNLayer_delete(self.ptr) };
    }
}

impl MVNLayer {
    #[inline(always)] pub fn as_raw_MVNLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MVNLayer {}

impl core::AlgorithmTrait for MVNLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for MVNLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl MVNLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfMVNLayer> {
        unsafe { sys::cv_dnn_MVNLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfMVNLayer { ptr })
    }
    
}

// boxed class cv::dnn::MaxUnpoolLayer
pub struct MaxUnpoolLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MaxUnpoolLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_MaxUnpoolLayer_delete(self.ptr) };
    }
}

impl MaxUnpoolLayer {
    #[inline(always)] pub fn as_raw_MaxUnpoolLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MaxUnpoolLayer {}

impl core::AlgorithmTrait for MaxUnpoolLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for MaxUnpoolLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl MaxUnpoolLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfMaxUnpoolLayer> {
        unsafe { sys::cv_dnn_MaxUnpoolLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfMaxUnpoolLayer { ptr })
    }
    
}

// boxed class cv::dnn::MishLayer
pub struct MishLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for MishLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_MishLayer_delete(self.ptr) };
    }
}

impl MishLayer {
    #[inline(always)] pub fn as_raw_MishLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MishLayer {}

impl core::AlgorithmTrait for MishLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for MishLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for MishLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl MishLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfMishLayer> {
        unsafe { sys::cv_dnn_MishLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfMishLayer { ptr })
    }
    
}

// boxed class cv::dnn::Net
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
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe { sys::cv_Net_delete(self.ptr) };
    }
}

impl Net {
    #[inline(always)] pub fn as_raw_Net(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Net {}

impl Net {
    /// Default constructor.
    pub fn default() -> Result<crate::dnn::Net> {
        unsafe { sys::cv_dnn_Net_Net() }.into_result().map(|ptr| crate::dnn::Net { ptr })
    }
    
    /// Create a network from Intel's Model Optimizer intermediate representation (IR).
    /// ## Parameters
    /// * xml: XML configuration file with network's topology.
    /// * bin: Binary file with trained weights.
    ///  Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
    ///  backend.
    pub fn read_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
        string_arg!(xml);
        string_arg!(bin);
        unsafe { sys::cv_dnn_Net_readFromModelOptimizer_String_String(xml.as_ptr(), bin.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
    }
    
    /// Create a network from Intel's Model Optimizer in-memory buffers with intermediate representation (IR).
    /// ## Parameters
    /// * bufferModelConfig: buffer with model's configuration.
    /// * bufferWeights: buffer with model's trained weights.
    /// ## Returns
    /// Net object.
    pub fn read_from_model_optimizer_1(buffer_model_config: &types::VectorOfuchar, buffer_weights: &types::VectorOfuchar) -> Result<crate::dnn::Net> {
        unsafe { sys::cv_dnn_Net_readFromModelOptimizer_VectorOfuchar_VectorOfuchar(buffer_model_config.as_raw_VectorOfuchar(), buffer_weights.as_raw_VectorOfuchar()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
    }
    
    /// Create a network from Intel's Model Optimizer in-memory buffers with intermediate representation (IR).
    /// ## Parameters
    /// * bufferModelConfigPtr: buffer pointer of model's configuration.
    /// * bufferModelConfigSize: buffer size of model's configuration.
    /// * bufferWeightsPtr: buffer pointer of model's trained weights.
    /// * bufferWeightsSize: buffer size of model's trained weights.
    /// ## Returns
    /// Net object.
    pub fn read_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
        unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_uchar_X_size_t_const_uchar_X_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size) }.into_result().map(|ptr| crate::dnn::Net { ptr })
    }
    
    /// Returns true if there are no layers in the network.
    pub fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_dnn_Net_empty_const(self.as_raw_Net()) }.into_result()
    }
    
    /// Dump net to String
    /// ## Returns
    /// String with structure, hyperparameters, backend, target and fusion
    ///  Call method after setInput(). To see correct backend, target and fusion run after forward().
    pub fn dump(&mut self) -> Result<String> {
        unsafe { sys::cv_dnn_Net_dump(self.as_raw_Net()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Dump net structure, hyperparameters, backend, target and fusion to dot file
    /// ## Parameters
    /// * path: path to output file with .dot extension
    ///  @see dump()
    pub fn dump_to_file(&mut self, path: &str) -> Result<()> {
        string_arg!(path);
        unsafe { sys::cv_dnn_Net_dumpToFile_String(self.as_raw_Net(), path.as_ptr()) }.into_result()
    }
    
    /// Adds new layer to the net.
    /// ## Parameters
    /// * name: unique name of the adding layer.
    /// * type: typename of the adding layer (type must be registered in LayerRegister).
    /// * params: parameters which will be used to initialize the creating layer.
    /// ## Returns
    /// unique identifier of created layer, or -1 if a failure will happen.
    pub fn add_layer(&mut self, name: &str, _type: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
        string_arg!(name);
        string_arg!(_type);
        unsafe { sys::cv_dnn_Net_addLayer_String_String_LayerParams(self.as_raw_Net(), name.as_ptr(), _type.as_ptr(), params.as_raw_LayerParams()) }.into_result()
    }
    
    /// Adds new layer and connects its first input to the first output of previously added layer.
    ///  @see addLayer()
    pub fn add_layer_to_prev(&mut self, name: &str, _type: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
        string_arg!(name);
        string_arg!(_type);
        unsafe { sys::cv_dnn_Net_addLayerToPrev_String_String_LayerParams(self.as_raw_Net(), name.as_ptr(), _type.as_ptr(), params.as_raw_LayerParams()) }.into_result()
    }
    
    /// Converts string name of the layer to the integer identifier.
    /// ## Returns
    /// id of the layer, or -1 if the layer wasn't found.
    pub fn get_layer_id(&mut self, layer: &str) -> Result<i32> {
        string_arg!(layer);
        unsafe { sys::cv_dnn_Net_getLayerId_String(self.as_raw_Net(), layer.as_ptr()) }.into_result()
    }
    
    pub fn get_layer_names(&self) -> Result<types::VectorOfString> {
        unsafe { sys::cv_dnn_Net_getLayerNames_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfString { ptr })
    }
    
    /// Returns pointer to layer with specified id or name which the network use.
    pub fn get_layer(&mut self, layer_id: &crate::dnn::DictValue) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_Net_getLayer_DictValue(self.as_raw_Net(), layer_id.as_raw_DictValue()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
    /// Returns pointers to input layers of specific layer.
    pub fn get_layer_inputs(&mut self, layer_id: &crate::dnn::DictValue) -> Result<types::VectorOfPtrOfLayer> {
        unsafe { sys::cv_dnn_Net_getLayerInputs_DictValue(self.as_raw_Net(), layer_id.as_raw_DictValue()) }.into_result().map(|ptr| types::VectorOfPtrOfLayer { ptr })
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
    ///
    ///  @see setNetInputs(), Layer::inputNameToIndex(), Layer::outputNameToIndex()
    pub fn connect_first_second(&mut self, out_pin: &str, inp_pin: &str) -> Result<()> {
        string_arg!(mut out_pin);
        string_arg!(mut inp_pin);
        unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_Net(), out_pin.as_ptr() as _, inp_pin.as_ptr() as _) }.into_result()
    }
    
    /// Connects #@p outNum output of the first layer to #@p inNum input of the second layer.
    /// ## Parameters
    /// * outLayerId: identifier of the first layer
    /// * outNum: number of the first layer output
    /// * inpLayerId: identifier of the second layer
    /// * inpNum: number of the second layer input
    pub fn connect(&mut self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
        unsafe { sys::cv_dnn_Net_connect_int_int_int_int(self.as_raw_Net(), out_layer_id, out_num, inp_layer_id, inp_num) }.into_result()
    }
    
    /// Sets outputs names of the network input pseudo layer.
    ///
    /// Each net always has special own the network input pseudo layer with id=0.
    /// This layer stores the user blobs only and don't make any computations.
    /// In fact, this layer provides the only way to pass user data into the network.
    /// As any other layer, this layer can label its outputs and this function provides an easy way to do this.
    pub fn set_inputs_names(&mut self, input_blob_names: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_dnn_Net_setInputsNames_VectorOfString(self.as_raw_Net(), input_blob_names.as_raw_VectorOfString()) }.into_result()
    }
    
    /// Runs forward pass to compute output of layer with name @p outputName.
    /// ## Parameters
    /// * outputName: name for layer which output is needed to get
    /// ## Returns
    /// blob for first output of specified layer.
    ///  @details By default runs forward pass for the whole network.
    ///
    /// ## C++ default parameters
    /// * output_name: String()
    pub fn forward(&mut self, output_name: &str) -> Result<core::Mat> {
        string_arg!(output_name);
        unsafe { sys::cv_dnn_Net_forward_String(self.as_raw_Net(), output_name.as_ptr()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Runs forward pass to compute output of layer with name @p outputName.
    /// ## Parameters
    /// * outputBlobs: contains all output blobs for specified layer.
    /// * outputName: name for layer which output is needed to get
    ///  @details If @p outputName is empty, runs forward pass for the whole network.
    ///
    /// ## C++ default parameters
    /// * output_name: String()
    pub fn forward_layer(&mut self, output_blobs: &mut dyn core::ToOutputArray, output_name: &str) -> Result<()> {
        output_array_arg!(output_blobs);
        string_arg!(output_name);
        unsafe { sys::cv_dnn_Net_forward__OutputArray_String(self.as_raw_Net(), output_blobs.as_raw__OutputArray(), output_name.as_ptr()) }.into_result()
    }
    
    /// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
    /// ## Parameters
    /// * outputBlobs: contains blobs for first outputs of specified layers.
    /// * outBlobNames: names for layers which outputs are needed to get
    pub fn forward_first_outputs(&mut self, output_blobs: &mut dyn core::ToOutputArray, out_blob_names: &types::VectorOfString) -> Result<()> {
        output_array_arg!(output_blobs);
        unsafe { sys::cv_dnn_Net_forward__OutputArray_VectorOfString(self.as_raw_Net(), output_blobs.as_raw__OutputArray(), out_blob_names.as_raw_VectorOfString()) }.into_result()
    }
    
    /// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
    /// ## Parameters
    /// * outputBlobs: contains all output blobs for each layer specified in @p outBlobNames.
    /// * outBlobNames: names for layers which outputs are needed to get
    pub fn forward_all(&mut self, output_blobs: &mut types::VectorOfVectorOfMat, out_blob_names: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_dnn_Net_forward_VectorOfVectorOfMat_VectorOfString(self.as_raw_Net(), output_blobs.as_raw_VectorOfVectorOfMat(), out_blob_names.as_raw_VectorOfString()) }.into_result()
    }
    
    /// Compile Halide layers.
    /// ## Parameters
    /// * scheduler: Path to YAML file with scheduling directives.
    /// @see setPreferableBackend
    ///
    /// Schedule layers that support Halide backend. Then compile them for
    /// specific target. For layers that not represented in scheduling file
    /// or if no manual scheduling used at all, automatic scheduling will be applied.
    pub fn set_halide_scheduler(&mut self, scheduler: &str) -> Result<()> {
        string_arg!(scheduler);
        unsafe { sys::cv_dnn_Net_setHalideScheduler_String(self.as_raw_Net(), scheduler.as_ptr()) }.into_result()
    }
    
    /// Ask network to use specific computation backend where it supported.
    /// ## Parameters
    /// * backendId: backend identifier.
    /// @see Backend
    ///
    /// If OpenCV is compiled with Intel's Inference Engine library, DNN_BACKEND_DEFAULT
    /// means DNN_BACKEND_INFERENCE_ENGINE. Otherwise it equals to DNN_BACKEND_OPENCV.
    pub fn set_preferable_backend(&mut self, backend_id: i32) -> Result<()> {
        unsafe { sys::cv_dnn_Net_setPreferableBackend_int(self.as_raw_Net(), backend_id) }.into_result()
    }
    
    /// Ask network to make computations on specific target device.
    /// ## Parameters
    /// * targetId: target identifier.
    /// @see Target
    ///
    /// List of supported combinations backend / target:
    /// |                        | DNN_BACKEND_OPENCV | DNN_BACKEND_INFERENCE_ENGINE | DNN_BACKEND_HALIDE |
    /// |------------------------|--------------------|------------------------------|--------------------|
    /// | DNN_TARGET_CPU         |                  + |                            + |                  + |
    /// | DNN_TARGET_OPENCL      |                  + |                            + |                  + |
    /// | DNN_TARGET_OPENCL_FP16 |                  + |                            + |                    |
    /// | DNN_TARGET_MYRIAD      |                    |                            + |                    |
    /// | DNN_TARGET_FPGA        |                    |                            + |                    |
    pub fn set_preferable_target(&mut self, target_id: i32) -> Result<()> {
        unsafe { sys::cv_dnn_Net_setPreferableTarget_int(self.as_raw_Net(), target_id) }.into_result()
    }
    
    /// Sets the new input value for the network
    /// ## Parameters
    /// * blob: A new blob. Should have CV_32F or CV_8U depth.
    /// * name: A name of input layer.
    /// * scalefactor: An optional normalization scale.
    /// * mean: An optional mean subtraction values.
    ///  @see connect(String, String) to know format of the descriptor.
    ///
    ///  If scale or mean values are specified, a final input blob is computed
    ///  as:
    /// ![block formula](https://latex.codecogs.com/png.latex?input%28n%2Cc%2Ch%2Cw%29%20%3D%20scalefactor%20%5Ctimes%20%28blob%28n%2Cc%2Ch%2Cw%29%20-%20mean_c%29)
    ///
    /// ## C++ default parameters
    /// * name: ""
    /// * scalefactor: 1.0
    /// * mean: Scalar()
    pub fn set_input(&mut self, blob: &dyn core::ToInputArray, name: &str, scalefactor: f64, mean: core::Scalar) -> Result<()> {
        input_array_arg!(blob);
        string_arg!(name);
        unsafe { sys::cv_dnn_Net_setInput__InputArray_String_double_Scalar(self.as_raw_Net(), blob.as_raw__InputArray(), name.as_ptr(), scalefactor, mean) }.into_result()
    }
    
    /// Sets the new value for the learned param of the layer.
    /// ## Parameters
    /// * layer: name or id of the layer.
    /// * numParam: index of the layer parameter in the Layer::blobs array.
    /// * blob: the new value.
    ///  @see Layer::blobs
    ///
    /// Note: If shape of the new blob differs from the previous shape,
    ///  then the following forward pass may fail.
    pub fn set_param(&mut self, layer: &crate::dnn::DictValue, num_param: i32, blob: &core::Mat) -> Result<()> {
        unsafe { sys::cv_dnn_Net_setParam_DictValue_int_Mat(self.as_raw_Net(), layer.as_raw_DictValue(), num_param, blob.as_raw_Mat()) }.into_result()
    }
    
    /// Returns parameter blob of the layer.
    /// ## Parameters
    /// * layer: name or id of the layer.
    /// * numParam: index of the layer parameter in the Layer::blobs array.
    ///  @see Layer::blobs
    ///
    /// ## C++ default parameters
    /// * num_param: 0
    pub fn get_param(&mut self, layer: &crate::dnn::DictValue, num_param: i32) -> Result<core::Mat> {
        unsafe { sys::cv_dnn_Net_getParam_DictValue_int(self.as_raw_Net(), layer.as_raw_DictValue(), num_param) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns indexes of layers with unconnected outputs.
    pub fn get_unconnected_out_layers(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_dnn_Net_getUnconnectedOutLayers_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
    /// Returns names of layers with unconnected outputs.
    pub fn get_unconnected_out_layers_names(&self) -> Result<types::VectorOfString> {
        unsafe { sys::cv_dnn_Net_getUnconnectedOutLayersNames_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfString { ptr })
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
    pub fn get_layers_shapes(&self, net_input_shapes: &types::VectorOfVectorOfint, layers_ids: &mut types::VectorOfint, in_layers_shapes: &mut types::VectorOfVectorOfVectorOfint, out_layers_shapes: &mut types::VectorOfVectorOfVectorOfint) -> Result<()> {
        unsafe { sys::cv_dnn_Net_getLayersShapes_const_VectorOfVectorOfint_VectorOfint_VectorOfVectorOfVectorOfint_VectorOfVectorOfVectorOfint(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfVectorOfint(), layers_ids.as_raw_VectorOfint(), in_layers_shapes.as_raw_VectorOfVectorOfVectorOfint(), out_layers_shapes.as_raw_VectorOfVectorOfVectorOfint()) }.into_result()
    }
    
    pub fn get_layer_shapes(&self, net_input_shapes: &types::VectorOfVectorOfint, layer_id: i32, in_layer_shapes: &mut types::VectorOfVectorOfint, out_layer_shapes: &mut types::VectorOfVectorOfint) -> Result<()> {
        unsafe { sys::cv_dnn_Net_getLayerShapes_const_VectorOfVectorOfint_int_VectorOfVectorOfint_VectorOfVectorOfint(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfVectorOfint(), layer_id, in_layer_shapes.as_raw_VectorOfVectorOfint(), out_layer_shapes.as_raw_VectorOfVectorOfint()) }.into_result()
    }
    
    /// Computes FLOP for whole loaded model with specified input shapes.
    /// ## Parameters
    /// * netInputShapes: vector of shapes for all net inputs.
    /// ## Returns
    /// computed FLOP.
    pub fn get_flops(&self, net_input_shapes: &types::VectorOfVectorOfint) -> Result<i64> {
        unsafe { sys::cv_dnn_Net_getFLOPS_const_VectorOfVectorOfint(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfVectorOfint()) }.into_result()
    }
    
    pub fn get_flops_1(&self, layer_id: i32, net_input_shapes: &types::VectorOfVectorOfint) -> Result<i64> {
        unsafe { sys::cv_dnn_Net_getFLOPS_const_int_VectorOfVectorOfint(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfVectorOfint()) }.into_result()
    }
    
    /// Returns list of types for layer used in model.
    /// ## Parameters
    /// * layersTypes: output parameter for returning types.
    pub fn get_layer_types(&self, layers_types: &mut types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_dnn_Net_getLayerTypes_const_VectorOfString(self.as_raw_Net(), layers_types.as_raw_VectorOfString()) }.into_result()
    }
    
    /// Returns count of layers of specified type.
    /// ## Parameters
    /// * layerType: type.
    /// ## Returns
    /// count of layers
    pub fn get_layers_count(&self, layer_type: &str) -> Result<i32> {
        string_arg!(layer_type);
        unsafe { sys::cv_dnn_Net_getLayersCount_const_String(self.as_raw_Net(), layer_type.as_ptr()) }.into_result()
    }
    
    /// Computes bytes number which are required to store
    /// all weights and intermediate blobs for model.
    /// ## Parameters
    /// * netInputShapes: vector of shapes for all net inputs.
    /// * weights: output parameter to store resulting bytes for weights.
    /// * blobs: output parameter to store resulting bytes for intermediate blobs.
    pub fn get_memory_consumption(&self, net_input_shapes: &types::VectorOfVectorOfint, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
        unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_VectorOfVectorOfint_size_t_size_t(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfVectorOfint(), weights, blobs) }.into_result()
    }
    
    pub fn get_memory_consumption_for_layer(&self, layer_id: i32, net_input_shapes: &types::VectorOfVectorOfint, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
        unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_int_VectorOfVectorOfint_size_t_size_t(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfVectorOfint(), weights, blobs) }.into_result()
    }
    
    /// Computes bytes number which are required to store
    /// all weights and intermediate blobs for each layer.
    /// ## Parameters
    /// * netInputShapes: vector of shapes for all net inputs.
    /// * layerIds: output vector to save layer IDs.
    /// * weights: output parameter to store resulting bytes for weights.
    /// * blobs: output parameter to store resulting bytes for intermediate blobs.
    pub fn get_memory_consumption_for_layers(&self, net_input_shapes: &types::VectorOfVectorOfint, layer_ids: &mut types::VectorOfint, weights: &mut types::VectorOfsize_t, blobs: &mut types::VectorOfsize_t) -> Result<()> {
        unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_VectorOfVectorOfint_VectorOfint_VectorOfsize_t_VectorOfsize_t(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfVectorOfint(), layer_ids.as_raw_VectorOfint(), weights.as_raw_VectorOfsize_t(), blobs.as_raw_VectorOfsize_t()) }.into_result()
    }
    
    /// Enables or disables layer fusion in the network.
    /// ## Parameters
    /// * fusion: true to enable the fusion, false to disable. The fusion is enabled by default.
    pub fn enable_fusion(&mut self, fusion: bool) -> Result<()> {
        unsafe { sys::cv_dnn_Net_enableFusion_bool(self.as_raw_Net(), fusion) }.into_result()
    }
    
    /// Returns overall time for inference and timings (in ticks) for layers.
    /// Indexes in returned vector correspond to layers ids. Some layers can be fused with others,
    /// in this case zero ticks count will be return for that skipped layers.
    /// ## Parameters
    /// * timings: vector for tick timings for all layers.
    /// ## Returns
    /// overall ticks for model inference.
    pub fn get_perf_profile(&mut self, timings: &mut types::VectorOfdouble) -> Result<i64> {
        unsafe { sys::cv_dnn_Net_getPerfProfile_VectorOfdouble(self.as_raw_Net(), timings.as_raw_VectorOfdouble()) }.into_result()
    }
    
}

// boxed class cv::dnn::NormalizeBBoxLayer
/// ![inline formula](https://latex.codecogs.com/png.latex?%20L_p%20) - normalization layer.
/// ## Parameters
/// * p: Normalization factor. The most common `p = 1` for ![inline formula](https://latex.codecogs.com/png.latex?%20L_1%20) -
///          normalization or `p = 2` for ![inline formula](https://latex.codecogs.com/png.latex?%20L_2%20) - normalization or a custom one.
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
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for NormalizeBBoxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_NormalizeBBoxLayer_delete(self.ptr) };
    }
}

impl NormalizeBBoxLayer {
    #[inline(always)] pub fn as_raw_NormalizeBBoxLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for NormalizeBBoxLayer {}

impl core::AlgorithmTrait for NormalizeBBoxLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for NormalizeBBoxLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl NormalizeBBoxLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfNormalizeBBoxLayer> {
        unsafe { sys::cv_dnn_NormalizeBBoxLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfNormalizeBBoxLayer { ptr })
    }
    
}

// boxed class cv::dnn::PaddingLayer
/// Adds extra values for specific axes.
/// ## Parameters
/// * paddings: Vector of paddings in format
///     ```ignore
///     [ pad_before, pad_after,  // [0]th dimension
///                   pad_before, pad_after,  // [1]st dimension
///                   ...
///                   pad_before, pad_after ] // [n]th dimension
///     ```
///
///     that represents number of padded values at every dimension
///     starting from the first one. The rest of dimensions won't
///     be padded.
/// * value: Value to be padded. Defaults to zero.
/// * type: Padding type: 'constant', 'reflect'
/// * input_dims: Torch's parameter. If @p input_dims is not equal to the
///                   actual input dimensionality then the `[0]th` dimension
///                   is considered as a batch dimension and @p paddings are shifted
///                   to a one dimension. Defaults to `-1` that means padding
///                   corresponding to @p paddings.
pub struct PaddingLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PaddingLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PaddingLayer_delete(self.ptr) };
    }
}

impl PaddingLayer {
    #[inline(always)] pub fn as_raw_PaddingLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PaddingLayer {}

impl core::AlgorithmTrait for PaddingLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PaddingLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl PaddingLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPaddingLayer> {
        unsafe { sys::cv_dnn_PaddingLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPaddingLayer { ptr })
    }
    
}

// boxed class cv::dnn::PermuteLayer
pub struct PermuteLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PermuteLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PermuteLayer_delete(self.ptr) };
    }
}

impl PermuteLayer {
    #[inline(always)] pub fn as_raw_PermuteLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PermuteLayer {}

impl core::AlgorithmTrait for PermuteLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PermuteLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl PermuteLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPermuteLayer> {
        unsafe { sys::cv_dnn_PermuteLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPermuteLayer { ptr })
    }
    
}

// boxed class cv::dnn::PoolingLayer
pub struct PoolingLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PoolingLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PoolingLayer_delete(self.ptr) };
    }
}

impl PoolingLayer {
    #[inline(always)] pub fn as_raw_PoolingLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PoolingLayer {}

impl core::AlgorithmTrait for PoolingLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PoolingLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl PoolingLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPoolingLayer> {
        unsafe { sys::cv_dnn_PoolingLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPoolingLayer { ptr })
    }
    
}

// boxed class cv::dnn::PowerLayer
pub struct PowerLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PowerLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PowerLayer_delete(self.ptr) };
    }
}

impl PowerLayer {
    #[inline(always)] pub fn as_raw_PowerLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PowerLayer {}

impl core::AlgorithmTrait for PowerLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for PowerLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PowerLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl PowerLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPowerLayer> {
        unsafe { sys::cv_dnn_PowerLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPowerLayer { ptr })
    }
    
}

// boxed class cv::dnn::PriorBoxLayer
pub struct PriorBoxLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for PriorBoxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_PriorBoxLayer_delete(self.ptr) };
    }
}

impl PriorBoxLayer {
    #[inline(always)] pub fn as_raw_PriorBoxLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for PriorBoxLayer {}

impl core::AlgorithmTrait for PriorBoxLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PriorBoxLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl PriorBoxLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPriorBoxLayer> {
        unsafe { sys::cv_dnn_PriorBoxLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPriorBoxLayer { ptr })
    }
    
}

// boxed class cv::dnn::ProposalLayer
pub struct ProposalLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ProposalLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ProposalLayer_delete(self.ptr) };
    }
}

impl ProposalLayer {
    #[inline(always)] pub fn as_raw_ProposalLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ProposalLayer {}

impl core::AlgorithmTrait for ProposalLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ProposalLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ProposalLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfProposalLayer> {
        unsafe { sys::cv_dnn_ProposalLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfProposalLayer { ptr })
    }
    
}

// Generating impl for trait crate::dnn::RNNLayer
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
pub trait RNNLayer: crate::dnn::LayerTrait {
    fn as_raw_RNNLayer(&self) -> *mut c_void;
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
    fn set_weights(&mut self, wxh: &core::Mat, bh: &core::Mat, whh: &core::Mat, who: &core::Mat, bo: &core::Mat) -> Result<()> {
        unsafe { sys::cv_dnn_RNNLayer_setWeights_Mat_Mat_Mat_Mat_Mat(self.as_raw_RNNLayer(), wxh.as_raw_Mat(), bh.as_raw_Mat(), whh.as_raw_Mat(), who.as_raw_Mat(), bo.as_raw_Mat()) }.into_result()
    }
    
    /// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
    /// @details Shape of the second output is the same as first output.
    ///
    /// ## C++ default parameters
    /// * produce: false
    fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
        unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput_bool(self.as_raw_RNNLayer(), produce) }.into_result()
    }
    
}

impl dyn RNNLayer + '_ {
    /// Creates instance of RNNLayer
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfRNNLayer> {
        unsafe { sys::cv_dnn_RNNLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfRNNLayer { ptr })
    }
    
}

// boxed class cv::dnn::ReLU6Layer
pub struct ReLU6Layer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ReLU6Layer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReLU6Layer_delete(self.ptr) };
    }
}

impl ReLU6Layer {
    #[inline(always)] pub fn as_raw_ReLU6Layer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ReLU6Layer {}

impl core::AlgorithmTrait for ReLU6Layer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for ReLU6Layer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ReLU6Layer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ReLU6Layer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReLU6Layer> {
        unsafe { sys::cv_dnn_ReLU6Layer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReLU6Layer { ptr })
    }
    
}

// boxed class cv::dnn::ReLULayer
pub struct ReLULayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ReLULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReLULayer_delete(self.ptr) };
    }
}

impl ReLULayer {
    #[inline(always)] pub fn as_raw_ReLULayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ReLULayer {}

impl core::AlgorithmTrait for ReLULayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for ReLULayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ReLULayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ReLULayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReLULayer> {
        unsafe { sys::cv_dnn_ReLULayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReLULayer { ptr })
    }
    
}

// boxed class cv::dnn::RegionLayer
pub struct RegionLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for RegionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_RegionLayer_delete(self.ptr) };
    }
}

impl RegionLayer {
    #[inline(always)] pub fn as_raw_RegionLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RegionLayer {}

impl core::AlgorithmTrait for RegionLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for RegionLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl RegionLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfRegionLayer> {
        unsafe { sys::cv_dnn_RegionLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfRegionLayer { ptr })
    }
    
}

// boxed class cv::dnn::ReorgLayer
pub struct ReorgLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ReorgLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReorgLayer_delete(self.ptr) };
    }
}

impl ReorgLayer {
    #[inline(always)] pub fn as_raw_ReorgLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ReorgLayer {}

impl core::AlgorithmTrait for ReorgLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ReorgLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ReorgLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReorgLayer> {
        unsafe { sys::cv_dnn_ReorgLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReorgLayer { ptr })
    }
    
}

// boxed class cv::dnn::ReshapeLayer
pub struct ReshapeLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ReshapeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ReshapeLayer_delete(self.ptr) };
    }
}

impl ReshapeLayer {
    #[inline(always)] pub fn as_raw_ReshapeLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ReshapeLayer {}

impl core::AlgorithmTrait for ReshapeLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ReshapeLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ReshapeLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReshapeLayer> {
        unsafe { sys::cv_dnn_ReshapeLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReshapeLayer { ptr })
    }
    
}

// boxed class cv::dnn::ResizeLayer
/// Resize input 4-dimensional blob by nearest neighbor or bilinear strategy.
///
/// Layer is used to support TensorFlow's resize_nearest_neighbor and resize_bilinear ops.
pub struct ResizeLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ResizeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ResizeLayer_delete(self.ptr) };
    }
}

impl ResizeLayer {
    #[inline(always)] pub fn as_raw_ResizeLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ResizeLayer {}

impl core::AlgorithmTrait for ResizeLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ResizeLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ResizeLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfResizeLayer> {
        unsafe { sys::cv_dnn_ResizeLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfResizeLayer { ptr })
    }
    
}

// boxed class cv::dnn::ScaleLayer
pub struct ScaleLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ScaleLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ScaleLayer_delete(self.ptr) };
    }
}

impl ScaleLayer {
    #[inline(always)] pub fn as_raw_ScaleLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ScaleLayer {}

impl core::AlgorithmTrait for ScaleLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ScaleLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ScaleLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfScaleLayer> {
        unsafe { sys::cv_dnn_ScaleLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfScaleLayer { ptr })
    }
    
}

// boxed class cv::dnn::ShiftLayer
pub struct ShiftLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ShiftLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ShiftLayer_delete(self.ptr) };
    }
}

impl ShiftLayer {
    #[inline(always)] pub fn as_raw_ShiftLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ShiftLayer {}

impl core::AlgorithmTrait for ShiftLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ShiftLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ShiftLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_ShiftLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::ShuffleChannelLayer
/// Permute channels of 4-dimensional input blob.
/// ## Parameters
/// * group: Number of groups to split input channels and pick in turns
///              into output blob.
///
/// ![block formula](https://latex.codecogs.com/png.latex?%20groupSize%20%3D%20%5Cfrac%7Bnumber%5C%20of%5C%20channels%7D%7Bgroup%7D%20)
/// ![block formula](https://latex.codecogs.com/png.latex?%20output%28n%2C%20c%2C%20h%2C%20w%29%20%3D%20input%28n%2C%20groupSize%20%5Ctimes%20%28c%20%5C%25%20group%29%20%2B%20%5Clfloor%20%5Cfrac%7Bc%7D%7Bgroup%7D%20%5Crfloor%2C%20h%2C%20w%29%20)
/// Read more at https://arxiv.org/pdf/1707.01083.pdf
pub struct ShuffleChannelLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ShuffleChannelLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_ShuffleChannelLayer_delete(self.ptr) };
    }
}

impl ShuffleChannelLayer {
    #[inline(always)] pub fn as_raw_ShuffleChannelLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ShuffleChannelLayer {}

impl core::AlgorithmTrait for ShuffleChannelLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ShuffleChannelLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ShuffleChannelLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
        unsafe { sys::cv_dnn_ShuffleChannelLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
    }
    
}

// boxed class cv::dnn::SigmoidLayer
pub struct SigmoidLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SigmoidLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SigmoidLayer_delete(self.ptr) };
    }
}

impl SigmoidLayer {
    #[inline(always)] pub fn as_raw_SigmoidLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SigmoidLayer {}

impl core::AlgorithmTrait for SigmoidLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for SigmoidLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SigmoidLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SigmoidLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSigmoidLayer> {
        unsafe { sys::cv_dnn_SigmoidLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSigmoidLayer { ptr })
    }
    
}

// boxed class cv::dnn::SliceLayer
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
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SliceLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SliceLayer_delete(self.ptr) };
    }
}

impl SliceLayer {
    #[inline(always)] pub fn as_raw_SliceLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SliceLayer {}

impl core::AlgorithmTrait for SliceLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SliceLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SliceLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSliceLayer> {
        unsafe { sys::cv_dnn_SliceLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSliceLayer { ptr })
    }
    
}

// boxed class cv::dnn::SoftmaxLayer
pub struct SoftmaxLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SoftmaxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SoftmaxLayer_delete(self.ptr) };
    }
}

impl SoftmaxLayer {
    #[inline(always)] pub fn as_raw_SoftmaxLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SoftmaxLayer {}

impl core::AlgorithmTrait for SoftmaxLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SoftmaxLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SoftmaxLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSoftmaxLayer> {
        unsafe { sys::cv_dnn_SoftmaxLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSoftmaxLayer { ptr })
    }
    
}

// boxed class cv::dnn::SplitLayer
pub struct SplitLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SplitLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SplitLayer_delete(self.ptr) };
    }
}

impl SplitLayer {
    #[inline(always)] pub fn as_raw_SplitLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SplitLayer {}

impl core::AlgorithmTrait for SplitLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SplitLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SplitLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSplitLayer> {
        unsafe { sys::cv_dnn_SplitLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSplitLayer { ptr })
    }
    
}

// boxed class cv::dnn::SwishLayer
pub struct SwishLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SwishLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_SwishLayer_delete(self.ptr) };
    }
}

impl SwishLayer {
    #[inline(always)] pub fn as_raw_SwishLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SwishLayer {}

impl core::AlgorithmTrait for SwishLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for SwishLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SwishLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl SwishLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSwishLayer> {
        unsafe { sys::cv_dnn_SwishLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSwishLayer { ptr })
    }
    
}

// boxed class cv::dnn::TanHLayer
pub struct TanHLayer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for TanHLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_TanHLayer_delete(self.ptr) };
    }
}

impl TanHLayer {
    #[inline(always)] pub fn as_raw_TanHLayer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for TanHLayer {}

impl core::AlgorithmTrait for TanHLayer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ActivationLayer for TanHLayer {
    #[inline(always)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for TanHLayer {
    #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl TanHLayer {
    pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfTanHLayer> {
        unsafe { sys::cv_dnn_TanHLayer_create_LayerParams(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfTanHLayer { ptr })
    }
    
}

pub use crate::manual::dnn::*;
