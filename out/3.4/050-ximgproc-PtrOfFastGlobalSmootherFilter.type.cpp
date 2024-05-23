extern "C" {
	// cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::FastGlobalSmootherFilter* cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::FastGlobalSmootherFilter* cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>();
	}

	// cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_delete(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_FastGlobalSmootherFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

