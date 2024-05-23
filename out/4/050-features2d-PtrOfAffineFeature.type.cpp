extern "C" {
	// cv::Ptr<cv::AffineFeature>::getInnerPtr() generated
	// ("cv::Ptr<cv::AffineFeature>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::AffineFeature* cv_PtrLcv_AffineFeatureG_getInnerPtr_const(const cv::Ptr<cv::AffineFeature>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AffineFeature>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::AffineFeature>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::AffineFeature* cv_PtrLcv_AffineFeatureG_getInnerPtrMut(cv::Ptr<cv::AffineFeature>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AffineFeature>::new_null() generated
	// ("cv::Ptr<cv::AffineFeature>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::AffineFeature>* cv_PtrLcv_AffineFeatureG_new_null_const() {
			return new cv::Ptr<cv::AffineFeature>();
	}

	// cv::Ptr<cv::AffineFeature>::delete() generated
	// ("cv::Ptr<cv::AffineFeature>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AffineFeatureG_delete(cv::Ptr<cv::AffineFeature>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::AffineFeature>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::AffineFeature>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AffineFeatureG_to_PtrOfAlgorithm(cv::Ptr<cv::AffineFeature>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::AffineFeature>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::AffineFeature>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_AffineFeatureG_to_PtrOfFeature2D(cv::Ptr<cv::AffineFeature>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

