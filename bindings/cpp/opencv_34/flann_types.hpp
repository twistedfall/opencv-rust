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
template struct Result<void*>;
extern "C" {
	cv::Ptr<cv::flann::IndexParams>* cv_PtrOfIndexParams_new(cv::flann::IndexParams* val) {
		return new cv::Ptr<cv::flann::IndexParams>(val);
	}
	
	void cv_PtrOfIndexParams_delete(cv::Ptr<cv::flann::IndexParams>* instance) {
		delete instance;
	}

	const cv::flann::IndexParams* cv_PtrOfIndexParams_get_inner_ptr(const cv::Ptr<cv::flann::IndexParams>* instance) {
		return instance->get();
	}

	cv::flann::IndexParams* cv_PtrOfIndexParams_get_inner_ptr_mut(cv::Ptr<cv::flann::IndexParams>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::flann::SearchParams>* cv_PtrOfSearchParams_new(cv::flann::SearchParams* val) {
		return new cv::Ptr<cv::flann::SearchParams>(val);
	}
	
	void cv_PtrOfSearchParams_delete(cv::Ptr<cv::flann::SearchParams>* instance) {
		delete instance;
	}

	const cv::flann::SearchParams* cv_PtrOfSearchParams_get_inner_ptr(const cv::Ptr<cv::flann::SearchParams>* instance) {
		return instance->get();
	}

	cv::flann::SearchParams* cv_PtrOfSearchParams_get_inner_ptr_mut(cv::Ptr<cv::flann::SearchParams>* instance) {
		return instance->get();
	}
}

