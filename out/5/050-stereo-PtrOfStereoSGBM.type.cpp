extern "C" {
	// cv::Ptr<cv::StereoSGBM>::getInnerPtr() generated
	// ("cv::Ptr<cv::StereoSGBM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::StereoSGBM* cv_PtrLcv_StereoSGBMG_getInnerPtr_const(const cv::Ptr<cv::StereoSGBM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereoSGBM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::StereoSGBM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::StereoSGBM* cv_PtrLcv_StereoSGBMG_getInnerPtrMut(cv::Ptr<cv::StereoSGBM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereoSGBM>::new_null() generated
	// ("cv::Ptr<cv::StereoSGBM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::StereoSGBM>* cv_PtrLcv_StereoSGBMG_new_null_const() {
			return new cv::Ptr<cv::StereoSGBM>();
	}

	// cv::Ptr<cv::StereoSGBM>::delete() generated
	// ("cv::Ptr<cv::StereoSGBM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_StereoSGBMG_delete(cv::Ptr<cv::StereoSGBM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::StereoSGBM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::StereoSGBM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_StereoSGBMG_to_PtrOfAlgorithm(cv::Ptr<cv::StereoSGBM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::StereoSGBM>::to_PtrOfStereoMatcher() generated
	// ("cv::Ptr<cv::StereoSGBM>::to_PtrOfStereoMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_StereoSGBMG_to_PtrOfStereoMatcher(cv::Ptr<cv::StereoSGBM>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}

}

