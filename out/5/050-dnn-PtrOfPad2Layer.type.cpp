extern "C" {
	// cv::Ptr<cv::dnn::Pad2Layer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::Pad2Layer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::Pad2Layer* cv_PtrLcv_dnn_Pad2LayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::Pad2Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Pad2Layer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::Pad2Layer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Pad2Layer* cv_PtrLcv_dnn_Pad2LayerG_getInnerPtrMut(cv::Ptr<cv::dnn::Pad2Layer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Pad2Layer>::new_null() generated
	// ("cv::Ptr<cv::dnn::Pad2Layer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::Pad2Layer>* cv_PtrLcv_dnn_Pad2LayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::Pad2Layer>();
	}

	// cv::Ptr<cv::dnn::Pad2Layer>::delete() generated
	// ("cv::Ptr<cv::dnn::Pad2Layer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_Pad2LayerG_delete(cv::Ptr<cv::dnn::Pad2Layer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::Pad2Layer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::Pad2Layer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_Pad2LayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::Pad2Layer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::Pad2Layer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::Pad2Layer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_Pad2LayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::Pad2Layer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::Pad2Layer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::Pad2Layer>::new", vec![(pred!(const, ["val"], ["cv::dnn::Pad2Layer"]), _)]),
	cv::Ptr<cv::dnn::Pad2Layer>* cv_PtrLcv_dnn_Pad2LayerG_new_const_Pad2Layer(cv::dnn::Pad2Layer* val) {
			return new cv::Ptr<cv::dnn::Pad2Layer>(val);
	}

}

