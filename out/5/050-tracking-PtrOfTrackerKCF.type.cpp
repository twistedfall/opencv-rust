extern "C" {
	// cv::Ptr<cv::tracking::TrackerKCF>::getInnerPtr() generated
	// ("cv::Ptr<cv::tracking::TrackerKCF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::tracking::TrackerKCF* cv_PtrLcv_tracking_TrackerKCFG_getInnerPtr_const(const cv::Ptr<cv::tracking::TrackerKCF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::tracking::TrackerKCF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::tracking::TrackerKCF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::tracking::TrackerKCF* cv_PtrLcv_tracking_TrackerKCFG_getInnerPtrMut(cv::Ptr<cv::tracking::TrackerKCF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::tracking::TrackerKCF>::new_null() generated
	// ("cv::Ptr<cv::tracking::TrackerKCF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::tracking::TrackerKCF>* cv_PtrLcv_tracking_TrackerKCFG_new_null_const() {
			return new cv::Ptr<cv::tracking::TrackerKCF>();
	}

	// cv::Ptr<cv::tracking::TrackerKCF>::delete() generated
	// ("cv::Ptr<cv::tracking::TrackerKCF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_tracking_TrackerKCFG_delete(cv::Ptr<cv::tracking::TrackerKCF>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::tracking::TrackerKCF>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::tracking::TrackerKCF>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_tracking_TrackerKCFG_to_PtrOfTracker(cv::Ptr<cv::tracking::TrackerKCF>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

