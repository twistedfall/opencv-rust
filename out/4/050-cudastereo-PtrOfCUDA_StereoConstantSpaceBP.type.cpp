extern "C" {
	// cv::Ptr<cv::cuda::StereoConstantSpaceBP>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::StereoConstantSpaceBP>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::StereoConstantSpaceBP* cv_PtrLcv_cuda_StereoConstantSpaceBPG_getInnerPtr_const(const cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoConstantSpaceBP>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::StereoConstantSpaceBP>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::StereoConstantSpaceBP* cv_PtrLcv_cuda_StereoConstantSpaceBPG_getInnerPtrMut(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoConstantSpaceBP>::new_null() generated
	// ("cv::Ptr<cv::cuda::StereoConstantSpaceBP>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::StereoConstantSpaceBP>* cv_PtrLcv_cuda_StereoConstantSpaceBPG_new_null_const() {
			return new cv::Ptr<cv::cuda::StereoConstantSpaceBP>();
	}

	// cv::Ptr<cv::cuda::StereoConstantSpaceBP>::delete() generated
	// ("cv::Ptr<cv::cuda::StereoConstantSpaceBP>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_StereoConstantSpaceBPG_delete(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::StereoConstantSpaceBP>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::StereoConstantSpaceBP>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_StereoConstantSpaceBPG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::StereoConstantSpaceBP>::to_PtrOfStereoMatcher() generated
	// ("cv::Ptr<cv::cuda::StereoConstantSpaceBP>::to_PtrOfStereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_cuda_StereoConstantSpaceBPG_to_PtrOfStereoMatcher(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}

	// cv::Ptr<cv::cuda::StereoConstantSpaceBP>::to_PtrOfCUDA_StereoBeliefPropagation() generated
	// ("cv::Ptr<cv::cuda::StereoConstantSpaceBP>::to_PtrOfCUDA_StereoBeliefPropagation", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::StereoBeliefPropagation>* cv_PtrLcv_cuda_StereoConstantSpaceBPG_to_PtrOfCUDA_StereoBeliefPropagation(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
			return new cv::Ptr<cv::cuda::StereoBeliefPropagation>(instance->dynamicCast<cv::cuda::StereoBeliefPropagation>());
	}

}

