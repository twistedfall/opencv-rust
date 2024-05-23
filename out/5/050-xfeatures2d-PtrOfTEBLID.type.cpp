extern "C" {
	// cv::Ptr<cv::xfeatures2d::TEBLID>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::TEBLID>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::TEBLID* cv_PtrLcv_xfeatures2d_TEBLIDG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::TEBLID>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::TEBLID>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::TEBLID* cv_PtrLcv_xfeatures2d_TEBLIDG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::TEBLID>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::TEBLID>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::TEBLID>* cv_PtrLcv_xfeatures2d_TEBLIDG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::TEBLID>();
	}

	// cv::Ptr<cv::xfeatures2d::TEBLID>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::TEBLID>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_TEBLIDG_delete(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::TEBLID>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::TEBLID>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_TEBLIDG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::TEBLID>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::TEBLID>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_TEBLIDG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::xfeatures2d::TEBLID>::new(TraitClass) generated
	// ("cv::Ptr<cv::xfeatures2d::TEBLID>::new", vec![(pred!(const, ["val"], ["cv::xfeatures2d::TEBLID"]), _)]),
	cv::Ptr<cv::xfeatures2d::TEBLID>* cv_PtrLcv_xfeatures2d_TEBLIDG_new_const_TEBLID(cv::xfeatures2d::TEBLID* val) {
			return new cv::Ptr<cv::xfeatures2d::TEBLID>(val);
	}

}

