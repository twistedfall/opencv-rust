extern "C" {
	// cv::Ptr<cv::TrackerNano>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerNano>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerNano* cv_PtrLcv_TrackerNanoG_getInnerPtr_const(const cv::Ptr<cv::TrackerNano>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerNano>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerNano>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerNano* cv_PtrLcv_TrackerNanoG_getInnerPtrMut(cv::Ptr<cv::TrackerNano>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerNano>::new_null() generated
	// ("cv::Ptr<cv::TrackerNano>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerNano>* cv_PtrLcv_TrackerNanoG_new_null_const() {
			return new cv::Ptr<cv::TrackerNano>();
	}

	// cv::Ptr<cv::TrackerNano>::delete() generated
	// ("cv::Ptr<cv::TrackerNano>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerNanoG_delete(cv::Ptr<cv::TrackerNano>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerNano>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerNano>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerNanoG_to_PtrOfTracker(cv::Ptr<cv::TrackerNano>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

