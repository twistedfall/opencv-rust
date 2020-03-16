template struct Result<bool>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>;
template struct Result<double>;
template struct Result<int>;
extern "C" void cv_PtrOfBackgroundSubtractorGMG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
	delete instance;
}

extern "C" cv::bgsegm::BackgroundSubtractorGMG* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBackgroundSubtractorMOG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
	delete instance;
}

extern "C" cv::bgsegm::BackgroundSubtractorMOG* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
	return instance->get();
}

