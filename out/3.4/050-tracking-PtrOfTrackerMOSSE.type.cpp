extern "C" {
	// cv::Ptr<cv::TrackerMOSSE>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerMOSSE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerMOSSE* cv_PtrLcv_TrackerMOSSEG_getInnerPtr_const(const cv::Ptr<cv::TrackerMOSSE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerMOSSE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerMOSSE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerMOSSE* cv_PtrLcv_TrackerMOSSEG_getInnerPtrMut(cv::Ptr<cv::TrackerMOSSE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerMOSSE>::new_null() generated
	// ("cv::Ptr<cv::TrackerMOSSE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerMOSSE>* cv_PtrLcv_TrackerMOSSEG_new_null_const() {
			return new cv::Ptr<cv::TrackerMOSSE>();
	}

	// cv::Ptr<cv::TrackerMOSSE>::delete() generated
	// ("cv::Ptr<cv::TrackerMOSSE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerMOSSEG_delete(cv::Ptr<cv::TrackerMOSSE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerMOSSE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TrackerMOSSE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerMOSSEG_to_PtrOfAlgorithm(cv::Ptr<cv::TrackerMOSSE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TrackerMOSSE>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerMOSSE>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerMOSSEG_to_PtrOfTracker(cv::Ptr<cv::TrackerMOSSE>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

