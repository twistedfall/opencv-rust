extern "C" {
	// cv::Ptr<cv::xfeatures2d::SURF>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::SURF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::SURF* cv_PtrLcv_xfeatures2d_SURFG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::SURF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::SURF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::SURF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::SURF* cv_PtrLcv_xfeatures2d_SURFG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::SURF>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::SURF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::SURF>* cv_PtrLcv_xfeatures2d_SURFG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::SURF>();
	}

	// cv::Ptr<cv::xfeatures2d::SURF>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::SURF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_SURFG_delete(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::SURF>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::SURF>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_SURFG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::SURF>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::SURF>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_SURFG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::SURF>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

