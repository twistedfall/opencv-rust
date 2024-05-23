extern "C" {
	// cv::Ptr<cv::xfeatures2d::LATCH>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::LATCH>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::LATCH* cv_PtrLcv_xfeatures2d_LATCHG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::LATCH>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::LATCH>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::LATCH* cv_PtrLcv_xfeatures2d_LATCHG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::LATCH>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::LATCH>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::LATCH>* cv_PtrLcv_xfeatures2d_LATCHG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::LATCH>();
	}

	// cv::Ptr<cv::xfeatures2d::LATCH>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::LATCH>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_LATCHG_delete(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::LATCH>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::LATCH>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_LATCHG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::LATCH>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::LATCH>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_LATCHG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::LATCH>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::xfeatures2d::LATCH>::new(TraitClass) generated
	// ("cv::Ptr<cv::xfeatures2d::LATCH>::new", vec![(pred!(const, ["val"], ["cv::xfeatures2d::LATCH"]), _)]),
	cv::Ptr<cv::xfeatures2d::LATCH>* cv_PtrLcv_xfeatures2d_LATCHG_new_const_LATCH(cv::xfeatures2d::LATCH* val) {
			return new cv::Ptr<cv::xfeatures2d::LATCH>(val);
	}

}

