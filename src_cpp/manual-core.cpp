#include "core.hpp"

template struct Result<void*>;
template struct Result<cv::Size>;
template struct Result<const unsigned char*>;

template<typename T> inline void ocvrs_input_array(const T* instance, Result<void*>* ocvrs_return) {
	try {
		Ok<void*>(new cv::_InputArray(*instance), ocvrs_return);
	} OCVRS_CATCH(Result<void*>)
}

template<typename T> inline void ocvrs_output_array(T* instance, Result<void*>* ocvrs_return) {
	try {
		Ok<void*>(new cv::_OutputArray(*instance), ocvrs_return);
	} OCVRS_CATCH(Result<void*>)
}

template<typename T> inline void ocvrs_input_output_array(T* instance, Result<void*>* ocvrs_return) {
	try {
		Ok<void*>(new cv::_InputOutputArray(*instance), ocvrs_return);
	} OCVRS_CATCH(Result<void*>)
}

#define ocvrs_ioa(base) \
	void cv_##base##_input_array(const cv::base* instance, Result<void*>* ocvrs_return) { return ocvrs_input_array(instance, ocvrs_return); } \
	void cv_##base##_output_array(cv::base* instance, Result<void*>* ocvrs_return) { return ocvrs_output_array(instance, ocvrs_return); } \
	void cv_##base##_input_output_array(cv::base* instance, Result<void*>* ocvrs_return) { return ocvrs_input_output_array(instance, ocvrs_return); }

#define ocvrs_ioa_df(base) \
	ocvrs_ioa(base##d) \
	ocvrs_ioa(base##f)

#define ocvrs_ioa_bdfisw(base) \
	ocvrs_ioa(base##b) \
	ocvrs_ioa(base##d) \
	ocvrs_ioa(base##f) \
	ocvrs_ioa(base##i) \
	ocvrs_ioa(base##s) \
	ocvrs_ioa(base##w)

extern "C" {
	void cv_manual_Mat_size(const cv::Mat* instance, Result<cv::Size>* ocvrs_return) {
		try {
			Ok<cv::Size>(instance->size(), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Size>)
	}

	void cv_manual_Mat_set(cv::Mat* instance, cv::Scalar s, Result_void* ocvrs_return) {
		try {
			*instance = s;
			Ok(ocvrs_return);
		} OCVRS_CATCH(Result_void)
	}

	const unsigned char* cv_manual_Mat_data(const cv::Mat* instance) {
		return instance->data;
	}

	void cv_manual_UMat_size(const cv::UMat* instance, Result<cv::Size>* ocvrs_return) {
		try {
			Ok<cv::Size>(instance->size(), ocvrs_return);
		} OCVRS_CATCH(Result<cv::Size>)
	}

	int cv_manual_MatSize_dims(const cv::MatSize* instance) {
		return *(instance->p - 1);
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

	void cv_InputArray_input_array(cv::_InputArray* instance, Result<void*>* ocvrs_return) { return ocvrs_input_array(instance, ocvrs_return); }
	void cv_OutputArray_output_array(cv::_OutputArray* instance, Result<void*>* ocvrs_return) { return ocvrs_output_array(instance, ocvrs_return); }
	void cv_InputOutputArray_input_output_array(cv::_InputOutputArray* instance, Result<void*>* ocvrs_return) { return ocvrs_input_output_array(instance, ocvrs_return); }

	ocvrs_ioa_df(Matx12)
	ocvrs_ioa_df(Matx13)
	ocvrs_ioa_df(Matx14)
	ocvrs_ioa_df(Matx16)

	ocvrs_ioa_df(Matx21)
	ocvrs_ioa_df(Matx31)
	ocvrs_ioa_df(Matx41)
	ocvrs_ioa_df(Matx61)

	ocvrs_ioa_df(Matx22)
	ocvrs_ioa_df(Matx23)
	ocvrs_ioa_df(Matx32)

	ocvrs_ioa_df(Matx33)

	ocvrs_ioa_df(Matx34)
	ocvrs_ioa_df(Matx43)

	ocvrs_ioa_df(Matx44)
	ocvrs_ioa_df(Matx66)

	ocvrs_ioa_bdfisw(Vec2)
	ocvrs_ioa_bdfisw(Vec3)
	ocvrs_ioa_bdfisw(Vec4)

	ocvrs_ioa_df(Vec6)
	ocvrs_ioa(Vec6i)

	ocvrs_ioa(Vec8i)

	void cv_Vec18d_input_array(cv::Vec<double, 18>* instance, Result<void*>* ocvrs_return) { return ocvrs_input_array(instance, ocvrs_return); }
	void cv_Vec18d_output_array(cv::Vec<double, 18>* instance, Result<void*>* ocvrs_return) { return ocvrs_output_array(instance, ocvrs_return); }
	void cv_Vec18d_input_output_array(cv::Vec<double, 18>* instance, Result<void*>* ocvrs_return) { return ocvrs_input_output_array(instance, ocvrs_return); }
}
