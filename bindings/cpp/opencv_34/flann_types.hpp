template struct Result<bool>;
template struct Result<const char*>;
template struct Result<cvflann::flann_algorithm_t>;
template struct Result<cvflann::flann_distance_t>;
template struct Result<double>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfIndexParams_delete(cv::Ptr<cv::flann::IndexParams>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfIndexParams_get_inner_ptr(cv::Ptr<cv::flann::IndexParams>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSearchParams_delete(cv::Ptr<cv::flann::SearchParams>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSearchParams_get_inner_ptr(cv::Ptr<cv::flann::SearchParams>* instance) {
	return instance->get();
}

