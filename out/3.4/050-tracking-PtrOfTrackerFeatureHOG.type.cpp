extern "C" {
	// cv::Ptr<cv::TrackerFeatureHOG>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerFeatureHOG>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerFeatureHOG* cv_PtrLcv_TrackerFeatureHOGG_getInnerPtr_const(const cv::Ptr<cv::TrackerFeatureHOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureHOG>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerFeatureHOG>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureHOG* cv_PtrLcv_TrackerFeatureHOGG_getInnerPtrMut(cv::Ptr<cv::TrackerFeatureHOG>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureHOG>::new_null() generated
	// ("cv::Ptr<cv::TrackerFeatureHOG>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerFeatureHOG>* cv_PtrLcv_TrackerFeatureHOGG_new_null_const() {
			return new cv::Ptr<cv::TrackerFeatureHOG>();
	}

	// cv::Ptr<cv::TrackerFeatureHOG>::delete() generated
	// ("cv::Ptr<cv::TrackerFeatureHOG>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerFeatureHOGG_delete(cv::Ptr<cv::TrackerFeatureHOG>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerFeatureHOG>::to_PtrOfTrackerFeature() generated
	// ("cv::Ptr<cv::TrackerFeatureHOG>::to_PtrOfTrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerFeature>* cv_PtrLcv_TrackerFeatureHOGG_to_PtrOfTrackerFeature(cv::Ptr<cv::TrackerFeatureHOG>* instance) {
			return new cv::Ptr<cv::TrackerFeature>(instance->dynamicCast<cv::TrackerFeature>());
	}

	// cv::Ptr<cv::TrackerFeatureHOG>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerFeatureHOG>::new", vec![(pred!(const, ["val"], ["cv::TrackerFeatureHOG"]), _)]),
	cv::Ptr<cv::TrackerFeatureHOG>* cv_PtrLcv_TrackerFeatureHOGG_new_const_TrackerFeatureHOG(cv::TrackerFeatureHOG* val) {
			return new cv::Ptr<cv::TrackerFeatureHOG>(val);
	}

}

