extern "C" {
	// cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerStateEstimatorMILBoosting* cv_PtrLcv_TrackerStateEstimatorMILBoostingG_getInnerPtr_const(const cv::Ptr<cv::TrackerStateEstimatorMILBoosting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimatorMILBoosting* cv_PtrLcv_TrackerStateEstimatorMILBoostingG_getInnerPtrMut(cv::Ptr<cv::TrackerStateEstimatorMILBoosting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::new_null() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerStateEstimatorMILBoosting>* cv_PtrLcv_TrackerStateEstimatorMILBoostingG_new_null_const() {
			return new cv::Ptr<cv::TrackerStateEstimatorMILBoosting>();
	}

	// cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::delete() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerStateEstimatorMILBoostingG_delete(cv::Ptr<cv::TrackerStateEstimatorMILBoosting>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::to_PtrOfTrackerStateEstimator() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::to_PtrOfTrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerStateEstimator>* cv_PtrLcv_TrackerStateEstimatorMILBoostingG_to_PtrOfTrackerStateEstimator(cv::Ptr<cv::TrackerStateEstimatorMILBoosting>* instance) {
			return new cv::Ptr<cv::TrackerStateEstimator>(instance->dynamicCast<cv::TrackerStateEstimator>());
	}

	// cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerStateEstimatorMILBoosting>::new", vec![(pred!(const, ["val"], ["cv::TrackerStateEstimatorMILBoosting"]), _)]),
	cv::Ptr<cv::TrackerStateEstimatorMILBoosting>* cv_PtrLcv_TrackerStateEstimatorMILBoostingG_new_const_TrackerStateEstimatorMILBoosting(cv::TrackerStateEstimatorMILBoosting* val) {
			return new cv::Ptr<cv::TrackerStateEstimatorMILBoosting>(val);
	}

}

