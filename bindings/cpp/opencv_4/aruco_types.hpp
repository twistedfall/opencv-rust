template struct Result<bool>;
template struct Result<cv::Size_<int>>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfBoard_delete(cv::Ptr<cv::aruco::Board>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBoard_get_inner_ptr(cv::Ptr<cv::aruco::Board>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCharucoBoard_delete(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfCharucoBoard_get_inner_ptr(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetectorParameters_delete(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetectorParameters_get_inner_ptr(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDictionary_delete(cv::Ptr<cv::aruco::Dictionary>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDictionary_get_inner_ptr(cv::Ptr<cv::aruco::Dictionary>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGridBoard_delete(cv::Ptr<cv::aruco::GridBoard>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfGridBoard_get_inner_ptr(cv::Ptr<cv::aruco::GridBoard>* instance) {
	return instance->get();
}

