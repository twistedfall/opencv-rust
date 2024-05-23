extern "C" {
	// cv::Ptr<cv::videostab::ColorAverageInpainter>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::ColorAverageInpainter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::ColorAverageInpainter* cv_PtrLcv_videostab_ColorAverageInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ColorAverageInpainter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::ColorAverageInpainter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ColorAverageInpainter* cv_PtrLcv_videostab_ColorAverageInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ColorAverageInpainter>::new_null() generated
	// ("cv::Ptr<cv::videostab::ColorAverageInpainter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::ColorAverageInpainter>* cv_PtrLcv_videostab_ColorAverageInpainterG_new_null_const() {
			return new cv::Ptr<cv::videostab::ColorAverageInpainter>();
	}

	// cv::Ptr<cv::videostab::ColorAverageInpainter>::delete() generated
	// ("cv::Ptr<cv::videostab::ColorAverageInpainter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_ColorAverageInpainterG_delete(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::ColorAverageInpainter>::to_PtrOfInpainterBase() generated
	// ("cv::Ptr<cv::videostab::ColorAverageInpainter>::to_PtrOfInpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_ColorAverageInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}

	// cv::Ptr<cv::videostab::ColorAverageInpainter>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::ColorAverageInpainter>::new", vec![(pred!(const, ["val"], ["cv::videostab::ColorAverageInpainter"]), _)]),
	cv::Ptr<cv::videostab::ColorAverageInpainter>* cv_PtrLcv_videostab_ColorAverageInpainterG_new_const_ColorAverageInpainter(cv::videostab::ColorAverageInpainter* val) {
			return new cv::Ptr<cv::videostab::ColorAverageInpainter>(val);
	}

}

