extern "C" {
	// cv::Ptr<cv::videostab::MotionInpainter>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MotionInpainter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MotionInpainter* cv_PtrLcv_videostab_MotionInpainterG_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionInpainter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MotionInpainter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionInpainter* cv_PtrLcv_videostab_MotionInpainterG_getInnerPtrMut(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionInpainter>::new_null() generated
	// ("cv::Ptr<cv::videostab::MotionInpainter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MotionInpainter>* cv_PtrLcv_videostab_MotionInpainterG_new_null_const() {
			return new cv::Ptr<cv::videostab::MotionInpainter>();
	}

	// cv::Ptr<cv::videostab::MotionInpainter>::delete() generated
	// ("cv::Ptr<cv::videostab::MotionInpainter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MotionInpainterG_delete(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MotionInpainter>::to_PtrOfInpainterBase() generated
	// ("cv::Ptr<cv::videostab::MotionInpainter>::to_PtrOfInpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_MotionInpainterG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}

	// cv::Ptr<cv::videostab::MotionInpainter>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::MotionInpainter>::new", vec![(pred!(const, ["val"], ["cv::videostab::MotionInpainter"]), _)]),
	cv::Ptr<cv::videostab::MotionInpainter>* cv_PtrLcv_videostab_MotionInpainterG_new_const_MotionInpainter(cv::videostab::MotionInpainter* val) {
			return new cv::Ptr<cv::videostab::MotionInpainter>(val);
	}

}

