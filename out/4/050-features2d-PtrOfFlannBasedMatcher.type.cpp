extern "C" {
	// cv::Ptr<cv::FlannBasedMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::FlannBasedMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::FlannBasedMatcher* cv_PtrLcv_FlannBasedMatcherG_getInnerPtr_const(const cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FlannBasedMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::FlannBasedMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::FlannBasedMatcher* cv_PtrLcv_FlannBasedMatcherG_getInnerPtrMut(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FlannBasedMatcher>::new_null() generated
	// ("cv::Ptr<cv::FlannBasedMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::FlannBasedMatcher>* cv_PtrLcv_FlannBasedMatcherG_new_null_const() {
			return new cv::Ptr<cv::FlannBasedMatcher>();
	}

	// cv::Ptr<cv::FlannBasedMatcher>::delete() generated
	// ("cv::Ptr<cv::FlannBasedMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FlannBasedMatcherG_delete(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::FlannBasedMatcher>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::FlannBasedMatcher>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_FlannBasedMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::FlannBasedMatcher>::to_PtrOfDescriptorMatcher() generated
	// ("cv::Ptr<cv::FlannBasedMatcher>::to_PtrOfDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DescriptorMatcher>* cv_PtrLcv_FlannBasedMatcherG_to_PtrOfDescriptorMatcher(cv::Ptr<cv::FlannBasedMatcher>* instance) {
			return new cv::Ptr<cv::DescriptorMatcher>(instance->dynamicCast<cv::DescriptorMatcher>());
	}

	// cv::Ptr<cv::FlannBasedMatcher>::new(TraitClass) generated
	// ("cv::Ptr<cv::FlannBasedMatcher>::new", vec![(pred!(const, ["val"], ["cv::FlannBasedMatcher"]), _)]),
	cv::Ptr<cv::FlannBasedMatcher>* cv_PtrLcv_FlannBasedMatcherG_new_const_FlannBasedMatcher(cv::FlannBasedMatcher* val) {
			return new cv::Ptr<cv::FlannBasedMatcher>(val);
	}

}

