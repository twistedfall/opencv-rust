extern "C" {
	// cv::Ptr<cv::videostab::LogToStdout>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::LogToStdout>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::LogToStdout* cv_PtrLcv_videostab_LogToStdoutG_getInnerPtr_const(const cv::Ptr<cv::videostab::LogToStdout>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::LogToStdout>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::LogToStdout>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::LogToStdout* cv_PtrLcv_videostab_LogToStdoutG_getInnerPtrMut(cv::Ptr<cv::videostab::LogToStdout>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::LogToStdout>::new_null() generated
	// ("cv::Ptr<cv::videostab::LogToStdout>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::LogToStdout>* cv_PtrLcv_videostab_LogToStdoutG_new_null_const() {
			return new cv::Ptr<cv::videostab::LogToStdout>();
	}

	// cv::Ptr<cv::videostab::LogToStdout>::delete() generated
	// ("cv::Ptr<cv::videostab::LogToStdout>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_LogToStdoutG_delete(cv::Ptr<cv::videostab::LogToStdout>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::LogToStdout>::to_PtrOfILog() generated
	// ("cv::Ptr<cv::videostab::LogToStdout>::to_PtrOfILog", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ILog>* cv_PtrLcv_videostab_LogToStdoutG_to_PtrOfILog(cv::Ptr<cv::videostab::LogToStdout>* instance) {
			return new cv::Ptr<cv::videostab::ILog>(instance->dynamicCast<cv::videostab::ILog>());
	}

	// cv::Ptr<cv::videostab::LogToStdout>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::LogToStdout>::new", vec![(pred!(const, ["val"], ["cv::videostab::LogToStdout"]), _)]),
	cv::Ptr<cv::videostab::LogToStdout>* cv_PtrLcv_videostab_LogToStdoutG_new_const_LogToStdout(cv::videostab::LogToStdout* val) {
			return new cv::Ptr<cv::videostab::LogToStdout>(val);
	}

}

