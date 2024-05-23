extern "C" {
	// cv::Ptr<cv::optflow::DISOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::DISOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::DISOpticalFlow* cv_PtrLcv_optflow_DISOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::DISOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::DISOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::DISOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::DISOpticalFlow* cv_PtrLcv_optflow_DISOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::DISOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::DISOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::optflow::DISOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::DISOpticalFlow>* cv_PtrLcv_optflow_DISOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::optflow::DISOpticalFlow>();
	}

	// cv::Ptr<cv::optflow::DISOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::optflow::DISOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_DISOpticalFlowG_delete(cv::Ptr<cv::optflow::DISOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::DISOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::optflow::DISOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_DISOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::DISOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::optflow::DISOpticalFlow>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::optflow::DISOpticalFlow>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_optflow_DISOpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::optflow::DISOpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

}

