extern "C" {
	// cv::Ptr<cv::TrackerStateEstimator>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerStateEstimator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerStateEstimator* cv_PtrLcv_TrackerStateEstimatorG_getInnerPtr_const(const cv::Ptr<cv::TrackerStateEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerStateEstimator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimator* cv_PtrLcv_TrackerStateEstimatorG_getInnerPtrMut(cv::Ptr<cv::TrackerStateEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimator>::new_null() generated
	// ("cv::Ptr<cv::TrackerStateEstimator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerStateEstimator>* cv_PtrLcv_TrackerStateEstimatorG_new_null_const() {
			return new cv::Ptr<cv::TrackerStateEstimator>();
	}

	// cv::Ptr<cv::TrackerStateEstimator>::delete() generated
	// ("cv::Ptr<cv::TrackerStateEstimator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerStateEstimatorG_delete(cv::Ptr<cv::TrackerStateEstimator>* instance) {
			delete instance;
	}

}

