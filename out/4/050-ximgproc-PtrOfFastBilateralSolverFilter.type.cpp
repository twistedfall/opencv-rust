extern "C" {
	// cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::FastBilateralSolverFilter* cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::FastBilateralSolverFilter* cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>();
	}

	// cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_delete(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

