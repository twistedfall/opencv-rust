// WeChatQRCode(const std::string &, const std::string &, const std::string &, const std::string &)(InString, InString, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:36
// ("cv::wechat_qrcode::WeChatQRCode::WeChatQRCode", vec![(pred!(mut, ["detector_prototxt_path", "detector_caffe_model_path", "super_resolution_prototxt_path", "super_resolution_caffe_model_path"], ["const std::string*", "const std::string*", "const std::string*", "const std::string*"]), _)]),
pub fn cv_wechat_qrcode_WeChatQRCode_WeChatQRCode_const_stringR_const_stringR_const_stringR_const_stringR(detector_prototxt_path: *const c_char, detector_caffe_model_path: *const c_char, super_resolution_prototxt_path: *const c_char, super_resolution_caffe_model_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::wechat_qrcode::WeChatQRCode::WeChatQRCode() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:36
// ("cv::wechat_qrcode::WeChatQRCode::WeChatQRCode", vec![(pred!(mut, [], []), _)]),
pub fn cv_wechat_qrcode_WeChatQRCode_WeChatQRCode(ocvrs_return: *mut Result<*mut c_void>);
// detectAndDecode(InputArray, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:51
// ("cv::wechat_qrcode::WeChatQRCode::detectAndDecode", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, img: *const c_void, points: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::wechat_qrcode::WeChatQRCode::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:51
// ("cv::wechat_qrcode::WeChatQRCode::detectAndDecode", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:65
// ("cv::wechat_qrcode::WeChatQRCode::setScaleFactor", vec![(pred!(mut, ["_scalingFactor"], ["float"]), _)]),
pub fn cv_wechat_qrcode_WeChatQRCode_setScaleFactor_float(instance: *mut c_void, _scaling_factor: f32, ocvrs_return: *mut Result<()>);
// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:67
// ("cv::wechat_qrcode::WeChatQRCode::getScaleFactor", vec![(pred!(mut, [], []), _)]),
pub fn cv_wechat_qrcode_WeChatQRCode_getScaleFactor(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// cv::wechat_qrcode::WeChatQRCode::delete() generated
// ("cv::wechat_qrcode::WeChatQRCode::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_wechat_qrcode_WeChatQRCode_delete(instance: *mut c_void);
