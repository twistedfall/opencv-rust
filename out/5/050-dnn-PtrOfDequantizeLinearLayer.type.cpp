extern "C" {
	// cv::Ptr<cv::dnn::DequantizeLinearLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::DequantizeLinearLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::DequantizeLinearLayer* cv_PtrLcv_dnn_DequantizeLinearLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::DequantizeLinearLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DequantizeLinearLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::DequantizeLinearLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::DequantizeLinearLayer* cv_PtrLcv_dnn_DequantizeLinearLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::DequantizeLinearLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::DequantizeLinearLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::DequantizeLinearLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::DequantizeLinearLayer>* cv_PtrLcv_dnn_DequantizeLinearLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::DequantizeLinearLayer>();
	}

	// cv::Ptr<cv::dnn::DequantizeLinearLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::DequantizeLinearLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_DequantizeLinearLayerG_delete(cv::Ptr<cv::dnn::DequantizeLinearLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::DequantizeLinearLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::DequantizeLinearLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_DequantizeLinearLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::DequantizeLinearLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::DequantizeLinearLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::DequantizeLinearLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_DequantizeLinearLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::DequantizeLinearLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::DequantizeLinearLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::DequantizeLinearLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::DequantizeLinearLayer"]), _)]),
	cv::Ptr<cv::dnn::DequantizeLinearLayer>* cv_PtrLcv_dnn_DequantizeLinearLayerG_new_const_DequantizeLinearLayer(cv::dnn::DequantizeLinearLayer* val) {
			return new cv::Ptr<cv::dnn::DequantizeLinearLayer>(val);
	}

}

