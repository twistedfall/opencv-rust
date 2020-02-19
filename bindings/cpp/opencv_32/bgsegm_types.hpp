template struct Result<bool>;
template struct Result<double>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfBackgroundSubtractorGMG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBackgroundSubtractorMOG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
	return instance->get();
}

