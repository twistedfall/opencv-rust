extern "C" {
	// cv::Ptr<cv::mcc::CChecker>::getInnerPtr() generated
	// ("cv::Ptr<cv::mcc::CChecker>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::mcc::CChecker* cv_PtrLcv_mcc_CCheckerG_getInnerPtr_const(const cv::Ptr<cv::mcc::CChecker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::mcc::CChecker>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::mcc::CChecker>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::mcc::CChecker* cv_PtrLcv_mcc_CCheckerG_getInnerPtrMut(cv::Ptr<cv::mcc::CChecker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::mcc::CChecker>::new_null() generated
	// ("cv::Ptr<cv::mcc::CChecker>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::mcc::CChecker>* cv_PtrLcv_mcc_CCheckerG_new_null_const() {
			return new cv::Ptr<cv::mcc::CChecker>();
	}

	// cv::Ptr<cv::mcc::CChecker>::delete() generated
	// ("cv::Ptr<cv::mcc::CChecker>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_mcc_CCheckerG_delete(cv::Ptr<cv::mcc::CChecker>* instance) {
			delete instance;
	}

}

