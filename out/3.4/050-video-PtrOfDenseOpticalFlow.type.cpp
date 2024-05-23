extern "C" {
	// cv::Ptr<cv::DenseOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::DenseOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::DenseOpticalFlow* cv_PtrLcv_DenseOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::DenseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DenseOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::DenseOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_PtrLcv_DenseOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::DenseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::DenseOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::DenseOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_DenseOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::DenseOpticalFlow>();
	}

	// cv::Ptr<cv::DenseOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::DenseOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_DenseOpticalFlowG_delete(cv::Ptr<cv::DenseOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::DenseOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::DenseOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DenseOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::DenseOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

