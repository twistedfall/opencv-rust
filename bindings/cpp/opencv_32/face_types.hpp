template struct Result<bool>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::face::BIF>*>;
template struct Result<cv::Ptr<cv::face::BasicFaceRecognizer>*>;
template struct Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*>;
template struct Result<cv::Ptr<cv::face::PredictCollector>*>;
template struct Result<cv::Ptr<cv::face::StandardCollector>*>;
template struct Result<cv::face::StandardCollector*>;
template struct Result<cv::face::StandardCollector::PredictResult>;
template struct Result<double>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<int>*>;
template struct Result<void*>;
extern "C" void cv_PtrOfBIF_delete(cv::Ptr<cv::face::BIF>* instance) {
	delete instance;
}

extern "C" cv::face::BIF* cv_PtrOfBIF_get_inner_ptr(cv::Ptr<cv::face::BIF>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBasicFaceRecognizer_delete(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
	delete instance;
}

extern "C" cv::face::BasicFaceRecognizer* cv_PtrOfBasicFaceRecognizer_get_inner_ptr(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLBPHFaceRecognizer_delete(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
	delete instance;
}

extern "C" cv::face::LBPHFaceRecognizer* cv_PtrOfLBPHFaceRecognizer_get_inner_ptr(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfPredictCollector_delete(cv::Ptr<cv::face::PredictCollector>* instance) {
	delete instance;
}

extern "C" cv::face::PredictCollector* cv_PtrOfPredictCollector_get_inner_ptr(cv::Ptr<cv::face::PredictCollector>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfStandardCollector_delete(cv::Ptr<cv::face::StandardCollector>* instance) {
	delete instance;
}

extern "C" cv::face::StandardCollector* cv_PtrOfStandardCollector_get_inner_ptr(cv::Ptr<cv::face::StandardCollector>* instance) {
	return instance->get();
}

