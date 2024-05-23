extern "C" {
	// cv::Ptr<cv::flann::SearchParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::SearchParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::SearchParams* cv_PtrLcv_flann_SearchParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::SearchParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::SearchParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::SearchParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::SearchParams* cv_PtrLcv_flann_SearchParamsG_getInnerPtrMut(cv::Ptr<cv::flann::SearchParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::SearchParams>::new_null() generated
	// ("cv::Ptr<cv::flann::SearchParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::SearchParams>* cv_PtrLcv_flann_SearchParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::SearchParams>();
	}

	// cv::Ptr<cv::flann::SearchParams>::delete() generated
	// ("cv::Ptr<cv::flann::SearchParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_SearchParamsG_delete(cv::Ptr<cv::flann::SearchParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::SearchParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::SearchParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_SearchParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::SearchParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::SearchParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::SearchParams>::new", vec![(pred!(const, ["val"], ["cv::flann::SearchParams"]), _)]),
	cv::Ptr<cv::flann::SearchParams>* cv_PtrLcv_flann_SearchParamsG_new_const_SearchParams(cv::flann::SearchParams* val) {
			return new cv::Ptr<cv::flann::SearchParams>(val);
	}

}

