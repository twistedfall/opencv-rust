extern "C" {
	// cv::Ptr<cv::TrackerFeatureLBP>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerFeatureLBP>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerFeatureLBP* cv_PtrLcv_TrackerFeatureLBPG_getInnerPtr_const(const cv::Ptr<cv::TrackerFeatureLBP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureLBP>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerFeatureLBP>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureLBP* cv_PtrLcv_TrackerFeatureLBPG_getInnerPtrMut(cv::Ptr<cv::TrackerFeatureLBP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeatureLBP>::new_null() generated
	// ("cv::Ptr<cv::TrackerFeatureLBP>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerFeatureLBP>* cv_PtrLcv_TrackerFeatureLBPG_new_null_const() {
			return new cv::Ptr<cv::TrackerFeatureLBP>();
	}

	// cv::Ptr<cv::TrackerFeatureLBP>::delete() generated
	// ("cv::Ptr<cv::TrackerFeatureLBP>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerFeatureLBPG_delete(cv::Ptr<cv::TrackerFeatureLBP>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerFeatureLBP>::to_PtrOfTrackerFeature() generated
	// ("cv::Ptr<cv::TrackerFeatureLBP>::to_PtrOfTrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerFeature>* cv_PtrLcv_TrackerFeatureLBPG_to_PtrOfTrackerFeature(cv::Ptr<cv::TrackerFeatureLBP>* instance) {
			return new cv::Ptr<cv::TrackerFeature>(instance->dynamicCast<cv::TrackerFeature>());
	}

	// cv::Ptr<cv::TrackerFeatureLBP>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerFeatureLBP>::new", vec![(pred!(const, ["val"], ["cv::TrackerFeatureLBP"]), _)]),
	cv::Ptr<cv::TrackerFeatureLBP>* cv_PtrLcv_TrackerFeatureLBPG_new_const_TrackerFeatureLBP(cv::TrackerFeatureLBP* val) {
			return new cv::Ptr<cv::TrackerFeatureLBP>(val);
	}

}

