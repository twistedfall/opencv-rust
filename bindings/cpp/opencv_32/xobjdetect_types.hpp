template struct Result<cv::Ptr<cv::xobjdetect::WBDetector>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<double>;
extern "C" {
	void cv_PtrOfWBDetector_delete(cv::Ptr<cv::xobjdetect::WBDetector>* instance) {
		delete instance;
	}

	const cv::xobjdetect::WBDetector* cv_PtrOfWBDetector_get_inner_ptr(const cv::Ptr<cv::xobjdetect::WBDetector>* instance) {
		return instance->get();
	}

	cv::xobjdetect::WBDetector* cv_PtrOfWBDetector_get_inner_ptr_mut(cv::Ptr<cv::xobjdetect::WBDetector>* instance) {
		return instance->get();
	}
}

