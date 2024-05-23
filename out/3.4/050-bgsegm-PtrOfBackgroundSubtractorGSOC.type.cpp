extern "C" {
	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::getInnerPtr() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bgsegm::BackgroundSubtractorGSOC* cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_getInnerPtr_const(const cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bgsegm::BackgroundSubtractorGSOC* cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_getInnerPtrMut(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::new_null() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_new_null_const() {
			return new cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::delete() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_to_PtrOfAlgorithm(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_bgsegm_BackgroundSubtractorGSOCG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

