extern "C" {
	// cv::Ptr<cv::flann::KMeansIndexParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::flann::KMeansIndexParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::flann::KMeansIndexParams* cv_PtrLcv_flann_KMeansIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::KMeansIndexParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::flann::KMeansIndexParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::flann::KMeansIndexParams* cv_PtrLcv_flann_KMeansIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::flann::KMeansIndexParams>::new_null() generated
	// ("cv::Ptr<cv::flann::KMeansIndexParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::flann::KMeansIndexParams>* cv_PtrLcv_flann_KMeansIndexParamsG_new_null_const() {
			return new cv::Ptr<cv::flann::KMeansIndexParams>();
	}

	// cv::Ptr<cv::flann::KMeansIndexParams>::delete() generated
	// ("cv::Ptr<cv::flann::KMeansIndexParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_flann_KMeansIndexParamsG_delete(cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::flann::KMeansIndexParams>::to_PtrOfIndexParams() generated
	// ("cv::Ptr<cv::flann::KMeansIndexParams>::to_PtrOfIndexParams", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_KMeansIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::KMeansIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}

	// cv::Ptr<cv::flann::KMeansIndexParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::flann::KMeansIndexParams>::new", vec![(pred!(const, ["val"], ["cv::flann::KMeansIndexParams"]), _)]),
	cv::Ptr<cv::flann::KMeansIndexParams>* cv_PtrLcv_flann_KMeansIndexParamsG_new_const_KMeansIndexParams(cv::flann::KMeansIndexParams* val) {
			return new cv::Ptr<cv::flann::KMeansIndexParams>(val);
	}

}

