extern "C" {
	// cv::Ptr<cv::StereoMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::StereoMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::StereoMatcher* cv_PtrLcv_StereoMatcherG_getInnerPtr_const(const cv::Ptr<cv::StereoMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereoMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::StereoMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::StereoMatcher* cv_PtrLcv_StereoMatcherG_getInnerPtrMut(cv::Ptr<cv::StereoMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereoMatcher>::new_null() generated
	// ("cv::Ptr<cv::StereoMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_StereoMatcherG_new_null_const() {
			return new cv::Ptr<cv::StereoMatcher>();
	}

	// cv::Ptr<cv::StereoMatcher>::delete() generated
	// ("cv::Ptr<cv::StereoMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_StereoMatcherG_delete(cv::Ptr<cv::StereoMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::StereoMatcher>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::StereoMatcher>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_StereoMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::StereoMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

