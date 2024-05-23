extern "C" {
	// cv::Ptr<cv::cuda::CLAHE>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::CLAHE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::CLAHE* cv_PtrLcv_cuda_CLAHEG_getInnerPtr_const(const cv::Ptr<cv::cuda::CLAHE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CLAHE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::CLAHE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::CLAHE* cv_PtrLcv_cuda_CLAHEG_getInnerPtrMut(cv::Ptr<cv::cuda::CLAHE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CLAHE>::new_null() generated
	// ("cv::Ptr<cv::cuda::CLAHE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::CLAHE>* cv_PtrLcv_cuda_CLAHEG_new_null_const() {
			return new cv::Ptr<cv::cuda::CLAHE>();
	}

	// cv::Ptr<cv::cuda::CLAHE>::delete() generated
	// ("cv::Ptr<cv::cuda::CLAHE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_CLAHEG_delete(cv::Ptr<cv::cuda::CLAHE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::CLAHE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::CLAHE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_CLAHEG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::CLAHE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::CLAHE>::to_PtrOfCLAHE() generated
	// ("cv::Ptr<cv::cuda::CLAHE>::to_PtrOfCLAHE", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::CLAHE>* cv_PtrLcv_cuda_CLAHEG_to_PtrOfCLAHE(cv::Ptr<cv::cuda::CLAHE>* instance) {
			return new cv::Ptr<cv::CLAHE>(instance->dynamicCast<cv::CLAHE>());
	}

}

