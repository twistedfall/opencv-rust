extern "C" {
	// cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::NvidiaOpticalFlow_2_0* cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_getInnerPtr_const(const cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::NvidiaOpticalFlow_2_0* cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_getInnerPtrMut(cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::new_null() generated
	// ("cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>* cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_new_null_const() {
			return new cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>();
	}

	// cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::delete() generated
	// ("cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_delete(cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::to_PtrOfCUDA_NvidiaHWOpticalFlow() generated
	// ("cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>::to_PtrOfCUDA_NvidiaHWOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>* cv_PtrLcv_cuda_NvidiaOpticalFlow_2_0G_to_PtrOfCUDA_NvidiaHWOpticalFlow(cv::Ptr<cv::cuda::NvidiaOpticalFlow_2_0>* instance) {
			return new cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>(instance->dynamicCast<cv::cuda::NvidiaHWOpticalFlow>());
	}

}

