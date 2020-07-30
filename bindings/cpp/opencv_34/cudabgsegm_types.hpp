template struct Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>*>;
template struct Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG>*>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfCUDA_BackgroundSubtractorMOG_delete(cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* instance) {
		delete instance;
	}

	cv::cuda::BackgroundSubtractorMOG* cv_PtrOfCUDA_BackgroundSubtractorMOG_get_inner_ptr(cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_BackgroundSubtractorMOG2_delete(cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>* instance) {
		delete instance;
	}

	cv::cuda::BackgroundSubtractorMOG2* cv_PtrOfCUDA_BackgroundSubtractorMOG2_get_inner_ptr(cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>* instance) {
		return instance->get();
	}
}

