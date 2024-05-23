extern "C" {
	// cv::Ptr<cv::xfeatures2d::StarDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::StarDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::StarDetector* cv_PtrLcv_xfeatures2d_StarDetectorG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::StarDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::StarDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::StarDetector* cv_PtrLcv_xfeatures2d_StarDetectorG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::StarDetector>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::StarDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::StarDetector>* cv_PtrLcv_xfeatures2d_StarDetectorG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::StarDetector>();
	}

	// cv::Ptr<cv::xfeatures2d::StarDetector>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::StarDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_StarDetectorG_delete(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::StarDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::StarDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_StarDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::StarDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::StarDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_StarDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::StarDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::xfeatures2d::StarDetector>::new(TraitClass) generated
	// ("cv::Ptr<cv::xfeatures2d::StarDetector>::new", vec![(pred!(const, ["val"], ["cv::xfeatures2d::StarDetector"]), _)]),
	cv::Ptr<cv::xfeatures2d::StarDetector>* cv_PtrLcv_xfeatures2d_StarDetectorG_new_const_StarDetector(cv::xfeatures2d::StarDetector* val) {
			return new cv::Ptr<cv::xfeatures2d::StarDetector>(val);
	}

}

