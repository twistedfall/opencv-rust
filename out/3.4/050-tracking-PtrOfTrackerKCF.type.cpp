extern "C" {
	// cv::Ptr<cv::TrackerKCF>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerKCF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerKCF* cv_PtrLcv_TrackerKCFG_getInnerPtr_const(const cv::Ptr<cv::TrackerKCF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerKCF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerKCF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerKCF* cv_PtrLcv_TrackerKCFG_getInnerPtrMut(cv::Ptr<cv::TrackerKCF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerKCF>::new_null() generated
	// ("cv::Ptr<cv::TrackerKCF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerKCF>* cv_PtrLcv_TrackerKCFG_new_null_const() {
			return new cv::Ptr<cv::TrackerKCF>();
	}

	// cv::Ptr<cv::TrackerKCF>::delete() generated
	// ("cv::Ptr<cv::TrackerKCF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerKCFG_delete(cv::Ptr<cv::TrackerKCF>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerKCF>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TrackerKCF>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TrackerKCFG_to_PtrOfAlgorithm(cv::Ptr<cv::TrackerKCF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TrackerKCF>::to_PtrOfTracker() generated
	// ("cv::Ptr<cv::TrackerKCF>::to_PtrOfTracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerKCFG_to_PtrOfTracker(cv::Ptr<cv::TrackerKCF>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}

}

