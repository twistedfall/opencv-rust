extern "C" {
	// cv::Ptr<cv::KAZE>::getInnerPtr() generated
	// ("cv::Ptr<cv::KAZE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::KAZE* cv_PtrLcv_KAZEG_getInnerPtr_const(const cv::Ptr<cv::KAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::KAZE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::KAZE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::KAZE* cv_PtrLcv_KAZEG_getInnerPtrMut(cv::Ptr<cv::KAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::KAZE>::new_null() generated
	// ("cv::Ptr<cv::KAZE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::KAZE>* cv_PtrLcv_KAZEG_new_null_const() {
			return new cv::Ptr<cv::KAZE>();
	}

	// cv::Ptr<cv::KAZE>::delete() generated
	// ("cv::Ptr<cv::KAZE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_KAZEG_delete(cv::Ptr<cv::KAZE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::KAZE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::KAZE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_KAZEG_to_PtrOfAlgorithm(cv::Ptr<cv::KAZE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::KAZE>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::KAZE>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_KAZEG_to_PtrOfFeature2D(cv::Ptr<cv::KAZE>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

