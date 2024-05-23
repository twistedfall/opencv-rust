extern "C" {
	// cv::Ptr<cv::GFTTDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::GFTTDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::GFTTDetector* cv_PtrLcv_GFTTDetectorG_getInnerPtr_const(const cv::Ptr<cv::GFTTDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GFTTDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::GFTTDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::GFTTDetector* cv_PtrLcv_GFTTDetectorG_getInnerPtrMut(cv::Ptr<cv::GFTTDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GFTTDetector>::new_null() generated
	// ("cv::Ptr<cv::GFTTDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::GFTTDetector>* cv_PtrLcv_GFTTDetectorG_new_null_const() {
			return new cv::Ptr<cv::GFTTDetector>();
	}

	// cv::Ptr<cv::GFTTDetector>::delete() generated
	// ("cv::Ptr<cv::GFTTDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_GFTTDetectorG_delete(cv::Ptr<cv::GFTTDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::GFTTDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::GFTTDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GFTTDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::GFTTDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::GFTTDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::GFTTDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_GFTTDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::GFTTDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

