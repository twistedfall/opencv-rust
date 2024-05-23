extern "C" {
	// cv::Ptr<cv::flann::SavedIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::SavedIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::SavedIndexParams* cv_PtrLcv_flann_SavedIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::SavedIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::SavedIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::SavedIndexParams* cv_PtrLcv_flann_SavedIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::SavedIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::SavedIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::SavedIndexParams>* cv_PtrLcv_flann_SavedIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::SavedIndexParams>();
	}

	// cv::Ptr<cv::flann::SavedIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::SavedIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_SavedIndexParamsG_delete(cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::SavedIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::SavedIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_SavedIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::SavedIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::SavedIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::SavedIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::SavedIndexParams"]), _)]),
	cv::Ptr<cv::flann::SavedIndexParams>* cv_PtrLcv_flann_SavedIndexParamsG_new_const_SavedIndexParams(cv::flann::SavedIndexParams* val) {
			return new cv::Ptr<cv::flann::SavedIndexParams>(val);
	}

}

