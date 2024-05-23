extern "C" {
	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::getInnerPtr() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bgsegm::BackgroundSubtractorGMG* cv_PtrLcv_bgsegm_BackgroundSubtractorGMGG_getInnerPtr_const(const cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bgsegm::BackgroundSubtractorGMG* cv_PtrLcv_bgsegm_BackgroundSubtractorGMGG_getInnerPtrMut(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::new_null() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* cv_PtrLcv_bgsegm_BackgroundSubtractorGMGG_new_null_const() {
			return new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::delete() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bgsegm_BackgroundSubtractorGMGG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bgsegm_BackgroundSubtractorGMGG_to_PtrOfAlgorithm(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_bgsegm_BackgroundSubtractorGMGG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

