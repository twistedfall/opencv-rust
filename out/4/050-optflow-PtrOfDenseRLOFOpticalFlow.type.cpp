extern "C" {
	// cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::DenseRLOFOpticalFlow* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::DenseRLOFOpticalFlow* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>();
	}

	// cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_delete(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

}

