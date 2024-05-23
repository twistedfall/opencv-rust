extern "C" {
	// cv::Ptr<cv::superres::DualTVL1OpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::superres::DualTVL1OpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::superres::DualTVL1OpticalFlow* cv_PtrLcv_superres_DualTVL1OpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::DualTVL1OpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::superres::DualTVL1OpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::superres::DualTVL1OpticalFlow* cv_PtrLcv_superres_DualTVL1OpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::DualTVL1OpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::superres::DualTVL1OpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::superres::DualTVL1OpticalFlow>* cv_PtrLcv_superres_DualTVL1OpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::superres::DualTVL1OpticalFlow>();
	}

	// cv::Ptr<cv::superres::DualTVL1OpticalFlow>::delete() generated
	// ("cv::Ptr<cv::superres::DualTVL1OpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_superres_DualTVL1OpticalFlowG_delete(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::superres::DualTVL1OpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::superres::DualTVL1OpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_DualTVL1OpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::superres::DualTVL1OpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt() generated
	// ("cv::Ptr<cv::superres::DualTVL1OpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_DualTVL1OpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}

}

