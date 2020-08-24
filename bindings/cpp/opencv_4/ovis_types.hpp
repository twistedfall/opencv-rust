template struct Result<cv::Ptr<cv::ovis::WindowScene>*>;
template struct Result<cv::Rect_<double>>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfWindowScene_delete(cv::Ptr<cv::ovis::WindowScene>* instance) {
		delete instance;
	}

	const cv::ovis::WindowScene* cv_PtrOfWindowScene_get_inner_ptr(const cv::Ptr<cv::ovis::WindowScene>* instance) {
		return instance->get();
	}

	cv::ovis::WindowScene* cv_PtrOfWindowScene_get_inner_ptr_mut(cv::Ptr<cv::ovis::WindowScene>* instance) {
		return instance->get();
	}
}

