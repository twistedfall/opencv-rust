extern "C" {
	// cv::Ptr<cv::SparsePyrLKOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::SparsePyrLKOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::SparsePyrLKOpticalFlow* cv_PtrLcv_SparsePyrLKOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SparsePyrLKOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::SparsePyrLKOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::SparsePyrLKOpticalFlow* cv_PtrLcv_SparsePyrLKOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SparsePyrLKOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::SparsePyrLKOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::SparsePyrLKOpticalFlow>* cv_PtrLcv_SparsePyrLKOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::SparsePyrLKOpticalFlow>();
	}

	// cv::Ptr<cv::SparsePyrLKOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::SparsePyrLKOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_SparsePyrLKOpticalFlowG_delete(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::SparsePyrLKOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::SparsePyrLKOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SparsePyrLKOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::SparsePyrLKOpticalFlow>::to_PtrOfSparseOpticalFlow() generated
	// ("cv::Ptr<cv::SparsePyrLKOpticalFlow>::to_PtrOfSparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::SparseOpticalFlow>* cv_PtrLcv_SparsePyrLKOpticalFlowG_to_PtrOfSparseOpticalFlow(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::SparseOpticalFlow>(instance->dynamicCast<cv::SparseOpticalFlow>());
	}

}

