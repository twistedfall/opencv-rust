#include "core.hpp"

template struct Result<void*>;
template struct Result<cv::Size>;
template struct Result<const unsigned char*>;

extern "C" {
	Result<void*> cv_InputArray_input_array(cv::_InputArray* instance) {
		try {
			return Ok<void*>(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<void*>)
	}

	Result<void*> cv_OutputArray_output_array(cv::_OutputArray* instance) {
		try {
			return Ok<void*>(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<void*>)
	}

	Result<void*> cv_InputOutputArray_input_output_array(cv::_InputOutputArray* instance) {
		try {
			return Ok<void*>(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<void*>)
	}

	Result<void*> cv_Scalar_input_array(cv::Scalar* instance) {
		try {
			return Ok<void*>(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<void*>)
	}

	Result<cv::Size> cv_manual_Mat_size(const cv::Mat* instance) {
		try {
			return Ok<cv::Size>(instance->size());
		} OCVRS_CATCH(Result<cv::Size>)
	}

	bool cv_manual_Mat_is_allocated(const cv::Mat* instance) {
		return instance->data != NULL;
	}

	Result_void cv_manual_Mat_set(cv::Mat* instance, cv::Scalar s) {
		try {
			*instance = s;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}

	Result<const unsigned char*> cv_manual_Mat_data(const cv::Mat* instance) {
		try {
			return Ok<const unsigned char*>(instance->data);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}

	Result<cv::Size> cv_manual_UMat_size(const cv::UMat* instance) {
		try {
			return Ok<cv::Size>(instance->size());
		} OCVRS_CATCH(Result<cv::Size>)
	}

	int cv_manual_MatSize_dims(const cv::MatSize* instance) {
		return *(instance->p - 1);
	}

	const int* cv_manual_MatSize_deref(const cv::MatSize* instance) {
		return instance->p;
	}

	const size_t* cv_manual_MatStep_deref(const cv::MatStep* instance) {
		return instance->p;
	}

	int cv_manual_MatConstIterator_type(const cv::MatConstIterator* instance) {
		return instance->m->type();
	}

	bool cv_manual_MatConstIterator_has_elements(const cv::MatConstIterator* instance) {
		return instance->ptr != instance->sliceEnd;
	}

	const unsigned char* cv_manual_MatConstIterator_current_unchecked(const cv::MatConstIterator* instance) {
		return instance->ptr;
	}

}
