template struct Result<void*>;
extern "C" void cv_PtrOfPlot2d_delete(cv::Ptr<cv::plot::Plot2d>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfPlot2d_get_inner_ptr(cv::Ptr<cv::plot::Plot2d>* instance) {
	return instance->get();
}

