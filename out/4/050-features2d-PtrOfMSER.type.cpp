extern "C" {
	// cv::Ptr<cv::MSER>::getInnerPtr() generated
	// ("cv::Ptr<cv::MSER>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MSER* cv_PtrLcv_MSERG_getInnerPtr_const(const cv::Ptr<cv::MSER>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MSER>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MSER>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MSER* cv_PtrLcv_MSERG_getInnerPtrMut(cv::Ptr<cv::MSER>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MSER>::new_null() generated
	// ("cv::Ptr<cv::MSER>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MSER>* cv_PtrLcv_MSERG_new_null_const() {
			return new cv::Ptr<cv::MSER>();
	}

	// cv::Ptr<cv::MSER>::delete() generated
	// ("cv::Ptr<cv::MSER>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MSERG_delete(cv::Ptr<cv::MSER>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MSER>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::MSER>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MSERG_to_PtrOfAlgorithm(cv::Ptr<cv::MSER>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::MSER>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::MSER>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_MSERG_to_PtrOfFeature2D(cv::Ptr<cv::MSER>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

