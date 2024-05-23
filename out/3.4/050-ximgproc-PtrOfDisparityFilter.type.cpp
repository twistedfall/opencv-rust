extern "C" {
	// cv::Ptr<cv::ximgproc::DisparityFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::DisparityFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::DisparityFilter* cv_PtrLcv_ximgproc_DisparityFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::DisparityFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::DisparityFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::DisparityFilter* cv_PtrLcv_ximgproc_DisparityFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::DisparityFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::DisparityFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::DisparityFilter>* cv_PtrLcv_ximgproc_DisparityFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::DisparityFilter>();
	}

	// cv::Ptr<cv::ximgproc::DisparityFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::DisparityFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_DisparityFilterG_delete(cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::DisparityFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::DisparityFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_DisparityFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::DisparityFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

