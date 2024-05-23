extern "C" {
	// cv::Ptr<cv::cuda::FarnebackOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::FarnebackOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::FarnebackOpticalFlow* cv_PtrLcv_cuda_FarnebackOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::FarnebackOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::FarnebackOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::FarnebackOpticalFlow* cv_PtrLcv_cuda_FarnebackOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::FarnebackOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::cuda::FarnebackOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::FarnebackOpticalFlow>* cv_PtrLcv_cuda_FarnebackOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::cuda::FarnebackOpticalFlow>();
	}

	// cv::Ptr<cv::cuda::FarnebackOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::cuda::FarnebackOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_FarnebackOpticalFlowG_delete(cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::FarnebackOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::FarnebackOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_FarnebackOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::FarnebackOpticalFlow>::to_PtrOfCUDA_DenseOpticalFlow() generated
	// ("cv::Ptr<cv::cuda::FarnebackOpticalFlow>::to_PtrOfCUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::DenseOpticalFlow>* cv_PtrLcv_cuda_FarnebackOpticalFlowG_to_PtrOfCUDA_DenseOpticalFlow(cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::cuda::DenseOpticalFlow>(instance->dynamicCast<cv::cuda::DenseOpticalFlow>());
	}

}

