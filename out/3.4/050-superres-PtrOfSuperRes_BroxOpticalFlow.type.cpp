extern "C" {
	// cv::Ptr<cv::superres::BroxOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::superres::BroxOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::superres::BroxOpticalFlow* cv_PtrLcv_superres_BroxOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::BroxOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::superres::BroxOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::superres::BroxOpticalFlow* cv_PtrLcv_superres_BroxOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::BroxOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::superres::BroxOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::superres::BroxOpticalFlow>* cv_PtrLcv_superres_BroxOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::superres::BroxOpticalFlow>();
	}

	// cv::Ptr<cv::superres::BroxOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::superres::BroxOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_superres_BroxOpticalFlowG_delete(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::superres::BroxOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::superres::BroxOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_BroxOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::superres::BroxOpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt() generated
	// ("cv::Ptr<cv::superres::BroxOpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_BroxOpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}

}

