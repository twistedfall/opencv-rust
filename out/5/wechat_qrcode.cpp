#include "ocvrs_common.hpp"
#include <opencv2/wechat_qrcode.hpp>
#include "wechat_qrcode_types.hpp"

extern "C" {
	// WeChatQRCode(const std::string &, const std::string &, const std::string &, const std::string &)(InString, InString, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:36
	// ("cv::wechat_qrcode::WeChatQRCode::WeChatQRCode", vec![(pred!(mut, ["detector_prototxt_path", "detector_caffe_model_path", "super_resolution_prototxt_path", "super_resolution_caffe_model_path"], ["const std::string*", "const std::string*", "const std::string*", "const std::string*"]), _)]),
	void cv_wechat_qrcode_WeChatQRCode_WeChatQRCode_const_stringR_const_stringR_const_stringR_const_stringR(const char* detector_prototxt_path, const char* detector_caffe_model_path, const char* super_resolution_prototxt_path, const char* super_resolution_caffe_model_path, Result<cv::wechat_qrcode::WeChatQRCode*>* ocvrs_return) {
		try {
			cv::wechat_qrcode::WeChatQRCode* ret = new cv::wechat_qrcode::WeChatQRCode(std::string(detector_prototxt_path), std::string(detector_caffe_model_path), std::string(super_resolution_prototxt_path), std::string(super_resolution_caffe_model_path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::wechat_qrcode::WeChatQRCode::WeChatQRCode() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:36
	// ("cv::wechat_qrcode::WeChatQRCode::WeChatQRCode", vec![(pred!(mut, [], []), _)]),
	void cv_wechat_qrcode_WeChatQRCode_WeChatQRCode(Result<cv::wechat_qrcode::WeChatQRCode*>* ocvrs_return) {
		try {
			cv::wechat_qrcode::WeChatQRCode* ret = new cv::wechat_qrcode::WeChatQRCode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndDecode(InputArray, OutputArrayOfArrays)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:51
	// ("cv::wechat_qrcode::WeChatQRCode::detectAndDecode", vec![(pred!(mut, ["img", "points"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR_const__OutputArrayR(cv::wechat_qrcode::WeChatQRCode* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<std::vector<std::string>*>* ocvrs_return) {
		try {
			std::vector<std::string> ret = instance->detectAndDecode(*img, *points);
			Ok(new std::vector<std::string>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::wechat_qrcode::WeChatQRCode::detectAndDecode(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:51
	// ("cv::wechat_qrcode::WeChatQRCode::detectAndDecode", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR(cv::wechat_qrcode::WeChatQRCode* instance, const cv::_InputArray* img, Result<std::vector<std::string>*>* ocvrs_return) {
		try {
			std::vector<std::string> ret = instance->detectAndDecode(*img);
			Ok(new std::vector<std::string>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:65
	// ("cv::wechat_qrcode::WeChatQRCode::setScaleFactor", vec![(pred!(mut, ["_scalingFactor"], ["float"]), _)]),
	void cv_wechat_qrcode_WeChatQRCode_setScaleFactor_float(cv::wechat_qrcode::WeChatQRCode* instance, float _scalingFactor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(_scalingFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/wechat_qrcode.hpp:67
	// ("cv::wechat_qrcode::WeChatQRCode::getScaleFactor", vec![(pred!(mut, [], []), _)]),
	void cv_wechat_qrcode_WeChatQRCode_getScaleFactor(cv::wechat_qrcode::WeChatQRCode* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::wechat_qrcode::WeChatQRCode::delete() generated
	// ("cv::wechat_qrcode::WeChatQRCode::delete", vec![(pred!(mut, [], []), _)]),
	void cv_wechat_qrcode_WeChatQRCode_delete(cv::wechat_qrcode::WeChatQRCode* instance) {
			delete instance;
	}

}
