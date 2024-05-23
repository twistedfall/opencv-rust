extern "C" {
	// cv::Ptr<cv::cuda::HoughSegmentDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::HoughSegmentDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::HoughSegmentDetector* cv_PtrLcv_cuda_HoughSegmentDetectorG_getInnerPtr_const(const cv::Ptr<cv::cuda::HoughSegmentDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HoughSegmentDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::HoughSegmentDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::HoughSegmentDetector* cv_PtrLcv_cuda_HoughSegmentDetectorG_getInnerPtrMut(cv::Ptr<cv::cuda::HoughSegmentDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::HoughSegmentDetector>::new_null() generated
	// ("cv::Ptr<cv::cuda::HoughSegmentDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::HoughSegmentDetector>* cv_PtrLcv_cuda_HoughSegmentDetectorG_new_null_const() {
			return new cv::Ptr<cv::cuda::HoughSegmentDetector>();
	}

	// cv::Ptr<cv::cuda::HoughSegmentDetector>::delete() generated
	// ("cv::Ptr<cv::cuda::HoughSegmentDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_HoughSegmentDetectorG_delete(cv::Ptr<cv::cuda::HoughSegmentDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::HoughSegmentDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::HoughSegmentDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_HoughSegmentDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::HoughSegmentDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

