template struct Result<bool>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Vec<double, 3>>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfStereoBM_delete(cv::Ptr<cv::StereoBM>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfStereoBM_get_inner_ptr(cv::Ptr<cv::StereoBM>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfStereoSGBM_delete(cv::Ptr<cv::StereoSGBM>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfStereoSGBM_get_inner_ptr(cv::Ptr<cv::StereoSGBM>* instance) {
	return instance->get();
}

