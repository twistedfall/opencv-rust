extern "C" {
	// cv::Ptr<cv::flann::IndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::IndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::IndexParams* cv_PtrLcv_flann_IndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::IndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::IndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::IndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::IndexParams* cv_PtrLcv_flann_IndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::IndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::IndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::IndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_IndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::IndexParams>();
	}

	// cv::Ptr<cv::flann::IndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::IndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_IndexParamsG_delete(cv::Ptr<cv::flann::IndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::IndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::IndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::IndexParams"]), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_IndexParamsG_new_const_IndexParams(cv::flann::IndexParams* val) {
			return new cv::Ptr<cv::flann::IndexParams>(val);
	}

}

