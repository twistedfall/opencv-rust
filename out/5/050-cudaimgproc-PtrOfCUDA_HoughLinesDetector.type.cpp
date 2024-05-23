extern "C" {
	// cv::Ptr<cv::cuda::HoughLinesDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::HoughLinesDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::HoughLinesDetector* cv_PtrLcv_cuda_HoughLinesDetectorG_getInnerPtr_const(const cv::Ptr<cv::cuda::HoughLinesDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HoughLinesDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::HoughLinesDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::HoughLinesDetector* cv_PtrLcv_cuda_HoughLinesDetectorG_getInnerPtrMut(cv::Ptr<cv::cuda::HoughLinesDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HoughLinesDetector>::new_null() generated
	// ("cv::Ptr<cv::cuda::HoughLinesDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::HoughLinesDetector>* cv_PtrLcv_cuda_HoughLinesDetectorG_new_null_const() {
			return new cv::Ptr<cv::cuda::HoughLinesDetector>();
	}

	// cv::Ptr<cv::cuda::HoughLinesDetector>::delete() generated
	// ("cv::Ptr<cv::cuda::HoughLinesDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_HoughLinesDetectorG_delete(cv::Ptr<cv::cuda::HoughLinesDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::HoughLinesDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::HoughLinesDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_HoughLinesDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::HoughLinesDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

