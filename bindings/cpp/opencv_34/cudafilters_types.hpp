template struct Result<cv::Ptr<cv::cuda::Filter>*>;
extern "C" {
	void cv_PtrOfFilter_delete(cv::Ptr<cv::cuda::Filter>* instance) {
		delete instance;
	}

	const cv::cuda::Filter* cv_PtrOfFilter_get_inner_ptr(const cv::Ptr<cv::cuda::Filter>* instance) {
		return instance->get();
	}

	cv::cuda::Filter* cv_PtrOfFilter_get_inner_ptr_mut(cv::Ptr<cv::cuda::Filter>* instance) {
		return instance->get();
	}
}

