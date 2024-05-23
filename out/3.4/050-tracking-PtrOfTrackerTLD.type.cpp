extern "C" {
	// cv::Ptr<cv::TrackerTLD>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerTLD>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerTLD* cv_PtrLcv_TrackerTLDG_getInnerPtr_const(const cv::Ptr<cv::TrackerTLD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerTLD>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerTLD>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerTLD* cv_PtrLcv_TrackerTLDG_getInnerPtrMut(cv::Ptr<cv::TrackerTLD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerTLD>::new_null() generated
	// ("cv::Ptr<cv::TrackerTLD>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerTLD>* cv_PtrLcv_TrackerTLDG_new_null_const() {
			return new cv::Ptr<cv::TrackerTLD>();
	}

	// cv::Ptr<cv::TrackerTLD>::delete() generated
	// ("cv::Ptr<cv::TrackerTLD>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerTLDG_delete(cv::Ptr<cv::TrackerTLD>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerTLD>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TrackerTLD>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerTLDG_to_PtrOfAlgorithm(cv::Ptr<cv::TrackerTLD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TrackerTLD>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerTLD>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerTLDG_to_PtrOfTracker(cv::Ptr<cv::TrackerTLD>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

