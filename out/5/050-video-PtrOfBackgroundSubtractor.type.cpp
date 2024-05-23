extern "C" {
	// cv::Ptr<cv::BackgroundSubtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::BackgroundSubtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::BackgroundSubtractor* cv_PtrLcv_BackgroundSubtractorG_getInnerPtr_const(const cv::Ptr<cv::BackgroundSubtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BackgroundSubtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::BackgroundSubtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_PtrLcv_BackgroundSubtractorG_getInnerPtrMut(cv::Ptr<cv::BackgroundSubtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BackgroundSubtractor>::new_null() generated
	// ("cv::Ptr<cv::BackgroundSubtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_BackgroundSubtractorG_new_null_const() {
			return new cv::Ptr<cv::BackgroundSubtractor>();
	}

	// cv::Ptr<cv::BackgroundSubtractor>::delete() generated
	// ("cv::Ptr<cv::BackgroundSubtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_BackgroundSubtractorG_delete(cv::Ptr<cv::BackgroundSubtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::BackgroundSubtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::BackgroundSubtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BackgroundSubtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::BackgroundSubtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

