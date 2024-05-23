extern "C" {
	// cv::Ptr<cv::VariationalRefinement>::getInnerPtr() generated
	// ("cv::Ptr<cv::VariationalRefinement>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::VariationalRefinement* cv_PtrLcv_VariationalRefinementG_getInnerPtr_const(const cv::Ptr<cv::VariationalRefinement>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::VariationalRefinement>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::VariationalRefinement>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::VariationalRefinement* cv_PtrLcv_VariationalRefinementG_getInnerPtrMut(cv::Ptr<cv::VariationalRefinement>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::VariationalRefinement>::new_null() generated
	// ("cv::Ptr<cv::VariationalRefinement>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::VariationalRefinement>* cv_PtrLcv_VariationalRefinementG_new_null_const() {
			return new cv::Ptr<cv::VariationalRefinement>();
	}

	// cv::Ptr<cv::VariationalRefinement>::delete() generated
	// ("cv::Ptr<cv::VariationalRefinement>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_VariationalRefinementG_delete(cv::Ptr<cv::VariationalRefinement>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::VariationalRefinement>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::VariationalRefinement>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_VariationalRefinementG_to_PtrOfAlgorithm(cv::Ptr<cv::VariationalRefinement>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::VariationalRefinement>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::VariationalRefinement>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_VariationalRefinementG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::VariationalRefinement>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

}

