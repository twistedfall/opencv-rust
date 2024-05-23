extern "C" {
	// cv::Ptr<cv::flann::AutotunedIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::AutotunedIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::AutotunedIndexParams* cv_PtrLcv_flann_AutotunedIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::AutotunedIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::AutotunedIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::AutotunedIndexParams* cv_PtrLcv_flann_AutotunedIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::AutotunedIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::AutotunedIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::AutotunedIndexParams>* cv_PtrLcv_flann_AutotunedIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::AutotunedIndexParams>();
	}

	// cv::Ptr<cv::flann::AutotunedIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::AutotunedIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_AutotunedIndexParamsG_delete(cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::AutotunedIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::AutotunedIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_AutotunedIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::AutotunedIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::AutotunedIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::AutotunedIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::AutotunedIndexParams"]), _)]),
	cv::Ptr<cv::flann::AutotunedIndexParams>* cv_PtrLcv_flann_AutotunedIndexParamsG_new_const_AutotunedIndexParams(cv::flann::AutotunedIndexParams* val) {
			return new cv::Ptr<cv::flann::AutotunedIndexParams>(val);
	}

}

