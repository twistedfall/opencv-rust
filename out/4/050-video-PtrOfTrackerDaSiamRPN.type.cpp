extern "C" {
	// cv::Ptr<cv::TrackerDaSiamRPN>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerDaSiamRPN>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerDaSiamRPN* cv_PtrLcv_TrackerDaSiamRPNG_getInnerPtr_const(const cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerDaSiamRPN>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerDaSiamRPN>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerDaSiamRPN* cv_PtrLcv_TrackerDaSiamRPNG_getInnerPtrMut(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerDaSiamRPN>::new_null() generated
	// ("cv::Ptr<cv::TrackerDaSiamRPN>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerDaSiamRPN>* cv_PtrLcv_TrackerDaSiamRPNG_new_null_const() {
			return new cv::Ptr<cv::TrackerDaSiamRPN>();
	}

	// cv::Ptr<cv::TrackerDaSiamRPN>::delete() generated
	// ("cv::Ptr<cv::TrackerDaSiamRPN>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerDaSiamRPNG_delete(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerDaSiamRPN>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerDaSiamRPN>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerDaSiamRPNG_to_PtrOfTracker(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

