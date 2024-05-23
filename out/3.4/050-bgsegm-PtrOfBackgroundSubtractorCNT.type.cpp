extern "C" {
	// cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::getInnerPtr() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bgsegm::BackgroundSubtractorCNT* cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_getInnerPtr_const(const cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bgsegm::BackgroundSubtractorCNT* cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_getInnerPtrMut(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::new_null() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_new_null_const() {
			return new cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>();
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::delete() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_delete(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_to_PtrOfAlgorithm(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_bgsegm_BackgroundSubtractorCNTG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

