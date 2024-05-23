extern "C" {
	// cv::Ptr<cv::videostab::NullLog>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::NullLog>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::NullLog* cv_PtrLcv_videostab_NullLogG_getInnerPtr_const(const cv::Ptr<cv::videostab::NullLog>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullLog>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::NullLog>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullLog* cv_PtrLcv_videostab_NullLogG_getInnerPtrMut(cv::Ptr<cv::videostab::NullLog>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::NullLog>::new_null() generated
	// ("cv::Ptr<cv::videostab::NullLog>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::NullLog>* cv_PtrLcv_videostab_NullLogG_new_null_const() {
			return new cv::Ptr<cv::videostab::NullLog>();
	}

	// cv::Ptr<cv::videostab::NullLog>::delete() generated
	// ("cv::Ptr<cv::videostab::NullLog>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_NullLogG_delete(cv::Ptr<cv::videostab::NullLog>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::NullLog>::to_PtrOfILog() generated
	// ("cv::Ptr<cv::videostab::NullLog>::to_PtrOfILog", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ILog>* cv_PtrLcv_videostab_NullLogG_to_PtrOfILog(cv::Ptr<cv::videostab::NullLog>* instance) {
			return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}

	// cv::Ptr<cv::videostab::NullLog>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::NullLog>::new", vec![(pred!(const, ["val"], ["cv::videostab::NullLog"]), _)]),
	cv::Ptr<cv::videostab::NullLog>* cv_PtrLcv_videostab_NullLogG_new_const_NullLog(cv::videostab::NullLog* val) {
			return new cv::Ptr<cv::videostab::NullLog>(val);
	}

}

