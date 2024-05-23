extern "C" {
	// cv::Ptr<cv::cuda::HoughCirclesDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::HoughCirclesDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::HoughCirclesDetector* cv_PtrLcv_cuda_HoughCirclesDetectorG_getInnerPtr_const(const cv::Ptr<cv::cuda::HoughCirclesDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HoughCirclesDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::HoughCirclesDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::HoughCirclesDetector* cv_PtrLcv_cuda_HoughCirclesDetectorG_getInnerPtrMut(cv::Ptr<cv::cuda::HoughCirclesDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HoughCirclesDetector>::new_null() generated
	// ("cv::Ptr<cv::cuda::HoughCirclesDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::HoughCirclesDetector>* cv_PtrLcv_cuda_HoughCirclesDetectorG_new_null_const() {
			return new cv::Ptr<cv::cuda::HoughCirclesDetector>();
	}

	// cv::Ptr<cv::cuda::HoughCirclesDetector>::delete() generated
	// ("cv::Ptr<cv::cuda::HoughCirclesDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_HoughCirclesDetectorG_delete(cv::Ptr<cv::cuda::HoughCirclesDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::HoughCirclesDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::HoughCirclesDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_HoughCirclesDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::HoughCirclesDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

