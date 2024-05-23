extern "C" {
	// cv::Ptr<cv::flann::LinearIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::LinearIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::LinearIndexParams* cv_PtrLcv_flann_LinearIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::LinearIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::LinearIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::LinearIndexParams* cv_PtrLcv_flann_LinearIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::LinearIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::LinearIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::LinearIndexParams>* cv_PtrLcv_flann_LinearIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::LinearIndexParams>();
	}

	// cv::Ptr<cv::flann::LinearIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::LinearIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_LinearIndexParamsG_delete(cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::LinearIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::LinearIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_LinearIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::LinearIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::LinearIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::LinearIndexParams"]), _)]),
	cv::Ptr<cv::flann::LinearIndexParams>* cv_PtrLcv_flann_LinearIndexParamsG_new_const_LinearIndexParams(cv::flann::LinearIndexParams* val) {
			return new cv::Ptr<cv::flann::LinearIndexParams>(val);
	}

}

