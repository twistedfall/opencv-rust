extern "C" {
	// cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::AdaptiveManifoldFilter* cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::AdaptiveManifoldFilter* cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>();
	}

	// cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_delete(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_AdaptiveManifoldFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

