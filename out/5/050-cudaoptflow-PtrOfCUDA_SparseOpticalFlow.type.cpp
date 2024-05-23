extern "C" {
	// cv::Ptr<cv::cuda::SparseOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::SparseOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::SparseOpticalFlow* cv_PtrLcv_cuda_SparseOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::cuda::SparseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::SparseOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::SparseOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::SparseOpticalFlow* cv_PtrLcv_cuda_SparseOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::cuda::SparseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::SparseOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::cuda::SparseOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::SparseOpticalFlow>* cv_PtrLcv_cuda_SparseOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::cuda::SparseOpticalFlow>();
	}

	// cv::Ptr<cv::cuda::SparseOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::cuda::SparseOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_SparseOpticalFlowG_delete(cv::Ptr<cv::cuda::SparseOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::SparseOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::SparseOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_SparseOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::SparseOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

