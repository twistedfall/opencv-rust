template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::hfs::HfsSegment>*>;
template struct Result<float>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfHfsSegment_delete(cv::Ptr<cv::hfs::HfsSegment>* instance) {
		delete instance;
	}

	cv::hfs::HfsSegment* cv_PtrOfHfsSegment_get_inner_ptr(cv::Ptr<cv::hfs::HfsSegment>* instance) {
		return instance->get();
	}
}

