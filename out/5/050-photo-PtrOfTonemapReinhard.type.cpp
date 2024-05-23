extern "C" {
	// cv::Ptr<cv::TonemapReinhard>::getInnerPtr() generated
	// ("cv::Ptr<cv::TonemapReinhard>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TonemapReinhard* cv_PtrLcv_TonemapReinhardG_getInnerPtr_const(const cv::Ptr<cv::TonemapReinhard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TonemapReinhard>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TonemapReinhard>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TonemapReinhard* cv_PtrLcv_TonemapReinhardG_getInnerPtrMut(cv::Ptr<cv::TonemapReinhard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TonemapReinhard>::new_null() generated
	// ("cv::Ptr<cv::TonemapReinhard>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TonemapReinhard>* cv_PtrLcv_TonemapReinhardG_new_null_const() {
			return new cv::Ptr<cv::TonemapReinhard>();
	}

	// cv::Ptr<cv::TonemapReinhard>::delete() generated
	// ("cv::Ptr<cv::TonemapReinhard>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TonemapReinhardG_delete(cv::Ptr<cv::TonemapReinhard>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TonemapReinhard>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TonemapReinhard>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapReinhardG_to_PtrOfAlgorithm(cv::Ptr<cv::TonemapReinhard>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TonemapReinhard>::to_PtrOfTonemap() generated
	// ("cv::Ptr<cv::TonemapReinhard>::to_PtrOfTonemap", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_TonemapReinhardG_to_PtrOfTonemap(cv::Ptr<cv::TonemapReinhard>* instance) {
			return new cv::Ptr<cv::Tonemap>(instance->dynamicCast<cv::Tonemap>());
	}

}

