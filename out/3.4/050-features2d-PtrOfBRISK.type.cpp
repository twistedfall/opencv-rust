extern "C" {
	// cv::Ptr<cv::BRISK>::getInnerPtr() generated
	// ("cv::Ptr<cv::BRISK>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::BRISK* cv_PtrLcv_BRISKG_getInnerPtr_const(const cv::Ptr<cv::BRISK>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BRISK>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::BRISK>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::BRISK* cv_PtrLcv_BRISKG_getInnerPtrMut(cv::Ptr<cv::BRISK>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BRISK>::new_null() generated
	// ("cv::Ptr<cv::BRISK>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::BRISK>* cv_PtrLcv_BRISKG_new_null_const() {
			return new cv::Ptr<cv::BRISK>();
	}

	// cv::Ptr<cv::BRISK>::delete() generated
	// ("cv::Ptr<cv::BRISK>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_BRISKG_delete(cv::Ptr<cv::BRISK>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::BRISK>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::BRISK>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BRISKG_to_PtrOfAlgorithm(cv::Ptr<cv::BRISK>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::BRISK>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::BRISK>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_BRISKG_to_PtrOfFeature2D(cv::Ptr<cv::BRISK>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::BRISK>::new(TraitClass) generated
	// ("cv::Ptr<cv::BRISK>::new", vec![(pred!(const, ["val"], ["cv::BRISK"]), _)]),
	cv::Ptr<cv::BRISK>* cv_PtrLcv_BRISKG_new_const_BRISK(cv::BRISK* val) {
			return new cv::Ptr<cv::BRISK>(val);
	}

}

