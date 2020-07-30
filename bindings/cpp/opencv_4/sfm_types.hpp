template struct Result<const std::vector<std::string>*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>*>;
template struct Result<cv::sfm::libmv_CameraIntrinsicsOptions>;
template struct Result<cv::sfm::libmv_ReconstructionOptions>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfSFMLibmvEuclideanReconstruction_delete(cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
		delete instance;
	}

	cv::sfm::SFMLibmvEuclideanReconstruction* cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr(cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>* instance) {
		return instance->get();
	}
}

