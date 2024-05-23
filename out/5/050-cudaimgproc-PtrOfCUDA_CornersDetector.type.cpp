extern "C" {
	// cv::Ptr<cv::cuda::CornersDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::CornersDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::CornersDetector* cv_PtrLcv_cuda_CornersDetectorG_getInnerPtr_const(const cv::Ptr<cv::cuda::CornersDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CornersDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::CornersDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::CornersDetector* cv_PtrLcv_cuda_CornersDetectorG_getInnerPtrMut(cv::Ptr<cv::cuda::CornersDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CornersDetector>::new_null() generated
	// ("cv::Ptr<cv::cuda::CornersDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::CornersDetector>* cv_PtrLcv_cuda_CornersDetectorG_new_null_const() {
			return new cv::Ptr<cv::cuda::CornersDetector>();
	}

	// cv::Ptr<cv::cuda::CornersDetector>::delete() generated
	// ("cv::Ptr<cv::cuda::CornersDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_CornersDetectorG_delete(cv::Ptr<cv::cuda::CornersDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::CornersDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::CornersDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_CornersDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::CornersDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

