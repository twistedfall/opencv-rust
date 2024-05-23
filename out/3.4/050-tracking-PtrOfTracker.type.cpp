extern "C" {
	// cv::Ptr<cv::Tracker>::getInnerPtr() generated
	// ("cv::Ptr<cv::Tracker>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Tracker* cv_PtrLcv_TrackerG_getInnerPtr_const(const cv::Ptr<cv::Tracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Tracker>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Tracker>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_PtrLcv_TrackerG_getInnerPtrMut(cv::Ptr<cv::Tracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Tracker>::new_null() generated
	// ("cv::Ptr<cv::Tracker>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerG_new_null_const() {
			return new cv::Ptr<cv::Tracker>();
	}

	// cv::Ptr<cv::Tracker>::delete() generated
	// ("cv::Ptr<cv::Tracker>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerG_delete(cv::Ptr<cv::Tracker>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::Tracker>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::Tracker>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::Tracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

