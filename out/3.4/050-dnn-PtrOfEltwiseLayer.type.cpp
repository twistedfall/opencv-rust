extern "C" {
	// cv::Ptr<cv::dnn::EltwiseLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::EltwiseLayer* cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::EltwiseLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::EltwiseLayer* cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::EltwiseLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::EltwiseLayer>* cv_PtrLcv_dnn_EltwiseLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::EltwiseLayer>();
	}

	// cv::Ptr<cv::dnn::EltwiseLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_EltwiseLayerG_delete(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::EltwiseLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::EltwiseLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::EltwiseLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::EltwiseLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::EltwiseLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::EltwiseLayer"]), _)]),
	cv::Ptr<cv::dnn::EltwiseLayer>* cv_PtrLcv_dnn_EltwiseLayerG_new_const_EltwiseLayer(cv::dnn::EltwiseLayer* val) {
			return new cv::Ptr<cv::dnn::EltwiseLayer>(val);
	}

}

