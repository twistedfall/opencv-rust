//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>

use libc::{c_void, c_char, size_t};
use std::ffi::{CStr, CString};
use crate::{core, sys, types};
use crate::{Error, Result};
pub const DNN_BACKEND_DEFAULT: i32 = 0;
pub const DNN_BACKEND_HALIDE: i32 = 1;
pub const DNN_BACKEND_INFERENCE_ENGINE: i32 = 2;
pub const DNN_BACKEND_OPENCV: i32 = 3;
pub const DNN_TARGET_CPU: i32 = 0;
pub const DNN_TARGET_FPGA: i32 = 4;
pub const DNN_TARGET_MYRIAD: i32 = 3;
pub const DNN_TARGET_OPENCL: i32 = 1;
pub const DNN_TARGET_OPENCL_FP16: i32 = 2;

/// This class allows to create and manipulate comprehensive artificial neural networks.
/// 
/// Neural network is presented as directed acyclic graph (DAG), where vertices are Layer instances,
/// and edges specify relationships between layers inputs and outputs.
/// 
/// Each network layer has unique integer id and unique string name inside its network.
/// LayerId can store either layer name or layer id.
/// 
/// This class supports reference counting of its instances, i. e. copies point to the same instance.
#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct dnn_Net {
    __rust_private: [u8; 0],
}

///
/// ## C++ default parameters:
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes_rotated_f64(bboxes: &types::VectorOfRect2d, scores: &types::VectorOffloat, score_threshold: f32, nms_threshold: f32, indices: &types::VectorOfint, eta: f32, top_k: i32) -> Result<()> {
// identifier: cv_dnn_NMSBoxes_VectorOfRect2d_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_NMSBoxes_VectorOfRect2d_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOffloat(), score_threshold, nms_threshold, indices.as_raw_VectorOfint(), eta, top_k);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Performs non maximum suppression given boxes and corresponding scores.
/// 
/// ## Parameters
/// * bboxes: a set of bounding boxes to apply NMS.
/// * scores: a set of corresponding confidences.
/// * score_threshold: a threshold used to filter boxes by score.
/// * nms_threshold: a threshold used in non maximum suppression.
/// * indices: the kept indices of bboxes after NMS.
/// * eta: a coefficient in adaptive threshold formula: <span lang='latex'>nms\_threshold_{i+1}=eta\cdot nms\_threshold_i</span>.
/// * top_k: if `>0`, keep at most @p top_k picked indices.
///
/// ## C++ default parameters:
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes(bboxes: &types::VectorOfRect, scores: &types::VectorOffloat, score_threshold: f32, nms_threshold: f32, indices: &types::VectorOfint, eta: f32, top_k: i32) -> Result<()> {
// identifier: cv_dnn_NMSBoxes_VectorOfRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_NMSBoxes_VectorOfRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOffloat(), score_threshold, nms_threshold, indices.as_raw_VectorOfint(), eta, top_k);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

///
/// ## C++ default parameters:
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes_rotated(bboxes: &types::VectorOfRotatedRect, scores: &types::VectorOffloat, score_threshold: f32, nms_threshold: f32, indices: &types::VectorOfint, eta: f32, top_k: i32) -> Result<()> {
// identifier: cv_dnn_NMSBoxes_VectorOfRotatedRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_NMSBoxes_VectorOfRotatedRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k(bboxes.as_raw_VectorOfRotatedRect(), scores.as_raw_VectorOffloat(), score_threshold, nms_threshold, indices.as_raw_VectorOfint(), eta, top_k);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Creates 4-dimensional blob from image.
///  @details This is an overloaded member function, provided for convenience.
///           It differs from the above function only in what argument(s) it accepts.
///
/// ## C++ default parameters:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_image(image: &core::Mat, blob: &mut core::Mat, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
// identifier: cv_dnn_blobFromImage_Mat_image_Mat_blob_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_blobFromImage_Mat_image_Mat_blob_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(image.as_raw_Mat(), blob.as_raw_Mat(), scalefactor, size, mean, swap_rb, crop, ddepth);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
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
///  @returns 4-dimensional Mat with NCHW dimensions order.
///
/// ## C++ default parameters:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_image_v0(image: &core::Mat, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
// identifier: cv_dnn_blobFromImage_Mat_image_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_blobFromImage_Mat_image_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(image.as_raw_Mat(), scalefactor, size, mean, swap_rb, crop, ddepth);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Creates 4-dimensional blob from series of images.
///  @details This is an overloaded member function, provided for convenience.
///           It differs from the above function only in what argument(s) it accepts.
///
/// ## C++ default parameters:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_images(images: &types::VectorOfMat, blob: &mut core::Mat, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
// identifier: cv_dnn_blobFromImages_VectorOfMat_images_Mat_blob_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_blobFromImages_VectorOfMat_images_Mat_blob_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(images.as_raw_VectorOfMat(), blob.as_raw_Mat(), scalefactor, size, mean, swap_rb, crop, ddepth);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
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
///  @returns 4-dimensional Mat with NCHW dimensions order.
///
/// ## C++ default parameters:
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_images_v0(images: &types::VectorOfMat, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
// identifier: cv_dnn_blobFromImages_VectorOfMat_images_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_blobFromImages_VectorOfMat_images_double_scalefactor_Size_size_Scalar_mean_bool_swapRB_bool_crop_int_ddepth(images.as_raw_VectorOfMat(), scalefactor, size, mean, swap_rb, crop, ddepth);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

pub fn clamp(r: &core::Range, axis_size: i32) -> Result<core::Range> {
// identifier: cv_dnn_clamp_Range_r_int_axisSize
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_clamp_Range_r_int_axisSize(r.as_raw_Range(), axis_size);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Range { ptr: rv.result })
    }
  }
}

pub fn clamp_v0(ax: i32, dims: i32) -> Result<i32> {
// identifier: cv_dnn_clamp_int_ax_int_dims
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_clamp_int_ax_int_dims(ax, dims);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

pub fn get_plane(m: &core::Mat, n: i32, cn: i32) -> Result<core::Mat> {
// identifier: cv_dnn_getPlane_Mat_m_int_n_int_cn
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_getPlane_Mat_m_int_n_int_cn(m.as_raw_Mat(), n, cn);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Parse a 4D blob and output the images it contains as 2D arrays through a simpler data structure
///  (std::vector<cv::Mat>).
/// ## Parameters
///  @param[in] blob_ 4 dimensional array (images, channels, height, width) in floating point precision (CV_32F) from
///  which you would like to extract the images.
///  @param[out] images_ array of 2D Mat containing the images extracted from the blob in floating point precision
///  (CV_32F). They are non normalized neither mean added. The number of returned images equals the first dimension
///  of the blob (batch size). Every image has a number of channels equals to the second dimension of the blob (depth).
pub fn images_from_blob(blob_: &core::Mat, images_: &mut types::VectorOfMat) -> Result<()> {
// identifier: cv_dnn_imagesFromBlob_Mat_blob__VectorOfMat_images_
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_imagesFromBlob_Mat_blob__VectorOfMat_images_(blob_.as_raw_Mat(), images_.as_raw_VectorOfMat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

/// Reads a network model stored in <a href="http://caffe.berkeleyvision.org">Caffe</a> framework's format.
/// ## Parameters
/// * prototxt:   path to the .prototxt file with text description of the network architecture.
/// * caffeModel: path to the .caffemodel file with learned network.
/// @returns Net object.
///
/// ## C++ default parameters:
/// * caffe_model: String()
pub fn read_net_from_caffe(prototxt:&str, caffe_model:&str) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromCaffe_String_prototxt_String_caffeModel
  unsafe {
    let prototxt = CString::new(prototxt).unwrap();
    let caffe_model = CString::new(caffe_model).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNetFromCaffe_String_prototxt_String_caffeModel(prototxt.as_ptr() as _, caffe_model.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Reads a network model stored in Caffe model in memory.
/// ## Parameters
/// * bufferProto: buffer containing the content of the .prototxt file
/// * bufferModel: buffer containing the content of the .caffemodel file
/// @returns Net object.
///
/// ## C++ default parameters:
/// * buffer_model: std::vector<uchar>()
pub fn read_net_from_caffe_v0(buffer_proto: &types::VectorOfuchar, buffer_model: &types::VectorOfuchar) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromCaffe_VectorOfuchar_bufferProto_VectorOfuchar_bufferModel
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_readNetFromCaffe_VectorOfuchar_bufferProto_VectorOfuchar_bufferModel(buffer_proto.as_raw_VectorOfuchar(), buffer_model.as_raw_VectorOfuchar());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * cfgFile:      path to the .cfg file with text description of the network architecture.
/// * darknetModel: path to the .weights file with learned network.
///  @returns Network object that ready to do forward, throw an exception in failure cases.
///  @returns Net object.
///
/// ## C++ default parameters:
/// * darknet_model: String()
pub fn read_net_from_darknet(cfg_file:&str, darknet_model:&str) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromDarknet_String_cfgFile_String_darknetModel
  unsafe {
    let cfg_file = CString::new(cfg_file).unwrap();
    let darknet_model = CString::new(darknet_model).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNetFromDarknet_String_cfgFile_String_darknetModel(cfg_file.as_ptr() as _, darknet_model.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * bufferCfg:   A buffer contains a content of .cfg file with text description of the network architecture.
/// * bufferModel: A buffer contains a content of .weights file with learned network.
///  @returns Net object.
///
/// ## C++ default parameters:
/// * buffer_model: std::vector<uchar>()
pub fn read_net_from_darknet_v0(buffer_cfg: &types::VectorOfuchar, buffer_model: &types::VectorOfuchar) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromDarknet_VectorOfuchar_bufferCfg_VectorOfuchar_bufferModel
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_readNetFromDarknet_VectorOfuchar_bufferCfg_VectorOfuchar_bufferModel(buffer_cfg.as_raw_VectorOfuchar(), buffer_model.as_raw_VectorOfuchar());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
///  @param[in] xml XML configuration file with network's topology.
///  @param[in] bin Binary file with trained weights.
///  @returns Net object.
///  Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
///  backend.
pub fn read_net_from_model_optimizer(xml:&str, bin:&str) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromModelOptimizer_String_xml_String_bin
  unsafe {
    let xml = CString::new(xml).unwrap();
    let bin = CString::new(bin).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNetFromModelOptimizer_String_xml_String_bin(xml.as_ptr() as _, bin.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Reads a network model <a href="https://onnx.ai/">ONNX</a>.
/// ## Parameters
/// * onnxFile: path to the .onnx file with text description of the network architecture.
///  @returns Network object that ready to do forward, throw an exception in failure cases.
pub fn read_net_from_onnx(onnx_file:&str) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromONNX_String_onnxFile
  unsafe {
    let onnx_file = CString::new(onnx_file).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNetFromONNX_String_onnxFile(onnx_file.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * model:  path to the .pb file with binary protobuf description of the network architecture
/// * config: path to the .pbtxt file that contains text graph definition in protobuf format.
///               Resulting Net object is built by text graph using weights from a binary one that
///               let us make it more flexible.
/// @returns Net object.
///
/// ## C++ default parameters:
/// * config: String()
pub fn read_net_from_tensorflow(model:&str, config:&str) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromTensorflow_String_model_String_config
  unsafe {
    let model = CString::new(model).unwrap();
    let config = CString::new(config).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNetFromTensorflow_String_model_String_config(model.as_ptr() as _, config.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * bufferModel: buffer containing the content of the pb file
/// * bufferConfig: buffer containing the content of the pbtxt file
/// @returns Net object.
///
/// ## C++ default parameters:
/// * buffer_config: std::vector<uchar>()
pub fn read_net_from_tensorflow_v0(buffer_model: &types::VectorOfuchar, buffer_config: &types::VectorOfuchar) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromTensorflow_VectorOfuchar_bufferModel_VectorOfuchar_bufferConfig
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_readNetFromTensorflow_VectorOfuchar_bufferModel_VectorOfuchar_bufferConfig(buffer_model.as_raw_VectorOfuchar(), buffer_config.as_raw_VectorOfuchar());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Reads a network model stored in <a href="http://torch.ch">Torch7</a> framework's format.
/// ## Parameters
/// * model:    path to the file, dumped from Torch by using torch.save() function.
/// * isBinary: specifies whether the network was serialized in ascii mode or binary.
/// * evaluate: specifies testing phase of network. If true, it's similar to evaluate() method in Torch.
///  @returns Net object.
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
/// ## C++ default parameters:
/// * is_binary: true
/// * evaluate: true
pub fn read_net_from_torch(model:&str, is_binary: bool, evaluate: bool) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNetFromTorch_String_model_bool_isBinary_bool_evaluate
  unsafe {
    let model = CString::new(model).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNetFromTorch_String_model_bool_isBinary_bool_evaluate(model.as_ptr() as _, is_binary, evaluate);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Read deep learning network represented in one of the supported formats.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// @param[in] framework    Name of origin framework.
/// @param[in] bufferModel  A buffer with a content of binary file with weights
/// @param[in] bufferConfig A buffer with a content of text file contains network configuration.
/// @returns Net object.
///
/// ## C++ default parameters:
/// * buffer_config: std::vector<uchar>()
pub fn read_net(framework:&str, buffer_model: &types::VectorOfuchar, buffer_config: &types::VectorOfuchar) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNet_String_framework_VectorOfuchar_bufferModel_VectorOfuchar_bufferConfig
  unsafe {
    let framework = CString::new(framework).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNet_String_framework_VectorOfuchar_bufferModel_VectorOfuchar_bufferConfig(framework.as_ptr() as _, buffer_model.as_raw_VectorOfuchar(), buffer_config.as_raw_VectorOfuchar());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Read deep learning network represented in one of the supported formats.
/// ## Parameters
/// @param[in] model Binary file contains trained weights. The following file
///                  extensions are expected for models from different frameworks:
///                  * `*.caffemodel` (Caffe, http://caffe.berkeleyvision.org/)
///                  * `*.pb` (TensorFlow, https://www.tensorflow.org/)
///                  * `*.t7` | `*.net` (Torch, http://torch.ch/)
///                  * `*.weights` (Darknet, https://pjreddie.com/darknet/)
///                  * `*.bin` (DLDT, https://software.intel.com/openvino-toolkit)
/// @param[in] config Text file contains network configuration. It could be a
///                   file with the following extensions:
///                  * `*.prototxt` (Caffe, http://caffe.berkeleyvision.org/)
///                  * `*.pbtxt` (TensorFlow, https://www.tensorflow.org/)
///                  * `*.cfg` (Darknet, https://pjreddie.com/darknet/)
///                  * `*.xml` (DLDT, https://software.intel.com/openvino-toolkit)
/// @param[in] framework Explicit framework name tag to determine a format.
/// @returns Net object.
/// 
/// This function automatically detects an origin framework of trained model
/// and calls an appropriate function such @ref readNetFromCaffe, @ref readNetFromTensorflow,
/// @ref readNetFromTorch or @ref readNetFromDarknet. An order of @p model and @p config
/// arguments does not matter.
///
/// ## C++ default parameters:
/// * config: ""
/// * framework: ""
pub fn read_net_v0(model:&str, config:&str, framework:&str) -> Result<super::dnn::dnn_Net> {
// identifier: cv_dnn_readNet_String_model_String_config_String_framework
  unsafe {
    let model = CString::new(model).unwrap();
    let config = CString::new(config).unwrap();
    let framework = CString::new(framework).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readNet_String_model_String_config_String_framework(model.as_ptr() as _, config.as_ptr() as _, framework.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Creates blob from .pb file.
/// ## Parameters
/// * path: to the .pb file with input tensor.
///  @returns Mat.
pub fn read_tensor_from_onnx(path:&str) -> Result<core::Mat> {
// identifier: cv_dnn_readTensorFromONNX_String_path
  unsafe {
    let path = CString::new(path).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readTensorFromONNX_String_path(path.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
///  @warning This function has the same limitations as readNetFromTorch().
///
/// ## C++ default parameters:
/// * is_binary: true
pub fn read_torch_blob(filename:&str, is_binary: bool) -> Result<core::Mat> {
// identifier: cv_dnn_readTorchBlob_String_filename_bool_isBinary
  unsafe {
    let filename = CString::new(filename).unwrap();
    let rv = sys::cv_dnn_cv_dnn_readTorchBlob_String_filename_bool_isBinary(filename.as_ptr() as _, is_binary);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Release a Myriad device is binded by OpenCV.
/// 
/// Single Myriad device cannot be shared across multiple processes which uses
/// Inference Engine's Myriad plugin.
pub fn reset_myriad_device() -> Result<()> {
// identifier: cv_dnn_resetMyriadDevice
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_resetMyriadDevice();
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
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
/// ## C++ default parameters:
/// * layers_types: std::vector<String>()
pub fn shrink_caffe_model(src:&str, dst:&str, layers_types: &types::VectorOfString) -> Result<()> {
// identifier: cv_dnn_shrinkCaffeModel_String_src_String_dst_VectorOfString_layersTypes
  unsafe {
    let src = CString::new(src).unwrap();
    let dst = CString::new(dst).unwrap();
    let rv = sys::cv_dnn_cv_dnn_shrinkCaffeModel_String_src_String_dst_VectorOfString_layersTypes(src.as_ptr() as _, dst.as_ptr() as _, layers_types.as_raw_VectorOfString());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

pub fn slice(m: &core::Mat, r0: &core::Range) -> Result<core::Mat> {
// identifier: cv_dnn_slice_Mat_m_Range_r0
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_slice_Mat_m_Range_r0(m.as_raw_Mat(), r0.as_raw_Range());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

pub fn slice_v0(m: &core::Mat, r0: &core::Range, r1: &core::Range) -> Result<core::Mat> {
// identifier: cv_dnn_slice_Mat_m_Range_r0_Range_r1
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_slice_Mat_m_Range_r0_Range_r1(m.as_raw_Mat(), r0.as_raw_Range(), r1.as_raw_Range());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

pub fn slice_v1(m: &core::Mat, r0: &core::Range, r1: &core::Range, r2: &core::Range) -> Result<core::Mat> {
// identifier: cv_dnn_slice_Mat_m_Range_r0_Range_r1_Range_r2
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_slice_Mat_m_Range_r0_Range_r1_Range_r2(m.as_raw_Mat(), r0.as_raw_Range(), r1.as_raw_Range(), r2.as_raw_Range());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

pub fn slice_v2(m: &core::Mat, r0: &core::Range, r1: &core::Range, r2: &core::Range, r3: &core::Range) -> Result<core::Mat> {
// identifier: cv_dnn_slice_Mat_m_Range_r0_Range_r1_Range_r2_Range_r3
  unsafe {
    let rv = sys::cv_dnn_cv_dnn_slice_Mat_m_Range_r0_Range_r1_Range_r2_Range_r3(m.as_raw_Mat(), r0.as_raw_Range(), r1.as_raw_Range(), r2.as_raw_Range(), r3.as_raw_Range());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(core::Mat { ptr: rv.result })
    }
  }
}

/// Create a text representation for a binary network stored in protocol buffer format.
/// ## Parameters
///  @param[in] model  A path to binary network.
///  @param[in] output A path to output text file to be created.
/// 
///  
/// Note: To reduce output file size, trained weights are not included.
pub fn write_text_graph(model:&str, output:&str) -> Result<()> {
// identifier: cv_dnn_writeTextGraph_String_model_String_output
  unsafe {
    let model = CString::new(model).unwrap();
    let output = CString::new(output).unwrap();
    let rv = sys::cv_dnn_cv_dnn_writeTextGraph_String_model_String_output(model.as_ptr() as _, output.as_ptr() as _);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(())
    }
  }
}

// boxed class cv::dnn::AbsLayer

#[allow(dead_code)]
pub struct AbsLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::AbsLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_AbsLayer(self.ptr) };
    }
}
impl super::dnn::AbsLayer {
    #[doc(hidden)] pub fn as_raw_AbsLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for AbsLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for AbsLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for AbsLayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl AbsLayer {

}
// Generating impl for trait cv::dnn::ActivationLayer (trait)
pub trait ActivationLayer : super::dnn::Layer {
  #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void;
}
impl<'a> ActivationLayer + 'a {

}

// boxed class cv::dnn::BNLLLayer

#[allow(dead_code)]
pub struct BNLLLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::BNLLLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_BNLLLayer(self.ptr) };
    }
}
impl super::dnn::BNLLLayer {
    #[doc(hidden)] pub fn as_raw_BNLLLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for BNLLLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for BNLLLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for BNLLLayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl BNLLLayer {

}
// boxed class cv::dnn::BackendNode
/// Derivatives of this class encapsulates functions of certain backends.

#[allow(dead_code)]
pub struct BackendNode {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::BackendNode {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_BackendNode(self.ptr) };
    }
}
impl super::dnn::BackendNode {
    #[doc(hidden)] pub fn as_raw_BackendNode(&self) -> *mut c_void { self.ptr }
}
impl BackendNode {

  pub fn new(backend_id: i32) -> Result<super::dnn::BackendNode> {
  // identifier: cv_dnn_BackendNode_BackendNode_int_backendId
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_BackendNode_BackendNode_int_backendId(backend_id);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::dnn::BackendNode { ptr: rv.result })
      }
    }
  }

}
// Generating impl for trait cv::dnn::BackendWrapper (trait)
/// Derivatives of this class wraps cv::Mat for different backends and targets.
pub trait BackendWrapper {
  #[doc(hidden)] fn as_raw_BackendWrapper(&self) -> *mut c_void;
  /// Transfer data to CPU host memory.
  fn copy_to_host(&mut self) -> Result<()> {
  // identifier: cv_dnn_BackendWrapper_copyToHost
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_BackendWrapper_copyToHost(self.as_raw_BackendWrapper());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Indicate that an actual data is on CPU.
  fn set_host_dirty(&mut self) -> Result<()> {
  // identifier: cv_dnn_BackendWrapper_setHostDirty
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_BackendWrapper_setHostDirty(self.as_raw_BackendWrapper());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> BackendWrapper + 'a {

}

// Generating impl for trait cv::dnn::BaseConvolutionLayer (trait)
pub trait BaseConvolutionLayer : super::dnn::Layer {
  #[doc(hidden)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void;
}
impl<'a> BaseConvolutionLayer + 'a {

}

// boxed class cv::dnn::BatchNormLayer

#[allow(dead_code)]
pub struct BatchNormLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::BatchNormLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_BatchNormLayer(self.ptr) };
    }
}
impl super::dnn::BatchNormLayer {
    #[doc(hidden)] pub fn as_raw_BatchNormLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for BatchNormLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for BatchNormLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for BatchNormLayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl BatchNormLayer {

}
// boxed class cv::dnn::BlankLayer
/// Partial List of Implemented Layers
/// 
/// # Partial List of Implemented Layers
/// @{
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

#[allow(dead_code)]
pub struct BlankLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::BlankLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_BlankLayer(self.ptr) };
    }
}
impl super::dnn::BlankLayer {
    #[doc(hidden)] pub fn as_raw_BlankLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for BlankLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for BlankLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl BlankLayer {

}
// boxed class cv::dnn::ChannelsPReLULayer

#[allow(dead_code)]
pub struct ChannelsPReLULayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ChannelsPReLULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ChannelsPReLULayer(self.ptr) };
    }
}
impl super::dnn::ChannelsPReLULayer {
    #[doc(hidden)] pub fn as_raw_ChannelsPReLULayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ChannelsPReLULayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ChannelsPReLULayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for ChannelsPReLULayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl ChannelsPReLULayer {

}
// boxed class cv::dnn::ConcatLayer

#[allow(dead_code)]
pub struct ConcatLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ConcatLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ConcatLayer(self.ptr) };
    }
}
impl super::dnn::ConcatLayer {
    #[doc(hidden)] pub fn as_raw_ConcatLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ConcatLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ConcatLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ConcatLayer {

}
// boxed class cv::dnn::ConstLayer
/// Constant layer produces the same data blob at an every forward pass.

#[allow(dead_code)]
pub struct ConstLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ConstLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ConstLayer(self.ptr) };
    }
}
impl super::dnn::ConstLayer {
    #[doc(hidden)] pub fn as_raw_ConstLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ConstLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ConstLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ConstLayer {

}
// boxed class cv::dnn::ConvolutionLayer

#[allow(dead_code)]
pub struct ConvolutionLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ConvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ConvolutionLayer(self.ptr) };
    }
}
impl super::dnn::ConvolutionLayer {
    #[doc(hidden)] pub fn as_raw_ConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::BaseConvolutionLayer for ConvolutionLayer {
    #[doc(hidden)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ConvolutionLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ConvolutionLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}
impl ConvolutionLayer {

}
// boxed class cv::dnn::CropAndResizeLayer

#[allow(dead_code)]
pub struct CropAndResizeLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::CropAndResizeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_CropAndResizeLayer(self.ptr) };
    }
}
impl super::dnn::CropAndResizeLayer {
    #[doc(hidden)] pub fn as_raw_CropAndResizeLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for CropAndResizeLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for CropAndResizeLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl CropAndResizeLayer {

}
// boxed class cv::dnn::CropLayer

#[allow(dead_code)]
pub struct CropLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::CropLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_CropLayer(self.ptr) };
    }
}
impl super::dnn::CropLayer {
    #[doc(hidden)] pub fn as_raw_CropLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for CropLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for CropLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl CropLayer {

}
// boxed class cv::dnn::DeconvolutionLayer

#[allow(dead_code)]
pub struct DeconvolutionLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::DeconvolutionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DeconvolutionLayer(self.ptr) };
    }
}
impl super::dnn::DeconvolutionLayer {
    #[doc(hidden)] pub fn as_raw_DeconvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::BaseConvolutionLayer for DeconvolutionLayer {
    #[doc(hidden)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for DeconvolutionLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for DeconvolutionLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}
impl DeconvolutionLayer {

}
// boxed class cv::dnn::DetectionOutputLayer

#[allow(dead_code)]
pub struct DetectionOutputLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::DetectionOutputLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DetectionOutputLayer(self.ptr) };
    }
}
impl super::dnn::DetectionOutputLayer {
    #[doc(hidden)] pub fn as_raw_DetectionOutputLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for DetectionOutputLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for DetectionOutputLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl DetectionOutputLayer {

}
// Generating impl for trait cv::dnn::Dict (trait)
/// This class implements name-value dictionary, values are instances of DictValue.
pub trait Dict {
  #[doc(hidden)] fn as_raw_Dict(&self) -> *mut c_void;
  fn has(&self, key:&str) -> Result<bool> {
  // identifier: cv_dnn_Dict_has_String_key
    unsafe {
      let key = CString::new(key).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Dict_has_String_key(self.as_raw_Dict(), key.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn ptr(&mut self, key:&str) -> Result<super::dnn::DictValue> {
  // identifier: cv_dnn_Dict_ptr_String_key
    unsafe {
      let key = CString::new(key).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Dict_ptr_String_key(self.as_raw_Dict(), key.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::dnn::DictValue { ptr: rv.result })
      }
    }
  }

  fn get(&self, key:&str) -> Result<super::dnn::DictValue> {
  // identifier: cv_dnn_Dict_get_String_key
    unsafe {
      let key = CString::new(key).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Dict_get_String_key(self.as_raw_Dict(), key.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::dnn::DictValue { ptr: rv.result })
      }
    }
  }

  /// @overload
  fn erase(&mut self, key:&str) -> Result<()> {
  // identifier: cv_dnn_Dict_erase_String_key
    unsafe {
      let key = CString::new(key).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Dict_erase_String_key(self.as_raw_Dict(), key.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> Dict + 'a {

}

// boxed class cv::dnn::DictValue
/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
///  @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.

#[allow(dead_code)]
pub struct DictValue {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::DictValue {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DictValue(self.ptr) };
    }
}
impl super::dnn::DictValue {
    #[doc(hidden)] pub fn as_raw_DictValue(&self) -> *mut c_void { self.ptr }
}
impl DictValue {

  pub fn new(r: &super::dnn::DictValue) -> Result<super::dnn::DictValue> {
  // identifier: cv_dnn_DictValue_DictValue_DictValue_r
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_DictValue_DictValue_r(r.as_raw_DictValue());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(super::dnn::DictValue { ptr: rv.result })
      }
    }
  }

  pub fn size(&self) -> Result<i32> {
  // identifier: cv_dnn_DictValue_size
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_size(self.as_raw_DictValue());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn is_int(&self) -> Result<bool> {
  // identifier: cv_dnn_DictValue_isInt
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_isInt(self.as_raw_DictValue());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn is_string(&self) -> Result<bool> {
  // identifier: cv_dnn_DictValue_isString
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_isString(self.as_raw_DictValue());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn is_real(&self) -> Result<bool> {
  // identifier: cv_dnn_DictValue_isReal
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_isReal(self.as_raw_DictValue());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  ///
  /// ## C++ default parameters:
  /// * idx: -1
  pub fn get_int_value(&self, idx: i32) -> Result<i32> {
  // identifier: cv_dnn_DictValue_getIntValue_int_idx
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_getIntValue_int_idx(self.as_raw_DictValue(), idx);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  ///
  /// ## C++ default parameters:
  /// * idx: -1
  pub fn get_real_value(&self, idx: i32) -> Result<f64> {
  // identifier: cv_dnn_DictValue_getRealValue_int_idx
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_getRealValue_int_idx(self.as_raw_DictValue(), idx);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  ///
  /// ## C++ default parameters:
  /// * idx: -1
  pub fn get_string_value(&self, idx: i32) -> Result<String> {
  // identifier: cv_dnn_DictValue_getStringValue_int_idx
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_DictValue_getStringValue_int_idx(self.as_raw_DictValue(), idx);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        let v = CStr::from_ptr(rv.result as _).to_bytes().to_vec();
        ::libc::free(rv.result as _);
        Ok(String::from_utf8(v).unwrap())
      }
    }
  }

}
// boxed class cv::dnn::ELULayer

#[allow(dead_code)]
pub struct ELULayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ELULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ELULayer(self.ptr) };
    }
}
impl super::dnn::ELULayer {
    #[doc(hidden)] pub fn as_raw_ELULayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ELULayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ELULayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for ELULayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl ELULayer {

}
// boxed class cv::dnn::EltwiseLayer

#[allow(dead_code)]
pub struct EltwiseLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::EltwiseLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_EltwiseLayer(self.ptr) };
    }
}
impl super::dnn::EltwiseLayer {
    #[doc(hidden)] pub fn as_raw_EltwiseLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for EltwiseLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for EltwiseLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl EltwiseLayer {

}
// boxed class cv::dnn::FlattenLayer

#[allow(dead_code)]
pub struct FlattenLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::FlattenLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_FlattenLayer(self.ptr) };
    }
}
impl super::dnn::FlattenLayer {
    #[doc(hidden)] pub fn as_raw_FlattenLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for FlattenLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for FlattenLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl FlattenLayer {

}
// boxed class cv::dnn::InnerProductLayer

#[allow(dead_code)]
pub struct InnerProductLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::InnerProductLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_InnerProductLayer(self.ptr) };
    }
}
impl super::dnn::InnerProductLayer {
    #[doc(hidden)] pub fn as_raw_InnerProductLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for InnerProductLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for InnerProductLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl InnerProductLayer {

}
// boxed class cv::dnn::InterpLayer
/// Bilinear resize layer from https://github.com/cdmh/deeplab-public
/// 
/// It differs from @ref ResizeLayer in output shape and resize scales computations.

#[allow(dead_code)]
pub struct InterpLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::InterpLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_InterpLayer(self.ptr) };
    }
}
impl super::dnn::InterpLayer {
    #[doc(hidden)] pub fn as_raw_InterpLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for InterpLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for InterpLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl InterpLayer {

}
// boxed class cv::dnn::LRNLayer

#[allow(dead_code)]
pub struct LRNLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::LRNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_LRNLayer(self.ptr) };
    }
}
impl super::dnn::LRNLayer {
    #[doc(hidden)] pub fn as_raw_LRNLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for LRNLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for LRNLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl LRNLayer {

}
// Generating impl for trait cv::dnn::LSTMLayer (trait)
pub trait LSTMLayer : super::dnn::Layer {
  #[doc(hidden)] fn as_raw_LSTMLayer(&self) -> *mut c_void;
  /// @deprecated Use LayerParams::blobs instead.
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
  /// where @f$\odot@f$ is per-element multiply operation and @f$i_t, f_t, o_t, g_t@f$ is internal gates that are computed using learned wights.
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
  /// * b:  is bias vector (i.e. according to above mentioned notation is @f$ b @f$)
  fn set_weights(&mut self, wh: &core::Mat, wx: &core::Mat, b: &core::Mat) -> Result<()> {
  // identifier: cv_dnn_LSTMLayer_setWeights_Mat_Wh_Mat_Wx_Mat_b
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_LSTMLayer_setWeights_Mat_Wh_Mat_Wx_Mat_b(self.as_raw_LSTMLayer(), wh.as_raw_Mat(), wx.as_raw_Mat(), b.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @deprecated Use flag `produce_cell_output` in LayerParams.
  ///  Specifies either interpret first dimension of input blob as timestamp dimenion either as sample.
  /// 
  /// If flag is set to true then shape of input blob will be interpreted as [`T`, `N`, `[data dims]`] where `T` specifies number of timestamps, `N` is number of independent streams.
  /// In this case each forward() call will iterate through `T` timestamps and update layer's state `T` times.
  /// 
  /// If flag is set to false then shape of input blob will be interpreted as [`N`, `[data dims]`].
  /// In this case each forward() call will make one iteration and produce one timestamp with shape [`N`, `[out dims]`].
  ///
  /// ## C++ default parameters:
  /// * _use: true
  fn set_use_timstamps_dim(&mut self, _use: bool) -> Result<()> {
  // identifier: cv_dnn_LSTMLayer_setUseTimstampsDim_bool_use
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_LSTMLayer_setUseTimstampsDim_bool_use(self.as_raw_LSTMLayer(), _use);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @deprecated Use flag `use_timestamp_dim` in LayerParams.
  ///  If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
  /// @details Shape of the second output is the same as first output.
  ///
  /// ## C++ default parameters:
  /// * produce: false
  fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
  // identifier: cv_dnn_LSTMLayer_setProduceCellOutput_bool_produce
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_LSTMLayer_setProduceCellOutput_bool_produce(self.as_raw_LSTMLayer(), produce);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn input_name_to_index(&mut self, input_name:&str) -> Result<i32> {
  // identifier: cv_dnn_LSTMLayer_inputNameToIndex_String_inputName
    unsafe {
      let input_name = CString::new(input_name).unwrap();
      let rv = sys::cv_dnn_cv_dnn_LSTMLayer_inputNameToIndex_String_inputName(self.as_raw_LSTMLayer(), input_name.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn output_name_to_index(&mut self, output_name:&str) -> Result<i32> {
  // identifier: cv_dnn_LSTMLayer_outputNameToIndex_String_outputName
    unsafe {
      let output_name = CString::new(output_name).unwrap();
      let rv = sys::cv_dnn_cv_dnn_LSTMLayer_outputNameToIndex_String_outputName(self.as_raw_LSTMLayer(), output_name.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> LSTMLayer + 'a {

}

// Generating impl for trait cv::dnn::Layer (trait)
/// This interface class allows to build new Layers - are building blocks of networks.
/// 
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub trait Layer : core::Algorithm {
  #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void;
  /// Computes and sets internal parameters according to inputs, outputs and blobs.
  /// ## Parameters
  ///  @param[in]  inputs  vector of already allocated input blobs
  ///  @param[out] outputs vector of already allocated output blobs
  /// 
  /// If this method is called after network has allocated all memory for input and output blobs
  /// and before inferencing.
  fn finalize(&mut self, inputs: &types::VectorOfMat, outputs: &mut types::VectorOfMat) -> Result<()> {
  // identifier: cv_dnn_Layer_finalize_VectorOfMat_inputs_VectorOfMat_outputs
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_finalize_VectorOfMat_inputs_VectorOfMat_outputs(self.as_raw_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Given the @p input blobs, computes the output @p blobs.
  /// ## Parameters
  ///  @param[in]  inputs  the input blobs.
  ///  @param[out] outputs allocated output blobs, which will store results of the computation.
  ///  @param[out] internals allocated internal blobs
  fn forward(&mut self, inputs: &types::VectorOfMat, outputs: &mut types::VectorOfMat, internals: &mut types::VectorOfMat) -> Result<()> {
  // identifier: cv_dnn_Layer_forward_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_forward_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals(self.as_raw_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), internals.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Given the @p input blobs, computes the output @p blobs.
  /// ## Parameters
  ///  @param[in]  inputs  the input blobs.
  ///  @param[out] outputs allocated output blobs, which will store results of the computation.
  ///  @param[out] internals allocated internal blobs
  fn forward_fallback(&mut self, inputs: &types::VectorOfMat, outputs: &mut types::VectorOfMat, internals: &mut types::VectorOfMat) -> Result<()> {
  // identifier: cv_dnn_Layer_forward_fallback_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_forward_fallback_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals(self.as_raw_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), internals.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// @overload
  /// @deprecated Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead
  fn finalize_v0(&mut self, inputs: &types::VectorOfMat) -> Result<types::VectorOfMat> {
  // identifier: cv_dnn_Layer_finalize_VectorOfMat_inputs
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_finalize_VectorOfMat_inputs(self.as_raw_Layer(), inputs.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfMat { ptr: rv.result })
      }
    }
  }

  /// Allocates layer and computes output.
  ///  @deprecated This method will be removed in the future release.
  fn run(&mut self, inputs: &types::VectorOfMat, outputs: &types::VectorOfMat, internals: &types::VectorOfMat) -> Result<()> {
  // identifier: cv_dnn_Layer_run_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_run_VectorOfMat_inputs_VectorOfMat_outputs_VectorOfMat_internals(self.as_raw_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), internals.as_raw_VectorOfMat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns index of input blob into the input array.
  /// ## Parameters
  /// * inputName: label of input blob
  /// 
  /// Each layer input and output can be labeled to easily identify them using "%<layer_name%>[.output_name]" notation.
  /// This method maps label of input blob to its index into input vector.
  fn input_name_to_index(&mut self, input_name:&str) -> Result<i32> {
  // identifier: cv_dnn_Layer_inputNameToIndex_String_inputName
    unsafe {
      let input_name = CString::new(input_name).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Layer_inputNameToIndex_String_inputName(self.as_raw_Layer(), input_name.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Returns index of output blob in output array.
  ///  @see inputNameToIndex()
  fn output_name_to_index(&mut self, output_name:&str) -> Result<i32> {
  // identifier: cv_dnn_Layer_outputNameToIndex_String_outputName
    unsafe {
      let output_name = CString::new(output_name).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Layer_outputNameToIndex_String_outputName(self.as_raw_Layer(), output_name.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Ask layer if it support specific backend for doing computations.
  /// ## Parameters
  /// @param[in] backendId computation backend identifier.
  /// @see Backend
  fn support_backend(&mut self, backend_id: i32) -> Result<bool> {
  // identifier: cv_dnn_Layer_supportBackend_int_backendId
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_supportBackend_int_backendId(self.as_raw_Layer(), backend_id);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Returns Halide backend node.
  /// ## Parameters
  /// @param[in] inputs Input Halide buffers.
  /// @see BackendNode, BackendWrapper
  /// 
  /// Input buffers should be exactly the same that will be used in forward invocations.
  /// Despite we can use Halide::ImageParam based on input shape only,
  /// it helps prevent some memory management issues (if something wrong,
  /// Halide tests will be failed).
  fn init_halide(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper) -> Result<types::PtrOfBackendNode> {
  // identifier: cv_dnn_Layer_initHalide_VectorOfPtrOfBackendWrapper_inputs
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_initHalide_VectorOfPtrOfBackendWrapper_inputs(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfBackendNode { ptr: rv.result })
      }
    }
  }

  fn init_inf_engine(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper) -> Result<types::PtrOfBackendNode> {
  // identifier: cv_dnn_Layer_initInfEngine_VectorOfPtrOfBackendWrapper_inputs
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_initInfEngine_VectorOfPtrOfBackendWrapper_inputs(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfBackendNode { ptr: rv.result })
      }
    }
  }

  /// Implement layers fusing.
  /// ## Parameters
  /// @param[in] node Backend node of bottom layer.
  /// @see BackendNode
  /// 
  /// Actual for graph-based backends. If layer attached successfully,
  /// returns non-empty cv::Ptr to node of the same backend.
  /// Fuse only over the last function.
  fn try_attach(&mut self, node: &types::PtrOfBackendNode) -> Result<types::PtrOfBackendNode> {
  // identifier: cv_dnn_Layer_tryAttach_PtrOfBackendNode_node
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_tryAttach_PtrOfBackendNode_node(self.as_raw_Layer(), node.as_raw_PtrOfBackendNode());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfBackendNode { ptr: rv.result })
      }
    }
  }

  /// Tries to attach to the layer the subsequent activation layer, i.e. do the layer fusion in a partial case.
  /// ## Parameters
  /// @param[in] layer The subsequent activation layer.
  /// 
  /// Returns true if the activation layer has been attached successfully.
  fn set_activation(&mut self, layer: &types::PtrOfActivationLayer) -> Result<bool> {
  // identifier: cv_dnn_Layer_setActivation_PtrOfActivationLayer_layer
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_setActivation_PtrOfActivationLayer_layer(self.as_raw_Layer(), layer.as_raw_PtrOfActivationLayer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Returns parameters of layers with channel-wise multiplication and addition.
  /// ## Parameters
  /// @param[out] scale Channel-wise multipliers. Total number of values should
  ///                   be equal to number of channels.
  /// @param[out] shift Channel-wise offsets. Total number of values should
  ///                   be equal to number of channels.
  /// 
  /// Some layers can fuse their transformations with further layers.
  /// In example, convolution + batch normalization. This way base layer
  /// use weights from layer after it. Fused layer is skipped.
  /// By default, @p scale and @p shift are empty that means layer has no
  /// element-wise multiplications or additions.
  fn get_scale_shift(&self, scale: &core::Mat, shift: &core::Mat) -> Result<()> {
  // identifier: cv_dnn_Layer_getScaleShift_Mat_scale_Mat_shift
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_getScaleShift_Mat_scale_Mat_shift(self.as_raw_Layer(), scale.as_raw_Mat(), shift.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// "Deattaches" all the layers, attached to particular layer.
  fn unset_attached(&mut self) -> Result<()> {
  // identifier: cv_dnn_Layer_unsetAttached
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Layer_unsetAttached(self.as_raw_Layer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> Layer + 'a {

}

// boxed class cv::dnn::LayerFactory
/// %Layer factory allows to create instances of registered layers.

#[allow(dead_code)]
pub struct LayerFactory {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::LayerFactory {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_LayerFactory(self.ptr) };
    }
}
impl super::dnn::LayerFactory {
    #[doc(hidden)] pub fn as_raw_LayerFactory(&self) -> *mut c_void { self.ptr }
}
impl LayerFactory {

  pub fn unregister_layer(_type:&str) -> Result<()> {
  // identifier: cv_dnn_LayerFactory_unregisterLayer_String_type
    unsafe {
      let _type = CString::new(_type).unwrap();
      let rv = sys::cv_dnn_cv_dnn_LayerFactory_unregisterLayer_String_type(_type.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
// boxed class cv::dnn::LayerParams
/// This class provides all data needed to initialize layer.
/// 
/// It includes dictionary with scalar params (which can be read by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.

#[allow(dead_code)]
pub struct LayerParams {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::LayerParams {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_LayerParams(self.ptr) };
    }
}
impl super::dnn::LayerParams {
    #[doc(hidden)] pub fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Dict for LayerParams {
    #[doc(hidden)] fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}
impl LayerParams {

}
// boxed class cv::dnn::MVNLayer

#[allow(dead_code)]
pub struct MVNLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::MVNLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MVNLayer(self.ptr) };
    }
}
impl super::dnn::MVNLayer {
    #[doc(hidden)] pub fn as_raw_MVNLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for MVNLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for MVNLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl MVNLayer {

}
// boxed class cv::dnn::MaxUnpoolLayer

#[allow(dead_code)]
pub struct MaxUnpoolLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::MaxUnpoolLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_MaxUnpoolLayer(self.ptr) };
    }
}
impl super::dnn::MaxUnpoolLayer {
    #[doc(hidden)] pub fn as_raw_MaxUnpoolLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for MaxUnpoolLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for MaxUnpoolLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl MaxUnpoolLayer {

}
impl dnn_Net {

  pub fn new() -> Result<super::dnn::dnn_Net> {
  // identifier: cv_dnn_Net_Net
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_Net();
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Create a network from Intel's Model Optimizer intermediate representation.
  /// ## Parameters
  ///  @param[in] xml XML configuration file with network's topology.
  ///  @param[in] bin Binary file with trained weights.
  ///  Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
  ///  backend.
  pub fn read_from_model_optimizer(xml:&str, bin:&str) -> Result<super::dnn::dnn_Net> {
  // identifier: cv_dnn_Net_readFromModelOptimizer_String_xml_String_bin
    unsafe {
      let xml = CString::new(xml).unwrap();
      let bin = CString::new(bin).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_readFromModelOptimizer_String_xml_String_bin(xml.as_ptr() as _, bin.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Returns true if there are no layers in the network.
  pub fn empty(self) -> Result<bool> {
  // identifier: cv_dnn_Net_empty
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_empty(self);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Adds new layer to the net.
  /// ## Parameters
  /// * name:   unique name of the adding layer.
  /// * type:   typename of the adding layer (type must be registered in LayerRegister).
  /// * params: parameters which will be used to initialize the creating layer.
  ///  @returns unique identifier of created layer, or -1 if a failure will happen.
  pub fn add_layer(self, name:&str, _type:&str, params: &super::dnn::LayerParams) -> Result<i32> {
  // identifier: cv_dnn_Net_addLayer_String_name_String_type_LayerParams_params
    unsafe {
      let name = CString::new(name).unwrap();
      let _type = CString::new(_type).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_addLayer_String_name_String_type_LayerParams_params(self, name.as_ptr() as _, _type.as_ptr() as _, params.as_raw_LayerParams());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Adds new layer and connects its first input to the first output of previously added layer.
  ///  @see addLayer()
  pub fn add_layer_to_prev(self, name:&str, _type:&str, params: &super::dnn::LayerParams) -> Result<i32> {
  // identifier: cv_dnn_Net_addLayerToPrev_String_name_String_type_LayerParams_params
    unsafe {
      let name = CString::new(name).unwrap();
      let _type = CString::new(_type).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_addLayerToPrev_String_name_String_type_LayerParams_params(self, name.as_ptr() as _, _type.as_ptr() as _, params.as_raw_LayerParams());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Converts string name of the layer to the integer identifier.
  ///  @returns id of the layer, or -1 if the layer wasn't found.
  pub fn get_layer_id(self, layer:&str) -> Result<i32> {
  // identifier: cv_dnn_Net_getLayerId_String_layer
    unsafe {
      let layer = CString::new(layer).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_getLayerId_String_layer(self, layer.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  pub fn get_layer_names(self) -> Result<types::VectorOfString> {
  // identifier: cv_dnn_Net_getLayerNames
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_getLayerNames(self);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfString { ptr: rv.result })
      }
    }
  }

  /// Connects output of the first layer to input of the second layer.
  /// ## Parameters
  /// * outPin: descriptor of the first layer output.
  /// * inpPin: descriptor of the second layer input.
  /// 
  /// Descriptors have the following template <DFN>&lt;layer_name&gt;[.input_number]</DFN>:
  /// - the first part of the template <DFN>layer_name</DFN> is sting name of the added layer.
  ///   If this part is empty then the network input pseudo layer will be used;
  /// - the second optional part of the template <DFN>input_number</DFN>
  ///   is either number of the layer input, either label one.
  ///   If this part is omitted then the first layer input will be used.
  /// 
  ///  @see setNetInputs(), Layer::inputNameToIndex(), Layer::outputNameToIndex()
  pub fn connect(self, out_pin:&str, inp_pin:&str) -> Result<()> {
  // identifier: cv_dnn_Net_connect_String_outPin_String_inpPin
    unsafe {
      let out_pin = CString::new(out_pin).unwrap();
      let inp_pin = CString::new(inp_pin).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_connect_String_outPin_String_inpPin(self, out_pin.as_ptr() as _, inp_pin.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Connects #@p outNum output of the first layer to #@p inNum input of the second layer.
  /// ## Parameters
  /// * outLayerId: identifier of the first layer
  /// * outNum: number of the first layer output
  /// * inpLayerId: identifier of the second layer
  /// * inpNum: number of the second layer input
  pub fn connect_v0(self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
  // identifier: cv_dnn_Net_connect_int_outLayerId_int_outNum_int_inpLayerId_int_inpNum
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_connect_int_outLayerId_int_outNum_int_inpLayerId_int_inpNum(self, out_layer_id, out_num, inp_layer_id, inp_num);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Sets outputs names of the network input pseudo layer.
  /// 
  /// Each net always has special own the network input pseudo layer with id=0.
  /// This layer stores the user blobs only and don't make any computations.
  /// In fact, this layer provides the only way to pass user data into the network.
  /// As any other layer, this layer can label its outputs and this function provides an easy way to do this.
  pub fn set_inputs_names(self, input_blob_names: &types::VectorOfString) -> Result<()> {
  // identifier: cv_dnn_Net_setInputsNames_VectorOfString_inputBlobNames
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_setInputsNames_VectorOfString_inputBlobNames(self, input_blob_names.as_raw_VectorOfString());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Runs forward pass to compute output of layer with name @p outputName.
  /// ## Parameters
  /// * outputName: name for layer which output is needed to get
  ///  @return blob for first output of specified layer.
  ///  @details By default runs forward pass for the whole network.
  ///
  /// ## C++ default parameters:
  /// * output_name: String()
  pub fn forward(self, output_name:&str) -> Result<core::Mat> {
  // identifier: cv_dnn_Net_forward_String_outputName
    unsafe {
      let output_name = CString::new(output_name).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_forward_String_outputName(self, output_name.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(core::Mat { ptr: rv.result })
      }
    }
  }

  /// Runs forward pass to compute output of layer with name @p outputName.
  /// ## Parameters
  /// * outputBlobs: contains all output blobs for specified layer.
  /// * outputName: name for layer which output is needed to get
  ///  @details If @p outputName is empty, runs forward pass for the whole network.
  ///
  /// ## C++ default parameters:
  /// * output_name: String()
  pub fn forward_v0(self, output_blobs: &mut types::VectorOfMat, output_name:&str) -> Result<()> {
  // identifier: cv_dnn_Net_forward_VectorOfMat_outputBlobs_String_outputName
    unsafe {
      let output_name = CString::new(output_name).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_forward_VectorOfMat_outputBlobs_String_outputName(self, output_blobs.as_raw_VectorOfMat(), output_name.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
  /// ## Parameters
  /// * outputBlobs: contains blobs for first outputs of specified layers.
  /// * outBlobNames: names for layers which outputs are needed to get
  pub fn forward_v1(self, output_blobs: &mut types::VectorOfMat, out_blob_names: &types::VectorOfString) -> Result<()> {
  // identifier: cv_dnn_Net_forward_VectorOfMat_outputBlobs_VectorOfString_outBlobNames
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_forward_VectorOfMat_outputBlobs_VectorOfString_outBlobNames(self, output_blobs.as_raw_VectorOfMat(), out_blob_names.as_raw_VectorOfString());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
  /// ## Parameters
  /// * outputBlobs: contains all output blobs for each layer specified in @p outBlobNames.
  /// * outBlobNames: names for layers which outputs are needed to get
  pub fn forward_v2(self, output_blobs: &types::VectorOfVectorOfMat, out_blob_names: &types::VectorOfString) -> Result<()> {
  // identifier: cv_dnn_Net_forward_VectorOfVectorOfMat_outputBlobs_VectorOfString_outBlobNames
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_forward_VectorOfVectorOfMat_outputBlobs_VectorOfString_outBlobNames(self, output_blobs.as_raw_VectorOfVectorOfMat(), out_blob_names.as_raw_VectorOfString());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Compile Halide layers.
  /// ## Parameters
  /// @param[in] scheduler Path to YAML file with scheduling directives.
  /// @see setPreferableBackend
  /// 
  /// Schedule layers that support Halide backend. Then compile them for
  /// specific target. For layers that not represented in scheduling file
  /// or if no manual scheduling used at all, automatic scheduling will be applied.
  pub fn set_halide_scheduler(self, scheduler:&str) -> Result<()> {
  // identifier: cv_dnn_Net_setHalideScheduler_String_scheduler
    unsafe {
      let scheduler = CString::new(scheduler).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_setHalideScheduler_String_scheduler(self, scheduler.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Ask network to use specific computation backend where it supported.
  /// ## Parameters
  /// @param[in] backendId backend identifier.
  /// @see Backend
  /// 
  /// If OpenCV is compiled with Intel's Inference Engine library, DNN_BACKEND_DEFAULT
  /// means DNN_BACKEND_INFERENCE_ENGINE. Otherwise it equals to DNN_BACKEND_OPENCV.
  pub fn set_preferable_backend(self, backend_id: i32) -> Result<()> {
  // identifier: cv_dnn_Net_setPreferableBackend_int_backendId
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_setPreferableBackend_int_backendId(self, backend_id);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Ask network to make computations on specific target device.
  /// ## Parameters
  /// @param[in] targetId target identifier.
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
  pub fn set_preferable_target(self, target_id: i32) -> Result<()> {
  // identifier: cv_dnn_Net_setPreferableTarget_int_targetId
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_setPreferableTarget_int_targetId(self, target_id);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Sets the new input value for the network
  /// ## Parameters
  /// * blob:        A new blob. Should have CV_32F or CV_8U depth.
  /// * name:        A name of input layer.
  /// * scalefactor: An optional normalization scale.
  /// * mean:        An optional mean subtraction values.
  ///  @see connect(String, String) to know format of the descriptor.
  /// 
  ///  If scale or mean values are specified, a final input blob is computed
  ///  as:
  /// <div lang='latex'>input(n,c,h,w) = scalefactor \times (blob(n,c,h,w) - mean_c)</div>
  ///
  /// ## C++ default parameters:
  /// * name: ""
  /// * scalefactor: 1.0
  /// * mean: Scalar()
  pub fn set_input(self, blob: &core::Mat, name:&str, scalefactor: f64, mean: core::Scalar) -> Result<()> {
  // identifier: cv_dnn_Net_setInput_Mat_blob_String_name_double_scalefactor_Scalar_mean
    unsafe {
      let name = CString::new(name).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_setInput_Mat_blob_String_name_double_scalefactor_Scalar_mean(self, blob.as_raw_Mat(), name.as_ptr() as _, scalefactor, mean);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns indexes of layers with unconnected outputs.
  pub fn get_unconnected_out_layers(self) -> Result<types::VectorOfint> {
  // identifier: cv_dnn_Net_getUnconnectedOutLayers
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_getUnconnectedOutLayers(self);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfint { ptr: rv.result })
      }
    }
  }

  /// Returns names of layers with unconnected outputs.
  pub fn get_unconnected_out_layers_names(self) -> Result<types::VectorOfString> {
  // identifier: cv_dnn_Net_getUnconnectedOutLayersNames
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_getUnconnectedOutLayersNames(self);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::VectorOfString { ptr: rv.result })
      }
    }
  }

  /// Returns list of types for layer used in model.
  /// ## Parameters
  /// * layersTypes: output parameter for returning types.
  pub fn get_layer_types(self, layers_types: &types::VectorOfString) -> Result<()> {
  // identifier: cv_dnn_Net_getLayerTypes_VectorOfString_layersTypes
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_getLayerTypes_VectorOfString_layersTypes(self, layers_types.as_raw_VectorOfString());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns count of layers of specified type.
  /// ## Parameters
  /// * layerType: type.
  /// @returns count of layers
  pub fn get_layers_count(self, layer_type:&str) -> Result<i32> {
  // identifier: cv_dnn_Net_getLayersCount_String_layerType
    unsafe {
      let layer_type = CString::new(layer_type).unwrap();
      let rv = sys::cv_dnn_cv_dnn_Net_getLayersCount_String_layerType(self, layer_type.as_ptr() as _);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Enables or disables layer fusion in the network.
  /// ## Parameters
  /// * fusion: true to enable the fusion, false to disable. The fusion is enabled by default.
  pub fn enable_fusion(self, fusion: bool) -> Result<()> {
  // identifier: cv_dnn_Net_enableFusion_bool_fusion
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_enableFusion_bool_fusion(self, fusion);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Returns overall time for inference and timings (in ticks) for layers.
  /// Indexes in returned vector correspond to layers ids. Some layers can be fused with others,
  /// in this case zero ticks count will be return for that skipped layers.
  /// ## Parameters
  /// * timings: vector for tick timings for all layers.
  /// @return overall ticks for model inference.
  pub fn get_perf_profile(self, timings: &types::VectorOfdouble) -> Result<i64> {
  // identifier: cv_dnn_Net_getPerfProfile_VectorOfdouble_timings
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_Net_getPerfProfile_VectorOfdouble_timings(self, timings.as_raw_VectorOfdouble());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
// boxed class cv::dnn::NormalizeBBoxLayer
/// <span lang='latex'> L_p </span> - normalization layer.
/// ## Parameters
/// * p: Normalization factor. The most common `p = 1` for <span lang='latex'> L_1 </span> -
///          normalization or `p = 2` for <span lang='latex'> L_2 </span> - normalization or a custom one.
/// * eps: Parameter <span lang='latex'> \epsilon </span> to prevent a division by zero.
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

#[allow(dead_code)]
pub struct NormalizeBBoxLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::NormalizeBBoxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_NormalizeBBoxLayer(self.ptr) };
    }
}
impl super::dnn::NormalizeBBoxLayer {
    #[doc(hidden)] pub fn as_raw_NormalizeBBoxLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for NormalizeBBoxLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for NormalizeBBoxLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl NormalizeBBoxLayer {

}
// boxed class cv::dnn::PaddingLayer
/// Adds extra values for specific axes.
/// ## Parameters
/// * paddings: Vector of paddings in format
///                 ```ignore
///                 [ pad_before, pad_after,  // [0]th dimension
///                   pad_before, pad_after,  // [1]st dimension
///                   ...
///                   pad_before, pad_after ] // [n]th dimension
///                 ```
/// 
///                 that represents number of padded values at every dimension
///                 starting from the first one. The rest of dimensions won't
///                 be padded.
/// * value: Value to be padded. Defaults to zero.
/// * type: Padding type: 'constant', 'reflect'
/// * input_dims: Torch's parameter. If @p input_dims is not equal to the
///                   actual input dimensionality then the `[0]th` dimension
///                   is considered as a batch dimension and @p paddings are shifted
///                   to a one dimension. Defaults to `-1` that means padding
///                   corresponding to @p paddings.

#[allow(dead_code)]
pub struct PaddingLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::PaddingLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PaddingLayer(self.ptr) };
    }
}
impl super::dnn::PaddingLayer {
    #[doc(hidden)] pub fn as_raw_PaddingLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for PaddingLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for PaddingLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl PaddingLayer {

}
// boxed class cv::dnn::PermuteLayer

#[allow(dead_code)]
pub struct PermuteLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::PermuteLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PermuteLayer(self.ptr) };
    }
}
impl super::dnn::PermuteLayer {
    #[doc(hidden)] pub fn as_raw_PermuteLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for PermuteLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for PermuteLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl PermuteLayer {

}
// boxed class cv::dnn::PoolingLayer

#[allow(dead_code)]
pub struct PoolingLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::PoolingLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PoolingLayer(self.ptr) };
    }
}
impl super::dnn::PoolingLayer {
    #[doc(hidden)] pub fn as_raw_PoolingLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for PoolingLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for PoolingLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl PoolingLayer {

}
// boxed class cv::dnn::PowerLayer

#[allow(dead_code)]
pub struct PowerLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::PowerLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PowerLayer(self.ptr) };
    }
}
impl super::dnn::PowerLayer {
    #[doc(hidden)] pub fn as_raw_PowerLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for PowerLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for PowerLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for PowerLayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl PowerLayer {

}
// boxed class cv::dnn::PriorBoxLayer

#[allow(dead_code)]
pub struct PriorBoxLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::PriorBoxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_PriorBoxLayer(self.ptr) };
    }
}
impl super::dnn::PriorBoxLayer {
    #[doc(hidden)] pub fn as_raw_PriorBoxLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for PriorBoxLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for PriorBoxLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl PriorBoxLayer {

}
// boxed class cv::dnn::ProposalLayer

#[allow(dead_code)]
pub struct ProposalLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ProposalLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ProposalLayer(self.ptr) };
    }
}
impl super::dnn::ProposalLayer {
    #[doc(hidden)] pub fn as_raw_ProposalLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ProposalLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ProposalLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ProposalLayer {

}
// Generating impl for trait cv::dnn::RNNLayer (trait)
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
pub trait RNNLayer : super::dnn::Layer {
  #[doc(hidden)] fn as_raw_RNNLayer(&self) -> *mut c_void;
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
  /// * bh:  is @f$ b_{h}  @f$ vector
  /// * Whh: is @f$ W_{hh} @f$ matrix
  /// * Who: is @f$ W_{xo} @f$ matrix
  /// * bo:  is @f$ b_{o}  @f$ vector
  fn set_weights(&mut self, wxh: &core::Mat, bh: &core::Mat, whh: &core::Mat, who: &core::Mat, bo: &core::Mat) -> Result<()> {
  // identifier: cv_dnn_RNNLayer_setWeights_Mat_Wxh_Mat_bh_Mat_Whh_Mat_Who_Mat_bo
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_RNNLayer_setWeights_Mat_Wxh_Mat_bh_Mat_Whh_Mat_Who_Mat_bo(self.as_raw_RNNLayer(), wxh.as_raw_Mat(), bh.as_raw_Mat(), whh.as_raw_Mat(), who.as_raw_Mat(), bo.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
  /// @details Shape of the second output is the same as first output.
  ///
  /// ## C++ default parameters:
  /// * produce: false
  fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
  // identifier: cv_dnn_RNNLayer_setProduceHiddenOutput_bool_produce
    unsafe {
      let rv = sys::cv_dnn_cv_dnn_RNNLayer_setProduceHiddenOutput_bool_produce(self.as_raw_RNNLayer(), produce);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> RNNLayer + 'a {

}

// boxed class cv::dnn::ReLU6Layer

#[allow(dead_code)]
pub struct ReLU6Layer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ReLU6Layer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ReLU6Layer(self.ptr) };
    }
}
impl super::dnn::ReLU6Layer {
    #[doc(hidden)] pub fn as_raw_ReLU6Layer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ReLU6Layer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ReLU6Layer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for ReLU6Layer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl ReLU6Layer {

}
// boxed class cv::dnn::ReLULayer

#[allow(dead_code)]
pub struct ReLULayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ReLULayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ReLULayer(self.ptr) };
    }
}
impl super::dnn::ReLULayer {
    #[doc(hidden)] pub fn as_raw_ReLULayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ReLULayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ReLULayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for ReLULayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl ReLULayer {

}
// boxed class cv::dnn::RegionLayer

#[allow(dead_code)]
pub struct RegionLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::RegionLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_RegionLayer(self.ptr) };
    }
}
impl super::dnn::RegionLayer {
    #[doc(hidden)] pub fn as_raw_RegionLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for RegionLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for RegionLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl RegionLayer {

}
// boxed class cv::dnn::ReorgLayer

#[allow(dead_code)]
pub struct ReorgLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ReorgLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ReorgLayer(self.ptr) };
    }
}
impl super::dnn::ReorgLayer {
    #[doc(hidden)] pub fn as_raw_ReorgLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ReorgLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ReorgLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ReorgLayer {

}
// boxed class cv::dnn::ReshapeLayer

#[allow(dead_code)]
pub struct ReshapeLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ReshapeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ReshapeLayer(self.ptr) };
    }
}
impl super::dnn::ReshapeLayer {
    #[doc(hidden)] pub fn as_raw_ReshapeLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ReshapeLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ReshapeLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ReshapeLayer {

}
// boxed class cv::dnn::ResizeLayer
/// Resize input 4-dimensional blob by nearest neighbor or bilinear strategy.
/// 
/// Layer is used to support TensorFlow's resize_nearest_neighbor and resize_bilinear ops.

#[allow(dead_code)]
pub struct ResizeLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ResizeLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ResizeLayer(self.ptr) };
    }
}
impl super::dnn::ResizeLayer {
    #[doc(hidden)] pub fn as_raw_ResizeLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ResizeLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ResizeLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ResizeLayer {

}
// boxed class cv::dnn::ScaleLayer

#[allow(dead_code)]
pub struct ScaleLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ScaleLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ScaleLayer(self.ptr) };
    }
}
impl super::dnn::ScaleLayer {
    #[doc(hidden)] pub fn as_raw_ScaleLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ScaleLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ScaleLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ScaleLayer {

}
// boxed class cv::dnn::ShiftLayer

#[allow(dead_code)]
pub struct ShiftLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ShiftLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ShiftLayer(self.ptr) };
    }
}
impl super::dnn::ShiftLayer {
    #[doc(hidden)] pub fn as_raw_ShiftLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ShiftLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ShiftLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ShiftLayer {

}
// boxed class cv::dnn::ShuffleChannelLayer
/// Permute channels of 4-dimensional input blob.
/// ## Parameters
/// * group: Number of groups to split input channels and pick in turns
///              into output blob.
/// 
/// <div lang='latex'> groupSize = \frac{number\ of\ channels}{group} </div>
/// <div lang='latex'> output(n, c, h, w) = input(n, groupSize \times (c \% group) + \lfloor \frac{c}{group} \rfloor, h, w) </div>
/// Read more at https://arxiv.org/pdf/1707.01083.pdf

#[allow(dead_code)]
pub struct ShuffleChannelLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::ShuffleChannelLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ShuffleChannelLayer(self.ptr) };
    }
}
impl super::dnn::ShuffleChannelLayer {
    #[doc(hidden)] pub fn as_raw_ShuffleChannelLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for ShuffleChannelLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ShuffleChannelLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl ShuffleChannelLayer {

}
// boxed class cv::dnn::SigmoidLayer

#[allow(dead_code)]
pub struct SigmoidLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::SigmoidLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_SigmoidLayer(self.ptr) };
    }
}
impl super::dnn::SigmoidLayer {
    #[doc(hidden)] pub fn as_raw_SigmoidLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for SigmoidLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for SigmoidLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for SigmoidLayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl SigmoidLayer {

}
// boxed class cv::dnn::SliceLayer
/// Slice layer has several modes:
/// 1. Caffe mode
/// ## Parameters
/// @param[in] axis Axis of split operation
/// @param[in] slice_point Array of split points
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

#[allow(dead_code)]
pub struct SliceLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::SliceLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_SliceLayer(self.ptr) };
    }
}
impl super::dnn::SliceLayer {
    #[doc(hidden)] pub fn as_raw_SliceLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for SliceLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for SliceLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl SliceLayer {

}
// boxed class cv::dnn::SoftmaxLayer

#[allow(dead_code)]
pub struct SoftmaxLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::SoftmaxLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_SoftmaxLayer(self.ptr) };
    }
}
impl super::dnn::SoftmaxLayer {
    #[doc(hidden)] pub fn as_raw_SoftmaxLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for SoftmaxLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for SoftmaxLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl SoftmaxLayer {

}
// boxed class cv::dnn::SplitLayer

#[allow(dead_code)]
pub struct SplitLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::SplitLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_SplitLayer(self.ptr) };
    }
}
impl super::dnn::SplitLayer {
    #[doc(hidden)] pub fn as_raw_SplitLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for SplitLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for SplitLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}
impl SplitLayer {

}
// boxed class cv::dnn::TanHLayer

#[allow(dead_code)]
pub struct TanHLayer {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::dnn::TanHLayer {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_TanHLayer(self.ptr) };
    }
}
impl super::dnn::TanHLayer {
    #[doc(hidden)] pub fn as_raw_TanHLayer(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::Layer for TanHLayer {
    #[doc(hidden)] fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for TanHLayer {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::dnn::ActivationLayer for TanHLayer {
    #[doc(hidden)] fn as_raw_ActivationLayer(&self) -> *mut c_void { self.ptr }
}
impl TanHLayer {

}
