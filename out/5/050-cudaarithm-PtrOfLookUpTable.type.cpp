extern "C" {
	// cv::Ptr<cv::cuda::LookUpTable>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::LookUpTable>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::LookUpTable* cv_PtrLcv_cuda_LookUpTableG_getInnerPtr_const(const cv::Ptr<cv::cuda::LookUpTable>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::LookUpTable>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::LookUpTable>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::LookUpTable* cv_PtrLcv_cuda_LookUpTableG_getInnerPtrMut(cv::Ptr<cv::cuda::LookUpTable>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::LookUpTable>::new_null() generated
	// ("cv::Ptr<cv::cuda::LookUpTable>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::LookUpTable>* cv_PtrLcv_cuda_LookUpTableG_new_null_const() {
			return new cv::Ptr<cv::cuda::LookUpTable>();
	}

	// cv::Ptr<cv::cuda::LookUpTable>::delete() generated
	// ("cv::Ptr<cv::cuda::LookUpTable>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_LookUpTableG_delete(cv::Ptr<cv::cuda::LookUpTable>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::LookUpTable>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::LookUpTable>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_LookUpTableG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::LookUpTable>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

