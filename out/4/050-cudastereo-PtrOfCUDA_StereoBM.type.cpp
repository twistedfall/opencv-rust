extern "C" {
	// cv::Ptr<cv::cuda::StereoBM>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::StereoBM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::StereoBM* cv_PtrLcv_cuda_StereoBMG_getInnerPtr_const(const cv::Ptr<cv::cuda::StereoBM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoBM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::StereoBM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::StereoBM* cv_PtrLcv_cuda_StereoBMG_getInnerPtrMut(cv::Ptr<cv::cuda::StereoBM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::StereoBM>::new_null() generated
	// ("cv::Ptr<cv::cuda::StereoBM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::StereoBM>* cv_PtrLcv_cuda_StereoBMG_new_null_const() {
			return new cv::Ptr<cv::cuda::StereoBM>();
	}

	// cv::Ptr<cv::cuda::StereoBM>::delete() generated
	// ("cv::Ptr<cv::cuda::StereoBM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_StereoBMG_delete(cv::Ptr<cv::cuda::StereoBM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::StereoBM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::StereoBM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_StereoBMG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::StereoBM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::StereoBM>::to_PtrOfStereoBM() generated
	// ("cv::Ptr<cv::cuda::StereoBM>::to_PtrOfStereoBM", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoBM>* cv_PtrLcv_cuda_StereoBMG_to_PtrOfStereoBM(cv::Ptr<cv::cuda::StereoBM>* instance) {
			return new cv::Ptr<cv::StereoBM>(instance->dynamicCast<cv::StereoBM>());
	}

	// cv::Ptr<cv::cuda::StereoBM>::to_PtrOfStereoMatcher() generated
	// ("cv::Ptr<cv::cuda::StereoBM>::to_PtrOfStereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_cuda_StereoBMG_to_PtrOfStereoMatcher(cv::Ptr<cv::cuda::StereoBM>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}

}

