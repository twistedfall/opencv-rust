extern "C" {
	// cv::Ptr<cv::videostab::IFrameSource>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::IFrameSource>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::IFrameSource* cv_PtrLcv_videostab_IFrameSourceG_getInnerPtr_const(const cv::Ptr<cv::videostab::IFrameSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::IFrameSource>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::IFrameSource>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IFrameSource* cv_PtrLcv_videostab_IFrameSourceG_getInnerPtrMut(cv::Ptr<cv::videostab::IFrameSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::IFrameSource>::new_null() generated
	// ("cv::Ptr<cv::videostab::IFrameSource>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_IFrameSourceG_new_null_const() {
			return new cv::Ptr<cv::videostab::IFrameSource>();
	}

	// cv::Ptr<cv::videostab::IFrameSource>::delete() generated
	// ("cv::Ptr<cv::videostab::IFrameSource>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_IFrameSourceG_delete(cv::Ptr<cv::videostab::IFrameSource>* instance) {
			delete instance;
	}

}

