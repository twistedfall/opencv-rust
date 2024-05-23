extern "C" {
	// cv::Ptr<cv::kinfu::Params>::getInnerPtr() generated
	// ("cv::Ptr<cv::kinfu::Params>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::kinfu::Params* cv_PtrLcv_kinfu_ParamsG_getInnerPtr_const(const cv::Ptr<cv::kinfu::Params>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::Params>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::kinfu::Params>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::kinfu::Params* cv_PtrLcv_kinfu_ParamsG_getInnerPtrMut(cv::Ptr<cv::kinfu::Params>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::Params>::new_null() generated
	// ("cv::Ptr<cv::kinfu::Params>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::kinfu::Params>* cv_PtrLcv_kinfu_ParamsG_new_null_const() {
			return new cv::Ptr<cv::kinfu::Params>();
	}

	// cv::Ptr<cv::kinfu::Params>::delete() generated
	// ("cv::Ptr<cv::kinfu::Params>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_kinfu_ParamsG_delete(cv::Ptr<cv::kinfu::Params>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::kinfu::Params>::new(TraitClass) generated
	// ("cv::Ptr<cv::kinfu::Params>::new", vec![(pred!(const, ["val"], ["cv::kinfu::Params"]), _)]),
	cv::Ptr<cv::kinfu::Params>* cv_PtrLcv_kinfu_ParamsG_new_const_Params(cv::kinfu::Params* val) {
			return new cv::Ptr<cv::kinfu::Params>(val);
	}

}

