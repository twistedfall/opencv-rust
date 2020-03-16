template struct Result<bool>;
template struct Result<cv::Ptr<cv::structured_light::GrayCodePattern>*>;
template struct Result<cv::Ptr<cv::structured_light::SinusoidalPattern::Params>*>;
template struct Result<cv::Ptr<cv::structured_light::SinusoidalPattern>*>;
template struct Result<cv::structured_light::GrayCodePattern::Params*>;
template struct Result<cv::structured_light::SinusoidalPattern::Params*>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::Point_<float>>*>;
template struct Result<std::vector<std::vector<cv::Mat>>*>;
template struct Result<unsigned long>;
extern "C" void cv_PtrOfGrayCodePattern_delete(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
	delete instance;
}

extern "C" cv::structured_light::GrayCodePattern* cv_PtrOfGrayCodePattern_get_inner_ptr(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSinusoidalPattern_delete(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
	delete instance;
}

extern "C" cv::structured_light::SinusoidalPattern* cv_PtrOfSinusoidalPattern_get_inner_ptr(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSinusoidalPattern_Params_delete(cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* instance) {
	delete instance;
}

extern "C" cv::structured_light::SinusoidalPattern::Params* cv_PtrOfSinusoidalPattern_Params_get_inner_ptr(cv::Ptr<cv::structured_light::SinusoidalPattern::Params>* instance) {
	return instance->get();
}

