extern "C" {
	// cv::Ptr<cv::cuda::HOG>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::HOG>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::HOG* cv_PtrLcv_cuda_HOGG_getInnerPtr_const(const cv::Ptr<cv::cuda::HOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HOG>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::HOG>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::HOG* cv_PtrLcv_cuda_HOGG_getInnerPtrMut(cv::Ptr<cv::cuda::HOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HOG>::new_null() generated
	// ("cv::Ptr<cv::cuda::HOG>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::HOG>* cv_PtrLcv_cuda_HOGG_new_null_const() {
			return new cv::Ptr<cv::cuda::HOG>();
	}

	// cv::Ptr<cv::cuda::HOG>::delete() generated
	// ("cv::Ptr<cv::cuda::HOG>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_HOGG_delete(cv::Ptr<cv::cuda::HOG>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::HOG>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::HOG>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_HOGG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::HOG>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

