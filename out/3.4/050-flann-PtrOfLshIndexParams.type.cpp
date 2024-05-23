extern "C" {
	// cv::Ptr<cv::flann::LshIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::LshIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::LshIndexParams* cv_PtrLcv_flann_LshIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::LshIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::LshIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::LshIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::LshIndexParams* cv_PtrLcv_flann_LshIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::LshIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::LshIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::LshIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::LshIndexParams>* cv_PtrLcv_flann_LshIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::LshIndexParams>();
	}

	// cv::Ptr<cv::flann::LshIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::LshIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_LshIndexParamsG_delete(cv::Ptr<cv::flann::LshIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::LshIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::LshIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_LshIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::LshIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::LshIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::LshIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::LshIndexParams"]), _)]),
	cv::Ptr<cv::flann::LshIndexParams>* cv_PtrLcv_flann_LshIndexParamsG_new_const_LshIndexParams(cv::flann::LshIndexParams* val) {
			return new cv::Ptr<cv::flann::LshIndexParams>(val);
	}

}

