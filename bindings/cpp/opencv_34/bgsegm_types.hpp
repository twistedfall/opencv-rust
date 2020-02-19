template struct Result<bool>;
template struct Result<double>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfBackgroundSubtractorCNT_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBackgroundSubtractorGMG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBackgroundSubtractorGSOC_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBackgroundSubtractorLSBP_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBackgroundSubtractorMOG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSyntheticSequenceGenerator_delete(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr(cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>* instance) {
	return instance->get();
}

