#include "common.hpp"
#include <opencv2/cvv.hpp>
#include "cvv_types.hpp"

extern "C" {
	Result_void cvv_impl_debugDMatch_const__InputArrayR_vector_KeyPoint__const__InputArrayR_vector_KeyPoint__vector_DMatch__const_CallMetaDataR_const_charX_const_charX_bool(const cv::_InputArray* img1, std::vector<cv::KeyPoint>* keypoints1, const cv::_InputArray* img2, std::vector<cv::KeyPoint>* keypoints2, std::vector<cv::DMatch>* matches, const cvv::impl::CallMetaData* data, const char* description, const char* view, bool useTrainDescriptor) {
		try {
			cvv::impl::debugDMatch(*img1, *keypoints1, *img2, *keypoints2, *matches, *data, description, view, useTrainDescriptor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cvv_impl_debugFilter_const__InputArrayR_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(const cv::_InputArray* original, const cv::_InputArray* result, const cvv::impl::CallMetaData* data, const char* description, const char* view) {
		try {
			cvv::impl::debugFilter(*original, *result, *data, description, view);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cvv_impl_finalShow() {
		try {
			cvv::impl::finalShow();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cvv_impl_showImage_const__InputArrayR_const_CallMetaDataR_const_charX_const_charX(const cv::_InputArray* img, const cvv::impl::CallMetaData* data, const char* description, const char* view) {
		try {
			cvv::impl::showImage(*img, *data, description, view);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cvv_impl_CallMetaData_getPropFile_const(const cvv::impl::CallMetaData* instance) {
		try {
			const char* ret = instance->file;
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<const size_t> cvv_impl_CallMetaData_getPropLine_const(const cvv::impl::CallMetaData* instance) {
		try {
			const size_t ret = instance->line;
			return Ok<const size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const size_t>))
	}
	
	Result<void*> cvv_impl_CallMetaData_getPropFunction_const(const cvv::impl::CallMetaData* instance) {
		try {
			const char* ret = instance->function;
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<const bool> cvv_impl_CallMetaData_getPropIsKnown_const(const cvv::impl::CallMetaData* instance) {
		try {
			const bool ret = instance->isKnown;
			return Ok<const bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const bool>))
	}
	
	void cv_CallMetaData_delete(cvv::impl::CallMetaData* instance) {
		delete instance;
	}
	Result<cvv::impl::CallMetaData*> cvv_impl_CallMetaData_CallMetaData() {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData();
			return Ok<cvv::impl::CallMetaData*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cvv::impl::CallMetaData*>))
	}
	
	Result<cvv::impl::CallMetaData*> cvv_impl_CallMetaData_CallMetaData_const_charX_size_t_const_charX(const char* file, size_t line, const char* function) {
		try {
			cvv::impl::CallMetaData* ret = new cvv::impl::CallMetaData(file, line, function);
			return Ok<cvv::impl::CallMetaData*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cvv::impl::CallMetaData*>))
	}
	
	Result<bool> cvv_impl_CallMetaData_operator_bool(cvv::impl::CallMetaData* instance) {
		try {
			bool ret = instance->operator bool();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
}
