extern "C" {
	// cv::Ptr<cv::BackgroundSubtractorMOG2>::getInnerPtr() generated
	// ("cv::Ptr<cv::BackgroundSubtractorMOG2>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::BackgroundSubtractorMOG2* cv_PtrLcv_BackgroundSubtractorMOG2G_getInnerPtr_const(const cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BackgroundSubtractorMOG2>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::BackgroundSubtractorMOG2>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractorMOG2* cv_PtrLcv_BackgroundSubtractorMOG2G_getInnerPtrMut(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BackgroundSubtractorMOG2>::new_null() generated
	// ("cv::Ptr<cv::BackgroundSubtractorMOG2>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractorMOG2>* cv_PtrLcv_BackgroundSubtractorMOG2G_new_null_const() {
			return new cv::Ptr<cv::BackgroundSubtractorMOG2>();
	}

	// cv::Ptr<cv::BackgroundSubtractorMOG2>::delete() generated
	// ("cv::Ptr<cv::BackgroundSubtractorMOG2>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_BackgroundSubtractorMOG2G_delete(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::BackgroundSubtractorMOG2>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::BackgroundSubtractorMOG2>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BackgroundSubtractorMOG2G_to_PtrOfAlgorithm(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::BackgroundSubtractorMOG2>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::BackgroundSubtractorMOG2>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_BackgroundSubtractorMOG2G_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

