extern "C" {
	// cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerStateEstimatorAdaBoosting* cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_getInnerPtr_const(const cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimatorAdaBoosting* cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_getInnerPtrMut(cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::new_null() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>* cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_new_null_const() {
			return new cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>();
	}

	// cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::delete() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_delete(cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::to_PtrOfTrackerStateEstimator() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::to_PtrOfTrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerStateEstimator>* cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_to_PtrOfTrackerStateEstimator(cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>* instance) {
			return new cv::Ptr<cv::TrackerStateEstimator>(instance->dynamicCast<cv::TrackerStateEstimator>());
	}

	// cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>::new", vec![(pred!(const, ["val"], ["cv::TrackerStateEstimatorAdaBoosting"]), _)]),
	cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>* cv_PtrLcv_TrackerStateEstimatorAdaBoostingG_new_const_TrackerStateEstimatorAdaBoosting(cv::TrackerStateEstimatorAdaBoosting* val) {
			return new cv::Ptr<cv::TrackerStateEstimatorAdaBoosting>(val);
	}

}

