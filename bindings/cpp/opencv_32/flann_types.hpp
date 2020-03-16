template struct Result<bool>;
template struct Result<cv::flann::AutotunedIndexParams*>;
template struct Result<cv::flann::CompositeIndexParams*>;
template struct Result<cv::flann::HierarchicalClusteringIndexParams*>;
template struct Result<cv::flann::IndexParams*>;
template struct Result<cv::flann::Index*>;
template struct Result<cv::flann::KDTreeIndexParams*>;
template struct Result<cv::flann::KMeansIndexParams*>;
template struct Result<cv::flann::LinearIndexParams*>;
template struct Result<cv::flann::LshIndexParams*>;
template struct Result<cv::flann::SavedIndexParams*>;
template struct Result<cv::flann::SearchParams*>;
template struct Result<cvflann::flann_algorithm_t>;
template struct Result<cvflann::flann_distance_t>;
template struct Result<double>;
template struct Result<int>;
template struct Result<std::vector<cv::String>*>;
template struct Result<std::vector<double>*>;
template struct Result<std::vector<int>*>;
template struct Result<void*>;
extern "C" void cv_PtrOfIndexParams_delete(cv::Ptr<cv::flann::IndexParams>* instance) {
	delete instance;
}

extern "C" cv::flann::IndexParams* cv_PtrOfIndexParams_get_inner_ptr(cv::Ptr<cv::flann::IndexParams>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSearchParams_delete(cv::Ptr<cv::flann::SearchParams>* instance) {
	delete instance;
}

extern "C" cv::flann::SearchParams* cv_PtrOfSearchParams_get_inner_ptr(cv::Ptr<cv::flann::SearchParams>* instance) {
	return instance->get();
}

