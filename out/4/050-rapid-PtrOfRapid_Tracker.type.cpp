extern "C" {
	// cv::Ptr<cv::rapid::Tracker>::getInnerPtr() generated
	// ("cv::Ptr<cv::rapid::Tracker>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rapid::Tracker* cv_PtrLcv_rapid_TrackerG_getInnerPtr_const(const cv::Ptr<cv::rapid::Tracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::Tracker>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rapid::Tracker>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rapid::Tracker* cv_PtrLcv_rapid_TrackerG_getInnerPtrMut(cv::Ptr<cv::rapid::Tracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::Tracker>::new_null() generated
	// ("cv::Ptr<cv::rapid::Tracker>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rapid::Tracker>* cv_PtrLcv_rapid_TrackerG_new_null_const() {
			return new cv::Ptr<cv::rapid::Tracker>();
	}

	// cv::Ptr<cv::rapid::Tracker>::delete() generated
	// ("cv::Ptr<cv::rapid::Tracker>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rapid_TrackerG_delete(cv::Ptr<cv::rapid::Tracker>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rapid::Tracker>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rapid::Tracker>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_TrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::Tracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

