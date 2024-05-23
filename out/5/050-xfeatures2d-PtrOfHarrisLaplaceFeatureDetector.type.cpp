extern "C" {
	// cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::HarrisLaplaceFeatureDetector* cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::HarrisLaplaceFeatureDetector* cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>();
	}

	// cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_delete(cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

