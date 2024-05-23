extern "C" {
	// cv::Ptr<cv::superres::DenseOpticalFlowExt>::getInnerPtr() generated
	// ("cv::Ptr<cv::superres::DenseOpticalFlowExt>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::superres::DenseOpticalFlowExt* cv_PtrLcv_superres_DenseOpticalFlowExtG_getInnerPtr_const(const cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::DenseOpticalFlowExt>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::superres::DenseOpticalFlowExt>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::superres::DenseOpticalFlowExt* cv_PtrLcv_superres_DenseOpticalFlowExtG_getInnerPtrMut(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::superres::DenseOpticalFlowExt>::new_null() generated
	// ("cv::Ptr<cv::superres::DenseOpticalFlowExt>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_DenseOpticalFlowExtG_new_null_const() {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>();
	}

	// cv::Ptr<cv::superres::DenseOpticalFlowExt>::delete() generated
	// ("cv::Ptr<cv::superres::DenseOpticalFlowExt>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_superres_DenseOpticalFlowExtG_delete(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::superres::DenseOpticalFlowExt>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::superres::DenseOpticalFlowExt>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_DenseOpticalFlowExtG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

