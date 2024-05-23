extern "C" {
	// cv::Ptr<cv::TrackerBoosting>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerBoosting>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerBoosting* cv_PtrLcv_TrackerBoostingG_getInnerPtr_const(const cv::Ptr<cv::TrackerBoosting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerBoosting>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerBoosting>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerBoosting* cv_PtrLcv_TrackerBoostingG_getInnerPtrMut(cv::Ptr<cv::TrackerBoosting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerBoosting>::new_null() generated
	// ("cv::Ptr<cv::TrackerBoosting>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerBoosting>* cv_PtrLcv_TrackerBoostingG_new_null_const() {
			return new cv::Ptr<cv::TrackerBoosting>();
	}

	// cv::Ptr<cv::TrackerBoosting>::delete() generated
	// ("cv::Ptr<cv::TrackerBoosting>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerBoostingG_delete(cv::Ptr<cv::TrackerBoosting>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerBoosting>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TrackerBoosting>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerBoostingG_to_PtrOfAlgorithm(cv::Ptr<cv::TrackerBoosting>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TrackerBoosting>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerBoosting>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerBoostingG_to_PtrOfTracker(cv::Ptr<cv::TrackerBoosting>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

