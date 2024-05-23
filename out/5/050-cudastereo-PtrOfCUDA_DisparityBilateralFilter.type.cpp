extern "C" {
	// cv::Ptr<cv::cuda::DisparityBilateralFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::DisparityBilateralFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::DisparityBilateralFilter* cv_PtrLcv_cuda_DisparityBilateralFilterG_getInnerPtr_const(const cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DisparityBilateralFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::DisparityBilateralFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DisparityBilateralFilter* cv_PtrLcv_cuda_DisparityBilateralFilterG_getInnerPtrMut(cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DisparityBilateralFilter>::new_null() generated
	// ("cv::Ptr<cv::cuda::DisparityBilateralFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::DisparityBilateralFilter>* cv_PtrLcv_cuda_DisparityBilateralFilterG_new_null_const() {
			return new cv::Ptr<cv::cuda::DisparityBilateralFilter>();
	}

	// cv::Ptr<cv::cuda::DisparityBilateralFilter>::delete() generated
	// ("cv::Ptr<cv::cuda::DisparityBilateralFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_DisparityBilateralFilterG_delete(cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::DisparityBilateralFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::DisparityBilateralFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_DisparityBilateralFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

