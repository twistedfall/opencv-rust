template struct Result<bool>;
template struct Result<cv::Ptr<cv::cuda::DescriptorMatcher>*>;
template struct Result<cv::Ptr<cv::cuda::FastFeatureDetector>*>;
template struct Result<cv::Ptr<cv::cuda::ORB>*>;
template struct Result<int>;
template struct Result<std::vector<cv::DMatch>*>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<cv::cuda::GpuMat>*>;
template struct Result<std::vector<std::vector<cv::DMatch>>*>;
extern "C" void cv_PtrOfCUDA_DescriptorMatcher_delete(cv::Ptr<cv::cuda::DescriptorMatcher>* instance) {
	delete instance;
}

extern "C" cv::cuda::DescriptorMatcher* cv_PtrOfCUDA_DescriptorMatcher_get_inner_ptr(cv::Ptr<cv::cuda::DescriptorMatcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCUDA_FastFeatureDetector_delete(cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
	delete instance;
}

extern "C" cv::cuda::FastFeatureDetector* cv_PtrOfCUDA_FastFeatureDetector_get_inner_ptr(cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCUDA_ORB_delete(cv::Ptr<cv::cuda::ORB>* instance) {
	delete instance;
}

extern "C" cv::cuda::ORB* cv_PtrOfCUDA_ORB_get_inner_ptr(cv::Ptr<cv::cuda::ORB>* instance) {
	return instance->get();
}

