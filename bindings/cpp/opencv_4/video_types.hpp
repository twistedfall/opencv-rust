template struct Result<bool>;
template struct Result<cv::KalmanFilter*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>;
template struct Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>;
template struct Result<cv::Ptr<cv::DISOpticalFlow>*>;
template struct Result<cv::Ptr<cv::FarnebackOpticalFlow>*>;
template struct Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>;
template struct Result<cv::Ptr<cv::VariationalRefinement>*>;
template struct Result<cv::RotatedRect*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::TermCriteria>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
extern "C" void cv_PtrOfBackgroundSubtractorKNN_delete(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
	delete instance;
}

extern "C" cv::BackgroundSubtractorKNN* cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBackgroundSubtractorMOG2_delete(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
	delete instance;
}

extern "C" cv::BackgroundSubtractorMOG2* cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDISOpticalFlow_delete(cv::Ptr<cv::DISOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::DISOpticalFlow* cv_PtrOfDISOpticalFlow_get_inner_ptr(cv::Ptr<cv::DISOpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFarnebackOpticalFlow_delete(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::FarnebackOpticalFlow* cv_PtrOfFarnebackOpticalFlow_get_inner_ptr(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSparsePyrLKOpticalFlow_delete(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::SparsePyrLKOpticalFlow* cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfVariationalRefinement_delete(cv::Ptr<cv::VariationalRefinement>* instance) {
	delete instance;
}

extern "C" cv::VariationalRefinement* cv_PtrOfVariationalRefinement_get_inner_ptr(cv::Ptr<cv::VariationalRefinement>* instance) {
	return instance->get();
}

