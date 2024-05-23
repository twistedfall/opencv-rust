extern "C" {
	// cv::Ptr<cv::DISOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::DISOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::DISOpticalFlow* cv_PtrLcv_DISOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::DISOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DISOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::DISOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::DISOpticalFlow* cv_PtrLcv_DISOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::DISOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DISOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::DISOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::DISOpticalFlow>* cv_PtrLcv_DISOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::DISOpticalFlow>();
	}

	// cv::Ptr<cv::DISOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::DISOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_DISOpticalFlowG_delete(cv::Ptr<cv::DISOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::DISOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::DISOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DISOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::DISOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::DISOpticalFlow>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::DISOpticalFlow>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_DISOpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::DISOpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

}

