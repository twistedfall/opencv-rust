template struct Result<bool>;
template struct Result<cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>*>;
template struct Result<cv::Ptr<cv::saliency::ObjectnessBING>*>;
template struct Result<cv::Ptr<cv::saliency::StaticSaliencyFineGrained>*>;
template struct Result<cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>*>;
template struct Result<cv::saliency::MotionSaliencyBinWangApr2014*>;
template struct Result<cv::saliency::ObjectnessBING*>;
template struct Result<cv::saliency::StaticSaliencyFineGrained*>;
template struct Result<cv::saliency::StaticSaliencySpectralResidual*>;
template struct Result<double>;
template struct Result<int>;
template struct Result<std::vector<float>*>;
extern "C" void cv_PtrOfMotionSaliencyBinWangApr2014_delete(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
	delete instance;
}

extern "C" cv::saliency::MotionSaliencyBinWangApr2014* cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfObjectnessBING_delete(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
	delete instance;
}

extern "C" cv::saliency::ObjectnessBING* cv_PtrOfObjectnessBING_get_inner_ptr(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfStaticSaliencyFineGrained_delete(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
	delete instance;
}

extern "C" cv::saliency::StaticSaliencyFineGrained* cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfStaticSaliencySpectralResidual_delete(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
	delete instance;
}

extern "C" cv::saliency::StaticSaliencySpectralResidual* cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
	return instance->get();
}

