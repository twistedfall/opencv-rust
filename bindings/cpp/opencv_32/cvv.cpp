#include "common.hpp"
#include <opencv2/cvv.hpp>
#include "cvv_types.hpp"

extern "C" {
	Result_void cvv_impl_debugDMatch_const__InputArrayX_vector_KeyPoint__const__InputArrayX_vector_KeyPoint__vector_DMatch__const_CallMetaDataX_const_charX_const_charX_bool(void* img1, void* keypoints1, void* img2, void* keypoints2, void* matches, void* data, const char* description, const char* view, bool useTrainDescriptor) {
		try {
			cvv::impl::debugDMatch(*reinterpret_cast<const cv::_InputArray*>(img1), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints1), *reinterpret_cast<const cv::_InputArray*>(img2), *reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints2), *reinterpret_cast<std::vector<cv::DMatch>*>(matches), *reinterpret_cast<const cvv::impl::CallMetaData*>(data), description, view, useTrainDescriptor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cvv_impl_debugFilter_const__InputArrayX_const__InputArrayX_const_CallMetaDataX_const_charX_const_charX(void* original, void* result, void* data, const char* description, const char* view) {
		try {
			cvv::impl::debugFilter(*reinterpret_cast<const cv::_InputArray*>(original), *reinterpret_cast<const cv::_InputArray*>(result), *reinterpret_cast<const cvv::impl::CallMetaData*>(data), description, view);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cvv_impl_finalShow() {
		try {
			cvv::impl::finalShow();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cvv_impl_showImage_const__InputArrayX_const_CallMetaDataX_const_charX_const_charX(void* img, void* data, const char* description, const char* view) {
		try {
			cvv::impl::showImage(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cvv::impl::CallMetaData*>(data), description, view);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cvv_impl_CallMetaData_file_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cvv::impl::CallMetaData*>(instance)->file;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cvv_impl_CallMetaData_line_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cvv::impl::CallMetaData*>(instance)->line;
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<void*> cvv_impl_CallMetaData_function_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cvv::impl::CallMetaData*>(instance)->function;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cvv_impl_CallMetaData_isKnown_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cvv::impl::CallMetaData*>(instance)->isKnown;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_CallMetaData_delete(cvv::impl::CallMetaData* instance) {
		delete instance;
	}
	Result<void*> cvv_impl_CallMetaData_CallMetaData() {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(const char* file, size_t line, const char* function) {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData(file, line, function);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cvv_impl_CallMetaData_operator_bool(void* instance) {
		try {
			bool ret = reinterpret_cast<cvv::impl::CallMetaData*>(instance)->operator bool();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
}
