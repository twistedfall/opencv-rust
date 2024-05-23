extern "C" {
	// cv::Ptr<cv::rapid::Rapid>::getInnerPtr() generated
	// ("cv::Ptr<cv::rapid::Rapid>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rapid::Rapid* cv_PtrLcv_rapid_RapidG_getInnerPtr_const(const cv::Ptr<cv::rapid::Rapid>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::Rapid>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rapid::Rapid>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rapid::Rapid* cv_PtrLcv_rapid_RapidG_getInnerPtrMut(cv::Ptr<cv::rapid::Rapid>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rapid::Rapid>::new_null() generated
	// ("cv::Ptr<cv::rapid::Rapid>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rapid::Rapid>* cv_PtrLcv_rapid_RapidG_new_null_const() {
			return new cv::Ptr<cv::rapid::Rapid>();
	}

	// cv::Ptr<cv::rapid::Rapid>::delete() generated
	// ("cv::Ptr<cv::rapid::Rapid>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rapid_RapidG_delete(cv::Ptr<cv::rapid::Rapid>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rapid::Rapid>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rapid::Rapid>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_RapidG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::Rapid>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rapid::Rapid>::to_PtrOfRapid_Tracker() generated
	// ("cv::Ptr<cv::rapid::Rapid>::to_PtrOfRapid_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rapid::Tracker>* cv_PtrLcv_rapid_RapidG_to_PtrOfRapid_Tracker(cv::Ptr<cv::rapid::Rapid>* instance) {
			return new cv::Ptr<cv::rapid::Tracker>(instance->dynamicCast<cv::rapid::Tracker>());
	}

}

