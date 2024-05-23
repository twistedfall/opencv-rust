extern "C" {
	// cv::Ptr<cv::TrackerStateEstimatorSVM>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorSVM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerStateEstimatorSVM* cv_PtrLcv_TrackerStateEstimatorSVMG_getInnerPtr_const(const cv::Ptr<cv::TrackerStateEstimatorSVM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimatorSVM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorSVM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimatorSVM* cv_PtrLcv_TrackerStateEstimatorSVMG_getInnerPtrMut(cv::Ptr<cv::TrackerStateEstimatorSVM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerStateEstimatorSVM>::new_null() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorSVM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerStateEstimatorSVM>* cv_PtrLcv_TrackerStateEstimatorSVMG_new_null_const() {
			return new cv::Ptr<cv::TrackerStateEstimatorSVM>();
	}

	// cv::Ptr<cv::TrackerStateEstimatorSVM>::delete() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorSVM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerStateEstimatorSVMG_delete(cv::Ptr<cv::TrackerStateEstimatorSVM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerStateEstimatorSVM>::to_PtrOfTrackerStateEstimator() generated
	// ("cv::Ptr<cv::TrackerStateEstimatorSVM>::to_PtrOfTrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerStateEstimator>* cv_PtrLcv_TrackerStateEstimatorSVMG_to_PtrOfTrackerStateEstimator(cv::Ptr<cv::TrackerStateEstimatorSVM>* instance) {
			return new cv::Ptr<cv::TrackerStateEstimator>(instance->dynamicCast<cv::TrackerStateEstimator>());
	}

	// cv::Ptr<cv::TrackerStateEstimatorSVM>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerStateEstimatorSVM>::new", vec![(pred!(const, ["val"], ["cv::TrackerStateEstimatorSVM"]), _)]),
	cv::Ptr<cv::TrackerStateEstimatorSVM>* cv_PtrLcv_TrackerStateEstimatorSVMG_new_const_TrackerStateEstimatorSVM(cv::TrackerStateEstimatorSVM* val) {
			return new cv::Ptr<cv::TrackerStateEstimatorSVM>(val);
	}

}

