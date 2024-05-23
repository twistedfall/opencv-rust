extern "C" {
	// cv::Ptr<cv::cuda::BroxOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::BroxOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::BroxOpticalFlow* cv_PtrLcv_cuda_BroxOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BroxOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::BroxOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::BroxOpticalFlow* cv_PtrLcv_cuda_BroxOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::BroxOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::cuda::BroxOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::BroxOpticalFlow>* cv_PtrLcv_cuda_BroxOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::cuda::BroxOpticalFlow>();
	}

	// cv::Ptr<cv::cuda::BroxOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::cuda::BroxOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_BroxOpticalFlowG_delete(cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::BroxOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::BroxOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_BroxOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::BroxOpticalFlow>::to_PtrOfCUDA_DenseOpticalFlow() generated
	// ("cv::Ptr<cv::cuda::BroxOpticalFlow>::to_PtrOfCUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::DenseOpticalFlow>* cv_PtrLcv_cuda_BroxOpticalFlowG_to_PtrOfCUDA_DenseOpticalFlow(cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
			return new cv::Ptr<cv::cuda::DenseOpticalFlow>(instance->dynamicCast<cv::cuda::DenseOpticalFlow>());
	}

}

