template struct Result<float>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfHistogramPhaseUnwrapping_delete(cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr(cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* instance) {
	return instance->get();
}

