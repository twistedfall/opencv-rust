extern "C" {
	// cv::Ptr<cv::AlignMTB>::getInnerPtr() generated
	// ("cv::Ptr<cv::AlignMTB>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::AlignMTB* cv_PtrLcv_AlignMTBG_getInnerPtr_const(const cv::Ptr<cv::AlignMTB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AlignMTB>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::AlignMTB>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::AlignMTB* cv_PtrLcv_AlignMTBG_getInnerPtrMut(cv::Ptr<cv::AlignMTB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AlignMTB>::new_null() generated
	// ("cv::Ptr<cv::AlignMTB>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::AlignMTB>* cv_PtrLcv_AlignMTBG_new_null_const() {
			return new cv::Ptr<cv::AlignMTB>();
	}

	// cv::Ptr<cv::AlignMTB>::delete() generated
	// ("cv::Ptr<cv::AlignMTB>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AlignMTBG_delete(cv::Ptr<cv::AlignMTB>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::AlignMTB>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::AlignMTB>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AlignMTBG_to_PtrOfAlgorithm(cv::Ptr<cv::AlignMTB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::AlignMTB>::to_PtrOfAlignExposures() generated
	// ("cv::Ptr<cv::AlignMTB>::to_PtrOfAlignExposures", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::AlignExposures>* cv_PtrLcv_AlignMTBG_to_PtrOfAlignExposures(cv::Ptr<cv::AlignMTB>* instance) {
			return new cv::Ptr<cv::AlignExposures>(instance->dynamicCast<cv::AlignExposures>());
	}

}

