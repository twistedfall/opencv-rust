extern "C" {
	// cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::getInnerPtr() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bgsegm::BackgroundSubtractorLSBP* cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_getInnerPtr_const(const cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bgsegm::BackgroundSubtractorLSBP* cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_getInnerPtrMut(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::new_null() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_new_null_const() {
			return new cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::delete() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_to_PtrOfAlgorithm(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_bgsegm_BackgroundSubtractorLSBPG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

