extern "C" {
	// cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::getInnerPtr() generated
	// ("cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::phase_unwrapping::HistogramPhaseUnwrapping* cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_getInnerPtr_const(const cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::phase_unwrapping::HistogramPhaseUnwrapping* cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_getInnerPtrMut(cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::new_null() generated
	// ("cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_new_null_const() {
			return new cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>();
	}

	// cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::delete() generated
	// ("cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_delete(cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_to_PtrOfAlgorithm(cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::to_PtrOfPhaseUnwrapping() generated
	// ("cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>::to_PtrOfPhaseUnwrapping", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_to_PtrOfPhaseUnwrapping(cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>* instance) {
			return new cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>(instance->dynamicCast<cv::phase_unwrapping::PhaseUnwrapping>());
	}

}

