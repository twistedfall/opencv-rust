template struct Result<cv::Ptr<cv::ovis::WindowScene>*>;
template struct Result<cv::Rect_<double>>;
template struct Result<int>;
template struct Result<std::vector<cv::String>*>;
extern "C" void cv_PtrOfWindowScene_delete(cv::Ptr<cv::ovis::WindowScene>* instance) {
	delete instance;
}

extern "C" cv::ovis::WindowScene* cv_PtrOfWindowScene_get_inner_ptr(cv::Ptr<cv::ovis::WindowScene>* instance) {
	return instance->get();
}

