extern "C" {
	// cv::Ptr<cv::TonemapMantiuk>::getInnerPtr() generated
	// ("cv::Ptr<cv::TonemapMantiuk>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TonemapMantiuk* cv_PtrLcv_TonemapMantiukG_getInnerPtr_const(const cv::Ptr<cv::TonemapMantiuk>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TonemapMantiuk>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TonemapMantiuk>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TonemapMantiuk* cv_PtrLcv_TonemapMantiukG_getInnerPtrMut(cv::Ptr<cv::TonemapMantiuk>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TonemapMantiuk>::new_null() generated
	// ("cv::Ptr<cv::TonemapMantiuk>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TonemapMantiuk>* cv_PtrLcv_TonemapMantiukG_new_null_const() {
			return new cv::Ptr<cv::TonemapMantiuk>();
	}

	// cv::Ptr<cv::TonemapMantiuk>::delete() generated
	// ("cv::Ptr<cv::TonemapMantiuk>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TonemapMantiukG_delete(cv::Ptr<cv::TonemapMantiuk>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TonemapMantiuk>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TonemapMantiuk>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapMantiukG_to_PtrOfAlgorithm(cv::Ptr<cv::TonemapMantiuk>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TonemapMantiuk>::to_PtrOfTonemap() generated
	// ("cv::Ptr<cv::TonemapMantiuk>::to_PtrOfTonemap", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_TonemapMantiukG_to_PtrOfTonemap(cv::Ptr<cv::TonemapMantiuk>* instance) {
			return new cv::Ptr<cv::Tonemap>(instance->dynamicCast<cv::Tonemap>());
	}

}

