extern "C" {
	// cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::NvidiaHWOpticalFlow* cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::NvidiaHWOpticalFlow* cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>* cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>();
	}

	// cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_delete(cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_NvidiaHWOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::NvidiaHWOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

