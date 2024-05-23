extern "C" {
	// cv::Ptr<cv::FastFeatureDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::FastFeatureDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::FastFeatureDetector* cv_PtrLcv_FastFeatureDetectorG_getInnerPtr_const(const cv::Ptr<cv::FastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FastFeatureDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::FastFeatureDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::FastFeatureDetector* cv_PtrLcv_FastFeatureDetectorG_getInnerPtrMut(cv::Ptr<cv::FastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FastFeatureDetector>::new_null() generated
	// ("cv::Ptr<cv::FastFeatureDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::FastFeatureDetector>* cv_PtrLcv_FastFeatureDetectorG_new_null_const() {
			return new cv::Ptr<cv::FastFeatureDetector>();
	}

	// cv::Ptr<cv::FastFeatureDetector>::delete() generated
	// ("cv::Ptr<cv::FastFeatureDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FastFeatureDetectorG_delete(cv::Ptr<cv::FastFeatureDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::FastFeatureDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::FastFeatureDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_FastFeatureDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::FastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::FastFeatureDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::FastFeatureDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_FastFeatureDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::FastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

