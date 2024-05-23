extern "C" {
	// cv::Ptr<cv::Tonemap>::getInnerPtr() generated
	// ("cv::Ptr<cv::Tonemap>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Tonemap* cv_PtrLcv_TonemapG_getInnerPtr_const(const cv::Ptr<cv::Tonemap>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Tonemap>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Tonemap>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Tonemap* cv_PtrLcv_TonemapG_getInnerPtrMut(cv::Ptr<cv::Tonemap>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Tonemap>::new_null() generated
	// ("cv::Ptr<cv::Tonemap>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_TonemapG_new_null_const() {
			return new cv::Ptr<cv::Tonemap>();
	}

	// cv::Ptr<cv::Tonemap>::delete() generated
	// ("cv::Ptr<cv::Tonemap>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TonemapG_delete(cv::Ptr<cv::Tonemap>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::Tonemap>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::Tonemap>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapG_to_PtrOfAlgorithm(cv::Ptr<cv::Tonemap>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

