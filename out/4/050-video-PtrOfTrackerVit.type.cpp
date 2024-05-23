extern "C" {
	// cv::Ptr<cv::TrackerVit>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerVit>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerVit* cv_PtrLcv_TrackerVitG_getInnerPtr_const(const cv::Ptr<cv::TrackerVit>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerVit>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerVit>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerVit* cv_PtrLcv_TrackerVitG_getInnerPtrMut(cv::Ptr<cv::TrackerVit>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerVit>::new_null() generated
	// ("cv::Ptr<cv::TrackerVit>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerVit>* cv_PtrLcv_TrackerVitG_new_null_const() {
			return new cv::Ptr<cv::TrackerVit>();
	}

	// cv::Ptr<cv::TrackerVit>::delete() generated
	// ("cv::Ptr<cv::TrackerVit>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerVitG_delete(cv::Ptr<cv::TrackerVit>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerVit>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerVit>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerVitG_to_PtrOfTracker(cv::Ptr<cv::TrackerVit>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

