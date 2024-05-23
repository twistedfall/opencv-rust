extern "C" {
	// cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::OpticalFlowDual_TVL1* cv_PtrLcv_cuda_OpticalFlowDual_TVL1G_getInnerPtr_const(const cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::OpticalFlowDual_TVL1* cv_PtrLcv_cuda_OpticalFlowDual_TVL1G_getInnerPtrMut(cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::new_null() generated
	// ("cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* cv_PtrLcv_cuda_OpticalFlowDual_TVL1G_new_null_const() {
			return new cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>();
	}

	// cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::delete() generated
	// ("cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_OpticalFlowDual_TVL1G_delete(cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_OpticalFlowDual_TVL1G_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::to_PtrOfCUDA_DenseOpticalFlow() generated
	// ("cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>::to_PtrOfCUDA_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::DenseOpticalFlow>* cv_PtrLcv_cuda_OpticalFlowDual_TVL1G_to_PtrOfCUDA_DenseOpticalFlow(cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
			return new cv::Ptr<cv::cuda::DenseOpticalFlow>(instance->dynamicCast<cv::cuda::DenseOpticalFlow>());
	}

}

