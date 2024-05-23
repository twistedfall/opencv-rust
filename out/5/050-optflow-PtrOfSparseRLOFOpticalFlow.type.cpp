extern "C" {
	// cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::SparseRLOFOpticalFlow* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::SparseRLOFOpticalFlow* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>();
	}

	// cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_delete(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::to_PtrOfSparseOpticalFlow() generated
	// ("cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>::to_PtrOfSparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::SparseOpticalFlow>* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_to_PtrOfSparseOpticalFlow(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::SparseOpticalFlow>(instance->dynamicCast<cv::SparseOpticalFlow>());
	}

}

