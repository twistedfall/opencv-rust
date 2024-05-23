extern "C" {
	// cv::Ptr<cv::MergeExposures>::getInnerPtr() generated
	// ("cv::Ptr<cv::MergeExposures>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MergeExposures* cv_PtrLcv_MergeExposuresG_getInnerPtr_const(const cv::Ptr<cv::MergeExposures>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeExposures>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MergeExposures>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MergeExposures* cv_PtrLcv_MergeExposuresG_getInnerPtrMut(cv::Ptr<cv::MergeExposures>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeExposures>::new_null() generated
	// ("cv::Ptr<cv::MergeExposures>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MergeExposures>* cv_PtrLcv_MergeExposuresG_new_null_const() {
			return new cv::Ptr<cv::MergeExposures>();
	}

	// cv::Ptr<cv::MergeExposures>::delete() generated
	// ("cv::Ptr<cv::MergeExposures>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MergeExposuresG_delete(cv::Ptr<cv::MergeExposures>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MergeExposures>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::MergeExposures>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeExposuresG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeExposures>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

