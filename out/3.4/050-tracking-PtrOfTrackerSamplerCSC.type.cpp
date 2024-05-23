extern "C" {
	// cv::Ptr<cv::TrackerSamplerCSC>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerSamplerCSC>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerSamplerCSC* cv_PtrLcv_TrackerSamplerCSCG_getInnerPtr_const(const cv::Ptr<cv::TrackerSamplerCSC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerSamplerCSC>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerSamplerCSC>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerCSC* cv_PtrLcv_TrackerSamplerCSCG_getInnerPtrMut(cv::Ptr<cv::TrackerSamplerCSC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerSamplerCSC>::new_null() generated
	// ("cv::Ptr<cv::TrackerSamplerCSC>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerSamplerCSC>* cv_PtrLcv_TrackerSamplerCSCG_new_null_const() {
			return new cv::Ptr<cv::TrackerSamplerCSC>();
	}

	// cv::Ptr<cv::TrackerSamplerCSC>::delete() generated
	// ("cv::Ptr<cv::TrackerSamplerCSC>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerSamplerCSCG_delete(cv::Ptr<cv::TrackerSamplerCSC>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerSamplerCSC>::to_PtrOfTrackerSamplerAlgorithm() generated
	// ("cv::Ptr<cv::TrackerSamplerCSC>::to_PtrOfTrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerSamplerAlgorithm>* cv_PtrLcv_TrackerSamplerCSCG_to_PtrOfTrackerSamplerAlgorithm(cv::Ptr<cv::TrackerSamplerCSC>* instance) {
			return new cv::Ptr<cv::TrackerSamplerAlgorithm>(instance->dynamicCast<cv::TrackerSamplerAlgorithm>());
	}

	// cv::Ptr<cv::TrackerSamplerCSC>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerSamplerCSC>::new", vec![(pred!(const, ["val"], ["cv::TrackerSamplerCSC"]), _)]),
	cv::Ptr<cv::TrackerSamplerCSC>* cv_PtrLcv_TrackerSamplerCSCG_new_const_TrackerSamplerCSC(cv::TrackerSamplerCSC* val) {
			return new cv::Ptr<cv::TrackerSamplerCSC>(val);
	}

}

