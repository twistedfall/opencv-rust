extern "C" {
	// cv::Ptr<cv::FarnebackOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::FarnebackOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::FarnebackOpticalFlow* cv_PtrLcv_FarnebackOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FarnebackOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::FarnebackOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::FarnebackOpticalFlow* cv_PtrLcv_FarnebackOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FarnebackOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::FarnebackOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::FarnebackOpticalFlow>* cv_PtrLcv_FarnebackOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::FarnebackOpticalFlow>();
	}

	// cv::Ptr<cv::FarnebackOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::FarnebackOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FarnebackOpticalFlowG_delete(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::FarnebackOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::FarnebackOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_FarnebackOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::FarnebackOpticalFlow>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::FarnebackOpticalFlow>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_FarnebackOpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

}

