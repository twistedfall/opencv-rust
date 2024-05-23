extern "C" {
	// cv::Ptr<cv::tracking::TrackerCSRT>::getInnerPtr() generated
	// ("cv::Ptr<cv::tracking::TrackerCSRT>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::tracking::TrackerCSRT* cv_PtrLcv_tracking_TrackerCSRTG_getInnerPtr_const(const cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::tracking::TrackerCSRT>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::tracking::TrackerCSRT>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::tracking::TrackerCSRT* cv_PtrLcv_tracking_TrackerCSRTG_getInnerPtrMut(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::tracking::TrackerCSRT>::new_null() generated
	// ("cv::Ptr<cv::tracking::TrackerCSRT>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::tracking::TrackerCSRT>* cv_PtrLcv_tracking_TrackerCSRTG_new_null_const() {
			return new cv::Ptr<cv::tracking::TrackerCSRT>();
	}

	// cv::Ptr<cv::tracking::TrackerCSRT>::delete() generated
	// ("cv::Ptr<cv::tracking::TrackerCSRT>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_tracking_TrackerCSRTG_delete(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::tracking::TrackerCSRT>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::tracking::TrackerCSRT>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_tracking_TrackerCSRTG_to_PtrOfTracker(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

