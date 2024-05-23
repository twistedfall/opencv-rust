extern "C" {
	// cv::Ptr<cv::xfeatures2d::LUCID>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::LUCID>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::LUCID* cv_PtrLcv_xfeatures2d_LUCIDG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::LUCID>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::LUCID>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::LUCID* cv_PtrLcv_xfeatures2d_LUCIDG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::LUCID>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::LUCID>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::LUCID>* cv_PtrLcv_xfeatures2d_LUCIDG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::LUCID>();
	}

	// cv::Ptr<cv::xfeatures2d::LUCID>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::LUCID>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_LUCIDG_delete(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::LUCID>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::LUCID>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_LUCIDG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::LUCID>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::LUCID>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_LUCIDG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

