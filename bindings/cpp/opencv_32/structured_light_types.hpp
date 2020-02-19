template struct Result<bool>;
template struct Result<float>;
template struct Result<int>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" void cv_PtrOfGrayCodePattern_delete(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfGrayCodePattern_get_inner_ptr(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSinusoidalPattern_delete(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSinusoidalPattern_get_inner_ptr(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
	return instance->get();
}

