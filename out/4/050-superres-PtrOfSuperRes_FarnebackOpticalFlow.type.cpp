extern "C" {
	// cv::Ptr<cv::superres::FarnebackOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::superres::FarnebackOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::superres::FarnebackOpticalFlow* cv_PtrLcv_superres_FarnebackOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::FarnebackOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::superres::FarnebackOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::superres::FarnebackOpticalFlow* cv_PtrLcv_superres_FarnebackOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::FarnebackOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::superres::FarnebackOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::superres::FarnebackOpticalFlow>* cv_PtrLcv_superres_FarnebackOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::superres::FarnebackOpticalFlow>();
	}

	// cv::Ptr<cv::superres::FarnebackOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::superres::FarnebackOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_superres_FarnebackOpticalFlowG_delete(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::superres::FarnebackOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::superres::FarnebackOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_FarnebackOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::superres::FarnebackOpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt() generated
	// ("cv::Ptr<cv::superres::FarnebackOpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_FarnebackOpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}

}

