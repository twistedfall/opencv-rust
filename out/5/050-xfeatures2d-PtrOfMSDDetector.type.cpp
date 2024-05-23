extern "C" {
	// cv::Ptr<cv::xfeatures2d::MSDDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::MSDDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::MSDDetector* cv_PtrLcv_xfeatures2d_MSDDetectorG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::MSDDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::MSDDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::MSDDetector* cv_PtrLcv_xfeatures2d_MSDDetectorG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::MSDDetector>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::MSDDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::MSDDetector>* cv_PtrLcv_xfeatures2d_MSDDetectorG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::MSDDetector>();
	}

	// cv::Ptr<cv::xfeatures2d::MSDDetector>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::MSDDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_MSDDetectorG_delete(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::MSDDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::MSDDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_MSDDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::MSDDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::MSDDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_MSDDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::MSDDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

