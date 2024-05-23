extern "C" {
	// cv::Ptr<cv::xfeatures2d::VGG>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::VGG>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::VGG* cv_PtrLcv_xfeatures2d_VGGG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::VGG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::VGG>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::VGG>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::VGG* cv_PtrLcv_xfeatures2d_VGGG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::VGG>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::VGG>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::VGG>* cv_PtrLcv_xfeatures2d_VGGG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::VGG>();
	}

	// cv::Ptr<cv::xfeatures2d::VGG>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::VGG>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_VGGG_delete(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::VGG>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::VGG>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_VGGG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::VGG>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::VGG>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_VGGG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::VGG>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

