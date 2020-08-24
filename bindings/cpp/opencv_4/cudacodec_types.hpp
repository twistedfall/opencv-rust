template struct Result<bool>;
template struct Result<cv::Ptr<cv::cudacodec::VideoReader>*>;
template struct Result<cv::Ptr<cv::cudacodec::VideoWriter>*>;
template struct Result<cv::cudacodec::ChromaFormat>;
template struct Result<cv::cudacodec::Codec>;
template struct Result<cv::cudacodec::EncoderParams*>;
template struct Result<cv::cudacodec::FormatInfo>;
template struct Result<int>;
template struct Result<unsigned char*>;
extern "C" {
	void cv_PtrOfEncoderCallBack_delete(cv::Ptr<cv::cudacodec::EncoderCallBack>* instance) {
		delete instance;
	}

	const cv::cudacodec::EncoderCallBack* cv_PtrOfEncoderCallBack_get_inner_ptr(const cv::Ptr<cv::cudacodec::EncoderCallBack>* instance) {
		return instance->get();
	}

	cv::cudacodec::EncoderCallBack* cv_PtrOfEncoderCallBack_get_inner_ptr_mut(cv::Ptr<cv::cudacodec::EncoderCallBack>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfRawVideoSource_delete(cv::Ptr<cv::cudacodec::RawVideoSource>* instance) {
		delete instance;
	}

	const cv::cudacodec::RawVideoSource* cv_PtrOfRawVideoSource_get_inner_ptr(const cv::Ptr<cv::cudacodec::RawVideoSource>* instance) {
		return instance->get();
	}

	cv::cudacodec::RawVideoSource* cv_PtrOfRawVideoSource_get_inner_ptr_mut(cv::Ptr<cv::cudacodec::RawVideoSource>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfVideoReader_delete(cv::Ptr<cv::cudacodec::VideoReader>* instance) {
		delete instance;
	}

	const cv::cudacodec::VideoReader* cv_PtrOfVideoReader_get_inner_ptr(const cv::Ptr<cv::cudacodec::VideoReader>* instance) {
		return instance->get();
	}

	cv::cudacodec::VideoReader* cv_PtrOfVideoReader_get_inner_ptr_mut(cv::Ptr<cv::cudacodec::VideoReader>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfVideoWriter_delete(cv::Ptr<cv::cudacodec::VideoWriter>* instance) {
		delete instance;
	}

	const cv::cudacodec::VideoWriter* cv_PtrOfVideoWriter_get_inner_ptr(const cv::Ptr<cv::cudacodec::VideoWriter>* instance) {
		return instance->get();
	}

	cv::cudacodec::VideoWriter* cv_PtrOfVideoWriter_get_inner_ptr_mut(cv::Ptr<cv::cudacodec::VideoWriter>* instance) {
		return instance->get();
	}
}

