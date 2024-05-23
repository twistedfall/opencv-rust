extern "C" {
	// cv::Ptr<cv::DetectionBasedTracker::IDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::DetectionBasedTracker::IDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::DetectionBasedTracker::IDetector* cv_PtrLcv_DetectionBasedTracker_IDetectorG_getInnerPtr_const(const cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DetectionBasedTracker::IDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::DetectionBasedTracker::IDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::DetectionBasedTracker::IDetector* cv_PtrLcv_DetectionBasedTracker_IDetectorG_getInnerPtrMut(cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DetectionBasedTracker::IDetector>::new_null() generated
	// ("cv::Ptr<cv::DetectionBasedTracker::IDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::DetectionBasedTracker::IDetector>* cv_PtrLcv_DetectionBasedTracker_IDetectorG_new_null_const() {
			return new cv::Ptr<cv::DetectionBasedTracker::IDetector>();
	}

	// cv::Ptr<cv::DetectionBasedTracker::IDetector>::delete() generated
	// ("cv::Ptr<cv::DetectionBasedTracker::IDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_DetectionBasedTracker_IDetectorG_delete(cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
			delete instance;
	}

}

