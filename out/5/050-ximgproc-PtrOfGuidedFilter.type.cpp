extern "C" {
	// cv::Ptr<cv::ximgproc::GuidedFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::GuidedFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::GuidedFilter* cv_PtrLcv_ximgproc_GuidedFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::GuidedFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::GuidedFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::GuidedFilter* cv_PtrLcv_ximgproc_GuidedFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::GuidedFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::GuidedFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::GuidedFilter>* cv_PtrLcv_ximgproc_GuidedFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::GuidedFilter>();
	}

	// cv::Ptr<cv::ximgproc::GuidedFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::GuidedFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_GuidedFilterG_delete(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::GuidedFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::GuidedFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_GuidedFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::GuidedFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

