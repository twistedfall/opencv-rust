extern "C" {
	// cv::Ptr<cv::TrackerFeatureHAAR>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerFeatureHAAR>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerFeatureHAAR* cv_PtrLcv_TrackerFeatureHAARG_getInnerPtr_const(const cv::Ptr<cv::TrackerFeatureHAAR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureHAAR>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerFeatureHAAR>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureHAAR* cv_PtrLcv_TrackerFeatureHAARG_getInnerPtrMut(cv::Ptr<cv::TrackerFeatureHAAR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureHAAR>::new_null() generated
	// ("cv::Ptr<cv::TrackerFeatureHAAR>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerFeatureHAAR>* cv_PtrLcv_TrackerFeatureHAARG_new_null_const() {
			return new cv::Ptr<cv::TrackerFeatureHAAR>();
	}

	// cv::Ptr<cv::TrackerFeatureHAAR>::delete() generated
	// ("cv::Ptr<cv::TrackerFeatureHAAR>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerFeatureHAARG_delete(cv::Ptr<cv::TrackerFeatureHAAR>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerFeatureHAAR>::to_PtrOfTrackerFeature() generated
	// ("cv::Ptr<cv::TrackerFeatureHAAR>::to_PtrOfTrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerFeature>* cv_PtrLcv_TrackerFeatureHAARG_to_PtrOfTrackerFeature(cv::Ptr<cv::TrackerFeatureHAAR>* instance) {
			return new cv::Ptr<cv::TrackerFeature>(instance->dynamicCast<cv::TrackerFeature>());
	}

	// cv::Ptr<cv::TrackerFeatureHAAR>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerFeatureHAAR>::new", vec![(pred!(const, ["val"], ["cv::TrackerFeatureHAAR"]), _)]),
	cv::Ptr<cv::TrackerFeatureHAAR>* cv_PtrLcv_TrackerFeatureHAARG_new_const_TrackerFeatureHAAR(cv::TrackerFeatureHAAR* val) {
			return new cv::Ptr<cv::TrackerFeatureHAAR>(val);
	}

}

