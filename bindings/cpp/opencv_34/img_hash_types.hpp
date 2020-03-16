template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::img_hash::AverageHash>*>;
template struct Result<cv::Ptr<cv::img_hash::BlockMeanHash>*>;
template struct Result<cv::Ptr<cv::img_hash::ColorMomentHash>*>;
template struct Result<cv::Ptr<cv::img_hash::MarrHildrethHash>*>;
template struct Result<cv::Ptr<cv::img_hash::PHash>*>;
template struct Result<cv::Ptr<cv::img_hash::RadialVarianceHash>*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<double>*>;
extern "C" void cv_PtrOfAverageHash_delete(cv::Ptr<cv::img_hash::AverageHash>* instance) {
	delete instance;
}

extern "C" cv::img_hash::AverageHash* cv_PtrOfAverageHash_get_inner_ptr(cv::Ptr<cv::img_hash::AverageHash>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBlockMeanHash_delete(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
	delete instance;
}

extern "C" cv::img_hash::BlockMeanHash* cv_PtrOfBlockMeanHash_get_inner_ptr(cv::Ptr<cv::img_hash::BlockMeanHash>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfColorMomentHash_delete(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
	delete instance;
}

extern "C" cv::img_hash::ColorMomentHash* cv_PtrOfColorMomentHash_get_inner_ptr(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMarrHildrethHash_delete(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
	delete instance;
}

extern "C" cv::img_hash::MarrHildrethHash* cv_PtrOfMarrHildrethHash_get_inner_ptr(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfPHash_delete(cv::Ptr<cv::img_hash::PHash>* instance) {
	delete instance;
}

extern "C" cv::img_hash::PHash* cv_PtrOfPHash_get_inner_ptr(cv::Ptr<cv::img_hash::PHash>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfRadialVarianceHash_delete(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
	delete instance;
}

extern "C" cv::img_hash::RadialVarianceHash* cv_PtrOfRadialVarianceHash_get_inner_ptr(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
	return instance->get();
}

