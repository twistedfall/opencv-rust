extern "C" {
	// cv::Ptr<cv::ximgproc::DTFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::DTFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::DTFilter* cv_PtrLcv_ximgproc_DTFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::DTFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::DTFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::DTFilter* cv_PtrLcv_ximgproc_DTFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::DTFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::DTFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::DTFilter>* cv_PtrLcv_ximgproc_DTFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::DTFilter>();
	}

	// cv::Ptr<cv::ximgproc::DTFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::DTFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_DTFilterG_delete(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::DTFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::DTFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_DTFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::DTFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

