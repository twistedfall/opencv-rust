extern "C" {
	// cv::Ptr<cv::cuda::Filter>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::Filter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::Filter* cv_PtrLcv_cuda_FilterG_getInnerPtr_const(const cv::Ptr<cv::cuda::Filter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::Filter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::Filter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::Filter* cv_PtrLcv_cuda_FilterG_getInnerPtrMut(cv::Ptr<cv::cuda::Filter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::Filter>::new_null() generated
	// ("cv::Ptr<cv::cuda::Filter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::Filter>* cv_PtrLcv_cuda_FilterG_new_null_const() {
			return new cv::Ptr<cv::cuda::Filter>();
	}

	// cv::Ptr<cv::cuda::Filter>::delete() generated
	// ("cv::Ptr<cv::cuda::Filter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_FilterG_delete(cv::Ptr<cv::cuda::Filter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::Filter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::Filter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_FilterG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::Filter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

