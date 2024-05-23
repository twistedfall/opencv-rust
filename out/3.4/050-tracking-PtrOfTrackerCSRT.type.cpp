extern "C" {
	// cv::Ptr<cv::TrackerCSRT>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerCSRT>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerCSRT* cv_PtrLcv_TrackerCSRTG_getInnerPtr_const(const cv::Ptr<cv::TrackerCSRT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerCSRT>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerCSRT>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerCSRT* cv_PtrLcv_TrackerCSRTG_getInnerPtrMut(cv::Ptr<cv::TrackerCSRT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerCSRT>::new_null() generated
	// ("cv::Ptr<cv::TrackerCSRT>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerCSRT>* cv_PtrLcv_TrackerCSRTG_new_null_const() {
			return new cv::Ptr<cv::TrackerCSRT>();
	}

	// cv::Ptr<cv::TrackerCSRT>::delete() generated
	// ("cv::Ptr<cv::TrackerCSRT>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerCSRTG_delete(cv::Ptr<cv::TrackerCSRT>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerCSRT>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TrackerCSRT>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerCSRTG_to_PtrOfAlgorithm(cv::Ptr<cv::TrackerCSRT>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TrackerCSRT>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerCSRT>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerCSRTG_to_PtrOfTracker(cv::Ptr<cv::TrackerCSRT>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

