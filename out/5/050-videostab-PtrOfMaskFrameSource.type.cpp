extern "C" {
	// cv::Ptr<cv::videostab::MaskFrameSource>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MaskFrameSource>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MaskFrameSource* cv_PtrLcv_videostab_MaskFrameSourceG_getInnerPtr_const(const cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MaskFrameSource>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MaskFrameSource>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MaskFrameSource* cv_PtrLcv_videostab_MaskFrameSourceG_getInnerPtrMut(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MaskFrameSource>::new_null() generated
	// ("cv::Ptr<cv::videostab::MaskFrameSource>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MaskFrameSource>* cv_PtrLcv_videostab_MaskFrameSourceG_new_null_const() {
			return new cv::Ptr<cv::videostab::MaskFrameSource>();
	}

	// cv::Ptr<cv::videostab::MaskFrameSource>::delete() generated
	// ("cv::Ptr<cv::videostab::MaskFrameSource>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MaskFrameSourceG_delete(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MaskFrameSource>::to_PtrOfIFrameSource() generated
	// ("cv::Ptr<cv::videostab::MaskFrameSource>::to_PtrOfIFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_MaskFrameSourceG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}

	// cv::Ptr<cv::videostab::MaskFrameSource>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::MaskFrameSource>::new", vec![(pred!(const, ["val"], ["cv::videostab::MaskFrameSource"]), _)]),
	cv::Ptr<cv::videostab::MaskFrameSource>* cv_PtrLcv_videostab_MaskFrameSourceG_new_const_MaskFrameSource(cv::videostab::MaskFrameSource* val) {
			return new cv::Ptr<cv::videostab::MaskFrameSource>(val);
	}

}

