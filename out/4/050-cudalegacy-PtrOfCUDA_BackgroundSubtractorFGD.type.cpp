extern "C" {
	// cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::BackgroundSubtractorFGD* cv_PtrLcv_cuda_BackgroundSubtractorFGDG_getInnerPtr_const(const cv::Ptr<cv::cuda::BackgroundSubtractorFGD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::BackgroundSubtractorFGD* cv_PtrLcv_cuda_BackgroundSubtractorFGDG_getInnerPtrMut(cv::Ptr<cv::cuda::BackgroundSubtractorFGD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::new_null() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::BackgroundSubtractorFGD>* cv_PtrLcv_cuda_BackgroundSubtractorFGDG_new_null_const() {
			return new cv::Ptr<cv::cuda::BackgroundSubtractorFGD>();
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::delete() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_BackgroundSubtractorFGDG_delete(cv::Ptr<cv::cuda::BackgroundSubtractorFGD>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_BackgroundSubtractorFGDG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::BackgroundSubtractorFGD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::to_PtrOfBackgroundSubtractor() generated
	// ("cv::Ptr<cv::cuda::BackgroundSubtractorFGD>::to_PtrOfBackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::BackgroundSubtractor>* cv_PtrLcv_cuda_BackgroundSubtractorFGDG_to_PtrOfBackgroundSubtractor(cv::Ptr<cv::cuda::BackgroundSubtractorFGD>* instance) {
			return new cv::Ptr<cv::BackgroundSubtractor>(instance->dynamicCast<cv::BackgroundSubtractor>());
	}

}

