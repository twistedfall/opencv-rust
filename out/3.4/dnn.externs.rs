// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1078
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
pub fn cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, ocvrs_return: *mut Result<()>);
// NMSBoxes(const std::vector<Rect2d> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1078
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect2d>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
pub fn cv_dnn_NMSBoxes_const_vectorLRect2dGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32, ocvrs_return: *mut Result<()>);
// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1073
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
pub fn cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, ocvrs_return: *mut Result<()>);
// NMSBoxes(const std::vector<Rect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1073
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::Rect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
pub fn cv_dnn_NMSBoxes_const_vectorLRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32, ocvrs_return: *mut Result<()>);
// cv::dnn::NMSBoxes(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1083
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*"]), _)]),
pub fn cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, ocvrs_return: *mut Result<()>);
// NMSBoxes(const std::vector<RotatedRect> &, const std::vector<float> &, const float, const float, std::vector<int> &, const float, const int)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1083
// ("cv::dnn::NMSBoxes", vec![(pred!(mut, ["bboxes", "scores", "score_threshold", "nms_threshold", "indices", "eta", "top_k"], ["const std::vector<cv::RotatedRect>*", "const std::vector<float>*", "const float", "const float", "std::vector<int>*", "const float", "const int"]), _)]),
pub fn cv_dnn_NMSBoxes_const_vectorLRotatedRectGR_const_vectorLfloatGR_const_float_const_float_vectorLintGR_const_float_const_int(bboxes: *const c_void, scores: *const c_void, score_threshold: f32, nms_threshold: f32, indices: *mut c_void, eta: f32, top_k: i32, ocvrs_return: *mut Result<()>);
// cv::dnn::blobFromImage(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:986
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_dnn_blobFromImage_const__InputArrayR(image: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::blobFromImage(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:994
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR(image: *const c_void, blob: *const c_void, ocvrs_return: *mut Result<()>);
// blobFromImage(InputArray, OutputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:994
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
pub fn cv_dnn_blobFromImage_const__InputArrayR_const__OutputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image: *const c_void, blob: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result<()>);
// blobFromImage(InputArray, double, const Size &, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:986
// ("cv::dnn::blobFromImage", vec![(pred!(mut, ["image", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "const cv::Size*", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
pub fn cv_dnn_blobFromImage_const__InputArrayR_double_const_SizeR_const_ScalarR_bool_bool_int(image: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::blobFromImages(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1016
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
pub fn cv_dnn_blobFromImages_const__InputArrayR(images: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::blobFromImages(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1024
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR(images: *const c_void, blob: *const c_void, ocvrs_return: *mut Result<()>);
// blobFromImages(InputArrayOfArrays, OutputArray, double, Size, const Scalar &, bool, bool, int)(InputArray, OutputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1024
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "blob", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
pub fn cv_dnn_blobFromImages_const__InputArrayR_const__OutputArrayR_double_Size_const_ScalarR_bool_bool_int(images: *const c_void, blob: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result<()>);
// blobFromImages(InputArrayOfArrays, double, Size, const Scalar &, bool, bool, int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1016
// ("cv::dnn::blobFromImages", vec![(pred!(mut, ["images", "scalefactor", "size", "mean", "swapRB", "crop", "ddepth"], ["const cv::_InputArray*", "double", "cv::Size", "const cv::Scalar*", "bool", "bool", "int"]), _)]),
pub fn cv_dnn_blobFromImages_const__InputArrayR_double_Size_const_ScalarR_bool_bool_int(images: *const c_void, scalefactor: f64, size: *const core::Size, mean: *const core::Scalar, swap_rb: bool, crop: bool, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
// concat(const MatShape &, const MatShape &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:206
// ("cv::dnn::concat", vec![(pred!(mut, ["a", "b"], ["const cv::dnn::MatShape*", "const cv::dnn::MatShape*"]), _)]),
pub fn cv_dnn_concat_const_MatShapeR_const_MatShapeR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getAvailableBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:104
// ("cv::dnn::getAvailableBackends", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_getAvailableBackends(ocvrs_return: *mut Result<*mut c_void>);
// getAvailableTargets(dnn::Backend)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:105
// ("cv::dnn::getAvailableTargets", vec![(pred!(mut, ["be"], ["cv::dnn::Backend"]), _)]),
pub fn cv_dnn_getAvailableTargets_Backend(be: crate::dnn::Backend, ocvrs_return: *mut Result<*mut c_void>);
// getInferenceEngineBackendType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:31
// ("cv::dnn::getInferenceEngineBackendType", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_getInferenceEngineBackendType(ocvrs_return: *mut Result<*mut c_void>);
// getInferenceEngineCPUType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:72
// ("cv::dnn::getInferenceEngineCPUType", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_getInferenceEngineCPUType(ocvrs_return: *mut Result<*mut c_void>);
// getInferenceEngineVPUType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:66
// ("cv::dnn::getInferenceEngineVPUType", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_getInferenceEngineVPUType(ocvrs_return: *mut Result<*mut c_void>);
// getPlane(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:108
// ("cv::dnn::getPlane", vec![(pred!(mut, ["m", "n", "cn"], ["const cv::Mat*", "int", "int"]), _)]),
pub fn cv_dnn_getPlane_const_MatR_int_int(m: *const c_void, n: i32, cn: i32, ocvrs_return: *mut Result<*mut c_void>);
// imagesFromBlob(const cv::Mat &, OutputArrayOfArrays)(TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1037
// ("cv::dnn::imagesFromBlob", vec![(pred!(mut, ["blob_", "images_"], ["const cv::Mat*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_imagesFromBlob_const_MatR_const__OutputArrayR(blob_: *const c_void, images_: *const c_void, ocvrs_return: *mut Result<()>);
// cv::dnn::readNetFromCaffe(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:787
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromCaffe_const_StringR(prototxt: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromCaffe(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:787
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["prototxt", "caffeModel"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromCaffe_const_StringR_const_StringR(prototxt: *const c_char, caffe_model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromCaffe(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:806
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto"], ["const char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromCaffe_const_charX_size_t(buffer_proto: *const c_char, len_proto: size_t, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromCaffe(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:806
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "lenProto", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(buffer_proto: *const c_char, len_proto: size_t, buffer_model: *const c_char, len_model: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromCaffe(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:794
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto"], ["const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR(buffer_proto: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromCaffe(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:794
// ("cv::dnn::readNetFromCaffe", vec![(pred!(mut, ["bufferProto", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromCaffe_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_proto: *const c_void, buffer_model: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromDarknet(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:762
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromDarknet_const_StringR(cfg_file: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromDarknet(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:762
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["cfgFile", "darknetModel"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromDarknet_const_StringR_const_StringR(cfg_file: *const c_char, darknet_model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromDarknet(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:779
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg"], ["const char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromDarknet_const_charX_size_t(buffer_cfg: *const c_char, len_cfg: size_t, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromDarknet(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:779
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "lenCfg", "bufferModel", "lenModel"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(buffer_cfg: *const c_char, len_cfg: size_t, buffer_model: *const c_char, len_model: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromDarknet(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:769
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg"], ["const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR(buffer_cfg: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromDarknet(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:769
// ("cv::dnn::readNetFromDarknet", vec![(pred!(mut, ["bufferCfg", "bufferModel"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromDarknet_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_cfg: *const c_void, buffer_model: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:916
// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromModelOptimizer_const_StringR_const_StringR(xml: *const c_char, bin: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:938
// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr: *const u8, buffer_model_config_size: size_t, buffer_weights_ptr: *const u8, buffer_weights_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:926
// ("cv::dnn::readNetFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_model_config: *const c_void, buffer_weights: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:945
// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["onnxFile"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromONNX_const_StringR(onnx_file: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromONNX(const char *, size_t)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:954
// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer", "sizeBuffer"], ["const char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromONNX_const_charX_size_t(buffer: *const c_char, size_buffer: size_t, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromONNX(const std::vector<uchar> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:962
// ("cv::dnn::readNetFromONNX", vec![(pred!(mut, ["buffer"], ["const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromONNX_const_vectorLunsigned_charGR(buffer: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromTensorflow(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:816
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromTensorflow_const_StringR(model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromTensorflow(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:816
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["model", "config"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromTensorflow_const_StringR_const_StringR(model: *const c_char, config: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromTensorflow(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:834
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel"], ["const char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromTensorflow_const_charX_size_t(buffer_model: *const c_char, len_model: size_t, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromTensorflow(const char *, size_t, const char *, size_t)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:834
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "lenModel", "bufferConfig", "lenConfig"], ["const char*", "size_t", "const char*", "size_t"]), _)]),
pub fn cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(buffer_model: *const c_char, len_model: size_t, buffer_config: *const c_char, len_config: size_t, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromTensorflow(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:823
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel"], ["const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR(buffer_model: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromTensorflow(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:823
// ("cv::dnn::readNetFromTensorflow", vec![(pred!(mut, ["bufferModel", "bufferConfig"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNetFromTensorflow_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_model: *const c_void, buffer_config: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNetFromTorch(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:863
// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readNetFromTorch_const_StringR(model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readNetFromTorch(const String &, bool, bool)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:863
// ("cv::dnn::readNetFromTorch", vec![(pred!(mut, ["model", "isBinary", "evaluate"], ["const cv::String*", "bool", "bool"]), _)]),
pub fn cv_dnn_readNetFromTorch_const_StringR_bool_bool(model: *const c_char, is_binary: bool, evaluate: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNet(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:889
// ("cv::dnn::readNet", vec![(pred!(mut, ["model"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readNet_const_StringR(model: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readNet(const String &, const String &, const String &)(InString, InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:889
// ("cv::dnn::readNet", vec![(pred!(mut, ["model", "config", "framework"], ["const cv::String*", "const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_readNet_const_StringR_const_StringR_const_StringR(model: *const c_char, config: *const c_char, framework: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readNet(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:900
// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel"], ["const cv::String*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR(framework: *const c_char, buffer_model: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readNet(const String &, const std::vector<uchar> &, const std::vector<uchar> &)(InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:900
// ("cv::dnn::readNet", vec![(pred!(mut, ["framework", "bufferModel", "bufferConfig"], ["const cv::String*", "const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_readNet_const_StringR_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(framework: *const c_char, buffer_model: *const c_void, buffer_config: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readTensorFromONNX(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:968
// ("cv::dnn::readTensorFromONNX", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readTensorFromONNX_const_StringR(path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::readTorchBlob(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:906
// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_dnn_readTorchBlob_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readTorchBlob(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:906
// ("cv::dnn::readTorchBlob", vec![(pred!(mut, ["filename", "isBinary"], ["const cv::String*", "bool"]), _)]),
pub fn cv_dnn_readTorchBlob_const_StringR_bool(filename: *const c_char, is_binary: bool, ocvrs_return: *mut Result<*mut c_void>);
// resetMyriadDevice()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:49
// ("cv::dnn::resetMyriadDevice", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_resetMyriadDevice(ocvrs_return: *mut Result<()>);
// setInferenceEngineBackendType(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/utils/inference_engine.hpp:41
// ("cv::dnn::setInferenceEngineBackendType", vec![(pred!(mut, ["newBackendType"], ["const cv::String*"]), _)]),
pub fn cv_dnn_setInferenceEngineBackendType_const_StringR(new_backend_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// shape(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:126
// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
pub fn cv_dnn_shape_const_MatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// shape(const MatSize &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:131
// ("cv::dnn::shape", vec![(pred!(mut, ["sz"], ["const cv::MatSize*"]), _)]),
pub fn cv_dnn_shape_const_MatSizeR(sz: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// shape(const UMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:136
// ("cv::dnn::shape", vec![(pred!(mut, ["mat"], ["const cv::UMat*"]), _)]),
pub fn cv_dnn_shape_const_UMatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// shape(const int *, const int)(Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:119
// ("cv::dnn::shape", vec![(pred!(mut, ["dims", "n"], ["const int*", "const int"]), _)]),
pub fn cv_dnn_shape_const_intX_const_int(dims: *const i32, n: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::shape(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:153
// ("cv::dnn::shape", vec![(pred!(mut, ["a0"], ["int"]), _)]),
pub fn cv_dnn_shape_int(a0: i32, ocvrs_return: *mut Result<*mut c_void>);
// shape(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:153
// ("cv::dnn::shape", vec![(pred!(mut, ["a0", "a1", "a2", "a3"], ["int", "int", "int", "int"]), _)]),
pub fn cv_dnn_shape_int_int_int_int(a0: i32, a1: i32, a2: i32, a3: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::shrinkCaffeModel(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1052
// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_shrinkCaffeModel_const_StringR_const_StringR(src: *const c_char, dst: *const c_char, ocvrs_return: *mut Result<()>);
// shrinkCaffeModel(const String &, const String &, const std::vector<String> &)(InString, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1052
// ("cv::dnn::shrinkCaffeModel", vec![(pred!(mut, ["src", "dst", "layersTypes"], ["const cv::String*", "const cv::String*", "const std::vector<cv::String>*"]), _)]),
pub fn cv_dnn_shrinkCaffeModel_const_StringR_const_StringR_const_vectorLStringGR(src: *const c_char, dst: *const c_char, layers_types: *const c_void, ocvrs_return: *mut Result<()>);
// slice(const Mat &, const _Range &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:63
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0"], ["const cv::Mat*", "const cv::dnn::_Range*"]), _)]),
pub fn cv_dnn_slice_const_MatR_const__RangeR(m: *const c_void, r0: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// slice(const Mat &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:72
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
pub fn cv_dnn_slice_const_MatR_const__RangeR_const__RangeR(m: *const c_void, r0: *const c_void, r1: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// slice(const Mat &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:83
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
pub fn cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR(m: *const c_void, r0: *const c_void, r1: *const c_void, r2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// slice(const Mat &, const _Range &, const _Range &, const _Range &, const _Range &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:95
// ("cv::dnn::slice", vec![(pred!(mut, ["m", "r0", "r1", "r2", "r3"], ["const cv::Mat*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*", "const cv::dnn::_Range*"]), _)]),
pub fn cv_dnn_slice_const_MatR_const__RangeR_const__RangeR_const__RangeR_const__RangeR(m: *const c_void, r0: *const c_void, r1: *const c_void, r2: *const c_void, r3: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::total(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:184
// ("cv::dnn::total", vec![(pred!(mut, ["mat"], ["const cv::Mat*"]), _)]),
pub fn cv_dnn_total_const_MatR(mat: *const c_void, ocvrs_return: *mut Result<i32>);
// total(const Mat &, int, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:184
// ("cv::dnn::total", vec![(pred!(mut, ["mat", "start", "end"], ["const cv::Mat*", "int", "int"]), _)]),
pub fn cv_dnn_total_const_MatR_int_int(mat: *const c_void, start: i32, end: i32, ocvrs_return: *mut Result<i32>);
// cv::dnn::total(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:161
// ("cv::dnn::total", vec![(pred!(mut, ["shape"], ["const cv::dnn::MatShape*"]), _)]),
pub fn cv_dnn_total_const_MatShapeR(shape: *const c_void, ocvrs_return: *mut Result<i32>);
// total(const MatShape &, int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:161
// ("cv::dnn::total", vec![(pred!(mut, ["shape", "start", "end"], ["const cv::dnn::MatShape*", "int", "int"]), _)]),
pub fn cv_dnn_total_const_MatShapeR_int_int(shape: *const c_void, start: i32, end: i32, ocvrs_return: *mut Result<i32>);
// writeTextGraph(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:1061
// ("cv::dnn::writeTextGraph", vec![(pred!(mut, ["model", "output"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_writeTextGraph_const_StringR_const_StringR(model: *const c_char, output: *const c_char, ocvrs_return: *mut Result<()>);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:494
// ("cv::dnn::AbsLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_AbsLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::AbsLayer::to_ActivationLayer() generated
// ("cv::dnn::AbsLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_AbsLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::AbsLayer::to_Algorithm() generated
// ("cv::dnn::AbsLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_AbsLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::AbsLayer::to_Layer() generated
// ("cv::dnn::AbsLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_AbsLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::AbsLayer::delete() generated
// ("cv::dnn::AbsLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_AbsLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:583
// ("cv::dnn::AccumLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_AccumLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::AccumLayer::defaultNew() generated
// ("cv::dnn::AccumLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_AccumLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::AccumLayer::to_Algorithm() generated
// ("cv::dnn::AccumLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_AccumLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::AccumLayer::to_Layer() generated
// ("cv::dnn::AccumLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_AccumLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::AccumLayer::delete() generated
// ("cv::dnn::AccumLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_AccumLayer_delete(instance: *mut c_void);
// forwardSlice(const float *, float *, int, size_t, int, int)(VariableArray, VariableArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:427
// ("cv::dnn::ActivationLayer::forwardSlice", vec![(pred!(const, ["src", "dst", "len", "outPlaneSize", "cn0", "cn1"], ["const float*", "float*", "int", "size_t", "int", "int"]), _)]),
pub fn cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(instance: *const c_void, src: *const f32, dst: *mut f32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32, ocvrs_return: *mut Result<()>);
// cv::dnn::ActivationLayer::to_AbsLayer() generated
// ("cv::dnn::ActivationLayer::to_AbsLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_AbsLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_BNLLLayer() generated
// ("cv::dnn::ActivationLayer::to_BNLLLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_BNLLLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_BatchNormLayer() generated
// ("cv::dnn::ActivationLayer::to_BatchNormLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_BatchNormLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_ChannelsPReLULayer() generated
// ("cv::dnn::ActivationLayer::to_ChannelsPReLULayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_ChannelsPReLULayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_ELULayer() generated
// ("cv::dnn::ActivationLayer::to_ELULayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_ELULayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_ExpLayer() generated
// ("cv::dnn::ActivationLayer::to_ExpLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_ExpLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_MishLayer() generated
// ("cv::dnn::ActivationLayer::to_MishLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_MishLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_PowerLayer() generated
// ("cv::dnn::ActivationLayer::to_PowerLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_PowerLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_ReLU6Layer() generated
// ("cv::dnn::ActivationLayer::to_ReLU6Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_ReLU6Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_ReLULayer() generated
// ("cv::dnn::ActivationLayer::to_ReLULayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_ReLULayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_SigmoidLayer() generated
// ("cv::dnn::ActivationLayer::to_SigmoidLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_SigmoidLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_SwishLayer() generated
// ("cv::dnn::ActivationLayer::to_SwishLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_SwishLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_TanHLayer() generated
// ("cv::dnn::ActivationLayer::to_TanHLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_TanHLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_Algorithm() generated
// ("cv::dnn::ActivationLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::to_Layer() generated
// ("cv::dnn::ActivationLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ActivationLayer::delete() generated
// ("cv::dnn::ActivationLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ActivationLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:488
// ("cv::dnn::BNLLLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_BNLLLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::BNLLLayer::to_ActivationLayer() generated
// ("cv::dnn::BNLLLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BNLLLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BNLLLayer::to_Algorithm() generated
// ("cv::dnn::BNLLLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BNLLLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BNLLLayer::to_Layer() generated
// ("cv::dnn::BNLLLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BNLLLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BNLLLayer::delete() generated
// ("cv::dnn::BNLLLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BNLLLayer_delete(instance: *mut c_void);
// cv::dnn::BackendNode::backendId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:132
// ("cv::dnn::BackendNode::backendId", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BackendNode_propBackendId_const(instance: *const c_void) -> i32;
// cv::dnn::BackendNode::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:132
// ("cv::dnn::BackendNode::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_BackendNode_propBackendId_const_int(instance: *mut c_void, val: i32);
// cv::dnn::BackendNode::delete() generated
// ("cv::dnn::BackendNode::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BackendNode_delete(instance: *mut c_void);
// copyToHost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:169
// ("cv::dnn::BackendWrapper::copyToHost", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BackendWrapper_copyToHost(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setHostDirty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:174
// ("cv::dnn::BackendWrapper::setHostDirty", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BackendWrapper_setHostDirty(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::dnn::BackendWrapper::backendId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:176
// ("cv::dnn::BackendWrapper::backendId", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BackendWrapper_propBackendId_const(instance: *const c_void) -> i32;
// cv::dnn::BackendWrapper::setBackendId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:176
// ("cv::dnn::BackendWrapper::setBackendId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_BackendWrapper_propBackendId_const_int(instance: *mut c_void, val: i32);
// cv::dnn::BackendWrapper::targetId() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:177
// ("cv::dnn::BackendWrapper::targetId", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BackendWrapper_propTargetId_const(instance: *const c_void) -> i32;
// cv::dnn::BackendWrapper::setTargetId(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:177
// ("cv::dnn::BackendWrapper::setTargetId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_BackendWrapper_propTargetId_const_int(instance: *mut c_void, val: i32);
// cv::dnn::BackendWrapper::delete() generated
// ("cv::dnn::BackendWrapper::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BackendWrapper_delete(instance: *mut c_void);
// cv::dnn::BaseConvolutionLayer::defaultNew() generated
// ("cv::dnn::BaseConvolutionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::kernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::kernel", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propKernel_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::BaseConvolutionLayer::setKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::setKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propKernel_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::BaseConvolutionLayer::stride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::stride", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propStride_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::BaseConvolutionLayer::setStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::setStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propStride_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::BaseConvolutionLayer::pad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::pad", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPad_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::BaseConvolutionLayer::setPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::setPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPad_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::BaseConvolutionLayer::dilation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::dilation", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propDilation_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::BaseConvolutionLayer::setDilation(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::setDilation", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propDilation_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::BaseConvolutionLayer::adjustPad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::adjustPad", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propAdjustPad_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::BaseConvolutionLayer::setAdjustPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:213
// ("cv::dnn::BaseConvolutionLayer::setAdjustPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propAdjustPad_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::BaseConvolutionLayer::adjust_pads() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:214
// ("cv::dnn::BaseConvolutionLayer::adjust_pads", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propAdjust_pads_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::setAdjust_pads(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:214
// ("cv::dnn::BaseConvolutionLayer::setAdjust_pads", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propAdjust_pads_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::BaseConvolutionLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
// ("cv::dnn::BaseConvolutionLayer::kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propKernel_size_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
// ("cv::dnn::BaseConvolutionLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propKernel_size_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::BaseConvolutionLayer::strides() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
// ("cv::dnn::BaseConvolutionLayer::strides", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propStrides_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
// ("cv::dnn::BaseConvolutionLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propStrides_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::BaseConvolutionLayer::dilations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
// ("cv::dnn::BaseConvolutionLayer::dilations", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propDilations_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::setDilations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:215
// ("cv::dnn::BaseConvolutionLayer::setDilations", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propDilations_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::BaseConvolutionLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
// ("cv::dnn::BaseConvolutionLayer::pads_begin", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPads_begin_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
// ("cv::dnn::BaseConvolutionLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPads_begin_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::BaseConvolutionLayer::pads_end() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
// ("cv::dnn::BaseConvolutionLayer::pads_end", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPads_end_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:216
// ("cv::dnn::BaseConvolutionLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPads_end_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::BaseConvolutionLayer::padMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:217
// ("cv::dnn::BaseConvolutionLayer::padMode", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPadMode_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:217
// ("cv::dnn::BaseConvolutionLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propPadMode_const_String(instance: *mut c_void, val: *const c_char);
// cv::dnn::BaseConvolutionLayer::numOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:218
// ("cv::dnn::BaseConvolutionLayer::numOutput", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propNumOutput_const(instance: *const c_void) -> i32;
// cv::dnn::BaseConvolutionLayer::setNumOutput(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:218
// ("cv::dnn::BaseConvolutionLayer::setNumOutput", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_BaseConvolutionLayer_propNumOutput_const_int(instance: *mut c_void, val: i32);
// cv::dnn::BaseConvolutionLayer::to_Algorithm() generated
// ("cv::dnn::BaseConvolutionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::to_Layer() generated
// ("cv::dnn::BaseConvolutionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BaseConvolutionLayer::delete() generated
// ("cv::dnn::BaseConvolutionLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BaseConvolutionLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:540
// ("cv::dnn::BatchNormLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_BatchNormLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::BatchNormLayer::hasWeights() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
// ("cv::dnn::BatchNormLayer::hasWeights", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BatchNormLayer_propHasWeights_const(instance: *const c_void) -> bool;
// cv::dnn::BatchNormLayer::setHasWeights(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
// ("cv::dnn::BatchNormLayer::setHasWeights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_BatchNormLayer_propHasWeights_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::BatchNormLayer::hasBias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
// ("cv::dnn::BatchNormLayer::hasBias", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BatchNormLayer_propHasBias_const(instance: *const c_void) -> bool;
// cv::dnn::BatchNormLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:537
// ("cv::dnn::BatchNormLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_BatchNormLayer_propHasBias_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::BatchNormLayer::epsilon() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:538
// ("cv::dnn::BatchNormLayer::epsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BatchNormLayer_propEpsilon_const(instance: *const c_void) -> f32;
// cv::dnn::BatchNormLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:538
// ("cv::dnn::BatchNormLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_BatchNormLayer_propEpsilon_const_float(instance: *mut c_void, val: f32);
// cv::dnn::BatchNormLayer::to_ActivationLayer() generated
// ("cv::dnn::BatchNormLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BatchNormLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BatchNormLayer::to_Algorithm() generated
// ("cv::dnn::BatchNormLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BatchNormLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BatchNormLayer::to_Layer() generated
// ("cv::dnn::BatchNormLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BatchNormLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BatchNormLayer::delete() generated
// ("cv::dnn::BatchNormLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BatchNormLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:77
// ("cv::dnn::BlankLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_BlankLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::BlankLayer::defaultNew() generated
// ("cv::dnn::BlankLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_BlankLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::BlankLayer::to_Algorithm() generated
// ("cv::dnn::BlankLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BlankLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BlankLayer::to_Layer() generated
// ("cv::dnn::BlankLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BlankLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::BlankLayer::delete() generated
// ("cv::dnn::BlankLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_BlankLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:450
// ("cv::dnn::ChannelsPReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ChannelsPReLULayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ChannelsPReLULayer::to_ActivationLayer() generated
// ("cv::dnn::ChannelsPReLULayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ChannelsPReLULayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ChannelsPReLULayer::to_Algorithm() generated
// ("cv::dnn::ChannelsPReLULayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ChannelsPReLULayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ChannelsPReLULayer::to_Layer() generated
// ("cv::dnn::ChannelsPReLULayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ChannelsPReLULayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ChannelsPReLULayer::delete() generated
// ("cv::dnn::ChannelsPReLULayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ChannelsPReLULayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:322
// ("cv::dnn::ConcatLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ConcatLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ConcatLayer::defaultNew() generated
// ("cv::dnn::ConcatLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ConcatLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ConcatLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:313
// ("cv::dnn::ConcatLayer::axis", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ConcatLayer_propAxis_const(instance: *const c_void) -> i32;
// cv::dnn::ConcatLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:313
// ("cv::dnn::ConcatLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_ConcatLayer_propAxis_const_int(instance: *mut c_void, val: i32);
// cv::dnn::ConcatLayer::padding() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:320
// ("cv::dnn::ConcatLayer::padding", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ConcatLayer_propPadding_const(instance: *const c_void) -> bool;
// cv::dnn::ConcatLayer::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:320
// ("cv::dnn::ConcatLayer::setPadding", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_ConcatLayer_propPadding_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::ConcatLayer::to_Algorithm() generated
// ("cv::dnn::ConcatLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConcatLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ConcatLayer::to_Layer() generated
// ("cv::dnn::ConcatLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConcatLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ConcatLayer::delete() generated
// ("cv::dnn::ConcatLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConcatLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:86
// ("cv::dnn::ConstLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ConstLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ConstLayer::defaultNew() generated
// ("cv::dnn::ConstLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ConstLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ConstLayer::to_Algorithm() generated
// ("cv::dnn::ConstLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConstLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ConstLayer::to_Layer() generated
// ("cv::dnn::ConstLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConstLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ConstLayer::delete() generated
// ("cv::dnn::ConstLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConstLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:224
// ("cv::dnn::ConvolutionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ConvolutionLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ConvolutionLayer::defaultNew() generated
// ("cv::dnn::ConvolutionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ConvolutionLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ConvolutionLayer::to_Algorithm() generated
// ("cv::dnn::ConvolutionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConvolutionLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ConvolutionLayer::to_BaseConvolutionLayer() generated
// ("cv::dnn::ConvolutionLayer::to_BaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConvolutionLayer_to_BaseConvolutionLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ConvolutionLayer::to_Layer() generated
// ("cv::dnn::ConvolutionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConvolutionLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ConvolutionLayer::delete() generated
// ("cv::dnn::ConvolutionLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ConvolutionLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:577
// ("cv::dnn::CorrelationLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_CorrelationLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::CorrelationLayer::defaultNew() generated
// ("cv::dnn::CorrelationLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_CorrelationLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::CorrelationLayer::to_Algorithm() generated
// ("cv::dnn::CorrelationLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CorrelationLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::CorrelationLayer::to_Layer() generated
// ("cv::dnn::CorrelationLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CorrelationLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::CorrelationLayer::delete() generated
// ("cv::dnn::CorrelationLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CorrelationLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:689
// ("cv::dnn::CropAndResizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_CropAndResizeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::CropAndResizeLayer::defaultNew() generated
// ("cv::dnn::CropAndResizeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_CropAndResizeLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::CropAndResizeLayer::to_Algorithm() generated
// ("cv::dnn::CropAndResizeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CropAndResizeLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::CropAndResizeLayer::to_Layer() generated
// ("cv::dnn::CropAndResizeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CropAndResizeLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::CropAndResizeLayer::delete() generated
// ("cv::dnn::CropAndResizeLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CropAndResizeLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:518
// ("cv::dnn::CropLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_CropLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::CropLayer::defaultNew() generated
// ("cv::dnn::CropLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_CropLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::CropLayer::to_Algorithm() generated
// ("cv::dnn::CropLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CropLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::CropLayer::to_Layer() generated
// ("cv::dnn::CropLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CropLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::CropLayer::delete() generated
// ("cv::dnn::CropLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_CropLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:571
// ("cv::dnn::DataAugmentationLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_DataAugmentationLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DataAugmentationLayer::defaultNew() generated
// ("cv::dnn::DataAugmentationLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DataAugmentationLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::DataAugmentationLayer::to_Algorithm() generated
// ("cv::dnn::DataAugmentationLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DataAugmentationLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::DataAugmentationLayer::to_Layer() generated
// ("cv::dnn::DataAugmentationLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DataAugmentationLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::DataAugmentationLayer::delete() generated
// ("cv::dnn::DataAugmentationLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DataAugmentationLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:230
// ("cv::dnn::DeconvolutionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_DeconvolutionLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DeconvolutionLayer::defaultNew() generated
// ("cv::dnn::DeconvolutionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DeconvolutionLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::DeconvolutionLayer::to_Algorithm() generated
// ("cv::dnn::DeconvolutionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DeconvolutionLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::DeconvolutionLayer::to_BaseConvolutionLayer() generated
// ("cv::dnn::DeconvolutionLayer::to_BaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DeconvolutionLayer_to_BaseConvolutionLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::DeconvolutionLayer::to_Layer() generated
// ("cv::dnn::DeconvolutionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DeconvolutionLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::DeconvolutionLayer::delete() generated
// ("cv::dnn::DeconvolutionLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DeconvolutionLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:621
// ("cv::dnn::DetectionOutputLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_DetectionOutputLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DetectionOutputLayer::defaultNew() generated
// ("cv::dnn::DetectionOutputLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DetectionOutputLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::DetectionOutputLayer::to_Algorithm() generated
// ("cv::dnn::DetectionOutputLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DetectionOutputLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::DetectionOutputLayer::to_Layer() generated
// ("cv::dnn::DetectionOutputLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DetectionOutputLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::DetectionOutputLayer::delete() generated
// ("cv::dnn::DetectionOutputLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DetectionOutputLayer_delete(instance: *mut c_void);
// has(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:122
// ("cv::dnn::Dict::has", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Dict_has_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<bool>);
// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:125
// ("cv::dnn::Dict::ptr", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Dict_ptr_const_StringR(instance: *mut c_void, key: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// ptr(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:128
// ("cv::dnn::Dict::ptr", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Dict_ptr_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// get(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:131
// ("cv::dnn::Dict::get", vec![(pred!(const, ["key"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Dict_get_const_const_StringR(instance: *const c_void, key: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::Dict::set(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_Dict_set_const_cv_String_const_StringR_const_StringR(instance: *mut c_void, key: *const c_char, value: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::Dict::set(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const cv::dnn::DictValue*"]), _)]),
pub fn cv_dnn_Dict_set_const_cv_dnn_DictValue_const_StringR_const_DictValueR(instance: *mut c_void, key: *const c_char, value: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const double*"]), _)]),
pub fn cv_dnn_Dict_set_const_double_const_StringR_const_doubleR(instance: *mut c_void, key: *const c_char, value: *const f64, ocvrs_return: *mut Result<f64>);
// cv::dnn::Dict::set(InString, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:143
// ("cv::dnn::Dict::set", vec![(pred!(mut, ["key", "value"], ["const cv::String*", "const int64_t*"]), _)]),
pub fn cv_dnn_Dict_set_const_int64_t_const_StringR_const_int64_tR(instance: *mut c_void, key: *const c_char, value: *const i64, ocvrs_return: *mut Result<i64>);
// erase(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:146
// ("cv::dnn::Dict::erase", vec![(pred!(mut, ["key"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Dict_erase_const_StringR(instance: *mut c_void, key: *const c_char, ocvrs_return: *mut Result<()>);
// cv::dnn::Dict::defaultNew() generated
// ("cv::dnn::Dict::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Dict_defaultNew_const() -> *mut c_void;
// cv::dnn::Dict::delete() generated
// ("cv::dnn::Dict::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Dict_delete(instance: *mut c_void);
// DictValue(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:62
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["r"], ["const cv::dnn::DictValue*"]), _)]),
pub fn cv_dnn_DictValue_DictValue_const_DictValueR(r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// DictValue(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:63
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["bool"]), _)]),
pub fn cv_dnn_DictValue_DictValue_bool(i: bool, ocvrs_return: *mut Result<*mut c_void>);
// DictValue(int64)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:64
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int64_t"]), _)]),
pub fn cv_dnn_DictValue_DictValue_int64_t(i: i64, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DictValue::DictValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:64
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DictValue_DictValue(ocvrs_return: *mut Result<*mut c_void>);
// DictValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:65
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["i"], ["int"]), _)]),
pub fn cv_dnn_DictValue_DictValue_int(i: i32, ocvrs_return: *mut Result<*mut c_void>);
// DictValue(unsigned int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:66
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["unsigned int"]), _)]),
pub fn cv_dnn_DictValue_DictValue_unsigned_int(p: u32, ocvrs_return: *mut Result<*mut c_void>);
// DictValue(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:67
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["p"], ["double"]), _)]),
pub fn cv_dnn_DictValue_DictValue_double(p: f64, ocvrs_return: *mut Result<*mut c_void>);
// DictValue(const char *)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:69
// ("cv::dnn::DictValue::DictValue", vec![(pred!(mut, ["s"], ["const char*"]), _)]),
pub fn cv_dnn_DictValue_DictValue_const_charX(s: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv_dnn_DictValue_get_cv_String_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_get_cv_String_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv_dnn_DictValue_get_double_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<f64>);
// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_get_double_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv_dnn_DictValue_get_int_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<i32>);
// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_get_int_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::dnn::DictValue::get(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv_dnn_DictValue_get_int64_t_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<i64>);
// cv::dnn::DictValue::get() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:79
// ("cv::dnn::DictValue::get", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_get_int64_t_const(instance: *const c_void, ocvrs_return: *mut Result<i64>);
// size()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:81
// ("cv::dnn::DictValue::size", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_size_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// isInt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:83
// ("cv::dnn::DictValue::isInt", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_isInt_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isString()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:84
// ("cv::dnn::DictValue::isString", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_isString_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isReal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:85
// ("cv::dnn::DictValue::isReal", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_isReal_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// getIntValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:87
// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv_dnn_DictValue_getIntValue_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<i32>);
// cv::dnn::DictValue::getIntValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:87
// ("cv::dnn::DictValue::getIntValue", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_getIntValue_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getRealValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:88
// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv_dnn_DictValue_getRealValue_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<f64>);
// cv::dnn::DictValue::getRealValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:88
// ("cv::dnn::DictValue::getRealValue", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_getRealValue_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// getStringValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:89
// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, ["idx"], ["int"]), _)]),
pub fn cv_dnn_DictValue_getStringValue_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::DictValue::getStringValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:89
// ("cv::dnn::DictValue::getStringValue", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_DictValue_getStringValue_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator=(const DictValue &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dict.hpp:91
// ("cv::dnn::DictValue::operator=", vec![(pred!(mut, ["r"], ["const cv::dnn::DictValue*"]), _)]),
pub fn cv_dnn_DictValue_operatorST_const_DictValueR(instance: *mut c_void, r: *const c_void, ocvrs_return: *mut Result<()>);
// cv::dnn::DictValue::delete() generated
// ("cv::dnn::DictValue::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_DictValue_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:458
// ("cv::dnn::ELULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ELULayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ELULayer::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:456
// ("cv::dnn::ELULayer::alpha", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ELULayer_propAlpha_const(instance: *const c_void) -> f32;
// cv::dnn::ELULayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:456
// ("cv::dnn::ELULayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_ELULayer_propAlpha_const_float(instance: *mut c_void, val: f32);
// cv::dnn::ELULayer::to_ActivationLayer() generated
// ("cv::dnn::ELULayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ELULayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ELULayer::to_Algorithm() generated
// ("cv::dnn::ELULayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ELULayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ELULayer::to_Layer() generated
// ("cv::dnn::ELULayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ELULayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ELULayer::delete() generated
// ("cv::dnn::ELULayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ELULayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:531
// ("cv::dnn::EltwiseLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_EltwiseLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::EltwiseLayer::defaultNew() generated
// ("cv::dnn::EltwiseLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_EltwiseLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::EltwiseLayer::to_Algorithm() generated
// ("cv::dnn::EltwiseLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_EltwiseLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::EltwiseLayer::to_Layer() generated
// ("cv::dnn::EltwiseLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_EltwiseLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::EltwiseLayer::delete() generated
// ("cv::dnn::EltwiseLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_EltwiseLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:510
// ("cv::dnn::ExpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ExpLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ExpLayer::base() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
// ("cv::dnn::ExpLayer::base", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ExpLayer_propBase_const(instance: *const c_void) -> f32;
// cv::dnn::ExpLayer::setBase(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
// ("cv::dnn::ExpLayer::setBase", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_ExpLayer_propBase_const_float(instance: *mut c_void, val: f32);
// cv::dnn::ExpLayer::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
// ("cv::dnn::ExpLayer::scale", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ExpLayer_propScale_const(instance: *const c_void) -> f32;
// cv::dnn::ExpLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
// ("cv::dnn::ExpLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_ExpLayer_propScale_const_float(instance: *mut c_void, val: f32);
// cv::dnn::ExpLayer::shift() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
// ("cv::dnn::ExpLayer::shift", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ExpLayer_propShift_const(instance: *const c_void) -> f32;
// cv::dnn::ExpLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:508
// ("cv::dnn::ExpLayer::setShift", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_ExpLayer_propShift_const_float(instance: *mut c_void, val: f32);
// cv::dnn::ExpLayer::to_ActivationLayer() generated
// ("cv::dnn::ExpLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ExpLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ExpLayer::to_Algorithm() generated
// ("cv::dnn::ExpLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ExpLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ExpLayer::to_Layer() generated
// ("cv::dnn::ExpLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ExpLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ExpLayer::delete() generated
// ("cv::dnn::ExpLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ExpLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:307
// ("cv::dnn::FlattenLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_FlattenLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::FlattenLayer::defaultNew() generated
// ("cv::dnn::FlattenLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_FlattenLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::FlattenLayer::to_Algorithm() generated
// ("cv::dnn::FlattenLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_FlattenLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::FlattenLayer::to_Layer() generated
// ("cv::dnn::FlattenLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_FlattenLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::FlattenLayer::delete() generated
// ("cv::dnn::FlattenLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_FlattenLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:589
// ("cv::dnn::FlowWarpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_FlowWarpLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::FlowWarpLayer::defaultNew() generated
// ("cv::dnn::FlowWarpLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_FlowWarpLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::FlowWarpLayer::to_Algorithm() generated
// ("cv::dnn::FlowWarpLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_FlowWarpLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::FlowWarpLayer::to_Layer() generated
// ("cv::dnn::FlowWarpLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_FlowWarpLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::FlowWarpLayer::delete() generated
// ("cv::dnn::FlowWarpLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_FlowWarpLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:281
// ("cv::dnn::InnerProductLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_InnerProductLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::InnerProductLayer::defaultNew() generated
// ("cv::dnn::InnerProductLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_InnerProductLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::InnerProductLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:280
// ("cv::dnn::InnerProductLayer::axis", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_InnerProductLayer_propAxis_const(instance: *const c_void) -> i32;
// cv::dnn::InnerProductLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:280
// ("cv::dnn::InnerProductLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_InnerProductLayer_propAxis_const_int(instance: *mut c_void, val: i32);
// cv::dnn::InnerProductLayer::to_Algorithm() generated
// ("cv::dnn::InnerProductLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_InnerProductLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::InnerProductLayer::to_Layer() generated
// ("cv::dnn::InnerProductLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_InnerProductLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::InnerProductLayer::delete() generated
// ("cv::dnn::InnerProductLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_InnerProductLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:677
// ("cv::dnn::InterpLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_InterpLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::InterpLayer::defaultNew() generated
// ("cv::dnn::InterpLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_InterpLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::InterpLayer::to_Algorithm() generated
// ("cv::dnn::InterpLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_InterpLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::InterpLayer::to_Layer() generated
// ("cv::dnn::InterpLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_InterpLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::InterpLayer::delete() generated
// ("cv::dnn::InterpLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_InterpLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:242
// ("cv::dnn::LRNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_LRNLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::LRNLayer::defaultNew() generated
// ("cv::dnn::LRNLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LRNLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::LRNLayer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:236
// ("cv::dnn::LRNLayer::type", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LRNLayer_propType_const(instance: *const c_void) -> i32;
// cv::dnn::LRNLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:236
// ("cv::dnn::LRNLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_LRNLayer_propType_const_int(instance: *mut c_void, val: i32);
// cv::dnn::LRNLayer::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:238
// ("cv::dnn::LRNLayer::size", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LRNLayer_propSize_const(instance: *const c_void) -> i32;
// cv::dnn::LRNLayer::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:238
// ("cv::dnn::LRNLayer::setSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_LRNLayer_propSize_const_int(instance: *mut c_void, val: i32);
// cv::dnn::LRNLayer::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
// ("cv::dnn::LRNLayer::alpha", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LRNLayer_propAlpha_const(instance: *const c_void) -> f32;
// cv::dnn::LRNLayer::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
// ("cv::dnn::LRNLayer::setAlpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_LRNLayer_propAlpha_const_float(instance: *mut c_void, val: f32);
// cv::dnn::LRNLayer::beta() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
// ("cv::dnn::LRNLayer::beta", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LRNLayer_propBeta_const(instance: *const c_void) -> f32;
// cv::dnn::LRNLayer::setBeta(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
// ("cv::dnn::LRNLayer::setBeta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_LRNLayer_propBeta_const_float(instance: *mut c_void, val: f32);
// cv::dnn::LRNLayer::bias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
// ("cv::dnn::LRNLayer::bias", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LRNLayer_propBias_const(instance: *const c_void) -> f32;
// cv::dnn::LRNLayer::setBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:239
// ("cv::dnn::LRNLayer::setBias", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_LRNLayer_propBias_const_float(instance: *mut c_void, val: f32);
// cv::dnn::LRNLayer::normBySize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:240
// ("cv::dnn::LRNLayer::normBySize", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LRNLayer_propNormBySize_const(instance: *const c_void) -> bool;
// cv::dnn::LRNLayer::setNormBySize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:240
// ("cv::dnn::LRNLayer::setNormBySize", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_LRNLayer_propNormBySize_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::LRNLayer::to_Algorithm() generated
// ("cv::dnn::LRNLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LRNLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::LRNLayer::to_Layer() generated
// ("cv::dnn::LRNLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LRNLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::LRNLayer::delete() generated
// ("cv::dnn::LRNLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LRNLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:94
// ("cv::dnn::LSTMLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_LSTMLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setWeights(const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:128
// ("cv::dnn::LSTMLayer::setWeights", vec![(pred!(mut, ["Wh", "Wx", "b"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_dnn_LSTMLayer_setWeights_const_MatR_const_MatR_const_MatR(instance: *mut c_void, wh: *const c_void, wx: *const c_void, b: *const c_void, ocvrs_return: *mut Result<()>);
// setOutShape(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:134
// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, ["outTailShape"], ["const cv::dnn::MatShape*"]), _)]),
pub fn cv_dnn_LSTMLayer_setOutShape_const_MatShapeR(instance: *mut c_void, out_tail_shape: *const c_void, ocvrs_return: *mut Result<()>);
// cv::dnn::LSTMLayer::setOutShape() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:134
// ("cv::dnn::LSTMLayer::setOutShape", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LSTMLayer_setOutShape(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setUseTimstampsDim(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:145
// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, ["use"], ["bool"]), _)]),
pub fn cv_dnn_LSTMLayer_setUseTimstampsDim_bool(instance: *mut c_void, use_: bool, ocvrs_return: *mut Result<()>);
// cv::dnn::LSTMLayer::setUseTimstampsDim() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:145
// ("cv::dnn::LSTMLayer::setUseTimstampsDim", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LSTMLayer_setUseTimstampsDim(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setProduceCellOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:151
// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
pub fn cv_dnn_LSTMLayer_setProduceCellOutput_bool(instance: *mut c_void, produce: bool, ocvrs_return: *mut Result<()>);
// cv::dnn::LSTMLayer::setProduceCellOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:151
// ("cv::dnn::LSTMLayer::setProduceCellOutput", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LSTMLayer_setProduceCellOutput(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:164
// ("cv::dnn::LSTMLayer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
pub fn cv_dnn_LSTMLayer_inputNameToIndex_String(instance: *mut c_void, input_name: *const c_char, ocvrs_return: *mut Result<i32>);
// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:165
// ("cv::dnn::LSTMLayer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
pub fn cv_dnn_LSTMLayer_outputNameToIndex_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<i32>);
// cv::dnn::LSTMLayer::to_Algorithm() generated
// ("cv::dnn::LSTMLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LSTMLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::LSTMLayer::to_Layer() generated
// ("cv::dnn::LSTMLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LSTMLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::LSTMLayer::delete() generated
// ("cv::dnn::LSTMLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LSTMLayer_delete(instance: *mut c_void);
// finalize(InputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:212
// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, inputs: *const c_void, outputs: *const c_void, ocvrs_return: *mut Result<()>);
// forward(std::vector<Mat *> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:221
// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["input", "output", "internals"], ["std::vector<cv::Mat*>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_dnn_Layer_forward_vectorLMatXGR_vectorLMatGR_vectorLMatGR(instance: *mut c_void, input: *mut c_void, output: *mut c_void, internals: *mut c_void, ocvrs_return: *mut Result<()>);
// forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:228
// ("cv::dnn::Layer::forward", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_Layer_forward_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, inputs: *const c_void, outputs: *const c_void, internals: *const c_void, ocvrs_return: *mut Result<()>);
// forward_fallback(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:235
// ("cv::dnn::Layer::forward_fallback", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_Layer_forward_fallback_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, inputs: *const c_void, outputs: *const c_void, internals: *const c_void, ocvrs_return: *mut Result<()>);
// finalize(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:242
// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs", "outputs"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_dnn_Layer_finalize_const_vectorLMatGR_vectorLMatGR(instance: *mut c_void, inputs: *const c_void, outputs: *mut c_void, ocvrs_return: *mut Result<()>);
// finalize(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:248
// ("cv::dnn::Layer::finalize", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_dnn_Layer_finalize_const_vectorLMatGR(instance: *mut c_void, inputs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// run(const std::vector<Mat> &, std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:253
// ("cv::dnn::Layer::run", vec![(pred!(mut, ["inputs", "outputs", "internals"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_dnn_Layer_run_const_vectorLMatGR_vectorLMatGR_vectorLMatGR(instance: *mut c_void, inputs: *const c_void, outputs: *mut c_void, internals: *mut c_void, ocvrs_return: *mut Result<()>);
// inputNameToIndex(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:262
// ("cv::dnn::Layer::inputNameToIndex", vec![(pred!(mut, ["inputName"], ["cv::String"]), _)]),
pub fn cv_dnn_Layer_inputNameToIndex_String(instance: *mut c_void, input_name: *const c_char, ocvrs_return: *mut Result<i32>);
// outputNameToIndex(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:266
// ("cv::dnn::Layer::outputNameToIndex", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Layer_outputNameToIndex_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<i32>);
// supportBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:273
// ("cv::dnn::Layer::supportBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
pub fn cv_dnn_Layer_supportBackend_int(instance: *mut c_void, backend_id: i32, ocvrs_return: *mut Result<bool>);
// initHalide(const std::vector<Ptr<BackendWrapper>> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:285
// ("cv::dnn::Layer::initHalide", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*"]), _)]),
pub fn cv_dnn_Layer_initHalide_const_vectorLPtrLBackendWrapperGGR(instance: *mut c_void, inputs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// initNgraph(const std::vector<Ptr<BackendWrapper>> &, const std::vector<Ptr<BackendNode>> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:287
// ("cv::dnn::Layer::initNgraph", vec![(pred!(mut, ["inputs", "nodes"], ["const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>*", "const std::vector<cv::Ptr<cv::dnn::BackendNode>>*"]), _)]),
pub fn cv_dnn_Layer_initNgraph_const_vectorLPtrLBackendWrapperGGR_const_vectorLPtrLBackendNodeGGR(instance: *mut c_void, inputs: *const c_void, nodes: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// applyHalideScheduler(Ptr<BackendNode> &, const std::vector<Mat *> &, const std::vector<Mat> &, int)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:300
// ("cv::dnn::Layer::applyHalideScheduler", vec![(pred!(const, ["node", "inputs", "outputs", "targetId"], ["cv::Ptr<cv::dnn::BackendNode>*", "const std::vector<cv::Mat*>*", "const std::vector<cv::Mat>*", "int"]), _)]),
pub fn cv_dnn_Layer_applyHalideScheduler_const_PtrLBackendNodeGR_const_vectorLMatXGR_const_vectorLMatGR_int(instance: *const c_void, node: *mut c_void, inputs: *const c_void, outputs: *const c_void, target_id: i32, ocvrs_return: *mut Result<()>);
// tryAttach(const Ptr<BackendNode> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:314
// ("cv::dnn::Layer::tryAttach", vec![(pred!(mut, ["node"], ["const cv::Ptr<cv::dnn::BackendNode>*"]), _)]),
pub fn cv_dnn_Layer_tryAttach_const_PtrLBackendNodeGR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setActivation(const Ptr<ActivationLayer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:322
// ("cv::dnn::Layer::setActivation", vec![(pred!(mut, ["layer"], ["const cv::Ptr<cv::dnn::ActivationLayer>*"]), _)]),
pub fn cv_dnn_Layer_setActivation_const_PtrLActivationLayerGR(instance: *mut c_void, layer: *const c_void, ocvrs_return: *mut Result<bool>);
// tryFuse(Ptr<Layer> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:329
// ("cv::dnn::Layer::tryFuse", vec![(pred!(mut, ["top"], ["cv::Ptr<cv::dnn::Layer>*"]), _)]),
pub fn cv_dnn_Layer_tryFuse_PtrLLayerGR(instance: *mut c_void, top: *mut c_void, ocvrs_return: *mut Result<bool>);
// getScaleShift(Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:344
// ("cv::dnn::Layer::getScaleShift", vec![(pred!(const, ["scale", "shift"], ["cv::Mat*", "cv::Mat*"]), _)]),
pub fn cv_dnn_Layer_getScaleShift_const_MatR_MatR(instance: *const c_void, scale: *mut c_void, shift: *mut c_void, ocvrs_return: *mut Result<()>);
// unsetAttached()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:349
// ("cv::dnn::Layer::unsetAttached", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_unsetAttached(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getMemoryShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:351
// ("cv::dnn::Layer::getMemoryShapes", vec![(pred!(const, ["inputs", "requiredOutputs", "outputs", "internals"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
pub fn cv_dnn_Layer_getMemoryShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(instance: *const c_void, inputs: *const c_void, required_outputs: i32, outputs: *mut c_void, internals: *mut c_void, ocvrs_return: *mut Result<bool>);
// getFLOPS(const std::vector<MatShape> &, const std::vector<MatShape> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:356
// ("cv::dnn::Layer::getFLOPS", vec![(pred!(const, ["inputs", "outputs"], ["const std::vector<cv::dnn::MatShape>*", "const std::vector<cv::dnn::MatShape>*"]), _)]),
pub fn cv_dnn_Layer_getFLOPS_const_const_vectorLMatShapeGR_const_vectorLMatShapeGR(instance: *const c_void, inputs: *const c_void, outputs: *const c_void, ocvrs_return: *mut Result<i64>);
// updateMemoryShapes(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:359
// ("cv::dnn::Layer::updateMemoryShapes", vec![(pred!(mut, ["inputs"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
pub fn cv_dnn_Layer_updateMemoryShapes_const_vectorLMatShapeGR(instance: *mut c_void, inputs: *const c_void, ocvrs_return: *mut Result<bool>);
// Layer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:365
// ("cv::dnn::Layer::Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_Layer(ocvrs_return: *mut Result<*mut c_void>);
// Layer(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:366
// ("cv::dnn::Layer::Layer", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_Layer_Layer_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setParamsFrom(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:367
// ("cv::dnn::Layer::setParamsFrom", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_Layer_setParamsFrom_const_LayerParamsR(instance: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<()>);
// cv::dnn::Layer::blobs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:192
// ("cv::dnn::Layer::blobs", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Layer_propBlobs_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::Layer::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:192
// ("cv::dnn::Layer::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_dnn_Layer_propBlobs_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::dnn::Layer::name() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:361
// ("cv::dnn::Layer::name", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Layer_propName_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::Layer::setName(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:361
// ("cv::dnn::Layer::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_dnn_Layer_propName_const_String(instance: *mut c_void, val: *const c_char);
// cv::dnn::Layer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:362
// ("cv::dnn::Layer::type", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Layer_propType_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::Layer::setType(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:362
// ("cv::dnn::Layer::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_dnn_Layer_propType_const_String(instance: *mut c_void, val: *const c_char);
// cv::dnn::Layer::preferableTarget() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:363
// ("cv::dnn::Layer::preferableTarget", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Layer_propPreferableTarget_const(instance: *const c_void) -> i32;
// cv::dnn::Layer::setPreferableTarget(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:363
// ("cv::dnn::Layer::setPreferableTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_Layer_propPreferableTarget_const_int(instance: *mut c_void, val: i32);
// cv::dnn::Layer::to_AbsLayer() generated
// ("cv::dnn::Layer::to_AbsLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_AbsLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_AccumLayer() generated
// ("cv::dnn::Layer::to_AccumLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_AccumLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ActivationLayer() generated
// ("cv::dnn::Layer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_BNLLLayer() generated
// ("cv::dnn::Layer::to_BNLLLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_BNLLLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_BaseConvolutionLayer() generated
// ("cv::dnn::Layer::to_BaseConvolutionLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_BaseConvolutionLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_BatchNormLayer() generated
// ("cv::dnn::Layer::to_BatchNormLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_BatchNormLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_BlankLayer() generated
// ("cv::dnn::Layer::to_BlankLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_BlankLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ChannelsPReLULayer() generated
// ("cv::dnn::Layer::to_ChannelsPReLULayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ChannelsPReLULayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ConcatLayer() generated
// ("cv::dnn::Layer::to_ConcatLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ConcatLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ConstLayer() generated
// ("cv::dnn::Layer::to_ConstLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ConstLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ConvolutionLayer() generated
// ("cv::dnn::Layer::to_ConvolutionLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ConvolutionLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_CorrelationLayer() generated
// ("cv::dnn::Layer::to_CorrelationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_CorrelationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_CropAndResizeLayer() generated
// ("cv::dnn::Layer::to_CropAndResizeLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_CropAndResizeLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_CropLayer() generated
// ("cv::dnn::Layer::to_CropLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_CropLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_DataAugmentationLayer() generated
// ("cv::dnn::Layer::to_DataAugmentationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_DataAugmentationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_DeconvolutionLayer() generated
// ("cv::dnn::Layer::to_DeconvolutionLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_DeconvolutionLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_DetectionOutputLayer() generated
// ("cv::dnn::Layer::to_DetectionOutputLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_DetectionOutputLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ELULayer() generated
// ("cv::dnn::Layer::to_ELULayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ELULayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_EltwiseLayer() generated
// ("cv::dnn::Layer::to_EltwiseLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_EltwiseLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ExpLayer() generated
// ("cv::dnn::Layer::to_ExpLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ExpLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_FlattenLayer() generated
// ("cv::dnn::Layer::to_FlattenLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_FlattenLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_FlowWarpLayer() generated
// ("cv::dnn::Layer::to_FlowWarpLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_FlowWarpLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_InnerProductLayer() generated
// ("cv::dnn::Layer::to_InnerProductLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_InnerProductLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_InterpLayer() generated
// ("cv::dnn::Layer::to_InterpLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_InterpLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_LRNLayer() generated
// ("cv::dnn::Layer::to_LRNLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_LRNLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_LSTMLayer() generated
// ("cv::dnn::Layer::to_LSTMLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_LSTMLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_MVNLayer() generated
// ("cv::dnn::Layer::to_MVNLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_MVNLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_MaxUnpoolLayer() generated
// ("cv::dnn::Layer::to_MaxUnpoolLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_MaxUnpoolLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_MishLayer() generated
// ("cv::dnn::Layer::to_MishLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_MishLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_NormalizeBBoxLayer() generated
// ("cv::dnn::Layer::to_NormalizeBBoxLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_NormalizeBBoxLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_PaddingLayer() generated
// ("cv::dnn::Layer::to_PaddingLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_PaddingLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_PermuteLayer() generated
// ("cv::dnn::Layer::to_PermuteLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_PermuteLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_PoolingLayer() generated
// ("cv::dnn::Layer::to_PoolingLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_PoolingLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_PowerLayer() generated
// ("cv::dnn::Layer::to_PowerLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_PowerLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_PriorBoxLayer() generated
// ("cv::dnn::Layer::to_PriorBoxLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_PriorBoxLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ProposalLayer() generated
// ("cv::dnn::Layer::to_ProposalLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ProposalLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_RNNLayer() generated
// ("cv::dnn::Layer::to_RNNLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_RNNLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ReLU6Layer() generated
// ("cv::dnn::Layer::to_ReLU6Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ReLU6Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ReLULayer() generated
// ("cv::dnn::Layer::to_ReLULayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ReLULayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_RegionLayer() generated
// ("cv::dnn::Layer::to_RegionLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_RegionLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ReorgLayer() generated
// ("cv::dnn::Layer::to_ReorgLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ReorgLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ReshapeLayer() generated
// ("cv::dnn::Layer::to_ReshapeLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ReshapeLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ResizeLayer() generated
// ("cv::dnn::Layer::to_ResizeLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ResizeLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ScaleLayer() generated
// ("cv::dnn::Layer::to_ScaleLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ScaleLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ShiftLayer() generated
// ("cv::dnn::Layer::to_ShiftLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ShiftLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_ShuffleChannelLayer() generated
// ("cv::dnn::Layer::to_ShuffleChannelLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_ShuffleChannelLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_SigmoidLayer() generated
// ("cv::dnn::Layer::to_SigmoidLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_SigmoidLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_SliceLayer() generated
// ("cv::dnn::Layer::to_SliceLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_SliceLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_SoftmaxLayer() generated
// ("cv::dnn::Layer::to_SoftmaxLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_SoftmaxLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_SplitLayer() generated
// ("cv::dnn::Layer::to_SplitLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_SplitLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_SwishLayer() generated
// ("cv::dnn::Layer::to_SwishLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_SwishLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_TanHLayer() generated
// ("cv::dnn::Layer::to_TanHLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_TanHLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::to_Algorithm() generated
// ("cv::dnn::Layer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::Layer::delete() generated
// ("cv::dnn::Layer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Layer_delete(instance: *mut c_void);
// registerLayer(const String &, Constructor)(InString, Function) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:64
// ("cv::dnn::LayerFactory::registerLayer", vec![(pred!(mut, ["type", "constructor"], ["const cv::String*", "cv::dnn::LayerFactory::Constructor"]), _)]),
pub fn cv_dnn_LayerFactory_registerLayer_const_StringR_Constructor(typ: *const c_char, constructor: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>, ocvrs_return: *mut Result<()>);
// unregisterLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:67
// ("cv::dnn::LayerFactory::unregisterLayer", vec![(pred!(mut, ["type"], ["const cv::String*"]), _)]),
pub fn cv_dnn_LayerFactory_unregisterLayer_const_StringR(typ: *const c_char, ocvrs_return: *mut Result<()>);
// createLayerInstance(const String &, LayerParams &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/layer.hpp:74
// ("cv::dnn::LayerFactory::createLayerInstance", vec![(pred!(mut, ["type", "params"], ["const cv::String*", "cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_LayerFactory_createLayerInstance_const_StringR_LayerParamsR(typ: *const c_char, params: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::LayerFactory::delete() generated
// ("cv::dnn::LayerFactory::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LayerFactory_delete(instance: *mut c_void);
// cv::dnn::LayerParams::defaultNew() generated
// ("cv::dnn::LayerParams::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LayerParams_defaultNew_const() -> *mut c_void;
// cv::dnn::LayerParams::blobs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:116
// ("cv::dnn::LayerParams::blobs", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LayerParams_propBlobs_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::LayerParams::setBlobs(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:116
// ("cv::dnn::LayerParams::setBlobs", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_dnn_LayerParams_propBlobs_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::dnn::LayerParams::name() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:118
// ("cv::dnn::LayerParams::name", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LayerParams_propName_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::LayerParams::setName(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:118
// ("cv::dnn::LayerParams::setName", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_dnn_LayerParams_propName_const_String(instance: *mut c_void, val: *const c_char);
// cv::dnn::LayerParams::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:119
// ("cv::dnn::LayerParams::type", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_LayerParams_propType_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::LayerParams::setType(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:119
// ("cv::dnn::LayerParams::setType", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_dnn_LayerParams_propType_const_String(instance: *mut c_void, val: *const c_char);
// cv::dnn::LayerParams::to_Dict() generated
// ("cv::dnn::LayerParams::to_Dict", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LayerParams_to_Dict(instance: *mut c_void) -> *mut c_void;
// cv::dnn::LayerParams::delete() generated
// ("cv::dnn::LayerParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_LayerParams_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:290
// ("cv::dnn::MVNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_MVNLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::MVNLayer::defaultNew() generated
// ("cv::dnn::MVNLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MVNLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::MVNLayer::eps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:287
// ("cv::dnn::MVNLayer::eps", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MVNLayer_propEps_const(instance: *const c_void) -> f32;
// cv::dnn::MVNLayer::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:287
// ("cv::dnn::MVNLayer::setEps", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_MVNLayer_propEps_const_float(instance: *mut c_void, val: f32);
// cv::dnn::MVNLayer::normVariance() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
// ("cv::dnn::MVNLayer::normVariance", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MVNLayer_propNormVariance_const(instance: *const c_void) -> bool;
// cv::dnn::MVNLayer::setNormVariance(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
// ("cv::dnn::MVNLayer::setNormVariance", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_MVNLayer_propNormVariance_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::MVNLayer::acrossChannels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
// ("cv::dnn::MVNLayer::acrossChannels", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MVNLayer_propAcrossChannels_const(instance: *const c_void) -> bool;
// cv::dnn::MVNLayer::setAcrossChannels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:288
// ("cv::dnn::MVNLayer::setAcrossChannels", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_MVNLayer_propAcrossChannels_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::MVNLayer::to_Algorithm() generated
// ("cv::dnn::MVNLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MVNLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::MVNLayer::to_Layer() generated
// ("cv::dnn::MVNLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MVNLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::MVNLayer::delete() generated
// ("cv::dnn::MVNLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MVNLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:550
// ("cv::dnn::MaxUnpoolLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_MaxUnpoolLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::MaxUnpoolLayer::defaultNew() generated
// ("cv::dnn::MaxUnpoolLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MaxUnpoolLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::MaxUnpoolLayer::poolKernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:546
// ("cv::dnn::MaxUnpoolLayer::poolKernel", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MaxUnpoolLayer_propPoolKernel_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::MaxUnpoolLayer::setPoolKernel(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:546
// ("cv::dnn::MaxUnpoolLayer::setPoolKernel", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_MaxUnpoolLayer_propPoolKernel_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::MaxUnpoolLayer::poolPad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:547
// ("cv::dnn::MaxUnpoolLayer::poolPad", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MaxUnpoolLayer_propPoolPad_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::MaxUnpoolLayer::setPoolPad(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:547
// ("cv::dnn::MaxUnpoolLayer::setPoolPad", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_MaxUnpoolLayer_propPoolPad_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::MaxUnpoolLayer::poolStride() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:548
// ("cv::dnn::MaxUnpoolLayer::poolStride", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_MaxUnpoolLayer_propPoolStride_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::MaxUnpoolLayer::setPoolStride(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:548
// ("cv::dnn::MaxUnpoolLayer::setPoolStride", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_MaxUnpoolLayer_propPoolStride_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::MaxUnpoolLayer::to_Algorithm() generated
// ("cv::dnn::MaxUnpoolLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MaxUnpoolLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::MaxUnpoolLayer::to_Layer() generated
// ("cv::dnn::MaxUnpoolLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MaxUnpoolLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::MaxUnpoolLayer::delete() generated
// ("cv::dnn::MaxUnpoolLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MaxUnpoolLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:476
// ("cv::dnn::MishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_MishLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::MishLayer::to_ActivationLayer() generated
// ("cv::dnn::MishLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MishLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::MishLayer::to_Algorithm() generated
// ("cv::dnn::MishLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MishLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::MishLayer::to_Layer() generated
// ("cv::dnn::MishLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MishLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::MishLayer::delete() generated
// ("cv::dnn::MishLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_MishLayer_delete(instance: *mut c_void);
// Net()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:385
// ("cv::dnn::Net::Net", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Net_Net(ocvrs_return: *mut Result<*mut c_void>);
// readFromModelOptimizer(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:394
// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["xml", "bin"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_Net_readFromModelOptimizer_const_StringR_const_StringR(xml: *const c_char, bin: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// readFromModelOptimizer(const std::vector<uchar> &, const std::vector<uchar> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:402
// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfig", "bufferWeights"], ["const std::vector<unsigned char>*", "const std::vector<unsigned char>*"]), _)]),
pub fn cv_dnn_Net_readFromModelOptimizer_const_vectorLunsigned_charGR_const_vectorLunsigned_charGR(buffer_model_config: *const c_void, buffer_weights: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// readFromModelOptimizer(const uchar *, size_t, const uchar *, size_t)(VariableArray, Primitive, VariableArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:412
// ("cv::dnn::Net::readFromModelOptimizer", vec![(pred!(mut, ["bufferModelConfigPtr", "bufferModelConfigSize", "bufferWeightsPtr", "bufferWeightsSize"], ["const unsigned char*", "size_t", "const unsigned char*", "size_t"]), _)]),
pub fn cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr: *const u8, buffer_model_config_size: size_t, buffer_weights_ptr: *const u8, buffer_weights_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:416
// ("cv::dnn::Net::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Net_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// dump()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:422
// ("cv::dnn::Net::dump", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Net_dump(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// dumpToFile(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:427
// ("cv::dnn::Net::dumpToFile", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_dumpToFile_const_StringR(instance: *mut c_void, path: *const c_char, ocvrs_return: *mut Result<()>);
// addLayer(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:434
// ("cv::dnn::Net::addLayer", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_Net_addLayer_const_StringR_const_StringR_LayerParamsR(instance: *mut c_void, name: *const c_char, typ: *const c_char, params: *mut c_void, ocvrs_return: *mut Result<i32>);
// addLayerToPrev(const String &, const String &, LayerParams &)(InString, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:438
// ("cv::dnn::Net::addLayerToPrev", vec![(pred!(mut, ["name", "type", "params"], ["const cv::String*", "const cv::String*", "cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_Net_addLayerToPrev_const_StringR_const_StringR_LayerParamsR(instance: *mut c_void, name: *const c_char, typ: *const c_char, params: *mut c_void, ocvrs_return: *mut Result<i32>);
// getLayerId(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:443
// ("cv::dnn::Net::getLayerId", vec![(pred!(const, ["layer"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_getLayerId_const_const_StringR(instance: *const c_void, layer: *const c_char, ocvrs_return: *mut Result<i32>);
// getLayerNames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:445
// ("cv::dnn::Net::getLayerNames", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Net_getLayerNames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getLayer(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:454
// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["int"]), _)]),
pub fn cv_dnn_Net_getLayer_const_int(instance: *const c_void, layer_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// getLayer(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:458
// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_getLayer_const_const_StringR(instance: *const c_void, layer_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getLayer(const LayerId &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:462
// ("cv::dnn::Net::getLayer", vec![(pred!(const, ["layerId"], ["const cv::dnn::Net::LayerId*"]), _)]),
pub fn cv_dnn_Net_getLayer_const_const_LayerIdR(instance: *const c_void, layer_id: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getLayerInputs(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:465
// ("cv::dnn::Net::getLayerInputs", vec![(pred!(const, ["layerId"], ["int"]), _)]),
pub fn cv_dnn_Net_getLayerInputs_const_int(instance: *const c_void, layer_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// connect(String, String)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:480
// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outPin", "inpPin"], ["cv::String", "cv::String"]), _)]),
pub fn cv_dnn_Net_connect_String_String(instance: *mut c_void, out_pin: *const c_char, inp_pin: *const c_char, ocvrs_return: *mut Result<()>);
// connect(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:488
// ("cv::dnn::Net::connect", vec![(pred!(mut, ["outLayerId", "outNum", "inpLayerId", "inpNum"], ["int", "int", "int", "int"]), _)]),
pub fn cv_dnn_Net_connect_int_int_int_int(instance: *mut c_void, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32, ocvrs_return: *mut Result<()>);
// registerOutput(const std::string &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:500
// ("cv::dnn::Net::registerOutput", vec![(pred!(mut, ["outputName", "layerId", "outputPort"], ["const std::string*", "int", "int"]), _)]),
pub fn cv_dnn_Net_registerOutput_const_stringR_int_int(instance: *mut c_void, output_name: *const c_char, layer_id: i32, output_port: i32, ocvrs_return: *mut Result<i32>);
// setInputsNames(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:509
// ("cv::dnn::Net::setInputsNames", vec![(pred!(mut, ["inputBlobNames"], ["const std::vector<cv::String>*"]), _)]),
pub fn cv_dnn_Net_setInputsNames_const_vectorLStringGR(instance: *mut c_void, input_blob_names: *const c_void, ocvrs_return: *mut Result<()>);
// setInputShape(const String &, const MatShape &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:513
// ("cv::dnn::Net::setInputShape", vec![(pred!(mut, ["inputName", "shape"], ["const cv::String*", "const cv::dnn::MatShape*"]), _)]),
pub fn cv_dnn_Net_setInputShape_const_StringR_const_MatShapeR(instance: *mut c_void, input_name: *const c_char, shape: *const c_void, ocvrs_return: *mut Result<()>);
// forward(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:520
// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_forward_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::Net::forward() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:520
// ("cv::dnn::Net::forward", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Net_forward(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// forwardAsync(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:529
// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, ["outputName"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_forwardAsync_const_StringR(instance: *mut c_void, output_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::Net::forwardAsync() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:529
// ("cv::dnn::Net::forwardAsync", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Net_forwardAsync(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// forward(OutputArrayOfArrays, const String &)(OutputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:536
// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outputName"], ["const cv::_OutputArray*", "const cv::String*"]), _)]),
pub fn cv_dnn_Net_forward_const__OutputArrayR_const_StringR(instance: *mut c_void, output_blobs: *const c_void, output_name: *const c_char, ocvrs_return: *mut Result<()>);
// cv::dnn::Net::forward(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:536
// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_Net_forward_const__OutputArrayR(instance: *mut c_void, output_blobs: *const c_void, ocvrs_return: *mut Result<()>);
// forward(OutputArrayOfArrays, const std::vector<String> &)(OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:542
// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["const cv::_OutputArray*", "const std::vector<cv::String>*"]), _)]),
pub fn cv_dnn_Net_forward_const__OutputArrayR_const_vectorLStringGR(instance: *mut c_void, output_blobs: *const c_void, out_blob_names: *const c_void, ocvrs_return: *mut Result<()>);
// forward(std::vector<std::vector<Mat>> &, const std::vector<String> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:549
// ("cv::dnn::Net::forward", vec![(pred!(mut, ["outputBlobs", "outBlobNames"], ["std::vector<std::vector<cv::Mat>>*", "const std::vector<cv::String>*"]), _)]),
pub fn cv_dnn_Net_forward_vectorLvectorLMatGGR_const_vectorLStringGR(instance: *mut c_void, output_blobs: *mut c_void, out_blob_names: *const c_void, ocvrs_return: *mut Result<()>);
// setHalideScheduler(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:561
// ("cv::dnn::Net::setHalideScheduler", vec![(pred!(mut, ["scheduler"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_setHalideScheduler_const_StringR(instance: *mut c_void, scheduler: *const c_char, ocvrs_return: *mut Result<()>);
// setPreferableBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:571
// ("cv::dnn::Net::setPreferableBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
pub fn cv_dnn_Net_setPreferableBackend_int(instance: *mut c_void, backend_id: i32, ocvrs_return: *mut Result<()>);
// setPreferableTarget(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:587
// ("cv::dnn::Net::setPreferableTarget", vec![(pred!(mut, ["targetId"], ["int"]), _)]),
pub fn cv_dnn_Net_setPreferableTarget_int(instance: *mut c_void, target_id: i32, ocvrs_return: *mut Result<()>);
// setInput(InputArray, const String &, double, const Scalar &)(InputArray, InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:600
// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob", "name", "scalefactor", "mean"], ["const cv::_InputArray*", "const cv::String*", "double", "const cv::Scalar*"]), _)]),
pub fn cv_dnn_Net_setInput_const__InputArrayR_const_StringR_double_const_ScalarR(instance: *mut c_void, blob: *const c_void, name: *const c_char, scalefactor: f64, mean: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::dnn::Net::setInput(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:600
// ("cv::dnn::Net::setInput", vec![(pred!(mut, ["blob"], ["const cv::_InputArray*"]), _)]),
pub fn cv_dnn_Net_setInput_const__InputArrayR(instance: *mut c_void, blob: *const c_void, ocvrs_return: *mut Result<()>);
// setParam(int, int, const Mat &)(Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:611
// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layer", "numParam", "blob"], ["int", "int", "const cv::Mat*"]), _)]),
pub fn cv_dnn_Net_setParam_int_int_const_MatR(instance: *mut c_void, layer: i32, num_param: i32, blob: *const c_void, ocvrs_return: *mut Result<()>);
// setParam(const String &, int, const Mat &)(InString, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:612
// ("cv::dnn::Net::setParam", vec![(pred!(mut, ["layerName", "numParam", "blob"], ["const cv::String*", "int", "const cv::Mat*"]), _)]),
pub fn cv_dnn_Net_setParam_const_StringR_int_const_MatR(instance: *mut c_void, layer_name: *const c_char, num_param: i32, blob: *const c_void, ocvrs_return: *mut Result<()>);
// getParam(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:619
// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer", "numParam"], ["int", "int"]), _)]),
pub fn cv_dnn_Net_getParam_const_int_int(instance: *const c_void, layer: i32, num_param: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::Net::getParam(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:619
// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layer"], ["int"]), _)]),
pub fn cv_dnn_Net_getParam_const_int(instance: *const c_void, layer: i32, ocvrs_return: *mut Result<*mut c_void>);
// getParam(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:620
// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName", "numParam"], ["const cv::String*", "int"]), _)]),
pub fn cv_dnn_Net_getParam_const_const_StringR_int(instance: *const c_void, layer_name: *const c_char, num_param: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::Net::getParam(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:620
// ("cv::dnn::Net::getParam", vec![(pred!(const, ["layerName"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_getParam_const_const_StringR(instance: *const c_void, layer_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getUnconnectedOutLayers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:626
// ("cv::dnn::Net::getUnconnectedOutLayers", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Net_getUnconnectedOutLayers_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getUnconnectedOutLayersNames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:632
// ("cv::dnn::Net::getUnconnectedOutLayersNames", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Net_getUnconnectedOutLayersNames_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getLayersShapes(const std::vector<MatShape> &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:643
// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShapes", "layersIds", "inLayersShapes", "outLayersShapes"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
pub fn cv_dnn_Net_getLayersShapes_const_const_vectorLMatShapeGR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(instance: *const c_void, net_input_shapes: *const c_void, layers_ids: *mut c_void, in_layers_shapes: *mut c_void, out_layers_shapes: *mut c_void, ocvrs_return: *mut Result<()>);
// getLayersShapes(const MatShape &, std::vector<int> &, std::vector<std::vector<MatShape>> &, std::vector<std::vector<MatShape>> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:649
// ("cv::dnn::Net::getLayersShapes", vec![(pred!(const, ["netInputShape", "layersIds", "inLayersShapes", "outLayersShapes"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<std::vector<cv::dnn::MatShape>>*", "std::vector<std::vector<cv::dnn::MatShape>>*"]), _)]),
pub fn cv_dnn_Net_getLayersShapes_const_const_MatShapeR_vectorLintGR_vectorLvectorLMatShapeGGR_vectorLvectorLMatShapeGGR(instance: *const c_void, net_input_shape: *const c_void, layers_ids: *mut c_void, in_layers_shapes: *mut c_void, out_layers_shapes: *mut c_void, ocvrs_return: *mut Result<()>);
// getLayerShapes(const MatShape &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:663
// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShape", "layerId", "inLayerShapes", "outLayerShapes"], ["const cv::dnn::MatShape*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
pub fn cv_dnn_Net_getLayerShapes_const_const_MatShapeR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(instance: *const c_void, net_input_shape: *const c_void, layer_id: i32, in_layer_shapes: *mut c_void, out_layer_shapes: *mut c_void, ocvrs_return: *mut Result<()>);
// getLayerShapes(const std::vector<MatShape> &, const int, std::vector<MatShape> &, std::vector<MatShape> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:669
// ("cv::dnn::Net::getLayerShapes", vec![(pred!(const, ["netInputShapes", "layerId", "inLayerShapes", "outLayerShapes"], ["const std::vector<cv::dnn::MatShape>*", "const int", "std::vector<cv::dnn::MatShape>*", "std::vector<cv::dnn::MatShape>*"]), _)]),
pub fn cv_dnn_Net_getLayerShapes_const_const_vectorLMatShapeGR_const_int_vectorLMatShapeGR_vectorLMatShapeGR(instance: *const c_void, net_input_shapes: *const c_void, layer_id: i32, in_layer_shapes: *mut c_void, out_layer_shapes: *mut c_void, ocvrs_return: *mut Result<()>);
// getFLOPS(const std::vector<MatShape> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:678
// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShapes"], ["const std::vector<cv::dnn::MatShape>*"]), _)]),
pub fn cv_dnn_Net_getFLOPS_const_const_vectorLMatShapeGR(instance: *const c_void, net_input_shapes: *const c_void, ocvrs_return: *mut Result<i64>);
// getFLOPS(const MatShape &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:680
// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["netInputShape"], ["const cv::dnn::MatShape*"]), _)]),
pub fn cv_dnn_Net_getFLOPS_const_const_MatShapeR(instance: *const c_void, net_input_shape: *const c_void, ocvrs_return: *mut Result<i64>);
// getFLOPS(const int, const std::vector<MatShape> &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:682
// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShapes"], ["const int", "const std::vector<cv::dnn::MatShape>*"]), _)]),
pub fn cv_dnn_Net_getFLOPS_const_const_int_const_vectorLMatShapeGR(instance: *const c_void, layer_id: i32, net_input_shapes: *const c_void, ocvrs_return: *mut Result<i64>);
// getFLOPS(const int, const MatShape &)(Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:685
// ("cv::dnn::Net::getFLOPS", vec![(pred!(const, ["layerId", "netInputShape"], ["const int", "const cv::dnn::MatShape*"]), _)]),
pub fn cv_dnn_Net_getFLOPS_const_const_int_const_MatShapeR(instance: *const c_void, layer_id: i32, net_input_shape: *const c_void, ocvrs_return: *mut Result<i64>);
// getLayerTypes(std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:691
// ("cv::dnn::Net::getLayerTypes", vec![(pred!(const, ["layersTypes"], ["std::vector<cv::String>*"]), _)]),
pub fn cv_dnn_Net_getLayerTypes_const_vectorLStringGR(instance: *const c_void, layers_types: *mut c_void, ocvrs_return: *mut Result<()>);
// getLayersCount(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:697
// ("cv::dnn::Net::getLayersCount", vec![(pred!(const, ["layerType"], ["const cv::String*"]), _)]),
pub fn cv_dnn_Net_getLayersCount_const_const_StringR(instance: *const c_void, layer_type: *const c_char, ocvrs_return: *mut Result<i32>);
// getMemoryConsumption(const std::vector<MatShape> &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:705
// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
pub fn cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_size_tR_size_tR(instance: *const c_void, net_input_shapes: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result<()>);
// getMemoryConsumption(const MatShape &, size_t &, size_t &)(CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:708
// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "weights", "blobs"], ["const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
pub fn cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_size_tR_size_tR(instance: *const c_void, net_input_shape: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result<()>);
// getMemoryConsumption(const int, const std::vector<MatShape> &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:711
// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShapes", "weights", "blobs"], ["const int", "const std::vector<cv::dnn::MatShape>*", "size_t*", "size_t*"]), _)]),
pub fn cv_dnn_Net_getMemoryConsumption_const_const_int_const_vectorLMatShapeGR_size_tR_size_tR(instance: *const c_void, layer_id: i32, net_input_shapes: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result<()>);
// getMemoryConsumption(const int, const MatShape &, size_t &, size_t &)(Primitive, CppPassByVoidPtr, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:715
// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["layerId", "netInputShape", "weights", "blobs"], ["const int", "const cv::dnn::MatShape*", "size_t*", "size_t*"]), _)]),
pub fn cv_dnn_Net_getMemoryConsumption_const_const_int_const_MatShapeR_size_tR_size_tR(instance: *const c_void, layer_id: i32, net_input_shape: *const c_void, weights: *mut size_t, blobs: *mut size_t, ocvrs_return: *mut Result<()>);
// getMemoryConsumption(const std::vector<MatShape> &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:726
// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShapes", "layerIds", "weights", "blobs"], ["const std::vector<cv::dnn::MatShape>*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
pub fn cv_dnn_Net_getMemoryConsumption_const_const_vectorLMatShapeGR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(instance: *const c_void, net_input_shapes: *const c_void, layer_ids: *mut c_void, weights: *mut c_void, blobs: *mut c_void, ocvrs_return: *mut Result<()>);
// getMemoryConsumption(const MatShape &, std::vector<int> &, std::vector<size_t> &, std::vector<size_t> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:731
// ("cv::dnn::Net::getMemoryConsumption", vec![(pred!(const, ["netInputShape", "layerIds", "weights", "blobs"], ["const cv::dnn::MatShape*", "std::vector<int>*", "std::vector<size_t>*", "std::vector<size_t>*"]), _)]),
pub fn cv_dnn_Net_getMemoryConsumption_const_const_MatShapeR_vectorLintGR_vectorLsize_tGR_vectorLsize_tGR(instance: *const c_void, net_input_shape: *const c_void, layer_ids: *mut c_void, weights: *mut c_void, blobs: *mut c_void, ocvrs_return: *mut Result<()>);
// enableFusion(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:739
// ("cv::dnn::Net::enableFusion", vec![(pred!(mut, ["fusion"], ["bool"]), _)]),
pub fn cv_dnn_Net_enableFusion_bool(instance: *mut c_void, fusion: bool, ocvrs_return: *mut Result<()>);
// getPerfProfile(std::vector<double> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/dnn.hpp:749
// ("cv::dnn::Net::getPerfProfile", vec![(pred!(mut, ["timings"], ["std::vector<double>*"]), _)]),
pub fn cv_dnn_Net_getPerfProfile_vectorLdoubleGR(instance: *mut c_void, timings: *mut c_void, ocvrs_return: *mut Result<i64>);
// cv::dnn::Net::implicitClone() generated
// ("cv::dnn::Net::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_Net_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::Net::delete() generated
// ("cv::dnn::Net::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_Net_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:655
// ("cv::dnn::NormalizeBBoxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::NormalizeBBoxLayer::defaultNew() generated
// ("cv::dnn::NormalizeBBoxLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::NormalizeBBoxLayer::pnorm() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
// ("cv::dnn::NormalizeBBoxLayer::pnorm", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_propPnorm_const(instance: *const c_void) -> f32;
// cv::dnn::NormalizeBBoxLayer::setPnorm(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
// ("cv::dnn::NormalizeBBoxLayer::setPnorm", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_propPnorm_const_float(instance: *mut c_void, val: f32);
// cv::dnn::NormalizeBBoxLayer::epsilon() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
// ("cv::dnn::NormalizeBBoxLayer::epsilon", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_propEpsilon_const(instance: *const c_void) -> f32;
// cv::dnn::NormalizeBBoxLayer::setEpsilon(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:652
// ("cv::dnn::NormalizeBBoxLayer::setEpsilon", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_propEpsilon_const_float(instance: *mut c_void, val: f32);
// cv::dnn::NormalizeBBoxLayer::acrossSpatial() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:653
// ("cv::dnn::NormalizeBBoxLayer::acrossSpatial", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const(instance: *const c_void) -> bool;
// cv::dnn::NormalizeBBoxLayer::setAcrossSpatial(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:653
// ("cv::dnn::NormalizeBBoxLayer::setAcrossSpatial", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_propAcrossSpatial_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::NormalizeBBoxLayer::to_Algorithm() generated
// ("cv::dnn::NormalizeBBoxLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::NormalizeBBoxLayer::to_Layer() generated
// ("cv::dnn::NormalizeBBoxLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::NormalizeBBoxLayer::delete() generated
// ("cv::dnn::NormalizeBBoxLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_NormalizeBBoxLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:420
// ("cv::dnn::PaddingLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_PaddingLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::PaddingLayer::defaultNew() generated
// ("cv::dnn::PaddingLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PaddingLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::PaddingLayer::to_Algorithm() generated
// ("cv::dnn::PaddingLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PaddingLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PaddingLayer::to_Layer() generated
// ("cv::dnn::PaddingLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PaddingLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PaddingLayer::delete() generated
// ("cv::dnn::PaddingLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PaddingLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:377
// ("cv::dnn::PermuteLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_PermuteLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::PermuteLayer::defaultNew() generated
// ("cv::dnn::PermuteLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PermuteLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::PermuteLayer::to_Algorithm() generated
// ("cv::dnn::PermuteLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PermuteLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PermuteLayer::to_Layer() generated
// ("cv::dnn::PermuteLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PermuteLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PermuteLayer::delete() generated
// ("cv::dnn::PermuteLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PermuteLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:266
// ("cv::dnn::PoolingLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_PoolingLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::PoolingLayer::defaultNew() generated
// ("cv::dnn::PoolingLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::PoolingLayer::type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:248
// ("cv::dnn::PoolingLayer::type", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propType_const(instance: *const c_void) -> i32;
// cv::dnn::PoolingLayer::setType(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:248
// ("cv::dnn::PoolingLayer::setType", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_PoolingLayer_propType_const_int(instance: *mut c_void, val: i32);
// cv::dnn::PoolingLayer::kernel_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
// ("cv::dnn::PoolingLayer::kernel_size", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propKernel_size_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::setKernel_size(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
// ("cv::dnn::PoolingLayer::setKernel_size", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_PoolingLayer_propKernel_size_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::PoolingLayer::strides() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
// ("cv::dnn::PoolingLayer::strides", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propStrides_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::setStrides(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:249
// ("cv::dnn::PoolingLayer::setStrides", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_PoolingLayer_propStrides_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::PoolingLayer::pads_begin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
// ("cv::dnn::PoolingLayer::pads_begin", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propPads_begin_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::setPads_begin(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
// ("cv::dnn::PoolingLayer::setPads_begin", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_PoolingLayer_propPads_begin_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::PoolingLayer::pads_end() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
// ("cv::dnn::PoolingLayer::pads_end", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propPads_end_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::setPads_end(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:250
// ("cv::dnn::PoolingLayer::setPads_end", vec![(pred!(mut, ["val"], ["const std::vector<size_t>"]), _)]),
pub fn cv_dnn_PoolingLayer_propPads_end_const_vectorLsize_tG(instance: *mut c_void, val: *const c_void);
// cv::dnn::PoolingLayer::globalPooling() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:251
// ("cv::dnn::PoolingLayer::globalPooling", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propGlobalPooling_const(instance: *const c_void) -> bool;
// cv::dnn::PoolingLayer::setGlobalPooling(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:251
// ("cv::dnn::PoolingLayer::setGlobalPooling", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_PoolingLayer_propGlobalPooling_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::PoolingLayer::isGlobalPooling() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:252
// ("cv::dnn::PoolingLayer::isGlobalPooling", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propIsGlobalPooling_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::setIsGlobalPooling(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:252
// ("cv::dnn::PoolingLayer::setIsGlobalPooling", vec![(pred!(mut, ["val"], ["const std::vector<bool>"]), _)]),
pub fn cv_dnn_PoolingLayer_propIsGlobalPooling_const_vectorLboolG(instance: *mut c_void, val: *const c_void);
// cv::dnn::PoolingLayer::computeMaxIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:253
// ("cv::dnn::PoolingLayer::computeMaxIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propComputeMaxIdx_const(instance: *const c_void) -> bool;
// cv::dnn::PoolingLayer::setComputeMaxIdx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:253
// ("cv::dnn::PoolingLayer::setComputeMaxIdx", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_PoolingLayer_propComputeMaxIdx_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::PoolingLayer::padMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:254
// ("cv::dnn::PoolingLayer::padMode", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propPadMode_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::setPadMode(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:254
// ("cv::dnn::PoolingLayer::setPadMode", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_dnn_PoolingLayer_propPadMode_const_String(instance: *mut c_void, val: *const c_char);
// cv::dnn::PoolingLayer::ceilMode() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:255
// ("cv::dnn::PoolingLayer::ceilMode", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propCeilMode_const(instance: *const c_void) -> bool;
// cv::dnn::PoolingLayer::setCeilMode(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:255
// ("cv::dnn::PoolingLayer::setCeilMode", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_PoolingLayer_propCeilMode_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::PoolingLayer::avePoolPaddedArea() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:259
// ("cv::dnn::PoolingLayer::avePoolPaddedArea", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propAvePoolPaddedArea_const(instance: *const c_void) -> bool;
// cv::dnn::PoolingLayer::setAvePoolPaddedArea(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:259
// ("cv::dnn::PoolingLayer::setAvePoolPaddedArea", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_PoolingLayer_propAvePoolPaddedArea_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::PoolingLayer::pooledSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:261
// ("cv::dnn::PoolingLayer::pooledSize", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propPooledSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::dnn::PoolingLayer::setPooledSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:261
// ("cv::dnn::PoolingLayer::setPooledSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_dnn_PoolingLayer_propPooledSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::dnn::PoolingLayer::spatialScale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:262
// ("cv::dnn::PoolingLayer::spatialScale", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propSpatialScale_const(instance: *const c_void) -> f32;
// cv::dnn::PoolingLayer::setSpatialScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:262
// ("cv::dnn::PoolingLayer::setSpatialScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_PoolingLayer_propSpatialScale_const_float(instance: *mut c_void, val: f32);
// cv::dnn::PoolingLayer::psRoiOutChannels() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:264
// ("cv::dnn::PoolingLayer::psRoiOutChannels", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PoolingLayer_propPsRoiOutChannels_const(instance: *const c_void) -> i32;
// cv::dnn::PoolingLayer::setPsRoiOutChannels(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:264
// ("cv::dnn::PoolingLayer::setPsRoiOutChannels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_PoolingLayer_propPsRoiOutChannels_const_int(instance: *mut c_void, val: i32);
// cv::dnn::PoolingLayer::to_Algorithm() generated
// ("cv::dnn::PoolingLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PoolingLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::to_Layer() generated
// ("cv::dnn::PoolingLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PoolingLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PoolingLayer::delete() generated
// ("cv::dnn::PoolingLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PoolingLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:502
// ("cv::dnn::PowerLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_PowerLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::PowerLayer::power() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
// ("cv::dnn::PowerLayer::power", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PowerLayer_propPower_const(instance: *const c_void) -> f32;
// cv::dnn::PowerLayer::setPower(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
// ("cv::dnn::PowerLayer::setPower", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_PowerLayer_propPower_const_float(instance: *mut c_void, val: f32);
// cv::dnn::PowerLayer::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
// ("cv::dnn::PowerLayer::scale", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PowerLayer_propScale_const(instance: *const c_void) -> f32;
// cv::dnn::PowerLayer::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
// ("cv::dnn::PowerLayer::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_PowerLayer_propScale_const_float(instance: *mut c_void, val: f32);
// cv::dnn::PowerLayer::shift() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
// ("cv::dnn::PowerLayer::shift", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PowerLayer_propShift_const(instance: *const c_void) -> f32;
// cv::dnn::PowerLayer::setShift(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:500
// ("cv::dnn::PowerLayer::setShift", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_PowerLayer_propShift_const_float(instance: *mut c_void, val: f32);
// cv::dnn::PowerLayer::to_ActivationLayer() generated
// ("cv::dnn::PowerLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PowerLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PowerLayer::to_Algorithm() generated
// ("cv::dnn::PowerLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PowerLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PowerLayer::to_Layer() generated
// ("cv::dnn::PowerLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PowerLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PowerLayer::delete() generated
// ("cv::dnn::PowerLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PowerLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:595
// ("cv::dnn::PriorBoxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_PriorBoxLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::PriorBoxLayer::defaultNew() generated
// ("cv::dnn::PriorBoxLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_PriorBoxLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::PriorBoxLayer::to_Algorithm() generated
// ("cv::dnn::PriorBoxLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PriorBoxLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PriorBoxLayer::to_Layer() generated
// ("cv::dnn::PriorBoxLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PriorBoxLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::PriorBoxLayer::delete() generated
// ("cv::dnn::PriorBoxLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_PriorBoxLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:683
// ("cv::dnn::ProposalLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ProposalLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ProposalLayer::defaultNew() generated
// ("cv::dnn::ProposalLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ProposalLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ProposalLayer::to_Algorithm() generated
// ("cv::dnn::ProposalLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ProposalLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ProposalLayer::to_Layer() generated
// ("cv::dnn::ProposalLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ProposalLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ProposalLayer::delete() generated
// ("cv::dnn::ProposalLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ProposalLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:185
// ("cv::dnn::RNNLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_RNNLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setWeights(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:201
// ("cv::dnn::RNNLayer::setWeights", vec![(pred!(mut, ["Wxh", "bh", "Whh", "Who", "bo"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_dnn_RNNLayer_setWeights_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR(instance: *mut c_void, wxh: *const c_void, bh: *const c_void, whh: *const c_void, who: *const c_void, bo: *const c_void, ocvrs_return: *mut Result<()>);
// setProduceHiddenOutput(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:206
// ("cv::dnn::RNNLayer::setProduceHiddenOutput", vec![(pred!(mut, ["produce"], ["bool"]), _)]),
pub fn cv_dnn_RNNLayer_setProduceHiddenOutput_bool(instance: *mut c_void, produce: bool, ocvrs_return: *mut Result<()>);
// cv::dnn::RNNLayer::setProduceHiddenOutput() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:206
// ("cv::dnn::RNNLayer::setProduceHiddenOutput", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_RNNLayer_setProduceHiddenOutput(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::dnn::RNNLayer::to_Algorithm() generated
// ("cv::dnn::RNNLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_RNNLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::RNNLayer::to_Layer() generated
// ("cv::dnn::RNNLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_RNNLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::RNNLayer::delete() generated
// ("cv::dnn::RNNLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_RNNLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:444
// ("cv::dnn::ReLU6Layer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ReLU6Layer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ReLU6Layer::minValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
// ("cv::dnn::ReLU6Layer::minValue", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ReLU6Layer_propMinValue_const(instance: *const c_void) -> f32;
// cv::dnn::ReLU6Layer::setMinValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
// ("cv::dnn::ReLU6Layer::setMinValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_ReLU6Layer_propMinValue_const_float(instance: *mut c_void, val: f32);
// cv::dnn::ReLU6Layer::maxValue() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
// ("cv::dnn::ReLU6Layer::maxValue", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ReLU6Layer_propMaxValue_const(instance: *const c_void) -> f32;
// cv::dnn::ReLU6Layer::setMaxValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:442
// ("cv::dnn::ReLU6Layer::setMaxValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_ReLU6Layer_propMaxValue_const_float(instance: *mut c_void, val: f32);
// cv::dnn::ReLU6Layer::to_ActivationLayer() generated
// ("cv::dnn::ReLU6Layer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLU6Layer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReLU6Layer::to_Algorithm() generated
// ("cv::dnn::ReLU6Layer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLU6Layer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReLU6Layer::to_Layer() generated
// ("cv::dnn::ReLU6Layer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLU6Layer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReLU6Layer::delete() generated
// ("cv::dnn::ReLU6Layer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLU6Layer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:436
// ("cv::dnn::ReLULayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ReLULayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ReLULayer::negativeSlope() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:434
// ("cv::dnn::ReLULayer::negativeSlope", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ReLULayer_propNegativeSlope_const(instance: *const c_void) -> f32;
// cv::dnn::ReLULayer::setNegativeSlope(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:434
// ("cv::dnn::ReLULayer::setNegativeSlope", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_dnn_ReLULayer_propNegativeSlope_const_float(instance: *mut c_void, val: f32);
// cv::dnn::ReLULayer::to_ActivationLayer() generated
// ("cv::dnn::ReLULayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLULayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReLULayer::to_Algorithm() generated
// ("cv::dnn::ReLULayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLULayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReLULayer::to_Layer() generated
// ("cv::dnn::ReLULayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLULayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReLULayer::delete() generated
// ("cv::dnn::ReLULayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReLULayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:607
// ("cv::dnn::RegionLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_RegionLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::RegionLayer::defaultNew() generated
// ("cv::dnn::RegionLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_RegionLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::RegionLayer::to_Algorithm() generated
// ("cv::dnn::RegionLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_RegionLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::RegionLayer::to_Layer() generated
// ("cv::dnn::RegionLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_RegionLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::RegionLayer::delete() generated
// ("cv::dnn::RegionLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_RegionLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:601
// ("cv::dnn::ReorgLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ReorgLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ReorgLayer::defaultNew() generated
// ("cv::dnn::ReorgLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ReorgLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ReorgLayer::to_Algorithm() generated
// ("cv::dnn::ReorgLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReorgLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReorgLayer::to_Layer() generated
// ("cv::dnn::ReorgLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReorgLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReorgLayer::delete() generated
// ("cv::dnn::ReorgLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReorgLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:301
// ("cv::dnn::ReshapeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ReshapeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ReshapeLayer::defaultNew() generated
// ("cv::dnn::ReshapeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ReshapeLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ReshapeLayer::newShapeDesc() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:298
// ("cv::dnn::ReshapeLayer::newShapeDesc", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ReshapeLayer_propNewShapeDesc_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::ReshapeLayer::setNewShapeDesc(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:298
// ("cv::dnn::ReshapeLayer::setNewShapeDesc", vec![(pred!(mut, ["val"], ["const cv::dnn::MatShape"]), _)]),
pub fn cv_dnn_ReshapeLayer_propNewShapeDesc_const_MatShape(instance: *mut c_void, val: *const c_void);
// cv::dnn::ReshapeLayer::newShapeRange() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:299
// ("cv::dnn::ReshapeLayer::newShapeRange", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ReshapeLayer_propNewShapeRange_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::ReshapeLayer::setNewShapeRange(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:299
// ("cv::dnn::ReshapeLayer::setNewShapeRange", vec![(pred!(mut, ["val"], ["const cv::Range"]), _)]),
pub fn cv_dnn_ReshapeLayer_propNewShapeRange_const_Range(instance: *mut c_void, val: *const c_void);
// cv::dnn::ReshapeLayer::to_Algorithm() generated
// ("cv::dnn::ReshapeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReshapeLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReshapeLayer::to_Layer() generated
// ("cv::dnn::ReshapeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReshapeLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ReshapeLayer::delete() generated
// ("cv::dnn::ReshapeLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ReshapeLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:666
// ("cv::dnn::ResizeLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ResizeLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ResizeLayer::defaultNew() generated
// ("cv::dnn::ResizeLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ResizeLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ResizeLayer::to_Algorithm() generated
// ("cv::dnn::ResizeLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ResizeLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ResizeLayer::to_Layer() generated
// ("cv::dnn::ResizeLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ResizeLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ResizeLayer::delete() generated
// ("cv::dnn::ResizeLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ResizeLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:559
// ("cv::dnn::ScaleLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ScaleLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ScaleLayer::defaultNew() generated
// ("cv::dnn::ScaleLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ScaleLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ScaleLayer::hasBias() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:556
// ("cv::dnn::ScaleLayer::hasBias", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ScaleLayer_propHasBias_const(instance: *const c_void) -> bool;
// cv::dnn::ScaleLayer::setHasBias(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:556
// ("cv::dnn::ScaleLayer::setHasBias", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_ScaleLayer_propHasBias_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::ScaleLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:557
// ("cv::dnn::ScaleLayer::axis", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ScaleLayer_propAxis_const(instance: *const c_void) -> i32;
// cv::dnn::ScaleLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:557
// ("cv::dnn::ScaleLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_ScaleLayer_propAxis_const_int(instance: *mut c_void, val: i32);
// cv::dnn::ScaleLayer::to_Algorithm() generated
// ("cv::dnn::ScaleLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ScaleLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ScaleLayer::to_Layer() generated
// ("cv::dnn::ScaleLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ScaleLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ScaleLayer::delete() generated
// ("cv::dnn::ScaleLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ScaleLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:565
// ("cv::dnn::ShiftLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ShiftLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ShiftLayer::defaultNew() generated
// ("cv::dnn::ShiftLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ShiftLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ShiftLayer::to_Algorithm() generated
// ("cv::dnn::ShiftLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ShiftLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ShiftLayer::to_Layer() generated
// ("cv::dnn::ShiftLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ShiftLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ShiftLayer::delete() generated
// ("cv::dnn::ShiftLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ShiftLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:392
// ("cv::dnn::ShuffleChannelLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_ShuffleChannelLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::ShuffleChannelLayer::defaultNew() generated
// ("cv::dnn::ShuffleChannelLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ShuffleChannelLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::ShuffleChannelLayer::group() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:394
// ("cv::dnn::ShuffleChannelLayer::group", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_ShuffleChannelLayer_propGroup_const(instance: *const c_void) -> i32;
// cv::dnn::ShuffleChannelLayer::setGroup(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:394
// ("cv::dnn::ShuffleChannelLayer::setGroup", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_ShuffleChannelLayer_propGroup_const_int(instance: *mut c_void, val: i32);
// cv::dnn::ShuffleChannelLayer::to_Algorithm() generated
// ("cv::dnn::ShuffleChannelLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ShuffleChannelLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ShuffleChannelLayer::to_Layer() generated
// ("cv::dnn::ShuffleChannelLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ShuffleChannelLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::ShuffleChannelLayer::delete() generated
// ("cv::dnn::ShuffleChannelLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_ShuffleChannelLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:482
// ("cv::dnn::SigmoidLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_SigmoidLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::SigmoidLayer::to_ActivationLayer() generated
// ("cv::dnn::SigmoidLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SigmoidLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SigmoidLayer::to_Algorithm() generated
// ("cv::dnn::SigmoidLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SigmoidLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SigmoidLayer::to_Layer() generated
// ("cv::dnn::SigmoidLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SigmoidLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SigmoidLayer::delete() generated
// ("cv::dnn::SigmoidLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SigmoidLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:371
// ("cv::dnn::SliceLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_SliceLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::SliceLayer::defaultNew() generated
// ("cv::dnn::SliceLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SliceLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::SliceLayer::sliceRanges() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:366
// ("cv::dnn::SliceLayer::sliceRanges", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SliceLayer_propSliceRanges_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::SliceLayer::setSliceRanges(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:366
// ("cv::dnn::SliceLayer::setSliceRanges", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Range>>"]), _)]),
pub fn cv_dnn_SliceLayer_propSliceRanges_const_vectorLvectorLRangeGG(instance: *mut c_void, val: *const c_void);
// cv::dnn::SliceLayer::sliceSteps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:367
// ("cv::dnn::SliceLayer::sliceSteps", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SliceLayer_propSliceSteps_const(instance: *const c_void) -> *mut c_void;
// cv::dnn::SliceLayer::setSliceSteps(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:367
// ("cv::dnn::SliceLayer::setSliceSteps", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<int>>"]), _)]),
pub fn cv_dnn_SliceLayer_propSliceSteps_const_vectorLvectorLintGG(instance: *mut c_void, val: *const c_void);
// cv::dnn::SliceLayer::axis() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:368
// ("cv::dnn::SliceLayer::axis", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SliceLayer_propAxis_const(instance: *const c_void) -> i32;
// cv::dnn::SliceLayer::setAxis(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:368
// ("cv::dnn::SliceLayer::setAxis", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_SliceLayer_propAxis_const_int(instance: *mut c_void, val: i32);
// cv::dnn::SliceLayer::num_split() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:369
// ("cv::dnn::SliceLayer::num_split", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SliceLayer_propNum_split_const(instance: *const c_void) -> i32;
// cv::dnn::SliceLayer::setNum_split(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:369
// ("cv::dnn::SliceLayer::setNum_split", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_SliceLayer_propNum_split_const_int(instance: *mut c_void, val: i32);
// cv::dnn::SliceLayer::to_Algorithm() generated
// ("cv::dnn::SliceLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SliceLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SliceLayer::to_Layer() generated
// ("cv::dnn::SliceLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SliceLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SliceLayer::delete() generated
// ("cv::dnn::SliceLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SliceLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:274
// ("cv::dnn::SoftmaxLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_SoftmaxLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::SoftmaxLayer::defaultNew() generated
// ("cv::dnn::SoftmaxLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SoftmaxLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::SoftmaxLayer::logSoftMax() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:272
// ("cv::dnn::SoftmaxLayer::logSoftMax", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SoftmaxLayer_propLogSoftMax_const(instance: *const c_void) -> bool;
// cv::dnn::SoftmaxLayer::setLogSoftMax(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:272
// ("cv::dnn::SoftmaxLayer::setLogSoftMax", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_dnn_SoftmaxLayer_propLogSoftMax_const_bool(instance: *mut c_void, val: bool);
// cv::dnn::SoftmaxLayer::to_Algorithm() generated
// ("cv::dnn::SoftmaxLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SoftmaxLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SoftmaxLayer::to_Layer() generated
// ("cv::dnn::SoftmaxLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SoftmaxLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SoftmaxLayer::delete() generated
// ("cv::dnn::SoftmaxLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SoftmaxLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:330
// ("cv::dnn::SplitLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_SplitLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::SplitLayer::defaultNew() generated
// ("cv::dnn::SplitLayer::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SplitLayer_defaultNew_const() -> *mut c_void;
// cv::dnn::SplitLayer::outputsCount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:328
// ("cv::dnn::SplitLayer::outputsCount", vec![(pred!(const, [], []), _)]),
pub fn cv_dnn_SplitLayer_propOutputsCount_const(instance: *const c_void) -> i32;
// cv::dnn::SplitLayer::setOutputsCount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:328
// ("cv::dnn::SplitLayer::setOutputsCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_dnn_SplitLayer_propOutputsCount_const_int(instance: *mut c_void, val: i32);
// cv::dnn::SplitLayer::to_Algorithm() generated
// ("cv::dnn::SplitLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SplitLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SplitLayer::to_Layer() generated
// ("cv::dnn::SplitLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SplitLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SplitLayer::delete() generated
// ("cv::dnn::SplitLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SplitLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:470
// ("cv::dnn::SwishLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_SwishLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::SwishLayer::to_ActivationLayer() generated
// ("cv::dnn::SwishLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SwishLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SwishLayer::to_Algorithm() generated
// ("cv::dnn::SwishLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SwishLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SwishLayer::to_Layer() generated
// ("cv::dnn::SwishLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SwishLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::SwishLayer::delete() generated
// ("cv::dnn::SwishLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_SwishLayer_delete(instance: *mut c_void);
// create(const LayerParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/all_layers.hpp:464
// ("cv::dnn::TanHLayer::create", vec![(pred!(mut, ["params"], ["const cv::dnn::LayerParams*"]), _)]),
pub fn cv_dnn_TanHLayer_create_const_LayerParamsR(params: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::TanHLayer::to_ActivationLayer() generated
// ("cv::dnn::TanHLayer::to_ActivationLayer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_TanHLayer_to_ActivationLayer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::TanHLayer::to_Algorithm() generated
// ("cv::dnn::TanHLayer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_TanHLayer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::dnn::TanHLayer::to_Layer() generated
// ("cv::dnn::TanHLayer::to_Layer", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_TanHLayer_to_Layer(instance: *mut c_void) -> *mut c_void;
// cv::dnn::TanHLayer::delete() generated
// ("cv::dnn::TanHLayer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_TanHLayer_delete(instance: *mut c_void);
// _Range(const Range &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:59
// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["r"], ["const cv::Range*"]), _)]),
pub fn cv_dnn__Range__Range_const_RangeR(r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// _Range(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:60
// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["start_", "size_"], ["int", "int"]), _)]),
pub fn cv_dnn__Range__Range_int_int(start_: i32, size_: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::_Range::_Range(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dnn/shape_utils.hpp:60
// ("cv::dnn::_Range::_Range", vec![(pred!(mut, ["start_"], ["int"]), _)]),
pub fn cv_dnn__Range__Range_int(start_: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn::_Range::to_Range() generated
// ("cv::dnn::_Range::to_Range", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn__Range_to_Range(instance: *mut c_void) -> *mut c_void;
// cv::dnn::_Range::delete() generated
// ("cv::dnn::_Range::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn__Range_delete(instance: *mut c_void);
