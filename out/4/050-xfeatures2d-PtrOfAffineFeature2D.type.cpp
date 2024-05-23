extern "C" {
	// cv::Ptr<cv::xfeatures2d::AffineFeature2D>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::AffineFeature2D>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::AffineFeature2D* cv_PtrLcv_xfeatures2d_AffineFeature2DG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::AffineFeature2D>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::AffineFeature2D>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::AffineFeature2D* cv_PtrLcv_xfeatures2d_AffineFeature2DG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::AffineFeature2D>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::AffineFeature2D>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::AffineFeature2D>* cv_PtrLcv_xfeatures2d_AffineFeature2DG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::AffineFeature2D>();
	}

	// cv::Ptr<cv::xfeatures2d::AffineFeature2D>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::AffineFeature2D>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_AffineFeature2DG_delete(cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::AffineFeature2D>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::AffineFeature2D>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_AffineFeature2DG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::AffineFeature2D>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::AffineFeature2D>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_AffineFeature2DG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::AffineFeature2D>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

