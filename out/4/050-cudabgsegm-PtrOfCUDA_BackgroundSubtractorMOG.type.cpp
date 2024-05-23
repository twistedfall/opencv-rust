extern "C" {
	// cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::BackgroundSubtractorMOG* cv_PtrLcv_cuda_BackgroundSubtractorMOGG_getInnerPtr_const(const cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::BackgroundSubtractorMOG* cv_PtrLcv_cuda_BackgroundSubtractorMOGG_getInnerPtrMut(cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::new_null() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* cv_PtrLcv_cuda_BackgroundSubtractorMOGG_new_null_const() {
			return new cv::Ptr<cv::cuda::BackgroundSubtractorMOG>();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::delete() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_BackgroundSubtractorMOGG_delete(cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_BackgroundSubtractorMOGG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorMOG>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_cuda_BackgroundSubtractorMOGG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::cuda::BackgroundSubtractorMOG>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

