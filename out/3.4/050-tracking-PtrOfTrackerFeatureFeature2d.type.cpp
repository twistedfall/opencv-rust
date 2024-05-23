extern "C" {
	// cv::Ptr<cv::TrackerFeatureFeature2d>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerFeatureFeature2d>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerFeatureFeature2d* cv_PtrLcv_TrackerFeatureFeature2dG_getInnerPtr_const(const cv::Ptr<cv::TrackerFeatureFeature2d>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureFeature2d>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerFeatureFeature2d>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureFeature2d* cv_PtrLcv_TrackerFeatureFeature2dG_getInnerPtrMut(cv::Ptr<cv::TrackerFeatureFeature2d>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureFeature2d>::new_null() generated
	// ("cv::Ptr<cv::TrackerFeatureFeature2d>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerFeatureFeature2d>* cv_PtrLcv_TrackerFeatureFeature2dG_new_null_const() {
			return new cv::Ptr<cv::TrackerFeatureFeature2d>();
	}

	// cv::Ptr<cv::TrackerFeatureFeature2d>::delete() generated
	// ("cv::Ptr<cv::TrackerFeatureFeature2d>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerFeatureFeature2dG_delete(cv::Ptr<cv::TrackerFeatureFeature2d>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerFeatureFeature2d>::to_PtrOfTrackerFeature() generated
	// ("cv::Ptr<cv::TrackerFeatureFeature2d>::to_PtrOfTrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerFeature>* cv_PtrLcv_TrackerFeatureFeature2dG_to_PtrOfTrackerFeature(cv::Ptr<cv::TrackerFeatureFeature2d>* instance) {
			return new cv::Ptr<cv::TrackerFeature>(instance->dynamicCast<cv::TrackerFeature>());
	}

	// cv::Ptr<cv::TrackerFeatureFeature2d>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerFeatureFeature2d>::new", vec![(pred!(const, ["val"], ["cv::TrackerFeatureFeature2d"]), _)]),
	cv::Ptr<cv::TrackerFeatureFeature2d>* cv_PtrLcv_TrackerFeatureFeature2dG_new_const_TrackerFeatureFeature2d(cv::TrackerFeatureFeature2d* val) {
			return new cv::Ptr<cv::TrackerFeatureFeature2d>(val);
	}

}

