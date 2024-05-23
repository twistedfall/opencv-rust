extern "C" {
	// cv::Ptr<cv::DualTVL1OpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::DualTVL1OpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::DualTVL1OpticalFlow* cv_PtrLcv_DualTVL1OpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DualTVL1OpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::DualTVL1OpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::DualTVL1OpticalFlow* cv_PtrLcv_DualTVL1OpticalFlowG_getInnerPtrMut(cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DualTVL1OpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::DualTVL1OpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::DualTVL1OpticalFlow>* cv_PtrLcv_DualTVL1OpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::DualTVL1OpticalFlow>();
	}

	// cv::Ptr<cv::DualTVL1OpticalFlow>::delete() generated
	// ("cv::Ptr<cv::DualTVL1OpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_DualTVL1OpticalFlowG_delete(cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::DualTVL1OpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::DualTVL1OpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DualTVL1OpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::DualTVL1OpticalFlow>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::DualTVL1OpticalFlow>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_DualTVL1OpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

}

