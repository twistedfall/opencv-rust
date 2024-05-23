extern "C" {
	// cv::Ptr<cv::colored_kinfu::Params>::getInnerPtr() generated
	// ("cv::Ptr<cv::colored_kinfu::Params>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::colored_kinfu::Params* cv_PtrLcv_colored_kinfu_ParamsG_getInnerPtr_const(const cv::Ptr<cv::colored_kinfu::Params>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::colored_kinfu::Params>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::colored_kinfu::Params>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::colored_kinfu::Params* cv_PtrLcv_colored_kinfu_ParamsG_getInnerPtrMut(cv::Ptr<cv::colored_kinfu::Params>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::colored_kinfu::Params>::new_null() generated
	// ("cv::Ptr<cv::colored_kinfu::Params>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::colored_kinfu::Params>* cv_PtrLcv_colored_kinfu_ParamsG_new_null_const() {
			return new cv::Ptr<cv::colored_kinfu::Params>();
	}

	// cv::Ptr<cv::colored_kinfu::Params>::delete() generated
	// ("cv::Ptr<cv::colored_kinfu::Params>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_colored_kinfu_ParamsG_delete(cv::Ptr<cv::colored_kinfu::Params>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::colored_kinfu::Params>::new(TraitClass) generated
	// ("cv::Ptr<cv::colored_kinfu::Params>::new", vec![(pred!(const, ["val"], ["cv::colored_kinfu::Params"]), _)]),
	cv::Ptr<cv::colored_kinfu::Params>* cv_PtrLcv_colored_kinfu_ParamsG_new_const_Params(cv::colored_kinfu::Params* val) {
			return new cv::Ptr<cv::colored_kinfu::Params>(val);
	}

}

