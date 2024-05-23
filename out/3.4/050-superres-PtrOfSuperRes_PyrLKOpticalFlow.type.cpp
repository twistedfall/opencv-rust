extern "C" {
	// cv::Ptr<cv::superres::PyrLKOpticalFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::superres::PyrLKOpticalFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::superres::PyrLKOpticalFlow* cv_PtrLcv_superres_PyrLKOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::PyrLKOpticalFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::superres::PyrLKOpticalFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::superres::PyrLKOpticalFlow* cv_PtrLcv_superres_PyrLKOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::PyrLKOpticalFlow>::new_null() generated
	// ("cv::Ptr<cv::superres::PyrLKOpticalFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::superres::PyrLKOpticalFlow>* cv_PtrLcv_superres_PyrLKOpticalFlowG_new_null_const() {
			return new cv::Ptr<cv::superres::PyrLKOpticalFlow>();
	}

	// cv::Ptr<cv::superres::PyrLKOpticalFlow>::delete() generated
	// ("cv::Ptr<cv::superres::PyrLKOpticalFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_superres_PyrLKOpticalFlowG_delete(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::superres::PyrLKOpticalFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::superres::PyrLKOpticalFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_PyrLKOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::superres::PyrLKOpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt() generated
	// ("cv::Ptr<cv::superres::PyrLKOpticalFlow>::to_PtrOfSuperRes_DenseOpticalFlowExt", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_PyrLKOpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}

}

