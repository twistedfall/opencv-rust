extern "C" {
	// cv::Ptr<cv::xfeatures2d::BEBLID>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::BEBLID>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::BEBLID* cv_PtrLcv_xfeatures2d_BEBLIDG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BEBLID>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::BEBLID>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::BEBLID* cv_PtrLcv_xfeatures2d_BEBLIDG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BEBLID>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::BEBLID>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::BEBLID>* cv_PtrLcv_xfeatures2d_BEBLIDG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::BEBLID>();
	}

	// cv::Ptr<cv::xfeatures2d::BEBLID>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::BEBLID>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_BEBLIDG_delete(cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::BEBLID>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::BEBLID>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_BEBLIDG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::BEBLID>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::BEBLID>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_BEBLIDG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BEBLID>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

