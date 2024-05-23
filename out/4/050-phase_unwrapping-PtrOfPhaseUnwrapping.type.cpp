extern "C" {
	// cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::getInnerPtr() generated
	// ("cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::phase_unwrapping::PhaseUnwrapping* cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_getInnerPtr_const(const cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::phase_unwrapping::PhaseUnwrapping* cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_getInnerPtrMut(cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::new_null() generated
	// ("cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_new_null_const() {
			return new cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>();
	}

	// cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::delete() generated
	// ("cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_delete(cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_to_PtrOfAlgorithm(cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

