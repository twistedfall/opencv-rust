extern "C" {
	// cv::Ptr<cv::superres::SuperResolution>::getInnerPtr() generated
	// ("cv::Ptr<cv::superres::SuperResolution>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::superres::SuperResolution* cv_PtrLcv_superres_SuperResolutionG_getInnerPtr_const(const cv::Ptr<cv::superres::SuperResolution>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::SuperResolution>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::superres::SuperResolution>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::superres::SuperResolution* cv_PtrLcv_superres_SuperResolutionG_getInnerPtrMut(cv::Ptr<cv::superres::SuperResolution>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::SuperResolution>::new_null() generated
	// ("cv::Ptr<cv::superres::SuperResolution>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::superres::SuperResolution>* cv_PtrLcv_superres_SuperResolutionG_new_null_const() {
			return new cv::Ptr<cv::superres::SuperResolution>();
	}

	// cv::Ptr<cv::superres::SuperResolution>::delete() generated
	// ("cv::Ptr<cv::superres::SuperResolution>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_superres_SuperResolutionG_delete(cv::Ptr<cv::superres::SuperResolution>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::superres::SuperResolution>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::superres::SuperResolution>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_SuperResolutionG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::SuperResolution>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::superres::SuperResolution>::to_PtrOfSuperRes_FrameSource() generated
	// ("cv::Ptr<cv::superres::SuperResolution>::to_PtrOfSuperRes_FrameSource", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::superres::FrameSource>* cv_PtrLcv_superres_SuperResolutionG_to_PtrOfSuperRes_FrameSource(cv::Ptr<cv::superres::SuperResolution>* instance) {
			return new cv::Ptr<cv::superres::FrameSource>(instance->dynamicCast<cv::superres::FrameSource>());
	}

}

