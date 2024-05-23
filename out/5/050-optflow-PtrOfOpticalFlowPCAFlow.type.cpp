extern "C" {
	// cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::OpticalFlowPCAFlow* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::OpticalFlowPCAFlow* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::new_null() generated
	// ("cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_new_null_const() {
			return new cv::Ptr<cv::optflow::OpticalFlowPCAFlow>();
	}

	// cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::delete() generated
	// ("cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_OpticalFlowPCAFlowG_delete(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::to_PtrOfDenseOpticalFlow() generated
	// ("cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::to_PtrOfDenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}

	// cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::new(TraitClass) generated
	// ("cv::Ptr<cv::optflow::OpticalFlowPCAFlow>::new", vec![(pred!(const, ["val"], ["cv::optflow::OpticalFlowPCAFlow"]), _)]),
	cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_new_const_OpticalFlowPCAFlow(cv::optflow::OpticalFlowPCAFlow* val) {
			return new cv::Ptr<cv::optflow::OpticalFlowPCAFlow>(val);
	}

}

