extern "C" {
	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_getInnerPtr_const(const cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_getInnerPtrMut(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::new_null() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_new_null_const() {
			return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>();
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::delete() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_delete(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::to_PtrOfISparseOptFlowEstimator() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::to_PtrOfISparseOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_to_PtrOfISparseOptFlowEstimator(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(instance->dynamicCast<cv::videostab::ISparseOptFlowEstimator>());
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::to_PtrOfPyrLkOptFlowEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::to_PtrOfPyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_to_PtrOfPyrLkOptFlowEstimatorBase(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return new cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>(instance->dynamicCast<cv::videostab::PyrLkOptFlowEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>::new", vec![(pred!(const, ["val"], ["cv::videostab::SparsePyrLkOptFlowEstimator"]), _)]),
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_new_const_SparsePyrLkOptFlowEstimator(cv::videostab::SparsePyrLkOptFlowEstimator* val) {
			return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>(val);
	}

}

