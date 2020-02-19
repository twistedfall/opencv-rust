template struct Result<bool>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfHDF5_delete(cv::Ptr<cv::hdf::HDF5>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfHDF5_get_inner_ptr(cv::Ptr<cv::hdf::HDF5>* instance) {
	return instance->get();
}

