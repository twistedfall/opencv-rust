extern "C" {
	// cv::Ptr<cv::xfeatures2d::DAISY>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::DAISY>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::DAISY* cv_PtrLcv_xfeatures2d_DAISYG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::DAISY>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::DAISY>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::DAISY* cv_PtrLcv_xfeatures2d_DAISYG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::DAISY>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::DAISY>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::DAISY>* cv_PtrLcv_xfeatures2d_DAISYG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::DAISY>();
	}

	// cv::Ptr<cv::xfeatures2d::DAISY>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::DAISY>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_DAISYG_delete(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::DAISY>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::DAISY>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_DAISYG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::DAISY>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::DAISY>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_DAISYG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

