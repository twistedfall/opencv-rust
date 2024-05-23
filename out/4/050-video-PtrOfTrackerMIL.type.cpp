extern "C" {
	// cv::Ptr<cv::TrackerMIL>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerMIL>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerMIL* cv_PtrLcv_TrackerMILG_getInnerPtr_const(const cv::Ptr<cv::TrackerMIL>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerMIL>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerMIL>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerMIL* cv_PtrLcv_TrackerMILG_getInnerPtrMut(cv::Ptr<cv::TrackerMIL>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerMIL>::new_null() generated
	// ("cv::Ptr<cv::TrackerMIL>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerMIL>* cv_PtrLcv_TrackerMILG_new_null_const() {
			return new cv::Ptr<cv::TrackerMIL>();
	}

	// cv::Ptr<cv::TrackerMIL>::delete() generated
	// ("cv::Ptr<cv::TrackerMIL>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerMILG_delete(cv::Ptr<cv::TrackerMIL>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerMIL>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerMIL>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerMILG_to_PtrOfTracker(cv::Ptr<cv::TrackerMIL>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

