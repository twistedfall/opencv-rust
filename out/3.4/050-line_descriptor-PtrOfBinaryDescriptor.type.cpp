extern "C" {
	// cv::Ptr<cv::line_descriptor::BinaryDescriptor>::getInnerPtr() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::line_descriptor::BinaryDescriptor* cv_PtrLcv_line_descriptor_BinaryDescriptorG_getInnerPtr_const(const cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::line_descriptor::BinaryDescriptor* cv_PtrLcv_line_descriptor_BinaryDescriptorG_getInnerPtrMut(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptor>::new_null() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::line_descriptor::BinaryDescriptor>* cv_PtrLcv_line_descriptor_BinaryDescriptorG_new_null_const() {
			return new cv::Ptr<cv::line_descriptor::BinaryDescriptor>();
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptor>::delete() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_line_descriptor_BinaryDescriptorG_delete(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_line_descriptor_BinaryDescriptorG_to_PtrOfAlgorithm(cv::Ptr<cv::line_descriptor::BinaryDescriptor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::line_descriptor::BinaryDescriptor>::new(TraitClass) generated
	// ("cv::Ptr<cv::line_descriptor::BinaryDescriptor>::new", vec![(pred!(const, ["val"], ["cv::line_descriptor::BinaryDescriptor"]), _)]),
	cv::Ptr<cv::line_descriptor::BinaryDescriptor>* cv_PtrLcv_line_descriptor_BinaryDescriptorG_new_const_BinaryDescriptor(cv::line_descriptor::BinaryDescriptor* val) {
			return new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(val);
	}

}

