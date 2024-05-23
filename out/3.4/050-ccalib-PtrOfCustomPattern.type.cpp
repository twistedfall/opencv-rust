extern "C" {
	// cv::Ptr<cv::ccalib::CustomPattern>::getInnerPtr() generated
	// ("cv::Ptr<cv::ccalib::CustomPattern>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ccalib::CustomPattern* cv_PtrLcv_ccalib_CustomPatternG_getInnerPtr_const(const cv::Ptr<cv::ccalib::CustomPattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ccalib::CustomPattern>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ccalib::CustomPattern>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ccalib::CustomPattern* cv_PtrLcv_ccalib_CustomPatternG_getInnerPtrMut(cv::Ptr<cv::ccalib::CustomPattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ccalib::CustomPattern>::new_null() generated
	// ("cv::Ptr<cv::ccalib::CustomPattern>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ccalib::CustomPattern>* cv_PtrLcv_ccalib_CustomPatternG_new_null_const() {
			return new cv::Ptr<cv::ccalib::CustomPattern>();
	}

	// cv::Ptr<cv::ccalib::CustomPattern>::delete() generated
	// ("cv::Ptr<cv::ccalib::CustomPattern>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ccalib_CustomPatternG_delete(cv::Ptr<cv::ccalib::CustomPattern>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ccalib::CustomPattern>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ccalib::CustomPattern>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ccalib_CustomPatternG_to_PtrOfAlgorithm(cv::Ptr<cv::ccalib::CustomPattern>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ccalib::CustomPattern>::new(TraitClass) generated
	// ("cv::Ptr<cv::ccalib::CustomPattern>::new", vec![(pred!(const, ["val"], ["cv::ccalib::CustomPattern"]), _)]),
	cv::Ptr<cv::ccalib::CustomPattern>* cv_PtrLcv_ccalib_CustomPatternG_new_const_CustomPattern(cv::ccalib::CustomPattern* val) {
			return new cv::Ptr<cv::ccalib::CustomPattern>(val);
	}

}

