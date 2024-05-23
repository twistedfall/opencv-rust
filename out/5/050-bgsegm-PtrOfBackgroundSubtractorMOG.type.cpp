extern "C" {
	// cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::getInnerPtr() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bgsegm::BackgroundSubtractorMOG* cv_PtrLcv_bgsegm_BackgroundSubtractorMOGG_getInnerPtr_const(const cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bgsegm::BackgroundSubtractorMOG* cv_PtrLcv_bgsegm_BackgroundSubtractorMOGG_getInnerPtrMut(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::new_null() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* cv_PtrLcv_bgsegm_BackgroundSubtractorMOGG_new_null_const() {
			return new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::delete() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bgsegm_BackgroundSubtractorMOGG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bgsegm_BackgroundSubtractorMOGG_to_PtrOfAlgorithm(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_bgsegm_BackgroundSubtractorMOGG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

