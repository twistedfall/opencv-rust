#include "core.hpp"

template struct Result<void*>;
template struct Result<cv::Size>;
template struct Result<const unsigned char*>;

template<typename T> inline Result<void*> ocvrs_input_array(const T* instance) {
	try {
		return Ok<void*>(new cv::_InputArray(*instance));
	} OCVRS_CATCH(Result<void*>)
}

template<typename T> inline Result<void*> ocvrs_output_array(T* instance) {
	try {
		return Ok<void*>(new cv::_OutputArray(*instance));
	} OCVRS_CATCH(Result<void*>)
}

template<typename T> inline Result<void*> ocvrs_input_output_array(T* instance) {
	try {
		return Ok<void*>(new cv::_InputOutputArray(*instance));
	} OCVRS_CATCH(Result<void*>)
}

#define ocvrs_matx(base) \
	Result<void*> cv_##base##f_input_array(const cv::base##f * instance) { return ocvrs_input_array(instance); } \
	Result<void*> cv_##base##f_output_array(cv::base##f * instance) { return ocvrs_output_array(instance); } \
	Result<void*> cv_##base##f_input_output_array(cv::base##f * instance) { return ocvrs_input_output_array(instance); } \
	Result<void*> cv_##base##d_input_array(const cv::base##d * instance) { return ocvrs_input_array(instance); } \
	Result<void*> cv_##base##d_output_array(cv::base##d * instance) { return ocvrs_output_array(instance); } \
	Result<void*> cv_##base##d_input_output_array(cv::base##d * instance) { return ocvrs_input_output_array(instance); }

extern "C" {
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

	Result<void*> cv_InputArray_input_array(cv::_InputArray* instance) { return ocvrs_input_array(instance); }
	Result<void*> cv_OutputArray_output_array(cv::_OutputArray* instance) { return ocvrs_output_array(instance); }
	Result<void*> cv_InputOutputArray_input_output_array(cv::_InputOutputArray* instance) { return ocvrs_input_output_array(instance); }

	Result<void*> cv_Scalar_input_array(cv::Scalar* instance) { return ocvrs_input_array(instance); }

	ocvrs_matx(Matx12)
	ocvrs_matx(Matx13)
	ocvrs_matx(Matx14)
	ocvrs_matx(Matx16)

	ocvrs_matx(Matx21)
	ocvrs_matx(Matx31)
	ocvrs_matx(Matx41)
	ocvrs_matx(Matx61)

	ocvrs_matx(Matx22)
	ocvrs_matx(Matx23)
	ocvrs_matx(Matx32)

	ocvrs_matx(Matx33)

	ocvrs_matx(Matx34)
	ocvrs_matx(Matx43)

	ocvrs_matx(Matx44)
	ocvrs_matx(Matx66)
}
