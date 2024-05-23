extern "C" {
	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::SparsePyrLkOptFlowEstimatorGpu* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_getInnerPtr_const(const cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::SparsePyrLkOptFlowEstimatorGpu* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_getInnerPtrMut(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::new_null() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_new_null_const() {
			return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>();
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::delete() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_delete(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::to_PtrOfISparseOptFlowEstimator() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::to_PtrOfISparseOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_to_PtrOfISparseOptFlowEstimator(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
			return new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(instance->dynamicCast<cv::videostab::ISparseOptFlowEstimator>());
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::to_PtrOfPyrLkOptFlowEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::to_PtrOfPyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_to_PtrOfPyrLkOptFlowEstimatorBase(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* instance) {
			return new cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>(instance->dynamicCast<cv::videostab::PyrLkOptFlowEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>::new", vec![(pred!(const, ["val"], ["cv::videostab::SparsePyrLkOptFlowEstimatorGpu"]), _)]),
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorGpuG_new_const_SparsePyrLkOptFlowEstimatorGpu(cv::videostab::SparsePyrLkOptFlowEstimatorGpu* val) {
			return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimatorGpu>(val);
	}

}

