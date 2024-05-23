extern "C" {
	// cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::SparsePyrLKOpticalFlow* cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::SparsePyrLKOpticalFlow* cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>();
	}

	// cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_delete(cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::to_PtrOfCUDA_SparseOpticalFlow() generated
	// ("cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>::to_PtrOfCUDA_SparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::SparseOpticalFlow>* cv_PtrLcv_cuda_SparsePyrLKOpticalFlowG_to_PtrOfCUDA_SparseOpticalFlow(cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::cuda::SparseOpticalFlow>(instance->dynamicCast<cv::cuda::SparseOpticalFlow>());
	}

}

