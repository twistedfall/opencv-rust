extern "C" {
	// cv::Ptr<cv::cuda::DescriptorMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::DescriptorMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::DescriptorMatcher* cv_PtrLcv_cuda_DescriptorMatcherG_getInnerPtr_const(const cv::Ptr<cv::cuda::DescriptorMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DescriptorMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::DescriptorMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DescriptorMatcher* cv_PtrLcv_cuda_DescriptorMatcherG_getInnerPtrMut(cv::Ptr<cv::cuda::DescriptorMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DescriptorMatcher>::new_null() generated
	// ("cv::Ptr<cv::cuda::DescriptorMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::DescriptorMatcher>* cv_PtrLcv_cuda_DescriptorMatcherG_new_null_const() {
			return new cv::Ptr<cv::cuda::DescriptorMatcher>();
	}

	// cv::Ptr<cv::cuda::DescriptorMatcher>::delete() generated
	// ("cv::Ptr<cv::cuda::DescriptorMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_DescriptorMatcherG_delete(cv::Ptr<cv::cuda::DescriptorMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::DescriptorMatcher>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::DescriptorMatcher>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_DescriptorMatcherG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::DescriptorMatcher>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

