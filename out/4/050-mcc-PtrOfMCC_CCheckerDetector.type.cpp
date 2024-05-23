extern "C" {
	// cv::Ptr<cv::mcc::CCheckerDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::mcc::CCheckerDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::mcc::CCheckerDetector* cv_PtrLcv_mcc_CCheckerDetectorG_getInnerPtr_const(const cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::mcc::CCheckerDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::mcc::CCheckerDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::mcc::CCheckerDetector* cv_PtrLcv_mcc_CCheckerDetectorG_getInnerPtrMut(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::mcc::CCheckerDetector>::new_null() generated
	// ("cv::Ptr<cv::mcc::CCheckerDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::mcc::CCheckerDetector>* cv_PtrLcv_mcc_CCheckerDetectorG_new_null_const() {
			return new cv::Ptr<cv::mcc::CCheckerDetector>();
	}

	// cv::Ptr<cv::mcc::CCheckerDetector>::delete() generated
	// ("cv::Ptr<cv::mcc::CCheckerDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_mcc_CCheckerDetectorG_delete(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::mcc::CCheckerDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::mcc::CCheckerDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_mcc_CCheckerDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

