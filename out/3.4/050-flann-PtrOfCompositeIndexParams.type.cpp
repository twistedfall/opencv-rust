extern "C" {
	// cv::Ptr<cv::flann::CompositeIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::CompositeIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::CompositeIndexParams* cv_PtrLcv_flann_CompositeIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::CompositeIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::CompositeIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::CompositeIndexParams* cv_PtrLcv_flann_CompositeIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::CompositeIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::CompositeIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::CompositeIndexParams>* cv_PtrLcv_flann_CompositeIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::CompositeIndexParams>();
	}

	// cv::Ptr<cv::flann::CompositeIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::CompositeIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_CompositeIndexParamsG_delete(cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::CompositeIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::CompositeIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_CompositeIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::CompositeIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::CompositeIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::CompositeIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::CompositeIndexParams"]), _)]),
	cv::Ptr<cv::flann::CompositeIndexParams>* cv_PtrLcv_flann_CompositeIndexParamsG_new_const_CompositeIndexParams(cv::flann::CompositeIndexParams* val) {
			return new cv::Ptr<cv::flann::CompositeIndexParams>(val);
	}

}

