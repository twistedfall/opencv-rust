extern "C" {
	// cv::Ptr<cv::TrackerSamplerPF>::getInnerPtr() generated
	// ("cv::Ptr<cv::TrackerSamplerPF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TrackerSamplerPF* cv_PtrLcv_TrackerSamplerPFG_getInnerPtr_const(const cv::Ptr<cv::TrackerSamplerPF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerSamplerPF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TrackerSamplerPF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerPF* cv_PtrLcv_TrackerSamplerPFG_getInnerPtrMut(cv::Ptr<cv::TrackerSamplerPF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TrackerSamplerPF>::new_null() generated
	// ("cv::Ptr<cv::TrackerSamplerPF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TrackerSamplerPF>* cv_PtrLcv_TrackerSamplerPFG_new_null_const() {
			return new cv::Ptr<cv::TrackerSamplerPF>();
	}

	// cv::Ptr<cv::TrackerSamplerPF>::delete() generated
	// ("cv::Ptr<cv::TrackerSamplerPF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TrackerSamplerPFG_delete(cv::Ptr<cv::TrackerSamplerPF>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TrackerSamplerPF>::to_PtrOfTrackerSamplerAlgorithm() generated
	// ("cv::Ptr<cv::TrackerSamplerPF>::to_PtrOfTrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::TrackerSamplerAlgorithm>* cv_PtrLcv_TrackerSamplerPFG_to_PtrOfTrackerSamplerAlgorithm(cv::Ptr<cv::TrackerSamplerPF>* instance) {
			return new cv::Ptr<cv::TrackerSamplerAlgorithm>(instance->dynamicCast<cv::TrackerSamplerAlgorithm>());
	}

	// cv::Ptr<cv::TrackerSamplerPF>::new(TraitClass) generated
	// ("cv::Ptr<cv::TrackerSamplerPF>::new", vec![(pred!(const, ["val"], ["cv::TrackerSamplerPF"]), _)]),
	cv::Ptr<cv::TrackerSamplerPF>* cv_PtrLcv_TrackerSamplerPFG_new_const_TrackerSamplerPF(cv::TrackerSamplerPF* val) {
			return new cv::Ptr<cv::TrackerSamplerPF>(val);
	}

}

