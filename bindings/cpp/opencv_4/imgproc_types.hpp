template struct Result<bool>;
template struct Result<const unsigned char*>;
template struct Result<cv::LineIterator*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Matx<double, 2, 3>>;
template struct Result<cv::Moments>;
template struct Result<cv::Point_<double>>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::CLAHE>*>;
template struct Result<cv::Ptr<cv::GeneralizedHoughBallard>*>;
template struct Result<cv::Ptr<cv::GeneralizedHoughGuil>*>;
template struct Result<cv::Ptr<cv::LineSegmentDetector>*>;
template struct Result<cv::Ptr<float>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::RotatedRect*>;
template struct Result<cv::Scalar_<double>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::Subdiv2D*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Point_<double>>*>;
template struct Result<std::vector<cv::Point_<float>>*>;
template struct Result<std::vector<cv::Point_<int>>*>;
template struct Result<std::vector<cv::Vec<float, 4>>*>;
template struct Result<std::vector<cv::Vec<float, 6>>*>;
template struct Result<std::vector<cv::Vec<int, 4>>*>;
template struct Result<std::vector<float>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::vector<cv::Point_<float>>>*>;
template struct Result<unsigned char*>;
extern "C" void cv_PtrOfCLAHE_delete(cv::Ptr<cv::CLAHE>* instance) {
	delete instance;
}

extern "C" cv::CLAHE* cv_PtrOfCLAHE_get_inner_ptr(cv::Ptr<cv::CLAHE>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGeneralizedHoughBallard_delete(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
	delete instance;
}

extern "C" cv::GeneralizedHoughBallard* cv_PtrOfGeneralizedHoughBallard_get_inner_ptr(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGeneralizedHoughGuil_delete(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
	delete instance;
}

extern "C" cv::GeneralizedHoughGuil* cv_PtrOfGeneralizedHoughGuil_get_inner_ptr(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLineSegmentDetector_delete(cv::Ptr<cv::LineSegmentDetector>* instance) {
	delete instance;
}

extern "C" cv::LineSegmentDetector* cv_PtrOfLineSegmentDetector_get_inner_ptr(cv::Ptr<cv::LineSegmentDetector>* instance) {
	return instance->get();
}

