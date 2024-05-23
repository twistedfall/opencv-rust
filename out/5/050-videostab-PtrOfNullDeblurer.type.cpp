extern "C" {
	// cv::Ptr<cv::videostab::NullDeblurer>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::NullDeblurer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::NullDeblurer* cv_PtrLcv_videostab_NullDeblurerG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullDeblurer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::NullDeblurer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullDeblurer* cv_PtrLcv_videostab_NullDeblurerG_getInnerPtrMut(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullDeblurer>::new_null() generated
	// ("cv::Ptr<cv::videostab::NullDeblurer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::NullDeblurer>* cv_PtrLcv_videostab_NullDeblurerG_new_null_const() {
			return new cv::Ptr<cv::videostab::NullDeblurer>();
	}

	// cv::Ptr<cv::videostab::NullDeblurer>::delete() generated
	// ("cv::Ptr<cv::videostab::NullDeblurer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_NullDeblurerG_delete(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::NullDeblurer>::to_PtrOfDeblurerBase() generated
	// ("cv::Ptr<cv::videostab::NullDeblurer>::to_PtrOfDeblurerBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrLcv_videostab_NullDeblurerG_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
			return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}

	// cv::Ptr<cv::videostab::NullDeblurer>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::NullDeblurer>::new", vec![(pred!(const, ["val"], ["cv::videostab::NullDeblurer"]), _)]),
	cv::Ptr<cv::videostab::NullDeblurer>* cv_PtrLcv_videostab_NullDeblurerG_new_const_NullDeblurer(cv::videostab::NullDeblurer* val) {
			return new cv::Ptr<cv::videostab::NullDeblurer>(val);
	}

}

