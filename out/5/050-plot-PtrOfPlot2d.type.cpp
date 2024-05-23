extern "C" {
	// cv::Ptr<cv::plot::Plot2d>::getInnerPtr() generated
	// ("cv::Ptr<cv::plot::Plot2d>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::plot::Plot2d* cv_PtrLcv_plot_Plot2dG_getInnerPtr_const(const cv::Ptr<cv::plot::Plot2d>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::plot::Plot2d>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::plot::Plot2d>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::plot::Plot2d* cv_PtrLcv_plot_Plot2dG_getInnerPtrMut(cv::Ptr<cv::plot::Plot2d>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::plot::Plot2d>::new_null() generated
	// ("cv::Ptr<cv::plot::Plot2d>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::plot::Plot2d>* cv_PtrLcv_plot_Plot2dG_new_null_const() {
			return new cv::Ptr<cv::plot::Plot2d>();
	}

	// cv::Ptr<cv::plot::Plot2d>::delete() generated
	// ("cv::Ptr<cv::plot::Plot2d>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_plot_Plot2dG_delete(cv::Ptr<cv::plot::Plot2d>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::plot::Plot2d>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::plot::Plot2d>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_plot_Plot2dG_to_PtrOfAlgorithm(cv::Ptr<cv::plot::Plot2d>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

