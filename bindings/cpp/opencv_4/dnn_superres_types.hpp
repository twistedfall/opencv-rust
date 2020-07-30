template struct Result<const std::vector<int>*>;
template struct Result<const std::vector<std::string>*>;
template struct Result<cv::Ptr<cv::dnn_superres::DnnSuperResImpl>*>;
template struct Result<cv::dnn_superres::DnnSuperResImpl*>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<void*>;
extern "C" {
	cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* cv_PtrOfDnnSuperResImpl_new(cv::dnn_superres::DnnSuperResImpl* val) {
		return new cv::Ptr<cv::dnn_superres::DnnSuperResImpl>(val);
	}
	
	void cv_PtrOfDnnSuperResImpl_delete(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
		delete instance;
	}

	cv::dnn_superres::DnnSuperResImpl* cv_PtrOfDnnSuperResImpl_get_inner_ptr(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
		return instance->get();
	}
}

