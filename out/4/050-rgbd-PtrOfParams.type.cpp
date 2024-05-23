extern "C" {
	// cv::Ptr<cv::large_kinfu::Params>::getInnerPtr() generated
	// ("cv::Ptr<cv::large_kinfu::Params>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::large_kinfu::Params* cv_PtrLcv_large_kinfu_ParamsG_getInnerPtr_const(const cv::Ptr<cv::large_kinfu::Params>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::large_kinfu::Params>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::large_kinfu::Params>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::large_kinfu::Params* cv_PtrLcv_large_kinfu_ParamsG_getInnerPtrMut(cv::Ptr<cv::large_kinfu::Params>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::large_kinfu::Params>::new_null() generated
	// ("cv::Ptr<cv::large_kinfu::Params>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::large_kinfu::Params>* cv_PtrLcv_large_kinfu_ParamsG_new_null_const() {
			return new cv::Ptr<cv::large_kinfu::Params>();
	}

	// cv::Ptr<cv::large_kinfu::Params>::delete() generated
	// ("cv::Ptr<cv::large_kinfu::Params>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_large_kinfu_ParamsG_delete(cv::Ptr<cv::large_kinfu::Params>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::large_kinfu::Params>::new(TraitClass) generated
	// ("cv::Ptr<cv::large_kinfu::Params>::new", vec![(pred!(const, ["val"], ["cv::large_kinfu::Params"]), _)]),
	cv::Ptr<cv::large_kinfu::Params>* cv_PtrLcv_large_kinfu_ParamsG_new_const_Params(cv::large_kinfu::Params* val) {
			return new cv::Ptr<cv::large_kinfu::Params>(val);
	}

}

