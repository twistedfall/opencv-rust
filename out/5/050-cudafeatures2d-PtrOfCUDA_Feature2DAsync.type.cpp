extern "C" {
	// cv::Ptr<cv::cuda::Feature2DAsync>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::Feature2DAsync>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::Feature2DAsync* cv_PtrLcv_cuda_Feature2DAsyncG_getInnerPtr_const(const cv::Ptr<cv::cuda::Feature2DAsync>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::Feature2DAsync>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::Feature2DAsync>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::Feature2DAsync* cv_PtrLcv_cuda_Feature2DAsyncG_getInnerPtrMut(cv::Ptr<cv::cuda::Feature2DAsync>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::Feature2DAsync>::new_null() generated
	// ("cv::Ptr<cv::cuda::Feature2DAsync>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::Feature2DAsync>* cv_PtrLcv_cuda_Feature2DAsyncG_new_null_const() {
			return new cv::Ptr<cv::cuda::Feature2DAsync>();
	}

	// cv::Ptr<cv::cuda::Feature2DAsync>::delete() generated
	// ("cv::Ptr<cv::cuda::Feature2DAsync>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_Feature2DAsyncG_delete(cv::Ptr<cv::cuda::Feature2DAsync>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::Feature2DAsync>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::Feature2DAsync>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_Feature2DAsyncG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::Feature2DAsync>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::Feature2DAsync>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::cuda::Feature2DAsync>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_cuda_Feature2DAsyncG_to_PtrOfFeature2D(cv::Ptr<cv::cuda::Feature2DAsync>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

