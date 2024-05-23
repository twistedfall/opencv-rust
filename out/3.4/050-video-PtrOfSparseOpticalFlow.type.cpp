extern "C" {
	// cv::Ptr<cv::SparseOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::SparseOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::SparseOpticalFlow* cv_PtrLcv_SparseOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::SparseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SparseOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::SparseOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::SparseOpticalFlow* cv_PtrLcv_SparseOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::SparseOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SparseOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::SparseOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::SparseOpticalFlow>* cv_PtrLcv_SparseOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::SparseOpticalFlow>();
	}

	// cv::Ptr<cv::SparseOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::SparseOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_SparseOpticalFlowG_delete(cv::Ptr<cv::SparseOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::SparseOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::SparseOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SparseOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::SparseOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

