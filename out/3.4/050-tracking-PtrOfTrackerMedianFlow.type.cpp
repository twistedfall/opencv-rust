extern "C" {
	// cv::Ptr<cv::TrackerMedianFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerMedianFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerMedianFlow* cv_PtrLcv_TrackerMedianFlowG_getInnerPtr_const(const cv::Ptr<cv::TrackerMedianFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerMedianFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerMedianFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerMedianFlow* cv_PtrLcv_TrackerMedianFlowG_getInnerPtrMut(cv::Ptr<cv::TrackerMedianFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerMedianFlow>::new_null() generated
	// ("cv::Ptr<cv::TrackerMedianFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerMedianFlow>* cv_PtrLcv_TrackerMedianFlowG_new_null_const() {
			return new cv::Ptr<cv::TrackerMedianFlow>();
	}

	// cv::Ptr<cv::TrackerMedianFlow>::delete() generated
	// ("cv::Ptr<cv::TrackerMedianFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerMedianFlowG_delete(cv::Ptr<cv::TrackerMedianFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerMedianFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TrackerMedianFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerMedianFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::TrackerMedianFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TrackerMedianFlow>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerMedianFlow>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerMedianFlowG_to_PtrOfTracker(cv::Ptr<cv::TrackerMedianFlow>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

