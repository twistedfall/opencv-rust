extern "C" {
	// cv::Ptr<cv::dnn::Reshape2Layer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::Reshape2Layer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::Reshape2Layer* cv_PtrLcv_dnn_Reshape2LayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::Reshape2Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Reshape2Layer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::Reshape2Layer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Reshape2Layer* cv_PtrLcv_dnn_Reshape2LayerG_getInnerPtrMut(cv::Ptr<cv::dnn::Reshape2Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Reshape2Layer>::new_null() generated
	// ("cv::Ptr<cv::dnn::Reshape2Layer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::Reshape2Layer>* cv_PtrLcv_dnn_Reshape2LayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::Reshape2Layer>();
	}

	// cv::Ptr<cv::dnn::Reshape2Layer>::delete() generated
	// ("cv::Ptr<cv::dnn::Reshape2Layer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_Reshape2LayerG_delete(cv::Ptr<cv::dnn::Reshape2Layer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::Reshape2Layer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::Reshape2Layer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_Reshape2LayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::Reshape2Layer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::Reshape2Layer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::Reshape2Layer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_Reshape2LayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::Reshape2Layer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::Reshape2Layer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::Reshape2Layer>::new", vec![(pred!(const, ["val"], ["cv::dnn::Reshape2Layer"]), _)]),
	cv::Ptr<cv::dnn::Reshape2Layer>* cv_PtrLcv_dnn_Reshape2LayerG_new_const_Reshape2Layer(cv::dnn::Reshape2Layer* val) {
			return new cv::Ptr<cv::dnn::Reshape2Layer>(val);
	}

}

