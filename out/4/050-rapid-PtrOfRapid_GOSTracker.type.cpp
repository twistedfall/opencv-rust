extern "C" {
	// cv::Ptr<cv::rapid::GOSTracker>::getInnerPtr() generated
	// ("cv::Ptr<cv::rapid::GOSTracker>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rapid::GOSTracker* cv_PtrLcv_rapid_GOSTrackerG_getInnerPtr_const(const cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::GOSTracker>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rapid::GOSTracker>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rapid::GOSTracker* cv_PtrLcv_rapid_GOSTrackerG_getInnerPtrMut(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::GOSTracker>::new_null() generated
	// ("cv::Ptr<cv::rapid::GOSTracker>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rapid::GOSTracker>* cv_PtrLcv_rapid_GOSTrackerG_new_null_const() {
			return new cv::Ptr<cv::rapid::GOSTracker>();
	}

	// cv::Ptr<cv::rapid::GOSTracker>::delete() generated
	// ("cv::Ptr<cv::rapid::GOSTracker>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rapid_GOSTrackerG_delete(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rapid::GOSTracker>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rapid::GOSTracker>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_GOSTrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rapid::GOSTracker>::to_PtrOfRapid_Tracker() generated
	// ("cv::Ptr<cv::rapid::GOSTracker>::to_PtrOfRapid_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rapid::Tracker>* cv_PtrLcv_rapid_GOSTrackerG_to_PtrOfRapid_Tracker(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return new cv::Ptr<cv::rapid::Tracker>(instance->dynamicCast<cv::rapid::Tracker>());
	}

}

