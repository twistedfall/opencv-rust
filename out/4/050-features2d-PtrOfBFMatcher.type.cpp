extern "C" {
	// cv::Ptr<cv::BFMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::BFMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::BFMatcher* cv_PtrLcv_BFMatcherG_getInnerPtr_const(const cv::Ptr<cv::BFMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BFMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::BFMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::BFMatcher* cv_PtrLcv_BFMatcherG_getInnerPtrMut(cv::Ptr<cv::BFMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BFMatcher>::new_null() generated
	// ("cv::Ptr<cv::BFMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::BFMatcher>* cv_PtrLcv_BFMatcherG_new_null_const() {
			return new cv::Ptr<cv::BFMatcher>();
	}

	// cv::Ptr<cv::BFMatcher>::delete() generated
	// ("cv::Ptr<cv::BFMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_BFMatcherG_delete(cv::Ptr<cv::BFMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::BFMatcher>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::BFMatcher>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BFMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::BFMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::BFMatcher>::to_PtrOfDescriptorMatcher() generated
	// ("cv::Ptr<cv::BFMatcher>::to_PtrOfDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DescriptorMatcher>* cv_PtrLcv_BFMatcherG_to_PtrOfDescriptorMatcher(cv::Ptr<cv::BFMatcher>* instance) {
			return new cv::Ptr<cv::DescriptorMatcher>(instance->dynamicCast<cv::DescriptorMatcher>());
	}

	// cv::Ptr<cv::BFMatcher>::new(TraitClass) generated
	// ("cv::Ptr<cv::BFMatcher>::new", vec![(pred!(const, ["val"], ["cv::BFMatcher"]), _)]),
	cv::Ptr<cv::BFMatcher>* cv_PtrLcv_BFMatcherG_new_const_BFMatcher(cv::BFMatcher* val) {
			return new cv::Ptr<cv::BFMatcher>(val);
	}

}

