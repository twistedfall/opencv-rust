template struct Result<const std::vector<cv::String>*>;
template struct Result<cv::Ptr<cv::ovis::WindowScene>*>;
template struct Result<cv::Rect_<double>>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfWindowScene_delete(cv::Ptr<cv::ovis::WindowScene>* instance) {
		delete instance;
	}

	cv::ovis::WindowScene* cv_PtrOfWindowScene_get_inner_ptr(cv::Ptr<cv::ovis::WindowScene>* instance) {
		return instance->get();
	}
}

