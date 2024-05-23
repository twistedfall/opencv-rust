extern "C" {
	// cv::Ptr<cv::optflow::VariationalRefinement>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::VariationalRefinement>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::VariationalRefinement* cv_PtrLcv_optflow_VariationalRefinementG_getInnerPtr_const(const cv::Ptr<cv::optflow::VariationalRefinement>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::VariationalRefinement>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::VariationalRefinement>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::VariationalRefinement* cv_PtrLcv_optflow_VariationalRefinementG_getInnerPtrMut(cv::Ptr<cv::optflow::VariationalRefinement>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::VariationalRefinement>::new_null() generated
	// ("cv::Ptr<cv::optflow::VariationalRefinement>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::VariationalRefinement>* cv_PtrLcv_optflow_VariationalRefinementG_new_null_const() {
			return new cv::Ptr<cv::optflow::VariationalRefinement>();
	}

	// cv::Ptr<cv::optflow::VariationalRefinement>::delete() generated
	// ("cv::Ptr<cv::optflow::VariationalRefinement>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_VariationalRefinementG_delete(cv::Ptr<cv::optflow::VariationalRefinement>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::VariationalRefinement>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::optflow::VariationalRefinement>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_VariationalRefinementG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::VariationalRefinement>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::optflow::VariationalRefinement>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::optflow::VariationalRefinement>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_optflow_VariationalRefinementG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::optflow::VariationalRefinement>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

}

