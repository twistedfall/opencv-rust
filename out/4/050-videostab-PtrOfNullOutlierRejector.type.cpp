extern "C" {
	// cv::Ptr<cv::videostab::NullOutlierRejector>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::NullOutlierRejector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::NullOutlierRejector* cv_PtrLcv_videostab_NullOutlierRejectorG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullOutlierRejector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::NullOutlierRejector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullOutlierRejector* cv_PtrLcv_videostab_NullOutlierRejectorG_getInnerPtrMut(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullOutlierRejector>::new_null() generated
	// ("cv::Ptr<cv::videostab::NullOutlierRejector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::NullOutlierRejector>* cv_PtrLcv_videostab_NullOutlierRejectorG_new_null_const() {
			return new cv::Ptr<cv::videostab::NullOutlierRejector>();
	}

	// cv::Ptr<cv::videostab::NullOutlierRejector>::delete() generated
	// ("cv::Ptr<cv::videostab::NullOutlierRejector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_NullOutlierRejectorG_delete(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::NullOutlierRejector>::to_PtrOfIOutlierRejector() generated
	// ("cv::Ptr<cv::videostab::NullOutlierRejector>::to_PtrOfIOutlierRejector", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrLcv_videostab_NullOutlierRejectorG_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
			return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}

	// cv::Ptr<cv::videostab::NullOutlierRejector>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::NullOutlierRejector>::new", vec![(pred!(const, ["val"], ["cv::videostab::NullOutlierRejector"]), _)]),
	cv::Ptr<cv::videostab::NullOutlierRejector>* cv_PtrLcv_videostab_NullOutlierRejectorG_new_const_NullOutlierRejector(cv::videostab::NullOutlierRejector* val) {
			return new cv::Ptr<cv::videostab::NullOutlierRejector>(val);
	}

}

