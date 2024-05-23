extern "C" {
	// cv::Ptr<cv::AlignExposures>::getInnerPtr() generated
	// ("cv::Ptr<cv::AlignExposures>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::AlignExposures* cv_PtrLcv_AlignExposuresG_getInnerPtr_const(const cv::Ptr<cv::AlignExposures>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AlignExposures>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::AlignExposures>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::AlignExposures* cv_PtrLcv_AlignExposuresG_getInnerPtrMut(cv::Ptr<cv::AlignExposures>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AlignExposures>::new_null() generated
	// ("cv::Ptr<cv::AlignExposures>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::AlignExposures>* cv_PtrLcv_AlignExposuresG_new_null_const() {
			return new cv::Ptr<cv::AlignExposures>();
	}

	// cv::Ptr<cv::AlignExposures>::delete() generated
	// ("cv::Ptr<cv::AlignExposures>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AlignExposuresG_delete(cv::Ptr<cv::AlignExposures>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::AlignExposures>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::AlignExposures>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AlignExposuresG_to_PtrOfAlgorithm(cv::Ptr<cv::AlignExposures>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

