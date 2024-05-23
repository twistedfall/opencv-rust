extern "C" {
	// cv::Ptr<cv::xfeatures2d::FREAK>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::FREAK>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::FREAK* cv_PtrLcv_xfeatures2d_FREAKG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::FREAK>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::FREAK>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::FREAK* cv_PtrLcv_xfeatures2d_FREAKG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::FREAK>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::FREAK>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::FREAK>* cv_PtrLcv_xfeatures2d_FREAKG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::FREAK>();
	}

	// cv::Ptr<cv::xfeatures2d::FREAK>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::FREAK>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_FREAKG_delete(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::FREAK>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::FREAK>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_FREAKG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::FREAK>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::FREAK>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_FREAKG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

