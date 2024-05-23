extern "C" {
	// cv::Ptr<cv::cuda::StereoBeliefPropagation>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::StereoBeliefPropagation>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::StereoBeliefPropagation* cv_PtrLcv_cuda_StereoBeliefPropagationG_getInnerPtr_const(const cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoBeliefPropagation>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::StereoBeliefPropagation>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::StereoBeliefPropagation* cv_PtrLcv_cuda_StereoBeliefPropagationG_getInnerPtrMut(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoBeliefPropagation>::new_null() generated
	// ("cv::Ptr<cv::cuda::StereoBeliefPropagation>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::StereoBeliefPropagation>* cv_PtrLcv_cuda_StereoBeliefPropagationG_new_null_const() {
			return new cv::Ptr<cv::cuda::StereoBeliefPropagation>();
	}

	// cv::Ptr<cv::cuda::StereoBeliefPropagation>::delete() generated
	// ("cv::Ptr<cv::cuda::StereoBeliefPropagation>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_StereoBeliefPropagationG_delete(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::StereoBeliefPropagation>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::StereoBeliefPropagation>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_StereoBeliefPropagationG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::StereoBeliefPropagation>::to_PtrOfStereoMatcher() generated
	// ("cv::Ptr<cv::cuda::StereoBeliefPropagation>::to_PtrOfStereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_cuda_StereoBeliefPropagationG_to_PtrOfStereoMatcher(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}

}

