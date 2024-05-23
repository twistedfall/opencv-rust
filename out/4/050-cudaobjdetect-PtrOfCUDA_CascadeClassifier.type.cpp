extern "C" {
	// cv::Ptr<cv::cuda::CascadeClassifier>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::CascadeClassifier>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::CascadeClassifier* cv_PtrLcv_cuda_CascadeClassifierG_getInnerPtr_const(const cv::Ptr<cv::cuda::CascadeClassifier>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CascadeClassifier>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::CascadeClassifier>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::CascadeClassifier* cv_PtrLcv_cuda_CascadeClassifierG_getInnerPtrMut(cv::Ptr<cv::cuda::CascadeClassifier>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::CascadeClassifier>::new_null() generated
	// ("cv::Ptr<cv::cuda::CascadeClassifier>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::CascadeClassifier>* cv_PtrLcv_cuda_CascadeClassifierG_new_null_const() {
			return new cv::Ptr<cv::cuda::CascadeClassifier>();
	}

	// cv::Ptr<cv::cuda::CascadeClassifier>::delete() generated
	// ("cv::Ptr<cv::cuda::CascadeClassifier>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_CascadeClassifierG_delete(cv::Ptr<cv::cuda::CascadeClassifier>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::CascadeClassifier>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::CascadeClassifier>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_CascadeClassifierG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::CascadeClassifier>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

