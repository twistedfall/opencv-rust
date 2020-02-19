#include "common.hpp"
#include <opencv2/imgcodecs.hpp>
#include "imgcodecs_types.hpp"

extern "C" {
	Result<bool> cv_haveImageReader_const_StringX(const char* filename) {
		try {
			bool ret = cv::haveImageReader(std::string(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_haveImageWriter_const_StringX(const char* filename) {
		try {
			bool ret = cv::haveImageWriter(std::string(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_imdecode_const__InputArrayX_int(void* buf, int flags) {
		try {
			cv::Mat ret = cv::imdecode(*reinterpret_cast<const cv::_InputArray*>(buf), flags);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_imdecode_const__InputArrayX_int_MatX(void* buf, int flags, void* dst) {
		try {
			cv::Mat ret = cv::imdecode(*reinterpret_cast<const cv::_InputArray*>(buf), flags, reinterpret_cast<cv::Mat*>(dst));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_imencode_const_StringX_const__InputArrayX_vector_unsigned_char_X_const_vector_int_X(const char* ext, void* img, void* buf, void* params) {
		try {
			bool ret = cv::imencode(std::string(ext), *reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<std::vector<unsigned char>*>(buf), *reinterpret_cast<const std::vector<int>*>(params));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_imread_const_StringX_int(const char* filename, int flags) {
		try {
			cv::Mat ret = cv::imread(std::string(filename), flags);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_imreadmulti_const_StringX_vector_Mat_X_int(const char* filename, void* mats, int flags) {
		try {
			bool ret = cv::imreadmulti(std::string(filename), *reinterpret_cast<std::vector<cv::Mat>*>(mats), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_imwrite_const_StringX_const__InputArrayX_const_vector_int_X(const char* filename, void* img, void* params) {
		try {
			bool ret = cv::imwrite(std::string(filename), *reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const std::vector<int>*>(params));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
}
