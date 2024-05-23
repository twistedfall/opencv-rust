extern "C" {
	// cv::Ptr<cv::xfeatures2d::TBMR>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::TBMR>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::TBMR* cv_PtrLcv_xfeatures2d_TBMRG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::TBMR>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::TBMR>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::TBMR* cv_PtrLcv_xfeatures2d_TBMRG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::TBMR>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::TBMR>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::TBMR>* cv_PtrLcv_xfeatures2d_TBMRG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::TBMR>();
	}

	// cv::Ptr<cv::xfeatures2d::TBMR>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::TBMR>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_TBMRG_delete(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::TBMR>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::TBMR>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::TBMR>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::TBMR>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::xfeatures2d::TBMR>::to_PtrOfAffineFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::TBMR>::to_PtrOfAffineFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::AffineFeature2D>* cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfAffineFeature2D(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(instance->dynamicCast<cv::xfeatures2d::AffineFeature2D>());
	}

}

