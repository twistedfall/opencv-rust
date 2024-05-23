extern "C" {
	// cv::Ptr<cv::cuda::DFT>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::DFT>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::DFT* cv_PtrLcv_cuda_DFTG_getInnerPtr_const(const cv::Ptr<cv::cuda::DFT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DFT>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::DFT>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::DFT* cv_PtrLcv_cuda_DFTG_getInnerPtrMut(cv::Ptr<cv::cuda::DFT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::DFT>::new_null() generated
	// ("cv::Ptr<cv::cuda::DFT>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::DFT>* cv_PtrLcv_cuda_DFTG_new_null_const() {
			return new cv::Ptr<cv::cuda::DFT>();
	}

	// cv::Ptr<cv::cuda::DFT>::delete() generated
	// ("cv::Ptr<cv::cuda::DFT>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_DFTG_delete(cv::Ptr<cv::cuda::DFT>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::DFT>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::DFT>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_DFTG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::DFT>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

