extern "C" {
	// cv::Ptr<cv::videostab::NullInpainter>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::NullInpainter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::NullInpainter* cv_PtrLcv_videostab_NullInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullInpainter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::NullInpainter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullInpainter* cv_PtrLcv_videostab_NullInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::NullInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullInpainter>::new_null() generated
	// ("cv::Ptr<cv::videostab::NullInpainter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::NullInpainter>* cv_PtrLcv_videostab_NullInpainterG_new_null_const() {
			return new cv::Ptr<cv::videostab::NullInpainter>();
	}

	// cv::Ptr<cv::videostab::NullInpainter>::delete() generated
	// ("cv::Ptr<cv::videostab::NullInpainter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_NullInpainterG_delete(cv::Ptr<cv::videostab::NullInpainter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::NullInpainter>::to_PtrOfInpainterBase() generated
	// ("cv::Ptr<cv::videostab::NullInpainter>::to_PtrOfInpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_NullInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::NullInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}

	// cv::Ptr<cv::videostab::NullInpainter>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::NullInpainter>::new", vec![(pred!(const, ["val"], ["cv::videostab::NullInpainter"]), _)]),
	cv::Ptr<cv::videostab::NullInpainter>* cv_PtrLcv_videostab_NullInpainterG_new_const_NullInpainter(cv::videostab::NullInpainter* val) {
			return new cv::Ptr<cv::videostab::NullInpainter>(val);
	}

}

