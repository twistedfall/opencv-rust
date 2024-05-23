extern "C" {
	// cv::Ptr<cv::ximgproc::FastLineDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::FastLineDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::FastLineDetector* cv_PtrLcv_ximgproc_FastLineDetectorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::FastLineDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::FastLineDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::FastLineDetector* cv_PtrLcv_ximgproc_FastLineDetectorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::FastLineDetector>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::FastLineDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::FastLineDetector>* cv_PtrLcv_ximgproc_FastLineDetectorG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::FastLineDetector>();
	}

	// cv::Ptr<cv::ximgproc::FastLineDetector>::delete() generated
	// ("cv::Ptr<cv::ximgproc::FastLineDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_FastLineDetectorG_delete(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::FastLineDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::FastLineDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_FastLineDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::FastLineDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

