extern "C" {
	// cv::Ptr<cv::ximgproc::DisparityWLSFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::DisparityWLSFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::DisparityWLSFilter* cv_PtrLcv_ximgproc_DisparityWLSFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::DisparityWLSFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::DisparityWLSFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::DisparityWLSFilter* cv_PtrLcv_ximgproc_DisparityWLSFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::DisparityWLSFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::DisparityWLSFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::DisparityWLSFilter>* cv_PtrLcv_ximgproc_DisparityWLSFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::DisparityWLSFilter>();
	}

	// cv::Ptr<cv::ximgproc::DisparityWLSFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::DisparityWLSFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_DisparityWLSFilterG_delete(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::DisparityWLSFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::DisparityWLSFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_DisparityWLSFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::DisparityWLSFilter>::to_PtrOfDisparityFilter() generated
	// ("cv::Ptr<cv::ximgproc::DisparityWLSFilter>::to_PtrOfDisparityFilter", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ximgproc::DisparityFilter>* cv_PtrLcv_ximgproc_DisparityWLSFilterG_to_PtrOfDisparityFilter(cv::Ptr<cv::ximgproc::DisparityWLSFilter>* instance) {
			return new cv::Ptr<cv::ximgproc::DisparityFilter>(instance->dynamicCast<cv::ximgproc::DisparityFilter>());
	}

}

