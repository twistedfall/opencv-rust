template struct Result<bool>;
template struct Result<cv::CirclesGridFinderParameters>;
template struct Result<cv::CirclesGridFinderParameters::GridType>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Size_<float>>;
template struct Result<cv::Vec<double, 3>>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfLMSolver_delete(cv::Ptr<cv::LMSolver>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfLMSolver_get_inner_ptr(cv::Ptr<cv::LMSolver>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLMSolver_Callback_delete(cv::Ptr<cv::LMSolver::Callback>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfLMSolver_Callback_get_inner_ptr(cv::Ptr<cv::LMSolver::Callback>* instance) {
	return instance->get();
}

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

