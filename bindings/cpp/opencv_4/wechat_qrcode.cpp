#include "ocvrs_common.hpp"
#include <opencv2/wechat_qrcode.hpp>
#include "wechat_qrcode_types.hpp"

extern "C" {
	void cv_WeChatQRCode_delete(cv::wechat_qrcode::WeChatQRCode* instance) {
		delete instance;
	}
	Result<cv::wechat_qrcode::WeChatQRCode*> cv_wechat_qrcode_WeChatQRCode_WeChatQRCode_const_stringR_const_stringR_const_stringR_const_stringR(const char* detector_prototxt_path, const char* detector_caffe_model_path, const char* super_resolution_prototxt_path, const char* super_resolution_caffe_model_path) {
		try {
			cv::wechat_qrcode::WeChatQRCode* ret = new cv::wechat_qrcode::WeChatQRCode(std::string(detector_prototxt_path), std::string(detector_caffe_model_path), std::string(super_resolution_prototxt_path), std::string(super_resolution_caffe_model_path));
			return Ok<cv::wechat_qrcode::WeChatQRCode*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::wechat_qrcode::WeChatQRCode*>))
	}
	
	Result<std::vector<std::string>*> cv_wechat_qrcode_WeChatQRCode_detectAndDecode_const__InputArrayR_const__OutputArrayR(cv::wechat_qrcode::WeChatQRCode* instance, const cv::_InputArray* img, const cv::_OutputArray* points) {
		try {
			std::vector<std::string> ret = instance->detectAndDecode(*img, *points);
			return Ok(new std::vector<std::string>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::string>*>))
	}
	
}
