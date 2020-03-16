template struct Result<bool>;
template struct Result<cv::Ptr<cv::hdf::HDF5>*>;
template struct Result<int>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<int>*>;
extern "C" void cv_PtrOfHDF5_delete(cv::Ptr<cv::hdf::HDF5>* instance) {
	delete instance;
}

extern "C" cv::hdf::HDF5* cv_PtrOfHDF5_get_inner_ptr(cv::Ptr<cv::hdf::HDF5>* instance) {
	return instance->get();
}

