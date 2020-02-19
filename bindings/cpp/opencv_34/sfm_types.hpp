template struct Result<cv::sfm::libmv_CameraIntrinsicsOptions>;
template struct Result<cv::sfm::libmv_ReconstructionOptions>;
template struct Result<double>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfSFMLibmvEuclideanReconstruction_delete(cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr(cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
	return instance->get();
}

