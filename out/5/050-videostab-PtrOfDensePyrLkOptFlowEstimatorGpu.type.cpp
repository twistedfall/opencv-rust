extern "C" {
	// cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::DensePyrLkOptFlowEstimatorGpu* cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_getInnerPtr_const(const cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::DensePyrLkOptFlowEstimatorGpu* cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_getInnerPtrMut(cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::new_null() generated
	// ("cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_new_null_const() {
			return new cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>();
	}

	// cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::delete() generated
	// ("cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_delete(cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::to_PtrOfIDenseOptFlowEstimator() generated
	// ("cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::to_PtrOfIDenseOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_to_PtrOfIDenseOptFlowEstimator(cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
			return new cv::Ptr<cv::videostab::IDenseOptFlowEstimator>(instance->dynamicCast<cv::videostab::IDenseOptFlowEstimator>());
	}

	// cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::to_PtrOfPyrLkOptFlowEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::to_PtrOfPyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>* cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_to_PtrOfPyrLkOptFlowEstimatorBase(cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* instance) {
			return new cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>(instance->dynamicCast<cv::videostab::PyrLkOptFlowEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>::new", vec![(pred!(const, ["val"], ["cv::videostab::DensePyrLkOptFlowEstimatorGpu"]), _)]),
	cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>* cv_PtrLcv_videostab_DensePyrLkOptFlowEstimatorGpuG_new_const_DensePyrLkOptFlowEstimatorGpu(cv::videostab::DensePyrLkOptFlowEstimatorGpu* val) {
			return new cv::Ptr<cv::videostab::DensePyrLkOptFlowEstimatorGpu>(val);
	}

}

