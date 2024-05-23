extern "C" {
	// cv::Ptr<cv::cuda::CornernessCriteria>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::CornernessCriteria>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::CornernessCriteria* cv_PtrLcv_cuda_CornernessCriteriaG_getInnerPtr_const(const cv::Ptr<cv::cuda::CornernessCriteria>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CornernessCriteria>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::CornernessCriteria>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::CornernessCriteria* cv_PtrLcv_cuda_CornernessCriteriaG_getInnerPtrMut(cv::Ptr<cv::cuda::CornernessCriteria>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CornernessCriteria>::new_null() generated
	// ("cv::Ptr<cv::cuda::CornernessCriteria>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::CornernessCriteria>* cv_PtrLcv_cuda_CornernessCriteriaG_new_null_const() {
			return new cv::Ptr<cv::cuda::CornernessCriteria>();
	}

	// cv::Ptr<cv::cuda::CornernessCriteria>::delete() generated
	// ("cv::Ptr<cv::cuda::CornernessCriteria>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_CornernessCriteriaG_delete(cv::Ptr<cv::cuda::CornernessCriteria>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::CornernessCriteria>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::CornernessCriteria>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_CornernessCriteriaG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::CornernessCriteria>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

