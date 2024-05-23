extern "C" {
	// cv::Ptr<cv::AgastFeatureDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::AgastFeatureDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::AgastFeatureDetector* cv_PtrLcv_AgastFeatureDetectorG_getInnerPtr_const(const cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AgastFeatureDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::AgastFeatureDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::AgastFeatureDetector* cv_PtrLcv_AgastFeatureDetectorG_getInnerPtrMut(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AgastFeatureDetector>::new_null() generated
	// ("cv::Ptr<cv::AgastFeatureDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::AgastFeatureDetector>* cv_PtrLcv_AgastFeatureDetectorG_new_null_const() {
			return new cv::Ptr<cv::AgastFeatureDetector>();
	}

	// cv::Ptr<cv::AgastFeatureDetector>::delete() generated
	// ("cv::Ptr<cv::AgastFeatureDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AgastFeatureDetectorG_delete(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::AgastFeatureDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::AgastFeatureDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AgastFeatureDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::AgastFeatureDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::AgastFeatureDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_AgastFeatureDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::AgastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

