extern "C" {
	// cv::Ptr<cv::BackgroundSubtractorKNN>::getInnerPtr() generated
	// ("cv::Ptr<cv::BackgroundSubtractorKNN>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::BackgroundSubtractorKNN* cv_PtrLcv_BackgroundSubtractorKNNG_getInnerPtr_const(const cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BackgroundSubtractorKNN>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::BackgroundSubtractorKNN>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractorKNN* cv_PtrLcv_BackgroundSubtractorKNNG_getInnerPtrMut(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BackgroundSubtractorKNN>::new_null() generated
	// ("cv::Ptr<cv::BackgroundSubtractorKNN>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractorKNN>* cv_PtrLcv_BackgroundSubtractorKNNG_new_null_const() {
			return new cv::Ptr<cv::BackgroundSubtractorKNN>();
	}

	// cv::Ptr<cv::BackgroundSubtractorKNN>::delete() generated
	// ("cv::Ptr<cv::BackgroundSubtractorKNN>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_BackgroundSubtractorKNNG_delete(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::BackgroundSubtractorKNN>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::BackgroundSubtractorKNN>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BackgroundSubtractorKNNG_to_PtrOfAlgorithm(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::BackgroundSubtractorKNN>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::BackgroundSubtractorKNN>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_BackgroundSubtractorKNNG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

