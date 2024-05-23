extern "C" {
	// cv::Ptr<cv::videostab::WeightingDeblurer>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::WeightingDeblurer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::WeightingDeblurer* cv_PtrLcv_videostab_WeightingDeblurerG_getInnerPtr_const(const cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::WeightingDeblurer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::WeightingDeblurer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::WeightingDeblurer* cv_PtrLcv_videostab_WeightingDeblurerG_getInnerPtrMut(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::WeightingDeblurer>::new_null() generated
	// ("cv::Ptr<cv::videostab::WeightingDeblurer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::WeightingDeblurer>* cv_PtrLcv_videostab_WeightingDeblurerG_new_null_const() {
			return new cv::Ptr<cv::videostab::WeightingDeblurer>();
	}

	// cv::Ptr<cv::videostab::WeightingDeblurer>::delete() generated
	// ("cv::Ptr<cv::videostab::WeightingDeblurer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_WeightingDeblurerG_delete(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::WeightingDeblurer>::to_PtrOfDeblurerBase() generated
	// ("cv::Ptr<cv::videostab::WeightingDeblurer>::to_PtrOfDeblurerBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrLcv_videostab_WeightingDeblurerG_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::WeightingDeblurer>* instance) {
			return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}

	// cv::Ptr<cv::videostab::WeightingDeblurer>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::WeightingDeblurer>::new", vec![(pred!(const, ["val"], ["cv::videostab::WeightingDeblurer"]), _)]),
	cv::Ptr<cv::videostab::WeightingDeblurer>* cv_PtrLcv_videostab_WeightingDeblurerG_new_const_WeightingDeblurer(cv::videostab::WeightingDeblurer* val) {
			return new cv::Ptr<cv::videostab::WeightingDeblurer>(val);
	}

}

