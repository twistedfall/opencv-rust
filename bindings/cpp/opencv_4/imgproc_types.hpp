template struct Result<bool>;
template struct Result<const unsigned char*>;
template struct Result<cv::Matx<double, 2, 3>>;
template struct Result<cv::Moments>;
template struct Result<cv::Point_<double>>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Scalar_<double>>;
template struct Result<cv::Size_<int>>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<unsigned char*>;
template struct Result<void*>;
extern "C" void cv_PtrOfCLAHE_delete(cv::Ptr<cv::CLAHE>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfCLAHE_get_inner_ptr(cv::Ptr<cv::CLAHE>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGeneralizedHoughBallard_delete(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfGeneralizedHoughBallard_get_inner_ptr(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGeneralizedHoughGuil_delete(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfGeneralizedHoughGuil_get_inner_ptr(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLineSegmentDetector_delete(cv::Ptr<cv::LineSegmentDetector>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfLineSegmentDetector_get_inner_ptr(cv::Ptr<cv::LineSegmentDetector>* instance) {
	return instance->get();
}

