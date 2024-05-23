extern "C" {
	// cv::Ptr<cv::TrackerGOTURN>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerGOTURN>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerGOTURN* cv_PtrLcv_TrackerGOTURNG_getInnerPtr_const(const cv::Ptr<cv::TrackerGOTURN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerGOTURN>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerGOTURN>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerGOTURN* cv_PtrLcv_TrackerGOTURNG_getInnerPtrMut(cv::Ptr<cv::TrackerGOTURN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerGOTURN>::new_null() generated
	// ("cv::Ptr<cv::TrackerGOTURN>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerGOTURN>* cv_PtrLcv_TrackerGOTURNG_new_null_const() {
			return new cv::Ptr<cv::TrackerGOTURN>();
	}

	// cv::Ptr<cv::TrackerGOTURN>::delete() generated
	// ("cv::Ptr<cv::TrackerGOTURN>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerGOTURNG_delete(cv::Ptr<cv::TrackerGOTURN>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerGOTURN>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TrackerGOTURN>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerGOTURNG_to_PtrOfAlgorithm(cv::Ptr<cv::TrackerGOTURN>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TrackerGOTURN>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerGOTURN>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerGOTURNG_to_PtrOfTracker(cv::Ptr<cv::TrackerGOTURN>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

