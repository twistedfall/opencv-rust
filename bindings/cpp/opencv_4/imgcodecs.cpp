#include "common.hpp"
#include <opencv2/imgcodecs.hpp>
#include "imgcodecs_types.hpp"

extern "C" {
	Result<bool> cv_haveImageReader_const_StringX(const char* filename) {
		try {
			bool ret = cv::haveImageReader(std::string(filename));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_haveImageWriter_const_StringX(const char* filename) {
		try {
			bool ret = cv::haveImageWriter(std::string(filename));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Mat*> cv_imdecode_const__InputArrayX_int(const cv::_InputArray* buf, int flags) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_imdecode_const__InputArrayX_int_MatX(const cv::_InputArray* buf, int flags, cv::Mat* dst) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags, dst);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<bool> cv_imencode_const_StringX_const__InputArrayX_vector_unsigned_char_X_const_vector_int_X(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, const std::vector<int>* params) {
		try {
			bool ret = cv::imencode(std::string(ext), *img, *buf, *params);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Mat*> cv_imread_const_StringX_int(const char* filename, int flags) {
		try {
			cv::Mat ret = cv::imread(std::string(filename), flags);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<bool> cv_imreadmulti_const_StringX_vector_Mat_X_int(const char* filename, std::vector<cv::Mat>* mats, int flags) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *mats, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_imwrite_const_StringX_const__InputArrayX_const_vector_int_X(const char* filename, const cv::_InputArray* img, const std::vector<int>* params) {
		try {
			bool ret = cv::imwrite(std::string(filename), *img, *params);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
}
