extern "C" {
	// cv::Ptr<cv::cuda::DenseOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::DenseOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::DenseOpticalFlow* cv_PtrLcv_cuda_DenseOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::cuda::DenseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DenseOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::DenseOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DenseOpticalFlow* cv_PtrLcv_cuda_DenseOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::cuda::DenseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DenseOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::cuda::DenseOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::DenseOpticalFlow>* cv_PtrLcv_cuda_DenseOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::cuda::DenseOpticalFlow>();
	}

	// cv::Ptr<cv::cuda::DenseOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::cuda::DenseOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_DenseOpticalFlowG_delete(cv::Ptr<cv::cuda::DenseOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::DenseOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::DenseOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_DenseOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::DenseOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

