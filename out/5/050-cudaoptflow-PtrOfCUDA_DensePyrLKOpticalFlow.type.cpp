extern "C" {
	// cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::DensePyrLKOpticalFlow* cv_PtrLcv_cuda_DensePyrLKOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DensePyrLKOpticalFlow* cv_PtrLcv_cuda_DensePyrLKOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* cv_PtrLcv_cuda_DensePyrLKOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>();
	}

	// cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_DensePyrLKOpticalFlowG_delete(cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_DensePyrLKOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::to_PtrOfCUDA_DenseOpticalFlow() generated
	// ("cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>::to_PtrOfCUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::DenseOpticalFlow>* cv_PtrLcv_cuda_DensePyrLKOpticalFlowG_to_PtrOfCUDA_DenseOpticalFlow(cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::cuda::DenseOpticalFlow>(instance->dynamicCast<cv::cuda::DenseOpticalFlow>());
	}

}

