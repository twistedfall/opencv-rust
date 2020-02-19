template struct Result<bool>;
template struct Result<const char*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>;
template struct Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>;
template struct Result<cv::bioinspired::SegmentationParameters>;
template struct Result<float>;
template struct Result<void*>;
extern "C" void cv_PtrOfRetina_delete(cv::Ptr<cv::bioinspired::Retina>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfRetina_get_inner_ptr(cv::Ptr<cv::bioinspired::Retina>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfRetinaFastToneMapping_delete(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfRetinaFastToneMapping_get_inner_ptr(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTransientAreasSegmentationModule_delete(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
	return instance->get();
}

