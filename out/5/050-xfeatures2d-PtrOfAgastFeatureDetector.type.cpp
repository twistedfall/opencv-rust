extern "C" {
	// cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::AgastFeatureDetector* cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::AgastFeatureDetector* cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>* cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>();
	}

	// cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_delete(cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

