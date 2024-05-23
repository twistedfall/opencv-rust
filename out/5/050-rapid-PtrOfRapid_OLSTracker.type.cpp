extern "C" {
	// cv::Ptr<cv::rapid::OLSTracker>::getInnerPtr() generated
	// ("cv::Ptr<cv::rapid::OLSTracker>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rapid::OLSTracker* cv_PtrLcv_rapid_OLSTrackerG_getInnerPtr_const(const cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::OLSTracker>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rapid::OLSTracker>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rapid::OLSTracker* cv_PtrLcv_rapid_OLSTrackerG_getInnerPtrMut(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::OLSTracker>::new_null() generated
	// ("cv::Ptr<cv::rapid::OLSTracker>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rapid::OLSTracker>* cv_PtrLcv_rapid_OLSTrackerG_new_null_const() {
			return new cv::Ptr<cv::rapid::OLSTracker>();
	}

	// cv::Ptr<cv::rapid::OLSTracker>::delete() generated
	// ("cv::Ptr<cv::rapid::OLSTracker>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rapid_OLSTrackerG_delete(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rapid::OLSTracker>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rapid::OLSTracker>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_OLSTrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rapid::OLSTracker>::to_PtrOfRapid_Tracker() generated
	// ("cv::Ptr<cv::rapid::OLSTracker>::to_PtrOfRapid_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rapid::Tracker>* cv_PtrLcv_rapid_OLSTrackerG_to_PtrOfRapid_Tracker(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return new cv::Ptr<cv::rapid::Tracker>(instance->dynamicCast<cv::rapid::Tracker>());
	}

}

