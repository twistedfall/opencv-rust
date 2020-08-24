template struct Result<bool>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>;
template struct Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfBackgroundSubtractorGMG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorGMG* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorGMG* cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBackgroundSubtractorMOG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		delete instance;
	}

	const cv::bgsegm::BackgroundSubtractorMOG* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(const cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		return instance->get();
	}

	cv::bgsegm::BackgroundSubtractorMOG* cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr_mut(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
		return instance->get();
	}
}

