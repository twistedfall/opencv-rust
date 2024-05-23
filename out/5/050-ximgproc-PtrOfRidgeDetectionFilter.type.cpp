extern "C" {
	// cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::RidgeDetectionFilter* cv_PtrLcv_ximgproc_RidgeDetectionFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::RidgeDetectionFilter* cv_PtrLcv_ximgproc_RidgeDetectionFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* cv_PtrLcv_ximgproc_RidgeDetectionFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::RidgeDetectionFilter>();
	}

	// cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_RidgeDetectionFilterG_delete(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::RidgeDetectionFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_RidgeDetectionFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::RidgeDetectionFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

