extern "C" {
	// cv::Ptr<cv::TrackerTargetState>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerTargetState>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerTargetState* cv_PtrLcv_TrackerTargetStateG_getInnerPtr_const(const cv::Ptr<cv::TrackerTargetState>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerTargetState>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerTargetState>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerTargetState* cv_PtrLcv_TrackerTargetStateG_getInnerPtrMut(cv::Ptr<cv::TrackerTargetState>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerTargetState>::new_null() generated
	// ("cv::Ptr<cv::TrackerTargetState>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerTargetState>* cv_PtrLcv_TrackerTargetStateG_new_null_const() {
			return new cv::Ptr<cv::TrackerTargetState>();
	}

	// cv::Ptr<cv::TrackerTargetState>::delete() generated
	// ("cv::Ptr<cv::TrackerTargetState>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerTargetStateG_delete(cv::Ptr<cv::TrackerTargetState>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerTargetState>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerTargetState>::new", vec![(pred!(const, ["val"], ["cv::TrackerTargetState"]), _)]),
	cv::Ptr<cv::TrackerTargetState>* cv_PtrLcv_TrackerTargetStateG_new_const_TrackerTargetState(cv::TrackerTargetState* val) {
			return new cv::Ptr<cv::TrackerTargetState>(val);
	}

}

