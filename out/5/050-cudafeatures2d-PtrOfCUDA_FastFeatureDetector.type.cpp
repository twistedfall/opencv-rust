extern "C" {
	// cv::Ptr<cv::cuda::FastFeatureDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::FastFeatureDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::FastFeatureDetector* cv_PtrLcv_cuda_FastFeatureDetectorG_getInnerPtr_const(const cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::FastFeatureDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::FastFeatureDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::FastFeatureDetector* cv_PtrLcv_cuda_FastFeatureDetectorG_getInnerPtrMut(cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::FastFeatureDetector>::new_null() generated
	// ("cv::Ptr<cv::cuda::FastFeatureDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::FastFeatureDetector>* cv_PtrLcv_cuda_FastFeatureDetectorG_new_null_const() {
			return new cv::Ptr<cv::cuda::FastFeatureDetector>();
	}

	// cv::Ptr<cv::cuda::FastFeatureDetector>::delete() generated
	// ("cv::Ptr<cv::cuda::FastFeatureDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_FastFeatureDetectorG_delete(cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::FastFeatureDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::FastFeatureDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_FastFeatureDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::FastFeatureDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::cuda::FastFeatureDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_cuda_FastFeatureDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::cuda::FastFeatureDetector>::to_PtrOfCUDA_Feature2DAsync() generated
	// ("cv::Ptr<cv::cuda::FastFeatureDetector>::to_PtrOfCUDA_Feature2DAsync", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::Feature2DAsync>* cv_PtrLcv_cuda_FastFeatureDetectorG_to_PtrOfCUDA_Feature2DAsync(cv::Ptr<cv::cuda::FastFeatureDetector>* instance) {
			return new cv::Ptr<cv::cuda::Feature2DAsync>(instance->dynamicCast<cv::cuda::Feature2DAsync>());
	}

}

