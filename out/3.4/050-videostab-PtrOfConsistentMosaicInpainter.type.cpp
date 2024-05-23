extern "C" {
	// cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::ConsistentMosaicInpainter* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ConsistentMosaicInpainter* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::new_null() generated
	// ("cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_new_null_const() {
			return new cv::Ptr<cv::videostab::ConsistentMosaicInpainter>();
	}

	// cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::delete() generated
	// ("cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_ConsistentMosaicInpainterG_delete(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::to_PtrOfInpainterBase() generated
	// ("cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::to_PtrOfInpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}

	// cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::ConsistentMosaicInpainter>::new", vec![(pred!(const, ["val"], ["cv::videostab::ConsistentMosaicInpainter"]), _)]),
	cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* cv_PtrLcv_videostab_ConsistentMosaicInpainterG_new_const_ConsistentMosaicInpainter(cv::videostab::ConsistentMosaicInpainter* val) {
			return new cv::Ptr<cv::videostab::ConsistentMosaicInpainter>(val);
	}

}

