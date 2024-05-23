extern "C" {
	// cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::BackgroundSubtractorGMG* cv_PtrLcv_cuda_BackgroundSubtractorGMGG_getInnerPtr_const(const cv::Ptr<cv::cuda::BackgroundSubtractorGMG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::BackgroundSubtractorGMG* cv_PtrLcv_cuda_BackgroundSubtractorGMGG_getInnerPtrMut(cv::Ptr<cv::cuda::BackgroundSubtractorGMG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::new_null() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::BackgroundSubtractorGMG>* cv_PtrLcv_cuda_BackgroundSubtractorGMGG_new_null_const() {
			return new cv::Ptr<cv::cuda::BackgroundSubtractorGMG>();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::delete() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_BackgroundSubtractorGMGG_delete(cv::Ptr<cv::cuda::BackgroundSubtractorGMG>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_BackgroundSubtractorGMGG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::BackgroundSubtractorGMG>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorGMG>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_cuda_BackgroundSubtractorGMGG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::cuda::BackgroundSubtractorGMG>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

