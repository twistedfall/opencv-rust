extern "C" {
	// cv::Ptr<cv::xfeatures2d::KAZE>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::KAZE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::KAZE* cv_PtrLcv_xfeatures2d_KAZEG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::KAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::KAZE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::KAZE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::KAZE* cv_PtrLcv_xfeatures2d_KAZEG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::KAZE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::KAZE>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::KAZE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::KAZE>* cv_PtrLcv_xfeatures2d_KAZEG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::KAZE>();
	}

	// cv::Ptr<cv::xfeatures2d::KAZE>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::KAZE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_KAZEG_delete(cv::Ptr<cv::xfeatures2d::KAZE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::KAZE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::KAZE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_KAZEG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::KAZE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::KAZE>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::KAZE>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_KAZEG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::KAZE>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

