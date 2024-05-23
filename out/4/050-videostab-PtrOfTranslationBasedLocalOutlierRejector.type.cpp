extern "C" {
	// cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_getInnerPtr_const(const cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_getInnerPtrMut(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::new_null() generated
	// ("cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_new_null_const() {
			return new cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>();
	}

	// cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::delete() generated
	// ("cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_delete(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::to_PtrOfIOutlierRejector() generated
	// ("cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::to_PtrOfIOutlierRejector", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
			return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}

	// cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>::new", vec![(pred!(const, ["val"], ["cv::videostab::TranslationBasedLocalOutlierRejector"]), _)]),
	cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* cv_PtrLcv_videostab_TranslationBasedLocalOutlierRejectorG_new_const_TranslationBasedLocalOutlierRejector(cv::videostab::TranslationBasedLocalOutlierRejector* val) {
			return new cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>(val);
	}

}

