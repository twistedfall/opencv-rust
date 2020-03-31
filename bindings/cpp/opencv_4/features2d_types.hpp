template struct Result<bool>;
template struct Result<cv::AKAZE::DescriptorType>;
template struct Result<cv::AgastFeatureDetector::DetectorType>;
template struct Result<cv::BFMatcher*>;
template struct Result<cv::BOWImgDescriptorExtractor*>;
template struct Result<cv::BOWKMeansTrainer*>;
template struct Result<cv::FastFeatureDetector::DetectorType>;
template struct Result<cv::FlannBasedMatcher*>;
template struct Result<cv::KAZE::DiffusivityType>;
template struct Result<cv::KeyPointsFilter*>;
template struct Result<cv::Mat*>;
template struct Result<cv::ORB::ScoreType>;
template struct Result<cv::Ptr<cv::AKAZE>*>;
template struct Result<cv::Ptr<cv::AgastFeatureDetector>*>;
template struct Result<cv::Ptr<cv::BFMatcher>*>;
template struct Result<cv::Ptr<cv::BRISK>*>;
template struct Result<cv::Ptr<cv::DescriptorMatcher>*>;
template struct Result<cv::Ptr<cv::FastFeatureDetector>*>;
template struct Result<cv::Ptr<cv::Feature2D>*>;
template struct Result<cv::Ptr<cv::FileStorage>*>;
template struct Result<cv::Ptr<cv::FlannBasedMatcher>*>;
template struct Result<cv::Ptr<cv::GFTTDetector>*>;
template struct Result<cv::Ptr<cv::KAZE>*>;
template struct Result<cv::Ptr<cv::MSER>*>;
template struct Result<cv::Ptr<cv::ORB>*>;
template struct Result<cv::Ptr<cv::SimpleBlobDetector>*>;
template struct Result<cv::Ptr<cv::flann::IndexParams>*>;
template struct Result<cv::Ptr<cv::flann::SearchParams>*>;
template struct Result<cv::SimpleBlobDetector::Params>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<char>*>;
template struct Result<std::vector<cv::DMatch>*>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::Point_<float>>*>;
template struct Result<std::vector<cv::Point_<int>>*>;
template struct Result<std::vector<cv::Rect_<int>>*>;
template struct Result<std::vector<float>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::vector<char>>*>;
template struct Result<std::vector<std::vector<cv::DMatch>>*>;
template struct Result<std::vector<std::vector<cv::KeyPoint>>*>;
template struct Result<std::vector<std::vector<cv::Point_<int>>>*>;
template struct Result<std::vector<std::vector<int>>*>;
template struct Result<std::vector<std::vector<unsigned char>>*>;
template struct Result<std::vector<unsigned char>*>;
template struct Result<unsigned char>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" void cv_PtrOfAKAZE_delete(cv::Ptr<cv::AKAZE>* instance) {
	delete instance;
}

extern "C" cv::AKAZE* cv_PtrOfAKAZE_get_inner_ptr(cv::Ptr<cv::AKAZE>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfAgastFeatureDetector_delete(cv::Ptr<cv::AgastFeatureDetector>* instance) {
	delete instance;
}

extern "C" cv::AgastFeatureDetector* cv_PtrOfAgastFeatureDetector_get_inner_ptr(cv::Ptr<cv::AgastFeatureDetector>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBFMatcher_delete(cv::Ptr<cv::BFMatcher>* instance) {
	delete instance;
}

extern "C" cv::BFMatcher* cv_PtrOfBFMatcher_get_inner_ptr(cv::Ptr<cv::BFMatcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBRISK_delete(cv::Ptr<cv::BRISK>* instance) {
	delete instance;
}

extern "C" cv::BRISK* cv_PtrOfBRISK_get_inner_ptr(cv::Ptr<cv::BRISK>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDescriptorMatcher_delete(cv::Ptr<cv::DescriptorMatcher>* instance) {
	delete instance;
}

extern "C" cv::DescriptorMatcher* cv_PtrOfDescriptorMatcher_get_inner_ptr(cv::Ptr<cv::DescriptorMatcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFastFeatureDetector_delete(cv::Ptr<cv::FastFeatureDetector>* instance) {
	delete instance;
}

extern "C" cv::FastFeatureDetector* cv_PtrOfFastFeatureDetector_get_inner_ptr(cv::Ptr<cv::FastFeatureDetector>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFeature2D_delete(cv::Ptr<cv::Feature2D>* instance) {
	delete instance;
}

extern "C" cv::Feature2D* cv_PtrOfFeature2D_get_inner_ptr(cv::Ptr<cv::Feature2D>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFlannBasedMatcher_delete(cv::Ptr<cv::FlannBasedMatcher>* instance) {
	delete instance;
}

extern "C" cv::FlannBasedMatcher* cv_PtrOfFlannBasedMatcher_get_inner_ptr(cv::Ptr<cv::FlannBasedMatcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGFTTDetector_delete(cv::Ptr<cv::GFTTDetector>* instance) {
	delete instance;
}

extern "C" cv::GFTTDetector* cv_PtrOfGFTTDetector_get_inner_ptr(cv::Ptr<cv::GFTTDetector>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfKAZE_delete(cv::Ptr<cv::KAZE>* instance) {
	delete instance;
}

extern "C" cv::KAZE* cv_PtrOfKAZE_get_inner_ptr(cv::Ptr<cv::KAZE>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMSER_delete(cv::Ptr<cv::MSER>* instance) {
	delete instance;
}

extern "C" cv::MSER* cv_PtrOfMSER_get_inner_ptr(cv::Ptr<cv::MSER>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfORB_delete(cv::Ptr<cv::ORB>* instance) {
	delete instance;
}

extern "C" cv::ORB* cv_PtrOfORB_get_inner_ptr(cv::Ptr<cv::ORB>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSimpleBlobDetector_delete(cv::Ptr<cv::SimpleBlobDetector>* instance) {
	delete instance;
}

extern "C" cv::SimpleBlobDetector* cv_PtrOfSimpleBlobDetector_get_inner_ptr(cv::Ptr<cv::SimpleBlobDetector>* instance) {
	return instance->get();
}

