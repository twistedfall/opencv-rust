extern "C" {
	// cv::Ptr<cv::cuda::Convolution>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::Convolution>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::Convolution* cv_PtrLcv_cuda_ConvolutionG_getInnerPtr_const(const cv::Ptr<cv::cuda::Convolution>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::Convolution>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::Convolution>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::Convolution* cv_PtrLcv_cuda_ConvolutionG_getInnerPtrMut(cv::Ptr<cv::cuda::Convolution>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::Convolution>::new_null() generated
	// ("cv::Ptr<cv::cuda::Convolution>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::Convolution>* cv_PtrLcv_cuda_ConvolutionG_new_null_const() {
			return new cv::Ptr<cv::cuda::Convolution>();
	}

	// cv::Ptr<cv::cuda::Convolution>::delete() generated
	// ("cv::Ptr<cv::cuda::Convolution>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_ConvolutionG_delete(cv::Ptr<cv::cuda::Convolution>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::Convolution>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::Convolution>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_ConvolutionG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::Convolution>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

