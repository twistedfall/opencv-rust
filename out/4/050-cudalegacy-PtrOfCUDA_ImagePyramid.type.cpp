extern "C" {
	// cv::Ptr<cv::cuda::ImagePyramid>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::ImagePyramid>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::ImagePyramid* cv_PtrLcv_cuda_ImagePyramidG_getInnerPtr_const(const cv::Ptr<cv::cuda::ImagePyramid>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::ImagePyramid>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::ImagePyramid>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::ImagePyramid* cv_PtrLcv_cuda_ImagePyramidG_getInnerPtrMut(cv::Ptr<cv::cuda::ImagePyramid>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::ImagePyramid>::new_null() generated
	// ("cv::Ptr<cv::cuda::ImagePyramid>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::ImagePyramid>* cv_PtrLcv_cuda_ImagePyramidG_new_null_const() {
			return new cv::Ptr<cv::cuda::ImagePyramid>();
	}

	// cv::Ptr<cv::cuda::ImagePyramid>::delete() generated
	// ("cv::Ptr<cv::cuda::ImagePyramid>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_ImagePyramidG_delete(cv::Ptr<cv::cuda::ImagePyramid>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::ImagePyramid>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::ImagePyramid>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_ImagePyramidG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::ImagePyramid>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

