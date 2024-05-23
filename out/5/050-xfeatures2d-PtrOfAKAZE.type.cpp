extern "C" {
	// cv::Ptr<cv::xfeatures2d::AKAZE>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::AKAZE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::AKAZE* cv_PtrLcv_xfeatures2d_AKAZEG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::AKAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::AKAZE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::AKAZE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::AKAZE* cv_PtrLcv_xfeatures2d_AKAZEG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::AKAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::AKAZE>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::AKAZE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::AKAZE>* cv_PtrLcv_xfeatures2d_AKAZEG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::AKAZE>();
	}

	// cv::Ptr<cv::xfeatures2d::AKAZE>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::AKAZE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_AKAZEG_delete(cv::Ptr<cv::xfeatures2d::AKAZE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::AKAZE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::AKAZE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_AKAZEG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::AKAZE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::AKAZE>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::AKAZE>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_AKAZEG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::AKAZE>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

