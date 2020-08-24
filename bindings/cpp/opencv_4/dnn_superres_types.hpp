template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::dnn_superres::DnnSuperResImpl>*>;
template struct Result<cv::dnn_superres::DnnSuperResImpl*>;
template struct Result<int>;
template struct Result<void*>;
extern "C" {
	cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* cv_PtrOfDnnSuperResImpl_new(cv::dnn_superres::DnnSuperResImpl* val) {
		return new cv::Ptr<cv::dnn_superres::DnnSuperResImpl>(val);
	}
	
	void cv_PtrOfDnnSuperResImpl_delete(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
		delete instance;
	}

	const cv::dnn_superres::DnnSuperResImpl* cv_PtrOfDnnSuperResImpl_get_inner_ptr(const cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
		return instance->get();
	}

	cv::dnn_superres::DnnSuperResImpl* cv_PtrOfDnnSuperResImpl_get_inner_ptr_mut(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
		return instance->get();
	}
}

