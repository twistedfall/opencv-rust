template struct Result<cv::Ptr<cv::dnn_superres::DnnSuperResImpl>*>;
template struct Result<cv::dnn_superres::DnnSuperResImpl*>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::string>*>;
template struct Result<void*>;
extern "C" void cv_PtrOfDnnSuperResImpl_delete(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
	delete instance;
}

extern "C" cv::dnn_superres::DnnSuperResImpl* cv_PtrOfDnnSuperResImpl_get_inner_ptr(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
	return instance->get();
}

