extern "C" {
	// cv::Ptr<cv::xfeatures2d::BoostDesc>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::BoostDesc>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::BoostDesc* cv_PtrLcv_xfeatures2d_BoostDescG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BoostDesc>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::BoostDesc>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::BoostDesc* cv_PtrLcv_xfeatures2d_BoostDescG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BoostDesc>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::BoostDesc>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::BoostDesc>* cv_PtrLcv_xfeatures2d_BoostDescG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::BoostDesc>();
	}

	// cv::Ptr<cv::xfeatures2d::BoostDesc>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::BoostDesc>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_BoostDescG_delete(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::BoostDesc>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::BoostDesc>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_BoostDescG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::BoostDesc>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::BoostDesc>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_BoostDescG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BoostDesc>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

