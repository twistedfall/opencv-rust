// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:60
// ("cv::dnn_superres::DnnSuperResImpl::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_create(ocvrs_return: *mut Result<*mut c_void>);
// DnnSuperResImpl()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:64
// ("cv::dnn_superres::DnnSuperResImpl::DnnSuperResImpl", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl(ocvrs_return: *mut Result<*mut c_void>);
// DnnSuperResImpl(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:74
// ("cv::dnn_superres::DnnSuperResImpl::DnnSuperResImpl", vec![(pred!(mut, ["algo", "scale"], ["const cv::String*", "int"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl_const_StringR_int(algo: *const c_char, scale: i32, ocvrs_return: *mut Result<*mut c_void>);
// readModel(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:79
// ("cv::dnn_superres::DnnSuperResImpl::readModel", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR(instance: *mut c_void, path: *const c_char, ocvrs_return: *mut Result<()>);
// readModel(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:85
// ("cv::dnn_superres::DnnSuperResImpl::readModel", vec![(pred!(mut, ["weights", "definition"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_readModel_const_StringR_const_StringR(instance: *mut c_void, weights: *const c_char, definition: *const c_char, ocvrs_return: *mut Result<()>);
// setModel(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:95
// ("cv::dnn_superres::DnnSuperResImpl::setModel", vec![(pred!(mut, ["algo", "scale"], ["const cv::String*", "int"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_setModel_const_StringR_int(instance: *mut c_void, algo: *const c_char, scale: i32, ocvrs_return: *mut Result<()>);
// setPreferableBackend(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:99
// ("cv::dnn_superres::DnnSuperResImpl::setPreferableBackend", vec![(pred!(mut, ["backendId"], ["int"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_setPreferableBackend_int(instance: *mut c_void, backend_id: i32, ocvrs_return: *mut Result<()>);
// setPreferableTarget(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:103
// ("cv::dnn_superres::DnnSuperResImpl::setPreferableTarget", vec![(pred!(mut, ["targetId"], ["int"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_setPreferableTarget_int(instance: *mut c_void, target_id: i32, ocvrs_return: *mut Result<()>);
// upsample(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:109
// ("cv::dnn_superres::DnnSuperResImpl::upsample", vec![(pred!(mut, ["img", "result"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_upsample_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, result: *const c_void, ocvrs_return: *mut Result<()>);
// upsampleMultioutput(InputArray, std::vector<Mat> &, const std::vector<int> &, const std::vector<String> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:117
// ("cv::dnn_superres::DnnSuperResImpl::upsampleMultioutput", vec![(pred!(mut, ["img", "imgs_new", "scale_factors", "node_names"], ["const cv::_InputArray*", "std::vector<cv::Mat>*", "const std::vector<int>*", "const std::vector<cv::String>*"]), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_upsampleMultioutput_const__InputArrayR_vectorLMatGR_const_vectorLintGR_const_vectorLStringGR(instance: *mut c_void, img: *const c_void, imgs_new: *mut c_void, scale_factors: *const c_void, node_names: *const c_void, ocvrs_return: *mut Result<()>);
// getScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:122
// ("cv::dnn_superres::DnnSuperResImpl::getScale", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_getScale(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// getAlgorithm()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/dnn_superres.hpp:127
// ("cv::dnn_superres::DnnSuperResImpl::getAlgorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_getAlgorithm(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::dnn_superres::DnnSuperResImpl::delete() generated
// ("cv::dnn_superres::DnnSuperResImpl::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_dnn_superres_DnnSuperResImpl_delete(instance: *mut c_void);
