extern "C" {
	// cv::Ptr<cv::TrackerFeature>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerFeature>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerFeature* cv_PtrLcv_TrackerFeatureG_getInnerPtr_const(const cv::Ptr<cv::TrackerFeature>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeature>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerFeature>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeature* cv_PtrLcv_TrackerFeatureG_getInnerPtrMut(cv::Ptr<cv::TrackerFeature>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerFeature>::new_null() generated
	// ("cv::Ptr<cv::TrackerFeature>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerFeature>* cv_PtrLcv_TrackerFeatureG_new_null_const() {
			return new cv::Ptr<cv::TrackerFeature>();
	}

	// cv::Ptr<cv::TrackerFeature>::delete() generated
	// ("cv::Ptr<cv::TrackerFeature>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerFeatureG_delete(cv::Ptr<cv::TrackerFeature>* instance) {
			delete instance;
	}

}

