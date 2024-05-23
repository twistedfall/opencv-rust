extern "C" {
	// cv::Ptr<cv::videostab::VideoFileSource>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::VideoFileSource>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::VideoFileSource* cv_PtrLcv_videostab_VideoFileSourceG_getInnerPtr_const(const cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::VideoFileSource>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::VideoFileSource>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::VideoFileSource* cv_PtrLcv_videostab_VideoFileSourceG_getInnerPtrMut(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::VideoFileSource>::new_null() generated
	// ("cv::Ptr<cv::videostab::VideoFileSource>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::VideoFileSource>* cv_PtrLcv_videostab_VideoFileSourceG_new_null_const() {
			return new cv::Ptr<cv::videostab::VideoFileSource>();
	}

	// cv::Ptr<cv::videostab::VideoFileSource>::delete() generated
	// ("cv::Ptr<cv::videostab::VideoFileSource>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_VideoFileSourceG_delete(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::VideoFileSource>::to_PtrOfIFrameSource() generated
	// ("cv::Ptr<cv::videostab::VideoFileSource>::to_PtrOfIFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_VideoFileSourceG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}

	// cv::Ptr<cv::videostab::VideoFileSource>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::VideoFileSource>::new", vec![(pred!(const, ["val"], ["cv::videostab::VideoFileSource"]), _)]),
	cv::Ptr<cv::videostab::VideoFileSource>* cv_PtrLcv_videostab_VideoFileSourceG_new_const_VideoFileSource(cv::videostab::VideoFileSource* val) {
			return new cv::Ptr<cv::videostab::VideoFileSource>(val);
	}

}

