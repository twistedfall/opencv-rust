extern "C" {
	// cv::Ptr<cv::AKAZE>::getInnerPtr() generated
	// ("cv::Ptr<cv::AKAZE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::AKAZE* cv_PtrLcv_AKAZEG_getInnerPtr_const(const cv::Ptr<cv::AKAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AKAZE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::AKAZE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::AKAZE* cv_PtrLcv_AKAZEG_getInnerPtrMut(cv::Ptr<cv::AKAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AKAZE>::new_null() generated
	// ("cv::Ptr<cv::AKAZE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::AKAZE>* cv_PtrLcv_AKAZEG_new_null_const() {
			return new cv::Ptr<cv::AKAZE>();
	}

	// cv::Ptr<cv::AKAZE>::delete() generated
	// ("cv::Ptr<cv::AKAZE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AKAZEG_delete(cv::Ptr<cv::AKAZE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::AKAZE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::AKAZE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AKAZEG_to_PtrOfAlgorithm(cv::Ptr<cv::AKAZE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::AKAZE>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::AKAZE>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_AKAZEG_to_PtrOfFeature2D(cv::Ptr<cv::AKAZE>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

