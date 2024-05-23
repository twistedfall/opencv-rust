extern "C" {
	// cv::Ptr<cv::DescriptorMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::DescriptorMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::DescriptorMatcher* cv_PtrLcv_DescriptorMatcherG_getInnerPtr_const(const cv::Ptr<cv::DescriptorMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DescriptorMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::DescriptorMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::DescriptorMatcher* cv_PtrLcv_DescriptorMatcherG_getInnerPtrMut(cv::Ptr<cv::DescriptorMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DescriptorMatcher>::new_null() generated
	// ("cv::Ptr<cv::DescriptorMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::DescriptorMatcher>* cv_PtrLcv_DescriptorMatcherG_new_null_const() {
			return new cv::Ptr<cv::DescriptorMatcher>();
	}

	// cv::Ptr<cv::DescriptorMatcher>::delete() generated
	// ("cv::Ptr<cv::DescriptorMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_DescriptorMatcherG_delete(cv::Ptr<cv::DescriptorMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::DescriptorMatcher>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::DescriptorMatcher>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DescriptorMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::DescriptorMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

