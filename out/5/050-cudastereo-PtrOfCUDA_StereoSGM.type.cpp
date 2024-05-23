extern "C" {
	// cv::Ptr<cv::cuda::StereoSGM>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::StereoSGM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::StereoSGM* cv_PtrLcv_cuda_StereoSGMG_getInnerPtr_const(const cv::Ptr<cv::cuda::StereoSGM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoSGM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::StereoSGM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::StereoSGM* cv_PtrLcv_cuda_StereoSGMG_getInnerPtrMut(cv::Ptr<cv::cuda::StereoSGM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoSGM>::new_null() generated
	// ("cv::Ptr<cv::cuda::StereoSGM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::StereoSGM>* cv_PtrLcv_cuda_StereoSGMG_new_null_const() {
			return new cv::Ptr<cv::cuda::StereoSGM>();
	}

	// cv::Ptr<cv::cuda::StereoSGM>::delete() generated
	// ("cv::Ptr<cv::cuda::StereoSGM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_StereoSGMG_delete(cv::Ptr<cv::cuda::StereoSGM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::StereoSGM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::StereoSGM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_StereoSGMG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::StereoSGM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::StereoSGM>::to_PtrOfStereoMatcher() generated
	// ("cv::Ptr<cv::cuda::StereoSGM>::to_PtrOfStereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_cuda_StereoSGMG_to_PtrOfStereoMatcher(cv::Ptr<cv::cuda::StereoSGM>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}

	// cv::Ptr<cv::cuda::StereoSGM>::to_PtrOfStereoSGBM() generated
	// ("cv::Ptr<cv::cuda::StereoSGM>::to_PtrOfStereoSGBM", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoSGBM>* cv_PtrLcv_cuda_StereoSGMG_to_PtrOfStereoSGBM(cv::Ptr<cv::cuda::StereoSGM>* instance) {
			return new cv::Ptr<cv::StereoSGBM>(instance->dynamicCast<cv::StereoSGBM>());
	}

}

