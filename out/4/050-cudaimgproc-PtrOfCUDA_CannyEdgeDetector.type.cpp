extern "C" {
	// cv::Ptr<cv::cuda::CannyEdgeDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::CannyEdgeDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::CannyEdgeDetector* cv_PtrLcv_cuda_CannyEdgeDetectorG_getInnerPtr_const(const cv::Ptr<cv::cuda::CannyEdgeDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CannyEdgeDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::CannyEdgeDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::CannyEdgeDetector* cv_PtrLcv_cuda_CannyEdgeDetectorG_getInnerPtrMut(cv::Ptr<cv::cuda::CannyEdgeDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CannyEdgeDetector>::new_null() generated
	// ("cv::Ptr<cv::cuda::CannyEdgeDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::CannyEdgeDetector>* cv_PtrLcv_cuda_CannyEdgeDetectorG_new_null_const() {
			return new cv::Ptr<cv::cuda::CannyEdgeDetector>();
	}

	// cv::Ptr<cv::cuda::CannyEdgeDetector>::delete() generated
	// ("cv::Ptr<cv::cuda::CannyEdgeDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_CannyEdgeDetectorG_delete(cv::Ptr<cv::cuda::CannyEdgeDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::CannyEdgeDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::CannyEdgeDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_CannyEdgeDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::CannyEdgeDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

