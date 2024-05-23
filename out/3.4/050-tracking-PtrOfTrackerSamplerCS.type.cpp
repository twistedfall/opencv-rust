extern "C" {
	// cv::Ptr<cv::TrackerSamplerCS>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerSamplerCS>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerSamplerCS* cv_PtrLcv_TrackerSamplerCSG_getInnerPtr_const(const cv::Ptr<cv::TrackerSamplerCS>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerSamplerCS>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerSamplerCS>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerCS* cv_PtrLcv_TrackerSamplerCSG_getInnerPtrMut(cv::Ptr<cv::TrackerSamplerCS>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerSamplerCS>::new_null() generated
	// ("cv::Ptr<cv::TrackerSamplerCS>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerSamplerCS>* cv_PtrLcv_TrackerSamplerCSG_new_null_const() {
			return new cv::Ptr<cv::TrackerSamplerCS>();
	}

	// cv::Ptr<cv::TrackerSamplerCS>::delete() generated
	// ("cv::Ptr<cv::TrackerSamplerCS>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerSamplerCSG_delete(cv::Ptr<cv::TrackerSamplerCS>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerSamplerCS>::to_PtrOfTrackerSamplerAlgorithm() generated
	// ("cv::Ptr<cv::TrackerSamplerCS>::to_PtrOfTrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerSamplerAlgorithm>* cv_PtrLcv_TrackerSamplerCSG_to_PtrOfTrackerSamplerAlgorithm(cv::Ptr<cv::TrackerSamplerCS>* instance) {
			return new cv::Ptr<cv::TrackerSamplerAlgorithm>(instance->dynamicCast<cv::TrackerSamplerAlgorithm>());
	}

	// cv::Ptr<cv::TrackerSamplerCS>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerSamplerCS>::new", vec![(pred!(const, ["val"], ["cv::TrackerSamplerCS"]), _)]),
	cv::Ptr<cv::TrackerSamplerCS>* cv_PtrLcv_TrackerSamplerCSG_new_const_TrackerSamplerCS(cv::TrackerSamplerCS* val) {
			return new cv::Ptr<cv::TrackerSamplerCS>(val);
	}

}

