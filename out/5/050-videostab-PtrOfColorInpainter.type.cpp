extern "C" {
	// cv::Ptr<cv::videostab::ColorInpainter>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::ColorInpainter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::ColorInpainter* cv_PtrLcv_videostab_ColorInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ColorInpainter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::ColorInpainter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ColorInpainter* cv_PtrLcv_videostab_ColorInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ColorInpainter>::new_null() generated
	// ("cv::Ptr<cv::videostab::ColorInpainter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::ColorInpainter>* cv_PtrLcv_videostab_ColorInpainterG_new_null_const() {
			return new cv::Ptr<cv::videostab::ColorInpainter>();
	}

	// cv::Ptr<cv::videostab::ColorInpainter>::delete() generated
	// ("cv::Ptr<cv::videostab::ColorInpainter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_ColorInpainterG_delete(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::ColorInpainter>::to_PtrOfInpainterBase() generated
	// ("cv::Ptr<cv::videostab::ColorInpainter>::to_PtrOfInpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_ColorInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}

	// cv::Ptr<cv::videostab::ColorInpainter>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::ColorInpainter>::new", vec![(pred!(const, ["val"], ["cv::videostab::ColorInpainter"]), _)]),
	cv::Ptr<cv::videostab::ColorInpainter>* cv_PtrLcv_videostab_ColorInpainterG_new_const_ColorInpainter(cv::videostab::ColorInpainter* val) {
			return new cv::Ptr<cv::videostab::ColorInpainter>(val);
	}

}

