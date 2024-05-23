extern "C" {
	// cv::Ptr<cv::dnn::LayerNormLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::LayerNormLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::LayerNormLayer* cv_PtrLcv_dnn_LayerNormLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::LayerNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LayerNormLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::LayerNormLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LayerNormLayer* cv_PtrLcv_dnn_LayerNormLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::LayerNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LayerNormLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::LayerNormLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::LayerNormLayer>* cv_PtrLcv_dnn_LayerNormLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::LayerNormLayer>();
	}

	// cv::Ptr<cv::dnn::LayerNormLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::LayerNormLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_LayerNormLayerG_delete(cv::Ptr<cv::dnn::LayerNormLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::LayerNormLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::LayerNormLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_LayerNormLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::LayerNormLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::LayerNormLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::LayerNormLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_LayerNormLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::LayerNormLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::LayerNormLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::LayerNormLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::LayerNormLayer"]), _)]),
	cv::Ptr<cv::dnn::LayerNormLayer>* cv_PtrLcv_dnn_LayerNormLayerG_new_const_LayerNormLayer(cv::dnn::LayerNormLayer* val) {
			return new cv::Ptr<cv::dnn::LayerNormLayer>(val);
	}

}

