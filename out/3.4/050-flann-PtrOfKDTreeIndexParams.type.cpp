extern "C" {
	// cv::Ptr<cv::flann::KDTreeIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::KDTreeIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::KDTreeIndexParams* cv_PtrLcv_flann_KDTreeIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::KDTreeIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::KDTreeIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::KDTreeIndexParams* cv_PtrLcv_flann_KDTreeIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::KDTreeIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::KDTreeIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::KDTreeIndexParams>* cv_PtrLcv_flann_KDTreeIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::KDTreeIndexParams>();
	}

	// cv::Ptr<cv::flann::KDTreeIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::KDTreeIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_KDTreeIndexParamsG_delete(cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::KDTreeIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::KDTreeIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_KDTreeIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::KDTreeIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::KDTreeIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::KDTreeIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::KDTreeIndexParams"]), _)]),
	cv::Ptr<cv::flann::KDTreeIndexParams>* cv_PtrLcv_flann_KDTreeIndexParamsG_new_const_KDTreeIndexParams(cv::flann::KDTreeIndexParams* val) {
			return new cv::Ptr<cv::flann::KDTreeIndexParams>(val);
	}

}

