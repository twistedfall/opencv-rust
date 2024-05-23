extern "C" {
	// cv::Ptr<cv::MultiTracker>::getInnerPtr() generated
	// ("cv::Ptr<cv::MultiTracker>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MultiTracker* cv_PtrLcv_MultiTrackerG_getInnerPtr_const(const cv::Ptr<cv::MultiTracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MultiTracker>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MultiTracker>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MultiTracker* cv_PtrLcv_MultiTrackerG_getInnerPtrMut(cv::Ptr<cv::MultiTracker>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MultiTracker>::new_null() generated
	// ("cv::Ptr<cv::MultiTracker>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MultiTracker>* cv_PtrLcv_MultiTrackerG_new_null_const() {
			return new cv::Ptr<cv::MultiTracker>();
	}

	// cv::Ptr<cv::MultiTracker>::delete() generated
	// ("cv::Ptr<cv::MultiTracker>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MultiTrackerG_delete(cv::Ptr<cv::MultiTracker>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MultiTracker>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::MultiTracker>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MultiTrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::MultiTracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::MultiTracker>::new(TraitClass) generated
	// ("cv::Ptr<cv::MultiTracker>::new", vec![(pred!(const, ["val"], ["cv::MultiTracker"]), _)]),
	cv::Ptr<cv::MultiTracker>* cv_PtrLcv_MultiTrackerG_new_const_MultiTracker(cv::MultiTracker* val) {
			return new cv::Ptr<cv::MultiTracker>(val);
	}

}

