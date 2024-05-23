extern "C" {
	// cv::Ptr<cv::xfeatures2d::BRISK>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::BRISK>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::BRISK* cv_PtrLcv_xfeatures2d_BRISKG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::BRISK>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BRISK>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::BRISK>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::BRISK* cv_PtrLcv_xfeatures2d_BRISKG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::BRISK>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BRISK>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::BRISK>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::BRISK>* cv_PtrLcv_xfeatures2d_BRISKG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::BRISK>();
	}

	// cv::Ptr<cv::xfeatures2d::BRISK>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::BRISK>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_BRISKG_delete(cv::Ptr<cv::xfeatures2d::BRISK>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::BRISK>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::BRISK>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_BRISKG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::BRISK>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::BRISK>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::BRISK>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_BRISKG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BRISK>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

