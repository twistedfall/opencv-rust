extern "C" {
	// cv::Ptr<cv::videostab::NullFrameSource>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::NullFrameSource>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::NullFrameSource* cv_PtrLcv_videostab_NullFrameSourceG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullFrameSource>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::NullFrameSource>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullFrameSource* cv_PtrLcv_videostab_NullFrameSourceG_getInnerPtrMut(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullFrameSource>::new_null() generated
	// ("cv::Ptr<cv::videostab::NullFrameSource>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::NullFrameSource>* cv_PtrLcv_videostab_NullFrameSourceG_new_null_const() {
			return new cv::Ptr<cv::videostab::NullFrameSource>();
	}

	// cv::Ptr<cv::videostab::NullFrameSource>::delete() generated
	// ("cv::Ptr<cv::videostab::NullFrameSource>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_NullFrameSourceG_delete(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::NullFrameSource>::to_PtrOfIFrameSource() generated
	// ("cv::Ptr<cv::videostab::NullFrameSource>::to_PtrOfIFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_NullFrameSourceG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}

	// cv::Ptr<cv::videostab::NullFrameSource>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::NullFrameSource>::new", vec![(pred!(const, ["val"], ["cv::videostab::NullFrameSource"]), _)]),
	cv::Ptr<cv::videostab::NullFrameSource>* cv_PtrLcv_videostab_NullFrameSourceG_new_const_NullFrameSource(cv::videostab::NullFrameSource* val) {
			return new cv::Ptr<cv::videostab::NullFrameSource>(val);
	}

}

