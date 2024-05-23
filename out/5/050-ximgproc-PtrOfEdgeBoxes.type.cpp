extern "C" {
	// cv::Ptr<cv::ximgproc::EdgeBoxes>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::EdgeBoxes>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::EdgeBoxes* cv_PtrLcv_ximgproc_EdgeBoxesG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::EdgeBoxes>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::EdgeBoxes>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::EdgeBoxes* cv_PtrLcv_ximgproc_EdgeBoxesG_getInnerPtrMut(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::EdgeBoxes>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::EdgeBoxes>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::EdgeBoxes>* cv_PtrLcv_ximgproc_EdgeBoxesG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::EdgeBoxes>();
	}

	// cv::Ptr<cv::ximgproc::EdgeBoxes>::delete() generated
	// ("cv::Ptr<cv::ximgproc::EdgeBoxes>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_EdgeBoxesG_delete(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::EdgeBoxes>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::EdgeBoxes>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_EdgeBoxesG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

