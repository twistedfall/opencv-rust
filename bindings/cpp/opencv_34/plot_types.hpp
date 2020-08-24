template struct Result<cv::Ptr<cv::plot::Plot2d>*>;
extern "C" {
	void cv_PtrOfPlot2d_delete(cv::Ptr<cv::plot::Plot2d>* instance) {
		delete instance;
	}

	const cv::plot::Plot2d* cv_PtrOfPlot2d_get_inner_ptr(const cv::Ptr<cv::plot::Plot2d>* instance) {
		return instance->get();
	}

	cv::plot::Plot2d* cv_PtrOfPlot2d_get_inner_ptr_mut(cv::Ptr<cv::plot::Plot2d>* instance) {
		return instance->get();
	}
}

