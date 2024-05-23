extern "C" {
	// cv::Ptr<cv::StereoBM>::getInnerPtr() generated
	// ("cv::Ptr<cv::StereoBM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::StereoBM* cv_PtrLcv_StereoBMG_getInnerPtr_const(const cv::Ptr<cv::StereoBM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereoBM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::StereoBM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::StereoBM* cv_PtrLcv_StereoBMG_getInnerPtrMut(cv::Ptr<cv::StereoBM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereoBM>::new_null() generated
	// ("cv::Ptr<cv::StereoBM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::StereoBM>* cv_PtrLcv_StereoBMG_new_null_const() {
			return new cv::Ptr<cv::StereoBM>();
	}

	// cv::Ptr<cv::StereoBM>::delete() generated
	// ("cv::Ptr<cv::StereoBM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_StereoBMG_delete(cv::Ptr<cv::StereoBM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::StereoBM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::StereoBM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_StereoBMG_to_PtrOfAlgorithm(cv::Ptr<cv::StereoBM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::StereoBM>::to_PtrOfStereoMatcher() generated
	// ("cv::Ptr<cv::StereoBM>::to_PtrOfStereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_StereoBMG_to_PtrOfStereoMatcher(cv::Ptr<cv::StereoBM>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}

}

