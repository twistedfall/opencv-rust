extern "C" {
	// cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::line_descriptor::BinaryDescriptorMatcher* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_getInnerPtr_const(const cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::line_descriptor::BinaryDescriptorMatcher* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_getInnerPtrMut(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::new_null() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_new_null_const() {
			return new cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>();
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::delete() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_delete(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::new(TraitClass) generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>::new", vec![(pred!(const, ["val"], ["cv::line_descriptor::BinaryDescriptorMatcher"]), _)]),
	cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>* cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_new_const_BinaryDescriptorMatcher(cv::line_descriptor::BinaryDescriptorMatcher* val) {
			return new cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>(val);
	}

}

