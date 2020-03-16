template struct Result<cv::Ptr<cv::xobjdetect::WBDetector>*>;
template struct Result<std::vector<cv::Rect_<int>>*>;
template struct Result<std::vector<double>*>;
extern "C" void cv_PtrOfWBDetector_delete(cv::Ptr<cv::xobjdetect::WBDetector>* instance) {
	delete instance;
}

extern "C" cv::xobjdetect::WBDetector* cv_PtrOfWBDetector_get_inner_ptr(cv::Ptr<cv::xobjdetect::WBDetector>* instance) {
	return instance->get();
}

